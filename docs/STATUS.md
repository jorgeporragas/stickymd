# STATUS

Everything with a state. This is the only file permitted to contain statements that expire.

---

## Current Phase

**Build** — Phase 1 — Foundation & Editor Core

## Current Active Step

Integrate CodeMirror 6 into the note window and build the inline-rendering layer — syntax hidden off the cursor's line, revealed on it, with formatting commands inserting real characters. This is the riskiest work in the project and is deliberately first. It is pure frontend, so it proceeds while Rust is being installed.

---

## Environment

| | |
|---|---|
| Development machine | Windows 11 Pro |
| Build target | Windows x64 |
| Node | v24.19.0, npm 11.17.0 — verified present 2026-09-05 |
| Rust | Absent on the development machine as of 2026-09-05. `rustc`, `cargo` and `rustup` are not on PATH. The Rust half of the project cannot build or run until rustup is installed. |
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

### [SMD-012] Scaffold the Tauri v2 + Svelte 5 shell and the token layer
Type:    chore
State:   active
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 1 — Foundation & Editor Core
Notes: Vite multi-page build with one entry per window type, the design token layer, and the first inventoried component. Ships in Solid surface mode; glass is SMD-015.

### [SMD-013] Vendor Handjet, Geist and Martian Mono
Type:    chore
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
Notes: Subset to the characters actually used, each with its `OFL.txt` alongside. Until then the type stacks fall back to system faces. Nothing may be fetched at runtime — MASTER veto 2.

### [SMD-014] App icon and bundle configuration
Type:    chore
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
Notes: `tauri.conf.json` carries no `bundle` section, so `tauri build` cannot produce an installer. `tauri dev` is unaffected. Blocks any release work.

### [SMD-015] Compositor glass and the Glass/Solid mode switch
Type:    feature
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
Notes: Acrylic applied to the transparent window from Rust, per `docs/DESIGN.md` principle 3. Adds a dependency, which owes a CLAUDE.md entry. Must degrade to Solid when the system disables transparency effects.

---

## Completed Milestones

*(A phase collapses into a single changelog line here when it closes.)*
