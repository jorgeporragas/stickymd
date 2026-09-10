# DESIGN

> **Authoritative for:** the visual system — design principles, the token contract, typography, themes, motion, and the component inventory.
> **Never contains:** work state, or token values. Components and themes that do not exist, and anything carrying a completion mark, live in `docs/STATUS.md`. Every token's value lives in `src/lib/tokens/tokens.css`. Product truth lives in `MASTER.md`. Engineering imperatives live in `CLAUDE.md`.
> **Last verified:** 2026-09-07

---

## Principles

**1. The surface is Aqua. The content is Rams.**
Chrome, edges, glass, and motion carry the personality. Everything inside a note — the user's text — is quiet, neutral, and typographic. The app is beautiful *around* the writing, never on top of it.

Aqua is not a contradiction of the Rams half, which is why the two names sit in one sentence (ADR-027): its chrome descends from Braun — neutral, restrained, hue concentrated in the small round things and almost nowhere else. That is also the colour rule, unchanged from ADR-025: a hue appears only where colour is what tells you what something does.

Two effects the founder named are still on the table and are recorded here so nothing gets designed in a way that shuts them out: **typographic scrambling** and **line boil**. Neither is built, and neither is settled against Aqua yet.

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
| Display | **Departure Mono** | App identity and the hub's title. Never inside note content, and not for sentences — an empty state is the application talking, and it takes the content face. |
| Content | **Geist** | Note body, all UI labels, everything functional. |
| Mono | **JetBrains Mono** | Fenced code blocks and inline code, at `--font-size-code` (13px). At that size it has the same x-height as the prose and runs 16% wider for the same sentence. It replaced Martian Mono, which ran 35% wider at the same height — the glyphs were never taller, there was just far more of them. One weight, no italic. |

**Departure Mono** is a monospaced pixel face drawn on a fixed grid (ADR-029). It ships one weight and no axes, so `font-synthesis: none` is set on the body: a synthesised bold thickens strokes off the grid the glyphs are drawn on, and it stops looking like pixels. Anything using the display face names its weight rather than inheriting a heading's default.

It replaced Handjet, whose `ELGR`/`ELSH` axes made degradation continuous and animatable. That is gone: degradation is a static property of this face.

`--tracking-display` is `-0.08em`, and negative by a lot because the face is monospaced: its letters sit on a fixed advance with the sidebearings a code face needs, not the ones a title wants. The value was measured rather than guessed — it renders the hub's title 15.2% narrower than the face sets it.

Every disc in the window is one diameter, `--space-4`, and a control's glyph is 9px — about 0.58 of it, which is the traffic lights' own proportion. The glyph is drawn *above* the specular: at nine pixels, a highlight across the top of one is the difference between reading it and guessing.

Weight is a token, not a literal. `--weight-emphasis` (600) is what `**bold**` renders as, and `--weight-heading` (650) is every heading level and the hub's table headers. Emphasis sits *below* heading on purpose: bold inside a paragraph is a change of voice, not a change of level, and Geist at 700 was reading as a second heading in the middle of a line.

**The display face never appears inside a note.** That boundary is Principle 1 made concrete: display type is surface, and note content is content.

---

## Surface modes

sticky.md renders every window in one of two surface modes.

**Glass** — the window is transparent, the compositor blurs what is behind it, and CSS layers a near-colourless tint, a fine grain, and a top edge highlight over the result. The note reads as heavily frosted translucent paper.

**Solid** — the same geometry, type, and spacing, with an opaque surface in place of the blur. Selected by the user, or forced by the system when transparency effects are unavailable.

Both modes read from the same semantic tokens. A component never branches on surface mode; the tokens carry the difference.

**Every window is built hidden and shows itself once it has painted.** A transparent window exists before its web view has drawn anything, and what fills that gap is the compositor's own backdrop — a grey pane that then fills in as the page arrives. Holding the *blur* back instead would only change the colour of the flash: the transparency is there from the first instant, and it is the paint that is missing.

`revealWindow` waits two animation frames, since one fires *before* the paint it is scheduled alongside, then hands over to `reveal_window` in Rust — which shows, focuses, and applies the surface **again**, because a backdrop set on a window nobody has shown yet is one the compositor has had no reason to compose. Rust also shows any window still hidden after two seconds: a front end that never runs would otherwise leave the application invisible, which is a worse failure than the flash this replaced.

