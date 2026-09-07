import { syntaxTree } from '@codemirror/language';
import { RangeSetBuilder } from '@codemirror/state';
import {
  Decoration,
  ViewPlugin,
  WidgetType,
  type DecorationSet,
  type EditorView,
  type ViewUpdate
} from '@codemirror/view';

/**
 * Task list checkboxes.
 *
 * The box is a decoration over the real `[ ]` or `[x]`, and clicking it edits
 * those two characters in the document. It never stands in for them: markdown
 * stays in the buffer at all times (CLAUDE.md, 'Editor'), so copying a note
 * yields `- [x] done` exactly as typed, and a note opened in any other editor
 * is unremarkable.
 *
 * The marker is replaced only when the cursor is elsewhere, on the same rule
 * the rest of the inline rendering follows: put the cursor on a task line and
 * the brackets come back, so they can be edited as text like anything else.
 */

class CheckboxWidget extends WidgetType {
  constructor(
    private readonly checked: boolean,
    private readonly at: number
  ) {
    super();
  }

  eq(other: CheckboxWidget): boolean {
    return other.checked === this.checked && other.at === this.at;
  }

  toDOM(): HTMLElement {
    const box = document.createElement('span');
    box.className = 'cm-md-task';
    box.dataset.at = String(this.at);
    box.dataset.checked = String(this.checked);
    box.setAttribute('role', 'checkbox');
    box.setAttribute('aria-checked', String(this.checked));
    box.setAttribute('aria-label', this.checked ? 'Done' : 'Not done');

    if (this.checked) {
      box.innerHTML =
        '<svg viewBox="0 0 12 12" aria-hidden="true"><path d="M3 6.2 5 8.2 9 4"/></svg>';
    }

    return box;
  }

  /**
   * The widget handles its own click.
   *
   * Returning true tells CodeMirror the event is dealt with, which stops the
   * click also moving the cursor into the line — ticking a box should not
   * disturb where someone was typing.
   */
  ignoreEvent(): boolean {
    return false;
  }
}

/** Every line number touched by any selection range. */
function linesInSelection(view: EditorView): Set<number> {
  const lines = new Set<number>();
  const { doc } = view.state;

  for (const range of view.state.selection.ranges) {
    const first = doc.lineAt(range.from).number;
    const last = doc.lineAt(range.to).number;
    for (let line = first; line <= last; line++) lines.add(line);
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
        if (node.name !== 'TaskMarker') return;
        if (revealed.has(doc.lineAt(node.from).number)) return;

        const marker = view.state.sliceDoc(node.from, node.to);
        const checked = marker.toLowerCase().includes('x');

        builder.add(
          node.from,
          node.to,
          Decoration.replace({ widget: new CheckboxWidget(checked, node.from) })
        );
      }
    });
  }

  return builder.finish();
}

/** Flip the marker under a checkbox, in the document itself. */
function toggleAt(view: EditorView, at: number): void {
  const marker = view.state.sliceDoc(at, at + 3);
  if (!marker.startsWith('[') || !marker.endsWith(']')) return;

  view.dispatch({
    changes: { from: at + 1, to: at + 2, insert: marker[1] === ' ' ? 'x' : ' ' }
  });
}

export const taskList = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;

    constructor(view: EditorView) {
      this.decorations = buildDecorations(view);
    }

    update(update: ViewUpdate): void {
      if (update.docChanged || update.selectionSet || update.viewportChanged) {
        this.decorations = buildDecorations(update.view);
      }
    }
  },
  {
    decorations: (plugin) => plugin.decorations,

    eventHandlers: {
      mousedown(event, view) {
        const target = event.target as HTMLElement | null;
        const box = target?.closest?.('.cm-md-task') as HTMLElement | null;
        if (!box?.dataset.at) return false;

        event.preventDefault();
        toggleAt(view, Number(box.dataset.at));
        return true;
      }
    }
  }
);
