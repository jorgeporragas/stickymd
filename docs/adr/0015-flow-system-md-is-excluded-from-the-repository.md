# ADR-015 — FLOW_SYSTEM.md is excluded from the repository
Status:     Superseded by ADR-042
Date:       2026-09-05
Context:    Gitignoring the FLOW documentation as a whole was considered and rejected: FLOW's central contract is that a change and the docs it invalidates land in the same commit, which is meaningless if the docs are untracked, and reconciliation works by diffing `git log` against STATUS.md. The founder's actual concern was narrower — FLOW is his own system, still in development, and he does not want to publish it unfinished.
Decision:   Track `CLAUDE.md`, `MASTER.md`, `docs/`, `.githooks/` and `.gitattributes`. Exclude only `FLOW_SYSTEM.md`.
Consequences: All four of FLOW's nets remain intact. A visitor sees a repository with a scope document and a decision log, not the spec that produced them. `CLAUDE.md` names the file as local-only so the reference is not dangling for someone who clones.
