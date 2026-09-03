Prüfe das Tool `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py`. Initial für die Python AST Engine `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/__init__.py` angelegt, soll sie auch für `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py` verwendet werden können. Die Kompatibilität ist zu prüfen und die Beschreibungen.
SystemInit:  id=e14729e9-4fb7-40d1-bfb4-8146338313f2, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: cc16-1
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  ids:
  - root
  reason: Tool script.py analysieren für Kompatibilitätsprüfung
```
/answer cc16-1 allow
Control Request:
```yaml
id: cc16-2
result:
  content:
  - type: text
    text: No node matched 'root'.
  isError: true
```
/answer cc16-2 allow
Control Request:
```yaml
id: cc16-3
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/__init__.py
  reason: Struktur der Python AST Engine für script.py Kompatibilität prüfen
```
/answer cc16-3 allow
Control Request:
```yaml
id: cc16-4
result:
  structuredContent:
    nodes:
    - id: j10hxN
      type: statements
    - id: 5zERFW
      type: imports
    - id: Rendl1
      type: statements
```
/answer cc16-4 allow
Control Request:
```yaml
id: cc16-5
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
  reason: Struktur der Generic AST Engine für Vergleich mit Python-Engine prüfen
```
/answer cc16-5 allow
Control Request:
```yaml
id: cc16-6
result:
  structuredContent:
    nodes:
    - id: 0eLqlO
      type: statements
    - id: gzS37E
      type: imports
    - id: jUkYYT
      type: statements
    - id: XdqYqL
      type: statements
    - id: language_for_extension
      type: FunctionDef
      signature: "def language_for_extension(ext: str) -> str | None:"
    - id: JFdKKW
      type: statements
    - id: TreeSitterEngine
      type: ClassDef
      signature: "class TreeSitterEngine(Engine):"
      docstring: One tree-sitter grammar exposed through the common :class:`Engine` API. Instanc…
      children:
      - id: TreeSitterEngine.jU5ePL
        type: statements
      - id: TreeSitterEngine.init
        type: FunctionDef
        signature: "def __init__(self, symbol: str) -> None:"
      - id: TreeSitterEngine.parse
        type: FunctionDef
        signature: "def _parse(self, data: bytes):"
      - id: TreeSitterEngine.parse_1
        type: FunctionDef
        signature: "def parse(self, source: str, path: Path | None=None) -> Tree:"
      - id: TreeSitterEngine.empty_tree
        type: FunctionDef
        signature: "def empty_tree(self, path: Path | None=None) -> Tree:"
      - id: TreeSitterEngine.serialize
        type: FunctionDef
        signature: "def serialize(self, tree: Tree) -> str:"
      - id: TreeSitterEngine.validate
        type: FunctionDef
        signature: "def validate(self, source: str) -> str | None:"
      - id: TreeSitterEngine.name
        type: FunctionDef
        signature: "def _name(self, node: Any) -> str | None:"
      - id: TreeSitterEngine.clean_heading
        type: FunctionDef
        signature: "@staticmethod def _clean_heading(raw: bytes) -> str:"
      - id: TreeSitterEngine.clean
        type: FunctionDef
        signature: "@staticmethod def _clean(raw: bytes) -> str:"
      - id: TreeSitterEngine.locate_all
        type: FunctionDef
        signature: "def locate_all(self, tree: Tree) -> list[Located]:"
      - id: TreeSitterEngine.signature
        type: FunctionDef
        signature: "def signature(self, node: Any, limit: int=80) -> str:"
      - id: TreeSitterEngine.docstring
        type: FunctionDef
        signature: "def docstring(self, node: Any) -> str | None:"
      - id: TreeSitterEngine.node_code
        type: FunctionDef
        signature: "def node_code(self, node: Any) -> str:"
      - id: TreeSitterEngine.splice
        type: FunctionDef
        signature: "def _splice(self, tree: Tree, start: int, end: int, text: str) -> None:"
      - id: TreeSitterEngine.replace
        type: FunctionDef
        signature: "def replace(self, loc: Located, code: str) -> None:"
      - id: TreeSitterEngine.insert
        type: FunctionDef
        signature: "def insert(self, loc: Located, code: str, position: str) -> int:"
      - id: TreeSitterEngine.delete
        type: FunctionDef
        signature: "def delete(self, loc: Located) -> None:"
      - id: TreeSitterEngine.append
        type: FunctionDef
        signature: "def append(self, tree: Tree, code: str) -> int:"
    - id: axJC4k
      type: statements
    - id: get_engine
      type: FunctionDef
      signature: "def get_engine(symbol: str) -> TreeSitterEngine:"
```
/answer cc16-6 allow
ReasoningToken: 14
Control Request:
```yaml
id: cc16-7
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  ids:
  - j10hxN
  - 5zERFW
  - Rendl1
  reason: Vollständigen Inhalt von script.py lesen
```
/answer cc16-7 allow
Control Request:
```yaml
id: cc16-8
result:
  content:
  - type: text
    text: No node matched 'j10hxN'.
  isError: true
```
/answer cc16-8 allow
ReasoningToken: 12
Control Request:
```yaml
id: cc16-9
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: IDs neu abrufen für script.py
```
/answer cc16-9 allow
Control Request:
```yaml
id: cc16-a
result:
  structuredContent:
    nodes:
    - id: saiGVi
      type: statements
    - id: 8t7Kee
      type: imports
    - id: uuST9t
      type: statements
    - id: HuXBMZ
      type: statements
    - id: ScriptError
      type: ClassDef
      signature: "class ScriptError(Exception):"
      docstring: Raised when an AST script cannot be run to completion.
    - id: AstScriptResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class AstScriptResult:"
      docstring: "Result of :func:`ast_script`. Attributes: result: Always ``\"success\"``. value: …"
    - id: ast_script
      type: FunctionDef
      signature: "def ast_script(path: str, code: str) -> AstScriptResult:"
      docstring: Execute ``code`` in a restricted sandbox exposing the AST of ``path`` as ``tree…
    - id: ScriptTool
      type: ClassDef
      signature: "class ScriptTool(ToolDefinition):"
      children:
      - id: ScriptTool.M4d1yn
        type: statements
      - id: ScriptTool.GglHto
        type: statements
      - id: ScriptTool.JYmhDk
        type: statements
      - id: ScriptTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_script`, translating the MCP schema to/from the Python A…"
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer cc16-a allow
ReasoningToken: 22
Control Request:
```yaml
id: cc16-b
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  ids:
  - saiGVi
  - 8t7Kee
  - uuST9t
  - HuXBMZ
  - ScriptError
  - AstScriptResult
  - ast_script
  - ScriptTool
  - register
  reason: Vollständigen Inhalt von script.py lesen für Kompatibilitätsprüfung
