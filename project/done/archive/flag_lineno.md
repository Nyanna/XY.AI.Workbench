Analog zu den AST Node ID's soll die Ausgabe der Zeilennummern per Flag steuerbar sein (default false).
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py`
SystemInit:  id=43b26146-a835-42a7-b8d0-f9e8c32dc548, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 9e96-2f
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
  reason: Understand the grep tool structure
```
/answer 9e96-2f allow
Control Request:
```yaml
id: 9e96-30
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
      nodes:
      - id: QA8gK6|AIzwjR
        signature: "'Grep tool – recursive extended-regex search for retrieval, for a batch of item…"
      - id: a7y1Wc|CpnlXj
        signature: from dataclasses import dataclass
      - id: 8Pl1k7|GddZCa
        signature: from xy.ai.mcpc.tools.ast import core
      - id: qoUXrH|KeLseF
        signature: "__all__ = ["
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
        signature: "def grep(items: list[GrepItem], *, with_ast: bool=False) -> GrepBatchResult:"
        docstring: "Run one or more independent grep searches. Limits apply per item, not per batch…"
      - id: GrepTool
        signature: "class GrepTool(ToolDefinition):"
        children:
        - id: GrepTool.zkYjDM|Gpi0QI
          signature: name = 'grep'
        - id: GrepTool.MlMMUV|U4cpub
          signature: "_ITEM_PROPERTIES = {"
        - id: GrepTool.8HAP7K|fdLcId
          signature: "_ITEM_REQUIRED = ['directory', 'pattern']"
        - id: GrepTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`grep`, translating the MCP schema to/from the Python API."
      - id: register_grep_tool
        signature: "def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 9e96-30 allow
Control Request:
```yaml
id: 9e96-31
toolName: python
arguments:
  script: print(open('/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py').read())
  reason: Read full file source
```
/answer 9e96-31 deny
Text:
 Das Python-Tool wurde abgelehnt, also lese ich die Knoten mit den AST-Tools.
Control Request:
```yaml
id: 9e96-32
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    ids:
    - QA8gK6|AIzwjR
    - GrepLineMatch
    - GrepNodeMatch
    - GrepMatch
    - GrepResult
    - grep
    - resolve_nodes
    - GrepTool
    - parse_grep_stdout
    - grep_one
  reason: Read relevant nodes to add a line-number flag
