# FLOW — Closing

Shipped with FLOW as `.flow/CLOSING.md`. Read it in full before a pull request that closes or
drops an issue, before removing a feature, retiring an affordance or deleting a code path, and
before setting `Current Phase` (FLOW_SYSTEM › *Closing work*). Never modify it: like
FLOW_SYSTEM.md, it changes only by adopting a FLOW release (C10). FLOW_SYSTEM.md, which you have
already read, holds every rule not stated here; a `§` reference below names a section of this
file or of FLOW_SYSTEM.

---

## Closing an Issue — and why it is a test, not a chore

An issue closes when its work ships or is dropped. Before it does, **everything durable in it
moves to a truth file**, in the pull request that closes it.

> **Everything durable in that issue was already owed to a truth file by the trigger table. So
> an issue that cannot close without losing something is evidence that a commit skipped an
> obligation.**

That makes closing the cheapest audit in the system. Work through the issue's body and comments,
and each piece goes one of four ways:

| What it is | Where it goes |
|---|---|
| A decision that was made | DECISIONS, as an entry |
| A hazard that must never be re-hit | FIXES, under the area it guards |
| A description of what now exists | DESIGN or SCHEMA |
| Already true in one of those files | nowhere — **leave it in the issue, and resist copying it** |

The fourth row is as important as the others. Copying a fact that already has a home is how one
duplication becomes eighteen.

**Merging is shipping.** A pull request whose work is fully code-verifiable says `Closes #N`:
the merge closes the issue and the host links the pull request to it, so the citation is never
written by hand. Work that still needs the founder's real-world confirmation says `Refs #N`
instead and labels the issue `needs-verification`; it closes once the founder confirms
(FLOW_SYSTEM § Verification). A dropped issue closes as *not planned*, with a one-line comment
giving the reason.

## The retraction pass — absence claims that went stale

Docs are additive by default, and that is the largest source of rot: a feature ships and the record of its absence survives.

> **Shipping a feature and retracting its absence are one task, not two — and one commit, not two.**

The pass is **issue-scoped and lands in the pull request that closes the issue**, not at session end. Search for claims that this work is unbuilt, and correct them. Vocabulary:

```
not built · not yet · does not · doesn't · planned · placeholder ·
coming soon · pending · deferred · post-MVP · TBD · blocked on ·
will be · future · missing · disabled · stub
```

**Scope the search to truth files and STATUS.** This vocabulary is tense-blind: `deferred` and `post-MVP` are wrong in a spec and *correct* in a decision record describing a choice made in the past. Exempt the `Context`, `Decision` and title fields of DECISIONS.md entries — a decision to defer something is accurate history forever. `Consequences` is not exempt.

## The presence pass — presence claims that went false

The inverse defect, and the harder one: a feature stops shipping and every doc still says it exists. No commit announces this. A redesign retires the affordance a feature hung on, and the feature goes with it silently — nothing was deleted, so no trigger fires.

> **When you retire an affordance, entry point, or code path, ask what depended on it — then check what documented that.**

Anything orphaned this way is a decision, not a cleanup. Log it and surface it to the founder. **Never delete a scope line to match reality** — that ratifies an accidental loss as though it had been chosen.

---

## Truth check — at every phase transition

`flow_check` already covers links and section references, supersession and the inventory on
every pull request. The truth check is what it cannot decide — and it
shrinks every time a rule gains a check.

Before setting `Current Phase`:

1. **Retraction grep** across truth files and STATUS, per § The Retraction Pass. Verify each hit against the repo. Correct what is false; report what you cannot verify from code.
2. **Scope presence.** Every entry in MASTER § In Scope still exists in the code. A feature can stop shipping without any commit announcing it.
3. **Resolve every truth file.** Every path and code name each one cites — file, class, function, table, column, token — exists in the repo, and every link resolves. Correct what does not; report what you cannot check from code. This, not rereading prose, is what found drift in practice.
4. **Run `flow_check`** with no range and clear every finding, warnings included.

Nothing else re-reads the docs for accuracy. This is the only net that catches what no commit could have triggered.
