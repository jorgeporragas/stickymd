import { syntaxTree } from '@codemirror/language';
import { RangeSetBuilder } from '@codemirror/state';
import {
  Decoration,
  ViewPlugin,
  type DecorationSet,
  type EditorView,
  type ViewUpdate
} from '@codemirror/view';

/**
 * The panel behind a fenced code block.
 *
 * Line decorations, not a block widget: a widget would replace the lines and
 * the source would stop being editable in place, which breaks the rule that
 * markdown stays in the buffer at all times. Decorating each line leaves the
 * text exactly where it was and only paints behind it.
 *
 * That is also why this can be a `ViewPlugin` where `tableView.ts` cannot be.
 * The restriction in docs/FIXES.md is on decorations that replace line breaks;
 * a line decoration adds a class to a line that is already there.
 *
 * The first and last lines carry extra classes so the panel can round its
 * corners — CSS cannot ask "is this the first line of the block?".
 */

const line = {
  middle: Decoration.line({ class: 'cm-md-code' }),
  open: Decoration.line({ class: 'cm-md-code cm-md-code-open' }),
  close: Decoration.line({ class: 'cm-md-code cm-md-code-close' }),
  only: Decoration.line({ class: 'cm-md-code cm-md-code-open cm-md-code-close' })
};

function buildDecorations(view: EditorView): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  const { doc } = view.state;

  for (const { from, to } of view.visibleRanges) {
    syntaxTree(view.state).iterate({
      from,
      to,
      enter: (node) => {
        if (node.name !== 'FencedCode') return;

        const firstLine = doc.lineAt(node.from).number;
        const lastLine = doc.lineAt(node.to).number;

        for (let number = firstLine; number <= lastLine; number++) {
          const current = doc.line(number);

          // A block can start above the viewport and end below it. Only the
          // lines actually being drawn are decorated; the classes marking the
          // ends are still decided by the block's real bounds, so a panel
          // scrolled halfway off screen keeps its corners in the right place.
          if (current.to < from || current.from > to) continue;

          const isFirst = number === firstLine;
          const isLast = number === lastLine;

          builder.add(
            current.from,
            current.from,
            isFirst && isLast
              ? line.only
              : isFirst
                ? line.open
                : isLast
                  ? line.close
                  : line.middle
          );
        }
      }
    });
  }

  return builder.finish();
}

export const codeBlockSurface = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
      this.decorations = buildDecorations(view);
    }

    update(update: ViewUpdate): void {
      if (update.docChanged || update.viewportChanged) {
        this.decorations = buildDecorations(update.view);
      }
    }
  },
  {
    decorations: (plugin) => plugin.decorations
  }
);
