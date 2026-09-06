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
      class="lozenge control"
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

  <button class="lozenge control" class:revealed type="button" aria-label={closeLabel} onclick={close}>
    <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
      <path d="M3.5 3.5 L8.5 8.5 M8.5 3.5 L3.5 8.5" />
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
  .control {
    display: grid;
    place-items: center;
    width: var(--space-4);
    height: var(--space-4);
    border-color: var(--rim-control);
    --lozenge-fill: var(--gloss-control);
    --lozenge-pressed: var(--gloss-pressed);
    color: var(--control-glyph);

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

  /* Hover darkens the glyph rather than colouring it: on a tinted control the
     glyph is already the note's hue taken most of the way to black, and there
     is nowhere darker for it to go that means anything. */
  .control:hover {
    color: var(--ink-primary);
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
  .control.active,
  .control.active:hover {
    color: var(--signal-engaged);
    --lozenge-fill: var(--gloss-tinted);
    --lozenge-pressed: var(--gloss-tinted-pressed);
    border-color: var(--swatch-rim);
  }

  /* The glyph cannot be `currentColor` here — that is now the fill's own hue.
     The hue taken most of the way to black is how Aqua drew a traffic light's
     glyph, and it holds against both ends of the gradient. */
  .control.active svg {
    stroke: color-mix(in oklab, var(--signal-engaged) 25%, var(--rim-shade));
  }

  .control svg {
    /* Proportioned to the disc rather than to the icon: the traffic lights
       run about 0.58 of their diameter, and the glyph is there to be
       recognised, not read.

       Above the specular, which is where Aqua drew it too — its glyphs sit on
       the glass, not under it. At nine pixels a highlight across the top of a
       glyph is the difference between reading it and guessing. */
    position: relative;
    z-index: 1;
    width: 9px;
    height: 9px;
    stroke: currentColor;
    stroke-width: 1.5;
    stroke-linecap: round;
    stroke-linejoin: round;
    fill: none;
  }
</style>
