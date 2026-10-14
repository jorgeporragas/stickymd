# WindowChrome

The drag region and window controls for any window. Takes `revealed: boolean`, an optional `closeLabel`, optionally `alwaysOnTop` and `onAlwaysOnTop` for the pin — omit those and no pin is rendered, which is how the hub uses it — and optionally a `controls` snippet for controls the window adds before the pin, as a note adds its tint picker.

Implements Principle 2, with one deliberate exception: a pinned note keeps its pin visible even when the chrome recedes. A state you cannot see is a state you cannot trust — chrome recedes, chrome that is carrying information does not. Controls fade via `opacity`, permitted for small non-glass elements.
