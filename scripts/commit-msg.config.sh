#!/usr/bin/env bash

ENABLED=true

MIN_LENGTH=10
MAX_LENGTH=52

ALLOW_SCOPE=true
ALLOW_BREAKING_CHANGE=true

SCOPE_PATTERN='[[:alnum:]._/-]+'

TYPES=(
  feat
  fix
  docs
  style
  refactor
  perf
  test
  chore
  build
  ci
  revert
)