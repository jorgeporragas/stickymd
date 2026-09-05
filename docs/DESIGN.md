# DESIGN

> **Authoritative for:** the visual system — design principles, the token contract, typography, themes, motion, and the component inventory.
> **Never contains:** work state. Components and themes that do not exist, and anything carrying a completion mark, live in `docs/STATUS.md`. Product truth lives in `MASTER.md`. Engineering imperatives live in `CLAUDE.md`.
> **Last verified:** 2026-09-05

---

## Principles

**1. The surface is Aero. The content is Rams.**
Chrome, edges, glass, colour, and motion carry the personality. Everything inside a note — the user's text — is quiet, neutral, and typographic. The app is beautiful *around* the writing, never on top of it.

**2. Chrome recedes.**
Controls withdraw when the window is not being used and return on pointer entry or focus. The text is the focal point. Personality lives in the controls; the controls are present only when wanted.

**3. Glass is a window property, not a CSS filter.**
The frosted surface comes from the OS compositor — Acrylic on Windows, vibrancy on macOS — applied to a transparent window from the Rust side. `backdrop-filter` cannot see the desktop behind a web view and is never used for the primary surface. CSS supplies the tint, grain, and edge highlight that sit *on top* of the compositor's blur.

**4. Solid is a first-class mode, not a fallback.**
Transparency is a system setting, and Windows disables it under battery saver. Some users will only ever see the Solid surface mode. It is designed to look deliberate, not degraded.

**5. Every colour is read through a token.**
No component declares a static colour, ever. This is what makes a theme a data file rather than a refactor.

**6. Tokens layer: primitive, then semantic.**
Primitives name values (`--blur-32`, `--aqua-500`). Semantics name roles (`--note-surface-tint`, `--accent`). Components read semantics only. When a semantic token starts serving two visually distinct purposes, it splits into two named tokens before the roles need to diverge.

**7. Motion moves objects. It never re-blurs them.**
Animate `transform` and `box-shadow`. See § Never Allowed.

**8. A theme is data.**
A theme is a complete set of token values and nothing else. Themes never replace components, ship CSS, or execute code.

---

## Typography

Three families, all SIL Open Font License, all vendored into the repository with their `OFL.txt` alongside. Nothing is fetched at runtime.

| Role | Face | Used for |
|---|---|---|
| Display | **Handjet** | App identity, hub headers, empty states. Never inside note content. |
| Content | **Geist** | Note body, all UI labels, everything functional. |
| Mono | **Martian Mono** | Fenced code blocks. |

**Handjet** is a variable face whose glyphs are built from repeated elements on a grid. `ELGR` controls how many elements compose each letter; `ELSH` morphs those elements between square and round. Degradation is therefore continuous, and is set through tokens rather than by choosing a file.

**Handjet never appears inside a note.** That boundary is Principle 1 made concrete: display type is surface, and note content is content.

### Type tokens

```
--font-display:        "Handjet", system-ui, sans-serif
--font-content:        "Geist", system-ui, sans-serif
--font-mono:           "Martian Mono", ui-monospace, monospace

--font-size-display:   2rem        /* 32px — app identity */
--font-size-title:     1.125rem    /* 18px — note titles in the hub */
--font-size-body:      0.9375rem   /* 15px — note content */
--font-size-label:     0.8125rem   /* 13px — controls, UI labels */
--font-size-caption:   0.75rem     /* 12px — metadata */
--font-size-code:      0.875rem    /* 14px — Martian Mono runs wide; one step down */

--line-height-prose:   1.6
--line-height-ui:      1.35
--line-height-code:    1.5

--tracking-display:    0.02em
--tracking-body:       0
--tracking-caps:       0.08em

--handjet-elgr:        1           /* element grid */
--handjet-elsh:        2           /* element shape */
--handjet-weight:      500
```

---

## Surface modes

StickyMD renders every window in one of two surface modes.

**Glass** — the window is transparent, the compositor blurs what is behind it, and CSS layers a near-colourless tint, a fine grain, and a top edge highlight over the result. The note reads as heavily frosted translucent paper.

**Solid** — the same geometry, type, and spacing, with an opaque surface in place of the blur. Selected by the user, or forced by the system when transparency effects are unavailable.

