Ich benötige analog einer ".gitignore" eine ".mcpcignore". Ihr Zweck ist allerdings die Inverse Verwendung. Sie soll diversen MCPC-Tools erlauben Verzeichnisse, Unterverzeichnisse und Dateitypen zu lesen und zu Verwenden. 

- pathspec wurde installiert
- Jede ".mcpcignore" Datei wird ausgehend vom Parameter bis hinaus ins Root ("/") betrachtet. Sie werden dabei umgekehrt kombiniert also zuerst Root dann die Verzeichnisse darunter.
- Eine ".mcpcignore" definiert Invers also negierte Einträge werden ausgeschlossen und somit ignoriert oder gefiltert.
- Die Funktionalität soll über eine eigene Klasse zur Verfügung gestellt werden

Die Funktionalität soll in folgenden Tools verwendet werden um die Ergebnisse und Ausgaben zu filtern:
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py`

Erreicht werden soll also das bei einem Aufruf von List und Search nicht freigegeben Pfade oder Dateien als nicht existent erscheinen.
Das ganze funktioniert analog einem leichtgewichtigen virtuellen Environment. Was der Agent nicht durch Tools sehen kann, existiert für ihn nicht.
SystemInit:  id=30dabdb0-5445-4fa7-a968-d385d89344bc, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 2418-1-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  reason: Understand structure of list and search tools
```
/answer 2418-1-0 allow
Control Request:
```yaml
id: 2418-2-1
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: JrBuUj|ISKZu0
        excert: "\"List tool – returns files below a directory, grouped by relative subdirectory …"
      - id: dvO55c|qVCPEH
        excert: import os
      - id: yTFZQA|eSu5x1
        excert: "from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult"
      - id: ZrSLiQ|rTmMVq
        excert: "__all__ = ["
      - id: ListError
        excert: "class ListError(Exception):"
        docstring: Raised when a directory listing cannot be performed.
      - id: ListItem
        excert: "@dataclass(frozen=True) class ListItem:"
        docstring: "One directory to list. Attributes: path: Absolute directory path to list (must …"
      - id: ListResult
        excert: "@dataclass(frozen=True) class ListResult(ItemResult):"
        docstring: "Result of listing a single directory, mirroring its input path for result assoc…"
        children:
        - id: ListResult.yz0QnR|RouVWm
          excert: "'Result of listing a single directory, mirroring its input path for result asso…"
        - id: ListResult.key
          excert: "def key(self) -> dict[str, Any]:"
        - id: ListResult.payload
          excert: "def payload(self) -> dict[str, Any]:"
      - id: ListItemError
        excert: "@dataclass(frozen=True) class ListItemError(ItemError):"
        docstring: "Error listing a single directory, mirroring its input path for result associati…"
        children:
        - id: ListItemError.6QbbII|opH0om
          excert: "'Error listing a single directory, mirroring its input path for result associat…"
        - id: ListItemError.key
          excert: "def key(self) -> dict[str, Any]:"
      - id: ListBatchResult
        excert: "class ListBatchResult(BatchResult[ListResult, ListItemError]):"
        docstring: "Result of :func:`list`. Attributes: results: One :class:`ListResult` per succes…"
      - id: list_one
        excert: "def _list_one(item: ListItem) -> ListResult:"
        docstring: "List all files below the absolute directory ``item.path``, grouped like ``ls -R…"
      - id: dedupe_hierarchy
        excert: "def _dedupe_hierarchy(items: list[ListItem]) -> list[ListItem]:"
        docstring: Remove items whose path lies below another item's path in the same directory hi…
      - id: list
        excert: "def list(items: list[ListItem]) -> ListBatchResult:"
        docstring: "List each directory in ``items``. Args: items: Directories to list. Must be non…"
      - id: ListTool
        excert: "class ListTool(ToolDefinition):"
        children:
        - id: ListTool.7wRGko|3xW2xR
          excert: name = 'list'
        - id: ListTool.4gWrRr|Coidj4
          excert: "_ITEM_PROPERTIES = {"
        - id: ListTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`list`, translating the MCP schema to/from the Python API."
      - id: register_list_tool
        excert: "def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: OyEiBH|qDejkd
        excert: "'Search tool – recursive extended-regex search for retrieval, for a batch of it…"
      - id: ZXLaUr|alUQX2
        excert: "from dataclasses import dataclass, replace"
      - id: 8Pl1k7|79r0Qn
        excert: "from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult"
      - id: wTnFQX|MKeuzI
        excert: "__all__ = ["
      - id: gph2jG|n4wV62
        excert: "_NODE_TOOLS = frozenset({'ast_read', 'ast_edit_block', 'ast_edit_marks', 'ast_i…"
      - id: SearchError
        excert: "class SearchError(Exception):"
        docstring: Raised when a search cannot be executed or its output cannot be parsed.
      - id: SearchLineMatch
        excert: "@dataclass(frozen=True) class SearchLineMatch:"
        docstring: A single matching line within a file.
      - id: SearchNodeMatch
        excert: "@dataclass(frozen=True) class SearchNodeMatch:"
        docstring: Matching lines attributed to the most specific enclosing AST node.
      - id: SearchMatch
        excert: "@dataclass(frozen=True) class SearchMatch:"
        docstring: All matches found within a single file. ``nodes`` is set instead of relying on …
      - id: SearchItem
        excert: "@dataclass(frozen=True) class SearchItem:"
        docstring: "One independent search to run. Attributes: directory: Absolute paths of the dir…"
      - id: SearchResult
        excert: "@dataclass(frozen=True) class SearchResult(ItemResult):"
        docstring: "Result of a single search. Attributes: directory: The directory list exactly as…"
        children:
        - id: SearchResult.iJIbrz|RbNsuT
          excert: "'Result of a single search.\\n\\n    Attributes:\\n        directory: The director…"
        - id: SearchResult.key
          excert: "def key(self) -> dict[str, Any]:"
      - id: SearchItemError
        excert: "@dataclass(frozen=True) class SearchItemError(ItemError):"
        docstring: "Error running a single search, mirroring its input for result association."
        children:
        - id: SearchItemError.VNAUO2|QXFu2a
          excert: "'Error running a single search, mirroring its input for result association.'"
        - id: SearchItemError.key
          excert: "def key(self) -> dict[str, Any]:"
      - id: SearchBatchResult
        excert: "class SearchBatchResult(BatchResult[SearchResult, SearchItemError]):"
        docstring: "Result of :func:`search`. Attributes: results: One :class:`SearchResult` per su…"
      - id: make_line_match
        excert: "def _make_line_match(lineno: int, text: str) -> SearchLineMatch:"
      - id: parse_search_stdout
        excert: "def _parse_search_stdout(stdout: str) -> list[SearchMatch]:"
        docstring: "Parse 'path:line:content' stdout into :class:`SearchMatch` objects, grouped by …"
      - id: as_list
        excert: "def _as_list(value: list[str] | str | None) -> list[str]:"
        docstring: Normalize an optional list into a list (empty if ``None``). A bare string is to…
      - id: run_search
        excert: "def _run_search(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> tuple[ProcessResult, int]:"
        docstring: Recursively search one or more directories for ``pattern`` (extended regexp). A…
      - id: search_one
        excert: "def _search_one(item: SearchItem) -> SearchResult:"
      - id: resolve_nodes
        excert: "def _resolve_nodes(match: SearchMatch, roots: list[str]) -> SearchMatch:"
        docstring: Regroup a file's line matches by the most specific enclosing AST node. ``match`…
      - id: search
        excert: "def search(items: list[SearchItem], *, intent: str=_DEFAULT_INTENT, with_nodes: bool=False) -> SearchBatchResult:"
        docstring: "Run one or more independent searches. Limits apply per item, not per batch. Arg…"
      - id: check_intent
        excert: "def _check_intent(intent: str) -> None:"
      - id: SearchTool
        excert: "class SearchTool(ToolDefinition):"
        children:
        - id: SearchTool.IRSVLT|PCuUvD
          excert: name = 'search'
        - id: SearchTool.g4tkLE|v74Wwu
          excert: "_ITEM_PROPERTIES = {"
        - id: SearchTool.SQiS9d|8EWkWE
          excert: "_ITEM_REQUIRED = ['directory', 'pattern']"
        - id: SearchTool.handle
          excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func: translating the MCP schema to/from the Python API."
      - id: register_search_tool
        excert: "def register_search_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 2418-2-1 allow
