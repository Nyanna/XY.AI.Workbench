#!/usr/bin/env bash
# Generiert einen Java-Client (openapi-generator, library=native, Jackson 3)
# aus einer OpenAPI-Spec unter libs/openapi/<name>.yaml.
# Existiert filters/<name>.yaml, wird die Spec vorher damit gefiltert.
#
# Usage: ./generate-client.sh <name>
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ $# -ne 1 ]; then
    echo "Usage: $0 <spec-name>" >&2
    exit 1
fi
NAME="$1"

BASE_SPEC="$SCRIPT_DIR/${NAME}.yaml"
FILTER_CONF="$SCRIPT_DIR/filters/${NAME}.yaml"

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

SPEC_FILE="$BASE_SPEC"
if [ -f "$FILTER_CONF" ]; then
    SPEC_FILE="$WORK_DIR/${NAME}.filtered.yaml"
    echo "Filtere Spec mit $FILTER_CONF ..."
    python3 "$SCRIPT_DIR/filter_spec.py" "$FILTER_CONF" "$SPEC_FILE"
	yq -i '(.. | select(type == "!!str")) style="double"' "$SPEC_FILE"
    cp "$SPEC_FILE" "$SCRIPT_DIR/filters/"
fi

if [ ! -f "$SPEC_FILE" ]; then
    echo "Spec nicht gefunden: $SPEC_FILE" >&2
    exit 1
fi

# we use our own generator now
echo "Done with filtering"
exit 0