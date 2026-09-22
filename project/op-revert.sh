#!/usr/bin/env bash
set -euo pipefail
TARGET_DIR="${1:?target directory required}"
TARGET_COMMIT="${2:?target commit or ref required}"

cd "$TARGET_DIR"
git read-tree --reset -u "$TARGET_COMMIT"
git clean -fd
echo "Working tree reset to $TARGET_COMMIT"