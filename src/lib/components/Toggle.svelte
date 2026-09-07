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

  Lit is `--signal-engaged` with a dot in that hue taken most of the way to
  black, as on every other lit lozenge (ADR-028). A dot rather than a check
  because a check is an *action* being confirmed, and this is a state being
  held — the same reason the pin draws a pin rather than a tick.
-->
<button
  class="lozenge toggle"
  class:lit={on}
  type="button"
  role="switch"
  aria-checked={on}
  aria-label={label}
  onclick={() => onChange(!on)}
>
  {#if on}
    <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
      <circle cx="6" cy="6" r="2.1" />
    </svg>
  {/if}
</button>

<style>
  .toggle {
    flex: 0 0 auto;
    display: grid;
    place-items: center;
    /* The diameter every other bubble in the application uses. */
    width: var(--space-4);
    height: var(--space-4);
    color: var(--control-glyph);
  }

  /* Only the hue. The fill, rim and glyph follow from `.lit`, in src/app.css. */
  .toggle.lit {
    color: var(--signal-engaged);
  }
</style>
