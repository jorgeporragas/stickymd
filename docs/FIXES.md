# FIXES

> **Authoritative for:** non-obvious solutions that must never be undone, each naming the area it guards.
> **Never contains:** anything that is not a fix. Work items live in `docs/STATUS.md`. Rules live in `CLAUDE.md`. Decisions live in `docs/DECISIONS.md`.
> **Last verified:** 2026-09-06

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
          A DPI explanation is tempting and was wrong here — the machine it was
          reported on runs at scale 1.0, where physical and logical pixels are
          identical. Measure the gap before theorising about why it exists.

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
          would still be clipped at the window edge, so `--shadow-rest` has no
          effect on the window itself — the shadow you see is the system's.
          Two wrong diagnoses preceded this one, both from theorising about the
          gap instead of measuring it. One rested on a DPI reading taken from a
          process that was not DPI-aware, which returned scale 1.0 when the
          real scale was 1.25. Measure inside the running application.