```
/answer cc16-b allow
Control Request:
```yaml
id: cc16-c
result:
  structuredContent:
    nodes:
    - id: saiGVi
      type: statements
      code: "'``ast_script`` – run restricted Python against the parsed AST.\\n\\nFor complex reorganisation/optimisation the model can operate on the tree\\ndirectly. The script runs with an empty ``__builtins__`` plus a small, curated\\nset of safe names; the only capability handed in is the AST itself (``tree``)\\nand the standard-library ``ast`` module. Any change to ``tree`` is persisted.\\n'"
    - id: 8t7Kee
      type: imports
      code: |-
        import ast
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: uuST9t
      type: statements
      code: "__all__ = ['ScriptError', 'AstScriptResult', 'ast_script', 'ScriptTool', 'register']"
    - id: HuXBMZ
      type: statements
      code: |-
        _SAFE_BUILTINS = {
            name: getattr(
                __builtins__,
                name,
                None) if not isinstance(
                    __builtins__,
                    dict) else __builtins__.get(name) for name in (
                        'isinstance',
                        'issubclass',
                        'getattr',
                        'setattr',
                        'hasattr',
                        'delattr',
                        'len',
                        'list',
                        'dict',
                        'set',
                        'tuple',
                        'str',
                        'int',
                        'float',
                        'bool',
                        'enumerate',
                        'range',
                        'sorted',
                        'reversed',
                        'zip',
                        'map',
                        'filter',
                        'any',
                        'all',
                        'min',
                        'max',
                        'sum',
                        'type',
                'repr')}
    - id: ScriptError
      type: ClassDef
      code: |-
        class ScriptError(Exception):
            """Raised when an AST script cannot be run to completion."""
    - id: AstScriptResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class AstScriptResult:
            """Result of :func:`ast_script`.

            Attributes:
                result: Always ``"success"``.
                value: ``repr()`` of the script's ``result`` variable, if the script set one;
                    otherwise ``None``.
            """
            result: str
            value: str | None = None
    - id: ast_script
      type: FunctionDef
      code: |-
        def ast_script(path: str, code: str) -> AstScriptResult:
            """Execute ``code`` in a restricted sandbox exposing the AST of ``path`` as ``tree``.

            ``code`` runs with an empty ``__builtins__`` plus a small, curated set of safe
            names (see ``_SAFE_BUILTINS``); the only capabilities handed in are the parsed
            tree (``tree``, an ``ast.Module``) and the standard-library ``ast`` module
            itself. Any mutation of ``tree`` is unparsed and persisted to ``path`` on
            success.

            Args:
                path: Absolute path to the file whose AST is exposed as ``tree``.
                code: Python script to execute against ``tree``. May assign a module-level
                    name ``result`` to return an arbitrary value (reported as its ``repr()``).

            Returns:
                AstScriptResult: Success status and, if the script set one, the ``repr()``
                of its ``result`` variable.

            Raises:
                core.AstError: If ``path`` is invalid.
                ScriptError: If ``code`` has a syntax error, or raises during execution.
            """
            file_path = core.require_path(path)
            tree = core.CACHE.get_tree(file_path)
            if tree.engine is not core.python.ENGINE:
                raise core.AstError('ast_script operates on the Python AST; it is only available for Python files.')
            env: dict[str, Any] = {'tree': tree.raw, 'ast': ast}
            sandbox_globals = {'__builtins__': _SAFE_BUILTINS}
            try:
                '# noqa: S102'
                exec(compile(code, '<ast-script>', 'exec'), sandbox_globals, env)
            except SyntaxError as exc:
                raise ScriptError(f'Script syntax error: {exc.msg}') from exc
            except Exception as exc:
                '# noqa: BLE001'
                raise ScriptError(f'Script failed: {type(exc).__name__}: {exc}') from exc
            core.CACHE.save(file_path, tree)
            if 'result' in env:
                return AstScriptResult(result='success', value=repr(env['result']))
            return AstScriptResult(result='success')
    - id: ScriptTool
      type: ClassDef
      signature: "class ScriptTool(ToolDefinition):"
      children:
      - id: ScriptTool.M4d1yn
        type: statements
        code: |-
          name = 'ast_script'
          title = 'Run AST python script'
          description = "Run a restricted Python script code against a file's AST for complex/incremental transforms. Globals expose 'tree' (ast.Module) and 'ast'; assign 'result' to return data. Changes to 'tree' are saved. Imports are not allowed."
      - id: ScriptTool.GglHto
        type: statements
        code: |-
          input_schema = {
              'type': 'object',
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute path to the Python file.'},
                  'code': {
                      'type': 'string',
                      'description': "Python script operating on 'tree';Environment is restricted; Don't use imports;"}},
              'required': [
                  'path',
                  'code']}
      - id: ScriptTool.JYmhDk
        type: statements
        code: |-
          output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}, 'value': {
              'description': "Repr of the script's 'result' variable, if set."}}, 'required': ['result']}
          annotations = {'readOnlyHint': False, 'openWorldHint': False}
      - id: ScriptTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_script`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              try:
                  result = ast_script(args['path'], args['code'])
              except (core.AstError, ScriptError) as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              structured: dict[str, Any] = {'result': result.result}
              if result.value is not None:
                  structured['value'] = result.value
              return ToolResult(structured_content=structured)
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(ScriptTool())
            functions.register(ast_script)
```
/answer cc16-c allow
Control Request:
```yaml
id: cc16-d
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: core.py Struktur prüfen für Tree/Engine Abstraktion
```
/answer cc16-d allow
Control Request:
```yaml
id: cc16-e
result:
  structuredContent:
    nodes:
    - id: ue626f
      type: statements
    - id: ZlvshC
      type: imports
    - id: RBFWwR
      type: statements
    - id: engine_for_path
      type: FunctionDef
      signature: "def engine_for_path(path: Path) -> Engine:"
      docstring: Return the engine responsible for ``path`` based on its extension.
    - id: CacheEntry
      type: ClassDef
      signature: "@dataclass class _CacheEntry:"
    - id: AstCache
      type: ClassDef
      signature: "class AstCache:"
      docstring: Content-hash validated cache of parsed trees keyed by absolute path.
      children:
      - id: AstCache.ayni5v
        type: statements
      - id: AstCache.init
        type: FunctionDef
        signature: "def __init__(self) -> None:"
      - id: AstCache.get_tree
        type: FunctionDef
        signature: "def get_tree(self, path: Path) -> Tree:"
      - id: AstCache.save
        type: FunctionDef
        signature: "def save(self, path: Path, tree: Tree) -> str:"
        docstring: "Serialise *tree*, write it to *path* and refresh the cache entry."
      - id: AstCache.invalidate
        type: FunctionDef
        signature: "def invalidate(self, path: Path) -> None:"
    - id: 5EG9eJ
      type: statements
    - id: load
      type: FunctionDef
      signature: "def load(path_str: str) -> tuple[Path, Tree]:"
      docstring: Resolve *path_str* and return it together with its cached tree.
    - id: parse_source
      type: FunctionDef
      signature: "def parse_source(source: str, engine: Engine | None=None) -> Tree:"
      docstring: Parse *source* with *engine* (Python by default for path-less snippets).
    - id: parse_for
      type: FunctionDef
      signature: "def parse_for(path_str: str, code: str) -> Tree:"
      docstring: Parse *code* with the engine selected for *path_str*'s extension.
    - id: locate_all
      type: FunctionDef
      signature: "def locate_all(tree: Tree) -> list[Located]:"
    - id: edit_node_source
      type: FunctionDef
      signature: "def edit_node_source(loc: Located) -> str:"
    - id: relocate
      type: FunctionDef
      signature: "def relocate(tree: Tree, old: Located) -> Located | None:"
      docstring: Find the node now occupying ``old``'s former slot after an in-place edit. Match…
    - id: replace_node
      type: FunctionDef
      signature: "def replace_node(loc: Located, code: str) -> str | None:"
      docstring: Replace ``loc``'s node with ``code``; return its new id if the id changed.
    - id: insert_node
      type: FunctionDef
      signature: "def insert_node(loc: Located, code: str, position: str) -> int:"
    - id: delete_node
      type: FunctionDef
      signature: "def delete_node(loc: Located) -> None:"
    - id: append_nodes
      type: FunctionDef
      signature: "def append_nodes(tree: Tree, code: str) -> int:"
    - id: empty_tree
      type: FunctionDef
      signature: "def empty_tree(path: Path) -> Tree:"
    - id: validate_source
      type: FunctionDef
      signature: "def validate_source(path: Path, source: str) -> str | None:"
```
/answer cc16-e allow
Control Request:
```yaml
id: cc16-f
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/__init__.py
  reason: Python Engine Struktur genauer prüfen für Vergleich mit generic Engine
```
/answer cc16-f allow
Control Request:
```yaml
id: cc16-10
result:
  structuredContent:
    nodes:
    - id: j10hxN
      type: statements
    - id: 5zERFW
      type: imports
    - id: Rendl1
      type: statements
```
/answer cc16-10 allow
ReasoningToken: 331
Control Request:
```yaml
id: cc16-11
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/__init__.py
  ids:
  - j10hxN
  - 5zERFW
  - Rendl1
  reason: Inhalt der Python-Engine __init__.py lesen
```
/answer cc16-11 allow
Control Request:
```yaml
id: cc16-12
result:
  structuredContent:
    nodes:
    - id: j10hxN
      type: statements
      code: "\"Python back-end for the ``ast_*`` tools, built on the standard-library ``ast``.\\n\\nSplit into :mod:`xy.ai.mcpc.tools.ast.python._comments` (comment-preserving\\npre-processing), :mod:`xy.ai.mcpc.tools.ast.python._nodes` (node\\nclassification/formatting and statement grouping) and\\n:mod:`xy.ai.mcpc.tools.ast.python._engine` (the :class:`PythonEngine` itself);\\nthis module re-exports the package's public surface.\\n\""
    - id: 5zERFW
      type: imports
      code: |-
        from __future__ import annotations
        from xy.ai.mcpc.tools.ast.python._comments import comments_to_annotations
        from xy.ai.mcpc.tools.ast.python._engine import ENGINE, PythonEngine
        from xy.ai.mcpc.tools.ast.python._nodes import import_names
    - id: Rendl1
      type: statements
      code: "__all__ = ['ENGINE', 'PythonEngine', 'comments_to_annotations', 'import_names']"
```
/answer cc16-12 allow
Control Request:
```yaml
id: cc16-13
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  ids:
  - ue626f
  - ZlvshC
  - RBFWwR
  - engine_for_path
  - 5EG9eJ
  reason: "Engine-Abstraktion in core.py verstehen (Tree, Located, Engine Protokoll)"
```
/answer cc16-13 allow
Control Request:
```yaml
id: cc16-14
result:
  structuredContent:
    nodes:
    - id: ue626f
      type: statements
      code: "'Facade the ``ast_*`` tools call, dispatching to a per-file engine.\\n\\nThe engine is chosen by file extension: Python files use the ``ast``-based\\n:mod:`.python` engine, everything else the generic tree-sitter :mod:`.generic`\\nengine. Snippets passed as raw ``code`` (no path) default to Python.\\n\\nA single content-hash validated :class:`AstCache` – reused across engines –\\nholds parsed :class:`~.base.Tree` objects keyed by absolute path and validated\\non every access by ``st_mtime_ns`` and, on change, by a content hash.\\n\\nThe engine-agnostic model (``AstError``, ``Located``, ``OutlineNode`` …) is\\nre-exported here so the tools only ever import :mod:`.core`.\\n'"
    - id: ZlvshC
      type: imports
      code: |-
        from __future__ import annotations
        import hashlib
        import threading
        from dataclasses import dataclass
        from pathlib import Path
        from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, OutlineNode, OUTLINE_NODE_SCHEMA, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, to_dict
        from xy.ai.mcpc.tools.ast import generic
        from xy.ai.mcpc.tools.ast import python
    - id: RBFWwR
      type: statements
      code: "_PYTHON_EXTENSIONS = ('.py', '.pyi')"
    - id: engine_for_path
      type: FunctionDef
      code: |-
        def engine_for_path(path: Path) -> Engine:
            """Return the engine responsible for ``path`` based on its extension."""
            ext = path.suffix.lower()
            if ext in _PYTHON_EXTENSIONS:
                return python.ENGINE
            symbol = generic.language_for_extension(ext)
            if symbol is None:
                raise AstError(f"No AST engine available for '{ext or path.name}' files.")
            return generic.get_engine(symbol)
    - id: 5EG9eJ
      type: statements
      code: |-
        '#: Process-wide shared cache instance.'
        CACHE = AstCache()
```
/answer cc16-14 allow
ReasoningToken: 31
Control Request:
```yaml
id: cc16-15
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: Tree/Engine Basisklassen prüfen für Serialisierung und Mutation
```
/answer cc16-15 allow
Control Request:
```yaml
id: cc16-16
result:
  structuredContent:
    nodes:
    - id: uJIfID
      type: statements
    - id: aKh2KG
      type: imports
    - id: AstError
      type: ClassDef
      signature: "class AstError(Exception):"
      docstring: "A user-facing, path-free error raised by the AST tools."
    - id: Tree
      type: ClassDef
      signature: "@dataclass class Tree:"
      docstring: "A parsed file/snippet plus the engine that owns it. Attributes: engine: The eng…"
    - id: Located
      type: ClassDef
      signature: "@dataclass class Located:"
      docstring: A node with the engine-independent metadata the selectors match on. Attributes:…
    - id: OutlineNode
      type: ClassDef
      signature: "@dataclass(frozen=True) class OutlineNode:"
      docstring: "One node in a structural (list/find/read) result. ``id`` is the node's unique, …"
    - id: line_range
      type: FunctionDef
      signature: "def line_range(loc: Located) -> str:"
      docstring: "Return ``loc``'s start line, or a ``\"start-end\"`` range if it spans several."
    - id: EXC8hj
      type: statements
    - id: hash
      type: FunctionDef
      signature: "def _hash(name: str, length: int) -> str:"
    - id: vomnxc
      type: statements
    - id: content_hash
      type: FunctionDef
      signature: "def _content_hash(content: str, length: int=6) -> str:"
      docstring: "Base62 (0-9a-zA-Z) digest of ``content``, stable across unrelated tree edits."
    - id: id_segment
      type: FunctionDef
      signature: "def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:"
      docstring: "Return a unique-within-siblings id segment, name-based when feasible. A clean, …"
    - id: rOzDt8
      type: statements
    - id: node_outline
      type: FunctionDef
      signature: "def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:"
      docstring: "Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``,…"
    - id: compact
      type: FunctionDef
      signature: "def _compact(value: Any) -> Any:"
      docstring: Recursively drop ``None`` values and empty lists from a dataclass-derived struc…
    - id: to_dict
      type: FunctionDef
      signature: "def to_dict(node: OutlineNode) -> dict:"
      docstring: "Serialize an :class:`OutlineNode` to MCP output, omitting empty fields."
    - id: TreeNode
      type: ClassDef
      signature: "@dataclass class _TreeNode:"
    - id: build_forest
      type: FunctionDef
      signature: "def _build_forest(located: list[Located]) -> list[_TreeNode]:"
      docstring: Nest a pre-order list of ``Located`` into a forest via ``node_id`` prefixes.
    - id: build_outline
      type: FunctionDef
      signature: "def build_outline(located: list[Located], *, with_code: bool=False, with_lines: bool=True) -> list[OutlineNode]:"
      docstring: "Build the nested outline of ``located`` (source only if ``with_code``, lines on…"
    - id: outline_nodes
      type: FunctionDef
      signature: "def _outline_nodes(nodes: list['_TreeNode'], *, with_code: bool, with_lines: bool=True) -> list[OutlineNode]:"
      docstring: "Convert a forest into OutlineNodes, collapsing non-expandable nodes to full sou…"
    - id: read_subtrees
      type: FunctionDef
      signature: "def read_subtrees(located: list[Located], keys: list[str], *, with_lines: bool=True) -> list[OutlineNode]:"
      docstring: "Return one read subtree per ``keys`` entry, matched by ``id``. Raises: AstError…"
    - id: matches
      type: FunctionDef
      signature: "def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> bool:"
    - id: find
      type: FunctionDef
      signature: "def find(tree: Tree, **filters: object) -> list[Located]:"
    - id: most_specific
      type: FunctionDef
      signature: "def most_specific(located: list[Located], lineno: int, end_lineno: int) -> Located | None:"
      docstring: "Return the smallest node in *located* fully containing lines [lineno, end_linen…"
    - id: Engine
      type: ClassDef
      signature: "class Engine(ABC):"
      docstring: "A parser back-end turning source into an addressable, mutable tree. Structural …"
      children:
      - id: Engine.1zTXCW
        type: statements
      - id: Engine.parse
        type: FunctionDef
        signature: "@abstractmethod def parse(self, source: str, path: Path | None=None) -> Tree:"
        docstring: "Parse ``source`` into a :class:`Tree`, raising :class:`AstError` on error."
      - id: Engine.empty_tree
        type: FunctionDef
        signature: "@abstractmethod def empty_tree(self, path: Path | None=None) -> Tree:"
        docstring: "Return an empty tree, used when appending to a not-yet-existing file."
      - id: Engine.serialize
        type: FunctionDef
        signature: "@abstractmethod def serialize(self, tree: Tree) -> str:"
        docstring: Render ``tree`` back to source text for writing to disk.
      - id: Engine.validate
        type: FunctionDef
        signature: "@abstractmethod def validate(self, source: str) -> str | None:"
        docstring: "Return an error message if ``source`` is malformed, else ``None``."
      - id: Engine.locate_all
        type: FunctionDef
        signature: "@abstractmethod def locate_all(self, tree: Tree) -> list[Located]:"
        docstring: "Flatten ``tree`` into every addressable node, in document order."
      - id: Engine.signature
        type: FunctionDef
        signature: "@abstractmethod def signature(self, node: Any) -> str:"
        docstring: One-line rendering of ``node``'s header (or the node itself).
      - id: Engine.docstring
        type: FunctionDef
        signature: "@abstractmethod def docstring(self, node: Any) -> str | None:"
        docstring: "Short docstring of ``node``, if the format has such a concept."
      - id: Engine.node_code
        type: FunctionDef
        signature: "@abstractmethod def node_code(self, node: Any) -> str:"
        docstring: Full source of a single ``node``.
      - id: Engine.replace
        type: FunctionDef
        signature: "@abstractmethod def replace(self, loc: Located, code: str) -> None:"
        docstring: Replace ``loc``'s node with ``code``.
      - id: Engine.insert
        type: FunctionDef
        signature: "@abstractmethod def insert(self, loc: Located, code: str, position: str) -> int:"
        docstring: Insert ``code`` ``"before"``/``"after"`` ``loc``; return units inserted.
      - id: Engine.delete
        type: FunctionDef
        signature: "@abstractmethod def delete(self, loc: Located) -> None:"
        docstring: Delete ``loc``'s node from its container.
      - id: Engine.append
        type: FunctionDef
        signature: "@abstractmethod def append(self, tree: Tree, code: str) -> int:"
        docstring: Append ``code`` at ``tree``'s top level; return units appended.
    - id: require_path
      type: FunctionDef
      signature: "def require_path(path_str: str, *, must_exist: bool=True) -> Path:"
      docstring: "Validate a mandatory absolute path, raising :class:`AstError` on failure."
    - id: xe6IXY
      type: statements
    - id: XTMl49
      type: statements
```
/answer cc16-16 allow
Control Request:
```yaml
id: cc16-17
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
  ids:
  - TreeSitterEngine.parse_1
  - TreeSitterEngine.serialize
  - TreeSitterEngine.splice
  reason: Serialisierung und Mutationslogik der TreeSitterEngine prüfen
```
/answer cc16-17 allow
Control Request:
```yaml
id: cc16-18
result:
  structuredContent:
    nodes:
    - id: TreeSitterEngine.parse_1
      type: FunctionDef
      code: |-
        def parse(self, source: str, path: Path | None=None) -> Tree:
            return Tree(self, self._parse(source.encode('utf-8')), source, path)
    - id: TreeSitterEngine.serialize
      type: FunctionDef
      code: |-
        def serialize(self, tree: Tree) -> str:
            return tree.source
    - id: TreeSitterEngine.splice
      type: FunctionDef
      code: |-
        def _splice(self, tree: Tree, start: int, end: int, text: str) -> None:
            data = tree.source.encode('utf-8')
            new = data[:start] + text.encode('utf-8') + data[end:]
            tree.source = new.decode('utf-8')
            tree.raw = self._parse(new)
```
/answer cc16-18 allow
ReasoningToken: 614
Control Request:
```yaml
id: cc16-19
toolName: ast_edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: saiGVi
  start_marker: For complex reorganisation
  end_marker: Any change to ``tree`` is persisted.
  content: |-
    For complex reorganisation/optimisation the model can operate on the tree
    directly. The script runs with an empty ``__builtins__`` plus a small, curated
    set of safe names; the only capability handed in is the AST itself (``tree``)
    and the standard-library ``ast`` module. Any change to ``tree`` is persisted.

    Python-only: the mutate-then-``ast.unparse`` model relies on the standard
    ``ast`` tree being a live, mutable Python object graph. The generic
    tree-sitter engine's nodes are read-only views into an immutable parse tree,
    mutated only via the byte-range ``_splice`` used by ``ast_replace`` &
    co.; mutating them here would silently have no effect on the saved file.
  reason: "Modulbeschreibung ergänzen: begründen, warum ast_script auf Python beschränkt bleibt (Kompatibilitätsprüfung generic engine)"
```
/answer cc16-19 allow
Control Request:
```yaml
id: cc16-1a
result:
  content:
  - type: text
    text: End marker not found in file.
  isError: true
```
/answer cc16-1a allow
Control Request:
```yaml
id: cc16-1b
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: ScriptTool.M4d1yn
  old_text: description = "Run a restricted Python script code against a file's AST for complex/incremental transforms. Globals expose 'tree' (ast.Module) and 'ast'; assign 'result' to return data. Changes to 'tree' are saved. Imports are not allowed."
  new_text: description = "Run a restricted Python script code against a Python file's AST for complex/incremental transforms. Globals expose 'tree' (ast.Module) and 'ast'; assign 'result' to return data. Changes to 'tree' are saved. Imports are not allowed. Python files only."
  reason: "Toolbeschreibung präzisieren: explizit auf Python-Dateien beschränkt, für Konsistenz mit Laufzeitprüfung"
```
/answer cc16-1b allow
ReasoningToken: 2049
Control Request:
```yaml
id: cc16-1c
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  ids:
  - saiGVi
  reason: "Aktuellen Inhalt des Moduldocstrings prüfen, um Marker-Fehler zu beheben"
```
/answer cc16-1c allow
Control Request:
```yaml
id: cc16-1d
result:
  structuredContent:
    nodes:
    - id: saiGVi
      type: statements
      code: "'``ast_script`` – run restricted Python against the parsed AST.\\n\\nFor complex reorganisation/optimisation the model can operate on the tree\\ndirectly. The script runs with an empty ``__builtins__`` plus a small, curated\\nset of safe names; the only capability handed in is the AST itself (``tree``)\\nand the standard-library ``ast`` module. Any change to ``tree`` is persisted.\\n'"
```
/answer cc16-1d allow
Control Request:
```yaml
id: cc16-1e
toolName: ast_edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: saiGVi
  start_marker: For complex reorganisation
  end_marker: tree`` is persisted.
  content: |-
    For complex reorganisation/optimisation the model can operate on the tree
    directly. The script runs with an empty ``__builtins__`` plus a small, curated
    set of safe names; the only capability handed in is the AST itself (``tree``)
    and the standard-library ``ast`` module. Any change to ``tree`` is persisted.

    Python-only: the mutate-then-``ast.unparse`` model relies on the standard
    ``ast`` tree being a live, mutable Python object graph. The generic
    tree-sitter engine's nodes are read-only views into an immutable parse tree,
    mutated only via the byte-range ``_splice`` used by ``ast_replace`` &
    co.; mutating them here would silently have no effect on the saved file.
  reason: "Modulbeschreibung ergänzen: begründen, warum ast_script auf Python beschränkt bleibt"
```
/answer cc16-1e allow
Control Request:
```yaml
id: cc16-1f
result:
  content:
  - type: text
    text: End marker not found in file.
  isError: true
```
/answer cc16-1f allow Sieht aus wie ein escaping Problem
ReasoningToken: 67
Control Request:
```yaml
id: cc16-20
toolName: ast_replace
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: saiGVi
  source: |-
    """``ast_script`` – run restricted Python against the parsed AST.

    For complex reorganisation/optimisation the model can operate on the tree
    directly. The script runs with an empty ``__builtins__`` plus a small, curated
    set of safe names; the only capability handed in is the AST itself (``tree``)
    and the standard-library ``ast`` module. Any change to ``tree`` is persisted.

    Python-only: the mutate-then-``ast.unparse`` model relies on the standard
    ``ast`` tree being a live, mutable Python object graph. The generic
    tree-sitter engine's nodes are read-only views into an immutable parse tree,
    mutated only via the byte-range ``_splice`` used by ``ast_replace`` & co.;
    mutating them here would silently have no effect on the saved file.
    """
  reason: "Modulbeschreibung ergänzen: begründen, warum ast_script auf Python beschränkt bleibt (Kompatibilitätsprüfung generic engine)"
```
/answer cc16-20 allow
ReasoningToken: 375
Control Request:
```yaml
id: cc16-21
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: Syntax nach den Bearbeitungen prüfen
```
/answer cc16-21 allow
ReasoningToken: 164
Text:
 Nun validieren:
