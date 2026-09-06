import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

/**
 * Which surface the window is drawn on.
 *
 * Rust decides — the compositor either frosted the window or it did not — and
 * the answer lands on the document root as `data-surface`. Design tokens carry
 * the difference from there, so no component ever branches on it
 * (docs/DESIGN.md principle 4).
 *
 * Windows listen for changes rather than reading the answer once. Transparency
 * is a system setting the user can switch off mid-session, and Windows switches
 * it off for them under battery saver — a window that only asked at startup
 * would keep painting a tint over a compositor that had stopped drawing
 * anything behind it, which reads as washed out rather than as Solid.
 */
export type SurfaceMode = 'glass' | 'solid';

export async function applySurfaceMode(): Promise<SurfaceMode> {
  let mode: SurfaceMode = 'solid';

  try {
    mode = await invoke<SurfaceMode>('surface_mode');
  } catch {
    // No Tauri backend — the window is being served in a plain browser for
    // testing. Solid is the honest answer: there is no compositor here.
  }

  document.documentElement.dataset.surface = mode;

  try {
    await listen<SurfaceMode>('surface-changed', (event) => {
      document.documentElement.dataset.surface = event.payload;
    });
  } catch {
    // Nothing to listen to without a backend.
  }

  return mode;
}
