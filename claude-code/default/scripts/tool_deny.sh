#!/bin/bash

LOG_FILE="${CLAUDE_PLUGIN_ROOT}/../tool_deny.$(date +%y%m%d).log"
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

# Resolve agent name to its definition file; '*' maps to default agent.
# Agent names are expected in the format 'plugin:agent' – the part before
# the colon is used as the plugin/directory name, the part after as the
# agent/file name.
agent_file() {
  if [ "$1" = "*" ]; then
    printf '%s/../default/agents/default.md' "${CLAUDE_PLUGIN_ROOT}"
  else
    local plugin="${1%%:*}"
    local agent="${1##*:}"
    printf '%s/../%s/agents/%s.md' "${CLAUDE_PLUGIN_ROOT}" "$plugin" "$agent"
  fi
}

DEFAULT_FILE=$(agent_file "*")
if [ ! -f "$DEFAULT_FILE" ]; then
  log "WARN" "EXIT(0) default agent file not found: $DEFAULT_FILE — permitting tool call"
  echo "tool_deny: default agent file not found: $DEFAULT_FILE" >&2
  exit 0
fi

INPUT=$(cat)
TOOLNAME=$(echo "$INPUT" | jq -r '.tool_name')
COMMAND=$(echo "$INPUT"  | jq -r '.tool_input.command // .tool_input.subagent_type  // .tool_input.skill // empty')
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

# Evaluate all rules in one agent+section combination.
#   $1  agent     – agent name or '*' (maps to default.md)
#   $2  section   – redirect | allow | deny
#   $3  label     – log label, e.g. 'check_redirect'
#   $4  log_level – level passed to emit_block / log on match: 'REDIR' | 'DENY' | 'ALLOW'
#   $5  mode      – 'block' → emit_block + return 0 | 'allow' → log + return 0
# Returns 0 when a rule matched, 1 otherwise.
check_section() {
  local agent="$1" section="$2" label="$3" log_level="$4" mode="$5"

  local file
  file=$(agent_file "$agent")
  [ -f "$file" ] || return 1

  local frontmatter
  frontmatter=$(awk 'BEGIN{n=0} /^---/{n++; if(n==2) exit; next} n==1{print}' "$file")

  while IFS=$'\t' read -r key value; do
    # Skip metadata keys starting with '_'
    [[ "$key" == _* ]] && continue
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
        log "$log_level" "EXIT(0) tool=$TOOLNAME agent=$SUBAGENT — matched $label '$agent' key '$key'"
      else
        emit_block "$value" "$log_level"
      fi
      return 0
    fi

  done < <(printf '%s\n' "$frontmatter" \
    | yq -r ".tool_deny.${section} // {} | to_entries | .[] | .key + \"\t\" + .value" 2>/dev/null)

  return 1
}

# 1. Redirect rules – block with specific message
if check_section "*"         "redirect" "check_redirect" "REDIR" "block"; then exit 2; fi
if check_section "$SUBAGENT" "redirect" "check_redirect" "REDIR" "block"; then exit 2; fi

# 2. Allow-list – explicit permit
if check_section "*"         "allow" "check_allowlist" "ALLOW" "allow"; then exit 0; fi
if check_section "$SUBAGENT" "allow" "check_allowlist" "ALLOW" "allow"; then exit 0; fi

# 3. Configured deny rules – specific deny message per tool/agent, before the hardcoded fallback
if check_section "*"         "deny" "check_deny" "DENY" "block"; then exit 2; fi
if check_section "$SUBAGENT" "deny" "check_deny" "DENY" "block"; then exit 2; fi

# 4. Generic fallback deny – no rule matched at all
block "Tool '$TOOLNAME' is neither allowed nor has a redirect or deny rule. Abort and ask the user." "DENY"
