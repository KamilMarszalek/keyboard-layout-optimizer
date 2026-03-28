#!/usr/bin/env bash

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
CONFIG_FILE="${COMMIT_MSG_CONFIG:-$REPO_ROOT/scripts/commit-msg.config.sh}"

if [[ ! -f "$CONFIG_FILE" ]]; then
  echo "Missing config file: $CONFIG_FILE"
  exit 1
fi

source "$CONFIG_FILE"

: "${ENABLED:=true}"
: "${MIN_LENGTH:=10}"
: "${MAX_LENGTH:=52}"
: "${ALLOW_SCOPE:=true}"
: "${ALLOW_BREAKING_CHANGE:=true}"
: "${SCOPE_PATTERN:=[[:alnum:]._/-]+}"

if [[ "$ENABLED" != "true" ]]; then
  exit 0
fi

if ! declare -p TYPES >/dev/null 2>&1 || ((${#TYPES[@]} == 0)); then
  echo "Config error: TYPES array is empty."
  exit 1
fi

if (( MIN_LENGTH < 1 )); then
  echo "Config error: MIN_LENGTH must be >= 1."
  exit 1
fi

if (( MAX_LENGTH < MIN_LENGTH )); then
  echo "Config error: MAX_LENGTH must be >= MIN_LENGTH."
  exit 1
fi

for type in "${TYPES[@]}"; do
  if [[ ! "$type" =~ ^[a-z][a-z0-9-]*$ ]]; then
    echo "Config error: invalid commit type '$type'."
    echo "Allowed format for types: lowercase letters, digits, dash; first char must be a letter."
    exit 1
  fi
done

types_regex="$(IFS='|'; echo "${TYPES[*]}")"

scope_regex=""
if [[ "$ALLOW_SCOPE" == "true" ]]; then
  scope_regex="(\\(${SCOPE_PATTERN}\\))?"
fi

breaking_regex=""
if [[ "$ALLOW_BREAKING_CHANGE" == "true" ]]; then
  breaking_regex="!?"
fi

pattern="^(${types_regex})${scope_regex}${breaking_regex}: .{${MIN_LENGTH},${MAX_LENGTH}}$"

commit_msg="$(head -n 1 "$1")"
commit_msg="${commit_msg%$'\r'}"

if [[ ! "$commit_msg" =~ $pattern ]]; then
  printf "\n\033[1;31m[INVALID COMMIT MESSAGE]\033[0m\n"
  printf "--------------------------------\n"
  printf "\033[1mYour message:\033[0m %s\n" "$commit_msg"
  printf "\033[1mAllowed types:\033[0m %s\n" "${TYPES[*]}"
  printf "\033[1mDescription length:\033[0m %s-%s characters\n" "$MIN_LENGTH" "$MAX_LENGTH"
  printf "\033[1mAccepted format:\033[0m type(scope)!: description\n"
  printf "\033[1mExamples:\033[0m\n"
  printf "  feat: add user registration endpoint\n"
  printf "  fix(auth): handle expired jwt token\n"
  printf "  refactor(core)!: remove legacy payment flow\n\n"
  exit 1
fi