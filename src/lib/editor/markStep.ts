import { syntaxTree } from '@codemirror/language';
import { EditorSelection, type EditorState, type Extension } from '@codemirror/state';
import { Direction, EditorView, keymap, type Command, type KeyBinding } from '@codemirror/view';

import { isInlineMark } from './markup';

/**
 * The caret steps over an inline mark as one unit.
 *
 * Syntax is revealed on the cursor's line, so `**` is two real characters
 * there and CodeMirror would step through them one at a time — landing the
 * caret between the asterisks of a mark that is meant to read as a single
 * boundary. Two behaviours make a mark one step:
 *
 * - ArrowLeft and ArrowRight, with or without Shift, cross a whole mark.
 * - Typing a mark character when the caret is in front of the same closing
 *   mark moves over it instead of inserting a second one, so `Mod-b`, a word
 *   and `**` end at `**word**|` rather than `**word****`.
 *
 * Neither ever edits the document. Both find marks in the syntax tree — the
 * source `livePreview.ts` hides them from — never by scanning for characters,
 * which cannot tell `2 * 3` from emphasis.
 *
 * Deliberately not `EditorView.atomicRanges`. That would give the arrow
 * behaviour for free, but CodeMirror also applies it to deletion, so Backspace
 * at `**word**|` would remove both asterisks and leave `**word`; and it applies
 * it to the mouse, so a click could no longer put the caret inside a mark.
 * Backspace and Delete stay one character at a time. See docs/FIXES.md.
 */

/**
 * Where the caret ends up after crossing the mark beside `pos`, or null when
 * no mark is there.
 *
 * `resolveInner` with a side picks the node that starts (forward) or ends
 * (backward) exactly at `pos`; the check below rejects a node that merely
 * contains it. Adjacent marks are separate nodes and so separate steps — in
 * `**a**_b_` the caret can rest between `**` and `_`.
 */
function markEdgeBeside(state: EditorState, pos: number, forward: boolean): number | null {
  const node = syntaxTree(state).resolveInner(pos, forward ? 1 : -1);
  if (!isInlineMark(node)) return null;
  if (forward ? node.from !== pos : node.to !== pos) return null;
  return forward ? node.to : node.from;
}

/**
 * The arrow command for one direction.
 *
 * It claims the key only when the caret has a mark beside it, or, extending,
 * when the selection's head does. Otherwise it returns false and the default
 * binding runs, so line ends, a selection that collapses, right-to-left text
 * and everything else keep CodeMirror's own behaviour.
 *
 * The editor allows one selection range, so `main` is the whole selection.
 */
function stepOverMark(rightward: boolean, extend: boolean): Command {
  return (view) => {
    const { state } = view;
    const { main } = state.selection;
    // A selection that is not extending collapses to its edge in the defaults,
    // so only a caret can step over a mark.
    if (!main.empty && !extend) return false;

    // ArrowRight is "forward" only in left-to-right text, as in the defaults.
    const ltr = view.textDirectionAt(main.head) === Direction.LTR;
    const edge = markEdgeBeside(state, main.head, rightward === ltr);
    if (edge === null) return false;

    view.dispatch({
      selection: extend ? EditorSelection.range(main.anchor, edge) : EditorSelection.cursor(edge),
      scrollIntoView: true,
      userEvent: 'select'
    });
    return true;
  };
}

/**
 * Not marked `preventDefault`: that would swallow the key even when the command
 * declines it, and the default binding that follows needs to see it.
 */
const markStepKeymap: readonly KeyBinding[] = [
  { key: 'ArrowLeft', run: stepOverMark(false, false), shift: stepOverMark(false, true) },
  { key: 'ArrowRight', run: stepOverMark(true, false), shift: stepOverMark(true, true) }
];

/**
 * Typing over a closing mark.
 *
 * It steps one character at a time, so typing `**` over a closing `**` takes
 * two keystrokes and the first leaves the caret between the asterisks, as
 * closing brackets do in a code editor.
 *
 * It fires only when the character about to be typed is the one in the
 * document at the caret *and* that character belongs to a closing mark. An
 * opening mark is left alone, so typing `*` in front of `**word**` still
 * inserts. A mark exists in the tree only once it is paired, which is what
 * keeps `2 * 3`, a lone `*` and a bare `***` typing exactly as before.
 */
function typeOverClosingMark(view: EditorView, from: number, to: number, text: string): boolean {
  const { state } = view;
  const { main } = state.selection;

  // A caret typing a single character. Anything else — a replaced selection, a
  // paste, an IME mid-composition — is not a keystroke aimed at a mark.
  if (view.composing) return false;
  if (from !== to || !main.empty || main.head !== from || text.length !== 1) return false;
  if (state.sliceDoc(from, from + 1) !== text) return false;

  const node = syntaxTree(state).resolveInner(from, 1);
  if (!isInlineMark(node)) return false;

  // Closing: the mark ends its element and is not also the element's start.
  const element = node.parent;
  if (element === null || node.to !== element.to || node.from === element.from) return false;

  view.dispatch({
    selection: { anchor: from + 1 },
    scrollIntoView: true,
    userEvent: 'select'
  });
  return true;
}

export const markStep: Extension = [
  keymap.of(markStepKeymap),
  EditorView.inputHandler.of(typeOverClosingMark)
];
