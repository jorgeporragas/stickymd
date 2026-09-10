<script lang="ts">
  import { onMount } from 'svelte';
  import NoteRow from '../../lib/components/NoteRow.svelte';
  import Scramble from '../../lib/components/Scramble.svelte';
  import WindowChrome from '../../lib/components/WindowChrome.svelte';
  import {
    deleteNote,
    listNotes,
    onNotesChanged,
    openNote,
    whenModified,
    type NoteSummary
  } from '../../lib/state/hub';
  import { openNewNoteWindow } from '../../lib/state/windows';

  let notes = $state<NoteSummary[]>([]);
  let loaded = $state(false);
  let revealed = $state(false);

  async function refresh(): Promise<void> {
    notes = await listNotes();
    loaded = true;
  }

  async function remove(name: string): Promise<void> {
    if (await deleteNote(name)) {
      await refresh();
    }
  }

  /**
   * Make a note from the window whose whole subject is notes.
   *
   * The hub is not closed afterwards. The new note takes focus, and a hub that
   * shut itself would be answering a question nobody asked — the list is where
   * you were, and it now has one more thing in it.
   */
  async function newNote(): Promise<void> {
    await openNewNoteWindow();
    await refresh();
  }

  onMount(() => {
    void refresh();

    // Refreshed when the folder changes, not only when this window is clicked.
    // Being focused is a fine moment to re-read and a poor one to rely on —
    // the case that matters is a note written in another window while the list
    // is in plain sight.
    return onNotesChanged(() => void refresh());
  });
</script>

<!--
  Refreshed whenever the hub is looked at again. Notes change in other windows
  and in the folder itself, and a list that is only right when it was opened is
  worse than one that is right when you look at it.
-->
<svelte:window
  onfocus={() => {
    revealed = true;
    // Still re-read on focus, as well as on the event. The event covers a
    // change this application made; this covers one it did not — a note edited
    // in another editor, or dropped into the folder.
    void refresh();
  }}
  onblur={() => (revealed = false)}
/>
<svelte:body onpointerenter={() => (revealed = true)} onpointerleave={() => (revealed = false)} />

<div class="surface">
  <WindowChrome {revealed} closeLabel="Close the notes list">
    {#snippet controls()}
      <button
        class="lozenge chrome-control"
        class:revealed
        type="button"
        aria-label="New note"
        onclick={newNote}
      >
        <!-- Two strokes on the same 12-unit grid the other chrome glyphs use.
             Hand-drawn rather than the pixel set: a chrome control is 16px with
             its glyph at 58% of that, and those are only sharp at 24px. See
             docs/DESIGN.md § Iconography. -->
        <svg viewBox="0 0 12 12" aria-hidden="true" focusable="false">
          <path d="M6 2.6v6.8M2.6 6h6.8" />
        </svg>
      </button>
    {/snippet}
  </WindowChrome>

  <h1><Scramble text="notes" /></h1>

  <div class="list">
    {#if !loaded}
      <!-- Deliberately nothing: reading a folder is fast, and a spinner that
           flashes for 20ms is worse than a moment of blank. -->
    {:else if notes.length === 0}
      <p class="empty">Nothing yet.</p>
    {:else}
      {#each notes as note (note.name)}
        <NoteRow
          title={note.title}
          when={whenModified(note.modifiedMs)}
          onOpen={() => openNote(note.name)}
          onDelete={() => remove(note.name)}
        />
      {/each}
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

  .list {
    flex: 1 1 auto;
    min-height: 0;
    overflow-y: auto;
    padding: 0 var(--space-2) var(--space-2);
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  /* The content face, not the display one. "Nothing yet." is a sentence the
     application is saying, and the display face is for identity — the app's
     name and the hub's title. A sentence set in it reads as a label on the
     furniture rather than as the app talking. */
  /* The content face at content dimensions. "Nothing yet." is a sentence the
     application is saying, and it should be the same size as the sentences the
     user reads everywhere else — the display face is for identity, and a
     larger size would make an empty folder feel like an announcement. */
  .empty {
    margin: var(--space-4) var(--space-2);
    font-family: var(--font-content);
    font-size: var(--font-size-body);
    line-height: var(--line-height-prose);
    letter-spacing: var(--tracking-body);
    color: var(--ink-muted);
  }
</style>
