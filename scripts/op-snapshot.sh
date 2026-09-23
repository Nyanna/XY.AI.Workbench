#!/usr/bin/env bash
#
# op-snapshot.sh — Snapshot an LLM operation as a commit outside of
#                       HEAD/branch, under refs/llm-ops/*.
#
# Usage:
#   op-snapshot.sh [TARGET_DIR]
#
# TARGET_DIR: path to the git repository, defaults to CWD.
#
# Behavior:
#   - Detects the current HEAD and compares it to the base commit the
#     active snapshot chain was started from.
#   - If HEAD has moved since the last snapshot (i.e. a real commit
#     happened on the branch in the meantime), a NEW chain is started,
#     rooted at the new HEAD. The old chain is left untouched under its
#     own namespace for later inspection.
#   - Otherwise, the snapshot is appended to the existing chain, parented
#     on the previous snapshot commit.
#   - Commit message is fixed to the current date/time (no free text).
#   - Prints the diff against the previous snapshot (or HEAD, if this is
#     the first snapshot of the chain) to stdout.
#   - Prints stable KEY=VALUE lines at the end for the calling hook to parse.

set -euo pipefail

TARGET_DIR="${1:-$(pwd)}"

# --- Preconditions -----------------------------------------------------

if ! GIT_DIR_CHECK=$(git -C "$TARGET_DIR" rev-parse --git-dir 2>&1); then
    echo "Error: '$TARGET_DIR' is not a git repository." >&2
    echo "$GIT_DIR_CHECK" >&2
    exit 1
fi

cd "$TARGET_DIR"

if ! CURRENT_HEAD=$(git rev-parse HEAD 2>/dev/null); then
    echo "Error: no HEAD found (empty repository?)." >&2
    exit 1
fi

BASE_MARKER="refs/llm-ops/_base"

# --- Determine whether the active chain is still valid ------------------

STORED_BASE=""
if git show-ref --verify --quiet "$BASE_MARKER"; then
    STORED_BASE=$(git rev-parse "$BASE_MARKER")
fi

if [ -z "$STORED_BASE" ] || [ "$STORED_BASE" != "$CURRENT_HEAD" ]; then
    if [ -n "$STORED_BASE" ]; then
        echo "HEAD has advanced since the last snapshot (real commit detected)." >&2
        echo "Previous chain base: $STORED_BASE" >&2
        echo "Starting a new chain rooted at: $CURRENT_HEAD" >&2
    else
        echo "No active chain found. Starting new chain rooted at: $CURRENT_HEAD" >&2
    fi
    git update-ref "$BASE_MARKER" "$CURRENT_HEAD"
    STORED_BASE="$CURRENT_HEAD"
fi

CHAIN_ID=$(git rev-parse --short "$STORED_BASE")
NAMESPACE="refs/llm-ops/chain-$CHAIN_ID"

# --- Find the last snapshot within the active chain ----------------------

LAST_N=0
LAST_OP_REF=""

while IFS= read -r ref; do
    [ -z "$ref" ] && continue
    num="${ref##*/op-}"
    if [[ "$num" =~ ^[0-9]+$ ]] && [ "$num" -gt "$LAST_N" ]; then
        LAST_N="$num"
        LAST_OP_REF="$ref"
    fi
done < <(git for-each-ref --format='%(refname)' "${NAMESPACE}/op-*" 2>/dev/null || true)

if [ -n "$LAST_OP_REF" ]; then
    PARENT=$(git rev-parse "$LAST_OP_REF")
else
    PARENT="$STORED_BASE"
fi

N=$((LAST_N + 1))
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
MSG="op-$N: $TIMESTAMP"

# --- Build the snapshot --------------------------------------------------

git add -A
TREE=$(git write-tree)

PARENT_TREE=$(git rev-parse "${PARENT}^{tree}")
if [ "$TREE" = "$PARENT_TREE" ]; then
    echo "No changes since last snapshot ($PARENT) — no new commit created." >&2
    echo "SNAPSHOT_REF="
    echo "SNAPSHOT_COMMIT=$PARENT"
    echo "SNAPSHOT_PARENT=$PARENT"
    echo "SNAPSHOT_CHAIN=$NAMESPACE"
    exit 0
fi

NEW_COMMIT=$(git commit-tree "$TREE" -p "$PARENT" -m "$MSG")

REF_NAME="${NAMESPACE}/op-$N"
git update-ref "$REF_NAME" "$NEW_COMMIT"

# --- Output ---------------------------------------------------------------

echo "----------------------------------------------------------------------"
echo "Snapshot created: $REF_NAME -> $NEW_COMMIT"
echo "Parent:           $PARENT"
echo "Chain base:       $STORED_BASE"
echo "----------------------------------------------------------------------"
git diff "$PARENT" "$NEW_COMMIT"

echo "SNAPSHOT_REF=$REF_NAME"
echo "SNAPSHOT_COMMIT=$NEW_COMMIT"
echo "SNAPSHOT_PARENT=$PARENT"
echo "SNAPSHOT_CHAIN=$NAMESPACE"