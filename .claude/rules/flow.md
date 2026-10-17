# FLOW — project rules

**Version 4.0.0.** Installed by FLOW; never edit it in a project (C10). It loads at every
session start. The founder owes two things: decisions, and checks on real hardware or
dashboards. Everything else is yours.

## The docs

| Doc | Holds | Read |
|---|---|---|
| `CLAUDE.md`, `.claude/rules/` | Rules for writing code here | Loaded for you |
| `docs/PRD.md` | What the product is: user, problem, core loop, domain model, stack, scope, vetoes | Before a new feature |
| `docs/adr/` | Architecture decision records, one file each | When a past decision is in question |
| `docs/DESIGN.md`, `docs/components/` | The visual system; one file per shared component | Before UI work, and the file for each component you touch |
| `docs/SCHEMA.md` | The database's invariants and their reasons | Before database work |
| `docs/FIXES.md` | Fixes no test can hold, by area | The area, before touching it |
| `docs/runbooks/` | How to operate things, and state that lives outside the repo | When doing that task |
| GitHub issues | All work: what is planned, in progress, blocked, or waiting on the founder | The issue you work on |

Docs describe what is true now. Plans and progress live in issues.

## Changes

- **Every change reaches `main` by a pull request the founder merges** (squash). Work on a
  branch, push, open the PR, stop. Never push to `main`, force push, merge or deploy.
- **CI must be green.** Never raise a limit in `.flow/config.toml`, skip a test or weaken a
  workflow to get green unless the founder says so in the PR.
- **Database changes need the founder's confirmation** before they run.
- **Docs change in the same PR as the code they describe.** The Definition of Done:
  - a migration → SCHEMA (C8 checks it);
  - a new shared component → its file in `components/` (C7);
  - removed code → search the docs for it and fix what still describes it;
  - a new dependency or pattern → CLAUDE.md or a rule file;
  - a non-obvious fix → a regression test; FIXES only if no test can hold it.
- **When you edit a doc, check that every path and code name in the part you touched still
  exists.**
- **Never undo a FIXES entry.** If a change would affect one, say so first.
- **A rule without a check is a suggestion.** When a rule can be a test, write the test, and
  prove it can fail: break it on purpose, watch it go red, revert.

## Decisions

A product or architecture decision gets an ADR the same day, even one made where the repo was
not visible. One file per decision, `docs/adr/NNNN-short-title.md`:

```markdown
# ADR-011 — Participation and share are independent
Status: Accepted                  (or: Superseded by ADR-0NN)
Date: 2026-08-01
Supersedes: ADR-004               (omit if none)

Context: what was known, and what forced the choice.
Decision: what was chosen.
Consequences: what follows. Dated lines may be appended later.
```

- Context and Decision are never edited. A change of mind is a new ADR that supersedes the old
  one, and both files say so (C6).
- Never invent a decision or its reasons. Unknown context is marked `[founder]` and left for them.
- An ADR explains why; it is not current policy. The rule itself goes in CLAUDE.md, a rule file
  or the PRD, amended in place.

## Work

One GitHub issue per piece of work. Labels: `feature`, `bug` or `chore`; `active` while being
worked; `needs-verification` when built but awaiting the founder's check; `founder-only` when
only the founder can do it (the body says where). A milestone names the build phase.

- The body starts with `Done when:` and statements someone can check. It is edited in place;
  discussion goes in comments.
- A PR whose work is code-verifiable says `Closes #N`. One the founder must confirm says
  `Refs #N`, and the issue gets `needs-verification`. Never mark on-device work done from code.
- Before an issue closes, anything durable in it moves to a doc or an ADR, in the closing PR.
- Log every feature idea as an issue with no milestone, without being asked.
- Issues use GitHub's `#N`. Older IDs in titles (`BS-154`) stay searchable; mint no new ones.

## Writing

- One home per fact: link to it, never restate it. If git, an issue or the code can answer
  it, no doc states it.
- Absolute dates (`2026-07-15`), never "today" or "recently". Never turn an estimate into a date.
- When writing to the founder, an issue number carries its title: `#208 (Reopen negotiation)`.
- Diff before you delete: move what belongs elsewhere, then delete the rest.

## Workers

A session handed one issue (the `hand-off` skill) is a worker. It reads its issue and what the
issue touches, and starts. It needs a `Done when` line, stops and reports when the issue does
not match the code, and never changes labels, milestones or ADR numbers, so parallel workers
cannot collide.
