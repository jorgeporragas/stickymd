import { EditorSelection, type StateCommand } from '@codemirror/state';

/**
 * The insert commands, for the things that are awkward to type by hand.
 *
 * Everything here writes real markdown into the document, exactly as the
 * formatting commands do: the file is the note, and the file is always valid
 * markdown. Nothing renders a table without there being a table in the text.
 *
 * Each one lands the cursor where a person would start typing — the first
 * header cell of a table, the language of a fence, the text half of a link.
 * A command that inserts a skeleton and leaves the cursor at the end has only
 * done half the job.
 */

/**
 * Insert `block` on a line of its own, landing the cursor `caret` characters
 * into it — and selecting `length` characters from there, where the skeleton
 * has placeholder words the user is meant to type over.
 *
 * A line is never split and a line of text is never overwritten. Which of the
 * three cases applies depends only on where the cursor sits:
 *
 *   - on a blank line, the block goes there;
 *   - at the start of a line with text, the block goes above it;
 *   - anywhere else in a line, the block goes below it.
 */
function insertBlock(block: string, caret: number, length = 0): StateCommand {
  return ({ state, dispatch }) => {
    const range = state.selection.main;
    const line = state.doc.lineAt(range.from);

    // Asked of the cursor, not of `at`: on a blank line the start and the end
    // of the line are the same position, and a test written against `at` reads
    // that as "mid-line" and opens a newline nobody asked for.
    const atStart = range.from === line.from;

    const at = atStart ? line.from : line.to;
    const before = atStart ? '' : '\n';
    const after = atStart && line.length > 0 ? '\n' : '';

    const from = at + before.length + caret;

    dispatch(
      state.update({
        changes: { from: at, to: at, insert: `${before}${block}${after}` },
        selection: length
          ? EditorSelection.range(from, from + length)
          : EditorSelection.cursor(from),
        scrollIntoView: true
      })
    );

    return true;
  };
}

/**
 * A two-column table with its header rule, the first heading selected.
 *
 * The rule is the part nobody remembers and the part that decides whether it
 * is a table at all — a header row without it is three lines of pipes.
 */
export const insertTable = insertBlock(
  ['| Column | Column |', '| --- | --- |', '|  |  |'].join('\n'),
  2,
  'Column'.length
);

/** A fenced block, cursor on the language, which is where typing starts. */
export const insertCodeBlock = insertBlock(['```', '', '```'].join('\n'), 3);

export const insertTaskList = insertBlock('- [ ] ', 6);

export const insertBulletList = insertBlock('- ', 2);

export const insertNumberedList = insertBlock('1. ', 3);

/**
 * A link, cursor in the text half.
 *
 * Inserted inline rather than as a block: a link belongs in a sentence, and
 * this is the one insert that is not a block of its own.
 */
export const insertLink: StateCommand = ({ state, dispatch }) => {
  const range = state.selection.main;
  const selected = state.sliceDoc(range.from, range.to);
  const insert = `[${selected}](url)`;

  dispatch(
    state.update({
      changes: { from: range.from, to: range.to, insert },
      // With text selected it becomes the link's text and the cursor goes to
      // the address; with nothing selected the cursor goes to the text.
      selection: selected
        ? EditorSelection.range(range.from + selected.length + 3, range.from + selected.length + 6)
        : EditorSelection.cursor(range.from + 1),
      scrollIntoView: true
    })
  );

  return true;
};
