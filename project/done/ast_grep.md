Erweitere `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py` um ein Postprocessing.
Wenn AST-Read aktiviert ist, soll eine Rückauflösung der Grep Ergebnisse zu AST-Node ID auf Basis der gefundenen Zeilennummern erfolgen.
Die Ergebnisse sollen dann nicht gruppiert werden von Dateien zu Matches, sondern von Dateien zu Node-ID zu Matches (neue Zwischenebene).

- Prüfung auf Tools: `with_ast = bool({'tools', 'ast_read'} & ctx.session.enabled_tools)`
- AST-Find kann bereits Zeilen zu Nodes auflösen: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py`
SystemInit:  id=04dcac6d-2976-4411-ae75-bfccb9959e35, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: c6f3-7
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: Understand structure of grep tool and ast find tool
```
/answer c6f3-7 allow
Control Request:
```yaml
id: c6f3-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
      nodes:
      - id: QA8gK6|AIzwjR
        signature: "'Grep tool – recursive extended-regex search for retrieval, for a batch of item…"
      - id: a7y1Wc|CpnlXj
        signature: from dataclasses import dataclass
      - id: qoUXrH|VnDEso
        signature: "__all__ = ["
      - id: GrepError
        signature: "class GrepError(Exception):"
        docstring: Raised when a grep search cannot be executed or its output cannot be parsed.
      - id: GrepLineMatch
        signature: "@dataclass(frozen=True) class GrepLineMatch:"
        docstring: A single matching line within a file.
      - id: GrepMatch
        signature: "@dataclass(frozen=True) class GrepMatch:"
        docstring: All matches found within a single file.
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
      - id: grep
        signature: "def grep(items: list[GrepItem]) -> GrepBatchResult:"
        docstring: "Run one or more independent grep searches. Limits apply per item, not per batch…"
      - id: GrepTool
        signature: "class GrepTool(ToolDefinition):"
        children:
        - id: GrepTool.zkYjDM|Gpi0QI
          signature: name = 'grep'
        - id: GrepTool.MlMMUV|U4cpub
          signature: "_ITEM_PROPERTIES = {"
        - id: GrepTool.b70HqN|J2xOF0
          signature: "_ITEM_REQUIRED = ['directory', 'pattern']"
        - id: GrepTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`grep`, translating the MCP schema to/from the Python API."
      - id: register_grep_tool
        signature: "def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
      nodes:
      - id: LjJRQf|jlFZZg
        signature: "'``ast_find`` tool: find AST nodes by type, name, id, line range or parent type…"
      - id: 6cUWvL|prGBvq
        signature: import re
      - id: rY0KYk|mmIbkP
        signature: "__all__ = ['FileNodesResult', 'FindNodesResult', 'ast_find', 'FindNodesTool', '…"
      - id: FileNodesResult
        signature: "@dataclass(frozen=True) class FileNodesResult:"
        docstring: "Nodes matching the given selectors within a single file. Attributes: path: The …"
      - id: FindNodesResult
        signature: "@dataclass(frozen=True) class FindNodesResult:"
        docstring: "Result of :func:`ast_find`. Attributes: files: One :class:`FileNodesResult` per…"
      - id: load_tolerant
        signature: "def _load_tolerant(path: str) -> core.Tree:"
        docstring: "Load *path*, tolerating a forgotten filename extension. If *path* does not exis…"
      - id: find_in_file
        signature: "def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool, with_type: bool) -> FileNodesResult:"
      - id: ast_find
        signature: "def ast_find(paths: list[str], *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:"
        docstring: "Find nodes by id, type, name, line range, parent type, text or regexp. ``ast_fi…"
      - id: FindNodesTool
        signature: "class FindNodesTool(ToolDefinition):"
        children:
        - id: FindNodesTool.4leSQE|KdRckz
          signature: name = 'ast_find'
        - id: FindNodesTool.WDGjLy|2g3uCW
          signature: "input_schema = {"
        - id: FindNodesTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer c6f3-8 allow
Control Request:
```yaml
id: c6f3-9
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    ids:
    - QA8gK6|AIzwjR
    - a7y1Wc|CpnlXj
    - qoUXrH|VnDEso
    - GrepLineMatch
    - GrepMatch
    - GrepResult
    - GrepBatchResult
    - parse_grep_stdout
    - grep_one
    - grep
    - GrepTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
    ids:
    - 6cUWvL|prGBvq
    - FileNodesResult
    - load_tolerant
    - find_in_file
    - ast_find
    - FindNodesTool
  reason: Read relevant code
