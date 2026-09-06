# STATUS

Everything with a state. This is the only file permitted to contain statements that expire.

---

## Current Phase

**Build** — Phase 5 — Theme & Motion

## Current Active Step

The theme system and its built-in themes. The dark theme (SMD-004) cannot ship alone: without a way to choose it, it is unreachable code. It lands with per-note colour (SMD-021), which is what makes a theme selectable at all — `Note.theme` has been in the domain model and the sidecar index since Phase 2 with nothing writing it.

Then the motion pass (SMD-048). Done — and it ended by dropping SMD-045 rather than building it: animating the written-to-rendered transition means animating a reflow on every cursor move, which would make it worse. Awaiting the founder's confirmation on that call.

---

## Environment

| | |
|---|---|
| Development machine | Windows 11 Pro |
| Build target | Windows x64 |
| Node | v24.19.0, npm 11.17.0 — verified present 2026-09-05 |
| Rust | rustc 1.98.1, cargo 1.98.1, rustup 1.29.1 — installed 2026-09-05. |
| MSVC linker | Visual Studio C++ build tools installed 2026-09-05. Both halves build and the application runs. |
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
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 5 — Theme & Motion (confirmed by founder)
  2026-09-06  shipped in 1fa1059, with SMD-021
Notes: A complete set of token values, not a code change — faint black tint over the frosted surface with warm white ink, inverting Frost. Light is the default mode. Chosen from the tray, and applied to every open window rather than only to windows opened afterwards.
  Its tint alpha ships at 0.63, not the 0.45 written here when the theme was sketched: 0.45 measured 2.72:1 against a white wallpaper, well under AA. Dark ink on light and light ink on dark are not symmetrical, and the sketch had never been checked. See ADR-024.

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
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
  2026-09-05  active
  2026-09-05  shipped — commit 2f56290
Notes: Documents the SmartScreen warning per ADR-010, and says plainly why it appears rather than burying it. Links to the FLOW files rather than restating them: current state points at STATUS, toolchain requirements at CLAUDE. Opens with a notice that the software is unfinished, placed above the feature list so a visitor cannot read the list as a claim about what runs today. The clone URL is deliberately absent — no remote exists yet, and inventing one would be a false instruction.

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
Notes: Eight variable woff2 files in `src/assets/fonts/`, 190 kB total, subset to latin and latin-ext — other scripts deliberately absent. Geist ships roman and italic; the others are roman only. Each family's licence text sits beside its files as `OFL-<Family>.txt`. Verified in a browser: all three families load and render, and the Handjet axes drive real degradation.

### [SMD-014] App icon and bundle configuration
Type:    chore
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
  2026-09-05  scope corrected — blocks the build, not only releases
  2026-09-05  active
  2026-09-05  shipped — commit e3ac885
Notes: `tauri-build` requires `src-tauri/icons/icon.ico` to generate the Windows resource file, so `cargo check` fails without it and the application cannot run at all. The earlier note claiming `tauri dev` was unaffected was wrong. Also needs the `bundle` section in `src-tauri/tauri.conf.json` for NSIS and the portable zip. The icon itself is an identity decision for the founder.

### [SMD-015] Compositor glass and the Glass/Solid mode switch
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
  2026-09-05  active — written, running, awaiting founder confirmation
  2026-09-05  shipped — commit 1567f90
  2026-09-05  confirmed frosted on device (founder)
Notes: Acrylic applied to the transparent window from Rust via `window-vibrancy`, per `docs/DESIGN.md` principle 3. The resulting mode is reported to the frontend by the `surface_mode` command and lands on the document root as `data-surface`; `--surface-paint` and `--surface-border` resolve from it, so no component branches on the mode. A window that cannot be frosted is Solid, which is a supported way to run rather than an error.
  Confirmed visually by the founder on 2026-09-05. Code alone could not establish this: a window that silently failed to frost would look like a plain white note and every automated check would still pass.

