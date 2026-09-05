import { HighlightStyle } from '@codemirror/language';
import { tags as t } from '@lezer/highlight';

/**
 * How markdown reads once its syntax is hidden.
 *
 * Content is Rams — quiet and typographic (docs/DESIGN.md principle 1). The
 * only colour used here is `--ink-syntax`, and only on the syntax characters
 * themselves, which are visible only on the cursor's line.
 *
 * Every value is a token reference. A literal here would be a bug.
 */
export const markdownHighlight = HighlightStyle.define([
  { tag: t.heading1, fontSize: '1.5em', fontWeight: '650', lineHeight: 'var(--line-height-ui)' },
  { tag: t.heading2, fontSize: '1.3em', fontWeight: '650', lineHeight: 'var(--line-height-ui)' },
  { tag: t.heading3, fontSize: '1.15em', fontWeight: '650', lineHeight: 'var(--line-height-ui)' },
  { tag: [t.heading4, t.heading5, t.heading6], fontWeight: '650' },

  { tag: t.strong, fontWeight: '700' },
  { tag: t.emphasis, fontStyle: 'italic' },
  { tag: t.strikethrough, textDecoration: 'line-through' },

  { tag: t.link, color: 'var(--accent)', textDecoration: 'underline' },
  { tag: t.url, color: 'var(--ink-muted)' },

  { tag: t.quote, color: 'var(--ink-secondary)', fontStyle: 'italic' },

  // Only `monospace`. `content` is a broad tag covering ordinary inline text,
  // and including it here renders the whole note in the code face.
  { tag: t.monospace, fontFamily: 'var(--font-mono)' },

  // The syntax characters themselves, shown only on the cursor's line.
  { tag: t.processingInstruction, color: 'var(--ink-syntax)' },

  // Fenced code contents.
  { tag: t.keyword, color: 'var(--accent-hover)' },
  { tag: [t.string, t.special(t.string)], color: 'var(--ink-secondary)' },
  { tag: [t.comment, t.lineComment, t.blockComment], color: 'var(--ink-muted)', fontStyle: 'italic' },
  { tag: [t.number, t.bool, t.null], color: 'var(--accent)' },
  { tag: [t.function(t.variableName), t.definition(t.variableName)], color: 'var(--ink-primary)' },
  { tag: [t.typeName, t.className], color: 'var(--accent-hover)' },
  { tag: t.operator, color: 'var(--ink-secondary)' },
  { tag: t.propertyName, color: 'var(--ink-primary)' }
]);
