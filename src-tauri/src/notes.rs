//! Note file I/O.
//!
//! A note is a `.md` file in a flat folder the user can open, grep and sync
//! (ADR-001). This module is the only place that reads or writes one — the
//! frontend never touches the filesystem (CLAUDE.md section 'Frontend').

use std::path::{Component, Path, PathBuf};

use serde::Serialize;
use tauri::{AppHandle, Manager};

/// Errors crossing the command boundary, as data rather than prose.
///
/// The frontend has to be able to tell "this note is gone" from "the disk is
/// full" without parsing an error message (CLAUDE.md section 'Backend').
#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NoteError {
    /// The name could escape the notes folder, or is not a note file.
    UnsafeName { name: String },
    /// The notes folder could not be located on this machine.
    NoNotesFolder,
    NotFound { name: String },
    Io { message: String },
}

impl From<std::io::Error> for NoteError {
    fn from(error: std::io::Error) -> Self {
        NoteError::Io { message: error.to_string() }
    }
}

/// A note file name that is safe to join onto the notes folder.
///
/// Names arrive from the frontend, so they are untrusted. Rejecting anything
/// that is not a single ordinary path component is what stops `../../` — or a
/// Windows stream name like `note.md:evil` — from being written outside the
/// folder. Never relax this into a substring check: `..` is a legitimate
/// substring of a title, and a blocklist of characters would miss a component
/// that resolves upward on some other platform.
fn safe_name(name: &str) -> Result<&str, NoteError> {
    let reject = || NoteError::UnsafeName { name: name.to_string() };

    if name.is_empty() || name.len() > 255 || !name.ends_with(".md") {
        return Err(reject());
    }

    let mut components = Path::new(name).components();
    match (components.next(), components.next()) {
        (Some(Component::Normal(single)), None) if single == name => Ok(name),
        _ => Err(reject()),
    }
}

/// The notes folder, created if it does not exist.
///
/// Under Documents rather than an application data directory: ADR-001 makes
/// the folder something the user opens, and a folder they cannot find is not
/// one they can grep or point another editor at.
pub fn notes_dir(app: &AppHandle) -> Result<PathBuf, NoteError> {
    let dir = app
        .path()
        .document_dir()
        .map_err(|_| NoteError::NoNotesFolder)?
        .join("sticky.md");

    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn note_path(app: &AppHandle, name: &str) -> Result<PathBuf, NoteError> {
    Ok(notes_dir(app)?.join(safe_name(name)?))
}

/// Every note file in the folder, newest first.
///
/// Anything that is not a `.md` file is ignored rather than reported: the user
/// owns this folder and may keep whatever they like in it.
#[tauri::command]
pub fn list_notes(app: AppHandle) -> Result<Vec<String>, NoteError> {
    let mut names: Vec<(std::time::SystemTime, String)> = Vec::new();

    for entry in std::fs::read_dir(notes_dir(&app)?)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if safe_name(&name).is_err() {
            continue;
        }

        let modified = entry.metadata().and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH);
        names.push((modified, name));
    }

    names.sort_by(|a, b| b.0.cmp(&a.0));
    Ok(names.into_iter().map(|(_, name)| name).collect())
}

#[tauri::command]
pub fn read_note(app: AppHandle, name: String) -> Result<String, NoteError> {
    let path = note_path(&app, &name)?;

    match std::fs::read_to_string(&path) {
        Ok(body) => Ok(body),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Err(NoteError::NotFound { name })
        }
        Err(error) => Err(error.into()),
    }
}

/// Write a note's body, replacing what was there.
///
/// Callers debounce. Never call this on every keystroke — CLAUDE.md section
/// 'Backend'.
#[tauri::command]
pub fn write_note(app: AppHandle, name: String, body: String) -> Result<(), NoteError> {
    let path = note_path(&app, &name)?;
    std::fs::write(path, body)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::safe_name;

    #[test]
    fn accepts_ordinary_note_names() {
        for name in ["untitled.md", "prompt-for-the-flow-refactor.md", "prompt-2.md"] {
            assert!(safe_name(name).is_ok(), "should accept {name}");
        }
    }

    #[test]
    fn rejects_traversal_and_separators() {
        for name in [
            "../escape.md",
            "..\\escape.md",
            "nested/note.md",
            "nested\\note.md",
            "/absolute.md",
            "C:\\absolute.md",
            "..",
            ".",
        ] {
            assert!(safe_name(name).is_err(), "should reject {name}");
        }
    }

    #[test]
    fn rejects_anything_that_is_not_a_markdown_file() {
        for name in ["", "note", "note.txt", "note.md.exe"] {
            assert!(safe_name(name).is_err(), "should reject {name}");
        }
    }
}
