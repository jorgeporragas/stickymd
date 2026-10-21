# ADR-043 — Work moves to GitHub issues; pre-3.0 items are frozen in docs/COMPLETED.md
Status:     Accepted
Date:       2026-10-13
Context:    Adopting FLOW 3.0, which tracks work in the host's issues rather than in files (FLOW's D-016).
Decision:   The open items and the real-world checks the record showed unconfirmed became GitHub issues. The 100 items closed before the adoption were kept word for word in `docs/COMPLETED.md`, with STATUS's completed milestones, rather than deleted, because truth files and FIXES cite SMD IDs. `docs/COMPLETED.md` is frozen; new work is tracked in issues. The founder's reason: it is a FLOW system change, and it solves the context issues of a growing item log while improving context and credit cost.
Consequences: STATUS no longer carries an item log. SMD IDs stay valid: a closed one resolves in `docs/COMPLETED.md`, and a migrated issue's title opens with its SMD ID.
            2026-10-13 — STATUS, MASTER, DECISIONS and FIXES headers amended to name issues.
