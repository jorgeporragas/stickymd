# ADR-023 — Deleting a note closes its window
Status:     Accepted
Date:       2026-09-06
Context:    A note deleted while its window was open left the window on screen, and the window would write the note back on its next save — it still held the text and the filename. Two behaviours were defensible: the open window wins and the note survives, or the delete wins and the window goes. It was logged as STATUS SMD-031 rather than settled quietly, because either is coherent and only the founder could say which the product means.
Decision:   The delete wins. Deleting a note closes the window showing it, and the window is destroyed rather than asked to close.
Consequences: Destroying skips the frontend's close handler, which flushes a pending save — going through the normal close path would resurrect the file that was just deleted. Unsaved edits in that window are lost, which is correct: the note was deleted.
            The window is closed before the file is trashed rather than after, so a save landing between the two cannot write the note back.
