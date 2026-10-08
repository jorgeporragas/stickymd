//! Virtual desktops.
//!
//! The hub and the settings window exist once each. Asked for while one of
//! them is open on another virtual desktop, focusing it took the user *to* that
//! desktop, away from whatever they were working on. The window now comes to
//! them instead. SMD-103.
//!
//! Windows only. macOS moves a window to the active Space when it is brought
//! forward, so there is nothing to do there, and the other halves are empty for
//! the reason `surface::watch` gives: the call sites stay plain.

use tauri::WebviewWindow;

/// Move a window to the virtual desktop the user is on, if it is not there.
///
/// Best effort, and silent when it fails. The fallback is the behaviour this
/// replaces: the window is focused where it is and Windows switches desktops
/// to show it. Nothing is lost, it is only less convenient.
#[cfg(target_os = "windows")]
pub fn bring_here(window: &WebviewWindow) {
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
    };
    use windows::Win32::UI::Shell::{IVirtualDesktopManager, VirtualDesktopManager};

    let Some(handle) = crate::surface::native_handle(window) else {
        return;
    };

    // This runs on the main thread from the tray, where COM is already up in
    // the web view's single-threaded apartment, and on the async runtime from
    // a command, where it is not. Asking for the multithreaded apartment
    // covers both. Where an apartment already exists the call fails with
    // RPC_E_CHANGED_MODE and the existing one is used, and only a call that
    // succeeded is balanced by an uninitialise.
    //
    // Safe: the handle comes from a live window, the GUID is a local that
    // outlives the call, and the COM objects are dropped before uninitialising.
    unsafe {
        let initialised = CoInitializeEx(None, COINIT_MULTITHREADED).is_ok();

        {
            let manager: windows::core::Result<IVirtualDesktopManager> =
                CoCreateInstance(&VirtualDesktopManager, None, CLSCTX_ALL);

            if let Ok(manager) = manager {
                let elsewhere = manager
                    .IsWindowOnCurrentVirtualDesktop(handle)
                    .map(|here| !here.as_bool())
                    .unwrap_or(false);

                if elsewhere {
                    if let Some(current) = current_desktop() {
                        let _ = manager.MoveWindowToDesktop(handle, &current);
                    }
                }
            }
        }

        if initialised {
            CoUninitialize();
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn bring_here(_window: &WebviewWindow) {}

/// The ID of the virtual desktop the user is looking at.
///
/// There is no public API for this. `IVirtualDesktopManager` can say whether a
/// window is on the current desktop and move a window to a desktop by ID, but
/// it will not say which desktop is current. Explorer records that in the
/// registry, and reading it from there is what Microsoft's own Windows
/// Terminal and PowerToys do to summon a window. Windows 11 keeps it under
/// `Explorer\VirtualDesktops`, and Windows 10 keeps it per logon session under
/// `Explorer\SessionInfo\<session>\VirtualDesktops`, so both are asked.
#[cfg(target_os = "windows")]
fn current_desktop() -> Option<windows::core::GUID> {
    use windows::core::HSTRING;
    use windows::Win32::System::RemoteDesktop::ProcessIdToSessionId;
    use windows::Win32::System::Threading::GetCurrentProcessId;

    let windows_11 = read_guid(&HSTRING::from(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\VirtualDesktops",
    ));
    if windows_11.is_some() {
        return windows_11;
    }

    let mut session = 0u32;
    // Safe: the session ID is written to a local u32.
    unsafe { ProcessIdToSessionId(GetCurrentProcessId(), &mut session) }.ok()?;

    read_guid(&HSTRING::from(format!(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\SessionInfo\\{session}\\VirtualDesktops"
    )))
}

/// Read `CurrentVirtualDesktop` under a key, as a GUID.
#[cfg(target_os = "windows")]
fn read_guid(key: &windows::core::HSTRING) -> Option<windows::core::GUID> {
    use windows::core::{w, PCWSTR};
    use windows::Win32::Foundation::ERROR_SUCCESS;
    use windows::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_BINARY};

    let mut bytes = [0u8; 16];
    let mut size = bytes.len() as u32;

    // Safe: the buffer is 16 bytes and its size is passed alongside it, and the
    // call is told to accept only binary data.
    let status = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            PCWSTR(key.as_ptr()),
            w!("CurrentVirtualDesktop"),
            RRF_RT_REG_BINARY,
            None,
            Some(bytes.as_mut_ptr().cast()),
            Some(&mut size),
        )
    };

    // A value that is not exactly a GUID is not one, whatever it says.
    if status != ERROR_SUCCESS || size != 16 {
        return None;
    }

    // Stored as the GUID's own in-memory layout, which is little-endian for
    // the first three fields on every machine Windows runs on.
    Some(windows::core::GUID::from_values(
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        u16::from_le_bytes([bytes[4], bytes[5]]),
        u16::from_le_bytes([bytes[6], bytes[7]]),
        [bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]],
    ))
}
