#!/bin/bash

export PYTHONDONTWRITEBYTECODE=1

clear && PYTHONPATH=src python3 -m xy.cgen --schema ../libs/openapi/filters/deepseek.filtered.yaml --out ../src --base-package xy.ai.workbench.connector.openapi.deepseek