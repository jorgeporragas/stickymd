# ADR-042 — FLOW's files are tracked in the repository
Status:     Accepted
Date:       2026-10-13
Supersedes: ADR-015
Context:    FLOW 3.0 vendors its files into the project (`FLOW_SYSTEM.md`, `.flow/`, the pre-commit hook and the session-start skill) and verifies them in CI against a manifest. Untracked, CI and a fresh clone's hook cannot run them. On 2026-10-13 the founder chose to track them, knowing the repository is public. The founder's reason: it is more practical to have FLOW's files in the repository.
Decision:   Track every file `flow_sync` installs. `.flow/dashboard.html` is generated and stays ignored.
Consequences: FLOW becomes publicly readable through this repository. `.gitignore` no longer excludes `FLOW_SYSTEM*.md`, and `CLAUDE.md` no longer describes the spec as untracked. The untracked `docs/FLOW_SYSTEM_v2.2.md` is no longer used and can be deleted.
            2026-10-13 — CLAUDE.md amended; ADR-015 marked superseded.
