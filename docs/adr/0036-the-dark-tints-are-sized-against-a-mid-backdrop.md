# ADR-036 — The dark tints are sized against a mid backdrop, not a white one
Status:     Accepted
Date:       2026-09-06
Context:    Computed against a white wallpaper — the rule since ADR-018 — the dark tints came out strong and nearly opaque, and the founder said they no longer read as a transparent window. His reasoning: someone running a dark desktop mostly has dark things behind the window, so sizing for white pays for a case that rarely happens.
            Measured, he is half right, and the half that is wrong matters. Relaxing to a mid backdrop does buy transparency and lets the colour come down. But at alpha 0.62 the ink falls to **2.36:1** over a white wallpaper, which is not legible — and the compositor blurs the desktop, so a bright photo behind a dark-theme note is an ordinary situation rather than a pathological one.
Decision:   Dark tints hold 4.5:1 against a **mid** backdrop and never fall below **3:1** against white. Alpha is 0.77, which is the most transparent value that keeps that floor; chroma is 0.06.
Consequences: This is a deliberate relaxation of the rule the light tints still follow, and its price is stated rather than buried: a dark-theme note over a bright wallpaper is legible but under AA for body text. Every other case — mid backdrop 4.5:1, dark backdrop 6.3:1 — is at or above it.
            Alpha 0.77 is not taste and should not be nudged without redoing the arithmetic. It is where the 3:1 floor lands, and no choice of chroma rescues a lower one.
            The light tints are untouched. A light theme composites against white anyway, so its worst case and its ordinary case are the same thing, and there is nothing to relax.
            Three passes to get here, each discarded on measurement rather than opinion: pale-and-murky, then maximum chroma (magenta), then strong-and-opaque. The founder's report each time named a symptom, and the numbers named the cause.