Control Request:
```yaml
id: 2418-3-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: .*
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 50
  reason: Survey package layout
```
/answer 2418-3-0 allow
Control Request:
```yaml
id: 2418-4-3
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/ast/openapi/AppendInfoNodesValidateServer.py
        matches:
        - "\"\"\"Generated by cgen. Transport-agnostic server base class (stdlib `http.server`,"
        - no external framework). Implement one abstract method per operation -- each has
        - the exact same signature (parameters and XxxResponse return type) as the
        - "corresponding client-interface method, so no HTTP type (status code, content"
        - "type, ...) ever appears in your implementation. Build the returned response via"
        - its no-arg constructor plus one of its set_code_<NNN>(...) methods."""
        - from __future__ import annotations
        - import abc
        - import http.server
        - import re
        - import socketserver
        - import typing
        - import urllib.parse
        - import xy.ai.mcpc.ast.openapi.JsonSupport
        - import xy.ai.mcpc.ast.openapi.components.CodeRequest
        - import xy.ai.mcpc.ast.openapi.components.SourceRequest
        - import xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest
        - import xy.ai.mcpc.ast.openapi.request.nodes.post.json.LocateRequest
        - import xy.ai.mcpc.ast.openapi.response.append.AppendResponse
        - import xy.ai.mcpc.ast.openapi.response.info.InfoResponse
        - import xy.ai.mcpc.ast.openapi.response.nodes.NodesResponse
        - import xy.ai.mcpc.ast.openapi.response.nodes.nodeid.NodesNodeIdResponse
        - import xy.ai.mcpc.ast.openapi.response.nodes.nodeid.delete.NodesNodeIdDeleteResponse
        - import xy.ai.mcpc.ast.openapi.response.nodes.nodeid.insert.NodesNodeIdInsertResponse
        - import xy.ai.mcpc.ast.openapi.response.nodes.nodeid.replace.NodesNodeIdReplaceResponse
        - import xy.ai.mcpc.ast.openapi.response.validate.ValidateResponse
        - ""
        - "class AppendInfoNodesValidateServer(abc.ABC):"
        - _PATTERN_APPENDTOPLEVEL = re.compile(r"/append")
        - _PATTERN_GETENGINEINFO = re.compile(r"/info")
        - _PATTERN_LISTNODES = re.compile(r"/nodes")
        - "_PATTERN_GETNODE = re.compile(r\"/nodes/([^/]+)\")"
        - "_PATTERN_DELETENODE = re.compile(r\"/nodes/([^/]+)/delete\")"
        - "_PATTERN_INSERTRELATIVETONODE = re.compile(r\"/nodes/([^/]+)/insert\")"
        - "_PATTERN_REPLACENODE = re.compile(r\"/nodes/([^/]+)/replace\")"
        - _PATTERN_VALIDATESOURCE = re.compile(r"/validate")
        - '"""'
        - Engine.append — append code at the tree's top level. An empty `source` is parsed via Engine.empty_tree (new file case).
        - '@abc.abstractmethod'
        - "def appendTopLevel(self, request: xy.ai.mcpc.ast.openapi.components.CodeRequest):"
        - '...'
        - "Engine metadata (Engine.name, Engine.validates_syntax)."
      warnings:
      - "Limit of 50 matches reached (total 19711); narrow pattern, directory or include/exclude filters."
```
/answer 2418-4-3 allow
Text:
 Das war zu breit. Ich lese die relevanten Funktionen.
