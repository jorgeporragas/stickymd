# ADR-019 — The product is styled "sticky.md"
Status:     Accepted
Date:       2026-09-05
Context:    The project was written throughout as "StickyMD". The founder asked for it to be styled "sticky.md" — lowercase, with the markdown file extension as part of the name rather than a suffix bolted on.
Decision:   "sticky.md" is the product's name wherever a person reads it: the window title, the installer, documentation prose, and the Start Menu. Machine identifiers keep the flat form — the npm package, the Cargo crate, the repository, and the bundle identifier `com.stickymd.app` all stay `stickymd`.
Consequences: `productName` in `tauri.conf.json` is "sticky.md", and `mainBinaryName` is set to `stickymd` so the executable does not become `sticky.md.exe` — a name with a false second extension that reads as a file rather than a program.
            Historical decision records keep the name they were written under. ADR-007's title still says "StickyMD" because Context, Decision and titles are immutable: a record states what was decided at the time, and rewriting it would make the log claim a name that did not yet exist. Only this file's header was amended.
            One measurement in STATUS SMD-006 quotes the literal string "StickyMD scratchpad". It was deliberately left unrenamed — the number is the width of that exact string, and changing it would make a recorded measurement false.
            2026-09-05 — MASTER, CLAUDE, DESIGN and STATUS amended.
