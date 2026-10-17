# ADR-012 — Themes are token data; there is no plugin or component-replacement API
Status:     Accepted
Date:       2026-09-05
Context:    The founder wanted users to be able to make the interface their own and suggested components should be swappable. Swappable components means a component API — a public contract supported forever, plus third-party code execution inside an app whose pitch is that it is small and does one thing.
Decision:   A theme is a complete set of design token values and nothing else. Themes never replace components, ship CSS, or execute code.
Consequences: Most of the customisation benefit at a fraction of the cost and risk. Forces the token architecture to be correct from the first commit — built-in themes that hardcode values are not a smaller version of a theme system but a rewrite waiting to happen. Recorded as `docs/DESIGN.md` principles 5, 6 and 8.
