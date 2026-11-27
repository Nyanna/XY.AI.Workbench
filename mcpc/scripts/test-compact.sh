#!/usr/bin/env bash
# Compact wrapper around `pytest` (token-saving for agents).
#
# Success: one line with the number of passed tests (+ warning hint if any)
# Failure: summary (failed tests, errors, warnings) + path to the full dump
#
# Usage: scripts/test-compact.sh [pytest-args...]   e.g. `-k human_in_the_loop` or `-x`
# Env:   MAX_LINES (default 15)  max. lines per summary section
#        DUMP_DIR  (default ${TMPDIR:-/tmp})

set -u
export PYTHONDONTWRITEBYTECODE=1
export PYTHONPATH="src${PYTHONPATH:+:${PYTHONPATH}}"
export NO_COLOR=1

MAX_LINES="${MAX_LINES:-15}"
DUMP_DIR="${DUMP_DIR:-${TMPDIR:-/tmp}}"

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 2

work="$(mktemp -d "$DUMP_DIR/pytest-XXXXXX")" || exit 2
out="$work/stdout.txt"
err="$work/stderr.txt"

python3 -m pytest tests -q --no-header --color=no -rfE -p no:cacheprovider "$@" >"$out" 2>"$err"
rc=$?

# Counters from the final summary line, e.g. "1 failed, 12 passed, 2 warnings in 0.5s"
summary="$(grep -E ' in [0-9.]+s' "$out" | tail -n 1)"
count() { # $1=regex of the counter label
  grep -oE "[0-9]+ $1" <<<"$summary" | head -n 1 | grep -oE '^[0-9]+' || echo 0
}
passed=$(count 'passed')
failed=$(count 'failed')
errors=$(count 'errors?')
skipped=$(count 'skipped')
warned=$(count 'warnings?')

# "FAILED path::test - reason" / "ERROR path - reason" from the short test summary
failed_tests() {
  grep -E '^(FAILED|ERROR) ' "$out" | cut -c1-200 | awk '!seen[$0]++'
}

# Entries of pytest's warnings summary (indented "file:line: XWarning: msg" lines)
warnings_list() {
  awk '
    /^=+ warnings summary =+$/ { on = 1; next }
    /^=+ .* =+$/               { on = 0 }
    on && /^ +.*:[0-9]+: [A-Za-z]*Warning/ { sub(/^ +/, ""); print }
  ' "$out" | cut -c1-200 | awk '!seen[$0]++'
}

section() { # $1=title  $2=content
  [ -z "$2" ] && return
  local total; total=$(grep -c '' <<<"$2")
  echo "$1"
  head -n "$MAX_LINES" <<<"$2" | sed 's/^/  /'
  [ "$total" -gt "$MAX_LINES" ] && echo "  ... +$((total - MAX_LINES)) more"
}

write_dump() {
  dump="$DUMP_DIR/pytest-dump-$(date +%Y%m%d-%H%M%S).log"
  { echo "### EXIT CODE: $rc"; echo "### STDERR"; cat "$err"; echo; echo "### STDOUT"; cat "$out"; } >"$dump"
}

# ---- Success ----
if [ "$rc" -eq 0 ]; then
  msg="OK: $passed tests passed"
  [ "$skipped" -gt 0 ] && msg+=", $skipped skipped"
  if [ "$warned" -gt 0 ]; then
    write_dump
    msg+=" ($warned warnings, details: $dump)"
  fi
  echo "$msg"
  rm -rf "$work"
  exit 0
fi

# ---- Failure ----
write_dump
ft="$(failed_tests)"

if [ "$rc" -eq 5 ]; then
  echo "FAIL (exit 5): no tests collected"
else
  echo "FAIL (exit $rc): $failed failed, $errors errors, $passed passed, $warned warnings"
fi
section "Failed tests:" "$ft"
section "Warnings:" "$(warnings_list)"

# Fallback: nothing structured recognized -> show last output lines
if [ -z "$ft" ]; then
  echo "Unrecognized failure, last output lines:"
  { cat "$out"; cat "$err"; } | tail -n 15 | sed 's/^/  /'
fi

echo "Full details: $dump"
rm -rf "$work"
exit "$rc"
