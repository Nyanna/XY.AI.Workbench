#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

SRC_DIR="src"
BIN_DIR=".bin"
LIBS_DIR="libs"
MAIN_CLASS="xy.ai.mcpc.ast.engine.Main"

CP="$BIN_DIR:$(find "$LIBS_DIR" -maxdepth 1 -name '*.jar' ! -name '*-sources.jar' ! -name '*-javadoc.jar' | paste -sd:)"

mkdir -p "$BIN_DIR"

if [[ "${1:-}" == "--build" ]]; then
  find "$SRC_DIR" -name '*.java' > /tmp/ast-engine-sources.txt
  javac -encoding UTF-8 -d "$BIN_DIR" -cp "$CP" @/tmp/ast-engine-sources.txt
  shift
fi

exec java -cp "$CP" "$MAIN_CLASS" "$@"