**A window opens opaque and dissolves into glass.** Even shown at the right moment, the compositor's blur is not always composed by the first frame, and a window that painted its tint straight away showed the desktop through it *unblurred* for a fraction of a second. So every window paints Solid to begin with, whatever mode it is in, and `settleSurface` lets it become glass after `--dur-glass-hold`. A frame was not enough: the compositor takes longer to compose its backdrop than the dissolve takes to run, so the window was translucent again before there was anything behind it to be translucent against. The hold is a token for one reason — it is the number to change when the timing is wrong, in either direction. There is then no moment where a window is pretending to be transparent over something that is not yet frosted.

The dissolve is `.surface`'s own `background-color` transition. That is the tint layer being animated over a backdrop that does not change — the sanctioned case in **Never Allowed** below, not the forbidden one: nothing is re-blurred per frame.

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
| `--ink-syntax` | Markdown syntax characters when revealed. **Holds 3:1 against every tint on its own theme's design basis** — a white backdrop on Frost (ADR-018), a mid one on Dark (ADR-036) — and stays roughly a third of `--ink-primary`'s contrast, so it reads as markup rather than as text. It was below 2:1 everywhere and 1.02:1 at worst until SMD-083; quiet is the intent, invisible was a bug |
| `--ink-on-accent` | Text on an accent-filled surface |

### Semantic — signals

| Token | Role |
|---|---|
| `--signal-danger` | A control that takes something away |
| `--signal-engaged` | A control holding a setting on — the always-on-top pin. **Follows the note's colour:** on a clear note it is the green the application mark is made of, and on a tinted one it is that same colour with only its hue turned to the note's, so a blue note gets a stronger blue. Identical lightness and chroma in every case, so "selected" carries the same weight whatever the note is |
| `--scrollbar-thumb`, `--scrollbar-thumb-hover` | The scrollbar's thumb. Its track is always transparent: the default is an opaque strip, which on a translucent window reads as a pale rectangle laid over the glass. Each theme also declares its colour scheme, which is what stops the platform drawing a light scrollbar on a dark window — the colours alone do not fix that. |
| `--code-surface` | The panel behind a fenced block. Dark in both themes; darker than the note on Frost, lighter than it on Dark, because on a dark surface a darker panel reads as a hole. |
| `--code-ink`, `--code-muted`, `--code-keyword`, `--code-string`, `--code-number`, `--code-type` | How a fenced block's tokens are told apart. The one place a hue is not a signal — and still not decoration, since inside a block colour is doing work. Drawn from the `--tide-*` ramp so code reads as part of this application. See ADR-030. |
| `--gloss-tinted` | The fill of a disc that carries a colour, mixed from `currentColor` — a primitive, since it says the same thing in every theme |
| `--gloss-neutral` | The fill of a disc that carries none: the window's own controls |
| `--gloss-seat` | The shade inside a disc's lower edge, and the contact shadow under it |
| `--gloss-specular` | The highlight across its top |
| `--gloss-blur` | How hard a control frosts what is behind it. Controls are translucent (ADR-039), so a bubble in the ring has the note's own text under it — this is the one `backdrop-filter` in the application, and it is correct because the app's own content is all that filter can ever see |
| `--swatch-rim` | A swatch's edge — its own hue darkened, via `currentColor`, never a neutral border. A control carrying no tint overrides it with `--rule`. |
| `--focus-ring` | The focus indicator |
| `--selection` | The wash behind selected text |

The palette is one ramp, `--tide-100` to `--tide-1000`, running from a yellow-green through teal to a deep blue (ADR-028). It is a primitive family: a stop nothing reaches for is a value, not a colour in the interface.

**There is no brand colour** (ADR-025). The interface is ink on a tinted surface, and a hue appears only where colour is what tells the user what something does — traffic-light reading, because it needs no learning. Today that is one control: delete.

Amber and green are deliberately not defined. A colour with no job is how a palette turns decorative, and the moment one has a job it can be added with its contrast computed then.

Every signal value is computed, never picked: it clears 4.5:1 against the worst of the seven tints in its own theme, so it stays legible on any note.

**A note window's controls wear the note's colour.** Disc, rim and glyph all derive from the tint, so the chrome belongs to the note rather than sitting on it. Untinted, they resolve to the neutral disc. One recipe covers both themes, because the disc carries its own hue and the glyph is derived from it.

