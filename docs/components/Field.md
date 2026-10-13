# Field

One row of the settings window: what the setting is, the control, and anything that went wrong underneath. Takes `label`, `note`, `problem`, `control`.

The problem line lives here rather than in each control, because every setting can fail the same way and a window that reports failures in a different place each time teaches people to look in several places.
