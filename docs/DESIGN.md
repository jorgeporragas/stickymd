# DESIGN

> **Authoritative for:** the visual system — design principles, the token contract, typography, themes, motion, and the component inventory.
> **Never contains:** work state, or token values. Components and themes that do not exist, and anything carrying a completion mark, live in `docs/STATUS.md`. Every token's value lives in `src/lib/tokens/tokens.css`. Product truth lives in `MASTER.md`. Engineering imperatives live in `CLAUDE.md`.
> **Last verified:** 2026-09-06

---

## Principles

**1. The surface carries the personality. The content is Rams.**
Chrome, edges, glass, and motion carry it — colour does not, since ADR-025. Everything inside a note — the user's text — is quiet, neutral, and typographic. The app is beautiful *around* the writing, never on top of it.

The direction is Rams restraint against the drafted feel of a scratchpad. Two effects belong to that second half and are named here before they are built, so that nothing gets designed in a way that shuts them out: **typographic scrambling** and **line boil**.

**2. Chrome recedes.**
Controls withdraw when the window is not being used and return on pointer entry or focus. The text is the focal point. Personality lives in the controls; the controls are present only when wanted.

**3. Glass is a window property, not a CSS filter.**
The frosted surface comes from the OS compositor — Acrylic on Windows, vibrancy on macOS — applied to a transparent window from the Rust side. `backdrop-filter` cannot see the desktop behind a web view and is never used for the primary surface. CSS supplies the tint, grain, and edge highlight that sit *on top* of the compositor's blur.

**4. Solid is a first-class mode, not a fallback.**
Transparency is a system setting, and Windows disables it under battery saver. Some users will only ever see the Solid surface mode. It is designed to look deliberate, not degraded.

**5. Every colour is read through a token.**
No component declares a static colour, ever. This is what makes a theme a data file rather than a refactor. The pre-commit hook rejects a raw colour outside `src/lib/tokens/`.

**6. Tokens layer: primitive, then semantic.**
Primitives name values. Semantics name roles. Components read semantics only. When a semantic token starts serving two visually distinct purposes, it splits into two named tokens before the roles need to diverge.

**7. Motion moves objects. It never re-blurs them.**
Animate `transform` and `box-shadow`. See § Never Allowed.

**8. A theme is data.**
A theme is a complete set of token values and nothing else. Themes never replace components, ship CSS, or execute code.

---

## Typography

Three families, all SIL Open Font License, all vendored into `src/assets/fonts/` with their licence texts alongside as `OFL-<Family>.txt`. Nothing is fetched at runtime.

| Role | Face | Used for |
|---|---|---|
| Display | **Handjet** | App identity, hub headers, empty states. Never inside note content. |
| Content | **Geist** | Note body, all UI labels, everything functional. |
| Mono | **Martian Mono** | Fenced code blocks. |

**Handjet** is a variable face whose glyphs are built from repeated elements on a grid. `ELGR` controls how many elements compose each letter; `ELSH` morphs those elements between square and round. Degradation is therefore continuous, and is set through tokens rather than by choosing a file.

Handjet's advance widths are identical at every setting of `ELGR`, `ELSH` and `wght` — the grid it is built on fixes them. Animating its axes therefore reflows nothing, which is what makes degradation usable as motion rather than only as a static choice.

Weight is a token, not a literal. `--weight-emphasis` (600) is what `**bold**` renders as, and `--weight-heading` (650) is every heading level and the hub's table headers. Emphasis sits *below* heading on purpose: bold inside a paragraph is a change of voice, not a change of level, and Geist at 700 was reading as a second heading in the middle of a line.

**Handjet never appears inside a note.** That boundary is Principle 1 made concrete: display type is surface, and note content is content.

---

## Surface modes

sticky.md renders every window in one of two surface modes.

**Glass** — the window is transparent, the compositor blurs what is behind it, and CSS layers a near-colourless tint, a fine grain, and a top edge highlight over the result. The note reads as heavily frosted translucent paper.

