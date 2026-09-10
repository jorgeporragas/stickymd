<script lang="ts">
  import { onMount } from 'svelte';
  import { PIXEL_ICONS, type PixelIcon } from '../icons/pixel';

  export interface RadialAction {
    id: string;
    /** What it does. The accessible name, and the label unless `short` says otherwise. */
    label: string;
    /**
     * A shorter form for the ring's centre, where space is the radius less a
     * bubble. `label` stays the accessible name — a screen reader saying
     * "star list" would be worse than one saying "bulleted list", and the
     * reason to shorten is entirely about a pill bumping into the buttons.
     */
    short?: string;
    icon: PixelIcon;
  }

  interface Props {
    actions: RadialAction[];
    /** Where the pointer was, in client coordinates. */
    at: { x: number; y: number };
    onChoose: (id: string) => void;
    onDismiss: () => void;
  }

  let { actions, at, onChoose, onDismiss }: Props = $props();

  /**
   * Distance from the centre to each bubble.
   *
   * Sixty rather than the fifty-two it was: six bubbles of forty need about
   * twenty between them or the ring reads as one lumpy shape instead of six
   * things you can aim at.
   */
  const RADIUS = 60;
  /** Half a bubble plus a little, so none of the ring meets the window edge. */
  const MARGIN = 30;

  let hovered = $state<string | undefined>(undefined);

  /** The hovered action itself, which is what the centre names. */
  const named = $derived(actions.find((action) => action.id === hovered));

  /**
   * Whether the ring is playing its exit.
   *
   * It has to outlive the click that ended it — the same shape the tint palette
   * uses (SMD-080), and for the same reason: a menu removed on the click never
   * gets to leave. `onChoose` and `onDismiss` are called when the animation is
   * done, so the window acts on the choice at the moment the ring is gone.
   */
  let leaving = $state<{ id?: string } | undefined>(undefined);

  function choose(id: string): void {
    if (leaving) return;
    leaving = { id };
  }

  function dismiss(): void {
    if (leaving) return;
    leaving = {};
  }

  /**
   * Called by the first bubble, which is the last to finish: every bubble pops
   * at once but they are not all the same distance through it, and taking the
   * first `animationend` would cut the others off mid-pop.
   */
  function gone(): void {
    if (!leaving) return;

    const chosen = leaving.id;
    leaving = undefined;

    if (chosen) onChoose(chosen);
    else onDismiss();
  }

  /**
   * The ring's centre, pulled back from the window's edges.
   *
   * The surface clips what leaves it, so a ring opened in a corner would lose
   * half its bubbles. It opens beside the pointer instead of being cut.
   */
  const centre = $derived({
    x: Math.min(Math.max(at.x, RADIUS + MARGIN), window.innerWidth - RADIUS - MARGIN),
    y: Math.min(Math.max(at.y, RADIUS + MARGIN), window.innerHeight - RADIUS - MARGIN)
  });

  /** Bubble positions, starting at the top and going clockwise. */
  const placed = $derived(
    actions.map((action, index) => {
      const angle = (index / actions.length) * Math.PI * 2 - Math.PI / 2;
      return {
        action,
        x: centre.x + Math.cos(angle) * RADIUS,
        y: centre.y + Math.sin(angle) * RADIUS
      };
    })
  );

  onMount(() => {
    const key = (event: KeyboardEvent) => {
      if (event.key === 'Escape') dismiss();
    };
    window.addEventListener('keydown', key);
    return () => window.removeEventListener('keydown', key);
  });
</script>

<!--
  A ring of seated bubbles, opened at the pointer.

  It held window actions first — new note, the hub, settings, the pin — and
  they moved out: those all had a control of their own already, where the
  markdown that is awkward to type by hand had nothing. A ring that both
  inserted a table and opened settings would have to be read every time. One
  that only inserts is a gesture you learn once.

  What is in the ring is still one array, held by the window.

  The backdrop catches the click that dismisses it. It is transparent rather
  than absent, because a menu you can dismiss only by choosing something is a
  menu that has taken the window hostage.
-->
<div
  class="backdrop"
  role="presentation"
  onpointerdown={dismiss}
  oncontextmenu={(event) => {
    event.preventDefault();
    dismiss();
  }}
></div>

