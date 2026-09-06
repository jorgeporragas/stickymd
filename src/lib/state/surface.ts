import { invoke } from '@tauri-apps/api/core';

/**
 * Which surface the window is drawn on.
 *
 * Rust decides — the compositor either frosted the window or it did not — and
 * the answer lands on the document root as `data-surface`. Design tokens carry
 * the difference from there, so no component ever branches on it
 * (docs/DESIGN.md principle 4).
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
  return mode;
}
