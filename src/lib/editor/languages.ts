import { css } from '@codemirror/lang-css';
import { html } from '@codemirror/lang-html';
import { javascript } from '@codemirror/lang-javascript';
import { LanguageDescription } from '@codemirror/language';

/**
 * Languages highlighted inside fenced code blocks.
 *
 * A curated set rather than the full CodeMirror catalogue: install size is a
 * stated product priority (MASTER.md section 'Stack').
 *
 * The set is split two ways, and the split is not arbitrary — see
 * docs/FIXES.md, 'HTML, CSS and JavaScript modes cannot be lazy-loaded'.
 *
 *   - HTML, CSS and JavaScript are imported statically, because
 *     @codemirror/lang-markdown already pulls them into the main chunk.
 *     Declaring them lazy would be a lie the bundler quietly ignores.
 *   - Everything else loads on demand and costs nothing until a fence asks
 *     for it.
 *
 * A fence whose language is not listed still renders as a code block. It is
 * simply not highlighted.
 */
export const codeLanguages: LanguageDescription[] = [
  LanguageDescription.of({
    name: 'javascript',
    alias: ['js', 'jsx', 'ts', 'tsx', 'typescript'],
    support: javascript({ typescript: true })
  }),
  LanguageDescription.of({
    name: 'html',
    alias: ['htm'],
    support: html()
  }),
  LanguageDescription.of({
    name: 'css',
    support: css()
  }),

  LanguageDescription.of({
    name: 'python',
    alias: ['py'],
    load: () => import('@codemirror/lang-python').then((m) => m.python())
  }),
  LanguageDescription.of({
    name: 'rust',
    alias: ['rs'],
    load: () => import('@codemirror/lang-rust').then((m) => m.rust())
  }),
  LanguageDescription.of({
    name: 'json',
    load: () => import('@codemirror/lang-json').then((m) => m.json())
  })
];
