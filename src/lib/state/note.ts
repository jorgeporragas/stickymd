import { invoke } from '@tauri-apps/api/core';

/**
 * The note this window is editing, and when it gets written.
 *
 * Writes are debounced. A note file is never written on a keystroke
 * (CLAUDE.md section 'Backend') — the disk, and any sync client watching the
 * folder, would be doing work on every character typed.
 *
 * The file's *name* comes from its first line, and Rust owns that: working out
 * a free name needs to see the folder. Each save reports back the name the
 * note now has, which is how the next save finds the same file.
 */

/** How long typing must stop before the note is written. */
const SAVE_DEBOUNCE_MS = 600;

/** Errors as the Rust side sends them. See src-tauri/src/notes.rs. */
export type NoteError =
  | { kind: 'unsafe_name'; name: string }
  | { kind: 'no_notes_folder' }
  | { kind: 'not_found'; name: string }
  | { kind: 'io'; message: string };

function asNoteError(value: unknown): NoteError | undefined {
  if (typeof value === 'object' && value !== null && 'kind' in value) {
    return value as NoteError;
  }
  return undefined;
}

/** Undefined until the note has been written once and has a file. */
let noteName: string | undefined;

let timer: ReturnType<typeof setTimeout> | undefined;
let pending: string | undefined;

/** What is currently on disk, so an unchanged buffer is never rewritten. */
let written: string | undefined;

export function noteFileName(): string | undefined {
  return noteName;
}

/**
 * Ask which note this window is editing and read it.
 *
 * A window with no note is a new one and starts empty. Rust tracks the mapping
 * by window label, so nothing has to be threaded through the URL.
 */
export async function loadNote(): Promise<string> {
  try {
    noteName = (await invoke<string | null>('window_note')) ?? undefined;
  } catch {
    // No backend — the window is being served in a plain browser for testing.
    return '';
  }

  if (noteName === undefined) return '';

  try {
    const body = await invoke<string>('read_note', { name: noteName });
    written = body;
    return body;
  } catch (error) {
    const noteError = asNoteError(error);

    if (noteError?.kind === 'not_found') {
      // The file went away between the window opening and this read — deleted
      // in Explorer, most likely. Treat it as a new note under that name.
      return '';
    }

    console.error('sticky.md: could not read the note', noteError ?? error);
    return '';
  }
}

/** The note's title: its first line, which Rust slugifies into the filename. */
function titleOf(body: string): string {
  const lineEnd = body.indexOf('\n');
  return lineEnd === -1 ? body : body.slice(0, lineEnd);
}

/** Note that the buffer changed. The write happens once typing stops. */
export function queueSave(body: string): void {
  pending = body;

  if (timer !== undefined) clearTimeout(timer);
  timer = setTimeout(() => void flushSave(), SAVE_DEBOUNCE_MS);
}

/**
 * Write now, if there is anything to write. Called on the debounce, and again
 * before the window closes so the last few characters are never lost.
 */
export async function flushSave(): Promise<void> {
  if (timer !== undefined) {
    clearTimeout(timer);
    timer = undefined;
  }

  const body = pending;
  pending = undefined;

  if (body === undefined || body === written) return;

  // A window opened and closed without typing leaves no file behind. Once a
  // note exists, emptying it is an edit like any other and is written.
  if (body.trim() === '' && noteName === undefined) return;

  try {
    const saved = await invoke<string>('save_note', {
      current: noteName ?? null,
      title: titleOf(body),
      body
    });

    if (saved !== noteName) {
      noteName = saved;
      // Tell Rust which note this window now holds, so a second window cannot
      // be opened onto the same file and overwrite it.
      await invoke('claim_note', { name: saved });
    }

    written = body;
  } catch (error) {
    // Keep the text queued so the next flush tries again rather than
    // discarding what the user typed.
    pending = body;
    console.error('sticky.md: could not save the note', asNoteError(error) ?? error);
  }
}
