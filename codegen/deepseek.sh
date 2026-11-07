#!/bin/bash

export PYTHONDONTWRITEBYTECODE=1

clear && PYTHONPATH=src python3 -m xy.cgen --schema ../libs/openapi/filters/deepseek.filtered.yaml --out ../connector/src --base-package xy.ai.conscious.connector.openapi.deepseek