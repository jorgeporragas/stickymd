// sticky.md — application entry point.
//
// Rust owns file I/O, the sidecar index, tray residency, global shortcuts and
// window lifecycle. See CLAUDE.md section 'Backend'.

// Release builds must not open a console window behind the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("sticky.md failed to start");
}