A state can be a colour when the state is the thing the control exists to report. The always-on-top pin lights to `--signal-engaged` when held, and stops receding — a pin you cannot see is a note you do not know is floating. It lights by taking the hue as `color`, feeding the same fill and rim every tint swatch uses, so it stays one kind of object rather than becoming a second kind of control.

**Which hue depends on the note.** A fixed green reads well on a clear note and like a sticker on a coloured one — the founder's observation, and the fix is not a different colour but the same one turned: each tint's engaged value is `--tide-400` at its own lightness and chroma with the hue rotated to that tint's. The glyph on top clears 8.4:1 at worst across the six. The comparison that matters is against what the green already did on the notes it already sat on: 1.86:1 on a clear Frost note against 1.74–1.86 for the six, so nothing is dimmer than it was. On Dark it is lower — 3.4–3.8 against the green's 9.0 — because a tinted dark note is far lighter than the near-black clear one, and 3:1 is the floor a control of this kind has to hold.

Note tints are not interface colour. They are the note's own paper, and ADR-025 leaves them untouched.

### Semantic — elevation

| Token | Role |
|---|---|
| `--shadow-lifted` | Something raised *inside* a window — a palette, a menu |

There is one shadow, and it is the only one there can be. A window's shadow at rest and while it is dragged belongs to the compositor: the surface fills the window exactly, so a shadow it casts falls outside the window and is never seen. Tokens for those two states existed and were removed rather than left to be reused for something they do not describe.

### Primitive families

Named in `src/lib/tokens/tokens.css`: neutrals, the `--tide-*` palette ramp, the signal reds, tint swatches (`--swatch-*` — the flat colour that *names* a tint, distinct from the translucent wash the tint paints with, because one token cannot be two values at once), duration (`--dur-*`), easing (`--ease-*`), type families, scale and weight (`--font-*`, `--line-height-*`, `--tracking-*`, `--weight-*`), radius (`--radius-*`), and spacing (`--space-*`).

Spacing and radius outside the scale are not permitted.

---

## Themes and tints

Two separate things, and keeping them separate is what lets both exist (ADR-024).

A **theme** is application-wide. It decides ink, accent, edges and shadows, and it is a complete set of values for every semantic token. A **tint** is per-note and decides only what that note's surface is painted with. A note keeps its tint in either theme.

Tint selectors are qualified by theme — `[data-theme='dark'][data-tint='sun']`. A tint block and a theme block carry equal specificity, so an unqualified tint would override the theme's surface entirely and a dark note would come back white.

**The dark tints are sized against a mid backdrop, the light ones against white** (ADR-036). A light theme composites against white anyway, so its worst case *is* its ordinary case. A dark theme's is not: sizing for a white wallpaper produced tints so opaque they stopped reading as a window. Dark tints hold 4.5:1 over a mid backdrop and never drop below 3:1 over white — a stated relaxation, whose price is that a dark note over a bright wallpaper is legible but under AA.

**Every tint alpha is computed, never chosen.** The tint sits over the compositor's blur, so the ink has to hold 4.5:1 against it over a worst-case wallpaper. In the light theme a coloured tint needs *more* alpha than Clear, because it is darker than white. In the dark theme the worst case inverts to a white wallpaper. A tint whose alpha has not been checked is not a tint yet.

**The stored preference is one of three: Frost, Dark, or *system*.** Only two of them are themes — `system` is resolved per window, in `src/lib/state/theme.ts`, because the operating system's setting is something a window can ask for and be told about while Rust would have to ask a window to find out. Rust stores the string and never interprets it. A window listening for system changes stops listening the moment the preference stops being `system`, or it would repaint over a choice the user had since made explicitly.

The tray's Dark check mark means *explicitly dark*, not *currently dark*: it cannot resolve `system` for the same reason. The settings window is where the three-way lives, and it says which is on.

### Frost — the default

Colourless frosted glass. The tint is present only so that dark wallpapers cannot swallow the text; it is not perceived as a colour. Warm neutrals, so the surface never reads clinical, and no accent at all — since ADR-025 the only hues are the signals and the notes' own tints.

Values: `src/lib/tokens/tokens.css`, under `:root[data-theme='frost']`.

