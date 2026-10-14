import type { Text } from '@codemirror/state';

/**
 * What counts as markup rather than content.
 *
 * One definition, used by both renderers: the line-scoped hiding in
 * `livePreview.ts`, and the cell rendering in `tableView.ts`. They must agree
 * about what a syntax character is, or a table cell will show marks that the
 * same text hides one line above it. The caret in `markStep.ts` reads the
 * inline marks from here for the same reason.
 */

/**
 * The marks that wrap inline text: `**`, `*`, `_`, `__`, `~~` and backticks.
 * `markStep.ts` makes each one a single step for the caret, so it needs the
 * inline marks apart from the line-level markup below.
 */
const INLINE_MARK_NODES = new Set(['EmphasisMark', 'StrikethroughMark', 'CodeMark']);

/** Lezer node names whose text is markup rather than content. */
export const MARKUP_NODES = new Set([
  ...INLINE_MARK_NODES,
  'HeaderMark',
  'LinkMark',
  'QuoteMark'
]);

/** The part of a Lezer node that `isInlineMark` reads; a `SyntaxNode` satisfies it. */
interface NodeLike {
  readonly name: string;
  readonly parent: NodeLike | null;
}

/**
 * Whether a node is an inline mark, as opposed to the fence of a code block.
 *
 * A fence is also a `CodeMark`, but it is a line of its own that the renderer
 * leaves visible, and it does not wrap text.
 */
export function isInlineMark(node: NodeLike): boolean {
  return INLINE_MARK_NODES.has(node.name) && node.parent?.name !== 'FencedCode';
}

/** Markup that owns the whitespace following it. Hiding `#` alone leaves an indent. */
const CONSUMES_TRAILING_SPACE = new Set(['HeaderMark', 'QuoteMark']);

/**
 * A link's destination is markup too — hiding only the brackets leaves
 * `the docshttps://example.com` on the line.
 *
 * It is hidden only inside a `Link`. A bare autolink is *also* a `URL` node,
 * and its destination is the only text it has: hide that and the line goes
 * blank. Never fold `URL` into MARKUP_NODES to simplify this — see
 * docs/FIXES.md, 'Link destinations hide only inside a Link node'.
 */
export function isHiddenUrl(name: string, inLink: boolean): boolean {
  return name === 'URL' && inLink;
}

/** Extend a markup range over the single space that follows it, where it owns one. */
export function markupEnd(name: string, to: number, doc: Text): number {
  if (!CONSUMES_TRAILING_SPACE.has(name)) return to;
  return doc.sliceString(to, to + 1) === ' ' ? to + 1 : to;
}
