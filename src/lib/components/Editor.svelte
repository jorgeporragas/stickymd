<script lang="ts">
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
  }

  let { value = '', onChange, keymap = [], formatting = DEFAULT_CHORDS }: Props = $props();

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
