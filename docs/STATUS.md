# STATUS

Everything with a state. This is the only file permitted to contain statements that expire.

---

## Current Phase

**Build** — Phase 5 — Theme & Motion

## Current Active Step

Awaiting the founder's verification of the second autonomous bundle. Shipped in it: the code face swapped to JetBrains Mono (SMD-063), the settings window with a configurable notes folder (SMD-064, closing SMD-046 and SMD-003), typographic scrambling on the window titles (SMD-051), and the radial menu (SMD-038).

Two of the six did not finish, and neither was skipped quietly:

- **SMD-052, the line boil, is blocked.** Its subject is the application mark and the founder is redrawing it. The mark on disk is also still the old one — it carries the aqua gradient ADR-025 removed and is titled "StickyMD" rather than "sticky.md".
- **SMD-065, release readiness, is active.** Everything testable without a remote was tested by hand and both artefacts build. The workflow's own first run is the only part that has never executed, and it needs a tag pushed to a remote that does not exist yet.

Build phases 1 to 5 are shipped. The log now holds nothing that is both unblocked and undecided.

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
State:   shipped
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
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as idea
  2026-09-06  accepted into an autonomous bundle by the founder
  2026-09-06  shipped
Notes: Shipped. The setting is read from the registry before the blur is applied, and watched with `RegNotifyChangeKeyValue` on a thread that blocks until something changes — not a poll. When it changes, every open window is re-applied and told, and each re-resolves `data-surface`; windows opened later get their answer on the way up.
  The root cause was worse than the item assumed, and is now in `docs/FIXES.md`: `apply_acrylic` **succeeds** when transparency is off. The mode was being decided by whether that call errored, so the window stayed in Glass with the compositor drawing nothing behind it. A successful call reports that it was accepted, not that anything is being drawn.
  A window that cannot be frosted still makes the whole application Solid rather than leaving two windows in different modes side by side.
  No new dependency: the `windows` crate was already here for the corner rounding and gained a feature.
  Verified as far as I can without changing a system setting on the founder's machine, which is his to change: with a temporary probe, the app read the real registry value (1) and chose Glass, and the watcher opened the key and blocked. The change path itself is on his verification list — turn transparency off in Settings > Personalisation > Colours with a note open, and the windows should go Solid without being reopened.
  Original note: Compositor blur is applied once, at window creation. If the user turns transparency off afterwards — or Windows does it for them under battery saver — the window stays in Glass mode with nothing behind the tint, which will read as a washed-out surface rather than a deliberate Solid one. Needs a listener on the setting and a way to re-resolve `data-surface` in a live window.

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
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: The note window's main chunk is 555 kB raw, 193 kB gzipped — mostly CodeMirror plus the HTML, CSS and JavaScript grammars that `@codemirror/lang-markdown` pulls in unconditionally (see `docs/FIXES.md`). Loaded from disk, so no network cost, but every open note window parses its own copy.
  2026-09-06 — measured rather than changed, which is what this item asked for. A production build ships **677 kB of raw JavaScript** across both windows, 14.5 kB of CSS and 176 kB of fonts.
  Where it sits matters more than the total. The hub's own chain is about 64 kB; effectively all the rest belongs to the note window, and it is CodeMirror plus the language set. That matches this item's own guess, and `docs/FIXES.md` records why the language set cannot simply be lazy-loaded.
  The number that actually bears on "optimized" is not the total but the multiplier: **each note window is its own WebView2 renderer and parses its own copy.** Ten notes open is ten parses of the same 600-odd kB, which is the shape of this product. That is the argument for trimming the language set, and it is a stronger argument than bundle size on its own would be.
  Not acted on. Cutting languages is a product decision — it decides which fenced blocks highlight — and it belongs to the founder rather than to a measurement.
  Measured 2026-09-05, one window, development build: the sticky.md process tree is 381.8 MB across 7 processes — 35.9 MB for the application and 345.9 MB across 6 WebView2 processes. Half the WebView2 processes on the machine belonged to other applications and are excluded.
  Still unmeasured, and both matter before drawing any conclusion: a release build, and the cost of the second and subsequent windows. Tauri shares one WebView2 environment across windows, so window two should cost a renderer rather than another tree — that is the number that decides whether the many-window architecture is affordable. `MASTER.md § Stack` claims materially lower memory across many windows; that claim is currently unverified in either direction.

