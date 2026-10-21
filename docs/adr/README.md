# sticky.md — Architecture Decision Records

Why sticky.md is the way it is: one file per decision, `NNNN-short-title.md`, headed
`# ADR-NNN — Title`. Work lives in the repository's GitHub issues, and current policy lives in
`docs/PRD.md` › *Permanent Vetoes* or in an issue's state; nothing may cite an ADR as current
policy. The template is in `.claude/rules/flow.md` › *Decisions*.

`Context` and `Decision` are immutable once written. `Status` changes only to record
supersession, and `flow_check`'s C6 checks that both ADRs say so. `Consequences` is append-only.

These were one file, `docs/DECISIONS.md`, until ADR-047 (2026-10-17).
