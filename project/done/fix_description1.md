Die Toolbeschreibung in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py` Zeile 165 ist unzureichend. Sie muss neu geschrieben werden und eine komplette Darstellung des Engine unabhängigen AST-Node Attribute für die Verwendung beinhalten.
Die Beschreibung soll über zwei Stellen verteilt sein. Die Toolbeschreibung und die Parameterbeschreibung für "code".
Das ganze muss Soft-Prompt ähnlich für einen Agenten lesbar sein, nicht für einen Menschen.
Sammel die notwendigen Informationen und dann mache einen Vorschlag.
SystemInit:  id=b2f7e87b-7411-46bb-944b-4fcad2daaed1, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 06be-17
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: "Zeile 165 und umgebenden Kontext der Tool-Registrierung finden, um die aktuelle Beschreibung zu lokalisieren."
  lineno: 165
```
/answer 06be-17 allow
Control Request:
```yaml
id: 06be-18
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
      nodes:
      - id: ScriptTool.Hn1yaW
        type: statements
        code: |-
          name = 'ast_script'
          title = 'Run AST script'
          description = "Run a restricted Python script code against a file's AST for complex/incremental transforms. Globals expose 'tree' (a ScriptTree with find/replace/insert/delete/append, plus 'tree.raw' for the engine-native ast.Module/tree_sitter.Tree) and 'ast'; assign 'result' to return data. Changes made through 'tree' are saved. Imports are not allowed."
```
/answer 06be-18 allow
Control Request:
```yaml
id: 06be-19
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: "Überblick über die Struktur von script.py verschaffen, um Kontext für Zeile 165 zu erhalten."
```
/answer 06be-19 allow
Control Request:
```yaml
id: 06be-1a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
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
        - id: ScriptTool.Hn1yaW
          type: statements
        - id: ScriptTool.UzC3p6
          type: statements
        - id: ScriptTool.handle
          type: FunctionDef
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_script`, translating the MCP schema to/from the Python A…"
      - id: register
        type: FunctionDef
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 06be-1a allow
ReasoningToken: 81
Control Request:
```yaml
id: 06be-1b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    ids:
    - ScriptTool.UzC3p6
    - ScriptTree
    - ScriptTree.find
    - ScriptTree.node_code
    - D86RaJ
    - HuXBMZ
    - 8t7Kee
  reason: "Aktuelle Parameterdefinitionen (u.a. 'code') und ScriptTree-Doku lesen, um die neue Beschreibung zu entwerfen."
```
/answer 06be-1b allow
Control Request:
```yaml
id: 06be-1c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
      nodes:
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
      - id: ScriptTree.find
        type: FunctionDef
        code: |-
          def find(self, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> list[core.Located]:
              return core.find(self._tree, id=id, node_type=node_type, name=name, parent_type=parent_type)
      - id: ScriptTree.node_code
        type: FunctionDef
        code: |-
          def node_code(self, loc: core.Located) -> str:
              return core.edit_node_source(loc)
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
/answer 06be-1c allow
Control Request:
```yaml
id: 06be-1d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: class Located|signature|docstring|children|lines
    include:
    - '*.py'
    exclude:
    - '*test*'
    limit: 40
  reason: "Definition der engine-unabhängigen Located/Node-Attribute (id, type, lines, signature, docstring, code, children) im Kernmodul finden."
