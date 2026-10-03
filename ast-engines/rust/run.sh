#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [[ "${1:-}" == "--build" ]]; then
  cargo build --release
  shift
fi

exec ./.bin/release/xy_ai_ast_rust "$@"