### [SMD-020] Fenced code block presentation
Type:    feature
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as idea
Notes: Fence markers stay visible by design — hiding them leaves a bare language name floating above the block. A proper treatment gives the block its own surface with the language shown deliberately rather than as leftover syntax.
  2026-09-06, from the founder using it: fenced blocks have no surface — he wants a light rounded panel separating code from prose, as Obsidian has, and suggested the panel could be black regardless of theme, since developers rarely read code on a light background. The text also reads much larger than body text, and there is no colour in it.
  Two of those three are measured rather than reported. **The size is a real defect:** `--font-size-code` exists but is applied only inside tables, so fenced code inherits `--font-size-body`. And at the same 15px, Martian Mono sets `const value = 42;` 52% wider than Geist — 10.5px per character against 8.23px. Code does not read larger because it is taller; it reads larger because it is far wider, and width is what reads as size in a block. Matching Geist's rhythm wants about 12px; 13px (`0.8125rem`) is the likely landing, at 11% wider.
  Shipped (ADR-030): a dark rounded panel in both themes, the mono face at 13px with `--line-height-code`, and tokens told apart by hue from the `--tide-*` ramp.
  One thing found only by rendering it: the face and size could not be set on the `monospace` highlight tag, which is what the earlier note assumed. That tag covers *inline* code only — the contents of a fenced block are tagged by the language inside it, so the block never saw either. They are set on the panel, which is the thing that says "this is code" anyway.
  Verified in the running dev server behind a seeded probe note, in both themes, and the probe removed: the panel paints across seven lines with the corners rounded at the ends, the gaps between adjacent line backgrounds measure exactly 0, and the code sets in Martian Mono at 13px on a 19.5px line.
  **The missing colour was mine, not the face's.** ADR-025 took the hues out of the highlight style along with the accent, and the commit that did it said that was the part of the change most likely to be wrong. The founder has now independently reported it as missing, which settles that. Restoring it wants an ADR: colour inside a fenced block is carrying token classes, which is the functional use ADR-025 permits, but a dark panel inside a quiet note is an exception to principle 1 and should be written down rather than assumed.

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
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as idea
  2026-09-06  shipped
Notes: A failed write is logged to the console and the text stays queued for the next flush, so nothing is lost while the window is open. But the user is told nothing, and if the window closes the queued text goes with it. A full disk or a permissions problem should be visible in the window rather than only in a console nobody has open.
  Shipped as `SaveTrouble`: a lit `--signal-danger` lozenge that appears in the chrome only when a write has failed, carries the reason as its label, and retries when clicked. It does not recede — principle 2 stops at chrome that is carrying information.
  The reason is built from the typed error rather than from a string, so a missing notes folder, an unusable title and an I/O failure each say what they are. That is what the typed error boundary was for.
  The second half is the closing path, and it is the half that was actually losing text: the window used to flush and destroy itself regardless of whether the flush worked. A failed flush now refuses the first close and shows the indicator instead. Asking again closes anyway — by then the user has been told, and a window that cannot be closed is its own kind of broken.
  Verified as far as the harness allows: rendered in the running dev server behind a temporary probe, which is where the screenshot of the lit lozenge came from, and the probe removed after. The wiring from a real failed write is type-checked but not exercised — the browser pane cannot take keyboard input while it is hidden, so no save could be made to fail on purpose. The founder can exercise it in one move: rename `Documents/sticky.md` while a note window is open, type a character, and wait a second.

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
  What remains true, and is recorded in `docs/FIXES.md` instead: a CSS shadow is clipped at the window edge, so a shadow on the surface has no effect on the window itself. The shadow is the system's. If a different one is ever wanted, that is when the window has to be padded — not before. `--shadow-rest` was removed on that reasoning in SMD-048.

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
State:   shipped
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

### [SMD-060] Tighten the display face, and give the empty state the content face
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: Two notes from the founder on seeing Departure Mono in the hub. Its letters sit too far apart in the title — the face is monospaced, so they carry a code face's sidebearings rather than a title's. `--tracking-display` is `-0.08em`, measured rather than guessed: it renders "notes" 15.2% narrower, which is the 15% he asked for.
  And the empty state read in the display face. It is a sentence the application is saying, not identity, so it takes the content face. `docs/DESIGN.md`'s typography table said the display face was for empty states; that line is corrected rather than left to contradict the code.

### [SMD-059] Departure Mono replaces Handjet
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: ADR-029. The founder asked for display faces in Redaction's spirit, was given a shortlist, and picked Departure Mono — SIL OFL, Helena Zhang, verified from the licence file in the release rather than from a directory listing, which is the lesson ADR-011 paid for. Version 1.500, one 22 kB woff2, vendored whole rather than subset.
  Two things went with Handjet. Its `ELGR`/`ELSH` axes, which made degradation continuous and animatable — counted as an identity moment when Handjet was chosen, and now gone. And with it the natural home for SMD-051, typographic scrambling: that idea needs a different mechanism or a different face now.
  One defect caught on the way, in the browser against the running server: the hub's title had been leaning on `--handjet-weight`, so removing the axis tokens left it inheriting a heading's default bold, and the browser synthesised it on a face that ships one weight. On a pixel face a faux bold thickens strokes off the grid and stops looking like pixels. `font-synthesis: none` on the body, and the title names `--weight-body`.
  SMD-009, the Chakra Petch theme, is unaffected — it was always an alternate theme rather than the display face.