Result Stats:  id=e14729e9-4fb7-40d1-bfb4-8146338313f2, total: 0, in: 159, out: 9254, reason: 0, read: 199782, write: 25043
Kompatibilität geprüft: `ast_script` bleibt bewusst Python-only. Die generische tree-sitter-Engine liefert in `tree.raw` einen unveränderlichen Parsebaum – Mutationen daran werden von `TreeSitterEngine.serialize()` (das nur `tree.source` zurückgibt) nicht erfasst, Änderungen würden also stillschweigend verloren gehen. Zudem ist das global exponierte `ast`-Modul für Nicht-Python-Quellen ohne Nutzen. Die bestehende Laufzeitprüfung (`tree.engine is not core.python.ENGINE`) ist daher korrekt und wurde nicht gelockert; stattdessen habe ich Modul-Docstring und Tool-Beschreibung präzisiert, damit der Grund der Beschränkung ohne Rückgriff auf die Fehlermeldung ersichtlich ist.
Die Lösung dafür ist ganz einfach. Du implementiert einen Wrapper Tree der die von den anderen Tools benutzen replace Funktionalitäten ausführt den baum einmal durch die Engine jagt und schwupp ist er kompatibel für Änderungen.
SystemInit:  id=e14729e9-4fb7-40d1-bfb4-8146338313f2, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: cc16-22
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  ids:
  - locate_all
  - replace_node
  - insert_node
  - delete_node
  - append_nodes
  - empty_tree
  - validate_source
  - load
  - parse_source
  - parse_for
  - edit_node_source
  - relocate
  reason: "Genaue Implementierung der engine-agnostischen Mutationsfunktionen prüfen, um Wrapper-Tree für ast_script zu entwerfen"
