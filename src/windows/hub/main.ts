import '../../app.css';
import { mount } from 'svelte';
import { applySurfaceMode } from '../../lib/state/surface';
import HubWindow from './HubWindow.svelte';

const target = document.getElementById('app');

if (!target) {
  throw new Error('sticky.md: hub window mount point #app is missing.');
}

// Resolved before mounting so the window is painted in its final surface mode
// once, rather than appearing solid and flicking to glass.
await applySurfaceMode();

export default mount(HubWindow, { target });
