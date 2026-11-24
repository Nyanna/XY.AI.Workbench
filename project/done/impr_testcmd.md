Ich verwende aktuell `cargo test --quiet` (in `/home/user/xyan/xy.ai.workbench/rag`). Ich brauche ein Bash Script das die Tests ausführt aber die Rückgabe komprimiert. Im Erfolgsfall reicht eine Ausgabe der Anzahl erfolgreicher Tests. Im Fehlerfall soll eine ZUsammenfassung der Warnings und Fehler ausgegeben werden. Die vollen Fehlerdetails sollen in einer Datei im Temp Folder gedumped werden. Die Ausgabe soll dann eine Meldung auf den Dump enthalten.
Ziel ist die Token Reduktion oder im Fehlerfall ein intelligentes vom Agenten gesteuert Handling der Fehler. 

```yaml
id: 6513-188-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    id: runTests
  reason: Kompilation/Tests erneut.
```
/answer 6513-188-0 allow
Control Request:
```yaml
id: 6513-189-188
result:
  structuredContent:
    results:
    - index: 0
      stdout: |2+

        running 2409 tests
        ....................................................................................... 87/2409
        ....................................................................................... 174/2409
        ....................................................................................... 261/2409
        ....................................................................................... 348/2409
        ....................................................................................... 435/2409
        ....................................................................................... 522/2409
        ....................................................................................... 609/2409
        ....................................................................................... 696/2409
        ....................................................................................... 783/2409
        ....................................................................................... 870/2409
        ....................................................................................... 957/2409
        ....................................................................................... 1044/2409
        ....................................................................................... 1131/2409
        ....................................................................................... 1218/2409
        ....................................................................................... 1305/2409
        ....................................................................................... 1392/2409
        ....................................................................................... 1479/2409
        ....................................................................................... 1566/2409
        ....................................................................................... 1653/2409
        ....................................................................................... 1740/2409
        ....................................................................................... 1827/2409
        ....................................................................................... 1914/2409
        ....................................................................................... 2001/2409
        ....................................................................................... 2088/2409
        ....................................................................................... 2175/2409
        ....................................................................................... 2262/2409
        ....................................................................................... 2349/2409
        ............................................................
        test result: ok. 2409 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.58s

        running 0 tests

        test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        running 2 tests
        ..
        test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

        running 2 tests
        ..
        test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

        running 4 tests
        ....
        test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s

        running 0 tests

        test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

      stderr: |+
        warning: function `median` is never used
          --> src/resident/l130/l120/l110/l100/l090/a/test/calibration_store.rs:80:4
           |
        80 | fn median(samples: &[f64]) -> f64 {
           |    ^^^^^^
           |
           = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

        warning: function `median` is never used
          --> src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/a/test/calibration_run.rs:38:4
           |
        38 | fn median(samples: &[f64]) -> f64 {
           |    ^^^^^^
```
```yaml
id: 6513-1cf-1ce
result:
  structuredContent:
    results:
    - index: 0
      stdout: |2+

        running 2418 tests
        ....................................................................................... 87/2418
        ....................................................................................... 174/2418
        ....................................................................................... 261/2418
        ....................................................................................... 348/2418
        ....................................................................................... 435/2418
        ....................................................................................... 522/2418
        ....................................................................................... 609/2418
        ....................................................................................... 696/2418
        ....................................................................................... 783/2418
        ....................................................................................... 870/2418
        ....................................................................................... 957/2418
        ....................................................................................... 1044/2418
        ....................................................................................... 1131/2418
        ....................................................................................... 1218/2418
        ....................................................................................... 1305/2418
        ....................................................................................... 1392/2418
        ....................................................................................... 1479/2418
        ....................................................................................... 1566/2418
        ....................................................................................... 1653/2418
        ....................................................................................... 1740/2418
        ....................................................................................... 1827/2418
        ....................................................................................... 1914/2418
        ....................................................................................... 2001/2418
        ....................................................................................... 2088/2418
        ....................................................................................... 2175/2418
        ....................................................................................... 2262/2418
        ....................................................................................... 2349/2418
        .....................................................................
        test result: ok. 2418 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.53s

        running 0 tests

        test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        running 2 tests
        ..
        test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

        running 2 tests
        ..
        test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        running 4 tests
        ....
        test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

        running 0 tests

        test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
```yaml
id: 6513-184-183
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 101
      stdout: ""
      stderr: |
        error[E0609]: no field `stats` on type `&Runner<'a>`
           --> src/resident/l130/l120/l110/l100/l090/l080/l070/l060/l050/l040/l030/a/calibration_run.rs:309:26
            |
        309 |                         .stats
            |                          ^^^^^ unknown field
            |
            = note: available fields are: `backend`, `cfg`, `seq`, `start`, `exhausted` ... and 3 others

        For more information about this error, try `rustc --explain E0609`.
        error: could not compile `xy-ai-rag` (lib) due to 1 previous error
        error: could not compile `xy-ai-rag` (lib test) due to 1 previous error
