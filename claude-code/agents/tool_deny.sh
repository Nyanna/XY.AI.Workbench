#!/bin/bash

RULES_FILE="$(dirname "$0")/tool_deny_rules.json"

if [ ! -f "$RULES_FILE" ]; then
  echo "tool_deny: rules file not found: $RULES_FILE" >&2
  exit 0
fi

INPUT=$(cat)
TOOLNAME=$(echo "$INPUT" | jq -r '.tool_name')
COMMAND=$(echo "$INPUT"  | jq -r '.tool_input.command // empty')
SUBAGENT=$(echo "$INPUT" | jq -r '.agent_type')

deny() {
  echo "$1" >&2
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

    # --- Bash(glob) pattern – matched against bash command ---
    elif [[ "$key" =~ ^Bash\((.+)\)$ ]]; then
      local pattern="${BASH_REMATCH[1]}"
      if [ "$TOOLNAME" = "Bash" ] && [[ "$COMMAND" == $pattern ]]; then
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

    # --- Bash(glob) pattern – matched against bash command ---
    elif [[ "$key" =~ ^Bash\((.+)\)$ ]]; then
      local pattern="${BASH_REMATCH[1]}"
      [ "$TOOLNAME" = "Bash" ] && [[ "$COMMAND" == $pattern ]] && return 0

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
check_allow_section "*"  && exit 0
check_allow_section "$SUBAGENT" && exit 0

# 3. Generic fallback deny – tool passed no deny rule but is also not in the allow-list
deny "Tool '$TOOLNAME' is neither allowed or has a specific rule. Abort and ask the user."
