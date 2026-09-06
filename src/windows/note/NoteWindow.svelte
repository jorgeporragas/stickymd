<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount } from 'svelte';
  import Editor from '../../lib/components/Editor.svelte';
  import TintPicker from '../../lib/components/TintPicker.svelte';
  import WindowChrome from '../../lib/components/WindowChrome.svelte';
  import { flushSave, queueSave, setAlwaysOnTop, setTint, type Tint } from '../../lib/state/note';

  interface Props {
    /** The note's source. A new window starts empty. */
    initial?: string;
    /**
     * Whether this note was pinned when it was last open. Read once, at
     * mount: after that the window owns the setting and writes it back.
     */
    initiallyPinned?: boolean;
    /** The note's colour when it was last open. Read once, as above. */
    initialTint?: Tint;
  }

  let { initial = '', initiallyPinned = false, initialTint = 'clear' }: Props = $props();

  let revealed = $state(false);

  // Capturing the initial value is the intent: the index seeds the window, and
  // from then on the window is authoritative and persists its own changes.
  // svelte-ignore state_referenced_locally
  let alwaysOnTop = $state(initiallyPinned);
  // svelte-ignore state_referenced_locally
  let tint = $state<Tint>(initialTint);

  function chooseTint(value: Tint): void {
    tint = value;
    void setTint(value);
  }

  function togglePin(value: boolean): void {
    alwaysOnTop = value;
    void setAlwaysOnTop(value);
  }

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
  <WindowChrome {revealed} {alwaysOnTop} onAlwaysOnTop={togglePin} closeLabel="Close note">
    {#snippet leading()}
      <TintPicker {revealed} {tint} onTint={chooseTint} />
    {/snippet}
  </WindowChrome>
  <Editor value={initial} onChange={queueSave} />
</div>

<style>
  .surface {
    display: flex;
    flex-direction: column;

    /* Pinned to the viewport rather than sized at 100%.
       A percentage height resolves to a fractional pixel, leaving the surface a
       sliver short of the window — and because the window is transparent over
       compositor blur, an unpainted sliver shows raw acrylic as a pale line
       down the right edge and along the bottom. The same failure as the corner
       artefacts: anything this surface does not cover, the backdrop does.
       See docs/FIXES.md. */
    position: fixed;
    inset: 0;
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
