#!/usr/bin/env bash

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
HOOK_FILE="$REPO_ROOT/.git/hooks/commit-msg"
VALIDATOR_FILE="$REPO_ROOT/scripts/commit-msg.sh"

if [[ ! -f "$VALIDATOR_FILE" ]]; then
  echo "Missing validator script: $VALIDATOR_FILE"
  exit 1
fi

mkdir -p "$(dirname "$HOOK_FILE")"

cat >"$HOOK_FILE" <<'EOF'
#!/usr/bin/env bash

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
exec "$REPO_ROOT/scripts/commit-msg.sh" "$1"
EOF

chmod +x "$HOOK_FILE"

echo "'commit-msg' hook installed successfully."