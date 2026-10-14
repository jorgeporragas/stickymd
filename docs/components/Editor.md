# Editor

The note's markdown editing surface. Takes `value?: string` as initial source, `onChange`, called with the full source whenever it changes, `keymap` for the window-level bindings it serves, `formatting` for the remappable chords, and `onReady`, which hands the window a way to run an editor command at a point — the insert menu's only route in. Owns the CodeMirror instance and its lifecycle.

Content, not surface — quiet and typographic per Principle 1. No gutters, no active-line highlight, no border. Editor internals live in `src/lib/editor/`.
