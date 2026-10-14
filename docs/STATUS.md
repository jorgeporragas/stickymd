# STATUS

Where the project is now. Work lives in the repository's GitHub issues, not here: what shipped is in closed issues and git, and `docs/COMPLETED.md` holds the work closed before FLOW 3.0.

---

## Current Phase

**Build** — Phase 6 — Cross-Platform Expansion, macOS first and then Linux.

## Current Active Step

**macOS.** The build is written through the vibrancy material, the menu-bar mark and the window behaviour, is compiled and tested on macOS by the Check workflow, and has been verified on no Mac. #6 carries the work and the list of what only a Mac can show.

**Blocked on the founder.** Step 6, the macOS updater and a notarised build, waits on the six Apple signing secrets (#19), which are his to create and which nothing here should ever see. Until they exist the release workflow skips the macOS job (ADR-046), so a release is Windows-only.

**What waits on the founder** is the open issues labelled `needs-verification` and `founder-only`. `/flow-start` lists them.

---

## Environment

| | |
|---|---|
| Development machine | Windows 11 Pro |
| Build target | Windows x64, and macOS arm64 since 2026-09-10 — built by CI, verified on no Mac |
| Node | v24.19.0, npm 11.17.0 — verified present 2026-09-05 |
| Rust | rustc 1.98.1, cargo 1.98.1, rustup 1.29.1 — installed 2026-09-05. |
| MSVC linker | Visual Studio C++ build tools installed 2026-09-05. Both halves build and the application runs. |
| WebView2 | Ships with Windows 11 |
| Notes folder | **A folder each, since SMD-100 on 2026-09-14.** A debug build reads `settings.dev.json` and defaults to `Documents\sticky.md (dev)`; the installed build keeps `settings.json` and points at `Documents\sticky.md`, read from the real `%APPDATA%` file (written 2026-10-11) on 2026-10-13. A session inside the Claude desktop app can see a stale copy instead: see `docs/FIXES.md`, "Anything the Claude desktop app starts sees a redirected `%APPDATA%`". |
| `core.hooksPath` | Set on the development machine 2026-09-05. |
