# FLOW — Setup

Shipped with FLOW as `.flow/SETUP.md`, and read only in the Planning and Initialization phases
(FLOW_SYSTEM › *Phase Detection*). Never modify it. Every other rule is in FLOW_SYSTEM.md, which
you have already read, or in the `.flow/` files it names.

---

## Phases

### Planning

**Goal:** produce MASTER.md, and DESIGN.md if the project has a UI.

- Ask questions. One at a time.
- Push back on vague answers. Do not accept "musicians" when you need "a bedroom producer who records chord progressions in their spare time."
- Write no code. Generate no project files.
- Work the Planning Checklist in order. Do not advance until each item is specific.
- Draft MASTER.md and present it for approval. Generate nothing else until it is approved.

**Done when:** the founder approves MASTER.md.

### Initialization

**Goal:** generate CLAUDE.md, docs/STATUS.md, docs/DECISIONS.md, docs/FIXES.md, and docs/SCHEMA.md if applicable; set up the project's issues; install FLOW's checks.

- Read MASTER.md fully first.
- Generate every required file in one pass, each truth file with its header block.
- CLAUDE.md must contain a "Before Every Session" block. It states **what is specific to this repo** — which files are needed more often than the default, and that FIXES is read by area before touching one. It does **not** restate the reading order or list the areas: FLOW_SYSTEM owns the order, and FIXES owns its own index.
- **CLAUDE.md holds imperatives only.** Stack *decisions and rationale* live in MASTER; stack *usage rules* ("Riverpod for all state, no setState") live in CLAUDE.
- Install FLOW with `flow_sync` (it writes `.flow/` and the hook), set this project's paths in `.flow/config.toml`, and add the CI step from FLOW_SYSTEM › *The Checks*. Prove each check can fail.
- Create the labels FLOW_SYSTEM › *Issue* names (`feature`, `bug`, `chore`, `active`, `needs-verification`, `founder-only`) and a milestone for each build phase, named as MASTER names it. Open an issue for each piece of work in the first phase.
- Commit agent settings (`.claude/settings.json`) that deny pushing to the default branch, merging and deploying, and ask before anything that touches production or a CI workflow. Opening, labelling and commenting on issues is routine and needs no ask; closing one is the merge's job, or the founder's confirmation. Where the host cannot enforce FLOW_SYSTEM › *Behavior Contract*'s pull-request rule, these do.
- Read `.flow/PATTERNS.md` before setting up CI.
- Present for review; correct anything flagged.

**Done when:** all files exist and are reviewed. Set `Current Phase` to `Build`.

---

## Planning Checklist

Work through these in order. Do not advance until the current item is answered with specifics.

- [ ] **User** — one specific person, not a demographic. What do they do today without this product?
- [ ] **Problem** — what specifically breaks without this product?
- [ ] **Core loop** — the minimum sequence that delivers value. Five steps or fewer. If it takes more, V1 is too wide.
- [ ] **Domain model** — entities and relationships. Get this right first; it propagates to the schema, the API, and the UI.
- [ ] **Stack** — propose options against the requirements. Never assume a technology. Get explicit approval.
- [ ] **In scope (V1)** — only what completes the core loop. This list is checked against the code at every phase transition, so write entries that can be verified.
- [ ] **Permanent vetoes** — things the product will *never* do. **Test: if shipping it would require deleting the line, it was never a veto — it is a deferred item.**
- [ ] **Build phases** — each independently shippable where possible. Named, not just numbered. Definitions only; completion state lives in the issues, one milestone per phase.
- [ ] **Deployment** — where it lives when it ships. Beta distribution, production platform, release process.

When every item is answered and MASTER.md is approved, move to Initialization.
