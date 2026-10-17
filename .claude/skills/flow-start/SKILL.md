---
name: flow-start
description: Start a session — bring the working copy up to date, list what waits on the founder, then wait for the founder to name the work.
disable-model-invocation: true
allowed-tools:
  - Bash(git fetch *)
  - Bash(git status *)
  - Bash(git switch *)
  - Bash(git pull --ff-only)
  - Bash(git config core.hooksPath)
  - Bash(gh api -X GET repos/{owner}/{repo}/issues *)
  - Bash(gh pr list *)
  - PowerShell(git fetch *)
  - PowerShell(git status *)
  - PowerShell(git switch *)
  - PowerShell(git pull --ff-only)
  - PowerShell(git config core.hooksPath)
  - PowerShell(gh api -X GET repos/{owner}/{repo}/issues *)
  - PowerShell(gh pr list *)
---

# /flow-start

Installed by FLOW; do not edit it. Run each command on its own. If one fails, say so and go on.
Never discard anything: no stash, reset or branch deletion.

1. `git fetch --prune origin`, then `git status -sb`.
   - **Uncommitted changes:** list them and change nothing.
   - **On a branch whose upstream is `gone`:** its pull request was merged or closed. `git switch main`, then `git pull --ff-only`. The local branch stays.
   - **On a branch with a live upstream:** it is work in progress. Ask before switching.
   - **On `main`:** `git pull --ff-only`.
2. `git config core.hooksPath` must print `.githooks`. If not, say so.
3. List, one command each:
   - `gh pr list --state open`
   - `gh api -X GET repos/{owner}/{repo}/issues -f state=open -f labels=<label> -f per_page=100 --jq '.[] | select(.pull_request | not) | "#\(.number) \(.title)"'` for `needs-verification`, `founder-only` and `active`.
4. Report what you found, then **wait for the founder to name the work.** Change nothing before they do.
