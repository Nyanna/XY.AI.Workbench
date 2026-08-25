#!/usr/bin/env bash
# Wrapper around "colgrep" that keeps config and index data inside the
# project directory itself (XDG_DATA_HOME/XDG_CONFIG_HOME=<project-directory>/.colgrep),
# so the project directory alone is enough to find/carry its index.
#
# Usage:
#   cgrep.sh init [project-directory]   Build/update the index (defaults to $PWD)
#   cgrep.sh <query-args...>            Run "colgrep <query-args...>" against
#                                        the index rooted at $PWD/.colgrep

set -euo pipefail

if ! command -v colgrep >/dev/null 2>&1; then
    echo "colgrep is not installed or not on PATH." >&2
    exit 1
fi

# --- init: (re-)build the index for a project directory --------------------
init() {
    local project_dir
    project_dir=$(cd "${1:-.}" && pwd)

    export XDG_DATA_HOME="$project_dir/.colgrep"
    export XDG_CONFIG_HOME="$project_dir/.colgrep"

    # Base set of always-ignored paths.
    local ignore_args=(--ignore .colgrep)

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
    # maybe use lightonai/mLateOn for language
    colgrep set-model lightonai/mLateOn
    colgrep settings --parallel 1
    colgrep init "$project_dir" -y
}

# --- query: run any colgrep subcommand/query against $PWD/.colgrep ---------
query() {
    local project_dir
    project_dir=$(pwd)

    export XDG_DATA_HOME="$project_dir/.colgrep"
    export XDG_CONFIG_HOME="$project_dir/.colgrep"

    colgrep "$@"
}

if [[ "${1:-}" == "init" ]]; then
    shift
    init "${1:-.}"
else
    query "$@"
fi
