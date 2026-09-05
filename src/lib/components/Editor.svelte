<script lang="ts">
  import type { EditorView } from '@codemirror/view';
  import { onMount } from 'svelte';
  import { createEditor } from '../editor';

  interface Props {
    /** Initial markdown source. */
    value?: string;
  }

  let { value = '' }: Props = $props();

  let host!: HTMLDivElement;
  let view: EditorView | undefined;

  onMount(() => {
    view = createEditor(host, value);
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
