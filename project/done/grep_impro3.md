Verbessere das Tool `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py`.

- Im "locate" Modus können exakt doppelte matches dedupliziert werden.

Beispiel:
```yaml
id: fc3c-e6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      matches:
      - path: upload.rs
        matches:
        - "#[cfg(feature = \"cuda\")]"
        - "#[cfg(feature = \"cuda\")]"
        - "#[cfg(feature = \"cuda\")]"
        - "#[cfg(test)]"
      - path: swap.rs
        matches:
        - "#[cfg(loom)]"
        - "#[cfg(not(loom))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(test, not(loom)))]"
        - "#[cfg(loom)]"
        - "#[cfg(all(test, feature = \"cuda\", not(loom)))]"
      - path: test/upload.rs
        matches:
        - "#[cfg(feature = \"cuda\")]"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
SystemInit:  id=e39c1a99-6770-4933-960d-aa8802848734, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 5b02-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
  reason: Understand the grep tool structure
```
/answer 5b02-1 allow
Control Request:
```yaml
id: 5b02-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
      nodes:
      - id: QA8gK6|AIzwjR
        signature: "'Grep tool – recursive extended-regex search for retrieval, for a batch of item…"
      - id: PBZWYD|FmTmX2
        signature: "from dataclasses import dataclass, replace"
      - id: 8Pl1k7|dBAVKD
        signature: "from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema"
      - id: SFh15A|ugB2xL
        signature: "__all__ = ["
      - id: 05uPHx|pOHO4c
        signature: "_INTENT_DESCRIPTION = \"What you will do with the results. 'locate': only need t…"
      - id: GrepError
        signature: "class GrepError(Exception):"
        docstring: Raised when a grep search cannot be executed or its output cannot be parsed.
      - id: GrepLineMatch
        signature: "@dataclass(frozen=True) class GrepLineMatch:"
        docstring: A single matching line within a file.
      - id: GrepNodeMatch
        signature: "@dataclass(frozen=True) class GrepNodeMatch:"
        docstring: Matching lines attributed to the most specific enclosing AST node.
      - id: GrepMatch
        signature: "@dataclass(frozen=True) class GrepMatch:"
        docstring: All matches found within a single file. ``nodes`` is set instead of relying on …
      - id: GrepItem
        signature: "@dataclass(frozen=True) class GrepItem:"
        docstring: "One independent grep search to run. Attributes: directory: Absolute paths of th…"
      - id: GrepResult
        signature: "@dataclass(frozen=True) class GrepResult:"
        docstring: "Result of a single grep search. Attributes: directory: The directory list exact…"
      - id: GrepItemError
        signature: "@dataclass(frozen=True) class GrepItemError:"
        docstring: "Error running a single grep search, mirroring its input for result association."
      - id: GrepBatchResult
        signature: "@dataclass(frozen=True) class GrepBatchResult:"
        docstring: "Result of :func:`grep`. Attributes: results: One :class:`GrepResult` per succes…"
      - id: parse_grep_stdout
        signature: "def _parse_grep_stdout(stdout: str) -> list[GrepMatch]:"
        docstring: "Parse grep's 'path:line:content' stdout into :class:`GrepMatch` objects, groupe…"
      - id: as_list
        signature: "def _as_list(value: list[str] | None) -> list[str]:"
        docstring: Normalize an optional list into a list (empty if ``None``).
      - id: run_grep
        signature: "def _run_grep(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:"
        docstring: Recursively search one or more directories for ``pattern`` (extended regexp). A…
      - id: grep_one
        signature: "def _grep_one(item: GrepItem) -> GrepResult:"
      - id: resolve_nodes
        signature: "def _resolve_nodes(match: GrepMatch, roots: list[str]) -> GrepMatch:"
        docstring: Regroup a file's line matches by the most specific enclosing AST node. ``match`…
      - id: grep
        signature: "def grep(items: list[GrepItem], *, intent: str=_DEFAULT_INTENT) -> GrepBatchResult:"
        docstring: "Run one or more independent grep searches. Limits apply per item, not per batch…"
      - id: check_intent
        signature: "def _check_intent(intent: str) -> None:"
      - id: GrepTool
        signature: "class GrepTool(ToolDefinition):"
        children:
        - id: GrepTool.KOzj71|BvGHdk
          signature: name = 'grep'
        - id: GrepTool.g4tkLE|v74Wwu
          signature: "_ITEM_PROPERTIES = {"
        - id: GrepTool.SQiS9d|AAWDcQ
          signature: "_ITEM_REQUIRED = ['directory', 'pattern']"
        - id: GrepTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`grep`, translating the MCP schema to/from the Python API."
      - id: register_grep_tool
        signature: "def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 5b02-2 allow