Control Request:
```yaml
id: 2418-5-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    ids:
    - dvO55c|qVCPEH
    - list_one
    - dedupe_hierarchy
    - list
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    ids:
    - parse_search_stdout
    - run_search
    - search_one
    - resolve_nodes
    - search
  reason: Read the functions to be modified
```
/answer 2418-5-0 allow
Control Request:
```yaml
id: 2418-6-5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
      nodes:
      - id: dvO55c|qVCPEH
        type: imports
        code: |-
          import os
          import re
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools._directories import normalize_directory
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
      - id: list_one
        type: FunctionDef
        code: |-
          def _list_one(item: ListItem) -> ListResult:
              """List all files below the absolute directory ``item.path``, grouped like ``ls -R``.

              Raises:
                  ListError: If path is not absolute.
                  ListError: If path does not exist or is not a directory.
                  ListError: If pattern is not a valid regular expression.
                  ListError: If more than ``_MAX_ENTRIES`` entries remain even after the result has been
                      reduced to one entry per directory level and then to top-level directories only.
              """
              '# When the flat file count exceeds the limit, the result is progressively reduced:'
              '# first to one entry per directory level (with a per-level file count), then, if'
              '# that still exceeds the limit, to top-level directories only (with an aggregated'
              '# file count below each). Both reduced results are returned with a warning.'
              dir_path = Path(item.path)
              if not dir_path.is_absolute():
                  raise ListError('Path must be absolute.')
              dir_path = normalize_directory(dir_path)
              if not dir_path.is_dir():
                  raise ListError('Directory not found or not a directory.')
              try:
                  regex = re.compile(item.pattern) if item.pattern else None
              except re.error as exc:
                  raise ListError(f'Invalid regex pattern: {exc}') from exc
              groups: dict[str, list[str]] = {}
              match_count = 0
              for root, dirs, files in os.walk(str(dir_path)):
                  rel_dir = os.path.relpath(root, str(dir_path))
                  matched_files = []
                  header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                  match_against_path = regex is not None and '/' in item.pattern
                  for file in sorted(files):
                      candidate = f'{header}/{file}' if match_against_path else file
                      if regex is None or regex.search(candidate):
                          matched_files.append(file)
                  if matched_files:
                      groups[rel_dir] = matched_files
                      match_count += len(matched_files)
                  if not item.recursive:
                      dirs.clear()
              if match_count <= _MAX_ENTRIES:
                  entries = []
                  for rel_dir in sorted(groups):
                      header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                      entries.append(f'{header}:')
                      entries.extend(groups[rel_dir])
                  return ListResult(path=item.path, entries=entries)
              '# Step 1: collapse each directory level to a single representative entry,'
              '# annotated with the total file count for that level.'
              level1_counts = {rel_dir: len(files) for rel_dir, files in groups.items()}
              if len(level1_counts) <= _MAX_ENTRIES:
                  entries = []
                  for rel_dir in sorted(groups):
                      header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
                      count = level1_counts[rel_dir]
                      entries.append(f'{header}: ({count} file{('s' if count != 1 else '')})')
                      entries.append(groups[rel_dir][0])
                  warning = f"Result reduced because the total number of entries ({match_count}) exceeds the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter."
                  return ListResult(path=item.path, entries=entries, warnings=(warning,))
              '# Step 2: the per-level reduction still exceeds the limit, so collapse the'
              '# tree depth and aggregate file counts below each top-level directory.'

              def _top_key(rel_dir: str) -> str:
                  if rel_dir == '.':
                      return '.'
                  return rel_dir.split(os.sep)[0]
              top_counts: dict[str, int] = {}
              for rel_dir, files in groups.items():
                  key = _top_key(rel_dir)
                  top_counts[key] = top_counts.get(key, 0) + len(files)
              if len(top_counts) > _MAX_ENTRIES:
                  raise ListError(
                      f"Too many entries even after reducing by directory level ({
                          len(top_counts)}) exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")
              entries = []
              for key in sorted(top_counts):
                  header = key if key == '.' else './' + key.replace(os.sep, '/')
                  count = top_counts[key]
                  entries.append(f'{header}: ({count} file{('s' if count != 1 else '')} below)')
              warning = f"Result reduced to top-level directories the limit of {_MAX_ENTRIES} is exceeded ({
                  len(level1_counts)}). Narrow down the result using the 'pattern' regular expression parameter."
              return ListResult(path=item.path, entries=entries, warnings=(warning,))
      - id: dedupe_hierarchy
        type: FunctionDef
        code: |-
          def _dedupe_hierarchy(items: list[ListItem]) -> list[ListItem]:
              """Remove items whose path lies below another item's path in the same directory hierarchy.

              Only the topmost item of each hierarchy is kept; items for separate hierarchies
              are left untouched. Original relative order of the kept items is preserved.
              """
              resolved = [(item, Path(item.path).resolve()) for item in items]
              resolved.sort(key=lambda pair: len(pair[1].parts))
              kept_paths: list[Path] = []
              kept_items: list[ListItem] = []
              for item, path in resolved:
                  if any((path == kept or kept in path.parents for kept in kept_paths)):
                      continue
                  if item.recursive:
                      kept_paths.append(path)
                  kept_items.append(item)
              kept_ids = {id(item) for item in kept_items}
              return [item for item in items if id(item) in kept_ids]
      - id: list
        type: FunctionDef
        code: |-
          def list(items: list[ListItem]) -> ListBatchResult:
              """List each directory in ``items``.

              Args:
                  items: Directories to list. Must be non-empty.

              Returns:
                  ListBatchResult: one result per successfully listed directory, one error per failed directory.

              Raises:
                  ListError: If items is empty.
              """
              if not items:
                  raise ListError("'items' must be a non-empty list.")
              items = _dedupe_hierarchy(items)
              results: list[ListResult] = []
              errors: list[ListItemError] = []
              for item in items:
                  try:
                      results.append(_list_one(item))
                  except ListError as exc:
                      errors.append(ListItemError(path=item.path, error=str(exc)))
              return ListBatchResult(results=results, errors=errors)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
      nodes:
      - id: parse_search_stdout
        type: FunctionDef
        code: |-
          def _parse_search_stdout(stdout: str) -> list[SearchMatch]:
              """Parse 'path:line:content' stdout into :class:`SearchMatch` objects, grouped by file."""
              grouped: dict[tuple[str, str], list[SearchLineMatch]] = {}
              order: list[tuple[str, str]] = []
              for line in stdout.splitlines():
                  if not line:
                      continue
                  path, sep, rest = line.partition(':')
                  if not sep:
                      raise SearchError(f'Cannot parse search output line: {line!r}')
                  lineno_str, sep, match = rest.partition(':')
                  if not sep or not lineno_str.isdigit():
                      raise SearchError(f'Cannot parse search output line: {line!r}')
                  directory, _, filename = path.rpartition('/')
                  key = (directory, filename)
                  if key not in grouped:
                      grouped[key] = []
                      order.append(key)
                  grouped[key].append(_make_line_match(int(lineno_str), match.strip()))
              return [SearchMatch(directory=d, filename=f, matches=grouped[d, f]) for d, f in order]
      - id: run_search
        type: FunctionDef
        code: |-
          def _run_search(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> tuple[ProcessResult, int]:
              """Recursively search one or more directories for ``pattern`` (extended regexp).

              Args:
                  directory: Absolute paths of the directories to search (each must exist and
                      be a directory).
                  pattern: Extended regular expression (grep -E syntax).
                  exclude: Globs of file names to exclude from the search, if given.
                  include: Globs of file names to include in the search, if given.
                  limit: Maximum number of matching lines to return (1..``_MAX_LIMIT``).

              Returns:
                  Tuple ``(result, total_lines)`` where ``total_lines`` is the number of matching
                  lines found before truncation to ``limit``, and ``result`` is a ProcessResult with:
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
              return (ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr), len(lines))
      - id: search_one
        type: FunctionDef
        code: |-
          def _search_one(item: SearchItem) -> SearchResult:
              result, total_lines = _run_search(item.directory, item.pattern, exclude=item.exclude,
                                                include=item.include, limit=item.limit)
              if result.exit_code >= 2:
                  raise SearchError(f'search failed (exit code {result.exit_code}): {result.stderr}')
              matches = _parse_search_stdout(result.stdout)
              warnings: list[str] = []
              if total_lines > item.limit:
                  hint = 'narrow pattern, directory or include/exclude filters'
                  if item.limit < _MAX_LIMIT:
                      hint += f', or raise limit (max {_MAX_LIMIT})'
                  warnings.append(f'Limit of {item.limit} matches reached (total {total_lines}); {hint}.')
              shortened = sum((lm.truncated for m in matches for lm in m.matches))
              if shortened:
                  warnings.append(
                      f'{shortened} match line(s) exceeded {_MAX_LINE_LENGTH} characters and were shortened in the middle.')
              return SearchResult(directory=item.directory, matches=matches, warnings=tuple(warnings))
      - id: resolve_nodes
        type: FunctionDef
        code: |-
          def _resolve_nodes(match: SearchMatch, roots: list[str]) -> SearchMatch:
              """Regroup a file's line matches by the most specific enclosing AST node.

              ``match`` paths are relative to one of ``roots``. Files that cannot be parsed
              are returned unchanged. Lines not covered by any node stay in ``matches``.
              """
              relative = f'{match.directory}/{match.filename}' if match.directory else match.filename
              path = next((str(Path(r) / relative) for r in roots if (Path(r) / relative).is_file()), None)
              if path is None:
                  return match
              try:
                  located = core.locate_all(core.load(path)[1])
              except (core.AstError, OSError):
                  return match
              by_node: dict[str, list[SearchLineMatch]] = {}
              unresolved: list[SearchLineMatch] = []
              for lm in match.matches:
                  loc = core.most_specific(located, lm.lineno, lm.lineno)
                  if loc is None:
                      unresolved.append(lm)
                  else:
                      by_node.setdefault(loc.node_id, []).append(lm)
              nodes = [SearchNodeMatch(node_id=nid, matches=lms) for nid, lms in by_node.items()]
              return SearchMatch(directory=match.directory, filename=match.filename, matches=unresolved, nodes=nodes)
      - id: search
        type: FunctionDef
        code: |-
          def search(items: list[SearchItem], *, intent: str=_DEFAULT_INTENT, with_nodes: bool=False) -> SearchBatchResult:
              """Run one or more independent searches. Limits apply per item, not per batch.

              Args:
                  items: Searches to run. Must be non-empty.
                  intent: Default intent for all items; ``SearchItem.intent`` takes precedence.
                  with_nodes: If true, items with intent ``'editing'`` resolve matched lines to AST node ids
                      (see :class:`SearchMatch`).

              Returns:
                  SearchBatchResult: one result per successful search, one error per failed search.

              Raises:
                  SearchError: If items is empty or the batch-level intent is invalid.
              """
              if not items:
                  raise SearchError("'items' must be a non-empty list.")
              _check_intent(intent)
              results: list[SearchResult] = []
              errors: list[SearchItemError] = []
              for item in items:
                  try:
                      effective = item.intent if item.intent is not None else intent
                      _check_intent(effective)
                      result = _search_one(item)
                      if effective == 'editing' and with_nodes:
                          result = replace(result, matches=[_resolve_nodes(m, item.directory) for m in result.matches])
                      results.append(result)
                  except SearchError as exc:
                      errors.append(SearchItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
              return SearchBatchResult(results=results, errors=errors)
```
/answer 2418-6-5 allow
Control Request:
```yaml
id: 2418-7-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    id: ls
  reason: List tools directory
```
/answer 2418-7-0 allow
Control Request:
```yaml
id: 2418-8-7
result:
  structuredContent:
    errors:
    - index: 0
      error: '''commands.json'' not found in working directory.'
```
/answer 2418-8-7 allow
Control Request:
```yaml
id: 2418-9-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/mcpc
    id: pip_show
  reason: Check pathspec version
```
/answer 2418-9-0 allow
Control Request:
```yaml
id: 2418-a-9
result:
  structuredContent:
    errors:
    - index: 0
      error: Unknown command id 'pip_show'.
```
/answer 2418-a-9 allow
ReasoningToken: 1492
Control Request:
```yaml
id: 2418-b-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: pathspec
    include:
    - '*.toml'
    - '*.txt'
    - '*.cfg'
    exclude:
    - .venv/**
    - node_modules/**
  reason: Find dependency declaration and version of pathspec
```
/answer 2418-b-0 allow
Control Request:
```yaml
id: 2418-c-b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
```
/answer 2418-c-b allow
ReasoningToken: 13
Control Request:
```yaml
id: 2418-d-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: ^(dependencies|requires)
    include:
    - pyproject.toml
    intent: editing
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: ^def check_file
    include:
    - gitignore.py
    - pathspec.py
    - _meta.py
    exclude:
    - node_modules/**
  reason: Locate pyproject dependencies and installed pathspec API
```
/answer 2418-d-0 allow
Control Request:
```yaml
id: 2418-e-d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: pyproject.toml
        nodes:
        - id: QJjxUF|5NIwWW
          matches:
          - "requires = [\"setuptools>=68\"]"
        - id: dCyDsi|xqd77g
          matches:
          - requires-python = ">=3.11"
          - "dependencies = ["
    - directory:
      - /home/user/xyan/xy.ai.workbench
```
/answer 2418-e-d allow
Control Request:
```yaml
id: 2418-f-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/pyproject.toml
    ids:
    - dCyDsi|xqd77g
  reason: Read dependencies block
```
/answer 2418-f-0 allow
Control Request:
```yaml
id: 2418-10-f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/pyproject.toml
      nodes:
      - id: dCyDsi|xqd77g
        type: table
        code: |+
          [project]
          name = "xy-ai-mcpc"
          version = "0.1.0"
          description = "MCP Controller — a stateful MCP server (Streamable HTTP, JSON-RPC 2.0)."
          readme = "README.md"
          requires-python = ">=3.11"
          license = { text = "Proprietary" }
          authors = [{ name = "xyan" }]
          keywords = ["mcp", "model-context-protocol", "json-rpc", "streamable-http"]

          # The Streamable-HTTP transport is built purely on the standard library; the
          # WebSocket transport (an additional, optional interface) uses "websockets".
          dependencies = [
              "websockets>=13",
          ]
```
/answer 2418-10-f allow
ReasoningToken: 44
ReasoningToken: 88
Control Request:
```yaml
id: 2418-11-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/pyproject.toml
    id: dCyDsi|xqd77g
    old_text: |2
          "websockets>=13",
    new_text: |2
          "websockets>=13",
          "pathspec>=0.12",
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: list_one
    old_text: |2-
              raise ListError('Directory not found or not a directory.')
          try:
    new_text: |2-
              raise ListError('Directory not found or not a directory.')
          access = McpcIgnore(dir_path)
          if not access.is_dir_visible(dir_path):
              raise ListError('Directory not found or not a directory.')
          try:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: list_one
    old_text: |2-
              rel_dir = os.path.relpath(root, str(dir_path))
              matched_files = []
    new_text: |2-
              dirs[:] = [d for d in dirs if access.is_dir_visible(Path(root) / d)]
              rel_dir = os.path.relpath(root, str(dir_path))
              matched_files = []
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: list_one
    old_text: |2-
              for file in sorted(files):
                  candidate
    new_text: |2-
              for file in sorted(files):
                  if not access.is_file_allowed(Path(root) / file):
                      continue
                  candidate
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: run_search
    old_text: "    prefixes = sorted((str(p).rstrip('/') + '/' for p in directory_paths), key=len, reverse=True)"
    new_text: |2-
          matchers = {p: McpcIgnore(p) for p in directory_paths}
          allowed_lines = []
          for line in result.stdout.splitlines():
              file_path = Path(line.partition(':')[0])
              owner = next((p for p in directory_paths if p in file_path.parents), None)
              if owner is None or matchers[owner].is_file_allowed(file_path):
                  allowed_lines.append(line)
          result = ProcessResult(exit_code=result.exit_code if allowed_lines else min(result.exit_code, 1), stdout='\n'.join(allowed_lines), stderr=result.stderr)
          prefixes = sorted((str(p).rstrip('/') + '/' for p in directory_paths), key=len, reverse=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: run_search
    old_text: "    directory_paths = normalize_directories([Path(d) for d in _as_list(directory)])"
    new_text: |2-
          directory_paths = normalize_directories([Path(d) for d in _as_list(directory)])
          directory_paths = [p for p in directory_paths if not p.is_absolute() or McpcIgnore(p).is_dir_visible(p)]
  insert:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: dvO55c|qVCPEH
    source: |
      from xy.ai.mcpc.tools._mcpcignore import McpcIgnore
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: 8Pl1k7|79r0Qn
    source: |
      from xy.ai.mcpc.tools._mcpcignore import McpcIgnore
    position: before
  create:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_mcpcignore.py
    source: |
      """Visibility filter based on ``.mcpcignore`` files.

      Syntax is that of ``.gitignore`` but with inverted meaning: a matching pattern
      *allows* a path, a negated pattern (``!pattern``) *excludes* it. Everything that
      is not allowed behaves as if it did not exist.
      """
      from __future__ import annotations

      from pathlib import Path

      from pathspec import GitIgnoreSpec

      IGNORE_FILE_NAME = '.mcpcignore'


      class McpcIgnore:
          """Evaluates the ``.mcpcignore`` files from ``/`` down to a base directory.

          Files are combined root first, so deeper files override shallower ones and,
          as in gitignore, the last matching pattern wins. Patterns are relative to the
          directory containing their ``.mcpcignore``. Without any ``.mcpcignore``
          everything is visible.

          Directories are visible unless explicitly excluded, so that allowed files
          below them (e.g. ``*.py``) stay reachable. Files must be explicitly allowed.
          """

          def __init__(self, base: Path) -> None:
              base = Path(base).resolve()
              self._specs: list[tuple[Path, GitIgnoreSpec]] = []
              for directory in reversed([base, *base.parents]):
                  ignore_file = directory / IGNORE_FILE_NAME
                  try:
                      lines = ignore_file.read_text(encoding='utf-8').splitlines()
                  except OSError:
                      continue
                  self._specs.append((directory, GitIgnoreSpec.from_lines(lines)))

          def _verdict(self, path: Path, is_dir: bool) -> bool | None:
              """Return True if allowed, False if excluded, None if no pattern matched."""
              verdict: bool | None = None
              for directory, spec in self._specs:
                  try:
                      relative = path.relative_to(directory).as_posix()
                  except ValueError:
                      continue
                  if is_dir:
                      relative += '/'
                  result = spec.check_file(relative)
                  if result.include is not None:
                      verdict = result.include
              return verdict

          def is_file_allowed(self, path: Path) -> bool:
              if not self._specs:
                  return True
              return self._verdict(Path(path).resolve(), is_dir=False) is True

          def is_dir_visible(self, path: Path) -> bool:
              if not self._specs:
                  return True
              return self._verdict(Path(path).resolve(), is_dir=True) is not False
  reason: "Add pathspec dependency, create McpcIgnore class, integrate into list and search"
```
/answer 2418-11-0 allow
ReasoningToken: 103
Control Request:
```yaml
id: 2418-12-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_mcpcignore.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  reason: Verify the edited files compile
```
/answer 2418-12-0 allow
Result Stats: id=30dabdb0-5445-4fa7-a968-d385d89344bc, total: 37712, in: 18, out: 9736, reason: 0, read: 150381, write: 27958, start: 1794956168266, end: 1794956176802, time: 8457, total: 71419, io: 0, control: 79, sent: 0, recv: 224
Die `.mcpcignore`-Filterung ist in `list` und `search` eingebaut. Die drei Dateien lassen sich kompilieren, ausgeführt oder getestet habe ich nichts.

