# ADR-006 — Filenames are slugified from the first line and renamed on a debounce
Status:     Accepted
Date:       2026-09-05
Context:    ADR-001 makes the notes folder browsable, so filenames should be human-meaningful rather than opaque identifiers. But the first line changes as the user types, and renaming per keystroke would hammer the disk and any sync client watching the folder.
Decision:   The filename is the slugified first line, deduplicated with a numeric suffix, applied on a debounce after typing stops or when the window closes. A note with no first line is `untitled`.
Consequences: The folder reads as a list of titles. Requires collision handling and debounce tuning. The hub displays titles without the deduplication suffix; only disk carries it.
            2026-09-05 — Implemented. Two cases the decision did not anticipate: Windows refuses `con`, `nul`, `com1` and eighteen other stems whatever the extension, so those get a `-note` suffix; and a title that slugifies to the stem a note already holds must *not* trigger a rename, or every save would churn the file. Naming lives in Rust because deduplication has to see the folder.
