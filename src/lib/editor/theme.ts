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
    caretColor: 'var(--accent)',
    tabSize: '2'
  },

  '.cm-line': {
    padding: '0'
  },

  '&.cm-focused .cm-cursor': {
    borderLeftColor: 'var(--accent)'
  },

  '&.cm-focused .cm-selectionBackground, ::selection': {
    backgroundColor: 'var(--accent-glow)'
  },

  '.cm-selectionBackground': {
    backgroundColor: 'var(--accent-glow)'
  },

  // Rendered tables. Wide tables scroll inside their own container so the note
  // never scrolls sideways.
  '.cm-md-table-wrapper': {
    overflowX: 'auto',
    margin: 'var(--space-2) 0'
  },

  '.cm-md-table': {
    borderCollapse: 'collapse',
    fontSize: 'var(--font-size-label)',
    lineHeight: 'var(--line-height-ui)'
  },

  '.cm-md-table th, .cm-md-table td': {
    border: '1px solid var(--rule)',
    padding: 'var(--space-1) var(--space-2)',
    textAlign: 'left',
    verticalAlign: 'top'
  },

  '.cm-md-table th': {
    fontWeight: '650',
    color: 'var(--ink-secondary)'
  },

  '.cm-md-table code': {
    fontFamily: 'var(--font-mono)',
    fontSize: 'var(--font-size-code)'
  },

  '.cm-placeholder': {
    color: 'var(--ink-muted)',
    fontStyle: 'normal'
  }
});