**Solid** — the same geometry, type, and spacing, with an opaque surface in place of the blur. Selected by the user, or forced by the system when transparency effects are unavailable.

Both modes read from the same semantic tokens. A component never branches on surface mode; the tokens carry the difference.

---

## Token contract

This section is authoritative for which tokens exist and what role each plays. **It is not authoritative for their values** — those live in `src/lib/tokens/tokens.css`, which is the single source for every visual value in the application. A value written in both places is a value that will disagree with itself.

Components read semantic tokens only. A theme supplies a complete set of values for every semantic token below.

### Semantic — surface

| Token | Role |
|---|---|
| `--surface-paint` | What a surface is painted with. Resolves to the opaque surface or the tint over compositor blur, according to `data-surface` on the document root. **Components read this one** — they never read the two below directly, and never branch on the mode. |
| `--surface-tint` | The colour layered over the compositor blur in Glass mode |
| `--surface-solid` | The opaque surface in Solid mode |
| `--surface-grain-opacity` | Fine noise; keeps glass reading as frosted paper |
| `--surface-edge-highlight` | The gloss line along the top edge |
| `--surface-border` | The hairline containing the window. Resolves per surface mode: a light hairline catching the blur on glass, an ink hairline on solid, where a white line would be invisible. |

### Semantic — ink

| Token | Role |
|---|---|
| `--ink-primary` | Note content |
| `--ink-secondary` | UI labels, metadata |
| `--ink-muted` | Placeholder and inactive states |
| `--ink-syntax` | Markdown syntax characters when revealed |
| `--ink-on-accent` | Text on an accent-filled surface |

### Semantic — signals

| Token | Role |
|---|---|
| `--signal-danger` | A control that takes something away |
| `--focus-ring` | The focus indicator |
| `--selection` | The wash behind selected text |

**There is no brand colour** (ADR-025). The interface is ink on a tinted surface, and a hue appears only where colour is what tells the user what something does — traffic-light reading, because it needs no learning. Today that is one control: delete.

Amber and green are deliberately not defined. A colour with no job is how a palette turns decorative, and the moment one has a job it can be added with its contrast computed then.

Every signal value is computed, never picked: it clears 4.5:1 against the worst of the seven tints in its own theme, so it stays legible on any note.

A state is not a colour. "On" is shown by a filled chip — see the always-on-top pin — because a state you can see beats a state you have to have learned.

Note tints are not interface colour. They are the note's own paper, and ADR-025 leaves them untouched.

### Semantic — elevation

| Token | Role |
|---|---|
| `--shadow-lifted` | Something raised *inside* a window — a palette, a menu |

There is one shadow, and it is the only one there can be. A window's shadow at rest and while it is dragged belongs to the compositor: the surface fills the window exactly, so a shadow it casts falls outside the window and is never seen. Tokens for those two states existed and were removed rather than left to be reused for something they do not describe.

### Primitive families

Named in `src/lib/tokens/tokens.css`: neutrals, accent ramp, tint swatches (`--swatch-*` — the flat colour that *names* a tint, distinct from the translucent wash the tint paints with, because one token cannot be two values at once), duration (`--dur-*`), easing (`--ease-*`), type families and scale (`--font-*`, `--line-height-*`, `--tracking-*`), the Handjet axes (`--handjet-*`), radius (`--radius-*`), and spacing (`--space-*`).

Spacing and radius outside the scale are not permitted.

---

## Themes and tints

Two separate things, and keeping them separate is what lets both exist (ADR-024).

A **theme** is application-wide. It decides ink, accent, edges and shadows, and it is a complete set of values for every semantic token. A **tint** is per-note and decides only what that note's surface is painted with. A note keeps its tint in either theme.

Tint selectors are qualified by theme — `[data-theme='dark'][data-tint='sun']`. A tint block and a theme block carry equal specificity, so an unqualified tint would override the theme's surface entirely and a dark note would come back white.