<div class="ring" class:leaving role="menu" aria-label="Insert">
  {#each placed as { action, x, y }, index (action.id)}
    <!-- The seat carries the position, the bubble carries the arrival. Kept
         apart because `.bubble-in` animates `transform`, and a bubble placed by
         its centre needs a translate that the keyframe would overwrite. -->
    <span class="seat" style="left: {x}px; top: {y}px">
      <button
        class="lozenge bubble"
        class:bubble-in={!leaving}
        class:bubble-out={leaving}
        role="menuitem"
        type="button"
        aria-label={action.label}
        style="animation-delay: {leaving ? 0 : index * 28}ms"
        onpointerenter={() => (hovered = action.id)}
        onpointerleave={() => (hovered = undefined)}
        onclick={() => choose(action.id)}
        onanimationend={index === 0 ? gone : undefined}
      >
        <svg viewBox="0 0 24 24" aria-hidden="true" focusable="false">
          <path d={PIXEL_ICONS[action.icon]} />
        </svg>
      </button>
    </span>
  {/each}

  {#if named}
    <span class="lozenge label" style="left: {centre.x}px; top: {centre.y}px">
      {named.short ?? named.label}
    </span>
  {/if}
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 2;
  }

  .ring {
    position: fixed;
    inset: 0;
    z-index: 3;
    pointer-events: none;
  }

  .seat {
    position: absolute;
    /* Placed by its centre rather than its corner: the ring is described in
       centres, and translating here keeps the arithmetic above honest. */
    transform: translate(-50%, -50%);
  }

  .bubble {
    pointer-events: auto;

    display: grid;
    place-items: center;
    /* 40px rather than the scale's 32: the glyphs are pixel art on a 24-unit
       grid and are only sharp at 24px, so the bubble has to be large enough to
       carry one at its own size. Off the scale deliberately — see DESIGN. */
    width: 40px;
    height: 40px;
    color: var(--control-glyph);
  }

  .bubble:hover {
    color: var(--ink-primary);
  }

  /* The pop itself is `.bubble-out` in src/app.css, applied above — shared with
     anything else that ever needs a bubble to leave, and named beside the
     arrival it answers.

     The label goes with them, and faster: it is a caption on a thing that is
     leaving, and a name left hanging over an empty ring reads as a bug. */
  .ring.leaving .label {
    animation: label-out var(--dur-instant) var(--ease-out) forwards;
  }

  @keyframes label-out {
    to {
      opacity: 0;
    }
  }

  /*
    The pixel glyphs fill rather than stroke, and are pinned to 24px rather
    than taking `.lozenge`'s proportional 58%: at any other size the grid falls
    between screen pixels and the glyph turns to mush. Stacking still comes
    from `.lozenge`.
  */
  .bubble svg {
    width: 24px;
    height: 24px;
    fill: currentColor;
    stroke: none;
  }

  /*
    The hovered action's name, in the middle of the ring — the one place
    nothing else occupies, and the only way a ring of glyphs says what it does
    without a legend around it.

    It wears `.lozenge`, which is what makes it part of the ring rather than a
    tooltip laid over it: the same glass, the same seat, the same frosting of
    whatever is behind it. It was an opaque chip with a hairline border, which
    is the one thing in this window that belonged to no family.

    A pill rather than a disc, so `.lozenge`'s 50% is overridden here — the
    only two shapes that treatment has are the round one and this, and the
    hub's square delete control (ADR-031). It is a lozenge in the word's own
    sense, which is why the name survived a square member.
  */
  .label {
    position: absolute;
    transform: translate(-50%, -50%);
    border-radius: var(--radius-chip);
    padding: 2px var(--space-2);

    /* The display face. This is the application naming its own parts — the
       same voice as the hub's title and the window titles — rather than a
       sentence it is saying to the user, which is what the content face is
       for. See docs/DESIGN.md § Typography. */
    font-family: var(--font-display);
    font-size: var(--font-size-caption);
    letter-spacing: var(--tracking-display);
    line-height: var(--line-height-ui);
    color: var(--ink-primary);
    white-space: nowrap;

    /* Not a control: nothing here is clickable, and a pointer over the middle
       of the ring should reach the backdrop that dismisses it. */
    pointer-events: none;
    cursor: default;
  }
</style>