**Neue Klasse `McpcIgnore`** (`tools/_mcpcignore.py`)
- Sie liest alle `.mcpcignore`-Dateien vom Basisverzeichnis aufwärts bis `/`. Kombiniert werden sie von Root nach unten, tiefere Dateien überschreiben also flachere.
- Die Syntax entspricht `.gitignore`, die Bedeutung ist invertiert. Ein Treffer gibt den Pfad frei, ein negierter Eintrag (`!muster`) schließt ihn aus. Der letzte passende Eintrag gewinnt.
- Pfade in einer Datei gelten relativ zu dem Verzeichnis, in dem diese `.mcpcignore` liegt.
- Dateien sind nur sichtbar, wenn ein Eintrag sie freigibt (`is_file_allowed`).
- Verzeichnisse sind sichtbar, solange sie nicht ausdrücklich ausgeschlossen sind (`is_dir_visible`). So bleiben Dateien darunter erreichbar, wenn z. B. nur `*.py` freigegeben ist.
- Gibt es auf dem Pfad gar keine `.mcpcignore`, ist alles sichtbar. Das habe ich so gewählt, damit bestehende Setups weiter funktionieren.

**`list`**
- Ein ausgeschlossenes Startverzeichnis liefert „Directory not found or not a directory.“.
- Beim Durchlaufen werden ausgeschlossene Unterverzeichnisse übersprungen und nicht freigegebene Dateien weggelassen.

