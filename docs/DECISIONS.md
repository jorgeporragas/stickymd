# DECISIONS

> **Authoritative for:** why sticky.md is the way it is. One entry per decision, append-only.
> **Never contains:** work tracking, or current policy. A decision is not a task — tasks live in `docs/STATUS.md`. Current policy lives in `MASTER.md § Permanent Vetoes` or in an item's state. Nothing may cite this file as current policy.
> **Last verified:** 2026-09-06

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
            2026-09-05 — Implemented as `.sticky-index.json` in the notes folder. Three properties the decision implied but did not state: writes go through a temporary file and a rename, so an interrupted write cannot leave a half-written index; an unreadable index is moved aside rather than overwritten, because a file we failed to parse may still be recoverable; and read-modify-write cycles are serialized by a mutex, since two note windows saving at once would each load, apply their own change, and write back, with the last one erasing the other. Renaming a note carries its entry — a retitled note is the same note.

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
            2026-09-05 — Implemented. Two cases the decision did not anticipate: Windows refuses `con`, `nul`, `com1` and eighteen other stems whatever the extension, so those get a `-note` suffix; and a title that slugifies to the stem a note already holds must *not* trigger a rename, or every save would churn the file. Naming lives in Rust because deduplication has to see the folder.

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
Status:     Superseded by ADR-029 as to the face. The licensing finding on Redaction stands and is why Redaction is still not an option.
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

## ADR-016 — Token values live in code; DESIGN.md owns only the contract
Status:     Accepted
Date:       2026-09-05
Context:    DESIGN.md was drafted during Planning with the full token contract and every value of the Frost theme written out. When `src/lib/tokens/tokens.css` was created during the Phase 1 scaffold, every one of those values existed in two files. Changing a colour would have required editing both, and one would eventually disagree with the other.
Decision:   `docs/DESIGN.md` is authoritative for which tokens exist and what role each plays. `src/lib/tokens/tokens.css` is the single source for every value. DESIGN links to it and never restates a value.
Consequences: A reader of DESIGN.md cannot see the palette without opening the code, which is the cost of the rule. Added to DESIGN's never-allowed list so the duplication cannot creep back. Window dimensions are the one value DESIGN still names, because they are a window property in `src-tauri/tauri.conf.json` and JSON carries no comment to point back with — DESIGN records that asymmetry explicitly.
            2026-09-05 — docs/DESIGN.md amended; values removed in commit 2a5b11b.

## ADR-017 — The application mark is the founder's hand-drawn blob
Status:     Accepted
Date:       2026-09-05
Context:    Four directions were proposed from the existing design system — a smooth blob, a peeling blob, a blob carrying Handjet's dot motif, and a glass outline. The founder had independently reached for "blob" from the word *sticky*, asked for something less splattered than the Claude mark, and then supplied a hand drawing: an irregular outline with a spike at the top, lobes down the left, and two leg-like protrusions at the bottom. He asked for the ruggedness to be kept.
Decision:   The mark is that drawing, traced as a 47-anchor Catmull-Rom spline at tension 0.88 — high enough to smooth the line, low enough to keep the corners from relaxing into a generic blob. Filled with an aqua gradient. `assets/icon/stickymd.svg` is the single source; every platform size is generated from it.
Consequences: Legibility at 16px was verified by rasterising to a real pixel grid rather than by scaling the vector, which cannot show the loss. The silhouette survives: the top spike and irregular edge remain, so it reads as this blob rather than a generic dot. Android and iOS icon sets that `tauri icon` also produces were deleted — neither platform is in scope, and both regenerate with one command. Regenerate with `npx tauri icon assets/icon/stickymd.svg` after any change to the source.

## ADR-018 — Palette: one aqua accent, seven note tints
Status:     Accepted; the accent is superseded by ADR-025. The seven tints stand.
Date:       2026-09-05
Context:    The design brief is Frutiger Aero warmth held inside Rams restraint, on a colourless frosted surface. Three accent candidates were put forward — aqua, a greener spring, and a cooler sky blue — alongside a set of note tints. The founder approved the palette as proposed, which carried a recommendation to keep aqua.
Decision:   `--accent` stays aqua `#2FB6D9`. Spring reads botanical rather than interface and fights the quiet-content rule on a focus ring; sky is the least distinctive of the three. Note tints are Clear (the default), Sun `#FFE9A3`, Spring `#C7F0D8`, Aqua `#B8ECF7`, Sky `#CFE4FD`, Lilac `#DCD4F7`, Blush `#FBD5E0`.
Consequences: The tints are pale by necessity, not by taste: they are layered over frosted glass, so `--ink-primary` must hold 4.5:1 against a tint sitting over an arbitrary wallpaper. A saturated Post-It yellow fails that the moment it goes translucent. The values land in `src/lib/tokens/tokens.css` when per-note colour is built (STATUS SMD-021); until then this record is the only place they exist, and it is not policy — `docs/DESIGN.md` and the token file become authoritative once implemented.

