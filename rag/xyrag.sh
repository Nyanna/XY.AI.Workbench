#!/usr/bin/env bash
# On-demand CLI-Start der xy.ai.rag Engine.
#
# Usage: xyrag.sh [--root PATH] <query...>
#   --root PATH   Persistenz-Root (.xyrag); Default: aktuelles Arbeitsverzeichnis (CWD).
#   <query...>    Freitext, wird als Feld "query" in das Query-Objekt gepackt
#                 und durch die Layer-Pipeline geschickt.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="${SCRIPT_DIR}/../src"

root=""
query_args=()

usage() {
  echo "Usage: $(basename "$0") [--root PATH] <query...>" >&2
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --root)
      [[ $# -ge 2 ]] || usage
      root="$2"
      shift 2
      ;;
    --root=*)
      root="${1#--root=}"
      shift
      ;;
    -h|--help)
      usage
      ;;
    *)
      query_args+=("$1")
      shift
      ;;
  esac
done

[[ ${#query_args[@]} -gt 0 ]] || usage

query_text="${query_args[*]}"

# Baut das Query-Objekt mit Feld "query" als JSON (sauberes Escaping).
json_query=$(python3 -c '
import json, sys
print(json.dumps({"query": sys.argv[1]}))
' "$query_text")

# Erlaubt den Start ohne vorherige Installation des Packages.
export PYTHONPATH="${SRC_DIR}${PYTHONPATH:+:${PYTHONPATH}}"

cmd=(python3 -m xy.ai.rag.cli --json "$json_query")
if [[ -n "$root" ]]; then
  cmd+=(--root "$root")
fi

exec "${cmd[@]}"