Control Request:
```yaml
id: 5b02-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    ids:
    - GrepLineMatch
    - GrepMatch
    - grep_one
    - parse_grep_stdout
  reason: Read relevant code
```
/answer 5b02-3 allow
Control Request:
```yaml
id: 5b02-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
      nodes:
      - id: GrepLineMatch
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class GrepLineMatch:
              """A single matching line within a file."""
              lineno: int
              match: str
      - id: GrepMatch
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class GrepMatch:
              """All matches found within a single file.

              ``nodes`` is set instead of relying on ``matches`` only if the file could be
              resolved to AST nodes; lines outside any node are kept under ``matches``.
              """
              directory: str
              filename: str
              matches: list[GrepLineMatch]
              nodes: list[GrepNodeMatch] | None = None
      - id: grep_one
        type: FunctionDef
        code: |-
          def _grep_one(item: GrepItem) -> GrepResult:
              result = _run_grep(item.directory, item.pattern, exclude=item.exclude, include=item.include, limit=item.limit)
              if result.exit_code >= 2:
                  raise GrepError(f'grep failed (exit code {result.exit_code}): {result.stderr}')
              matches = _parse_grep_stdout(result.stdout)
              total_matches = sum((len(m.matches) for m in matches))
              warning = None
              if total_matches >= item.limit:
                  warning = f'Limit of {
                      item.limit} matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit.'
              return GrepResult(directory=item.directory, matches=matches, warning=warning)
      - id: parse_grep_stdout
        type: FunctionDef
        code: |-
          def _parse_grep_stdout(stdout: str) -> list[GrepMatch]:
              """Parse grep's 'path:line:content' stdout into :class:`GrepMatch` objects, grouped by file."""
              grouped: dict[tuple[str, str], list[GrepLineMatch]] = {}
              order: list[tuple[str, str]] = []
              for line in stdout.splitlines():
                  if not line:
                      continue
                  path, sep, rest = line.partition(':')
                  if not sep:
                      raise GrepError(f'Cannot parse grep output line: {line!r}')
                  lineno_str, sep, match = rest.partition(':')
                  if not sep or not lineno_str.isdigit():
                      raise GrepError(f'Cannot parse grep output line: {line!r}')
                  directory, _, filename = path.rpartition('/')
                  key = (directory, filename)
                  if key not in grouped:
                      grouped[key] = []
                      order.append(key)
                  grouped[key].append(GrepLineMatch(lineno=int(lineno_str), match=match.strip()))
              return [GrepMatch(directory=d, filename=f, matches=grouped[d, f]) for d, f in order]
```
/answer 5b02-4 allow
ReasoningToken: 25
Control Request:
```yaml
id: 5b02-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    ids:
    - grep
    - GrepTool.handle
  reason: See locate mode output
```
/answer 5b02-5 allow
Control Request:
```yaml
id: 5b02-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
      nodes:
      - id: grep
        type: FunctionDef
        code: |-
          def grep(items: list[GrepItem], *, intent: str=_DEFAULT_INTENT) -> GrepBatchResult:
              """Run one or more independent grep searches. Limits apply per item, not per batch.

              Args:
                  items: Grep searches to run. Must be non-empty.
                  intent: Default intent for all items; ``GrepItem.intent`` takes precedence.
                      ``'ast_node_ids'`` resolves matched lines to AST node ids (see :class:`GrepMatch`).

              Returns:
                  GrepBatchResult: one result per successful search, one error per failed search.

              Raises:
                  GrepError: If items is empty or the batch-level intent is invalid.
              """
              if not items:
                  raise GrepError("'items' must be a non-empty list.")
              _check_intent(intent)
              results: list[GrepResult] = []
              errors: list[GrepItemError] = []
              for item in items:
                  try:
                      effective = item.intent if item.intent is not None else intent
                      _check_intent(effective)
                      result = _grep_one(item)
                      if effective == 'ast_node_ids':
                          result = replace(result, matches=[_resolve_nodes(m, item.directory) for m in result.matches])
                      results.append(result)
                  except GrepError as exc:
                      errors.append(GrepItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
              return GrepBatchResult(results=results, errors=errors)
      - id: GrepTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`grep`, translating the MCP schema to/from the Python API."""
              intent = ctx.arguments.get('intent', _DEFAULT_INTENT)

              def item_factory(it: dict[str, Any]) -> GrepItem:
                  return GrepItem(
                      directory=it['directory'],
                      pattern=it['pattern'],
                      exclude=it.get('exclude'),
                      include=it.get('include'),
                      limit=int(
                          it.get(
                              'limit',
                              _DEFAULT_LIMIT)),
                      intent=it.get('intent'))

              def line_dicts(lms: list[GrepLineMatch], with_lineno: bool) -> list[Any]:
                  if with_lineno:
                      return [{'lineno': lm.lineno, 'match': lm.match} for lm in lms]
                  return [lm.match for lm in lms]

              def file_dict(m: GrepMatch, item_intent: str) -> dict[str, Any]:
                  entry: dict[str, Any] = {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename}
                  if m.nodes:
                      entry['nodes'] = [{'id': n.node_id, 'matches': line_dicts(
                          n.matches, item_intent == 'line_numbers')} for n in m.nodes]
                  if m.matches:
                      entry['matches'] = line_dicts(m.matches, item_intent != 'locate')
                  return entry

              def result_serializer(rendered: tuple[GrepResult, str]) -> dict[str, Any]:
                  r, item_intent = rendered
                  entry: dict[str, Any] = {'directory': r.directory}
                  if r.matches:
                      entry['matches'] = [file_dict(m, item_intent) for m in r.matches]
                  if r.warning is not None:
                      entry['warning'] = r.warning
                  return entry

              def error_serializer(e: GrepItemError) -> dict[str, Any]:
                  return {'directory': e.directory, 'pattern': e.pattern, 'error': e.error}

              def run(items: list[GrepItem]) -> SimpleNamespace:
                  results: list[tuple[GrepResult, str]] = []
                  errors: list[GrepItemError] = []
                  for item in items:
                      batch = grep([item], intent=intent)
                      effective = item.intent if item.intent is not None else intent
                      results += [(r, effective) for r in batch.results]
                      errors += batch.errors
                  return SimpleNamespace(results=results, errors=errors)
              return handle_batch_tool(ctx, item_factory, run, GrepError, result_serializer, error_serializer)
```
/answer 5b02-6 allow
Text:
 Dedupe in the serializer for locate mode (order-preserving).
