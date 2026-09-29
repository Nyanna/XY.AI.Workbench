#!/bin/bash
set -euo pipefail

BASE_DIR=".bin"
SRC_BASE="src"
PKG_PATH="xy/ai/workbench/connector/openapi/deepseek"
JAR_FILE="${1:-deepseek-connector.jar}"

SRC_DIR="$BASE_DIR/$PKG_PATH"

[[ -d "$SRC_DIR" ]] || { echo "Dir not found: $SRC_DIR" >&2; exit 1; }
[[ -n "$(ls -A "$SRC_DIR")" ]] || { echo "Dir is empty: $SRC_DIR" >&2; exit 1; }
command -v jar >/dev/null || { echo "'jar' not found" >&2; exit 1; }

jar --create --file "$JAR_FILE" -C "$BASE_DIR" "$PKG_PATH"

if ! jar --list --file "$JAR_FILE" | grep -q "^$PKG_PATH/"; then
  echo "JAR check failed" >&2
  exit 1
fi

find "$BASE_DIR/$PKG_PATH" -mindepth 1 -delete

echo "Created: $JAR_FILE ($(jar --list --file "$JAR_FILE" | grep -vc '/$') Files), $SRC_DIR cleared."