//! Tray residency.
//!
//! sticky.md lives in the tray so that the global shortcut has something to
//! reach. Closing the last note window puts the application away rather than
//! ending it — that is what makes a note summonable a second later without a
//! cold start.
//!
//! The tray is also the only visible affordance for quitting on Windows,
//! because the windows have no menu bar and closing one does not exit. macOS
//! has its own: Tauri installs a menu there whether or not one is displayed,
//! and Cmd+Q reaches it.

use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager};

use tauri_plugin_autostart::ManagerExt;

use crate::settings;
use crate::shortcuts::ShortcutStatus;
use crate::windows;

const NEW_NOTE: &str = "new-note";
const OPEN_HUB: &str = "open-hub";
const OPEN_SETTINGS: &str = "open-settings";
const DARK: &str = "dark";
const AUTOSTART: &str = "autostart";
const SHORTCUT_PROBLEM: &str = "shortcut-problem";
const QUIT: &str = "quit";

fn new_note(app: &AppHandle) {
    if let Err(error) = windows::open(app, None) {
        eprintln!("sticky.md: the tray could not open a note: {error}");
    }
}

/// Switch the application-wide theme.
///
/// Every open window is told rather than left stale: a theme that only applies
/// to windows opened afterwards is a setting that appears not to work.
///
/// The check mark means *explicitly dark*, not *currently dark*. A third
/// preference exists — `system` — and this side cannot resolve it: the
/// operating system's theme is something a window can be told, and Rust would
/// have to ask one to find out. So following the system leaves this unchecked,
/// and ticking it is how you stop following and choose dark outright. The
/// settings window is where the three-way lives, and it says which one is on.
fn set_dark(app: &AppHandle, item: &CheckMenuItem<tauri::Wry>) {
    let mut settings = settings::load(app);
    settings.theme = if settings.theme == "dark" { "frost".into() } else { "dark".into() };

    settings::save(app, &settings);
    let _ = app.emit("theme-changed", settings.theme.clone());
    let _ = item.set_checked(settings.theme == "dark");
}

fn open_hub(app: &AppHandle) {
    if let Err(error) = windows::open_hub(app) {
        eprintln!("sticky.md: the tray could not open the hub: {error}");
    }
}

fn open_settings(app: &AppHandle) {
    if let Err(error) = windows::open_settings(app) {
        eprintln!("sticky.md: the tray could not open settings: {error}");
    }
}

/// Turn launching at startup on or off.
///
/// The checkbox is set from what the system reports afterwards, not from what
/// was asked for: if the change failed, the menu must not claim it succeeded.
fn set_autostart(app: &AppHandle, item: &CheckMenuItem<tauri::Wry>) {
    let launcher = app.autolaunch();
    let wanted = !launcher.is_enabled().unwrap_or(false);

    let outcome = if wanted { launcher.enable() } else { launcher.disable() };

    if let Err(error) = outcome {
        eprintln!("sticky.md: could not change launch at startup: {error}");
    }

    let _ = item.set_checked(launcher.is_enabled().unwrap_or(false));
}

/// The letter S from the application mark, as a 5x8 lattice.
///
/// macOS menu-bar extras are *template* images: the system reads the alpha
/// channel alone and paints the result itself, so the icon follows the bar's
/// own light or dark and its own vibrancy. The full-colour disc that
/// `default_window_icon()` returns is not one — as a silhouette it is a filled
/// circle, and the S disappears into it. So the menu bar gets the letter by
/// itself, which is the part of the mark that identifies it anyway.
///
/// Transcribed from assets/icon/stickymd.svg, where the glyph is already drawn
/// as 30-unit rectangles on exactly this grid rather than as text. It is not a
/// second drawing of the mark; it is the same lattice at a size a menu bar can
/// render.
#[cfg(target_os = "macos")]
const MENU_BAR_GLYPH: [&str; 8] =
    [".###.", "#...#", "#....", ".###.", "....#", "....#", "#...#", ".###."];

