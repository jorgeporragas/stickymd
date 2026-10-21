# ADR-010 — Builds are distributed unsigned
Status:     Accepted
Date:       2026-09-05
Context:    Unsigned Windows binaries trigger a SmartScreen warning. A certificate costs roughly $200–400 per year, which the project does not carry.
Decision:   Ship unsigned. The README documents the warning honestly.
Consequences: A stranger's first experience includes a security warning they must click through. Revisitable if the project ever attracts funding.