**`search`**
- Verzeichnisse, die nicht sichtbar sind, werden vorab entfernt.
- Die Trefferzeilen von grep werden gefiltert, bevor das Limit angewendet wird. Limit und Gesamtzahl in der Warnung zählen also nur sichtbare Treffer.
- Wenn nach dem Filtern nichts übrig bleibt, ist der Exit-Code 1 („keine Treffer“).
- Wenn alle Suchverzeichnisse unsichtbar sind, lautet die Fehlermeldung „At least one directory is required.“. Das ist etwas irreführend, ich habe es nicht angepasst.

**Abhängigkeit**
- `pathspec>=0.12` steht jetzt in `pyproject.toml`, weil die Klasse `GitIgnoreSpec.check_file` nutzt.

**Offen**
- `.mcpcignore`-Dateien in Unterverzeichnissen unterhalb des Parameters werden nicht ausgewertet. Du hattest nur die Richtung vom Parameter aufwärts beschrieben.
- Ein nicht ausgeschlossenes Verzeichnis ohne freigegebene Dateien erscheint nicht als „nicht existent“, sondern als leer.

Soll ich beides ergänzen, also Unterverzeichnis-Dateien auswerten und Verzeichnisse ohne freigegebenen Inhalt als nicht existent behandeln?
Ich halte es nicht für notwendig Eingabeparameter zu Filtern. Das würde es unnötig machen Verzeichnisse per Default anzuzeigen. Die beiden Tools zielen auf Dateien an. 
SystemInit:  id=30dabdb0-5445-4fa7-a968-d385d89344bc, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 2418-13-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
    id: list_one
    old_text: |2
          access = McpcIgnore(dir_path)
          if not access.is_dir_visible(dir_path):
              raise ListError('Directory not found or not a directory.')
    new_text: |2
          access = McpcIgnore(dir_path)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
    id: run_search
    old_text: |2-

          directory_paths = [p for p in directory_paths if not p.is_absolute() or McpcIgnore(p).is_dir_visible(p)]
    new_text: ""
  reason: Remove filtering of input directory parameters in list and search