/// Draw that lattice for the menu bar.
///
/// `tray-icon` shows every macOS tray image at 18 points tall and scales the
/// width to match, so the pixel height chosen here is what decides whether it
/// is crisp: 36 lands 1:1 on a Retina display and halves exactly on any other.
/// Within that, three pixels to a cell puts the glyph at 12 points — a cap
/// height that sits beside the menu bar's own 14-point text rather than
/// towering over it — and the padding is what keeps it off its neighbours.
#[cfg(target_os = "macos")]
fn menu_bar_mark() -> tauri::image::Image<'static> {
    const CELL: usize = 3;
    const PAD_X: usize = 4;
    const PAD_Y: usize = 6;
    const WIDTH: usize = 5 * CELL + PAD_X * 2;
    const HEIGHT: usize = 8 * CELL + PAD_Y * 2;

    // Transparent everywhere the glyph is not. What lies under the alpha is
    // never shown — macOS recolours a template image outright — so leaving the
    // colour channels at zero is a convention here rather than a choice.
    let mut rgba = vec![0u8; WIDTH * HEIGHT * 4];

    for (row, cells) in MENU_BAR_GLYPH.iter().enumerate() {
        for (column, cell) in cells.bytes().enumerate() {
            if cell != b'#' {
                continue;
            }

            for y in 0..CELL {
                for x in 0..CELL {
                    let at = ((PAD_Y + row * CELL + y) * WIDTH + PAD_X + column * CELL + x) * 4;
                    rgba[at + 3] = 255;
                }
            }
        }
    }

    tauri::image::Image::new_owned(rgba, WIDTH as u32, HEIGHT as u32)
}

pub fn install(app: &AppHandle) -> tauri::Result<()> {
    // Bound separately: a slice needs one type, and these are several.
    let new_note_item = MenuItem::with_id(app, NEW_NOTE, "New note", true, None::<&str>)?;
    let hub_item = MenuItem::with_id(app, OPEN_HUB, "All notes", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, OPEN_SETTINGS, "Settings", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let second_separator = PredefinedMenuItem::separator(app)?;

    // Reflects the real state rather than a remembered one: the user may have
    // removed the entry outside the application.
    let launching = app.autolaunch().is_enabled().unwrap_or(false);
    let autostart_item =
        CheckMenuItem::with_id(app, AUTOSTART, "Launch at startup", true, launching, None::<&str>)?;

    let dark_item = CheckMenuItem::with_id(
        app,
        DARK,
        "Dark",
        true,
        settings::load(app).theme == "dark",
        None::<&str>,
    )?;

    let quit_item = MenuItem::with_id(app, QUIT, "Quit sticky.md", true, None::<&str>)?;

    // A shortcut that could not be claimed is shown here rather than left in a
    // console nobody has open. Disabled: there is nothing to click, it is there
    // to be read, and it names the file to edit.
    let menu = match app.state::<ShortcutStatus>().problem() {
        Some(problem) => {
            let note = MenuItem::with_id(
                app,
                SHORTCUT_PROBLEM,
                format!("Shortcut unavailable — {problem}. Edit {}", settings::location(app)),
                false,
                None::<&str>,
            )?;
            Menu::with_items(
                app,
                &[&new_note_item, &hub_item, &settings_item, &note, &separator, &dark_item, &autostart_item, &second_separator, &quit_item],
            )?
        }
        None => Menu::with_items(
            app,
            &[&new_note_item, &hub_item, &settings_item, &separator, &dark_item, &autostart_item, &second_separator, &quit_item],
        )?,
    };

    let autostart_checkbox = autostart_item.clone();
    let dark_checkbox = dark_item.clone();

    // The menu bar gets the template; every other tray gets the mark itself.
    #[cfg(target_os = "macos")]
    let icon = menu_bar_mark();

    #[cfg(not(target_os = "macos"))]
    let icon = app.default_window_icon().cloned().ok_or_else(|| {
        tauri::Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "tauri.conf.json defines no application icon for the tray to use",
        ))
    })?;

    TrayIconBuilder::with_id("tray")
        .icon(icon)
        // Ignored everywhere but macOS, and stated as a fact about the icon
        // rather than as a platform branch: the image above *is* a template on
        // one platform and is not on the others.
        .icon_as_template(cfg!(target_os = "macos"))
        .tooltip("sticky.md")
        .menu(&menu)
        // The menu belongs on the right button. Left-clicking a tray icon to be
        // shown a menu is a Windows convention this app has no reason to
        // follow — one click opens the notes.
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            NEW_NOTE => new_note(app),
            OPEN_HUB => open_hub(app),
            OPEN_SETTINGS => open_settings(app),
            DARK => set_dark(app, &dark_checkbox),
            AUTOSTART => set_autostart(app, &autostart_checkbox),
            QUIT => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                // One click opens the hub. The founder confirmed this is what
                // left-click should do once a hub exists; before that it wrote
                // a new note, which the menu still offers.
                open_hub(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}
