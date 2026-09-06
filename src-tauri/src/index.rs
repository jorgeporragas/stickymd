//! The sidecar index.
//!
//! A note's `.md` file holds its content and nothing else (ADR-002). Window
//! geometry, theme, always-on-top and open state live here instead, in one
//! JSON file beside the notes, keyed by filename.
//!
//! The split is why dragging a window never rewrites a note, and why a folder
//! under sync does not churn on mouse movement.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use crate::notes::{notes_dir, NoteError};

/// Lives in the notes folder so that copying the folder carries the state with
/// it. `list_notes` ignores anything that is not a `.md` file, so it never
/// appears as a note.
pub const INDEX_FILE: &str = ".sticky-index.json";

/// Where a corrupt index is moved rather than deleted. Losing window positions
/// is a nuisance; overwriting a file we failed to understand is how you lose
/// something that was actually recoverable.
const SALVAGE_FILE: &str = ".sticky-index.json.unreadable";

/// Everything about a note that is not its text.
///
/// Every field is optional or has a default: a note with no entry at all is
/// valid and opens with defaults (ADR-002), so absence must never be an error.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct NoteState {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub theme: Option<String>,
    pub always_on_top: bool,
    pub open: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Index {
    notes: BTreeMap<String, NoteState>,
}

/// Serializes read-modify-write cycles across windows.
///
/// Every note window runs in the same process, so a mutex is enough: two
/// windows saving at once would otherwise each load the file, apply their own
/// change, and write back — and whichever finished last would erase the other.
#[derive(Default)]
pub struct IndexLock(pub Mutex<()>);

/// Take the index lock, turning a poisoned mutex into an ordinary error.
///
/// Extracted at the second use rather than the third — see CLAUDE.md, DRY.
pub fn guard(lock: &IndexLock) -> Result<std::sync::MutexGuard<'_, ()>, NoteError> {
    lock.0.lock().map_err(|_| NoteError::Io {
        message: "the note index lock was poisoned".to_string(),
    })
}

fn index_path(dir: &Path) -> PathBuf {
    dir.join(INDEX_FILE)
}

/// Read the index. A missing file is an empty index, not an error.
///
/// An unreadable one is moved aside and treated as empty, so a note's window
/// position is lost but the file itself is kept for inspection.
fn load(dir: &Path) -> Index {
    let path = index_path(dir);

    let Ok(text) = std::fs::read_to_string(&path) else {
        return Index::default();
    };

    match serde_json::from_str(&text) {
        Ok(index) => index,
        Err(error) => {
            eprintln!("sticky.md: the note index could not be read ({error}); moving it aside");
            let _ = std::fs::rename(&path, dir.join(SALVAGE_FILE));
            Index::default()
        }
    }
}

/// Write the index through a temporary file, so an interrupted write cannot
/// leave a half-written index behind.
fn store(dir: &Path, index: &Index) -> Result<(), NoteError> {
    let path = index_path(dir);
    let temporary = path.with_extension("json.writing");

    let text = serde_json::to_string_pretty(index)
        .map_err(|error| NoteError::Io { message: error.to_string() })?;

    std::fs::write(&temporary, text)?;
    std::fs::rename(&temporary, &path)?;
    Ok(())
}

/// Move a note's entry to follow a rename, keeping its window state.
///
/// Called from the save path: a note that is retitled keeps its position and
/// settings, because it is the same note.
pub fn rename_entry(dir: &Path, from: &str, to: &str) -> Result<(), NoteError> {
    if from == to {
        return Ok(());
    }

    let mut index = load(dir);

    if let Some(state) = index.notes.remove(from) {
        index.notes.insert(to.to_string(), state);
        store(dir, &index)?;
    }

    Ok(())
}

/// Forget a note's entry. Called when a note is deleted.
pub fn forget_entry(dir: &Path, name: &str) -> Result<(), NoteError> {
    let mut index = load(dir);

    if index.notes.remove(name).is_some() {
        store(dir, &index)?;
    }

    Ok(())
}

