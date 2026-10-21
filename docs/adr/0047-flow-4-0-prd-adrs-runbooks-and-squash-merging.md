# ADR-047 — FLOW 4.0: a PRD, one file per ADR, runbooks, squash merging, no STATUS
Status:     Accepted
Date:       2026-10-17
Context:    FLOW 4.0 cut FLOW down to what a check enforces or one sentence states, under
            standard names (jorgeporragas/flow#31). Banana Splits adopted it first
            (jorgeporragas/banana_splits#99), and the founder asked for this repository to follow.
Decision:   Adopt FLOW 4.0. FLOW's rules are `.claude/rules/flow.md`, loaded at every session
            start, replacing `FLOW_SYSTEM.md`. `MASTER.md` becomes `docs/PRD.md`. Decisions are one
            file each in `docs/adr/`. STATUS is retired: its work was already in issues (#6, #19),
            and its environment table moved to `docs/runbooks/environment.md`. Pull requests are
            squash-merged. No new `SMD-NNN` IDs: issues use GitHub's `#N`, and the migrated issues
            keep their `[SMD-NNN]` titles.
Consequences: flow_check 0.3.0 checks ADR pointers across files (C6) and a pull request as one
            change (C8). Citations of `MASTER.md`, `docs/DECISIONS.md` and `FLOW_SYSTEM.md` were
            repointed in the adopting pull request, except in `docs/COMPLETED.md`, which is never
            edited, and in ADRs, which are history.
