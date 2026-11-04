#!/usr/bin/env bash
# Builds the Rust AST engine: runs the tests, then produces the release binary.
# Usage: ./build.sh [--skip-tests]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

SKIP_TESTS=false
for arg in "$@"; do
  case "$arg" in
    --skip-tests) SKIP_TESTS=true ;;
    -h|--help)
      echo "Usage: $0 [--skip-tests]"
      exit 0
      ;;
    *)
      echo "Unknown argument: $arg" >&2
      exit 1
      ;;
  esac
done

if [[ "$SKIP_TESTS" == false ]]; then
  echo "==> cargo test"
  cargo test
fi

echo "==> cargo build --release"
cargo build --release

BIN="./.bin/release/xy_ai_ast_rust"
if [[ -x "$BIN" ]]; then
  echo "==> Built: $SCRIPT_DIR/${BIN#./}"
else
  echo "==> Build finished (binary not at $BIN; check target-dir config)"
fi
