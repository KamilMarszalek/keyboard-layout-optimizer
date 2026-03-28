#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="keyboard-layout-optimizer"
GITHUB_URL="https://github.com/KamilMarszalek/keyboard-layout-optimizer.git"
GITLAB_URL="git@gitlab-stud.elka.pw.edu.pl:kmarsza1/keyboard-layout-optimizer.git"



git remote remove origin 2>/dev/null || true

git remote add origin "$GITHUB_URL"

git remote set-url --add --push origin "$GITHUB_URL"
git remote set-url --add --push origin "$GITLAB_URL"

git remote -v