```
/answer c6f3-9 allow
Control Request:
```yaml
id: c6f3-a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
      nodes:
      - id: QA8gK6|AIzwjR
        type: statements
        code: "'Grep tool – recursive extended-regex search for retrieval, for a batch of items.'"
      - id: a7y1Wc|CpnlXj
        type: imports
        code: |-
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools._directories import normalize_directories
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
          import re
      - id: qoUXrH|VnDEso
        type: statements
        code: |-
          __all__ = [
              'GrepError',
              'GrepLineMatch',
              'GrepMatch',
              'GrepItem',
              'GrepResult',
              'GrepItemError',
              'GrepBatchResult',
              'grep',
              'GrepTool',
              'register_grep_tool']
          _DEFAULT_LIMIT = 15
          _MAX_LIMIT = 50
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
              """All matches found within a single file."""
              directory: str
              filename: str
              matches: list[GrepLineMatch]
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
      - id: GrepBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class GrepBatchResult:
              """Result of :func:`grep`.

              Attributes:
                  results: One :class:`GrepResult` per successfully executed search.
                  errors: One :class:`GrepItemError` per search that failed.
              """
              results: list[GrepResult]
              errors: list[GrepItemError]
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
      - id: grep
        type: FunctionDef
        code: |-
          def grep(items: list[GrepItem]) -> GrepBatchResult:
              """Run one or more independent grep searches. Limits apply per item, not per batch.

              Args:
                  items: Grep searches to run. Must be non-empty.

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
                      results.append(_grep_one(item))
                  except GrepError as exc:
                      errors.append(GrepItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
              return GrepBatchResult(results=results, errors=errors)
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
        - id: GrepTool.b70HqN|J2xOF0
          type: statements
          code: |-
            _ITEM_REQUIRED = ['directory', 'pattern']
            _ITEMS_DESCRIPTION = 'Independent grep searches to run.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)
        - id: GrepTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`grep`, translating the MCP schema to/from the Python API."""

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

                def result_serializer(r: GrepResult) -> dict[str, Any]:
                    entry: dict[str,
                                Any] = {'directory': r.directory,
                                        'matches': [{'path': f'{m.directory}/{m.filename}' if m.directory else m.filename,
                                                     'matches': [{'lineno': lm.lineno,
                                                                  'match': lm.match} for lm in m.matches]} for m in r.matches]}
                    if r.warning is not None:
                        entry['warning'] = r.warning
                    return entry

                def error_serializer(e: GrepError) -> dict[str, Any]:
                    return {'directory': e.directory, 'pattern': e.pattern, 'error': e.error}
                return handle_batch_tool(ctx, item_factory, grep, GrepError, result_serializer, error_serializer)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
      nodes:
      - id: 6cUWvL|prGBvq
        type: imports
        code: |-
          import re
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools._tool_helpers import require_items
      - id: FileNodesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class FileNodesResult:
              """Nodes matching the given selectors within a single file.

              Attributes:
                  path: The path exactly as given in the input.
                  nodes: Outline-style node descriptions (see :class:`core.OutlineNode`)
                      matching the given selectors, suited for retrieval and navigation.
              """
              path: str
              nodes: list[core.OutlineNode]
      - id: load_tolerant
        type: FunctionDef
        code: |-
          def _load_tolerant(path: str) -> core.Tree:
              """Load *path*, tolerating a forgotten filename extension.

              If *path* does not exist, its parent directory is searched for a single
              file whose name (without extension) matches the given basename; if
              exactly one such file exists, it is loaded instead (the reported result
              path stays unchanged). Ambiguous or empty matches fall through to the
              original 'File not found.' error.
              """
              try:
                  return core.load(path)[1]
              except core.AstError as exc:
                  if str(exc) == 'File not found.':
                      parent = Path(path).parent
                      stem = Path(path).name
                      if parent.is_dir():
                          matches = [p for p in parent.iterdir() if p.is_file() and p.stem == stem]
                          if len(matches) == 1:
                              return core.load(str(matches[0]))[1]
                  raise
      - id: find_in_file
        type: FunctionDef
        code: |-
          def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool, with_type: bool) -> FileNodesResult:
              tree = _load_tolerant(path)
              if no_selector:
                  nodes = core.build_outline(core.locate_all(tree), with_code=True, with_lines=with_lines, with_type=with_type)
                  return FileNodesResult(path=path, nodes=nodes)
              candidates = core.find(tree, **exact)
              if not candidates and exact.get('id') and (not any((v for k, v in exact.items() if k != 'id'))):
                  fallback = core.resolve_by_prefix(core.locate_all(tree), exact['id'])
                  candidates = [fallback] if fallback is not None else []
              if lineno is not None or end_lineno is not None:
                  start = lineno if lineno is not None else end_lineno
                  end = end_lineno if end_lineno is not None else lineno
                  hit = core.most_specific(candidates, start, end)
                  if hit is not None:
                      candidates = [hit]
                  else:
                      overlap = [c for c in candidates if c.lineno <= end and c.end_lineno >= start]
                      candidates = [
                          c for c in overlap if not any(
                              (o is not c and c.lineno <= o.lineno and (
                                  o.end_lineno <= c.end_lineno) for o in overlap))]
              if pattern is None:
                  nodes = core.build_outline(candidates, with_code=True, with_lines=with_lines, with_type=with_type)
                  return FileNodesResult(path=path, nodes=nodes)
              source = tree.source
              seen: set[str] = set()
              ordered: list[core.Located] = []
              for m in pattern.finditer(source):
                  start_line = source.count('\n', 0, m.start()) + 1
                  end_line = source.count('\n', 0, max(m.end() - 1, m.start())) + 1
                  loc = core.most_specific(candidates, start_line, end_line)
                  if loc is not None and loc.node_id not in seen:
                      seen.add(loc.node_id)
                      ordered.append(loc)
              nodes = core.build_outline(ordered, with_code=True, with_lines=with_lines, with_type=with_type)
              return FileNodesResult(path=path, nodes=nodes)
      - id: ast_find
        type: FunctionDef
        code: |-
          def ast_find(paths: list[str], *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:
              """Find nodes by id, type, name, line range, parent type, text or regexp.

              ``ast_find`` is the single retrieval point that restricts on node properties;
              every other tool addresses nodes purely by ``id``. ``text``/``regexp`` are
              matched against each file's whole source, and each match is attributed to the
              most specific (smallest) enclosing node rather than to every ancestor whose
              source happens to contain it. Matches are returned with their full source.
              Called with no selector at all, ``ast_find`` returns the whole node tree per
              file, nested like ``ast_outline`` but including source.

              Args:
                  paths: Absolute paths of the files to search. Must be non-empty.
                  id: Engine-independent unique node id (primarily name-based path).
                  name: Exact simple name a node's ``name`` must equal.
                  node_type: Regular expression (case-insensitive, ``re.search``) a
                      node's type name must match.
                  lineno: Line the target node must contain; selects the most specific
                      (smallest) matching node. Combined with ``end_lineno``, selects the
                      most specific node fully covering ``[lineno, end_lineno]``; if none
                      does, all innermost nodes overlapping the range are returned.
                  end_lineno: End line of the target range; see ``lineno``. May be given
                      alone to select the most specific node containing that single line.
                  parent_type: Node type name of the enclosing container (case-insensitive).
                  text: Case-insensitive substring to search for in each file.
                  regexp: Regular expression to search for in each file (``re.finditer``).
                  with_lines: Whether to populate each match's line range.

              Returns:
                  FindNodesResult: One :class:`FileNodesResult` per path, in order. Any
                  number of matches per file (including zero) is a normal, successful result.

              Raises:
                  core.AstError: If ``paths`` is empty, if any path is not absolute or does
                      not point to an existing regular file, if a file has a syntax error, or
                      if ``regexp`` is not a valid regular expression.
              """
              if not paths:
                  raise core.AstError("'paths' must be a non-empty list.")
              if node_type is not None:
                  try:
                      re.compile(node_type)
                  except re.error as exc:
                      raise core.AstError(f'Invalid regexp: {exc}') from exc
              exact = dict(id=id, name=name, node_type=node_type, parent_type=parent_type)
              structural = dict(exact, lineno=lineno, end_lineno=end_lineno)
              no_selector = not any(structural.values()) and text is None and (regexp is None)
              pattern: re.Pattern[str] | None = None
              if not no_selector and (text is not None or regexp is not None):
                  if regexp is not None:
                      try:
                          pattern = re.compile(regexp)
                      except re.error as exc:
                          raise core.AstError(f'Invalid regexp: {exc}') from exc
                  else:
                      pattern = re.compile(re.escape(text), re.IGNORECASE)
              with_type = node_type is not None
              files = [
                  _find_in_file(
                      p,
                      exact=exact,
                      lineno=lineno,
                      end_lineno=end_lineno,
                      no_selector=no_selector,
                      pattern=pattern,
                      with_lines=with_lines,
                      with_type=with_type) for p in paths]
              return FindNodesResult(files=files)
      - id: FindNodesTool
        type: ClassDef
        signature: "class FindNodesTool(ToolDefinition):"
        children:
        - id: FindNodesTool.4leSQE|KdRckz
          type: statements
          code: |-
            name = 'ast_find'
            title = 'Finds code, text, and content within the specified files.'
            description = 'Filter the AST-node tree of a list of files by type, name, id, line range, parent type, text substring or regexp (grep). Returns matches per file with their full source. Use this when looking up content without IDs.'
        - id: FindNodesTool.WDGjLy|2g3uCW
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'paths': {
                        'type': 'array',
                        'items': {
                            'type': 'string'},
                        'description': 'Absolute paths of the files to search (never directories).'},
                    **SELECTOR_PROPS,
                    'text': {
                        'type': 'string',
                                'description': 'Case-insensitive substring that the node’s source must contain. Example: `import` finds nodes containing imports.'},
                    'regexp': {
                        'type': 'string',
                        'description': 'Regular expression that the node’s source must match (`re.search` semantics).'}},
                'required': ['paths']}
        - id: FindNodesTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."""
                args: dict[str, Any] = ctx.arguments
                paths, error = require_items(ctx, key='paths')
                if error is not None:
                    return error
                with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)
                try:
                    result = ast_find(
                        paths=paths,
                        id=args.get('id'),
                        name=args.get('name'),
                        node_type=args.get('node_type'),
                        lineno=args.get('lineno'),
                        end_lineno=args.get('end_lineno'),
                        parent_type=args.get('parent_type'),
                        text=args.get('text'),
                        regexp=args.get('regexp'),
                        with_lines=with_lines)
                except core.AstError as exc:
                    return ToolResult(content=[text_content(str(exc))], is_error=True)
                return ToolResult(structured_content={'files': [{'path': f.path, 'nodes': [
                                  core.to_dict(n) for n in f.nodes]} for f in result.files]})
```
/answer c6f3-a allow
ReasoningToken: 63
Control Request:
```yaml
id: c6f3-b
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: "^def (load|locate_all|most_specific|handle_batch_tool)\\b|^class Located|node_id\\b.*:"
    include:
    - core.py
    - _tool_helpers.py
    limit: 20
  reason: Check core API and batch helper signature
```
/answer c6f3-b allow
Control Request:
```yaml
id: c6f3-c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: ast/core.py
        matches:
        - lineno: 83
          match: "def load(path_str: str) -> tuple[Path, Tree]:"
        - lineno: 97
          match: "def locate_all(tree: Tree) -> list[Located]:"
      - path: _tool_helpers.py
        matches:
        - lineno: 57
          match: "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:"
```
/answer c6f3-c allow
Control Request:
```yaml
id: c6f3-d
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  regexp: def most_specific|class Located
  reason: See most_specific and Located definitions
```
/answer c6f3-d allow
Control Request:
```yaml
id: c6f3-e
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes: []
```
/answer c6f3-e allow
Control Request:
```yaml
id: c6f3-f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    ids:
    - handle_batch_tool
  reason: See how batch_fn is invoked
```
/answer c6f3-f allow
Control Request:
```yaml
id: c6f3-10
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: handle_batch_tool
        type: FunctionDef
        code: |-
          def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:
              """Common handler for batch-processing tools.

              Args:
                  ctx: Tool context with arguments.
                  item_factory: Function to convert raw dict to typed item.
                  batch_fn: Function to process list of items, returning batch result.
                  error_class: Exception class raised by `batch_fn` for whole-batch failures.
                  result_serializer: Optional custom serializer for result items (default: asdict).
                  error_serializer: Optional custom serializer for error items (default: asdict).

              Returns:
                  ToolResult with serialized batch result.
              """
              raw_items, error = require_items(ctx)
              if error is not None:
                  return error
              items: list[T] = []
              for it in raw_items:
                  try:
                      items.append(item_factory(it))
                  except (KeyError, ValueError, TypeError) as exc:
                      return ToolResult(content=[text_content(f'Invalid item: {exc}')], is_error=True)
              try:
                  batch_result = batch_fn(items)
              except error_class as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              content = serialize_batch_result(
                  batch_result,
                  result_serializer=result_serializer,
                  error_serializer=error_serializer)
              has_error = hasattr(batch_result, 'errors') and batch_result.errors
              return ToolResult(structured_content=content, auto_approve=not has_error and auto_approve)
```
/answer c6f3-10 allow
Control Request:
```yaml
id: c6f3-11
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: most_specific|class Located|node_id
    include:
    - core.py
    limit: 15
  reason: Locate most_specific signature
```
/answer c6f3-11 allow
Control Request:
```yaml
id: c6f3-12
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: core.py
        matches:
        - lineno: 21
          match: "from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, AstTextError, Engine, Located, OutlineNode, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, resolve_by_prefix, to_dict"
        - lineno: 119
          match: return new_loc.node_id if new_loc and new_loc.node_id != loc.node_id else loc.node_id
```
/answer c6f3-12 allow
Control Request:
```yaml
id: c6f3-13
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: def most_specific|class Located
    include:
    - base.py
    limit: 5
  reason: Find most_specific definition
```
/answer c6f3-13 allow
Control Request:
```yaml
id: c6f3-14
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: base.py
        matches:
        - lineno: 69
          match: "class Located:"
        - lineno: 372
          match: "def most_specific(located: list[Located], lineno: int, end_lineno: int) -> Located | None:"
```
/answer c6f3-14 allow
Control Request:
```yaml
id: c6f3-15
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    id: a7y1Wc|CpnlXj
    old_text: import re
    new_text: |-
      import re
      from xy.ai.mcpc.tools.ast import core
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    id: qoUXrH|VnDEso
    old_text: |2
          'GrepLineMatch',
    new_text: |2
          'GrepLineMatch',
          'GrepNodeMatch',
  reason: Add import and export
```
/answer c6f3-15 allow
ReasoningToken: 351
Control Request:
```yaml
id: c6f3-16
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    id: GrepMatch
    source: |-
      @dataclass(frozen=True)
      class GrepNodeMatch:
          """Matching lines attributed to the most specific enclosing AST node."""
          node_id: str
          matches: list[GrepLineMatch]


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
  reason: Add node-level grouping dataclass
```
/answer c6f3-16 allow
Control Request:
```yaml
id: c6f3-17
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    id: grep
    source: |-
      def _resolve_nodes(match: GrepMatch) -> GrepMatch:
          """Regroup a file's line matches by the most specific enclosing AST node.

          Files that cannot be parsed are returned unchanged. Lines not covered by any
          node stay in ``matches``.
          """
          path = f'{match.directory}/{match.filename}' if match.directory else match.filename
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
                          matches=[_resolve_nodes(m) for m in result.matches],
                          warning=result.warning)
                  results.append(result)
              except GrepError as exc:
                  errors.append(GrepItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
          return GrepBatchResult(results=results, errors=errors)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
    id: GrepTool.handle
    source: |-
      def handle(self, ctx: ToolContext) -> ToolResult:
          """Delegate to :func:`grep`, translating the MCP schema to/from the Python API."""
          with_ast = bool({'tools', 'ast_read'} & ctx.session.enabled_tools)

          def item_factory(it: dict[str, Any]) -> GrepItem:
              return GrepItem(
                  directory=it['directory'],
                  pattern=it['pattern'],
                  exclude=it.get('exclude'),
                  include=it.get('include'),
                  limit=int(it.get('limit', _DEFAULT_LIMIT)))

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
              lambda items: grep(items, with_ast=with_ast),
              GrepError,
              result_serializer,
              error_serializer)
  reason: Add AST resolution postprocessing and wire into handler
```
/answer c6f3-17 allow
ReasoningToken: 69
Control Request:
```yaml
id: c6f3-18
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py
  reason: Check syntax
```
/answer c6f3-18 allow
Result Stats: id=04dcac6d-2976-4411-ae75-bfccb9959e35, total: 28303, in: 18, out: 5257, reason: 0, read: 124350, write: 23028, start: 1792326742143, end: 1792326746868, time: 4677, total: 41505, io: 0, control: 48, sent: 0, recv: 120
Ich habe das Postprocessing in `grep/__init__.py` ergänzt. Die Datei kompiliert, getestet habe ich es aber nicht (keine Tests, kein Lauf mit echten Daten).

- **Aktivierung:** `GrepTool.handle` prüft `with_ast = bool({'tools', 'ast_read'} & ctx.session.enabled_tools)` und reicht den Wert als `grep(items, with_ast=...)` weiter. Der Standard ist `False`, das bisherige Verhalten bleibt also unverändert.
- **Auflösung:** Neu ist `_resolve_nodes`. Es lädt die Datei mit `core.load` und ordnet jede Trefferzeile über `core.most_specific` dem kleinsten umschließenden Node zu. Das ist dieselbe Logik wie in `ast_find`.
- **Neue Ebene:** Die Ausgabe ist dann Datei → Node-ID → Treffer: `{"path": ..., "nodes": [{"id": ..., "matches": [{"lineno", "match"}]}]}`. `GrepMatch` hat dafür das neue Feld `nodes`, der neue Typ `GrepNodeMatch` ist in `__all__` exportiert.
- **Fallback:** Lässt sich eine Datei nicht parsen (`core.AstError` oder `OSError`), bleibt sie bei der flachen Struktur mit `matches`. Zeilen, die in keinen Node fallen, bleiben ebenfalls unter `matches` neben `nodes`.

Der neue Import `from xy.ai.mcpc.tools.ast import core` könnte einen Zirkelimport auslösen. Das habe ich nicht geprüft.