### Dark

Its controls wear the **bezel** rather than the gloss (ADR-035) — flat, seated by a bright top edge and a shade below. A glossy disc needs light to be glossy about, and a dark window has none to give it. This is a theme changing values only: the controls are the same objects, and none of them knows which theme it is in.

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

**The list is live.** Rust emits `notes-changed` whenever a note is written or deleted and the hub re-reads on it, so a note written in another window moves in a hub that is merely open rather than one that is focused. It re-reads on focus as well: the event covers a change this application made, focus covers one it did not — a note edited elsewhere, or dropped into the folder. Newest first means a note being typed in climbs to the top while you watch, which is the list being true rather than restless.

It is defined in `src-tauri/tauri.conf.json` with `create: false`, so its dimensions live in the same place as the note window's without a window being built at startup. There is only ever one hub: a second list of the same notes would be two things to keep in step for no gain.

---

## Application mark

The application's own seated lozenge, in `--tide-400`, carrying the letter **S** from Departure Mono. Source: `assets/icon/stickymd.svg` — the vector, never a generated size.

Two things about it are deliberate rather than convenient. The glyph is **rectangles, not text**: its grid was sampled from the face rendered in the running application, so the mark needs no font present to draw. And every colour is `--tide-400` mixed in oklab, the same way `color-mix` does it in the stylesheet, so the mark is the same green as the controls rather than nearly.

The rim is the one proportion that does *not* follow the interface. A control's rim is a sixteenth of its diameter, which is a hairline at 16px and a heavy ring at 1024; the mark's is scaled to read as a hairline at the sizes an icon is actually seen at.

Temporary, at the founder's word, until he draws the mark properly.


## Iconography

Two kinds of glyph, and which one to use is decided by size rather than by taste.

**Hand-drawn strokes**, written inline in the component that shows them, for anything on a window's chrome. They are drawn at 9px, where a stroke width and a couple of anchor points are the whole glyph and any icon set would be thrown away at that size anyway.

**Pixelarticons** — MIT, vendored at `src/lib/icons/pixel.ts` with its licence at `src/assets/icons/LICENSE-Pixelarticons.txt` — for the insert menu, and for anything else large enough to carry one. Two things govern their use:

- They are pixel art on a **24-unit grid** and are sharp only at 24px or an integer multiple. At any other size the grid falls between screen pixels and the glyph blurs. A container is sized to the glyph, never the glyph to the container — which is why the ring's bubbles are 40px.
- They **fill**; they do not stroke. A stroke rule applied to one draws nothing.

Only the paths in use are vendored. Nothing fetches an icon at runtime or at build time, the same rule the typefaces follow.


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
| Animate the alpha of a large glass surface | Animate the tint layer above it — a paint, not a re-blur. A window dissolving from Solid into Glass on arrival *is* this: `background-color` over a constant backdrop |
| `backdrop-filter` for the primary window surface | OS compositor blur, applied from Rust. A **control** is the exception, and the only one: it sits over the app's own content, which is all that filter can see — `--gloss-blur`, ADR-039 |
| A static colour value in a component | A semantic token |
| Spacing or radius outside the scale | A scale token |
| The display face inside note content | Geist |
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
| `.lozenge` (`src/app.css`) | The Aqua seated control: shape, seat, specular, and the pressed state, with no size, fill or rim of its own. A control declares `--lozenge-fill` and optionally `--lozenge-pressed`; the states belong to the shared treatment, so pressing one is written once rather than by every control that has a fill. It carries the neutral fill **and rim** by default, and `.lit` for anything held on — the pin, a settings toggle, an engaged action in the ring, the save indicator, and every tint swatch. A component using it supplies only the hue, as `color`; the fill, the pressed fill, the rim and the glyph's colour all follow. Glyphs are sized at 58% of the diameter here rather than per component, so they scale with the disc.

The neutral is the default and `.lit` the exception, which is not arbitrary: a component that declared the neutral itself would win over `.lit` — same specificity, and component styles are injected after `app.css`. That produced two faults in turn, a lit control coming out grey and then a lit control keeping the pale rim of the unlit state.

The specular is a gradient rather than a flat ellipse. Solid, it was a white sticker across the top half of every bubble, and it sat over the glyph instead of on the glass. Light falls off.