Both modes read from the same semantic tokens. A component never branches on surface mode; the tokens carry the difference.

---

## Token contract

Components read semantic tokens only. A theme supplies a complete set of values for every semantic token below.

### Surface

```
--surface-tint            /* the colour layered over the compositor blur */
--surface-solid           /* the opaque surface in Solid mode */
--surface-grain-opacity   /* fine noise; keeps glass reading as frosted paper */
--surface-edge-highlight  /* the gloss line along the top edge */
--surface-border          /* the hairline containing the window */
```

### Ink

```
--ink-primary             /* note content */
--ink-secondary           /* UI labels, metadata */
--ink-muted               /* placeholder, disabled */
--ink-syntax              /* markdown syntax characters when revealed */
--ink-on-accent
```

### Accent

```
--accent                  /* the single interactive colour */
--accent-hover
--accent-glow             /* the Aero bloom; used on focus and active states */
--focus-ring
```

### Geometry

```
--radius-window:   18px
--radius-control:  10px
--radius-chip:     6px

--space-1:  4px
--space-2:  8px
--space-3:  12px
--space-4:  16px
--space-6:  24px
--space-8:  32px
```

Spacing and radius outside this scale are not permitted.

### Glass

```
--blur-window:     32     /* consumed by the Rust side, not by CSS */
--blur-popover:    20
```

### Elevation

```
--shadow-rest
--shadow-lifted     /* hover, focus */
--shadow-dragging
```

### Motion

```
--motion-instant:  90ms
--motion-quick:    160ms
--motion-settle:   240ms

--ease-out:        cubic-bezier(0.22, 1, 0.36, 1)
--ease-in-out:     cubic-bezier(0.65, 0, 0.35, 1)
```

---

## Themes

A theme is a complete set of values for every semantic token, and nothing else.

### Frost — the default

Colourless frosted glass. The tint is present only so that dark wallpapers cannot swallow the text; it is not perceived as a colour.

```
--surface-tint            rgba(255, 255, 255, 0.55)
--surface-solid           #F7F6F3
--surface-grain-opacity   0.035
--surface-edge-highlight  inset 0 1px 0 rgba(255, 255, 255, 0.65)
--surface-border          rgba(255, 255, 255, 0.35)

--ink-primary             #1A1A17
--ink-secondary           #4A4945
--ink-muted               #8B8A85
--ink-syntax              #A8A6A0
--ink-on-accent           #FFFFFF

--accent                  #2FB6D9
--accent-hover            #1FA3C6
--accent-glow             rgba(47, 182, 217, 0.35)
--focus-ring              #2FB6D9

--shadow-rest             0 2px 12px rgba(0, 0, 0, 0.12)
--shadow-lifted           0 8px 28px rgba(0, 0, 0, 0.18)
--shadow-dragging         0 18px 48px rgba(0, 0, 0, 0.24)
```

Contrast: `--ink-primary` against `--surface-tint` over a worst-case wallpaper is the constraint that sets the tint's alpha. Body text holds at 4.5:1 or better in both surface modes. A theme that cannot meet that is not shippable.

---

## Note window

A square with rounded corners — the Post-It proportion, and the shape the app is recognised by.

```
--note-size-default:  320px × 320px
--note-size-expanded: 480px × 480px
--note-size-min:      240px × 200px
```

Default and expanded are a toggle. Free resize is available between the minimum and the display bounds; the square is the default proportion, not a locked aspect ratio.

---

## Motion

Motion is how the Aero personality is expressed without cost. Three rules govern all of it:

**Objects arrive, they do not fade in.** A note appearing scales up slightly into place while its shadow deepens, at full glass throughout. This reads as an object arriving rather than an image loading.

**Chrome crossfades; surfaces transform.** Small controls may fade — that is cheap and correct. Large glass surfaces never change alpha.

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

---

## Component Inventory

Complete and authoritative. **If a component is not listed here, it does not exist.** Before building any UI, read this list — reuse is only the default when it has been checked.

Every entry lands in the same commit as the component it describes.

| Component | Role | Tokens it owns |
|---|---|---|