```
/answer 06be-1d allow
Control Request:
```yaml
id: 06be-1e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      pattern: class Located|signature|docstring|children|lines
      matches:
      - path: server/json_codec.py
        lineno: 106
        match: "    def read_lines(stream: IO[str]) -> Iterator[Any]:"
      - path: server/json_codec.py
        lineno: 107
        match: "        \"\"\"Yield JSON objects from a text stream, skipping blank/garbage lines.\"\"\""
      - path: tools/mcp/client.py
        lineno: 110
        match: "            for line in text.splitlines():"
      - path: tools/mcp/exa/fetch_results.py
        lineno: 4
        match: "(``grep -E`` semantics), including a configurable amount of context lines."
      - path: tools/mcp/exa/fetch_results.py
        lineno: 22
        match: "                                                       'description': 'Extended regular expression (grep -E semantics) to filter text lines.'},"
      - path: tools/mcp/exa/fetch_results.py
        lineno: 24
        match: "                                                       'description': 'Context lines kept before/after each match (default: 1); only used with pattern.',"
      - path: tools/mcp/exa/fetch_results.py
        lineno: 28
        match: "def _grep_lines(text: str, pattern: str, context: int) -> str:"
      - path: tools/mcp/exa/fetch_results.py
        lineno: 29
        match: '    """Keep lines matching *pattern* plus *context* lines around each match (''grep -E'' style)."""'
      - path: tools/mcp/exa/fetch_results.py
        lineno: 30
        match: '    lines = text.splitlines()'
      - path: tools/mcp/exa/fetch_results.py
        lineno: 32
        match: "    matched = [i for i, line in enumerate(lines) if regex.search(line)]"
      - path: tools/mcp/exa/fetch_results.py
        lineno: 35
        match: "    keep = sorted({j for i in matched for j in range(max(0, i - context), min(len(lines), i + context + 1))})"
      - path: tools/mcp/exa/fetch_results.py
        lineno: 42
        match: "    return '\\n--\\n'.join(('\\n'.join((lines[i] for i in group)) for group in grouped))"
      - path: tools/mcp/exa/fetch_results.py
        lineno: 49
        match: "        pattern: Optional extended regular expression to filter text lines"
      - path: tools/mcp/exa/fetch_results.py
        lineno: 52
        match: "        context: Context lines kept before/after each match (default: 1)."
      - path: tools/mcp/exa/fetch_results.py
        lineno: 65
        match: "            text = _grep_lines(text, pattern, context)"
      - path: tools/mcp/exa/fetch.py
        lineno: 32
        match: "                              'lines': {'type': 'integer'},"
      - path: tools/tool_usage/__init__.py
        lineno: 1
        match: "\"\"\"``tool_usage`` – full signature, docstring and type sources for one or more functions.\"\"\""
      - path: tools/tool_usage/__init__.py
        lineno: 21
        match: "    signature: str"
      - path: tools/tool_usage/__init__.py
        lineno: 22
        match: "    docstring: str"
      - path: tools/tool_usage/__init__.py
        lineno: 76
        match: "        ToolUsageInfo: Signature, full docstring, and the source of every"
      - path: tools/tool_usage/__init__.py
        lineno: 87
        match: "        signature = f'{entry.name}{inspect.signature(entry.func)}'"
      - path: tools/tool_usage/__init__.py
        lineno: 89
        match: '        signature = entry.name'
      - path: tools/tool_usage/__init__.py
        lineno: 96
        match: "    return ToolUsageInfo(signature=signature, docstring=entry.doc, type_sources=sources)"
      - path: tools/tool_usage/__init__.py
        lineno: 101
        match: "    description = 'Get usage and information for one or more function-based tools: their signatures and sources.'"
      - path: tools/tool_usage/__init__.py
        lineno: 135
        match: "                           'signature': info.signature,"
      - path: tools/tool_usage/__init__.py
        lineno: 136
        match: "                           'docstring': info.docstring,"
      - path: tools/ast/base.py
        lineno: 51
        match: "class Located:"
      - path: tools/ast/base.py
        lineno: 58
        match: "        index: Position of ``node`` among its parent's addressable children."
      - path: tools/ast/base.py
        lineno: 66
        match: "        expandable: Whether ``read`` should descend into children instead of"
      - path: tools/ast/base.py
        lineno: 87
        match: '    ``find``/``read`` – ``list`` always leaves it ``None``. ``signature``/'
      - path: tools/ast/base.py
        lineno: 88
        match: '    ``docstring`` are only set for class/function nodes whose ``code`` is'
      - path: tools/ast/base.py
        lineno: 94
        match: "    lines: str | None"
      - path: tools/ast/base.py
        lineno: 95
        match: "    signature: str | None"
      - path: tools/ast/base.py
        lineno: 96
        match: "    docstring: str | None"
      - path: tools/ast/base.py
        lineno: 98
        match: "    children: list['OutlineNode'] = field(default_factory=list)"
      - path: tools/ast/base.py
        lineno: 147
        match: "'#: definition across engines, the only nodes a \"signature\" makes sense for.'"
      - path: tools/ast/base.py
        lineno: 150
        match: "def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:"
      - path: tools/ast/base.py
        lineno: 151
        match: "    \"\"\"Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``, lines only if ``with_lines``)."
      - path: tools/ast/base.py
        lineno: 153
        match: "    ``signature``/``docstring`` are only computed when ``code`` is not, since the"
      - path: tools/ast/base.py
        lineno: 158
        match: '        signature = docstring = None'
      warning: "Limit of 40 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 06be-1e allow