Translucent, and the alpha runs down the gradient rather than sitting flat across it — dense where the light lands, thin through the body (ADR-039). A disc of even translucency reads as a hole cut in the window. The rim, the seat and the specular stay at full strength: those are what keep it an object. A **lit** lozenge stays opaque, because its fill is the information.

Round, and worn only by a note window's own chrome — the controls, the tint swatches, the save indicator. Nothing outside that window joins it: the hub's delete control is flat and secondary (ADR-031). Pressed, the fill lights from below and the specular goes out: a highlight on a face no longer turned toward the light is what makes a pressed state look painted on rather than pushed in. Worn by the window's controls and by every tint swatch. Not a component — a shared treatment, in `app.css` for the same reason `.surface` is: two components wanted it. |
| `SaveTrouble` | Shown only when a write has failed: a lit `--signal-danger` lozenge carrying the reason as its label, which retries the write when clicked. Takes `reason`, `onRetry`. | Does not recede. Principle 2 stops at chrome carrying information, and "what is on screen is not what is on disk" is the most important thing a note window can say. Colour is doing the work rather than decorating, which is the case ADR-025 allows. |
| `RadialMenu` | A ring of seated bubbles opened at the pointer on right-click. Takes `actions`, `at`, `onChoose`, `onDismiss`. | The insert menu (ADR-038): six things that write markdown into the note. It held window actions first and they moved out, which is what ADR-034 built it to allow — the mechanism is the component, the contents are one array held by the window. Its centre is clamped away from the window's edges, because the surface clips what leaves it. The hovered action's name sits in the middle of the ring — the one place nothing else occupies — on a `.lozenge` pill in the **display face**, which is the application naming its own parts rather than saying a sentence. It takes each action's `short` form where it has one: the centre holds about two words before the pill meets the bubbles, and `label` stays the accessible name so a screen reader hears "bulleted list" rather than "star list". It wears the bubbles' own glass so it reads as part of the ring instead of a tooltip over it; it was an opaque chip with a hairline border, the one thing in the window belonging to no family.

Bubbles arrive staggered, by transform and opacity; never blur — `.bubble-in`, shared with the tint palette. They leave by **popping**: swelling past full size and going, all at once rather than staggered, over `--dur-instant`. An arrival spends 300ms establishing six objects and an object that vanishes in a frame was never one; a stagger on the way out would be six waits before the window can act on what you clicked. The ring outlives the click that ended it and calls back when the animation is done, so the choice lands at the moment the ring is gone.

