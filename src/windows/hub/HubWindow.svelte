<script lang="ts">
  import { onMount } from 'svelte';
  import NoteRow from '../../lib/components/NoteRow.svelte';
  import Scramble from '../../lib/components/Scramble.svelte';
  import WindowChrome from '../../lib/components/WindowChrome.svelte';
  import { deleteNote, listNotes, openNote, whenModified, type NoteSummary } from '../../lib/state/hub';

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

  onMount(() => {
    void refresh();
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
    void refresh();
  }}
  onblur={() => (revealed = false)}
/>
<svelte:body onpointerenter={() => (revealed = true)} onpointerleave={() => (revealed = false)} />

<div class="surface">
  <WindowChrome {revealed} closeLabel="Close the notes list" />

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
