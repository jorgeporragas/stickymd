import '../../app.css';
import { mount } from 'svelte';
import { applySurfaceMode } from '../../lib/state/surface';
import { revealWindow } from '../../lib/state/windows';
import { applyTheme } from '../../lib/state/theme';
import HubWindow from './HubWindow.svelte';

const target = document.getElementById('app');

if (!target) {
  throw new Error('sticky.md: hub window mount point #app is missing.');
}

// Resolved before mounting so the window is painted in its final surface mode
// once, rather than appearing solid and flicking to glass.
await Promise.all([applySurfaceMode(), applyTheme()]);

const app = mount(HubWindow, { target });

// The window is built hidden and shown here, once the surface has been
// painted — see `revealWindow`. Not awaited: nothing below depends on it, and
// the mount is what it is waiting for.
void revealWindow();

export default app;