## ADR-019 — The product is styled "sticky.md"
Status:     Accepted
Date:       2026-09-05
Context:    The project was written throughout as "StickyMD". The founder asked for it to be styled "sticky.md" — lowercase, with the markdown file extension as part of the name rather than a suffix bolted on.
Decision:   "sticky.md" is the product's name wherever a person reads it: the window title, the installer, documentation prose, and the Start Menu. Machine identifiers keep the flat form — the npm package, the Cargo crate, the repository, and the bundle identifier `com.stickymd.app` all stay `stickymd`.
Consequences: `productName` in `tauri.conf.json` is "sticky.md", and `mainBinaryName` is set to `stickymd` so the executable does not become `sticky.md.exe` — a name with a false second extension that reads as a file rather than a program.
            Historical decision records keep the name they were written under. ADR-007's title still says "StickyMD" because Context, Decision and titles are immutable: a record states what was decided at the time, and rewriting it would make the log claim a name that did not yet exist. Only this file's header was amended.
            One measurement in STATUS SMD-006 quotes the literal string "StickyMD scratchpad". It was deliberately left unrenamed — the number is the width of that exact string, and changing it would make a recorded measurement false.
            2026-09-05 — MASTER, CLAUDE, DESIGN and STATUS amended.

## ADR-020 — Notes live in Documents/sticky.md
Status:     Accepted
Date:       2026-09-05
Context:    Phase 2 needs a default location for the notes folder, and ADR-001 constrains the choice: notes are files the user can open in another editor, grep, and sync. An application data directory — `%APPDATA%` or `%LOCALAPPDATA%` on Windows — is the conventional home for app-managed files, but it is a folder most people never open, which would make "your notes are files you own" true in principle and useless in practice. Making the location configurable is deferred (STATUS SMD-003), so this default is the only location.
Decision:   The notes folder is `sticky.md` inside the user's Documents directory, resolved through Tauri's path API rather than a constructed string, and created on first use.
Consequences: The folder is somewhere a person can find without being told where to look, which is what ADR-001 was for. On machines where Documents is redirected into OneDrive the notes will sync — that is the user's own arrangement and does not touch MASTER veto 2, which forbids *the application* transmitting content, not the user syncing their own files.
            Changing this later means moving a user's notes, so it is cheap to revisit only while no one has any. Note names arriving from the frontend are untrusted and validated before being joined to this path.

## ADR-021 — Shortcuts that the web view or the keyboard layout would eat are registered globally
Status:     Accepted
Date:       2026-09-05
Context:    "New note" was first bound inside the editor as `Ctrl+N` and did nothing. WebView2 keeps that chord as a browser accelerator and never passes it to the page; wry can disable that, but Tauri 2.11.5 does not expose the setting. Moving it to `Ctrl+Alt+N` also did nothing: the development machine carries the Latin American keyboard layout, where Windows treats `Ctrl+Alt` as AltGr and consumes the combination to compose a character. Two chords, two unrelated causes, both silent.
Decision:   Shortcuts that either hazard can reach are registered with the operating system from Rust rather than bound in the web view. The new-note shortcut is `Ctrl+Shift+Space`, and the in-app `Mod-` binding for it was removed rather than moved a third time. In-editor bindings remain appropriate for text formatting, which touches neither hazard.
Consequences: `MASTER.md § Core Loop` already made a global hotkey the way a note is summoned; this makes it the only way, which is simpler than maintaining two paths that fail differently. A global registration claims the chord from every application on the machine, so the default is a deliberately uncommon one, and failing to claim it is reported rather than raised — another application may already hold it, and the app runs fine without it. Remappable shortcuts (`MASTER.md § In Scope`) become more important than they looked: a user whose chord is already taken currently has no remedy.

## ADR-022 — Application settings live in the config directory, not the notes folder
Status:     Accepted
Date:       2026-09-06
Context:    `MASTER.md § In Scope` requires a small set of remappable shortcuts, and nothing in scope is a settings window. The remapping therefore has to be a file the user edits. ADR-002 already put per-note state in a sidecar index beside the notes; the question was whether application settings belong there too.
Decision:   Application settings live in `settings.json` in the platform's application config directory, resolved through Tauri's path API. The notes folder holds notes and the sidecar index that describes them, and nothing else.
Consequences: The notes folder stays something a person can open without meeting the application's internals — which is what ADR-001 was for. The settings file is written with defaults the first time it is read, so there is always something to edit rather than a format to guess at. A settings file that cannot be parsed falls back to defaults rather than stopping the application: a typo in a hand-edited file should cost a custom shortcut, not access to notes.
            This is a file, not a settings surface. It satisfies "remappable" but only for someone willing to edit JSON, and the tray now names the file when a shortcut cannot be claimed. A real settings window is not in scope for V1 and is not logged as one — see STATUS SMD-046.

## ADR-023 — Deleting a note closes its window
Status:     Accepted
Date:       2026-09-06
Context:    A note deleted while its window was open left the window on screen, and the window would write the note back on its next save — it still held the text and the filename. Two behaviours were defensible: the open window wins and the note survives, or the delete wins and the window goes. It was logged as STATUS SMD-031 rather than settled quietly, because either is coherent and only the founder could say which the product means.
Decision:   The delete wins. Deleting a note closes the window showing it, and the window is destroyed rather than asked to close.
Consequences: Destroying skips the frontend's close handler, which flushes a pending save — going through the normal close path would resurrect the file that was just deleted. Unsaved edits in that window are lost, which is correct: the note was deleted.
            The window is closed before the file is trashed rather than after, so a save landing between the two cannot write the note back.

