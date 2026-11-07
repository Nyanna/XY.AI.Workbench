#!/bin/bash
set -euo pipefail

export PYTHONDONTWRITEBYTECODE=1

PKG="xy.ai.conscious.connector.openapi.anthropic"

clear
PYTHONPATH=src python3 -m xy.cgen --schema ../libs/openapi/filters/anthropic.filtered.yaml --out ../connector/src --base-package "$PKG"

"$(dirname "$0")/pack-jar.sh" "$PKG" anthropic-connector.jar ../connector/.bin
