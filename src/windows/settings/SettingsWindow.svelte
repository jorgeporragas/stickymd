<script lang="ts">
  import { onMount } from 'svelte';
  import Field from '../../lib/components/Field.svelte';
  import ChordInput from '../../lib/components/ChordInput.svelte';
  import Toggle from '../../lib/components/Toggle.svelte';
  import {
    checkForUpdate,
    installUpdate,
    restart,
    runningVersion,
    type UpdateState
  } from '../../lib/state/updates';
  import Scramble from '../../lib/components/Scramble.svelte';
  import WindowChrome from '../../lib/components/WindowChrome.svelte';
  import {
    chooseFolder,
    inspectFolder,
    readPreferences,
    setLaunchAtStartup,
    setNewNoteShortcut,
    setFormattingShortcut,
    setNotesFolder,
    setTheme,
    type FormattingChords,
    type Preferences
  } from '../../lib/state/preferences';

  let prefs = $state<Preferences | undefined>(undefined);
  let revealed = $state(false);
  let shortcutProblem = $state<string | undefined>(undefined);

  /** A folder the user has picked but not yet decided what to do about. */
  let pending = $state<{ path: string; notesToMove: number; hasNotes: boolean } | undefined>(
    undefined
  );

  onMount(async () => {
    VERSION = await runningVersion();

    const loaded = await readPreferences();
    prefs = loaded;
    shortcutProblem = loaded.shortcutProblem ?? undefined;
  });

  async function toggleDark(on: boolean): Promise<void> {
    const theme = on ? 'dark' : 'frost';
    if (prefs) prefs.theme = theme;
    // Applied here as well as saved: this window listens for the same event it
    // is about to cause, but seeing it happen should not wait on a round trip.
    document.documentElement.dataset.theme = theme;
    await setTheme(theme);
  }

  async function toggleStartup(on: boolean): Promise<void> {
    const took = await setLaunchAtStartup(on);
    if (prefs) prefs.launchAtStartup = took;
  }

  /**
   * The chord arrives already valid — `ChordInput` cannot produce one that is
   * not — so what comes back here is the operating system's answer, which is a
   * different question: the chord is well-formed and another application may
   * still hold it.
   */
  async function commitShortcut(chord: string): Promise<void> {
    if (!prefs || chord === prefs.newNoteShortcut) return;

    const problem = await setNewNoteShortcut(chord);
    shortcutProblem = problem ?? undefined;
    prefs.newNoteShortcut = chord;
  }

  /**
   * The four formatting chords, as rows.
   *
   * A fixed list rather than something derived: the set is deliberately small
   * and closed (ADR-037), and a settings file cannot name a command that does
   * not exist.
   */
  const FORMATTING: { action: keyof FormattingChords; label: string }[] = [
    { action: 'bold', label: 'Bold' },
    { action: 'italic', label: 'Italic' },
    { action: 'inlineCode', label: 'Inline code' },
    { action: 'strikethrough', label: 'Strikethrough' }
  ];

  async function commitFormatting(action: keyof FormattingChords, chord: string): Promise<void> {
    if (!prefs || chord === prefs.formatting[action]) return;

    const settled = await setFormattingShortcut(action, chord);
    if (settled) prefs.formatting = settled;
  }

  /**
   * The update flow, which never advances on its own.
   *
   * Three deliberate stops: checking is a button, installing is a second
   * button after the version is named, and restarting is a third. ADR-014's
   * condition is that the updater prompts before replacing anything — and a
   * restart that took away the note someone was mid-sentence in would be its
   * own kind of replacing.
   */
  let update = $state<UpdateState>({ status: 'idle' });

  /** The running version, read from the binary rather than from a manifest. */
  let VERSION = $state('');

  /**
   * What the row says under its label. The version is the useful fact at rest
   * — "you are on 1.0.0" answers the question the button exists to ask.
   */
  const updateNote = $derived.by(() => {
    switch (update.status) {
      case 'checking':
        return 'Asking GitHub…';
      case 'current':
        return `You are on ${VERSION}, which is the latest.`;
      case 'found':
        return `${update.version} is available. Nothing is replaced until you say so.`;
      case 'downloading':
        return update.percent === undefined
          ? `Downloading ${update.version}…`
          : `Downloading ${update.version} — ${update.percent}%`;
      case 'ready':
        return `${update.version} is installed. It takes effect when sticky.md restarts.`;
      case 'problem':
        return `You are on ${VERSION}.`;
      default:
        return `You are on ${VERSION}. Nothing is checked until you ask.`;
    }
  });

  async function lookForUpdate(): Promise<void> {
    update = { status: 'checking' };
    update = await checkForUpdate();
  }

  async function acceptUpdate(): Promise<void> {
    if (update.status !== 'found') return;

    const { update: found, version } = update;
    update = { status: 'downloading', version };

    update = await installUpdate(found, (percent) => {
      if (update.status === 'downloading') update = { status: 'downloading', version, percent };
    });
  }

  async function pickFolder(): Promise<void> {
    if (!prefs) return;
    const picked = await chooseFolder(prefs.notesFolder);
    if (!picked || picked === prefs.notesFolder) return;

    const change = await inspectFolder(picked);
    pending = {
      path: picked,
      notesToMove: change.notesToMove,
      hasNotes: change.destinationHasNotes
    };
  }

  async function resolveFolder(move: boolean): Promise<void> {
    if (!pending || !prefs) return;
    const settled = await setNotesFolder(pending.path, move);
    if (settled) prefs.notesFolder = settled;
    pending = undefined;
  }
</script>

<svelte:body onpointerenter={() => (revealed = true)} onpointerleave={() => (revealed = false)} />
<svelte:window onfocus={() => (revealed = true)} onblur={() => (revealed = false)} />

