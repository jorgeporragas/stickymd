<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  interface Props {
    /** Whether the chrome is currently revealed. The window shell decides. */
    revealed: boolean;
  }

  let { revealed }: Props = $props();

  function close(): void {
    void getCurrentWindow().close();
  }
</script>

<!--
  The drag region is the whole bar. Controls sit above it and recede when the
  window is not in use — docs/DESIGN.md principle 2.
-->
<div class="chrome" data-tauri-drag-region>
  <button class="control" class:revealed type="button" aria-label="Close note" onclick={close}>
    <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
      <path d="M3.5 3.5 L8.5 8.5 M8.5 3.5 L3.5 8.5" />
    </svg>
  </button>
</div>

<style>
  .chrome {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: var(--space-1);
    padding: var(--space-2);
    flex: 0 0 auto;
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
  .control:focus-visible {
    opacity: 1;
  }

  .control:hover {
    color: var(--ink-primary);
    background: var(--surface-border);
  }

  .control svg {
    width: 12px;
    height: 12px;
    stroke: currentColor;
    stroke-width: 1.5;
    stroke-linecap: round;
    fill: none;
  }
</style>
