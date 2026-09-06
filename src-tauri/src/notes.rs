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

/// Names Windows refuses to give a file, whatever the extension.
const RESERVED_STEMS: [&str; 22] = [
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// The longest slug we will produce, in characters. Long enough to stay
/// readable in a folder listing, short enough to leave room for a suffix
/// inside the platform's path limits.
const MAX_SLUG_CHARS: usize = 60;

/// Turn a note's first line into a filename stem.
///
/// The first line is usually a heading, so the leading `#` characters are
/// dropped — otherwise every note in the folder would sort under `-`.
fn slugify(title: &str) -> String {
    let text = title.trim_start().trim_start_matches('#').trim();

    let mut slug = String::new();
    let mut last_was_dash = true; // suppresses a leading dash

    for character in text.chars() {
        if character.is_alphanumeric() {
            for lowered in character.to_lowercase() {
                slug.push(lowered);
            }
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }

        if slug.chars().count() >= MAX_SLUG_CHARS {
            break;
        }
    }

    let slug = slug.trim_matches('-').to_string();

    if slug.is_empty() {
        return "untitled".to_string();
    }

    if RESERVED_STEMS.contains(&slug.as_str()) {
        return format!("{slug}-note");
    }

    slug
}

/// Whether an existing file already carries this stem, with or without a
/// deduplication suffix. `prompt-2.md` still belongs to the title "Prompt", so
/// a note does not get renamed every time it is saved.
fn stem_matches(file_name: &str, slug: &str) -> bool {
    let Some(stem) = file_name.strip_suffix(".md") else {
        return false;
    };

    if stem == slug {
        return true;
    }

    stem.strip_prefix(slug)
        .and_then(|rest| rest.strip_prefix('-'))
        .is_some_and(|digits| !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()))
}

/// The first free name for this stem: `slug.md`, then `slug-2.md`, and so on.
fn free_name(dir: &Path, slug: &str) -> String {
    let first = format!("{slug}.md");
    if !dir.join(&first).exists() {
        return first;
    }

    // Bounded so a pathological folder cannot spin here forever.
    for suffix in 2..10_000 {
        let candidate = format!("{slug}-{suffix}.md");
        if !dir.join(&candidate).exists() {
            return candidate;
        }
    }

    format!("{slug}-{}.md", std::process::id())
}

/// Write a note, naming the file from its title.
///
/// Returns the name the note now has, which the caller must remember: it is
/// how the next save finds the same file. Renaming happens here rather than in
/// the frontend because deduplication needs to see the folder.
///
/// Callers debounce. Never call this on every keystroke — CLAUDE.md section
/// 'Backend'.
#[tauri::command]
pub fn save_note(
    app: AppHandle,
    current: Option<String>,
    title: String,
    body: String,
) -> Result<String, NoteError> {
    save_into(&notes_dir(&app)?, current.as_deref(), &title, &body)
}

