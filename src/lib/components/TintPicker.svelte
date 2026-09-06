<script lang="ts">
  import { TINTS, type Tint } from '../state/note';

  interface Props {
    /** Whether the chrome is currently revealed. */
    revealed: boolean;
    tint: Tint;
    onTint: (value: Tint) => void;
  }

  let { revealed, tint, onTint }: Props = $props();

  let open = $state(false);

  function choose(value: Tint): void {
    onTint(value);
    open = false;
  }
</script>

<!--
  A swatch that opens the palette. Closed, it is one control the width of the
  others; open, it is the seven tints in a row.

  Like the pin, a note whose tint is not Clear keeps its swatch visible when the
  chrome recedes — the swatch is carrying information, and docs/DESIGN.md
  principle 2 stops receding at that point.
-->
<div class="picker">
  <button
    class="swatch control"
    class:revealed
    class:coloured={tint !== 'clear'}
    data-tint-swatch={tint}
    type="button"
    aria-label="Note colour"
    aria-expanded={open}
    onclick={() => (open = !open)}
  ></button>

  {#if open}
    <div class="palette" role="group" aria-label="Note colour">
      {#each TINTS as option (option)}
        <button
          class="dot"
          class:selected={option === tint}
          data-tint-swatch={option}
          type="button"
          aria-label={option}
          aria-pressed={option === tint}
          onclick={() => choose(option)}
        ></button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .picker {
    position: relative;
    display: flex;
    align-items: center;
  }

  .control {
    width: var(--space-4);
    height: var(--space-4);
    padding: 0;
    border-radius: 50%;
    cursor: pointer;

    /* Small control, not a glass surface: fading is cheap and correct. */
    opacity: 0;
    transition: opacity var(--dur-quick) var(--ease-out);
  }

  .swatch {
    border: 1px solid var(--rule);
    background: var(--surface-paint);
  }

  .swatch.revealed,
  .swatch.coloured,
  .swatch:focus-visible {
    opacity: 1;
  }

  .palette {
    position: absolute;
    top: calc(100% + var(--space-1));
    left: 0;
    z-index: 1;
    display: flex;
    gap: var(--space-1);
    padding: var(--space-2);
    border-radius: var(--radius-control);
    border: 1px solid var(--rule);
    background: var(--surface-paint);
    box-shadow: var(--shadow-lifted);
  }

  .dot {
    width: var(--space-4);
    height: var(--space-4);
    padding: 0;
    border-radius: 50%;
    border: 1px solid var(--rule);
    cursor: pointer;
    opacity: 1;
  }

  .dot.selected {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  /* The swatches show the tints themselves. These are the only place a colour
     is named outside the token file, and they are named *through* tokens: each
     reads the same --surface-solid the tint paints a window with. */
  [data-tint-swatch='clear'] {
    background: var(--surface-solid);
  }

  [data-tint-swatch='sun'] {
    background: var(--swatch-sun);
  }

  [data-tint-swatch='spring'] {
    background: var(--swatch-spring);
  }

  [data-tint-swatch='aqua'] {
    background: var(--swatch-aqua);
  }

  [data-tint-swatch='sky'] {
    background: var(--swatch-sky);
  }

  [data-tint-swatch='lilac'] {
    background: var(--swatch-lilac);
  }

  [data-tint-swatch='blush'] {
    background: var(--swatch-blush);
  }
</style>