### [SMD-058] The palette, and the pin lights instead of inverting
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: ADR-028. The founder set a ten-stop ramp — yellow-green through teal to a deep blue — and said the pin looked wrong inverted. It is a lit lozenge now, taking its hue as `color` so the same `--gloss-tinted` and `--swatch-rim` the tint swatches use light it: one kind of object lit, not a second kind of control.
  The measurement picked the stop and the glyph together. The gloss lightens the top of a disc to 45% of its colour over white, which puts a *white* glyph under 2.3:1 at every stop on this ramp — so the glyph had to be dark, and a dark glyph clears 4:1 through the teal stops and falls away past them. `--tide-600` at 4.06:1 is the darkest that holds, and it is the middle of the ramp.
  The other half of the request needed no change and is not claimed as one: `.control.active` has kept `opacity: 1` since the chrome was built, so a pinned control has never receded. Confirmed by reading the rule, not by watching it — the browser pane throttles transitions while it is hidden, and an opacity reading taken there is worth nothing. Worth the founder's eye in the running app.
  Amended within the hour at the founder's pick: `--tide-600` was too strong at full strength, and having seen the ramp and a dilution series rendered as the control itself he chose `#78bec0`, the same hue at 65% toward the light surface. It is written out rather than mixed at read time so it cannot drift with a note's tint. The glyph gains contrast going paler — 7.34:1 against 4.96:1.
  Nine stops have no role yet. Principle 6 is what permits that: primitives name values, semantics name roles, and a stop nothing reaches for is not a colour in the interface. Red is untouched — `--signal-danger` has no equivalent on this ramp, and deleting a note is not a green.

### [SMD-057] The direction is Aqua; the lozenges take the gloss
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: ADR-027. Having seen the bezel running, the founder named what he had been after since the first design conversation: not Aero, Aqua — Rams with personality, and old enough now to read as retro rather than dated. The lozenges take the full gloss and every disc in the window shrinks to `--space-4`, the diameter the palette's dots already were, which he picked as correct.
  No accent colour comes back with it. ADR-025's colour rule turns out to describe Aqua rather than fight it — neutral chrome, hue reserved for the small round things — so only ADR-026's refusal of the gloss is superseded.
  Glyphs went to 9px and sit above the specular. At that size a highlight across the top of one is the difference between reading it and guessing.
  A bug came out of this that was already shipped: ADR-026's tokens had been written into the Frost block, so on Dark `--swatch-rim` was undefined and `border-color` fell back to `currentColor` — every disc rimmed in its own fill. I had looked at a dark screenshot and called it fine. The rule now recorded: a token mixed from `currentColor` is a primitive, a token that depends on what is behind it is theme-level, and neither theme is verified until both have been read. Both are now read from the running application: rims resolve per hue on Dark, and the seat and specular differ per theme as intended.

### [SMD-056] The tint swatches take the Aqua bezel
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  proposed to the founder as three treatments; he chose the bezel
  2026-09-06  shipped
Notes: ADR-026. What the Aqua traffic lights are actually made of is a rim in the fill's own hue, a bezel seating the disc, and going grey when the window is not in use — the gloss is the least of it. All three are taken; the gloss is not, because it would be the only glossy object in the application and it is the language ADR-025 had just removed.
  `--swatch-rim` is a `color-mix` against `currentColor`, so one declaration rims every tint and an eighth would need no new value. Verified in the running application rather than assumed: `color-mix` resolves in WebView2 and all seven rims come out as darkened versions of their own hue.
  Greying out while the chrome recedes was already what principle 2 asked for; the traffic lights just do it too.
  The founder then said one bezelled control among flat ones felt off, which it did. The pin and the close button are seated discs as well now, and the swatch grew to `--space-6` to sit level with them — the three together are the cluster the reference was reaching for. Pinned inverts rather than filling harder: once both were discs a stronger fill was too close to hover to tell apart. The disc moved to `src/app.css` as `.lozenge` when the second component wanted it.
  Checked in the browser against the running dev server, in both themes: all three controls measure 24px, the rims resolve per hue, and the pin's inversion stays legible on dark. An earlier reading in that session said the controls were 14px — that was a stale stylesheet in the harness, not a defect, and nothing was changed on account of it.

### [SMD-055] Palette dismissal, and bold at a lighter weight
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: Three things the founder asked for after using it. The palette's arrival was 160ms and is now 240ms. It had no dismissal at all — `{#if open}` removed it on the click, so it vanished rather than closed; it now stays mounted while it plays out and its own `animationend` reports when it is gone. That is deliberately not a Svelte transition: those run in JavaScript and would keep animating under `prefers-reduced-motion`, where a CSS animation has its duration collapsed by the global rule and still fires the event, so the palette disappears at once as it should.
  It leaves in 160ms rather than the 240ms it takes to arrive. Arriving is the palette presenting itself; leaving is getting out of the way.
  `**bold**` was 700 and is now `--weight-emphasis` at 600. Bold inside a paragraph is a change of voice, not a change of level, and 700 was reading as a second heading mid-line.
  Weight became tokens on the way: 650 was written as a literal in five places across the highlight style and the editor theme, which is the second use twice over.

