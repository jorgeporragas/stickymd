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
   * The tints the palette is showing, captured when it opens.
   *
   * The note's own colour is not among them: the swatch *is* that option, and
   * an option cannot be in two places at once — the founder's point, and it is
   * also what makes the palette read as the swatch opening rather than as a
   * list appearing beside it.
   *
   * Captured rather than derived because choosing a tint changes the tint, and
   * a derived list would rearrange itself mid-dismissal: the chosen disc would
   * vanish and the old one appear, in the middle of the animation carrying them
   * all back into the swatch.
   */
  let shown = $state<Tint[]>([]);

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
      shown = TINTS.filter((option) => option !== tint);
      open = true;
      closing = false;
    }
  }

  function choose(value: Tint): void {
    onTint(value);
    closing = true;
  }

  /**
   * Called by the disc nearest the swatch, which is the last one to leave.
   *
   * The palette retracts into the swatch, so the far end goes first and this
   * one goes last — when it is done, there is nothing left on screen. It has to
   * be a specific disc rather than whichever animation happens to end: they all
   * report, and the first to finish would take the rest down with it.
   */
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
    class="lozenge lit swatch"
    class:revealed
    data-tint-swatch={tint}
    type="button"
    aria-label="Note colour"
    aria-expanded={open && !closing}
    onclick={toggle}
  ></button>

  {#if open}
    <div class="palette" class:closing role="group" aria-label="Note colour">
      {#each shown as option, index (option)}
        <button
          class="lozenge lit dot bubble-in"
          data-tint-swatch={option}
          type="button"
          aria-label={option}
          style="animation-delay: {(closing ? shown.length - 1 - index : index) * 28}ms"
          onclick={() => choose(option)}
          onanimationend={index === 0 ? settled : undefined}
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

  /*
    Both discs take their colour as `color`, so the fill, the rim and the
    greyed-out state all follow from one declaration per tint. The disc itself
    is `.lozenge`, in src/app.css — the window's controls wear it too.
  */
  /* One diameter for every disc in the window — the founder's call, and the
     traffic lights' own proportion: small, and read by colour and position
     rather than by size. */
  .swatch {
    width: var(--space-4);
    height: var(--space-4);
    /* Small control, not a glass surface: fading is cheap and correct. */
    opacity: 0;
    transition: opacity var(--dur-quick) var(--ease-out);
  }

  /*
    Recedes like every other control, tinted or not. It kept itself visible
    while a note had a colour — chrome carrying information, by principle 2 —
    but the founder found a swatch that would not go away worse than one he has
    to hover for, and the note's colour is already visible: it is the window.
  */
  .swatch.revealed,
  .swatch:focus-visible {
    opacity: 1;
  }

  .palette {
    position: absolute;
    /*
      Beside the swatch, on its line, running away to the left.

      It used to drop below, which put seven discs over the note's first
      paragraph — and now that they are translucent, the writing sat inside
      them. Here there is nothing behind them but the window's own drag bar.

      Leftwards because the picker sits near the window's trailing edge and the
      surface clips what leaves it. The same fact as before; a different answer,
      because the palette is now a row on the chrome rather than a panel under
      it.
    */
    top: 50%;
    right: calc(100% + var(--space-2));
    transform: translateY(-50%);
    z-index: 1;
    display: flex;
    /*
      Reversed, so the first disc in the markup is the one nearest the swatch.

      That is what makes the order mean something: the discs arrive outward
      from the swatch and retract back into it, and both are the same index
      counted in opposite directions.
    */
    flex-direction: row-reverse;
    /* Wider than it was, because there is no longer a panel holding the discs
       together — the spacing is what says they are one group. */
    gap: var(--space-2);

    /*
      No panel: the discs float on the note.

      It had one, and the panel was opaque for a good reason — painted with the
      window's own translucent tint, the note's writing read through it. But
      that reason was about a *panel*, and there is none now. What is left is
      seven opaque discs with the note between them, which is what the insert
      ring already does over the same text.

      Its arrival went with it. The palette used to open as one object, scaling
      out of the swatch; now each disc arrives on its own, staggered, and a
      container scaling underneath that would be a second animation saying the
      same thing more slowly.
    */
  }

  /*
    Retracting: the same motion run backwards, and the same order counted from
    the other end — the far disc goes first and the one against the swatch goes
    last, so the row draws back into the control it came out of. Quicker than
    the arrival, because leaving is getting out of the way.

    `forwards`, so a disc that has gone stays gone: without it each one would
    snap back to full size the moment its own animation ended, and the row
    would reassemble itself while the last disc was still leaving.
  */
  .palette.closing .dot {
    animation: disc-out var(--dur-instant) var(--ease-in-out) forwards;
  }

  @keyframes disc-out {
    from {
      opacity: 1;
      transform: scale(1);
    }
    to {
      opacity: 0;
      transform: scale(0.4);
    }
  }

  .dot {
    width: var(--space-4);
    height: var(--space-4);
    /*
      Each disc grows out of its own trailing edge — the side facing the swatch
      — rather than out of its middle. Six discs each swelling in place is six
      things appearing; six growing towards you from the control you clicked is
      one thing opening.
    */
    transform-origin: right center;
  }

  /* The tints themselves. These are the only place a colour is named outside
     the token file, and they are named *through* tokens — and as `color`
     rather than `background`, which is what lets one rule rim every disc. */
  [data-tint-swatch='sun'] {
    color: var(--swatch-sun);
  }

  [data-tint-swatch='spring'] {
    color: var(--swatch-spring);
  }

  [data-tint-swatch='aqua'] {
    color: var(--swatch-aqua);
  }

  [data-tint-swatch='sky'] {
    color: var(--swatch-sky);
  }

  [data-tint-swatch='lilac'] {
    color: var(--swatch-lilac);
  }

  [data-tint-swatch='blush'] {
    color: var(--swatch-blush);
  }

  /* Clear is the absence of a tint, so it is drawn as one: a lit rim with
     nothing in it. No slash, no label — an empty lozenge says it. */
  [data-tint-swatch='clear'] {
    --lozenge-fill: transparent;
    --lozenge-pressed: transparent;
    border-color: var(--rim-neutral);
  }
</style>
