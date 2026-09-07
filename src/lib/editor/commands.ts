import { EditorSelection, type ChangeSpec, type StateCommand } from '@codemirror/state';
import type { KeyBinding } from '@codemirror/view';

/**
 * Formatting commands.
 *
 * Every command inserts real markdown characters. A command that produced
 * styled text without the corresponding syntax would be a bug — the document
 * is the file, and the file must always be valid markdown.
 *
 * Key names use CodeMirror's `Mod-` prefix, which resolves to Ctrl on Windows
 * and Linux and Cmd on macOS. Never write a platform modifier literally
 * (CLAUDE.md section 'Cross-platform discipline').
 */

/**
 * Wrap each selection range in `mark`, or unwrap it if it is already wrapped.
 *
 * With an empty selection this inserts the pair and places the cursor between
 * them, so `Mod-b` then typing produces bold text.
 */
function toggleWrap(mark: string): StateCommand {
  const width = mark.length;

  return ({ state, dispatch }) => {
    const transaction = state.changeByRange((range) => {
      const wrapped =
        state.sliceDoc(range.from - width, range.from) === mark &&
        state.sliceDoc(range.to, range.to + width) === mark;

      if (wrapped) {
        const changes: ChangeSpec[] = [
          { from: range.from - width, to: range.from },
          { from: range.to, to: range.to + width }
        ];

        return {
          changes,
          range: EditorSelection.range(range.from - width, range.to - width)
        };
      }

      const changes: ChangeSpec[] = [
        { from: range.from, insert: mark },
        { from: range.to, insert: mark }
      ];

      return {
        changes,
        range: EditorSelection.range(range.from + width, range.to + width)
      };
    });

    dispatch(state.update(transaction, { scrollIntoView: true, userEvent: 'input.format' }));
    return true;
  };
}

export const toggleBold = toggleWrap('**');
export const toggleItalic = toggleWrap('*');
export const toggleInlineCode = toggleWrap('`');
export const toggleStrikethrough = toggleWrap('~~');

/** The chords, as the settings file holds them. */
export interface FormattingChords {
  bold: string;
  italic: string;
  inlineCode: string;
  strikethrough: string;
}

/**
 * The defaults.
 *
 * Deliberately small. A scratchpad that claims four common chords is far less
 * likely to collide with the tools a developer already has bound — and the
 * set stays fixed so that a settings file cannot name a command that does not
 * exist.
 */
export const DEFAULT_CHORDS: FormattingChords = {
  bold: 'Mod-b',
  italic: 'Mod-i',
  inlineCode: 'Mod-e',
  strikethrough: 'Mod-Shift-x'
};

/**
 * Bindings for a set of chords.
 *
 * `Mod-` is the platform abstraction for the modifier key and is what the
 * stored chords use — never Ctrl or Cmd literally (CLAUDE.md, cross-platform).
 * A chord CodeMirror cannot parse simply never fires, which is why the
 * settings window is where one gets typed rather than a text file alone.
 */
export function formattingKeymapFor(chords: FormattingChords): readonly KeyBinding[] {
  return [
    { key: chords.bold, run: toggleBold },
    { key: chords.italic, run: toggleItalic },
    { key: chords.inlineCode, run: toggleInlineCode },
    { key: chords.strikethrough, run: toggleStrikethrough }
  ];
}
