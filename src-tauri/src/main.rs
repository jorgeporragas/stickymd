// sticky.md — application entry point.
//
// Rust owns file I/O, the sidecar index, tray residency, global shortcuts and
// window lifecycle. See CLAUDE.md section 'Backend'.

// Release builds must not open a console window behind the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod index;
mod notes;
mod settings;
mod shortcuts;
mod surface;
mod tray;
mod windows;

use surface::SurfaceMode;
use tauri::Manager;

/// The surface mode this window ended up in. The frontend reads it once and
/// sets it on the document root; the design tokens carry the difference from
/// there, so no component branches on it.
#[tauri::command]
fn surface_mode(mode: tauri::State<'_, SurfaceMode>) -> SurfaceMode {
    *mode
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app
                .get_webview_window(windows::FIRST_WINDOW)
                .ok_or("the note window is missing from tauri.conf.json")?;

            app.manage(surface::apply(&window));
            app.manage(index::IndexLock::default());
            app.manage(windows::OpenNotes::default());

            // The window Tauri built from the config is a note window like any
            // other, so it is tracked like any other.
            app.state::<windows::OpenNotes>()
                .register(windows::FIRST_WINDOW, None)?;

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
            notes::list_notes,
            notes::read_note,
            notes::save_note,
            notes::delete_note,
            index::note_state,
            index::set_note_always_on_top,
            windows::window_note,
            windows::claim_note,
            windows::new_note_window,
            windows::open_note_window,
            windows::show_hub
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
