# DECISIONS

> **Authoritative for:** why StickyMD is the way it is. One entry per decision, append-only.
> **Never contains:** work tracking, or current policy. A decision is not a task — tasks live in `docs/STATUS.md`. Current policy lives in `MASTER.md § Permanent Vetoes` or in an item's state. Nothing may cite this file as current policy.
> **Last verified:** 2026-09-05

`Context` and `Decision` are immutable once written. `Status` changes only to record supersession. `Consequences` is append-only.

---

## ADR-001 — Notes are plain markdown files on disk
Status:     Accepted
Date:       2026-09-05
Context:    Notes could live in an application database with markdown as the content format, or as real files in a folder. The product exists because a vault is too much ceremony for a scrap, and the founder wanted notes reachable outside the app — greppable, syncable, openable in any editor.
Decision:   Each note is a `.md` file in a single flat folder. The file is the note's identity and its only record of content.
Consequences: The app can never hold a user's text hostage; deleting the app leaves every note readable. Costs file watching, rename collision handling, and title deduplication. Forces ADR-002.
            2026-09-05 — MASTER § Domain Model written to this decision.

## ADR-002 — Application state lives in a sidecar index, not in frontmatter
Status:     Accepted
Date:       2026-09-05
Context:    A note carries content plus window geometry, theme, always-on-top, and open state. Following ADR-001, content is the file. The remainder had two possible homes: YAML frontmatter inside each note, or a separate index beside them.
Decision:   One sidecar index file, keyed by filename, holds all application state. The `.md` files contain user content and nothing else.
Consequences: Note files stay portable and clean; dragging a window never rewrites a note, and a folder under sync does not churn on mouse movement. A note whose sidecar entry is missing must open with sane defaults rather than fail. Moving or renaming a note outside the app orphans its entry.

## ADR-003 — Tauri v2 rather than Electron
Status:     Accepted
Date:       2026-09-05
Context:    Inline markdown rendering requires a web view (see ADR-004), so the choice was which shell wraps it. The founder named low resource use as a top-two priority, and the product's architecture is many small windows.
Decision:   Tauri v2, using the operating system's web view.
Consequences: Roughly 5–10 MB bundled against Electron's ~150 MB, and no bundled Chromium process per window. Introduces Rust to a project whose founder does not know it. Narrows the contributor pool, accepted knowingly in ADR-005. Codified as MASTER veto 4.

## ADR-004 — Inline markdown rendering on CodeMirror 6 is the core requirement
Status:     Accepted
Date:       2026-09-05
Context:    "Renders markdown the way Obsidian does" admits three readings: a split preview pane, a render-on-blur toggle, or true inline rendering where syntax hides on unfocused lines and formatting commands insert real characters. The founder described the third. No native desktop toolkit offers a component that does it.
Decision:   Inline rendering is in scope and non-negotiable, built on CodeMirror 6. This requirement decided the stack.
Consequences: The decoration layer — hiding syntax off the cursor line, revealing it on, keeping commands writing real characters — is the central engineering work of the project, not a step within it. Fenced-code syntax highlighting comes largely free with the engine and was pulled into V1 on that basis.

## ADR-005 — Svelte 5 rather than React, trading contributor pool for efficiency
Status:     Accepted
Date:       2026-09-05
Context:    React plus Electron has perhaps ten times the pool of developers able to open a pull request. The project is open source and contributions are welcome. The founder was asked to choose knowingly.
Decision:   Svelte 5 with TypeScript. The founder stated the app being efficient matters more than receiving help on a project he will build regardless.
Consequences: Smaller runtime, built-in transitions serving the motion goal. A narrower door for contributors, accepted.

## ADR-006 — Filenames are slugified from the first line and renamed on a debounce
Status:     Accepted
Date:       2026-09-05
Context:    ADR-001 makes the notes folder browsable, so filenames should be human-meaningful rather than opaque identifiers. But the first line changes as the user types, and renaming per keystroke would hammer the disk and any sync client watching the folder.
Decision:   The filename is the slugified first line, deduplicated with a numeric suffix, applied on a debounce after typing stops or when the window closes. A note with no first line is `untitled`.
Consequences: The folder reads as a list of titles. Requires collision handling and debounce tuning. The hub displays titles without the deduplication suffix; only disk carries it.

## ADR-007 — StickyMD never adds AI features
Status:     Accepted
Date:       2026-09-05
Context:    The product exists to edit prompts bound for Claude, which makes an AI feature an obvious adjacency. The founder was asked whether this was a permanent position or a deferral, since FLOW treats a line that would be deleted on shipping as a deferred item rather than a veto.
Decision:   Permanent veto. The scratchpad's value is that it is frictionless and does one thing; an AI button is friction with a rationale attached.
Consequences: Recorded as MASTER veto 7. Allowed ADR-008 to be written in its absolute form rather than narrowed to make room for AI. A future session encountering this veto should read it as deliberate, not as an oversight to helpfully correct.

## ADR-008 — Note content never leaves the machine
Status:     Accepted
Date:       2026-09-05
Context:    A narrower version was drafted first — content is never transmitted without an explicit per-action request — specifically to leave room for a future AI feature. ADR-007 removed that need.
Decision:   Absolute. No accounts, no telemetry, no analytics, no sync, no crash reports carrying note text.
Consequences: Recorded as MASTER veto 2. Forbids share-to-gist, export-to-paste-service, and similar affordances by the same rule. Fonts and assets are vendored rather than fetched, which `CLAUDE.md § Build and release` enforces. The version check in ADR-014 transmits no note content and does not conflict.