```
/answer cc16-22 allow Eigentlich noch besser, die Engine reicht gleich diesen View heraus.
Control Request:
```yaml
id: cc16-23
result:
  structuredContent:
    nodes:
    - id: locate_all
      type: FunctionDef
      code: |-
        def locate_all(tree: Tree) -> list[Located]:
            return tree.engine.locate_all(tree)
    - id: replace_node
      type: FunctionDef
      code: |-
        def replace_node(loc: Located, code: str) -> str | None:
            """Replace ``loc``'s node with ``code``; return its new id if the id changed."""
            loc.tree.engine.replace(loc, code)
            new_loc = relocate(loc.tree, loc)
            return new_loc.node_id if new_loc and new_loc.node_id != loc.node_id else None
    - id: insert_node
      type: FunctionDef
      code: |-
        def insert_node(loc: Located, code: str, position: str) -> int:
            return loc.tree.engine.insert(loc, code, position)
    - id: delete_node
      type: FunctionDef
      code: |-
        def delete_node(loc: Located) -> None:
            loc.tree.engine.delete(loc)
    - id: append_nodes
      type: FunctionDef
      code: |-
        def append_nodes(tree: Tree, code: str) -> int:
            return tree.engine.append(tree, code)
    - id: empty_tree
      type: FunctionDef
      code: |-
        def empty_tree(path: Path) -> Tree:
            return engine_for_path(path).empty_tree(path)
    - id: validate_source
      type: FunctionDef
      code: |-
        def validate_source(path: Path, source: str) -> str | None:
            return engine_for_path(path).validate(source)
    - id: load
      type: FunctionDef
      code: |-
        def load(path_str: str) -> tuple[Path, Tree]:
            """Resolve *path_str* and return it together with its cached tree."""
            path = require_path(path_str)
            return (path, CACHE.get_tree(path))
    - id: parse_source
      type: FunctionDef
      code: |-
        def parse_source(source: str, engine: Engine | None=None) -> Tree:
            """Parse *source* with *engine* (Python by default for path-less snippets)."""
            return (engine or python.ENGINE).parse(source)
    - id: parse_for
      type: FunctionDef
      code: |-
        def parse_for(path_str: str, code: str) -> Tree:
            """Parse *code* with the engine selected for *path_str*'s extension."""
            path = require_path(path_str, must_exist=False)
            return engine_for_path(path).parse(code, path)
    - id: edit_node_source
      type: FunctionDef
      code: |-
        def edit_node_source(loc: Located) -> str:
            return loc.tree.engine.node_code(loc.node)
    - id: relocate
      type: FunctionDef
      code: |-
        def relocate(tree: Tree, old: Located) -> Located | None:
            """Find the node now occupying ``old``'s former slot after an in-place edit.

            Matched by start line and parent type (unaffected by the edit itself), then
            disambiguated by sibling index if several candidates share that line.
            """
            candidates = [loc for loc in locate_all(tree) if loc.lineno == old.lineno and loc.parent_type == old.parent_type]
            for loc in candidates:
                if loc.index == old.index:
                    return loc
            return candidates[0] if candidates else None
```
/answer cc16-23 allow
Control Request:
```yaml
id: cc16-24
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  ids:
  - Tree
  - Located
  - find
  - matches
  reason: Tree/Located Struktur und find/matches genau prüfen für Wrapper-Design
```
/answer cc16-24 allow
Control Request:
```yaml
id: cc16-25
result:
  structuredContent:
    nodes:
    - id: Tree
      type: ClassDef
      code: |-
        @dataclass
        class Tree:
            """A parsed file/snippet plus the engine that owns it.

            Attributes:
                engine: The engine that parsed ``raw`` and knows how to mutate it.
                raw: Engine-native tree (``ast.Module`` or ``tree_sitter.Tree``).
                source: Current source text; the single source of truth for tree-sitter
                    engines, refreshed by the Python engine only on save.
                path: Absolute path the tree was loaded from, or ``None`` for snippets.
            """
            engine: 'Engine'
            raw: Any
            source: str
            path: Path | None = None
    - id: Located
      type: ClassDef
      code: |-
        @dataclass
        class Located:
            """A node with the engine-independent metadata the selectors match on.

            Attributes:
                tree: The owning tree (for engine dispatch).
                node: Engine-native node object.
                parent: Engine-native container node.
                index: Position of ``node`` among its parent's addressable children.
                node_id: The node's unique ``id`` — its fully-qualified path from the
                    root (e.g. ``"MyClass.method"``), in name/hash form or, for nameless
                    nodes/segments, a stable content-hash fallback. There is no separate FQN.
                node_type: Engine-reported node type name.
                name: Simple name, if the node carries one.
                lineno / end_lineno: 1-based inclusive line span.
                parent_type: Type name of ``parent``, or ``None`` at the top level.
                expandable: Whether ``read`` should descend into children instead of
                    returning the node's full source (a pure container of nested defs).
            """
            tree: Tree
            node: Any
            parent: Any
            index: int
            node_id: str
            node_type: str
            name: str | None
            lineno: int
            end_lineno: int
            parent_type: str | None
            expandable: bool = False
    - id: find
      type: FunctionDef
      code: |-
        def find(tree: Tree, **filters: object) -> list[Located]:
            active = {k: v for k, v in filters.items() if v is not None}
            '# type: ignore[arg-type]'
            return [loc for loc in tree.engine.locate_all(tree) if matches(loc, **active)]
    - id: matches
      type: FunctionDef
      code: |-
        def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> bool:
            if id is not None and loc.node_id != id:
                return False
            if node_type is not None and loc.node_type.lower() != node_type.lower():
                return False
            if name is not None and loc.name != name:
                return False
            if parent_type is not None and (loc.parent_type or '').lower() != parent_type.lower():
                return False
            return True