## ADR-024 — A theme is application-wide; a note carries a tint
Status:     Accepted
Date:       2026-09-06
Context:    Phase 5 needed the dark theme, and a dark theme cannot ship alone — without a way to choose it, it is unreachable code. That forced the question of what a theme is here. Two readings of earlier decisions were both defensible: ADR-018's seven note tints and `Note.theme` living per-note in the sidecar index point at per-note themes; `MASTER.md § In Scope` asking for "several built-in themes" points at an application-wide one. The founder was given both plus a third and left the choice to the recommendation.
Decision:   They are two different things. The **theme** is application-wide and decides ink, accent, edges and shadows — Frost and Dark. A **tint** is per-note and decides only what that note's surface is painted with. A note keeps its tint in either theme.
Consequences: `MASTER.md § Domain Model` amended: the per-note field is a tint, not a theme. The sidecar index field is renamed to match, and an index written before this reads `tint` as absent, which is Clear — the same as the default.
            Tint selectors are qualified by theme (`[data-theme='dark'][data-tint='sun']`). A tint block and a theme block carry equal specificity, so an unqualified tint would override the theme's surface entirely and a dark note would come back white.
            Every tint alpha is computed rather than chosen: the ink must hold 4.5:1 over a worst-case wallpaper. In the light theme a coloured tint needs *more* alpha than Clear, because it is darker than white — 0.58 to 0.63 against Clear's 0.55. In the dark theme the worst case inverts to a white wallpaper and the alphas land near 0.78.
            The dark theme's tint is 0.63, not the 0.45 sketched when DESIGN was written. Dark ink on light and light ink on dark are not symmetrical problems, and 0.45 measured 2.72:1 — well under AA. The sketch was never checked.

## ADR-025 — Aero is dropped; colour becomes functional
Status:     Accepted
Date:       2026-09-06
Context:    The design brief was Frutiger Aero warmth held inside Rams restraint. The founder is moving off Aero as a philosophy, keeping Rams, and naming two effects he wants the app to be built around — typographic scrambling, and line boil. His reading of what exists: it "didn't feel very Aero yet anyways", which matches what was actually built — the frosted surface reads as glass, but nothing else was carrying Aero.
            He asked for one change now: the colour. Everything else waits until the new direction is worked out.
Decision:   The Aero accent is removed. There is no brand colour. The interface is ink on a tinted surface, and a hue appears only where colour is what tells the user what something does — traffic-light reading, because it needs no learning.
            Only red is defined, as `--signal-danger`, on the one control that takes a note away. Amber and green have nothing to say yet and are not defined in advance. A colour with no job is how a palette turns decorative.
            Note tints are not affected and stay exactly as ADR-018 set them. They are the note's own paper, not interface colour.
            The two effects the founder named are recorded but not built: STATUS SMD-051 and SMD-052.
Consequences: `--accent`, `--accent-hover`, `--accent-glow`, `--ink-on-accent` and the aqua ramp are gone from the token contract. What read them now reads ink, with two replacements worth naming: `--selection` for the editor's selection wash, which was the accent glow and is a role of its own; and the always-on-top pin, which showed "on" as a colour and now shows it as a filled chip. That is the better control anyway — a state you can see without knowing what the colour meant.
            Fenced code loses its hue with everything else. Token classes are told apart by weight and by how dark they are. In a scratchpad a fenced block is something pasted in rather than something being written, so this costs little; if it turns out to cost more than expected, the ADR to write is a narrow one about code, not a return to an accent.
            Every signal value is computed rather than picked: each clears 4.5:1 against the worst of the seven tints in its own theme — 5.73:1 on Frost, 5.25:1 on Dark — so a signal stays legible on any note.
            `docs/DESIGN.md` principle 1 no longer says the surface is Aero. The frosted surface itself is unchanged and is not what this decision is about: glass here is a window property (principle 3), and dropping a visual philosophy does not un-frost a window.

## ADR-026 — The tint swatches take the Aqua bezel, not the Aqua gloss
Status:     Accepted; the refusal of the gloss is superseded by ADR-027, and the greying-out behaviour by ADR-031. The rim and the seat stand.
Date:       2026-09-06
Context:    The founder asked for the tint picker to look like the Mac OS X Aqua traffic lights. Three treatments were put to him a commit after ADR-025 dropped Aero: the flat dots as built, a bezel, and full candy — radial fill, specular highlight, drop shadow. He chose the bezel, in his words liking the candy and picking the other anyway.
Decision:   What the traffic lights are actually made of, in the order it matters: a rim in the fill's own hue rather than a neutral border; a bezel seating the disc in the surface; and going grey when the window is not in use. All three are taken. The gloss is not.
Consequences: `--swatch-rim` is `color-mix(in oklab, currentColor 72%, var(--rim-shade))`, so one declaration rims every tint and adding a tint needs no second value. Each swatch carries its colour as `color` rather than `background`, which is what makes that work. Verified rendering in the running application, not assumed: `color-mix` resolves in WebView2 and all seven rims come out as darkened versions of their own hue.
            Clear is drawn as rim and bezel with nothing in them. An empty ring says "no colour" without a slash or a label.
            Greying out while the chrome is receded is the traffic lights' own behaviour and `docs/DESIGN.md` principle 2 asked for it independently. A tinted note keeps its swatch visible either way — findable without a colour calling for attention.
            The gloss was refused for one reason worth keeping: it would be the only glossy object in the application, which reads as an import rather than as a system, and it is the language ADR-025 had just removed. If it is ever wanted, it needs its own decision — this one is not a licence for it.
            The window's own controls take the disc too. One bezelled control in a row of flat ones reads as an accident rather than as an emphasis, which the founder said on seeing it, so the pin and the close button are seated discs as well and the swatch grows to `--space-6` to sit level with them. The three together are the traffic-light cluster the reference was reaching for in the first place. Their rim is the exception to reading `currentColor`: they carry no tint, and a rim mixed from the glyph's own ink would draw a hard ring around a control whose job is to stay quiet.
            Pinned inverts — dark disc, glyph in the note's own paper — rather than filling a little harder. Once both were discs, a stronger fill was too close to hover to tell apart, and an inverted control is the plainest "held down" there is, with no colour needed to say it.
            The disc itself lives in `src/app.css` as `.lozenge`, not in either component. The moment the second one wanted it, it stopped being one component's styling.
            The swatch's palette stays at `--space-4` with `--space-1` gaps. The traffic lights' proportion is nearer 12px with 8px gaps; that was offered and not taken, and it is a one-line change if the proportion turns out to matter more than the size.