Its bubbles are **40px**, which is off the spacing scale, and its glyphs are pinned to 24px rather than `.lozenge`'s 58%. The glyphs are pixel art and only sharp at their own size — see Iconography. This is the one place in the inventory where a size is set by what it contains, and it is not a precedent for anything that does not have that constraint. |
| `.chrome-control` (`src/app.css`) | What makes a lozenge a control on a window's chrome: `--space-4`, the quiet glyph colour, the hover, and the receding that Principle 2 asks for. Glyphs are drawn on a 12-unit grid at stroke-width 2. | Shared the moment a second window grew one. `WindowChrome` renders its own controls *and* takes a `controls` snippet whose markup belongs to the calling window — and Svelte scopes styles per component, so a control passed in cannot reach anything the chrome declares. Without this the hub's new-note button would be three copied declarations that then drift. The one thing that stays in `WindowChrome` is the pin's refusal to recede, which is that chrome's alone. |
| `.bubble-in` (`src/app.css`) | The arrival of a bubble in a group: out of nothing, scale and opacity, `--dur-quick`. Worn by the insert ring and by the tint palette's discs. The stagger belongs to the caller — an `animation-delay` per item, since only the caller knows the order. | Shared the moment there were two of them, per CLAUDE.md's DRY rule: two hand-written copies of one arrival is how the same gesture ends up half a frame apart in two places. It animates `transform`, so anything needing a transform of its own puts that on a wrapper rather than fighting the keyframe — which is why a ring bubble sits inside a positioning seat. Never blur. |
| `Scramble` | Typographic scrambling: text that settles into itself over `--dur-*`-scale time, left to right. Takes `text`, `durationMs`. | Only ever over chrome, never note content — scrambling something being edited would mean rewriting glyphs in the buffer. It reflows nothing, because the display face is monospaced. The pool is punctuation rather than letters: letters mid-scramble read as words that are not there. `aria-label` carries the real text throughout. `prefers-reduced-motion` is honoured in JavaScript, since the global CSS rule cannot reach a rAF loop. |
| `Toggle` | A setting that is on or off, **or one of a set**: one seated bubble that lights and shows a dot when held. `role` is `switch` or `radio` — the control looks the same and the difference is what a screen reader announces, since "switch, off" said three times over a three-way choice describes the wrong thing. A radio pressed while already on does nothing, because you cannot deselect one of three themes into having none; the `radiogroup` around it is the caller's. A dot rather than a check: a check is an action being confirmed, and this is a state being held — the same reason the pin draws a pin. The same control the always-on-top pin is, doing the same job. Takes `on`, `label`, `role`, `onChange`. | It was a sliding switch first, with the bubble as its knob — one component too many. A switch has a track, a travel distance and an end stop to get wrong, and the knob overshot its track. The bubble already says on-or-off in this application's language and cannot overshoot anything. |
| `ChordInput` | A keyboard shortcut, pressed rather than typed. Shows the chord in the platform's own names — `Ctrl + Shift + X`, not `Mod-Shift-x` — and on click waits for a keypress. Takes `value`, `format` (`editor` for CodeMirror's dialect, `global` for the operating system's), `label`, `onChange`. | Replaced a text field that handed free text to CodeMirror, which ignores what it cannot parse: a typo made a shortcut that never fired and never said why (SMD-074). Capturing the keypress removes the class of error rather than reporting it. A button, not an input — it takes one keypress and shows one value, where an input would carry a caret, a selection and a paste target that mean nothing here. It refuses a bare letter, a lone modifier, and **Ctrl+Alt**, which is AltGr on most non-US layouts; a refusal keeps it listening, since every one of them is a chord the user was one key away from. Listening is drawn in `--signal-engaged`, because the control is holding the keyboard and every other shortcut in the window is inert until it stops. |
| `Field` | One row of the settings window: what the setting is, the control, and anything that went wrong underneath. Takes `label`, `note`, `problem`, `control`. | The problem line lives here rather than in each control, because every setting can fail the same way and a window that reports failures in a different place each time teaches people to look in several places. |
| `TintPicker` | A note's colour. A swatch that opens the other tints in a row **beside it, on its own line, running leftwards** — leftwards because the picker sits near the window's trailing edge and the surface clips what leaves it. **No panel:** the discs sit on the window's drag bar, where there is nothing behind them. They dropped below the swatch once, which put them over the note's first paragraph — and a translucent disc with the writing inside it is not a swatch. **The note's own colour is not among them:** the swatch *is* that option, and an option cannot be in two places at once. The list is captured when the palette opens rather than derived, because choosing a tint changes the tint, and a derived list would rearrange itself mid-dismissal. Discs arrive outward from the swatch, staggered, by `.bubble-in`, each growing from its trailing edge so the row reads as opening rather than as six things appearing. Dismissal is the same order counted from the far end, so the row retracts into the control it came out of — and the animation of the disc nearest the swatch, the last to leave, is what unmounts it. That also keeps reduced motion correct, since a collapsed duration still fires the event. Each disc is rimmed and lit from its own hue (ADR-026, ADR-027), carrying that hue as `color` rather than `background` so one rule fills and rims all seven. Clear is a lit rim with nothing in it. Takes `revealed`, `tint`, `onTint`. | Recedes like every other control, tinted or not (ADR-031). It used to stay visible on a coloured note, as chrome carrying information — but that information is already in full view, since the note *is* the colour. |
| `NoteRow` | One note in the hub: its title, when it changed, and a delete control. Takes `title`, `when`, `onOpen`, `onDelete`. | The row is not the control — the buttons inside it are. Delete is revealed by hovering the row in CSS rather than through state, which avoids putting a pointer handler on a `div` that would then need an ARIA role it does not deserve. |
| `Editor` | The note's markdown editing surface. Takes `value?: string` as initial source, `formatting` for the remappable chords, and `onReady`, which hands the window a way to run an editor command at a point — the insert menu's only route in. Owns the CodeMirror instance and its lifecycle. | Content, not surface — quiet and typographic per Principle 1. No gutters, no active-line highlight, no border. Editor internals live in `src/lib/editor/`. |
