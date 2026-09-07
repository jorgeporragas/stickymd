import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
import { markdown, markdownLanguage } from '@codemirror/lang-markdown';
import { syntaxHighlighting } from '@codemirror/language';
import { Compartment, EditorState } from '@codemirror/state';
import { EditorView, keymap, placeholder, type KeyBinding } from '@codemirror/view';

import { codeBlockSurface } from './codeBlock';
import { DEFAULT_CHORDS, formattingKeymapFor, type FormattingChords } from './commands';
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
  /** The formatting chords. Defaults when the settings are unavailable. */
  formatting?: FormattingChords;
}

/**
 * Holds the formatting bindings so they can be swapped without rebuilding the
 * editor. A chord that only applied to windows opened afterwards would be a
 * setting that appears not to work — the same reason the theme is broadcast.
 */
const formattingChords = new Compartment();

/** Re-bind an open editor to a new set of chords. */
export function applyFormattingChords(view: EditorView, chords: FormattingChords): void {
  view.dispatch({
    effects: formattingChords.reconfigure(keymap.of([...formattingKeymapFor(chords)]))
  });
}

export function createEditor({
  parent,
  doc,
  onDocChange,
  extraKeymap = [],
  formatting = DEFAULT_CHORDS
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
        // No `drawSelection`. It replaces the caret with an element of its own
        // and blanks the native one, and an element outside the line cannot be
        // coloured per surface — the cursor has to be visible on a dark code
        // panel as well as on paper. The native caret takes `caret-color`,
        // which is a property of the line it sits in. Selection is styled
        // through `::selection` either way.
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
        // a chord, and sit in their own compartment so a changed chord can be
        // swapped into an open editor. Everything else is fixed for its life.
        formattingChords.of(keymap.of([...formattingKeymapFor(formatting)])),
        keymap.of([...extraKeymap, ...historyKeymap, ...defaultKeymap])
      ]
    })
  });
}
