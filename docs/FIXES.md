# FIXES

> **Authoritative for:** non-obvious solutions that must never be undone, each naming the area it guards.
> **Never contains:** anything that is not a fix. Work items live in `docs/STATUS.md`. Rules live in `CLAUDE.md`. Decisions live in `docs/DECISIONS.md`.
> **Last verified:** 2026-09-05

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
