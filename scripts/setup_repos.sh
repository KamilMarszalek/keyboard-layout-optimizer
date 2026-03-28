#!/usr/bin/env bash
set -euo pipefail

GITHUB_URL="https://github.com/KamilMarszalek/keyboard-layout-optimizer.git"
GITLAB_URL="https://gitlab-stud.elka.pw.edu.pl/kmarsza1/keyboard-layout-optimizer.git"

git rev-parse --is-inside-work-tree >/dev/null

git remote remove origin 2>/dev/null || true
git remote add origin "$GITHUB_URL"
git remote set-url --add --push origin "$GITHUB_URL"
git remote set-url --add --push origin "$GITLAB_URL"

git remote -v