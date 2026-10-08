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
    /// The application-wide theme preference: `frost`, `dark`, or `system`.
    ///
    /// A *preference*, not a theme — `system` is resolved by each window, in
    /// `src/lib/state/theme.ts`, because the operating system's setting is
    /// something a window can ask for and be told about and this side cannot.
    /// Stored here as the string it arrives as and never interpreted.
    ///
    /// Notes carry a tint of their own; the theme decides the ink. See ADR-024.
    pub theme: String,
    /// The formatting chords, inside a note window.
    ///
    /// Separate from the new-note shortcut because they are a different kind
    /// of thing: that one is registered with the operating system and can be
    /// refused by it, these are the web view's own and always take.
    pub formatting: Formatting,
    /// Where notes live. Absent means the default, which is worked out from
    /// the user's Documents folder — stored as absent rather than resolved so
    /// that a user who has never chosen one keeps following the default if
    /// their Documents folder ever moves.
    pub notes_folder: Option<String>,
    /// What opens when the application starts and there is no session to
    /// bring back. Restored notes always come back first. This only decides
    /// what appears when nothing else would.
    pub on_launch: OnLaunch,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            new_note_shortcut: DEFAULT_NEW_NOTE_SHORTCUT.to_string(),
            theme: "frost".to_string(),
            formatting: Formatting::default(),
            notes_folder: None,
            on_launch: OnLaunch::default(),
        }
    }
}

/// What a launch with nothing to restore opens.
///
/// The hub by default. A blank note was the only answer until SMD-104, and
/// the founder's reasoning for changing it is that you launch the
/// application to find a note at least as often as to start one. The global
/// shortcut is still there for starting one.
///
/// Stored as `"hub"` or `"note"`. Read leniently: any other string is the
/// default. Without that, a typo in this one field of a hand-edited file
/// would fail the whole parse and quietly reset every other setting with it,
/// which is the opposite of what `load` promises.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OnLaunch {
    #[default]
    Hub,
    Note,
}

impl<'de> Deserialize<'de> for OnLaunch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "note" => OnLaunch::Note,
            _ => OnLaunch::Hub,
        })
    }
}

/// The four formatting chords.
///
/// Named fields rather than a map: the set is deliberately small and fixed
/// (`docs/DECISIONS.md` ADR-037), and a map would invite a settings file to
/// name a command that does not exist.
///
/// `CmdOrCtrl` in string form is the platform abstraction — never write Ctrl
/// or Cmd literally (CLAUDE.md, cross-platform).
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct Formatting {
    pub bold: String,
    pub italic: String,
    pub inline_code: String,
    pub strikethrough: String,
}

impl Default for Formatting {
    fn default() -> Self {
        Self {
            bold: "Mod-b".to_string(),
            italic: "Mod-i".to_string(),
            inline_code: "Mod-e".to_string(),
            strikethrough: "Mod-Shift-x".to_string(),
        }
    }
}

/// A debug build keeps its settings in a file of its own.
///
/// The config directory is `app_config_dir()`, which is the bundle identifier
/// — and the identifier is the same string whether the binary came from
/// `tauri dev` or from the release workflow. So without this the two builds
/// share one settings file, and pointing the development build at a scratch
/// notes folder points the installed one there too. That is the founder's
/// case exactly, and it is why the two files sit side by side rather than the
/// development build being told to use a different directory: the same folder
/// is where he would look for either.
///
/// `cfg!` rather than a configuration flag, for the reason `dev.rs` gives: a
/// release build cannot take the wrong branch, because the branch is not in
/// it. Nothing in the bundler or the release workflow needs to know.
fn settings_file() -> &'static str {
    if cfg!(debug_assertions) {
        "settings.dev.json"
    } else {
        "settings.json"
    }
}

fn settings_path(app: &AppHandle) -> Option<PathBuf> {
    let dir = app.path().app_config_dir().ok()?;
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join(settings_file()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_settings_file_from_before_the_launch_choice_opens_the_hub() {
        let settings: Settings = serde_json::from_str(r#"{"theme":"dark"}"#).unwrap();
        assert_eq!(settings.on_launch, OnLaunch::Hub);
        assert_eq!(settings.theme, "dark");
    }

    #[test]
    fn the_launch_choice_round_trips() {
        for choice in [OnLaunch::Hub, OnLaunch::Note] {
            let text = serde_json::to_string(&choice).unwrap();
            assert_eq!(serde_json::from_str::<OnLaunch>(&text).unwrap(), choice);
        }
        assert_eq!(serde_json::to_string(&OnLaunch::Note).unwrap(), r#""note""#);
    }

    #[test]
    fn a_mistyped_launch_choice_costs_only_itself() {
        let settings: Settings =
            serde_json::from_str(r#"{"theme":"dark","onLaunch":"nope"}"#).unwrap();
        assert_eq!(settings.on_launch, OnLaunch::Hub);
        assert_eq!(settings.theme, "dark");
    }
}
