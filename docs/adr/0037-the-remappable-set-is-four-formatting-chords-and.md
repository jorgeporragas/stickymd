# ADR-037 — The remappable set is four formatting chords, and closed
Status:     Accepted
Date:       2026-09-06
Context:    `MASTER.md § In Scope` promises "a small set of remappable keyboard shortcuts". Only the global new-note chord was remappable; the four formatting chords were hard-coded. Asked which should be exposed, the founder said he is not a shortcut user and the basic ones are fine, leaving the choice here.
Decision:   Bold, italic, inline code and strikethrough become settable, alongside the new-note chord already there. Nothing else. The set is fixed in code rather than open-ended.
Consequences: Close and quit are deliberately not exposed. A mis-set chord on either is hard to recover from — the window it would close is the one you would fix it in — and neither is a chord anyone needs to move.
            The set is named fields rather than a map, in both the settings file and the window. A map would let a settings file name a command that does not exist, and the failure would be silence.
            The chords live in a CodeMirror `Compartment`, so a change is swapped into open editors rather than waiting for the next window. A shortcut that only applied to windows opened afterwards is a setting that appears not to work, which is the same reason the theme is broadcast.
            Remapping earns its keep less for the founder than for the audience: `Mod-` is the platform abstraction, and `docs/FIXES.md` already records a chord lost to AltGr on his own layout. Someone whose layout collides now has somewhere to go.
            A chord CodeMirror cannot parse simply never fires. That is why the settings window exists as the place to type one, and it is also a gap: nothing validates the string yet. Worth knowing before someone types nonsense into it.
