<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    /** The text this settles into. */
    text: string;
    /** How long the whole settle takes. */
    durationMs?: number;
  }

  let { text, durationMs = 420 }: Props = $props();

  // Capturing the initial value is the intent: the scramble owns what is drawn
  // from mount onwards, and these titles do not change while the window is
  // open. A title that could change would want a `$derived` reset instead.
  // svelte-ignore state_referenced_locally
  let shown = $state(text);

  /**
   * The pool scrambled characters are drawn from.
   *
   * Deliberately not letters. Letters mid-scramble read as words that are not
   * there, and a title that appears to say something else for a third of a
   * second is worse than one that appears to be assembling.
   */
  const POOL = '#*+=-<>/\\|_[]{}';

  const REDUCED = '(prefers-reduced-motion: reduce)';

  onMount(() => {
    // Honoured here rather than in CSS: this is a JavaScript effect, so the
    // global rule that collapses transition durations cannot reach it, and an
    // effect that ignores the setting is worse than one that does not exist.
    if (window.matchMedia(REDUCED).matches) return;

    const started = performance.now();
    let frame = 0;

    const tick = (now: number) => {
      const progress = Math.min((now - started) / durationMs, 1);

      // Characters settle left to right, so the word resolves rather than
      // stopping all at once.
      const settled = Math.floor(progress * text.length);

      shown = text
        .split('')
        .map((character, index) => {
          if (index < settled || character === ' ') return character;
          return POOL[Math.floor(Math.random() * POOL.length)];
        })
        .join('');

      if (progress < 1) {
        frame = requestAnimationFrame(tick);
      } else {
        shown = text;
      }
    };

    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  });
</script>

<!--
  Typographic scrambling, settling into the real text.

  Only ever over chrome — never note content. Scrambling something the user is
  editing would mean rewriting glyphs in the buffer, and markdown stays in the
  buffer at all times (CLAUDE.md, 'Editor').

  It reflows nothing: the display face is monospaced, so every substituted
  character occupies exactly the width of the one it replaces. That was the
  property Handjet was chosen for, and Departure Mono has it for a different
  reason.

  `aria-label` carries the real text throughout, so a screen reader is never
  read the scramble.
-->
<span aria-label={text}><span aria-hidden="true">{shown}</span></span>
