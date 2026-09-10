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

/**
 * The mode this window will settle into, while it is still painted opaque.
 *
 * Undefined once it has settled, or when it was Solid to begin with and there
 * is nothing to settle into.
 */
let settling: SurfaceMode | undefined;

export async function applySurfaceMode(): Promise<SurfaceMode> {
  let mode: SurfaceMode = 'solid';

  try {
    mode = await invoke<SurfaceMode>('surface_mode');
  } catch {
    // No Tauri backend — the window is being served in a plain browser for
    // testing. Solid is the honest answer: there is no compositor here.
  }

  // **Painted opaque first, whatever the mode.** A window is shown the instant
  // it has painted (SMD-089), and the compositor's blur is not always composed
  // by then — so a window that painted its glass straight away showed the
  // desktop through it, unblurred, for a fraction of a second. Opening as a
  // solid card and dissolving into glass is the founder's own suggestion and
  // the right one: there is no moment where the window is pretending to be
  // transparent over something that is not yet frosted.
  document.documentElement.dataset.surface = 'solid';
  settling = mode === 'glass' ? mode : undefined;

  try {
    // A change *after* the window is up applies at once. The deferral above is
    // about arriving, not about the setting: someone switching transparency off
    // is watching for it to happen.
    await listen<SurfaceMode>('surface-changed', (event) => {
      settling = undefined;
      document.documentElement.dataset.surface = event.payload;
    });
  } catch {
    // Nothing to listen to without a backend.
  }

  return mode;
}

/**
 * Let the window become glass, once it is on screen and frosted.
 *
 * The fade itself is `.surface`'s own `background-color` transition — a paint
 * over a backdrop that does not change, which is the tint layer `docs/DESIGN.md`
 * says to animate rather than the surface's own alpha. Nothing is re-blurred
 * per frame.
 */
export function settleSurface(): void {
  if (!settling) return;

  document.documentElement.dataset.surface = settling;
  settling = undefined;
}

/**
 * How long to stay opaque first, from `--dur-glass-hold`.
 *
 * Read from the stylesheet rather than written here: durations live in the
 * token layer, and this one exists to be tuned by whoever is watching a window
 * open. Falls back to nothing if the token is unreadable — an early dissolve
 * is a worse look than no delay, but it is not a broken window.
 */
export function glassHoldMs(): number {
  const value = getComputedStyle(document.documentElement)
    .getPropertyValue('--dur-glass-hold')
    .trim();

  if (value.endsWith('ms')) return Number.parseFloat(value) || 0;
  if (value.endsWith('s')) return (Number.parseFloat(value) || 0) * 1000;

  return 0;
}
