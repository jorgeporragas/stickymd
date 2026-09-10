import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

/**
 * The application-wide theme.
 *
 * Chosen in the tray or in settings, and applied to every window. A note's own
 * colour is separate and per-note — see `setTint` in `note.ts` and ADR-024.
 *
 * Windows listen for changes rather than reading the setting once: a theme that
 * only applied to windows opened afterwards is a setting that appears not to
 * work.
 *
 * **Rust stores the preference; this file resolves it.** The preference is
 * `frost`, `dark`, or `system`, and only the last needs resolving — the
 * operating system's own setting is something a window can ask for and be told
 * about, and Rust would have to ask a window to find out. Keeping the question
 * where the answer already lives means Rust stores a string it never has to
 * interpret.
 */

/** What is stored. `system` is a preference, not a theme. */
export type ThemePreference = 'frost' | 'dark' | 'system';

/** What a window is actually painted in. */
export type Theme = 'frost' | 'dark';

/**
 * Live only while the preference is `system`.
 *
 * Dropped the moment it stops being, because a window that kept listening
 * would repaint itself the next time the operating system changed — overriding
 * a choice the user had since made explicitly.
 */
let watching: UnlistenFn | undefined;

/** What the operating system is set to, as one of our two themes. */
export async function systemTheme(): Promise<Theme> {
  try {
    return (await getCurrentWindow().theme()) === 'dark' ? 'dark' : 'frost';
  } catch {
    // No backend, or a platform that will not say. Frost is the default
    // everywhere else in this application and is the honest fallback.
    return 'frost';
  }
}

/** Resolve a preference and paint this window in it. */
export async function applyPreference(preference: ThemePreference): Promise<Theme> {
  if (watching) {
    watching();
    watching = undefined;
  }

  const theme = preference === 'system' ? await systemTheme() : preference;
  document.documentElement.dataset.theme = theme;

  if (preference === 'system') {
    try {
      watching = await getCurrentWindow().onThemeChanged(({ payload }) => {
        document.documentElement.dataset.theme = payload === 'dark' ? 'dark' : 'frost';
      });
    } catch {
      // Nothing to listen to without a backend. The theme resolved above is
      // still correct; it simply will not follow a change made later.
    }
  }

  return theme;
}

export async function applyTheme(): Promise<Theme> {
  let preference: ThemePreference = 'frost';

  try {
    preference = await invoke<ThemePreference>('app_theme');
  } catch {
    // No backend — the window is being served in a plain browser for testing.
  }

  const theme = await applyPreference(preference);

  try {
    await listen<ThemePreference>('theme-changed', (event) => {
      void applyPreference(event.payload);
    });
  } catch {
    // Nothing to listen to without a backend.
  }

  return theme;
}
