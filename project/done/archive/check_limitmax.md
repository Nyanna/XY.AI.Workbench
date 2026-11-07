Prüfe `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py`.

Ausgabe:
"warning: "Limit of 30 matches reached (total 30); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."

Die Anzahl totaler Treffer ist immer gleich dem Lmit. Ist das ein Bug?
SystemInit:  id=6758d0c7-a24f-4b67-a698-50fc1ed68c68, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: ce0c-e
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search
    pattern: limit|total|Limit of
    include:
    - __init__.py
  intent: survey
  limit: 50
  reason: "Stellen finden, an denen limit und total berechnet werden"
```
/answer ce0c-e allow
Control Request:
```yaml
id: ce0c-f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search
      matches:
      - path: __init__.py
        matches:
        - "limit: Maximum number of matches to return (1..``_MAX_LIMIT``). Applies per item, not per batch."
        - "limit: int = _DEFAULT_LIMIT"
        - "warning: Set if ``limit`` was reached and further matches may exist."
        - "def _run_search(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:"
        - "limit: Maximum number of matching lines to return (1..``_MAX_LIMIT``)."
        - '``limit`` lines.'
        - "SearchError: If limit is not between 1 and ``_MAX_LIMIT``."
        - "if not 1 <= limit <= _MAX_LIMIT:"
        - "raise SearchError(f'limit must be between 1 and {_MAX_LIMIT}.')"
        - "stdout = '\\n'.join(lines[:limit])"
        - "result = _run_search(item.directory, item.pattern, exclude=item.exclude, include=item.include, limit=item.limit)"
        - total_matches = sum((len(m.matches) for m in matches))
        - "if total_matches >= item.limit:"
        - "if item.limit < _MAX_LIMIT:"
        - "hint += f', or raise limit (max {_MAX_LIMIT})'"
      warning: "Limit of 15 matches reached (total 15); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer ce0c-f allow
ReasoningToken: 32
Control Request:
```yaml
id: ce0c-10
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  regexp: "lines\\[:limit\\]|total_matches >= item.limit"
  reason: Quelltext von _run_search und der Warnlogik lesen
```
/answer ce0c-10 allow
Control Request:
```yaml
id: ce0c-11
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: run_search.aEIrpu|iSMaRo
        code: |-
          try:
              result = run_process(cmd)
          except LaunchError as exc:
              raise SearchError(f'Failed to launch: {exc}') from exc
          prefixes = sorted((str(p).rstrip('/') + '/' for p in directory_paths), key=len, reverse=True)
          prefix_pattern = '|'.join((re.escape(p) for p in prefixes))
          stdout = re.sub(f'^(?:{prefix_pattern})', '', result.stdout, flags=re.MULTILINE)
          lines = stdout.splitlines()
          stdout = '\n'.join(lines[:limit])
          return ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr)
      - id: search_one.aOQxbm|dCXGWI
        code: |-
          if total_matches >= item.limit:
              hint = 'narrow pattern, directory or include/exclude filters'
              if item.limit < _MAX_LIMIT:
                  hint += f', or raise limit (max {_MAX_LIMIT})'
              warning = f'Limit of {item.limit} matches reached (total {total_matches}); {hint}.'
          shortened = sum((lm.truncated for m in matches for lm in m.matches))
          error = None
```
/answer ce0c-11 allow
Result Stats: id=6758d0c7-a24f-4b67-a698-50fc1ed68c68, total: 3539, in: 6, out: 1066, reason: 0, read: 19174, write: 2467, start: 1793918833093, end: 1793918838698, time: 5545, total: 14310, io: 6, control: 60, sent: 0, recv: 146
Ja, das ist ein Bug (genauer: ein Designfehler). „total“ ist nie aussagekräftig.

**Ursache**
- In `_run_search` wird die Ausgabe schon vor der Auswertung abgeschnitten: `stdout = '\n'.join(lines[:limit])`.
- In `search_one` ist `total_matches = sum(len(m.matches) for m in matches)`. Das zählt nur die bereits gekürzten Zeilen, also höchstens `limit`.
- Die Warnung wird bei `total_matches >= item.limit` ausgelöst. Dann gilt immer `total == limit`, und die Angabe „total 30“ ist bedeutungslos.
- Die Bedingung kann auch bei genau `limit` echten Treffern zutreffen, obwohl nichts abgeschnitten wurde. Das ist ein falscher Alarm.