### [SMD-022] Respond to the system transparency setting changing at runtime
Type:    bug
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: Compositor blur is applied once, at window creation. If the user turns transparency off afterwards — or Windows does it for them under battery saver — the window stays in Glass mode with nothing behind the tint, which will read as a washed-out surface rather than a deliberate Solid one. Needs a listener on the setting and a way to re-resolve `data-surface` in a live window.

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
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 1 — Foundation & Editor Core
  2026-09-06  confirmed on device (founder) — pasting from sticky.md gives raw markdown
Notes: The document holds real markdown and CodeMirror serialises the clipboard from state rather than the DOM, so raw source was expected. Confirmed by the founder pasting out of a rendered note. No commit — nothing needed changing. This is the single property the product exists to provide, which is why it was checked rather than assumed.

### [SMD-019] Note window JavaScript weight
Type:    idea
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: The note window's main chunk is 555 kB raw, 193 kB gzipped — mostly CodeMirror plus the HTML, CSS and JavaScript grammars that `@codemirror/lang-markdown` pulls in unconditionally (see `docs/FIXES.md`). Loaded from disk, so no network cost, but every open note window parses its own copy.
  Measured 2026-09-05, one window, development build: the sticky.md process tree is 381.8 MB across 7 processes — 35.9 MB for the application and 345.9 MB across 6 WebView2 processes. Half the WebView2 processes on the machine belonged to other applications and are excluded.
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
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 5 — Theme & Motion
  2026-09-06  shipped in 1fa1059
Notes: Shipped as `TintPicker`, a swatch on the note's chrome that opens the seven tints. Stored per note in the sidecar index and restored with the window.
  Two things this item assumed turned out to be wrong. The field is a *tint*, not a theme — the theme is application-wide, and ADR-024 records the split, with `MASTER.md § Domain Model` amended. And the alphas could not be taken from ADR-018 as written: every one was recomputed, and a coloured tint needs *more* alpha than Clear in the light theme, not less, because it is darker than white and lifts the composite less over a dark wallpaper.
  Original note follows. The note tints from ADR-018 — Clear, Sun, Spring, Aqua, Sky, Lilac, Blush — as `--note-tint-*` tokens, selectable per note. `Note.theme` already carries it in the domain model and the sidecar index already stores it. Each tint must hold `--ink-primary` at 4.5:1 when layered over frosted glass on an arbitrary wallpaper. Values live in ADR-018 until this ships; `src/lib/tokens/tokens.css` becomes authoritative then.

### [SMD-023] Note file I/O
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 2 — Files & Persistence
  2026-09-05  shipped — commit 691c402
Notes: Rust owns reading, writing and listing note files; the frontend never touches the filesystem. Errors cross the boundary as typed data — `unsafe_name`, `no_notes_folder`, `not_found`, `io` — so the frontend can tell a missing note from a full disk without parsing prose. Note names arrive from the frontend and are untrusted: a name is accepted only if it is a single ordinary path component ending in `.md`, which is what stops `../` and Windows stream names from escaping the folder. Three unit tests cover that validation and are the first tests in the project.

### [SMD-024] Debounced autosave
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 2 — Files & Persistence
  2026-09-05  shipped — commit b91b32d
  2026-09-05  confirmed on device (founder) — folder empty, then the note appeared after typing
Notes: The editor reports every change; the note is written 600ms after typing stops, and again when the window loses focus or is closed. Closing is intercepted so a pending write completes before the window is destroyed — otherwise the last few characters typed would be lost. An unchanged buffer is never rewritten, and a window opened and closed without typing leaves no file behind, which is verified: launching the application creates `Documents/sticky.md` and leaves it empty.
  Confirmed by the founder on 2026-09-05: the folder was empty, and the note appeared after typing and a short pause.

### [SMD-025] Surface save failures in the window
Type:    bug
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: A failed write is logged to the console and the text stays queued for the next flush, so nothing is lost while the window is open. But the user is told nothing, and if the window closes the queued text goes with it. A full disk or a permissions problem should be visible in the window rather than only in a console nobody has open.

