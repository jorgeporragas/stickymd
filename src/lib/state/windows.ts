import { invoke } from '@tauri-apps/api/core';

/**
 * Opening note windows.
 *
 * Rust owns window lifecycle (CLAUDE.md section 'Backend'), so this is a thin
 * request rather than any logic: which label a window gets, and whether an
 * already-open note is focused instead of opened twice, are decided there.
 */
export async function openNewNoteWindow(): Promise<void> {
  try {
    await invoke('new_note_window');
  } catch (error) {
    console.error('sticky.md: could not open a new note window', error);
  }
}
