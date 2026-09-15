//! What tells a development build apart from the real one.
//!
//! Two builds of sticky.md can be on screen at once and they are identical:
//! same mark, same chromeless windows, same tray. The founder kept arriving in
//! the wrong one. So a debug build wears the mark in a different colour and
//! says `(dev)` wherever a name is shown.
//!
//! **Everything here is gated on `debug_assertions`**, which is a property of
//! the compiler rather than of a configuration file. A release build does not
//! contain this code, so there is no switch to forget, no file to exclude from
//! the bundle, and no way for a shipped binary to wear the dev mark. The
//! release workflow needs to know nothing about any of it.
//!
//! The `not(debug_assertions)` halves are empty on purpose: the call sites stay
//! plain, the way `surface::watch` is a no-op off Windows rather than something
//! every caller has to branch on.

#[cfg(debug_assertions)]
use std::sync::OnceLock;

#[cfg(debug_assertions)]
use tauri::Manager;
use tauri::{AppHandle, WebviewWindow};

/// How a development build names itself, appended to whatever a window or the
/// tray already says.
#[cfg(debug_assertions)]
const SUFFIX: &str = " (dev)";

/// The hue the mark is turned to, in Oklab degrees.
///
/// 60 is the amber SMD-093 landed on when dark yellow turned out to be olive —
/// reused rather than picked again, because it is the warm slot this palette
/// has already agreed on. Against the mark's own green it is most of the way
/// round the wheel, which is what makes the two legible apart at the 16px the
/// tray actually draws.
#[cfg(debug_assertions)]
const DEV_HUE: f32 = 60.0;

/// The application mark with only its hue turned.
///
/// Derived from the real mark at runtime rather than drawn as a second asset,
/// and that is the point: `docs/DESIGN.md` says the mark is temporary until the
/// founder draws it properly, and a hand-maintained dev copy would still be the
/// old one the morning after he does. This one cannot go stale — it is whatever
/// the mark currently is, in a different colour.
///
/// Only the hue moves. Lightness and chroma are held, which is the same
/// arithmetic `--swatch-*-engaged` uses to make "selected" follow a note's
/// tint: one colour with the hue rotated reads as the same thing wearing a
/// different coat, where a freshly picked colour reads as a different thing.
/// The near-black of the S has almost no chroma, so turning its hue moves it
/// almost not at all — the disc changes colour and the letter stays the letter.
#[cfg(debug_assertions)]
fn mark(app: &AppHandle) -> Option<&'static tauri::image::Image<'static>> {
    static MARK: OnceLock<Option<tauri::image::Image<'static>>> = OnceLock::new();

    MARK.get_or_init(|| {
        let source = app.default_window_icon()?;
        let mut rgba = source.rgba().to_vec();

        for pixel in rgba.chunks_exact_mut(4) {
            // A fully transparent pixel has no colour to turn, and the icon is
            // mostly those — the disc is round and the bitmap is square.
            if pixel[3] == 0 {
                continue;
            }

            let (r, g, b) = turn_hue(pixel[0], pixel[1], pixel[2], DEV_HUE);
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        }

        Some(tauri::image::Image::new_owned(rgba, source.width(), source.height()))
    })
    .as_ref()
}

/// Set one sRGB colour's hue in Oklab, holding its lightness and chroma.
#[cfg(debug_assertions)]
fn turn_hue(r: u8, g: u8, b: u8, hue: f32) -> (u8, u8, u8) {
    let (l, a, bb) = oklab(to_linear(r), to_linear(g), to_linear(b));

    let chroma = (a * a + bb * bb).sqrt();
    let radians = hue.to_radians();

    let (red, green, blue) = from_oklab(l, chroma * radians.cos(), chroma * radians.sin());

    (to_srgb(red), to_srgb(green), to_srgb(blue))
}

#[cfg(debug_assertions)]
fn to_linear(channel: u8) -> f32 {
    let c = channel as f32 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(debug_assertions)]
fn to_srgb(channel: f32) -> u8 {
    let c = if channel <= 0.0031308 {
        12.92 * channel
    } else {
        1.055 * channel.powf(1.0 / 2.4) - 0.055
    };

    // Turning a hue can land outside the sRGB gamut. Clamped rather than
    // scaled: this is an icon, not a colour-managed image, and the alternative
    // is a whole gamut-mapping pass for a mark nobody will measure.
    (c.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// Linear sRGB to Oklab, by Björn Ottosson's matrices.
#[cfg(debug_assertions)]
fn oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();

    (
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    )
}

/// Oklab back to linear sRGB.
#[cfg(debug_assertions)]
fn from_oklab(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let long = (l + 0.3963377774 * a + 0.2158037573 * b).powi(3);
    let medium = (l - 0.1055613458 * a - 0.0638541728 * b).powi(3);
    let short = (l - 0.0894841775 * a - 1.2914855480 * b).powi(3);

    (
        4.0767416621 * long - 3.3077115913 * medium + 0.2309699292 * short,
        -1.2684380046 * long + 2.6097574011 * medium - 0.3413193965 * short,
        -0.0041960863 * long - 0.7034186147 * medium + 1.7076147010 * short,
    )
}

/// Put the dev mark and the dev name on one window.
///
/// The title is what Alt+Tab and the taskbar show, and it is the only name a
/// window of this application has: the chrome is custom and draws no title of
/// its own. So the icon answers "which one is this?" from the taskbar and the
/// title answers it everywhere a name is written instead of drawn.
#[cfg(debug_assertions)]
pub fn brand(window: &WebviewWindow) {
    if let Some(mark) = mark(window.app_handle()) {
        let _ = window.set_icon(mark.clone());
    }

    // Read rather than assumed, because three window types carry three titles
    // and this runs once per window at creation — the only place a title is set
    // after the config, so there is nothing to append twice.
    if let Ok(title) = window.title() {
        let _ = window.set_title(&format!("{title}{SUFFIX}"));
    }
}

#[cfg(not(debug_assertions))]
pub fn brand(_window: &WebviewWindow) {}

/// Put them on every window that is already open.
///
/// For the window Tauri builds from the config before anything else runs, which
/// never passes through `windows::build`. `surface::refresh` exists for the
/// same reason and is called from the same place.
#[cfg(debug_assertions)]
pub fn brand_open_windows(app: &AppHandle) {
    for window in app.webview_windows().values() {
        brand(window);
    }
}

#[cfg(not(debug_assertions))]
pub fn brand_open_windows(_app: &AppHandle) {}

/// The mark the tray should wear, or `None` in a release build.
///
/// Returned rather than applied, because the tray is built once with a builder
/// and setting its icon afterwards would be a second write of the same fact.
#[cfg(debug_assertions)]
pub fn tray_mark(app: &AppHandle) -> Option<tauri::image::Image<'static>> {
    mark(app).cloned()
}

#[cfg(not(debug_assertions))]
pub fn tray_mark(_app: &AppHandle) -> Option<tauri::image::Image<'static>> {
    None
}

/// What the tray calls itself on hover.
#[cfg(debug_assertions)]
pub fn tooltip() -> String {
    format!("sticky.md{SUFFIX}")
}

#[cfg(not(debug_assertions))]
pub fn tooltip() -> String {
    "sticky.md".to_string()
}
