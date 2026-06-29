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
EXPLICIT_MODEL=false
EXPLICIT_EFFORT=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --profile)
            PROFILE="$2"
            shift 2
            ;;
        --model)
            EXPLICIT_MODEL=true
            EXTRA_ARGS+=("$1" "$2")
            shift 2
            ;;
        --effort)
            EXPLICIT_EFFORT=true
            EXTRA_ARGS+=("$1" "$2")
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
# Resolve script directory for plugin loading
# ---------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export AGENTS_DIR="${SCRIPT_DIR}"

# ---------------------------------------------------------------------------
# Build and run Claude command
# ---------------------------------------------------------------------------
CLAUDE_ARGS=(--system-prompt=\"\" --verbose)

# Array to collect plugin directories for PATH update
PLUGIN_DIRS=()

if [[ -n "$AGENT_ARG" ]]; then
	echo Loading Agent: $AGENT_ARG
    # Agent name equals plugin name
    AGENT_PLUGIN_DIR="${SCRIPT_DIR}/${AGENT_ARG}"
    AGENT_FILE="${AGENT_PLUGIN_DIR}/agents/${AGENT_ARG}.md"

    # Add the main agent plugin directory
    CLAUDE_ARGS+=(--plugin-dir "$AGENT_PLUGIN_DIR")
    PLUGIN_DIRS+=("$AGENT_PLUGIN_DIR")

    if [[ -f "$AGENT_FILE" ]]; then
    	echo Using agent file: $AGENT_FILE
        # Extract frontmatter block (between first pair of ---)
        FRONTMATTER="$(awk '/^---/{if(++n==2) exit; next} n==1' "$AGENT_FILE")"
        echo Frontmatter: $FRONTMATTER

        AGENT_MODEL="$(echo "$FRONTMATTER" | grep -E '^model:[[:space:]]*' | sed 's/^model:[[:space:]]*//' | tr -d '[:space:]')" || true
        AGENT_EFFORT="$(echo "$FRONTMATTER" | grep -E '^effort:[[:space:]]*' | sed 's/^effort:[[:space:]]*//' | tr -d '[:space:]')" || true
        AGENT_PLUGINS="$(echo "$FRONTMATTER" | grep -E '^plugin:[[:space:]]*' | sed 's/^plugin:[[:space:]]*//' | tr -d '[:space:]')" || true

        [[ -n "$AGENT_MODEL"  ]] && [[ "$EXPLICIT_MODEL"  == "false" ]] && CLAUDE_ARGS+=(--model  "$AGENT_MODEL")
        [[ -n "$AGENT_EFFORT" ]] && [[ "$EXPLICIT_EFFORT" == "false" ]] && CLAUDE_ARGS+=(--effort "$AGENT_EFFORT")

        # Add additional plugins defined in frontmatter
        if [[ -n "$AGENT_PLUGINS" ]]; then
        	echo Loading further plugins: $AGENT_PLUGINS
            IFS=',' read -ra PLUGIN_ARRAY <<< "$AGENT_PLUGINS"
            for plugin_name in "${PLUGIN_ARRAY[@]}"; do
                # Trim whitespace
                plugin_name=$(echo "$plugin_name" | xargs)
                if [[ -n "$plugin_name" ]]; then
                    PLUGIN_DIR="${SCRIPT_DIR}/${plugin_name}"
                    CLAUDE_ARGS+=(--plugin-dir "$PLUGIN_DIR")
                    PLUGIN_DIRS+=("$PLUGIN_DIR")
                fi
            done
        fi
    fi

    CLAUDE_ARGS+=(--agent "$AGENT_ARG")
fi

CLAUDE_ARGS+=("${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"}")

echo Updating PATH
# Update PATH with all plugin directories
if [[ ${#PLUGIN_DIRS[@]} -gt 0 ]]; then
    for plugin_dir in "${PLUGIN_DIRS[@]}"; do
        export PATH="${plugin_dir}:${PATH}"
    done
fi

clear
claude "${CLAUDE_ARGS[@]}"
clear
