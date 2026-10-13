---
name: flow-start
description: Start a session on the FLOW system — sync the default branch, read FLOW and STATUS, run flow_check, list the issues and pull requests waiting on the founder, then stop for confirmation.
disable-model-invocation: true
allowed-tools:
  - Bash(git fetch *)
  - Bash(git status *)
  - Bash(git branch --show-current)
  - Bash(git switch *)
  - Bash(git pull --ff-only)
  - Bash(git log *)
  - Bash(git diff *)
  - Bash(git config core.hooksPath)
  - Bash(python .flow/flow_check.py *)
  - Bash(python3 .flow/flow_check.py *)
  - Bash(gh api -X GET repos/{owner}/{repo}/issues *)
  - Bash(gh pr list *)
  - Bash(gh run list *)
  - PowerShell(git fetch *)
  - PowerShell(git status *)
  - PowerShell(git branch --show-current)
  - PowerShell(git switch *)
  - PowerShell(git pull --ff-only)
  - PowerShell(git log *)
  - PowerShell(git diff *)
  - PowerShell(git config core.hooksPath)
  - PowerShell(python .flow/flow_check.py *)
  - PowerShell(gh api -X GET repos/{owner}/{repo}/issues *)
  - PowerShell(gh pr list *)
  - PowerShell(gh run list *)
---

# /flow-start

This runs FLOW_SYSTEM.md › *Session Contract* › *Start of every session* in fixed steps. **FLOW
owns what each step means; this file owns only the order and the commands.** Shipped with FLOW
and verified by `flow_check` (C10): do not edit it. A project adds its own steps in CLAUDE.md ›
*Before Every Session*.

Run each command on its own, not chained, so a failure names its step. **If a command fails —
no network, `gh` not signed in — report it in that step and carry on.** One unavailable check
never stops the start. `<default>` below is the repository's default branch, usually `main`.

## 1. The working copy

1. `git fetch --prune origin`
2. `git status -sb` and `git branch --show-current`.
3. Act on what they show, and **never discard anything**: no stash, reset, restore, or branch
   deletion.
   - **Uncommitted changes:** list them, change nothing, and read the working copy as it stands.
   - **Clean, on another branch:** ask what it holds that the default branch does not —
     `git log --oneline --cherry-pick --right-only origin/<default>...HEAD`. It compares
     patches, because rebase merges rewrite hashes. Lists nothing: `git switch <default>`.
     Lists commits: report them and **ask the founder** before switching.
   - **Clean, on the default branch:** `git pull --ff-only` if it is behind. If it has
     diverged, report it and stop this step.
4. If step 3 switched branches, check `git diff --stat <old HEAD> HEAD -- CLAUDE.md`; if it
   lists the file, re-read CLAUDE.md.

## 2. Read

`FLOW_SYSTEM.md` and `docs/STATUS.md`, both in full. CLAUDE.md is already loaded. Everything else
is read on demand, per FLOW's reading order and CLAUDE › *Before Every Session*.

## 3. Phase

Per FLOW › *Phase Detection*. STATUS › *Current Phase* is authoritative.

## 4. Reconcile

1. `git log -1 --format=%h origin/<default> -- docs/STATUS.md` gives the commit STATUS last
   changed in. Then `python .flow/flow_check.py --range <that commit>..origin/<default>`.
   Report its findings. For the commits in that range, check what it cannot judge against FLOW's
   trigger table.
2. `git config core.hooksPath` must print `.githooks`. Anything else is a gap.

## 5. What waits on the founder

1. **The issues waiting on the founder**, one command per label — `needs-verification` (the
   confirmation list), then `founder-only`:
   `gh api -X GET repos/{owner}/{repo}/issues -f state=open -f labels=<label> -f per_page=100 --jq '.[] | select(.pull_request | not) | "#\(.number) \(.title)"'`.
   It reads the REST API on purpose: it also works in a cloud session.
2. **Open pull requests and their checks:**
   `gh pr list --state open --json number,title,isDraft,statusCheckRollup`.
3. **CI on the default branch:**
   `gh run list --branch <default> --limit 1 --json conclusion,status,workflowName,headSha,displayTitle`.

## 6. Report, then stop

Gaps first: anything step 1 left unresolved and anything step 4 found, each with what closes it.
Then, in order:

1. The phase, by name.
2. The project in two or three sentences, from STATUS › *Current Active Step*.
3. What waits on the founder: step 5, as lists.

Then **stop and wait for the founder to confirm or correct it.** Write no code and change no file
before they reply.

## 7. Once the founder names the work

From an up-to-date default branch, `git switch -c <area>/<short-slug>`. Every change reaches the
default branch by pull request (FLOW › *Behavior Contract*); a pull request that closes an issue
owes CLOSING › *Closing an Issue*, read in full first.