```
SystemInit:  id=2c6819c6-12ea-4d39-b2f0-ec63b22f19eb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 2680
Control Request:
```yaml
id: 6513-1d0-0
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/scripts/test-compact.sh
    mode: replace
    content: |
      #!/usr/bin/env bash
      # Kompakter Wrapper um `cargo test --quiet` (Token-sparend für Agenten).
      #
      # Erfolg : eine Zeile mit Anzahl erfolgreicher Tests (+ ggf. Warning-Hinweis)
      # Fehler : Zusammenfassung (Errors, Warnings, fehlgeschlagene Tests) + Pfad zum Volldump
      #
      # Aufruf : scripts/test-compact.sh [cargo-test-args...]   z.B. `-- --test-threads=4` oder `my_filter`
      # Env    : MAX_LINES (Default 15) max. Zeilen pro Abschnitt
      #          DUMP_DIR  (Default ${TMPDIR:-/tmp})

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

      # Zähler aus allen "test result:"-Zeilen (lib, bins, integration tests, doctests)
      read -r passed failed ignored < <(
        awk '/^test result:/ {
               for (i = 1; i <= NF; i++) {
                 if ($i == "passed;")  p  += $(i-1)
                 if ($i == "failed;")  f  += $(i-1)
                 if ($i == "ignored;") ig += $(i-1)
               }
             } END { print p+0, f+0, ig+0 }' "$out"
      )

      # Rustc-Diagnostics aus stderr: "E|msg @ file:line" bzw. "W|msg @ file:line" (dedupliziert)
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

      # Fehlgeschlagene Tests (Panic-Meldungen) aus stdout
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

      section() { # $1=Titel  $2=Inhalt
        [ -z "$2" ] && return
        local total; total=$(grep -c '' <<<"$2")
        echo "$1"
        head -n "$MAX_LINES" <<<"$2" | sed 's/^/  /'
        [ "$total" -gt "$MAX_LINES" ] && echo "  ... +$((total - MAX_LINES)) weitere"
      }

      if [ "$rc" -eq 0 ]; then
        msg="OK: $passed Tests erfolgreich"
        [ "$ignored" -gt 0 ] && msg+=", $ignored ignoriert"
        if [ "$n_warn" -gt 0 ]; then
          dump="$DUMP_DIR/cargo-test-dump-$(date +%Y%m%d-%H%M%S).log"
          { echo "### STDERR"; cat "$err"; echo; echo "### STDOUT"; cat "$out"; } >"$dump"
          msg+=" ($n_warn Warnings, Details: $dump)"
        fi
        echo "$msg"
        rm -rf "$work"
        exit 0
      fi

      # ---- Fehlerfall ----
      dump="$DUMP_DIR/cargo-test-dump-$(date +%Y%m%d-%H%M%S).log"
      { echo "### EXIT CODE: $rc"; echo "### STDERR"; cat "$err"; echo; echo "### STDOUT"; cat "$out"; } >"$dump"

      ft="$(failed_tests)"
      if [ "$n_err" -gt 0 ] && [ "$passed" -eq 0 ] && [ "$failed" -eq 0 ]; then
        echo "FAIL (exit $rc): Build-Fehler – $n_err Errors, $n_warn Warnings"
      else
        echo "FAIL (exit $rc): $failed fehlgeschlagen, $passed erfolgreich – $n_err Errors, $n_warn Warnings"
      fi

      section "Errors:"   "$(grep '^E|' <<<"$diag_all" | cut -c3-)"
      section "Failed tests:" "$ft"
      section "Warnings:" "$(grep '^W|' <<<"$diag_all" | cut -c3-)"

      # Fallback: nichts Strukturiertes erkannt -> letzte stderr-Zeilen zeigen
      if [ "$n_err" -eq 0 ] && [ -z "$ft" ]; then
        echo "Unklarer Fehler, letzte stderr-Zeilen:"
        tail -n 15 "$err" | sed 's/^/  /'
      fi

      echo "Volle Details: $dump"
      rm -rf "$work"
      exit "$rc"
  reason: Das kompakte Test-Wrapper-Script im rag-Projekt anlegen.
