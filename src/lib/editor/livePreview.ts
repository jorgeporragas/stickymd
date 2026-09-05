import { syntaxTree } from '@codemirror/language';
import { RangeSetBuilder, type Text } from '@codemirror/state';
import {
  Decoration,
  ViewPlugin,
  type DecorationSet,
  type EditorView,
  type ViewUpdate
} from '@codemirror/view';

/**
 * Inline rendering.
 *
 * Markdown syntax stays in the document at all times — this layer only decides
 * what is drawn. Syntax characters are hidden on every line the cursor is not
 * on, and revealed on the line it is. Nothing here ever edits the document, so
 * the buffer and the file always hold real markdown.
 *
 * The reveal unit is the *line*, not the node: put the cursor anywhere on a
 * line and that line's syntax comes back whole. A node-scoped rule reveals
 * marks one at a time as the cursor crosses them, which reads as flicker.
 */

/** Lezer node names whose text is markup rather than content. */
const MARKUP_NODES = new Set([
  'HeaderMark',
  'EmphasisMark',
  'StrikethroughMark',
  'CodeMark',
  'LinkMark',
  'QuoteMark'
]);

/** Markup that owns the whitespace following it. Hiding `#` alone leaves an indent. */
const CONSUMES_TRAILING_SPACE = new Set(['HeaderMark', 'QuoteMark']);

/**
 * A link's destination is markup too — hiding only the brackets leaves
 * `the docshttps://example.com` on the line.
 *
 * It is hidden only inside a `Link`. A bare autolink is *also* a `URL` node,
 * and its destination is the only text it has: hide that and the line goes
 * blank. Never add `URL` to MARKUP_NODES to simplify this — see docs/FIXES.md.
 */
function isHiddenUrl(name: string, inLink: boolean): boolean {
  return name === 'URL' && inLink;
}

const hidden = Decoration.replace({});

/** Every line number touched by any selection range. */
function linesInSelection(view: EditorView): Set<number> {
  const lines = new Set<number>();
  const { doc } = view.state;

  for (const range of view.state.selection.ranges) {
    const first = doc.lineAt(range.from).number;
    const last = doc.lineAt(range.to).number;
    for (let line = first; line <= last; line++) {
      lines.add(line);
    }
  }

  return lines;
}

/** Extend a markup range over the single space that follows it, where it owns one. */
function markupEnd(name: string, to: number, doc: Text): number {
  if (!CONSUMES_TRAILING_SPACE.has(name)) return to;
  return doc.sliceString(to, to + 1) === ' ' ? to + 1 : to;
}

function buildDecorations(view: EditorView): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const revealed = linesInSelection(view);
  const { doc } = view.state;

  for (const { from, to } of view.visibleRanges) {
    syntaxTree(view.state).iterate({
      from,
      to,
      enter: (node) => {
        const markup =
          MARKUP_NODES.has(node.name) || isHiddenUrl(node.name, node.matchContext(['Link']));

        if (!markup) return;
        if (node.from === node.to) return;

        // Fence markers stay visible. Hiding them leaves a bare language name
        // floating above the block, which reads as a bug rather than a render.
        if (node.matchContext(['FencedCode'])) return;

        if (revealed.has(doc.lineAt(node.from).number)) return;

        builder.add(node.from, markupEnd(node.name, node.to, doc), hidden);
      }
    });
  }

  return builder.finish();
}

export const livePreview = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
      this.decorations = buildDecorations(view);
    }

    update(update: ViewUpdate): void {
      // Selection changes move the reveal; document and viewport changes move
      // what there is to reveal.
      if (update.docChanged || update.selectionSet || update.viewportChanged) {
        this.decorations = buildDecorations(update.view);
      }
    }
  },
  {
    decorations: (plugin) => plugin.decorations
  }
);