```
/answer cc16-25 allow
ReasoningToken: 2336
Control Request:
```yaml
id: cc16-26
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py
  reason: Filterlogik des find-Tools prüfen für konsistente Wrapper-API
```
/answer cc16-26 allow Die engine enthält ja schon alle notwendigen operationen replace/insert/delete/append
Control Request:
```yaml
id: cc16-27
result:
  structuredContent:
    nodes:
    - id: jlFZZg
      type: statements
    - id: Zi6vd4
      type: imports
    - id: qcW43R
      type: statements
    - id: FindNodesResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class FindNodesResult:"
      docstring: "Result of :func:`ast_find`. Attributes: nodes: Outline-style node descriptions …"
    - id: ast_find
      type: FunctionDef
      signature: "def ast_find(path: str, *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:"
      docstring: "Find nodes by id, type, name, line range, parent type, text or regexp. ``ast_fi…"
    - id: FindNodesTool
      type: ClassDef
      signature: "class FindNodesTool(ToolDefinition):"
      children:
      - id: FindNodesTool.SZISZL
        type: statements
      - id: FindNodesTool.IwNjGT
        type: statements
      - id: FindNodesTool.vbrqXE
        type: statements
      - id: FindNodesTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer cc16-27 allow
Control Request:
```yaml
id: cc16-28
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
  ids:
  - PythonEngine
  reason: PythonEngine-Methoden signature/docstring prüfen für Kompatibilität mit generic Engine im Wrapper
```
/answer cc16-28 allow
Control Request:
```yaml
id: cc16-29
result:
  structuredContent:
    nodes:
    - id: PythonEngine
      type: ClassDef
      signature: "class PythonEngine(Engine):"
      docstring: "``ast``-based engine: comment-preserving parse, ``unparse`` serialisation."
      children:
      - id: PythonEngine.i4mNz3
        type: statements
        code: |-
          '``ast``-based engine: comment-preserving parse, ``unparse`` serialisation.'
          name = 'python'
      - id: PythonEngine.parse
        type: FunctionDef
        code: |-
          def parse(self, source: str, path: Path | None=None) -> Tree:
              return Tree(self, self._parse_module(source), source, path)
      - id: PythonEngine.parse_module
        type: FunctionDef
        code: |-
          def _parse_module(self, source: str) -> ast.Module:
              try:
                  return ast.parse(comments_to_annotations(source))
              except SyntaxError as exc:
                  raise AstError(f'Syntax error: {exc.msg} (line {exc.lineno})') from exc
      - id: PythonEngine.parse_fragment
        type: FunctionDef
        code: |-
          def _parse_fragment(self, code: str) -> list[ast.stmt]:
              return self._parse_module(code).body
      - id: PythonEngine.empty_tree
        type: FunctionDef
        code: |-
          def empty_tree(self, path: Path | None=None) -> Tree:
              return Tree(self, ast.Module(body=[], type_ignores=[]), '', path)
      - id: PythonEngine.serialize
        type: FunctionDef
        code: |-
          def serialize(self, tree: Tree) -> str:
              return _unparse(ast.fix_missing_locations(tree.raw))
      - id: PythonEngine.validate
        type: FunctionDef
        code: |-
          def validate(self, source: str) -> str | None:
              try:
                  compile(source, '<validate>', 'exec')
              except SyntaxError as exc:
                  return f'{exc.msg} (line {exc.lineno})'
              return None
      - id: PythonEngine.loc
        type: FunctionDef
        code: |-
          def _loc(self, tree, node, parent, index, name, nid, expandable=False) -> Located:
              node_type = node.kind if isinstance(node, _StatementGroup) else type(node).__name__
              return Located(
                  tree=tree,
                  node=node,
                  parent=parent,
                  index=index,
                  node_id=nid,
                  node_type=node_type,
                  name=name,
                  lineno=node.lineno,
                  end_lineno=getattr(
                      node,
                      'end_lineno',
                      node.lineno),
                  parent_type=type(parent).__name__,
                  expandable=expandable)
      - id: PythonEngine.locate_all
        type: FunctionDef
        code: |-
          def locate_all(self, tree: Tree) -> list[Located]:
              results: list[Located] = []

              def walk(container: ast.AST, path: str) -> None:
                  used: dict[str, int] = {}
                  body = getattr(container, 'body', [])
                  i = 0
                  while i < len(body):
                      node = body[i]
                      if isinstance(node, _DEF_TYPES):
                          seg = id_segment(node.name, i, used)
                          nid = f'{path}.{seg}' if path else seg
                          results.append(self._loc(tree, node, container, i, node.name, nid, _is_expandable(node)))
                          walk(node, nid)
                          i += 1
                          continue
                      start = i
                      kind = 'imports' if isinstance(node, _IMPORT_TYPES) else 'statements'
                      length = 0
                      while i < len(body):
                          current = body[i]
                          if isinstance(current, _DEF_TYPES):
                              break
                          current_kind = 'imports' if isinstance(current, _IMPORT_TYPES) else 'statements'
                          if current_kind != kind:
                              break
                          piece = len(self.node_code(current))
                          if i > start and length + piece > SEGMENT_MAX_CHARS:
                              break
                          length += piece
                          i += 1
                      group = _StatementGroup(container, start, i, kind)
                      seg = id_segment(None, start, used, content=self.node_code(group))
                      nid = f'{path}.{seg}' if path else seg
                      results.append(self._loc(tree, group, container, start, None, nid))
              walk(tree.raw, '')
              return results
      - id: PythonEngine.signature
        type: FunctionDef
        code: |-
          def signature(self, node: Any, limit: int=80) -> str:
              if isinstance(node, _StatementGroup):
                  first_line = (self.node_code(node).splitlines() or [''])[0]
                  return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'
              if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                  keyword = 'async def' if isinstance(node, ast.AsyncFunctionDef) else 'def'
                  returns = f' -> {ast.unparse(node.returns)}' if node.returns is not None else ''
                  return f'{_decorators(node)}{keyword} {node.name}({ast.unparse(node.args)}){returns}:'
              if isinstance(node, ast.ClassDef):
                  bases = [ast.unparse(b) for b in node.bases] + [f'{kw.arg}={ast.unparse(kw.value)}' for kw in node.keywords]
                  bases_str = f'({', '.join(bases)})' if bases else ''
                  return f'{_decorators(node)}class {node.name}{bases_str}:'
              first_line = ast.unparse(node).splitlines()[0]
              return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'
      - id: PythonEngine.docstring
        type: FunctionDef
        code: |-
          def docstring(self, node: Any, limit: int=80) -> str | None:
              if not isinstance(node, (ast.Module, ast.ClassDef, *_DEF_TYPES)):
                  return None
              doc = ast.get_docstring(node, clean=True)
              if doc is None:
                  return None
              doc = ' '.join(doc.split())
              return doc if len(doc) <= limit else doc[:limit - 1] + '…'
      - id: PythonEngine.node_code
        type: FunctionDef
        code: |-
          def node_code(self, node: Any) -> str:
              if isinstance(node, _StatementGroup):
                  return '\n'.join((_unparse(ast.fix_missing_locations(s)) for s in node.stmts))
              return _unparse(ast.fix_missing_locations(node))
      - id: PythonEngine.replace
        type: FunctionDef
        code: |-
          def replace(self, loc: Located, code: str) -> None:
              node = loc.node
              if isinstance(node, _StatementGroup):
                  node.parent.body[node.start:node.stop] = self._parse_fragment(code)
              else:
                  loc.parent.body[loc.index:loc.index + 1] = self._parse_fragment(code)
      - id: PythonEngine.insert
        type: FunctionDef
        code: |-
          def insert(self, loc: Located, code: str, position: str) -> int:
              stmts = self._parse_fragment(code)
              node = loc.node
              if isinstance(node, _StatementGroup):
                  body = node.parent.body
                  index = node.stop if position == 'after' else node.start
              else:
                  body = loc.parent.body
                  offset = 1 if position == 'after' else 0
                  index = body.index(loc.node) + offset
              body[index:index] = stmts
              return len(stmts)
      - id: PythonEngine.delete
        type: FunctionDef
        code: |-
          def delete(self, loc: Located) -> None:
              node = loc.node
              if isinstance(node, _StatementGroup):
                  del node.parent.body[node.start:node.stop]
              else:
                  del loc.parent.body[loc.index]
      - id: PythonEngine.append
        type: FunctionDef
        code: |-
          def append(self, tree: Tree, code: str) -> int:
              stmts = self._parse_fragment(code)
              tree.raw.body.extend(stmts)
              return len(stmts)
```
/answer cc16-29 allow
ReasoningToken: 2828
Control Request:
```yaml
id: cc16-2a
toolName: ast_replace
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: oRJJ61
  source: |-
    """``ast_script`` – run restricted Python against the parsed AST.

    For complex reorganisation/optimisation the model can operate on the tree
    directly. The script runs with an empty ``__builtins__`` plus a small, curated
    set of safe names; the only capabilities handed in are ``tree`` (a
    :class:`ScriptTree` wrapping the parsed file) and the standard-library
    ``ast`` module. Any change made through ``tree`` is persisted.

    ``tree`` exposes the same locate/replace/insert/delete/append primitives the
    other ``ast_*`` tools use, so scripts work the same way for Python and
    tree-sitter files alike. ``tree.raw`` gives direct access to the engine-native
    tree (``ast.Module`` for Python, ``tree_sitter.Tree`` otherwise); only the
    Python ``ast.Module`` is safely mutable in place – tree-sitter's parse tree is
    read-only and must be edited through ``tree``'s methods instead.
    """
  reason: Moduldocstring auf die neue engine-agnostische ScriptTree-API umstellen
```
/answer cc16-2a allow
Control Request:
```yaml
id: cc16-2b
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  ids:
  - 8t7Kee
  reason: Import-Block erneut lesen um Any/Located Import zu prüfen vor Erweiterung
```
/answer cc16-2b allow
Control Request:
```yaml
id: cc16-2c
result:
  structuredContent:
    nodes:
    - id: 8t7Kee
      type: imports
      code: |-
        import ast
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
```
/answer cc16-2c allow
ReasoningToken: 3135
Text:
 Jetzt füge ich die `ScriptTree`-Wrapper-Klasse ein und passe `ast_script` an:
