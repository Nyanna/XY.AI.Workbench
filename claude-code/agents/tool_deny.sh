#!/bin/bash

SCRIPT_DIR="$(dirname "$0")"
RULES_FILE="$SCRIPT_DIR/tool_deny_rules.json"
LOG_FILE="$SCRIPT_DIR/tool_deny.log"
RE_GLOB_KEY='^([^(]+)\((.+)\)$'

log() {
  local level="$1"
  shift
  printf '[%s] [%-5s] %s\n' "$(date '+%Y-%m-%d %H:%M:%S')" "$level" "$*" >> "$LOG_FILE"
}

log_json() {
  local label="$1"
  local json="$2"
  log "INFO" "$label"
  printf '%s\n' "$json" | jq '.' >> "$LOG_FILE" 2>/dev/null || printf '%s\n' "$json" >> "$LOG_FILE"
}

if [ ! -f "$RULES_FILE" ]; then
  log "WARN" "EXIT(0) rules file not found: $RULES_FILE — permitting tool call"
  echo "tool_deny: rules file not found: $RULES_FILE" >&2
  exit 0
fi

INPUT=$(cat)
TOOLNAME=$(echo "$INPUT" | jq -r '.tool_name')
COMMAND=$(echo "$INPUT"  | jq -r '.tool_input.command // .tool_input.subagent_type // empty')
SUBAGENT=$(echo "$INPUT" | jq -r '.agent_type')

log "INFO" "--- new invocation --- tool=$TOOLNAME agent=$SUBAGENT${COMMAND:+ command=$COMMAND}"
log_json "INPUT JSON:" "$INPUT"

deny() {
  local reason="$1"
  log "DENY" "EXIT(2) tool=$TOOLNAME agent=$SUBAGENT — $reason"
  echo "$reason" >&2
  exit 2
}

# Evaluate all deny rules for a given agent section (agent_id or '*')
check_section() {
  local section="$1"

  while IFS= read -r key; do
    # Skip metadata keys starting with '_'
    [[ "$key" == _* ]] && continue

    # Fetch the deny reason; skip if empty (placeholder)
    local reason
    reason=$(jq -r --arg s "$section" --arg k "$key" '.[$s][$k] // empty' "$RULES_FILE")
    [ -z "$reason" ] && continue

    # --- /regex/ pattern – matched against tool name ---
    if [[ "$key" =~ ^/(.+)/$ ]]; then
      local pattern="${BASH_REMATCH[1]}"
      if [[ "$TOOLNAME" =~ $pattern ]]; then
        deny "$reason"
      fi

    # --- ToolName(glob) pattern – matched against COMMAND (bash command or subagent_type) ---
    elif [[ "$key" =~ $RE_GLOB_KEY ]]; then
      local tool="${BASH_REMATCH[1]}"
      local pattern="${BASH_REMATCH[2]}"
      if [ "$TOOLNAME" = "$tool" ] && [[ "$COMMAND" == $pattern ]]; then
        deny "$reason"
      fi

    # --- Plain tool name – exact match ---
    elif [ "$key" = "$TOOLNAME" ]; then
      deny "$reason"
    fi

  done < <(jq -r --arg s "$section" '.[$s] | keys[]' "$RULES_FILE" 2>/dev/null)
}

# Check a single allow-list section; returns 0 if the tool is explicitly allowed
check_allow_section() {
  local section="$1"

  while IFS= read -r key; do
    # Skip metadata keys starting with '_'
    [[ "$key" == _* ]] && continue

    # Fetch the allow entry; skip if empty (placeholder)
    local entry
    entry=$(jq -r --arg s "$section" --arg k "$key" '._allow[$s][$k] // empty' "$RULES_FILE")
    [ -z "$entry" ] && continue

    # --- /regex/ pattern – matched against tool name ---
    if [[ "$key" =~ ^/(.+)/$ ]]; then
      local pattern="${BASH_REMATCH[1]}"
      [[ "$TOOLNAME" =~ $pattern ]] && return 0

    # --- ToolName(glob) pattern – matched against COMMAND (bash command or subagent_type) ---
    elif [[ "$key" =~ $RE_GLOB_KEY ]]; then
      local tool="${BASH_REMATCH[1]}"
      local pattern="${BASH_REMATCH[2]}"
      [ "$TOOLNAME" = "$tool" ] && [[ "$COMMAND" == $pattern ]] && return 0

    # --- Plain tool name – exact match ---
    elif [ "$key" = "$TOOLNAME" ]; then
      return 0
    fi

  done < <(jq -r --arg s "$section" '._allow[$s] | keys[]' "$RULES_FILE" 2>/dev/null)

  return 1
}

# 1. Deny rules (specific reasons)
check_section "*"
check_section "$SUBAGENT"

# 2. Allow-list (last resort permit)
if check_allow_section "*"; then
  log "ALLOW" "EXIT(0) tool=$TOOLNAME agent=$SUBAGENT — matched allow-list section '*'"
  exit 0
fi
if check_allow_section "$SUBAGENT"; then
  log "ALLOW" "EXIT(0) tool=$TOOLNAME agent=$SUBAGENT — matched allow-list section '$SUBAGENT'"
  exit 0
fi

# 3. Generic fallback deny – tool passed no deny rule but is also not in the allow-list
deny "Tool '$TOOLNAME' is neither allowed or has a specific rule. Abort and ask the user."
