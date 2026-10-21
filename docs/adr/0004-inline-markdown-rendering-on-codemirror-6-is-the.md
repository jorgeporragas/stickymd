# ADR-004 — Inline markdown rendering on CodeMirror 6 is the core requirement
Status:     Accepted
Date:       2026-09-05
Context:    "Renders markdown the way Obsidian does" admits three readings: a split preview pane, a render-on-blur toggle, or true inline rendering where syntax hides on unfocused lines and formatting commands insert real characters. The founder described the third. No native desktop toolkit offers a component that does it.
Decision:   Inline rendering is in scope and non-negotiable, built on CodeMirror 6. This requirement decided the stack.
Consequences: The decoration layer — hiding syntax off the cursor line, revealing it on, keeping commands writing real characters — is the central engineering work of the project, not a step within it. Fenced-code syntax highlighting comes largely free with the engine and was pulled into V1 on that basis.
