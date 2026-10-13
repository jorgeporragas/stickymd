# Scramble

Typographic scrambling: text that settles into itself over `--dur-*`-scale time, left to right. Takes `text`, `durationMs`.

Only ever over chrome, never note content — scrambling something being edited would mean rewriting glyphs in the buffer. It reflows nothing, because the display face is monospaced. The pool is punctuation rather than letters: letters mid-scramble read as words that are not there. `aria-label` carries the real text throughout. `prefers-reduced-motion` is honoured in JavaScript, since the global CSS rule cannot reach a rAF loop.
