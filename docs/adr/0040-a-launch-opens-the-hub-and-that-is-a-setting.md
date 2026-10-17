# ADR-040 — A launch opens the hub, and that is a setting
Status:     Accepted
Date:       2026-10-07
Context:    A launch with no session to restore opened an empty note window. That is because Tauri built the note window from the config at startup and session restore reused it, not because anyone chose it. The founder asked for the hub instead, as the default, with a setting to choose between the two.
Decision:   A launch brings back the notes that were open. If none come back, it opens the hub by default, or a new note if the settings say so. The setting is "At launch" in the settings window, stored as `onLaunch`, `"hub"` or `"note"`. Launching again while sticky.md is already running always opens the hub, whatever the setting says. That was the founder's call, and it keeps a relaunch the same gesture as a tray click.
Consequences: No window is created from the config any more: the note entry is `create: false`, like the hub and settings. `windows::launch` decides, and the config keeps only the geometry. The window that used to be reused for the first restored note is gone, and every restored note is now built the same way.
            "Nothing to restore" is measured by what ended up on screen, not by what the index listed. A restore that fails outright still opens the launch choice, because starting into nothing but a tray icon reads as a launch that failed.
            The setting is read leniently. Any value but `"note"` means the hub, because a strict enum would fail the whole settings file on one typo and reset every other setting with it.
            With launch-at-startup on and no notes left open, the hub now appears at login where a blank note used to. Choosing "New note" restores the old behaviour exactly.
