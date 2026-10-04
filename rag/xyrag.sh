#!/usr/bin/env bash
# On-demand CLI start of the xy.ai.rag engine.
#
# Usage: xyrag.sh [--root PATH] <query...>
#   --root PATH   Persistence root (.xyrag); default: current working directory (CWD).
#   <query...>    Free text, packed as field "query" into the query object
#                 and sent through the layer pipeline.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

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

cmd=(cargo run --quiet --manifest-path "${SCRIPT_DIR}/Cargo.toml" --bin xyrag -- "query=${query_text}")
if [[ -n "$root" ]]; then
  cmd+=(--root "$root")
fi

exec "${cmd[@]}"
