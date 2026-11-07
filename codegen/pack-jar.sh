#!/bin/bash
set -euo pipefail

if [[ $# -lt 2 || $# -gt 3 ]]; then
  echo "Usage: $0 <package> <jar-file> [base-dir=.bin]" >&2
  exit 2
fi

PKG_PATH="${1//.//}"
JAR_FILE="$2"
BASE_DIR="${3:-.bin}"

SRC_DIR="$BASE_DIR/$PKG_PATH"

[[ -d "$SRC_DIR" ]] || { echo "Dir not found: $SRC_DIR" >&2; exit 1; }
[[ -n "$(ls -A "$SRC_DIR")" ]] || { echo "Dir is empty: $SRC_DIR" >&2; exit 1; }
command -v jar >/dev/null || { echo "'jar' not found" >&2; exit 1; }

jar --create --file "$JAR_FILE" -C "$BASE_DIR" "$PKG_PATH"

if ! jar --list --file "$JAR_FILE" | grep -q "^$PKG_PATH/"; then
  echo "JAR check failed" >&2
  exit 1
fi

find "$SRC_DIR" -mindepth 1 -delete

echo "Created: $JAR_FILE ($(jar --list --file "$JAR_FILE" | grep -vc '/$') Files), $SRC_DIR cleared."
