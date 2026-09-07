<script lang="ts">
  import { onMount } from 'svelte';

  export interface RadialAction {
    id: string;
    /** What it does, for screen readers and the hovered label. */
    label: string;
    /** Which glyph to draw. Kept to a named set so the paths live here. */
    icon: 'note' | 'hub' | 'settings' | 'pin';
    /** Whether the action is currently engaged — the pin, when pinned. */
    engaged?: boolean;
  }

  interface Props {
    actions: RadialAction[];
    /** Where the pointer was, in client coordinates. */
    at: { x: number; y: number };
    onChoose: (id: string) => void;
    onDismiss: () => void;
  }

  let { actions, at, onChoose, onDismiss }: Props = $props();

  /** Distance from the centre to each bubble. */
  const RADIUS = 52;
  /** Half a bubble plus a little, so none of the ring meets the window edge. */
  const MARGIN = 26;

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

  Everything in it is an action that already exists elsewhere — this is a
  faster way to reach them, not a second set of capabilities. That is what
  keeps it cheap to change: which actions are in the ring is one array.

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

<div class="ring" role="menu" aria-label="Note actions">
  {#each placed as { action, x, y }, index (action.id)}
    <button
      class="lozenge bubble"
      class:engaged={action.engaged}
      class:lit={action.engaged}
      role="menuitem"
      type="button"
      aria-label={action.label}
      style="left: {x}px; top: {y}px; animation-delay: {index * 28}ms"
      onpointerenter={() => (hovered = action.id)}
      onpointerleave={() => (hovered = undefined)}
      onclick={() => onChoose(action.id)}
    >
      <svg viewBox="0 0 16 16" aria-hidden="true" focusable="false">
        {#if action.icon === 'note'}
          <path d="M4 2.5h8v11H4z" />
          <path d="M6 5.5h4M6 8h4M6 10.5h2.5" />
        {:else if action.icon === 'hub'}
          <path d="M2.5 3.5h11M2.5 8h11M2.5 12.5h11" />
        {:else if action.icon === 'settings'}
          <path d="M2.5 5h11M2.5 11h11" />
          <path d="M6 5v0.01M10.5 11v0.01" />
        {:else}
          <path d="M6.2 2h3.6l-.5 3.6 2.2 2.2v1.2H4.5V7.8l2.2-2.2z" />
          <path d="M8 9v5" />
        {/if}
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
    width: var(--space-8);
    height: var(--space-8);
    border-color: var(--rim-control);
    color: var(--control-glyph);

    animation: bubble-in var(--dur-quick) var(--ease-out) backwards;
  }

  .bubble.engaged {
    color: var(--signal-engaged);
  }

  .bubble:hover {
    color: var(--ink-primary);
  }

  .bubble.engaged:hover {
    color: var(--signal-engaged);
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

  /* Size and stacking come from `.lozenge`. */
  .bubble svg {
    stroke: currentColor;
    stroke-width: 1.2;
    stroke-linecap: round;
    stroke-linejoin: round;
    fill: none;
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
