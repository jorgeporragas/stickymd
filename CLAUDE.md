# CLAUDE.md

> **Authoritative for:** engineering imperatives for this repository — what is specific to this repo in a session's reading, project structure, framework usage rules, design enforcement, cross-platform discipline, local setup, and build.
> **Never contains:** project context, stack rationale, or current state. What sticky.md is and why the stack was chosen live in `MASTER.md`. The visual system lives in `docs/DESIGN.md`. Current phase and active work live in `docs/STATUS.md`. FLOW's own rules live in `FLOW_SYSTEM.md`. How to set up and run the project lives in the README.

This project runs on FLOW, installed from the FLOW repository by `flow_sync`. `FLOW_SYSTEM.md` and the files in `.flow/` other than `config.toml` are FLOW's and are never edited here, nor are `.githooks/pre-commit` and `.claude/skills/flow-start/`: `flow_check` fails when any of them differs from `.flow/manifest.json` (C10). Read `FLOW_SYSTEM.md` before doing anything else.

---

## Before Every Session

Start every session with `/flow-start`. What is specific to this repository:

- Read `docs/DESIGN.md` before any UI work, without exception, and `docs/components/` for each component you touch.
- Read `docs/FIXES.md` before touching any area listed below. FIXES has no area index of its own, so this list is it.
- Read `MASTER.md` when starting a new phase or implementing a new feature.
- There is no `docs/SCHEMA.md`. This project has no database.

### Areas that require reading FIXES.md first

- Where a build's settings file is, and reading it from this machine
- The CodeMirror inline-rendering layer and its decorations
- The editor's language set and bundle composition
- Window transparency, vibrancy, and compositor blur setup
- The sidecar index, and note file rename and deduplication logic
- Global and in-app shortcut registration, and tray lifecycle
- Tauri bundler and release workflow configuration

---

## Project structure

