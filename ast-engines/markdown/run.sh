#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [[ "${1:-}" == "--build" ]]; then
  ./node_modules/.bin/tsc -p .
  shift
fi

exec node src/xy/ai/mcpc/ast/engine/Main.ts "$@"
