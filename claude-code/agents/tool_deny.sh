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

# Log + stderr output for a block event; does NOT exit (exit is at the call site)
emit_block() {
  local message="$1"
  local level="$2"
  log "$level" "EXIT(2) tool=$TOOLNAME agent=$SUBAGENT — $message"
  echo "$message" >&2
}

# Convenience wrapper for the generic fallback (call site is the exit point)
block() {
  emit_block "$1" "$2"
  exit 2
}

# Evaluate all redirect rules for a given agent section (agent_id or '*')
check_redirect_section() {
  local section="$1"
  log "INFO" "check_redirect: evaluating section '$section'"

  while IFS= read -r key; do
    # Skip metadata keys starting with '_'
    [[ "$key" == _* ]] && continue

    # Fetch the redirect message; skip if empty (placeholder)
    local message
    message=$(jq -r --arg s "$section" --arg k "$key" '.[$s][$k] // empty' "$RULES_FILE")
    [ -z "$message" ] && continue

    # --- /regex/ pattern – matched against tool name ---
    if [[ "$key" =~ ^/(.+)/$ ]]; then
      local pattern="${BASH_REMATCH[1]}"
      if [[ "$TOOLNAME" =~ $pattern ]]; then
        emit_block "$message" "REDIR"; return 0
      fi

    # --- ToolName(glob) pattern – matched against COMMAND (bash command or subagent_type) ---
    elif [[ "$key" =~ $RE_GLOB_KEY ]]; then
      local tool="${BASH_REMATCH[1]}"
      local pattern="${BASH_REMATCH[2]}"
      if [ "$TOOLNAME" = "$tool" ] && [[ "$COMMAND" == $pattern ]]; then
        emit_block "$message" "REDIR"; return 0
      fi

    # --- Plain tool name – exact match ---
    elif [ "$key" = "$TOOLNAME" ]; then
      emit_block "$message" "REDIR"; return 0
    fi

  done < <(jq -r --arg s "$section" '.[$s] | keys[]' "$RULES_FILE" 2>/dev/null)

  log "INFO" "check_redirect: no match in section '$section'"
  return 1
}

# Evaluate all deny rules for a given agent section (agent_id or '*');
# used as configured fallback before the hardcoded generic deny
check_deny_section() {
  local section="$1"
  log "INFO" "check_deny: evaluating section '$section'"

  while IFS= read -r key; do
    # Skip metadata keys starting with '_'
    [[ "$key" == _* ]] && continue

    # Fetch the deny message; skip if empty (placeholder)
    local message
    message=$(jq -r --arg s "$section" --arg k "$key" '._deny[$s][$k] // empty' "$RULES_FILE")
    [ -z "$message" ] && continue

    # --- /regex/ pattern – matched against tool name ---
    if [[ "$key" =~ ^/(.+)/$ ]]; then
      local pattern="${BASH_REMATCH[1]}"
      if [[ "$TOOLNAME" =~ $pattern ]]; then
        emit_block "$message" "DENY"; return 0
      fi

    # --- ToolName(glob) pattern – matched against COMMAND (bash command or subagent_type) ---
    elif [[ "$key" =~ $RE_GLOB_KEY ]]; then
      local tool="${BASH_REMATCH[1]}"
      local pattern="${BASH_REMATCH[2]}"
      if [ "$TOOLNAME" = "$tool" ] && [[ "$COMMAND" == $pattern ]]; then
        emit_block "$message" "DENY"; return 0
      fi

    # --- Plain tool name – exact match ---
    elif [ "$key" = "$TOOLNAME" ]; then
      emit_block "$message" "DENY"; return 0
    fi

  done < <(jq -r --arg s "$section" '._deny[$s] | keys[]' "$RULES_FILE" 2>/dev/null)

  log "INFO" "check_deny: no match in section '$section'"
  return 1
}

# Check a single allow-list section; returns 0 if the tool is explicitly allowed
check_allowlist_section() {
  local section="$1"
  log "INFO" "check_allowlist: evaluating section '$section'"

  while IFS= read -r key; do
    # Skip metadata keys starting with '_'
    [[ "$key" == _* ]] && continue

    # Fetch the allow-list entry; skip if empty (placeholder)
    local entry
    entry=$(jq -r --arg s "$section" --arg k "$key" '._allow[$s][$k] // empty' "$RULES_FILE")
    [ -z "$entry" ] && continue

    # --- /regex/ pattern – matched against tool name ---
    if [[ "$key" =~ ^/(.+)/$ ]]; then
      local pattern="${BASH_REMATCH[1]}"
      if [[ "$TOOLNAME" =~ $pattern ]]; then
        log "ALLOW" "EXIT(0) tool=$TOOLNAME agent=$SUBAGENT — matched allow-list section '$section' key '$key'"
        return 0
      fi

    # --- ToolName(glob) pattern – matched against COMMAND (bash command or subagent_type) ---
    elif [[ "$key" =~ $RE_GLOB_KEY ]]; then
      local tool="${BASH_REMATCH[1]}"
      local pattern="${BASH_REMATCH[2]}"
      if [ "$TOOLNAME" = "$tool" ] && [[ "$COMMAND" == $pattern ]]; then
        log "ALLOW" "EXIT(0) tool=$TOOLNAME agent=$SUBAGENT — matched allow-list section '$section' key '$key'"
        return 0
      fi

    # --- Plain tool name – exact match ---
    elif [ "$key" = "$TOOLNAME" ]; then
      log "ALLOW" "EXIT(0) tool=$TOOLNAME agent=$SUBAGENT — matched allow-list section '$section' key '$key'"
      return 0
    fi

  done < <(jq -r --arg s "$section" '._allow[$s] | keys[]' "$RULES_FILE" 2>/dev/null)

  log "INFO" "check_allowlist: no match in section '$section'"
  return 1
}

# 1. Redirect rules – block with specific message
if check_redirect_section "*";      then exit 2; fi
if check_redirect_section "$SUBAGENT"; then exit 2; fi

# 2. Allow-list – explicit permit
if check_allowlist_section "*";       then exit 0; fi
if check_allowlist_section "$SUBAGENT"; then exit 0; fi

# 3. Configured deny rules – specific deny message per tool/agent, before the hardcoded fallback
if check_deny_section "*";      then exit 2; fi
if check_deny_section "$SUBAGENT"; then exit 2; fi

# 4. Generic fallback deny – no rule matched at all
block "Tool '$TOOLNAME' is neither allowed nor has a redirect or deny rule. Abort and ask the user." "DENY"
