<script lang="ts">
  interface Props {
    /** Why the note is not saving. Undefined when it is saving fine. */
    reason: string | undefined;
    /** Try the write again. */
    onRetry: () => void;
  }

  let { reason, onRetry }: Props = $props();
</script>

<!--
  Shown only when a write has failed.

  It does not recede with the rest of the chrome, and that is deliberate:
  docs/DESIGN.md principle 2 stops receding at chrome that is carrying
  information, and "what is on screen is not what is on disk" is the most
  important thing this window can say. Colour is doing the work here rather
  than decorating — the note is at risk — which is the one case ADR-025 allows.

  Clicking it retries. The text is still queued, so a retry is free.
-->
{#if reason}
  <button
    class="lozenge lit trouble"
    type="button"
    title={`${reason} Click to try again.`}
    aria-label={`${reason} Click to try again.`}
    onclick={onRetry}
  >
    <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
      <path d="M6 3.2V6.6" />
      <path d="M6 8.4v0.1" />
    </svg>
  </button>
{/if}

<style>
  .trouble {
    display: grid;
    place-items: center;
    width: var(--space-4);
    height: var(--space-4);
    color: var(--signal-danger);
  }

  /* Size, stacking and colour come from `.lozenge.lit`. */
  .trouble svg {
    stroke-width: 2.2;
    stroke-linecap: round;
  }
</style>
