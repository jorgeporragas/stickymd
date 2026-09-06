<script lang="ts">
  import Editor from '../../lib/components/Editor.svelte';
  import WindowChrome from '../../lib/components/WindowChrome.svelte';

  let revealed = $state(false);

  function reveal(): void {
    revealed = true;
  }

  function recede(): void {
    revealed = false;
  }
</script>

<!--
  Chrome reveals on entering the window and recedes on leaving it or on the
  window losing focus — docs/DESIGN.md principle 2. These belong on the window
  and body rather than on the surface element: entering the *window* is the
  gesture, and the surface is not an interactive control.
-->
<svelte:body onpointerenter={reveal} onpointerleave={recede} />
<svelte:window onfocus={reveal} onblur={recede} />

<div class="surface">
  <WindowChrome {revealed} />
  <Editor />
</div>

<style>
  .surface {
    display: flex;
    flex-direction: column;
    height: 100%;
    border-radius: var(--radius-window);
    border: 1px solid var(--surface-border);

    /* The window itself is transparent so the corners can round; the surface is
       painted here. --surface-paint resolves to the opaque surface or the tint
       over compositor blur, per data-surface. No branching here — the token
       carries the difference. See docs/DESIGN.md principle 4. */
    background: var(--surface-paint);
    box-shadow:
      var(--shadow-rest),
      var(--surface-edge-highlight);

    overflow: hidden;
  }
</style>
