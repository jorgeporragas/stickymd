import { check, type Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { getVersion } from '@tauri-apps/api/app';

/**
 * Checking for a new version, and installing one if the user says so.
 *
 * The only network request this application makes. It carries nothing — no
 * note content, no identifier, no telemetry — which is why it does not
 * conflict with `MASTER.md` veto 2: that veto is about content leaving the
 * machine, not about sockets existing. See ADR-014.
 *
 * **Nothing here runs on its own.** There is no check at startup and no timer.
 * A person opens settings and presses a button, and is asked again before
 * anything is replaced. ADR-014's condition was that the updater must prompt
 * before replacing anything and never install silently; the simplest way to
 * keep that promise is to have no code that could break it.
 */

export type UpdateState =
  | { status: 'idle' }
  | { status: 'checking' }
  | { status: 'current' }
  /** A version is available and waiting for the user's decision. */
  | { status: 'found'; version: string; notes?: string; update: Update }
  | { status: 'downloading'; version: string; percent?: number }
  /** Installed, waiting to restart. */
  | { status: 'ready'; version: string }
  | { status: 'problem'; reason: string };

/**
 * The running version, from the binary itself.
 *
 * Read rather than imported from `package.json`: the manifest is what the
 * build was made from, and the thing on screen should be what is actually
 * executing. They agree — the release workflow refuses a tag where they do not
 * — but only one of them is evidence.
 */
export async function runningVersion(): Promise<string> {
  try {
    return await getVersion();
  } catch {
    return 'unknown';
  }
}

/**
 * Ask whether there is a newer release.
 *
 * A failure here is ordinary rather than exceptional — no network, GitHub
 * down, a corporate proxy — so it is reported as a sentence rather than
 * thrown. The application works perfectly well without ever succeeding at
 * this.
 */
export async function checkForUpdate(): Promise<UpdateState> {
  try {
    const update = await check();

    if (!update) return { status: 'current' };

    return {
      status: 'found',
      version: update.version,
      notes: update.body ?? undefined,
      update
    };
  } catch (error) {
    return { status: 'problem', reason: reasonFor(error) };
  }
}

/**
 * Download and install an update the user has agreed to.
 *
 * `onProgress` is fed a percentage where the server declared a length, and
 * nothing where it did not — a progress bar that invents its own numbers is
 * worse than a spinner.
 */
export async function installUpdate(
  update: Update,
  onProgress: (percent?: number) => void
): Promise<UpdateState> {
  let total = 0;
  let received = 0;

  try {
    await update.downloadAndInstall((event) => {
      if (event.event === 'Started') {
        total = event.data.contentLength ?? 0;
        onProgress(total ? 0 : undefined);
      } else if (event.event === 'Progress') {
        received += event.data.chunkLength;
        if (total) onProgress(Math.min(100, Math.round((received / total) * 100)));
      }
    });

    return { status: 'ready', version: update.version };
  } catch (error) {
    return { status: 'problem', reason: reasonFor(error) };
  }
}

/**
 * Restart into the new version.
 *
 * Separate from installing, and asked for separately: the update is on disk
 * either way, and a restart that takes away the note someone was in the middle
 * of writing is not an improvement. Windows are asked to close first, so each
 * one flushes its pending write the way it does on any close.
 */
export async function restart(): Promise<void> {
  await relaunch();
}

function reasonFor(error: unknown): string {
  const text = error instanceof Error ? error.message : String(error);

  // The two failures worth naming, because each has a different answer.
  if (/pubkey|signature|verify/i.test(text)) {
    return 'That download could not be verified as genuine, so it was not installed.';
  }
  if (/network|dns|connect|timed out|request/i.test(text)) {
    return 'Could not reach GitHub. Check the connection and try again.';
  }

  return text;
}
