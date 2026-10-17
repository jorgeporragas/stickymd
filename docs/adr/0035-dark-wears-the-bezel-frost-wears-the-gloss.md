# ADR-035 — Dark wears the bezel; Frost wears the gloss
Status:     Accepted
Date:       2026-09-06
Context:    The founder found the glossy controls wrong on the dark window and asked for the bezel back — the treatment ADR-026 chose and ADR-027 replaced — as an exception for the dark theme. The complaint is sound: a glossy disc needs light to be glossy about. Frost has plenty. On a dark window the gradient and the specular have nothing to catch, and read as plastic stuck to the surface rather than glass set into it.
Decision:   Dark controls are flat, seated by a bright hairline along the top inside edge and a shade along the bottom. Frost keeps the gloss. Neither theme gains a second control.
Consequences: This is entirely token values. `--gloss-seat` becomes the bezel, `--gloss-specular` becomes transparent so the overlay paints nothing, and the fills become flat colours instead of gradients. No component changed, and no component knows which theme it is in.
            That is `docs/DESIGN.md` principle 8 being cashed rather than asserted: a theme is a complete set of token values and nothing else. The founder asked for a bezel *component*, and a component would have been the wrong shape — two controls to keep in step, diverging the first time one gained a state the other did not.
            The pressed state needed one addition, `--gloss-seat-pressed`, because a bezel inverts when pushed: the light moves to the bottom edge. Applied by the shared treatment rather than by the theme, since pressing is a state and not a palette.
            ADR-026 and ADR-027 both stand. The gloss was not a mistake and the bezel was not a regression; they are the right answer on different grounds, which is what a theme is for.
