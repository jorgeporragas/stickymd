# flow_check — specification

**Status:** implemented in `flow_check.py` 0.2.0 (#3, D-007; C1, C2 and C9 retired by D-016,
C4 by D-017). Shipped to projects as `.flow/SPEC.md`.

`flow_check` measures whether a project's FLOW docs follow FLOW's rules. It is the machine
check for the rules that broke in Banana Splits because nothing enforced them
(`reports/2026-10-10-banana-splits.md` §3). One run produces one JSON report, which CI, the
session-start skill and the dashboard all read (D-004). Rules belong to FLOW; numbers belong to
the project (D-005).

## Scope

**In:** checks a script can decide with no judgement: counts, dates against git, references
that resolve or do not, pointers that agree or do not, files changed together or not.

**Out:**
- Anything needing judgement: whether a note *should* be in FIXES, whether a sentence is true.
  Those stay with the trigger table and the truth check.
- The retraction grep. Its vocabulary is tense-blind, so every hit needs a human or agent to
  judge it (FLOW § The retraction pass). It may come back later as a listing, never as a failure.
- The dashboard (#8). It renders this tool's output and is built separately.

## Runtime

- **Python 3.11 or later, standard library only.** 3.11 is the first version with `tomllib`,
  which reads the config file. The founder's machine runs 3.14; GitHub's runners ship 3.12.
- **Reads the working tree and runs `git`.** No network, no writes, no dependencies.
- **Deterministic.** The same tree and history always give byte-identical output: no
  timestamps, stable ordering. The dashboard adds its own time.

## Interface

```
python .flow/flow_check.py [--root DIR] [--config FILE] [--range REV_RANGE | --staged] [--json] [--only ID,...]
```

| Option | Default | Meaning |
|---|---|---|
| `--root` | `.` | Project root |
| `--config` | `.flow/config.toml` | Project configuration; every key has a default |
| `--range` | none | Also run the commit checks (C8) over each commit in the range, e.g. `origin/main..HEAD` |
| `--staged` | off | Run the commit checks (C8) on the staged diff instead: the pre-commit hook |
| `--json` | off | JSON report on stdout instead of the human summary |
| `--only` | all | Run only the named checks |

| Exit code | Meaning |
|---|---|
| 0 | No finding at `error` severity |
| 1 | At least one finding at `error` severity |
| 2 | The tool could not run: bad config, missing file it was told to read, not a git repo |

Exit 2 is never confused with a failing project: a broken check must not read as a clean one.

## Definitions

**Word.** A maximal run of non-whitespace characters, as `str.split()` with no argument: the
same count as `wc -w` in a UTF-8 locale (`LC_ALL=C.UTF-8 wc -w`; the C locale skips
tokens made only of non-ASCII characters, such as `—`). Markdown syntax, code spans and links count as written. Chosen because
anyone can reproduce it with one command.

**Entry.** A heading plus everything up to the next heading of the same or higher level.

**Severity.** Each check is `error`, `warn` or `off` in config. `warn` findings are reported and
never fail the run.

## Checks

Each check names the FLOW rule it enforces. Every check ships with a fixture that passes and a
fixture that fails, and its test proves both: a check that cannot fail is not a check.

### Retired: C1, C2, C4, C9

Retired in 0.2.0 (FLOW 3.0). When work moved to the host's issues (D-016): **C1** item notes,
**C2** completed lines, **C9** item integrity. When truth files stopped carrying a `Last
verified` stamp (D-017): **C4** stale stamps. Their IDs are never reused. A config that sets
their severity, or `--only` naming one, is a tool error (exit 2) that says so, rather than a
setting that silently does nothing.

### C3 — Entry size in accumulating files
**Rule:** FLOW § Entry format, every accumulating file defines its entry shape.
**Measures:** entries in each configured file, at a configured heading level.
**Fails when:** an entry exceeds its configured `max_words`.
**Built in:** STATUS's Current Active Step (`current_step.max_words`, default 250), and every
file in `files.components_dir` as one entry (`component_files.max_words`, default 400) — in
separate files an oversized entry looks normal, so each file carries its own limit (D-008).
**Configured:** projects list their other files, e.g. FIXES or DECISIONS.

### C5 — References resolve
**Rule:** CLOSING § Truth check, every cross-reference resolves, including to sections.
**Measures two forms:**
1. Markdown links to repo files, `[text](path)` and `[text](path#anchor)`. The path must exist;
   the anchor must match a heading under GitHub's slug rules.
2. **Section references**, `FILE › *Heading*`, including chains (`FILE › *A* › *B*`) and
   linked file names. `FILE` is one of the FLOW files named in config. The heading must exist
   in that file after normalisation: Markdown emphasis and code marks stripped, whitespace
   collapsed, case ignored. A reference may omit a trailing qualifier (`*Theme colour*` finds
   `Theme colour (the rule)`). A **bold rule name** opening a list item or paragraph counts as a
   target, because FLOW files name rules that way and refer to them like headings.

**Why form 2 matters:** Banana Splits has 216 section references and 94 Markdown links. Section
references break silently when a heading moves, because nothing renders them.
**Fails when:** a reference does not resolve.
**Scope:** Markdown files, plus code comments in configured globs. DECISIONS is exempt by
default: its entries describe the past and are immutable, so a reference there may name a
section that has since gone, accurately.
**Banana Splits:** 216 section references and 94 links, 0 findings (3 before the bold-name and
DECISIONS rules; all 3 were false). C5 therefore defaults to `error`.

### C6 — Supersession is reciprocal
**Rule:** FLOW § Decision record, supersession pointers are reciprocal.
**Measures:** for every `Superseded by X` in an entry's `Status`, entry X exists and its
`Supersedes:` names the first entry, and the reverse. A `Supersedes:` line starting with `—`
records an amendment, not a supersession, and is skipped.
**Port of:** Banana Splits' `test/decisions_pointers_test.dart`.

### C7 — Inventory is complete
**Rule:** FLOW § Build, the component inventory is complete and authoritative.
**Measures:** for each configured pair `{dir, glob, doc}`, every matching file is named in the
doc — by file name, stem, stem in PascalCase (`action_popover` → `ActionPopover`), or stem as
words (`Action popover`, case ignored). `doc` may be a directory (v2.6 `components/`): its file
names and contents together are the inventory.
**Fails when:** a file is not named, or the directory matches no files at all — a check that
scans nothing passes forever.
**Not ported:** Banana Splits' test also accepts any public symbol a file declares. That needs a
language parser; a project that wants it keeps its own test.
**Banana Splits:** 0 findings over `lib/core/widgets` (8 before the PascalCase and words
spellings; all 8 were false).

### C8 — Commit checks (`--range` only)
**Rule:** FLOW § Documentation Is Part of Done, a change and its docs land in the same commit.
**Runs:** over every commit in the range, judging each commit's own diff, which is what the
pre-commit hook does for one staged diff.
**Rules, from config:**
- **Co-change:** when a commit adds (or touches) a path matching `when`, it also changes
  `require`. Banana Splits' hook checks 1 and 2 become two entries:
  migrations → SCHEMA, and a new file in the shared component directories → DESIGN.

Until 0.2.0, C8 also required a shipped item to cite its pull request and its SCOPING section
to be deleted. Issues closed by a pull request carry both by construction (D-016).

**Modes:** `--range` judges each commit in the range; `--staged` judges the staged diff as one
change, which is what the pre-commit hook runs (`--staged --only C8`).
**Why:** the hook only runs where it is enabled, and `--no-verify` skips it. CI running C8 is
the version nobody can skip (Banana Splits BS-238).
**Banana Splits, last 200 commits (0.1.0):** 3 findings, all real — BS-161, BS-077 and BS-076
collapsed without a citation, and each later cost a separate fix-up commit. That failure cannot
happen when the merge closes the issue.

### C10 — FLOW's files are unmodified
**Rule:** FLOW_SYSTEM, "Never modify this file", and every file `flow_sync` installs.
**Measures:** each file listed in `.flow/manifest.json` against its recorded SHA-256, line
endings normalised so a Windows checkout matches.
**Fails when:** a listed file is missing or differs. Projects without a manifest skip it.

## Existing violations

A project adopting `flow_check` already has violations: Banana Splits had 20 oversized notes,
3 stale stamps, and unknown reference breakage. Failing CI on all of them on day one would make
adoption a cleanup project.

**Decided:** size checks (C3; C1 and C2 until 0.2.0) under `--range` fail only on entries the range touched. CI on
a pull request then blocks new and edited oversized entries; untouched old ones are reported as
`warn`. The debt shrinks as entries get edited, and nothing new joins it. All other checks fail
regardless.

**A move is not an edit.** An entry in a file that did not exist at the range's base counts as
untouched when the text the check measures (a component file's body) appears
word for word in a Markdown file at the base, ignoring whitespace and link targets. Moving
components into `docs/components/` therefore keeps old oversized entries as warnings; editing
one while moving it does not (#24).

## Metrics

Reported in the JSON, never failing a run:

| Metric | Definition |
|---|---|
| `reading_cost.session_start` | Words in the files configured as read at every session start |
| `reading_cost.with_optional` | The same, plus files configured as read in most sessions |
| `docs_only_share` | Share of commits in the last `metrics.days` days (default 30) that touched only doc paths |
| Per check | Findings count, pass/fail |
| `overrides` | Every config value that differs from FLOW's default, with both values. Filling a list FLOW leaves empty (paths, rules) is configuration, not an override |
| `phase` | STATUS › *Current Phase* |

Open work is the host's to count: the dashboard and `/flow-start` read it from the issues, and
flow_check stays offline.

## Output

```json
{
  "tool": "flow_check",
  "tool_version": "0.1.0",
  "config": ".flow/config.toml",
  "range": "origin/main..HEAD",
  "result": "fail",
  "checks": [
    {
      "id": "C3",
      "name": "entry-size",
      "severity": "error",
      "status": "fail",
      "findings": [
        {
          "file": "docs/STATUS.md",
          "line": 5,
          "message": "entry 'Current Active Step' is 312 words; the limit is 250",
          "severity": "error",
          "measured": 312,
          "limit": 250
        }
      ]
    }
  ],
  "metrics": {
    "reading_cost": {"session_start": 30700, "with_optional": 67000},
    "docs_only_share": {"days": 30, "commits": 246, "docs_only": 94, "share": 0.38}
  }
}
```

Without `--json`, errors print one line each (`file:line: [C3] message`), so editors and
terminals can jump to the line, followed by a summary. Warnings are counted in the summary but
not listed, because the report is read at every session start and old warnings would dominate
it; `--all` lists them too. The JSON always carries every finding.

## Configuration

`.flow/config.toml`. Every key is optional; omitted keys take FLOW's defaults.

```toml
[files]                     # where the FLOW files live
status = "docs/STATUS.md"
decisions = "docs/DECISIONS.md"

[severity]                  # error | warn | off, per check
C3 = "error"

[current_step]
max_words = 250

[[entries]]                 # C3, one block per file
file = "docs/DESIGN.md"
heading_level = 3
under = "UI Components"     # only entries below this heading
max_words = 400

[references]                # C5
code_globs = ["lib/**/*.dart"]

[[inventory]]               # C7
dir = "lib/core/widgets"
glob = "*.dart"
doc = "docs/DESIGN.md"

[[co_change]]               # C8
when = "supabase/migrations/*.sql"
on = "any"                  # any | added
require = "docs/SCHEMA.md"

[reading_cost]
session_start = ["FLOW_SYSTEM.md", "docs/STATUS.md", "CLAUDE.md"]
optional = ["docs/DESIGN.md"]

[metrics]
days = 30
doc_paths = ["docs/**", "*.md"]
```

## Acceptance

- Every check has a passing and a failing fixture, and the test suite proves both.
- Running against a read-only snapshot of Banana Splits reproduced this spec's 0.1.0 numbers
  (20 notes over 150, 3 stale stamps), and the PR reported C5's real false-positive count before
  C5 was allowed to default to `error`. A new check meets the same bar.
- No third-party import anywhere; CI runs the suite on the oldest supported Python.

## Decisions

Settled with the founder on 2026-10-12 and recorded as D-007.

1. **Python 3.11 or later, TOML config.**
2. **Existing violations:** size checks block only the entries a pull request touches;
   everything else fails regardless.
3. **Default limits:** 150 words per note, 50 per completed line, overridable per project
   (both retired with C1 and C2 by D-016).
   **Overrides are reported, not hidden:** every value that differs from FLOW's default appears
   in the report's `overrides` metric and on the dashboard. A limit raised to turn CI green is
   then visible as exactly that, instead of disappearing into a config file.