## ADR-009 — GPL-3.0
Status:     Accepted
Date:       2026-09-05
Context:    The founder plans to publish the project and mentioned possibly offering a paid version someday. MIT would permit anyone to fork the project, close the source, and sell it.
Decision:   GPL-3.0. As sole copyright holder the founder retains the right to license the same code commercially.
Consequences: Forks must stay open; the founder's own commercial options are unaffected. Every bundled asset must be redistributable under terms compatible with handing recipients the same rights — which is what disqualified Redaction in ADR-011.

## ADR-010 — Builds are distributed unsigned
Status:     Accepted
Date:       2026-09-05
Context:    Unsigned Windows binaries trigger a SmartScreen warning. A certificate costs roughly $200–400 per year, which the project does not carry.
Decision:   Ship unsigned. The README documents the warning honestly.
Consequences: A stranger's first experience includes a security warning they must click through. Revisitable if the project ever attracts funding.

## ADR-011 — Handjet is the display face; Redaction was rejected on licensing
Status:     Accepted
Date:       2026-09-05
Context:    The founder chose Redaction, drawn to its seven grades of halftone degradation. Multiple secondary sources — including the Use & Modify libre-font directory — state Redaction is under the SIL Open Font License. The licence file shipped in the actual download is MCKL's App Embedding EULA, which fails on four independent grounds: it defines "Application" as mobile-only (§1a), forbids making the font available on a network reachable by more than one device (§4e), forbids format conversion and subsetting (§4b), and licenses by monthly unique users against a sales receipt (§3). It also cannot be reconciled with ADR-009, which requires granting recipients redistribution rights.
Decision:   Handjet, SIL OFL, verified in `google/fonts` as `Handjet[ELGR,ELSH,wght].ttf`. Its variable element-grid and element-shape axes deliver the degradation the founder wanted as a continuous axis rather than seven fixed files.
Consequences: One variable font file instead of twenty-one statics. Degradation becomes animatable, which opens an identity moment the seven-grade approach could not. The tone shifts from decayed legal serif to constructed digital, accepted knowingly. Chakra Petch was the founder's stated fallback and remains a candidate for an alternate built-in theme — see STATUS SMD-009. Secondary font directories are not a licence; only the file shipped with the download is.

## ADR-012 — Themes are token data; there is no plugin or component-replacement API
Status:     Accepted
Date:       2026-09-05
Context:    The founder wanted users to be able to make the interface their own and suggested components should be swappable. Swappable components means a component API — a public contract supported forever, plus third-party code execution inside an app whose pitch is that it is small and does one thing.
Decision:   A theme is a complete set of design token values and nothing else. Themes never replace components, ship CSS, or execute code.
Consequences: Most of the customisation benefit at a fraction of the cost and risk. Forces the token architecture to be correct from the first commit — built-in themes that hardcode values are not a smaller version of a theme system but a rewrite waiting to happen. Recorded as `docs/DESIGN.md` principles 5, 6 and 8.

## ADR-013 — Windows first, cross-platform later, with four disciplines held from the start
Status:     Accepted
Date:       2026-09-05
Context:    The founder wants cross-platform eventually and asked whether deferring it would make it harder. Tauri v2 is cross-platform natively; the expensive part of a port is not the build target but retrofitting platform assumptions baked in throughout.
Decision:   Build for Windows now. Hold four disciplines from the first commit: modifier keys resolved through an abstraction, no string-concatenated paths, deletion through a trash abstraction, custom window chrome everywhere.
Consequences: Porting becomes largely a build-target change plus a rendering bug hunt on WebKitGTK, which cannot be predicted and must simply happen. Recorded as `CLAUDE.md § Cross-platform discipline`. Windows-only is explicitly not a veto.

## ADR-014 — Auto-update ships after V1
Status:     Accepted
Date:       2026-09-05
Context:    The Tauri updater would check GitHub Releases and offer new versions. It transmits no note content and so does not conflict with ADR-008, but it is the application making a network request on its own initiative. The founder approved the feature.
Decision:   Not in V1. V1 makes zero network requests; updates are downloaded manually from GitHub Releases. The updater ships in Phase 7 — Release & Auto-Update.
Consequences: "V1 makes zero network requests" is a promise that needs no footnote. The updater is useless until a second release exists anyway. When it ships it must prompt before replacing anything, never install silently.

## ADR-015 — FLOW_SYSTEM.md is excluded from the repository
Status:     Accepted
Date:       2026-09-05
Context:    Gitignoring the FLOW documentation as a whole was considered and rejected: FLOW's central contract is that a change and the docs it invalidates land in the same commit, which is meaningless if the docs are untracked, and reconciliation works by diffing `git log` against STATUS.md. The founder's actual concern was narrower — FLOW is his own system, still in development, and he does not want to publish it unfinished.
Decision:   Track `CLAUDE.md`, `MASTER.md`, `docs/`, `.githooks/` and `.gitattributes`. Exclude only `FLOW_SYSTEM.md`.
Consequences: All four of FLOW's nets remain intact. A visitor sees a repository with a scope document and a decision log, not the spec that produced them. `CLAUDE.md` names the file as local-only so the reference is not dangling for someone who clones.
