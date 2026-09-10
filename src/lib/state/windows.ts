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

export async function showHub(): Promise<void> {
  try {
    await invoke('show_hub');
  } catch (error) {
    console.error('sticky.md: could not open the notes list', error);
  }
}

export async function showSettings(): Promise<void> {
  try {
    await invoke('show_settings');
  } catch (error) {
    console.error('sticky.md: could not open settings', error);
  }
}

/**
 * Show this window, once there is something in it to see.
 *
 * Every window is built hidden. A transparent window is created before its web
 * view has painted anything, and what shows in that gap is the compositor's
 * own backdrop — a grey pane that then fills in as the page arrives. The
 * founder described it exactly: "first kind of grayish and then the actual
 * window builds".
 *
 * His guess was that the transparency arrives late. It is the other way round:
 * the transparency is there from the first instant and there is simply nothing
 * painted over it yet. Delaying the blur would leave an *opaque* grey pane for
 * a few milliseconds instead, which is the same flash in a different colour.
 * The fix is to not show the window until the paint exists.
 *
 * Two frames, not one. A single `requestAnimationFrame` fires *before* the
 * paint it is scheduled alongside; the second one is called after the first
 * has been composited, which is the earliest moment the surface is really on
 * screen.
 */
export async function revealWindow(): Promise<void> {
  await new Promise<void>((resolve) => {
    requestAnimationFrame(() => requestAnimationFrame(() => resolve()));
  });

  try {
    // Rust shows it, rather than this side calling `show` itself: the surface
    // is re-applied at the same moment, and that has to happen where the
    // compositor is spoken to. See `reveal_window`.
    await invoke('reveal_window');
  } catch {
    // No backend — the window is being served in a plain browser for testing,
    // where there is nothing to show and nothing was ever hidden.
  }
}
