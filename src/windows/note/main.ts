import '../../app.css';
import { mount } from 'svelte';
import NoteWindow from './NoteWindow.svelte';

const target = document.getElementById('app');

if (!target) {
  throw new Error('StickyMD: note window mount point #app is missing.');
}

export default mount(NoteWindow, { target });
