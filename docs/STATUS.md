# STATUS

Everything with a state. This is the only file permitted to contain statements that expire.

---

## Current Phase

**Build** — Phase 3 — Windows, Tray & Shortcuts

## Current Active Step

SMD-041 is written and running, awaiting confirmation that closing the app with notes open and reopening it brings those notes back, in the same places, and that a note closed deliberately stays closed.

Then, to close Phase 3 — Windows, Tray & Shortcuts: launch at startup (SMD-037) and making the shortcut set remappable, which SMD-033 showed is load-bearing rather than a nicety.

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
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 5 — Theme & Motion
Notes: The note tints from ADR-018 — Clear, Sun, Spring, Aqua, Sky, Lilac, Blush — as `--note-tint-*` tokens, selectable per note. `Note.theme` already carries it in the domain model and the sidecar index already stores it. Each tint must hold `--ink-primary` at 4.5:1 when layered over frosted glass on an arbitrary wallpaper. Values live in ADR-018 until this ships; `src/lib/tokens/tokens.css` becomes authoritative then.

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
State:   active
Created: 2026-09-05
History:
  2026-09-05  logged as active — Phase 3 — Windows, Tray & Shortcuts
Notes: Notes that were open when the application stopped come back where they were. The index carries `open` and the geometry; a note is marked open when it takes a name, and closed when its window is closed deliberately. Position is written when a window loses focus and again as it closes — the two moments it is worth writing. Writing on every drag frame would put the disk to work for the whole gesture.
  The window Tauri builds from the config is reused for the first restored note rather than left empty beside them, or every restored session would come back with one more note than it had.
  Geometry is in logical pixels. Physical ones were tried first and were wrong — see SMD-043. Placement failures are ignored: a saved position can be off-screen after a monitor is unplugged, and a note that opens in the wrong place beats one that refuses to open.
  A restore that fails does not stop the application starting. The notes are still on disk and an empty window is a working app.
  Not confirmed: that a session comes back, and that a deliberately closed note stays closed.

### [SMD-043] Pale lines down the right and bottom of a restored window
Type:    bug
State:   active
Created: 2026-09-05
History:
  2026-09-05  reported by founder after a session restore
  2026-09-05  active — cause found
Notes: SMD-041 stored geometry in physical pixels, recorded at the time as a deliberate trade. It was not a sound one. A physical size does not divide evenly into CSS pixels at a fractional DPI scale, so a restored window is a sliver wider than the web view inside it, and that strip is painted by nothing — a pale line down the right edge and along the bottom.
  Geometry is now logical pixels, the units the page itself thinks in, so nothing is left over. That also fixes the cross-display problem the original note wrote off: logical units carry correctly onto a monitor with a different scale factor.
  Also corrected while here: the size measured was the *outer* size while `set_size` sets the *inner* one, so a window would have grown slightly on every session.
  Entries written before this change hold physical values. They are rewritten the first time each window loses focus, so at worst one restore is off on a scaled display.

### [SMD-042] Apostrophes were turned into separators in filenames
Type:    bug
State:   active
Created: 2026-09-05
History:
  2026-09-05  found while reading the notes folder after a session-restore test
  2026-09-05  active
Notes: A note titled "I'm Jorge" produced `i-m-jorge.md`. An apostrophe joins a word rather than separating one, so it is now dropped rather than converted to a hyphen: `im-jorge.md`. Both the typewriter apostrophe and the typographic one, which is what most editors insert.
  Notes already on disk keep the names they have. Editing one's title renames it as normal, and the index entry follows.

### [SMD-037] Launch at startup, off by default
Type:    feature
State:   accepted
Created: 2026-09-05
History:
  2026-09-05  logged and accepted into Phase 3 — Windows, Tray & Shortcuts
Notes: `MASTER.md § In Scope` requires it off by default and user-togglable, and MASTER veto 5 forbids enabling it without explicit consent. The tray menu is the natural home for the toggle, since there is no settings surface. Needs `tauri-plugin-autostart`.

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
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: If another application already holds `Ctrl+Shift+Space`, registration fails, the failure is printed to a console nobody has open, and the only way to summon a note is to click the application. `MASTER.md § In Scope` lists remappable shortcuts, which is the remedy — this records that it is load-bearing rather than a nicety, and that the failure needs to be visible somewhere the user will see it.

### [SMD-031] Deleting a note whose window is open does not stick
Type:    bug
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: The window keeps the note's content and filename in memory, so the next autosave writes the file straight back. Reachable today by deleting in Explorer while a window is open, and it becomes reachable from inside the app once the hub can delete (Phase 4 — The Hub). Confirmed by reading `save_into`: the rename is skipped because the old file is gone, then `fs::write` recreates it. "The open window wins" is defensible, but it should be a decision rather than an accident — the alternative is that deleting a note closes its window.

### [SMD-027] list_notes, delete_note and the index commands have no frontend consumer
Type:    chore
State:   idea
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: `list_notes` and `delete_note` are written, tested where testable, and registered — but nothing calls them. Both wait on the hub, since deleting a note needs somewhere to delete it from. `read_note` and `window_note` gained their callers with SMD-030; `note_state` and `set_note_always_on_top` gained theirs with SMD-040. Their consumers are session restore and the always-on-top toggle (Phase 3 — Windows, Tray & Shortcuts) and the hub (Phase 4 — The Hub). Kept rather than deleted because they are the file and state API those phases consume; logged so the gap is visible rather than assumed. The index does have one live consumer: renaming a note moves its entry, which happens on every retitle today.

---

## Completed Milestones

### Phase 2 — Files & Persistence — closed 2026-09-05
Notes became files. Rust owns note I/O with typed errors and untrusted-name validation; debounced autosave that flushes on blur and intercepts window close; filenames slugified from the first line with deduplication, renaming, Windows reserved-stem handling and no churn on re-save; the sidecar index with atomic writes, corrupt-file salvage and a mutex against concurrent windows; delete to the operating system's trash. Items SMD-023, SMD-024, SMD-026, SMD-028, SMD-029.

Truth check run 2026-09-05 before the transition: retraction grep clean; all fourteen built entries of `MASTER.md § In Scope` verified present in code; no supersession pointers to reconcile; 29 items all carrying a state, a date and a history line, every shipped item citing a commit that exists, and no item left `active`; 117 cross-references checked with one genuine error corrected — an unqualified `index.rs` in CLAUDE. The remaining link-checker hits are paths inside the user's notes folder rather than repository files.

21 Rust tests pass. Nothing in the phase is unverified except what SMD-027 names.

### Phase 1 — Foundation & Editor Core — closed 2026-09-05
Tauri v2 + Svelte 5 shell with custom chrome and the design token layer; CodeMirror 6 with the inline-rendering layer, formatting commands, fenced-code highlighting and rendered tables; three vendored OFL typefaces; the application mark; compositor glass with a Solid fallback; the README. Items SMD-006, SMD-007, SMD-008, SMD-012, SMD-013, SMD-014, SMD-015, SMD-016, SMD-017.

Truth check run 2026-09-05 before the transition: retraction grep clean across truth files; all ten built entries of `MASTER.md § In Scope` verified present in code; no supersession pointers to reconcile; 22 items all carrying a state, a date and a history line, with every shipped item citing a commit that exists; 111 cross-references checked and two genuine errors corrected — the font licence filenames and three unqualified paths.

The item entries above are kept rather than folded away. Their commit citations and findings — the Handjet metric measurement, the memory figures, the licence discrepancy — are not reproducible from a changelog line.
