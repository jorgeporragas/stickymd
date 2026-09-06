//! Application settings.
//!
//! Distinct from the sidecar index, which is per-note and lives beside the
//! notes (ADR-002). These are settings about the application itself, so they
//! live in the application's own config directory rather than in the user's
//! notes folder — a folder that is meant to hold notes and nothing else.
//!
//! The file is written with defaults the first time it is read, so there is
//! always something to edit.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

/// The shortcut that opens a new note from anywhere.
///
/// `CmdOrCtrl` is the platform abstraction in string form — it resolves to Cmd
/// on macOS and Ctrl elsewhere. Never write either literally (CLAUDE.md
/// section 'Cross-platform discipline').
///
/// Space rather than a letter because a global registration takes the chord
/// away from every other application on the machine, and `Ctrl+Alt` is avoided
/// entirely: it is AltGr on Latin American, Spanish and most European layouts.
/// See docs/FIXES.md.
pub const DEFAULT_NEW_NOTE_SHORTCUT: &str = "CmdOrCtrl+Shift+Space";

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub new_note_shortcut: String,
    /// The application-wide theme. Notes carry a tint of their own; the theme
    /// decides the ink. See ADR-024.
    pub theme: String,
    /// Where notes live. Absent means the default, which is worked out from
    /// the user's Documents folder — stored as absent rather than resolved so
    /// that a user who has never chosen one keeps following the default if
    /// their Documents folder ever moves.
    pub notes_folder: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            new_note_shortcut: DEFAULT_NEW_NOTE_SHORTCUT.to_string(),
            theme: "frost".to_string(),
            notes_folder: None,
        }
    }
}

fn settings_path(app: &AppHandle) -> Option<PathBuf> {
    let dir = app.path().app_config_dir().ok()?;
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join("settings.json"))
}

/// Read the settings, writing a default file if there is not one yet.
///
/// Never fails: a settings file that cannot be read or parsed falls back to
/// defaults rather than stopping the application. A typo in a hand-edited file
/// should cost the user their custom shortcut, not their notes.
pub fn load(app: &AppHandle) -> Settings {
    let Some(path) = settings_path(app) else {
        return Settings::default();
    };

    match std::fs::read_to_string(&path) {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(settings) => settings,
            Err(error) => {
                eprintln!("sticky.md: {} could not be read ({error}); using defaults", path.display());
                Settings::default()
            }
        },
        Err(_) => {
            // No file yet. Write one, so the user has something to edit.
            let settings = Settings::default();
            if let Ok(text) = serde_json::to_string_pretty(&settings) {
                let _ = std::fs::write(&path, text);
            }
            settings
        }
    }
}

/// Write the settings back.
///
/// Best effort: a setting that cannot be saved is a preference lost on the next
/// launch, not a reason to refuse the change the user just made.
pub fn save(app: &AppHandle, settings: &Settings) {
    let Some(path) = settings_path(app) else { return };

    match serde_json::to_string_pretty(settings) {
        Ok(text) => {
            if let Err(error) = std::fs::write(&path, text) {
                eprintln!("sticky.md: could not write {}: {error}", path.display());
            }
        }
        Err(error) => eprintln!("sticky.md: could not serialise the settings: {error}"),
    }
}

/// Where the settings file lives, for telling the user.
pub fn location(app: &AppHandle) -> String {
    settings_path(app)
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "the application config directory".to_string())
}