ReasoningToken: 134
Control Request:
```yaml
id: 06be-1f
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  reason: "Vollständige Struktur von base.py sehen, um OutlineNode und Located Klassen zu lokalisieren."
```
/answer 06be-1f allow
Control Request:
```yaml
id: 06be-20
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: uJIfID
        type: statements
      - id: e2q5UN
        type: imports
      - id: AstError
        type: ClassDef
        signature: "class AstError(Exception):"
        docstring: "A user-facing, path-free error raised by the AST tools."
      - id: AstAmbiguous
        type: ClassDef
        signature: "class AstAmbiguous(AstError):"
        docstring: Raised when a text-based node search (no id given) finds several unrelated cand…
        children:
        - id: AstAmbiguous.jchvnz
          type: statements
        - id: AstAmbiguous.init
          type: FunctionDef
          signature: "def __init__(self, message: str, candidates: list[str]) -> None:"
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
      - id: 8da9Jr
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
      - id: resolve_by_name
        type: FunctionDef
        signature: "def _resolve_by_name(key: str, by_name: dict[str, list['_TreeNode']]) -> tuple['_TreeNode | None', str | None]:"
        docstring: Resolve ``key`` against node names when it doesn't match an id directly. Tries …
      - id: read_subtrees
        type: FunctionDef
        signature: "def read_subtrees(located: list[Located], keys: list[str], *, with_lines: bool=True) -> tuple[list[OutlineNode], list[str]]:"
        docstring: "Return one read subtree per resolvable ``keys`` entry. Each key is matched, in …"
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
        - id: Engine.bz1VNl
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
        - id: Engine.is_definition
          type: FunctionDef
          signature: "def is_definition(self, node_type: str) -> bool:"
          docstring: Whether ``node_type`` is "def-like" enough for a ``signature`` to make sense (a…
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
      - id: b4hZTj
        type: statements
```
/answer 06be-20 allow
ReasoningToken: 32
Control Request:
```yaml
id: 06be-21
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    ids:
    - Located
    - OutlineNode
    - Tree
    - node_outline
    - to_dict
  reason: "Exakte Attribute und Semantik von Located/OutlineNode/Tree lesen, für eine engine-unabhängige Beschreibung der AST-Node-Attribute."
```
/answer 06be-21 allow
Control Request:
```yaml
id: 06be-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
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
      - id: OutlineNode
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class OutlineNode:
              """One node in a structural (list/find/read) result.

              ``id`` is the node's unique, primarily name-based path used by every tool to
              address it. ``code`` carries the node's full source and is populated by
              ``find``/``read`` – ``list`` always leaves it ``None``. ``signature``/
              ``docstring`` are only set for class/function nodes whose ``code`` is
              *not* included, since the full source already makes them visible.
              Serialization drops ``None``/empty fields, see :func:`to_dict`.
              """
              id: str
              type: str
              lines: str | None
              signature: str | None
              docstring: str | None
              code: str | None = None
              children: list['OutlineNode'] = field(default_factory=list)
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
      - id: node_outline
        type: FunctionDef
        code: |-
          def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:
              """Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``, lines only if ``with_lines``).

              ``signature``/``docstring`` are only computed when ``code`` is not, since the
              full source already makes them visible.
              """
              engine = loc.tree.engine
              if with_code:
                  signature = docstring = None
                  code = engine.node_code(loc.node)
              else:
                  signature = engine.signature(loc.node) if engine.is_definition(loc.node_type) else None
                  docstring = engine.docstring(loc.node)
                  code = None
              return OutlineNode(
                  id=loc.node_id,
                  type=loc.node_type,
                  lines=line_range(loc) if with_lines else None,
                  signature=signature,
                  docstring=docstring,
                  code=code,
                  children=children or [])
      - id: to_dict
        type: FunctionDef
        code: |-
          def to_dict(node: OutlineNode) -> dict:
              """Serialize an :class:`OutlineNode` to MCP output, omitting empty fields."""
              return _compact(asdict(node))
```
/answer 06be-22 allow
ReasoningToken: 2665
Result Stats:  id=b2f7e87b-7411-46bb-944b-4fcad2daaed1, total: 0, in: 141, out: 5196, reason: 0, read: 42432, write: 18184
## Vorschlag

