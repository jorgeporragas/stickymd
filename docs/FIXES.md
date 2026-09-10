# FIXES

> **Authoritative for:** non-obvious solutions that must never be undone, each naming the area it guards.
> **Never contains:** anything that is not a fix. Work items live in `docs/STATUS.md`. Rules live in `CLAUDE.md`. Decisions live in `docs/DECISIONS.md`.
> **Last verified:** 2026-09-07

An entry belongs here when a solution was non-obvious, or when it took more than one failed attempt to get right. Each entry names the area it guards so that `CLAUDE.md § Areas that require reading FIXES.md first` stays accurate.

Never undo a documented fix. If a change would affect one, flag it before making the change.

## Entry format

```markdown
### <Short title>
Area:     <the area this guards — must match an area named in CLAUDE.md>
Date:     YYYY-MM-DD
Commit:   <hash>
Problem:  what went wrong, and what made it non-obvious.
Fix:      what was done.
Never:    what must never be done to this code again, and why.
```

---

### Link destinations hide only inside a Link node
Area:     The CodeMirror inline-rendering layer and its decorations
Date:     2026-09-05
Commit:   b930795
Problem:  Hiding only `LinkMark` left the destination on the line, rendering
          `[the docs](https://example.com)` as `the docshttps://example.com`.
          The obvious fix — adding `URL` to the hidden-markup set — breaks bare
          autolinks: `<https://example.com>` is *also* a `URL` node, and its
          destination is the only text it has. Hiding it blanks the line.
Fix:      `URL` is hidden only when `matchContext(['Link'])` is true.
Never:    Never add `URL` to `MARKUP_NODES` to simplify the check. The two
          cases look identical in the node name and differ only in context.
          Test both a `[text](url)` link and a bare `<url>` autolink after any
          change to this layer.

### HTML, CSS and JavaScript modes cannot be lazy-loaded
Area:     The editor's language set and bundle composition
Date:     2026-09-05
Commit:   b930795
Problem:  All six fenced-code languages were declared with a dynamic `load()`.
          Rollup reported INEFFECTIVE_DYNAMIC_IMPORT for three of them:
          `@codemirror/lang-markdown` statically imports `lang-html`, which
          statically imports `lang-css` and `lang-javascript`. They are in the
          main chunk whatever the language list says.
Fix:      Those three are imported statically and registered with `support:`.
          Python, Rust and JSON stay lazy with `load:` and genuinely split out.
Never:    Never "tidy" the list by making all six lazy again. It reads as
          consistent and is factually wrong — the bundler ignores it silently,
          and the code then claims a saving that does not exist.

### Block widgets must come from a StateField, not a ViewPlugin
Area:     The CodeMirror inline-rendering layer and its decorations
Date:     2026-09-05
Commit:   793ff8a
Problem:  Table rendering replaces several whole lines with one widget. Every
          other part of the rendering layer is a ViewPlugin, so that is where
          it was reached for first. CodeMirror forbids a plugin from producing
          decorations that replace line breaks — a block widget must come from
          the state.
Fix:      `tableView` is a StateField providing decorations through
          `EditorView.decorations.from(field)`. `livePreview` stays a
          ViewPlugin and skips descending into `Table` nodes so the two never
          decorate the same range.
Never:    Never move table rendering into `livePreview` for tidiness. The
          restriction is structural, not stylistic. The cost of the StateField
          is that it scans the whole document rather than the viewport, which
          is acceptable only because notes are small.

### Never style `t.content` with the monospace family
Area:     The CodeMirror inline-rendering layer and its decorations
Date:     2026-09-05
Commit:   793ff8a
Problem:  `HighlightStyle` mapped `[t.monospace, t.content]` to the mono
          family, on the assumption that `content` meant code content. It does
          not — `content` is a broad tag covering ordinary inline text, so the
          entire note rendered in the code face. Type-checking and DOM
          assertions both passed; only a screenshot showed it.
