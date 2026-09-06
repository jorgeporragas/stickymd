import { invoke } from '@tauri-apps/api/core';

/**
 * The hub's view of the notes folder.
 *
 * Rust owns the reading, the naming and the deleting; this asks and reports.
 */

export interface NoteSummary {
  /** The filename, which is how every command refers to a note. */
  name: string;
  /** The note's first line. What a person calls it. */
  title: string;
  modifiedMs: number;
}

export async function listNotes(): Promise<NoteSummary[]> {
  try {
    return await invoke<NoteSummary[]>('list_notes');
  } catch (error) {
    console.error('sticky.md: could not list the notes', error);
    return [];
  }
}

export async function openNote(name: string): Promise<void> {
  try {
    await invoke('open_note_window', { name });
  } catch (error) {
    console.error('sticky.md: could not open the note', error);
  }
}

/**
 * Delete a note. It goes to the operating system's trash, so this is
 * recoverable and asks for no confirmation — a dialog for something the OS
 * already made undoable is friction with no benefit.
 */
export async function deleteNote(name: string): Promise<boolean> {
  try {
    await invoke('delete_note', { name });
    return true;
  } catch (error) {
    console.error('sticky.md: could not delete the note', error);
    return false;
  }
}

/** How long ago, in words, without pulling in a date library. */
export function whenModified(modifiedMs: number): string {
  if (!modifiedMs) return '';

  const seconds = Math.round((modifiedMs - Date.now()) / 1000);
  const format = new Intl.RelativeTimeFormat(undefined, { numeric: 'auto' });

  const scales: [Intl.RelativeTimeFormatUnit, number][] = [
    ['second', 60],
    ['minute', 60],
    ['hour', 24],
    ['day', 7],
    ['week', 4.35],
    ['month', 12]
  ];

  let value = seconds;
  for (const [unit, step] of scales) {
    if (Math.abs(value) < step) return format.format(Math.round(value), unit);
    value /= step;
  }

  return format.format(Math.round(value), 'year');
}