Control Request:
```yaml
id: cc16-2d
toolName: ast_insert
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: ScriptError
  position: before
  source: |+
    class ScriptTree:
        """Engine-agnostic ``tree`` handle exposed to sandboxed scripts.

        Wraps a :class:`core.Tree`, exposing the locate/replace/insert/delete/append
        primitives the other ``ast_*`` tools use, so scripts behave identically
        regardless of which engine parsed the file. ``raw`` gives direct access to
        the engine-native tree; only the Python engine's ``ast.Module`` is safely
        mutable in place, the tree-sitter engine's parse tree is read-only and
        must be edited through the methods below.
        """

        def __init__(self, tree: core.Tree) -> None:
            self._tree = tree

        @property
        def raw(self) -> Any:
            return self._tree.raw

        @property
        def source(self) -> str:
            return self._tree.source

        @property
        def path(self) -> Any:
            return self._tree.path

        def find(self, *, id: str | None = None, node_type: str | None = None, name: str | None = None, parent_type: str | None = None) -> list[core.Located]:
            return core.find(self._tree, id=id, node_type=node_type, name=name, parent_type=parent_type)

        def locate_all(self) -> list[core.Located]:
            return core.locate_all(self._tree)

        def node_code(self, loc: core.Located) -> str:
            return core.edit_node_source(loc)

        def replace(self, loc: core.Located, code: str) -> str | None:
            return core.replace_node(loc, code)

        def insert(self, loc: core.Located, code: str, position: str = 'after') -> int:
            return core.insert_node(loc, code, position)

        def delete(self, loc: core.Located) -> None:
            core.delete_node(loc)

        def append(self, code: str) -> int:
            return core.append_nodes(self._tree, code)


  reason: "ScriptTree-Wrapper hinzufügen, der die replace/insert/delete/append/find-Primitive der übrigen ast_*-Tools engine-unabhängig bereitstellt"
```
/answer cc16-2d allow
Control Request:
```yaml
id: cc16-2e
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  ids:
  - ast_script
  reason: Aktuellen Stand von ast_script vor Anpassung lesen
```
/answer cc16-2e allow
Control Request:
```yaml
id: cc16-2f
result:
  structuredContent:
    nodes:
    - id: ast_script
      type: FunctionDef
      code: |-
        def ast_script(path: str, code: str) -> AstScriptResult:
            """Execute ``code`` in a restricted sandbox exposing the AST of ``path`` as ``tree``.

            ``code`` runs with an empty ``__builtins__`` plus a small, curated set of safe
            names (see ``_SAFE_BUILTINS``); the only capabilities handed in are the parsed
            tree (``tree``, an ``ast.Module``) and the standard-library ``ast`` module
            itself. Any mutation of ``tree`` is unparsed and persisted to ``path`` on
            success.

            Args:
                path: Absolute path to the file whose AST is exposed as ``tree``.
                code: Python script to execute against ``tree``. May assign a module-level
                    name ``result`` to return an arbitrary value (reported as its ``repr()``).

            Returns:
                AstScriptResult: Success status and, if the script set one, the ``repr()``
                of its ``result`` variable.

            Raises:
                core.AstError: If ``path`` is invalid.
                ScriptError: If ``code`` has a syntax error, or raises during execution.
            """
            file_path = core.require_path(path)
            tree = core.CACHE.get_tree(file_path)
            if tree.engine is not core.python.ENGINE:
                raise core.AstError('ast_script operates on the Python AST; it is only available for Python files.')
            env: dict[str, Any] = {'tree': tree.raw, 'ast': ast}
            sandbox_globals = {'__builtins__': _SAFE_BUILTINS}
            try:
                '# noqa: S102'
                exec(compile(code, '<ast-script>', 'exec'), sandbox_globals, env)
            except SyntaxError as exc:
                raise ScriptError(f'Script syntax error: {exc.msg}') from exc
            except Exception as exc:
                '# noqa: BLE001'
                raise ScriptError(f'Script failed: {type(exc).__name__}: {exc}') from exc
            core.CACHE.save(file_path, tree)
            if 'result' in env:
                return AstScriptResult(result='success', value=repr(env['result']))
            return AstScriptResult(result='success')
```
/answer cc16-2f allow
Control Request:
```yaml
id: cc16-30
toolName: ast_replace
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: ast_script
  source: |-
    def ast_script(path: str, code: str) -> AstScriptResult:
        """Execute ``code`` in a restricted sandbox exposing the file's tree as ``tree``.

        ``code`` runs with an empty ``__builtins__`` plus a small, curated set of safe
        names (see ``_SAFE_BUILTINS``); the only capabilities handed in are ``tree``
        (a :class:`ScriptTree`) and the standard-library ``ast`` module itself. Any
        mutation made through ``tree`` is persisted to ``path`` on success, for any
        file type the AST tools support (Python or tree-sitter).

        Args:
            path: Absolute path to the file whose AST is exposed as ``tree``.
            code: Python script to execute against ``tree``. May assign a module-level
                name ``result`` to return an arbitrary value (reported as its ``repr()``).

        Returns:
            AstScriptResult: Success status and, if the script set one, the ``repr()``
            of its ``result`` variable.

        Raises:
            core.AstError: If ``path`` is invalid.
            ScriptError: If ``code`` has a syntax error, or raises during execution.
        """
        file_path = core.require_path(path)
        tree = core.CACHE.get_tree(file_path)
        env: dict[str, Any] = {'tree': ScriptTree(tree), 'ast': ast}
        sandbox_globals = {'__builtins__': _SAFE_BUILTINS}
        try:
            '# noqa: S102'
            exec(compile(code, '<ast-script>', 'exec'), sandbox_globals, env)
        except SyntaxError as exc:
            raise ScriptError(f'Script syntax error: {exc.msg}') from exc
        except Exception as exc:
            '# noqa: BLE001'
            raise ScriptError(f'Script failed: {type(exc).__name__}: {exc}') from exc
        core.CACHE.save(file_path, tree)
        if 'result' in env:
            return AstScriptResult(result='success', value=repr(env['result']))
        return AstScriptResult(result='success')
  reason: "ast_script auf ScriptTree umstellen; engine-agnostisch, Python-Restriktion entfällt"
```
/answer cc16-30 allow
Text:
 Nun `ast_script` selbst anpassen: Guard entfernen, `tree` als `ScriptTree` reichen, Docstring aktualisieren.
Control Request:
```yaml
id: cc16-31
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: ScriptTool.M4d1yn
  old_text: description = "Run a restricted Python script code against a Python file's AST for complex/incremental transforms. Globals expose 'tree' (ast.Module) and 'ast'; assign 'result' to return data. Changes to 'tree' are saved. Imports are not allowed. Python files only."
  new_text: "description = \"Run a restricted Python script code against a file's AST for complex/incremental transforms. Globals expose 'tree' (a ScriptTree with find/replace/insert/delete/append, plus 'tree.raw' for the engine-native ast.Module/tree_sitter.Tree) and 'ast'; assign 'result' to return data. Changes made through 'tree' are saved. Imports are not allowed.\""
  reason: Toolbeschreibung an die neue engine-agnostische ScriptTree-API anpassen
```
/answer cc16-31 allow
Control Request:
```yaml
id: cc16-32
result:
  content:
  - type: text
    text: No node matched the selector.
  isError: true
```
/answer cc16-32 allow
Control Request:
```yaml
id: cc16-33
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: uuST9t
  old_text: "__all__ = ['ScriptError', 'AstScriptResult', 'ast_script', 'ScriptTool', 'register']"
  new_text: "__all__ = ['ScriptTree', 'ScriptError', 'AstScriptResult', 'ast_script', 'ScriptTool', 'register']"
  reason: ScriptTree in öffentliche API aufnehmen
```
/answer cc16-33 allow
Control Request:
```yaml
id: cc16-34
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: Aktuelle IDs nach Edits neu abrufen
```
/answer cc16-34 allow
Control Request:
```yaml
id: cc16-35
result:
  structuredContent:
    nodes:
    - id: amT2KR
      type: statements
    - id: 8t7Kee
      type: imports
    - id: D86RaJ
      type: statements
    - id: HuXBMZ
      type: statements
    - id: ScriptTree
      type: ClassDef
      signature: "class ScriptTree:"
      docstring: Engine-agnostic ``tree`` handle exposed to sandboxed scripts. Wraps a :class:`c…
      children:
      - id: ScriptTree.R2UlTU
        type: statements
      - id: ScriptTree.init
        type: FunctionDef
        signature: "def __init__(self, tree: core.Tree) -> None:"
      - id: ScriptTree.raw
        type: FunctionDef
        signature: "@property def raw(self) -> Any:"
      - id: ScriptTree.source
        type: FunctionDef
        signature: "@property def source(self) -> str:"
      - id: ScriptTree.path
        type: FunctionDef
        signature: "@property def path(self) -> Any:"
      - id: ScriptTree.find
        type: FunctionDef
        signature: "def find(self, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> list[core.Located]:"
      - id: ScriptTree.locate_all
        type: FunctionDef
        signature: "def locate_all(self) -> list[core.Located]:"
      - id: ScriptTree.node_code
        type: FunctionDef
        signature: "def node_code(self, loc: core.Located) -> str:"
      - id: ScriptTree.replace
        type: FunctionDef
        signature: "def replace(self, loc: core.Located, code: str) -> str | None:"
      - id: ScriptTree.insert
        type: FunctionDef
        signature: "def insert(self, loc: core.Located, code: str, position: str='after') -> int:"
      - id: ScriptTree.delete
        type: FunctionDef
        signature: "def delete(self, loc: core.Located) -> None:"
      - id: ScriptTree.append
        type: FunctionDef
        signature: "def append(self, code: str) -> int:"
    - id: ScriptError
      type: ClassDef
      signature: "class ScriptError(Exception):"
      docstring: Raised when an AST script cannot be run to completion.
    - id: AstScriptResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class AstScriptResult:"
      docstring: "Result of :func:`ast_script`. Attributes: result: Always ``\"success\"``. value: …"
    - id: ast_script
      type: FunctionDef
      signature: "def ast_script(path: str, code: str) -> AstScriptResult:"
      docstring: Execute ``code`` in a restricted sandbox exposing the file's tree as ``tree``. …
    - id: ScriptTool
      type: ClassDef
      signature: "class ScriptTool(ToolDefinition):"
      children:
      - id: ScriptTool.07w3Cm
        type: statements
      - id: ScriptTool.GglHto
        type: statements
      - id: ScriptTool.JYmhDk
        type: statements
      - id: ScriptTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_script`, translating the MCP schema to/from the Python A…"
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer cc16-35 allow
Control Request:
```yaml
id: cc16-36
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  ids:
  - ScriptTool.07w3Cm
  - ScriptTool.GglHto
  - D86RaJ
  reason: "Aktuellen Stand von Description, Schema und __all__ prüfen"
