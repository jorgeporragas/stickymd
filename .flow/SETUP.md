# FLOW — Starting a project

Shipped with FLOW as `.flow/SETUP.md`. Read it in full when a project has no `docs/PRD.md`
yet, and at no other time. Never modify it. The rules in `.claude/rules/flow.md` apply
throughout.

---

## 1. Write the PRD

**Goal:** `docs/PRD.md`, and `docs/DESIGN.md` if the project has a UI.

- Ask questions, one at a time. Push back on vague answers: not "musicians" but "a bedroom
  producer who records chord progressions in their spare time".
- Write no code and create no other file.
- Work through the checklist below in order. Do not move on until each item is specific.
- Present the PRD for approval. Nothing else is created until the founder approves it.

### Checklist

- [ ] **User** — one specific person, not a demographic. What do they do today without this product?
- [ ] **Problem** — what specifically breaks without this product?
- [ ] **Core loop** — the minimum sequence that delivers value. Five steps or fewer. If it takes more, V1 is too wide.
- [ ] **Domain model** — entities and relationships. Get this right first; it propagates to the schema, the API, and the UI.
- [ ] **Stack** — propose options against the requirements. Never assume a technology. Get explicit approval.
- [ ] **In scope (V1)** — only what completes the core loop, written so each entry can be checked against the code.
- [ ] **Permanent vetoes** — things the product will *never* do. **Test: if shipping it would require deleting the line, it was never a veto — it is a deferred issue.**
- [ ] **Build phases** — each independently shippable where possible, and named. Definitions only; each becomes a milestone.
- [ ] **Deployment** — where it lives when it ships: beta distribution, production platform, release process.

## 2. Set up the repository

Once the PRD is approved, in one pull request:

- `CLAUDE.md`: imperatives only, the rules for writing code in this stack ("Riverpod for all
  state, no setState"). Stack *choices and reasons* stay in the PRD. Under 200 lines; rules for
  one area of the code go in `.claude/rules/<area>.md` with a `paths:` filter, so they load only
  when Claude touches those files.
- `docs/adr/`, with the first ADRs for the stack choices; `docs/FIXES.md` with its entry format
  at the top; `docs/SCHEMA.md` if there is a database.
- FLOW, installed with `flow_sync`, with `.claude/rules/` committed, not ignored; this
  project's paths in `.flow/config.toml`; the CI job from FLOW's `system/ci/github-actions.yml`.
  Prove each check can fail.
- Agent settings (`.claude/settings.json`) that deny pushing to the default branch, merging and
  deploying, and ask before anything that touches production or a CI workflow.
- Read `.flow/PATTERNS.md` before setting up CI.
- On GitHub: the labels `feature`, `bug`, `chore`, `active`, `needs-verification`,
  `founder-only`; a milestone per build phase, named as the PRD names it; an issue for each
  piece of work in the first phase; squash merging only; branches deleted on merge.

Present the pull request for review.
