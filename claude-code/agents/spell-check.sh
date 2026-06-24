#!/bin/bash

INPUT=$(cat)
PROMPT=$(echo "$INPUT" | jq -r '.prompt')

# Hash of current prompt for bypass detection
BYPASS_DIR="/tmp/spell-check-bypass"
mkdir -p "$BYPASS_DIR"
PROMPT_HASH=$(echo "$PROMPT" | sha256sum | cut -d' ' -f1)
BYPASS_FILE="$BYPASS_DIR/$PROMPT_HASH"

# If this prompt was previously blocked and is resent → one-shot bypass
if [ -f "$BYPASS_FILE" ]; then
  rm -f "$BYPASS_FILE"
  exit 0
fi

# LanguageTool API check
RESPONSE=$(curl -s --max-time 5 -X POST http://localhost:8010/v2/check \
  --data-urlencode "language=de-DE" \
  --data-urlencode "text=$PROMPT")

# Check if LanguageTool is reachable
if [ $? -ne 0 ] || [ -z "$RESPONSE" ]; then
  echo "LanguageTool not reachable – prompt blocked." >&2
  exit 2
fi

# Check if response is valid JSON
if ! echo "$RESPONSE" | jq -e . > /dev/null 2>&1; then
  echo "LanguageTool: invalid response – prompt blocked." >&2
  exit 2
fi

MATCHES=$(echo "$RESPONSE" | jq '.matches | length')

if [ "$MATCHES" -gt 0 ]; then
  # Build correction list: error → suggestion(s)
  SUGGESTIONS=$(echo "$RESPONSE" | jq -r '
    .matches[] |
    "• \"" + .context.text[.context.offset:.context.offset+.context.length] +
    "\"  →  " +
    ([ .replacements[:3][].value ] | join(" | "))
  ')

  # Build fully corrected prompt via Python (offset-based replacement, back to front)
  CORRECTED=$(ORIGINAL_TEXT="$PROMPT" MATCHES_JSON="$RESPONSE" python3 -c "
import os, json
text = os.environ['ORIGINAL_TEXT']
response = json.loads(os.environ['MATCHES_JSON'])
matches = sorted(response['matches'], key=lambda m: m['offset'], reverse=True)
for m in matches:
    offset = m['offset']
    length = m['length']
    replacement = m['replacements'][0]['value'] if m['replacements'] else ''
    text = text[:offset] + replacement + text[offset + length:]
print(text)
")

  # Save hash so an identical resend triggers bypass
  touch "$BYPASS_FILE"

  # Output both options
  echo "" >&2
  echo "╔══ Spell Check Errors ($MATCHES found) ══════════════════════" >&2
  echo "" >&2
  echo "$SUGGESTIONS" >&2
  echo "" >&2
  echo "╠══ Corrected Prompt (copy & paste) ═══════════════" >&2
  echo "" >&2
  echo "$CORRECTED" >&2
  echo "" >&2
  echo "╚═════════════════════════════════════════════════════════════" >&2
  echo "" >&2
  echo "→ Correct manually, with copy block, or bypass by sending again" >&2
  exit 2
fi

exit 0