### [SMD-053] Drop Aero; colour becomes functional
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: The founder is moving off Aero as a philosophy and keeping Rams. He asked for one thing to change in the app now — the colour — with the rest of the new direction to be worked out later. ADR-025 records it.
  The Aero accent is gone and nothing replaces it: the interface is ink on a tinted surface, and a hue appears only where colour is what tells the user what a control does. That is one control today, delete, in red. Amber and green are deliberately undefined — a colour with no job is how a palette turns decorative.
  Two things fell out of removing the accent that are improvements on their own. The always-on-top pin showed "on" as a colour and now shows it as a filled chip, which is a state you can see rather than one you have to have learned. And `--accent-glow`, whose only use was the editor's selection wash, became `--selection`, which is what it always was.
  Fenced code lost its hue too. Token classes are told apart by weight and darkness now. In a scratchpad a fenced block is usually something pasted in rather than something being written, so it costs little — but this is the part of the change most likely to be wrong, and it is a narrow ADR to reverse if it is.
  Note tints are untouched, as the founder specified. They are the note's paper, not interface colour.

### [SMD-054] Move the tint picker to the trailing edge
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: The swatch was at the leading edge of the chrome; the founder asked for it beside the other controls, before the pin. `WindowChrome`'s `leading` snippet becomes `controls` and no longer claims the space at the start, so the drag region is now the whole bar in one piece rather than split around something.
  The palette had to be re-anchored with it. It opened down and to the right from a control that is now near the window's trailing edge, and `.surface` clips what leaves it, so it would have been cut in half. It opens leftward from its right edge now, and scales out of that corner.
  The founder also reported the palette reading as see-through with text behind it, which it was: it painted itself with `--surface-paint`, and in Glass mode that is a translucent tint meant to sit over the compositor's blur — but what is behind a popover is the note's own text, not the desktop. It uses `--surface-solid` now, the same colour the note would be if it were opaque, which is the founder's own second suggestion and reads as part of the note rather than as a panel from elsewhere.

### [SMD-070] Dark takes the bezel back
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: ADR-035. The founder found the glossy controls wrong on the dark window and asked for the bezel — the treatment ADR-026 chose and ADR-027 replaced — as a dark-theme exception. He is right about why: a glossy disc needs light to be glossy about, and a dark window has none to give it, so the gradient and the specular read as plastic stuck on rather than glass set in.
  He asked for a bezel *component*. It is not one, and should not be: the difference is entirely token values — the seat becomes a bezel, the specular becomes transparent so the overlay paints nothing, and the fills become flat colours. No component changed and none knows which theme it is in.
  That is worth stating plainly because it is the first time principle 8 has been cashed rather than asserted: a theme is a complete set of token values and nothing else. A second component would have been two controls to keep in step, and they would have diverged the first time one gained a state the other did not.
  One thing did need adding: `--gloss-seat-pressed`, because a bezel inverts when pushed and the light moves to its bottom edge. It lives with the shared treatment rather than the theme, since pressing is a state and not a palette.
  Verified in both themes at four times size: dark renders flat with no background image, Frost keeps its gradient and highlight.

### [SMD-069] The dark bubble sat below its own window
Type:    bug
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  reported by the founder, and two more found by enlarging it
  2026-09-06  shipped
Notes: The founder reported two things about the dark theme and both were real, and looking at the control enlarged found a third that was worse than either.
  **The bubbles were darker than the window.** `--gloss-neutral` sat on its own dark scale and bottomed out at `#232220`, below the window's own `#16161a` once glass let a wallpaper through — so a control read as a hole punched in the window rather than an object resting on it. It is built up from `--surface-solid` now, every stop lighter than it, which is what the Frost gloss already did relative to its own surface.
  **A lit control kept a pale rim.** Exactly the specificity trap that had made a lit control come out grey an hour earlier, on the other property: components declared `border-color: var(--rim-control)`, which beat `.lozenge.lit` — same specificity, injected later. The rim is a default on `.lozenge` now, like the fill. The lesson did not generalise the first time because it was fixed as one property rather than as a rule.
  **The specular was a sticker.** A flat white ellipse across 56% of the bubble, hard-edged, painted over the glyph rather than on the glass. Obvious at four times size and quietly wrong at one. It is a radial gradient now, smaller, and it falls off.
  The glyphs also went to Aqua's weight — the founder's pick of the three icon options. At 58% of a 16px disc a 1.2px stroke is a glyph you infer rather than see.

