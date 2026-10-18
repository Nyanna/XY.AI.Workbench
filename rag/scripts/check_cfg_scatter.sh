#!/usr/bin/env bash
# Fails if feature gating appears outside the allow-listed files under src/resident.
set -euo pipefail
cd "$(dirname "$0")/.."

allowed_cuda='^src/resident/backend\.rs:'
allowed_candle='^src/resident/memory/dtype\.rs:'
allowed_mock='^src/resident/(mod|mock|failpoint|kernel/mod|kernel/mock)\.rs:'
allowed_json='^src/resident/telemetry\.rs:'

status=0
check() {
  local feature=$1 allowed=$2 hits
  hits=$(grep -rnE "(cfg\(|cfg!\().*feature *= *\"$feature\"" src/resident | grep -vE "$allowed" || true)
  if [ -n "$hits" ]; then
    echo "feature \"$feature\" used outside its allowed files:" >&2
    echo "$hits" >&2
    status=1
  fi
}
check cuda "$allowed_cuda"
check candle "$allowed_candle"
check mock "$allowed_mock"
check json "$allowed_json"
exit $status
