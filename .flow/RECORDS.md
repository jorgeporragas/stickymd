# FLOW — Records

Shipped with FLOW as `.flow/RECORDS.md`. Read it before writing a truth file's header, STATUS ›
*Current Active Step*, a component file, a DECISIONS entry, or the first entry of a file that
accumulates them (FLOW_SYSTEM › *Templates*). Never modify it: like FLOW_SYSTEM.md, it changes
only by adopting a FLOW release (C10). FLOW_SYSTEM.md, which you have already read, holds every
rule not stated here; a `§` reference below names a section of this file or of FLOW_SYSTEM.

---

## Truth file header

Every truth file opens with this block.

```markdown
> **Authoritative for:** <what this file owns>
> **Never contains:** <what belongs elsewhere, and the file it lives in, by name>
```

**No date goes in the header.** Git already says when a file last changed (`git log -1 --
<file>`). A "verified" stamp would claim someone checked the file against reality, but writing
one is a free keystroke, and a file can read `verified` while being months wrong — worse than
no stamp at all.

## Current Active Step

The section of STATUS read first in every session. **Present tense**: what is being worked on,
what blocks it, and what waits on the founder — at most 250 words by default. What shipped is in
closed issues; what was deployed is in git. It changes in the pull request whose work changes it,
unless that is a worker's: then the session that handed over the issue updates it.

## Component

One file per shared component, `docs/components/GlassCard.md`, named for the component.

```markdown
# GlassCard
One line on what it is.

| Value | Token |
|---|---|
| Radius | `kCardRadius` |

Rules:
- The one to three things a caller must not get wrong.
```

At most 400 words by default; `flow_check` measures each file. *Why* a visual decision was
made belongs in DECISIONS.

## Entry format — every accumulating file defines one

A file with no defined entry shape grows without resistance: each session writes whatever seems
relevant at that moment and nothing pushes back. This is measurable — the files with a template
barely change; the ones without change as often as STATUS.

So **every file that accumulates entries states its own format at the top**: what an entry opens
with, what belongs in the body, and what disqualifies an entry entirely. Two rules generalise:

- **Open with the claim, not a title.** *"Never downgrade X below 7.x"* is testable — you can
  tell whether you are about to violate it. *"Dependency notes"* is not.
- **Say what an entry must NOT contain**, or the file becomes where everything lands.

## Decision record

Append-only. Entries are immutable once written, with two exceptions named below.

```markdown
## ADR-011 — 0% participants may join and sign Master splits
Status:     Accepted          (Accepted | Superseded by ADR-0NN)
Date:       2026-08-01
Supersedes: ADR-004
Context:    Label reps and engineers need to sign without holding points.
Decision:   Participation and share allocation are independent.
Consequences: can_finalize_composition() must not require share > 0.
            2026-08-01 — MASTER § Domain Model amended.
```

Rules:

- **Context and Decision are immutable.** They record what was known and what was chosen *at the time*. Never edit them, and never write them with hindsight — a superseded ADR must not read as though it anticipated its own reversal, or the chain stops being legible.
- **Status may change**, and only to record supersession.
- **Consequences is append-only.** What follows a decision keeps happening. Appending a dated line — most often recording which truth file an amendment touched — is permitted. Editing or removing an existing line is not.
- **Supersession pointers are reciprocal.** Every `Superseded by ADR-0NN` has a matching `Supersedes: ADR-0NN`. A one-way pointer means the chain disagrees with itself, and it will point somewhere plausible and wrong.
- **Fulfilled is not superseded.** A deferral that was later executed as written was *honoured*, not reversed — the decision held, and its condition came due. Mark it superseded only if the position actually changed. Recording "we did it later, as planned" as "we changed our minds" destroys the distinction between a plan and a reversal.
- **Nothing may cite DECISIONS.md as current policy.** Current policy lives in MASTER's vetoes or in an issue's state. A decision record explains *why* the truth is what it is; it is not itself the truth.
- **Never fabricate a decision.** If the record shows only that something remains open, no ADR exists yet. "We decided this is next to decide" is not a decision.
- **Context is the founder's.** Where rationale is not recoverable from the repo, mark it `[founder]` and leave it empty. Do not reconstruct it.
