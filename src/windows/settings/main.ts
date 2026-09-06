import '../../app.css';
import { mount } from 'svelte';
import { applySurfaceMode } from '../../lib/state/surface';
import { applyTheme } from '../../lib/state/theme';
import SettingsWindow from './SettingsWindow.svelte';

const target = document.getElementById('app');

if (!target) {
  throw new Error('sticky.md: settings window mount point #app is missing.');
}

// Resolved before mounting so the window is painted in its final surface mode
// once, rather than appearing solid and flicking to glass.
await Promise.all([applySurfaceMode(), applyTheme()]);

export default mount(SettingsWindow, { target });
