//! Note window lifecycle.
//!
//! One window per note. Which note a window is editing is tracked here rather
//! than in the URL: a window's label is stable and Tauri hands it to every
//! command, so a window can simply ask.
//!
//! A window with no note yet is a new, unsaved note. It gains a name the first
//! time it is written, and reports that name back.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder};

use crate::index::{self, IndexLock, NoteState};
use crate::notes::{notes_dir, NoteError};
use crate::surface;

/// The label of the window Tauri creates from `tauri.conf.json` at startup.
pub const FIRST_WINDOW: &str = "note";

/// The hub. Defined in `tauri.conf.json` with `create: false`, so its geometry
/// has one home without a window being made at startup — this is a
/// tray-resident application and the hub is opened on request.
pub const HUB: &str = "hub";
const SETTINGS: &str = "settings";

/// Which note each open window is editing. `None` means a new, unsaved note.
#[derive(Default)]
pub struct OpenNotes(Mutex<HashMap<String, Option<String>>>);

impl OpenNotes {
    fn lock(&self) -> Result<std::sync::MutexGuard<'_, HashMap<String, Option<String>>>, NoteError> {
        self.0.lock().map_err(|_| NoteError::Io {
            message: "the open-notes lock was poisoned".to_string(),
        })
    }

    pub fn register(&self, label: &str, note: Option<String>) -> Result<(), NoteError> {
        self.lock()?.insert(label.to_string(), note);
        Ok(())
    }

    fn forget(&self, label: &str) -> Result<(), NoteError> {
        self.lock()?.remove(label);
        Ok(())
    }

    /// The label of the window already editing this note, if there is one.
    pub fn window_showing(&self, note: &str) -> Result<Option<String>, NoteError> {
        Ok(self
            .lock()?
            .iter()
            .find(|(_, open)| open.as_deref() == Some(note))
            .map(|(label, _)| label.clone()))
    }
}

/// Labels have to be unique for the lifetime of the process. A counter is
/// enough: they are never persisted, and session restore rebuilds them.
static NEXT_LABEL: AtomicUsize = AtomicUsize::new(1);

fn next_label() -> String {
    format!("note-{}", NEXT_LABEL.fetch_add(1, Ordering::Relaxed))
}

/// Build a note window from the same configuration as the first one.
///
/// The window's geometry lives in `tauri.conf.json` (docs/DESIGN.md section
/// 'Note window'), so every window is built from that entry rather than from
/// values repeated here. Two sources for one size is how they drift.
fn build(app: &AppHandle, template: &str, label: &str) -> Result<WebviewWindow, NoteError> {
    let mut config = app
        .config()
        .app
        .windows
        .iter()
        .find(|window| window.label == template)
        .cloned()
        .ok_or_else(|| NoteError::Io {
            message: format!("tauri.conf.json has no window labelled '{template}'"),
        })?;

    // The builder takes its label from the config, so the label is set here
    // rather than on the builder.
    config.label = label.to_string();

    WebviewWindowBuilder::from_config(app, &config)
        .map_err(|error| NoteError::Io { message: error.to_string() })?
        .build()
        .map_err(|error| NoteError::Io { message: error.to_string() })
}

/// Put a restored window back where it was.
///
/// Failures are ignored on purpose: a saved position can be off-screen after a
/// monitor is unplugged, and a note that opens in the wrong place is far better
/// than one that refuses to open at all.
fn place(window: &WebviewWindow, state: &NoteState) {
    if let (Some(x), Some(y)) = (state.x, state.y) {
        let _ = window.set_position(tauri::LogicalPosition::new(f64::from(x), f64::from(y)));
    }

    if let (Some(width), Some(height)) = (state.width, state.height) {
        let _ = window.set_size(tauri::LogicalSize::new(f64::from(width), f64::from(height)));
    }
}