### [SMD-068] One selected colour, and the lit lozenge extracted
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: Three of the founder's, and one thing found by looking.
  `--signal-engaged` is `--tide-400` now — the same green the application mark is made of — so "selected" is one colour across the application rather than one per surface. The diluted teal it replaced is gone rather than left as a token nothing reads.
  The settings toggle shows a dot rather than a check, and is `--space-4` like every other bubble. A dot because a check is an action being confirmed and this is a state being held.
  The scrollbar's track is inset at both ends, so the thumb stops short of the window's corners rather than running into them.
  The reuse check the founder asked for turned up one real duplication: the lit-lozenge recipe — tinted fill, pressed fill, swatch rim, and a glyph in the hue taken most of the way to black — was written out in four components, and glyph sizing in four. Both moved to `.lozenge` and `.lozenge.lit` in `src/app.css`; the tint swatches joined them, since a swatch is a lit lozenge by any other name. Components now supply only a hue.
  **The extraction broke something, and looking is what caught it.** A lit toggle came out grey in the dark theme: `.lozenge.lit` and a component's own `.toggle` have equal specificity, and component styles are injected after `app.css`, so the neutral fill won even when lit. The neutral is now the default on `.lozenge` and `.lit` the exception — set the exception, not the rule.

### [SMD-067] The toggle overshoots, and the scrollbars ignore the theme
Type:    bug
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  reported by the founder from the settings window
  2026-09-06  shipped
Notes: Two faults, both mine, both from building more than the situation asked for.
  The toggle was a sliding switch with a `.lozenge` as its knob, and the knob travelled past the end of its track. The founder's fix is better than a corrected travel distance: make it the bubble itself, lit when held, which is what the always-on-top pin already is. A switch has a track, a travel distance and an end stop to get wrong; a bubble has none of them. One component's worth of geometry deleted rather than debugged.
  The scrollbars were never styled at all, so they were the engine's default: an opaque light strip that stayed light in the dark theme and read as a pale rectangle laid over the glass rather than part of the window. Fixed in two halves, and it needs both — a transparent track with a rounded, inset thumb, and a colour scheme declared per theme. The scheme is what stops the platform drawing a light scrollbar on a dark window; no amount of styling the thumb reaches that.
  The `-webkit-` pseudo-elements are used rather than the standard `scrollbar-color`, deliberately: setting that property switches the engine to its own drawing and ignores the rest, and it can neither round the thumb nor inset it.
  Verified in both themes in the running dev server.

### [SMD-064] The settings window
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  accepted into the second autonomous bundle by the founder, who chose a window of its own and "ask at the time" for the folder
  2026-09-06  shipped
Notes: ADR-033. Closes SMD-046 and SMD-003 together — the notes folder is a setting, and it had no home. A third window type, opened from the tray, carrying theme, notes folder, the new-note shortcut and launch-at-startup, and naming the settings file for anyone who would still rather edit it.
  A bug found while building it, which no one would have hit until the shortcut became changeable: the global shortcut's handler compared the pressed chord against one captured when the plugin was installed. The plugin installs once, so changing the chord would have registered a new one the handler then ignored — the shortcut would have gone silently deaf. The handler no longer checks which chord fired, since only one is ever registered.
  The folder move renames first and copies only across volumes, and never removes an original before its copy succeeds: an interrupted move leaves the note in the old folder rather than nowhere. Names already taken in the destination are not overwritten.
  `notes_dir` deliberately does not fall back to the default when a configured folder has gone. Writing notes somewhere the user is not looking is worse than saying the folder is missing.
  One defect caught by looking: the shortcut field clipped the last letter of the default chord at 11rem. It is 13rem.
  Verified in the browser against the running dev server — the window renders, both toggles and all four fields are present. The commands behind it are type-checked and compile, but nothing exercised them: a plain browser has no backend, so the picker, the move and the shortcut change are on the founder's list.

### [SMD-063] Monospaced text still reads larger than prose
Type:    bug
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  reported by the founder a second time, after the size was already reduced once
Notes: Measured properly this time, and the first measurement was wrong. Martian Mono is loaded lazily, so on a note with no code in it the face is not loaded at all and a canvas measurement silently falls back to another one. The x-height figures taken that way said mono was 15% *smaller* per em; with the face actually loaded it is 15% larger.
  What is true: on screen, code at 13px and prose at 15px have the same x-height — 7.92px against 7.97 — so code was never taller than the text around it. It is wider. The same sentence runs 35% longer, and that extra ink is what reads as a larger size.
  No size fixes both. At 12px the width gap closes to 25% and the glyphs go 8% shorter than the prose; at 11px it is 14% and 16%, and the code starts to look shrunken. 12px is shipped as the better of the two trades.
  Shipped as a face swap, which the founder chose over living with the compromise: JetBrains Mono at 13px, where it has the same x-height as the prose and runs 16% wider rather than 35%. ADR-032. Both faces were measured at 12, 13 and 14px before choosing.
  It costs 29 kB — one 92 kB file replacing two subsets totalling 63 kB — and it costs the italic on comments, since the face ships one style and `font-synthesis: none` means an italic that is not in the file would be a rule doing nothing. Colour carries comments alone now.

