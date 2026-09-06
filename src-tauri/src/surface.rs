//! Surface mode: whether a window is frosted glass or an opaque surface.
//!
//! The frosted surface comes from the OS compositor, applied to a transparent
//! window from here. A web view cannot see the desktop behind it, so this
//! cannot be done in CSS. See docs/DESIGN.md principle 3.
//!
//! Solid is not a failure state. Transparency is a system setting and Windows
//! disables it under battery saver, so some users only ever see Solid. It is
//! designed to look deliberate — docs/DESIGN.md principle 4.

use serde::Serialize;
use tauri::WebviewWindow;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SurfaceMode {
    Glass,
    Solid,
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
