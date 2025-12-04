#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

git config core.hooksPath githooks
echo "Configured git core.hooksPath to githooks/"
echo "pre-commit hook will now run 'make precommit' on every commit."
