import { HighlightStyle } from '@codemirror/language';
import { tags as t } from '@lezer/highlight';

/**
 * How markdown reads once its syntax is hidden.
 *
 * Content is Rams — quiet and typographic (docs/DESIGN.md principle 1), and
 * since ADR-025 that holds for code as well: nothing here carries a hue.
 * Token classes are told apart by weight and by how dark they are, which is
 * enough in a scratchpad, where a fenced block is something pasted in rather
 * than something being written.
 *
 * Every value is a token reference. A literal here would be a bug.
 */
export const markdownHighlight = HighlightStyle.define([
  { tag: t.heading1, fontSize: '1.5em', fontWeight: 'var(--weight-heading)', lineHeight: 'var(--line-height-ui)' },
  { tag: t.heading2, fontSize: '1.3em', fontWeight: 'var(--weight-heading)', lineHeight: 'var(--line-height-ui)' },
  { tag: t.heading3, fontSize: '1.15em', fontWeight: 'var(--weight-heading)', lineHeight: 'var(--line-height-ui)' },
  { tag: [t.heading4, t.heading5, t.heading6], fontWeight: 'var(--weight-heading)' },

  { tag: t.strong, fontWeight: 'var(--weight-emphasis)' },
  { tag: t.emphasis, fontStyle: 'italic' },
  { tag: t.strikethrough, textDecoration: 'line-through' },

  { tag: t.link, color: 'var(--ink-primary)', textDecoration: 'underline' },
  { tag: t.url, color: 'var(--ink-muted)' },

  { tag: t.quote, color: 'var(--ink-secondary)', fontStyle: 'italic' },

  // Only `monospace`. `content` is a broad tag covering ordinary inline text,
  // and including it here renders the whole note in the code face.
  //
  // The size is set here rather than left to inherit. It had been inheriting
  // --font-size-body, and Martian Mono at that size sets a line half again as
  // wide as the prose around it. See --font-size-code.
  { tag: t.monospace, fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-code)' },

  // The syntax characters themselves, shown only on the cursor's line.
  { tag: t.processingInstruction, color: 'var(--ink-syntax)' },

  // Fenced code contents, on the dark panel `codeBlock.ts` paints.
  //
  // These are the only hues in the application that are not signals, and
  // ADR-030 is why: inside a fenced block, colour is telling token classes
  // apart, which is work rather than decoration. They come from the same
  // --tide-* ramp as the rest of the palette.
  { tag: t.keyword, color: 'var(--code-keyword)' },
  { tag: [t.string, t.special(t.string)], color: 'var(--code-string)' },
  { tag: [t.comment, t.lineComment, t.blockComment], color: 'var(--code-muted)', fontStyle: 'italic' },
  { tag: [t.number, t.bool, t.null], color: 'var(--code-number)' },
  { tag: [t.function(t.variableName), t.definition(t.variableName)], color: 'var(--code-ink)' },
  { tag: [t.typeName, t.className], color: 'var(--code-type)' },
  { tag: t.operator, color: 'var(--code-muted)' },
  { tag: t.propertyName, color: 'var(--code-ink)' }
]);
