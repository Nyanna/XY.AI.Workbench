#!/bin/bash
set -euo pipefail

export PYTHONDONTWRITEBYTECODE=1

PKG="xy.ai.conscious.connector.openapi.deepseek"

clear
PYTHONPATH=src python3 -m xy.cgen --schema ../libs/openapi/filters/deepseek.filtered.yaml --out ../connector/src --base-package "$PKG"

"$(dirname "$0")/pack-jar.sh" "$PKG" deepseek-connector.jar ../connector/.bin
