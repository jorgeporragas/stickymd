# ADR-014 — Auto-update ships after V1
Status:     Accepted
Date:       2026-09-05
Context:    The Tauri updater would check GitHub Releases and offer new versions. It transmits no note content and so does not conflict with ADR-008, but it is the application making a network request on its own initiative. The founder approved the feature.
Decision:   Not in V1. V1 makes zero network requests; updates are downloaded manually from GitHub Releases. The updater ships in Phase 7 — Release & Auto-Update.
Consequences: "V1 makes zero network requests" is a promise that needs no footnote. The updater is useless until a second release exists anyway. When it ships it must prompt before replacing anything, never install silently.
