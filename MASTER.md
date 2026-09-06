# sticky.md

> **Authoritative for:** what sticky.md is — problem, user, core loop, domain model, stack choices and their rationale, scope, permanent vetoes, build phase definitions, and deployment.
> **Never contains:** anything with a state. Current phase, active work, the item log and completed milestones live in `docs/STATUS.md`. Engineering imperatives live in `CLAUDE.md`. Decision history lives in `docs/DECISIONS.md`. The visual system lives in `docs/DESIGN.md`.
> **Last verified:** 2026-09-05

---

## Problem

Editing a chunk of markdown-formatted text — a prompt bound for Claude, a passage headed somewhere else — costs either opening an entire vault and polluting it with a scrap, or losing the formatting entirely.

Two tools already solve half of it each:

- **Obsidian** renders markdown properly, but it is a vault. Scraps clutter a structure meant for permanent notes, and it is a heavy application to switch into for thirty seconds of editing.
- **Sticky Notes** is the right shape — always there, no save ceremony, a hub to find things again — but it is plaintext. Markdown lands as literal asterisks and hashes.

Neither tool is wrong. Neither is the right shape for text that lives a few days and then goes somewhere else.

## User

A developer who writes and edits prompts and other chunks of prose destined for somewhere else. They keep an organized Obsidian vault for things worth keeping, and they do not want scratch text in it. Their scraps live a few days, get pasted somewhere, and are eventually deleted.

## Core Loop

1. **Summon** — a global hotkey opens a new note window, focused and empty
2. **Type or paste** markdown, which renders live as it is written
3. **Edit** with formatting commands that insert real syntax
4. **Copy out** — the clipboard receives raw markdown source
5. **Dismiss** — the window closes, the note persists, and the hub can find it again

## Domain Model

One entity: the **Note**.

A note is a markdown file on disk. The file is the note's identity and its only record of content.

| Property | Lives in | Notes |
|---|---|---|
| Body | The `.md` file | The complete content of the file. Nothing else is in it. |
| Title | Derived | The note's first line. Display only; never stored. |
| Filename | The filesystem | Slugified from the title, deduplicated with a numeric suffix, renamed on a debounce. |
| Created, modified | The filesystem | Filesystem timestamps. |
| Window geometry | Sidecar index | Position and size. |
| Theme | Sidecar index | Per-note. |
| Always-on-top | Sidecar index | Per-note, off by default. |
| Open | Sidecar index | Whether a window for this note is on screen. |

### Storage model

Notes are plain `.md` files in a single flat folder. Application state lives beside them in one sidecar index file, keyed by filename.

The split exists so the markdown files stay portable and clean: a note opened in any other editor contains only what was typed into it, and dragging a window never rewrites a note.

**Invariants:**

- A note's `.md` file contains user content and nothing else. No frontmatter, no metadata, no application data.
- A note with no sidecar entry is valid. It opens centered, with default theme and always-on-top off.
- Deleting a note sends its file to the operating system's trash and removes its sidecar entry.
- `open` and `exists` are independent. Closing a note's window does not delete the note.
- Filenames are unique within the notes folder.

### Rendering

The editor renders markdown inline as it is typed — syntax characters are hidden on lines the cursor is not on, and revealed on the line it is. Formatting commands insert real syntax: `Ctrl+B` on a selection produces `**bold**` in the file and bold text on screen.

Tables follow the same rule as everything else: rendered when the cursor is outside them, raw pipes when the cursor is inside.

Copying yields raw markdown source, never rendered text. This is the property the product exists to provide.

## Stack

| Layer | Choice | Why |
|---|---|---|
| Shell | Tauri v2 | Uses the operating system's web view rather than bundling a browser engine. A ~5–10 MB binary against Electron's ~150 MB, and materially lower memory across many windows — which is the architecture this product requires. Multi-window, tray, and global shortcuts are first-class. |
| Backend | Rust | Tauri's native side. Owns file I/O, the sidecar index, tray residency, global shortcuts, and window lifecycle. |
| Frontend | Svelte 5 + TypeScript | Compiles to near-nothing at runtime, which serves the efficiency priority directly, and ships real transitions — which serves the motion priority. |
| Editor | CodeMirror 6 | The only mature engine capable of inline markdown rendering with the raw source preserved in the buffer. Obsidian is built on it. Fenced-code syntax highlighting is built in. |

Two consequences worth stating plainly:

