<script lang="ts">
  import type { Snippet } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';

  interface Props {
    /** Whether the chrome is currently revealed. The window shell decides. */
    revealed: boolean;
    /** Whether this note floats above other windows. */
    alwaysOnTop?: boolean;
    /**
     * Called when the pin is clicked, with the value it should take. Omit it
     * and no pin is rendered — the hub is a window, but not a note.
     */
    onAlwaysOnTop?: (value: boolean) => void;
    /** What the close button is called, for screen readers. */
    closeLabel?: string;
    /** A control this window puts at the start of the bar, before the drag
        region's empty space. The hub has none; a note has its tint picker. */
    leading?: Snippet;
  }

  let {
    revealed,
    alwaysOnTop = false,
    onAlwaysOnTop,
    closeLabel = 'Close window',
    leading
  }: Props = $props();

  function close(): void {
    void getCurrentWindow().close();
  }
</script>

<!--
  The drag region is the whole bar. Controls sit above it and recede when the
  window is not in use — docs/DESIGN.md principle 2.

  The pin is the exception: a pinned note keeps it visible, because a state you
  cannot see is a state you cannot trust. Chrome recedes; chrome that is
  carrying information does not.
-->
<div class="chrome" data-tauri-drag-region>
  {#if leading}
    <div class="leading">{@render leading()}</div>
  {/if}

  {#if onAlwaysOnTop}
    <button
      class="control"
      class:revealed
      class:active={alwaysOnTop}
      type="button"
      aria-label="Keep this note on top"
      aria-pressed={alwaysOnTop}
      onclick={() => onAlwaysOnTop(!alwaysOnTop)}
    >
      <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
        <path d="M4.6 1.5h2.8l-.4 2.7 1.7 1.7v.9H3.3v-.9l1.7-1.7z" />
        <path d="M6 6.8V10.5" />
      </svg>
    </button>
  {/if}

  <button class="control" class:revealed type="button" aria-label={closeLabel} onclick={close}>
    <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
      <path d="M3.5 3.5 L8.5 8.5 M8.5 3.5 L3.5 8.5" />
    </svg>
  </button>
</div>

<style>
  .chrome {
    display: flex;
    justify-content: flex-end;
    /* The leading slot claims the space, so the drag region keeps the whole
       bar rather than being split by it. */
    align-items: center;
    gap: var(--space-1);
    padding: var(--space-2);
    flex: 0 0 auto;
  }

  .leading {
    margin-right: auto;
    display: flex;
    align-items: center;
  }

  .control {
    display: grid;
    place-items: center;
    width: var(--space-6);
    height: var(--space-6);
    padding: 0;
    border: 0;
    border-radius: var(--radius-chip);
    background: transparent;
    color: var(--ink-muted);
    cursor: pointer;

    /* Small control, not a glass surface: fading is cheap and correct.
       See docs/DESIGN.md section 'Never Allowed'. */
    opacity: 0;
    transition:
      opacity var(--dur-quick) var(--ease-out),
      color var(--dur-quick) var(--ease-out),
      background-color var(--dur-quick) var(--ease-out);
  }

  .control.revealed,
  .control.active,
  .control:focus-visible {
    opacity: 1;
  }

  .control.active {
    color: var(--accent);
  }

  .control:hover {
    color: var(--ink-primary);
    background: var(--control-hover);
  }

  .control.active:hover {
    color: var(--accent-hover);
  }

  .control svg {
    width: 12px;
    height: 12px;
    stroke: currentColor;
    stroke-width: 1.5;
    stroke-linecap: round;
    stroke-linejoin: round;
    fill: none;
  }
</style>
