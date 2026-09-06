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
    noteName = await invoke<string>('save_note', {
      current: noteName ?? null,
      title: titleOf(body),
      body
    });
    written = body;
  } catch (error) {
    // Keep the text queued so the next flush tries again rather than
    // discarding what the user typed.
    pending = body;
    console.error('sticky.md: could not save the note', asNoteError(error) ?? error);
  }
}