```
/answer 9e96-32 allow
Control Request:
```yaml
id: 9e96-33
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
      nodes:
      - id: QA8gK6|AIzwjR
        type: statements
        code: "'Grep tool – recursive extended-regex search for retrieval, for a batch of items.'"
      - id: GrepLineMatch
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class GrepLineMatch:
              """A single matching line within a file."""
              lineno: int
              match: str
      - id: GrepNodeMatch
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class GrepNodeMatch:
              """Matching lines attributed to the most specific enclosing AST node."""
              node_id: str
              matches: list[GrepLineMatch]
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
      - id: GrepResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class GrepResult:
              """Result of a single grep search.

              Attributes:
                  directory: The directory list exactly as given in the input.
                  matches: The matches found, grouped by file (empty if none).
                  warning: Set if ``limit`` was reached and further matches may exist.
              """
              directory: list[str]
              matches: list[GrepMatch]
              warning: str | None = None
      - id: grep
        type: FunctionDef
        code: |-
          def grep(items: list[GrepItem], *, with_ast: bool=False) -> GrepBatchResult:
              """Run one or more independent grep searches. Limits apply per item, not per batch.

              Args:
                  items: Grep searches to run. Must be non-empty.
                  with_ast: Resolve matched lines to AST node ids (see :class:`GrepMatch`).

              Returns:
                  GrepBatchResult: one result per successful search, one error per failed search.

              Raises:
                  GrepError: If items is empty.
              """
              if not items:
                  raise GrepError("'items' must be a non-empty list.")
              results: list[GrepResult] = []
              errors: list[GrepItemError] = []
              for item in items:
                  try:
                      result = _grep_one(item)
                      if with_ast:
                          result = GrepResult(
                              directory=result.directory,
                              matches=[
                                  _resolve_nodes(
                                      m,
                                      item.directory) for m in result.matches],
                              warning=result.warning)
                      results.append(result)
                  except GrepError as exc:
                      errors.append(GrepItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
              return GrepBatchResult(results=results, errors=errors)
      - id: resolve_nodes
        type: FunctionDef
        code: |-
          def _resolve_nodes(match: GrepMatch, roots: list[str]) -> GrepMatch:
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
              by_node: dict[str, list[GrepLineMatch]] = {}
              unresolved: list[GrepLineMatch] = []
              for lm in match.matches:
                  loc = core.most_specific(located, lm.lineno, lm.lineno)
                  if loc is None:
                      unresolved.append(lm)
                  else:
                      by_node.setdefault(loc.node_id, []).append(lm)
              nodes = [GrepNodeMatch(node_id=nid, matches=lms) for nid, lms in by_node.items()]
              return GrepMatch(directory=match.directory, filename=match.filename, matches=unresolved, nodes=nodes)
      - id: GrepTool
        type: ClassDef
        signature: "class GrepTool(ToolDefinition):"
        children:
        - id: GrepTool.zkYjDM|Gpi0QI
          type: statements
          code: |-
            name = 'grep'
            title = 'Search files with grep'
            description = "Run one or more independent grep searches for lines matching an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters. Limits apply per item, not per batch."
        - id: GrepTool.MlMMUV|U4cpub
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'directory': {
                    'type': 'array',
                    'items': {
                        'type': 'string'},
                    'minItems': 1,
                    'description': 'Absolute paths of the directories to search recursively. Always use the narrowest subtree(s) that are likely to contain the target files.'},
                'pattern': {
                    'type': 'string',
                            'description': 'Extended regular expression to search for. Make the pattern as specific as possible to reduce noise.'},
                'exclude': {
                    'type': 'array',
                    'items': {
                        'type': 'string'},
                    'description': "Globs of file names to exclude from the search, e.g. '*.min.js'. Always set this to exclude build artefacts, dependencies (e.g. 'node_modules/**'), and minified files."},
                'include': {
                    'type': 'array',
                    'items': {
                        'type': 'string'},
                    'description': "Globs of file names to include in the search, e.g. '*.py'. Always set this to restrict the search to the relevant file types; omit only when the file type is unknown."},
                'limit': {
                    'type': 'integer',
                    'description': 'Maximum number of matching lines to return for this item.',
                    'default': _DEFAULT_LIMIT,
                    'minimum': 1,
                    'maximum': _MAX_LIMIT}}
        - id: GrepTool.8HAP7K|fdLcId
          type: statements
          code: |-
            _ITEM_REQUIRED = ['directory', 'pattern']
            _ITEMS_DESCRIPTION = 'Independent grep searches to run.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)
            input_schema['properties']['with_ast'] = {'type': 'boolean', 'default': False,
                                                      'description': 'Return node id (usable with ast_read and other ast_* tools).'}
        - id: GrepTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`grep`, translating the MCP schema to/from the Python API."""
                with_ast = bool(ctx.arguments.get('with_ast', False))

                def item_factory(it: dict[str, Any]) -> GrepItem:
                    return GrepItem(
                        directory=it['directory'],
                        pattern=it['pattern'],
                        exclude=it.get('exclude'),
                        include=it.get('include'),
                        limit=int(
                            it.get(
                                'limit',
                                _DEFAULT_LIMIT)))

                def line_dicts(lms: list[GrepLineMatch]) -> list[dict[str, Any]]:
                    return [{'lineno': lm.lineno, 'match': lm.match} for lm in lms]

                def file_dict(m: GrepMatch) -> dict[str, Any]:
                    entry: dict[str, Any] = {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename}
                    if m.nodes is not None:
                        entry['nodes'] = [{'id': n.node_id, 'matches': line_dicts(n.matches)} for n in m.nodes]
                        if m.matches:
                            entry['matches'] = line_dicts(m.matches)
                    else:
                        entry['matches'] = line_dicts(m.matches)
                    return entry

                def result_serializer(r: GrepResult) -> dict[str, Any]:
                    entry: dict[str, Any] = {'directory': r.directory, 'matches': [file_dict(m) for m in r.matches]}
                    if r.warning is not None:
                        entry['warning'] = r.warning
                    return entry

                def error_serializer(e: GrepError) -> dict[str, Any]:
                    return {'directory': e.directory, 'pattern': e.pattern, 'error': e.error}
                return handle_batch_tool(
                    ctx,
                    item_factory,
                    lambda items: grep(
                        items,
                        with_ast=with_ast),
                    GrepError,
                    result_serializer,
                    error_serializer)
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
```
/answer 9e96-33 allow
Control Request:
```yaml
id: 9e96-34
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    id: GrepTool.8HAP7K|fdLcId
    old_text: "'description': 'Return node id (usable with ast_read and other ast_* tools).'}"
    new_text: |-
      'description': 'Return node id (usable with ast_read and other ast_* tools).'}
      input_schema['properties']['with_lineno'] = {'type': 'boolean', 'default': False,
                                                   'description': 'Include the line number of each match.'}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    id: GrepTool.handle
    old_text: "with_ast = bool(ctx.arguments.get('with_ast', False))"
    new_text: |-
      with_ast = bool(ctx.arguments.get('with_ast', False))
          with_lineno = bool(ctx.arguments.get('with_lineno', False))
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    id: GrepTool.handle
    old_text: "return [{'lineno': lm.lineno, 'match': lm.match} for lm in lms]"
    new_text: |-
      if with_lineno:
                  return [{'lineno': lm.lineno, 'match': lm.match} for lm in lms]
              return [{'match': lm.match} for lm in lms]
  reason: Add with_lineno flag to schema and serialization
```
/answer 9e96-34 allow
ReasoningToken: 163
Control Request:
```yaml
id: 9e96-35
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
  reason: Verify the file compiles
```
/answer 9e96-35 allow
Result Stats: id=43b26146-a835-42a7-b8d0-f9e8c32dc548, total: 15857, in: 10, out: 1851, reason: 0, read: 34709, write: 13996, start: 1792351456453, end: 1792351462039, time: 5522, total: 22666, io: 0, control: 64, sent: 0, recv: 87
Das Grep-Tool hat jetzt ein Flag `with_lineno` (boolean, Default `false`), genau wie `with_ast`. Die Datei lässt sich kompilieren. Getestet habe ich es nicht.

- **Schema:** `with_lineno` steht als Top-Level-Property im `input_schema` von `GrepTool`.
- **Ausgabe:** `handle` liest das Flag und gibt `lineno` nur dann pro Treffer aus, wenn es gesetzt ist. Ohne das Flag enthält jeder Treffer nur `{"match": ...}`.
- **Unverändert:** Die Python-API (`grep()`, `GrepLineMatch`) ist gleich geblieben. Die Zeilennummern werden weiterhin intern ermittelt, weil `with_ast` sie braucht. Das Flag steuert nur die Serialisierung.

Das Standardverhalten ändert sich damit. Aufrufer, die `lineno` erwarten, müssen jetzt `with_lineno: true` übergeben.
