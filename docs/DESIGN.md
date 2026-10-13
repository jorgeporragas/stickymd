# DESIGN

> **Authoritative for:** the visual system — design principles, the token contract, typography, themes, and motion.
> **Never contains:** work state, token values, or component entries. Components live in `docs/components/`, one file each. Components and themes that do not exist, and anything carrying a completion mark, are issues. Every token's value lives in `src/lib/tokens/tokens.css`. Product truth lives in `MASTER.md`. Engineering imperatives live in `CLAUDE.md`.

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
No component declares a static colour, ever. This is what makes a theme a data file rather than a refactor. CI rejects a pull request that adds a raw colour outside `src/lib/tokens/`, through `scripts/check-raw-colours.sh`.

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
| `--ink-muted` | Inactive states and metadata on the hub and settings, which are surfaces that are never tinted |
| `--ink-note-muted` | Quiet ink written on a note: the placeholder and a link's destination. **Holds 3:1 on every note, clear or tinted, on its theme's design basis.** On a dark tint it sits *above* the surface, for the reason `--ink-syntax` does, and stays below `--ink-primary` so a placeholder never reads as writing. Split from `--ink-muted` by SMD-098, which found the placeholder at 1.21:1 on tinted dark notes |
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

**It is what a launch opens** when there are no notes to bring back from the last session, unless the settings say a new note (ADR-040). Asked for while it is open on another virtual desktop, it comes to the desktop you are on rather than taking you to it (ADR-041). The settings window and an already-open note behave the same way.

---

## Application mark

The application's own seated lozenge, in `--tide-400`, carrying the letter **S** from Departure Mono. Source: `assets/icon/stickymd.svg` — the vector, never a generated size.

Two things about it are deliberate rather than convenient. The glyph is **rectangles, not text**: its grid was sampled from the face rendered in the running application, so the mark needs no font present to draw. And every colour is `--tide-400` mixed in oklab, the same way `color-mix` does it in the stylesheet, so the mark is the same green as the controls rather than nearly.

The rim is the one proportion that does *not* follow the interface. A control's rim is a sixteenth of its diameter, which is a hairline at 16px and a heavy ring at 1024; the mark's is scaled to read as a hairline at the sizes an icon is actually seen at.

Temporary, at the founder's word, until he draws the mark properly.

### In a development build

The same mark with **only its hue turned**, from the green's 154 to 60 — the amber SMD-093 chose when dark yellow turned out to be olive. Lightness and chroma are held, which is the arithmetic `--swatch-*-engaged` already uses: one colour with the hue rotated reads as the same thing wearing a different coat, where a freshly picked colour reads as a different thing. `#76c893` becomes `#e7a267`; the S is near-black at chroma 0.029 and barely moves, so the disc changes colour and the letter stays the letter.

Computed from the real mark at runtime in `src-tauri/src/dev.rs`, never drawn as a second asset — the mark above is temporary until the founder redraws it, and a hand-maintained dev copy would be the old one the morning after he does.

It exists only where `debug_assertions` does. A released build has no dev mark because it has no code for one.

### In the macOS menu bar

The mark cannot go there. A menu-bar extra is a **template image**: macOS reads its alpha channel alone and paints the result itself, so the icon follows the bar's own light or dark and its vibrancy — and a full-colour disc reduced to a silhouette is a filled circle with the S gone.

So the menu bar gets **the letter by itself**, drawn from the same 5x8 lattice the SVG uses, in `src-tauri/src/tray.rs`. Not a second drawing of the mark: the same grid, at a size a menu bar can render. Every other platform's tray keeps the mark as it is.

Three numbers, and all three follow from one fact: `tray-icon` shows every macOS tray image at **18 points tall** and scales the width to match. So the pixel height is what decides whether it is crisp — **36** lands 1:1 on a Retina display and halves exactly on any other. **Three pixels to a lattice cell** puts the glyph at 12 points, a cap height that sits beside the menu bar's own 14-point text rather than towering over it. And **4px of padding either side, 6px above and below**, is what keeps it off its neighbours: 23x36 in all, shown at 11.5x18pt.


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

## Components

Each shared component has its own file in [`docs/components/`](components/): what it is, and the rules a caller must not get wrong. **That directory is complete and authoritative: if a component has no file there, it does not exist.** Before building any UI, look in it — reuse is only the default when it has been checked.