Fix:      The mono family is applied to `t.monospace` alone.
Never:    Never add `t.content` to a rule carrying a font family, colour or
          size intended for code. Check any highlight change against a
          rendered screenshot, not only the DOM — a wrong font is invisible in
          `textContent`.

### Ctrl+N cannot be bound inside the webview
Area:     Global and in-app shortcut registration, and tray lifecycle
Date:     2026-09-05
Commit:   01af465
Problem:  `Mod-n` was bound to "new note window" through CodeMirror and did
          nothing in the running application. The binding was not at fault:
          dispatching the chord in a browser fired it correctly, and a probe
          proved the Rust window-creation path worked on its own. WebView2
          claims Ctrl+N as a browser accelerator — Edge's "new window" — and
          swallows it before the page sees it.
Fix:      The chord is not bound in the web view at all. It is a global
          shortcut registered with the operating system from Rust, which sees
          it before WebView2 can. wry exposes
          `with_browser_accelerator_keys(false)`, which would disable the
          interception, but Tauri 2.11.5 does not surface it, so the chord
          cannot be reclaimed through Tauri's API.
          `Mod-Alt-n` was tried first and failed for an unrelated second
          reason — see the AltGr entry below.
Never:    Never "correct" this back to Mod-n because it reads more naturally.
          It is not a preference — the chord does not arrive. The same applies
          to the other accelerators WebView2 keeps: Ctrl+T, Ctrl+W, Ctrl+P,
          Ctrl+F, Ctrl+R, Ctrl+D, Ctrl+Shift+N and F5. A shortcut that must
          use one of those has to be registered as an OS-level global shortcut
          from Rust instead, which bypasses the webview entirely.

### Ctrl+Alt is AltGr on most non-US keyboard layouts
Area:     Global and in-app shortcut registration, and tray lifecycle
Date:     2026-09-05
Commit:   997460a
Problem:  After WebView2 swallowed `Ctrl+N`, the new-note chord was moved to
          `Ctrl+Alt+N`. That did not fire either. The development machine
          carries the Latin American layout (`0409:0000080A`), and on Latin
          American, Spanish and most European layouts Windows treats
          `Ctrl+Alt` as AltGr: the combination is consumed to compose a
          character and never arrives as a shortcut.
Fix:      No `Ctrl+Alt` chord anywhere. The new-note shortcut is registered
          with the operating system instead, as `Ctrl+Shift+Space`.
Never:    Never bind a `Ctrl+Alt` combination, in the editor or globally. It
          works on a US layout and silently does nothing for a large share of
          users, which is the worst kind of bug: invisible to whoever wrote it.
          Check a chord against both hazards before choosing it — browser
          accelerators and AltGr — or register it globally, which avoids both.

### A CSS radius does not round a window that has a compositor backdrop
Area:     Window transparency, vibrancy, and compositor blur setup
Date:     2026-09-05
Commit:   33aa6ac
Problem:  With acrylic applied, the note window showed grey triangles in each
          corner. The compositor draws its backdrop across the whole window
          rectangle, which is square; `border-radius` only rounds what the web
          view paints, so the corners outside it showed raw acrylic with none
          of the CSS tint over it.
Fix:      The window itself is rounded, through
          `DwmSetWindowAttribute(DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND)`
          in `src-tauri/src/surface.rs`, which makes the compositor clip its
          own backdrop. `--radius-window` was reduced to match the radius
          Windows chooses. `shadow` was turned on at the same time and had to
          be turned back off — see the entry below.
Never:    Never raise `--radius-window` above the system radius while glass is
          on — the difference reappears as untinted acrylic in the corners.
          Windows does not accept an arbitrary radius, only its own; a rounder
          note means giving up compositor glass, which is a product decision
          rather than a styling one.
          Tauri's `hwnd()` returns an HWND from its own version of the windows
          crate. Carry the raw handle value across rather than forcing the
          versions to match.