Control Request:
```yaml
id: 5b02-7
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    id: GrepTool.handle
    old_text: "            entry['matches'] = line_dicts(m.matches, item_intent != 'locate')"
    new_text: |2-
                  lines = line_dicts(m.matches, item_intent != 'locate')
                  entry['matches'] = list(dict.fromkeys(lines)) if item_intent == 'locate' else lines
  reason: Deduplicate identical matches in locate mode
```
/answer 5b02-7 allow
Control Request:
```yaml
id: 5b02-8
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
  reason: Check syntax
```
/answer 5b02-8 allow
Result Stats: id=e39c1a99-6770-4933-960d-aa8802848734, total: 13448, in: 10, out: 1205, reason: 0, read: 36037, write: 12233, start: 1792927151316, end: 1792927154076, time: 2705, total: 15041, io: 0, control: 55, sent: 0, recv: 60
Im "locate"-Modus entfernt das Grep-Tool jetzt exakt doppelte Treffer pro Datei und behält dabei die Reihenfolge bei. Die Änderung steht in `GrepTool.handle` in `__init__.py`. Die Datei kompiliert, ich habe das Tool aber nicht gegen Ihr Beispiel laufen lassen.

Die anderen Modi (`line_numbers`, `ast_node_ids`) sind unverändert. Das `limit` zählt weiterhin die rohen Treffer vor der Deduplizierung. Die Limit-Warnung kann also erscheinen, obwohl nach dem Entfernen der Duplikate weniger als `limit` Zeilen angezeigt werden, so wie in Ihrem Beispiel.
