# STATUS

Everything with a state. This is the only file permitted to contain statements that expire.

---

## Current Phase

**Build** — Phase 1 — Foundation & Editor Core

## Current Active Step

Wire compositor glass and the Glass/Solid mode switch (SMD-015). `window-vibrancy` is already in the dependency tree as a transitive dependency of Tauri, so it needs promoting to a direct one. This is the first Rust work in the project.

---

## Environment

| | |
|---|---|
| Development machine | Windows 11 Pro |
| Build target | Windows x64 |
| Node | v24.19.0, npm 11.17.0 — verified present 2026-09-05 |
| Rust | rustc 1.98.1, cargo 1.98.1, rustup 1.29.1 — installed 2026-09-05. |
| MSVC linker | Visual Studio C++ build tools installed 2026-09-05. Rust links; `cargo check` now fails only on the missing application icon (SMD-014). |
| WebView2 | Ships with Windows 11 |
| `core.hooksPath` | Set on the development machine 2026-09-05. Must be set again on every clone — see `CLAUDE.md § Local setup`. |

---

## Item Log

One log. `Type` is a field. An item never moves file and never gets copied; its state changes in place.

### [SMD-001] Search in the hub
Type:    feature            (feature | bug | idea | chore)
State:   idea               (idea → accepted → active → shipped | dropped)
Created: 2026-09-05
History:
  2026-09-05  logged as idea — cut from V1 during Planning
Notes: With a handful of notes living a few days, scrolling the hub beats building search, and the notes folder is greppable. Revisit when the hub becomes uncomfortable to scan.

### [SMD-002] User-authored custom themes
Type:    feature
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea — cut from V1 during Planning
Notes: Depends on the token architecture being right from the first commit, which ADR-012 requires regardless. Shipping a theme format makes it a public API to support, so built-in themes come first.

### [SMD-003] Configurable notes folder location
Type:    feature
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea — cut from V1 during Planning
Notes: One sane default for V1. Making it configurable brings a settings surface, a migration path for existing notes, and a class of bugs, for a setting used once.

### [SMD-004] Dark theme
Type:    feature
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 5 — Theme & Motion (confirmed by founder)
Notes: A complete set of token values, not a code change — faint black tint over the frosted surface with warm white ink, inverting Frost. Light is the default mode.

### [SMD-005] Auto-update via the Tauri updater
Type:    feature
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 7 — Release & Auto-Update, per ADR-014
Notes: Must prompt before replacing anything. Requires a minisign keypair generated locally and a version manifest published alongside releases.

### [SMD-006] Verify Handjet axis behaviour and metrics
Type:    chore
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
  2026-09-05  active — measured
  2026-09-05  shipped — commit 7417271
Notes: Measured in a browser against the vendored file: the string "StickyMD scratchpad" is 221.19px wide at 32px under every combination of ELGR 1–2, ELSH 0–16 and wght 100–900. Advance widths do not move, so animating the axes reflows nothing. Recorded in `docs/DESIGN.md § Typography`.

### [SMD-007] Add the GPL-3.0 licence file
Type:    chore
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active
  2026-09-05  shipped — commit 641f340
Notes: The canonical text from gnu.org, verbatim, as `LICENSE` in the repository root. Per ADR-009. Fetched rather than retyped, so it is byte-exact.

### [SMD-008] Write the README
Type:    chore
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
Notes: Must document the SmartScreen warning honestly, per ADR-010. Sits outside FLOW's scheme; may link to FLOW files but must not restate what they own.

### [SMD-009] Chakra Petch as an alternate built-in theme's display face
Type:    idea
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea during Planning
Notes: The founder's fallback display face when Redaction's licensing was in question — see ADR-011. A sharper, more technical skin. Exercises the theme system as more than a colour picker.

### [SMD-010] Code signing certificate
Type:    idea
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea during Planning
Notes: Roughly $200–400 per year, which the project does not carry — see ADR-010. The founder raised GitHub Sponsors as a possible route if the project attracts contributions.

### [SMD-011] Distribution via winget and Scoop
Type:    idea
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea during Planning
Notes: Trivial to add once releases exist. GitHub Releases is the source of truth either way.

### [SMD-012] Scaffold the Tauri v2 + Svelte 5 shell and the token layer
Type:    chore
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 1 — Foundation & Editor Core
  2026-09-05  shipped — commit 2a5b11b