### Any sliver the surface does not cover shows the compositor backdrop
Area:     Window transparency, vibrancy, and compositor blur setup
Date:     2026-09-05
Commit:   baf4059
Problem:  Pale lines appeared down the right edge and along the bottom of
          restored note windows. `.surface` was sized `height: 100%`, which
          resolves to a fractional pixel — measured at 433.6px inside a 434px
          viewport. The window is transparent over acrylic, so the 0.4px the
          surface did not cover was painted by the backdrop rather than by
          nothing, and read as a pale line.
Fix:      `.surface` is `position: fixed; inset: 0`, which is laid out against
          the viewport itself and cannot land on a fraction. Verified: the gap
          on both axes is exactly 0.
Never:    Never size the note surface with a percentage. This is the same
          failure as the corner artefacts and has the same rule behind it:
          anything the surface does not cover, the compositor's backdrop does.
          A DPI explanation is tempting and was wrong here — but not for the
          reason first written down. It was recorded that the machine ran at
          scale 1.0; it runs at 1.25, and the 1.0 came from a PowerShell process
          that was not DPI-aware and was reporting its own virtualised view.
          The fractional height was real and had nothing to do with either
          number. Measure the gap before theorising about why it exists, and
          measure it from inside the application.

### `shadow: true` gives an undecorated window a frame the page cannot paint
Area:     Window transparency, vibrancy, and compositor blur setup
Date:     2026-09-05
Commit:   ab76fe1
Problem:  Thick pale lines ran down the right edge and along the bottom of every
          note window. Measured from inside the running application, the web
          view filled its viewport exactly — 436x420 CSS in a 436x420 viewport
          — but the window was larger than its own client area: inner 545x525,
          outer 563x535. Eighteen physical pixels on the right and ten at the
          bottom belonged to the window and to nothing else, and rendered white.
          `shadow: true`, added when the corners were rounded, created that
          frame.
Fix:      `shadow` is `false` in `src-tauri/tauri.conf.json`. With it off,
          `outer` equals `inner` exactly and nothing is left unpainted.
Never:    Never turn `shadow` on for a window with `decorations: false`. The
          frame it adds is not paintable by the page, and on a transparent
          window it shows as a light border rather than as nothing.
          Turning it off costs nothing visually: Windows draws its own shadow
          for a DWM-rounded window whatever this setting says. A CSS shadow
          would still be clipped at the window edge. `--shadow-rest` was removed
          for that reason in 8b72e28 — it had no
          effect on the window itself — the shadow you see is the system's.
          Two wrong diagnoses preceded this one, both from theorising about the
          gap instead of measuring it. One rested on a DPI reading taken from a
          process that was not DPI-aware, which returned scale 1.0 when the
          real scale was 1.25. Measure inside the running application.

### A window surface cannot scale, so a window cannot arrive by moving
Area:     Window transparency, vibrancy, and compositor blur setup
Date:     2026-09-06
Commit:   8b72e28
Problem:  `docs/DESIGN.md § Motion` called for a note to scale slightly into
          place as it appears. It cannot. `.surface` covers the window exactly,
          over a transparent window with compositor acrylic behind it: scaled
          down, the ring it vacates is painted by the backdrop as raw acrylic;
          scaled up, its rounded corners go past the window's edge and are
          clipped square. Both are the corner and sliver artefacts again,
          animated.
Fix:      The surface never moves. A window arrives by the light on it instead —
          an overlay pseudo-element carrying a sheen that fades out over
          `--dur-settle`. One paint, no layout, and nothing that touches the
          surface's own alpha or blur. DESIGN was amended rather than left
          describing something unbuildable.
Never:    Never animate the geometry of the element that covers a glass window.
          Anything nested *inside* it may scale freely — what is behind a
          palette is the window, not the desktop. That is the line: the surface
          is the window's own paint, everything else sits on it.

