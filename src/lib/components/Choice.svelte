<script lang="ts" generics="T extends string">
  import Toggle from './Toggle.svelte';

  interface Props {
    /** The answers, in the order they are shown. */
    options: readonly { value: T; label: string }[];
    /** The answer currently held. */
    value: T | undefined;
    /** What the question is, for screen readers. The visible label is the
        field it sits in. */
    label: string;
    onChange: (value: T) => void;
  }

  let { options, value, label, onChange }: Props = $props();
</script>

<!--
  One question with a few answers: a row of bubbles, each captioned.

  The captions are what make it a choice rather than several unrelated
  switches. An unlabelled row of identical discs is a puzzle, and in a choice
  which one is which cannot be inferred from position.

  Each bubble is a `Toggle` with `role="radio"`, so pressing the answer already
  held does nothing — you cannot deselect your way into having no answer.
-->
<div class="choice" role="radiogroup" aria-label={label}>
  {#each options as option (option.value)}
    <span class="option">
      <Toggle
        on={value === option.value}
        label={option.label}
        role="radio"
        onChange={() => onChange(option.value)}
      />
      <span class="caption">{option.label}</span>
    </span>
  {/each}
</div>

<style>
  .choice {
    display: flex;
    gap: var(--space-3);
  }

  .option {
    display: grid;
    justify-items: center;
    gap: var(--space-1);
  }

  .caption {
    font-size: var(--font-size-caption);
    line-height: var(--line-height-ui);
    color: var(--ink-secondary);
  }
</style>