/// Open a note window. An already-open note is focused rather than opened
/// twice — two windows editing one file would overwrite each other.
pub fn open(app: &AppHandle, note: Option<String>) -> Result<String, NoteError> {
    eprintln!("PROBE open: entered, note={note:?}");
    let open_notes = app.state::<OpenNotes>();

    if let Some(name) = note.as_deref() {
        if let Some(label) = open_notes.window_showing(name)? {
            if let Some(window) = app.get_webview_window(&label) {
                let _ = window.set_focus();
                return Ok(label);
            }
            // The window is gone but its entry survived — clean up and open.
            open_notes.forget(&label)?;
        }
    }

    eprintln!("PROBE open: past window_showing");
    let label = next_label();
    eprintln!("PROBE open: building {label}");
    let window = build(app, FIRST_WINDOW, &label)?;
    eprintln!("PROBE open: built {label}");

    let _ = surface::apply(&window);
    eprintln!("PROBE open: surface applied");
    open_notes.register(&label, note)?;
    eprintln!("PROBE open: registered, done");

    Ok(label)
}

/// Open the hub, or focus it if it is already open.
///
/// There is only ever one: a second list of the same notes would be two things
/// to keep in step for no gain.
pub fn open_hub(app: &AppHandle) -> Result<(), NoteError> {
    if let Some(window) = app.get_webview_window(HUB) {
        let _ = window.show();
        let _ = window.set_focus();
        return Ok(());
    }

    let window = build(app, HUB, HUB)?;
    let _ = surface::apply(&window);
    Ok(())
}

#[tauri::command]
pub fn show_hub(app: AppHandle) -> Result<(), NoteError> {
    open_hub(&app)
}

/// Open the settings window, or focus the one already open.
///
/// Built from the config entry like every other window, so its geometry lives
/// in one place. It is not a note and is not tracked as one — nothing about it
/// is restored, because a settings window left open is not a session worth
/// bringing back.
pub fn open_settings(app: &AppHandle) -> Result<(), NoteError> {
    if let Some(window) = app.get_webview_window(SETTINGS) {
        let _ = window.show();
        let _ = window.set_focus();
        return Ok(());
    }

    let window = build(app, SETTINGS, SETTINGS)?;
    let _ = surface::apply(&window);
    Ok(())
}

#[tauri::command]
pub fn show_settings(app: AppHandle) -> Result<(), NoteError> {
    open_settings(&app)
}

/// Open a note by name, or focus the window already showing it.
#[tauri::command]
pub fn open_note_window(app: AppHandle, name: String) -> Result<String, NoteError> {
    open(&app, Some(name))
}

/// Which note the calling window is editing, if it has one yet.
#[tauri::command]
pub fn window_note(
    window: WebviewWindow,
    open_notes: tauri::State<'_, OpenNotes>,
) -> Result<Option<String>, NoteError> {
    Ok(open_notes.lock()?.get(window.label()).cloned().flatten())
}

/// Record the name a window's note took on its first save.
///
/// Without this a second window could be opened onto the same note, and the
/// two would overwrite each other.
#[tauri::command]
pub fn claim_note(
    app: AppHandle,
    window: WebviewWindow,
    open_notes: tauri::State<'_, OpenNotes>,
    lock: tauri::State<'_, IndexLock>,
    name: String,
) -> Result<(), NoteError> {
    open_notes.register(window.label(), Some(name.clone()))?;

    // The note is open from this moment, which is what a restored session
    // reads. Its position is recorded too, so a note written and left alone
    // still comes back where it was put.
    let dir = notes_dir(&app)?;
    let _guard = index::guard(&lock)?;
    index::set_placement(&dir, &name, position_of(&window), size_of(&window), true)
}