**The editor decided the stack.** No native desktop toolkit has a component that renders markdown inline while keeping raw syntax in the buffer. Requiring that behaviour requires a web view; everything above is a choice about what wraps it.

**The inline rendering layer is the project.** CodeMirror provides markdown parsing and a decoration API, not this feature. Hiding syntax on unfocused lines, revealing it on the cursor line, and keeping formatting commands writing real characters is the central engineering work.

## In Scope

What sticky.md does. Amended by decision as capability is added or removed, and verified against the code at every phase transition. Phases 1 through 5 constitute V1.

- Tray-resident application; launch-at-startup off by default and user-togglable
- A global hotkey that creates a new note window, focused and empty
- Inline markdown rendering in the note window, with syntax revealed on the cursor's line
- Formatting commands that insert real markdown syntax
- CommonMark, plus strikethrough, task lists, tables, and fenced code blocks with syntax highlighting
- Copy yields raw markdown source
- Autosave to the `.md` file on a debounce; no save action exists
- Filenames slugified from the first line, deduplicated with a numeric suffix, renamed on a debounce
- A sidecar index holding window geometry, theme, always-on-top, and open state
- Open windows restore when the application launches
- A per-note always-on-top toggle, off by default
- A hub window listing every note, from which notes are opened, focused, and deleted
- Deleting a note sends the file to the operating system's trash
- A small set of remappable keyboard shortcuts
- A theme system with several built-in themes
- Window and hub animations

## Permanent Vetoes

1. **Never becomes a knowledge base.** No tags, folders, backlinks, graph view, or links between notes. That is the vault this product exists to avoid.
2. **Note content never leaves the machine.** No accounts, no telemetry, no analytics, no sync, no crash reports carrying note text. Nothing is transmitted, ever.
3. **Content lives only in plain `.md` files the user owns.** No proprietary format, no database of record. Delete the application and every note is still readable.
4. **Never bundles its own browser engine.**
5. **Never launches at startup without explicit consent.**
6. **The free version stays fully functional.** A paid tier may add; it may never remove or gate what already shipped free. No licence key is required to use the application.
7. **Never adds AI features.** The scratchpad's value is that it is frictionless and does one thing.

## Build Phases

**Phase 1 — Foundation & Editor Core.** The Tauri shell, custom window chrome, design tokens, and a single note window. CodeMirror 6 with the inline rendering layer, formatting commands, fenced-code highlighting, tables, and copy-yields-raw. Ordered first because it carries the most risk.

**Phase 2 — Files & Persistence.** Notes as `.md` files on disk. Debounced autosave, slugified filenames with deduplication and debounced renaming, the sidecar index, and delete-to-trash.

**Phase 3 — Windows, Tray & Shortcuts.** Multiple note windows. Tray residency, the global hotkey, session restore of open windows, the per-note always-on-top toggle, the remappable shortcut set, and the launch-at-startup toggle.

**Phase 4 — The Hub.** The manager window: every note listed, opened, focused, and deleted from one place.

**Phase 5 — Theme & Motion.** The theme system, the built-in themes, and the motion pass across every surface.

**Phase 6 — Cross-Platform Expansion.** macOS and Linux build targets, and the rendering differences that come with WebKitGTK and WKWebView.

**Phase 7 — Release & Auto-Update.** The Tauri updater against GitHub Releases, prompting before it replaces anything.

This list is open-ended. Phases are appended by decision as work is approved; the numbering records the order they were defined in, not a plan with an ending.

## Deployment

Releases are published on **GitHub Releases**, each pinned to a `v*` git tag with written release notes.

Every release carries two assets:

- an **NSIS installer** — the conventional Windows experience: a wizard, a Start Menu shortcut, an entry in Add/Remove Programs
- a **portable `.zip`** — the compiled application with no installer. Extract and run. Nothing touches the registry and no administrator rights are needed.

**GitHub Actions** builds both. Pushing a `v*` tag triggers a workflow that checks out the tagged commit on a clean machine, builds, and attaches the assets to the release. Builds come from a known commit rather than from a developer's machine, and the same workflow is what makes non-Windows targets possible.

Builds are distributed **unsigned**. Windows SmartScreen warns on unsigned binaries, and that warning is disclosed to users rather than hidden. A signing certificate is an annual cost this project does not carry.

The project is licensed **GPL-3.0**. Forks must stay open; as sole copyright holder the founder retains the right to license the same code commercially.
