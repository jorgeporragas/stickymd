# STATUS

Everything with a state. This is the only file permitted to contain statements that expire.

---

## Current Phase

**Build** — Phase 1 — Foundation & Editor Core

## Current Active Step

Scaffold the Tauri v2 + Svelte 5 + TypeScript project against the structure in `CLAUDE.md § Project structure`, and establish the design token layer from `docs/DESIGN.md § Token contract` before any component is written.

---

## Environment

| | |
|---|---|
| Development machine | Windows 11 Pro |
| Build target | Windows x64 |
| Toolchain | Rust stable (rustup), Node LTS with npm, WebView2 runtime |
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
  2026-09-05  logged and accepted into Phase 5 — Theme & Motion
Notes: A complete set of token values, not a code change — faint black tint over the frosted surface with warm white ink, inverting Frost. Light is the default mode. Needs founder confirmation that Phase 5 is the right home rather than post-V1.

### [SMD-005] Auto-update via the Tauri updater
Type:    feature
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 7 — Release & Auto-Update, per ADR-014
Notes: Must prompt before replacing anything. Requires a minisign keypair generated locally and a version manifest published alongside releases.

### [SMD-006] Verify Handjet axis behaviour and metrics
Type:    chore
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
Notes: Confirm whether changing ELGR or ELSH alters advance widths. If it does, animating the axes causes reflow and any identity animation must be sized to a fixed box. Establishes whether the app-title animation described in `docs/DESIGN.md § Motion` is affordable.

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

---

## Completed Milestones

*(A phase collapses into a single changelog line here when it closes.)*
