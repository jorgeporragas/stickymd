<script lang="ts">
  import { onMount } from 'svelte';
  import { PIXEL_ICONS, type PixelIcon } from '../icons/pixel';

  export interface RadialAction {
    id: string;
    /** What it does, for screen readers and the hovered label. */
    label: string;
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
      if (event.key === 'Escape') onDismiss();
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
  onpointerdown={onDismiss}
  oncontextmenu={(event) => {
    event.preventDefault();
    onDismiss();
  }}
></div>

<div class="ring" role="menu" aria-label="Insert">
  {#each placed as { action, x, y }, index (action.id)}
    <button
      class="lozenge bubble"
      role="menuitem"
      type="button"
      aria-label={action.label}
      style="left: {x}px; top: {y}px; animation-delay: {index * 28}ms"
      onpointerenter={() => (hovered = action.id)}
      onpointerleave={() => (hovered = undefined)}
      onclick={() => onChoose(action.id)}
    >
      <svg viewBox="0 0 24 24" aria-hidden="true" focusable="false">
        <path d={PIXEL_ICONS[action.icon]} />
      </svg>
    </button>
  {/each}

  {#if hovered}
    <span class="label" style="left: {centre.x}px; top: {centre.y}px">
      {actions.find((action) => action.id === hovered)?.label}
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

  .bubble {
    position: absolute;
    /* Placed by its centre rather than its corner: the ring is described in
       centres, and translating here keeps the arithmetic above honest. */
    transform: translate(-50%, -50%);
    pointer-events: auto;

    display: grid;
    place-items: center;
    /* 40px rather than the scale's 32: the glyphs are pixel art on a 24-unit
       grid and are only sharp at 24px, so the bubble has to be large enough to
       carry one at its own size. Off the scale deliberately — see DESIGN. */
    width: 40px;
    height: 40px;
    color: var(--control-glyph);

    animation: bubble-in var(--dur-quick) var(--ease-out) backwards;
  }

  .bubble:hover {
    color: var(--ink-primary);
  }

  /* Arrives from the centre, staggered, one after another — the founder's own
     description of this menu. Transform and opacity only; never blur. */
  @keyframes bubble-in {
    from {
      opacity: 0;
      transform: translate(-50%, -50%) scale(0.4);
    }
    to {
      opacity: 1;
      transform: translate(-50%, -50%) scale(1);
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

  /* The hovered action's name, in the middle of the ring — the one place
     nothing else occupies, and the only way a ring of glyphs says what it
     does without a legend around it. */
  .label {
    position: absolute;
    transform: translate(-50%, -50%);
    padding: 2px var(--space-2);
    border-radius: var(--radius-chip);
    background: var(--surface-solid);
    border: 1px solid var(--rule);
    color: var(--ink-primary);
    font-size: var(--font-size-caption);
    line-height: var(--line-height-ui);
    white-space: nowrap;
  }
</style>
