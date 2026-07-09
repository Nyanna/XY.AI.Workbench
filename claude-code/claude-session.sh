#!/bin/bash
# claude-session.sh - Manages Claude Code sessions with LanguageTool lifecycle.
#
# Usage: claude-session.sh --profile <personal|work> [agent-name] [--other-claude-flags]
#
# The first non-key-value argument is treated as the Claude agent parameter.
# Key-value pairs (e.g. --profile <value>) are consumed by this script.

clear
set -euo pipefail

LT_COMPOSE_FILE="${HOME}/xyan/xy.ai.workbench/language-tool/docker-compose.yml"
LT_PID_DIR="/tmp/claude-languagetool-pids"

# ---------------------------------------------------------------------------
# Resolve script directory (needed early for agent listing)
# ---------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=claude-session-list.sh
source "${SCRIPT_DIR}/claude-session-list.sh"

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
PROFILE=""
AGENT_ARG=""
EXTRA_ARGS=()
EXPLICIT_MODEL=false
EXPLICIT_EFFORT=false
EXPLICIT_SESSION_ID=
LIST_AGENTS=false
SET_MCPC_TOOLS=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --list)
            LIST_AGENTS=true
            shift
            ;;
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
        --session-id)
            EXPLICIT_SESSION_ID="$2"
            EXTRA_ARGS+=("$1" "$2")
            shift 2
            ;;
        --resume)
            EXPLICIT_SESSION_ID="$2"
            EXTRA_ARGS+=("$1" "$2")
            shift 2
            ;;
        --mcp-tools)
            SET_MCPC_TOOLS="$2"
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
# List agents and exit (no profile required)
# ---------------------------------------------------------------------------
if [[ "$LIST_AGENTS" == "true" ]]; then
    list_agents
    exit 0
fi

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
}

if [[ "$LT_ALREADY_RUNNING" == "false" ]]; then
    echo "Starting LanguageTool..."
    LT_EXIT=0
    LT_ERROR="$(docker compose -f "$LT_COMPOSE_FILE" up -d 2>&1)" || LT_EXIT=$?
    if [[ $LT_EXIT -ne 0 ]]; then
        echo "Docker Error: $LT_ERROR" >&2
        if echo "$LT_ERROR" | grep -qiE "Cannot connect to the docker daemon|docker daemon is not running|is the docker daemon running|connect: no such file or directory"; then
            echo "Docker is not running. Starting Docker via systemctl --user start docker ..." >&2
            systemctl --user start docker
            echo "Retrying LanguageTool start..."
            docker compose -f "$LT_COMPOSE_FILE" up -d
        else
            echo "Error: LanguageTool failed to start." >&2
            exit $LT_EXIT
        fi
    fi
fi

# Trap und PID-Datei erst aktivieren, nachdem Docker erfolgreich gestartet ist
trap cleanup EXIT INT TERM
mkdir -p "$LT_PID_DIR"
echo $$ > "${LT_PID_DIR}/$$"

# ---------------------------------------------------------------------------
# Resolve script directory for plugin loading
# ---------------------------------------------------------------------------
export AGENTS_DIR="${SCRIPT_DIR}"

# ---------------------------------------------------------------------------
# Build and run Claude command
# ---------------------------------------------------------------------------
CLAUDE_ARGS=(--system-prompt=\"\" --verbose)

# Array to collect plugin directories for PATH update
PLUGIN_DIRS=()

if [[ -n "$AGENT_ARG" ]]; then
	#DEBUG echo Loading Agent: $AGENT_ARG
    # Agent name equals plugin name
    AGENT_PLUGIN_DIR="${SCRIPT_DIR}/${AGENT_ARG}"
    AGENT_FILE="${AGENT_PLUGIN_DIR}/agents/${AGENT_ARG}.md"

    # Add the main agent plugin directory
    CLAUDE_ARGS+=(--plugin-dir "$AGENT_PLUGIN_DIR")
    PLUGIN_DIRS+=("$AGENT_PLUGIN_DIR")

    if [[ -f "$AGENT_FILE" ]]; then
    	#DEBUG echo Using agent file: $AGENT_FILE
        # Extract frontmatter block (between first pair of ---)
        FRONTMATTER="$(awk '/^---/{if(++n==2) exit; next} n==1' "$AGENT_FILE")"
        #DEBUG echo Frontmatter: $FRONTMATTER

        AGENT_MODEL="$(echo "$FRONTMATTER" | grep -E '^model:[[:space:]]*' | sed 's/^model:[[:space:]]*//' | tr -d '[:space:]')" || true
        AGENT_EFFORT="$(echo "$FRONTMATTER" | grep -E '^effort:[[:space:]]*' | sed 's/^effort:[[:space:]]*//' | tr -d '[:space:]')" || true
        AGENT_PLUGINS="$(echo "$FRONTMATTER" | grep -E '^plugin:[[:space:]]*' | sed 's/^plugin:[[:space:]]*//' | tr -d '[:space:]')" || true
		AGENT_THINKING="$(echo "$FRONTMATTER" | grep -E '^thinking:[[:space:]]*' | sed 's/^thinking:[[:space:]]*//' | tr -d '[:space:]')" || true
		
        [[ -n "$AGENT_MODEL"  ]] && [[ "$EXPLICIT_MODEL"  == "false" ]] && CLAUDE_ARGS+=(--model  "$AGENT_MODEL")
        [[ -n "$AGENT_EFFORT" ]] && [[ "$EXPLICIT_EFFORT" == "false" ]] && CLAUDE_ARGS+=(--effort "$AGENT_EFFORT")
        if [[ "$EXPLICIT_EFFORT" == "false" ]] && [[ "$AGENT_THINKING" == "false" ]]; then
    		export MAX_THINKING_TOKENS=0
		fi
		export MCPC_SESSION_ID="${EXPLICIT_SESSION_ID:-$(uuidgen)}"
		export MCPC_TOOLS="${MCPC_TOOLS:-${SET_MCPC_TOOLS:-read}}"
		export MCPC_CC_PROFILE=${PROFILE}
		export CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY=1
		export CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT=0
		export MCP_TOOL_TIMEOUT=86400000
		export CLAUDE_ENABLE_STREAM_WATCHDOG=0
		export CLAUDE_ENABLE_BYTE_WATCHDOG=0
		export CLAUDE_STREAM_IDLE_TIMEOUT_MS=86400000
		export API_FORCE_IDLE_TIMEOUT=0
		export MCP_TIMEOUT=86400000

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

#DEBUG echo Updating PATH
# Update PATH with all plugin directories
if [[ ${#PLUGIN_DIRS[@]} -gt 0 ]]; then
    for plugin_dir in "${PLUGIN_DIRS[@]}"; do
        export PATH="${plugin_dir}:${PATH}"
    done
fi

claude "${CLAUDE_ARGS[@]}"
