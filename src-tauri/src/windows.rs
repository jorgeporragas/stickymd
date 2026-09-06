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

use crate::notes::NoteError;
use crate::surface;

/// The label of the window Tauri creates from `tauri.conf.json` at startup.
pub const FIRST_WINDOW: &str = "note";

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
    fn window_showing(&self, note: &str) -> Result<Option<String>, NoteError> {
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
fn build(app: &AppHandle, label: &str) -> Result<WebviewWindow, NoteError> {
    let mut config = app
        .config()
        .app
        .windows
        .iter()
        .find(|window| window.label == FIRST_WINDOW)
        .cloned()
        .ok_or_else(|| NoteError::Io {
            message: format!("tauri.conf.json has no window labelled '{FIRST_WINDOW}'"),
        })?;

    // The builder takes its label from the config, so the label is set here
    // rather than on the builder.
    config.label = label.to_string();

    WebviewWindowBuilder::from_config(app, &config)
        .map_err(|error| NoteError::Io { message: error.to_string() })?
        .build()
        .map_err(|error| NoteError::Io { message: error.to_string() })
}

/// Open a note window. An already-open note is focused rather than opened
/// twice — two windows editing one file would overwrite each other.
pub fn open(app: &AppHandle, note: Option<String>) -> Result<String, NoteError> {
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

    let label = next_label();
    let window = build(app, &label)?;

    let _ = surface::apply(&window);
    open_notes.register(&label, note)?;

    Ok(label)
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
    window: WebviewWindow,
    open_notes: tauri::State<'_, OpenNotes>,
    name: String,
) -> Result<(), NoteError> {
    open_notes.register(window.label(), Some(name))
}

/// Open a new, empty note window.
#[tauri::command]
pub fn new_note_window(app: AppHandle) -> Result<String, NoteError> {
    open(&app, None)
}

/// Stop tracking a window that has closed.
pub fn closed(app: &AppHandle, label: &str) {
    let _ = app.state::<OpenNotes>().forget(label);
}