#[tauri::command]
pub fn note_state(
    app: AppHandle,
    lock: State<'_, IndexLock>,
    name: String,
) -> Result<NoteState, NoteError> {
    let dir = notes_dir(&app)?;
    let _guard = guard(&lock)?;

    Ok(load(&dir).notes.get(&name).cloned().unwrap_or_default())
}

#[tauri::command]
pub fn set_note_state(
    app: AppHandle,
    lock: State<'_, IndexLock>,
    name: String,
    state: NoteState,
) -> Result<(), NoteError> {
    let dir = notes_dir(&app)?;
    let _guard = guard(&lock)?;

    let mut index = load(&dir);
    index.notes.insert(name, state);
    store(&dir, &index)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{forget_entry, load, rename_entry, store, Index, NoteState, INDEX_FILE};

    fn scratch(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("stickymd-index-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch folder");
        dir
    }

    fn with_entry(dir: &PathBuf, name: &str, state: NoteState) {
        let mut index = load(dir);
        index.notes.insert(name.to_string(), state);
        store(dir, &index).expect("store");
    }

    #[test]
    fn a_missing_index_is_empty_rather_than_an_error() {
        let dir = scratch("missing");
        assert!(load(&dir).notes.is_empty());
    }

    #[test]
    fn a_note_with_no_entry_gets_defaults() {
        let dir = scratch("defaults");
        with_entry(&dir, "other.md", NoteState { always_on_top: true, ..Default::default() });

        let state = load(&dir).notes.get("absent.md").cloned().unwrap_or_default();
        assert_eq!(state, NoteState::default());
        assert!(!state.always_on_top);
    }

    #[test]
    fn state_survives_a_round_trip() {
        let dir = scratch("roundtrip");
        let state = NoteState {
            x: Some(-40),
            y: Some(120),
            width: Some(320),
            height: Some(320),
            theme: Some("frost".to_string()),
            always_on_top: true,
            open: true,
        };

        with_entry(&dir, "prompt.md", state.clone());
        assert_eq!(load(&dir).notes.get("prompt.md"), Some(&state));
    }

    #[test]
    fn an_unreadable_index_is_moved_aside_not_deleted() {
        let dir = scratch("corrupt");
        std::fs::write(dir.join(INDEX_FILE), "{ this is not json").expect("write");

        assert!(load(&dir).notes.is_empty(), "a corrupt index reads as empty");
        assert!(
            dir.join(".sticky-index.json.unreadable").exists(),
            "the unreadable file must be kept for inspection"
        );
    }

    #[test]
    fn renaming_a_note_carries_its_state() {
        let dir = scratch("rename");
        with_entry(&dir, "draft.md", NoteState { x: Some(7), always_on_top: true, ..Default::default() });

        rename_entry(&dir, "draft.md", "final.md").expect("rename");

        let index = load(&dir);
        assert!(index.notes.get("draft.md").is_none(), "the old key must not survive");
        assert_eq!(index.notes.get("final.md").map(|s| s.x), Some(Some(7)));
    }

    #[test]
    fn renaming_a_note_with_no_entry_is_harmless() {
        let dir = scratch("rename-absent");
        rename_entry(&dir, "draft.md", "final.md").expect("rename");
        assert!(load(&dir).notes.is_empty());
    }

    #[test]
    fn forgetting_removes_only_that_note() {
        let dir = scratch("forget");
        with_entry(&dir, "keep.md", NoteState { x: Some(1), ..Default::default() });
        with_entry(&dir, "drop.md", NoteState { x: Some(2), ..Default::default() });

        forget_entry(&dir, "drop.md").expect("forget");

        let index = load(&dir);
        assert!(index.notes.get("drop.md").is_none());
        assert_eq!(index.notes.get("keep.md").map(|s| s.x), Some(Some(1)));
    }

    #[test]
    fn writing_leaves_no_temporary_file_behind() {
        let dir = scratch("temp");
        with_entry(&dir, "note.md", NoteState::default());

        let leftovers: Vec<String> = std::fs::read_dir(&dir)
            .expect("read")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .filter(|name| name.contains("writing"))
            .collect();

        assert!(leftovers.is_empty(), "found {leftovers:?}");
    }

    #[test]
    fn index_default_is_empty() {
        assert!(Index::default().notes.is_empty());
    }
}