Notes: Vite multi-page build with one entry per window type, the design token layer, and the first inventoried component. Ships in Solid surface mode; glass is SMD-015. Frontend verified by svelte-check and vite build; the Rust half is unverified until rustup is installed.

### [SMD-013] Vendor Handjet, Geist and Martian Mono
Type:    chore
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
  2026-09-05  active
  2026-09-05  shipped — commit 7417271
Notes: Eight variable woff2 files in `src/assets/fonts/`, 190 kB total, subset to latin and latin-ext — other scripts deliberately absent. Geist ships roman and italic; the others are roman only. Each family's `OFL.txt` sits beside its files. Verified in a browser: all three families load and render, and the Handjet axes drive real degradation.

### [SMD-014] App icon and bundle configuration
Type:    chore
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
  2026-09-05  scope corrected — blocks the build, not only releases
  2026-09-05  active
  2026-09-05  shipped — commit e3ac885
Notes: `tauri-build` requires `icons/icon.ico` to generate the Windows resource file, so `cargo check` fails without it and the application cannot run at all. The earlier note claiming `tauri dev` was unaffected was wrong. Also needs the `bundle` section in `tauri.conf.json` for NSIS and the portable zip. The icon itself is an identity decision for the founder.

### [SMD-015] Compositor glass and the Glass/Solid mode switch
Type:    feature
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
Notes: Acrylic applied to the transparent window from Rust, per `docs/DESIGN.md` principle 3. Adds a dependency, which owes a CLAUDE.md entry. Must degrade to Solid when the system disables transparency effects.

### [SMD-016] Inline rendering layer
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 1 — Foundation & Editor Core
  2026-09-05  shipped — commit b930795
Notes: CodeMirror 6 with markdown syntax hidden on every line the cursor is not on and revealed on the line it is. Formatting commands insert real characters — `Mod-b`, `Mod-i`, `Mod-e`, `Mod-Shift-x`. Fenced code highlighted across six languages. Verified in a browser against the live DOM: hiding, revealing, the link/autolink distinction, and bold round-tripping.

### [SMD-017] Render tables as tables
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
  2026-09-05  active
  2026-09-05  shipped — commit 793ff8a
Notes: Rendered when the cursor is outside, raw pipes when inside, with column alignment read from the delimiter row and inline markup rendered inside cells. Clicking a cell puts the cursor in that cell's source, which is the only way back into text a widget has replaced. Verified in a browser across the full cycle.

### [SMD-018] Verify copy yields raw markdown in the running application
Type:    chore
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
Notes: The document holds real markdown and CodeMirror serialises the clipboard from state rather than the DOM, so raw source is expected. Unverified — reading the clipboard was not possible in the browser harness. This is the single property the product exists to provide, so it gets an explicit check.

### [SMD-019] Note window JavaScript weight
Type:    idea
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: The note window's main chunk is 555 kB raw, 193 kB gzipped — mostly CodeMirror plus the HTML, CSS and JavaScript grammars that `@codemirror/lang-markdown` pulls in unconditionally (see `docs/FIXES.md`). Loaded from disk, so no network cost, but every open note window parses its own copy.
  Measured 2026-09-05, one window, development build: the StickyMD process tree is 381.8 MB across 7 processes — 35.9 MB for the application and 345.9 MB across 6 WebView2 processes. Half the WebView2 processes on the machine belonged to other applications and are excluded.
  Still unmeasured, and both matter before drawing any conclusion: a release build, and the cost of the second and subsequent windows. Tauri shares one WebView2 environment across windows, so window two should cost a renderer rather than another tree — that is the number that decides whether the many-window architecture is affordable. `MASTER.md § Stack` claims materially lower memory across many windows; that claim is currently unverified in either direction.

### [SMD-020] Fenced code block presentation
Type:    feature
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: Fence markers stay visible by design — hiding them leaves a bare language name floating above the block. A proper treatment gives the block its own surface with the language shown deliberately rather than as leftover syntax.

### [SMD-021] Per-note colour
Type:    feature
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 5 — Theme & Motion
Notes: The note tints from ADR-018 — Clear, Sun, Spring, Aqua, Sky, Lilac, Blush — as `--note-tint-*` tokens, selectable per note. `Note.theme` already carries it in the domain model and the sidecar index already stores it. Each tint must hold `--ink-primary` at 4.5:1 when layered over frosted glass on an arbitrary wallpaper. Values live in ADR-018 until this ships; `src/lib/tokens/tokens.css` becomes authoritative then.

---

## Completed Milestones

*(A phase collapses into a single changelog line here when it closes.)*
