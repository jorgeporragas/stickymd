#!/bin/sh
#
# Fails when a pull request adds a hard-coded colour outside the token layer.
#
# docs/DESIGN.md principle 5: every colour is read through a semantic token.
# Raw values are permitted only in src/lib/tokens/.
#
# This was check 2 of the hand-written pre-commit hook. FLOW 3.0 replaced that
# hook with its own, which runs only FLOW's commit-level checks, so this rule
# moved to CI as a code check: FLOW's checks guard the docs, and the project's
# own checks guard the code. It reports only. It never rewrites or stages anything.
#
# POSIX shell only: it runs on GitHub's ubuntu runner, and should also run
# under Git Bash on Windows and zsh on macOS without modification.
#
# Usage: scripts/check-raw-colours.sh [base-ref]     (default: origin/main)
#
# Only lines the branch ADDS are judged, so a raw colour that already exists
# elsewhere never fails an unrelated change.

base="${1:-origin/main}"

# Three dots: the diff from the merge base, which is what the pull request
# introduces, rather than also counting what main gained since the branch left.
files=$(git diff --name-only --diff-filter=ACM "$base...HEAD" \
        | grep -E '\.(svelte|ts|css)$' \
        | grep -v '^src/lib/tokens/')

hits=""
for f in $files; do
  # `+++ b/path` is a diff header, not an added line.
  if git diff -U0 "$base...HEAD" -- "$f" \
     | grep '^+' \
     | grep -v '^+++' \
     | grep -Eqi '#[0-9a-f]{3}([0-9a-f]{3})?\b|rgba?\(|hsla?\('; then
    hits="$hits $f"
  fi
done

if [ -n "$hits" ]; then
  echo "FLOW: hard-coded colour value added outside src/lib/tokens/:"
  for f in $hits; do echo "    $f"; done
  echo "  Fix: read the colour through a semantic token."
  echo "  See docs/DESIGN.md section 'Token contract'."
  exit 1
fi

exit 0
