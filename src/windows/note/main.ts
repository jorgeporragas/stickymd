import '../../app.css';
import { mount } from 'svelte';
import { loadNote } from '../../lib/state/note';
import { applySurfaceMode } from '../../lib/state/surface';
import { applyTheme } from '../../lib/state/theme';
import NoteWindow from './NoteWindow.svelte';

const target = document.getElementById('app');

if (!target) {
  throw new Error('sticky.md: note window mount point #app is missing.');
}

// Both resolved before mounting: the surface mode so the window is painted in
// its final mode once rather than flicking from solid to glass, and the note
// so the editor is created with its content already in it.
const [, , note] = await Promise.all([applySurfaceMode(), applyTheme(), loadNote()]);

export default mount(NoteWindow, {
  target,
  props: {
    initial: note.body,
    initiallyPinned: note.alwaysOnTop,
    initialTint: note.tint
  }
});