### [SMD-026] Slugified filenames with deduplication and renaming
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 2 — Files & Persistence
  2026-09-05  shipped — commit 11184fb
Notes: ADR-006 implemented. The frontend sends a title; Rust slugifies it, finds a free name, renames the file if the title changed, and returns the name the note now has. Leading `#` characters are stripped, so notes do not all sort under `-`. Windows reserved stems get a `-note` suffix. A title that still slugifies to the stem a note already holds does not trigger a rename, so editing never churns the folder. Twelve unit tests, six of them against a real scratch folder, covering the two properties that move a user's file: retitling leaves nothing behind, and retitling onto a name another note holds does not clobber it.
  Replaces the fixed `untitled.md`, which meant a second window would have overwritten the first.

### [SMD-028] The sidecar index
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 2 — Files & Persistence
  2026-09-05  shipped — commit ddc6945
Notes: ADR-002 implemented as `.sticky-index.json` in the notes folder, keyed by filename, holding geometry, theme, always-on-top and open state. Writes go through a temporary file and a rename; an unreadable index is moved aside rather than overwritten; read-modify-write is serialized by a mutex, because two windows saving at once would otherwise each load, apply their own change, and write back with the last erasing the other. A retitled note carries its entry, since it is the same note. Nine tests.

### [SMD-029] Delete a note to the operating system's trash
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 2 — Files & Persistence
  2026-09-05  shipped — commit 6dad028
  2026-09-06  confirmed on device (founder) — a note deleted from the hub left the folder and appeared in the Recycle Bin
Notes: `trash` crate 5.2.7, never an unlink — a note deleted by mistake has to be recoverable, which is why the app has no bin of its own. Deleting also forgets the note's index entry, under the lock. A trash failure is its own error variant rather than a generic IO one: some locations have no trash at all, and offering a permanent delete is a different conversation from reporting a broken disk.
  Deliberately not unit-tested. Exercising it would put files in the developer's real Recycle Bin, and what it delegates to is the crate's job. The parts worth covering — name validation and forgetting the entry — are tested through `safe_name` and `forget_entry`.

### [SMD-030] Multiple note windows
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 3 — Windows, Tray & Shortcuts
  2026-09-05  shipped — commit d7726fd
  2026-09-05  confirmed on device (founder) — two windows write two different files
Notes: One window per note. Which note a window holds is tracked in Rust by window label, so nothing is threaded through the URL and a window can simply ask. A window with no note is a new, unsaved one; it reports the name it takes on its first save, which is what stops a second window being opened onto the same file and the two overwriting each other. `Mod-n` opens a new window, bound through CodeMirror because its `Mod-` prefix is the platform abstraction and a DOM listener would mean writing Ctrl literally.
  Every window is built from the window entry in `src-tauri/tauri.conf.json`, so geometry has one home.
  The in-app chord was removed. It hit two unrelated platform hazards in a row — see ADR-021 — and the global shortcut in SMD-032 is the affordance `MASTER.md § Core Loop` specifies anyway.
  Confirmed by the founder on 2026-09-05: two windows write two different files.
  Known sloppiness: `surface_mode` returns the mode measured on the first window. A second window has acrylic applied and its result discarded. In practice every window on a machine resolves the same way, but it is an assumption rather than a measurement.

### [SMD-032] Global shortcut for a new note
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 3 — Windows, Tray & Shortcuts
  2026-09-05  shipped — commit 997460a
  2026-09-05  confirmed on device (founder)
Notes: `Ctrl+Shift+Space`, registered with the operating system from Rust, per ADR-021. Space rather than a letter because a global registration takes the chord from every application on the machine. Failing to claim it is reported, not raised: another application may already hold it, and the app runs fine without it.
  Confirmed by the founder on 2026-09-05: `Ctrl+Shift+Space` opens a note window.

### [SMD-034] Note windows could not be dragged
Type:    bug
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  reported by founder — the window could not be moved from anywhere, including the drag strip
  2026-09-05  active — cause found
  2026-09-05  shipped — commit 33aa6ac
  2026-09-05  confirmed by founder — dragging, the close button and therefore the flush before close all work
