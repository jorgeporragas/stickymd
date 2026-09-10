// sticky.md — application entry point.
//
// Rust owns file I/O, the sidecar index, tray residency, global shortcuts and
// window lifecycle. See CLAUDE.md section 'Backend'.

// Release builds must not open a console window behind the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod index;
mod notes;
mod preferences;
mod settings;
mod shortcuts;
mod surface;
mod tray;
mod windows;

use surface::SurfaceMode;
use tauri::Manager;

/// The application-wide theme. Windows read it once at startup and then
/// listen for changes, so switching it does not require reopening anything.
#[tauri::command]
fn app_theme(app: tauri::AppHandle) -> String {
    settings::load(&app).theme
}

/// The surface mode this window ended up in. The frontend reads it once and
/// sets it on the document root; the design tokens carry the difference from
/// there, so no component branches on it.
#[tauri::command]
fn surface_mode(current: tauri::State<'_, surface::Current>) -> SurfaceMode {
    current.get()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        // The one part of this application that touches the network, and it
        // carries no note content — ADR-014, and MASTER veto 2 is about
        // content rather than about sockets. It runs in Rust, so the web
        // view's Content Security Policy is untouched and stays as strict as
        // it was.
        .plugin(tauri_plugin_updater::Builder::new().build())
        // Restarting into the new version, and nothing else this plugin can do
        // is reachable — the capability grants `process:allow-restart` alone.
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            // Checked rather than used: the surface is applied to every open
            // window by `surface::refresh` below. This is here so a missing
            // window entry in the config fails at startup with a sentence
            // rather than as an empty screen.
            app.get_webview_window(windows::FIRST_WINDOW)
                .ok_or("the note window is missing from tauri.conf.json")?;

            app.manage(surface::Current::default());
            app.manage(index::IndexLock::default());
            app.manage(windows::OpenNotes::default());

            // The window Tauri built from the config is a note window like any
            // other, so it is tracked like any other.
            app.state::<windows::OpenNotes>()
                .register(windows::FIRST_WINDOW, None)?;

            // Applies the surface to the window Tauri already built, records
            // the mode, and starts watching for the setting changing under us.
            surface::refresh(app.handle());
            surface::watch(app.handle().clone());

            // A session that cannot be restored is not a reason to refuse to
            // start: the notes are still on disk, and an empty window is a
            // working application.
            if let Err(error) = windows::restore(app.handle()) {
                eprintln!("sticky.md: could not restore the last session: {error}");
            }

            // Registered, never enabled: MASTER veto 5 forbids launching at
            // startup without explicit consent, so only the tray toggle turns
            // it on.
            app.handle().plugin(tauri_plugin_autostart::init(
                tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                None,
            ))?;

            app.manage(shortcuts::ShortcutStatus::default());
            shortcuts::register(app.handle(), &app.state::<shortcuts::ShortcutStatus>());
            tray::install(app.handle())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            surface_mode,
            app_theme,
            notes::list_notes,
            notes::read_note,
            notes::save_note,
            notes::delete_note,
            index::note_state,
            index::set_note_always_on_top,
            index::set_note_tint,
            windows::window_note,
            windows::claim_note,
            windows::new_note_window,
            windows::open_note_window,
            windows::reveal_window,
            windows::show_hub,
            windows::show_settings,
            preferences::read_preferences,
            preferences::set_theme,
            preferences::set_launch_at_startup,
            preferences::set_new_note_shortcut,
            preferences::set_formatting_shortcut,
            preferences::inspect_notes_folder,
            preferences::set_notes_folder
        ])
        .on_window_event(|window, event| match event {
            // Closing means this note should not come back next time.
            tauri::WindowEvent::CloseRequested { .. } => {
                windows::remember(window.app_handle(), window.label(), false);
            }
            // Losing focus is the moment a position is worth writing. Writing
            // on every drag frame would put the disk to work for the whole
            // gesture.
            tauri::WindowEvent::Focused(false) => {
                windows::remember(window.app_handle(), window.label(), true);
            }
            tauri::WindowEvent::Destroyed => {
                windows::closed(window.app_handle(), window.label());
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("sticky.md failed to start")
        .run(|_app, event| {
            // Closing the last note window must not end the application: it is
            // tray-resident, and the global shortcut has to have something to
            // reach. An explicit exit carries a code and is honoured.
            if let tauri::RunEvent::ExitRequested { api, code: None, .. } = event {
                api.prevent_exit();
            }
        });
}