**`description` (Tool-Ebene):**

```
Run a restricted Python script against a file's AST for complex/incremental transforms, portable across engines (Python `ast` / tree_sitter). Globals: 'tree' (ScriptTree: find/locate_all/node_code/replace/insert/delete/append/raw/source/path) and 'ast'; assign 'result' to return data. Imports disallowed; only a fixed safe-builtins subset is available. Changes made through 'tree''s methods are saved automatically; 'tree.raw' is only safely mutable in place for the Python engine, tree-sitter trees are read-only and must go through the methods. 'tree.find()'/'tree.locate_all()' return Located objects exposing the same engine-independent node attributes as every other ast_* tool's output (see 'code' param); use them instead of inspecting engine-native nodes to keep scripts engine-agnostic.
```

**`code`-Parameter (detailliert, Agent-lesbar):**

```
Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limited to isinstance/issubclass/getattr/setattr/hasattr/delattr/len/list/dict/set/tuple/str/int/float/bool/enumerate/range/sorted/reversed/zip/map/filter/any/all/min/max/sum/type/repr.
tree API: find(id=None, node_type=None, name=None, parent_type=None)->list[Located]; locate_all()->list[Located] (every addressable node, document order); node_code(loc)->str; replace(loc, code)->str|None; insert(loc, code, position='after'|'before')->int; delete(loc)->None; append(code)->int; raw (engine-native ast.Module or read-only tree_sitter.Tree; mutate only via these methods); source->str; path.
Located attrs (engine-independent, same semantics on Python-ast and tree-sitter files, mirror the id/type/lines/code fields used by ast_list/ast_find/ast_read/ast_edit_*/ast_replace/ast_insert/ast_delete): node_id (unique dotted name/hash path from root, e.g. 'MyClass.method' — the id used by every ast_* tool); node_type (engine-reported type, e.g. 'FunctionDef'/'pair'); name (simple name or None); lineno/end_lineno (1-based inclusive span, i.e. the 'lines' field elsewhere); parent_type (enclosing node's type, None at top level); expandable (True = pure container of nested defs, i.e. what 'read' descends into instead of inlining code); node/parent (engine-native objects, not portable, pass only to tree.* calls); index (position among parent's addressable children); tree (owning Tree: .engine/.raw/.source/.path).
Note: 'signature'/'docstring' shown by list/find/read (one-line header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) and simple parsing if needed.
```

Soll ich das so anwenden?
Den RAW Zugriff entfernen. Der Agenten soll nicht über Python und Tree Sitter backend spekulieren(Agnostisch).
SystemInit:  id=b2f7e87b-7411-46bb-944b-4fcad2daaed1, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 1133
Result Stats:  id=b2f7e87b-7411-46bb-944b-4fcad2daaed1, total: 0, in: 143, out: 7246, reason: 0, read: 60616, write: 21948
## Aktualisierter Vorschlag

**`description` (Tool-Ebene):**

