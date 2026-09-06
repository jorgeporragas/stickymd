# sticky.md

A markdown scratchpad for Windows. Sticky notes that render markdown as you type — and stay plain `.md` files you own.

It exists for the text that isn't worth a vault: a prompt you're editing, a paragraph headed somewhere else, a scrap that lives a few days and then gets pasted and forgotten. Obsidian renders markdown properly but makes you open a vault and clutter it. Sticky Notes is the right shape but turns `**bold**` into literal asterisks. This is the gap between them.

> **This is not finished software.** What follows describes what sticky.md is being built to be. Some of it works today and some of it does not — **[docs/STATUS.md](docs/STATUS.md)** says which, item by item, and is kept accurate as a rule rather than as a courtesy. Don't install this expecting a product yet.

- **Markdown renders inline.** Syntax hides on the lines your cursor isn't on and comes back on the line it is. `Ctrl+B` inserts real `**` characters — what you see and what's in the file are never different things.
- **Copy gives you the source.** Select a rendered note, copy, paste into anything: you get raw markdown, not styled text. This is the property the whole thing exists to provide.
- **Your notes are files.** One `.md` per note in a plain folder. Point another editor at it, grep it, sync it, or delete the app entirely — the notes are still there and still readable.
- **Nothing leaves your machine.** No account, no telemetry, no analytics, no sync, no AI. Not as a setting you can turn off — the app has no code that sends anything anywhere.

## Installing

Releases will be published [here](../../releases), as a Windows installer and a portable zip. The portable zip needs no installer and touches no registry — extract it and run. Until then, building from source is the only way to run it.

**Windows will warn you when you run a downloaded build.** You'll see *"Windows protected your PC"* and have to click **More info → Run anyway**.

That warning is not a judgement about the software. It appears because the binaries are unsigned, and a certificate that removes it costs a few hundred dollars a year — which this project doesn't have. The warning will keep appearing until that changes. If you'd rather not click through it, building from source takes about five minutes and produces the same application.

## Building from source

You'll need Node, Rust, and the MSVC C++ build tools. Full toolchain requirements are in [CLAUDE.md § Local setup](CLAUDE.md#local-setup).

Clone the repository, then:

```bash
npm install
git config core.hooksPath .githooks
npm run tauri dev
```

That last step builds the Rust side too, so the first run takes a few minutes. Afterwards it's seconds.

## How it's built

Tauri v2 with a Rust backend, Svelte 5 and TypeScript in the frontend, CodeMirror 6 for the editor. It uses the operating system's web view rather than bundling a browser engine.

The documentation is unusually complete for a project this size, because it's maintained under a system that treats stale docs as bugs:

| | |
|---|---|
| [MASTER.md](MASTER.md) | What the product is — problem, scope, stack rationale, and the things it will never do |
| [docs/DESIGN.md](docs/DESIGN.md) | The visual system: principles, design tokens, the component inventory |
| [docs/DECISIONS.md](docs/DECISIONS.md) | Why things are the way they are, one decision per entry |
| [docs/STATUS.md](docs/STATUS.md) | Everything with a state |
| [docs/FIXES.md](docs/FIXES.md) | Hard-won fixes that must not be undone |

If you're wondering why a decision was made, ADRs are usually a faster answer than the code.

## Licence

[GPL-3.0-only](LICENSE). Forks stay open.

The bundled typefaces — [Departure Mono](https://departuremono.com/), [Geist](https://fonts.google.com/specimen/Geist) and [Martian Mono](https://fonts.google.com/specimen/Martian+Mono) — are under the SIL Open Font License, with each licence text alongside the files in `src/assets/fonts/`.
