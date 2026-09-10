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
    /** A control this window adds to the set, before the pin. The hub has
        none; a note has its tint picker. */
    controls?: Snippet;
  }

  let {
    revealed,
    alwaysOnTop = false,
    onAlwaysOnTop,
    closeLabel = 'Close window',
    controls
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
  {#if controls}
    {@render controls()}
  {/if}

  {#if onAlwaysOnTop}
    <button
      class="lozenge chrome-control"
      class:revealed
      class:active={alwaysOnTop}
      class:lit={alwaysOnTop}
      type="button"
      aria-label="Keep this note on top"
      aria-pressed={alwaysOnTop}
      onclick={() => onAlwaysOnTop(!alwaysOnTop)}
    >
      <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
        <path d="M4.4 2h3.2v2.6l1.6 1.7v.6H2.8v-.6l1.6-1.7z" />
        <path d="M6 7v3.2" />
      </svg>
    </button>
  {/if}

  <button class="lozenge chrome-control" class:revealed type="button" aria-label={closeLabel} onclick={close}>
    <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
      <path d="M3.8 3.8 8.2 8.2M8.2 3.8 3.8 8.2" />
    </svg>
  </button>
</div>

<style>
  .chrome {
    display: flex;
    /* Every control sits at the trailing end, so the drag region is the rest
       of the bar in one piece rather than split around something. */
    justify-content: flex-end;
    align-items: center;
    gap: var(--space-1);
    padding: var(--space-2);
    flex: 0 0 auto;
  }

  /*
    Seated discs, like the tint swatch beside them — one bezelled control in a
    row of flat ones reads as an accident rather than as an emphasis. The disc
    itself is `.lozenge`, in src/app.css.

    The rim is the exception to reading `currentColor`: these are not coloured
    by a tint, and mixing a rim out of the glyph's own ink would draw a hard
    ring around a control whose job is to stay quiet.
  */
  /* Size, glyph colour and the receding are `.chrome-control` in src/app.css —
     shared with the hub's new-note button, which is rendered into the snippet
     above and so cannot reach anything scoped to this component.

     What stays here is the one thing that is this chrome's alone: the pin does
     not recede. */
  .chrome-control.active {
    opacity: 1;
  }

  /*
    Pinned inverts rather than fills a little harder. Filling was too close to
    hover to tell apart once both were discs, and an inverted control is the
    plainest "held down" there is — no colour required to say it.
  */
  /*
    Pinned takes a colour rather than inverting. Inverting made it the darkest
    thing in the window for a setting that is not an alarm, and this is what
    the traffic lights do anyway: a lit control is a coloured one.

    It reads its hue as `color`, which is what feeds the same --gloss-tinted
    and --swatch-rim the tint swatches use — one lit lozenge, not a second
    kind of control.

    It also does not recede. A pin you cannot see is a note you do not know is
    floating, which is the same reason a tinted swatch stays put: chrome
    recedes, chrome that is carrying information does not.
  */
  .chrome-control.active,
  .chrome-control.active:hover {
    color: var(--signal-engaged);
  }

  /* The glyph cannot be `currentColor` here — that is now the fill's own hue.
     The hue taken most of the way to black is how Aqua drew a traffic light's
     glyph, and it holds against both ends of the gradient. */
  .chrome-control.active svg {
    stroke: color-mix(in oklab, var(--signal-engaged) 25%, var(--rim-shade));
  }

</style>
