//! Surface mode: whether a window is frosted glass or an opaque surface.
//!
//! The frosted surface comes from the OS compositor, applied to a transparent
//! window from here. A web view cannot see the desktop behind it, so this
//! cannot be done in CSS. See docs/DESIGN.md principle 3.
//!
//! Solid is not a failure state. Transparency is a system setting and Windows
//! disables it under battery saver, so some users only ever see Solid. It is
//! designed to look deliberate — docs/DESIGN.md principle 4.

use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SurfaceMode {
    Glass,
    Solid,
}

/// The mode every window is currently in.
///
/// Held rather than computed on demand because it can change while the
/// application is running: the user can switch transparency off, and Windows
/// does it for them under battery saver.
#[derive(Default)]
pub struct Current(Mutex<Option<SurfaceMode>>);

impl Current {
    pub fn get(&self) -> SurfaceMode {
        self.0
            .lock()
            .ok()
            .and_then(|mode| *mode)
            .unwrap_or(SurfaceMode::Solid)
    }

    fn set(&self, mode: SurfaceMode) {
        if let Ok(mut held) = self.0.lock() {
            *held = Some(mode);
        }
    }
}

/// The event a window listens for to re-resolve its surface.
pub const CHANGED: &str = "surface-changed";

/// Where Windows keeps the transparency setting.
#[cfg(target_os = "windows")]
const PERSONALIZE: windows::core::PCWSTR =
    windows::core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize");

/// Whether transparency effects are switched on.
///
/// Unreadable means yes. The setting is normally present, and a missing value
/// is far more likely to be an unusual install than a user who has turned
/// transparency off — and guessing Glass is the recoverable guess, because
/// `apply_acrylic` failing lands on Solid anyway.
#[cfg(target_os = "windows")]
pub fn transparency_enabled() -> bool {
    use windows::core::w;
    use windows::Win32::Foundation::ERROR_SUCCESS;
    use windows::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD};

    let mut value: u32 = 1;
    let mut size = std::mem::size_of::<u32>() as u32;

    // Safe: the key path is a static wide string, the buffer is a u32 whose
    // size is passed alongside it, and the call is told to accept only a DWORD.
    let status = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            PERSONALIZE,
            w!("EnableTransparency"),
            RRF_RT_REG_DWORD,
            None,
            Some(std::ptr::addr_of_mut!(value).cast()),
            Some(&mut size),
        )
    };


    if status != ERROR_SUCCESS {
        return true;
    }

    value != 0
}

#[cfg(not(target_os = "windows"))]
pub fn transparency_enabled() -> bool {
    false
}

/// Round the window itself.
///
/// The compositor draws its backdrop across the whole window rectangle, which
/// is square. A CSS `border-radius` only rounds what the web view paints, so
/// the corners outside it show raw, untinted acrylic — visible as grey
/// triangles. Rounding the window makes the compositor clip its own backdrop.
///
/// Windows chooses the radius; it cannot be set to an arbitrary value. That is
/// why `--radius-window` matches the system radius rather than the other way
/// round. See docs/FIXES.md.
#[cfg(target_os = "windows")]
fn round_corners(window: &WebviewWindow) {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Dwm::{
        DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND,
        DWM_WINDOW_CORNER_PREFERENCE,
    };

    let Ok(handle) = window.hwnd() else {
        return;
    };

    // Tauri's HWND comes from its own version of the windows crate, which is
    // not necessarily this one. The raw value is the same handle either way,
    // so it is carried across rather than the versions being forced to match.
    let handle = HWND(handle.0 as isize);

    let preference = DWMWCP_ROUND;

    // Safe: the handle comes from a live window, and the attribute and its
    // size are matched to the type the API expects.
    unsafe {
        let _ = DwmSetWindowAttribute(
            handle,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            std::ptr::addr_of!(preference).cast(),
            std::mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
        );
    }
}

