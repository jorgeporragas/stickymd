<script lang="ts">
  import { chordFrom, describeChord, formatChord, type ChordFormat } from '../keys';

  interface Props {
    /** The chord as stored, in `format`'s dialect. */
    value: string;
    /** Which dialect to read and write — see `src/lib/keys.ts`. */
    format: ChordFormat;
    /** For screen readers: what this shortcut does. */
    label: string;
    /** Called with the new chord, in the same dialect. */
    onChange: (chord: string) => void;
  }

  let { value, format, label, onChange }: Props = $props();

  /** Whether this control is currently waiting for a keypress. */
  let listening = $state(false);

  /** Why the last keypress was refused, if it was. */
  let problem = $state<string | undefined>(undefined);

  function start(): void {
    listening = true;
    problem = undefined;
  }

  function stop(): void {
    listening = false;
    problem = undefined;
  }

  function capture(event: KeyboardEvent): void {
    if (!listening) return;

    // Everything, including Tab and Enter: while this control is listening it
    // is a keyboard capture, not a form field, and a Tab that moved focus
    // instead of being recorded would make Tab unbindable.
    event.preventDefault();
    event.stopPropagation();

    if (event.key === 'Escape') {
      stop();
      return;
    }

    const result = chordFrom(event);

    if (typeof result === 'string') {
      // Stay listening. The refusals are all "that one, but not that one" —
      // closing on a refusal would make the user re-open the control to try
      // again for something they were one key away from.
      problem = result;
      return;
    }

    const chord = formatChord(result, format);
    listening = false;
    problem = undefined;

    if (chord !== value) onChange(chord);
  }
</script>

<!--
  A shortcut you press rather than type.

  It replaced a text field that took `Mod-Shift-x` as free text and handed it
  straight to CodeMirror, which ignores what it cannot parse — so a typo made a
  shortcut that never fired and never said why (SMD-074). Capturing the keypress
  removes the class of error rather than reporting it: there is no longer a way
  to express a chord that does not exist.

  It still refuses things, and each refusal is a chord that would have been
  accepted as text and then silently never worked — a bare letter, a lone
  modifier, and Ctrl+Alt, which is AltGr on most non-US layouts.
-->
<div class="chord-input">
  <button
    type="button"
    class="field"
    class:listening
    aria-label={listening ? `Press a shortcut for ${label}` : `${label}: ${describeChord(value, format)}. Change it.`}
    onclick={start}
    onkeydown={capture}
    onblur={stop}
  >
    {#if listening}
      <span class="prompt">Press a shortcut…</span>
    {:else}
      <span class="chord">{describeChord(value, format)}</span>
    {/if}
  </button>

  {#if problem}
    <p class="problem" role="status">{problem}</p>
  {/if}
</div>

<style>
  .chord-input {
    display: grid;
    justify-items: end;
    gap: var(--space-1);
  }

  /*
    A button rather than an input, because it is not text being edited: it
    takes one keypress and shows one value. An input would carry a caret, a
    selection and a paste target, none of which mean anything here.
  */
  .field {
    min-width: 12ch;
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-chip);
    border: 1px solid var(--rule);
    background: var(--surface-solid);
    color: var(--ink-primary);
    font-family: var(--font-mono);
    font-size: var(--font-size-caption);
    line-height: var(--line-height-ui);
    text-align: center;
    cursor: pointer;
  }

  .field:hover {
    border-color: var(--rim-neutral);
  }

  /* Listening is a state worth seeing across the room: the control is holding
     the keyboard, and every other shortcut in the window is inert until it
     stops. */
  .field.listening {
    border-color: var(--signal-engaged);
    color: var(--ink-secondary);
  }

  .prompt {
    font-family: var(--font-content);
  }

  .problem {
    margin: 0;
    max-width: 28ch;
    text-align: right;
    color: var(--signal-danger);
    font-size: var(--font-size-caption);
    line-height: var(--line-height-ui);
  }
</style>