Notes: The project had no `src-tauri/capabilities/` directory. It was hand-scaffolded rather than generated, and Tauri v2 gates *core plugin* commands behind capabilities while leaving application-defined commands ungated — which is exactly why every custom command worked and nothing from `core:window` did. `core:window:default` is read-only: it grants `allow-is-*`, `allow-title` and the monitor queries, and not `allow-start-dragging`, `allow-close` or `allow-destroy`.
  Three things were silently denied, not one: dragging, the close button, and the flush-before-close wired into `onCloseRequested` — which means a note's last few characters could have been lost on close and nothing would have said so.

### [SMD-035] Grey triangles in the window corners
Type:    bug
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  reported by founder
  2026-09-05  active — cause found
  2026-09-05  shipped — commit 33aa6ac
  2026-09-05  confirmed by founder — corners read clean at the 8px system radius
Notes: The compositor draws its backdrop across the whole window rectangle, which is square. `border-radius` rounds only what the web view paints, so the corners outside it showed raw acrylic with none of the CSS tint over it. The window itself is now rounded through `DwmSetWindowAttribute`, which makes the compositor clip its own backdrop.
  Design consequence, recorded in `docs/DESIGN.md § Note window`: Windows chooses the radius and will not accept an arbitrary one, so `--radius-window` dropped from 18px to 8px to match. A generous Post-It radius and compositor glass cannot both be had here.

### [SMD-036] Tray residency
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 3 — Windows, Tray & Shortcuts
  2026-09-05  shipped — commit e042bcd
  2026-09-05  confirmed on device (founder) — tray behaves, closing every window leaves the app summonable, quit exits
Notes: The application lives in the tray so the global shortcut has something to reach — closing the last note window puts it away rather than ending it, which is what lets a note appear a second later without a cold start. Only an explicit quit exits, distinguished by the exit code carried on the request.
  Left-clicking the tray writes a new note rather than opening a menu; the menu is on the right button. That should become "open the hub" once there is one (Phase 4 — The Hub).
  Confirmed by the founder on 2026-09-05. The founder also confirmed the intent that left-click should open the hub once one exists.

### [SMD-040] Per-note always-on-top
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 3 — Windows, Tray & Shortcuts
  2026-09-05  shipped — commit c0e413f
  2026-09-05  confirmed on device (founder)
Notes: A pin in the window chrome, off by default per `MASTER.md § In Scope`. The setting lives in the sidecar index and survives closing and reopening the note — which makes this the first consumer the index has had since it was built in Phase 2.
  A note can be pinned before it has a file. The setting is held in the window until the note takes a name on its first save, then written.
  `set_note_state` was replaced by `set_note_always_on_top` rather than added to. The frontend does not hold a note's geometry, so writing a whole `NoteState` back from there would erase whatever it did not know about — a targeted command cannot.
  Deliberate exception to `docs/DESIGN.md` principle 2: a pinned note keeps its pin visible when the chrome recedes. A state you cannot see is a state you cannot trust.
  Confirmed by the founder on 2026-09-05.

### [SMD-041] Session restore of open windows
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 3 — Windows, Tray & Shortcuts
  2026-09-05  shipped — commit 5426939
  2026-09-05  confirmed on device (founder)
Notes: Notes that were open when the application stopped come back where they were. The index carries `open` and the geometry; a note is marked open when it takes a name, and closed when its window is closed deliberately. Position is written when a window loses focus and again as it closes — the two moments it is worth writing. Writing on every drag frame would put the disk to work for the whole gesture.
  The window Tauri builds from the config is reused for the first restored note rather than left empty beside them, or every restored session would come back with one more note than it had.
  Geometry is in logical pixels. Physical ones were tried first and were wrong — see SMD-043. Placement failures are ignored: a saved position can be off-screen after a monitor is unplugged, and a note that opens in the wrong place beats one that refuses to open.
  A restore that fails does not stop the application starting. The notes are still on disk and an empty window is a working app.
  Confirmed by the founder on 2026-09-05: a session comes back.