**Korrekturvorschlag**
1. `_run_search` soll die echte Trefferzahl zurückgeben, zum Beispiel als zusätzliches Feld `total_lines = len(lines)`, oder die Rohzeilen liefern. Das Kürzen passiert dann erst später.
2. `search_one` warnt nur bei `total > limit`.
3. Der Text lautet dann zum Beispiel: `Limit of 30 matches reached (total 142); ...`.

Falls `rg` oder `grep` mit `-m` pro Datei begrenzt, ist die Gesamtzahl nur so genau, wie die Rohausgabe es hergibt. Falls die Ausgabe sehr groß werden kann, wäre als Alternative „mindestens limit+1 Zeilen lesen“ mit der Meldung „mehr als N Treffer“ sinnvoll.

Soll ich die Änderung umsetzen?
Korrekturvorschlag 1 klingt gut
SystemInit:  id=6758d0c7-a24f-4b67-a698-50fc1ed68c68, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 23
Control Request:
```yaml
id: ce0c-12
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  reason: Struktur und IDs der betroffenen Funktionen ermitteln
```
/answer ce0c-12 allow
Control Request:
```yaml
id: ce0c-13
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: OyEiBH|qDejkd
        signature: "'Search tool – recursive extended-regex search for retrieval, for a batch of it…"
      - id: PBZWYD|FmTmX2
        signature: "from dataclasses import dataclass, replace"
      - id: 8Pl1k7|dBAVKD
        signature: "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - id: wTnFQX|MKeuzI
        signature: "__all__ = ["
      - id: gph2jG|n4wV62
        signature: "_NODE_TOOLS = frozenset({'ast_read', 'ast_edit_block', 'ast_edit_marks', 'ast_i…"
      - id: SearchError
        signature: "class SearchError(Exception):"
        docstring: Raised when a search cannot be executed or its output cannot be parsed.
      - id: SearchLineMatch
        signature: "@dataclass(frozen=True) class SearchLineMatch:"
        docstring: A single matching line within a file.
      - id: SearchNodeMatch
        signature: "@dataclass(frozen=True) class SearchNodeMatch:"
        docstring: Matching lines attributed to the most specific enclosing AST node.
      - id: SearchMatch
        signature: "@dataclass(frozen=True) class SearchMatch:"
        docstring: All matches found within a single file. ``nodes`` is set instead of relying on …
      - id: SearchItem
        signature: "@dataclass(frozen=True) class SearchItem:"
        docstring: "One independent search to run. Attributes: directory: Absolute paths of the dir…"
      - id: SearchResult
        signature: "@dataclass(frozen=True) class SearchResult:"
        docstring: "Result of a single search. Attributes: directory: The directory list exactly as…"
      - id: SearchItemError
        signature: "@dataclass(frozen=True) class SearchItemError:"
        docstring: "Error running a single search, mirroring its input for result association."
      - id: SearchBatchResult
        signature: "@dataclass(frozen=True) class SearchBatchResult:"
        docstring: "Result of :func:`search`. Attributes: results: One :class:`SearchResult` per su…"
      - id: make_line_match
        signature: "def _make_line_match(lineno: int, text: str) -> SearchLineMatch:"
      - id: parse_search_stdout
        signature: "def _parse_search_stdout(stdout: str) -> list[SearchMatch]:"
        docstring: "Parse 'path:line:content' stdout into :class:`SearchMatch` objects, grouped by …"
      - id: as_list
        signature: "def _as_list(value: list[str] | str | None) -> list[str]:"
        docstring: Normalize an optional list into a list (empty if ``None``). A bare string is to…
      - id: run_search
        signature: "def _run_search(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:"
        docstring: Recursively search one or more directories for ``pattern`` (extended regexp). A…
      - id: search_one
        signature: "def _search_one(item: SearchItem) -> SearchResult:"
      - id: resolve_nodes
        signature: "def _resolve_nodes(match: SearchMatch, roots: list[str]) -> SearchMatch:"
        docstring: Regroup a file's line matches by the most specific enclosing AST node. ``match`…
      - id: search
        signature: "def search(items: list[SearchItem], *, intent: str=_DEFAULT_INTENT, with_nodes: bool=False) -> SearchBatchResult:"
        docstring: "Run one or more independent searches. Limits apply per item, not per batch. Arg…"
      - id: check_intent
        signature: "def _check_intent(intent: str) -> None:"
      - id: SearchTool
        signature: "class SearchTool(ToolDefinition):"
        children:
        - id: SearchTool.IRSVLT|PCuUvD
          signature: name = 'search'
        - id: SearchTool.g4tkLE|v74Wwu
          signature: "_ITEM_PROPERTIES = {"
        - id: SearchTool.SQiS9d|evREwK
          signature: "_ITEM_REQUIRED = ['directory', 'pattern']"
        - id: SearchTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func: translating the MCP schema to/from the Python API."
      - id: register_search_tool
        signature: "def register_search_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer ce0c-13 allow
