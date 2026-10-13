# FLOW_SYSTEM

**Version 3.0.0**

You are reading this because you are about to work on a software project using the FLOW system. Read this file fully before doing anything else. It defines the files you will work with, what phase the project is in, how to behave, and what is expected of you at every stage.

Never modify this file, or the files in `.flow/` it tells you to read. They are portable across projects. If a rule here is wrong, the founder amends it deliberately and bumps the version — you do not edit it mid-session.

---

## What the Founder Owes You

The founder is responsible for exactly two things:

1. **Decisions.** Product, architecture, scope. You propose; they decide.
2. **Real-world verification.** Anything you cannot check from the repo — on-device behaviour, DNS, App Store state, visual checks on hardware.

Everything else is yours: reading the right files, keeping them true, flagging drift, updating the docs, committing. **Any rule that requires the founder to remember something is a rule that will fail.** If maintaining this system starts costing them more than the two items above, something is over-engineered — say so.

---

## The Files

```
project_root/
├── FLOW_SYSTEM.md     ← this file. Never modify it.
├── CLAUDE.md          ← engineering rules for this repo. Entry point.
├── MASTER.md          ← what the product is and why.
├── .flow/             ← FLOW's checks and their config, and the rules read on demand: SETUP, RECORDS, CLOSING, PATTERNS.
├── .githooks/         ← the pre-commit hook. See § The Checks.
└── docs/
    ├── STATUS.md      ← where the project is now. Short; read every session.
    ├── DECISIONS.md   ← why things are the way they are. Append-only.
    ├── DESIGN.md      ← the visual system. Required for any project with a UI.
    ├── components/    ← one file per shared component: the inventory.
    ├── SCHEMA.md      ← the database. Required for any project with a database.
    └── FIXES.md       ← hard-won fixes. Read on demand, before touching a listed area.
```

**Work is tracked in the repository host's issues, not in files** — see § Work Tracking.

DESIGN, `components/` and SCHEMA are conditional. No UI, no DESIGN. No database, no SCHEMA.
The visual system may be written as `DESIGN.html`, so its tokens render as swatches; keep it
plain HTML and CSS with no script, because you edit it like any other file.

**Only three files are read at the start of every session** — FLOW_SYSTEM, STATUS and
CLAUDE — plus the report `flow_check` prints. Everything else is read on demand, and that is what keeps the standing cost bounded
as the project grows. A file that has to be read every session is a file whose length is a
tax; a file read when it is relevant can afford to be thorough. An issue is read when it is
worked.

**This list is what FLOW owns, not the only files permitted in a repo.** A README, a runbook, a deployment log, a licence — all fine. They sit outside the scheme, need no header, and are governed only by one-home-per-fact: they may point at a FLOW file, but must not restate what one owns.

**Project state lives in these files and the project's issues, and nowhere else.** Claude
Code's own memory is machine-local, unreviewed and unchecked: use it for personal workflow
preferences, never for the project's state, rules or decisions. A fact about the project worth
keeping belongs in one of these files or an issue, where it is reviewed and checked.

### What each file owns

