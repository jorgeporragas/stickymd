import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { syntaxHighlighting } from '@codemirror/language';
import { EditorState } from '@codemirror/state';
import { EditorView, drawSelection, keymap, placeholder, type KeyBinding } from '@codemirror/view';

import { codeBlockSurface } from './codeBlock';
import { formattingKeymap } from './commands';
import { markdownHighlight } from './highlight';
import { codeLanguages } from './languages';
import { livePreview } from './livePreview';
import { tableView } from './tableView';
import { taskList } from './taskList';
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
export interface EditorOptions {
  parent: HTMLElement;
  /** Initial markdown source. */
  doc: string;
  /** Called with the full source whenever the document changes. */
  onDocChange?: (body: string) => void;
  /**
   * Window-level bindings the editor should serve.
   *
   * They live here rather than on a DOM listener because CodeMirror's `Mod-`
   * prefix is the platform abstraction for modifier keys, and CLAUDE.md
   * forbids hardcoding Ctrl or Cmd.
   */
  extraKeymap?: readonly KeyBinding[];
}

export function createEditor({
  parent,
  doc,
  onDocChange,
  extraKeymap = []
}: EditorOptions): EditorView {
  return new EditorView({
    parent,
    state: EditorState.create({
      doc,
      extensions: [
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            onDocChange?.(update.state.doc.toString());
          }
        }),

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
        // Tables first: a StateField's block widget takes precedence over the
        // plugin's inline decorations inside the same range.
        tableView,
        codeBlockSurface,
        // Before livePreview: both replace ranges, and the task marker is
        // inside what livePreview would otherwise hide as list syntax.
        taskList,
        livePreview,
        editorTheme,

        // Formatting bindings come first so they win over any default sharing
        // a chord.
        keymap.of([...extraKeymap, ...formattingKeymap, ...historyKeymap, ...defaultKeymap])
      ]
    })
  });
}
