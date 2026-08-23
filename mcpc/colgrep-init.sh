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

# Base set of always-ignored paths.
ignore_args=(--ignore .colgrep)

# Additionally honor patterns from every .gitignore found in the project
# tree (root and subdirectories): skip blank lines and comments, strip a
# leading "/" (root-anchor) and a trailing "/" (directory marker), and
# skip negation patterns ("!...") since colgrep's --ignore has no
# equivalent "un-ignore" semantics. Patterns from a .gitignore located in
# a subdirectory are prefixed with that subdirectory's path (relative to
# the project root) so they keep applying only to that subtree, mirroring
# how git itself scopes nested .gitignore files.
process_gitignore() {
    local gitignore_file="$1"
    local prefix="$2"

    [[ -f "$gitignore_file" ]] || return 0

    while IFS= read -r line || [[ -n "$line" ]]; do
        line="${line%$'\r'}"
        line="${line#"${line%%[![:space:]]*}"}"
        line="${line%"${line##*[![:space:]]}"}"

        [[ -z "$line" ]] && continue
        [[ "$line" == \#* ]] && continue
        [[ "$line" == \!* ]] && continue

        line="${line#/}"
        line="${line%/}"

        [[ -z "$line" ]] && continue

        if [[ -n "$prefix" ]]; then
            ignore_args+=(--ignore "$prefix/$line")
        else
            ignore_args+=(--ignore "$line")
        fi
    done < "$gitignore_file"
}

process_gitignore "$project_dir/.gitignore" ""

while IFS= read -r -d '' nested_gitignore; do
    nested_dir=$(dirname "$nested_gitignore")
    prefix="${nested_dir#"$project_dir"/}"
    process_gitignore "$nested_gitignore" "$prefix"
done < <(find "$project_dir" -mindepth 2 \
    \( -name .git -o -name .colgrep \) -prune -o \
    -type f -name .gitignore -print0 | sort -z)

# -y: auto-confirm indexing of large codebases (non-interactive setup).
colgrep settings "${ignore_args[@]}"
colgrep init "$project_dir" -y
