#!/usr/bin/env bash
set -euo pipefail

if ! command -v npx >/dev/null 2>&1; then
  echo "npx is required to run markdownlint. Install Node.js/npm first." >&2
  exit 1
fi

CMD=(npx --yes markdownlint-cli2@0.15.0)
PATTERNS=(
  "**/*.md"
  "!target/**"
  "!book/book/**"
  "!node_modules/**"
  "!.cargo-home/**"
  "!.cargo-tmp/**"
  "!.cargo/**"
)

echo "running markdownlint-cli2..."
"${CMD[@]}" "${PATTERNS[@]}"