```
/answer 2418-13-0 allow
ReasoningToken: 123
Control Request:
```yaml
id: 2418-14-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/search/__init__.py
  reason: Verify files still compile
```
/answer 2418-14-0 allow
Result Stats: id=30dabdb0-5445-4fa7-a968-d385d89344bc, total: 41125, in: 22, out: 10833, reason: 0, read: 207550, write: 30270, start: 1794956374794, end: 1794956378424, time: 3572, total: 80430, io: 0, control: 58, sent: 0, recv: 89
Ich habe die Prüfung der Eingabeverzeichnisse in `list` und `search` entfernt. Beide Dateien lassen sich weiterhin kompilieren, getestet habe ich nichts.

- **Eingabeparameter:** Ein angegebenes Verzeichnis wird immer akzeptiert. Der Fehler „Directory not found“ und die irreführende Meldung „At least one directory is required.“ entfallen damit.
- **Filter:** Gefiltert werden nur noch Dateien, also die Einträge von `list` und die Treffer von `search`. Eine Datei ist sichtbar, wenn die `.mcpcignore` sie freigibt.
- **Unterverzeichnisse in `list`:** Beim Durchlaufen überspringt `list` weiterhin Verzeichnisse, die ausdrücklich per `!`-Eintrag ausgeschlossen sind. Das spart Laufzeit und ändert nichts an der Dateifilterung. Sag Bescheid, wenn auch das entfallen soll.

`McpcIgnore.is_dir_visible` wird jetzt nur noch für diese Unterverzeichnisse aufgerufen.
Ich denke das passt. Der ausschluss eines Dateityps in gitignore negiert auch nicht ein ignoriertes verzeichnis oder?
Result Stats: id=30dabdb0-5445-4fa7-a968-d385d89344bc, total: 42786, in: 24, out: 11962, reason: 587, read: 237820, write: 30800, start: 1794956458242, end: 1794956467727, time: 9408, total: 89838, io: 0, control: 77, sent: 1, recv: 124
SystemInit:  id=30dabdb0-5445-4fa7-a968-d385d89344bc, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 587
Ja, das stimmt, und die Umsetzung verhält sich im Kern genauso. Ein negierter Dateityp-Eintrag wie `!*.log` betrifft nur Pfade, auf die das Muster passt. Ein Verzeichnis wird dadurch weder ausgeschlossen noch wieder freigegeben. Es sei denn, es heißt selbst `x.log`, denn auch in Git trifft ein solches Muster Verzeichnisse.

Es gibt allerdings eine Abweichung von Git, und das kann ich dir so noch nicht zusichern:

- **Git:** Eine Datei lässt sich nicht wieder freigeben, wenn ein übergeordnetes Verzeichnis ausgeschlossen ist.
- **`list`:** Hier gilt das ebenfalls. Ausgeschlossene Verzeichnisse werden beim Durchlaufen übersprungen.
- **`search`:** Hier wird jede Datei einzeln bewertet. Hat die `.mcpcignore` zuerst `!gen/` und später `*.py`, bleibt `gen/x.py` in `search` sichtbar, in `list` dagegen nicht.

Ich kann `McpcIgnore.is_file_allowed` so ändern, dass es zusätzlich alle übergeordneten Verzeichnisse bis zum Basisverzeichnis prüft und die Datei ausblendet, sobald eines davon ausgeschlossen ist. Dann verhalten sich beide Tools gleich und wie Git. Soll ich das umsetzen?
Ja
SystemInit:  id=30dabdb0-5445-4fa7-a968-d385d89344bc, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 2418-15-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_mcpcignore.py
    id: McpcIgnore
    old_text: |2
              base = Path(base).resolve()
    new_text: |2
              base = Path(base).resolve()
              self._base = base
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_mcpcignore.py
    id: McpcIgnore
    old_text: "        return self._verdict(Path(path).resolve(), is_dir=False) is True"
    new_text: |2-
              resolved = Path(path).resolve()
              if self._base in resolved.parents:
                  for parent in resolved.parents:
                      if parent == self._base:
                          break
                      if not self.is_dir_visible(parent):
                          return False
              return self._verdict(resolved, is_dir=False) is True
  reason: Make is_file_allowed respect excluded parent directories like gitignore
```
/answer 2418-15-0 allow
ReasoningToken: 106
Control Request:
```yaml
id: 2418-16-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_mcpcignore.py
  reason: Verify file compiles
```
/answer 2418-16-0 allow
Result Stats: id=30dabdb0-5445-4fa7-a968-d385d89344bc, total: 45959, in: 28, out: 12950, reason: 0, read: 300579, write: 32981, start: 1794956548011, end: 1794956550580, time: 2516, total: 96888, io: 0, control: 53, sent: 0, recv: 59
`McpcIgnore.is_file_allowed` prüft jetzt zusätzlich alle übergeordneten Verzeichnisse zwischen Datei und Basisverzeichnis. Ist eines davon ausdrücklich ausgeschlossen, gilt die Datei als nicht vorhanden, auch wenn ein späterer Eintrag sie freigeben würde.

`gen/x.py` mit `!gen/` gefolgt von `*.py` wird damit in `search` genauso ausgeblendet wie in `list`. Die Datei lässt sich kompilieren, getestet habe ich die Änderung nicht.

Verzeichnisse oberhalb des Basisverzeichnisses werden nicht geprüft, weil die Eingabeparameter ungefiltert bleiben. Ihre `.mcpcignore`-Dateien werden aber weiterhin ausgewertet.
