/**
 * Keyboard chords: capturing them, writing them, reading them back.
 *
 * Two string formats are in play and neither is negotiable. CodeMirror takes
 * `Mod-Shift-x`; the operating system, through Tauri's global shortcut plugin,
 * takes `CmdOrCtrl+Shift+X`. They describe the same thing and agree on nothing
 * else, so a chord is held here as a value and written out in whichever dialect
 * the destination speaks.
 *
 * `Mod` and `CmdOrCtrl` are the platform abstraction CLAUDE.md requires:
 * neither format ever names Ctrl or Cmd, so a chord set on Windows is the right
 * chord on a Mac.
 */

export interface Chord {
  /** Ctrl on Windows and Linux, Cmd on macOS. */
  mod: boolean;
  shift: boolean;
  alt: boolean;
  /** A single lowercase letter, a digit, or a named key like `Space` or `F5`. */
  key: string;
}

export type ChordFormat = 'editor' | 'global';

/** Why a keypress cannot be a shortcut, or undefined if it can. */
export type ChordProblem = string;

/** Keys that are only ever a modifier — a chord cannot end on one. */
const MODIFIERS = new Set(['Control', 'Shift', 'Alt', 'Meta', 'AltGraph', 'CapsLock']);

/**
 * `event.key` values that both dialects already name identically, give or take
 * case. Anything outside this set and the printable characters is refused
 * rather than guessed at — a chord written from a guess is one that silently
 * never fires, which is the whole defect this file exists to remove.
 */
const NAMED = new Set([
  'Space',
  'Enter',
  'Tab',
  'Backspace',
  'Delete',
  'Home',
  'End',
  'PageUp',
  'PageDown',
  'ArrowUp',
  'ArrowDown',
  'ArrowLeft',
  'ArrowRight'
]);

function isFunctionKey(key: string): boolean {
  return /^F([1-9]|1[0-9]|2[0-4])$/.test(key);
}

/**
 * Read a chord out of a keypress, or say why it is not one.
 *
 * The refusals are the point. Each is a shortcut that would have been accepted
 * as text and then never fired.
 */
export function chordFrom(event: KeyboardEvent): Chord | ChordProblem {
  if (MODIFIERS.has(event.key)) return 'Hold a modifier and press a key.';

  const mod = event.ctrlKey || event.metaKey;
  const alt = event.altKey;

  // Ctrl+Alt is AltGr on Latin American, Spanish and most European layouts —
  // the chord is never delivered, and it works on a US layout, which is what
  // makes it dangerous. See docs/FIXES.md. Refused rather than warned about:
  // a warning you can click past is a shortcut that does not work.
  //
  // **`ctrlKey` specifically, not the platform modifier.** The two are the same
  // key on Windows and Linux and are not on macOS, where the modifier is Cmd
  // and Ctrl is a key of its own. Testing `mod` here refused Cmd+Option, which
  // is an ordinary macOS chord with nothing to do with AltGr — found while
  // scoping the macOS build (SMD-095), before there was a macOS build to find
  // it in.
  if (event.ctrlKey && alt) {
    return 'Ctrl and Alt together is AltGr on many keyboard layouts. Pick another.';
  }

  // A bare letter would swallow that letter in the editor.
  if (!mod && !alt) return 'A shortcut needs Ctrl or Alt.';

  let key = event.key;

  if (key === ' ') key = 'Space';
  else if (key.length === 1) key = key.toLowerCase();
  else if (!NAMED.has(key) && !isFunctionKey(key)) {
    return `${key} cannot be part of a shortcut.`;
  }

  // Shift changes what a printable key produces, so `event.key` for Shift+3 is
  // `#` on one layout and something else on another. The physical key is the
  // stable thing, and `event.code` is the only place it survives.
  if (event.shiftKey && key.length === 1) {
    const code = event.code;
    if (/^Key[A-Z]$/.test(code)) key = code.slice(3).toLowerCase();
    else if (/^Digit[0-9]$/.test(code)) key = code.slice(5);
  }

  return { mod, shift: event.shiftKey, alt, key };
}

/** Write a chord in the dialect its destination speaks. */
export function formatChord(chord: Chord, format: ChordFormat): string {
  const parts: string[] = [];

  if (format === 'editor') {
    if (chord.mod) parts.push('Mod');
    if (chord.shift) parts.push('Shift');
    if (chord.alt) parts.push('Alt');
    parts.push(chord.key);
    return parts.join('-');
  }

  if (chord.mod) parts.push('CmdOrCtrl');
  if (chord.shift) parts.push('Shift');
  if (chord.alt) parts.push('Alt');
  parts.push(chord.key.length === 1 ? chord.key.toUpperCase() : chord.key);
  return parts.join('+');
}

/**
 * Read a stored chord back. Returns undefined for anything this file would not
 * have written — which is how a hand-edited `settings.json` stops being able to
 * produce a shortcut that fails in silence.
 */
export function parseChord(value: string, format: ChordFormat): Chord | undefined {
  const parts = value.split(format === 'editor' ? '-' : '+').filter(Boolean);
  if (parts.length < 2) return undefined;

  const key = parts.pop() as string;
  const chord: Chord = { mod: false, shift: false, alt: false, key };

  for (const part of parts) {
    const name = part.toLowerCase();
    if (name === 'mod' || name === 'cmdorctrl') chord.mod = true;
    else if (name === 'shift') chord.shift = true;
    else if (name === 'alt') chord.alt = true;
    else return undefined;
  }

  if (!chord.mod && !chord.alt) return undefined;

  if (key.length === 1) chord.key = key.toLowerCase();
  else if (!NAMED.has(key) && !isFunctionKey(key)) return undefined;

  return chord;
}

/** Whether a stored string is a chord this application can actually bind. */
export function isBindable(value: string, format: ChordFormat): boolean {
  return parseChord(value, format) !== undefined;
}

/**
 * How a chord is shown to a person: `Ctrl + Shift + X`.
 *
 * The platform's own names, because `Mod-Shift-x` is a serialisation format and
 * nobody reads their keyboard in it. Unparseable input is handed back as it
 * came — a settings file edited by hand can hold anything, and showing what is
 * actually in there beats showing nothing.
 */
export function describeChord(value: string, format: ChordFormat): string {
  const chord = parseChord(value, format);
  if (!chord) return value;

  const apple = /mac|iphone|ipad/i.test(navigator.platform || navigator.userAgent);
  const parts: string[] = [];

  if (chord.mod) parts.push(apple ? '⌘' : 'Ctrl');
  if (chord.shift) parts.push(apple ? '⇧' : 'Shift');
  if (chord.alt) parts.push(apple ? '⌥' : 'Alt');

  parts.push(chord.key.length === 1 ? chord.key.toUpperCase() : chord.key);

  return parts.join(' + ');
}
