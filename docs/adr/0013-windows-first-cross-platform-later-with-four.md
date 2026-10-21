# ADR-013 — Windows first, cross-platform later, with four disciplines held from the start
Status:     Accepted
Date:       2026-09-05
Context:    The founder wants cross-platform eventually and asked whether deferring it would make it harder. Tauri v2 is cross-platform natively; the expensive part of a port is not the build target but retrofitting platform assumptions baked in throughout.
Decision:   Build for Windows now. Hold four disciplines from the first commit: modifier keys resolved through an abstraction, no string-concatenated paths, deletion through a trash abstraction, custom window chrome everywhere.
Consequences: Porting becomes largely a build-target change plus a rendering bug hunt on WebKitGTK, which cannot be predicted and must simply happen. Recorded as `CLAUDE.md § Cross-platform discipline`. Windows-only is explicitly not a veto.
