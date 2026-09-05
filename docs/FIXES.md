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