### Deleting a note races its own window's last write
Area:     The sidecar index, and note file rename and deduplication logic
Date:     2026-09-06
Commit:   a948181
Problem:  Deleting a note destroys its window first, then trashes the file,
          then forgets its index entry. But destroying a focused window makes
          the system take focus away from it, and a focus loss is one of the
          two moments placement is written — so the delete and the window's
          last write are running at once, on different threads, and whichever
          lands second wins. When the write wins it puts back the entry the
          delete had just removed, marked `open`, and the note comes back at
          the next launch as an empty window carrying its name. `restorable`
          filtered on `open` alone and had no reason to doubt the entry.
Fix:      Two independent guards, because either one alone still leaves a hole.
          `set_placement` writes nothing for a note that is not in the folder,
          so the losing write cannot recreate an entry. `restorable` requires
          the file to exist as well as the entry to say open, so an entry that
          got in another way — a note deleted from the folder while the
          application was not running, which no amount of in-process ordering
          can prevent — still cannot open a window onto a file that is gone.
          Both are tested.
Never:    Never treat the index as evidence that a note exists. It records what
          the windows were doing; the folder is what says what there is. Any
          read that acts on an entry has to ask the folder too.
          And do not fix this by reordering the delete. The two writes are on
          different threads and the ordering is the system's to decide, not
          ours — an ordering that happens to work is a race that has not fired
          yet.

### `apply_acrylic` succeeds with transparency switched off
Area:     Window transparency, vibrancy, and compositor blur setup
Date:     2026-09-06
Commit:   12ccc32
Problem:  Surface mode was decided by whether `apply_acrylic` returned an
          error. It does not error when the user has transparency effects
          switched off — it succeeds, and the compositor then draws nothing
          behind the window. The window stays in Glass mode painting a
          near-colourless tint over a transparent window with no backdrop, so
          the desktop shows through a faint wash. That reads as a washed-out
          window rather than as the deliberate Solid mode, which is exactly
          what `docs/DESIGN.md` principle 4 exists to prevent. Windows also
          switches the setting off on the user's behalf under battery saver,
          so this is reachable without anyone choosing it.
Fix:      The setting is read before the blur is applied, from
          `HKCU\...\Themes\Personalize\EnableTransparency`, and a window is
          Solid when it is off. It is then *watched*, with
          `RegNotifyChangeKeyValue` on a thread that is asleep the rest of the
          time, and every open window is re-applied and told when it changes.
Never:    Never treat a successful `apply_acrylic` as proof there is blur
          behind the window. The call reports whether it was accepted, not
          whether the compositor is drawing.
          And never poll for this. The key notification blocks until something
          happens, which is the whole reason a thread is acceptable here.
          The API cannot watch a single value, only a key, so other settings
          under `Personalize` wake it too — the handler re-reads and may
          conclude nothing changed, which is why it must be cheap.

### A command that builds a window must be `async`, or the application wedges
Area:     Window lifecycle
Date:     2026-09-06
Commit:   8fb5726
Problem:  Opening a note from the hub froze the whole application. The founder
          reported it as a crash on the open path; it was neither. Probes in the
          running application showed `open` reaching `.build()` and never
          returning, on **ThreadId(1)** — the main thread.
          A synchronous Tauri command invoked over IPC runs on the main thread,
          from inside the web view's own message callback. Creating a web view
          from there cannot complete: the creation needs the message loop to
          pump, and the loop is busy dispatching the callback we are standing
          in. The native window appears and stays empty, the event loop is
          wedged, and everything that needs it goes with it — the global
          shortcut, window dragging, and every window built afterwards. Typing
          in an already-open note still worked, because that is the web view's
          own process and needs nothing from the loop.
Fix:      `new_note_window`, `open_note_window`, `show_hub` and `show_settings`
          are `async`. Tauri runs an async command on the async runtime rather
          than the main thread, so the loop stays free to service the creation.
