Passe `/home/user/xyan/xy.ai.workbench/mcpc/colgrep-init.sh` an. Ich möchte diesen Wrapper für eine breitere Nutzung auslegen.

- "colgrep-init.sh" soll "cgrep.sh" heißen
- `cgrep init` soll wie bisher colgrep im aktuellen CWD initialisieren optional kann ein anderes Verzeichnis übergeben werden `cgrep init /path/to/dir`
- Ignors werden beim init wie bisher eingefügt
- Ich möchte aber auch cgrep für die Query verwenden können, also soll im fall von `cgrep` ohne init alle folgenden Parameter durchgeschleift werden. Die Umgebungsvariablen für data und config werden auf das "$CWD/.colgrep/" gesetzt. 

Colgrep Readme: `/home/user/Downloads/README.md`
SystemInit:  id=3a8b51d8-9540-4dfb-98d2-8f0d9272b4ae, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 1ef7-1
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/colgrep-init.sh
  reason: "Bestehendes Skript lesen, um es anzupassen"
```
/answer 1ef7-1 allow
Control Request:
```yaml
id: 1ef7-2
toolName: read-file
arguments:
  path: /home/user/Downloads/README.md
  reason: Colgrep README lesen für Env-Var-Namen und CLI-Nutzung
```
/answer 1ef7-2 allow
ReasoningToken: 28
Control Request:
```yaml
id: 1ef7-3
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc
  reason: "Prüfen, ob colgrep-init.sh anderweitig referenziert wird (z.B. in configs)"
```
/answer 1ef7-3 deny Nein wird es nicht
Control Request:
```yaml
id: 1ef7-4
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "colgrep-init" . 2>/dev/null
  reason: Referenzen auf colgrep-init.sh im Repo finden