## ADR-027 — The direction is Aqua
Status:     Accepted
Date:       2026-09-06
Context:    ADR-025 dropped Aero on the founder's instruction and ADR-026 refused the gloss on the grounds that it was the language just removed. Having seen the bezel in the running application, the founder named what he had been reaching for the whole time: not Aero, Aqua. His argument, and it holds — Aqua *is* Rams with personality. Its chrome descends from Braun: neutral, restrained, colour concentrated in the traffic lights and almost nowhere else. And it is two decades old, which makes it retro rather than dated.
Decision:   Aqua is the direction. The lozenges take the full gloss: a fill lighter at the top than the bottom, a rim in the disc's own hue, a seat shading the inside of its lower edge, and a specular highlight. Every disc in the window is one diameter — `--space-4`, the size the palette's dots already were, which the founder picked as correct.
Consequences: This does not restore an accent colour, and nothing in ADR-025's colour rule is reversed. That rule turns out to describe Aqua rather than to fight it: neutral chrome, with hue reserved for the small round things. The two decisions sit together, which is why only ADR-026's refusal is superseded and not ADR-025.
            The glyph sits *above* the specular, which is where Aqua drew it too. At nine pixels a highlight across the top of a glyph is the difference between reading it and guessing.
            Controls shrink from `--space-6` to `--space-4`, and their glyphs from 12px to 9px — about 0.58 of the diameter, which is the traffic lights' own proportion. A 16px target is small; it is acceptable here because the bar around them is a drag region, so a miss drags the window rather than doing something else.
            `--gloss-tinted`, `--swatch-rim`, `--rim-shade` and `--rim-light` are primitives, defined once, because they are mixed from `currentColor` and therefore say the same thing in every theme. `--gloss-seat`, `--gloss-specular` and `--gloss-neutral` are theme-level: a specular at Frost's strength reads as a blown-out spot on a dark panel rather than as a curve.
            That last point is a rule the hard way. ADR-026's tokens were written into the Frost block, so on Dark they were undefined and `border-color` fell back to its initial value — `currentColor` — rimming each disc in its own fill. It looked plausible in a screenshot and was reported as working. **A token that derives from `currentColor` belongs in the primitives; a token whose value depends on what is behind it belongs in the theme. And a token has to be read in both themes before either is called verified.**

## ADR-028 — The palette, and a lit control instead of an inverted one
Status:     Accepted
Date:       2026-09-06
Context:    The founder set a palette — ten stops running from a yellow-green through teal to a deep blue — and said the always-on-top pin looked wrong inverted. He asked for it to take a colour from the palette when held, and to stop receding with the rest of the chrome.
Decision:   The ramp lands as `--tide-100` to `--tide-1000`, primitives in the founder's own order, light to dark. One role reads from it: `--signal-engaged`, a control holding a setting on, at `--tide-600`.
            The pinned control is a lit lozenge. It takes its hue as `color`, which feeds the same `--gloss-tinted` and `--swatch-rim` every tint swatch uses, so it is one kind of object lit rather than a second kind of control.
Consequences: The stop was chosen by measurement, and the measurement decided the glyph as well. The gloss lightens the top of a disc to 45% of its colour over white, which puts a white glyph under 2.3:1 at every stop on this ramp — so the glyph is dark, and a dark glyph clears 4:1 through the teal stops and falls away past them. `--tide-600` is the darkest stop that still holds, at 4.06:1, and it is the middle of the ramp.
            The glyph cannot read `currentColor` in this state, because that is now the fill's own hue. It is the hue taken most of the way to black, which is how Aqua drew a traffic light's glyph and holds against both ends of the gradient.
            **Amended the same day, at the founder's pick:** he found `--tide-600` too strong at full strength, was shown the ramp and a dilution series rendered as the control itself, and chose `#78bec0` — `--tide-600` at 65% toward the light surface. `--signal-engaged` reads `--tide-600-diluted`, which is written out rather than mixed at read time so it cannot drift with a note's tint. Diluting rather than lowering `opacity` was deliberate: opacity fades the rim, the seat and the specular along with the fill, and a lozenge with a faded rim reads as switched off rather than as quietly on. The glyph gains contrast going paler — 7.34:1 against the new fill, against 4.96:1 before.
            The nine stops with no role are values, not interface colours. `docs/DESIGN.md` principle 6 is what permits them: primitives name values, semantics name roles, and a stop nothing reaches for is not a colour in the interface. The rule from ADR-025 is unchanged — a hue appears only where colour carries the meaning.
            Red stays where it is. `--signal-danger` is not on this ramp and has no equivalent on it; deleting a note is the other thing colour is doing here, and it is not a green.
            "No longer fades" needed no change: `.control.active` has kept `opacity: 1` since the chrome was built, for the same reason a tinted swatch stays visible. A pin you cannot see is a note you do not know is floating.

