<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount } from 'svelte';
  import { DEFAULT_CHORDS, type FormattingChords } from '../../lib/editor/commands';
  import Editor from '../../lib/components/Editor.svelte';
  import RadialMenu, { type RadialAction } from '../../lib/components/RadialMenu.svelte';
  import SaveTrouble from '../../lib/components/SaveTrouble.svelte';
  import TintPicker from '../../lib/components/TintPicker.svelte';
  import WindowChrome from '../../lib/components/WindowChrome.svelte';
  import {
    flushSave,
    onSaveTrouble,
    queueSave,
    setAlwaysOnTop,
    setTint,
    type Tint
  } from '../../lib/state/note';
  import { openNewNoteWindow, showHub, showSettings } from '../../lib/state/windows';

  interface Props {
    /** The note's source. A new window starts empty. */
    initial?: string;
    /**
     * Whether this note was pinned when it was last open. Read once, at
     * mount: after that the window owns the setting and writes it back.
     */
    initiallyPinned?: boolean;
    /** The note's colour when it was last open. Read once, as above. */
    initialTint?: Tint;
    /** The formatting chords, as configured. */
    formatting?: FormattingChords;
  }

  let {
    initial = '',
    initiallyPinned = false,
    initialTint = 'clear',
    formatting = DEFAULT_CHORDS
  }: Props = $props();

  let revealed = $state(false);

  // Capturing the initial value is the intent: the index seeds the window, and
  // from then on the window is authoritative and persists its own changes.
  // svelte-ignore state_referenced_locally
  let alwaysOnTop = $state(initiallyPinned);
  // svelte-ignore state_referenced_locally
  let tint = $state<Tint>(initialTint);

  function chooseTint(value: Tint): void {
    tint = value;
    void setTint(value);
  }

  function togglePin(value: boolean): void {
    alwaysOnTop = value;
    void setAlwaysOnTop(value);
  }

  let trouble = $state<string | undefined>(undefined);

  /** Where the radial menu is open, if it is. */
  let menuAt = $state<{ x: number; y: number } | undefined>(undefined);

  const menuActions = $derived<RadialAction[]>([
    { id: 'new', label: 'New note', icon: 'note' },
    { id: 'hub', label: 'All notes', icon: 'hub' },
    { id: 'settings', label: 'Settings', icon: 'settings' },
    { id: 'pin', label: alwaysOnTop ? 'Unpin' : 'Keep on top', icon: 'pin', engaged: alwaysOnTop }
  ]);

  function openMenu(event: MouseEvent): void {
    // The web view draws its own context menu otherwise, beside this one.
    event.preventDefault();
    menuAt = { x: event.clientX, y: event.clientY };
  }

  function chooseFromMenu(id: string): void {
    menuAt = undefined;

    switch (id) {
      case 'new':
        void openNewNoteWindow();
        break;
      case 'hub':
        void showHub();
        break;
      case 'settings':
        void showSettings();
        break;
      case 'pin':
        togglePin(!alwaysOnTop);
        break;
    }
  }

  function retrySave(): void {
    void flushSave();
  }

  function reveal(): void {
    revealed = true;
  }

  function recede(): void {
    revealed = false;
  }

  onMount(() => {
    const watching = onSaveTrouble((reason) => (trouble = reason));

    // Closing the window must not lose the last few characters typed: the
    // debounce may still be pending. Take over the close, write, then close.
    //
    // If that write fails, the first close is refused and the window says so
    // instead — closing would take the queued text with it, and a note
    // disappearing quietly is the failure this whole indicator exists to
    // prevent. Asking again closes anyway: by then the user has been told,
    // and a window that cannot be closed is its own kind of broken.
    let refused = false;

    const unlisten = getCurrentWindow()
      .onCloseRequested(async (event) => {
        event.preventDefault();

        const saved = await flushSave();

        if (!saved && !refused) {
          refused = true;
          reveal();
          return;
        }

        await getCurrentWindow().destroy();
      })
      .catch(() => undefined);

    return () => {
      watching();
      void unlisten.then((stop) => stop?.());
    };
  });
</script>

<!--
  Chrome reveals on entering the window and recedes on leaving it or on the
  window losing focus — docs/DESIGN.md principle 2. These belong on the window
  and body rather than on the surface element: entering the *window* is the
  gesture, and the surface is not an interactive control.

  Losing focus also flushes any pending write. Switching away from a note is
  the moment a user expects it to be saved.
-->
<svelte:body onpointerenter={reveal} onpointerleave={recede} />
<svelte:window
  onfocus={reveal}
  onblur={() => {
    recede();
    void flushSave();
  }}
/>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="surface" oncontextmenu={openMenu}>
  <WindowChrome {revealed} {alwaysOnTop} onAlwaysOnTop={togglePin} closeLabel="Close note">
    {#snippet controls()}
      <SaveTrouble reason={trouble} onRetry={retrySave} />
      <TintPicker {revealed} {tint} onTint={chooseTint} />
    {/snippet}
  </WindowChrome>
  <Editor value={initial} onChange={queueSave} {formatting} />

  {#if menuAt}
    <RadialMenu
      actions={menuActions}
      at={menuAt}
      onChoose={chooseFromMenu}
      onDismiss={() => (menuAt = undefined)}
    />
  {/if}
</div>
