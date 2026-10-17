# ADR-041 — A window you ask for comes to the desktop you are on
Status:     Accepted
Date:       2026-10-07
Context:    SMD-103. With the hub open on one virtual desktop, asking for it from another (tray, relaunch, launch) focused it where it was, and Windows switched desktops to show it, taking the user away from what they were doing.
Decision:   A window that is already open is moved to the virtual desktop in use before it is focused. This covers the hub, the settings window and an already-open note opened from the hub. `windows::summon` does it, through `IVirtualDesktopManager::MoveWindowToDesktop`.
Consequences: There is no public API for the current desktop's ID. It is read from where Explorer records it in the registry, which is what Windows Terminal and PowerToys do. Windows 11 and Windows 10 keep it in different keys, and both are read. If either half fails, the old behaviour is the fallback: the window is focused where it is.
            Windows only. macOS brings a window to the active Space when it is brought forward, so there is nothing to do there.
            A note opened fresh from the hub was never the problem: a new window is created on the desktop in use.