## ADR-029 — Departure Mono replaces Handjet as the display face
Status:     Accepted
Date:       2026-09-06
Context:    The founder asked for display alternatives in Redaction's spirit and picked Departure Mono — a monospaced pixel face by Helena Zhang, SIL OFL, verified from the licence file shipped in the release rather than from a directory listing, which is the lesson ADR-011 paid for. It replaces Handjet everywhere: app identity, hub headers, empty states.
Decision:   `--font-display` is `'Departure Mono', ui-monospace, monospace`. Handjet's files, licence and axis tokens are removed rather than left for a face nothing uses.
Consequences: Version 1.500, one 22 kB woff2, vendored whole. Geist and Martian Mono are subset to latin and latin-ext; this one arrives as a single file covering Latin, Cyrillic and Greek, and subsetting it would cost a build step and a dependency to save a few kilobytes.
            The degradation axis goes with Handjet. `ELGR` and `ELSH` made decay a continuous, animatable thing, and ADR-011 counted that as an identity moment the seven-grade approach could not offer. Departure Mono has no axes: degradation is now a static property of the face. That also removes the natural home for STATUS SMD-051, typographic scrambling, which will need a different mechanism if it is still wanted.
            It ships one weight, 400, so `font-synthesis: none` is set on the body. A synthesised bold on a pixel face thickens strokes off the grid it is drawn on and stops looking like pixels. The hub's title had been leaning on Handjet's weight token and briefly rendered faux-bold; it names `--weight-body` now.
            The face is monospaced, which is a real change of voice — terminal rather than constructed-digital. Accepted knowingly, as the tone shift in ADR-011 was.
            `docs/DESIGN.md`'s rule that display type never appears inside a note is unchanged. It was never about which face.

## ADR-030 — A fenced code block is a dark panel, and the one place hue is not a signal
Status:     Accepted
Date:       2026-09-06
Context:    The founder reported three things about fenced code from using the build: it has no surface separating it from prose, the text reads much larger than body text, and there is no syntax colour. He suggested the panel could be black regardless of theme, since developers rarely read code on a light background.
Decision:   A fenced block is painted as a dark rounded panel in both themes, set in the mono face at `--font-size-code` with `--line-height-code`, and its tokens are told apart by hue drawn from the `--tide-*` ramp.
Consequences: Two of the three complaints were defects rather than taste. `--font-size-code` existed but reached only tables, so fenced code inherited body size; and at the same pixel size Martian Mono sets a line 52% wider than Geist — 10.5px per character against 8.23 — so code read as larger because it was far wider, not taller. It is 13px now, an advance 11% wider than the prose rather than 28%. The face and size are set on the panel rather than on a highlight tag, because `monospace` tags only *inline* code: the contents of a fenced block are tagged by the language inside it and never saw either.
            The missing colour was mine. ADR-025 took the hues out of the highlight style along with the accent, and the commit that did it recorded that as the part most likely to be wrong. It was.
            This is the one place in the application where a hue is not a signal, and it is still not decoration: inside a block, colour tells token classes apart, which is the functional use ADR-025 permits. Drawing it from the same ramp as everything else is what keeps a fenced block reading as part of this application rather than as a theme imported from another one. Every value clears 4.5:1 against both panels; `--tide-700` and darker do not, and are excluded for text.
            The panel is darker than the note on Frost and *lighter* than it on Dark. On a dark surface a darker panel reads as a hole rather than as a block set into the page.
            It is painted with line decorations, not a block widget. A widget would replace the source and the block would stop being editable in place, which breaks the rule that markdown stays in the buffer at all times. That is also why this can be a `ViewPlugin` where `tableView.ts` cannot: `docs/FIXES.md` bars decorations that replace line breaks, not classes on lines that are already there.
            It is a departure from principle 1, which keeps note content quiet, and it is a deliberate one: a fenced block is the one thing in a note that is usually not the user's own prose but something pasted in to be read as code.

## ADR-031 — The seated controls are the window's; the hub's are not
Status:     Accepted
Date:       2026-09-06
Context:    Three reports from the founder using the build, which turned out to be one question: what belongs to the family of seated Aqua controls. He found the tint swatch staying visible on a coloured note wrong; he found the hub's delete button, which had just joined the family, wrong in it; and he found the window's controls staying grey beside a coloured window wrong the other way.
Decision:   The family is the note window's own chrome, and it wears the note's colour. Every control on a note window takes its hue from the note's tint — disc, rim and glyph — as the swatches and the lit pin already did. Nothing outside that window joins the family: the hub's delete control is flat, and larger, and secondary.
            The tint swatch recedes with the rest of the chrome.