Control Request:
```yaml
id: ce0c-14
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    ids:
    - run_search
    - search_one
    - SearchResult
  reason: Vollständigen Code der zu ändernden Funktionen lesen
```
/answer ce0c-14 allow
Control Request:
```yaml
id: ce0c-15
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: run_search
        type: FunctionDef
        code: |-
          def _run_search(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:
              """Recursively search one or more directories for ``pattern`` (extended regexp).

              Args:
                  directory: Absolute paths of the directories to search (each must exist and
                      be a directory).
                  pattern: Extended regular expression (grep -E syntax).
                  exclude: Globs of file names to exclude from the search, if given.
                  include: Globs of file names to include in the search, if given.
                  limit: Maximum number of matching lines to return (1..``_MAX_LIMIT``).

              Returns:
                  ProcessResult with:
                      exit_code: 0 if matches were found, 1 if none were found, >=2 on search error.
                      stdout: Matching lines as 'path:line:content', with ``path`` relative to
                          whichever searched directory it was found under, truncated to at most
                          ``limit`` lines.
                      stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
                      stdout_file: Absolute path to temp file with full stdout if to large.
                      stderr_file: Absolute path to temp file with full stderr if to large.

              Raises:
                  SearchError: If a directory is not absolute.
                  SearchError: If a directory does not exist or is not a directory.
                  SearchError: If no directory is given.
                  SearchError: If pattern is empty.
                  SearchError: If limit is not between 1 and ``_MAX_LIMIT``.
                  SearchError: If search binary cannot be launched.
              """
              directory_paths = normalize_directories([Path(d) for d in _as_list(directory)])
              if not directory_paths:
                  raise SearchError('At least one directory is required.')
              for directory_path in directory_paths:
                  if not directory_path.is_absolute():
                      raise SearchError('directory must be an absolute path.')
                  if not directory_path.is_dir():
                      raise SearchError('Directory not found or not a directory.')
              if not pattern:
                  raise SearchError('pattern must not be empty.')
              if not 1 <= limit <= _MAX_LIMIT:
                  raise SearchError(f'limit must be between 1 and {_MAX_LIMIT}.')
              cmd = ['grep', '--recursive', '--line-number', '--extended-regexp', '--binary-files=without-match', '--color=never']
              for pattern_glob in _as_list(include):
                  cmd.append(f'--include={pattern_glob}')
              for pattern_glob in _as_list(exclude):
                  cmd.append(f'--exclude={pattern_glob}')
              cmd += ['--', pattern, *(str(p) for p in directory_paths)]
              try:
                  result = run_process(cmd)
              except LaunchError as exc:
                  raise SearchError(f'Failed to launch: {exc}') from exc
              prefixes = sorted((str(p).rstrip('/') + '/' for p in directory_paths), key=len, reverse=True)
              prefix_pattern = '|'.join((re.escape(p) for p in prefixes))
              stdout = re.sub(f'^(?:{prefix_pattern})', '', result.stdout, flags=re.MULTILINE)
              lines = stdout.splitlines()
              stdout = '\n'.join(lines[:limit])
              return ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr)
      - id: search_one
        type: FunctionDef
        code: |-
          def _search_one(item: SearchItem) -> SearchResult:
              result = _run_search(item.directory, item.pattern, exclude=item.exclude, include=item.include, limit=item.limit)
              if result.exit_code >= 2:
                  raise SearchError(f'search failed (exit code {result.exit_code}): {result.stderr}')
              matches = _parse_search_stdout(result.stdout)
              total_matches = sum((len(m.matches) for m in matches))
              warning = None
              if total_matches >= item.limit:
                  hint = 'narrow pattern, directory or include/exclude filters'
                  if item.limit < _MAX_LIMIT:
                      hint += f', or raise limit (max {_MAX_LIMIT})'
                  warning = f'Limit of {item.limit} matches reached (total {total_matches}); {hint}.'
              shortened = sum((lm.truncated for m in matches for lm in m.matches))
              error = None
              if shortened:
                  error = f'{shortened} match line(s) exceeded {_MAX_LINE_LENGTH} characters and were shortened in the middle with an ellipsis ({
                      _MAX_LINE_LENGTH // 2} leading and trailing characters kept).'
              return SearchResult(directory=item.directory, matches=matches, warning=warning, error=error)
      - id: SearchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class SearchResult:
              """Result of a single search.

              Attributes:
                  directory: The directory list exactly as given in the input.
                  matches: The matches found, grouped by file (empty if none).
                  warning: Set if ``limit`` was reached and further matches may exist.
                  error: Set if match lines exceeded the maximum length and were shortened with an ellipsis.
              """
              directory: list[str]
              matches: list[SearchMatch]
              warning: str | None = None
              error: str | None = None
```
/answer ce0c-15 allow
Control Request:
```yaml
id: ce0c-16
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: _run_search|ProcessResult\(
    include:
    - '*.py'
    exclude:
    - .venv/**
    - node_modules/**
  reason: Weitere Aufrufer von _run_search prüfen
```
/answer ce0c-16 allow
Control Request:
```yaml
id: ce0c-17
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: src/xy/ai/mcpc/tools/process.py
        matches:
        - "return ProcessResult(exit_code=proc.returncode, stdout=proc.stdout or '', stderr=proc.stderr or '')"
      - path: src/xy/ai/mcpc/tools/search/__init__.py
        matches:
        - "def _run_search(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:"
        - "return ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr)"
        - "result = _run_search(item.directory, item.pattern, exclude=item.exclude, include=item.include, limit=item.limit)"
```
/answer ce0c-17 allow
Control Request:
```yaml
id: ce0c-18
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: run_search
    old_text: "limit: int=_DEFAULT_LIMIT) -> ProcessResult:"
    new_text: "limit: int=_DEFAULT_LIMIT) -> tuple[ProcessResult, int]:"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: run_search
    old_text: |2-
          Returns:
              ProcessResult with:
    new_text: |2-
          Returns:
              Tuple ``(result, total_lines)`` where ``total_lines`` is the number of matching
              lines found before truncation to ``limit``, and ``result`` is a ProcessResult with:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: run_search
    old_text: "    return ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr)"
    new_text: "    return (ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr), len(lines))"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: search_one
    old_text: "    result = _run_search(item.directory,"
    new_text: "    result, total_lines = _run_search(item.directory,"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: search_one
    old_text: |2-
          total_matches = sum((len(m.matches) for m in matches))
          warning = None
          if total_matches >= item.limit:
    new_text: |2-
          total_matches = total_lines
          warning = None
          if total_matches > item.limit:
  reason: _run_search liefert die echte Trefferzahl; search_one warnt nur bei total > limit
```
/answer ce0c-18 allow
ReasoningToken: 126
Control Request:
```yaml
id: ce0c-19
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  reason: Syntax nach der Änderung prüfen
```
/answer ce0c-19 allow
Result Stats: id=6758d0c7-a24f-4b67-a698-50fc1ed68c68, total: 12780, in: 14, out: 3187, reason: 0, read: 59928, write: 9579, start: 1793918906390, end: 1793918910122, time: 3673, total: 36097, io: 0, control: 59, sent: 0, recv: 92
Ich habe Vorschlag 1 umgesetzt, und die Datei lässt sich fehlerfrei kompilieren. Getestet habe ich die Änderung nicht.

- **`_run_search`:** Die Funktion gibt jetzt `(ProcessResult, total_lines)` zurück. `total_lines` ist die Zahl der Treffer vor dem Kürzen auf `limit`. Die Docstring habe ich angepasst. `ProcessResult` selbst habe ich nicht verändert, weil es in `process.py` liegt und von anderen Stellen benutzt wird.
- **`_search_one`:** `total_matches` ist jetzt `total_lines`. Die Warnung erscheint nur noch bei `total > limit`. Sie zeigt dann die echte Anzahl, zum Beispiel „Limit of 30 matches reached (total 142); …“. Bei genau `limit` Treffern gibt es keinen falschen Alarm mehr.

`_run_search` hat nur einen Aufrufer, nämlich `_search_one`. Weitere Aufrufer habe ich im ganzen `mcpc`-Verzeichnis nicht gefunden. Ich habe keine Tests gesucht oder ausgeführt. Wenn es Tests gibt, die den Rückgabewert von `_run_search` oder den Warntext prüfen, müssen sie angepasst werden.
