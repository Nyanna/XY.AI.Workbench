#!/usr/bin/env bash
# Compact wrapper around `cargo test --quiet` (token-saving for agents).
#
# Success: one line with the number of passed tests (+ warning hint if any)
# Failure: summary (errors, failed tests, warnings) + path to the full dump
#
# Usage: scripts/test-compact.sh [cargo-test-args...]   e.g. `my_filter` or `-- --test-threads=4`
# Env:   MAX_LINES (default 15)  max. lines per summary section
#        DUMP_DIR  (default ${TMPDIR:-/tmp})

set -u
export CARGO_TERM_COLOR=never
MAX_LINES="${MAX_LINES:-15}"
DUMP_DIR="${DUMP_DIR:-${TMPDIR:-/tmp}}"

cd "$(dirname "${BASH_SOURCE[0]}")/.." || exit 2

work="$(mktemp -d "$DUMP_DIR/cargo-test-XXXXXX")" || exit 2
out="$work/stdout.txt"
err="$work/stderr.txt"

cargo test --quiet "$@" >"$out" 2>"$err"
rc=$?

# Counters from all "test result:" lines (lib, bins, integration tests, doctests)
read -r passed failed ignored < <(
  awk '/^test result:/ {
         for (i = 1; i <= NF; i++) {
           if ($i == "passed;")  p  += $(i-1)
           if ($i == "failed;")  f  += $(i-1)
           if ($i == "ignored;") ig += $(i-1)
         }
       } END { print p+0, f+0, ig+0 }' "$out"
)

# rustc diagnostics from stderr as "E|msg @ file:line" / "W|msg @ file:line" (deduplicated)
diag() {
  awk '
    function flush() { if (msg != "") { print kind "|" msg; msg = "" } }
    /^error: (test failed|could not compile|aborting)/ { flush(); next }
    /^warning: .*generated [0-9]+ warning/            { flush(); next }
    /^(error|warning)(\[[A-Za-z0-9]+\])?: / {
      flush()
      kind = ($0 ~ /^error/) ? "E" : "W"
      msg = $0
      next
    }
    /^ *--> / { if (msg != "") { print kind "|" msg " @ " $2; msg = "" } next }
    END { flush() }
  ' "$err" | awk '!seen[$0]++'
}
diag_all="$(diag)"
n_err=$(grep -c '^E|' <<<"$diag_all")
n_warn=$(grep -c '^W|' <<<"$diag_all")

# Failed tests (panic messages) from stdout
failed_tests() {
  awk '
    /^---- .* stdout ----$/ { name = $2; next }
    /panicked at/ {
      line = $0
      sub(/.*panicked at /, "", line)
      if (line ~ /:$/) { getline m; line = line " " m }
      print name " @ " line
    }
  ' "$out" | awk '!seen[$0]++'
}

section() { # $1=title  $2=content
  [ -z "$2" ] && return
  local total; total=$(grep -c '' <<<"$2")
  echo "$1"
  head -n "$MAX_LINES" <<<"$2" | sed 's/^/  /'
  [ "$total" -gt "$MAX_LINES" ] && echo "  ... +$((total - MAX_LINES)) more"
}

write_dump() {
  dump="$DUMP_DIR/cargo-test-dump-$(date +%Y%m%d-%H%M%S).log"
  { echo "### EXIT CODE: $rc"; echo "### STDERR"; cat "$err"; echo; echo "### STDOUT"; cat "$out"; } >"$dump"
}

if [ "$rc" -eq 0 ]; then
  msg="OK: $passed tests passed"
  [ "$ignored" -gt 0 ] && msg+=", $ignored ignored"
  if [ "$n_warn" -gt 0 ]; then
    write_dump
    msg+=" ($n_warn warnings, details: $dump)"
  fi
  echo "$msg"
  rm -rf "$work"
  exit 0
fi

# ---- Failure ----
write_dump
ft="$(failed_tests)"

if [ "$passed" -eq 0 ] && [ "$failed" -eq 0 ]; then
  echo "FAIL (exit $rc): build failed - $n_err errors, $n_warn warnings"
else
  echo "FAIL (exit $rc): $failed failed, $passed passed - $n_err errors, $n_warn warnings"
fi

section "Errors:"       "$(grep '^E|' <<<"$diag_all" | cut -c3-)"
section "Failed tests:" "$ft"
section "Warnings:"     "$(grep '^W|' <<<"$diag_all" | cut -c3-)"

# Fallback: nothing structured recognized -> show last stderr lines
if [ "$n_err" -eq 0 ] && [ -z "$ft" ]; then
  echo "Unrecognized failure, last stderr lines:"
  tail -n 15 "$err" | sed 's/^/  /'
fi

echo "Full details: $dump"
rm -rf "$work"
exit "$rc"
