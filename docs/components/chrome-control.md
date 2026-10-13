# .chrome-control (`src/app.css`)

What makes a lozenge a control on a window's chrome: `--space-4`, the quiet glyph colour, the hover, and the receding that Principle 2 asks for. Glyphs are drawn on a 12-unit grid at stroke-width 2.

Shared the moment a second window grew one. `WindowChrome` renders its own controls *and* takes a `controls` snippet whose markup belongs to the calling window — and Svelte scopes styles per component, so a control passed in cannot reach anything the chrome declares. Without this the hub's new-note button would be three copied declarations that then drift. The one thing that stays in `WindowChrome` is the pin's refusal to recede, which is that chrome's alone.
