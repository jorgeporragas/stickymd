# ADR-044 — The caret steps over an inline mark as one unit
Status:     Accepted
Date:       2026-10-13
Context:    Syntax is revealed on the cursor's line (MASTER § In Scope), so on that line `**` is two real characters and the caret stepped through them one at a time. After `Mod-b` and typing, ArrowRight landed between the closing asterisks (`**word*|*`) instead of leaving the mark. The founder asked for the behaviour of Obsidian.
Decision:   The caret steps over a whole inline mark (`**`, `__`, `*`, `_`, `~~`, a backtick) in both directions, with or without Shift, wherever the mark sits. Typing a mark character when the caret is in front of the same closing mark steps over it instead of inserting. Backspace and Delete stay one character at a time.
Consequences: MASTER § In Scope amended. Implemented as a keymap and an input handler, not `EditorView.atomicRanges`, which would also make Backspace remove a whole mark and stop a click from placing the caret inside one; see `docs/FIXES.md`.
            Marks come from the syntax tree, so `2 * 3` and a lone `*` are unaffected. Typing over a closing mark is one character at a time, so the first `*` of a closing `**` leaves the caret between the two.
            2026-10-13 — MASTER § In Scope amended.
