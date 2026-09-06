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
  A switch, with the seated lozenge as its knob — the same object the window's
  controls are made of, at the same diameter, so a setting reads as something
  you press rather than as a form control that wandered in.

  On is `--signal-engaged`, which is what a held setting looks like everywhere
  else in the application (ADR-028).
-->
<button
  class="track"
  class:on
  type="button"
  role="switch"
  aria-checked={on}
  aria-label={label}
  onclick={() => onChange(!on)}
>
  <span class="lozenge knob"></span>
</button>

<style>
  .track {
    flex: 0 0 auto;
    display: flex;
    align-items: center;
    width: calc(var(--space-4) * 2 + var(--space-1));
    padding: 2px;
    border: 1px solid var(--rule);
    border-radius: var(--space-4);
    background: var(--control-hover);
    cursor: pointer;
    transition: background-color var(--dur-quick) var(--ease-out);
  }

  .track.on {
    background: color-mix(in oklab, var(--signal-engaged) 35%, transparent);
    border-color: var(--swatch-rim);
  }

  .knob {
    width: var(--space-4);
    height: var(--space-4);
    --lozenge-fill: var(--gloss-control);
    --lozenge-pressed: var(--gloss-pressed);
    border-color: var(--rim-control);

    /* Moves rather than fades: the knob is an object, and objects travel.
       See docs/DESIGN.md section 'Motion'. */
    transform: translateX(0);
    transition: transform var(--dur-quick) var(--ease-out);
  }

  .track.on .knob {
    --lozenge-fill: var(--gloss-tinted);
    --lozenge-pressed: var(--gloss-tinted-pressed);
    color: var(--signal-engaged);
    border-color: var(--swatch-rim);
    transform: translateX(calc(var(--space-4) + var(--space-1) - 2px));
  }
</style>
