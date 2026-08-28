#!/usr/bin/env bash
# test.sh — Run the MCPC test suite.
#
# Runs pytest against tests/ with src/ on PYTHONPATH (the package is not
# installed). Any extra arguments are forwarded to pytest, e.g.:
#
#   ./test.sh                              # run everything
#   ./test.sh -k human_in_the_loop         # run a subset
#   ./test.sh -x -v                        # stop on first failure, verbose
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"
export PYTHONDONTWRITEBYTECODE=1

PYTHONPATH="src${PYTHONPATH:+:${PYTHONPATH}}" python3 -m pytest tests "$@"
