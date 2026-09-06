//! Global shortcuts.
//!
//! Registered with the operating system rather than the web view. That is not
//! an optimisation — it is the only thing that works. A chord bound inside the
//! web view has to survive two separate hazards, and the new-note chord hit
//! both of them: WebView2 keeps `Ctrl+N` for itself, and `Ctrl+Alt` is AltGr
//! on Latin American, Spanish and most European layouts. See docs/FIXES.md.
//!
//! An OS-level registration sees the chord before either can intervene.

use tauri::AppHandle;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::windows;

/// The primary modifier, per platform.
///
/// Never write `Ctrl` or `Cmd` literally — CLAUDE.md section 'Cross-platform
/// discipline'. This is that abstraction for global shortcuts, the way
/// CodeMirror's `Mod-` is for in-editor ones.
const PRIMARY: Modifiers = if cfg!(target_os = "macos") {
    Modifiers::SUPER
} else {
    Modifiers::CONTROL
};

/// Open a new note from anywhere.
///
/// Deliberately not a `Ctrl+Alt` combination and deliberately not a chord any
/// browser claims. Space rather than a letter because a global registration
/// takes the chord away from every other application on the machine, and a
/// common letter combination would be a rude thing to claim.
fn new_note_shortcut() -> Shortcut {
    Shortcut::new(Some(PRIMARY | Modifiers::SHIFT), Code::Space)
}

/// Register the global shortcuts.
///
/// A shortcut another application already holds cannot be registered, and that
/// is not a failure worth stopping for: the app runs fine without it, and the
/// remedy is for the user to choose a different one. It is reported, not
/// raised.
pub fn register(app: &AppHandle) {
    let new_note = new_note_shortcut();

    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |app, shortcut, event| {
            // Fire on press. Without this check the handler runs twice, once
            // on the way down and once on the way up.
            if event.state() != ShortcutState::Pressed || shortcut != &new_note {
                return;
            }

            if let Err(error) = windows::open(app, None) {
                eprintln!("sticky.md: the global shortcut could not open a note: {error}");
            }
        })
        .build();

    if let Err(error) = app.plugin(plugin) {
        eprintln!("sticky.md: global shortcuts are unavailable: {error}");
        return;
    }

    if let Err(error) = app.global_shortcut().register(new_note_shortcut()) {
        eprintln!(
            "sticky.md: could not claim the new-note shortcut, most likely because another \
             application already holds it: {error}"
        );
    }
}
