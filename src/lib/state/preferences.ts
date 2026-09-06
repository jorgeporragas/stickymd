import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

/**
 * The settings window's view of the application's own settings.
 *
 * Rust owns every one of these, and each command here is a whole operation
 * rather than a field write: changing the notes folder can move files, and
 * changing the theme has to tell every open window. A frontend that set fields
 * one at a time would have to know all of that.
 *
 * Until this window existed, the only way to reach any of it was to hand-edit
 * `settings.json` — which satisfies "remappable" for someone willing to open a
 * text editor and nobody else. See ADR-022.
 */

export interface Preferences {
  theme: string;
  newNoteShortcut: string;
  /** Why the shortcut is not held, if it is not. */
  shortcutProblem: string | null;
  notesFolder: string;
  launchAtStartup: boolean;
  settingsFile: string;
}

export interface FolderChange {
  notesToMove: number;
  destinationHasNotes: boolean;
}

const unavailable: Preferences = {
  theme: 'frost',
  newNoteShortcut: 'CmdOrCtrl+Shift+Space',
  shortcutProblem: null,
  notesFolder: 'unavailable',
  launchAtStartup: false,
  settingsFile: 'unavailable'
};

export async function readPreferences(): Promise<Preferences> {
  try {
    return await invoke<Preferences>('read_preferences');
  } catch {
    // No backend — the window is being served in a plain browser for testing.
    return unavailable;
  }
}

export async function setTheme(theme: string): Promise<void> {
  try {
    await invoke('set_theme', { theme });
  } catch (error) {
    console.error('sticky.md: could not change the theme', error);
  }
}

/** Returns the state that actually took, which may not be the one asked for. */
export async function setLaunchAtStartup(enabled: boolean): Promise<boolean> {
  try {
    return await invoke<boolean>('set_launch_at_startup', { enabled });
  } catch (error) {
    console.error('sticky.md: could not change launch at startup', error);
    return !enabled;
  }
}

/** Returns the problem with the new chord, or null if it is held. */
export async function setNewNoteShortcut(chord: string): Promise<string | null> {
  try {
    return await invoke<string | null>('set_new_note_shortcut', { chord });
  } catch (error) {
    console.error('sticky.md: could not change the shortcut', error);
    return 'sticky.md could not change the shortcut.';
  }
}

/**
 * Ask for a folder. Returns undefined if the user closed the picker.
 *
 * A native picker rather than a text field: a notes folder typed by hand is a
 * notes folder that can be wrong, and this one is where every note lives.
 */
export async function chooseFolder(current: string): Promise<string | undefined> {
  try {
    const picked = await open({ directory: true, multiple: false, defaultPath: current });
    return typeof picked === 'string' ? picked : undefined;
  } catch (error) {
    console.error('sticky.md: could not open the folder picker', error);
    return undefined;
  }
}

export async function inspectFolder(path: string): Promise<FolderChange> {
  try {
    return await invoke<FolderChange>('inspect_notes_folder', { path });
  } catch {
    return { notesToMove: 0, destinationHasNotes: false };
  }
}

export async function setNotesFolder(path: string, moveNotes: boolean): Promise<string | undefined> {
  try {
    return await invoke<string>('set_notes_folder', { path, moveNotes });
  } catch (error) {
    console.error('sticky.md: could not change the notes folder', error);
    return undefined;
  }
}
