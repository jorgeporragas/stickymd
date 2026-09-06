//! Global shortcuts.
//!
//! Registered with the operating system rather than the web view. That is not
//! an optimisation — it is the only thing that works. A chord bound inside the
//! web view has to survive two separate hazards, and the new-note chord hit
//! both of them: WebView2 keeps `Ctrl+N` for itself, and `Ctrl+Alt` is AltGr
//! on Latin American, Spanish and most European layouts. See docs/FIXES.md.
//!
//! An OS-level registration sees the chord before either can intervene.

use std::str::FromStr;
use std::sync::Mutex;

use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::settings::{self, DEFAULT_NEW_NOTE_SHORTCUT};
use crate::windows;

/// Whether the new-note shortcut is actually held, and what went wrong if not.
///
/// A shortcut another application already owns cannot be registered, and until
/// this was surfaced the only sign was a line in a console nobody has open —
/// leaving a user with no way to summon a note and no idea why.
#[derive(Default)]
pub struct ShortcutStatus(Mutex<Option<String>>);

impl ShortcutStatus {
    fn set(&self, problem: Option<String>) {
        if let Ok(mut held) = self.0.lock() {
            *held = problem;
        }
    }

    /// The problem to show the user, if there is one.
    pub fn problem(&self) -> Option<String> {
        self.0.lock().ok().and_then(|held| held.clone())
    }
}

/// Register the global shortcuts, reporting what happened rather than failing.
///
/// The application runs perfectly well without a shortcut; it is simply less
/// convenient. Refusing to start over one would be the wrong trade.
pub fn register(app: &AppHandle, status: &ShortcutStatus) {
    let configured = settings::load(app).new_note_shortcut;

    let (shortcut, parse_problem) = match Shortcut::from_str(&configured) {
        Ok(shortcut) => (shortcut, None),
        Err(_) => (
            Shortcut::from_str(DEFAULT_NEW_NOTE_SHORTCUT).expect("the default shortcut must parse"),
            Some(format!("\"{configured}\" is not a shortcut sticky.md understands")),
        ),
    };

    let handled = shortcut;

    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |app, pressed, event| {
            // Fire on press. Without this the handler runs twice, once on the
            // way down and once on the way up.
            if event.state() != ShortcutState::Pressed || pressed != &handled {
                return;
            }

            if let Err(error) = windows::open(app, None) {
                eprintln!("sticky.md: the global shortcut could not open a note: {error}");
            }
        })
        .build();

    if let Err(error) = app.plugin(plugin) {
        status.set(Some(format!("global shortcuts are unavailable: {error}")));
        return;
    }

    match app.global_shortcut().register(shortcut) {
        Ok(()) => status.set(parse_problem),
        Err(_) => status.set(Some(format!(
            "{configured} is already held by another application"
        ))),
    }
}
