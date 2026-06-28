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

if ! jq empty "$RULES_FILE" 2>/dev/null; then
  JQ_ERROR=$(jq empty "$RULES_FILE" 2>&1)
  log "ERROR" "EXIT(2) rules file is not valid JSON: $JQ_ERROR"
  echo "tool_deny: rules file is not valid JSON: $JQ_ERROR" >&2
  exit 2
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

# Evaluate all rules in one configuration section.
#   $1  section   – agent_id or '*'
#   $2  jq_root   – jq path prefix for the rule object: '.' | '._deny' | '._allow'
#   $3  label     – log label, e.g. 'check_redirect'
#   $4  log_level – level passed to emit_block / log on match: 'REDIR' | 'DENY' | 'ALLOW'
#   $5  mode      – 'block' → emit_block + return 0 | 'allow' → log + return 0
# Returns 0 when a rule matched, 1 otherwise.
check_section() {
  local section="$1" jq_root="$2" label="$3" log_level="$4" mode="$5"
  #log "DEBUG" "$label: evaluating section '$section'"

  while IFS= read -r key; do
    # Skip metadata keys starting with '_'
    [[ "$key" == _* ]] && continue

    # Fetch the rule value; skip empty placeholders
    local value
    value=$(jq -r --arg s "$section" --arg k "$key" "${jq_root}[\$s][\$k] // empty" "$RULES_FILE")
    [ -z "$value" ] && continue

    local matched=false

    # --- /regex/ pattern – matched against tool name ---
    if [[ "$key" =~ ^/(.+)/$ ]]; then
      [[ "$TOOLNAME" =~ ${BASH_REMATCH[1]} ]] && matched=true

    # --- ToolName(glob) or ToolName(/regex/) pattern – matched against COMMAND ---
    elif [[ "$key" =~ $RE_GLOB_KEY ]]; then
      local tool_part="${BASH_REMATCH[1]}" cmd_pattern="${BASH_REMATCH[2]}"
      if [ "$TOOLNAME" = "$tool_part" ]; then
        if [[ "$cmd_pattern" =~ ^/(.+)/$ ]]; then
          [[ "$COMMAND" =~ ${BASH_REMATCH[1]} ]] && matched=true
        else
          [[ "$COMMAND" == $cmd_pattern ]] && matched=true
        fi
      fi

    # --- Plain tool name – exact match ---
    elif [ "$key" = "$TOOLNAME" ]; then
      matched=true
    fi

    if $matched; then
      if [ "$mode" = "allow" ]; then
        log "$log_level" "EXIT(0) tool=$TOOLNAME agent=$SUBAGENT — matched $label section '$section' key '$key'"
      else
        emit_block "$value" "$log_level"
      fi
      return 0
    fi

  done < <(jq -r --arg s "$section" "${jq_root}[\$s] | keys[]" "$RULES_FILE" 2>/dev/null)

  #log "DEBUG" "$label: no match in section '$section'"
  return 1
}

# 1. Redirect rules – block with specific message
if check_section "*"        "._redirect" "check_redirect"  "REDIR" "block"; then exit 2; fi
if check_section "$SUBAGENT" "._redirect" "check_redirect"  "REDIR" "block"; then exit 2; fi

# 2. Allow-list – explicit permit
if check_section "*"        "._allow" "check_allowlist" "ALLOW" "allow"; then exit 0; fi
if check_section "$SUBAGENT" "._allow" "check_allowlist" "ALLOW" "allow"; then exit 0; fi

# 3. Configured deny rules – specific deny message per tool/agent, before the hardcoded fallback
if check_section "*"        "._deny"  "check_deny"      "DENY"  "block"; then exit 2; fi
if check_section "$SUBAGENT" "._deny" "check_deny"      "DENY"  "block"; then exit 2; fi

# 4. Generic fallback deny – no rule matched at all
block "Tool '$TOOLNAME' is neither allowed nor has a redirect or deny rule. Abort and ask the user." "DENY"