/// Geometry is stored in logical pixels, not physical ones.
///
/// A physical size does not divide evenly into CSS pixels at a fractional DPI
/// scale, so restoring one leaves the web view a sliver short of the window and
/// that strip is painted by nothing — visible as a pale line down the right
/// edge and along the bottom. Logical units are the same units the page thinks
/// in, so nothing is left over. They also carry correctly onto a display with a
/// different scale factor, which physical units did not.
fn position_of(window: &WebviewWindow) -> Option<(i32, i32)> {
    let scale = window.scale_factor().ok()?;
    let position = window.outer_position().ok()?.to_logical::<f64>(scale);
    Some((position.x.round() as i32, position.y.round() as i32))
}

/// Paired with `set_size`, which sets the *inner* size — so the inner size is
/// what gets measured. Measuring the outer size and restoring it as the inner
/// one would grow the window a little on every session.
fn size_of(window: &WebviewWindow) -> Option<(u32, u32)> {
    let scale = window.scale_factor().ok()?;
    let size = window.inner_size().ok()?.to_logical::<f64>(scale);
    Some((size.width.round() as u32, size.height.round() as u32))
}

/// Record where a window is, and whether its note should reopen next time.
///
/// Called when a window loses focus and again when it is closing — the two
/// moments a position is worth writing. Writing on every drag frame would put
/// the disk to work for the whole gesture.
pub fn remember(app: &AppHandle, label: &str, still_open: bool) {
    eprintln!("PROBE remember: entered for {label}");
    let Some(window) = app.get_webview_window(label) else {
        return;
    };

    let Ok(Some(name)) = app.state::<OpenNotes>().lock().map(|open| {
        open.get(label).cloned().flatten()
    }) else {
        // An unnamed note has no file, so there is nothing to remember it by.
        return;
    };

    eprintln!("PROBE remember: have name, asking for notes_dir");
    let Ok(dir) = notes_dir(app) else { return };
    eprintln!("PROBE remember: have dir, asking for the index lock");
    let lock = app.state::<IndexLock>();
    let Ok(_guard) = index::guard(&lock) else { return };
    eprintln!("PROBE remember: holding the index lock");

    if let Err(error) =
        index::set_placement(&dir, &name, position_of(&window), size_of(&window), still_open)
    {
        eprintln!("sticky.md: could not record where '{name}' was: {error}");
    }
}

/// Reopen the notes that were open when the application last stopped.
///
/// The window Tauri built from the config is reused for the first of them
/// rather than left empty beside them — otherwise every restored session would
/// come back with one more note than it had.
pub fn restore(app: &AppHandle) -> Result<(), NoteError> {
    let dir = notes_dir(app)?;
    let notes = index::restorable(&dir);

    let mut notes = notes.into_iter();

    if let Some((name, state)) = notes.next() {
        if let Some(window) = app.get_webview_window(FIRST_WINDOW) {
            app.state::<OpenNotes>().register(FIRST_WINDOW, Some(name))?;
            place(&window, &state);
        }
    }

    for (name, state) in notes {
        let label = open(app, Some(name))?;
        if let Some(window) = app.get_webview_window(&label) {
            place(&window, &state);
        }
    }

    Ok(())
}

/// Open a new, empty note window.
#[tauri::command]
pub fn new_note_window(app: AppHandle) -> Result<String, NoteError> {
    open(&app, None)
}

/// Close the window showing a note, if one is open.
///
/// Destroys rather than closes: a close request runs the frontend's
/// close handler, which flushes a pending save — and the window still holds
/// the text, so the note would be written straight back after being deleted.
/// Destroying skips that path entirely.
///
/// Unsaved edits in that window are lost, which is correct. The note was
/// deleted.
pub fn close_showing(app: &AppHandle, note: &str) {
    let Ok(Some(label)) = app.state::<OpenNotes>().window_showing(note) else {
        return;
    };

    if let Some(window) = app.get_webview_window(&label) {
        let _ = window.destroy();
    }
}

/// Stop tracking a window that has closed.
pub fn closed(app: &AppHandle, label: &str) {
    let _ = app.state::<OpenNotes>().forget(label);
}