```
/answer cc16-36 allow
Control Request:
```yaml
id: cc16-37
result:
  structuredContent:
    nodes:
    - id: ScriptTool.07w3Cm
      type: statements
      code: |-
        name = 'ast_script'
        title = 'Run AST python script'
        description = "Run a restricted Python script code against a Python file's AST for complex/incremental transforms. Globals expose 'tree' (ast.Module) and 'ast'; assign 'result' to return data. Changes to 'tree' are saved. Imports are not allowed. Python files only."
    - id: ScriptTool.GglHto
      type: statements
      code: |-
        input_schema = {
            'type': 'object',
            'properties': {
                'path': {
                    'type': 'string',
                    'description': 'Absolute path to the Python file.'},
                'code': {
                    'type': 'string',
                    'description': "Python script operating on 'tree';Environment is restricted; Don't use imports;"}},
            'required': [
                'path',
                'code']}
    - id: D86RaJ
      type: statements
      code: "__all__ = ['ScriptTree', 'ScriptError', 'AstScriptResult', 'ast_script', 'ScriptTool', 'register']"
```
/answer cc16-37 allow
Text:
 Nun ScriptTool-Beschreibung, input_schema und `__all__` aktualisieren:
Control Request:
```yaml
id: cc16-38
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: ScriptTool.07w3Cm
  old_text: description = "Run a restricted Python script code against a Python file's AST for complex/incremental transforms. Globals expose 'tree' (ast.Module) and 'ast'; assign 'result' to return data. Changes to 'tree' are saved. Imports are not allowed. Python files only."
  new_text: "description = \"Run a restricted Python script code against a file's AST for complex/incremental transforms. Globals expose 'tree' (a ScriptTree with find/replace/insert/delete/append, plus 'tree.raw' for the engine-native ast.Module/tree_sitter.Tree) and 'ast'; assign 'result' to return data. Changes made through 'tree' are saved. Imports are not allowed.\""
  reason: Toolbeschreibung an die neue engine-agnostische ScriptTree-API anpassen
```
/answer cc16-38 allow
Control Request:
```yaml
id: cc16-39
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: ScriptTool.GglHto
  old_text: "'description': 'Absolute path to the Python file.'"
  new_text: "'description': 'Absolute path to the file.'"
  reason: Pfadbeschreibung nicht mehr auf Python beschränken
```
/answer cc16-39 allow
Control Request:
```yaml
id: cc16-3a
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: ScriptTool.GglHto
  old_text: "'description': \"Python script operating on 'tree';Environment is restricted; Don't use imports;\""
  new_text: "'description': \"Python script operating on 'tree' (find/replace/insert/delete/append); Environment is restricted; Don't use imports;\""
  reason: Code-Feld-Beschreibung um die ScriptTree-Methoden ergänzen
```
/answer cc16-3a allow
Control Request:
```yaml
id: cc16-3b
result:
  content:
  - type: text
    text: No node matched the selector.
  isError: true
```
/answer cc16-3b allow
Control Request:
```yaml
id: cc16-3c
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  ids:
  - ScriptTool.GglHto
  reason: Aktuellen Stand des input_schema nach Edits prüfen
```
/answer cc16-3c allow
Control Request:
```yaml
id: cc16-3d
result:
  content:
  - type: text
    text: No node matched 'ScriptTool.GglHto'.
  isError: true
```
/answer cc16-3d allow
Control Request:
```yaml
id: cc16-3e
toolName: ast_list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: IDs nach Edits neu abrufen
```
/answer cc16-3e allow
Control Request:
```yaml
id: cc16-3f
result:
  structuredContent:
    nodes:
    - id: amT2KR
      type: statements
    - id: 8t7Kee
      type: imports
    - id: D86RaJ
      type: statements
    - id: HuXBMZ
      type: statements
    - id: ScriptTree
      type: ClassDef
      signature: "class ScriptTree:"
      docstring: Engine-agnostic ``tree`` handle exposed to sandboxed scripts. Wraps a :class:`c…
      children:
      - id: ScriptTree.R2UlTU
        type: statements
      - id: ScriptTree.init
        type: FunctionDef
        signature: "def __init__(self, tree: core.Tree) -> None:"
      - id: ScriptTree.raw
        type: FunctionDef
        signature: "@property def raw(self) -> Any:"
      - id: ScriptTree.source
        type: FunctionDef
        signature: "@property def source(self) -> str:"
      - id: ScriptTree.path
        type: FunctionDef
        signature: "@property def path(self) -> Any:"
      - id: ScriptTree.find
        type: FunctionDef
        signature: "def find(self, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> list[core.Located]:"
      - id: ScriptTree.locate_all
        type: FunctionDef
        signature: "def locate_all(self) -> list[core.Located]:"
      - id: ScriptTree.node_code
        type: FunctionDef
        signature: "def node_code(self, loc: core.Located) -> str:"
      - id: ScriptTree.replace
        type: FunctionDef
        signature: "def replace(self, loc: core.Located, code: str) -> str | None:"
      - id: ScriptTree.insert
        type: FunctionDef
        signature: "def insert(self, loc: core.Located, code: str, position: str='after') -> int:"
      - id: ScriptTree.delete
        type: FunctionDef
        signature: "def delete(self, loc: core.Located) -> None:"
      - id: ScriptTree.append
        type: FunctionDef
        signature: "def append(self, code: str) -> int:"
    - id: ScriptError
      type: ClassDef
      signature: "class ScriptError(Exception):"
      docstring: Raised when an AST script cannot be run to completion.
    - id: AstScriptResult
      type: ClassDef
      signature: "@dataclass(frozen=True) class AstScriptResult:"
      docstring: "Result of :func:`ast_script`. Attributes: result: Always ``\"success\"``. value: …"
    - id: ast_script
      type: FunctionDef
      signature: "def ast_script(path: str, code: str) -> AstScriptResult:"
      docstring: Execute ``code`` in a restricted sandbox exposing the file's tree as ``tree``. …
    - id: ScriptTool
      type: ClassDef
      signature: "class ScriptTool(ToolDefinition):"
      children:
      - id: ScriptTool.qedlra
        type: statements
      - id: ScriptTool.ZMeBmH
        type: statements
      - id: ScriptTool.JYmhDk
        type: statements
      - id: ScriptTool.handle
        type: FunctionDef
        signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
        docstring: "Delegate to :func:`ast_script`, translating the MCP schema to/from the Python A…"
    - id: register
      type: FunctionDef
      signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer cc16-3f allow
ReasoningToken: 49
Control Request:
```yaml
id: cc16-40
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: ScriptTool.ZMeBmH
  old_text: "'description': \"Python script operating on 'tree';Environment is restricted; Don't use imports;\""
  new_text: "'description': \"Python script operating on 'tree' (find/replace/insert/delete/append); Environment is restricted; Don't use imports;\""
  reason: Code-Feld-Beschreibung um die ScriptTree-Methoden ergänzen
```
/answer cc16-40 allow
ReasoningToken: 88
Text:
 Gut, `__all__` ist bereits aktualisiert. Description und input_schema müssen noch angepasst werden:
Control Request:
```yaml
id: cc16-41
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: Syntax nach allen Änderungen prüfen
```
/answer cc16-41 allow
Control Request:
```yaml
id: cc16-42
toolName: ast_read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  ids:
  - amT2KR
  - 8t7Kee
  - D86RaJ
  - HuXBMZ
  - ScriptTree
  - ScriptError
  - AstScriptResult
  - ast_script
  - ScriptTool
  - register
  reason: Gesamten Dateiinhalt final prüfen
