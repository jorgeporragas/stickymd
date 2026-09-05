import { syntaxTree } from '@codemirror/language';
import { RangeSetBuilder, StateField, type EditorState, type Extension } from '@codemirror/state';
import { Decoration, EditorView, WidgetType, type DecorationSet } from '@codemirror/view';
import type { SyntaxNode } from '@lezer/common';

import { isHiddenUrl, MARKUP_NODES } from './markup';

/**
 * Table rendering.
 *
 * A table displays as a table when the cursor is outside it, and as raw pipes
 * when the cursor is inside — the same reveal rule everything else follows,
 * applied to a block instead of a line.
 *
 * This is a StateField rather than a ViewPlugin because CodeMirror forbids a
 * plugin from producing decorations that replace line breaks. A block widget
 * must come from the state. The cost is that it scans the whole document
 * rather than the viewport; notes are small, and a table is cheap to find.
 */

type Alignment = 'left' | 'center' | 'right' | null;

interface Cell {
  from: number;
  to: number;
}

interface Row {
  header: boolean;
  cells: Cell[];
}

/** Inline elements that survive into a rendered cell. */
const ELEMENT_FOR: Record<string, string> = {
  StrongEmphasis: 'strong',
  Emphasis: 'em',
  Strikethrough: 'del',
  InlineCode: 'code'
};

/**
 * Render a span of markdown into DOM, dropping its syntax characters.
 *
 * Uses the same markup definition as the line-scoped renderer, so a bold word
 * in a cell reads exactly as it would one line above the table.
 */
function renderInline(state: EditorState, parent: SyntaxNode, from: number, to: number, out: Node): void {
  let pos = from;

  for (let child = parent.firstChild; child; child = child.nextSibling) {
    if (child.to <= from || child.from >= to) continue;

    if (child.from > pos) {
      out.appendChild(document.createTextNode(state.sliceDoc(pos, child.from)));
    }

    const isMarkup =
      MARKUP_NODES.has(child.name) || isHiddenUrl(child.name, parent.name === 'Link');

    if (!isMarkup) {
      const tag = ELEMENT_FOR[child.name];

      if (tag) {
        const element = document.createElement(tag);
        renderInline(state, child, child.from, child.to, element);
        out.appendChild(element);
      } else if (child.firstChild) {
        renderInline(state, child, child.from, child.to, out);
      } else {
        out.appendChild(document.createTextNode(state.sliceDoc(child.from, child.to)));
      }
    }

    pos = Math.max(pos, child.to);
  }

  if (pos < to) {
    out.appendChild(document.createTextNode(state.sliceDoc(pos, to)));
  }
}

/** Column alignment, read from the delimiter row: `:---`, `:---:`, `---:`. */
function parseAlignment(spec: string): Alignment[] {
  return spec
    .trim()
    .replace(/^\|/, '')
    .replace(/\|$/, '')
    .split('|')
    .map((column) => {
      const text = column.trim();
      const left = text.startsWith(':');
      const right = text.endsWith(':');

      if (left && right) return 'center';
      if (right) return 'right';
      if (left) return 'left';
      return null;
    });
}

function collectRows(table: SyntaxNode): Row[] {
  const rows: Row[] = [];

  for (let child = table.firstChild; child; child = child.nextSibling) {
    if (child.name !== 'TableHeader' && child.name !== 'TableRow') continue;

    const cells: Cell[] = [];
    for (let cell = child.firstChild; cell; cell = cell.nextSibling) {
      if (cell.name === 'TableCell') {
        cells.push({ from: cell.from, to: cell.to });
      }
    }

    rows.push({ header: child.name === 'TableHeader', cells });
  }

  return rows;
}

class TableWidget extends WidgetType {
  constructor(
    private readonly rows: Row[],
    private readonly alignment: Alignment[],
    private readonly source: string
  ) {
    super();
  }

  override eq(other: TableWidget): boolean {
    return other.source === this.source;
  }

  override toDOM(view: EditorView): HTMLElement {
    const wrapper = document.createElement('div');
    wrapper.className = 'cm-md-table-wrapper';

    const table = document.createElement('table');
    table.className = 'cm-md-table';

    const tree = syntaxTree(view.state);

    for (const row of this.rows) {
      const tr = document.createElement('tr');

      row.cells.forEach((cell, column) => {
        const td = document.createElement(row.header ? 'th' : 'td');
        const align = this.alignment[column];
        if (align) td.style.textAlign = align;

        // resolveInner lands on the innermost node at that offset — inside a
        // cell beginning with `**bold**` that is the emphasis mark, not the
        // cell. Climb to the TableCell so the children walk starts in the
        // right place.
        let node: SyntaxNode | null = tree.resolveInner(cell.from, 1);
        while (node && node.name !== 'TableCell') node = node.parent;

        if (node) {
          renderInline(view.state, node, cell.from, cell.to, td);
        } else {
          td.textContent = view.state.sliceDoc(cell.from, cell.to);
        }

        // Clicking a cell puts the cursor in that cell's source, which reveals
        // the raw table. It is the only way back into an editor whose text has
        // been replaced by a widget.
        td.addEventListener('mousedown', (event) => {
          event.preventDefault();
          view.dispatch({ selection: { anchor: cell.from }, scrollIntoView: true });
          view.focus();
        });

        tr.appendChild(td);
      });

      table.appendChild(tr);
    }

    wrapper.appendChild(table);
    return wrapper;
  }
}

function selectionTouches(state: EditorState, from: number, to: number): boolean {
  return state.selection.ranges.some((range) => range.from <= to && range.to >= from);
}

function buildTables(state: EditorState): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();

  syntaxTree(state).iterate({
    enter: (node) => {
      if (node.name !== 'Table') return;

      // Cursor inside: show the source, so it can be edited.
      if (selectionTouches(state, node.from, node.to)) return false;

      const source = state.sliceDoc(node.from, node.to);
      const lines = source.split('\n');
      const rows = collectRows(node.node);

      // A table without a delimiter row is not a table.
      if (rows.length === 0 || lines.length < 2) return false;

      builder.add(
        node.from,
        node.to,
        Decoration.replace({
          widget: new TableWidget(rows, parseAlignment(lines[1] ?? ''), source),
          block: true
        })
      );

      return false;
    }
  });

  return builder.finish();
}

export const tableView: Extension = StateField.define<DecorationSet>({
  create: (state) => buildTables(state),

  update(decorations, transaction) {
    // Compare the selections rather than testing `transaction.selection`:
    // that property holds only what a transaction carried explicitly, while
    // the cursor can also move as a result of changes being mapped through.
    // The comparison covers both.
    const movedCursor = !transaction.state.selection.eq(transaction.startState.selection);

    if (transaction.docChanged || movedCursor) {
      return buildTables(transaction.state);
    }

    return decorations.map(transaction.changes);
  },

  provide: (field) => EditorView.decorations.from(field)
});
