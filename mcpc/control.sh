#!/usr/bin/env bash
# control.sh — Human-in-the-loop tool control interface for MCPC
#
# Polls /control/tool in a continuous loop and presents each pending tool-call
# for approval.  The operator may only Allow (default) or Deny with a reason.
# Modification of arguments or results is intentionally not supported.

# ---------------------------------------------------------------------------
# Configuration (override via environment)
# ---------------------------------------------------------------------------
MCPC_HOST="${MCPC_HOST:-127.0.0.1}"
MCPC_PORT="${MCPC_PORT:-9093}"
CONTROL_PATH="${CONTROL_PATH:-/control/tool}"
BASE_URL="http://${MCPC_HOST}:${MCPC_PORT}${CONTROL_PATH}"
POLL_INTERVAL="${POLL_INTERVAL:-1}"   # seconds to wait when no items are pending

# ---------------------------------------------------------------------------
# Terminal colours
# ---------------------------------------------------------------------------
if [[ -t 1 ]]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    DIM='\033[2m'
    RESET='\033[0m'
    JQ_COLOUR="--color-output"
else
    RED='' GREEN='' YELLOW='' CYAN='' BOLD='' DIM='' RESET=''
    JQ_COLOUR="--monochrome-output"
fi

# ---------------------------------------------------------------------------
# Dependency check
# ---------------------------------------------------------------------------
for cmd in curl jq; do
    if ! command -v "$cmd" &>/dev/null; then
        printf 'Error: "%s" is required but not found in PATH.\n' "$cmd" >&2
        exit 1
    fi
done

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
sep() {
    printf "${DIM}"
    printf '─%.0s' {1..72}
    printf "${RESET}\n"
}

# POST to the control endpoint; first argument is the JSON payload (default: {})
poll() {
    curl -s --fail-with-body -X POST \
        -H "Content-Type: application/json" \
        -d "${1:-{\}}" \
        "$BASE_URL" 2>/dev/null
}

display_item() {
    local item="$1"
    local phase tool_name item_id

    phase=$(jq -r '.phase' <<< "$item")
    tool_name=$(jq -r '.toolName' <<< "$item")
    item_id=$(jq -r '.id' <<< "$item")

    echo
    sep
    printf "  ${BOLD}Tool :${RESET}  ${CYAN}%s${RESET}\n" "$tool_name"
    printf "  ${BOLD}Phase:${RESET}  ${YELLOW}%s${RESET}\n" "$phase"
    printf "  ${BOLD}ID   :${RESET}  ${DIM}%s${RESET}\n"   "$item_id"
    echo

    case "$phase" in
        request)
            local args
            args=$(jq -c '.arguments' <<< "$item")
            if [[ "$args" != "null" && -n "$args" ]]; then
                printf "  ${BOLD}Arguments:${RESET}\n"
                jq $JQ_COLOUR '.' <<< "$args" | sed 's/^/    /'
                echo
            fi
            ;;
        result)
            local res
            res=$(jq -c '.result' <<< "$item")
            if [[ "$res" != "null" && -n "$res" ]]; then
                printf "  ${BOLD}Result:${RESET}\n"
                jq $JQ_COLOUR '.' <<< "$res" | sed 's/^/    /'
                echo
            fi
            ;;
    esac

    sep
}

# Prompt the operator and return a JSON approval payload via stdout.
# All display output goes to stderr so the captured return value is pure JSON.
prompt_decision() {
    local item_id="$1"
    local reason

    printf "\n  ${DIM}Reason to deny — empty to allow:${RESET} " >&2
    read -r reason </dev/tty

    if [[ -z "$reason" ]]; then
        printf "  ${GREEN}✔ Allowed${RESET}\n\n" >&2
        jq -nc --arg id "$item_id" \
            '{"approvals":[{"id":$id}]}'
    else
        printf "  ${RED}✘ Denied${RESET} — %s\n\n" "$reason" >&2
        jq -nc --arg id "$item_id" --arg reason "$reason" \
            '{"approvals":[{"id":$id,"rejected":true,"reason":$reason}]}'
    fi
}

# ---------------------------------------------------------------------------
# Main loop
# ---------------------------------------------------------------------------
clear
trap 'printf "\n${DIM}Aborted.${RESET}\n"; exit 0' INT TERM

printf "\n${BOLD}MCPC Tool Control${RESET}  ${DIM}%s${RESET}\n" "$BASE_URL"
printf "${DIM}Polling for pending tool calls … Press Ctrl+C to quit.${RESET}\n"

while true; do
    response=$(poll '{}') || {
        printf "${DIM}.${RESET}"   # indicate a failed poll without flooding the terminal
        sleep "$POLL_INTERVAL"
        continue
    }

    # Validate JSON
    if ! jq -e . <<< "$response" &>/dev/null; then
        sleep "$POLL_INTERVAL"
        continue
    fi

    # Take the first pending item (process one at a time, re-poll after each)
    item=$(jq -c '.pending[0] // empty' <<< "$response")

    if [[ -z "$item" ]]; then
        sleep "$POLL_INTERVAL"
        continue
    fi

    item_id=$(jq -r '.id' <<< "$item")
    display_item "$item"
    decision=$(prompt_decision "$item_id")

    # Send decision; remaining pending items will appear on the next poll.
    poll "$decision" >/dev/null
    # No sleep — re-poll immediately so the next pending call is shown at once.
done