### [SMD-043] Pale lines down the right and bottom of a restored window
Type:    bug
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  reported by founder after a session restore
  2026-09-05  active — cause found
  2026-09-05  shipped — commit ab76fe1
  2026-09-05  confirmed on device (founder) — the lines are gone
Notes: The cause was `shadow: true`, added in the corner-rounding commit. It gives a window with `decorations: false` a frame the page cannot paint. Measured from inside the application: the web view filled its viewport exactly at 436x420 CSS, while the window was inner 545x525 and outer 563x535. Eighteen physical pixels on the right and ten at the bottom belonged to nothing and rendered white. With `shadow: false`, outer equals inner exactly.
  Two wrong diagnoses came first, both from theorising instead of measuring. The first blamed fractional DPI. The second rejected that on a scale reading of 1.0 taken from a process that was not DPI-aware — the real scale is 1.25, so the theory being dismissed was closer than the correction dismissing it.
  Both attempts are kept, because each fixed something real on its own: geometry is stored in logical pixels rather than physical; `set_size`'s inner size is measured rather than the outer one, which had windows growing every session — 418x410 to 436x433 in the founder's own index; and `.surface` is pinned with `position: fixed; inset: 0` rather than a percentage height.
  Two real bugs were fixed on the way to the right answer, both in SMD-041's geometry handling: the size measured was the *outer* size while `set_size` sets the *inner* one, so windows grew on every session — visible in the founder's index, 418×410 becoming 436×433 — and geometry is now stored in logical rather than physical pixels, which is what carries correctly onto a display with a different scale factor.

### [SMD-044] A note window casts no shadow
Type:    bug
State:   dropped
Created: 2026-09-05
History:
  2026-09-05  logged as idea while fixing SMD-043
  2026-09-06  dropped — the premise was false
Notes: Logged on the inference that `shadow: false` would leave the window casting nothing. The founder observed that it does cast one: Windows draws its own shadow for a DWM-rounded window whatever that setting says. Inferred from code rather than looked at, which is the whole reason on-device confirmation exists.
  What remains true, and is recorded in `docs/FIXES.md` instead: a CSS shadow is clipped at the window edge, so `--shadow-rest` has no effect on the window itself. The shadow is the system's. If a different one is ever wanted, that is when the window has to be padded — not before.

### [SMD-042] Apostrophes were turned into separators in filenames
Type:    bug
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  found while reading the notes folder after a session-restore test
  2026-09-05  active
  2026-09-05  shipped — commit c838e8c
Notes: A note titled "I'm Jorge" produced `i-m-jorge.md`. An apostrophe joins a word rather than separating one, so it is now dropped rather than converted to a hyphen: `im-jorge.md`. Both the typewriter apostrophe and the typographic one, which is what most editors insert.
  Notes already on disk keep the names they have. Editing one's title renames it as normal, and the index entry follows.

### [SMD-037] Launch at startup, off by default
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 3 — Windows, Tray & Shortcuts
  2026-09-06  active
  2026-09-06  shipped — commit 5211972
  2026-09-06  confirmed — the founder ticked it and the Run key gained `sticky.md`; unticked it is absent
Notes: A checkable "Launch at startup" item in the tray menu, since there is no settings surface. `tauri-plugin-autostart` is registered but never enables anything on its own — only the toggle does, which is what MASTER veto 5 requires.
  The checkbox is read from what the system reports, both when the menu is built and after every toggle, rather than from a remembered value. The user may have removed the entry outside the application, and a menu that claims a change succeeded when it failed is worse than one that shows nothing.
  Verified from the registry rather than by inference: with the box unticked `HKCU\...\CurrentVersion\Run` has no sticky.md value, and ticking it adds one pointing at the executable. A dev build registers the dev binary, which is correct — an installed build would register the installed one.

### [SMD-047] The hub
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged as active — Phase 4 — The Hub
  2026-09-06  shipped — commit 8c7f373
  2026-09-06  confirmed on device (founder) — opens, drags, lists, deletes
