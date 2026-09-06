<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    label: string;
    /** One line saying what the setting does, or what it costs. Optional —
        a setting whose label says everything does not need explaining. */
    note?: string;
    /** Shown under the field in `--signal-danger` when something is wrong. */
    problem?: string;
    /** The control itself. */
    control: Snippet;
  }

  let { label, note, problem, control }: Props = $props();
</script>

<!--
  One row of the settings window: what the setting is on the left, the control
  on the right, and anything that went wrong underneath.

  The problem line lives here rather than in each control because every setting
  can fail the same way — a shortcut another application holds, a folder that
  has gone — and a window that reports failures in a different place each time
  teaches the user to look in several places.
-->
<div class="field">
  <div class="row">
    <div class="text">
      <span class="label">{label}</span>
      {#if note}<span class="note">{note}</span>{/if}
    </div>
    {@render control()}
  </div>

  {#if problem}
    <p class="problem" role="status">{problem}</p>
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-3) 0;
    border-bottom: 1px solid var(--rule);
  }

  .row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  .text {
    flex: 1 1 auto;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .label {
    font-size: var(--font-size-body);
    line-height: var(--line-height-ui);
    color: var(--ink-primary);
  }

  .note {
    font-size: var(--font-size-caption);
    line-height: var(--line-height-ui);
    color: var(--ink-muted);

    /* A path can be longer than the window. It wraps rather than being cut,
       because half a path is not an answer to "where are my notes". */
    overflow-wrap: anywhere;
  }

  .problem {
    margin: 0;
    font-size: var(--font-size-caption);
    line-height: var(--line-height-ui);
    color: var(--signal-danger);
  }
</style>