| File | Owns | Never contains |
|---|---|---|
| **CLAUDE.md** | Imperatives for writing code in this repo: folder structure, framework usage rules, backend constraints, coding standards, and the rules that setup, deploying and releasing must not break. Every line tells you to do or not do something. | Project context, stack inventory, current state, product rationale. **Procedures** — how to set up, run, deploy or release, step by step: they go in a skill or runbook read when that work happens. **Work state inside a rule** — see § An Imperative Never Encodes Work State. |
| **MASTER.md** | What the product is: problem, user, core loop, domain model, stack choices, phase definitions, permanent vetoes. Present tense. True right now. | Anything with a state. No backlog, no completion marks, no "not yet", no decision history. |
| **STATUS.md** | Current phase, current active step (present tense, capped — § Templates), environment and machine state. | **Work** — it lives in issues; a list of them here is a copy. Rules, specs, domain truth, design decisions. History — closed issues and git already say what happened (§ Derive, don't record). |
| **Issues** (the host's) | One issue per piece of work: its type, state, `Done when`, a note, and its scoping while unbuilt. | Rules, specs and decisions: they are owed to a truth file before the issue closes (§ Closing work). |
| **DECISIONS.md** | One entry per decision: context, decision, consequences, status. Immutable once written, except as `.flow/RECORDS.md` › *Decision record* permits. | Work tracking. A decision is not a task. |
| **DESIGN.md** | Tokens, typography, colour, architectural principles for the UI, the never-allowed list. Describes what **exists**. | Component entries — they live in `components/`. Planned components, planned themes, work status. |
| **components/** | One file per shared component: what it is, its values unless the code generates them, the one to three rules a caller must not get wrong. **Complete and authoritative**: a component with no file does not exist. | Why a visual decision was made (DECISIONS). A component that does not exist yet — that is an issue. |
| **SCHEMA.md** | Tables, columns, relationships, functions, policies, and the invariants behind them. Where the project generates a schema reference, the tables and columns are there, and SCHEMA keeps the invariants and their reasons. | A "what's missing" section. Missing things are issues. A copy of what a generated reference already lists. |
| **FIXES.md** | Non-obvious solutions that must never be undone, each naming the area it guards. | Anything that is not a fix. |

---

## The Lifetime Rule

This is the rule the rest of the system rests on.

> **MASTER, CLAUDE, DESIGN, SCHEMA and DECISIONS are truth files.** Every sentence must be true regardless of what work is or is not underway.
>
> **Only STATUS and issues may contain statements that expire.**

**Test:** *could this sentence become false without anyone editing it?* If yes, it belongs in STATUS or an issue.

**An issue may expire because its expiry is scheduled:** it closes when its work ships or is
dropped, so its scoping cannot outlive the work it describes. No file can earn that guarantee.

**The inverse also disqualifies.** A statement that can *never* become false is not STATUS's
either. STATUS is for what is true **now** and will change; permanent history is not "now", and
closed issues and git already hold it.

### An imperative never encodes work state

A rule that says *never build X* is indistinguishable from *X is not built yet* once X ships — and it is far more dangerous than a stale description, because a session that obeys it will refuse to touch working code, or tear it out as non-compliant.

> **Rules constrain how work is done. Whether work is done is an issue's business.**

Write `Read every colour through context.colors; never declare a static colour`, not `light mode is planned — do not add a brightness branch`. If you cannot phrase a rule without referring to what is or is not built, it is not a rule. It is a status line in the wrong file.

The same applies to headings. `(PLANNED)`, `(SHIPPED)`, `(ACTIVE)`, and checkbox syntax in a truth file are all work state wearing a spec's clothes.

### Truth files change — by decision, not by work

The lifetime rule does not mean truth files are frozen. It means they change for a different reason.

> **A truth file changes when a decision changes it. STATUS and issues change when work changes them.**
>
> **Test:** did this edit follow a conversation, or a commit? Conversation → truth file. Commit → STATUS or an issue.

When a decision amends product truth:

1. **Record it in DECISIONS.md** — including what it supersedes.
2. **Amend the truth file in place, the same day the decision is made** — not when the code ships. Between decision and shipping, an unamended file makes you build against a rule that no longer holds.
3. **Open issues** for the work the amendment implies.

**Amend, do not annotate.** The amended file reads as though the new rule were always true. No "previously X was not allowed", no strikethrough, no dated aside. The old truth is gone from the truth file; its history lives in DECISIONS.md. A truth file that accumulates its own history stops being readable as present-tense truth and becomes a journal.

---

## Documentation Is Part of Done

**A change is not finished until the docs it invalidates are corrected. Both land in the same commit.**

Documentation is not a phase at the end of a session. Session-end updates fail three ways: they ask for maximum recall at the moment of least context; deferred work gets compressed; and when a session ends abruptly — context limit, crash, laptop closed — every doc obligation from that session is lost silently. Nobody finds out for weeks.

So the unit of documentation is the **commit**, not the session. This also makes obligations auditable: git history shows whether a commit met them.

### The trigger table

| You did this | You owe, in the same commit |
|---|---|
| Created or extracted a shared component | Its file in `components/` |
| Created a token | DESIGN |
| Changed a token's value, or split one token into two | DESIGN |
| Added or edited a migration | SCHEMA |
| Changed an RLS policy or a database function | SCHEMA, and FIXES if the reasoning was non-obvious |
| Changed the source of a generated doc | The regenerated doc — see § Derive, don't record |
| Opened a pull request that closes an issue, or dropped one | The closing pass for it — see § Closing work |
| **Removed a feature, retired an affordance, or deleted a code path** | **Every doc that describes it as existing — see CLOSING › *The presence pass*** |
| Found a bug | An issue, labelled `bug` |
| Solved something non-obvious, or got it right after more than one failed attempt | FIXES |
| Added a dependency, or established an architectural pattern | CLAUDE |
| Changed deployment, environment handling, or build setup | CLAUDE |
| Made a product or architecture decision | DECISIONS, plus amend the affected truth file — same day, not necessarily same commit |
| Made a decision where the repo was not visible — a chat, a call, a phone | DECISIONS, the same day, written by the next session that can see the repo |

For anything this table misses: **if you had to learn something to make the change, some doc owes an entry.**

### Closing work

**Read `.flow/CLOSING.md` in full before** you open a pull request that closes or drops an issue,
before you remove a feature, retire an affordance or delete a code path, and before you set
`Current Phase`. It holds the closing pass, the retraction and presence passes, and the truth
check. What they rest on:

- **Everything durable in an issue moves to a truth file before the issue closes**, in the
  pull request that closes it. An issue that cannot close without losing something is evidence
  that a commit skipped an obligation.
- **Merging is shipping.** Code-verifiable work says `Closes #N`; work the founder must confirm
  says `Refs #N` and is labelled `needs-verification`.
- **Shipping a feature and retracting its absence are one task, in one pull request.**
- **When you retire an affordance, ask what depended on it.** An orphaned feature is the
  founder's decision; never delete a scope line to match reality.

---

## The Checks

**A rule without a check is a suggestion.** In the first project to run FLOW, every rule a
machine checked held and every rule left to memory broke. So every rule a script can decide is
decided by `flow_check` (`.flow/flow_check.py`, shipped with FLOW); what is left for you is
judgement.

| Where | Command | Can it be skipped? |
|---|---|---|
| Pre-commit hook | `python .flow/flow_check.py --staged --only C8` | Yes: `--no-verify`, or a clone without the hook. An early warning. |
| CI, every pull request | `python .flow/flow_check.py --range origin/main..HEAD` | No. This is the check that counts. |
| Session start | `python .flow/flow_check.py` | It reports; it does not block. |
| For the founder | `python .flow/flow_dashboard.py` | Writes `.flow/dashboard.html` from the same report: generated, never edited, kept out of git. |

The hook runs only the commit-level checks, so an old finding elsewhere never blocks an
unrelated commit; CI runs everything, with the full history (`fetch-depth: 0`) the date and
commit checks need. The hook is enabled per clone with `git config core.hooksPath .githooks`;
reconciliation verifies it. Commit its executable bit (`git update-index --chmod=+x`) and pin `.githooks/` to
LF in `.gitattributes`, or it silently fails to run on some machines.

**What it checks** is defined exactly in `.flow/SPEC.md`: entry sizes (Current Active Step
included), links and section references, supersession, inventory
completeness, the commit-level obligations (a migration without SCHEMA, and the project's other
co-change rules), and that FLOW's own files are unmodified. Issues are the host's, and the host
keeps their state and history.

**Code checks are the project's.** When a rule in CLAUDE.md can be tested — never show a raw
error, every icon goes through one table — write it as a test beside the code and run it in CI,
so it stops depending on memory. `flow_check` covers the docs; guard tests cover the code.

**The numbers belong to the project.** Limits live in `.flow/config.toml`, and every value that
differs from FLOW's default appears in the report. **Never raise a limit, switch a check off
or bypass CI to get green** without the founder saying so in the pull request.

**Size limits fail only on what a pull request touched.** Old oversized entries stay warnings
until someone edits them; editing one means bringing it under its limit.

### Prove the check can fail

A check that cannot fail is not a check. When you add or change one — a limit, a guard test, a
CI step — prove it before trusting it:

1. Make a deliberate, throwaway violation.
2. Run the check. It must fail and name the violation.
3. Revert and run it again. It must pass.
4. Say in the pull request that the check was proven.

A check that fires on changes that owe nothing is wrong: fix what it measures, or delete it.

---

## Writing Rules

### Dates

- **Use dates, never relative time.** Never "this session", "last session", "today", "recently". A doc outlives the session that wrote it, so a relative reference is wrong the moment it is committed. Write `Shipped 2026-07-15`. State descriptions ("currently OFF") are fine — it is *time* references that rot.
- **Date each entry by when its work landed, from its own commit** (`git log --date=short`), not by when the session opened. A session can span days; batch-stamping hides real gaps.
- **Never convert an estimate into a date or a target.** "About a month away" is not a commitment. If the timing is worth keeping, record it as what it was — attributed and dated: *"founder estimated ~1 month out as of 2026-06-20"*.
- **Date the past, never manufacture the future.**
- **A state without a date is an incomplete entry.**

### Naming

- **Cite phases by name, never by number.** Phase numbers are doc-local: MASTER's Phase 1 and STATUS's Phase 1 routinely describe different work, and a bare number silently points at the wrong one. Write `Phase 1 — Foundation & UI Shell`.
- **A cross-reference names an issue, an ID or a section, or it is a rumour.** "Tracked separately" without an issue number means nothing is tracked.
- **When you write to the founder — chat, a pull request — an issue number carries its title:** `#208 (Reopen negotiation)`. In the docs, the bare number is enough.

### One home per fact

- Every fact has exactly one authoritative file. Other files **link**; they never restate.
- **A link is not duplication. Restatement is.** Two files may legitimately need the same fact for different purposes — the second one points at the first. There is no exception for "legitimate dual purpose"; that is how one duplication becomes eighteen.
- **When a value must cross a language or system boundary** and cannot be imported, both copies carry reciprocal comments naming the other. A third copy is a bug.
- **Flag duplication** when you notice it or are about to create it. Do not silently copy-paste.
- **A fact in the wrong file has no maintainer, so it does not merely risk drift — it
  guarantees it.** The trigger table routes each kind of change to one file. A visual spec
  kept in the engineering rules is never reached by a design change; a schema detail in the
  product doc is never reached by a migration. The copy is not updated because nobody thinks
  of that file as the place where this kind of thing lives. **When you find the same fact in
  two files, ask which one a change would actually reach — that is the home.**

### Derive, don't record

**If git, a tag or the build can answer it, no doc states it.** Deploys, versions, what shipped
when, who changed a file and when — these are computed, so a written copy is a second copy that
drifts. STATUS says what is true now and what blocks it; `git log`, tags and closed issues say
what happened. **The same goes for work:** an issue's history, state and the pull request that
closed it are the host's record, so no doc lists them.

**And for what the code can generate.** A schema reference from the migrations, token values
from the token source, a component's props from its code: where the project can generate a
fact, it does, and no one writes it by hand. A generated doc opens with the command that
produced it, is never edited, and is regenerated in the commit that changes its source; CI
regenerates it and fails when the result differs (`.flow/PATTERNS.md`). It is not a truth file
— it cannot be false, only stale, and staleness is CI's to catch. The hand-written file keeps
what generation cannot know: the invariants, the reasons behind them, and the rules a caller
must not get wrong — and links to the generated one for the rest.

### Constraints and preferences

A rule saying *never do X* is either a **constraint** — undoing it breaks something — or a
**preference** — someone tried both and chose. On the page they look identical. They are not.

> **A constraint binds because reality binds. A preference binds only until the founder
> changes their mind, which they may do at any time, for any reason, without justifying it.**

Write a constraint with its force, and say what breaks. Write a preference as *what was chosen
and when*, and leave the door open.

**Never stamp a preference `SETTLED`, `FROZEN` or `do not reopen`.** The stamp does not prevent
the change — it prevents the *record* from being updated when the change happens, because
nobody re-reads a section marked settled. A reversal five days later can then survive in the
spec for months.

**Test: who is this protecting against?** If a future session could break it without knowing,
write it down. If the only person who would change it is the founder, and they are allowed to,
it is not a guard. It is history, and history goes in DECISIONS.

### Deleting

- **Diff before you delete.** Before removing any section — including one this file bans outright — compare its contents against where that content is supposed to live. Move what belongs; delete only the remainder. A banned section is often a symptom of a gap elsewhere, and obeying the ban literally can destroy the only copy of something real.
- **Report what you found before deleting a file.** Same reasoning, larger blast radius.

### Verification

**What you change, you resolve.** When you edit a truth file, every path, code name and link
in the sections you touched must still exist in the repo. That is the check that catches drift:
a stale name is a false sentence that does not look false. The whole file is resolved at the
truth check.

- **Never mark on-device work done from code alone.** You can see what the code does. You cannot see what the founder verified on hardware, in TestFlight, in a DNS panel, or in App Store Connect.
- **Close freely only what is code-verifiable** — tests, analyzer, build, migrations present.
- Everything else stays open, labelled `needs-verification`, until the founder confirms it. Session start lists it.
- **A confirmation is one comment on the issue, which then closes:** `Verified on device
  (founder)`. The host dates it. How it was tested belongs in the pull request thread. It enters
  a doc only as a hazard (FIXES) or a decision (DECISIONS).
- **Flagged is not fixed.** A finding you reported but did not action remains open. Open an issue for it.

---

## Work Tracking

### Issue

One issue per piece of work, in one tracker — not separate lists for ideas, features and bugs,
which drift apart. Its body, edited in place so it reads as current:

```markdown
Done when: either side can reopen a finalized split; the other side sees it on next load.

What it is and what someone needs to start, in one paragraph.

## Scoping
⚠️ <any constraint that must be read before building>
Options, none chosen:
  - <option> — <what it costs>
Open: <the questions that must be answered first>
```

| Field | Where it lives |
|---|---|
| Type | A label: `feature`, `bug` or `chore` |
| State | `idea`: open with no milestone · `accepted`: open, in the milestone named for its phase · `active`: labelled `active` · `shipped`: closed as completed by its pull request · `dropped`: closed as not planned, with a reason |
| Awaiting the founder | Labelled `needs-verification` (§ Verification) |
| Only the founder can do it | Labelled `founder-only`, and the body names where the work happens: a dashboard, a DNS panel, a device |
| History | The issue's timeline. Never written by hand |

Rules:

- **An issue is never copied.** A list of issues in a doc is a copy, and goes stale the first
  time one changes. Query the host instead.
- **`Done when` is what makes the issue finished, as statements someone can check.** It is
  required before the issue is handed to another session, and the pull request is reviewed
  against it.
- **Scoping lives in the issue while the work is unbuilt.** Decisions it reaches go to DECISIONS
  when they are made, not when the issue closes.
- **Discussion goes in comments; the body stays current.** A confirmation, a dropped reason, a
  finding from review: comments, which the host dates.
- **Issue IDs from before 3.0** (`BS-208`) stay valid where the docs cite them. A migrated
  issue's title opens with the old ID, so a search finds it.

---

## Templates

The shape of every record FLOW keeps is in `.flow/RECORDS.md`. **Read it before you write** a
truth file's header, STATUS › Current Active Step, a component file, a DECISIONS entry, or the
first entry of a file that accumulates them. Two of its rules bind everywhere:

- **DECISIONS is append-only.** Context and Decision are never edited once written; supersession
  pointers are reciprocal (C6).
- **Current Active Step is present tense and capped** (C3): what is being worked on, what blocks
  it, and what waits on the founder.

---

## Phase Detection

| State | Phase |
|---|---|
| Only FLOW's own files exist | **Planning** |
| MASTER.md exists, no CLAUDE.md | **Initialization** |
| STATUS.md `Current Phase` = Build | **Build** |
| STATUS.md `Current Phase` = Live | **Live** |

STATUS.md `## Current Phase` is always authoritative. If files exist but STATUS.md is missing or ambiguous, ask.

---

## Phases

### Planning and Initialization

A new project's first two phases are defined in `.flow/SETUP.md`. Read it in full when Phase
Detection finds either one; no other phase needs it.

### Build

**Goal:** implement the current phase as defined in MASTER.md.

- One task at a time. Finish it — code **and** its doc obligations — before starting the next.
- Read the relevant files before proposing changes to existing code.
- **Before building any UI, look in `components/`.** Reuse is only the default if you have looked.
- Confirm schema changes explicitly before applying them.
- Flag scope creep. Name it, and ask whether to proceed or open an issue for it.

**Done when:** every issue in the phase's milestone is closed. Run the truth check (`.flow/CLOSING.md`), then set `Current Phase` to `Live`.

### Live

**Goal:** maintain and evolve the shipped product. Bugs take priority over features.

- Check the open `bug` issues at session start. Address them first.
- With no bugs open, surface open issues with no milestone (ideas) or in the current milestone.
- When a feature is approved, add its phase definition to MASTER, put its issue in that phase's milestone, and set `Current Phase` back to `Build` for that cycle.
- Keep FIXES.md current. A live product accumulates fixes faster than one in development.

**Done when:** never. Live is the permanent state of a shipped product.

---

## Session Contract

### Start of every session

**A project may encode these steps as a session-start skill. If it does, run the skill.**

1. Detect the phase from STATUS.md.
2. Read, in order:
   - `docs/STATUS.md` — where the project is now
   - `CLAUDE.md` — engineering rules
   - The issue being worked, body and comments; the open `needs-verification` issues are the confirmation list
   - `docs/DESIGN.md` — if any UI work is planned, and `components/` for each component you will touch
   - `docs/SCHEMA.md` — if any database work is planned
   - `docs/FIXES.md` — the area it guards, before touching that area
   - `MASTER.md` — if implementing a new feature or starting a new phase
   - `docs/DECISIONS.md` — only when a past decision is in question. Not routine reading.
3. **Reconcile.** Run `flow_check --range <the commit that last changed STATUS>..HEAD` and report what it finds. Check the same commits against the trigger table for what it cannot judge. Verify `core.hooksPath` is set on this machine. Report gaps **before doing any work**, then close them.
4. State the phase and summarize the project in 2–3 sentences.
5. Wait for confirmation. If the summary is corrected, update your understanding before proceeding. Do not write a line of code on stale context.

There is no end-of-session step: every obligation is met in the commit or pull request that
incurs it.

### Handing an issue to another session

A project that runs one session at a time never needs this. When the founder or a planning
session hands an issue to another session to implement, that session is a **worker**:

- **The issue is its confirmation.** It reads FLOW_SYSTEM, CLAUDE and its issue, then what the
  issue touches from the reading order above, and starts. It skips STATUS, reconciliation and
  the summary.
- **It needs a `Done when` line.** Given an issue without one, it asks instead of guessing.
- **It stops when the issue does not match the code** — a file, table or approach the issue
  assumes is not there — and reports it, changing nothing. The issue goes back to whoever
  planned it, as does any decision the work needs.
- **It never mints a decision ID, never edits STATUS, and never changes an issue's labels or
  milestone**, so parallel workers cannot collide on them. Its pull request says `Closes #N`
  or `Refs #N`; the host numbers issues, so nothing else needs coordinating.

---

## Behavior Contract

- **Direct.** No filler. No excessive qualification.
- **One task at a time**, and a task includes its doc obligations.
- **Read before touching.** Always read the relevant files before proposing changes to existing code.
- **Ask, don't assume.** Any architectural, product, or scope decision — ask. One question at a time.
- **Push back.** If an approach has a problem, say so. Do not implement something you know is wrong.
- **Complete work.** No TODOs. No placeholders. No half-implementations.
- **Protect FIXES.** Never undo a documented fix. If a change would affect one, flag it first.
- **Log every idea.** If a feature idea surfaces in any session, open an issue for it with no milestone, without being asked.
- **Changes reach the default branch by pull request, and the founder merges.** Work on a branch, push it, open the pull request, and stop. Never push to the default branch, force push, merge, or deploy: the merge is the founder's sign-off, which is what lets `shipped` mean something. Where the host cannot enforce this — branch rules on a private repository need a paid plan — it rests on your settings and on CI going red when it should.
- **Database changes need confirmation.** State the change explicitly; wait for approval.
- **Scope creep gets flagged, not ignored.** Name it. Ask whether to proceed or log it.
