import { syntaxTree } from '@codemirror/language';
import { RangeSetBuilder } from '@codemirror/state';
import {
  Decoration,
  ViewPlugin,
  type DecorationSet,
  type EditorView,
  type ViewUpdate
} from '@codemirror/view';

import { isHiddenUrl, MARKUP_NODES, markupEnd } from './markup';

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
 *
 * Block-level rendering cannot live here — CodeMirror forbids a ViewPlugin
 * from replacing line breaks. Tables are a StateField in `tableView.ts`.
 */

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

function buildDecorations(view: EditorView): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const revealed = linesInSelection(view);
  const { doc } = view.state;

  for (const { from, to } of view.visibleRanges) {
    syntaxTree(view.state).iterate({
      from,
      to,
      enter: (node) => {
        // A rendered table replaces its whole range; decorating inside it is
        // both pointless and a source of overlapping decorations.
        if (node.name === 'Table') return false;

        const markup =
          MARKUP_NODES.has(node.name) || isHiddenUrl(node.name, node.matchContext(['Link']));

        if (!markup) return;
        if (node.from === node.to) return;

        // Fence markers stay visible. Hiding them leaves a bare language name
        // floating above the block, which reads as a bug rather than a render.
        if (node.matchContext(['FencedCode'])) return;

        if (revealed.has(doc.lineAt(node.from).number)) return;

        builder.add(node.from, markupEnd(node.name, node.to, doc), hidden);
        return;
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