```
/answer 6513-1d0-0 deny Zielsprache ist englisch
ReasoningToken: 44
Control Request:
```yaml
id: 6513-1d1-0
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/rag/scripts/test-compact.sh
    mode: replace
    content: |
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
  reason: Das Test-Wrapper-Script mit englischen Kommentaren und Ausgaben anlegen.
```
/answer 6513-1d1-0 allow
Control Request:
```yaml
id: 6513-1d2-0
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/rag
    script:
    - chmod +x scripts/test-compact.sh
    - bash -n scripts/test-compact.sh && echo syntax-ok
    - scripts/test-compact.sh; echo "exit=$?"
  reason: "Script ausführbar machen, Syntax prüfen und gegen den aktuellen Stand (Build-Fehler E0609) testen."
```
/answer 6513-1d2-0 allow
Control Request:
```yaml
id: 6513-1d3-1d2
result:
  structuredContent:
    results:
    - index: 0
      stdout: |
        syntax-ok
        OK: 2426 tests passed
        exit=0
```
/answer 6513-1d3-1d2 allow
ReasoningToken: 50
Control Request:
```yaml
id: 6513-1d4-0
toolName: bash
arguments:
  items:
  - cwd: /tmp
    script:
    - d=$(mktemp -d /tmp/fakecargo-XXXXXX)
    - |-
      cat > $d/cargo <<'EOF'
      #!/usr/bin/env bash
      cat >&2 <<'X'
      warning: function `median` is never used
        --> src/a/calibration_store.rs:80:4
         |
      80 | fn median() {}

      error[E0609]: no field `stats` on type `&Runner<'a>`
         --> src/a/calibration_run.rs:309:26
          |
      309 |  .stats

      error: could not compile `xy-ai-rag` (lib test) due to 1 previous error
      X
      exit 101
      EOF
    - |-
      cat > $d/cargo2 <<'EOF'
      #!/usr/bin/env bash
      cat <<'X'
      running 3 tests
      ..F
      failures:

      ---- foo::bar stdout ----

      thread 'foo::bar' panicked at src/foo.rs:10:5:
      assertion failed: x == 1

      failures:
          foo::bar

      test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
      X
      echo 'error: test failed, to rerun pass `--lib`' >&2
      exit 101
      EOF
    - chmod +x $d/cargo $d/cargo2
    - cd /home/user/xyan/xy.ai.workbench/rag
    - echo '--- build error'; PATH=$d:$PATH scripts/test-compact.sh; echo exit=$?
    - mkdir -p $d/b && mv $d/cargo2 $d/b/cargo
    - echo '--- test failure'; PATH=$d/b:$PATH scripts/test-compact.sh; echo exit=$?
    - rm -rf $d
  reason: Fehlerpfad mit einem Fake-`cargo` (Build-Fehler und Test-Failure) verifizieren.
```
/answer 6513-1d4-0 allow
Control Request:
```yaml
id: 6513-1d5-1d4
result:
  structuredContent:
    results:
    - index: 0
      stdout: |
        --- build error
        FAIL (exit 101): build failed - 1 errors, 1 warnings
        Errors:
          error[E0609]: no field `stats` on type `&Runner<'a>` @ src/a/calibration_run.rs:309:26
        Warnings:
          warning: function `median` is never used @ src/a/calibration_store.rs:80:4
        Full details: /tmp/cargo-test-dump-20261124-190048.log
        exit=101
        --- test failure
        FAIL (exit 101): 1 failed, 2 passed - 0 errors, 0 warnings
        Failed tests:
          foo::bar @ src/foo.rs:10:5: assertion failed: x == 1
        Full details: /tmp/cargo-test-dump-20261124-190048.log
        exit=101
```
/answer 6513-1d5-1d4 allow
[xy.ai.req:ClaudeCode:8c1aaf6d-1166-43f7-9249-df241b9339eb]