```
FLOW_SYSTEM.md            FLOW's rules — installed by flow_sync, never edited
.flow/                    FLOW's checks, installed by flow_sync; config.toml is this project's
.claude/                  agent settings, and FLOW's session-start skill
scripts/                  this project's code checks, run in CI
assets/icon/              the application mark — source SVG, never a generated size
src/                      Svelte frontend
  assets/fonts/           vendored typefaces, each with its licence text
  lib/
    components/           shared components — every file here has its file in docs/components/
    tokens/               design tokens — the only place a raw colour value may appear
    editor/               CodeMirror configuration and the inline-rendering layer
    state/                Svelte 5 rune-based state
  windows/                one entry point per window type
src-tauri/                Rust backend
  src/
    notes.rs              note file I/O
    settings.rs           application settings, in the config directory
    index.rs              the sidecar index
    windows.rs            window lifecycle, and what a launch opens
    desktop.rs            virtual desktops — bringing a window to the one in use
    surface.rs            compositor blur, window rounding, and the transparency setting
    tray.rs               tray residency and its menu
    preferences.rs        the settings window's commands
    shortcuts.rs          global and in-app shortcuts
    dev.rs                what marks a debug build — compiled out of a release one
docs/                     FLOW documentation
  components/             one file per shared component — the inventory
  COMPLETED.md            work closed before FLOW 3.0: frozen, never edited
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
- No window is created from `src-tauri/tauri.conf.json` at startup. Every entry is `create: false`, and `windows::launch` decides what opens. Bring a window that is already open forward through `windows::summon`, never with a bare `set_focus`, so it comes to the virtual desktop in use rather than pulling the user to another one.
- Build every note window from the window entry in `src-tauri/tauri.conf.json` rather than repeating its geometry in code. Two sources for one size is how they drift, and `docs/DESIGN.md` names that file as where the dimensions live.
- Derive filenames here, never in the frontend. The frontend sends a title; Rust slugifies it, deduplicates against the folder, and returns the name the note now has. Deduplication cannot be done without seeing the folder.
- Apply compositor blur here, through `window-vibrancy`, never from CSS. A web view cannot see the desktop behind it, so `backdrop-filter` is not an alternative — it is a different effect that looks correct only over the app's own content.
- Treat a window that could not be frosted as Solid, not as an error. Solid is a supported way to run.
- Keep a debug build's settings out of the installed build's. `app_config_dir()` is the bundle identifier and does not vary by build profile, so both binaries resolve one directory; `settings.rs` picks `settings.dev.json` under `cfg!(debug_assertions)` and `notes_dir` defaults elsewhere the same way. Without that, pointing a development build at a scratch notes folder moves the installed one too. See SMD-100.
- Never let closing the last window end the application. It is tray-resident, and a summonable note is the product. Only an explicit quit exits.
- Keep the application to one copy. `tauri-plugin-single-instance` makes a second launch hand over to the copy already running, which opens the hub. It is registered in release builds only, because it keys on the bundle identifier that a debug build shares with the installed one. See `docs/FIXES.md`.

## Editor

- Markdown syntax stays in the buffer at all times. Rendering is a decoration layer over real source — never a transform of it.
- Formatting commands insert real syntax characters. A command that produces styled text without the corresponding markdown is a bug.
- Copy yields raw source. Verify this whenever the editor's clipboard behaviour is touched.
- Do not add a markdown feature beyond what `MASTER.md § In Scope` lists without a decision recorded first.

## Design enforcement

`docs/DESIGN.md` is authoritative for the visual system. This file does not restate it. Three imperatives follow from it:

- **Read every colour through a semantic token.** No component declares a static colour value. Raw values live only in `src/lib/tokens/`.
- **Use only the spacing and radius scale.** No arbitrary pixel values.
- **Before building any UI, look in `docs/components/`.** If a component has no file there, it does not exist — and if you create one, its file lands in the same commit.

## DRY

- **Second use means extract.** The moment a treatment, helper, or pattern appears a second time, pull it into one component, token, or function and point both uses at it.
- **Flag duplication** when you notice it or are about to create it. Never silently copy-paste.
- **Split a token or function serving two roles** before the roles need to diverge.
- Applies to Rust and TypeScript equally, not only to visual treatments.

## Cross-platform discipline

Windows and macOS on Apple Silicon are the build targets; Linux is next. These rules were held from the first commit while Windows was the only one, and that is why the macOS port was a week of work rather than a rewrite — five platform branches in the whole of Rust. Hold them for Linux the same way.

- Resolve modifier keys through the platform abstraction. Never hardcode `Ctrl` or `Cmd` in a shortcut definition.
- Never bind a `Ctrl+Alt` chord. On Latin American, Spanish and most European layouts that combination is AltGr, and the shortcut silently never fires. It works on a US layout, which is what makes it dangerous.
- Register a shortcut globally, from Rust, when the web view would otherwise claim it. WebView2 keeps `Ctrl+N`, `Ctrl+T`, `Ctrl+W`, `Ctrl+P`, `Ctrl+F`, `Ctrl+R`, `Ctrl+D`, `Ctrl+Shift+N` and `F5` for itself. See `docs/FIXES.md`.
- Never build a path by string concatenation, and never write `\` or `/` as a separator literal.
- Delete through the trash abstraction. Never call the Windows Recycle Bin directly.
- Use custom window chrome everywhere. Never rely on native window decorations.

## Local setup

- `core.hooksPath` must be `.githooks` in every clone. The pre-commit hook does not run without it, and `/flow-start` verifies it. The command is in the README under *Build it yourself*.
- Never start a dev server with a bare `npm run dev` — the Tauri shell will not be attached. Run the app with `npm run tauri dev`.

## Build and release

- Build artifacts are produced by GitHub Actions on a `v*` tag, never by hand for distribution.
- Every Windows release carries both the NSIS installer and the portable zip; a macOS release carries the `.app` and the `.dmg`.
- Vendor all fonts and assets into the repository with their licence files alongside. Never fetch an asset over the network at runtime or at build time.
- The README discloses the SmartScreen warning that unsigned builds produce. Never remove that disclosure while builds are unsigned.
- Do not add a dependency that makes a network request at runtime. **One exception, and it is named rather than general:** the Tauri updater, per ADR-014 and Phase 7. It carries no note content, checks only when a person presses a button, and prompts before replacing anything. Anything else that reaches the network is a bug, and a second exception needs its own decision.
- The updater runs in Rust and the Content Security Policy is therefore untouched by it. If something ever appears to need `connect-src` widened for an update check, it is being done in the wrong half of the application.
- The Content Security Policy in `src-tauri/tauri.conf.json` is restrictive by design — it is MASTER veto 2 enforced by the engine rather than by discipline. Never widen it to make something work. If a change appears to require a wider CSP, the change is wrong.