Never:    Never make a command that builds a window synchronous, however
          trivial it looks. The symptom does not point at the cause: the freeze
          surfaces at whatever *next* needs the event loop, which is usually
          opening something, so it reads as a bug in the open path.
          Three wrong answers were reached before the right one, each of which
          fitted the evidence at the time. What settled it was reproducing the
          failure automatically — the Rust side opening a window from a spawned
          thread, then from the main thread, then the hub's own frontend
          invoking over IPC — and only the last hung. A hypothesis that cannot
          be reproduced on demand is not a diagnosis.

### Regenerating the icons does not rebuild the icon into the application
Area:     Tauri bundler and release workflow configuration
Date:     2026-09-06
Commit:   88fd5e0
Problem:  The mark was regenerated and the application went on showing the old
          one. Nothing was wrong with the icon: `icons/icon.ico` was written at
          18:16:44 and the running binary had been built at 18:12:51. The icon
          is embedded as a Windows *resource* at build time, and `tauri-build`
          only reruns when something it watches changes — which the icon files
          are not. No Rust source had changed either, so the dev watcher had
          nothing to rebuild and the stale resource stayed in the binary.
Fix:      Touch `src-tauri/tauri.conf.json` after regenerating icons. The build
          script watches it, so the resource is rebuilt and the new icon is
          embedded. Confirm by comparing the timestamps: the binary must be
          newer than `icon.ico`.
Never:    Never conclude an icon is wrong because the application still shows
          the old one. Check that the binary postdates the icon file first —
          this looks exactly like a bad icon, and it is a stale build.
          Windows also caches icons for the taskbar and Explorer separately
          from the running process, so a pinned entry can lag behind a window
          that is already correct.

### The macOS branches cannot be compiled on the development machine
Area:     Tauri bundler and release workflow configuration
Date:     2026-09-10
Commit:   12cf412
Problem:  Every `#[cfg(target_os = "macos")]` branch is invisible to the
          compiler on Windows, so `cargo check` passing says nothing whatever
          about the code that only runs on a Mac — and three commits of it had
          accumulated that way before anyone noticed. The obvious answer does
          not work either: `rustup target add aarch64-apple-darwin` succeeds,
          and `cargo check --target aarch64-apple-darwin` then stops at
          `objc2-exception-helper`, whose build script compiles a `.m` file and
          wants clang together with the macOS SDK. Neither is obtainable here.
Fix:      `.github/workflows/check.yml` runs `cargo test` on `macos-latest` on
          every push. That builds the binary target, so every macOS branch in
          it goes through a compiler — and the tests it runs are the path and
          name validation, which is the code most likely to differ on a
          filesystem that is not Windows'.
Never:    Never take a macOS branch as compiled because `cargo check` passed
          locally, and never delete the Check workflow to save runner minutes:
          it is the only compiler that sees that code before a tag is pushed,
          and the alternative is proving the build by cutting a release. After
          changing anything under a macOS cfg, watch Check on the push.

### `Path::components` only knows this platform's separators
Area:     The sidecar index, and note file rename and deduplication logic
Date:     2026-09-10
Commit:   2784945
Problem:  `safe_name` rejected anything that was not a single ordinary path
          component, which is the right rule and was not enough. On Unix a
          backslash is an ordinary character, so `..\escape.md` arrives as one
          `Normal` component and passes — and so do `nested\note.md` and
          `C:\absolute.md`. The same names are traversals on Windows, where the
          component check catches them for free, which is exactly why the gap
          could not show: the test asserted all three from the day it was
          written and only Windows had ever run it.
Fix:      Reject `/` and `\` outright, on every platform, *before* the
          component check — an addition to that rule, not a replacement for it.
          The doc comment on `safe_name` says which check catches what.
Never:    Never remove the separator check as redundant because the component
          check "already handles paths": it handles them on the platform doing
          the compiling, and this validates a name that will be read on the
          others. Equally, never let it replace the component check — `..` and
          a Windows stream name like `note.md:evil` carry no separator at all.
          Any change here runs on macOS as well as Windows before it lands.
