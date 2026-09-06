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

/// Apply compositor blur to a window, reporting which surface mode resulted.
///
/// Never returns an error: a window that cannot be frosted is a Solid window,
/// which is a supported way to run rather than something to recover from.
pub fn apply(window: &WebviewWindow) -> SurfaceMode {
    #[cfg(target_os = "windows")]
    {
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
