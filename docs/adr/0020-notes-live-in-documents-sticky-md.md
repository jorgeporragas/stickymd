# ADR-020 — Notes live in Documents/sticky.md
Status:     Accepted
Date:       2026-09-05
Context:    Phase 2 needs a default location for the notes folder, and ADR-001 constrains the choice: notes are files the user can open in another editor, grep, and sync. An application data directory — `%APPDATA%` or `%LOCALAPPDATA%` on Windows — is the conventional home for app-managed files, but it is a folder most people never open, which would make "your notes are files you own" true in principle and useless in practice. Making the location configurable is deferred (STATUS SMD-003), so this default is the only location.
Decision:   The notes folder is `sticky.md` inside the user's Documents directory, resolved through Tauri's path API rather than a constructed string, and created on first use.
Consequences: The folder is somewhere a person can find without being told where to look, which is what ADR-001 was for. On machines where Documents is redirected into OneDrive the notes will sync — that is the user's own arrangement and does not touch MASTER veto 2, which forbids *the application* transmitting content, not the user syncing their own files.
            Changing this later means moving a user's notes, so it is cheap to revisit only while no one has any. Note names arriving from the frontend are untrusted and validated before being joined to this path.