```
/answer cc16-42 allow
Control Request:
```yaml
id: cc16-43
result:
  structuredContent:
    nodes:
    - id: amT2KR
      type: statements
      code: "\"``ast_script`` – run restricted Python against the parsed AST.\\n\\nFor complex reorganisation/optimisation the model can operate on the tree\\ndirectly. The script runs with an empty ``__builtins__`` plus a small, curated\\nset of safe names; the only capabilities handed in are ``tree`` (a\\n:class:`ScriptTree` wrapping the parsed file) and the standard-library\\n``ast`` module. Any change made through ``tree`` is persisted.\\n\\n``tree`` exposes the same locate/replace/insert/delete/append primitives the\\nother ``ast_*`` tools use, so scripts work the same way for Python and\\ntree-sitter files alike. ``tree.raw`` gives direct access to the engine-native\\ntree (``ast.Module`` for Python, ``tree_sitter.Tree`` otherwise); only the\\nPython ``ast.Module`` is safely mutable in place – tree-sitter's parse tree is\\nread-only and must be edited through ``tree``'s methods instead.\\n\""
    - id: 8t7Kee
      type: imports
      code: |-
        import ast
        from dataclasses import dataclass
        from typing import Any
        from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
        from xy.ai.mcpc.tools.tool_context import ToolContext
        from xy.ai.mcpc.tools.ast import core
        from xy.ai.mcpc.tools.function_registry import FunctionRegistry
    - id: D86RaJ
      type: statements
      code: "__all__ = ['ScriptTree', 'ScriptError', 'AstScriptResult', 'ast_script', 'ScriptTool', 'register']"
    - id: HuXBMZ
      type: statements
      code: |-
        _SAFE_BUILTINS = {
            name: getattr(
                __builtins__,
                name,
                None) if not isinstance(
                    __builtins__,
                    dict) else __builtins__.get(name) for name in (
                        'isinstance',
                        'issubclass',
                        'getattr',
                        'setattr',
                        'hasattr',
                        'delattr',
                        'len',
                        'list',
                        'dict',
                        'set',
                        'tuple',
                        'str',
                        'int',
                        'float',
                        'bool',
                        'enumerate',
                        'range',
                        'sorted',
                        'reversed',
                        'zip',
                        'map',
                        'filter',
                        'any',
                        'all',
                        'min',
                        'max',
                        'sum',
                        'type',
                'repr')}
    - id: ScriptTree
      type: ClassDef
      signature: "class ScriptTree:"
      docstring: Engine-agnostic ``tree`` handle exposed to sandboxed scripts. Wraps a :class:`c…
      children:
      - id: ScriptTree.R2UlTU
        type: statements
        code: "\"Engine-agnostic ``tree`` handle exposed to sandboxed scripts.\\n\\n    Wraps a :class:`core.Tree`, exposing the locate/replace/insert/delete/append\\n    primitives the other ``ast_*`` tools use, so scripts behave identically\\n    regardless of which engine parsed the file. ``raw`` gives direct access to\\n    the engine-native tree; only the Python engine's ``ast.Module`` is safely\\n    mutable in place, the tree-sitter engine's parse tree is read-only and\\n    must be edited through the methods below.\\n    \""
      - id: ScriptTree.init
        type: FunctionDef
        code: |-
          def __init__(self, tree: core.Tree) -> None:
              self._tree = tree
      - id: ScriptTree.raw
        type: FunctionDef
        code: |-
          @property
          def raw(self) -> Any:
              return self._tree.raw
      - id: ScriptTree.source
        type: FunctionDef
        code: |-
          @property
          def source(self) -> str:
              return self._tree.source
      - id: ScriptTree.path
        type: FunctionDef
        code: |-
          @property
          def path(self) -> Any:
              return self._tree.path
      - id: ScriptTree.find
        type: FunctionDef
        code: |-
          def find(self, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> list[core.Located]:
              return core.find(self._tree, id=id, node_type=node_type, name=name, parent_type=parent_type)
      - id: ScriptTree.locate_all
        type: FunctionDef
        code: |-
          def locate_all(self) -> list[core.Located]:
              return core.locate_all(self._tree)
      - id: ScriptTree.node_code
        type: FunctionDef
        code: |-
          def node_code(self, loc: core.Located) -> str:
              return core.edit_node_source(loc)
      - id: ScriptTree.replace
        type: FunctionDef
        code: |-
          def replace(self, loc: core.Located, code: str) -> str | None:
              return core.replace_node(loc, code)
      - id: ScriptTree.insert
        type: FunctionDef
        code: |-
          def insert(self, loc: core.Located, code: str, position: str='after') -> int:
              return core.insert_node(loc, code, position)
      - id: ScriptTree.delete
        type: FunctionDef
        code: |-
          def delete(self, loc: core.Located) -> None:
              core.delete_node(loc)
      - id: ScriptTree.append
        type: FunctionDef
        code: |-
          def append(self, code: str) -> int:
              return core.append_nodes(self._tree, code)
    - id: ScriptError
      type: ClassDef
      code: |-
        class ScriptError(Exception):
            """Raised when an AST script cannot be run to completion."""
    - id: AstScriptResult
      type: ClassDef
      code: |-
        @dataclass(frozen=True)
        class AstScriptResult:
            """Result of :func:`ast_script`.

            Attributes:
                result: Always ``"success"``.
                value: ``repr()`` of the script's ``result`` variable, if the script set one;
                    otherwise ``None``.
            """
            result: str
            value: str | None = None
    - id: ast_script
      type: FunctionDef
      code: |-
        def ast_script(path: str, code: str) -> AstScriptResult:
            """Execute ``code`` in a restricted sandbox exposing the file's tree as ``tree``.

            ``code`` runs with an empty ``__builtins__`` plus a small, curated set of safe
            names (see ``_SAFE_BUILTINS``); the only capabilities handed in are ``tree``
            (a :class:`ScriptTree`) and the standard-library ``ast`` module itself. Any
            mutation made through ``tree`` is persisted to ``path`` on success, for any
            file type the AST tools support (Python or tree-sitter).

            Args:
                path: Absolute path to the file whose AST is exposed as ``tree``.
                code: Python script to execute against ``tree``. May assign a module-level
                    name ``result`` to return an arbitrary value (reported as its ``repr()``).

            Returns:
                AstScriptResult: Success status and, if the script set one, the ``repr()``
                of its ``result`` variable.

            Raises:
                core.AstError: If ``path`` is invalid.
                ScriptError: If ``code`` has a syntax error, or raises during execution.
            """
            file_path = core.require_path(path)
            tree = core.CACHE.get_tree(file_path)
            env: dict[str, Any] = {'tree': ScriptTree(tree), 'ast': ast}
            sandbox_globals = {'__builtins__': _SAFE_BUILTINS}
            try:
                '# noqa: S102'
                exec(compile(code, '<ast-script>', 'exec'), sandbox_globals, env)
            except SyntaxError as exc:
                raise ScriptError(f'Script syntax error: {exc.msg}') from exc
            except Exception as exc:
                '# noqa: BLE001'
                raise ScriptError(f'Script failed: {type(exc).__name__}: {exc}') from exc
            core.CACHE.save(file_path, tree)
            if 'result' in env:
                return AstScriptResult(result='success', value=repr(env['result']))
            return AstScriptResult(result='success')
    - id: ScriptTool
      type: ClassDef
      signature: "class ScriptTool(ToolDefinition):"
      children:
      - id: ScriptTool.qedlra
        type: statements
        code: |-
          name = 'ast_script'
          title = 'Run AST python script'
          description = "Run a restricted Python script code against a file's AST for complex/incremental transforms. Globals expose 'tree' (a ScriptTree with find/replace/insert/delete/append, plus 'tree.raw' for the engine-native ast.Module/tree_sitter.Tree) and 'ast'; assign 'result' to return data. Changes made through 'tree' are saved. Imports are not allowed."
      - id: ScriptTool.UzC3p6
        type: statements
        code: |-
          input_schema = {
              'type': 'object',
              'properties': {
                  'path': {
                      'type': 'string',
                      'description': 'Absolute path to the file.'},
                  'code': {
                      'type': 'string',
                      'description': "Python script operating on 'tree' (find/replace/insert/delete/append); Environment is restricted; Don't use imports;"}},
              'required': [
                  'path',
                  'code']}
      - id: ScriptTool.JYmhDk
        type: statements
        code: |-
          output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}, 'value': {
              'description': "Repr of the script's 'result' variable, if set."}}, 'required': ['result']}
          annotations = {'readOnlyHint': False, 'openWorldHint': False}
      - id: ScriptTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_script`, translating the MCP schema to/from the Python API."""
              args: dict[str, Any] = ctx.arguments
              try:
                  result = ast_script(args['path'], args['code'])
              except (core.AstError, ScriptError) as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              structured: dict[str, Any] = {'result': result.result}
              if result.value is not None:
                  structured['value'] = result.value
              return ToolResult(structured_content=structured)
    - id: register
      type: FunctionDef
      code: |-
        def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
            registry.register(ScriptTool())
            functions.register(ast_script)
```
/answer cc16-43 allow
Control Request:
```yaml
id: cc16-44
toolName: ast_edit_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  id: ScriptTool.qedlra
  old_text: title = 'Run AST python script'
  new_text: title = 'Run AST script'
  reason: Titel nicht mehr Python-exklusiv formulieren
```
/answer cc16-44 allow
ReasoningToken: 104
ReasoningToken: 19
Control Request:
```yaml
id: cc16-45
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: Finale Syntaxprüfung
```
/answer cc16-45 allow
Text:
 Jetzt Syntax validieren und finalen Inhalt prüfen:
ReasoningToken: 211
ReasoningToken: 46
Result Stats:  id=e14729e9-4fb7-40d1-bfb4-8146338313f2, total: 0, in: 255, out: 26212, reason: 0, read: 1003679, write: 57075
Umgesetzt: `ScriptTree`-Wrapper in `script.py` hinzugefügt, der `find`/`locate_all`/`node_code`/`replace`/`insert`/`delete`/`append` engine-unabhängig über `core.find`/`core.replace_node`/`core.insert_node`/`core.delete_node`/`core.append_nodes` bereitstellt (dieselben Primitive, die auch `find.py`, `edit.py` usw. nutzen). `ast_script` reicht jetzt für jede Datei (Python oder tree-sitter) ein `ScriptTree`-Objekt als `tree` in die Sandbox statt des rohen `ast.Module`; die frühere Python-only-Prüfung entfällt. `tree.raw` bleibt für direkten Zugriff auf den engine-nativen Baum erhalten (bei Python weiterhin direkt mutierbar, beim tree-sitter-Engine nur lesend – Mutationen dort laufen über die neuen Methoden, die intern per `_splice` persistieren). Modul-/Funktions-Docstrings, Tool-`description`, `title` und `input_schema` wurden entsprechend aktualisiert; `ScriptTree` ist Teil von `__all__`.