**Every tint alpha is computed, never chosen.** The tint sits over the compositor's blur, so the ink has to hold 4.5:1 against it over a worst-case wallpaper. In the light theme a coloured tint needs *more* alpha than Clear, because it is darker than white. In the dark theme the worst case inverts to a white wallpaper. A tint whose alpha has not been checked is not a tint yet.

### Frost — the default

Colourless frosted glass. The tint is present only so that dark wallpapers cannot swallow the text; it is not perceived as a colour. Warm neutrals, so the surface never reads clinical, and a single aqua accent.

Values: `src/lib/tokens/tokens.css`, under `:root[data-theme='frost']`.

### Dark

The inverse of Frost: a faint black tint over the same compositor blur, with warm white ink. The roles are identical and only the values differ, which is what ADR-012 meant by a theme being data.

Values: `src/lib/tokens/tokens.css`, under `:root[data-theme='dark']`.

**Contrast is the binding constraint.** `--ink-primary` against `--surface-tint` over a worst-case wallpaper is what sets the tint's alpha. Body text holds at 4.5:1 or better in both surface modes and under every tint. A theme that cannot meet that is not shippable — and the dark theme's first sketched value could not: 0.45 measured 2.72:1, and it ships at 0.63.

---

## Note window

A square with rounded corners — the Post-It proportion, and the shape the app is recognised by.

**The corner radius is the platform's, not ours.** The compositor draws the frosted backdrop across the whole window rectangle, so rounding only what the web view paints leaves untinted acrylic showing in each corner. The window itself is rounded instead, and Windows picks that radius — `--radius-window` matches it rather than the other way round. A generous Post-It radius and compositor glass cannot both be had on Windows; glass won.

Default and expanded sizes are a toggle. Free resize is available between the minimum and the display bounds; the square is the default proportion, not a locked aspect ratio.

Dimensions are a window property, not a style: they live in `src-tauri/tauri.conf.json` under `app.windows`. JSON carries no comments, so this is the only pointer between the two — treat it as the reciprocal reference.

## The hub

A taller window listing every note, newest first. Its heading and its empty state are set in the display face; everything a note contains stays in the content face, which is Principle 1 applied across windows rather than only within one.

It is defined in `src-tauri/tauri.conf.json` with `create: false`, so its dimensions live in the same place as the note window's without a window being built at startup. There is only ever one hub: a second list of the same notes would be two things to keep in step for no gain.

---

## Application mark

An irregular blob with a spike at the top, lobes down the left, and two leg-like protrusions at the bottom. Rugged rather than smooth: the roughness is what reads as *sticky*, and a clean geometric blob does not.

`assets/icon/stickymd.svg` is the single source. Every platform size is generated from it with `npx tauri icon assets/icon/stickymd.svg`, which writes into `src-tauri/icons/`. Never hand-edit a generated size.

The silhouette is the identity. Any change to the mark is checked by rasterising to a real pixel grid at 16px — scaling the vector down cannot show what is lost, and 16px in a taskbar is where this shape spends most of its life.

## Motion

Motion is how the surface's personality is expressed without cost. Three rules govern all of it:

**Objects arrive, they do not fade in.** Something appearing moves into place at full glass throughout. It never fades up from nothing, which reads as an image loading rather than an object arriving.

**A window's own surface is the exception, and it cannot move at all.** It is pinned to the viewport because anything it does not cover, the compositor paints — a scaled-down surface shows a ring of raw acrylic, and a scaled-up one loses its rounded corners past the window's edge. Its outer shadow is unseeable for the same reason: it falls outside the window. The window's shadow is the compositor's own.

So a window arrives by the light on it rather than by moving: a sheen across the glass that fades as it settles, painted by an overlay and never by the surface's own alpha. Everything *inside* the surface — a palette, a menu, a row — may scale freely, because what is behind it is the window, not the desktop.

