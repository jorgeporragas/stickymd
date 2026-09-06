<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount } from 'svelte';
  import Editor from '../../lib/components/Editor.svelte';
  import WindowChrome from '../../lib/components/WindowChrome.svelte';
  import { flushSave, queueSave } from '../../lib/state/note';
  import { openNewNoteWindow } from '../../lib/state/windows';

  interface Props {
    /** The note's source. A new window starts empty; reopening an existing
        note arrives with session restore, which is Phase 3 work. */
    initial?: string;
  }

  let { initial = '' }: Props = $props();

  let revealed = $state(false);

  // Mod- resolves to Ctrl or Cmd per platform. Never write either literally
  // (CLAUDE.md section 'Cross-platform discipline').
  //
  // Mod-Alt-n rather than the obvious Mod-n: WebView2 keeps Ctrl+N for itself
  // and the chord never reaches the page. See docs/FIXES.md.
  const windowKeymap = [
    {
      key: 'Mod-Alt-n',
      run: () => {
        void openNewNoteWindow();
        return true;
      }
    }
  ];

  function reveal(): void {
    revealed = true;
  }

  function recede(): void {
    revealed = false;
  }

  onMount(() => {
    // Closing the window must not lose the last few characters typed: the
    // debounce may still be pending. Take over the close, write, then close.
    const unlisten = getCurrentWindow()
      .onCloseRequested(async (event) => {
        event.preventDefault();
        await flushSave();
        await getCurrentWindow().destroy();
      })
      .catch(() => undefined);

    return () => {
      void unlisten.then((stop) => stop?.());
    };
  });
</script>

<!--
  Chrome reveals on entering the window and recedes on leaving it or on the
  window losing focus — docs/DESIGN.md principle 2. These belong on the window
  and body rather than on the surface element: entering the *window* is the
  gesture, and the surface is not an interactive control.

  Losing focus also flushes any pending write. Switching away from a note is
  the moment a user expects it to be saved.
-->
<svelte:body onpointerenter={reveal} onpointerleave={recede} />
<svelte:window
  onfocus={reveal}
  onblur={() => {
    recede();
    void flushSave();
  }}
/>

<div class="surface">
  <WindowChrome {revealed} />
  <Editor value={initial} onChange={queueSave} keymap={windowKeymap} />
</div>

<style>
  .surface {
    display: flex;
    flex-direction: column;
    height: 100%;
    border-radius: var(--radius-window);
    border: 1px solid var(--surface-border);

    /* The window itself is transparent so the corners can round; the surface is
       painted here. --surface-paint resolves to the opaque surface or the tint
       over compositor blur, per data-surface. No branching here — the token
       carries the difference. See docs/DESIGN.md principle 4. */
    background: var(--surface-paint);
    box-shadow:
      var(--shadow-rest),
      var(--surface-edge-highlight);

    overflow: hidden;
  }
</style>
