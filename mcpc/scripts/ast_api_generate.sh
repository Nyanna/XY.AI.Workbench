#!/bin/bash

export PYTHONDONTWRITEBYTECODE=1
export PYTHONPATH=../codegen/src

clear && python3 -m "xy.cgen" \
--schema "../ast-engines/openapi.yaml" \
--out "./src" \
--language "python" \
--base-package "xy.ai.mcpc.ast.openapi"