### [SMD-062] The seated controls are the window's, and wear the note's colour
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped (founder)
Notes: ADR-031. Four reports from the founder, three of which were one question — what belongs to the family of seated controls.
  The tint swatch staying visible on a coloured note was not the shared-widget bug he suspected; it was deliberate, and mine. `.swatch.coloured` kept it up on the reasoning that it was chrome carrying information. The information is already in full view — the note is the colour — so the exemption is gone and it recedes like everything else. The greyed-out state went with it: that existed only for a swatch that stayed.
  The window's controls now take the note's hue, disc, rim and glyph. One block keyed on the bare `[data-tint]` attribute does it, unqualified by theme because it declares no surface token; both themes work from one recipe because the disc carries its own hue and the glyph derives from it.
  The hub's delete control is flat again, at `--space-5` — 25%, not the founder's 15–20%, because the spacing scale is 4px-based and 18px is not on it. It still answers a press: pressable and bezelled are different things. `--space-5` is new to the scale and was simply missing.
  Code blocks got more room: 16px at the sides, 12px top and bottom.
  Verified in the running dev server, both themes, with probes for a tinted window and a hub row, all removed after. One reading not to trust: a receded swatch still reports `opacity: 1` in the hidden browser pane. The rules are unambiguous — `opacity: 0` unless `.revealed` or `:focus-visible`, and no `.coloured` rule survives — but the pane does not recompute while hidden, so the founder's eye is the check.

### [SMD-061] Restyle the hub's delete control
Type:    feature
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged as idea (founder)
Notes: The founder wants it square rather than round, with the grey bezel the window's controls wear, appearing only on hover, and pressing it should read as a click.
  Half of that already holds and should not be claimed as work: it is revealed by `.row:hover` in CSS today. What is missing is the lozenge treatment at a square radius, and a press state — the control has a hover colour but nothing that responds to the press itself.
  Shipped, then partly reversed the same day by SMD-062 at the founder's report: the seated treatment was the wrong family for it, and it is flat and larger now. The press state this item produced is what survived, and it is on every seated control.
  As shipped: square at `--radius-chip`, `--space-4` like every other seated control, with the neutral gloss and the grey rim. Round is now the default rather than the only option: the round ones are a note's own colour and its window's controls, and this is neither.
  The press turned into the more useful half of the item. It lives on `.lozenge` rather than on this button, because a control that answers a press is not a property of the hub — so the fill arrives as `--lozenge-fill` and the shared treatment owns the states. Pressed, the fill lights from below and the specular goes out; a highlight on a face no longer turned toward the light is what makes a pressed state look painted on rather than pushed in. Every seated control in the application answers a press now, not only this one.
  The name `.lozenge` stayed. A lozenge is a small tablet, not specifically a circle, so it survives having a square member; renaming it would have been churn across three files for no gain.
  Verified in the running dev server behind a probe that seeded two rows, since the hub has no backend in a browser: the control measures 16x16 at a 6px radius with the neutral gloss and rim, the hover-only rule is intact, and both `:active` rules resolve with the pressed fill. The click *feel* is the founder's to judge — the browser pane takes no clicks while it is hidden.

### [SMD-051] Typographic scrambling
Type:    idea
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged as idea (founder)
Notes: One of the two effects the founder named as the new direction's second half — the drafted feel of a scratchpad, against Rams restraint. Not specified yet beyond the name.
  Worth deciding before it is built: where it is allowed to happen. `docs/DESIGN.md` principle 1 keeps the user's text quiet, so scrambling settling into place on *note content* would fight it — on the hub's title, an empty state, or the mark, it would not. That is a design decision, not an implementation one.
  Also worth knowing in advance: it must not run on text the user is editing. Anything that rewrites glyphs in the buffer would break the rule that markdown syntax stays in the buffer at all times, so this is a rendering effect over stable text, never a transform of it.
  Shipped as `Scramble`, on the two titles the display face names: the hub's and the settings window's, each settling as its window opens. Both were the "chrome, not content" answer this note asked for.
  The mechanism this item lost with Handjet turned out not to be needed. Handjet's constant advance widths were what made *axis* animation reflow-free; Departure Mono is monospaced, which gives the same guarantee for substituted characters, for a different reason. The pool is punctuation rather than letters, because letters mid-scramble read as words that are not there.
  `prefers-reduced-motion` is honoured in JavaScript rather than CSS: the global rule collapses transition and animation durations, and cannot reach a `requestAnimationFrame` loop.
  **Not observed running.** The browser pane runs zero animation frames while it is hidden — measured, 0 in 500ms with the page reporting itself visible and reduced motion off — so nothing rAF-driven can be seen there. That is the same root cause as the stuck opacity transitions and the unobservable press state earlier in the session, and it is worth remembering before chasing another one: **if an effect depends on rAF or on a transition, the pane cannot show it, and a reading taken there means nothing.**

### [SMD-066] Opening a note from the hub wedges the event loop
Type:    bug
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  reported by the founder: the app freezes when opening a note from the hub
  2026-09-06  traced with probes — the open path is not the cause