/// Apply compositor blur to a window, reporting which surface mode resulted.
///
/// Never returns an error: a window that cannot be frosted is a Solid window,
/// which is a supported way to run rather than something to recover from.
pub fn apply(window: &WebviewWindow) -> SurfaceMode {
    #[cfg(target_os = "windows")]
    {
        round_corners(window);

        // Asked before the blur is applied, not only when it fails. With
        // transparency off, `apply_acrylic` still reports success and the
        // compositor then draws nothing behind the window — which is the whole
        // bug: a tint over nothing is a washed-out window rather than a Solid
        // one. See docs/FIXES.md.
        if !transparency_enabled() {
            let _ = window_vibrancy::clear_acrylic(window);
            return SurfaceMode::Solid;
        }

        // A fully transparent tint: the CSS layer above supplies the colour, so
        // the compositor contributes blur only. See src/lib/tokens/tokens.css.
        match window_vibrancy::apply_acrylic(window, Some((0, 0, 0, 0))) {
            Ok(()) => SurfaceMode::Glass,
            Err(_) => SurfaceMode::Solid,
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = window;
        SurfaceMode::Solid
    }
}

/// Re-apply the surface to every open window and report the mode.
///
/// Called at startup and again whenever the setting changes. Windows opened
/// later get theirs from `apply` on the way up, so this only has to catch the
/// ones already on screen.
pub fn refresh(app: &AppHandle) -> SurfaceMode {
    let mut mode = if transparency_enabled() {
        SurfaceMode::Glass
    } else {
        SurfaceMode::Solid
    };

    for window in app.webview_windows().values() {
        // A single window that cannot be frosted decides for all of them:
        // two windows in different modes side by side is worse than both
        // being Solid, and Solid is a supported way to run.
        if matches!(apply(window), SurfaceMode::Solid) {
            mode = SurfaceMode::Solid;
        }
    }

    app.state::<Current>().set(mode);

    // Best effort. A window that missed the event is a window drawn in the
    // previous mode until it is reopened, which is a blemish rather than a
    // fault, and there is nothing useful to do about a failed emit.
    let _ = app.emit(CHANGED, mode);

    mode
}

/// Watch the transparency setting and re-apply the surface when it changes.
///
/// A dedicated thread, because `RegNotifyChangeKeyValue` blocks until
/// something happens — which is exactly what is wanted, and the reason this is
/// not a poll. The thread ends when the key can no longer be watched.
#[cfg(target_os = "windows")]
pub fn watch(app: AppHandle) {
    use windows::Win32::Foundation::{ERROR_SUCCESS, HANDLE};
    use windows::Win32::System::Registry::{
        RegCloseKey, RegNotifyChangeKeyValue, RegOpenKeyExW, HKEY, HKEY_CURRENT_USER, KEY_NOTIFY,
        REG_NOTIFY_CHANGE_LAST_SET,
    };

    std::thread::spawn(move || {
        let mut key = HKEY::default();

        // Safe: a static key path, and the handle is closed before returning.
        let opened = unsafe {
            RegOpenKeyExW(HKEY_CURRENT_USER, PERSONALIZE, 0, KEY_NOTIFY, &mut key)
        };

        if opened != ERROR_SUCCESS {
            eprintln!(
                "sticky.md: cannot watch the transparency setting; the surface will \
                 be whatever it was when each window opened"
            );
            return;
        }

        loop {
            // Blocks until the key changes. Synchronous by request — the whole
            // point of this thread is to be asleep the rest of the time.
            let waited = unsafe {
                RegNotifyChangeKeyValue(key, false, REG_NOTIFY_CHANGE_LAST_SET, HANDLE::default(), false)
            };

            if waited != ERROR_SUCCESS {
                break;
            }

            // The whole Personalize key is watched, not one value: the API
            // cannot watch a single value. Other settings under it change too,
            // so `refresh` re-reads and may well conclude nothing changed.
            // Two clones: one for the closure to own and one to call with.
            let handle = app.clone();
            let inner = app.clone();

            if handle
                .run_on_main_thread(move || {
                    refresh(&inner);
                })
                .is_err()
            {
                break;
            }
        }

        // Safe: the handle was opened above and is not used after this.
        let _ = unsafe { RegCloseKey(key) };
    });
}

#[cfg(not(target_os = "windows"))]
pub fn watch(app: AppHandle) {
    let _ = app;
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    /// Does reading the key block while a synchronous notification is pending
    /// on it from another thread?
    ///
    /// This is the shape the application runs in: `watch` parks a thread inside
    /// `RegNotifyChangeKeyValue` on the Personalize key, and every later
    /// `apply` reads a value out of that same key. If the read blocks, the
    /// event loop stops the moment a window is frosted after startup, and every
    /// window built afterwards waits for a thread that is never coming back.
    #[test]
    fn reading_the_key_does_not_block_while_a_notification_is_pending() {
        use windows::core::w;
        use windows::Win32::Foundation::{ERROR_SUCCESS, HANDLE};
        use windows::Win32::System::Registry::{
            RegNotifyChangeKeyValue, RegOpenKeyExW, HKEY, HKEY_CURRENT_USER, KEY_NOTIFY,
            REG_NOTIFY_CHANGE_LAST_SET,
        };

        let (ready, started) = mpsc::channel::<bool>();

        std::thread::spawn(move || {
            let mut key = HKEY::default();
            let opened = unsafe {
                RegOpenKeyExW(HKEY_CURRENT_USER, super::PERSONALIZE, 0, KEY_NOTIFY, &mut key)
            };
            let _ = ready.send(opened == ERROR_SUCCESS);

            // Parks here until something writes to the key, exactly as `watch`
            // does. The thread is deliberately never joined.
            unsafe {
                let _ = RegNotifyChangeKeyValue(
                    key,
                    false,
                    REG_NOTIFY_CHANGE_LAST_SET,
                    HANDLE::default(),
                    false,
                );
            }
        });

        assert!(started.recv().expect("the watcher thread reported"), "the key opened");
        std::thread::sleep(Duration::from_millis(250));

        let at = Instant::now();
        let _ = super::transparency_enabled();
        let took = at.elapsed();

        assert!(
            took < Duration::from_secs(2),
            "reading the transparency setting took {took:?} while a notification was pending"
        );
    }
}
