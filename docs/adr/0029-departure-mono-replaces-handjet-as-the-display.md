# ADR-029 — Departure Mono replaces Handjet as the display face
Status:     Accepted
Date:       2026-09-06
Supersedes: ADR-011 (as to the face)
Context:    The founder asked for display alternatives in Redaction's spirit and picked Departure Mono — a monospaced pixel face by Helena Zhang, SIL OFL, verified from the licence file shipped in the release rather than from a directory listing, which is the lesson ADR-011 paid for. It replaces Handjet everywhere: app identity, hub headers, empty states.
Decision:   `--font-display` is `'Departure Mono', ui-monospace, monospace`. Handjet's files, licence and axis tokens are removed rather than left for a face nothing uses.
Consequences: Version 1.500, one 22 kB woff2, vendored whole. Geist and Martian Mono are subset to latin and latin-ext; this one arrives as a single file covering Latin, Cyrillic and Greek, and subsetting it would cost a build step and a dependency to save a few kilobytes.
            The degradation axis goes with Handjet. `ELGR` and `ELSH` made decay a continuous, animatable thing, and ADR-011 counted that as an identity moment the seven-grade approach could not offer. Departure Mono has no axes: degradation is now a static property of the face. That also removes the natural home for STATUS SMD-051, typographic scrambling, which will need a different mechanism if it is still wanted.
            It ships one weight, 400, so `font-synthesis: none` is set on the body. A synthesised bold on a pixel face thickens strokes off the grid it is drawn on and stops looking like pixels. The hub's title had been leaning on Handjet's weight token and briefly rendered faux-bold; it names `--weight-body` now.
            The face is monospaced, which is a real change of voice — terminal rather than constructed-digital. Accepted knowingly, as the tone shift in ADR-011 was.
            `docs/DESIGN.md`'s rule that display type never appears inside a note is unchanged. It was never about which face.