/// The naming and writing itself, against a folder rather than an app handle,
/// so it can be tested. Renaming moves a user's file — it is the part of this
/// module most worth covering.
fn save_into(
    dir: &Path,
    current: Option<&str>,
    title: &str,
    body: &str,
) -> Result<String, NoteError> {
    let current = match current {
        Some(name) => Some(safe_name(name)?.to_string()),
        None => None,
    };

    let slug = slugify(title);

    // Keep the existing name when the title still slugifies to it. Renaming a
    // file on every save would churn the folder and any sync client watching.
    let name = match &current {
        Some(existing) if stem_matches(existing, &slug) => existing.clone(),
        _ => free_name(dir, &slug),
    };

    // Move first, then write: the note keeps one identity on disk rather than
    // briefly existing under two names.
    if let Some(existing) = &current {
        if existing != &name && dir.join(existing).exists() {
            std::fs::rename(dir.join(existing), dir.join(&name))?;
        }
    }

    std::fs::write(dir.join(&name), body)?;
    Ok(name)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{safe_name, save_into, slugify, stem_matches};

    /// A scratch folder of its own, so tests cannot see each other's files.
    fn scratch(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("stickymd-test-{label}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch folder");
        dir
    }

    fn names(dir: &PathBuf) -> Vec<String> {
        let mut found: Vec<String> = std::fs::read_dir(dir)
            .expect("read scratch")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        found.sort();
        found
    }

    #[test]
    fn names_a_new_note_from_its_first_line() {
        let dir = scratch("new");
        let name = save_into(&dir, None, "# Prompt draft", "# Prompt draft
body").unwrap();

        assert_eq!(name, "prompt-draft.md");
        assert_eq!(names(&dir), vec!["prompt-draft.md"]);
        assert_eq!(
            std::fs::read_to_string(dir.join("prompt-draft.md")).unwrap(),
            "# Prompt draft
body"
        );
    }

    #[test]
    fn deduplicates_two_notes_sharing_a_title() {
        let dir = scratch("dedup");
        let first = save_into(&dir, None, "# Prompt", "one").unwrap();
        let second = save_into(&dir, None, "# Prompt", "two").unwrap();
        let third = save_into(&dir, None, "# Prompt", "three").unwrap();

        assert_eq!((first.as_str(), second.as_str(), third.as_str()),
                   ("prompt.md", "prompt-2.md", "prompt-3.md"));
        assert_eq!(std::fs::read_to_string(dir.join("prompt.md")).unwrap(), "one");
        assert_eq!(std::fs::read_to_string(dir.join("prompt-3.md")).unwrap(), "three");
    }

    #[test]
    fn editing_a_note_does_not_rename_it() {
        let dir = scratch("stable");
        let name = save_into(&dir, None, "# Prompt", "one").unwrap();
        let again = save_into(&dir, Some(&name), "# Prompt", "one, edited").unwrap();

        assert_eq!(again, name);
        assert_eq!(names(&dir), vec!["prompt.md"]);
    }

    #[test]
    fn a_deduplicated_note_keeps_its_suffix_when_edited() {
        let dir = scratch("suffix");
        save_into(&dir, None, "# Prompt", "one").unwrap();
        let second = save_into(&dir, None, "# Prompt", "two").unwrap();
        let again = save_into(&dir, Some(&second), "# Prompt", "two, edited").unwrap();

        assert_eq!(again, "prompt-2.md");
        assert_eq!(names(&dir), vec!["prompt-2.md", "prompt.md"]);
    }

    #[test]
    fn retitling_renames_the_file_and_leaves_nothing_behind() {
        let dir = scratch("rename");
        let first = save_into(&dir, None, "# Draft", "body").unwrap();
        let renamed = save_into(&dir, Some(&first), "# Final wording", "body").unwrap();

        assert_eq!(renamed, "final-wording.md");
        assert_eq!(names(&dir), vec!["final-wording.md"], "the old file must not survive");
        assert_eq!(std::fs::read_to_string(dir.join("final-wording.md")).unwrap(), "body");
    }

    #[test]
    fn retitling_onto_a_taken_name_does_not_clobber_it() {
        let dir = scratch("collide");
        save_into(&dir, None, "# Taken", "someone else's note").unwrap();
        let mine = save_into(&dir, None, "# Mine", "mine").unwrap();

        let renamed = save_into(&dir, Some(&mine), "# Taken", "mine").unwrap();

        assert_eq!(renamed, "taken-2.md");
        assert_eq!(
            std::fs::read_to_string(dir.join("taken.md")).unwrap(),
            "someone else's note",
            "the existing note must be untouched"
        );
    }


    #[test]
    fn slugifies_a_heading_into_a_readable_stem() {
        assert_eq!(slugify("# Prompt for the FLOW refactor"), "prompt-for-the-flow-refactor");
        assert_eq!(slugify("Meeting notes: Q3 / budget"), "meeting-notes-q3-budget");
        assert_eq!(slugify("### Deeply nested heading"), "deeply-nested-heading");
        assert_eq!(slugify("  leading space"), "leading-space");
    }

    #[test]
    fn slugify_never_produces_an_unusable_name() {
        // Nothing to work with.
        assert_eq!(slugify(""), "untitled");
        assert_eq!(slugify("###"), "untitled");
        assert_eq!(slugify("!!! ???"), "untitled");

        // Windows refuses these whatever the extension.
        assert_eq!(slugify("con"), "con-note");
        assert_eq!(slugify("# NUL"), "nul-note");

        // No leading, trailing or doubled separators.
        let slug = slugify("--- a  //  b ---");
        assert_eq!(slug, "a-b");

        // Bounded length, and still no trailing separator after truncation.
        let long = slugify(&"word ".repeat(80));
        assert!(long.chars().count() <= 60, "slug was {} chars", long.chars().count());
        assert!(!long.ends_with('-'));
    }

    #[test]
    fn stem_matches_tolerates_the_deduplication_suffix() {
        assert!(stem_matches("prompt.md", "prompt"));
        assert!(stem_matches("prompt-2.md", "prompt"));
        assert!(stem_matches("prompt-317.md", "prompt"));

        // A different title that merely starts the same way is a different note.
        assert!(!stem_matches("prompt-draft.md", "prompt"));
        assert!(!stem_matches("prompts.md", "prompt"));
        assert!(!stem_matches("prompt-.md", "prompt"));
        assert!(!stem_matches("prompt.txt", "prompt"));
    }

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