Consequences: This reverses two things decided earlier, both on the founder's own report, and both were mine rather than his to begin with. ADR-026 had the swatch grey out while staying visible, on the reasoning that a tinted note's swatch was chrome carrying information. It was — but the information is already there in full view: the note is the colour. A control that will not go away is a worse trade than one you hover for, and `docs/DESIGN.md` principle 2's exemption is narrower than I read it.
            SMD-061 had made the delete control a seated one. Wrong family: those are a note's own chrome, and this acts on a row in a list. It is flat, `--space-5` (25% larger — the spacing scale is 4px-based and 18px is not on it, so the founder's 15–20% is rounded up rather than invented), and still answers a press. Pressable and bezelled are different things.
            `--space-5` is new to the scale. It was simply absent; 20px is a 4px step like every other.
            A control's hue comes from `:root[data-tint]` — the bare attribute, because `applyTint` removes it for Clear rather than setting it to a value. That block is deliberately *not* qualified by theme, unlike the tint blocks beneath it: it declares no surface token, so there is nothing for a theme block to lose a fight with. Both themes work from one recipe because the disc carries its own hue and the glyph is derived from it, which is the same reason the lit pin needed no per-theme answer.

## ADR-032 — JetBrains Mono replaces Martian Mono as the code face
Status:     Accepted
Date:       2026-09-06
Context:    The founder reported monospaced text reading larger than prose twice, the second time after its size had already been reduced. Measured with both faces forced to load first: at 13px against 15px prose, Martian Mono has the *same* x-height and runs 35% wider for the same sentence. The glyphs were never taller; there was simply far more of them, and that is what reads as a larger size. No size fixes both — at 12px the width gap closes to 25% while the glyphs go 8% shorter than the prose.
Decision:   The code face is JetBrains Mono, at 13px. There it has the same x-height as the prose and runs 16% wider, against Martian Mono's 35%.
Consequences: One 92 kB woff2 replaces two subset files totalling 63 kB — 29 kB more, accepted, because the alternative was a face that cannot be made to sit correctly beside the prose at any size. SIL OFL, read from the file in the release rather than a directory listing.
            One weight and no italic. `font-synthesis: none` is on the body, so an italic that is not in the file simply does not appear — which is why the comment style no longer asks for one. Colour carries comments on its own.
            A measurement lesson, recorded because it produced a wrong answer that nearly shipped: **fonts load lazily, so a face that is not in use on the page is not loaded, and canvas measurement of it silently falls back to another face.** The first x-height numbers were taken on a note with no code in it and said mono was 15% *smaller* per em; with the face loaded it is 15% larger. Force the face to load before measuring it.

## ADR-033 — Settings get a window of their own
Status:     Accepted
Date:       2026-09-06
Context:    Every setting the application has was reachable only by hand-editing `settings.json` (ADR-022), which its own record admitted satisfies "remappable" for someone willing to open a text editor and nobody else. The founder chose a window of its own over a panel in the hub or an expanded tray menu, and chose that a notes-folder change asks at the time what should happen to the notes already in the old folder.
Decision:   A third window type, `settings`, built from its config entry like the others and opened from the tray. It carries theme, notes folder, the new-note shortcut and launch-at-startup, and names the settings file's own path for anyone who still wants to edit it.
Consequences: The hub stays about notes. That was the argument for a separate window and it is the same one that keeps note windows and the hub apart: one window type per job.
            Every command behind it is a whole operation rather than a field write — `preferences.rs`, kept apart from `settings.rs` because that one is a serialisation concern and this one is a user interface with a file move in it. Changing the theme tells every open window; changing the folder can move notes. A frontend setting fields one at a time would have to know all of that.
            `Settings.notes_folder` is stored absent rather than resolved when the user has never chosen one, so a default-following install keeps following the default if their Documents folder ever moves. `notes_dir` does not fall back to the default when a configured folder has gone: writing notes somewhere the user is not looking is worse than reporting that the folder is missing.
            The folder move renames first and only copies across volumes, and never removes the original before the copy succeeds — an interrupted move leaves the note in the old folder rather than nowhere. A name already taken in the destination is not overwritten; the note arrives beside it with a suffix.
            The global shortcut can now change at runtime, which the old registration could not survive: its handler compared the pressed chord against one captured at startup, so a changed chord would have gone deaf. The handler no longer checks which chord fired — only one is ever registered — and the chord registration is separate from the plugin's installation.
            `tauri-plugin-dialog` is a new dependency, for the folder picker. A notes folder typed by hand is a notes folder that can be wrong.

