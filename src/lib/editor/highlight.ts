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
  // --font-size-body, and a mono face at that size sets a line far wider than
  // the prose around it. See --font-size-code.
  { tag: t.monospace, fontFamily: 'var(--font-mono)', fontSize: 'var(--font-size-code)' },

  // The syntax characters themselves, shown only on the cursor's line.
  { tag: t.processingInstruction, color: 'var(--ink-syntax)' },

  // Fenced code contents, on the dark panel `codeBlock.ts` paints.
  //
  // These are the only hues in the application that are not signals, and
  // ADR-030 is why: inside a fenced block, colour is telling token classes
  // apart, which is work rather than decoration.
  //
  // They are *classes*, not colours, and the colours are attached in
  // `theme.ts` under `.cm-md-code` — because a highlight tag is global and
  // these tags are not exclusive to fenced code. `t.keyword` also matches a
  // task-list marker, which was arriving in prose wearing a code colour. A
  // rule that says "inside a fenced block" has to be written where that can
  // be said, and a highlight style cannot say it.
  { tag: t.keyword, class: 'tok-keyword' },
  { tag: [t.string, t.special(t.string)], class: 'tok-string' },
  // No italic. The code face ships one style, and `font-synthesis: none` means
  // an italic that is not in the file does not appear — asking for one would be
  // a rule that quietly does nothing. Colour carries comments on its own.
  { tag: [t.comment, t.lineComment, t.blockComment], class: 'tok-muted' },
  { tag: [t.number, t.bool, t.null], class: 'tok-number' },
  { tag: [t.function(t.variableName), t.definition(t.variableName)], class: 'tok-ink' },
  { tag: [t.typeName, t.className], class: 'tok-type' },
  { tag: t.operator, class: 'tok-muted' },
  { tag: t.propertyName, class: 'tok-ink' }
]);