```
Run a restricted Python script against a file's AST for complex/incremental transforms. Globals: 'tree' (ScriptTree: find/locate_all/node_code/replace/insert/delete/append/source/path) and 'ast'; assign 'result' to return data. Imports disallowed; only a fixed safe-builtins subset is available. Changes made through 'tree''s methods are saved automatically. 'tree.find()'/'tree.locate_all()' return Located objects exposing the same engine-independent node attributes as every other ast_* tool's output (see 'code' param); always use these attributes and methods instead of introspecting node objects directly, so scripts stay identical regardless of file type.
```

**`code`-Parameter:**

```
Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limited to isinstance/issubclass/getattr/setattr/hasattr/delattr/len/list/dict/set/tuple/str/int/float/bool/enumerate/range/sorted/reversed/zip/map/filter/any/all/min/max/sum/type/repr.
tree API: find(id=None, node_type=None, name=None, parent_type=None)->list[Located]; locate_all()->list[Located] (every addressable node, document order); node_code(loc)->str; replace(loc, code)->str|None; insert(loc, code, position='after'|'before')->int; delete(loc)->None; append(code)->int; source->str; path.
Located attrs (engine-independent, mirror the id/type/lines/code fields used by ast_list/ast_find/ast_read/ast_edit_*/ast_replace/ast_insert/ast_delete): node_id (unique dotted name/hash path from root, e.g. 'MyClass.method' — the id used by every ast_* tool); node_type (reported type, e.g. 'FunctionDef'/'pair'); name (simple name or None); lineno/end_lineno (1-based inclusive span, i.e. the 'lines' field elsewhere); parent_type (enclosing node's type, None at top level); expandable (True = pure container of nested defs, i.e. what 'read' descends into instead of inlining code); node/parent (opaque node handles for use only as tree.* arguments, never introspect their internals); index (position among parent's addressable children); tree (owning Tree; use its .source/.path, nothing else).
Note: 'signature'/'docstring' shown by list/find/read (one-line header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed.
```

Soll ich das so anwenden?
Ist berücksichtig das Die Description eher für die Toolwahl verwendet wird während der Parameter erst und dann auch aber zwingend relevant wird wenn sich für das Tool entschieden wurde, oder würdest du dann etwas verbessern wollen?
SystemInit:  id=b2f7e87b-7411-46bb-944b-4fcad2daaed1, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 616
Result Stats:  id=b2f7e87b-7411-46bb-944b-4fcad2daaed1, total: 0, in: 145, out: 8841, reason: 0, read: 82564, write: 24091
Guter Punkt – aktuell wiederholt die `description` schon einen Teil der API, die eigentlich erst nach Toolwahl relevant ist. Besser: `description` fokussiert nur auf die **Auswahlentscheidung** (wann dieses statt der anderen `ast_*`-Tools nehmen), die Detail-API/Attribute wandert komplett in `code`.

**`description` (kurz, auswahlrelevant):**

```
Run a restricted Python script against a file's AST when the single-operation ast_* tools (list/find/read/replace/insert/delete) are insufficient for a complex, multi-step, or conditional transform. Sandboxed globals: 'tree' and 'ast'; no imports; assign 'result' to return data; edits made via 'tree' are saved automatically. See the 'code' parameter for the full tree API and node-attribute reference.
```

**`code`-Parameter (unverändert als vollständige Referenz, jetzt alleinstehend):**

