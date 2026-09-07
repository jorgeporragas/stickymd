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
      <path d="M2.4 3.5h7.2M4.8 3.5V2.2h2.4v1.3M3.5 3.5l.5 5.8h4l.5-5.8" />
    </svg>
  </button>
</div>

<style>
  .row {
    display: flex;
    align-items: stretch;
    gap: var(--space-1);
    border-radius: var(--radius-control);
    transition: background-color var(--dur-quick) var(--ease-out);
  }

  .row:hover {
    background: var(--control-hover);
  }

  /* The press is a colour, not a scale. Scaling the row would resample its
     text for the length of the transition, and a note's title is the one thing
     in the hub that has to stay readable. */
  .row:has(.open:active) {
    background: var(--control-active);
    transition-duration: var(--dur-instant);
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

  /*
    Flat, deliberately. It wore the seated treatment briefly and the founder
    read it as belonging to the same family as the window's controls, which it
    should not: those are the note's own chrome, and this acts on a row in a
    list. A secondary control, and secondary here means unlit.

    It still answers a press — pressable and bezelled are different things.
  */
  .delete {
    flex: 0 0 auto;
    display: grid;
    place-items: center;
    width: var(--space-5);
    height: var(--space-5);
    margin: var(--space-2) var(--space-2) var(--space-2) 0;
    align-self: center;
    padding: 0;
    border: 0;
    border-radius: var(--radius-chip);
    background: transparent;
    color: var(--ink-muted);
    cursor: pointer;

    opacity: 0;
    transition:
      opacity var(--dur-quick) var(--ease-out),
      color var(--dur-quick) var(--ease-out),
      background-color var(--dur-quick) var(--ease-out);
  }

  /* Revealed by hovering the row, in CSS rather than through state. A pointer
     handler on the row would need an ARIA role it does not deserve — the row
     is not the control, the buttons inside it are. */
  .row:hover .delete,
  .delete:focus-visible {
    opacity: 1;
  }

  /* The one place a hue is doing work: this control takes a note away, and
     red is the reading nobody has to learn. Everything else in the interface
     is ink. */
  .delete:hover {
    color: var(--signal-danger);
    background: var(--control-hover);
  }

  /* The press, without a bezel to do it: the fill firms up and the transition
     shortens so it lands under the finger rather than catching up with it. */
  .delete:active {
    background: var(--control-active);
    transition-duration: var(--dur-instant);
  }

  .delete svg {
    width: 12px;
    height: 12px;
    stroke: currentColor;
    stroke-width: 1.8;
    stroke-linecap: round;
    stroke-linejoin: round;
    fill: none;
  }
</style>
