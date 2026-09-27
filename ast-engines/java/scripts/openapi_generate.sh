#!/bin/bash

export PYTHONDONTWRITEBYTECODE=1
export PYTHONPATH=../../codegen/src

clear && python3 -m "xy.cgen" \
--schema "../openapi.yaml" \
--out "./src" \
--base-package "xy.ai.mcpc.ast.openapi"