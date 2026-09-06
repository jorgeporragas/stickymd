// sticky.md — application entry point.
//
// Rust owns file I/O, the sidecar index, tray residency, global shortcuts and
// window lifecycle. See CLAUDE.md section 'Backend'.

// Release builds must not open a console window behind the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod notes;
mod surface;

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
                .get_webview_window("note")
                .ok_or("the note window is missing from tauri.conf.json")?;

            app.manage(surface::apply(&window));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            surface_mode,
            notes::list_notes,
            notes::read_note,
            notes::save_note
        ])
        .run(tauri::generate_context!())
        .expect("sticky.md failed to start");
}
