# ADR-001 — Notes are plain markdown files on disk
Status:     Accepted
Date:       2026-09-05
Context:    Notes could live in an application database with markdown as the content format, or as real files in a folder. The product exists because a vault is too much ceremony for a scrap, and the founder wanted notes reachable outside the app — greppable, syncable, openable in any editor.
Decision:   Each note is a `.md` file in a single flat folder. The file is the note's identity and its only record of content.
Consequences: The app can never hold a user's text hostage; deleting the app leaves every note readable. Costs file watching, rename collision handling, and title deduplication. Forces ADR-002.
            2026-09-05 — MASTER § Domain Model written to this decision.
