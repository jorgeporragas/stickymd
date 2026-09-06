<script lang="ts">
  import type { EditorView, KeyBinding } from '@codemirror/view';
  import { onMount } from 'svelte';
  import { createEditor } from '../editor';

  interface Props {
    /** Initial markdown source. */
    value?: string;
    /** Called with the full source whenever the document changes. */
    onChange?: (body: string) => void;
    /** Window-level key bindings this editor should serve. */
    keymap?: readonly KeyBinding[];
  }

  let { value = '', onChange, keymap = [] }: Props = $props();

  let host!: HTMLDivElement;
  let view: EditorView | undefined;

  onMount(() => {
    view = createEditor({ parent: host, doc: value, onDocChange: onChange, extraKeymap: keymap });
    view.focus();

    return () => {
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
