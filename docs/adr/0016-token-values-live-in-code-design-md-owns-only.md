# ADR-016 — Token values live in code; DESIGN.md owns only the contract
Status:     Accepted
Date:       2026-09-05
Context:    DESIGN.md was drafted during Planning with the full token contract and every value of the Frost theme written out. When `src/lib/tokens/tokens.css` was created during the Phase 1 scaffold, every one of those values existed in two files. Changing a colour would have required editing both, and one would eventually disagree with the other.
Decision:   `docs/DESIGN.md` is authoritative for which tokens exist and what role each plays. `src/lib/tokens/tokens.css` is the single source for every value. DESIGN links to it and never restates a value.
Consequences: A reader of DESIGN.md cannot see the palette without opening the code, which is the cost of the rule. Added to DESIGN's never-allowed list so the duplication cannot creep back. Window dimensions are the one value DESIGN still names, because they are a window property in `src-tauri/tauri.conf.json` and JSON carries no comment to point back with — DESIGN records that asymmetry explicitly.
            2026-09-05 — docs/DESIGN.md amended; values removed in commit 2a5b11b.
