# ADR-034 — The radial menu holds actions that already exist
Status:     Accepted; its contents are superseded by ADR-038 and its refusal of `backdrop-filter` by ADR-039. The mechanism — the ring, the clamped centre, the centre label, the dismissal — stands unchanged, which is what it was built to be.
Date:       2026-09-06
Context:    The founder asked for a ring of glass bubbles on right-click, as a home for anything that cannot sit cleanly on the chrome. Building it without direction on its contents risked a large rework, so it was built as a mechanism first.
Decision:   Right-click on a note window opens a ring of seated bubbles at the pointer: New note, All notes, Settings, and the always-on-top pin. Every one of them is an action reachable elsewhere already — the ring is a faster way to them, not a second set of capabilities.
Consequences: Which actions are in the ring is one array, so changing it is cheap. That was the point of building it this way: the mechanism is the work, and the contents are a preference the founder can change without touching any of it.
            It suppresses the web view's own context menu, which would otherwise appear beside it.
            The ring's centre is pulled back from the window's edges. `.surface` clips what leaves it, so a ring opened in a corner would lose half its bubbles; it opens beside the pointer instead of being cut.
            The hovered action's name appears in the middle of the ring — the one place nothing else occupies, and the only way a ring of glyphs says what it does without a legend around it.
            The bubbles are the same seated lozenge as every other control, at `--space-8`. SMD-038's original note expected them to need `backdrop-filter`, on the reasoning that a bubble sits over the app's own content where that filter is legitimate. They do not: the lozenge is an opaque gloss, so there is nothing to see through and nothing to blur.
            A transparent backdrop catches the click that dismisses it, and Escape does too. A menu that can only be dismissed by choosing something has taken the window hostage.
