#!/bin/bash
set -euo pipefail

export PYTHONDONTWRITEBYTECODE=1

PKG="xy.ai.mcpc.ast.openapi"

clear
PYTHONPATH=src python3 -m xy.cgen --schema ../ast-engines/openapi.yaml --out ../ast-engines/markdown/src --base-package "$PKG" --language typescript