```
/answer 1ef7-4 deny es gibt keine
ReasoningToken: 76
Text:
 Gut, keine weiteren Referenzen. Ich erstelle `cgrep.sh` neu und lösche `colgrep-init.sh`.
Control Request:
```yaml
id: 1ef7-5
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/cgrep.sh
  mode: replace
  content: |
    #!/usr/bin/env bash
    # Wrapper around "colgrep" that keeps config and index data inside the
    # project directory itself (XDG_DATA_HOME/XDG_CONFIG_HOME=<project-directory>/.colgrep),
    # so the project directory alone is enough to find/carry its index.
    #
    # Usage:
    #   cgrep.sh init [project-directory]   Build/update the index (defaults to $PWD)
    #   cgrep.sh <query-args...>            Run "colgrep <query-args...>" against
    #                                        the index rooted at $PWD/.colgrep

    set -euo pipefail

    if ! command -v colgrep >/dev/null 2>&1; then
        echo "colgrep is not installed or not on PATH." >&2
        exit 1
    fi

    # --- init: (re-)build the index for a project directory --------------------
    init() {
        local project_dir
        project_dir=$(cd "${1:-.}" && pwd)

        export XDG_DATA_HOME="$project_dir/.colgrep"
        export XDG_CONFIG_HOME="$project_dir/.colgrep"

        # Base set of always-ignored paths.
        local ignore_args=(--ignore .colgrep)

        # Additionally honor patterns from every .gitignore found in the project
        # tree (root and subdirectories): skip blank lines and comments, strip a
        # leading "/" (root-anchor) and a trailing "/" (directory marker), and
        # skip negation patterns ("!...") since colgrep's --ignore has no
        # equivalent "un-ignore" semantics. Patterns from a .gitignore located in
        # a subdirectory are prefixed with that subdirectory's path (relative to
        # the project root) so they keep applying only to that subtree, mirroring
        # how git itself scopes nested .gitignore files.
        process_gitignore() {
            local gitignore_file="$1"
            local prefix="$2"

            [[ -f "$gitignore_file" ]] || return 0

            while IFS= read -r line || [[ -n "$line" ]]; do
                line="${line%$'\r'}"
                line="${line#"${line%%[![:space:]]*}"}"
                line="${line%"${line##*[![:space:]]}"}"

                [[ -z "$line" ]] && continue
                [[ "$line" == \#* ]] && continue
                [[ "$line" == \!* ]] && continue

                line="${line#/}"
                line="${line%/}"

                [[ -z "$line" ]] && continue

                if [[ -n "$prefix" ]]; then
                    ignore_args+=(--ignore "$prefix/$line")
                else
                    ignore_args+=(--ignore "$line")
                fi
            done < "$gitignore_file"
        }

        process_gitignore "$project_dir/.gitignore" ""

        while IFS= read -r -d '' nested_gitignore; do
            nested_dir=$(dirname "$nested_gitignore")
            prefix="${nested_dir#"$project_dir"/}"
            process_gitignore "$nested_gitignore" "$prefix"
        done < <(find "$project_dir" -mindepth 2 \
            \( -name .git -o -name .colgrep \) -prune -o \
            -type f -name .gitignore -print0 | sort -z)

        # -y: auto-confirm indexing of large codebases (non-interactive setup).
        colgrep settings "${ignore_args[@]}"
        colgrep settings --parallel 12
        colgrep init "$project_dir" -y
    }

    # --- query: run any colgrep subcommand/query against $PWD/.colgrep ---------
    query() {
        local project_dir
        project_dir=$(pwd)

        export XDG_DATA_HOME="$project_dir/.colgrep"
        export XDG_CONFIG_HOME="$project_dir/.colgrep"

        colgrep "$@"
    }

    if [[ "${1:-}" == "init" ]]; then
        shift
        init "${1:-.}"
    else
        query "$@"
    fi
  reason: Neuen breiter nutzbaren Wrapper cgrep.sh anlegen (init + query pass-through)
```
/answer 1ef7-5 allow
Control Request:
```yaml
id: 1ef7-6
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: chmod +x cgrep.sh && rm -f colgrep-init.sh && ls -la
  reason: "Skript ausführbar machen, alte Datei entfernen, Verzeichnis prüfen"
```
/answer 1ef7-6 allow
Control Request:
```yaml
id: 1ef7-7
result:
  structuredContent:
    stdout: |
      total 108
      drwxrwxr-x  8 user user  4096 Aug 24 11:12 .
      drwxrwxr-x 18 user user  4096 Aug 23 23:04 ..
      -rwxrwxr-x  1 user user  3268 Aug 24 11:12 cgrep.sh
      drwxrwxr-x  2 user user  4096 Jul 21 15:33 .claude
      drwxrwxr-x  3 user user  4096 Aug 23 23:05 .colgrep
      -rwxrwxr-x  1 user user  5124 Jul 28 21:42 control.sh
      -rwxrwxr-x  1 user user   304 Jul  9 21:35 .env.sh
      -rw-rw-r--  1 user user    45 Aug 14 20:02 .gitignore
      drwxrwxr-x  2 user user 32768 Aug 24 11:11 logs
      -rw-rw-r--  1 user user   368 Jul 28 21:42 .project
      -rw-rw-r--  1 user user   459 Jul 28 21:42 .pydevproject
      -rw-rw-r--  1 user user   729 Jul 28 21:42 pyproject.toml
      drwxrwxr-x  3 user user  4096 Jul  9 23:03 .pytest_cache
      -rw-rw-r--  1 user user  6199 Jul 28 21:42 README.md
      drwxrwxr-x  2 user user  4096 Jul 28 21:42 .settings
      drwxrwxr-x  3 user user  4096 Jul  5 19:35 src
      -rwxrwxr-x  1 user user  1161 Jul 28 21:42 start.sh
```
/answer 1ef7-7 allow
Fertig. `cgrep.sh` ersetzt `colgrep-init.sh`:

- `cgrep.sh init [dir]` – wie bisher, initialisiert/aktualisiert den Index für `dir` (Default `.`) inkl. `.gitignore`-basierten Ignores.
- `cgrep.sh <beliebige Query-Args>` – ohne `init` als erstes Argument werden alle Argumente 1:1 an `colgrep` durchgereicht, `XDG_DATA_HOME`/`XDG_CONFIG_HOME` werden dabei auf `$PWD/.colgrep` gesetzt (z. B. `cgrep.sh "database connection pooling"` oder `cgrep.sh status`).
