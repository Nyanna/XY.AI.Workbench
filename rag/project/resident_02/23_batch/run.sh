#!/usr/bin/env bash
set -euo pipefail

BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export PYTHONPATH="/home/user/xyan/xy.ai.workbench/mcpc/src${PYTHONPATH:+:$PYTHONPATH}"

clear
exec python3 "$BASE_DIR/batch_specs.py" "$@"