<div class="surface">
  <WindowChrome {revealed} closeLabel="Close settings" />

  <h1><Scramble text="settings" /></h1>

  <div class="body">
    {#if !prefs}
      <!-- Deliberately nothing. Reading a small JSON file is instant, and a
           spinner that flashes for 20ms is worse than a moment of blank. -->
    {:else if pending}
      <!--
        The folder question, asked at the time rather than decided in advance.
        It takes over the window instead of appearing beside the settings: it
        is a question with consequences, and a question you can ignore while
        clicking something else is a question that gets answered by accident.
      -->
      <div class="asking">
        <p class="asking-title">Move your notes?</p>
        <p class="asking-note">
          {pending.notesToMove === 1
            ? 'There is 1 note in the folder you are leaving.'
            : `There are ${pending.notesToMove} notes in the folder you are leaving.`}
          {#if pending.hasNotes}
            The folder you picked already has notes of its own; nothing there will be overwritten.
          {/if}
        </p>
        <p class="asking-path">{pending.path}</p>

        <div class="asking-actions">
          <button class="action primary" type="button" onclick={() => resolveFolder(true)}>
            Move them
          </button>
          <button class="action" type="button" onclick={() => resolveFolder(false)}>
            Leave them
          </button>
          <button class="action quiet" type="button" onclick={() => (pending = undefined)}>
            Cancel
          </button>
        </div>
      </div>
    {:else}
      <Field label="Dark" note="Applies to every window, including the ones already open.">
        {#snippet control()}
          <Toggle on={prefs?.theme === 'dark'} label="Dark theme" onChange={toggleDark} />
        {/snippet}
      </Field>

      <Field label="Notes folder" note={prefs.notesFolder}>
        {#snippet control()}
          <button class="action" type="button" onclick={pickFolder}>Change</button>
        {/snippet}
      </Field>

      <Field
        label="New note shortcut"
        note="Held with the operating system, so it works from any application."
        problem={shortcutProblem}
      >
        {#snippet control()}
          <ChordInput
            value={prefs?.newNoteShortcut ?? ''}
            format="global"
            label="New note shortcut"
            onChange={commitShortcut}
          />
        {/snippet}
      </Field>

      {#each FORMATTING as row (row.action)}
        <Field label={row.label} note="Inside a note.">
          {#snippet control()}
            <ChordInput
              value={prefs?.formatting[row.action] ?? ''}
              format="editor"
              label={row.label}
              onChange={(chord) => commitFormatting(row.action, chord)}
            />
          {/snippet}
        </Field>
      {/each}

      <Field
        label="Updates"
        note={updateNote}
        problem={update.status === 'problem' ? update.reason : undefined}
      >
        {#snippet control()}
          {#if update.status === 'found'}
            <button class="action primary" type="button" onclick={acceptUpdate}>
              Install {update.version}
            </button>
          {:else if update.status === 'ready'}
            <button class="action primary" type="button" onclick={restart}>Restart</button>
          {:else}
            <button
              class="action"
              type="button"
              disabled={update.status === 'checking' || update.status === 'downloading'}
              onclick={lookForUpdate}
            >
              Check
            </button>
          {/if}
        {/snippet}
      </Field>

      <Field label="Launch at startup" note="Off unless you turn it on.">
        {#snippet control()}
          <Toggle
            on={prefs?.launchAtStartup ?? false}
            label="Launch at startup"
            onChange={toggleStartup}
          />
        {/snippet}
      </Field>

      <p class="file">Settings file: {prefs.settingsFile}</p>
    {/if}
  </div>
</div>

<style>
  h1 {
    margin: 0;
    padding: 0 var(--space-4) var(--space-2);
    font-family: var(--font-display);
    font-size: var(--font-size-display);
    font-weight: var(--weight-body);
    letter-spacing: var(--tracking-display);
    line-height: var(--line-height-ui);
    color: var(--ink-primary);
  }

  .body {
    flex: 1 1 auto;
    min-height: 0;
    overflow-y: auto;
    padding: 0 var(--space-4) var(--space-4);
  }

  .action {
    flex: 0 0 auto;
    padding: var(--space-1) var(--space-3);
    border: 1px solid var(--rule);
    border-radius: var(--radius-chip);
    background: var(--control-hover);
    color: var(--ink-primary);
    font-family: inherit;
    font-size: var(--font-size-label);
    cursor: pointer;
    transition: background-color var(--dur-quick) var(--ease-out);
  }

  .action:hover {
    background: var(--control-active);
  }

  .action:active {
    background: var(--control-active);
    transition-duration: var(--dur-instant);
  }

  .action.primary {
    border-color: var(--swatch-rim);
    background: color-mix(in oklab, var(--signal-engaged) 35%, transparent);
  }

  .action.quiet {
    background: transparent;
    border-color: transparent;
    color: var(--ink-muted);
  }


  .asking {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-3) 0;
  }

  .asking-title {
    margin: 0;
    font-size: var(--font-size-title);
    color: var(--ink-primary);
  }

  .asking-note {
    margin: 0;
    font-size: var(--font-size-body);
    line-height: var(--line-height-prose);
    color: var(--ink-secondary);
  }

  .asking-path {
    margin: 0;
    font-family: var(--font-mono);
    font-size: var(--font-size-code);
    line-height: var(--line-height-code);
    color: var(--ink-muted);
    overflow-wrap: anywhere;
  }

  .asking-actions {
    display: flex;
    gap: var(--space-2);
    padding-top: var(--space-2);
  }

  .file {
    margin: var(--space-4) 0 0;
    font-size: var(--font-size-caption);
    line-height: var(--line-height-ui);
    color: var(--ink-muted);
    overflow-wrap: anywhere;
  }
</style>
