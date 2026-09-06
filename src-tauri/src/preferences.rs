//! The settings surface's command layer.
//!
//! `settings.rs` owns the file; this owns what the settings *window* is
//! allowed to do to it. Kept apart because they answer to different things:
//! that one is a serialisation concern, this one is a user interface with a
//! folder move in it.
//!
//! Every command here is a whole operation rather than a field write. Setting
//! the notes folder moves files; setting the theme tells every open window.
//! A window that could set fields one at a time would have to know all of that
//! itself, and the frontend is the wrong place for any of it.

use std::path::{Path, PathBuf};

use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_autostart::ManagerExt;

use crate::notes::{notes_dir, NoteError};
use crate::settings;
use crate::shortcuts::{self, ShortcutStatus};

/// Everything the settings window shows, resolved.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub theme: String,
    pub new_note_shortcut: String,
    /// The problem with that shortcut, if it could not be registered.
    pub shortcut_problem: Option<String>,
    /// Where notes are, resolved — not the stored value, which may be absent.
    pub notes_folder: String,
    pub launch_at_startup: bool,
    /// Where the settings file itself lives, for the user who would rather
    /// edit it by hand. That was the only way to reach these until now.
    pub settings_file: String,
}

#[tauri::command]
pub fn read_preferences(app: AppHandle, status: tauri::State<'_, ShortcutStatus>) -> Preferences {
    let stored = settings::load(&app);

    Preferences {
        theme: stored.theme,
        new_note_shortcut: stored.new_note_shortcut,
        shortcut_problem: status.problem(),
        notes_folder: notes_dir(&app)
            .map(|dir| dir.display().to_string())
            .unwrap_or_else(|_| "unavailable".to_string()),
        launch_at_startup: app.autolaunch().is_enabled().unwrap_or(false),
        settings_file: settings::location(&app),
    }
}

/// Switch the application-wide theme.
///
/// Every open window is told rather than left stale — the same reason the tray
/// does it this way. See `tray.rs`.
#[tauri::command]
pub fn set_theme(app: AppHandle, theme: String) {
    let mut stored = settings::load(&app);
    stored.theme = theme.clone();
    settings::save(&app, &stored);
    let _ = app.emit("theme-changed", theme);
}

#[tauri::command]
pub fn set_launch_at_startup(app: AppHandle, enabled: bool) -> bool {
    // MASTER veto 5: never enabled without the user asking. This command *is*
    // the user asking, and it is the only path that turns it on besides the
    // tray's own toggle.
    let result = if enabled {
        app.autolaunch().enable()
    } else {
        app.autolaunch().disable()
    };

    if let Err(error) = result {
        eprintln!("sticky.md: could not change launch at startup: {error}");
    }

    app.autolaunch().is_enabled().unwrap_or(false)
}

/// Take a new chord for the new-note shortcut, and report what happened.
///
/// The old registration is dropped first. Registering the new one can fail —
/// another application may already hold it — and the window says so rather
/// than leaving the user with a shortcut that silently does nothing.
#[tauri::command]
pub fn set_new_note_shortcut(
    app: AppHandle,
    status: tauri::State<'_, ShortcutStatus>,
    chord: String,
) -> Option<String> {
    let mut stored = settings::load(&app);
    stored.new_note_shortcut = chord;
    settings::save(&app, &stored);

    shortcuts::reregister(&app, &status);
    status.problem()
}

/// What a folder change would involve, so the window can ask before doing it.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderChange {
    /// Notes in the folder being left behind.
    pub notes_to_move: usize,
    /// Whether the chosen folder already holds notes of its own.
    pub destination_has_notes: bool,
}

fn count_notes(dir: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };

    entries
        .flatten()
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
        })
        .count()
}

/// Look at a candidate folder without changing anything.
///
/// The founder's call was that the choice is offered at the time rather than
/// decided in advance, and a choice offered without saying how many notes it
/// concerns is not really a choice.
#[tauri::command]
pub fn inspect_notes_folder(app: AppHandle, path: String) -> FolderChange {
    let destination = PathBuf::from(&path);

    FolderChange {
        notes_to_move: notes_dir(&app).map(|dir| count_notes(&dir)).unwrap_or(0),
        destination_has_notes: count_notes(&destination) > 0,
    }
}

/// Point the application at a different notes folder.
///
/// With `move_notes`, every `.md` file and the sidecar index follow. Without
/// it the old folder is left exactly as it is and the new one is used from
/// here on — which is the option that has to be labelled carefully, because a
/// folder that looks empty afterwards is the same thing a user sees when their
/// notes have been lost.
#[tauri::command]
pub fn set_notes_folder(app: AppHandle, path: String, move_notes: bool) -> Result<String, NoteError> {
    let destination = PathBuf::from(&path);

    if !destination.is_dir() {
        return Err(NoteError::NoNotesFolder);
    }

    let previous = notes_dir(&app)?;

    if previous == destination {
        return Ok(destination.display().to_string());
    }

    if move_notes {
        move_notes_between(&previous, &destination)?;
    }

    let mut stored = settings::load(&app);
    stored.notes_folder = Some(destination.display().to_string());
    settings::save(&app, &stored);

    // Open windows read the folder fresh on their next save, so nothing has to
    // be told. The hub is the exception: it is showing a list from the old
    // folder until it looks again.
    let _ = app.emit("notes-folder-changed", destination.display().to_string());

    Ok(destination.display().to_string())
}

/// Move every note, and the index, from one folder to another.
///
/// A rename first, because within a volume it is instant and atomic. Across
/// volumes it fails, and only then is the file copied and the original
/// removed — never the other way round, so an interrupted move leaves the note
/// in the old folder rather than nowhere.
fn move_notes_between(from: &Path, to: &Path) -> Result<(), NoteError> {
    let entries = std::fs::read_dir(from)?;

    for entry in entries.flatten() {
        let source = entry.path();

        let is_note = source
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("md"));
        let is_index = source.file_name().is_some_and(|name| name == crate::index::INDEX_FILE);

        if !source.is_file() || !(is_note || is_index) {
            continue;
        }

        let Some(name) = source.file_name() else { continue };
        let mut target = to.join(name);

        // A name already taken in the destination is not a reason to stop, and
        // certainly not a reason to overwrite. The note arrives beside it.
        if target.exists() {
            let stem = source.file_stem().and_then(|s| s.to_str()).unwrap_or("note");
            let extension = source.extension().and_then(|s| s.to_str()).unwrap_or("md");

            for suffix in 2..1000 {
                let candidate = to.join(format!("{stem}-{suffix}.{extension}"));
                if !candidate.exists() {
                    target = candidate;
                    break;
                }
            }
        }

        if std::fs::rename(&source, &target).is_err() {
            std::fs::copy(&source, &target)?;
            std::fs::remove_file(&source)?;
        }
    }

    Ok(())
}
