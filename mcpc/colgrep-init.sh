#!/usr/bin/env bash
# Initialize (or incrementally update) a colgrep index for a project.
#
# Config and index data are stored inside the project directory itself
# (XDG_DATA_HOME/XDG_CONFIG_HOME=<project-directory>), matching the
# convention the "colgrep" MCPC tool uses to find the index - that tool only
# ever searches, it never calls `colgrep init`, so the index must already
# exist when it's used.
#
# Usage: colgrep-init.sh <project-directory>

set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $(basename "$0") <project-directory>" >&2
    exit 1
fi

if ! command -v colgrep >/dev/null 2>&1; then
    echo "colgrep is not installed or not on PATH." >&2
    exit 1
fi

project_dir=$(cd "$1" && pwd)

export XDG_DATA_HOME="$project_dir/.colgrep"
export XDG_CONFIG_HOME="$project_dir/.colgrep"

# -y: auto-confirm indexing of large codebases (non-interactive setup).
colgrep init "$project_dir" -y
