#!/bin/bash

INPUT=$(cat)

# Skip check when hook fires inside a subagent
AGENT_ID=$(echo "$INPUT" | jq -r '.agent_id // empty')
if [ -n "$AGENT_ID" ]; then
  exit 0
fi

PROMPT=$(echo "$INPUT" | jq -r '.prompt')

# --- Skip single-line prompts starting with "@" (externally injected prompts) ---
if echo "$PROMPT" | grep -qP '^\@[^\n]*$'; then
  exit 0
fi

# --- Skip single-line prompts starting with "/" (skill) ---
if echo "$PROMPT" | grep -qP '^\/[^\n]*$'; then
  exit 0
fi

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

# --- Build AnnotatedText JSON ---
# markup regions are skipped by LanguageTool; text regions are checked.
# Skipped: fenced code blocks, inline code, URLs, file paths, @mentions
ANNOTATED=$(printf '%s' "$PROMPT" | python3 -c '
import sys, re, json

text = sys.stdin.read()

MARKUP_RE = re.compile(
    r"""(```[\s\S]*?```|`[^`]+`|https?://\S+|/\S+|@\S+)"""
)

parts = MARKUP_RE.split(text)
annotation = []
for i, part in enumerate(parts):
    if not part:
        continue
    if i % 2 == 0:
        annotation.append({"text": part})
    else:
        annotation.append({"markup": part, "interpretAs": " "})

print(json.dumps({"annotation": annotation}))
')

# --- LanguageTool API check ---
RESPONSE=$(curl -s --max-time 30 -X POST http://localhost:8010/v2/check \
  --data-urlencode "language=de-DE" \
  --data-urlencode "data=$ANNOTATED")

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

MAX_SHOWN=15

if [ "$MATCHES" -gt 0 ]; then
  # Build error list: offending word → suggestion(s), capped at MAX_SHOWN
  SUGGESTIONS=$(echo "$RESPONSE" | jq -r --argjson max "$MAX_SHOWN" '
    .matches[:$max][] |
    "• \"" + .context.text[.context.offset:.context.offset+.context.length] +
    "\"  →  " +
    ([ .replacements[:3][].value ] | join(" | "))
  ')

  # Save hash so an identical resend triggers a one-shot bypass
  touch "$BYPASS_FILE"

  echo "" >&2
  if [ "$MATCHES" -gt "$MAX_SHOWN" ]; then
    echo "╔══ Spell Check Errors ($MATCHES found, showing first $MAX_SHOWN) ═══" >&2
  else
    echo "╔══ Spell Check Errors ($MATCHES found) ══════════════════════" >&2
  fi
  echo "" >&2
  echo "$SUGGESTIONS" >&2
  exit 2
fi

exit 0
