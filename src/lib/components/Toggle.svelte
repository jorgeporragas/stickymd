<script lang="ts">
  interface Props {
    /** Whether the setting is on. */
    on: boolean;
    /** What the toggle is for, for screen readers. The visible label is the
        field it sits in. */
    label: string;
    onChange: (value: boolean) => void;
  }

  let { on, label, onChange }: Props = $props();
</script>

<!--
  One seated bubble that lights when the setting is held — the same control the
  always-on-top pin is, doing the same job.

  It was a sliding switch first, with the bubble as its knob. That was one
  component too many: a switch has a track, a travel distance and an end stop
  to get wrong, and the knob overshot its track. The bubble already says
  on-or-off in this application's own language, and it cannot overshoot
  anything.

  Lit is `--signal-engaged` with the check in that hue taken most of the way to
  black, as on every other lit lozenge (ADR-028).
-->
<button
  class="lozenge toggle"
  class:on
  type="button"
  role="switch"
  aria-checked={on}
  aria-label={label}
  onclick={() => onChange(!on)}
>
  {#if on}
    <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
      <path d="M3.2 6.3 5.1 8.2 8.9 4.1" />
    </svg>
  {/if}
</button>

<style>
  .toggle {
    flex: 0 0 auto;
    display: grid;
    place-items: center;
    width: var(--space-5);
    height: var(--space-5);
    border-color: var(--rim-control);
    --lozenge-fill: var(--gloss-control);
    --lozenge-pressed: var(--gloss-pressed);
    color: var(--control-glyph);
  }

  .toggle.on {
    border-color: var(--swatch-rim);
    color: var(--signal-engaged);
    --lozenge-fill: var(--gloss-tinted);
    --lozenge-pressed: var(--gloss-tinted-pressed);
  }

  .toggle svg {
    /* Above the specular, as on every other seated control: at this size a
       highlight across the top of a glyph is the difference between reading it
       and guessing. */
    position: relative;
    z-index: 1;
    width: 11px;
    height: 11px;
    stroke: color-mix(in oklab, var(--signal-engaged) 25%, var(--rim-shade));
    stroke-width: 1.8;
    stroke-linecap: round;
    stroke-linejoin: round;
    fill: none;
  }
</style>
