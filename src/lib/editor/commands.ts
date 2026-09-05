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

/**
 * The default formatting bindings.
 *
 * Deliberately small. A scratchpad that claims four common chords is far less
 * likely to collide with the tools a developer already has bound.
 */
export const formattingKeymap: readonly KeyBinding[] = [
  { key: 'Mod-b', run: toggleBold },
  { key: 'Mod-i', run: toggleItalic },
  { key: 'Mod-e', run: toggleInlineCode },
  { key: 'Mod-Shift-x', run: toggleStrikethrough }
];