Notes: The founder reported it as "crashes when opening a note from the hub". It is neither a crash nor the open path. Probes in the running application show:
  `surface::apply` entered for the hub, printed "rounded, reading the setting", and never printed again. `open` for the note then entered, reached `build`, and never returned. Nothing panicked; every thread was in `Wait` with flat CPU.
  So the sequence is: frosting the hub hangs, that stops the event loop, and the note window's `build` — dispatched to the event loop from a command thread — waits for a thread that is never coming back. Opening a note is the first thing that *needs* the event loop, which is why it looks like the cause.
  `surface::refresh` never appeared in the trace, so the transparency watcher is not firing and is not involved. Two `refresh` lines later in the log are app restarts caused by icon files landing in `src-tauri/`, which the dev watcher rebuilds on.
  **One hypothesis raised and disproved.** It looked like the registry read might block while the watcher's synchronous `RegNotifyChangeKeyValue` was pending on the same key — it fit the evidence exactly, including working at startup, because `watch` starts after the first `refresh`. A test that reproduces that shape says otherwise: the read returns in well under a second with a notification pending. The test is kept, in `surface.rs`, because it documents a real hazard that was worth ruling out and would otherwise be re-guessed.
  **Found, and it was none of the things it looked like.** A probe on the far side of `apply_acrylic` showed it returning `Glass` — the frost is fine. The hang is `.build()`, on **ThreadId(1)**, the main thread.
  A synchronous Tauri command invoked over IPC runs on the main thread from inside the web view's message callback. Creating a web view there cannot finish: creation needs the message loop to pump and the loop is inside the callback. The native window appears empty — which is the blank window the founder saw — the loop wedges, and the global shortcut and window dragging go with it. Typing in an open note kept working because that is the web view's own process.
  Fixed by making the four window-building commands `async`, so Tauri runs them on the async runtime. `docs/FIXES.md` carries the rule.
  Four attempts, three wrong, and the method is the point. Guessing produced: the transparency watcher (disproved — `refresh` never appears in the trace), the registry read blocking under a pending notification (disproved by a test, which is kept), and `apply_acrylic` (disproved by a probe on its far side). What worked was making the application reproduce the failure by itself — opening a window from a spawned thread, then from the main thread, then letting the hub's own frontend invoke over IPC. Only the last hung, and it hung every time.
  The earlier exit with code `0xcfffffff` is a separate thing and stays attributed to a release build run against the same target directory as the live dev instance.

### [SMD-052] Line boil animation
Type:    idea
State:   idea
Created: 2026-09-06
History:
  2026-09-06  logged as idea (founder)
Notes: The other named effect: edges that wobble between a few frames, the way hand-drawn animation does. It is what would make the app look drafted rather than rendered.
  The constraint to check first is cost, since low resource use is one of the founder's two stated priorities. A boil is per-frame, and doing it to a *window edge* is the expensive case — that edge is the compositor's, and the surface cannot move (`docs/FIXES.md`). Doing it to an SVG stroke inside a window, cycling two or three prepared paths at a low frame rate, is cheap. The cheap version is almost certainly the right one, and it points at the mark and at icons rather than at the window.
  2026-09-06 — parked by the founder: "a cool concept but I don't really know how I would fit it. Maybe later." Not dropped, because he may come back to it; not blocked either, since the blocker below is no longer what is stopping it. What stops it is that nothing in the application has asked for it yet.
  The original blocker, still true: the subject this item points at is the application mark, and the founder is redrawing it. The mark in `assets/icon/` is also still the old one — it carries the aqua gradient ADR-025 removed and is titled "StickyMD" rather than "sticky.md" (ADR-019).
  Boiling a mark that is about to be replaced is work thrown away, and every other candidate is worse: the window's glyphs are controls a user looks at constantly, and the titles are text, which is scrambling's job rather than a boil's. This wants the new mark first.

### [SMD-050] A deleted note could come back as an empty window
Type:    bug
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  found while checking the notes folder after the founder's hub test
  2026-09-06  shipped
Notes: Found by looking rather than by report. The founder's notes folder held an index entry for `testing-hub-responsiveness.md`, a note whose file is not there — so something leaves entries behind.
  The path that can do it: deleting a note destroys its window before trashing the file, and destroying a focused window makes the system take focus away from it first. A focus loss is one of the two moments placement is written. So the delete and the window's last write run at once on different threads, and if the write lands after `forget_entry` it puts the entry back, marked open. `restorable` filtered on `open` alone, so the next launch would open an empty window carrying the deleted note's name.
  Fixed with two independent guards, since either alone leaves a hole: `set_placement` writes nothing for a note that is not in the folder, and `restorable` requires the file to exist as well as the entry to say open. The second one also covers the case no in-process ordering can reach — a note deleted from the folder while the application is not running. Three tests, 27 passing.
  What is *not* claimed: that this is how the founder's orphan got there. Its entry says `open: false`, and the race produces `open: true`. Something else can leave an entry behind and I have not found it. The guards above make an orphan harmless rather than absent, which is the right order to do this in, but the second cause is still open. Not chased further because it is untraceable after the fact — worth watching for a fresh orphan appearing in a folder whose history is known.