```
Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limited to isinstance/issubclass/getattr/setattr/hasattr/delattr/len/list/dict/set/tuple/str/int/float/bool/enumerate/range/sorted/reversed/zip/map/filter/any/all/min/max/sum/type/repr.
tree API: find(id=None, node_type=None, name=None, parent_type=None)->list[Located]; locate_all()->list[Located] (every addressable node, document order); node_code(loc)->str; replace(loc, code)->str|None; insert(loc, code, position='after'|'before')->int; delete(loc)->None; append(code)->int; source->str; path.
Located attrs (engine-independent, mirror the id/type/lines/code fields used by ast_list/ast_find/ast_read/ast_edit_*/ast_replace/ast_insert/ast_delete): node_id (unique dotted name/hash path from root, e.g. 'MyClass.method' — the id used by every ast_* tool); node_type (reported type, e.g. 'FunctionDef'/'pair'); name (simple name or None); lineno/end_lineno (1-based inclusive span, i.e. the 'lines' field elsewhere); parent_type (enclosing node's type, None at top level); expandable (True = pure container of nested defs, i.e. what 'read' descends into instead of inlining code); node/parent (opaque node handles for use only as tree.* arguments, never introspect their internals); index (position among parent's addressable children); tree (owning Tree; use its .source/.path, nothing else).
Note: 'signature'/'docstring' shown by list/find/read (one-line header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed.
```

Soll ich das so anwenden?
Ja so übernehmen
SystemInit:  id=b2f7e87b-7411-46bb-944b-4fcad2daaed1, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 06be-23
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    id: ScriptTool.Hn1yaW
    old_text: "description = \"Run a restricted Python script code against a file's AST for complex/incremental transforms. Globals expose 'tree' (a ScriptTree with find/replace/insert/delete/append, plus 'tree.raw' for the engine-native ast.Module/tree_sitter.Tree) and 'ast'; assign 'result' to return data. Changes made through 'tree' are saved. Imports are not allowed.\""
    new_text: "description = \"Run a restricted Python script against a file's AST when the single-operation ast_* tools (list/find/read/replace/insert/delete) are insufficient for a complex, multi-step, or conditional transform. Sandboxed globals: 'tree' and 'ast'; no imports; assign 'result' to return data; edits made via 'tree' are saved automatically. See the 'code' parameter for the full tree API and node-attribute reference.\""
  reason: "Toolbeschreibung durch die neue, auf Toolauswahl fokussierte Fassung ersetzen."
```
/answer 06be-23 allow
Control Request:
```yaml
id: 06be-24
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    id: ScriptTool.UzC3p6
    old_text: "'description': \"Python script operating on 'tree' (find/replace/insert/delete/append); Environment is restricted; Don't use imports;\""
    new_text: "'description': \"Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limited to isinstance/issubclass/getattr/setattr/hasattr/delattr/len/list/dict/set/tuple/str/int/float/bool/enumerate/range/sorted/reversed/zip/map/filter/any/all/min/max/sum/type/repr. tree API: find(id=None, node_type=None, name=None, parent_type=None)->list[Located]; locate_all()->list[Located] (every addressable node, document order); node_code(loc)->str; replace(loc, code)->str|None; insert(loc, code, position='after'|'before')->int; delete(loc)->None; append(code)->int; source->str; path. Located attrs (engine-independent, mirror the id/type/lines/code fields used by ast_list/ast_find/ast_read/ast_edit_*/ast_replace/ast_insert/ast_delete): node_id (unique dotted name/hash path from root, e.g. 'MyClass.method' — the id used by every ast_* tool); node_type (reported type, e.g. 'FunctionDef'/'pair'); name (simple name or None); lineno/end_lineno (1-based inclusive span, i.e. the 'lines' field elsewhere); parent_type (enclosing node's type, None at top level); expandable (True = pure container of nested defs, i.e. what 'read' descends into instead of inlining code); node/parent (opaque node handles for use only as tree.* arguments, never introspect their internals); index (position among parent's addressable children); tree (owning Tree; use its .source/.path, nothing else). Note: 'signature'/'docstring' shown by list/find/read (one-line header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed.\""
  reason: Parameterbeschreibung von 'code' durch die vollständige API- und Attributreferenz ersetzen.
```
/answer 06be-24 allow
Control Request:
```yaml
id: 06be-25
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: "Sicherstellen, dass die Datei nach den Änderungen syntaktisch korrekt ist."
```
/answer 06be-25 allow
Result Stats:  id=b2f7e87b-7411-46bb-944b-4fcad2daaed1, total: 0, in: 151, out: 10388, reason: 0, read: 159707, write: 27600
Übernommen und validiert.