Notes: Every note listed newest first, opened by clicking, deleted from the row. Opened by clicking the tray, or from its menu. `list_notes` now returns a title and a modified time rather than a filename: the hub shows what a person calls a note, which is its first line, while every command still refers to notes by filename.
  Deleting asks for no confirmation. The file goes to the operating system's trash, so it is already undoable — a dialog guarding something the OS made recoverable is friction with no benefit.
  A note that cannot be read is listed under its filename rather than hidden. A note the hub silently omits is a note the user cannot recover.
  The list refreshes whenever the hub is focused. Notes change in other windows and in the folder itself, and a list that is only right when it was opened is worse than one that is right when you look at it.
  Confirmed by the founder on 2026-09-06: the hub opens, the window drags, and deleting a note removes it from the folder and puts it in the Recycle Bin.
  A first report that the hub could not be dragged and its buttons did nothing turned out to be a stale build: the launch being tested had exited with an error and left an older instance running, from before the hub had a capability entry. Nothing was changed to fix it. The lesson is that "the app is running" is not the same as "the app is running this code" — check that a launch actually succeeded before asking for a test.

### [SMD-046] A settings surface
Type:    idea
State:   idea
Created: 2026-09-06
History:
  2026-09-06  logged as idea
Notes: Settings are a JSON file the user edits by hand (ADR-022). That satisfies "remappable" for someone willing to open a text editor and no one else. Not in `MASTER.md § In Scope` and not smuggled in as though it were — a settings window is a real feature with its own design, and it would also be the natural home for the notes-folder location (SMD-003) and theme choice.

### [SMD-045] Animate the transition between written and rendered markdown
Type:    idea
State:   dropped
Created: 2026-09-06
History:
  2026-09-06  logged as idea (founder)
  2026-09-06  dropped in the motion pass — animating it would make it worse
Dropped because: what changes when the cursor leaves a line is a reflow — the hidden marks stop taking width and the text closes up. Animating a reflow means a layout on every frame, on every cursor move, and arrowing down a document would set the whole page shimmering. The cost is real and the result would be worse than the instant switch.
  What the motion pass did instead is narrow the gap between the two states so there is less to soften: syntax marks are drawn in `--ink-syntax` rather than at full ink, and styled text keeps its size and weight whether or not its marks are showing — so leaving a line changes the marks and nothing else. Recorded in `docs/DESIGN.md § Motion` as a rule rather than left as a preference.
  Worth reopening only if the founder finds it harsh in daily use, in which case the honest fix is a design one — quieter marks still — rather than an animation.
Notes: Syntax currently appears and disappears instantly as the cursor enters and leaves a line. The founder would like that transition softened. Beyond V1 by his own framing, and it belongs with Phase 5 — Theme & Motion.
  Constrained by `docs/DESIGN.md § Never Allowed`: the syntax characters are hidden with replace decorations, so there is no element to fade — a transition means rendering the marks and animating their width or opacity rather than removing them, which changes how the decoration layer works. Not a styling change.

### [SMD-048] The motion pass
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped
Notes: Phase 5's second half. What it mostly produced was a constraint. `docs/DESIGN.md § Motion` asked for a note to scale slightly into place as it arrives, and on a frosted window that is unbuildable: the surface covers the window exactly, so scaling it down shows a ring of raw acrylic and scaling it up clips its rounded corners square — the corner and sliver artefacts again, animated. DESIGN was amended rather than left describing something that cannot be built, and `docs/FIXES.md` carries the entry.
  A window arrives by the light on it instead: an overlay that carries a sheen across the glass and fades over `--dur-settle`. One paint, no layout, and nothing touching the surface's own alpha or blur. Anything nested inside the surface may still scale freely, which is what the tint palette does — what is behind a palette is the window, not the desktop.
  Two dead things came out with it. `--shadow-rest` was on both window surfaces and could never have been seen: an outer shadow on an element that fills the window falls outside the window. It was tinting the rounded corner notches slightly and doing nothing else. It and `--shadow-dragging` are gone from the token contract, both describing window-level states the compositor owns. One shadow remains, for things raised inside a window.
  The two window shells had carried the same `.surface` block twice; it is now one rule in `src/app.css`. Second use means extract, and this was the second use.

