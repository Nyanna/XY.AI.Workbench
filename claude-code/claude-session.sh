#!/bin/bash
# claude-session.sh - Manages Claude Code sessions with LanguageTool lifecycle.
#
# Usage: claude-session.sh --profile <personal|work> [agent-name] [--other-claude-flags]
#
# The first non-key-value argument is treated as the Claude agent parameter.
# Key-value pairs (e.g. --profile <value>) are consumed by this script.

set -euo pipefail

LT_COMPOSE_FILE="${HOME}/xyan/xy.ai.workbench/language-tool/docker-compose.yml"
LT_PID_DIR="/tmp/claude-languagetool-pids"

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
PROFILE=""
AGENT_ARG=""
EXTRA_ARGS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --profile)
            PROFILE="$2"
            shift 2
            ;;
        --*)
            # Pass unrecognised flags through to Claude
            EXTRA_ARGS+=("$1")
            shift
            ;;
        *)
            # First positional argument → agent parameter
            if [[ -z "$AGENT_ARG" ]]; then
                AGENT_ARG="$1"
            else
                EXTRA_ARGS+=("$1")
            fi
            shift
            ;;
    esac
done

# ---------------------------------------------------------------------------
# Profile → config directory
# ---------------------------------------------------------------------------
case "$PROFILE" in
    personal)
        CLAUDE_CONFIG_DIR="${HOME}/.claude-personal"
        ;;
    work)
        CLAUDE_CONFIG_DIR="${HOME}/.claude-work"
        ;;
    "")
        echo "Error: --profile is required. Use --profile personal or --profile work." >&2
        exit 1
        ;;
    *)
        echo "Error: Unknown profile '${PROFILE}'. Valid values: personal, work." >&2
        exit 1
        ;;
esac
export CLAUDE_CONFIG_DIR

# ---------------------------------------------------------------------------
# LanguageTool lifecycle management
# ---------------------------------------------------------------------------
lt_is_running() {
    docker compose -f "$LT_COMPOSE_FILE" ps --status running 2>/dev/null \
        | grep -q languagetool
}

LT_ALREADY_RUNNING=false
if lt_is_running; then
    LT_ALREADY_RUNNING=true
fi

cleanup() {
    # Remove our PID file
    rm -f "${LT_PID_DIR}/$$"

    # Only shut down LanguageTool if this session started it
    if [[ "$LT_ALREADY_RUNNING" == "false" ]]; then
        local remaining
        remaining=$(find "$LT_PID_DIR" -maxdepth 1 -type f 2>/dev/null | wc -l)
        if [[ "$remaining" -eq 0 ]]; then
            echo "Stopping LanguageTool..."
            docker compose -f "$LT_COMPOSE_FILE" down
            # Clear stale PID files left by unclean exits
            rm -rf "$LT_PID_DIR"
        fi
    fi
    clear
}
trap cleanup EXIT INT TERM

mkdir -p "$LT_PID_DIR"
echo $$ > "${LT_PID_DIR}/$$"

if [[ "$LT_ALREADY_RUNNING" == "false" ]]; then
    echo "Starting LanguageTool..."
    docker compose -f "$LT_COMPOSE_FILE" up -d
    sleep 2
fi

# ---------------------------------------------------------------------------
# Resolve script directory and export agents path for subagents
# ---------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export AGENTS_DIR="${SCRIPT_DIR}/agents"
export PATH="${AGENTS_DIR}:${PATH}"

# ---------------------------------------------------------------------------
# Build and run Claude command
# ---------------------------------------------------------------------------
CLAUDE_ARGS=(--system-prompt=\"\" --verbose)

if [[ -n "$AGENT_ARG" ]]; then
    CLAUDE_ARGS+=(--agent "$AGENT_ARG")

    AGENT_FILE="${AGENTS_DIR}/${AGENT_ARG}.md"

    if [[ -f "$AGENT_FILE" ]]; then
        # Extract frontmatter block (between first pair of ---)
        FRONTMATTER="$(awk '/^---/{if(++n==2) exit; next} n==1' "$AGENT_FILE")"

        AGENT_MODEL="$(echo "$FRONTMATTER" | grep -E '^model:[[:space:]]*' | sed 's/^model:[[:space:]]*//' | tr -d '[:space:]')"
        AGENT_EFFORT="$(echo "$FRONTMATTER" | grep -E '^effort:[[:space:]]*' | sed 's/^effort:[[:space:]]*//' | tr -d '[:space:]')"

        [[ -n "$AGENT_MODEL"  ]] && CLAUDE_ARGS+=(--model  "$AGENT_MODEL")
        [[ -n "$AGENT_EFFORT" ]] && CLAUDE_ARGS+=(--effort "$AGENT_EFFORT")
    fi
fi

CLAUDE_ARGS+=("${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"}")

clear
claude "${CLAUDE_ARGS[@]}"
clear
