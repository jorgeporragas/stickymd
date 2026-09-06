<script lang="ts">
  interface Props {
    title: string;
    /** How long ago it changed, already in words. */
    when: string;
    onOpen: () => void;
    onDelete: () => void;
  }

  let { title, when, onOpen, onDelete }: Props = $props();
</script>

<!--
  The row is the button. Making the whole row clickable and hanging a second
  button inside it would nest one control in another; instead the delete
  control is a sibling and the row sits behind it.
-->
<div class="row">
  <button class="open" type="button" onclick={onOpen}>
    <span class="title">{title}</span>
    <span class="when">{when}</span>
  </button>

  <button
    class="delete"
    type="button"
    aria-label="Delete {title}"
    onclick={onDelete}
  >
    <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
      <path d="M2.6 3.6h6.8M4.9 3.6V2.5h2.2v1.1M3.6 3.6l.4 5.6h4l.4-5.6" />
    </svg>
  </button>
</div>

<style>
  .row {
    display: flex;
    align-items: stretch;
    gap: var(--space-1);
    border-radius: var(--radius-control);
  }

  .row:hover {
    background: var(--control-hover);
  }

  .open {
    flex: 1 1 auto;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: var(--space-2) var(--space-3);
    border: 0;
    background: transparent;
    text-align: left;
    cursor: pointer;
    font-family: inherit;
    color: var(--ink-primary);
  }

  .title {
    font-size: var(--font-size-body);
    line-height: var(--line-height-ui);

    /* One line. A note's first line can be a paragraph. */
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .when {
    font-size: var(--font-size-caption);
    color: var(--ink-muted);
  }

  .delete {
    flex: 0 0 auto;
    display: grid;
    place-items: center;
    width: var(--space-6);
    margin: var(--space-2) var(--space-2) var(--space-2) 0;
    padding: 0;
    border: 0;
    border-radius: var(--radius-chip);
    background: transparent;
    color: var(--ink-muted);
    cursor: pointer;

    opacity: 0;
    transition:
      opacity var(--dur-quick) var(--ease-out),
      color var(--dur-quick) var(--ease-out);
  }

  /* Revealed by hovering the row, in CSS rather than through state. A pointer
     handler on the row would need an ARIA role it does not deserve — the row
     is not the control, the buttons inside it are. */
  .row:hover .delete,
  .delete:focus-visible {
    opacity: 1;
  }

  .delete:hover {
    color: var(--ink-primary);
  }

  .delete svg {
    width: 12px;
    height: 12px;
    stroke: currentColor;
    stroke-width: 1.2;
    stroke-linecap: round;
    stroke-linejoin: round;
    fill: none;
  }
</style>