### [SMD-038] Radial glass menu on right-click
Type:    idea
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea (founder)
Notes: A ring of glass bubbles opening from the pointer on right-click, each appearing staggered after the last. The founder's intent is a home for anything that cannot sit cleanly on the chrome, which is a real problem given `docs/DESIGN.md` principle 2 keeps the chrome minimal.
  Feasible, with one nuance worth recording before anyone builds it: the bubbles cannot carry *compositor* glass, which is a per-window property. They would use `backdrop-filter`, and that is legitimate here — DESIGN's never-allowed rule forbids it for the *primary window surface*, because a web view cannot see the desktop. Over the app's own content, which is what a bubble sits on, it is the correct tool, and `--blur-popover` already exists in the contract for it.
  The stagger must animate `transform` and `opacity` per bubble with a delay, never blur. Small non-glass elements may fade; that is already allowed.
  Also needs the web view's own context menu suppressed, or the native menu will appear alongside it.

### [SMD-039] Verify dead-key and accented input in the editor
Type:    chore
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as active
  2026-09-05  confirmed on device (founder) — dead keys work
Notes: Storage is proven: 23 Rust tests pass, including a real file round-trip where `# Año nuevo` becomes `año-nuevo.md` with the body byte-identical, and slugs keep accented letters rather than stripping them — a Spanish note should not become an unreadable filename.
  Composition in the editor was confirmed by the founder on 2026-09-05: dead keys work. The inline-rendering layer rebuilding decorations on every update does not disturb an in-progress composition. No commit — nothing needed changing.

### [SMD-033] A user whose global shortcut is already taken has no remedy
Type:    bug
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as idea
  2026-09-06  shipped — commit e69f46a
Notes: If another application already holds the chord, registration fails. Previously the only sign was a line in a console nobody has open, leaving a user with no way to summon a note and no idea why.
  Both halves are now addressed. The shortcut is read from `settings.json` (ADR-022), so it can be changed. And a failure appears in the tray menu as a disabled line naming both the problem and the file to edit, rather than being swallowed.
  A parse failure is distinguished from a chord already held: a typo falls back to the default shortcut and still says so, rather than leaving the user with nothing.

### [SMD-031] Deleting a note whose window is open does not stick
Type:    bug
State:   active
Created: 2026-09-05
History:
  2026-09-05  logged as idea
  2026-09-06  reported by the founder from the hub, and decided: the delete wins (ADR-023)
  2026-09-06  active
Notes: The window keeps the note's content and filename in memory, so the next autosave writes the file straight back. Reachable today by deleting in Explorer while a window is open, and it becomes reachable from inside the app once the hub can delete (Phase 4 — The Hub). Confirmed by reading `save_into`: the rename is skipped because the old file is gone, then `fs::write` recreates it. The founder hit it from the hub and chose: deleting a note closes its window (ADR-023). The window is destroyed rather than asked to close, because a close request runs the frontend's flush and would write the note straight back; and it is closed before the file is trashed, so a save landing between the two cannot resurrect it.

### [SMD-027] list_notes, delete_note and the index commands have no frontend consumer
Type:    chore
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as idea
  2026-09-06  shipped — commit 8c7f373, the hub gave the last of them callers
Notes: `list_notes` and `delete_note` are written, tested where testable, and registered — but nothing calls them. Both wait on the hub, since deleting a note needs somewhere to delete it from. `read_note` and `window_note` gained their callers with SMD-030; `note_state` and `set_note_always_on_top` gained theirs with SMD-040. Their consumers are session restore and the always-on-top toggle (Phase 3 — Windows, Tray & Shortcuts) and the hub (Phase 4 — The Hub). Kept rather than deleted because they are the file and state API those phases consume; logged so the gap is visible rather than assumed. The index does have one live consumer: renaming a note moves its entry, which happens on every retitle today.

