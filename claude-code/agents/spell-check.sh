#!/bin/bash

INPUT=$(cat)
PROMPT=$(echo "$INPUT" | jq -r '.prompt')

# --- Bypass hash directory ---
BYPASS_DIR="/tmp/spell-check-bypass"
mkdir -p "$BYPASS_DIR"

# Remove bypass hashes older than 6 hours (runs silently in background)
find "$BYPASS_DIR" -maxdepth 1 -type f -mmin +360 -delete &

# --- Bypass via identical resend ---
PROMPT_HASH=$(echo "$PROMPT" | sha256sum | cut -d' ' -f1)
BYPASS_FILE="$BYPASS_DIR/$PROMPT_HASH"

# If this prompt was previously blocked, allow it through once and remove the marker
if [ -f "$BYPASS_FILE" ]; then
  rm -f "$BYPASS_FILE"
  exit 0
fi

# --- LanguageTool API check ---
RESPONSE=$(curl -s --max-time 30 -X POST http://localhost:8010/v2/check \
  --data-urlencode "language=de-DE" \
  --data-urlencode "text=$PROMPT")

# Block if LanguageTool is unreachable
if [ $? -ne 0 ] || [ -z "$RESPONSE" ]; then
  touch "$BYPASS_FILE"
  echo "" >&2
  echo "╔══ Spell Check: LanguageTool not reachable ══════════════════" >&2
  echo "" >&2
  echo "  Resend the prompt unchanged to bypass this check." >&2
  echo "" >&2
  echo "Response: $RESPONSE" >&2
  exit 2
fi

# Block if response is not valid JSON
if ! echo "$RESPONSE" | jq -e . > /dev/null 2>&1; then
  touch "$BYPASS_FILE"
  echo "" >&2
  echo "╔══ Spell Check: invalid LanguageTool response ═══════════════" >&2
  echo "" >&2
  echo "  Resend the prompt unchanged to bypass this check." >&2
  exit 2
fi

MATCHES=$(echo "$RESPONSE" | jq '.matches | length')

if [ "$MATCHES" -gt 0 ]; then
  # Build error list: offending word → suggestion(s)
  SUGGESTIONS=$(echo "$RESPONSE" | jq -r '
    .matches[] |
    "• \"" + .context.text[.context.offset:.context.offset+.context.length] +
    "\"  →  " +
    ([ .replacements[:3][].value ] | join(" | "))
  ')

  # Save hash so an identical resend triggers a one-shot bypass
  touch "$BYPASS_FILE"

  echo "" >&2
  echo "╔══ Spell Check Errors ($MATCHES found) ══════════════════════" >&2
  echo "" >&2
  echo "$SUGGESTIONS" >&2
  exit 2
fi

exit 0