## ADR-034 — The radial menu holds actions that already exist
Status:     Accepted; its contents are superseded by ADR-038 and its refusal of `backdrop-filter` by ADR-039. The mechanism — the ring, the clamped centre, the centre label, the dismissal — stands unchanged, which is what it was built to be.
Date:       2026-09-06
Context:    The founder asked for a ring of glass bubbles on right-click, as a home for anything that cannot sit cleanly on the chrome. Building it without direction on its contents risked a large rework, so it was built as a mechanism first.
Decision:   Right-click on a note window opens a ring of seated bubbles at the pointer: New note, All notes, Settings, and the always-on-top pin. Every one of them is an action reachable elsewhere already — the ring is a faster way to them, not a second set of capabilities.
Consequences: Which actions are in the ring is one array, so changing it is cheap. That was the point of building it this way: the mechanism is the work, and the contents are a preference the founder can change without touching any of it.
            It suppresses the web view's own context menu, which would otherwise appear beside it.
            The ring's centre is pulled back from the window's edges. `.surface` clips what leaves it, so a ring opened in a corner would lose half its bubbles; it opens beside the pointer instead of being cut.
            The hovered action's name appears in the middle of the ring — the one place nothing else occupies, and the only way a ring of glyphs says what it does without a legend around it.
            The bubbles are the same seated lozenge as every other control, at `--space-8`. SMD-038's original note expected them to need `backdrop-filter`, on the reasoning that a bubble sits over the app's own content where that filter is legitimate. They do not: the lozenge is an opaque gloss, so there is nothing to see through and nothing to blur.
            A transparent backdrop catches the click that dismisses it, and Escape does too. A menu that can only be dismissed by choosing something has taken the window hostage.

## ADR-035 — Dark wears the bezel; Frost wears the gloss
Status:     Accepted
Date:       2026-09-06
Context:    The founder found the glossy controls wrong on the dark window and asked for the bezel back — the treatment ADR-026 chose and ADR-027 replaced — as an exception for the dark theme. The complaint is sound: a glossy disc needs light to be glossy about. Frost has plenty. On a dark window the gradient and the specular have nothing to catch, and read as plastic stuck to the surface rather than glass set into it.
Decision:   Dark controls are flat, seated by a bright hairline along the top inside edge and a shade along the bottom. Frost keeps the gloss. Neither theme gains a second control.
Consequences: This is entirely token values. `--gloss-seat` becomes the bezel, `--gloss-specular` becomes transparent so the overlay paints nothing, and the fills become flat colours instead of gradients. No component changed, and no component knows which theme it is in.
            That is `docs/DESIGN.md` principle 8 being cashed rather than asserted: a theme is a complete set of token values and nothing else. The founder asked for a bezel *component*, and a component would have been the wrong shape — two controls to keep in step, diverging the first time one gained a state the other did not.
            The pressed state needed one addition, `--gloss-seat-pressed`, because a bezel inverts when pushed: the light moves to the bottom edge. Applied by the shared treatment rather than by the theme, since pressing is a state and not a palette.
            ADR-026 and ADR-027 both stand. The gloss was not a mistake and the bezel was not a regression; they are the right answer on different grounds, which is what a theme is for.

## ADR-036 — The dark tints are sized against a mid backdrop, not a white one
Status:     Accepted
Date:       2026-09-06
Context:    Computed against a white wallpaper — the rule since ADR-018 — the dark tints came out strong and nearly opaque, and the founder said they no longer read as a transparent window. His reasoning: someone running a dark desktop mostly has dark things behind the window, so sizing for white pays for a case that rarely happens.
            Measured, he is half right, and the half that is wrong matters. Relaxing to a mid backdrop does buy transparency and lets the colour come down. But at alpha 0.62 the ink falls to **2.36:1** over a white wallpaper, which is not legible — and the compositor blurs the desktop, so a bright photo behind a dark-theme note is an ordinary situation rather than a pathological one.
Decision:   Dark tints hold 4.5:1 against a **mid** backdrop and never fall below **3:1** against white. Alpha is 0.77, which is the most transparent value that keeps that floor; chroma is 0.06.
Consequences: This is a deliberate relaxation of the rule the light tints still follow, and its price is stated rather than buried: a dark-theme note over a bright wallpaper is legible but under AA for body text. Every other case — mid backdrop 4.5:1, dark backdrop 6.3:1 — is at or above it.
            Alpha 0.77 is not taste and should not be nudged without redoing the arithmetic. It is where the 3:1 floor lands, and no choice of chroma rescues a lower one.
            The light tints are untouched. A light theme composites against white anyway, so its worst case and its ordinary case are the same thing, and there is nothing to relax.
            Three passes to get here, each discarded on measurement rather than opinion: pale-and-murky, then maximum chroma (magenta), then strong-and-opaque. The founder's report each time named a symptom, and the numbers named the cause.

## ADR-037 — The remappable set is four formatting chords, and closed
Status:     Accepted
Date:       2026-09-06
Context:    `MASTER.md § In Scope` promises "a small set of remappable keyboard shortcuts". Only the global new-note chord was remappable; the four formatting chords were hard-coded. Asked which should be exposed, the founder said he is not a shortcut user and the basic ones are fine, leaving the choice here.
Decision:   Bold, italic, inline code and strikethrough become settable, alongside the new-note chord already there. Nothing else. The set is fixed in code rather than open-ended.
Consequences: Close and quit are deliberately not exposed. A mis-set chord on either is hard to recover from — the window it would close is the one you would fix it in — and neither is a chord anyone needs to move.
            The set is named fields rather than a map, in both the settings file and the window. A map would let a settings file name a command that does not exist, and the failure would be silence.
            The chords live in a CodeMirror `Compartment`, so a change is swapped into open editors rather than waiting for the next window. A shortcut that only applied to windows opened afterwards is a setting that appears not to work, which is the same reason the theme is broadcast.
            Remapping earns its keep less for the founder than for the audience: `Mod-` is the platform abstraction, and `docs/FIXES.md` already records a chord lost to AltGr on his own layout. Someone whose layout collides now has somewhere to go.
            A chord CodeMirror cannot parse simply never fires. That is why the settings window exists as the place to type one, and it is also a gap: nothing validates the string yet. Worth knowing before someone types nonsense into it.