---

## Completed Milestones

### Phase 4 — The Hub — closed 2026-09-06
Every note in one place: listed newest first by title rather than filename, opened by clicking, focused rather than duplicated if already open, and deleted to the operating system's trash without a confirmation dialog for something the OS already made undoable. Opened by clicking the tray icon or from its menu. Item SMD-047, which also closed SMD-027 by giving the last written-but-uncalled commands their callers.

Truth check run 2026-09-06: retraction grep clean; all five hub-related entries of `MASTER.md § In Scope` verified present in code; no supersession pointers to reconcile; 47 items, with one genuine failure found and fixed — SMD-027 was marked shipped without citing the commit that closed it.

### Phase 3 — Windows, Tray & Shortcuts — closed 2026-09-06
Notes became windows. One window per note with its own file identity, tracked by window label; tray residency, so closing the last window puts the application away rather than ending it; a global new-note shortcut registered with the operating system, configurable through `settings.json` and reporting in the tray when it cannot be claimed; session restore of open windows with their geometry; a per-note always-on-top pin; launch at startup, off by default. Items SMD-030, SMD-032, SMD-033, SMD-034, SMD-035, SMD-036, SMD-037, SMD-040, SMD-041, SMD-042, SMD-043.

Truth check run 2026-09-06: retraction grep clean; all six Phase 3 entries of `MASTER.md § In Scope` verified present in code; no supersession pointers to reconcile; 46 items all carrying a state, a date and a history line, every shipped item citing a commit that exists, the one dropped item carrying a reason, and none left `active`; 154 cross-references checked with one genuine error corrected — an unqualified `index.rs`.

Four bugs in this phase were found by the founder looking at the running application rather than by any test, and two of them had causes I had reasoned my way past. That is the phase's real lesson: measure inside the running application.

24 Rust tests pass.

### Phase 2 — Files & Persistence — closed 2026-09-05
Notes became files. Rust owns note I/O with typed errors and untrusted-name validation; debounced autosave that flushes on blur and intercepts window close; filenames slugified from the first line with deduplication, renaming, Windows reserved-stem handling and no churn on re-save; the sidecar index with atomic writes, corrupt-file salvage and a mutex against concurrent windows; delete to the operating system's trash. Items SMD-023, SMD-024, SMD-026, SMD-028, SMD-029.

Truth check run 2026-09-05 before the transition: retraction grep clean; all fourteen built entries of `MASTER.md § In Scope` verified present in code; no supersession pointers to reconcile; 29 items all carrying a state, a date and a history line, every shipped item citing a commit that exists, and no item left `active`; 117 cross-references checked with one genuine error corrected — an unqualified `src-tauri/src/index.rs` in CLAUDE. The remaining link-checker hits are paths inside the user's notes folder rather than repository files.

21 Rust tests pass. Nothing in the phase is unverified except what SMD-027 names.

### Phase 1 — Foundation & Editor Core — closed 2026-09-05
Tauri v2 + Svelte 5 shell with custom chrome and the design token layer; CodeMirror 6 with the inline-rendering layer, formatting commands, fenced-code highlighting and rendered tables; three vendored OFL typefaces; the application mark; compositor glass with a Solid fallback; the README. Items SMD-006, SMD-007, SMD-008, SMD-012, SMD-013, SMD-014, SMD-015, SMD-016, SMD-017.

Truth check run 2026-09-05 before the transition: retraction grep clean across truth files; all ten built entries of `MASTER.md § In Scope` verified present in code; no supersession pointers to reconcile; 22 items all carrying a state, a date and a history line, with every shipped item citing a commit that exists; 111 cross-references checked and two genuine errors corrected — the font licence filenames and three unqualified paths.

The item entries above are kept rather than folded away. Their commit citations and findings — the Handjet metric measurement, the memory figures, the licence discrepancy — are not reproducible from a changelog line.