**A tint change is a colour change, not a fade.** The surface crossfades between two tints at full glass. The rule below bars fading a surface in or out; it does not bar it changing colour.

**Chrome crossfades; surfaces transform.** Small controls may fade — that is cheap and correct. Large glass surfaces never change alpha, and never animate blur: both force the compositor to recompute the frosting every frame.

**Text is not animated.** Inline rendering switches instantly when the cursor leaves a line. Softening it would mean animating a reflow on every cursor move, which makes the whole document shimmer as you arrow through it and costs a layout per frame. The two states are kept close instead: syntax marks are drawn in `--ink-syntax`, and styled text keeps its size and weight whether or not its marks are showing, so leaving a line changes the marks and nothing else.

**`prefers-reduced-motion` is honoured everywhere.** Under it, transforms collapse to instant state changes. Nothing is left mid-transition.

---

## Never Allowed

| Never | Instead |
|---|---|
| Animate blur radius | Switch between discrete blur states |
| Animate a window's opacity | Animate `transform` and `box-shadow` |
| Animate the alpha of a large glass surface | Animate the tint layer above it — a paint, not a re-blur |
| `backdrop-filter` for the primary window surface | OS compositor blur, applied from Rust |
| A static colour value in a component | A semantic token |
| Spacing or radius outside the scale | A scale token |
| Handjet inside note content | Geist |
| Text over glass without the tint layer beneath it | The tint is load-bearing for contrast, not decoration |
| One semantic token serving two visual roles | Split it into two named tokens |
| Fetching a font or any asset over the network | Vendored assets — MASTER veto 2 |
| Branching a component on surface mode or theme | Read the token; the theme carries the difference |
| Restating a token value in this file | Link to `src/lib/tokens/tokens.css` |

---

## Component Inventory

Complete and authoritative. **If a component is not listed here, it does not exist.** Before building any UI, read this list — reuse is only the default when it has been checked.

Every entry lands in the same commit as the component it describes.

| Component | Role | Notes |
|---|---|---|
| `WindowChrome` | The drag region and window controls for any window. Takes `revealed: boolean`, an optional `closeLabel`, and optionally `alwaysOnTop` and `onAlwaysOnTop` for the pin — omit those and no pin is rendered, which is how the hub uses it. | Implements Principle 2, with one deliberate exception: a pinned note keeps its pin visible even when the chrome recedes. A state you cannot see is a state you cannot trust — chrome recedes, chrome that is carrying information does not. Controls fade via `opacity`, permitted for small non-glass elements. |
| `TintPicker` | A note's colour. A swatch that opens the seven tints in a row, sitting with the other controls at the window's trailing edge, before the pin. The palette opens down and to the left, anchored to its right edge — the surface clips what leaves it. Its own surface is opaque `--surface-solid`, not the window's translucent paint, or the note's text reads through the swatches. It arrives over `--dur-settle` and leaves over `--dur-quick`: arriving is the palette presenting itself and is worth the time, while leaving is getting out of the way, and a dismissal as long as the arrival reads as lag. It stays mounted while it leaves and its own `animationend` says when it is gone — which is also what keeps reduced motion correct, since the collapsed duration still fires the event. Takes `revealed`, `tint`, `onTint`. | Like the pin, a note whose tint is not Clear keeps its swatch visible when the chrome recedes: the swatch is carrying information, and Principle 2 stops receding at that point. |
| `NoteRow` | One note in the hub: its title, when it changed, and a delete control. Takes `title`, `when`, `onOpen`, `onDelete`. | The row is not the control — the buttons inside it are. Delete is revealed by hovering the row in CSS rather than through state, which avoids putting a pointer handler on a `div` that would then need an ARIA role it does not deserve. |
| `Editor` | The note's markdown editing surface. Takes `value?: string` as initial source. Owns the CodeMirror instance and its lifecycle. | Content, not surface — quiet and typographic per Principle 1. No gutters, no active-line highlight, no border. Editor internals live in `src/lib/editor/`. |