### [SMD-065] V1 release readiness
Type:    chore
State:   active
Created: 2026-09-06
History:
  2026-09-06  accepted into the second autonomous bundle by the founder
  2026-09-06  everything testable without a remote, tested
Notes: The release workflow could not be run — that needs a remote and a tag, and neither exists yet — so what it does was run by hand instead, which catches everything except the Actions runner itself.
  `npm run tauri build` completes: release profile in 2m45s, NSIS fetched and verified, installer produced at **1.6 MB**. The portable zip was assembled with the workflow's own PowerShell and comes to **1.84 MB**, holding `stickymd.exe` (3.79 MB uncompressed), `LICENSE` and `README.md` in a versioned folder. Both artefacts are what `MASTER.md § Deployment` promises.
  **One thing will stop the first tag, by design.** Both manifests say `0.1.0`, and the workflow refuses a tag that disagrees with them. Tagging `v1.0.0` fails until `package.json` and `src-tauri/tauri.conf.json` are bumped. That is the check working — an installer named after the wrong version is only ever noticed after someone downloads it — but it is worth knowing before the tag rather than after.
  Left `active`: the workflow's first real run is still unobserved, and it is the only part of this that has never executed.

### [SMD-049] The release workflow
Type:    chore
State:   shipped
Created: 2026-09-06
History:
  2026-09-06  logged and shipped
Notes: `CLAUDE.md` has said since Phase 1 that build artifacts come from GitHub Actions on a `v*` tag and never from a developer's machine, and that every release carries both the NSIS installer and the portable zip. Nothing implemented either. `.github/workflows/release.yml` does now.
  Untested, and it cannot be tested yet: there is no remote. It will run for the first time on the first tag pushed to one, and the first run is the one to watch.
  Four decisions in it worth having written down. It uses no third-party actions — only `actions/checkout`, `actions/setup-node`, the runner's own rustup and the `gh` CLI — because this is the one workflow whose output people download and execute, and a third-party action in it is a supply-chain dependency in exactly the wrong place. It runs `cargo test` before building, since those tests carry the path and name validation that handles untrusted input. It fails early if the tag disagrees with `tauri.conf.json` and `package.json`, because a version mismatch produces an installer named after the wrong version and is noticed only after someone downloads it. And it creates the release as a **draft**: `MASTER.md` says every release carries written notes, and notes generated by a workflow are not written notes — the assets are attached and the release waits for the founder.
  The portable zip carries `LICENSE` and `README.md` alongside the binary. GPL-3.0 requires the terms to be conveyed with the binary, and a zip someone extracts on its own is the case that would otherwise arrive without them.
  Not cached. A cold Rust build on a runner is several minutes, and releases are rare; the cache actions available are third-party, which is the thing this workflow is avoiding. Revisit only if release builds become frequent enough to be annoying.

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
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as idea (founder)
Notes: A ring of glass bubbles opening from the pointer on right-click, each appearing staggered after the last. The founder's intent is a home for anything that cannot sit cleanly on the chrome, which is a real problem given `docs/DESIGN.md` principle 2 keeps the chrome minimal.
  Feasible, with one nuance worth recording before anyone builds it: the bubbles cannot carry *compositor* glass, which is a per-window property. They would use `backdrop-filter`, and that is legitimate here — DESIGN's never-allowed rule forbids it for the *primary window surface*, because a web view cannot see the desktop. Over the app's own content, which is what a bubble sits on, it is the correct tool, and a popover blur token would be the thing to add for it. (An earlier version of this note said `--blur-popover` already existed in the contract. It never did — checked, in the course of SMD-053, which needed a popover surface and found nothing to read.)
  The stagger must animate `transform` and `opacity` per bubble with a delay, never blur. Small non-glass elements may fade; that is already allowed.
  Also needs the web view's own context menu suppressed, or the native menu will appear alongside it.
  Shipped (ADR-034) with four actions that all exist elsewhere: New note, All notes, Settings, and the pin. Built as a mechanism first on purpose — which actions are in the ring is one array, so the founder can change the contents without touching any of the work.
  One thing this note got wrong: the bubbles do not need `backdrop-filter`. The reasoning here was that a bubble sits over the app's own content, where the filter is legitimate — true, but beside the point, because the seated lozenge is an opaque gloss. There is nothing to see through.
  Verified in the browser: four bubbles at the right ring geometry with staggered delays, the centre clamped so a right-click in either corner keeps every bubble inside the window. The stagger itself was not observed — the pane freezes animations partway, so only the bubbles whose delay had elapsed were drawn.

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
State:   shipped
Created: 2026-09-05
History:
  2026-09-05  logged as idea
  2026-09-06  reported by the founder from the hub, and decided: the delete wins (ADR-023)
  2026-09-06  active
  2026-09-06  shipped in af3bf6e; confirmed by the founder, who deleted from the hub and saw the files reach the recycle bin
  2026-09-06  state corrected — it had been left `active` after shipping, and was caught by a reconciliation sweep rather than at the commit that fixed it
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
