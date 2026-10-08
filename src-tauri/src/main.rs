// sticky.md — application entry point.
//
// Rust owns file I/O, the sidecar index, tray residency, global shortcuts and
// window lifecycle. See CLAUDE.md section 'Backend'.

// Release builds must not open a console window behind the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod desktop;
mod dev;
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
    let builder = tauri::Builder::default();

    // One copy of the application, and registered first so a second copy
    // exits before it has built a window, restored a session or put a second
    // icon in the tray. It used to do all three: launch sticky.md from a second
    // virtual desktop, whose taskbar does not show the first copy as running,
    // and both copies stayed resident.
    //
    // Launching it again means "show me sticky.md", so the copy already
    // running answers the way the tray answers a left click and the Dock
    // answers a click on macOS: it brings the notes back. Spawned rather than
    // run in place, because the plugin calls this from inside a window
    // procedure on the main thread, and building a window from there is the
    // wedge docs/FIXES.md describes.
    //
    // Release builds only. The guard keys on the bundle identifier, which a
    // debug build shares with the installed one — the same fact behind
    // SMD-100 — so in a debug build it would make the two builds one, and
    // starting `tauri dev` would summon the installed copy and quit.
    let builder = if cfg!(debug_assertions) {
        builder
    } else {
        builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let app = app.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(error) = windows::open_hub(&app) {
                    eprintln!("sticky.md: a second launch could not open the hub: {error}");
                }
            });
        }))
    };

    builder
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
            // Checked up front so a missing window entry in the config fails
            // at startup with a sentence rather than as an empty screen. Every
            // note window is built from this entry, and no window is made from
            // it until `launch` asks for one.
            if !app.config().app.windows.iter().any(|window| window.label == windows::NOTE) {
                return Err("the note window is missing from tauri.conf.json".into());
            }

            app.manage(surface::Current::default());
            app.manage(index::IndexLock::default());
            app.manage(windows::OpenNotes::default());

            // The restored notes, or the hub or a blank note if there are
            // none. A debug build's mark and title are put on each window as
            // it is built, so nothing here needs to brand anything.
            if let Err(error) = windows::launch(app.handle()) {
                eprintln!("sticky.md: could not open anything at launch: {error}");
            }

            // After the launch rather than before it, so the mode recorded is
            // measured on the windows that are actually open. Each window has
            // already had the surface applied as it was built. This applies it
            // again, records the mode the front ends will ask for, and starts
            // watching for the setting changing under us. No front end has
            // asked yet: their pages load once the event loop is running.
            surface::refresh(app.handle());
            surface::watch(app.handle().clone());

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
            preferences::set_on_launch,
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
        .run(|app, event| match event {
            // Closing the last note window must not end the application: it is
            // tray-resident, and the global shortcut has to have something to
            // reach. An explicit exit carries a code and is honoured.
            //
            // Cmd+Q is not caught here and does not need to be. macOS quits
            // through `terminate:`, which never asks the run loop — so the
            // guard cannot swallow it, and the one gesture every Mac user
            // reaches for still works.
            tauri::RunEvent::ExitRequested { api, code: None, .. } => {
                api.prevent_exit();
            }
            // The Dock icon, clicked with nothing on screen. macOS asks the
            // application what it would like to do about that, and the answer
            // is the one the tray already gives a left click: bring the notes
            // back. They are the same gesture on two platforms.
            //
            // Unanswered, the Dock icon does nothing at all, which reads as a
            // broken application rather than as a resident one.
            #[cfg(target_os = "macos")]
            tauri::RunEvent::Reopen { has_visible_windows: false, .. } => {
                if let Err(error) = windows::open_hub(app) {
                    eprintln!("sticky.md: the Dock could not open the hub: {error}");
                }
            }
            _ => {
                // Read on macOS alone, where there is a Dock to click.
                let _ = app;
            }
        });
}
