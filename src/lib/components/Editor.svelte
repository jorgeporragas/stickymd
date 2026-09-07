<script lang="ts">
  import type { StateCommand } from '@codemirror/state';
  import type { EditorView, KeyBinding } from '@codemirror/view';
  import { onMount } from 'svelte';
  import { applyFormattingChords, createEditor } from '../editor';
  import { DEFAULT_CHORDS, type FormattingChords } from '../editor/commands';
  import { listen } from '@tauri-apps/api/event';

  interface Props {
    /** Initial markdown source. */
    value?: string;
    /** Called with the full source whenever the document changes. */
    onChange?: (body: string) => void;
    /** Window-level key bindings this editor should serve. */
    keymap?: readonly KeyBinding[];
    /** The formatting chords. */
    formatting?: FormattingChords;
    /**
     * Handed a way to run editor commands, once the editor exists.
     *
     * The insert menu's route into the document. A callback rather than the
     * view itself: the window gets to insert, and does not get to reach into
     * CodeMirror — that layer stays behind `src/lib/editor/`.
     */
    onReady?: (run: (command: StateCommand, at?: { x: number; y: number }) => void) => void;
  }

  let {
    value = '',
    onChange,
    keymap = [],
    formatting = DEFAULT_CHORDS,
    onReady
  }: Props = $props();

  let host!: HTMLDivElement;
  let view: EditorView | undefined;

  onMount(() => {
    view = createEditor({
      parent: host,
      doc: value,
      onDocChange: onChange,
      extraKeymap: keymap,
      formatting
    });
    view.focus();

    // Focus returns to the text afterwards: an insert leaves the cursor
    // somewhere you are meant to type, and it would be no use behind a menu
    // that has just taken the keyboard.
    //
    // `at` is where the pointer opened the menu. Right-clicking does not move
    // the caret, so without this a table asked for at the bottom of a note
    // would appear wherever the caret happened to be last — which is the kind
    // of thing you only notice after it has moved your text.
    onReady?.((command, at) => {
      if (!view) return;

      if (at) {
        const pos = view.posAtCoords(at);
        if (pos !== null) view.dispatch({ selection: { anchor: pos } });
      }

      command(view);
      view.focus();
    });

    // Re-bound in place rather than on the next window: a shortcut that only
    // applied to windows opened afterwards is a setting that appears not to
    // work, which is the same reason the theme is broadcast.
    const listening = listen<FormattingChords>('formatting-changed', (event) => {
      if (view) applyFormattingChords(view, event.payload);
    }).catch(() => undefined);

    return () => {
      void listening.then((stop) => stop?.());
      view?.destroy();
      view = undefined;
    };
  });
</script>

<div class="editor" bind:this={host}></div>

<style>
  .editor {
    flex: 1 1 auto;
    min-height: 0;
    overflow: hidden;
  }
</style>
