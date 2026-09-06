import { EditorView } from '@codemirror/view';

/**
 * Editor chrome.
 *
 * Every value is a design token. The editor is content, so it is quiet: no
 * gutters, no active-line highlight, no border. See docs/DESIGN.md principle 1.
 */
export const editorTheme = EditorView.theme({
  '&': {
    height: '100%',
    fontFamily: 'var(--font-content)',
    fontSize: 'var(--font-size-body)',
    lineHeight: 'var(--line-height-prose)',
    color: 'var(--ink-primary)',
    backgroundColor: 'transparent'
  },

  '&.cm-focused': {
    // The window is the focus indicator, not the text area.
    outline: 'none'
  },

  '.cm-scroller': {
    fontFamily: 'inherit',
    lineHeight: 'inherit',
    padding: `0 var(--space-4) var(--space-4)`,
    overflowY: 'auto',
    // Body sets user-select: none so a drag is never fought by a selection.
    // Text in the editor is selectable, obviously.
    userSelect: 'text'
  },

  '.cm-content': {
    padding: '0',
    caretColor: 'var(--ink-primary)',
    tabSize: '2'
  },

  '.cm-line': {
    padding: '0'
  },

  '&.cm-focused .cm-cursor': {
    borderLeftColor: 'var(--rule)'
  },

  '&.cm-focused .cm-selectionBackground, ::selection': {
    backgroundColor: 'var(--selection)'
  },

  '.cm-selectionBackground': {
    backgroundColor: 'var(--selection)'
  },

  // Rendered tables. Wide tables scroll inside their own container so the note
  // never scrolls sideways.
  // The fenced-code panel. Painted per line by `codeBlock.ts`, because a block
  // widget would replace the source and it has to stay editable in place.
  '.cm-md-code': {
    backgroundColor: 'var(--code-surface)',
    color: 'var(--code-ink)',
    paddingLeft: 'var(--space-3)',
    paddingRight: 'var(--space-3)',

    // The face and size belong here rather than on a highlight tag. `monospace`
    // only tags *inline* code; the contents of a fenced block are tagged by the
    // language inside it, so the block never saw them. The panel is what says
    // "this is code" — so the panel is what sets how code is set.
    fontFamily: 'var(--font-mono)',
    fontSize: 'var(--font-size-code)',

    // Tighter than the prose around it. Prose line height on 13px code is a
    // ratio over two, which makes a short block look like a long one — part of
    // why code read as oversized in the first place.
    lineHeight: 'var(--line-height-code)'
  },

  '.cm-md-code-open': {
    paddingTop: 'var(--space-2)',
    borderTopLeftRadius: 'var(--radius-control)',
    borderTopRightRadius: 'var(--radius-control)'
  },

  '.cm-md-code-close': {
    paddingBottom: 'var(--space-2)',
    borderBottomLeftRadius: 'var(--radius-control)',
    borderBottomRightRadius: 'var(--radius-control)'
  },

  '.cm-md-table-wrapper': {
    overflowX: 'auto',
    margin: 'var(--space-2) 0'
  },

  '.cm-md-table': {
    borderCollapse: 'collapse',
    fontSize: 'var(--font-size-label)',
    lineHeight: 'var(--line-height-code)'
  },

  '.cm-md-table th, .cm-md-table td': {
    border: '1px solid var(--rule)',
    padding: 'var(--space-1) var(--space-2)',
    textAlign: 'left',
    verticalAlign: 'top'
  },

  '.cm-md-table th': {
    fontWeight: 'var(--weight-heading)',
    color: 'var(--ink-secondary)'
  },

  '.cm-md-table code': {
    fontFamily: 'var(--font-mono)',
    fontSize: 'var(--font-size-code)',

    // Tighter than the prose around it. Prose line height on 13px code is a
    // ratio over two, which makes a short block look like a long one — part of
    // why code read as oversized in the first place.
    lineHeight: 'var(--line-height-code)'
  },

  '.cm-placeholder': {
    color: 'var(--ink-muted)',
    fontStyle: 'normal'
  }
});
