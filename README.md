# sticky.md

Sticky notes that render markdown as you type, and stay plain `.md` files you own.

It's for the text that isn't worth a vault: a prompt you're editing, a paragraph headed somewhere else, a scrap that lives a few days and then gets pasted and forgotten. Obsidian renders markdown properly but makes you open a vault. Sticky Notes is the right shape but turns `**bold**` into literal asterisks. This is the gap between them.

- **Markdown renders inline.** Syntax hides on the lines your cursor isn't on and comes back on the line it is. `Ctrl+B` inserts real `**` characters — what you see and what's in the file are never different things.
- **Copy gives you the source.** Paste into anything and you get raw markdown, not styled text.
- **Your notes are files.** One `.md` per note in a plain folder. Grep them, sync them, edit them elsewhere, or delete the app and keep them.
- **Nothing leaves your machine.** No account, no telemetry, no sync, no AI — not as a setting you can turn off, but as code that doesn't exist.

## Install

[**Download the latest release.**](../../releases/latest) Windows 10 or 11, 64-bit. Take the installer, or the portable zip if you'd rather nothing touched the registry.

Windows will say **"Windows protected your PC"** — click **More info → Run anyway**. The binaries are unsigned, and a certificate that removes that warning costs a few hundred dollars a year which this project doesn't have.

It lives in the tray. **Ctrl+Shift+Space** makes a note from anywhere; right-click the tray icon for everything else. Notes land in `Documents\sticky.md`.

## Build it yourself

Needs Node, Rust, and the MSVC C++ build tools.

```bash
git clone https://github.com/jorgeporragas/stickymd.git
cd stickymd
npm install
git config core.hooksPath .githooks
npm run tauri dev
```

That third line isn't optional: the pre-commit hook lives in `.githooks/`, and git doesn't look there until it's told to — on every clone, not once per repository.

## Under it

Tauri v2 with a Rust backend, Svelte 5 and TypeScript in the frontend, CodeMirror 6 for the editor. It uses the web view Windows already has rather than bundling a browser engine, which is most of why it's under 2 MB.

The docs are unusually complete for something this size, because they're kept under a system that treats a stale doc as a bug: [MASTER.md](MASTER.md) for what the product is, [docs/DECISIONS.md](docs/DECISIONS.md) for why anything is the way it is, [docs/DESIGN.md](docs/DESIGN.md) for the visual system, [docs/STATUS.md](docs/STATUS.md) for everything with a state, and [docs/FIXES.md](docs/FIXES.md) for the fixes that must not be undone.

## Licence

[GPL-3.0-only](LICENSE). Forks stay open.

The bundled typefaces — [Departure Mono](https://departuremono.com/), [Geist](https://fonts.google.com/specimen/Geist) and [JetBrains Mono](https://www.jetbrains.com/lp/mono/) — are under the SIL Open Font License, each licence text alongside the files in `src/assets/fonts/`.
