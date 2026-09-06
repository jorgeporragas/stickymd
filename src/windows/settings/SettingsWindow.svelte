<script lang="ts">
  import { onMount } from 'svelte';
  import Field from '../../lib/components/Field.svelte';
  import Toggle from '../../lib/components/Toggle.svelte';
  import WindowChrome from '../../lib/components/WindowChrome.svelte';
  import {
    chooseFolder,
    inspectFolder,
    readPreferences,
    setLaunchAtStartup,
    setNewNoteShortcut,
    setNotesFolder,
    setTheme,
    type Preferences
  } from '../../lib/state/preferences';

  let prefs = $state<Preferences | undefined>(undefined);
  let revealed = $state(false);
  let shortcutDraft = $state('');
  let shortcutProblem = $state<string | undefined>(undefined);

  /** A folder the user has picked but not yet decided what to do about. */
  let pending = $state<{ path: string; notesToMove: number; hasNotes: boolean } | undefined>(
    undefined
  );

  onMount(async () => {
    const loaded = await readPreferences();
    prefs = loaded;
    shortcutDraft = loaded.newNoteShortcut;
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

  async function commitShortcut(): Promise<void> {
    if (!prefs || shortcutDraft === prefs.newNoteShortcut) return;
    const problem = await setNewNoteShortcut(shortcutDraft);
    shortcutProblem = problem ?? undefined;
    prefs.newNoteShortcut = shortcutDraft;
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

  <h1>settings</h1>

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
          <input
            class="chord"
            type="text"
            spellcheck="false"
            aria-label="New note shortcut"
            bind:value={shortcutDraft}
            onblur={commitShortcut}
          />
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

  .chord {
    flex: 0 0 auto;
    /* Wide enough for the default chord. "CmdOrCtrl+Shift+Space" is 21
       characters, and at 13px the code face sets that in about 10.3rem before
       padding — 11rem clipped the last letter. */
    width: 13rem;
    padding: var(--space-1) var(--space-2);
    border: 1px solid var(--rule);
    border-radius: var(--radius-chip);
    background: var(--control-hover);
    color: var(--ink-primary);
    font-family: var(--font-mono);
    font-size: var(--font-size-code);
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
