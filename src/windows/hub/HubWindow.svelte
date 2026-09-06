<script lang="ts">
  import { onMount } from 'svelte';
  import NoteRow from '../../lib/components/NoteRow.svelte';
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

  <h1>notes</h1>

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
  .surface {
    display: flex;
    flex-direction: column;

    /* Pinned to the viewport, not sized at 100%: a percentage height resolves
       to a fractional pixel and the sliver it leaves is painted by the
       compositor. See docs/FIXES.md. */
    position: fixed;
    inset: 0;
    border-radius: var(--radius-window);
    border: 1px solid var(--surface-border);
    background: var(--surface-paint);
    box-shadow:
      var(--shadow-rest),
      var(--surface-edge-highlight);
    overflow: hidden;
  }

  h1 {
    margin: 0;
    padding: 0 var(--space-4) var(--space-2);
    font-family: var(--font-display);
    font-size: var(--font-size-display);
    font-weight: var(--handjet-weight);
    font-variation-settings:
      'ELGR' var(--handjet-elgr),
      'ELSH' var(--handjet-elsh);
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

  .empty {
    margin: var(--space-4) var(--space-2);
    font-family: var(--font-display);
    font-size: var(--font-size-title);
    font-variation-settings:
      'ELGR' var(--handjet-elgr),
      'ELSH' var(--handjet-elsh);
    color: var(--ink-muted);
  }
</style>
