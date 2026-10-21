# ADR-046 — No macOS build is released until Apple signing exists
Status:     Accepted
Date:       2026-10-14
Context:    SMD-106 (#8). The macOS release job failed on v1.2.0 and v1.3.0: with no Apple secrets, each APPLE_* variable reached Tauri as an empty string, which it reads as an instruction to sign, and the build died importing an empty certificate. Letting it build unsigned would attach a `.dmg` and `.app` that Gatekeeper refuses to open when downloaded. The founder's reason: the macOS app has not actually been made yet; it is written and compiled by CI but has run on no Mac (#6).
Decision:   The release workflow skips the macOS job, rather than failing it or building unsigned, until the Apple certificate is configured. A release is Windows-only until then.
Consequences: The Windows job reports whether `APPLE_CERTIFICATE` is set, and the macOS job runs only when it is. Creating the Apple secrets (#19) is what turns macOS releases on; nothing in the workflow has to change.
            2026-10-14 — CLAUDE § Build and release amended.
