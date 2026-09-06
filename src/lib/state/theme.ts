import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

/**
 * The application-wide theme.
 *
 * Chosen once, in the tray, and applied to every window. A note's own colour is
 * separate and per-note — see `setTint` in `note.ts` and ADR-024.
 *
 * Windows listen for changes rather than reading the setting once: a theme that
 * only applied to windows opened afterwards is a setting that appears not to
 * work.
 */
export async function applyTheme(): Promise<void> {
  let theme = 'frost';

  try {
    theme = await invoke<string>('app_theme');
  } catch {
    // No backend — the window is being served in a plain browser for testing.
  }

  document.documentElement.dataset.theme = theme;

  try {
    await listen<string>('theme-changed', (event) => {
      document.documentElement.dataset.theme = event.payload;
    });
  } catch {
    // Nothing to listen to without a backend.
  }
}