## ADR-038 — The ring inserts markdown, and wears a pixel icon set
Status:     Accepted
Date:       2026-09-06
Context:    ADR-034 built the ring as a mechanism and left its contents open, on the reasoning that the contents are a preference the founder can change without touching any of it. He has: the ring becomes an insert menu for tables, code blocks and lists — the markdown that is a nuisance to type by hand — and the four window actions it held move out entirely rather than sharing it. His argument is that those four were the weaker set, since each already has a control of its own.
Decision:   Right-click on a note opens six inserts: table, code block, task, bulleted list, numbered list, link. Every one writes real markdown at the pointer. The glyphs are Pixelarticons, vendored, drawn at 24px in a 40px bubble.
Consequences: That ADR-034 cost nothing to change is the claim it made, now tested: the ring, the clamped centre, the hovered label, the backdrop and Escape are all untouched. What changed is one array and the glyphs it names.
            Nothing is lost by the four leaving. New note has the tray and a global chord, the hub has the tray, settings has the tray, and the pin is on the chrome of every note.
            The insert lands at the **pointer**, not at the caret. Right-clicking does not move the caret, so without that a table asked for at the foot of a note would appear wherever the caret happened to be — the kind of thing noticed only after it has moved your text. The window hands the editor the coordinates and the editor resolves them; a window that could reach into CodeMirror to do it itself would be a window that can do anything.
            The commands write syntax, exactly as the formatting commands do. Nothing renders a table without a table being in the text, and copy still yields source.
            Each lands the caret where a person would start typing — the first header cell, a fence's language, the text half of a link — and a table's first heading arrives selected, to be typed over. A command that inserts a skeleton and leaves the caret at the end has done half the job.
            The glyphs are **pixel art on a 24-unit grid** and are only sharp at 24px or a multiple of it, which is why the ring's bubbles are 40px rather than the scale's 32: the bubble is sized to the glyph. That is a deliberate exception to the spacing scale and the only one in the inventory. The window's own 9px controls keep their hand-drawn strokes, where a design pixel would be under half a screen pixel.
            They fill rather than stroke, so a stroke rule applied to them draws nothing. `.lozenge`'s 58% glyph sizing is overridden here for the same reason.
            Six is what the ring holds before the bubbles crowd. Headings and blockquotes were the next candidates and are not in it; the set is closed at six until something is taken out.
            Pixelarticons is MIT, vendored as six path strings with its licence beside them, like the typefaces. Nothing is fetched at runtime. The numbered list borrows `list-box`, the nearest the set has — the hovered label says which it is, which is a thing a ring can do that a wordless toolbar cannot.

## ADR-039 — The controls are glass on glass
Status:     Accepted
Date:       2026-09-06
Context:    The founder asked whether the bubbles could be transparent like the windows. They could: the window is already composited over the desktop's blur, so a control with alpha in its fill picks that up and reads as glass resting on glass rather than as a disc laid on a window — which is what Aqua's own controls did.
Decision:   A control's fill is translucent in both themes. The rim, the seat and the specular stay at full strength, and a **lit** control stays opaque. Controls carry `backdrop-filter: blur(var(--gloss-blur))`.
Consequences: The alpha runs down the gradient rather than sitting flat across it — 0.94 at the lit top, 0.68 through the body, 0.60 at the edge on Frost. Even translucency reads as a hole cut in the window; graded translucency reads as glass. Dark's is flat at 0.66 because there is no gradient there to grade, and its bezel is what makes it an object.
            The rim, the seat and the specular are what survive the fill going thin. Those are the edges of the thing; the fill is only what it is made of.
            **A lit control stays opaque, and gains meaning from it.** Its fill is the information — an engaged pin, a tint swatch, a failed save — and diluting it would dilute what it says. The side effect is worth naming: something held on is now denser than something idle, which is a second reading of the same state and cost nothing.
            **This is the application's one `backdrop-filter`, and it does not contradict the rule against it.** That rule is about the primary window surface, where the filter cannot see the desktop and so cannot do the job — the compositor does that, from Rust. Here the thing behind the control *is* the app's own content: the note's text, under a bubble in the ring. Without the blur the words read straight through the glyph.
            ADR-034 said these bubbles would not need it, on the reasoning that an opaque gloss has nothing to see through. That reasoning was sound and its premise is now gone. SMD-038's original note — that a bubble sits over the app's own content, where the filter is legitimate — turns out to have been right about the principle before there was a reason to use it.
            Contrast was checked before the look was: over white, mid and black desktops, in both themes, for the neutral control and all six tinted ones. The worst case is 4.01:1 and most are above 8:1, against a 3:1 floor for a graphical object. Legibility was never the binding constraint here — appearance was — but that is worth knowing rather than assuming.
            Solid mode needs no branch. There the window is opaque, so a translucent control simply picks up the surface beneath it and reads a shade lighter. One recipe, two modes, which is what `--surface-paint` is for.
