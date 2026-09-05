import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { syntaxHighlighting } from '@codemirror/language';
import { EditorState } from '@codemirror/state';
import { EditorView, drawSelection, keymap, placeholder } from '@codemirror/view';

import { formattingKeymap } from './commands';
import { markdownHighlight } from './highlight';
import { codeLanguages } from './languages';
import { livePreview } from './livePreview';
import { editorTheme } from './theme';

export { toggleBold, toggleInlineCode, toggleItalic, toggleStrikethrough } from './commands';

/**
 * Build the note editor.
 *
 * The document is the note. There is no separate model, no serialization step,
 * and no rendered representation to keep in sync — `view.state.doc` is markdown
 * source and always has been, which is what makes copy yield raw source for
 * free.
 */
export function createEditor(parent: HTMLElement, doc: string): EditorView {
  return new EditorView({
    parent,
    state: EditorState.create({
      doc,
      extensions: [
        history(),
        drawSelection(),
        EditorView.lineWrapping,
        placeholder('Write something.'),

        markdown({
          // markdownLanguage is the GFM-enabled base: tables, task lists and
          // strikethrough, which MASTER lists in scope.
          base: markdownLanguage,
          codeLanguages
        }),
        syntaxHighlighting(markdownHighlight),
        livePreview,
        editorTheme,

        // Formatting bindings come first so they win over any default sharing
        // a chord.
        keymap.of([...formattingKeymap, ...historyKeymap, ...defaultKeymap])
      ]
    })
  });
}
