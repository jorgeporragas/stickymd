# NoteRow

One note in the hub: its title, when it changed, and a delete control. Takes `title`, `when`, `onOpen`, `onDelete`.

The row is not the control — the buttons inside it are. Delete is revealed by hovering the row in CSS rather than through state, which avoids putting a pointer handler on a `div` that would then need an ARIA role it does not deserve.
