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

  '::selection': {
    backgroundColor: 'var(--selection)'
  },

  // Rendered tables. Wide tables scroll inside their own container so the note
  // never scrolls sideways.
  // The fenced-code panel. Painted per line by `codeBlock.ts`, because a block
  // widget would replace the source and it has to stay editable in place.
  '.cm-md-code': {
    backgroundColor: 'var(--code-surface)',
    color: 'var(--code-ink)',

    // Ink on paper is invisible on this panel, so the caret takes the colour
    // the application already uses to say "this one". Green on
    // `--code-surface` is 7:1, where ink is nothing at all.
    caretColor: 'var(--signal-engaged)',
    paddingLeft: 'var(--space-4)',
    paddingRight: 'var(--space-4)',

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

  // A task list's checkbox. A seated control like the ones on a window's
  // chrome, at the size of the text it sits in rather than a fixed pixel
  // value, so it stays proportionate if the note's type ever changes.
  '.cm-md-task': {
    display: 'inline-grid',
    placeItems: 'center',
    width: '1em',
    height: '1em',
    verticalAlign: '-0.15em',
    marginRight: '0.15em',
    borderRadius: 'var(--radius-chip)',
    border: '1px solid var(--rim-control)',
    background: 'var(--gloss-control)',
    boxShadow: 'var(--gloss-seat)',
    cursor: 'pointer'
  },

  '.cm-md-task[data-checked="true"]': {
    borderColor: 'var(--swatch-rim)',
    color: 'var(--signal-engaged)',
    background: 'var(--gloss-tinted)'
  },

  // Hidden rather than absent — see `taskList.ts`. `visibility` keeps the
  // element in flow, which is the whole point: it is what holds the box's
  // baseline still between ticked and not.
  '.cm-md-task[data-checked="false"] svg': { visibility: 'hidden' },

  '.cm-md-task svg': {
    width: '0.7em',
    height: '0.7em',
    fill: 'none',
    // The hue taken most of the way to black, as on every other lit control.
    stroke: 'color-mix(in oklab, var(--signal-engaged) 25%, var(--rim-shade))',
    strokeWidth: '2',
    strokeLinecap: 'round',
    strokeLinejoin: 'round'
  },

  // The code palette, scoped to the panel. Outside it these classes are still
  // applied — the tags are global — and deliberately paint nothing, so a task
  // marker in prose stays ink.
  '.cm-md-code .tok-keyword': { color: 'var(--code-keyword)' },
  '.cm-md-code .tok-string': { color: 'var(--code-string)' },
  '.cm-md-code .tok-number': { color: 'var(--code-number)' },
  '.cm-md-code .tok-type': { color: 'var(--code-type)' },
  '.cm-md-code .tok-muted': { color: 'var(--code-muted)' },
  '.cm-md-code .tok-ink': { color: 'var(--code-ink)' },

  // The fences are separated from the code as well as from the panel's edge.
  // With padding only against the edge, the gap above the opening fence came
  // from the panel while the gap below the closing one came from nothing —
  // symmetric in the stylesheet and lopsided on screen, because the eye
  // compares the closing fence to the code line above it rather than to the
  // panel's edge below.
  /*
    Every line but the first bleeds half a pixel upward, in the panel's own
    colour, to meet the line above it.

    The lines abut exactly — the gap between them measures 0 — but their tops
    land on fractional device pixels, so each boundary is antialiased and the
    window behind shows through a sliver of it. Same failure as the pale edges
    in docs/FIXES.md, at a smaller scale: anything the paint does not cover,
    something else does.

    Upward only, and not on the first line, so nothing bleeds past the panel's
    rounded corners.
  */
  '.cm-md-code:not(.cm-md-code-open)': {
    boxShadow: '0 -0.5px 0 0 var(--code-surface)'
  },

  '.cm-md-code-open': {
    paddingBottom: 'var(--space-2)',
    paddingTop: 'var(--space-3)',
    borderTopLeftRadius: 'var(--radius-control)',
    borderTopRightRadius: 'var(--radius-control)'
  },

  '.cm-md-code-close': {
    paddingTop: 'var(--space-2)',
    paddingBottom: 'var(--space-3)',
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
