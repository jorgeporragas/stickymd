//! Tray residency.
//!
//! sticky.md lives in the tray so that the global shortcut has something to
//! reach. Closing the last note window puts the application away rather than
//! ending it — that is what makes a note summonable a second later without a
//! cold start.
//!
//! The tray is also the only visible affordance for quitting, because the
//! windows have no menu bar and closing one does not exit.

use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

use tauri_plugin_autostart::ManagerExt;

use crate::settings;
use crate::shortcuts::ShortcutStatus;
use crate::windows;

const NEW_NOTE: &str = "new-note";
const OPEN_HUB: &str = "open-hub";
const AUTOSTART: &str = "autostart";
const SHORTCUT_PROBLEM: &str = "shortcut-problem";
const QUIT: &str = "quit";

fn new_note(app: &AppHandle) {
    if let Err(error) = windows::open(app, None) {
        eprintln!("sticky.md: the tray could not open a note: {error}");
    }
}

fn open_hub(app: &AppHandle) {
    if let Err(error) = windows::open_hub(app) {
        eprintln!("sticky.md: the tray could not open the hub: {error}");
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

pub fn install(app: &AppHandle) -> tauri::Result<()> {
    // Bound separately: a slice needs one type, and these are several.
    let new_note_item = MenuItem::with_id(app, NEW_NOTE, "New note", true, None::<&str>)?;
    let hub_item = MenuItem::with_id(app, OPEN_HUB, "All notes", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let second_separator = PredefinedMenuItem::separator(app)?;

    // Reflects the real state rather than a remembered one: the user may have
    // removed the entry outside the application.
    let launching = app.autolaunch().is_enabled().unwrap_or(false);
    let autostart_item =
        CheckMenuItem::with_id(app, AUTOSTART, "Launch at startup", true, launching, None::<&str>)?;

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
                &[&new_note_item, &hub_item, &note, &separator, &autostart_item, &second_separator, &quit_item],
            )?
        }
        None => Menu::with_items(
            app,
            &[&new_note_item, &hub_item, &separator, &autostart_item, &second_separator, &quit_item],
        )?,
    };

    let autostart_checkbox = autostart_item.clone();

    let icon = app.default_window_icon().cloned().ok_or_else(|| {
        tauri::Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "tauri.conf.json defines no application icon for the tray to use",
        ))
    })?;

    TrayIconBuilder::with_id("tray")
        .icon(icon)
        .tooltip("sticky.md")
        .menu(&menu)
        // The menu belongs on the right button. Left-clicking a tray icon to be
        // shown a menu is a Windows convention this app has no reason to
        // follow — one click opens the notes.
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            NEW_NOTE => new_note(app),
            OPEN_HUB => open_hub(app),
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
