# CLAUDE.md

> **Authoritative for:** engineering imperatives for this repository — session procedure, project structure, framework usage rules, design enforcement, cross-platform discipline, local setup, and build.
> **Never contains:** project context, stack rationale, or current state. What sticky.md is and why the stack was chosen live in `MASTER.md`. The visual system lives in `docs/DESIGN.md`. Current phase and active work live in `docs/STATUS.md`.
> **Last verified:** 2026-09-05

This project runs on FLOW. The spec sits in the working tree at `docs/FLOW_SYSTEM_v*.md` and is **deliberately untracked** — it is the founder's portable system, excluded by `.gitignore` per ADR-015. Read it before doing anything else. If it is absent from your working copy, ask the founder for it rather than proceeding without it.

---

## Before Every Session

Read, in this order:

1. `docs/STATUS.md` — current phase, active step, item log
2. `CLAUDE.md` — this file
3. `docs/DESIGN.md` — before any UI work, without exception
4. `docs/FIXES.md` — before touching any area listed below
5. `MASTER.md` — when starting a new phase or implementing a new feature
6. `docs/DECISIONS.md` — only when a past decision is in question

Then reconcile: run `git log` for commits since STATUS.md was last updated, check each against FLOW's trigger table, verify `core.hooksPath` is set on this machine, and report gaps before doing any work.

There is no `docs/SCHEMA.md`. This project has no database.

### Areas that require reading FIXES.md first

- The CodeMirror inline-rendering layer and its decorations
- The editor's language set and bundle composition
- Window transparency, vibrancy, and compositor blur setup
- The sidecar index, and note file rename and deduplication logic
- Global shortcut registration and tray lifecycle
- Tauri bundler and release workflow configuration

---

## Project structure

```
assets/icon/              the application mark — source SVG, never a generated size
src/                      Svelte frontend
  assets/fonts/           vendored typefaces, each with its licence text
  lib/
    components/           shared components — every file here is in the DESIGN inventory
    tokens/               design tokens — the only place a raw colour value may appear
    editor/               CodeMirror configuration and the inline-rendering layer
    state/                Svelte 5 rune-based state
  windows/                one entry point per window type
src-tauri/                Rust backend
  src/
    notes.rs              note file I/O
    index.rs              the sidecar index
    windows.rs            window lifecycle
    shortcuts.rs          global and in-app shortcuts
docs/                     FLOW documentation
.githooks/                committed git hooks
```

Place shared components in `src/lib/components/`. Nothing shared lives anywhere else.

## Frontend

- Use **Svelte 5 runes** (`$state`, `$derived`, `$effect`) for all state. Do not use the legacy store API or import from `svelte/store`.
- Use **TypeScript** everywhere. Do not add `.js` source files.
- One Vite entry point per window type. Do not route between window types client-side.
- The frontend never touches the filesystem. All persistence goes through Tauri commands.
- Do not add a UI framework, component library, or CSS framework. Components are written here.

## Backend

- Rust owns file I/O, the sidecar index, tray residency, global shortcuts, and window lifecycle. Do not move any of these into the frontend.
- Build every path with `PathBuf` or Tauri's path APIs. Never concatenate path strings, and never write a separator literal.
- Send files to the trash through the `trash` crate. Never call a platform-specific delete.
- Return typed errors across the Tauri command boundary. Do not surface a raw `String` error to the frontend.
- Treat every name and path arriving from the frontend as untrusted. Validate that it is a single ordinary path component before joining it to anything. A substring check for `..` is not validation — `..` is a legitimate substring of a title.
- Run the Rust tests with `cargo test` from `src-tauri/`. Path and name validation carries tests; do not change that logic without them.
- Debounce writes. Never write a note file on every keystroke.
- Write the sidecar index through the lock and the temporary-file rename in `src-tauri/src/index.rs`. Never write it in place, and never read-modify-write it without holding the lock — two windows doing that at once lose one of the two changes.
- Build every note window from the window entry in `src-tauri/tauri.conf.json` rather than repeating its geometry in code. Two sources for one size is how they drift, and `docs/DESIGN.md` names that file as where the dimensions live.
- Derive filenames here, never in the frontend. The frontend sends a title; Rust slugifies it, deduplicates against the folder, and returns the name the note now has. Deduplication cannot be done without seeing the folder.
- Apply compositor blur here, through `window-vibrancy`, never from CSS. A web view cannot see the desktop behind it, so `backdrop-filter` is not an alternative — it is a different effect that looks correct only over the app's own content.
- Treat a window that could not be frosted as Solid, not as an error. Solid is a supported way to run.

## Editor

- Markdown syntax stays in the buffer at all times. Rendering is a decoration layer over real source — never a transform of it.
- Formatting commands insert real syntax characters. A command that produces styled text without the corresponding markdown is a bug.
- Copy yields raw source. Verify this whenever the editor's clipboard behaviour is touched.
- Do not add a markdown feature beyond what `MASTER.md § In Scope` lists without a decision recorded first.

## Design enforcement

`docs/DESIGN.md` is authoritative for the visual system. This file does not restate it. Three imperatives follow from it:

- **Read every colour through a semantic token.** No component declares a static colour value. Raw values live only in `src/lib/tokens/`.
- **Use only the spacing and radius scale.** No arbitrary pixel values.
- **Before building any UI, read the component inventory in `docs/DESIGN.md`.** If a component is not listed, it does not exist — and if you create one, it lands in the inventory in the same commit.

## DRY

- **Second use means extract.** The moment a treatment, helper, or pattern appears a second time, pull it into one component, token, or function and point both uses at it.
- **Flag duplication** when you notice it or are about to create it. Never silently copy-paste.
- **Split a token or function serving two roles** before the roles need to diverge.
- Applies to Rust and TypeScript equally, not only to visual treatments.

## Cross-platform discipline

Windows is the current build target. Hold these from the first commit regardless — retrofitting them costs far more than writing them correctly now.

- Resolve modifier keys through the platform abstraction. Never hardcode `Ctrl` or `Cmd` in a shortcut definition.
- Never build a path by string concatenation, and never write `\` or `/` as a separator literal.
- Delete through the trash abstraction. Never call the Windows Recycle Bin directly.
- Use custom window chrome everywhere. Never rely on native window decorations.

## Local setup

Required on every machine, and per clone:

```
git config core.hooksPath .githooks
```

The hook does not run without it. Reconciliation verifies it at session start.

Toolchain:

- Rust stable, via rustup
- Node LTS, with npm
- WebView2 runtime — present by default on Windows 11

Run the app with `npm run tauri dev`. Never start a dev server with a bare `npm run dev` — the Tauri shell will not be attached.

## Build and release

- Build artifacts are produced by GitHub Actions on a `v*` tag, never by hand for distribution.
- Every release carries both the NSIS installer and the portable zip.
- Vendor all fonts and assets into the repository with their licence files alongside. Never fetch an asset over the network at runtime or at build time.
- The README discloses the SmartScreen warning that unsigned builds produce. Never remove that disclosure while builds are unsigned.
- Do not add a dependency that makes a network request at runtime.
- The Content Security Policy in `src-tauri/tauri.conf.json` is restrictive by design — it is MASTER veto 2 enforced by the engine rather than by discipline. Never widen it to make something work. If a change appears to require a wider CSP, the change is wrong.
