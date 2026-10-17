# ADR-008 — Note content never leaves the machine
Status:     Accepted
Date:       2026-09-05
Context:    A narrower version was drafted first — content is never transmitted without an explicit per-action request — specifically to leave room for a future AI feature. ADR-007 removed that need.
Decision:   Absolute. No accounts, no telemetry, no analytics, no sync, no crash reports carrying note text.
Consequences: Recorded as MASTER veto 2. Forbids share-to-gist, export-to-paste-service, and similar affordances by the same rule. Fonts and assets are vendored rather than fetched, which `CLAUDE.md § Build and release` enforces. The version check in ADR-014 transmits no note content and does not conflict.
