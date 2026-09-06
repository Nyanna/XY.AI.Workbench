Erhöhe die Toleranz von `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py`. Wie im Beispiel sichtbar hat ein Agent die Dateiendung vergessen. Es gibt jedoch nur eine Datei namens "tool_registry.py" in diesem Verzeichnis. Dies kann relativ sicher antizipiert werden. 

Beispiel:
```yaml
id: f727-5
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry
  reason: ToolResult/text_content Definition prüfen für korrekte Nutzung von structured_content und is_error"
  node_type: ClassDef
  
id: f727-6
result:
  content:
  - type: text
    text: File not found.
  isError: true
```
SystemInit:  id=303a4280-9593-4764-9731-8416ca836af0, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: b9ea-1
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: "Aktuelle Implementierung von ast_find prüfen, um Datei-Lookup-Logik für fehlende Dateiendung anzupassen"
```
/answer b9ea-1 allow
Control Request:
```yaml
id: b9ea-2
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
      nodes:
      - id: jlFZZg
        type: statements
        code: "'``ast_find`` tool: find AST nodes by type, name, id, line range or parent type.'"
      - id: JqX5Pc
        type: imports
        code: |-
          import re
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: mmIbkP
        type: statements
        code: "__all__ = ['FileNodesResult', 'FindNodesResult', 'ast_find', 'FindNodesTool', 'register']"
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
      - id: FindNodesResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class FindNodesResult:
              """Result of :func:`ast_find`.

              Attributes:
                  files: One :class:`FileNodesResult` per input path, in order.
              """
              files: list[FileNodesResult]
      - id: find_in_file
        type: FunctionDef
        code: |-
          def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool) -> FileNodesResult:
              tree = core.load(path)[1]
              if no_selector:
                  nodes = core.build_outline(core.locate_all(tree), with_code=True, with_lines=with_lines)
                  return FileNodesResult(path=path, nodes=nodes)
              candidates = core.find(tree, **exact)
              if lineno is not None or end_lineno is not None:
                  start = lineno if lineno is not None else end_lineno
                  end = end_lineno if end_lineno is not None else lineno
                  hit = core.most_specific(candidates, start, end)
                  candidates = [hit] if hit is not None else []
              if pattern is None:
                  nodes = [core.node_outline(h, with_code=True, with_lines=with_lines) for h in candidates]
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
              nodes = [core.node_outline(h, with_code=True, with_lines=with_lines) for h in ordered]
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
              file, nested like ``ast_list`` but including source.

              Args:
                  paths: Absolute paths of the files to search. Must be non-empty.
                  id: Engine-independent unique node id (primarily name-based path).
                  name: Exact simple name a node's ``name`` must equal.
                  node_type: Node type name a node must match (case-insensitive).
                  lineno: Line the target node must contain; selects the most specific
                      (smallest) matching node. Combined with ``end_lineno``, selects the
                      most specific node fully covering ``[lineno, end_lineno]``.
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
              files = [
                  _find_in_file(
                      p,
                      exact=exact,
                      lineno=lineno,
                      end_lineno=end_lineno,
                      no_selector=no_selector,
                      pattern=pattern,
                      with_lines=with_lines) for p in paths]
              return FindNodesResult(files=files)
      - id: FindNodesTool
        type: ClassDef
        signature: "class FindNodesTool(ToolDefinition):"
        children:
        - id: FindNodesTool.vMKkuI
          type: statements
          code: |-
            name = 'ast_find'
            title = 'Find AST nodes'
            description = 'Filter the AST-node tree of a list of files by type, name, id, line range, parent type, text substring or regexp. Returns matches per file with their full source.'
        - id: FindNodesTool.nplwMP
          type: statements
          code: |-
            input_schema = {
                'type': 'object',
                'properties': {
                    'paths': {
                        'type': 'array',
                        'items': {
                            'type': 'string'},
                        'description': 'Absolute paths of the files to search.'},
                    **SELECTOR_PROPS,
                    'text': {
                        'type': 'string',
                                'description': "Case-insensitive substring the node's source must contain."},
                    'regexp': {
                        'type': 'string',
                        'description': "Regular expression the node's source must match (re.search)."}},
                'required': ['paths']}
        - id: FindNodesTool.ZaIxOu
          type: statements
          code: |-
            output_schema = {
                '$defs': {
                    'outline_node': core.OUTLINE_NODE_SCHEMA},
                'type': 'object',
                'properties': {
                        'files': {
                            'type': 'array',
                            'items': {
                                'type': 'object',
                                'properties': {
                                    'path': {
                                        'type': 'string'},
                                    'nodes': {
                                        'type': 'array',
                                        'items': {
                                                '$ref': '#/$defs/outline_node'}}},
                                'required': [
                                    'path',
                                    'nodes']}}},
                'required': ['files']}
        - id: FindNodesTool.e5owbr
          type: statements
          code: "annotations = {'readOnlyHint': True, 'openWorldHint': False}"
        - id: FindNodesTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."""
                args: dict[str, Any] = ctx.arguments
                paths = args.get('paths')
                if not isinstance(paths, list):
                    return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
                with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
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
      - id: register
        type: FunctionDef
        code: |-
          def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(FindNodesTool())
              functions.register(ast_find)
```
/answer b9ea-2 allow
ReasoningToken: 21
Control Request:
```yaml
id: b9ea-3
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: "Implementierung von core.load prüfen, um dort die Toleranz für fehlende Dateiendung zu ergänzen"
  name: load
```
/answer b9ea-3 allow
Control Request:
```yaml
id: b9ea-4
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
      - id: load
        type: FunctionDef
        code: |-
          def load(path_str: str) -> tuple[Path, Tree]:
              """Resolve *path_str* and return it together with its cached tree."""
              path = require_path(path_str)
              return (path, CACHE.get_tree(path))
```
/answer b9ea-4 allow
ReasoningToken: 20
Control Request:
```yaml
id: b9ea-5
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: "require_path Implementierung prüfen, um Fallback-Logik für fehlende Dateiendung zu ergänzen"
  name: require_path
```
/answer b9ea-5 allow
Control Request:
```yaml
id: b9ea-6
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes: []
```
/answer b9ea-6 allow
Control Request:
```yaml
id: b9ea-7
toolName: grep
arguments:
  directory:
  - /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: def require_path
  include:
  - '*.py'
  exclude:
  - '*.min.py'
  reason: "Definition von require_path finden, um die Datei-Auflösungslogik zu ergänzen"
```
/answer b9ea-7 allow
Control Request:
```yaml
id: b9ea-8
result:
  structuredContent:
    matches:
    - path: xy/ai/mcpc/tools/ast/base.py
      lineno: 379
      match: "def require_path(path_str: str, *, must_exist: bool=True) -> Path:"
```
/answer b9ea-8 allow
Control Request:
```yaml
id: b9ea-9
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: "require_path Implementierung lesen, um Fallback für fehlende Dateiendung dort oder in find.py zu ergänzen"
  name: require_path
```
/answer b9ea-9 allow
Control Request:
```yaml
id: b9ea-a
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: require_path
        type: FunctionDef
        code: |-
          def require_path(path_str: str, *, must_exist: bool=True) -> Path:
              """Validate a mandatory absolute path, raising :class:`AstError` on failure."""
              path = Path(path_str)
              if not path.is_absolute():
                  raise AstError('Path must be absolute.')
              if must_exist:
                  if not path.exists():
                      raise AstError('File not found.')
                  if not path.is_file():
                      raise AstError('Not a regular file.')
              return path
```
/answer b9ea-a allow
Control Request:
```yaml
id: b9ea-b
toolName: ast_edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  id: find_in_file
  start_marker: "def _find_in_file(path: str, *, exact:"
  end_marker: "tree = core.load(path)[1]"
  content: |-
    def _load_tolerant(path: str) -> core.Tree:
        """Load *path*, tolerating a missing '.py' extension if unambiguous.

        If *path* does not exist but appending '.py' points to an existing file,
        that file is loaded instead (the reported result path stays unchanged).
        """
        try:
            return core.load(path)[1]
        except core.AstError as exc:
            if str(exc) == 'File not found.' and not path.endswith('.py'):
                try:
                    return core.load(path + '.py')[1]
                except core.AstError:
                    pass
            raise


    def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool) -> FileNodesResult:
        tree = _load_tolerant(path)
  reason: "Toleranz für fehlende '.py'-Dateiendung in ast_find ergänzen, wie vom Nutzer gefordert"
```
/answer b9ea-b allow
ReasoningToken: 1091
Control Request:
```yaml
id: b9ea-c
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: Syntaxvalidität nach Änderung prüfen
```
/answer b9ea-c deny Das ist ein Missverständnis, das gilt nicht nur für Python Dateien. In so einem Fall soll versucht werden auf ein Verzeichnis zu schließen und eine Datei zu finden die ohne Endung dazu passt.
Control Request:
```yaml
id: b9ea-d
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  ids:
  - find_in_file
  reason: "Aktuellen Stand der Funktion lesen, um generischere Verzeichnis-basierte Toleranz zu implementieren"
```
/answer b9ea-d allow
Control Request:
```yaml
id: b9ea-e
result:
  structuredContent:
    nodes:
    - id: find_in_file
      type: FunctionDef
      code: |-
        def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool) -> FileNodesResult:
            tree = _load_tolerant(path)
            if no_selector:
                nodes = core.build_outline(core.locate_all(tree), with_code=True, with_lines=with_lines)
                return FileNodesResult(path=path, nodes=nodes)
            candidates = core.find(tree, **exact)
            if lineno is not None or end_lineno is not None:
                start = lineno if lineno is not None else end_lineno
                end = end_lineno if end_lineno is not None else lineno
                hit = core.most_specific(candidates, start, end)
                candidates = [hit] if hit is not None else []
            if pattern is None:
                nodes = [core.node_outline(h, with_code=True, with_lines=with_lines) for h in candidates]
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
            nodes = [core.node_outline(h, with_code=True, with_lines=with_lines) for h in ordered]
            return FileNodesResult(path=path, nodes=nodes)
```
/answer b9ea-e allow
ReasoningToken: 542
Control Request:
```yaml
id: b9ea-f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: "Node-ID der _load_tolerant Funktion ermitteln, um sie generisch zu überarbeiten"
  name: _load_tolerant
```
/answer b9ea-f allow
Control Request:
```yaml
id: b9ea-10
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
      nodes:
      - id: load_tolerant
        type: FunctionDef
        code: |-
          def _load_tolerant(path: str) -> core.Tree:
              """Load *path*, tolerating a missing '.py' extension if unambiguous.

              If *path* does not exist but appending '.py' points to an existing file,
              that file is loaded instead (the reported result path stays unchanged).
              """
              try:
                  return core.load(path)[1]
              except core.AstError as exc:
                  if str(exc) == 'File not found.' and (not path.endswith('.py')):
                      try:
                          return core.load(path + '.py')[1]
                      except core.AstError:
                          pass
                  raise
```
/answer b9ea-10 allow
ReasoningToken: 19
Text:
 Ich brauche `Path` und `Tree`-Import; prüfe vorhandene Imports.
Control Request:
```yaml
id: b9ea-11
toolName: ast_replace
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  id: load_tolerant
  source: |-
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
  reason: "Generische verzeichnisbasierte Toleranz statt hartkodierter '.py'-Endung implementieren, wie vom Nutzer klargestellt"
```
/answer b9ea-11 allow
Control Request:
```yaml
id: b9ea-12
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: "Import-Block prüfen, ob Path bereits importiert ist"
  node_type: imports
```
/answer b9ea-12 allow
Control Request:
```yaml
id: b9ea-13
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
      nodes:
      - id: JqX5Pc
        type: imports
        code: |-
          import re
          from dataclasses import dataclass
          from typing import Any
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
          from xy.ai.mcpc.tools.tool_context import ToolContext
          from xy.ai.mcpc.tools.ast import core
          from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
```
/answer b9ea-13 allow
Text:
 Jetzt `Path`-Import ergänzen.
Control Request:
```yaml
id: b9ea-14
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  id: JqX5Pc
  old_text: |-
    import re
    from dataclasses import dataclass
  new_text: |-
    import re
    from dataclasses import dataclass
    from pathlib import Path
  reason: Path für generische Verzeichnis-Fallback-Logik in find.py importieren
```
/answer b9ea-14 allow
Control Request:
```yaml
id: b9ea-15
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: Syntaxvalidität der geänderten Datei prüfen
```
/answer b9ea-15 allow
Result Stats:  id=303a4280-9593-4764-9731-8416ca836af0, total: 0, in: 161, out: 5555, reason: 0, read: 171128, write: 16719
Erledigt: `ast_find` toleriert nun eine fehlende Dateiendung generisch – bei nicht existierendem Pfad wird das Elternverzeichnis nach genau einer Datei mit passendem Stamm-Namen durchsucht und diese geladen.
