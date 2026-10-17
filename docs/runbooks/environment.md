# sticky.md — Environment

The development machine and its toolchain as they stand. Read it before touching the toolchain,
the notes folder or a build's settings. Moved from STATUS when STATUS was retired (ADR-047,
2026-10-17). What is being worked on, and what blocks it, is in the GitHub issues.

## Machine and toolchain

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
