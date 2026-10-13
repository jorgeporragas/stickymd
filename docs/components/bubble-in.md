# .bubble-in (`src/app.css`)

The arrival of a bubble in a group: out of nothing, scale and opacity, `--dur-quick`. Worn by the insert ring and by the tint palette's discs. The stagger belongs to the caller — an `animation-delay` per item, since only the caller knows the order.

Shared the moment there were two of them, per CLAUDE.md's DRY rule: two hand-written copies of one arrival is how the same gesture ends up half a frame apart in two places. It animates `transform`, so anything needing a transform of its own puts that on a wrapper rather than fighting the keyframe — which is why a ring bubble sits inside a positioning seat. Never blur.
