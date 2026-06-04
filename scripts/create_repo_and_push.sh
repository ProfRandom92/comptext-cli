#!/usr/bin/env bash
set -euo pipefail

REPO_FULL_NAME="${1:-ProfRandom92/comptext-cli}"
VISIBILITY="${2:-public}"

if ! command -v gh >/dev/null 2>&1; then
  echo "error: GitHub CLI 'gh' is required for automatic repository creation" >&2
  echo "install gh or create the repo manually, then run:" >&2
  echo "  git remote add origin https://github.com/${REPO_FULL_NAME}.git" >&2
  echo "  git push -u origin main" >&2
  exit 1
fi

if [ ! -d .git ]; then
  git init
  git branch -M main
fi

git add .
git commit -m "Initialize CompText CLI scaffold" || true

gh repo create "${REPO_FULL_NAME}" --source=. --remote=origin --push "--${VISIBILITY}"

# Expected local binary after build: ctxt
