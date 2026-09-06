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

  /**
   * Whether the palette is playing its dismissal.
   *
   * It has to stay mounted while it leaves — removing it on the click is what
   * made it vanish rather than close. The animation itself says when it is
   * done, which is also what keeps reduced motion working: the global rule
   * collapses the duration, `animationend` still fires, and the palette is
   * gone at once rather than left mid-transition.
   */
  let closing = $state(false);

  function toggle(): void {
    if (open && !closing) {
      closing = true;
    } else {
      // Clicking again mid-dismissal takes it back rather than queueing a
      // second open behind the one that is leaving.
      open = true;
      closing = false;
    }
  }

  function choose(value: Tint): void {
    onTint(value);
    closing = true;
  }

  function settled(): void {
    if (!closing) return;
    open = false;
    closing = false;
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
    aria-expanded={open && !closing}
    onclick={toggle}
  ></button>

  {#if open}
    <div class="palette" class:closing role="group" aria-label="Note colour" onanimationend={settled}>
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
    /* Anchored to its right edge, not its left. The picker sits near the
       window's trailing edge and the surface clips what leaves it, so a
       palette opening rightwards would be cut in half. */
    right: 0;
    z-index: 1;
    display: flex;
    gap: var(--space-1);
    padding: var(--space-2);
    border-radius: var(--radius-control);
    border: 1px solid var(--rule);
    box-shadow: var(--shadow-lifted);

    /* Opaque, not the window's own paint. --surface-paint is translucent in
       Glass mode, and what is behind this popover is the note's text, so the
       writing showed through the swatches. --surface-solid is the same colour
       the note would be painted if it were opaque — so the palette reads as
       part of this note rather than as a panel from somewhere else. */
    background: var(--surface-solid);

    /* Opens out of the swatch rather than appearing over it. A control this
       small sits inside the window's own surface, so it may scale: there is no
       backdrop behind it to show through. */
    transform-origin: top right;
    animation: palette-open var(--dur-settle) var(--ease-out);
  }

  /* Leaves faster than it arrives. Arriving is the palette presenting itself
     and is worth the time; leaving is getting out of the way, and a dismissal
     that takes as long as the arrival reads as lag. */
  .palette.closing {
    animation: palette-close var(--dur-quick) var(--ease-in-out) forwards;
  }

  @keyframes palette-open {
    from {
      opacity: 0;
      transform: scale(0.88);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  @keyframes palette-close {
    from {
      opacity: 1;
      transform: scale(1);
    }
    to {
      opacity: 0;
      transform: scale(0.92);
    }
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
    outline: 2px solid var(--ink-primary);
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
