# ADR-003 — Tauri v2 rather than Electron
Status:     Accepted
Date:       2026-09-05
Context:    Inline markdown rendering requires a web view (see ADR-004), so the choice was which shell wraps it. The founder named low resource use as a top-two priority, and the product's architecture is many small windows.
Decision:   Tauri v2, using the operating system's web view.
Consequences: Roughly 5–10 MB bundled against Electron's ~150 MB, and no bundled Chromium process per window. Introduces Rust to a project whose founder does not know it. Narrows the contributor pool, accepted knowingly in ADR-005. Codified as MASTER veto 4.
