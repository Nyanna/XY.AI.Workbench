Ich möchte gerne Agenten einen Teil der Arbeit abnehmen und für die AST Engine eine Unterstützung für das Handling von Imports/Includes einbauen.
Analysiere den Stand und überlege eine Lösung.

- Import/Include Handling muss sprachspezifisch unterstützt werden.
- Alle AST Node Edit Operationen bekommen zwei neue Felder "addImport" und "removeImport" (Listen)
- Im fall der generischen Engine wird nur geprüft, ob der Import(Text) bereits in der Datei vorkommt. Wird der Text nicht gefunden, also wenn er hinzugefügt werden muss, wird eine Aufforderung zurückgegeben das der Agent den Import/Include/Use selbst prüfen muss (Nur ein generischer Hinweis für alle Batches das die Import/Includes nicht geprüft werden konnten oder nicht unterstützt wird)
- Ein "addImport" der Fehlt wird den vorhandenen Imports/Includes/Use angefügt
- Ein "removeImport" der vorhanden ist, wird in der Datei gesucht und entfernt
- Das Inputscheme hat eine verständliche allgemein zutreffende Beschreibung

AST-Engines:
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_java.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_rust.py`

AST-Tools:
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py`
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Ich beginne mit einer Bestandsaufnahme der Engines und Tools.
Control Request:
```yaml
id: 0a99-4c-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_java.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_rust.py
  reason: Struktur aller AST-Engines erfassen
```
/answer 0a99-4c-0 allow
Control Request:
```yaml
id: 0a99-4d-4c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
      nodes:
      - id: 1wOyVk|BtswSM
        signature: '"Generic tree-sitter back-end for every non-Python language/format.\n\nGrammars…'
      - id: 4Yt1ea|lCuXJr
        signature: from __future__ import annotations
      - id: ijYyvI|izD3O0
        signature: "__all__ = ['TreeSitterEngine', 'GenericEngine']"
      - id: SynthNode
        signature: "class _SynthNode:"
        docstring: Minimal tree-sitter-node stand-in for a node an engine subclass rebuilds itself…
        children:
        - id: SynthNode.Fs8Bxv|yS8Eci
          signature: '''Minimal tree-sitter-node stand-in for a node an engine subclass rebuilds\n    …'
        - id: SynthNode.init
          signature: "def __init__(self, node_type: str, children: list[Any], source: bytes) -> None:"
        - id: SynthNode.text
          signature: "@property def text(self) -> bytes:"
        - id: SynthNode.child_by_field_name
          signature: "@staticmethod def child_by_field_name(_field: str) -> None:"
      - id: RootHolder
        signature: "class _RootHolder:"
        docstring: Fake container so the shared node-walker can start from a plain child list.
        children:
        - id: RootHolder.iVia8b|Rk25Eh
          signature: '''Fake container so the shared node-walker can start from a plain child list.'''
        - id: RootHolder.init
          signature: "def __init__(self, children: list[Any]) -> None:"
      - id: TreeSitterEngine
        signature: "class TreeSitterEngine(Engine):"
        docstring: One tree-sitter grammar exposed through the common :class:`Engine` API. Instanc…
        children:
        - id: TreeSitterEngine.G4mRXX|jU5ePL
          signature: '''One tree-sitter grammar exposed through the common :class:`Engine` API.\n\n   …'
        - id: TreeSitterEngine.init
          signature: "def __init__(self, symbol: str) -> None:"
        - id: TreeSitterEngine.parse
          signature: "def _parse(self, data: bytes):"
        - id: TreeSitterEngine.parse_1
          signature: "def parse(self, source: str, path: Path | None=None) -> Tree:"
        - id: TreeSitterEngine.empty_tree
          signature: "def empty_tree(self, path: Path | None=None) -> Tree:"
        - id: TreeSitterEngine.serialize
          signature: "def serialize(self, tree: Tree) -> str:"
        - id: TreeSitterEngine.validate
          signature: "def validate(self, source: str) -> str | None:"
        - id: TreeSitterEngine.name
          signature: "def _name(self, node: Any) -> str | None:"
        - id: TreeSitterEngine.clean_heading
          signature: "@staticmethod def _clean_heading(raw: bytes) -> str:"
        - id: TreeSitterEngine.clean
          signature: "@staticmethod def _clean(raw: bytes) -> str:"
        - id: TreeSitterEngine.locate_all
          signature: "def locate_all(self, tree: Tree) -> list[Located]:"
        - id: TreeSitterEngine.locate_from
          signature: "def _locate_from(self, tree: Tree, root: Any, addressable: Callable[[Any, int], bool]) -> list[Located]:"
          docstring: "Shared node-walker: ``root`` and ``addressable`` let subclasses curate a differ…"
        - id: TreeSitterEngine.signature
          signature: "def signature(self, node: Any, limit: int=80) -> str:"
        - id: TreeSitterEngine.docstring
          signature: "def docstring(self, node: Any) -> str | None:"
        - id: TreeSitterEngine.node_code
          signature: "def node_code(self, node: Any) -> str:"
        - id: TreeSitterEngine.splice
          signature: "def _splice(self, tree: Tree, start: int, end: int, text: str) -> None:"
        - id: TreeSitterEngine.replace
          signature: "def replace(self, loc: Located, code: str) -> None:"
        - id: TreeSitterEngine.insert
          signature: "def insert(self, loc: Located, code: str, position: str) -> int:"
        - id: TreeSitterEngine.delete
          signature: "def delete(self, loc: Located) -> None:"
        - id: TreeSitterEngine.append
          signature: "def append(self, tree: Tree, code: str) -> int:"
      - id: dVVE3O|SWTGx5
        signature: _GENERIC_LEAF_LIMIT = 1500
      - id: GenericUnit
        signature: "class _GenericUnit:"
        docstring: "One collapsed node: a leaf once its own text fits the size budget (or it has no…"
        children:
        - id: GenericUnit.bfrIf8|TpeidL
          signature: "'One collapsed node: a leaf once its own text fits the size budget (or it\\n    …"
        - id: GenericUnit.init
          signature: "def __init__(self, node: Any, children: list[Any], source: bytes) -> None:"
        - id: GenericUnit.text
          signature: "@property def text(self) -> bytes:"
        - id: GenericUnit.child_by_field_name
          signature: "def child_by_field_name(self, field: str) -> Any:"
          docstring: "Delegate to the wrapped native node, so field-based naming keeps working."
      - id: generic_collapse
        signature: "def _generic_collapse(node: Any, source: bytes) -> _GenericUnit:"
        docstring: Collapse ``node`` into a leaf if its full text fits the budget or it has no nam…
      - id: GenericEngine
        signature: "class GenericEngine(TreeSitterEngine):"
        docstring: "Fallback for every tree-sitter grammar without a dedicated engine (XML, JSON, T…"
        children:
        - id: GenericEngine.bsYXGO|TZjEDE
          signature: "'Fallback for every tree-sitter grammar without a dedicated engine (XML,\\n    J…"
        - id: GenericEngine.locate_all
          signature: "def locate_all(self, tree: Tree) -> list[Located]:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_html.py
      nodes:
      - id: y8mVTk|6CKDFw
        signature: "'HTML tree-sitter engine: elements collapsed to a size budget, mirroring\\n:mod:…"
      - id: jBljOR|qkfu8N
        signature: from __future__ import annotations
      - id: gZoS4T|gb4FvV
        signature: "__all__ = ['HtmlEngine']"
      - id: html_units
        signature: "def _html_units(node: Any) -> list[Any]:"
      - id: HtmlUnit
        signature: "class _HtmlUnit:"
        docstring: "One collapsed HTML unit: either a childless leaf (its full text already fits th…"
        children:
        - id: HtmlUnit.WvyMBm|jsiKiz
          signature: "'One collapsed HTML unit: either a childless leaf (its full text already fits\\n…"
        - id: HtmlUnit.init
          signature: "def __init__(self, node: Any, children: list[Any], source: bytes) -> None:"
        - id: HtmlUnit.text
          signature: "@property def text(self) -> bytes:"
        - id: HtmlUnit.start_tag
          signature: "@property def start_tag(self) -> Any | None:"
          docstring: "The wrapped native node's own 'start_tag' child (tag name + attributes), looked…"
      - id: html_collapse
        signature: "def _html_collapse(node: Any, source: bytes) -> _HtmlUnit:"
        docstring: Collapse ``node`` into a leaf if its full text fits the budget (or it has no ne…
      - id: html_root_children
        signature: "def _html_root_children(root_node: Any, source: bytes) -> list[Any]:"
        docstring: "Top-level children of an HTML file: the doctype plus the root element(s)."
      - id: html_attr
        signature: "def _html_attr(start_tag: Any, attr_name: str) -> str | None:"
        docstring: "Value of ``attr_name`` on ``start_tag``, or ``None`` if absent/valueless."
      - id: HtmlEngine
        signature: "class HtmlEngine(TreeSitterEngine):"
        docstring: Tree-sitter HTML restructured into the size-collapsed leaves above.
        children:
        - id: HtmlEngine.p6ekgQ|XcU3kQ
          signature: '''Tree-sitter HTML restructured into the size-collapsed leaves above.'''
        - id: HtmlEngine.init
          signature: "def __init__(self) -> None:"
        - id: HtmlEngine.is_definition
          signature: "def is_definition(self, node_type: str) -> bool:"
        - id: HtmlEngine.signature
          signature: "def signature(self, node: Any, limit: int=80) -> str:"
        - id: HtmlEngine.html_name
          signature: "def _html_name(self, node: Any) -> str | None:"
        - id: HtmlEngine.locate_all
          signature: "def locate_all(self, tree: Tree) -> list[Located]:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_java.py
      nodes:
      - id: XbNVQm|m7KSDf
        signature: "\"Java tree-sitter engine: Classes/Interfaces/Enums -> Methods, mirroring the\\nP…"
      - id: jQxmOt|M0GhNd
        signature: from __future__ import annotations
      - id: sXmiXg|B2wymS
        signature: "__all__ = ['JavaEngine']"
      - id: QgXrqq|uA16qj
        signature: "'#: A single-line annotation (e.g. \\'@Override\\', \\'@SuppressWarnings(\"x\")\\'), …"
      - id: body_of
        signature: "def _body_of(def_node: Any) -> Any | None:"
      - id: JavaEngine
        signature: "class JavaEngine(TreeSitterEngine):"
        docstring: "Tree-sitter Java restructured like the Python engine: types/methods as real nod…"
        children:
        - id: JavaEngine.dlIP7g|KX2c79
          signature: "'Tree-sitter Java restructured like the Python engine: types/methods as\\n    re…"
        - id: JavaEngine.init
          signature: "def __init__(self) -> None:"
        - id: JavaEngine.is_definition
          signature: "def is_definition(self, node_type: str) -> bool:"
        - id: JavaEngine.signature
          signature: "def signature(self, node: Any, limit: int=80) -> str:"
          docstring: "Like the base heading, but skips leading annotation-only lines (e.g. a bare \"@O…"
        - id: JavaEngine.locate_all
          signature: "def locate_all(self, tree: Tree) -> list[Located]:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
      nodes:
      - id: qtFn4F|MX9hng
        signature: "'JavaScript tree-sitter engine: Classes/Functions -> Methods, mirroring the\\nJa…"
      - id: jQxmOt|BqPOJi
        signature: from __future__ import annotations
      - id: y0nJI5|6hMrNB
        signature: "__all__ = ['JavaScriptEngine']"
      - id: HiOEEk|lU1avj
        signature: "'#: declarator whose value is one of ``_FUNC_EXPR_TYPES``.'"
      - id: is_def
        signature: "def _is_def(node: Any) -> bool:"
        docstring: "Whether ``node`` is - or wraps, through a single ``export``/``const``/field lay…"
      - id: def_name
        signature: "def _def_name(node: Any) -> str | None:"
        docstring: "Name of a node ``_is_def`` accepted: the declaration's own identifier, or - unw…"
      - id: def_body
        signature: "def _def_body(node: Any) -> Any | None:"
        docstring: The innermost 'body' field reachable through the same unwrapping as ``_def_name…
      - id: JavaScriptEngine
        signature: "class JavaScriptEngine(TreeSitterEngine):"
        docstring: "Tree-sitter JavaScript restructured like the Python/Java engines: functions/cla…"
        children:
        - id: JavaScriptEngine.vxc2Dt|NqXM0s
          signature: '"Tree-sitter JavaScript restructured like the Python/Java engines:\n    functio…'
        - id: JavaScriptEngine.init
          signature: "def __init__(self) -> None:"
        - id: JavaScriptEngine.is_definition
          signature: "def is_definition(self, node_type: str) -> bool:"
        - id: JavaScriptEngine.signature
          signature: "def signature(self, node: Any, limit: int=80) -> str:"
        - id: JavaScriptEngine.locate_all
          signature: "def locate_all(self, tree: Tree) -> list[Located]:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
      nodes:
      - id: jdID6T|1rfWOt
        signature: "'``PythonEngine``: comment-preserving parse and ``unparse``-based serialisation…"
      - id: GDIcFO|KJiDkF
        signature: from __future__ import annotations
      - id: x58qrx|K1liXR
        signature: logger = logging.getLogger('xy.ai.mcpc.tools.ast.python')
      - id: FormattingUnparser
        signature: "class _FormattingUnparser(ast._Unparser):"
        docstring: '``ast.unparse`` variant that reflows overlong single-line statements. ``ast.unp…'
        children:
        - id: FormattingUnparser.mYwOBV|pRGi2c
          signature: '''``ast.unparse`` variant that reflows overlong single-line statements.\n\n    `…'
        - id: FormattingUnparser.4huP6F|GGM4gX
          signature: MAX_LINE_LENGTH = 120
        - id: FormattingUnparser.traverse
          signature: "def traverse(self, node):"
        - id: FormattingUnparser.fix_code
          signature: "def _fix_code(self, code: str, max_line_length: int, node: ast.AST) -> str | None:"
      - id: unparse
        signature: "def _unparse(node: ast.AST) -> str:"
      - id: PythonEngine
        signature: "class PythonEngine(Engine):"
        docstring: "``ast``-based engine: comment-preserving parse, ``unparse`` serialisation."
        children:
        - id: PythonEngine.ZQwNsa|SemoV5
          signature: "'``ast``-based engine: comment-preserving parse, ``unparse`` serialisation.'"
        - id: PythonEngine.parse
          signature: "def parse(self, source: str, path: Path | None=None) -> Tree:"
        - id: PythonEngine.parse_module
          signature: "def _parse_module(self, source: str) -> ast.Module:"
        - id: PythonEngine.parse_fragment
          signature: "def _parse_fragment(self, code: str) -> list[ast.stmt]:"
        - id: PythonEngine.empty_tree
          signature: "def empty_tree(self, path: Path | None=None) -> Tree:"
        - id: PythonEngine.serialize
          signature: "def serialize(self, tree: Tree) -> str:"
        - id: PythonEngine.validate
          signature: "def validate(self, source: str) -> str | None:"
        - id: PythonEngine.loc
          signature: "def _loc(self, tree, node, parent, index, name, nid, expandable=False) -> Located:"
        - id: PythonEngine.locate_all
          signature: "def locate_all(self, tree: Tree) -> list[Located]:"
        - id: PythonEngine.is_definition
          signature: "def is_definition(self, node_type: str) -> bool:"
        - id: PythonEngine.signature
          signature: "def signature(self, node: Any, limit: int=80) -> str:"
        - id: PythonEngine.docstring
          signature: "def docstring(self, node: Any, limit: int=80) -> str | None:"
        - id: PythonEngine.node_code
          signature: "def node_code(self, node: Any) -> str:"
        - id: PythonEngine.replace
          signature: "def replace(self, loc: Located, code: str) -> None:"
        - id: PythonEngine.insert
          signature: "def insert(self, loc: Located, code: str, position: str) -> int:"
        - id: PythonEngine.delete
          signature: "def delete(self, loc: Located) -> None:"
        - id: PythonEngine.append
          signature: "def append(self, tree: Tree, code: str) -> int:"
      - id: aNHEQO|lnQb1N
        signature: '''# Shared instance; the Python engine is stateless.'''
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_java.py
      nodes:
      - id: zrXNbR|3dRepm
        signature: "\"Java engine: JavaParser-based ``ast-java`` process, driven over RPC/PIC.\\n\\nRe…"
      - id: ILN4fu|sCWK6E
        signature: from __future__ import annotations
      - id: JpPcRr|tupArz
        signature: "__all__ = ['JavaEngine']"
      - id: workspace_root
        signature: "def _workspace_root() -> Path:"
      - id: vz3nHu|7E9CVU
        signature: _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'java'
      - id: JavaProcess
        signature: "class JavaProcess(RpcProcess):"
        docstring: Starts the prebuilt ``ast-java`` JavaParser engine (built via ``run.sh --build`…
        children:
        - id: JavaProcess.0qSEGQ|3JgJWY
          signature: '''Starts the prebuilt ``ast-java`` JavaParser engine (built via ``run.sh --build…'
        - id: JavaProcess.default_port
          signature: "def default_port(self) -> int:"
        - id: JavaProcess.cwd
          signature: "def cwd(self) -> str | None:"
        - id: JavaProcess.command
          signature: "def command(self, port: int) -> list[str]:"
        - id: JavaProcess.java_tool
          signature: "def _java_tool(self, name: str) -> str:"
        - id: JavaProcess.classpath
          signature: "def _classpath(self) -> str:"
      - id: g8i7OL|HTI1Ea
        signature: _PROCESS = JavaProcess()
      - id: JavaEngine
        signature: "class JavaEngine(RpcEngine):"
        docstring: "Java, parsed and mutated by JavaParser via the ``ast-java`` RPC process."
        children:
        - id: JavaEngine.1qmNSs|TC4ByX
          signature: "'Java, parsed and mutated by JavaParser via the ``ast-java`` RPC process.'"
        - id: JavaEngine.init
          signature: "def __init__(self) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_rust.py
      nodes:
      - id: reOPSB|2w0163
        signature: "\"Rust engine: syn-based ``xy_ai_ast_rust`` process, driven over RPC/PIC.\\n\\nSta…"
      - id: ILN4fu|sCWK6E
        signature: from __future__ import annotations
      - id: b5aB3x|qU00Yb
        signature: "__all__ = ['RustEngine']"
      - id: workspace_root
        signature: "def _workspace_root() -> Path:"
      - id: mnFJcA|J3zcEx
        signature: _ENGINE_DIR = _workspace_root() / 'ast-engines' / 'rust'
      - id: RustProcess
        signature: "class RustProcess(RpcProcess):"
        docstring: Starts the prebuilt ``xy_ai_ast_rust`` syn engine (built via ``run.sh --build``…
        children:
        - id: RustProcess.0qSEGQ|1SP6FA
          signature: '''Starts the prebuilt ``xy_ai_ast_rust`` syn engine (built via ``run.sh --build`…'
        - id: RustProcess.default_port
          signature: "def default_port(self) -> int:"
        - id: RustProcess.cwd
          signature: "def cwd(self) -> str | None:"
        - id: RustProcess.command
          signature: "def command(self, port: int) -> list[str]:"
      - id: F0cCaO|KRBgB9
        signature: _PROCESS = RustProcess()
      - id: RustEngine
        signature: "class RustEngine(RpcEngine):"
        docstring: "Rust, parsed and mutated by ``syn`` via the ``xy_ai_ast_rust`` RPC process."
        children:
        - id: RustEngine.pPxVPO|QPQZCQ
          signature: "'Rust, parsed and mutated by ``syn`` via the ``xy_ai_ast_rust`` RPC process.'"
        - id: RustEngine.init
          signature: "def __init__(self) -> None:"
```
/answer 0a99-4d-4c allow
ReasoningToken: 22
Control Request:
```yaml
id: 0a99-4e-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  reason: Struktur aller AST-Tools erfassen
```
/answer 0a99-4e-0 allow
Control Request:
```yaml
id: 0a99-4f-4e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
      nodes:
      - id: ZAIFPk|rsb8Xe
        signature: "'``ast_create`` tool: create one or more files from source, creating missing di…"
      - id: wlvDdw|agm8HT
        signature: from dataclasses import dataclass
      - id: wtzT47|EP6uG9
        signature: "__all__ = ["
      - id: CreateFileItem
        signature: "@dataclass(frozen=True) class CreateFileItem:"
        docstring: "One file to create. Attributes: path: Absolute path of the file to create. sour…"
      - id: CreateFileResult
        signature: "@dataclass(frozen=True) class CreateFileResult:"
        docstring: "Result of creating a single file. Attributes: path: The path exactly as given i…"
      - id: CreateFileError
        signature: "@dataclass(frozen=True) class CreateFileError:"
        docstring: "Error creating a single file. Attributes: path: The path exactly as given in th…"
      - id: CreateFilesResult
        signature: "@dataclass(frozen=True) class CreateFilesResult:"
        docstring: "Result of :func:`ast_create`. Attributes: results: One :class:`CreateFileResult…"
      - id: create_one
        signature: "def _create_one(item: CreateFileItem) -> CreateFileResult:"
      - id: ast_create
        signature: "def ast_create(items: list[CreateFileItem]) -> CreateFilesResult:"
        docstring: Create one or more files from source (each validated by parsing it). Creating a…
      - id: CreateFileTool
        signature: "class CreateFileTool(ToolDefinition):"
        children:
        - id: CreateFileTool.omImBZ|s2PVZe
          signature: name = 'ast_create'
        - id: CreateFileTool.ei5nIo|Lu9IT0
          signature: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTIO…"
        - id: CreateFileTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_create`, translating the MCP schema to/from the Python A…"
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
      - id: CpezhJ|nJnt4G
        signature: "'``ast_delete`` tool: delete selected nodes, or whole files if none selected.'"
      - id: wlvDdw|eXR9kh
        signature: from dataclasses import dataclass
      - id: zTddKa|ZtwRn0
        signature: "__all__ = ['DeleteItem', 'DeleteResult', 'DeleteError', 'DeleteBatchResult', 'a…"
      - id: DeleteItem
        signature: "@dataclass(frozen=True) class DeleteItem:"
        docstring: "One node (or file) to delete. Attributes: path: Absolute path to the file to mo…"
      - id: DeleteResult
        signature: "@dataclass(frozen=True) class DeleteResult:"
        docstring: "Result of deleting a single node or file. Attributes: path: The path exactly as…"
      - id: DeleteError
        signature: "@dataclass(frozen=True) class DeleteError:"
        docstring: "Error deleting a single node or file. Attributes: path: The path exactly as giv…"
      - id: DeleteBatchResult
        signature: "@dataclass(frozen=True) class DeleteBatchResult:"
        docstring: "Result of :func:`ast_delete`. Attributes: results: One :class:`DeleteResult` pe…"
      - id: delete_one
        signature: "def _delete_one(item: DeleteItem) -> DeleteResult:"
      - id: ast_delete
        signature: "def ast_delete(items: list[DeleteItem]) -> DeleteBatchResult:"
        docstring: "Delete one or more selected nodes, or whole files if their ``id`` is omitted. A…"
      - id: DeleteTool
        signature: "class DeleteTool(ToolDefinition):"
        children:
        - id: DeleteTool.gLGwBi|LEwdCL
          signature: name = 'ast_delete'
        - id: DeleteTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python A…"
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: mQ7CrW|aoOqEy
        signature: "'``ast_edit_block`` tool: exact-block (old_text -> new_text) edits within selec…"
      - id: 0NkjOX|zkSTbq
        signature: from dataclasses import dataclass
      - id: qzaxoR|Ancd56
        signature: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: B1sD7y|298I9E
        signature: "__all__ = ["
      - id: EditBlockItem
        signature: "@dataclass(frozen=True) class EditBlockItem:"
        docstring: "One block edit to apply. Attributes: path: Absolute path to the file to modify.…"
      - id: EditBlockResult
        signature: "@dataclass(frozen=True) class EditBlockResult:"
        docstring: "Result of applying a single block edit. Attributes: path: The path exactly as g…"
      - id: EditBlockError
        signature: "@dataclass(frozen=True) class EditBlockError:"
        docstring: "Error applying a single block edit. Attributes: path: The path exactly as given…"
      - id: EditBlockBatchResult
        signature: "@dataclass(frozen=True) class EditBlockBatchResult:"
        docstring: "Result of :func:`ast_edit_block`. Attributes: results: One :class:`EditBlockRes…"
      - id: node_guard
        signature: "def _node_guard(engine, reference):"
        docstring: Guard for tolerant node edits. Engines that flag malformed edits on replace nee…
      - id: edit_block_one
        signature: "def _edit_block_one(item: EditBlockItem) -> EditBlockResult:"
      - id: ast_edit_block
        signature: "def ast_edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:"
        docstring: Replace occurrence(s) of ``old_text`` with ``new_text`` inside one or more node…
      - id: EditBlockNodeTool
        signature: "class EditBlockNodeTool(ToolDefinition):"
        children:
        - id: EditBlockNodeTool.h2DGqi|I7XJBX
          signature: name = 'ast_edit_block'
        - id: EditBlockNodeTool.MuT0pV|iYKmvH
          signature: "_ITEM_PROPERTIES = {"
        - id: EditBlockNodeTool.ZDSH0j|Jr7UgN
          signature: "_ITEM_REQUIRED = ['path', 'old_text', 'new_text', 'id']"
        - id: EditBlockNodeTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST …"
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
      nodes:
      - id: zKe9Du|5qARzC
        signature: "'``ast_edit_marks`` tool: mark-based edits within the source of selected nodes.'"
      - id: 0NkjOX|Zg7YTi
        signature: from dataclasses import dataclass
      - id: qzaxoR|Ancd56
        signature: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: IklaUl|hqD6GH
        signature: "__all__ = ["
      - id: EditMarksItem
        signature: "@dataclass(frozen=True) class EditMarksItem:"
        docstring: "One marker-based edit to apply. Attributes: path: Absolute path to the file to …"
      - id: EditMarksResult
        signature: "@dataclass(frozen=True) class EditMarksResult:"
        docstring: "Result of applying a single marker-based edit. Attributes: path: The path exact…"
      - id: EditMarksError
        signature: "@dataclass(frozen=True) class EditMarksError:"
        docstring: "Error applying a single marker-based edit. Attributes: path: The path exactly a…"
      - id: EditMarksBatchResult
        signature: "@dataclass(frozen=True) class EditMarksBatchResult:"
        docstring: "Result of :func:`ast_edit_marks`. Attributes: results: One :class:`EditMarksRes…"
      - id: node_marks_guard
        signature: "def _node_marks_guard(engine, begin_marker, end_marker):"
        docstring: Guard for tolerant marker edits (see ``ast.edit_block._node_guard``).
      - id: edit_marks_one
        signature: "def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:"
      - id: ast_edit_marks
        signature: "def ast_edit_marks(items: list[EditMarksItem]) -> EditMarksBatchResult:"
        docstring: Replace everything between the start/end markers inside one or more nodes. Each…
      - id: EditMarksNodeTool
        signature: "class EditMarksNodeTool(ToolDefinition):"
        children:
        - id: EditMarksNodeTool.nfiMrL|QgOVDW
          signature: name = 'ast_edit_marks'
        - id: EditMarksNodeTool.MuT0pV|RNoeCK
          signature: "_ITEM_PROPERTIES = {"
        - id: EditMarksNodeTool.ZDSH0j|PEqhnG
          signature: "_ITEM_REQUIRED = ['path', 'start_marker', 'end_marker', 'content']"
        - id: EditMarksNodeTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST …"
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      nodes:
      - id: 9CmN9x|nHobTc
        signature: "'``ast_insert`` tool: insert statement(s) relative to selected nodes.'"
      - id: wlvDdw|eXR9kh
        signature: from dataclasses import dataclass
      - id: oTjM4y|Jut0OL
        signature: "__all__ = ['InsertItem', 'InsertResult', 'InsertError', 'InsertBatchResult', 'a…"
      - id: InsertItem
        signature: "@dataclass(frozen=True) class InsertItem:"
        docstring: "One insert operation. Attributes: path: Absolute path to the file to modify. so…"
      - id: InsertResult
        signature: "@dataclass(frozen=True) class InsertResult:"
        docstring: "Result of a single insert operation. Attributes: path: The path exactly as give…"
      - id: InsertError
        signature: "@dataclass(frozen=True) class InsertError:"
        docstring: "Error applying a single insert operation. Attributes: path: The path exactly as…"
      - id: InsertBatchResult
        signature: "@dataclass(frozen=True) class InsertBatchResult:"
        docstring: "Result of :func:`ast_insert`. Attributes: results: One :class:`InsertResult` pe…"
      - id: insert_one
        signature: "def _insert_one(item: InsertItem) -> InsertResult:"
      - id: ast_insert
        signature: "def ast_insert(items: list[InsertItem]) -> InsertBatchResult:"
        docstring: Insert statement(s) parsed from ``source`` relative to one or more selected nod…
      - id: InsertNodeTool
        signature: "class InsertNodeTool(ToolDefinition):"
        children:
        - id: InsertNodeTool.T43CMn|tx9Kq0
          signature: name = 'ast_insert'
        - id: InsertNodeTool.c0C4qn|tz50Lf
          signature: "_ITEM_PROPERTIES = {"
        - id: InsertNodeTool.ei5nIo|Lu9IT0
          signature: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTIO…"
        - id: InsertNodeTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: G65kv0|hrDhpa
        signature: "'``ast_replace`` tool: replace selected nodes with new source.'"
      - id: wlvDdw|eXR9kh
        signature: from dataclasses import dataclass
      - id: xj2nBU|s4L0vm
        signature: "__all__ = ["
      - id: ReplaceItem
        signature: "@dataclass(frozen=True) class ReplaceItem:"
        docstring: "One node replacement to apply. Attributes: path: Absolute path to the file to m…"
      - id: ReplaceResult
        signature: "@dataclass(frozen=True) class ReplaceResult:"
        docstring: "Result of a single node replacement. Attributes: path: The path exactly as give…"
      - id: ReplaceError
        signature: "@dataclass(frozen=True) class ReplaceError:"
        docstring: "Error applying a single node replacement. Attributes: path: The path exactly as…"
      - id: ReplaceBatchResult
        signature: "@dataclass(frozen=True) class ReplaceBatchResult:"
        docstring: "Result of :func:`ast_replace`. Attributes: results: One :class:`ReplaceResult` …"
      - id: replace_one
        signature: "def _replace_one(item: ReplaceItem) -> ReplaceResult:"
      - id: ast_replace
        signature: "def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:"
        docstring: Replace one or more selected nodes with new source. ``id`` and replacement ``so…
      - id: ReplaceNodeTool
        signature: "class ReplaceNodeTool(ToolDefinition):"
        children:
        - id: ReplaceNodeTool.jKVWKu|S7XtH2
          signature: name = 'ast_replace'
        - id: ReplaceNodeTool.hKJ2Rq|SrCCxe
          signature: _ITEMS_DESCRIPTION = 'Node replacements to apply.'
        - id: ReplaceNodeTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 0a99-4f-4e allow
Control Request:
```yaml
id: 0a99-50-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  reason: Gesamte Dateistruktur des ast-Pakets sehen
```
/answer 0a99-50-0 allow
Control Request:
```yaml
id: 0a99-51-50
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      entries:
      - ".:"
      - __init__.py
      - base.py
      - common.py
      - core.py
      - create.py
      - delete.py
      - edit_block.py
      - edit_marks.py
      - find.py
      - insert.py
      - outline.py
      - read.py
      - replace.py
      - script.py
      - validate.py
      - "./generic:"
      - __init__.py
      - _css.py
      - _engine.py
      - _html.py
      - _java.py
      - _javascript.py
      - _markdown.py
      - _text.py
      - _xml.py
      - _yaml.py
      - "./python:"
      - __init__.py
      - _comments.py
      - _engine.py
      - _nodes.py
      - reformat_cli.py
      - "./rpc:"
      - __init__.py
      - _engine.py
      - _java.py
      - _process.py
      - _rust.py
```
/answer 0a99-51-50 allow
ReasoningToken: 72
Control Request:
```yaml
id: 0a99-52-0
toolName: python
arguments:
  script: |-
    import pathlib
    b=pathlib.Path('/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast')
    for n in ['base.py','common.py']:
        t=(b/n).read_text()
        print('=====',n,len(t.splitlines()))
        print(t)
  reason: "base.py und common.py komplett lesen (Engine-API, Schema-Helfer)"
```
/answer 0a99-52-0 deny nutze AST
Text:
 Verstanden, ich nutze die AST-Tools.
Control Request:
```yaml
id: 0a99-53-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
  reason: "Engine-Basisklasse, Schema-Helfer und RPC-Engine strukturell erfassen"
```
/answer 0a99-53-0 allow
Control Request:
```yaml
id: 0a99-54-53
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: ZjoNfB|uJIfID
        signature: '''Engine-agnostic model shared by every ``ast_*`` tool.\n\nThe tools address nod…'
      - id: QuHMWF|e2q5UN
        signature: from __future__ import annotations
      - id: AstError
        signature: "class AstError(Exception):"
        docstring: "A user-facing, path-free error raised by the AST tools."
      - id: AstAmbiguous
        signature: "class AstAmbiguous(AstError):"
        docstring: Raised when a text-based node search (no id given) finds several unrelated cand…
        children:
        - id: AstAmbiguous.FSy0jO|jchvnz
          signature: '''Raised when a text-based node search (no id given) finds several\n    unrelate…'
        - id: AstAmbiguous.init
          signature: "def __init__(self, message: str, candidates: list[str]) -> None:"
      - id: AstTextError
        signature: "class AstTextError(AstError):"
        docstring: Raised when a text/marker-based edit's search text could not be applied. Carrie…
        children:
        - id: AstTextError.xHQ70j|YI3gjQ
          signature: '"Raised when a text/marker-based edit''s search text could not be applied.\n\n  …'
        - id: AstTextError.init
          signature: "def __init__(self, message: str, *, reason: str | None=None, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:"
      - id: Tree
        signature: "@dataclass class Tree:"
        docstring: "A parsed file/snippet plus the engine that owns it. Attributes: engine: The eng…"
      - id: Located
        signature: "@dataclass class Located:"
        docstring: A node with the engine-independent metadata the selectors match on. Attributes:…
      - id: OutlineNode
        signature: "@dataclass(frozen=True) class OutlineNode:"
        docstring: "One node in a structural (list/find/read) result. ``id`` is the node's unique, …"
      - id: line_range
        signature: "def line_range(loc: Located) -> str:"
        docstring: "Return ``loc``'s start line, or a ``\"start-end\"`` range if it spans several."
      - id: 1vtTBH|EXC8hj
        signature: _ID_CLEAN_RE = re.compile('\\W+')
      - id: hash
        signature: "def _hash(name: str, length: int) -> str:"
      - id: ThMOZm|vomnxc
        signature: _ID_HASH_ALPHABET = string.digits + string.ascii_letters
      - id: base62_hash
        signature: "def _base62_hash(text: str, length: int) -> str:"
        docstring: Base62 (0-9a-zA-Z) digest of ``text``.
      - id: content_hash
        signature: "def _content_hash(content: str, length: int=6) -> str:"
        docstring: "Base62 digest of ``content``, stable across unrelated tree edits."
      - id: content_prefix_hash
        signature: "def _content_prefix_hash(content: str, length: int=6) -> str:"
        docstring: Base62 digest of ``content``'s whitespace-stripped first/last 20 chars. Forms a…
      - id: mFoC5C|Xl4C8a
        signature: _ID_SUFFIX_RE = re.compile('_\\d+$')
      - id: id_prefix
        signature: "def _id_prefix(segment: str) -> str | None:"
        docstring: "An anonymous id segment's stable prefix (before ``'|'``), if any."
      - id: resolve_by_prefix
        signature: "def resolve_by_prefix(located: list['Located'], target_id: str) -> 'Located | None':"
        docstring: Fallback selector for a ``target_id`` that matches no node exactly. Used when a…
      - id: id_segment
        signature: "def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:"
        docstring: "Return a unique-within-siblings id segment, name-based when feasible. A clean, …"
      - id: node_outline
        signature: "def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, with_type: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:"
        docstring: "Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``,…"
      - id: compact
        signature: "def _compact(value: Any) -> Any:"
        docstring: Recursively drop ``None`` values and empty lists from a dataclass-derived struc…
      - id: to_dict
        signature: "def to_dict(node: OutlineNode) -> dict:"
        docstring: "Serialize an :class:`OutlineNode` to MCP output, omitting empty fields."
      - id: TreeNode
        signature: "@dataclass class _TreeNode:"
      - id: build_forest
        signature: "def _build_forest(located: list[Located]) -> list[_TreeNode]:"
        docstring: Nest a pre-order list of ``Located`` into a forest via ``node_id`` prefixes.
      - id: build_outline
        signature: "def build_outline(located: list[Located], *, with_code: bool=False, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:"
        docstring: "Build the nested outline of ``located`` (source only if ``with_code``, lines on…"
      - id: outline_nodes
        signature: "def _outline_nodes(nodes: list['_TreeNode'], *, with_code: bool, with_lines: bool=True, with_type: bool=True) -> list[OutlineNode]:"
        docstring: "Convert a forest into OutlineNodes, collapsing non-expandable nodes to full sou…"
      - id: resolve_by_name
        signature: "def _resolve_by_name(key: str, by_name: dict[str, list['_TreeNode']]) -> tuple['_TreeNode | None', str | None]:"
        docstring: Resolve ``key`` against node names when it doesn't match an id directly. Tries …
      - id: read_subtrees
        signature: "def read_subtrees(located: list[Located], keys: list[str], *, with_lines: bool=True) -> tuple[list[OutlineNode], list[str]]:"
        docstring: "Return one read subtree per resolvable ``keys`` entry. Each key is matched, in …"
      - id: matches
        signature: "def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> bool:"
      - id: find
        signature: "def find(tree: Tree, **filters: object) -> list[Located]:"
      - id: most_specific
        signature: "def most_specific(located: list[Located], lineno: int, end_lineno: int) -> Located | None:"
        docstring: "Return the smallest node in *located* fully containing lines [lineno, end_linen…"
      - id: Engine
        signature: "class Engine(ABC):"
        docstring: "A parser back-end turning source into an addressable, mutable tree. Structural …"
        children:
        - id: Engine.yeTUw1|1zTXCW
          signature: "'A parser back-end turning source into an addressable, mutable tree.\\n\\n    Str…"
        - id: Engine.dqC7gX|bz1VNl
          signature: "'#: Whether ``validate``/``replace`` reliably reject malformed edits. Only then'"
        - id: Engine.parse
          signature: "@abstractmethod def parse(self, source: str, path: Path | None=None) -> Tree:"
          docstring: "Parse ``source`` into a :class:`Tree`, raising :class:`AstError` on error."
        - id: Engine.empty_tree
          signature: "@abstractmethod def empty_tree(self, path: Path | None=None) -> Tree:"
          docstring: "Return an empty tree, used when appending to a not-yet-existing file."
        - id: Engine.serialize
          signature: "@abstractmethod def serialize(self, tree: Tree) -> str:"
          docstring: Render ``tree`` back to source text for writing to disk.
        - id: Engine.validate
          signature: "@abstractmethod def validate(self, source: str) -> str | None:"
          docstring: "Return an error message if ``source`` is malformed, else ``None``."
        - id: Engine.locate_all
          signature: "@abstractmethod def locate_all(self, tree: Tree) -> list[Located]:"
          docstring: "Flatten ``tree`` into every addressable node, in document order."
        - id: Engine.is_definition
          signature: "def is_definition(self, node_type: str) -> bool:"
          docstring: Whether ``node_type`` is "def-like" enough for a ``signature`` to make sense. D…
        - id: Engine.signature
          signature: "@abstractmethod def signature(self, node: Any) -> str:"
          docstring: One-line rendering of ``node``'s header (or the node itself).
        - id: Engine.default_signature
          signature: "def default_signature(self, node: Any, limit: int=80) -> str:"
          docstring: "Fallback used when ``is_definition`` is False: the first non-blank line of the …"
        - id: Engine.docstring
          signature: "@abstractmethod def docstring(self, node: Any) -> str | None:"
          docstring: "Short docstring of ``node``, if the format has such a concept."
        - id: Engine.node_code
          signature: "@abstractmethod def node_code(self, node: Any) -> str:"
          docstring: Full source of a single ``node``.
        - id: Engine.replace
          signature: "@abstractmethod def replace(self, loc: Located, code: str) -> None:"
          docstring: Replace ``loc``'s node with ``code``.
        - id: Engine.insert
          signature: "@abstractmethod def insert(self, loc: Located, code: str, position: str) -> int:"
          docstring: Insert ``code`` ``"before"``/``"after"`` ``loc``; return units inserted.
        - id: Engine.delete
          signature: "@abstractmethod def delete(self, loc: Located) -> None:"
          docstring: Delete ``loc``'s node from its container.
        - id: Engine.append
          signature: "@abstractmethod def append(self, tree: Tree, code: str) -> int:"
          docstring: Append ``code`` at ``tree``'s top level; return units appended.
      - id: require_path
        signature: "def require_path(path_str: str, *, must_exist: bool=True) -> Path:"
        docstring: "Validate a mandatory absolute path, raising :class:`AstError` on failure."
      - id: GaNJOk|lfM39j
        signature: "_CONTROL_CHAR_RE = re.compile('[\\x00-\\x08\\x0b\\x0c\\x0e-\\x1f\\x7f]')"
      - id: check_no_control_chars
        signature: "def check_no_control_chars(code: str) -> None:"
        docstring: Raise :class:`AstError` if ``code`` contains abnormal control characters (e.g. …
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
      nodes:
      - id: Q9NkRl|1c3Fpi
        signature: '''Selector machinery shared by the ``ast_*`` tools.\n\n``ast_find`` is the only …'
      - id: QUDAPO|6eecMb
        signature: from typing import Any
      - id: w0xyc1|6KBBvL
        signature: "__all__ = ['SELECTOR_PROPS', 'PATH_SELECTOR_PROPS', 'PATH_PROP', 'select_one', …"
      - id: clnjIr|Jz72IC
        signature: "SELECTOR_PROPS = {"
      - id: BdeVO2|NSUKUv
        signature: "'#: Path-only selectors used by every mutation tool (replace/insert/delete/edit…"
      - id: select_one
        signature: "def select_one(tree, **selectors: Any) -> core.Located:"
        docstring: "Return the single node in *tree* matching *selectors*. Raises: core.AstError: I…"
      - id: select_by_path
        signature: "def select_by_path(tree, *, id: str | None=None) -> core.Located:"
        docstring: "Return the single node in *tree* addressed by its unique ``id``. Raises: core.A…"
      - id: select_by_text
        signature: "def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:"
        docstring: "Return the node addressed by ``id``, or, if omitted, the single node whose sour…"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
      - id: V6fdSR|z5Q7Tq
        signature: "'Facade the ``ast_*`` tools call, dispatching to a per-file engine.\\n\\nThe engi…"
      - id: CXxOCc|yqtm8z
        signature: from __future__ import annotations
      - id: 85RFV8|RBFWwR
        signature: "_PYTHON_EXTENSIONS = ('.py', '.pyi')"
      - id: engine_for_path
        signature: "def engine_for_path(path: Path) -> Engine:"
        docstring: Return the engine responsible for ``path`` based on its extension.
      - id: CacheEntry
        signature: "@dataclass class _CacheEntry:"
      - id: AstCache
        signature: "class AstCache:"
        docstring: Content-hash validated cache of parsed trees keyed by absolute path.
        children:
        - id: AstCache.AzRmHX|ayni5v
          signature: '''Content-hash validated cache of parsed trees keyed by absolute path.'''
        - id: AstCache.init
          signature: "def __init__(self) -> None:"
        - id: AstCache.get_tree
          signature: "def get_tree(self, path: Path) -> Tree:"
        - id: AstCache.save
          signature: "def save(self, path: Path, tree: Tree) -> str:"
          docstring: "Serialise *tree*, write it to *path* and refresh the cache entry."
        - id: AstCache.invalidate
          signature: "def invalidate(self, path: Path) -> None:"
      - id: dRq1FJ|5EG9eJ
        signature: "'#: Process-wide shared cache instance.'"
      - id: load
        signature: "def load(path_str: str) -> tuple[Path, Tree]:"
        docstring: Resolve *path_str* and return it together with its cached tree.
      - id: parse_source
        signature: "def parse_source(source: str, engine: Engine | None=None) -> Tree:"
        docstring: Parse *source* with *engine* (Python by default for path-less snippets).
      - id: parse_for
        signature: "def parse_for(path_str: str, code: str) -> Tree:"
        docstring: Parse *code* with the engine selected for *path_str*'s extension.
      - id: locate_all
        signature: "def locate_all(tree: Tree) -> list[Located]:"
      - id: edit_node_source
        signature: "def edit_node_source(loc: Located) -> str:"
      - id: relocate
        signature: "def relocate(tree: Tree, old: Located) -> Located | None:"
        docstring: Find the node now occupying ``old``'s former slot after an in-place edit. Match…
      - id: replace_node
        signature: "def replace_node(loc: Located, code: str) -> str | None:"
        docstring: Replace ``loc``'s node with ``code``; return its new id.
      - id: insert_node
        signature: "def insert_node(loc: Located, code: str, position: str) -> int:"
      - id: delete_node
        signature: "def delete_node(loc: Located) -> None:"
      - id: append_nodes
        signature: "def append_nodes(tree: Tree, code: str) -> int:"
      - id: empty_tree
        signature: "def empty_tree(path: Path) -> Tree:"
      - id: validate_source
        signature: "def validate_source(path: Path, source: str) -> str | None:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
      nodes:
      - id: BQIdfu|0WxhPP
        signature: '"Generic :class:`Engine` back-end for any parser exposed via the RPC/PIC\nHTTP …'
      - id: Da4IMM|t2aMCb
        signature: from __future__ import annotations
      - id: OAbX3K|ljF50V
        signature: from xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest…
      - id: ibsDCF|76lOk2
        signature: "__all__ = ['RpcEngine']"
      - id: RpcNode
        signature: "@dataclass class _RpcNode:"
        docstring: One node as reported by ``POST /nodes``. Carries everything :meth:`RpcEngine.si…
      - id: RpcEngine
        signature: "class RpcEngine(Engine):"
        docstring: One RPC/PIC engine process exposed through the common :class:`Engine` API. Stru…
        children:
        - id: RpcEngine.95ZeHe|fDr6kO
          signature: '"One RPC/PIC engine process exposed through the common :class:`Engine` API.\n\n…'
        - id: RpcEngine.init
          signature: "def __init__(self, symbol: str, process: RpcProcess) -> None:"
        - id: RpcEngine.rpc
          signature: "def _rpc(self) -> AppendInfoNodesValidateClientImpl:"
        - id: RpcEngine.unwrap
          signature: "@staticmethod def _unwrap(response: Any) -> Any:"
          docstring: "Return a 200 response's typed body, or raise on any other status."
        - id: RpcEngine.MltvBv|wKemBs
          signature: '''# -- Engine ------------------------------------------------------------'''
        - id: RpcEngine.parse
          signature: "def parse(self, source: str, path: Path | None=None) -> Tree:"
        - id: RpcEngine.empty_tree
          signature: "def empty_tree(self, path: Path | None=None) -> Tree:"
        - id: RpcEngine.serialize
          signature: "def serialize(self, tree: Tree) -> str:"
        - id: RpcEngine.validate
          signature: "def validate(self, source: str) -> str | None:"
        - id: RpcEngine.is_definition
          signature: "def is_definition(self, node_type: str) -> bool:"
        - id: RpcEngine.locate_all
          signature: "def locate_all(self, tree: Tree) -> list[Located]:"
        - id: RpcEngine.signature
          signature: "def signature(self, node: _RpcNode) -> str:"
        - id: RpcEngine.docstring
          signature: "def docstring(self, node: _RpcNode) -> str | None:"
        - id: RpcEngine.node_code
          signature: "def node_code(self, node: _RpcNode) -> str:"
        - id: RpcEngine.replace
          signature: "def replace(self, loc: Located, code: str) -> None:"
        - id: RpcEngine.insert
          signature: "def insert(self, loc: Located, code: str, position: str) -> int:"
        - id: RpcEngine.delete
          signature: "def delete(self, loc: Located) -> None:"
        - id: RpcEngine.append
          signature: "def append(self, tree: Tree, code: str) -> int:"
```
/answer 0a99-54-53 allow
ReasoningToken: 59
Control Request:
```yaml
id: 0a99-55-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    ids:
    - Tree
    - Located
    - Engine
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    ids:
    - engine_for_path
    - AstCache
    - load
    - append_nodes
    - empty_tree
    - validate_source
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    ids:
    - ReplaceItem
    - ReplaceResult
    - replace_one
    - ast_replace
    - ReplaceNodeTool
    - register
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
    ids:
    - clnjIr|Jz72IC
    - BdeVO2|NSUKUv
  reason: "Kern-API (Tree, Engine, Cache) und ein komplettes Tool (replace) als Referenz lesen"
```
/answer 0a99-55-0 allow
Control Request:
```yaml
id: 0a99-56-55
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
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
      - id: Engine
        type: ClassDef
        signature: "class Engine(ABC):"
        docstring: "A parser back-end turning source into an addressable, mutable tree. Structural …"
        children:
        - id: Engine.yeTUw1|1zTXCW
          type: statements
          code: |-
            'A parser back-end turning source into an addressable, mutable tree.\n\n    Structural mutations differ fundamentally between back-ends: the Python\n    engine edits the ``ast`` object graph and re-serialises it via ``unparse``,\n    whereas generic engines splice source text at node byte-ranges and re-parse.\n    Both, however, expose the same node-oriented operations below.\n    '
            '#: Human-readable engine name (used e.g. to guard Python-only tools).'
            name: str = 'engine'
        - id: Engine.dqC7gX|bz1VNl
          type: statements
          code: |-
            '#: Whether ``validate``/``replace`` reliably reject malformed edits. Only then'
            '#: may callers rely on re-parse to catch corruption (false for markup grammars'
            '#: whose parser accepts almost any text without reporting errors).'
            validates_syntax: bool = False
        - id: Engine.parse
          type: FunctionDef
          code: |-
            @abstractmethod
            def parse(self, source: str, path: Path | None=None) -> Tree:
                """Parse ``source`` into a :class:`Tree`, raising :class:`AstError` on error."""
        - id: Engine.empty_tree
          type: FunctionDef
          code: |-
            @abstractmethod
            def empty_tree(self, path: Path | None=None) -> Tree:
                """Return an empty tree, used when appending to a not-yet-existing file."""
        - id: Engine.serialize
          type: FunctionDef
          code: |-
            @abstractmethod
            def serialize(self, tree: Tree) -> str:
                """Render ``tree`` back to source text for writing to disk."""
        - id: Engine.validate
          type: FunctionDef
          code: |-
            @abstractmethod
            def validate(self, source: str) -> str | None:
                """Return an error message if ``source`` is malformed, else ``None``."""
        - id: Engine.locate_all
          type: FunctionDef
          code: |-
            @abstractmethod
            def locate_all(self, tree: Tree) -> list[Located]:
                """Flatten ``tree`` into every addressable node, in document order."""
        - id: Engine.is_definition
          type: FunctionDef
          code: |-
            def is_definition(self, node_type: str) -> bool:
                """Whether ``node_type`` is "def-like" enough for a ``signature`` to make
                    sense. Defaults to ``True`` so every node falls back to a signature
                    (via ``signature``'s own trimmed-content fallback) unless an engine
                    narrows this down; engines with a precise, known node-type set (see
                    :class:`xy.ai.mcpc.tools.ast.generic._java.JavaEngine`) should override
                    this to exclude statements/imports/etc. where a signature makes no sense."""
                return True
        - id: Engine.signature
          type: FunctionDef
          code: |-
            @abstractmethod
            def signature(self, node: Any) -> str:
                """One-line rendering of ``node``'s header (or the node itself)."""
        - id: Engine.default_signature
          type: FunctionDef
          code: |-
            def default_signature(self, node: Any, limit: int=80) -> str:
                """Fallback used when ``is_definition`` is False: the first non-blank
                line of the node's source, trimmed to ``limit`` chars."""
                first_line = next((line.strip() for line in self.node_code(node).splitlines() if line.strip()), '')
                return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'
        - id: Engine.docstring
          type: FunctionDef
          code: |-
            @abstractmethod
            def docstring(self, node: Any) -> str | None:
                """Short docstring of ``node``, if the format has such a concept."""
        - id: Engine.node_code
          type: FunctionDef
          code: |-
            @abstractmethod
            def node_code(self, node: Any) -> str:
                """Full source of a single ``node``."""
        - id: Engine.replace
          type: FunctionDef
          code: |-
            @abstractmethod
            def replace(self, loc: Located, code: str) -> None:
                """Replace ``loc``'s node with ``code``."""
        - id: Engine.insert
          type: FunctionDef
          code: |-
            @abstractmethod
            def insert(self, loc: Located, code: str, position: str) -> int:
                """Insert ``code`` ``"before"``/``"after"`` ``loc``; return units inserted."""
        - id: Engine.delete
          type: FunctionDef
          code: |-
            @abstractmethod
            def delete(self, loc: Located) -> None:
                """Delete ``loc``'s node from its container."""
        - id: Engine.append
          type: FunctionDef
          code: |-
            @abstractmethod
            def append(self, tree: Tree, code: str) -> int:
                """Append ``code`` at ``tree``'s top level; return units appended."""
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
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
                  return generic.fallback_engine()
              return generic.get_engine(symbol)
      - id: AstCache
        type: ClassDef
        signature: "class AstCache:"
        docstring: Content-hash validated cache of parsed trees keyed by absolute path.
        children:
        - id: AstCache.AzRmHX|ayni5v
          type: statements
          code: '''Content-hash validated cache of parsed trees keyed by absolute path.'''
        - id: AstCache.init
          type: FunctionDef
          code: |-
            def __init__(self) -> None:
                self._entries: dict[str, _CacheEntry] = {}
                self._lock = threading.RLock()
        - id: AstCache.get_tree
          type: FunctionDef
          code: |-
            def get_tree(self, path: Path) -> Tree:
                key = str(path)
                engine = engine_for_path(path)
                with self._lock:
                    entry = self._entries.get(key)
                    mtime_ns = path.stat().st_mtime_ns
                    if entry is not None and entry.mtime_ns == mtime_ns:
                        return entry.tree
                    source = path.read_text(encoding='utf-8')
                    digest = hashlib.sha256(source.encode('utf-8')).hexdigest()
                    if entry is not None and entry.content_hash == digest:
                        entry.mtime_ns = mtime_ns
                        return entry.tree
                    tree = engine.parse(source, path)
                    self._entries[key] = _CacheEntry(mtime_ns, digest, tree)
                    return tree
        - id: AstCache.save
          type: FunctionDef
          code: |-
            def save(self, path: Path, tree: Tree) -> str:
                """Serialise *tree*, write it to *path* and refresh the cache entry."""
                source = tree.engine.serialize(tree)
                path.write_text(source, encoding='utf-8')
                '# Re-parse so cached positions match the file exactly.'
                normalized = tree.engine.parse(source, path)
                digest = hashlib.sha256(source.encode('utf-8')).hexdigest()
                with self._lock:
                    self._entries[str(path)] = _CacheEntry(path.stat().st_mtime_ns, digest, normalized)
                return source
        - id: AstCache.invalidate
          type: FunctionDef
          code: |-
            def invalidate(self, path: Path) -> None:
                with self._lock:
                    self._entries.pop(str(path), None)
      - id: load
        type: FunctionDef
        code: |-
          def load(path_str: str) -> tuple[Path, Tree]:
              """Resolve *path_str* and return it together with its cached tree."""
              path = require_path(path_str)
              return (path, CACHE.get_tree(path))
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
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: ReplaceItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceItem:
              """One node replacement to apply.

              Attributes:
                  path: Absolute path to the file to modify.
                  source: Replacement source.
                  id: Unique id of the target node.
              """
              path: str
              source: str
              id: str | None = None
      - id: ReplaceResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceResult:
              """Result of a single node replacement.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  new_id: The node's new id, only set if the replacement changed it.
              """
              path: str
              id: str | None
              result: str
              new_id: str | None = None
      - id: replace_one
        type: FunctionDef
        code: |-
          def _replace_one(item: ReplaceItem) -> ReplaceResult:
              file_path = core.require_path(item.path)
              tree = core.CACHE.get_tree(file_path)
              target = select_by_path(tree, id=item.id)
              new_id = core.replace_node(target, item.source)
              core.CACHE.save(file_path, tree)
              return ReplaceResult(path=item.path, id=item.id, result='success', new_id=new_id)
      - id: ast_replace
        type: FunctionDef
        code: |-
          def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:
              """Replace one or more selected nodes with new source.

              ``id`` and replacement ``source`` stay together per item, since replacing
              the same node twice makes no sense; several items may target the same or
              different files.

              Args:
                  items: Node replacements to apply. Must be non-empty.

              Returns:
                  ReplaceBatchResult: One result per successful replacement, one error per
                  failed replacement.

              Raises:
                  core.AstError: If ``items`` is empty.
              """
              if not items:
                  raise core.AstError("'items' must be a non-empty list.")
              results: list[ReplaceResult] = []
              errors: list[ReplaceError] = []
              for item in items:
                  try:
                      results.append(_replace_one(item))
                  except core.AstError as exc:
                      errors.append(ReplaceError(path=item.path, id=item.id, error=str(exc)))
              return ReplaceBatchResult(results=results, errors=errors)
      - id: ReplaceNodeTool
        type: ClassDef
        signature: "class ReplaceNodeTool(ToolDefinition):"
        children:
        - id: ReplaceNodeTool.jKVWKu|S7XtH2
          type: statements
          code: |-
            name = 'ast_replace'
            title = 'Replace the complete source of the AST nodes identified by the given IDs with new content.'
            description = 'Replace the source or text of selected AST nodes in a batch; operations may target the same or different files. Returns the updated IDs.'
            _ITEM_PROPERTIES = {
                'path': PATH_PROP,
                'source': {
                    'type': 'string',
                    'description': 'Replacement source.'},
                **PATH_SELECTOR_PROPS}
            _ITEM_REQUIRED = ['path', 'source', 'id']
        - id: ReplaceNodeTool.hKJ2Rq|SrCCxe
          type: statements
          code: |-
            _ITEMS_DESCRIPTION = 'Node replacements to apply.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: ReplaceNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."""

                def item_factory(it: dict[str, Any]) -> ReplaceItem:
                    return ReplaceItem(path=it['path'], source=it['source'], id=it.get('id'))

                def result_serializer(r: ReplaceResult) -> dict[str, Any]:
                    entry = {'path': r.path, 'result': r.result}
                    if r.id is not None:
                        entry['id'] = r.id
                    if r.new_id is not None:
                        entry['new_id'] = r.new_id
                    return entry

                def error_serializer(e: ReplaceError) -> dict[str, Any]:
                    entry = {'path': e.path, 'error': e.error}
                    if e.id is not None:
                        entry['id'] = e.id
                    return entry
                return handle_batch_tool(
                    ctx,
                    item_factory,
                    ast_replace,
                    core.AstError,
                    result_serializer,
                    error_serializer,
                    auto_approve=True)
      - id: register
        type: FunctionDef
        code: |-
          def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(ReplaceNodeTool())
              functions.register(ast_replace)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
      nodes:
      - id: clnjIr|Jz72IC
        type: statements
        code: |-
          SELECTOR_PROPS = {
              'id': {
                  'type': 'string',
                  'description': 'Unique node id'},
              'name': {
                  'type': 'string',
                  'description': 'Simple node name.'},
              'node_type': {
                  'type': 'string',
                          'description': "Regular expression (case-insensitive) a node type name must match, e.g. 'FunctionDef' or 'pair'."},
              'lineno': {
                  'type': 'integer',
                  'description': 'Line in the target node.'},
              'end_lineno': {
                  'type': 'integer',
                  'description': 'End line of a range to get all nodes touching the lines.'},
              'parent_type': {
                  'type': 'string',
                  'description': 'Node type name of the container.'}}
      - id: BdeVO2|NSUKUv
        type: statements
        code: |-
          '#: Path-only selectors used by every mutation tool (replace/insert/delete/edit_*).'
          PATH_SELECTOR_PROPS = {'id': SELECTOR_PROPS['id']}
          "#: Absolute-path property shared by every batch tool's item schema."
          PATH_PROP = {'type': 'string', 'description': 'Absolute path to the file (never directories).'}
```
/answer 0a99-56-55 allow
Control Request:
```yaml
id: 0a99-57-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: def (handle_batch_tool|batch_schema)
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    intent: editing
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: ^(from|import)
    include:
    - replace.py
    - insert.py
    - create.py
    exclude:
    - '**/__pycache__/**'
  reason: Fundstelle von batch_schema/handle_batch_tool und die Imports der Tools finden
```
/answer 0a99-57-0 allow
Control Request:
```yaml
id: 0a99-58-57
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: _tool_helpers.py
        nodes:
        - id: batch_schema
          matches:
          - "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
        - id: handle_batch_tool
          matches:
          - "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_ser…: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None, auto_approve: bool=False) -> ToolResult:"
      error: 1 match line(s) exceeded 300 characters and were shortened in the middle with an ellipsis (150 leading and trailing characters kept).
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: replace.py
        matches:
        - from dataclasses import dataclass
        - from typing import Any
        - "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult"
        - from xy.ai.mcpc.tools.tool_context import ToolContext
        - from xy.ai.mcpc.tools.ast import core
        - "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
        - from xy.ai.mcpc.tools.function_registry import FunctionRegistry
        - from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
      - path: create.py
        matches:
        - from dataclasses import dataclass
        - from typing import Any
        - "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult"
        - from xy.ai.mcpc.tools.tool_context import ToolContext
        - from xy.ai.mcpc.tools.ast import core
        - from xy.ai.mcpc.tools._tool_helpers import batch_schema
      warning: "Limit of 15 matches reached (total 26); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 0a99-58-57 allow
ReasoningToken: 122
Control Request:
```yaml
id: 0a99-59-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    ids:
    - InsertItem
    - InsertResult
    - insert_one
    - InsertNodeTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    ids:
    - EditBlockItem
    - EditBlockResult
    - node_guard
    - edit_block_one
    - EditBlockNodeTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    ids:
    - EditMarksItem
    - EditMarksResult
    - edit_marks_one
    - EditMarksNodeTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    ids:
    - DeleteItem
    - DeleteResult
    - delete_one
    - DeleteTool
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
    ids:
    - CreateFileItem
    - CreateFileResult
    - create_one
    - CreateFileTool
  reason: "Alle Mutations-Tools im Detail lesen, um gemeinsamen Einhängepunkt für Import-Handling zu finden"
```
/answer 0a99-59-0 allow
Control Request:
```yaml
id: 0a99-5a-59
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      nodes:
      - id: InsertItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class InsertItem:
              """One insert operation.

              Attributes:
                  path: Absolute path to the file to modify.
                  source: Source of the statement(s) to insert.
                  position: ``"before"`` or ``"after"`` the selected node. Defaults to ``"after"``.
                  id: Unique id of the target node.
              """
              path: str
              source: str
              position: str = 'after'
              id: str | None = None
      - id: InsertResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class InsertResult:
              """Result of a single insert operation.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  inserted: Number of top-level statements parsed from ``source`` and inserted.
                  ids: The newly inserted top-level node(s)' ids.
              """
              path: str
              id: str | None
              result: str
              inserted: int
              ids: list[str] | None = None
      - id: insert_one
        type: FunctionDef
        code: |-
          def _insert_one(item: InsertItem) -> InsertResult:
              file_path = core.require_path(item.path)
              tree = core.CACHE.get_tree(file_path)
              target = select_by_path(tree, id=item.id)
              before_ids = {loc.node_id for loc in core.locate_all(tree)}
              inserted = core.insert_node(target, item.source, item.position)
              new_ids = [loc.node_id for loc in core.locate_all(tree) if loc.node_id not in before_ids]
              core.CACHE.save(file_path, tree)
              return InsertResult(path=item.path, id=item.id, result='success', inserted=inserted, ids=new_ids or None)
      - id: InsertNodeTool
        type: ClassDef
        signature: "class InsertNodeTool(ToolDefinition):"
        children:
        - id: InsertNodeTool.T43CMn|tx9Kq0
          type: statements
          code: |-
            name = 'ast_insert'
            title = 'Insert AST nodes'
            description = "Insert source relative to selected nodes ('before' or 'after'), for a batch of items; several operations may target the same or different files. Returns changed IDs in the result."
        - id: InsertNodeTool.c0C4qn|tz50Lf
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'path': PATH_PROP,
                'source': {
                    'type': 'string',
                    'description': 'Source to insert.'},
                'position': {
                    'type': 'string',
                    'enum': [
                            'before',
                            'after'],
                    'description': 'Placement relative to the selected node.',
                    'default': 'after'},
                **PATH_SELECTOR_PROPS}
            _ITEM_REQUIRED = ['path', 'source']
            _ITEMS_DESCRIPTION = 'Insert operations to apply.'
        - id: InsertNodeTool.ei5nIo|Lu9IT0
          type: statements
          code: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - id: InsertNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""

                def item_factory(it: dict[str, Any]) -> InsertItem:
                    return InsertItem(path=it['path'], source=it['source'], position=it.get('position', 'after'), id=it.get('id'))

                def result_serializer(r: InsertResult) -> dict[str, Any]:
                    entry = {'path': r.path, 'result': r.result, 'inserted': r.inserted}
                    if r.id is not None:
                        entry['id'] = r.id
                    if r.ids is not None:
                        entry['ids'] = r.ids
                    return entry

                def error_serializer(e: InsertError) -> dict[str, Any]:
                    entry = {'path': e.path, 'error': e.error}
                    if e.id is not None:
                        entry['id'] = e.id
                    return entry
                return handle_batch_tool(
                    ctx,
                    item_factory,
                    ast_insert,
                    core.AstError,
                    result_serializer,
                    error_serializer,
                    auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: EditBlockItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockItem:
              """One block edit to apply.

              Attributes:
                  path: Absolute path to the file to modify.
                  old_text: Unique 10-100 char block to find within the node's source (unless ``replace_all``).
                  new_text: Replacement text (may be empty to delete the block).
                  exact: If False (default), whitespace in ``old_text`` is matched tolerantly.
                  replace_all: If True, replace every occurrence instead of requiring a single match.
                  id: Unique id of the target node. If omitted, the node is searched for by ``old_text``.
              """
              path: str
              old_text: str
              new_text: str
              exact: bool = False
              replace_all: bool = False
              id: str | None = None
      - id: EditBlockResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditBlockResult:
              """Result of applying a single block edit.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  new_id: The node's new id, only set if the edit changed it.
              """
              path: str
              id: str | None
              result: str
              new_id: str | None = None
      - id: node_guard
        type: FunctionDef
        code: |-
          def _node_guard(engine, reference):
              """Guard for tolerant node edits.

              Engines that flag malformed edits on replace need no extra check. Others
              (e.g. markup grammars) must not merge lines and must still re-parse cleanly.
              """
              if engine.validates_syntax:
                  return None
              keep_lines = line_preserving(reference)
              return lambda span, result: keep_lines(span, result) and engine.validate(result) is None
      - id: edit_block_one
        type: FunctionDef
        code: |-
          def _edit_block_one(item: EditBlockItem) -> EditBlockResult:
              file_path = core.require_path(item.path)
              tree = core.CACHE.get_tree(file_path)
              target = select_by_text(tree, [item.old_text], id=item.id)
              node_source = core.edit_node_source(target)
              try:
                  new_source = replace_in_block(
                      node_source,
                      item.old_text,
                      item.new_text,
                      exact=item.exact,
                      replace_all=item.replace_all,
                      accept=_node_guard(
                          tree.engine,
                          item.old_text),
                      max_level=3 if tree.engine.validates_syntax else 2,
                      where='node')
              except TextMatchError as exc:
                  raise core.AstTextError(
                      str(exc), reason=getattr(
                          exc, 'reason', None), position=getattr(
                              exc, 'position', None), corrected_text=getattr(
                                  exc, 'corrected_text', None), guess=getattr(
                                      exc, 'guess', None), next_step=getattr(
                                          exc, 'next_step', None)) from exc
              new_id = core.replace_node(target, new_source)
              core.CACHE.save(file_path, tree)
              return EditBlockResult(path=item.path, id=item.id, result='success', new_id=new_id)
      - id: EditBlockNodeTool
        type: ClassDef
        signature: "class EditBlockNodeTool(ToolDefinition):"
        children:
        - id: EditBlockNodeTool.h2DGqi|I7XJBX
          type: statements
          code: |-
            name = 'ast_edit_block'
            title = 'Replace short text within AST nodes when id is known'
            description = "Replace occurrence(s) of short 'old_text' with 'new_text', within nodes addressed by known id, for a batch of items. Don't use for large edits, use ast_edit_marks instead. Returns changed IDs in the result."
        - id: EditBlockNodeTool.MuT0pV|iYKmvH
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'path': PATH_PROP,
                'old_text': {
                    'type': 'string',
                    'minLength': 10,
                    'maxLength': 100,
                    'description': 'Short text (10-100 chars) to replace within the node. Must occur exactly once, or replaceAll is set.'},
                'new_text': {
                    'type': 'string',
                    'description': 'Replacement text, may be empty to remove the text.'},
                'exact': {
                    'type': 'boolean',
                            'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                            'default': False},
                'replaceAll': {
                    'type': 'boolean',
                    'description': "If true, replace every occurrence of 'old_text' within the node instead of a single unique match.",
                    'default': False},
                **PATH_SELECTOR_PROPS}
        - id: EditBlockNodeTool.ZDSH0j|Jr7UgN
          type: statements
          code: |-
            _ITEM_REQUIRED = ['path', 'old_text', 'new_text', 'id']
            _ITEMS_DESCRIPTION = 'Block edits to apply.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: EditBlockNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST API."""

                def item_factory(it: dict[str, Any]) -> EditBlockItem:
                    return EditBlockItem(
                        path=it['path'],
                        old_text=it['old_text'],
                        new_text=it['new_text'],
                        exact=it.get(
                            'exact',
                            False),
                        replace_all=it.get(
                            'replaceAll',
                            False),
                        id=it.get('id'))

                def result_serializer(r: EditBlockResult) -> dict[str, Any]:
                    entry = {'path': r.path, 'result': r.result}
                    if r.id is not None:
                        entry['id'] = r.id
                    if r.new_id is not None:
                        entry['new_id'] = r.new_id
                    return entry

                def error_serializer(e: EditBlockError) -> dict[str, Any]:
                    entry = {'path': e.path, 'error': e.error}
                    if e.id is not None:
                        entry['id'] = e.id
                    for field in ('candidates', 'reason', 'corrected_text', 'guess', 'next_step'):
                        value = getattr(e, field)
                        if value is not None:
                            entry[field] = value
                    return entry
                return handle_batch_tool(
                    ctx,
                    item_factory,
                    ast_edit_block,
                    core.AstError,
                    result_serializer,
                    error_serializer,
                    auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
      nodes:
      - id: EditMarksItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditMarksItem:
              """One marker-based edit to apply.

              Attributes:
                  path: Absolute path to the file to modify.
                  start_marker: Unique 10-30 char substring marking the beginning of the block, within the node's source.
                  end_marker: Unique 10-30 char substring marking the end of the block, within the node's source.
                  content: Replacement source for the marked block.
                  exact: If False (default), whitespace in start/end is matched tolerantly.
                  id: Unique id of the target node. If omitted, the node is searched for by the markers.
              """
              path: str
              start_marker: str
              end_marker: str
              content: str
              exact: bool = False
              id: str | None = None
      - id: EditMarksResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class EditMarksResult:
              """Result of applying a single marker-based edit.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
                  new_id: The node's new id, only set if the edit changed it.
              """
              path: str
              id: str | None
              result: str
              new_id: str | None = None
      - id: edit_marks_one
        type: FunctionDef
        code: |-
          def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:
              file_path = core.require_path(item.path)
              tree = core.CACHE.get_tree(file_path)
              target = select_by_text(tree, [item.start_marker, item.end_marker], id=item.id)
              node_source = core.edit_node_source(target)
              begin, end = (item.start_marker, item.end_marker) if item.exact else (
                  item.start_marker.strip(), item.end_marker.strip())
              try:
                  new_source = replace_between(
                      node_source,
                      begin,
                      end,
                      item.content,
                      exact=item.exact,
                      accept=_node_marks_guard(
                          tree.engine,
                          begin,
                          end),
                      max_level=3 if tree.engine.validates_syntax else 2,
                      where='node')
              except TextMatchError as exc:
                  raise core.AstTextError(
                      str(exc), reason=getattr(
                          exc, 'reason', None), position=getattr(
                              exc, 'position', None), corrected_text=getattr(
                                  exc, 'corrected_text', None), guess=getattr(
                                      exc, 'guess', None), next_step=getattr(
                                          exc, 'next_step', None)) from exc
              new_id = core.replace_node(target, new_source)
              core.CACHE.save(file_path, tree)
              return EditMarksResult(path=item.path, id=item.id, result='success', new_id=new_id)
      - id: EditMarksNodeTool
        type: ClassDef
        signature: "class EditMarksNodeTool(ToolDefinition):"
        children:
        - id: EditMarksNodeTool.nfiMrL|QgOVDW
          type: statements
          code: |-
            name = 'ast_edit_marks'
            title = 'Replace large text regions within AST nodes between markers when id is known'
            description = "Replace everything between and including the unique 'start_marker' and 'end_marker' markers, found within nodes addressed by a known id, with new 'content', for a batch of items. Returns changed IDs in the result."
        - id: EditMarksNodeTool.MuT0pV|RNoeCK
          type: statements
          code: |-
            _ITEM_PROPERTIES = {
                'path': PATH_PROP,
                'start_marker': {
                    'type': 'string',
                    'minLength': 10,
                    'maxLength': 30,
                    'description': "Unique 10-30 char substring marking the beginning of the text to replace, within the node's source."},
                'end_marker': {
                    'type': 'string',
                    'minLength': 10,
                    'maxLength': 30,
                    'description': "Unique 10-30 char substring marking the end of the text to replace, within the node's source."},
                'content': {
                    'type': 'string',
                            'description': 'Replacement source for the marked text.'},
                'exact': {
                    'type': 'boolean',
                    'description': "If true, 'start_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                    'default': False},
                **PATH_SELECTOR_PROPS}
        - id: EditMarksNodeTool.ZDSH0j|PEqhnG
          type: statements
          code: |-
            _ITEM_REQUIRED = ['path', 'start_marker', 'end_marker', 'content']
            _ITEMS_DESCRIPTION = 'Marker-based edits to apply.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: EditMarksNodeTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST API."""

                def item_factory(it: dict[str, Any]) -> EditMarksItem:
                    return EditMarksItem(
                        path=it['path'],
                        start_marker=it['start_marker'],
                        end_marker=it['end_marker'],
                        content=it['content'],
                        exact=it.get(
                            'exact',
                            False),
                        id=it.get('id'))

                def result_serializer(r: EditMarksResult) -> dict[str, Any]:
                    entry = {'path': r.path, 'result': r.result}
                    if r.id is not None:
                        entry['id'] = r.id
                    if r.new_id is not None:
                        entry['new_id'] = r.new_id
                    return entry

                def error_serializer(e: EditMarksError) -> dict[str, Any]:
                    entry = {'path': e.path, 'error': e.error}
                    if e.id is not None:
                        entry['id'] = e.id
                    for field in ('candidates', 'reason', 'position', 'corrected_text', 'guess', 'next_step'):
                        value = getattr(e, field)
                        if value is not None:
                            entry[field] = value
                    return entry
                return handle_batch_tool(
                    ctx,
                    item_factory,
                    ast_edit_marks,
                    core.AstError,
                    result_serializer,
                    error_serializer,
                    auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
      - id: DeleteItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class DeleteItem:
              """One node (or file) to delete.

              Attributes:
                  path: Absolute path to the file to modify.
                  id: Unique id of the target node. Omit to delete the whole file.
              """
              path: str
              id: str | None = None
      - id: DeleteResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class DeleteResult:
              """Result of deleting a single node or file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  result: Always ``"success"``.
              """
              path: str
              id: str | None
              result: str
      - id: delete_one
        type: FunctionDef
        code: |-
          def _delete_one(item: DeleteItem) -> DeleteResult:
              file_path = core.require_path(item.path)
              if item.id is None:
                  try:
                      file_path.unlink()
                  except OSError as exc:
                      raise core.AstError('Delete failed.') from exc
                  core.CACHE.invalidate(file_path)
                  parent = file_path.parent
                  if not any(parent.iterdir()):
                      parent.rmdir()
                  return DeleteResult(path=item.path, id=None, result='success')
              tree = core.CACHE.get_tree(file_path)
              target = select_by_path(tree, id=item.id)
              core.delete_node(target)
              core.CACHE.save(file_path, tree)
              return DeleteResult(path=item.path, id=item.id, result='success')
      - id: DeleteTool
        type: ClassDef
        signature: "class DeleteTool(ToolDefinition):"
        children:
        - id: DeleteTool.gLGwBi|LEwdCL
          type: statements
          code: |-
            name = 'ast_delete'
            title = 'Delete AST nodes or files'
            description = 'Delete selected nodes from files, or whole files if no selector is given, for a batch of items.'
            _ITEM_PROPERTIES = {'path': PATH_PROP, **PATH_SELECTOR_PROPS}
            _ITEM_REQUIRED = ['path']
            _ITEMS_DESCRIPTION = 'Nodes/files to delete.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: DeleteTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""

                def item_factory(it: dict[str, Any]) -> DeleteItem:
                    return DeleteItem(path=it['path'], id=it.get('id'))
                return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, auto_approve=True)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py
      nodes:
      - id: CreateFileItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class CreateFileItem:
              """One file to create.

              Attributes:
                  path: Absolute path of the file to create.
                  source: Source for the new file.
                  overwrite: Allow replacing an existing file. Defaults to ``False``.
              """
              path: str
              source: str
              overwrite: bool = False
      - id: CreateFileResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class CreateFileResult:
              """Result of creating a single file.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  result: Always ``"success"``.
              """
              path: str
              result: str
      - id: create_one
        type: FunctionDef
        code: |-
          def _create_one(item: CreateFileItem) -> CreateFileResult:
              file_path = core.require_path(item.path, must_exist=False)
              if file_path.exists() and (not item.overwrite):
                  raise core.AstError('File already exists.')
              tree = core.parse_for(item.path, item.source)
              file_path.parent.mkdir(parents=True, exist_ok=True)
              core.CACHE.save(file_path, tree)
              return CreateFileResult(path=item.path, result='success')
      - id: CreateFileTool
        type: ClassDef
        signature: "class CreateFileTool(ToolDefinition):"
        children:
        - id: CreateFileTool.omImBZ|s2PVZe
          type: statements
          code: |-
            name = 'ast_create'
            title = 'Create files'
            description = 'Create one or more files from source.'
            _ITEM_PROPERTIES = {
                'path': {
                    'type': 'string', 'description': 'Absolute path of the file to create.'}, 'source': {
                        'type': 'string', 'description': 'Source for the new file.'}, 'overwrite': {
                            'type': 'boolean', 'description': 'Allow replacing an existing file.', 'default': False}}
            _ITEM_REQUIRED = ['path', 'source']
            _ITEMS_DESCRIPTION = 'Files to create.'
        - id: CreateFileTool.ei5nIo|Lu9IT0
          type: statements
          code: "input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)"
        - id: CreateFileTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_create`, translating the MCP schema to/from the Python API."""

                def item_factory(it: dict[str, Any]) -> CreateFileItem:
                    return CreateFileItem(path=it['path'], source=it['source'], overwrite=it.get('overwrite', False))
                return handle_batch_tool(ctx, item_factory, ast_create, core.AstError, auto_approve=True)
```
/answer 0a99-5a-59 allow
Control Request:
```yaml
id: 0a99-5b-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  name: handle_batch_tool
  reason: "Batch-Handler lesen: wie Ergebnisse/Fehler zu ToolResult werden (Platz für generischen Hinweis)"
```
/answer 0a99-5b-0 allow
Control Request:
```yaml
id: 0a99-5c-5b
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: handle_batch_tool
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
/answer 0a99-5c-5b allow
Control Request:
```yaml
id: 0a99-5d-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  name: batch_schema
  reason: batch_schema lesen
```
/answer 0a99-5d-0 allow
Control Request:
```yaml
id: 0a99-5e-5d
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: batch_schema
        code: |-
          def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:
              """Build the standard ``{items: [...]}`` input schema shared by all batch tools.

              Neither the top level nor the items accept properties beyond the declared ones.

              Args:
                  item_properties: The ``properties`` of a single item.
                  required: The ``required`` keys of a single item.
                  description: Description of the ``items`` array.
              """
              item_schema: dict[str, Any] = {'type': 'object', 'properties': item_properties,
                                             'required': required, 'additionalProperties': False}
              return {
                  'type': 'object',
                  'properties': {
                      'items': {
                          'type': 'array',
                          'minItems': 1,
                          'items': item_schema,
                          'description': description}},
                  'required': ['items'],
                  'additionalProperties': False}
```
/answer 0a99-5e-5d allow
ReasoningToken: 225
Control Request:
```yaml
id: 0a99-5f-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    ids:
    - PythonEngine.parse
    - PythonEngine.parse_module
    - PythonEngine.parse_fragment
    - PythonEngine.empty_tree
    - PythonEngine.serialize
    - PythonEngine.locate_all
    - PythonEngine.append
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
    ids:
    - TreeSitterEngine.init
    - TreeSitterEngine.parse_1
    - TreeSitterEngine.empty_tree
    - TreeSitterEngine.serialize
    - TreeSitterEngine.splice
    - TreeSitterEngine.append
    - TreeSitterEngine.locate_all
  reason: "Python- und TreeSitter-Engine: parse/serialize/append/splice, Basis für Import-Handling"
```
/answer 0a99-5f-0 allow
Control Request:
```yaml
id: 0a99-60-5f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
      nodes:
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
      - id: PythonEngine.append
        type: FunctionDef
        code: |-
          def append(self, tree: Tree, code: str) -> int:
              check_no_control_chars(code)
              stmts = self._parse_fragment(code)
              tree.raw.body.extend(stmts)
              return len(stmts)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
      nodes:
      - id: TreeSitterEngine.init
        type: FunctionDef
        code: |-
          def __init__(self, symbol: str) -> None:
              self.symbol = symbol
              self.name = f'tree-sitter:{symbol}'
              self._parser = None
      - id: TreeSitterEngine.parse_1
        type: FunctionDef
        code: |-
          def parse(self, source: str, path: Path | None=None) -> Tree:
              return Tree(self, self._parse(source.encode('utf-8')), source, path)
      - id: TreeSitterEngine.empty_tree
        type: FunctionDef
        code: |-
          def empty_tree(self, path: Path | None=None) -> Tree:
              return Tree(self, self._parse(b''), '', path)
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
      - id: TreeSitterEngine.append
        type: FunctionDef
        code: |-
          def append(self, tree: Tree, code: str) -> int:
              check_no_control_chars(code)
              sep = '' if not tree.source or tree.source.endswith('\n') else '\n'
              self._splice(tree, len(tree.source.encode('utf-8')), len(tree.source.encode('utf-8')), sep + code)
              return 1
      - id: TreeSitterEngine.locate_all
        type: FunctionDef
        code: |-
          def locate_all(self, tree: Tree) -> list[Located]:
              return self._locate_from(tree, tree.raw.root_node, lambda child, depth: True)
```
/answer 0a99-60-5f allow
Control Request:
```yaml
id: 0a99-61-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_java.py
    ids:
    - JavaEngine.is_definition
    - JavaEngine.locate_all
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
    ids:
    - JavaScriptEngine.locate_all
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    ids:
    - RpcEngine.init
    - RpcEngine.parse
    - RpcEngine.serialize
    - RpcEngine.locate_all
    - RpcEngine.append
    - RpcEngine.insert
  reason: Wie Java/JS-Engines und RPC-Engine Imports aktuell behandeln
```
/answer 0a99-61-0 allow
Control Request:
```yaml
id: 0a99-62-61
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_java.py
      nodes:
      - id: JavaEngine.is_definition
        type: FunctionDef
        code: |-
          def is_definition(self, node_type: str) -> bool:
              return node_type in _DEF_TYPES
      - id: JavaEngine.locate_all
        type: FunctionDef
        code: |-
          def locate_all(self, tree: Tree) -> list[Located]:
              results: list[Located] = []
              source = tree.source.encode('utf-8')

              def walk(children: list[Any], container: Any, path: str) -> None:
                  used: dict[str, int] = {}
                  i, n = (0, len(children))
                  while i < n:
                      node = children[i]
                      if node.type in _DEF_TYPES:
                          name = self._name(node)
                          seg = id_segment(name, i, used)
                          nid = f'{path}.{seg}' if path else seg
                          body = _body_of(node) if node.type in _TYPE_DEF_TYPES else None
                          expandable = bool(body) and any((c.type in _DEF_TYPES for c in body.named_children))
                          results.append(
                              Located(
                                  tree=tree,
                                  node=node,
                                  parent=container,
                                  index=i,
                                  node_id=nid,
                                  node_type=node.type,
                                  name=name,
                                  lineno=node.start_point[0] + 1,
                                  end_lineno=node.end_point[0] + 1,
                                  parent_type=container.type,
                                  expandable=expandable))
                          if body is not None:
                              walk(body.named_children, body, nid)
                          i += 1
                          continue
                      start = i
                      kind = 'imports' if node.type in _IMPORT_TYPES else 'statements'
                      length = 0
                      while i < n:
                          current = children[i]
                          if current.type in _DEF_TYPES:
                              break
                          current_kind = 'imports' if current.type in _IMPORT_TYPES else 'statements'
                          if current_kind != kind:
                              break
                          piece = current.end_byte - current.start_byte
                          if i > start and length + piece > SEGMENT_MAX_CHARS:
                              break
                          length += piece
                          i += 1
                      group = _SynthNode(kind, children[start:i], source)
                      seg = id_segment(None, start, used, content=self.node_code(group))
                      nid = f'{path}.{seg}' if path else seg
                      results.append(
                          Located(
                              tree=tree,
                              node=group,
                              parent=container,
                              index=start,
                              node_id=nid,
                              node_type=kind,
                              name=None,
                              lineno=group.start_point[0] + 1,
                              end_lineno=group.end_point[0] + 1,
                              parent_type=container.type,
                              expandable=False))
              walk(tree.raw.root_node.named_children, tree.raw.root_node, '')
              return results
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
      nodes:
      - id: JavaScriptEngine.locate_all
        type: FunctionDef
        code: |-
          def locate_all(self, tree: Tree) -> list[Located]:
              results: list[Located] = []
              source = tree.source.encode('utf-8')

              def walk(children: list[Any], container: Any, path: str) -> None:
                  used: dict[str, int] = {}
                  i, n = (0, len(children))
                  while i < n:
                      node = children[i]
                      if _is_def(node):
                          name = _def_name(node)
                          seg = id_segment(name, i, used, content=self.node_code(node))
                          nid = f'{path}.{seg}' if path else seg
                          body = _def_body(node)
                          '#: Only class bodies nest further definitions (methods/fields);'
                          "#: a function/method's own statement_block is left as one leaf,"
                          "#: exactly like the Java engine never splits a method's body."
                          inner = body if body is not None and body.type == 'class_body' else None
                          expandable = bool(inner) and any((_is_def(c) for c in inner.named_children))
                          results.append(
                              Located(
                                  tree=tree,
                                  node=node,
                                  parent=container,
                                  index=i,
                                  node_id=nid,
                                  node_type=node.type,
                                  name=name,
                                  lineno=node.start_point[0] + 1,
                                  end_lineno=node.end_point[0] + 1,
                                  parent_type=container.type,
                                  expandable=expandable))
                          if inner is not None:
                              walk(inner.named_children, inner, nid)
                          i += 1
                          continue
                      start = i
                      kind = 'imports' if node.type in _IMPORT_TYPES else 'statements'
                      length = 0
                      while i < n:
                          current = children[i]
                          if _is_def(current):
                              break
                          current_kind = 'imports' if current.type in _IMPORT_TYPES else 'statements'
                          if current_kind != kind:
                              break
                          piece = current.end_byte - current.start_byte
                          if i > start and length + piece > SEGMENT_MAX_CHARS:
                              break
                          length += piece
                          i += 1
                      group = _SynthNode(kind, children[start:i], source)
                      seg = id_segment(None, start, used, content=self.node_code(group))
                      nid = f'{path}.{seg}' if path else seg
                      results.append(
                          Located(
                              tree=tree,
                              node=group,
                              parent=container,
                              index=start,
                              node_id=nid,
                              node_type=kind,
                              name=None,
                              lineno=group.start_point[0] + 1,
                              end_lineno=group.end_point[0] + 1,
                              parent_type=container.type,
                              expandable=False))
              walk(tree.raw.root_node.named_children, tree.raw.root_node, '')
              return results
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
      nodes:
      - id: RpcEngine.init
        type: FunctionDef
        code: |-
          def __init__(self, symbol: str, process: RpcProcess) -> None:
              self.symbol = symbol
              self.name = symbol
              self._process = process
              self._client: AppendInfoNodesValidateClientImpl | None = None
              '#: node_type -> is_definition, learned from the last ``locate_all`` calls'
              '#: (Engine.is_definition only takes a type name, not a specific node).'
              self._is_definition: dict[str, bool] = {}
      - id: RpcEngine.parse
        type: FunctionDef
        code: |-
          def parse(self, source: str, path: Path | None=None) -> Tree:
              error = self.validate(source)
              if error:
                  raise AstError(error)
              return Tree(self, None, source, path)
      - id: RpcEngine.serialize
        type: FunctionDef
        code: |-
          def serialize(self, tree: Tree) -> str:
              return tree.source
      - id: RpcEngine.locate_all
        type: FunctionDef
        code: |-
          def locate_all(self, tree: Tree) -> list[Located]:
              request = LocateRequest()
              request.set_Source(tree.source)
              if tree.path is not None:
                  request.set_Path(str(tree.path))
              request.set_IncludeCode(False)
              payload = self._unwrap(self._rpc().listNodes(request))
              nodes = payload.get_Nodes() if payload is not None else None
              count = len(nodes) if nodes is not None else 0
              results: list[Located] = []
              sibling_index: dict[str, int] = {}
              for i in range(count):
                  node = nodes.get(i)
                  node_id = node.get_Id()
                  node_type = node.get_Type()
                  self._is_definition[node_type] = bool(node.get_IsDefinition())
                  parent_id = node_id.rsplit('.', 1)[0] if '.' in node_id else ''
                  index = sibling_index.get(parent_id, 0)
                  sibling_index[parent_id] = index + 1
                  lineno = node.get_Lineno()
                  end_lineno = node.get_EndLineno()
                  results.append(
                      Located(
                          tree=tree,
                          node=_RpcNode(
                              tree=tree,
                              lineno=lineno,
                              end_lineno=end_lineno,
                              signature_text=node.get_Signature(),
                              docstring_text=node.get_Docstring(),
                              code=node.get_Code()),
                          parent=None,
                          index=index,
                          node_id=node_id,
                          node_type=node_type,
                          name=node.get_Name(),
                          lineno=lineno,
                          end_lineno=end_lineno,
                          parent_type=node.get_ParentType(),
                          expandable=bool(
                              node.get_Expandable())))
              return results
      - id: RpcEngine.append
        type: FunctionDef
        code: |-
          def append(self, tree: Tree, code: str) -> int:
              check_no_control_chars(code)
              request = CodeRequest()
              request.set_Source(tree.source)
              if tree.path is not None:
                  request.set_Path(str(tree.path))
              request.set_Code(code)
              payload = self._unwrap(self._rpc().appendTopLevel(request))
              tree.source = payload.get_Source()
              return payload.get_UnitsAppended()
      - id: RpcEngine.insert
        type: FunctionDef
        code: |-
          def insert(self, loc: Located, code: str, position: str) -> int:
              check_no_control_chars(code)
              request = InsertRequest()
              request.set_Source(loc.tree.source)
              if loc.tree.path is not None:
                  request.set_Path(str(loc.tree.path))
              request.set_Code(code)
              request.set_Position(PositionEnum.BEFORE if position == 'before' else PositionEnum.AFTER)
              payload = self._unwrap(self._rpc().insertRelativeToNode(loc.node_id, request))
              loc.tree.source = payload.get_Source()
              return payload.get_UnitsInserted()
```
/answer 0a99-62-61 allow
ReasoningToken: 123
Control Request:
```yaml
id: 0a99-63-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic
    pattern: import_(declaration|statement|from)|use_declaration|_IMPORT_TYPES|preproc_include
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: (?i)"?(imports?|use_declaration)"?\b.*(rpc|operationId|paths)
    include:
    - '*.yaml'
    - '*.yml'
    - '*.json'
    exclude:
    - '**/node_modules/**'
    - '**/target/**'
    - '**/build/**'
    - '**/.git/**'
    limit: 10
  reason: "Vorhandene Import-Erkennung finden und prüfen, ob die RPC-API (OpenAPI) Import-Operationen kennt"
```
/answer 0a99-63-0 deny Es gibt keine gesonderte Import Behandlung biser und die API kennt auch keine. Das sollte allerdings kein Problem sein. Die Sprachspezifik kann in MCPC rein auf Basis der AST Struktur implementiert werden. Die Sprachspezifischen unterstützungen bietet allesamt kein compile sondern nur Strukturverifizierung.
ReasoningToken: 205
Control Request:
```yaml
id: 0a99-64-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: _IMPORT_TYPES\s*=|_DEF_TYPES\s*=|SEGMENT_MAX_CHARS\s*=|def language_for_extension|def fallback_engine|def get_engine
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    intent: editing
  reason: Definitionen der Import-Typen je Engine und Engine-Registry im ast-Paket finden
```
/answer 0a99-64-0 allow
Control Request:
```yaml
id: 0a99-65-64
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: base.py
        nodes:
        - id: 1vtTBH|EXC8hj
          matches:
          - SEGMENT_MAX_CHARS = 500
      - path: generic/_css.py
        nodes:
        - id: DN45Um|IQPNWM
          matches:
          - _CSS_SEGMENT_MAX_CHARS = 1500
          - "_DEF_TYPES = {'rule_set', 'media_statement', 'supports_statement', 'keyframes_statement', 'at_rule', 'keyframe_block'}"
        - id: U4anTw|kT0bcM
          matches:
          - "_IMPORT_TYPES = {'import_statement', 'charset_statement', 'namespace_statement'}"
      - path: generic/__init__.py
        nodes:
        - id: language_for_extension
          matches:
          - "def language_for_extension(ext: str) -> str | None:"
        - id: get_engine
          matches:
          - "def get_engine(symbol: str) -> Engine:"
        - id: fallback_engine
          matches:
          - "def fallback_engine() -> PlainTextEngine:"
      - path: generic/_java.py
        nodes:
        - id: sXmiXg|B2wymS
          matches:
          - "_TYPE_DEF_TYPES = {"
          - "_DEF_TYPES = _TYPE_DEF_TYPES | {'method_declaration', 'constructor_declaration'}"
          - "_IMPORT_TYPES = {'package_declaration', 'import_declaration'}"
      - path: generic/_javascript.py
        nodes:
        - id: y0nJI5|6hMrNB
          matches:
          - "_CORE_DEF_TYPES = {'function_declaration', 'generator_function_declaration', 'class_declaration', 'method_definition'}"
        - id: HiOEEk|lU1avj
          matches:
          - "_IMPORT_TYPES = {'import_statement'}"
      - path: python/_nodes.py
        nodes:
        - id: Gi15Uw|DC4Lj2
          matches:
          - "_DEF_TYPES = (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)"
          - "_IMPORT_TYPES = (ast.Import, ast.ImportFrom)"
```
/answer 0a99-65-64 allow
ReasoningToken: 190
Control Request:
```yaml
id: 0a99-66-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
  reason: "Sprach-/Engine-Registry: welche Sprachen/Engines existieren und wie sie gewählt werden"
```
/answer 0a99-66-0 allow
Control Request:
```yaml
id: 0a99-67-66
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
      nodes:
      - id: Nna9mi|guMKQx
        signature: '"Generic tree-sitter back-ends for every non-Python language/format.\n\nSplit i…'
      - id: LvTUnt|ti8sGW
        signature: from __future__ import annotations
      - id: 95AfuA|pyxV6G
        signature: from xy.ai.mcpc.tools.ast.rpc._java import JavaEngine
      - id: 9C41E7|yvHPZT
        signature: "__all__ = ["
      - id: acL0qu|XdqYqL
        signature: "EXT_LANGUAGE = {"
      - id: language_for_extension
        signature: "def language_for_extension(ext: str) -> str | None:"
      - id: FnS4Dz|ZBavT9
        signature: "'#: Language symbol -> dedicated Engine subclass; anything absent here falls'"
      - id: jxTk2k|zmn29i
        signature: "_ENGINE_CLASSES: dict[str,"
      - id: Ygq4pQ|qTOATu
        signature: "_ENGINES: dict[str, Engine] = {}"
      - id: get_engine
        signature: "def get_engine(symbol: str) -> Engine:"
      - id: 5P6pDx|byGZAx
        signature: _FALLBACK_ENGINE = PlainTextEngine()
      - id: fallback_engine
        signature: "def fallback_engine() -> PlainTextEngine:"
        docstring: Whole-file engine used when no tree-sitter grammar claims the extension.
```
/answer 0a99-67-66 allow
Control Request:
```yaml
id: 0a99-68-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
    ids:
    - acL0qu|XdqYqL
    - jxTk2k|zmn29i
  reason: Extension→Sprache-Mapping und Engine-Klassen-Zuordnung lesen
```
/answer 0a99-68-0 allow
Control Request:
```yaml
id: 0a99-69-68
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/__init__.py
      nodes:
      - id: acL0qu|XdqYqL
        type: statements
        code: |-
          EXT_LANGUAGE = {
              '.json': 'json',
              '.jsonl': 'json',
              '.yaml': 'yaml',
              '.yml': 'yaml',
              '.toml': 'toml',
              '.xml': 'xml',
              '.html': 'html',
              '.htm': 'html',
              '.css': 'css',
              '.scss': 'css',
              '.md': 'markdown',
              '.markdown': 'markdown',
              '.js': 'javascript',
              '.jsx': 'javascript',
              '.mjs': 'javascript',
              '.cjs': 'javascript',
              '.ts': 'typescript',
              '.tsx': 'tsx',
              '.go': 'go',
              '.rs': 'rust',
              '.java': 'java',
              '.c': 'c',
              '.h': 'c',
              '.cpp': 'cpp',
              '.cc': 'cpp',
              '.cxx': 'cpp',
              '.hpp': 'cpp',
              '.hh': 'cpp',
              '.cs': 'csharp',
              '.rb': 'ruby',
              '.php': 'php',
              '.sh': 'bash',
              '.bash': 'bash',
              '.sql': 'sql',
              '.lua': 'lua',
              '.scala': 'scala',
              '.kt': 'kotlin',
              '.hs': 'haskell',
              '.ex': 'elixir',
              '.exs': 'elixir',
              '.dockerfile': 'dockerfile'}
      - id: jxTk2k|zmn29i
        type: statements
        code: |-
          _ENGINE_CLASSES: dict[str,
                                type[Engine]] = {'markdown': MarkdownEngine,
                                                 'java': JavaEngine,
                                                 'rust': RustEngine,
                                                 'yaml': YamlEngine,
                                                 'css': CssEngine,
                                                 'html': HtmlEngine,
                                                 'javascript': JavaScriptEngine,
                                                 'xml': XmlEngine}
```
/answer 0a99-69-68 allow
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 75637, in: 22, out: 14205, reason: 4668, read: 399796, write: 61410, start: 1794263344085, end: 1794263406514, time: 62188, total: 106716, io: 0, control: 241, sent: 0, recv: 699
ReasoningToken: 4668
Ich habe noch nichts geändert. Das hier ist die Analyse und mein Lösungsvorschlag.

## Stand

- **Gemeinsames Muster der Tools:** `replace`, `insert`, `delete`, `edit_block` und `edit_marks` laden den Tree über `core.CACHE.get_tree`, mutieren ihn und rufen dann `core.CACHE.save` auf. Jedes Tool hat eigene `Item`/`Result`/`Error`-Dataclasses, ein `_ITEM_PROPERTIES`-Schema und einen `handle()` mit `item_factory`. Gemeinsam sind `batch_schema` und `handle_batch_tool` in `_tool_helpers.py` sowie `PATH_SELECTOR_PROPS` in `common.py`. Das sind die natürlichen Einhängepunkte.
- **Drei Engine-Typen:**
  - Python arbeitet auf `ast.Module` (`tree.raw.body`) und serialisiert per `unparse`.
  - Tree-sitter-Engines arbeiten auf `tree.source` und parsen nach jedem `_splice` neu.
  - RPC-Engines (`.java` und `.rs`) haben `tree.raw = None`. Sie arbeiten nur mit `tree.source` und `locate_all`/`insert`/`append` über den Prozess.
- **Import-Erkennung gibt es schon, aber nur für die Outline:**
  - `locate_all` bildet bereits `imports`-Gruppen. Python nutzt `_IMPORT_TYPES = (ast.Import, ast.ImportFrom)`.
  - Tree-sitter-Java nutzt `{package_declaration, import_declaration}`.
  - JS nutzt `{import_statement}` und CSS `{import_statement, charset_statement, namespace_statement}`.
  - Es gibt aber keine Operation, die Imports gezielt hinzufügt oder entfernt.
- **`generic/_java.py` wirkt ungenutzt:** `generic/__init__.py` importiert `JavaEngine` aus `rpc/_java`, und `.java` läuft damit über JavaParser. Das habe ich nur aus der Registry gelesen und nicht weiter geprüft.

## Lösungsvorschlag

**1. Neues Modul `ast/imports.py` plus drei Hooks in `Engine` (`base.py`)**

Default (generisch, keine Sprachunterstützung):
- `import_support = False`.
- `apply_imports(tree, add, remove) -> ImportOutcome` prüft nur, ob der whitespace-normalisierte Text als Zeile in `tree.source` vorkommt.
- Vorhanden zählt als `present`. Fehlend landet in `unchecked` und wird nicht eingefügt.
- Für `remove` gilt: eine Zeile, die exakt passt, wird entfernt. Sonst gibt es keine Aktion.

`ImportOutcome` hat die Listen `added`, `removed`, `present`, `absent` und `unchecked`.

**2. Sprachspezifisch rein auf AST-Struktur, ohne Compile**

Dein Hinweis passt dazu: Die RPC-API braucht dafür nichts Neues.

- **Python:**
  - Der Import-String wird mit `_parse_fragment` geparst und muss aus `Import`/`ImportFrom` bestehen.
  - Der Vergleich ist semantisch über Modul, Level, Name und `asname`.
  - Einfügepunkt ist hinter dem letzten Top-Level-Import. Ohne Imports kommt er hinter Docstring und `from __future__`.
  - Beim Entfernen wird nur der Alias gelöscht. Bleibt das Statement leer, wird es ganz entfernt.
- **Tree-sitter (JS, TS, CSS, später Go, C und C++, C# und weitere):**
  - Eine tabellengetriebene Basis, `TreeSitterImports`, nutzt die bereits vorhandenen `_IMPORT_TYPES` plus eine `normalize_import`-Funktion pro Sprache.
  - Der Import-String wird als Snippet geparst. Die Top-Level-Typen müssen in `_IMPORT_TYPES` liegen, sonst gibt es einen Fehler.
  - Einfügen: neue Zeile hinter dem Ende des letzten Imports per `_splice`. Ohne Imports gilt die Sprachregel, bei Java etwa hinter `package`.
  - Entfernen: die Zeilen des passenden Import-Knotens inklusive Zeilenumbruch.
- **RPC (Java und Rust):**
  - `raw` ist `None`, deshalb arbeite ich über `locate_all` (Zeilenbereiche und `code` der Import-Knoten) und `tree.source`. Einfügen läuft über `insert(loc, …, 'after')`.
  - Ich habe noch nicht verifiziert, welche Node-Typen die Prozesse für `import`/`use` melden. Das prüfe ich zu Beginn der Umsetzung an einer echten Datei.
  - Fallback ist die zeilenbasierte Variante.
  - Rust-Vergleich: zuerst nur normalisierter Text. Das Flachklopfen von `use a::{b, c}` wäre eine spätere Stufe.

**3. Tool-Schicht**

- `common.py` bekommt `IMPORT_PROPS` mit `addImport` und `removeImport`, jeweils `array<string>`. Das wird wie `PATH_SELECTOR_PROPS` in die `_ITEM_PROPERTIES` der fünf Edit-Tools gespreadet.
- Jedes `Item` bekommt `add_import`/`remove_import` mit Default `None`. Die Werte werden im `item_factory` aus `addImport`/`removeImport` gelesen.
- In jedem `_xxx_one` gilt diese Reihenfolge:
  1. Import-Argumente vorab validieren, damit nichts halb angewendet wird.
  2. Den Node-Edit durchführen.
  3. `apply_item_imports(tree, item)` aufrufen.
  4. `CACHE.save`.
- Das Ergebnis erhält optional `imports: {added, removed}`. Das Serialisieren übernimmt ein gemeinsamer Helfer.
- Der generische Hinweis kommt nur einmal pro Batch als Feld `importNotice`. Er erscheint, wenn mindestens ein Item `unchecked` hatte. Text, sinngemäß: „Imports/Includes/Use konnten für diese Dateien nicht geprüft oder ergänzt werden – bitte selbst prüfen: …“. Dafür kommt ein optionales `notice`-Feld in die Batch-Ergebnisse und in `serialize_batch_result`.
- Das Schema beschreibt die Felder sprachneutral. Beispiele: `import os`, `import a.b.C;`, `use std::fmt;`, `#include <x>`. Der Text sagt: Es wird ein vollständiges Statement pro Eintrag angegeben. Fehlende Einträge werden hinzugefügt, vorhandene bei `removeImport` entfernt. Bei nicht unterstützten Dateitypen wird nur geprüft und gegebenenfalls ein Hinweis zurückgegeben.

## Risiken

- **IDs:** Ändern sich Imports nach dem Node-Edit, ändert sich die ID der `imports`-Gruppe (Content-Hash). Anonyme Geschwister-IDs können sich verschieben. Benannte Knoten bleiben stabil, und `new_id` ist vorher berechnet. Das sollte in der Tool-Beschreibung stehen.
- **Dirty Cache:** Schlägt etwas nach dem Node-Edit fehl, bleibt der gecachte Tree mutiert. Das Problem besteht heute schon. Ich würde in den Fehlerpfaden `CACHE.invalidate` aufrufen.
- **`ast_delete` ohne `id`:** Das löscht die ganze Datei. Dort wären Import-Felder nicht sinnvoll, also Fehler oder ignorieren.

## Entscheidungen, die ich von dir brauche

1. **`ast_create`:** Ich würde es nicht berücksichtigen, weil der `source` dort die ganze Datei ist und Imports direkt hineingehören. Sonst baue ich es mit ein. Passt das?
2. **Generisches `removeImport`:** Ich würde exakte Zeilenübereinstimmung entfernen und sonst nichts tun. Oder soll die generische Engine nur prüfen und bei `remove` nie schreiben?
3. **Reichweite:** Soll die erste Stufe Python, JS/TS, CSS und Java/Rust (RPC) abdecken, mit generischem Fallback für den Rest? Go, C/C++ und C# wären nur je ein Tabelleneintrag und könnten folgen.

Sag Bescheid, dann setze ich es in dieser Reihenfolge um: `imports.py` und Engine-Hooks, Python, Tree-sitter, RPC, dann Tools und Schema, dann Tests.
Lass mich das prüfen
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 145124, in: 24, out: 14345, reason: 0, read: 399796, write: 130755, start: 1794263661070, end: 1794263663051, time: 1932, total: 108648, io: 0, control: 49, sent: 1, recv: 39
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Klar, nimm dir die Zeit. Ich ändere nichts, bis du dich meldest.

Ich brauche von dir nur die drei offenen Entscheidungen: `ast_create`, das Verhalten von `removeImport` in der generischen Engine und die Reichweite der ersten Stufe. Wenn du außerdem etwas am Ansatz ändern willst, passe ich den Plan an.
1. ast_create ist akzeptiert, wir berücksichtigen es nicht
2. Wenn ein sicheres remove möglich ist durchführen sonst eher auf nummer sicher gehen und eine notice erzeugen
3. Im generischen Fall reicht die Basis. Wird etwas wie C++ wirklich benötigt würde ein entsprechendes Tool per RPC angebunden
Ich würde die Imports nach dem jeweiligen Batch item durchführen damit die ID noch passt
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Alles klar, ich setze es um: Python, JavaScript und die RPC-Engines für Java und Rust bekommen Sprachunterstützung, alle anderen die generische Basis. Imports werden pro Batch-Item nach dem Node-Edit angewendet. Zuerst lese ich die noch fehlenden Stellen.
Control Request:
```yaml
id: 0a99-6a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: def serialize_batch_result|def require_items|def text_content
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    intent: editing
  reason: "serialize_batch_result lokalisieren, um den Batch-Hinweis (notice) einzubauen"
```
/answer 0a99-6a-0 allow
Control Request:
```yaml
id: 0a99-6b-6a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: _tool_helpers.py
        nodes:
        - id: require_items
          matches:
          - "def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':"
        - id: serialize_batch_result
          matches:
          - "def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:"
      - path: tool_registry.py
        nodes:
        - id: text_content
          matches:
          - "def text_content(text: str) -> dict[str, Any]:"
```
/answer 0a99-6b-6a allow
ReasoningToken: 960
Control Request:
```yaml
id: 0a99-6c-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_nodes.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
  reason: Python-Knotenhelfer (_StatementGroup) und Paket-Exporte (core) ansehen
```
/answer 0a99-6c-0 allow
Control Request:
```yaml
id: 0a99-6d-6c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_nodes.py
      nodes:
      - id: uRRD8B|kMDygS
        signature: "'Node classification, formatting and statement-grouping helpers for the Python …"
      - id: gvaAvH|JWflmW
        signature: from __future__ import annotations
      - id: Gi15Uw|DC4Lj2
        signature: "_DEF_TYPES = (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)"
      - id: import_names
        signature: "def import_names(node: ast.Import | ast.ImportFrom) -> str:"
        docstring: "Return a compact, canonical description of an import statement."
      - id: only_defs
        signature: "def _only_defs(body: list[ast.stmt]) -> bool:"
      - id: is_expandable
        signature: "def _is_expandable(node: ast.FunctionDef | ast.AsyncFunctionDef | ast.ClassDef) -> bool:"
        docstring: Whether ``read``/``list`` should descend into ``node`` instead of returning its…
      - id: decorators
        signature: "def _decorators(node: ast.FunctionDef | ast.AsyncFunctionDef | ast.ClassDef) -> str:"
      - id: StatementGroup
        signature: "@dataclass class _StatementGroup:"
        docstring: A run of consecutive same-kind statements addressed as a single node. Individua…
        children:
        - id: StatementGroup.Rt1Igo|4X3jer
          signature: '''A run of consecutive same-kind statements addressed as a single node.\n\n    I…'
        - id: StatementGroup.stmts
          signature: "@property def stmts(self) -> list[ast.stmt]:"
        - id: StatementGroup.lineno
          signature: "@property def lineno(self) -> int:"
        - id: StatementGroup.end_lineno
          signature: "@property def end_lineno(self) -> int:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py
      nodes:
      - id: 12csGL|fKey3N
        signature: '''``ast_*`` tool family built on the standard-library ``ast`` module.\n\nA conte…'
      - id: ukjPdF|1qq8iY
        signature: from xy.ai.mcpc.tools.tool_registry import ToolRegistry
      - id: mADxqb|1vgpZd
        signature: "__all__ = ['register_ast_tools', 'ALIAS']"
      - id: register_ast_tools
        signature: "def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
        docstring: Register every ``ast_*`` tool and the ``ast`` alias.
```
/answer 0a99-6d-6c allow
Control Request:
```yaml
id: 0a99-6e-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    ids:
    - serialize_batch_result
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    ids:
    - PythonEngine.insert
    - PythonEngine.delete
    - PythonEngine.validate
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    ids:
    - RpcEngine.delete
    - RpcEngine.node_code
    - RpcEngine.validate
    - RpcEngine.replace
  reason: "Serializer, Python-insert/delete und RPC-delete/replace lesen, um Import-Operationen passend zu bauen"
```
/answer 0a99-6e-0 allow
Control Request:
```yaml
id: 0a99-6f-6e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: serialize_batch_result
        type: FunctionDef
        code: |-
          def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:
              """Serialize a batch result, only including non-empty result/error lists.

              Args:
                  batch_result: Object with optional `results` and `errors` attributes.
                  result_serializer: Optional custom serializer for result items (default: asdict).
                  error_serializer: Optional custom serializer for error items (default: asdict).

              Returns:
                  Dict with only populated fields.
              """
              if result_serializer is None:
                  result_serializer = lambda r: _compact(asdict(r)) if hasattr(r, '__dataclass_fields__') else r
              if error_serializer is None:
                  error_serializer = lambda e: _compact(asdict(e)) if hasattr(e, '__dataclass_fields__') else e
              content: dict[str, Any] = {}
              if hasattr(batch_result, 'results') and batch_result.results:
                  content['results'] = [result_serializer(r) for r in batch_result.results]
              if hasattr(batch_result, 'errors') and batch_result.errors:
                  content['errors'] = [error_serializer(e) for e in batch_result.errors]
              return content
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
      nodes:
      - id: PythonEngine.insert
        type: FunctionDef
        code: |-
          def insert(self, loc: Located, code: str, position: str) -> int:
              check_no_control_chars(code)
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
      - id: PythonEngine.validate
        type: FunctionDef
        code: |-
          def validate(self, source: str) -> str | None:
              try:
                  compile(source, '<validate>', 'exec')
              except SyntaxError as exc:
                  return f'{exc.msg} (line {exc.lineno})'
              return None
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
      nodes:
      - id: RpcEngine.delete
        type: FunctionDef
        code: |-
          def delete(self, loc: Located) -> None:
              request = SourceRequest()
              request.set_Source(loc.tree.source)
              if loc.tree.path is not None:
                  request.set_Path(str(loc.tree.path))
              payload = self._unwrap(self._rpc().deleteNode(loc.node_id, request))
              loc.tree.source = payload.get_Source()
      - id: RpcEngine.node_code
        type: FunctionDef
        code: |-
          def node_code(self, node: _RpcNode) -> str:
              if node.code is not None:
                  return node.code
              lines = node.tree.source.splitlines(keepends=True)
              return ''.join(lines[node.lineno - 1:node.end_lineno])
      - id: RpcEngine.validate
        type: FunctionDef
        code: |-
          def validate(self, source: str) -> str | None:
              request = SourceRequest()
              request.set_Source(source)
              payload = self._unwrap(self._rpc().validateSource(request))
              return payload.get_Error() if payload is not None else None
      - id: RpcEngine.replace
        type: FunctionDef
        code: |-
          def replace(self, loc: Located, code: str) -> None:
              check_no_control_chars(code)
              request = CodeRequest()
              request.set_Source(loc.tree.source)
              if loc.tree.path is not None:
                  request.set_Path(str(loc.tree.path))
              request.set_Code(code)
              payload = self._unwrap(self._rpc().replaceNode(loc.node_id, request))
              loc.tree.source = payload.get_Source()
```
/answer 0a99-6f-6e allow
ReasoningToken: 165
Control Request:
```yaml
id: 0a99-70-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines
    pattern: \.(java|rs)$
    recursive: true
  reason: "Eine echte Java- und Rust-Datei finden, um zu sehen welche Node-Typen die RPC-Engines für Imports liefern"
```
/answer 0a99-70-0 allow
Control Request:
```yaml
id: 0a99-71-70
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines
      entries:
      - "./java: (57 files below)"
      - "./rust: (124 files below)"
      warning: Result reduced to top-level directories the limit of 50 is exceeded (103). Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 0a99-71-70 allow
Control Request:
```yaml
id: 0a99-72-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
    pattern: "src/main/.*(Engine|Locat|Node)[A-Za-z]*\\.java$"
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust
    pattern: "^(\\./)?src/.*\\.rs$|/src/[^/]*\\.rs$"
    recursive: true
  reason: "Engine-Quelldateien eingrenzen, die Node-Typen für Imports definieren"
```
/answer 0a99-72-0 allow
Control Request:
```yaml
id: 0a99-73-72
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
      entries: []
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/rust
      entries:
      - "./src: (122 files below)"
      warning: Result reduced to top-level directories the limit of 50 is exceeded (66). Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 0a99-73-72 allow
ReasoningToken: 105
Control Request:
```yaml
id: 0a99-74-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java
    - /home/user/xyan/xy.ai.workbench/ast-engines/rust
    pattern: '"(imports|import|use_declaration|use|package)"|"imports"|"statements"'
    include:
    - '*.java'
    - '*.rs'
    exclude:
    - '**/target/**'
    - '**/build/**'
    - '**/test/**'
    - '**/tests/**'
    limit: 20
  reason: Node-Typ-Namen für Import-Knoten in den Java-/Rust-Engines ermitteln
```
/answer 0a99-74-0 allow
Control Request:
```yaml
id: 0a99-75-74
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java
      - /home/user/xyan/xy.ai.workbench/ast-engines/rust
      matches:
      - path: src/xy/ai/mcpc/ast/engine/AddressableNode.java
        matches:
        - "* for a grouped \"imports\"/\"statements\" segment, a {@link NodeGroup}."
      - path: src/xy/ai/mcpc/ast/engine/NodeLocator.java
        matches:
        - '* "imports"/"statements" segments.'
      - path: src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
        matches:
        - "return unwrap(parser.parseImport(stripped.endsWith(\";\") ? stripped : stripped + \";\"), \"import\");"
        - "CompilationUnit fragment = unwrap(parser.parse(code), \"imports\");"
      - path: src/engine/node_locator.rs
        matches:
        - "//! \"imports\"/\"statements\" segments. Function/method bodies are leaves, never expanded."
```
/answer 0a99-75-74 allow
ReasoningToken: 108
Control Request:
```yaml
id: 0a99-76-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
    - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src
    pattern: imports|ImportDeclaration|PackageDeclaration|ItemUse|Item::Use
    include:
    - NodeLocator.java
    - node_locator.rs
    - AddressableNode.java
    limit: 30
    intent: survey
  reason: "Klären, welche Elemente (package, use, import) zur \"imports\"-Gruppe zählen"
```
/answer 0a99-76-0 allow
Control Request:
```yaml
id: 0a99-77-76
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
      - /home/user/xyan/xy.ai.workbench/ast-engines/rust/src
      matches:
      - path: xy/ai/mcpc/ast/engine/AddressableNode.java
        matches:
        - "* for a grouped \"imports\"/\"statements\" segment, a {@link NodeGroup}."
      - path: xy/ai/mcpc/ast/engine/NodeLocator.java
        matches:
        - import com.github.javaparser.ast.ImportDeclaration;
        - "* Everything else — imports at the top level, fields/initializers/annotation"
        - '* "imports"/"statements" segments.'
        - NodeList<ImportDeclaration> imports = cu.getImports();
        - "int n = imports.size(), i = 0;"
        - int piece = imports.get(i).toString().length();
        - "addGroup(imports, NodeGroup.KIND_IMPORTS, \"CompilationUnit\", \"\", start, i, used, out);"
      - path: engine/node_locator.rs
        matches:
        - "//! \"imports\"/\"statements\" segments. Function/method bodies are leaves, never expanded."
        - "if matches!(item, Item::Use(_))"
        - "Item::Use(_) => \"ItemUse\","
```
/answer 0a99-77-76 allow
Thinking:
Beide RPC-Engines melden Import-Gruppen über `node_type == 'imports'`, sodass sich alles ohne API-Änderung auf der Struktur aufbauen lässt. Als Nächstes prüfe ich Header und Bausteine, an die ich mich anhänge.


Control Request:
```yaml
id: 0a99-78-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
  node_type: Import|ImportFrom
  reason: "Import-Header der betroffenen Dateien lesen, um zirkuläre Importe zu vermeiden"
```
/answer 0a99-78-0 allow
Control Request:
```yaml
id: 0a99-79-78
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: QuHMWF|e2q5UN
        type: imports
        code: |-
          from __future__ import annotations
          import difflib
          import hashlib
          import re
          import string
          from abc import ABC, abstractmethod
          from dataclasses import asdict, dataclass, field
          from pathlib import Path
          from typing import Any
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
      - id: CXxOCc|yqtm8z
        type: imports
        code: |-
          from __future__ import annotations
          import hashlib
          import threading
          from dataclasses import dataclass
          from pathlib import Path
          from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, AstTextError, Engine, Located, OutlineNode, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, resolve_by_prefix, to_dict
          from xy.ai.mcpc.tools.ast import generic
          from xy.ai.mcpc.tools.ast import python
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
      nodes:
      - id: GDIcFO|KJiDkF
        type: imports
        code: |-
          from __future__ import annotations
          import ast
          import autopep8
          import logging
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, SEGMENT_MAX_CHARS, Tree, check_no_control_chars, id_segment
          from xy.ai.mcpc.tools.ast.python._comments import comments_to_annotations
          from xy.ai.mcpc.tools.ast.python._nodes import _DEF_TYPES, _IMPORT_TYPES, _StatementGroup, _decorators, _is_expandable
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
      nodes:
      - id: Da4IMM|t2aMCb
        type: imports
        code: |-
          from __future__ import annotations
          from dataclasses import dataclass
          from pathlib import Path
          from typing import Any
          from xy.ai.mcpc.ast.openapi.AppendInfoNodesValidateClientImpl import AppendInfoNodesValidateClientImpl
          from xy.ai.mcpc.ast.openapi.components.CodeRequest import CodeRequest
          from xy.ai.mcpc.ast.openapi.components.Error import Error
          from xy.ai.mcpc.ast.openapi.components.SourceRequest import SourceRequest
      - id: OAbX3K|ljF50V
        type: imports
        code: |-
          from xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest import InsertRequest
          from xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.PositionEnum import PositionEnum
          from xy.ai.mcpc.ast.openapi.request.nodes.post.json.LocateRequest import LocateRequest
          from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, Engine, Located, Tree, check_no_control_chars
          from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
      nodes:
      - id: jQxmOt|BqPOJi
        type: imports
        code: |-
          from __future__ import annotations
          from typing import Any
          from xy.ai.mcpc.tools.ast.base import SEGMENT_MAX_CHARS, Located, Tree, id_segment
          from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _SynthNode
```
/answer 0a99-79-78 allow
ReasoningToken: 11989
Control Request:
```yaml
id: 0a99-7a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
    ids:
    - y0nJI5|6hMrNB
    - JavaScriptEngine.init
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
    ids:
    - TreeSitterEngine.parse
  reason: JS-Engine-Konstanten und TreeSitter._parse lesen
```
/answer 0a99-7a-0 allow
Control Request:
```yaml
id: 0a99-7b-7a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
      nodes:
      - id: y0nJI5|6hMrNB
        type: statements
        code: |-
          __all__ = ['JavaScriptEngine']
          "#: Declaration/definition node types with their own 'name' (and usually 'body') field."
          _CORE_DEF_TYPES = {'function_declaration', 'generator_function_declaration', 'class_declaration', 'method_definition'}
          '#: Function/class *expression* types - only a definition once assigned to a name (below).'
          _FUNC_EXPR_TYPES = {'arrow_function', 'function_expression', 'generator_function', 'class'}
          '#: ``let``/``const``/``var`` wrappers - a definition only with exactly one'
      - id: JavaScriptEngine.init
        type: FunctionDef
        code: |-
          def __init__(self) -> None:
              super().__init__('javascript')
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py
      nodes:
      - id: TreeSitterEngine.parse
        type: FunctionDef
        code: |-
          def _parse(self, data: bytes):
              if self._parser is None:
                  try:
                      self._parser = get_parser(self.symbol)
                  except Exception as exc:
                      '# noqa: BLE001'
                      raise AstError(f"Tree-sitter grammar '{self.symbol}' is unavailable.") from exc
              return self._parser.parse(data)
```
/answer 0a99-7b-7a allow
Control Request:
```yaml
id: 0a99-7c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: ast_replace|ast_insert
    include:
    - test_*.py
    - '*_test.py'
    exclude:
    - '**/__pycache__/**'
    - '**/.venv/**'
    - '**/node_modules/**'
    limit: 10
  reason: "Vorhandene Tests der ast_*-Tools finden, um Tests für Import-Handling im gleichen Stil anzulegen"
```
/answer 0a99-7c-0 allow
Control Request:
```yaml
id: 0a99-7d-7c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: tests/test_ast_engines.py
        matches:
        - "from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert"
        - "from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace"
        - "ast_replace(items=[ReplaceItem(path=py_file, source='def bar():\\n    return 42', id='bar')])"
        - "ast_insert(items=[InsertItem(path=py_file, source='z = 5', id='bar', position='after')])"
        - "ast_replace(items=[ReplaceItem(path=json_file, source='{\"a\": 111}', id=obj_id)])"
```
/answer 0a99-7d-7c allow
ReasoningToken: 8110
Control Request:
```yaml
id: 0a99-7e-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_comments.py
    ids:
    - comments_to_annotations
  reason: "Verstehen, wie Kommentare im Python-AST abgebildet werden (Einfügeposition von Imports)"
```
/answer 0a99-7e-0 allow
Control Request:
```yaml
id: 0a99-7f-7e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_comments.py
      nodes:
      - id: comments_to_annotations
        type: FunctionDef
        code: |-
          def comments_to_annotations(source: str) -> str:
              """Rewrite ``#`` comments into standalone string-literal statements.

              A comment on its own line becomes an equally-indented string literal; a
              trailing comment is lifted onto its own literal line in front of the
              statement it belonged to. Comments inside brackets/continuations cannot be
              represented as standalone literals without breaking syntax and are dropped.
              Style and exact placement are explicitly *not* preserved – only semantics
              plus the recovered annotation text. Comments preceding or trailing a
              continuation header (``elif``/``else``/``except``/``finally``/``case``) are
              moved into the suite that header opens.
              """
              if '#' not in source:
                  return source
              lines = source.splitlines(keepends=True)
              replaces: dict[int, str] = {}
              strips: dict[int, int] = {}
              inserts: dict[int, list[str]] = {}
              depth = 0
              logical_start: int | None = None
              try:
                  for tok in tokenize.generate_tokens(io.StringIO(source).readline):
                      ttype = tok.type
                      if ttype == tokenize.NEWLINE:
                          logical_start = None
                          continue
                      if ttype in (tokenize.NL, tokenize.INDENT, tokenize.DEDENT, tokenize.ENCODING, tokenize.ENDMARKER):
                          continue
                      if ttype == tokenize.COMMENT:
                          lineno, col = tok.start
                          prefix = lines[lineno - 1][:col]
                          standalone = prefix.strip() == ''
                          literal = _annotation_literal(tok.string)
                          if depth == 0 and standalone and (logical_start is None):
                              next_idx = _next_code_line_index(lines, lineno)
                              if next_idx is not None and _is_continuation_header(lines[next_idx]):
                                  header_lineno = next_idx + 1
                                  target_indent = _suite_indent(lines, header_lineno)
                                  inserts.setdefault(header_lineno + 1, []).append(f'{target_indent}{literal}\n')
                                  replaces[lineno] = '\n'
                              else:
                                  replaces[lineno] = f'{prefix}{literal}\n'
                          elif depth == 0 and (not standalone) and (logical_start is not None):
                              stmt_line = lines[logical_start - 1]
                              if _is_continuation_header(stmt_line):
                                  target_indent = _suite_indent(lines, lineno)
                                  inserts.setdefault(lineno + 1, []).append(f'{target_indent}{literal}\n')
                              else:
                                  indent = stmt_line[:len(stmt_line) - len(stmt_line.lstrip())]
                                  inserts.setdefault(logical_start, []).append(f'{indent}{literal}\n')
                              strips[lineno] = col
                          elif standalone:
                              replaces[lineno] = '\n'
                          else:
                              strips[lineno] = col
                          continue
                      if logical_start is None:
                          logical_start = tok.start[0]
                      if ttype == tokenize.OP:
                          if tok.string in '([{':
                              depth += 1
                          elif tok.string in ')]}':
                              depth = max(0, depth - 1)
              except (tokenize.TokenError, IndentationError):
                  return source
              out: list[str] = []
              for i, line in enumerate(lines, start=1):
                  if i in inserts:
                      out.extend(inserts[i])
                  if i in replaces:
                      out.append(replaces[i])
                  elif i in strips:
                      out.append(line[:strips[i]].rstrip() + '\n')
                  else:
                      out.append(line)
              return ''.join(out)
```
/answer 0a99-7f-7e allow
Text:
 Ich habe jetzt alles, was ich brauche. Zuerst entsteht das gemeinsame Modul `ast/imports.py` mit Ergebnis-Typ, Normalisierung und den drei Strategien (generisch, Tree-sitter, RPC).
ReasoningToken: 229
Control Request:
```yaml
id: 0a99-80-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    source: |
      """Import/include/use handling shared by the ``ast_*`` edit tools.

      Every edit tool applies ``addImport``/``removeImport`` through
      :meth:`Engine.apply_imports` after the item's node edit. Support is purely
      structural (no compilation). Engines without dedicated support fall back to
      :func:`apply_generic`, which only verifies and removes whole lines.
      """
      from __future__ import annotations

      import re
      from dataclasses import dataclass, field
      from typing import Any, Callable

      from xy.ai.mcpc.tools.ast.base import AstError, Engine, Tree, check_no_control_chars

      __all__ = [
          'IMPORT_NOTICE', 'ImportOutcome', 'parse_import_args', 'import_notice', 'import_entry',
          'normalize_import', 'apply_generic', 'apply_treesitter', 'apply_rpc',
      ]

      IMPORT_NOTICE = (
          'Imports/includes/use statements could not be verified or are not supported for part of this batch. '
          'Check the affected files yourself and add or remove them manually where needed.')


      @dataclass
      class ImportOutcome:
          """What an engine did for one item's ``addImport``/``removeImport``.

          Attributes:
              added: Statements that were missing and have been inserted.
              removed: Statements that were present and have been removed.
              unchecked: Whether at least one statement could not be handled and the
                  agent has to check it manually.
          """
          added: list[str] = field(default_factory=list)
          removed: list[str] = field(default_factory=list)
          unchecked: bool = False


      def _string_list(item: dict[str, Any], key: str) -> list[str] | None:
          value = item.get(key)
          if value is None:
              return None
          if not isinstance(value, list) or not all(isinstance(v, str) and v.strip() for v in value):
              raise ValueError(f"'{key}' must be a list of non-empty strings")
          return value or None


      def parse_import_args(item: dict[str, Any]) -> tuple[list[str] | None, list[str] | None]:
          """``(addImport, removeImport)`` of a raw MCP item; ``ValueError`` on malformed values."""
          return _string_list(item, 'addImport'), _string_list(item, 'removeImport')


      def import_notice(results: list[Any]) -> str | None:
          """The single batch-level hint, if any result carries unchecked imports."""
          if any(r.imports is not None and r.imports.unchecked for r in results):
              return IMPORT_NOTICE
          return None


      def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:
          """Serialized ``imports`` field of a result, or ``None`` if nothing changed."""
          if outcome is None:
              return None
          entry = {key: value for key, value in (('added', outcome.added), ('removed', outcome.removed)) if value}
          return entry or None


      _SPACE = re.compile(r'\s+')
      _PUNCT = re.compile(r'\s*([{},:;.*()])\s*')


      def normalize_import(text: str) -> str:
          """Whitespace-, quote- and trailing-``;``-insensitive form used to compare statements."""
          squashed = _SPACE.sub(' ', text.strip()).replace('"', "'")
          return _PUNCT.sub(r'\1', squashed).replace(',}', '}').rstrip(';').strip()


      def _checked(statement: str) -> str:
          check_no_control_chars(statement)
          return statement.strip()


      def _unique(statements: list[str]) -> list[str]:
          return list(dict.fromkeys(statements))


      def statement_spans(data: bytes) -> list[tuple[int, int]]:
          """Byte spans of the ``;``-terminated top-level statements in ``data`` (comments skipped)."""
          spans: list[tuple[int, int]] = []
          depth, start, i, n = 0, None, 0, len(data)
          while i < n:
              if data.startswith(b'//', i):
                  end = data.find(b'\n', i)
                  i = n if end < 0 else end
                  continue
              if data.startswith(b'/*', i):
                  end = data.find(b'*/', i + 2)
                  i = n if end < 0 else end + 2
                  continue
              char = data[i:i + 1]
              if start is None and not char.isspace():
                  start = i
              if char == b'{':
                  depth += 1
              elif char == b'}':
                  depth = max(depth - 1, 0)
              elif char == b';' and depth == 0:
                  spans.append((start, i + 1))
                  start = None
              i += 1
          if start is not None:
              spans.append((start, n))
          return spans


      def delete_span(data: bytes, start: int, end: int) -> bytes:
          """Remove ``data[start:end]``, including its line if nothing else stands on it."""
          line_start = data.rfind(b'\n', 0, start) + 1
          line_end = data.find(b'\n', end)
          line_end = len(data) if line_end < 0 else line_end
          if not data[line_start:start].strip() and not data[end:line_end].strip():
              return data[:line_start] + data[min(line_end + 1, len(data)):]
          return data[:start] + data[end:]


      def _next_line_start(data: bytes, offset: int) -> int:
          newline = data.find(b'\n', offset)
          return len(data) if newline < 0 else newline + 1


      def _line_starts(data: bytes) -> list[int]:
          starts = [0]
          for line in data.splitlines(keepends=True):
              starts.append(starts[-1] + len(line))
          return starts


      def _store(engine: Engine, tree: Tree, data: bytes) -> None:
          source = data.decode('utf-8')
          error = engine.validate(source) if engine.validates_syntax else None
          if error:
              raise AstError(f'The import change would leave the file invalid: {error}')
          tree.raw = engine.parse(source, tree.path).raw
          tree.source = source


      def apply_generic(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
          """Engine-agnostic fallback.

          ``add`` is only checked: a statement missing as a line is reported as unchecked
          and never written. ``remove`` deletes lines that equal the statement; if the text
          occurs anywhere else (or spans several lines) nothing is touched and the batch
          is flagged instead.
          """
          outcome = ImportOutcome()
          source = tree.source
          present = {normalize_import(line) for line in source.splitlines()}
          outcome.unchecked = any(normalize_import(_checked(s)) not in present for s in add)
          lines = source.splitlines(keepends=True)
          for statement in _unique([_checked(s) for s in remove]):
              wanted = normalize_import(statement)
              kept = [line for line in lines if normalize_import(line) != wanted]
              if len(kept) != len(lines):
                  lines = kept
                  outcome.removed.append(statement)
              elif statement.rstrip(';').strip() in ''.join(lines):
                  outcome.unchecked = True
          if outcome.removed:
              _store(engine, tree, ''.join(lines).encode('utf-8'))
          return outcome


      def _prologue_end(root: Any, data: bytes, is_prologue: Callable[[Any], bool] | None) -> int:
          """Offset after leading shebang/directives and header comments (comments followed by a blank line)."""
          nodes = root.named_children
          pos, i = 0, 0
          while i < len(nodes):
              node = nodes[i]
              if node.type == 'hash_bang_line' or (is_prologue is not None and is_prologue(node)):
                  pos, i = _next_line_start(data, node.end_byte), i + 1
                  continue
              if node.type != 'comment':
                  break
              j = i
              while j + 1 < len(nodes) and nodes[j + 1].type == 'comment' and nodes[j + 1].start_point[0] - nodes[j].end_point[0] <= 1:
                  j += 1
              following = nodes[j + 1] if j + 1 < len(nodes) else None
              if following is None or following.start_point[0] - nodes[j].end_point[0] <= 1:
                  break
              pos, i = _next_line_start(data, nodes[j].end_byte), j + 1
          return pos


      def _snippet(engine: Engine, statement: str, import_types: set[str]) -> str:
          text = _checked(statement)
          root = engine.parse(text).raw.root_node
          nodes = root.named_children
          if root.has_error or len(nodes) != 1 or nodes[0].type not in import_types:
              raise AstError(f'Not a single import statement: {text!r}')
          return text


      def apply_treesitter(
              engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str],
              is_prologue: Callable[[Any], bool] | None = None) -> ImportOutcome:
          """Structure-aware handling for tree-sitter engines whose top-level import nodes are ``import_types``."""
          adds = _unique([_snippet(engine, s, import_types) for s in add])
          removes = {normalize_import(_snippet(engine, s, import_types)): s.strip() for s in remove}
          outcome = ImportOutcome()
          data = tree.source.encode('utf-8')

          def imports(root: Any) -> list[Any]:
              return [n for n in root.named_children if n.type in import_types]

          doomed = [n for n in imports(tree.raw.root_node)
                    if normalize_import(data[n.start_byte:n.end_byte].decode('utf-8')) in removes]
          for node in sorted(doomed, key=lambda n: n.start_byte, reverse=True):
              data = delete_span(data, node.start_byte, node.end_byte)
          outcome.removed = _unique([removes[normalize_import(tree.source.encode('utf-8')[n.start_byte:n.end_byte].decode('utf-8'))]
                                     for n in doomed])
          if doomed:
              root = engine.parse(data.decode('utf-8'), tree.path).raw.root_node
          else:
              root = tree.raw.root_node
          present = {normalize_import(data[n.start_byte:n.end_byte].decode('utf-8')) for n in imports(root)}
          missing = [s for s in adds if normalize_import(s) not in present]
          if missing:
              existing = imports(root)
              if existing:
                  pos = _next_line_start(data, existing[-1].end_byte)
                  text = '\n'.join(missing).encode('utf-8') + b'\n'
                  if pos == len(data) and not data.endswith(b'\n'):
                      text = b'\n' + text[:-1]
              else:
                  pos = _prologue_end(root, data, is_prologue)
                  text = '\n'.join(missing).encode('utf-8') + b'\n'
                  if data[pos:].strip():
                      text += b'\n'
                  if pos > 0 and data[pos - 1:pos] != b'\n':
                      text = b'\n' + text
              data = data[:pos] + text + data[pos:]
              outcome.added = missing
          if doomed or missing:
              _store(engine, tree, data)
          return outcome


      def _rpc_statement(statement: str) -> str:
          text = _checked(statement)
          if len(statement_spans(text.encode('utf-8'))) != 1:
              raise AstError(f'Not a single import statement: {text!r}')
          return text if text.endswith(';') else text + ';'


      def _rpc_present(engine: Engine, tree: Tree) -> tuple[list[Any], dict[str, tuple[int, int]]]:
          """Top-level ``imports`` groups plus every statement's byte span, keyed by normalized text."""
          data = tree.source.encode('utf-8')
          starts = _line_starts(data)
          groups = [loc for loc in engine.locate_all(tree) if loc.node_type == 'imports' and '.' not in loc.node_id]
          present: dict[str, tuple[int, int]] = {}
          for loc in groups:
              low = starts[min(loc.lineno - 1, len(starts) - 1)]
              high = starts[min(loc.end_lineno, len(starts) - 1)]
              for begin, end in statement_spans(data[low:high]):
                  present.setdefault(normalize_import(data[low + begin:low + end].decode('utf-8')), (low + begin, low + end))
          return groups, present


      def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
          """Structure-aware handling for RPC engines that report ``imports`` groups (Java ``import``, Rust ``use``).

          Statements are added through the engine's own ``insert``/``append`` (so the
          process parses them), then re-located to verify they were recognized as imports.
          Removal deletes the statement's span inside its group; the result is re-validated.
          """
          adds = _unique([_rpc_statement(s) for s in add])
          removes = _unique([_rpc_statement(s) for s in remove])
          outcome = ImportOutcome()
          if removes:
              _, present = _rpc_present(engine, tree)
              data = tree.source.encode('utf-8')
              hits = [(present[normalize_import(s)], s) for s in removes if normalize_import(s) in present]
              for (begin, end), _ in sorted(hits, key=lambda hit: hit[0][0], reverse=True):
                  data = delete_span(data, begin, end)
              if hits:
                  source = data.decode('utf-8')
                  error = engine.validate(source)
                  if error:
                      raise AstError(f'The import change would leave the file invalid: {error}')
                  tree.source = source
                  outcome.removed = [s for _, s in hits]
          groups, present = _rpc_present(engine, tree)
          missing = [s for s in adds if normalize_import(s) not in present]
          if missing:
              code = '\n'.join(missing)
              if groups:
                  engine.insert(groups[-1], code, 'after')
              else:
                  top = [loc for loc in engine.locate_all(tree) if '.' not in loc.node_id]
                  if top:
                      engine.insert(top[0], code, 'before')
                  else:
                      engine.append(tree, code)
              _, present = _rpc_present(engine, tree)
              if any(normalize_import(s) not in present for s in missing):
                  raise AstError(f'Not recognized as an import by the parser: {missing!r}')
              outcome.added = missing
          return outcome
  reason: "Neues gemeinsames Modul für Import-Handling (generisch, Tree-sitter, RPC) anlegen"
```
/answer 0a99-80-0 allow
Text:
 Beim Gegenlesen ist mir in `apply_treesitter` eine unsaubere Stelle aufgefallen: Die `removed`-Liste wird über einen erneuten Zugriff auf `tree.source` berechnet. Das räume ich auf.
ReasoningToken: 1164
Control Request:
```yaml
id: 0a99-81-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  name: apply_treesitter
  reason: Node-ID von apply_treesitter für die gezielte Korrektur ermitteln
```
/answer 0a99-81-0 allow
Control Request:
```yaml
id: 0a99-82-81
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: apply_treesitter
        code: |-
          def apply_treesitter(engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str], is_prologue: Callable[[Any], bool] | None=None) -> ImportOutcome:
              """Structure-aware handling for tree-sitter engines whose top-level import nodes are ``import_types``."""
              adds = _unique([_snippet(engine, s, import_types) for s in add])
              removes = {normalize_import(_snippet(engine, s, import_types)): s.strip() for s in remove}
              outcome = ImportOutcome()
              data = tree.source.encode('utf-8')

              def imports(root: Any) -> list[Any]:
                  return [n for n in root.named_children if n.type in import_types]
              doomed = [n for n in imports(tree.raw.root_node) if normalize_import(
                  data[n.start_byte:n.end_byte].decode('utf-8')) in removes]
              for node in sorted(doomed, key=lambda n: n.start_byte, reverse=True):
                  data = delete_span(data, node.start_byte, node.end_byte)
              outcome.removed = _unique([removes[normalize_import(tree.source.encode(
                  'utf-8')[n.start_byte:n.end_byte].decode('utf-8'))] for n in doomed])
              if doomed:
                  root = engine.parse(data.decode('utf-8'), tree.path).raw.root_node
              else:
                  root = tree.raw.root_node
              present = {normalize_import(data[n.start_byte:n.end_byte].decode('utf-8')) for n in imports(root)}
              missing = [s for s in adds if normalize_import(s) not in present]
              if missing:
                  existing = imports(root)
                  if existing:
                      pos = _next_line_start(data, existing[-1].end_byte)
                      text = '\n'.join(missing).encode('utf-8') + b'\n'
                      if pos == len(data) and (not data.endswith(b'\n')):
                          text = b'\n' + text[:-1]
                  else:
                      pos = _prologue_end(root, data, is_prologue)
                      text = '\n'.join(missing).encode('utf-8') + b'\n'
                      if data[pos:].strip():
                          text += b'\n'
                      if pos > 0 and data[pos - 1:pos] != b'\n':
                          text = b'\n' + text
                  data = data[:pos] + text + data[pos:]
                  outcome.added = missing
              if doomed or missing:
                  _store(engine, tree, data)
              return outcome
```
/answer 0a99-82-81 allow
Control Request:
```yaml
id: 0a99-83-0
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: apply_treesitter
    start_marker: |2-
          outcome = ImportOutcome()
          data = tree.source.encode('utf-8')

          def imports
    end_marker: |2
              root = tree.raw.root_node
    exact: false
    content: |2
          outcome = ImportOutcome()
          data = tree.source.encode('utf-8')

          def imports(root: Any) -> list[Any]:
              return [n for n in root.named_children if n.type in import_types]

          def text_of(node: Any, buffer: bytes) -> str:
              return buffer[node.start_byte:node.end_byte].decode('utf-8')

          doomed = [(n, key) for n in imports(tree.raw.root_node)
                    if (key := normalize_import(text_of(n, data))) in removes]
          for node, _ in sorted(doomed, key=lambda hit: hit[0].start_byte, reverse=True):
              data = delete_span(data, node.start_byte, node.end_byte)
          outcome.removed = _unique([removes[key] for _, key in doomed])
          if doomed:
              root = engine.parse(data.decode('utf-8'), tree.path).raw.root_node
          else:
              root = tree.raw.root_node
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: apply_treesitter
    start_marker: "    present = {normalize_import(data[n.start_byte"
    end_marker: "for n in imports(root)}"
    content: "    present = {normalize_import(text_of(n, data)) for n in imports(root)}"
  reason: "removed-Berechnung in apply_treesitter bereinigen (Hilfsfunktion text_of, keine doppelte Dekodierung)"
```
/answer 0a99-83-0 allow
Control Request:
```yaml
id: 0a99-84-83
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      result: success
      id: apply_treesitter
      new_id: apply_treesitter
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      error: "Syntax error: unexpected indent (line 5)"
      id: apply_treesitter
```
/answer 0a99-84-83 allow
ReasoningToken: 154
Control Request:
```yaml
id: 0a99-85-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    ids:
    - apply_treesitter
  reason: Zustand nach der teilweise fehlgeschlagenen Mehrfachänderung prüfen
```
/answer 0a99-85-0 allow
Control Request:
```yaml
id: 0a99-86-85
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: apply_treesitter
        type: FunctionDef
        code: |-
          def apply_treesitter(engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str], is_prologue: Callable[[Any], bool] | None=None) -> ImportOutcome:
              """Structure-aware handling for tree-sitter engines whose top-level import nodes are ``import_types``."""
              adds = _unique([_snippet(engine, s, import_types) for s in add])
              removes = {normalize_import(_snippet(engine, s, import_types)): s.strip() for s in remove}
              outcome = ImportOutcome()
              data = tree.source.encode('utf-8')

              def imports(root: Any) -> list[Any]:
                  return [n for n in root.named_children if n.type in import_types]
              doomed = [n for n in imports(tree.raw.root_node) if normalize_import(
                  data[n.start_byte:n.end_byte].decode('utf-8')) in removes]
              for node in sorted(doomed, key=lambda n: n.start_byte, reverse=True):
                  data = delete_span(data, node.start_byte, node.end_byte)
              outcome.removed = _unique([removes[normalize_import(tree.source.encode(
                  'utf-8')[n.start_byte:n.end_byte].decode('utf-8'))] for n in doomed])
              if doomed:
                  root = engine.parse(data.decode('utf-8'), tree.path).raw.root_node
              else:
                  root = tree.raw.root_node
                  present = {normalize_import(text_of(n, data)) for n in imports(root)}
              missing = [s for s in adds if normalize_import(s) not in present]
              if missing:
                  existing = imports(root)
                  if existing:
                      pos = _next_line_start(data, existing[-1].end_byte)
                      text = '\n'.join(missing).encode('utf-8') + b'\n'
                      if pos == len(data) and (not data.endswith(b'\n')):
                          text = b'\n' + text[:-1]
                  else:
                      pos = _prologue_end(root, data, is_prologue)
                      text = '\n'.join(missing).encode('utf-8') + b'\n'
                      if data[pos:].strip():
                          text += b'\n'
                      if pos > 0 and data[pos - 1:pos] != b'\n':
                          text = b'\n' + text
                  data = data[:pos] + text + data[pos:]
                  outcome.added = missing
              if doomed or missing:
                  _store(engine, tree, data)
              return outcome
```
/answer 0a99-86-85 allow
Text:
 Nur die zweite Änderung wurde (falsch eingerückt) angewendet. Ich ersetze die ganze Funktion in einem Zug.
ReasoningToken: 60
Control Request:
```yaml
id: 0a99-87-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: apply_treesitter
    source: |-
      def apply_treesitter(
              engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str],
              is_prologue: Callable[[Any], bool] | None = None) -> ImportOutcome:
          """Structure-aware handling for tree-sitter engines whose top-level import nodes are ``import_types``."""
          adds = _unique([_snippet(engine, s, import_types) for s in add])
          removes = {normalize_import(_snippet(engine, s, import_types)): s.strip() for s in remove}
          outcome = ImportOutcome()
          data = tree.source.encode('utf-8')

          def imports(root: Any) -> list[Any]:
              return [n for n in root.named_children if n.type in import_types]

          def text_of(node: Any, buffer: bytes) -> str:
              return buffer[node.start_byte:node.end_byte].decode('utf-8')

          doomed = [(n, key) for n in imports(tree.raw.root_node)
                    if (key := normalize_import(text_of(n, data))) in removes]
          for node, _ in sorted(doomed, key=lambda hit: hit[0].start_byte, reverse=True):
              data = delete_span(data, node.start_byte, node.end_byte)
          outcome.removed = _unique([removes[key] for _, key in doomed])
          root = engine.parse(data.decode('utf-8'), tree.path).raw.root_node if doomed else tree.raw.root_node
          present = {normalize_import(text_of(n, data)) for n in imports(root)}
          missing = [s for s in adds if normalize_import(s) not in present]
          if missing:
              existing = imports(root)
              block = '\n'.join(missing).encode('utf-8')
              if existing:
                  pos = _next_line_start(data, existing[-1].end_byte)
                  text = block + b'\n'
                  if pos == len(data) and not data.endswith(b'\n'):
                      text = b'\n' + block
              else:
                  pos = _prologue_end(root, data, is_prologue)
                  text = block + b'\n'
                  if data[pos:].strip():
                      text += b'\n'
                  if pos > 0 and data[pos - 1:pos] != b'\n':
                      text = b'\n' + text
              data = data[:pos] + text + data[pos:]
              outcome.added = missing
          if doomed or missing:
              _store(engine, tree, data)
          return outcome
  reason: apply_treesitter sauber neu schreiben
```
/answer 0a99-87-0 allow
Text:
 Als Nächstes folgt die Python-Variante, die semantisch auf dem `ast` arbeitet.
ReasoningToken: 89
Control Request:
```yaml
id: 0a99-88-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_imports.py
    source: |
      """Semantic import add/remove on the top-level body of a Python module."""
      from __future__ import annotations

      import ast

      from xy.ai.mcpc.tools.ast.imports import ImportOutcome
      from xy.ai.mcpc.tools.ast.python._nodes import _IMPORT_TYPES

      __all__ = ['apply_python_imports']

      _Key = tuple[str, str, int, str, str | None]


      def _key(node: ast.Import | ast.ImportFrom, alias: ast.alias) -> _Key:
          if isinstance(node, ast.Import):
              return ('import', '', 0, alias.name, alias.asname)
          return ('from', node.module or '', node.level, alias.name, alias.asname)


      def _is_future(node: ast.stmt) -> bool:
          return isinstance(node, ast.ImportFrom) and node.module == '__future__' and node.level == 0


      def _after_docstring(body: list[ast.stmt]) -> int:
          first = body[0] if body else None
          is_doc = isinstance(first, ast.Expr) and isinstance(first.value, ast.Constant) and isinstance(first.value.value, str)
          return 1 if is_doc else 0


      def _insert(body: list[ast.stmt], node: ast.Import | ast.ImportFrom) -> None:
          last = last_future = None
          for i, stmt in enumerate(body):
              if isinstance(stmt, _IMPORT_TYPES):
                  last = i
              if _is_future(stmt):
                  last_future = i
          if _is_future(node):
              index = _after_docstring(body) if last_future is None else last_future + 1
          else:
              index = _after_docstring(body) if last is None else last + 1
          body.insert(index, node)


      def apply_python_imports(
              body: list[ast.stmt], add: list[tuple[str, ast.Import | ast.ImportFrom]],
              remove: list[tuple[str, ast.Import | ast.ImportFrom]]) -> ImportOutcome:
          """Apply parsed ``(text, node)`` statements to ``body``; only top-level imports count."""
          outcome = ImportOutcome()
          for text, node in remove:
              wanted = {_key(node, alias) for alias in node.names}
              hit, i = False, 0
              while i < len(body):
                  stmt = body[i]
                  if isinstance(stmt, _IMPORT_TYPES):
                      kept = [a for a in stmt.names if _key(stmt, a) not in wanted]
                      if len(kept) != len(stmt.names):
                          hit = True
                          if kept:
                              stmt.names = kept
                          else:
                              del body[i]
                              continue
                  i += 1
              if hit:
                  outcome.removed.append(text)
          existing = {_key(s, a) for s in body if isinstance(s, _IMPORT_TYPES) for a in s.names}
          for text, node in add:
              missing = [a for a in node.names if _key(node, a) not in existing]
              if not missing:
                  continue
              node.names = missing
              existing.update(_key(node, a) for a in missing)
              _insert(body, node)
              outcome.added.append(text)
          return outcome
  reason: "Python-spezifische, semantische Import-Logik auf dem ast-Body anlegen"
```
/answer 0a99-88-0 allow
Text:
 Jetzt hänge ich die Hooks in die Engines ein, zuerst die Basis-Klasse (generischer Default) und die Python-Engine.
Control Request:
```yaml
id: 0a99-89-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    id: Engine.append
    position: after
    source: |2-
          def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> 'ImportOutcome':
              """Add missing / remove present import statements (``addImport``/``removeImport``).

              The default only verifies and removes whole lines; engines with structural
              knowledge of their language's import syntax override this.
              """
              from xy.ai.mcpc.tools.ast.imports import apply_generic
              return apply_generic(self, tree, add, remove)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    id: PythonEngine.append
    position: after
    source: |2-
          def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
              def parse(texts: list[str]) -> list[tuple[str, ast.Import | ast.ImportFrom]]:
                  parsed = []
                  for text in texts:
                      check_no_control_chars(text)
                      stmts = self._parse_fragment(text)
                      if len(stmts) != 1 or not isinstance(stmts[0], _IMPORT_TYPES):
                          raise AstError(f'Not a single import statement: {text.strip()!r}')
                      parsed.append((text.strip(), stmts[0]))
                  return parsed
              return apply_python_imports(tree.raw.body, parse(add), parse(remove))
  reason: apply_imports-Default in Engine und Python-Override einfügen
```
/answer 0a99-89-0 allow
ReasoningToken: 218
Control Request:
```yaml
id: 0a99-8a-89
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      error: "Syntax error: unexpected indent (line 1)"
      id: Engine.append
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
      error: "Syntax error: unexpected indent (line 1)"
      id: PythonEngine.append
```
/answer 0a99-8a-89 allow
Control Request:
```yaml
id: 0a99-8b-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    id: GDIcFO|KJiDkF
    old_text: from xy.ai.mcpc.tools.ast.python._comments import comments_to_annotations
    new_text: |-
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome
      from xy.ai.mcpc.tools.ast.python._comments import comments_to_annotations
      from xy.ai.mcpc.tools.ast.python._imports import apply_python_imports
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    id: QuHMWF|e2q5UN
    old_text: |
      from typing import Any
    new_text: |
      from typing import TYPE_CHECKING, Any

      if TYPE_CHECKING:
          from xy.ai.mcpc.tools.ast.imports import ImportOutcome
  reason: Benötigte Imports in Python-Engine und base.py ergänzen
```
/answer 0a99-8b-0 allow
Control Request:
```yaml
id: 0a99-8c-8b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
      result: success
      id: GDIcFO|KJiDkF
      new_id: GDIcFO|KJiDkF
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      error: Text not found in node; its content appears to have changed.
      id: QuHMWF|e2q5UN
      reason: content_changed
      guess: |-
        from pathlib import Path
        from typing import Any
```
/answer 0a99-8c-8b allow
Text:
 Die Einfügungen scheiterten an der Einrückung, denn der Quelltext muss ohne führende Einrückung übergeben werden. Ich wiederhole sie entsprechend.
Control Request:
```yaml
id: 0a99-8d-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    id: Engine.append
    position: after
    source: |-
      def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> 'ImportOutcome':
          """Add missing / remove present import statements (``addImport``/``removeImport``).

          The default only verifies and removes whole lines; engines with structural
          knowledge of their language's import syntax override this.
          """
          from xy.ai.mcpc.tools.ast.imports import apply_generic
          return apply_generic(self, tree, add, remove)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
    id: PythonEngine.append
    position: after
    source: |-
      def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
          def parse(texts: list[str]) -> list[tuple[str, ast.Import | ast.ImportFrom]]:
              parsed = []
              for text in texts:
                  check_no_control_chars(text)
                  stmts = self._parse_fragment(text)
                  if len(stmts) != 1 or not isinstance(stmts[0], _IMPORT_TYPES):
                      raise AstError(f'Not a single import statement: {text.strip()!r}')
                  parsed.append((text.strip(), stmts[0]))
              return parsed
          return apply_python_imports(tree.raw.body, parse(add), parse(remove))
  reason: apply_imports in Engine-Basisklasse und PythonEngine einfügen (ohne führende Einrückung)
```
/answer 0a99-8d-0 allow
ReasoningToken: 175
Control Request:
```yaml
id: 0a99-8e-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    id: QuHMWF|e2q5UN
    old_text: |-
      from pathlib import Path
      from typing import Any
    new_text: |-
      from pathlib import Path
      from typing import TYPE_CHECKING, Any

      if TYPE_CHECKING:
          from xy.ai.mcpc.tools.ast.imports import ImportOutcome
  reason: TYPE_CHECKING-Import für ImportOutcome in base.py
```
/answer 0a99-8e-0 allow
Text:
 Jetzt die Overrides für JavaScript und die RPC-Engine (Java und Rust).
Control Request:
```yaml
id: 0a99-8f-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
    id: JavaScriptEngine.locate_all
    position: after
    source: |-
      def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
          return apply_treesitter(self, tree, add, remove, _IMPORT_TYPES, _is_directive)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
    id: def_body
    position: after
    source: |-
      def _is_directive(node: Any) -> bool:
          """A ``'use strict'``-style prologue statement."""
          return node.type == 'expression_statement' and [c.type for c in node.named_children] == ['string']
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    id: RpcEngine.append
    position: after
    source: |-
      def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
          return apply_rpc(self, tree, add, remove)
  reason: apply_imports-Overrides für JavaScript (Tree-sitter) und RpcEngine einfügen
```
/answer 0a99-8f-0 allow
ReasoningToken: 148
Control Request:
```yaml
id: 0a99-90-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
    id: jQxmOt|BqPOJi
    old_text: "from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _SynthNode"
    new_text: |-
      from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _SynthNode
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, apply_treesitter
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    id: OAbX3K|ljF50V
    old_text: from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
    new_text: |-
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, apply_rpc
      from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
  reason: Imports für die neuen Overrides ergänzen
```
/answer 0a99-90-0 allow
Text:
 Jetzt der Einstiegspunkt in `core.py`, der bei Fehlern den ungespeicherten Edit verwirft, plus Schema und Batch-Hinweis.
Control Request:
```yaml
id: 0a99-91-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: append_nodes
    position: after
    source: |-
      def apply_imports(path: Path, tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:
          """Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``.

          On failure the cached tree is dropped, so the item's unsaved node edit is not leaked.
          """
          if not add and not remove:
              return None
          try:
              return tree.engine.apply_imports(tree, add or [], remove or [])
          except AstError as exc:
              CACHE.invalidate(path)
              raise AstError(f'Import handling failed, nothing was written: {exc}') from exc
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
    id: BdeVO2|NSUKUv
    position: after
    source: |-
      '#: Import handling shared by every node-editing tool (replace/insert/delete/edit_*).'
      IMPORT_PROPS = {
          'addImport': {
              'type': 'array',
              'items': {'type': 'string', 'minLength': 1},
              'description': 'Import/include/use statements this edit needs, one complete statement per entry in the '
                             "file's own syntax (e.g. 'import os', 'import a.b.C;', 'use std::fmt;'). Missing ones are "
                             'added next to the existing imports; present ones are left alone. For file types without '
                             'import support nothing is written and you are asked to check the imports yourself.'},
          'removeImport': {
              'type': 'array',
              'items': {'type': 'string', 'minLength': 1},
              'description': 'Import/include/use statements that are no longer needed, same form as addImport. '
                             'Removed if present, ignored otherwise.'}}
  reason: core.apply_imports und IMPORT_PROPS (Schema) ergänzen
```
/answer 0a99-91-0 allow
ReasoningToken: 246
Control Request:
```yaml
id: 0a99-92-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: CXxOCc|yqtm8z
    old_text: |
      from xy.ai.mcpc.tools.ast import generic
    new_text: |
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome
      from xy.ai.mcpc.tools.ast import generic
  reason: ImportOutcome in core.py importieren
```
/answer 0a99-92-0 allow
Control Request:
```yaml
id: 0a99-93-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: serialize_batch_result
    old_text: '    return content'
    new_text: |2-
          notice = getattr(batch_result, 'notice', None)
          if notice:
              content['notice'] = notice
          return content
  reason: Batch-weiten Hinweis (notice) im gemeinsamen Serializer ausgeben
```
/answer 0a99-93-0 allow
ReasoningToken: 98
Control Request:
```yaml
id: 0a99-94-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
    ids:
    - w0xyc1|6KBBvL
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    ids:
    - ReplaceError
    - ReplaceBatchResult
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  reason: __all__ von common.py sowie Error-/Batch-Dataclasses (replace) lesen
```
/answer 0a99-94-0 allow
Control Request:
```yaml
id: 0a99-95-94
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
      nodes:
      - id: w0xyc1|6KBBvL
        type: statements
        code: |-
          __all__ = ['SELECTOR_PROPS', 'PATH_SELECTOR_PROPS', 'PATH_PROP', 'select_one', 'select_by_path', 'select_by_text']
          '#: Full node selectors – only ``ast_find`` may restrict on node properties.'
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: ReplaceError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceError:
              """Error applying a single node replacement.

              Attributes:
                  path: The path exactly as given in the input, for result association.
                  id: The id exactly as given in the input, for result association.
                  error: The error message.
              """
              path: str
              id: str | None
              error: str
      - id: ReplaceBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class ReplaceBatchResult:
              """Result of :func:`ast_replace`.

              Attributes:
                  results: One :class:`ReplaceResult` per successful replacement.
                  errors: One :class:`ReplaceError` per replacement that failed.
              """
              results: list[ReplaceResult]
              errors: list[ReplaceError]
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      nodes:
      - id: G65kv0|hrDhpa
        signature: "'``ast_replace`` tool: replace selected nodes with new source.'"
      - id: wlvDdw|eXR9kh
        signature: from dataclasses import dataclass
      - id: xj2nBU|s4L0vm
        signature: "__all__ = ["
      - id: ReplaceItem
        signature: "@dataclass(frozen=True) class ReplaceItem:"
        docstring: "One node replacement to apply. Attributes: path: Absolute path to the file to m…"
      - id: ReplaceResult
        signature: "@dataclass(frozen=True) class ReplaceResult:"
        docstring: "Result of a single node replacement. Attributes: path: The path exactly as give…"
      - id: ReplaceError
        signature: "@dataclass(frozen=True) class ReplaceError:"
        docstring: "Error applying a single node replacement. Attributes: path: The path exactly as…"
      - id: ReplaceBatchResult
        signature: "@dataclass(frozen=True) class ReplaceBatchResult:"
        docstring: "Result of :func:`ast_replace`. Attributes: results: One :class:`ReplaceResult` …"
      - id: replace_one
        signature: "def _replace_one(item: ReplaceItem) -> ReplaceResult:"
      - id: ast_replace
        signature: "def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:"
        docstring: Replace one or more selected nodes with new source. ``id`` and replacement ``so…
      - id: ReplaceNodeTool
        signature: "class ReplaceNodeTool(ToolDefinition):"
        children:
        - id: ReplaceNodeTool.jKVWKu|S7XtH2
          signature: name = 'ast_replace'
        - id: ReplaceNodeTool.hKJ2Rq|SrCCxe
          signature: _ITEMS_DESCRIPTION = 'Node replacements to apply.'
        - id: ReplaceNodeTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: "Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."
      - id: register
        signature: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer 0a99-95-94 allow
Text:
 Jetzt die fünf Tools. Ich beginne mit `replace.py` und übernehme das Muster danach für die anderen.
Control Request:
```yaml
id: 0a99-96-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceItem
    old_text: |2
              id: Unique id of the target node.
    new_text: |2
              id: Unique id of the target node.
              add_import: Import statements to add if missing, applied after the node edit.
              remove_import: Import statements to remove if present, applied after the node edit.
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceItem
    old_text: |2-
          source: str
          id: str | None = None
    new_text: |2-
          source: str
          id: str | None = None
          add_import: list[str] | None = None
          remove_import: list[str] | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceResult
    old_text: |2-
          result: str
          new_id: str | None = None
    new_text: |2-
          result: str
          new_id: str | None = None
          imports: ImportOutcome | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceBatchResult
    old_text: "    errors: list[ReplaceError]"
    new_text: |2-
          errors: list[ReplaceError]
          notice: str | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: replace_one
    old_text: |2-
          core.CACHE.save(file_path, tree)
          return ReplaceResult(path=item.path, id=item.id, result='success', new_id=new_id)
    new_text: |2-
          imports = core.apply_imports(file_path, tree, item.add_import, item.remove_import)
          core.CACHE.save(file_path, tree)
          return ReplaceResult(path=item.path, id=item.id, result='success', new_id=new_id, imports=imports)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ast_replace
    old_text: "    return ReplaceBatchResult(results=results, errors=errors)"
    new_text: "    return ReplaceBatchResult(results=results, errors=errors, notice=import_notice(results))"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceNodeTool.jKVWKu|S7XtH2
    old_text: "    **PATH_SELECTOR_PROPS}"
    new_text: |2-
          **PATH_SELECTOR_PROPS,
          **IMPORT_PROPS}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceNodeTool.handle
    old_text: "        return ReplaceItem(path=it['path'], source=it['source'], id=it.get('id'))"
    new_text: |2-
              add_import, remove_import = parse_import_args(it)
              return ReplaceItem(
                  path=it['path'], source=it['source'], id=it.get('id'),
                  add_import=add_import, remove_import=remove_import)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: ReplaceNodeTool.handle
    old_text: |2-
                  entry['new_id'] = r.new_id
              return entry
    new_text: |2-
                  entry['new_id'] = r.new_id
              if (imports := import_entry(r.imports)) is not None:
                  entry['imports'] = imports
              return entry
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: wlvDdw|eXR9kh
    old_text: from dataclasses import dataclass
    new_text: |-
      from dataclasses import dataclass
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
  reason: "replace.py: Felder addImport/removeImport, Import-Anwendung pro Item, Notice und Schema ergänzen"
```
/answer 0a99-96-0 allow
ReasoningToken: 613
Control Request:
```yaml
id: 0a99-97-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
    id: w0xyc1|6KBBvL
    old_text: "'PATH_PROP', 'select_one'"
    new_text: "'PATH_PROP', 'IMPORT_PROPS', 'select_one'"
  reason: IMPORT_PROPS in __all__ von common.py aufnehmen
```
/answer 0a99-97-0 allow
Text:
 `replace.py` ist durch. Für die übrigen vier Tools lese ich noch Import-Zeilen, Batch-Klassen und die `ast_*`-Funktionen.
Control Request:
```yaml
id: 0a99-98-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: "^from xy.ai.mcpc.tools.ast.common import|^from dataclasses import|return \\w+BatchResult\\(|^    errors: list\\["
    include:
    - replace.py
    - insert.py
    - delete.py
    - edit_block.py
    - edit_marks.py
    exclude:
    - '**/__pycache__/**'
    limit: 30
    intent: editing
  reason: "Exakte Textstellen für Import-Zeilen, Batch-Result-Felder und Rückgaben in den vier restlichen Tools"
```
/answer 0a99-98-0 allow
ReasoningToken: 164
Control Request:
```yaml
id: 0a99-99-98
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: replace.py
        nodes:
        - id: G5Js58|rYnJ83
          matches:
          - from dataclasses import dataclass
          - "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
        - id: ReplaceBatchResult.O157Kg|dDqIjM
          matches:
          - "errors: list[ReplaceError]"
        - id: ast_replace.Azmusq|6gXh74
          matches:
          - "errors: list[ReplaceError] = []"
          - "return ReplaceBatchResult(results=results, errors=errors, notice=import_notice(results))"
      - path: delete.py
        nodes:
        - id: wlvDdw|eXR9kh
          matches:
          - from dataclasses import dataclass
          - "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
        - id: DeleteBatchResult.zOyxzB|kwp73p
          matches:
          - "errors: list[DeleteError]"
        - id: ast_delete.aIgmMT|skC10h
          matches:
          - "errors: list[DeleteError] = []"
          - "return DeleteBatchResult(results=results, errors=errors)"
      - path: edit_marks.py
        nodes:
        - id: 0NkjOX|Zg7YTi
          matches:
          - from dataclasses import dataclass
          - "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, PATH_PROP, select_by_text"
        - id: EditMarksBatchResult.YBc8cH|4wuYBA
          matches:
          - "errors: list[EditMarksError]"
        - id: ast_edit_marks.bCr7sG|2ynpT0
          matches:
          - "errors: list[EditMarksError] = []"
        - id: ast_edit_marks.h6YkF5|5tvRCI
          matches:
          - "return EditMarksBatchResult(results=results, errors=errors)"
      - path: edit_block.py
        nodes:
        - id: 0NkjOX|zkSTbq
          matches:
          - from dataclasses import dataclass
          - "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, PATH_PROP, select_by_text"
        - id: EditBlockBatchResult.i9mput|c1NHxX
          matches:
          - "errors: list[EditBlockError]"
        - id: ast_edit_block.0dRkZw|FMYCNa
          matches:
          - "errors: list[EditBlockError] = []"
        - id: ast_edit_block.GqLOu2|bj1cye
          matches:
          - "return EditBlockBatchResult(results=results, errors=errors)"
      - path: insert.py
        nodes:
        - id: wlvDdw|eXR9kh
          matches:
          - from dataclasses import dataclass
          - "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
        - id: InsertBatchResult.wBmVDD|sQ7wIV
          matches:
          - "errors: list[InsertError]"
        - id: ast_insert.aIgmMT|0y9bk9
          matches:
          - "errors: list[InsertError] = []"
          - "return InsertBatchResult(results=results, errors=errors)"
```
/answer 0a99-99-98 allow
Control Request:
```yaml
id: 0a99-9a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    ids:
    - DeleteBatchResult
    - ast_delete
  reason: "delete: Batch-Result und ast_delete lesen"
```
/answer 0a99-9a-0 allow
Control Request:
```yaml
id: 0a99-9b-9a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
      - id: DeleteBatchResult
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class DeleteBatchResult:
              """Result of :func:`ast_delete`.

              Attributes:
                  results: One :class:`DeleteResult` per successfully deleted item.
                  errors: One :class:`DeleteError` per item that failed.
              """
              results: list[DeleteResult]
              errors: list[DeleteError]
      - id: ast_delete
        type: FunctionDef
        code: |-
          def ast_delete(items: list[DeleteItem]) -> DeleteBatchResult:
              """Delete one or more selected nodes, or whole files if their ``id`` is omitted.

              A whole file is deleted by omitting its ``id`` selector – there is no other
              way to address the root, since it is never itself an addressable child.
              Deleting a file also removes it from the AST cache and, if its parent
              directory becomes empty as a result, removes that directory too.

              Args:
                  items: Nodes/files to delete. Must be non-empty.

              Returns:
                  DeleteBatchResult: One result per deleted item, one error per failed item.

              Raises:
                  core.AstError: If ``items`` is empty.
              """
              if not items:
                  raise core.AstError("'items' must be a non-empty list.")
              results: list[DeleteResult] = []
              errors: list[DeleteError] = []
              for item in items:
                  try:
                      results.append(_delete_one(item))
                  except core.AstError as exc:
                      errors.append(DeleteError(path=item.path, id=item.id, error=str(exc)))
              return DeleteBatchResult(results=results, errors=errors)
```
/answer 0a99-9b-9a allow
Control Request:
```yaml
id: 0a99-9c-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: G5Js58|rYnJ83
    old_text: "import PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
    new_text: "import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: wlvDdw|eXR9kh
    old_text: "import PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
    new_text: "import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: wlvDdw|eXR9kh
    old_text: from dataclasses import dataclass
    new_text: |-
      from dataclasses import dataclass
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertItem
    old_text: |2
              id: Unique id of the target node.
    new_text: |2
              id: Unique id of the target node.
              add_import: Import statements to add if missing, applied after the insert.
              remove_import: Import statements to remove if present, applied after the insert.
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertItem
    old_text: |2-
          position: str = 'after'
          id: str | None = None
    new_text: |2-
          position: str = 'after'
          id: str | None = None
          add_import: list[str] | None = None
          remove_import: list[str] | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertResult
    old_text: "    ids: list[str] | None = None"
    new_text: |2-
          ids: list[str] | None = None
          imports: ImportOutcome | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertBatchResult
    old_text: "    errors: list[InsertError]"
    new_text: |2-
          errors: list[InsertError]
          notice: str | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: insert_one
    old_text: |2-
          core.CACHE.save(file_path, tree)
          return InsertResult(path=item.path, id=item.id, result='success', inserted=inserted, ids=new_ids or None)
    new_text: |2-
          imports = core.apply_imports(file_path, tree, item.add_import, item.remove_import)
          core.CACHE.save(file_path, tree)
          return InsertResult(
              path=item.path, id=item.id, result='success', inserted=inserted, ids=new_ids or None, imports=imports)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: ast_insert
    old_text: "    return InsertBatchResult(results=results, errors=errors)"
    new_text: "    return InsertBatchResult(results=results, errors=errors, notice=import_notice(results))"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertNodeTool.c0C4qn|tz50Lf
    old_text: "    **PATH_SELECTOR_PROPS}"
    new_text: |2-
          **PATH_SELECTOR_PROPS,
          **IMPORT_PROPS}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertNodeTool.handle
    old_text: "        return InsertItem(path=it['path'], source=it['source'], position=it.get('position', 'after'), id=it.get('id'))"
    new_text: |2-
              add_import, remove_import = parse_import_args(it)
              return InsertItem(
                  path=it['path'], source=it['source'], position=it.get('position', 'after'), id=it.get('id'),
                  add_import=add_import, remove_import=remove_import)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: InsertNodeTool.handle
    old_text: |2-
                  entry['ids'] = r.ids
              return entry
    new_text: |2-
                  entry['ids'] = r.ids
              if (imports := import_entry(r.imports)) is not None:
                  entry['imports'] = imports
              return entry
  reason: replace.py-Import korrigieren und insert.py um Import-Handling erweitern
```
/answer 0a99-9c-0 allow
Control Request:
```yaml
id: 0a99-9d-9c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
      result: success
      id: G5Js58|rYnJ83
      new_id: G5Js58|rYnJ83
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: wlvDdw|eXR9kh
      new_id: wlvDdw|eXR9kh
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: InsertItem
      new_id: InsertItem
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: InsertItem
      new_id: InsertItem
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: InsertResult
      new_id: InsertResult
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: InsertBatchResult
      new_id: InsertBatchResult
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: insert_one
      new_id: insert_one
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: ast_insert
      new_id: ast_insert
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: InsertNodeTool.c0C4qn|tz50Lf
      new_id: InsertNodeTool.c0C4qn|tz50Lf
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: InsertNodeTool.handle
      new_id: InsertNodeTool.handle
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      result: success
      id: InsertNodeTool.handle
      new_id: InsertNodeTool.handle
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: wlvDdw|eXR9kh
```
/answer 0a99-9d-9c allow
ReasoningToken: 1990
Control Request:
```yaml
id: 0a99-9e-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: wlvDdw|eXR9kh
    old_text: "import PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
    new_text: "import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: wlvDdw|eXR9kh
    old_text: from dataclasses import dataclass
    new_text: |-
      from dataclasses import dataclass
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteItem
    old_text: |2
              id: Unique id of the target node. Omit to delete the whole file.
    new_text: |2
              id: Unique id of the target node. Omit to delete the whole file.
              add_import: Import statements to add if missing, applied after the delete (needs ``id``).
              remove_import: Import statements to remove if present, applied after the delete (needs ``id``).
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteItem
    old_text: |2-
          path: str
          id: str | None = None
    new_text: |2-
          path: str
          id: str | None = None
          add_import: list[str] | None = None
          remove_import: list[str] | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteResult
    old_text: |2-
          id: str | None
          result: str
    new_text: |2-
          id: str | None
          result: str
          imports: ImportOutcome | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteBatchResult
    old_text: "    errors: list[DeleteError]"
    new_text: |2-
          errors: list[DeleteError]
          notice: str | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: delete_one
    old_text: |2-
          if item.id is None:
              try:
    new_text: |2-
          if item.id is None:
              if item.add_import or item.remove_import:
                  raise core.AstError("'addImport'/'removeImport' need a node 'id'; they cannot be used when deleting a file.")
              try:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: delete_one
    old_text: |2-
          core.CACHE.save(file_path, tree)
          return DeleteResult(path=item.path, id=item.id, result='success')
    new_text: |2-
          imports = core.apply_imports(file_path, tree, item.add_import, item.remove_import)
          core.CACHE.save(file_path, tree)
          return DeleteResult(path=item.path, id=item.id, result='success', imports=imports)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: ast_delete
    old_text: "    return DeleteBatchResult(results=results, errors=errors)"
    new_text: "    return DeleteBatchResult(results=results, errors=errors, notice=import_notice(results))"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteTool.gLGwBi|LEwdCL
    old_text: "_ITEM_PROPERTIES = {'path': PATH_PROP, **PATH_SELECTOR_PROPS}"
    new_text: "_ITEM_PROPERTIES = {'path': PATH_PROP, **PATH_SELECTOR_PROPS, **IMPORT_PROPS}"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteTool.handle
    old_text: |2-
              return DeleteItem(path=it['path'], id=it.get('id'))
          return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, auto_approve=True)
    new_text: |2-
              add_import, remove_import = parse_import_args(it)
              return DeleteItem(path=it['path'], id=it.get('id'), add_import=add_import, remove_import=remove_import)

          def result_serializer(r: DeleteResult) -> dict[str, Any]:
              entry = {'path': r.path, 'result': r.result}
              if r.id is not None:
                  entry['id'] = r.id
              if (imports := import_entry(r.imports)) is not None:
                  entry['imports'] = imports
              return entry
          return handle_batch_tool(
              ctx, item_factory, ast_delete, core.AstError, result_serializer, auto_approve=True)
  reason: delete.py um Import-Handling erweitern (inkl. Schutz für Datei-Löschung ohne id)
```
/answer 0a99-9e-0 allow
Control Request:
```yaml
id: 0a99-9f-9e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: wlvDdw|eXR9kh
      new_id: wlvDdw|eXR9kh
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteItem
      new_id: DeleteItem
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteItem
      new_id: DeleteItem
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteResult
      new_id: DeleteResult
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteBatchResult
      new_id: DeleteBatchResult
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: delete_one
      new_id: delete_one
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: delete_one
      new_id: delete_one
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: ast_delete
      new_id: ast_delete
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteTool.gLGwBi|LEwdCL
      new_id: DeleteTool.gLGwBi|LEwdCL
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteTool.handle
      new_id: DeleteTool.handle
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: wlvDdw|eXR9kh
```
/answer 0a99-9f-9e allow
Text:
 Die zweite Änderung am Import-Block ist in beiden Dateien gescheitert, weil sich die Node-ID nach der ersten Änderung verschoben hat. Ich hole die aktuelle ID und trage die Import-Zeile nach.
ReasoningToken: 141
Control Request:
```yaml
id: 0a99-a0-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: ^from dataclasses import dataclass
    include:
    - insert.py
    - delete.py
    - replace.py
    exclude:
    - '**/__pycache__/**'
    intent: editing
  reason: Aktuelle Node-IDs der Import-Blöcke ermitteln
```
/answer 0a99-a0-0 allow
Control Request:
```yaml
id: 0a99-a1-a0
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: replace.py
        nodes:
        - id: G5Js58|nIvKCC
          matches:
          - from dataclasses import dataclass
      - path: delete.py
        nodes:
        - id: PBZWYD|BDnoTC
          matches:
          - from dataclasses import dataclass
      - path: insert.py
        nodes:
        - id: PBZWYD|BDnoTC
          matches:
          - from dataclasses import dataclass
```
/answer 0a99-a1-a0 allow
ReasoningToken: 98
Control Request:
```yaml
id: 0a99-a2-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: PBZWYD|BDnoTC
    old_text: from dataclasses import dataclass
    new_text: |-
      from dataclasses import dataclass
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: PBZWYD|BDnoTC
    old_text: from dataclasses import dataclass
    new_text: |-
      from dataclasses import dataclass
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
  reason: Fehlende Import-Zeile in insert.py und delete.py nachtragen
```
/answer 0a99-a2-0 allow
ReasoningToken: 217
Control Request:
```yaml
id: 0a99-a3-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockItem
    old_text: |
      searched for by ``old_text``.
    new_text: |
      searched for by ``old_text``.
              add_import: Import statements to add if missing, applied after the edit.
              remove_import: Import statements to remove if present, applied after the edit.
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockItem
    old_text: |2-
          replace_all: bool = False
          id: str | None = None
    new_text: |2-
          replace_all: bool = False
          id: str | None = None
          add_import: list[str] | None = None
          remove_import: list[str] | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockResult
    old_text: |2-
          result: str
          new_id: str | None = None
    new_text: |2-
          result: str
          new_id: str | None = None
          imports: ImportOutcome | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockBatchResult
    old_text: "    errors: list[EditBlockError]"
    new_text: |2-
          errors: list[EditBlockError]
          notice: str | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: edit_block_one
    old_text: |2-
          core.CACHE.save(file_path, tree)
          return EditBlockResult(path=item.path, id=item.id, result='success', new_id=new_id)
    new_text: |2-
          imports = core.apply_imports(file_path, tree, item.add_import, item.remove_import)
          core.CACHE.save(file_path, tree)
          return EditBlockResult(path=item.path, id=item.id, result='success', new_id=new_id, imports=imports)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: ast_edit_block
    old_text: "    return EditBlockBatchResult(results=results, errors=errors)"
    new_text: "    return EditBlockBatchResult(results=results, errors=errors, notice=import_notice(results))"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockNodeTool.MuT0pV|iYKmvH
    old_text: "    **PATH_SELECTOR_PROPS}"
    new_text: |2-
          **PATH_SELECTOR_PROPS,
          **IMPORT_PROPS}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockNodeTool.handle
    old_text: '        return EditBlockItem('
    new_text: |2-
              add_import, remove_import = parse_import_args(it)
              return EditBlockItem(
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockNodeTool.handle
    old_text: |
      id=it.get('id'))
    new_text: |
      id=it.get('id'),
                  add_import=add_import,
                  remove_import=remove_import)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: EditBlockNodeTool.handle
    old_text: |2-
                  entry['new_id'] = r.new_id
              return entry
    new_text: |2-
                  entry['new_id'] = r.new_id
              if (imports := import_entry(r.imports)) is not None:
                  entry['imports'] = imports
              return entry
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksItem
    old_text: |
      searched for by the markers.
    new_text: |
      searched for by the markers.
              add_import: Import statements to add if missing, applied after the edit.
              remove_import: Import statements to remove if present, applied after the edit.
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksItem
    old_text: |2-
          exact: bool = False
          id: str | None = None
    new_text: |2-
          exact: bool = False
          id: str | None = None
          add_import: list[str] | None = None
          remove_import: list[str] | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksResult
    old_text: |2-
          result: str
          new_id: str | None = None
    new_text: |2-
          result: str
          new_id: str | None = None
          imports: ImportOutcome | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksBatchResult
    old_text: "    errors: list[EditMarksError]"
    new_text: |2-
          errors: list[EditMarksError]
          notice: str | None = None
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: edit_marks_one
    old_text: |2-
          core.CACHE.save(file_path, tree)
          return EditMarksResult(path=item.path, id=item.id, result='success', new_id=new_id)
    new_text: |2-
          imports = core.apply_imports(file_path, tree, item.add_import, item.remove_import)
          core.CACHE.save(file_path, tree)
          return EditMarksResult(path=item.path, id=item.id, result='success', new_id=new_id, imports=imports)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: ast_edit_marks
    old_text: "    return EditMarksBatchResult(results=results, errors=errors)"
    new_text: "    return EditMarksBatchResult(results=results, errors=errors, notice=import_notice(results))"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksNodeTool.MuT0pV|RNoeCK
    old_text: "    **PATH_SELECTOR_PROPS}"
    new_text: |2-
          **PATH_SELECTOR_PROPS,
          **IMPORT_PROPS}
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksNodeTool.handle
    old_text: '        return EditMarksItem('
    new_text: |2-
              add_import, remove_import = parse_import_args(it)
              return EditMarksItem(
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksNodeTool.handle
    old_text: |
      id=it.get('id'))
    new_text: |
      id=it.get('id'),
                  add_import=add_import,
                  remove_import=remove_import)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: EditMarksNodeTool.handle
    old_text: |2-
                  entry['new_id'] = r.new_id
              return entry
    new_text: |2-
                  entry['new_id'] = r.new_id
              if (imports := import_entry(r.imports)) is not None:
                  entry['imports'] = imports
              return entry
  reason: "edit_block.py und edit_marks.py um Import-Handling erweitern (Items, Results, Batch-Notice, Schema, Factory)"
```
/answer 0a99-a3-0 allow
Text:
 Jetzt noch die Import-Zeilen in den beiden Dateien, jeweils einzeln, damit sich die Node-IDs nicht dazwischenschieben.
Control Request:
```yaml
id: 0a99-a4-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: ^from dataclasses import dataclass|^from xy.ai.mcpc.tools.ast.common import
    include:
    - edit_block.py
    - edit_marks.py
    exclude:
    - '**/__pycache__/**'
    intent: editing
  reason: Aktuelle IDs der Import-Blöcke in edit_block.py und edit_marks.py
```
/answer 0a99-a4-0 allow
Control Request:
```yaml
id: 0a99-a5-a4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: edit_marks.py
        nodes:
        - id: 0NkjOX|Zg7YTi
          matches:
          - from dataclasses import dataclass
          - "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, PATH_PROP, select_by_text"
      - path: edit_block.py
        nodes:
        - id: 0NkjOX|zkSTbq
          matches:
          - from dataclasses import dataclass
          - "from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, PATH_PROP, select_by_text"
```
/answer 0a99-a5-a4 allow
Control Request:
```yaml
id: 0a99-a6-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: 0NkjOX|zkSTbq
    old_text: "import PATH_SELECTOR_PROPS, PATH_PROP, select_by_text"
    new_text: |-
      import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_text
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: 0NkjOX|Zg7YTi
    old_text: "import PATH_SELECTOR_PROPS, PATH_PROP, select_by_text"
    new_text: |-
      import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_text
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args
  reason: Imports für IMPORT_PROPS und Import-Helfer in edit_block.py/edit_marks.py ergänzen
```
/answer 0a99-a6-0 allow
ReasoningToken: 198
Control Request:
```yaml
id: 0a99-a7-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/common.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_imports.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_javascript.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
  reason: Alle geänderten Dateien auf Syntax prüfen
```
/answer 0a99-a7-0 allow
Text:
 Die Syntax ist überall in Ordnung. Ich lese noch stichprobenartig gegen, ob die Textersetzungen an den richtigen Stellen gelandet sind.
ReasoningToken: 228
Control Request:
```yaml
id: 0a99-a8-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    ids:
    - EditBlockNodeTool.handle
    - edit_block_one
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    ids:
    - delete_one
    - DeleteTool
  reason: Ergebnis der Textersetzungen in edit_block.py und delete.py kontrollieren
```
/answer 0a99-a8-0 allow
Control Request:
```yaml
id: 0a99-a9-a8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
      nodes:
      - id: EditBlockNodeTool.handle
        type: FunctionDef
        code: |-
          def handle(self, ctx: ToolContext) -> ToolResult:
              """Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST API."""

              def item_factory(it: dict[str, Any]) -> EditBlockItem:
                  add_import, remove_import = parse_import_args(it)
                  return EditBlockItem(
                      path=it['path'],
                      old_text=it['old_text'],
                      new_text=it['new_text'],
                      exact=it.get(
                          'exact',
                          False),
                      replace_all=it.get(
                          'replaceAll',
                          False),
                      id=it.get('id'),
                      add_import=add_import,
                      remove_import=remove_import)

              def result_serializer(r: EditBlockResult) -> dict[str, Any]:
                  entry = {'path': r.path, 'result': r.result}
                  if r.id is not None:
                      entry['id'] = r.id
                  if r.new_id is not None:
                      entry['new_id'] = r.new_id
                  if (imports := import_entry(r.imports)) is not None:
                      entry['imports'] = imports
                  return entry

              def error_serializer(e: EditBlockError) -> dict[str, Any]:
                  entry = {'path': e.path, 'error': e.error}
                  if e.id is not None:
                      entry['id'] = e.id
                  for field in ('candidates', 'reason', 'corrected_text', 'guess', 'next_step'):
                      value = getattr(e, field)
                      if value is not None:
                          entry[field] = value
                  return entry
              return handle_batch_tool(
                  ctx,
                  item_factory,
                  ast_edit_block,
                  core.AstError,
                  result_serializer,
                  error_serializer,
                  auto_approve=True)
      - id: edit_block_one
        type: FunctionDef
        code: |-
          def _edit_block_one(item: EditBlockItem) -> EditBlockResult:
              file_path = core.require_path(item.path)
              tree = core.CACHE.get_tree(file_path)
              target = select_by_text(tree, [item.old_text], id=item.id)
              node_source = core.edit_node_source(target)
              try:
                  new_source = replace_in_block(
                      node_source,
                      item.old_text,
                      item.new_text,
                      exact=item.exact,
                      replace_all=item.replace_all,
                      accept=_node_guard(
                          tree.engine,
                          item.old_text),
                      max_level=3 if tree.engine.validates_syntax else 2,
                      where='node')
              except TextMatchError as exc:
                  raise core.AstTextError(
                      str(exc), reason=getattr(
                          exc, 'reason', None), position=getattr(
                              exc, 'position', None), corrected_text=getattr(
                                  exc, 'corrected_text', None), guess=getattr(
                                      exc, 'guess', None), next_step=getattr(
                                          exc, 'next_step', None)) from exc
              new_id = core.replace_node(target, new_source)
              imports = core.apply_imports(file_path, tree, item.add_import, item.remove_import)
              core.CACHE.save(file_path, tree)
              return EditBlockResult(path=item.path, id=item.id, result='success', new_id=new_id, imports=imports)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
      - id: delete_one
        type: FunctionDef
        code: |-
          def _delete_one(item: DeleteItem) -> DeleteResult:
              file_path = core.require_path(item.path)
              if item.id is None:
                  if item.add_import or item.remove_import:
                      raise core.AstError(
                          "'addImport'/'removeImport' need a node 'id'; they cannot be used when deleting a file.")
                  try:
                      file_path.unlink()
                  except OSError as exc:
                      raise core.AstError('Delete failed.') from exc
                  core.CACHE.invalidate(file_path)
                  parent = file_path.parent
                  if not any(parent.iterdir()):
                      parent.rmdir()
                  return DeleteResult(path=item.path, id=None, result='success')
              tree = core.CACHE.get_tree(file_path)
              target = select_by_path(tree, id=item.id)
              core.delete_node(target)
              imports = core.apply_imports(file_path, tree, item.add_import, item.remove_import)
              core.CACHE.save(file_path, tree)
              return DeleteResult(path=item.path, id=item.id, result='success', imports=imports)
      - id: DeleteTool
        type: ClassDef
        signature: "class DeleteTool(ToolDefinition):"
        children:
        - id: DeleteTool.gLGwBi|JNohfS
          type: statements
          code: |-
            name = 'ast_delete'
            title = 'Delete AST nodes or files'
            description = 'Delete selected nodes from files, or whole files if no selector is given, for a batch of items.'
            _ITEM_PROPERTIES = {'path': PATH_PROP, **PATH_SELECTOR_PROPS, **IMPORT_PROPS}
            _ITEM_REQUIRED = ['path']
            _ITEMS_DESCRIPTION = 'Nodes/files to delete.'
            input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)
        - id: DeleteTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult:
                """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""

                def item_factory(it: dict[str, Any]) -> DeleteItem:
                    add_import, remove_import = parse_import_args(it)
                    return DeleteItem(path=it['path'], id=it.get('id'), add_import=add_import, remove_import=remove_import)

                def result_serializer(r: DeleteResult) -> dict[str, Any]:
                    entry = {'path': r.path, 'result': r.result}
                    if r.id is not None:
                        entry['id'] = r.id
                    if (imports := import_entry(r.imports)) is not None:
                        entry['imports'] = imports
                    return entry
                return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, result_serializer, auto_approve=True)
```
/answer 0a99-a9-a8 allow
Text:
 Die Implementierung ist vollständig. Jetzt schreibe ich Tests im Stil der vorhandenen. Dafür sehe ich mir kurz an, wie `tests/test_ast_engines.py` aufgebaut ist.
ReasoningToken: 355
Control Request:
```yaml
id: 0a99-aa-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_engines.py
  reason: "Aufbau, Fixtures und Stil der bestehenden AST-Tests erfassen"
```
/answer 0a99-aa-0 allow
Control Request:
```yaml
id: 0a99-ab-aa
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_engines.py
      nodes:
      - id: yGxCcZ|eOnjml
        signature: '''Tests for the multi-engine ``ast_*`` tool family.\n\nCovers engine selection b…'
      - id: 0ekC7u|9IXYUc
        signature: from __future__ import annotations
      - id: Rpzfkv|9iKcDL
        signature: '''# Make the ``src`` layout importable without requiring an editable install.'''
      - id: 8Pl1k7|GddZCa
        signature: from xy.ai.mcpc.tools.ast import core
      - id: 3Mpw8b|t0uVQE
        signature: "'# noqa: E402'"
      - id: Jod45v|byUhT4
        signature: "from xy.ai.mcpc.tools.ast.create import CreateFileItem, ast_create"
      - id: 3Mpw8b|t0uVQE_1
        signature: "'# noqa: E402'"
      - id: NREYuD|nugLQv
        signature: "from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete"
      - id: 3Mpw8b|t0uVQE_2
        signature: "'# noqa: E402'"
      - id: GuxMWv|egumyg
        signature: "from xy.ai.mcpc.tools.ast.edit_marks import EditMarksItem, ast_edit_marks"
      - id: 3Mpw8b|t0uVQE_3
        signature: "'# noqa: E402'"
      - id: LgnMR1|79MOp3
        signature: from xy.ai.mcpc.tools.ast.find import ast_find
      - id: 3Mpw8b|t0uVQE_4
        signature: "'# noqa: E402'"
      - id: cNhh2C|88ZZ6p
        signature: "from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert"
      - id: 3Mpw8b|t0uVQE_5
        signature: "'# noqa: E402'"
      - id: JD7CYF|Tclncr
        signature: from xy.ai.mcpc.tools.ast.outline import ast_outline
      - id: 3Mpw8b|t0uVQE_6
        signature: "'# noqa: E402'"
      - id: oEgkx1|6grABN
        signature: "from xy.ai.mcpc.tools.ast.read import ReadItem, ast_read"
      - id: 3Mpw8b|t0uVQE_7
        signature: "'# noqa: E402'"
      - id: mO2aS8|5v2aZQ
        signature: "from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace"
      - id: 3Mpw8b|t0uVQE_8
        signature: "'# noqa: E402'"
      - id: Txu2YL|gFjfI4
        signature: from xy.ai.mcpc.tools.ast.validate import ast_validate
      - id: jS0K2v|hUR3iy
        signature: PY_SOURCE = 'import os\n\nclass A:\n    def foo(self):\n        x = 1\n        …
      - id: py_file
        signature: "@pytest.fixture def py_file(tmp_path: Path) -> str:"
      - id: json_file
        signature: "@pytest.fixture def json_file(tmp_path: Path) -> str:"
      - id: iz0N40|zFMezT
        signature: '''# --------------------------------------------------------------------------- …'
      - id: test_engine_selection_by_extension
        signature: "def test_engine_selection_by_extension(tmp_path: Path):"
      - id: h766ca315
        signature: "def test_engine_selection_unsupported_extension(tmp_path: Path):"
        docstring: Unknown extensions no longer raise; they fall back to the whole-file plain-text…
      - id: test_snippet_defaults_to_python
        signature: "def test_snippet_defaults_to_python():"
      - id: iz0N40|tC4SaG
        signature: '''# --------------------------------------------------------------------------- …'
      - id: test_python_outline
        signature: "def test_python_outline(py_file):"
      - id: test_python_find_by_name_and_type
        signature: "def test_python_find_by_name_and_type(py_file):"
      - id: test_python_read_returns_source
        signature: "def test_python_read_returns_source(py_file):"
      - id: test_python_full_crud_roundtrip
        signature: "def test_python_full_crud_roundtrip(py_file):"
      - id: iz0N40|icJAmV
        signature: '''# --------------------------------------------------------------------------- …'
      - id: test_generic_uses_treesitter_engine
        signature: "def test_generic_uses_treesitter_engine(json_file):"
      - id: test_generic_qualified_names
        signature: "def test_generic_qualified_names(json_file):"
      - id: test_generic_replace_by_qualified_name
        signature: "def test_generic_replace_by_qualified_name(json_file):"
      - id: test_generic_edit_between_markers
        signature: "def test_generic_edit_between_markers(json_file):"
      - id: test_generic_validate_reports_error
        signature: "def test_generic_validate_reports_error(tmp_path: Path):"
      - id: iz0N40|KdNRVD
        signature: '''# --------------------------------------------------------------------------- …'
      - id: hf5a87d78
        signature: "def test_addressing_by_id_matches_located_node(json_file):"
      - id: test_addressing_by_id_is_parser_agnostic
        signature: "def test_addressing_by_id_is_parser_agnostic(py_file):"
      - id: test_list_filters_by_type
        signature: "def test_list_filters_by_type(py_file):"
      - id: iz0N40|gINtIU
        signature: '''# --------------------------------------------------------------------------- …'
      - id: h824e2be2
        signature: "def test_cache_returns_same_tree_until_changed(py_file):"
      - id: test_cache_is_shared_across_engines
        signature: "def test_cache_is_shared_across_engines(py_file, json_file):"
      - id: iz0N40|SjpxQG
        signature: '''# --------------------------------------------------------------------------- …'
      - id: test_create_file_typescript
        signature: "def test_create_file_typescript(tmp_path: Path):"
```
/answer 0a99-ab-aa allow
Control Request:
```yaml
id: 0a99-ac-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_engines.py
    ids:
    - Rpzfkv|9iKcDL
    - py_file
    - test_python_full_crud_roundtrip
  reason: "Pfad-Setup, Fixture und einen typischen Test als Vorlage lesen"
```
/answer 0a99-ac-0 allow
Control Request:
```yaml
id: 0a99-ad-ac
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_engines.py
      nodes:
      - id: Rpzfkv|9iKcDL
        type: statements
        code: |-
          '# Make the ``src`` layout importable without requiring an editable install.'
          _SRC = Path(__file__).resolve().parents[1] / 'src'
          if _SRC.is_dir() and str(_SRC) not in sys.path:
              sys.path.insert(0, str(_SRC))
          '# noqa: E402'
      - id: py_file
        type: FunctionDef
        code: |-
          @pytest.fixture
          def py_file(tmp_path: Path) -> str:
              path = tmp_path / 'm.py'
              path.write_text(PY_SOURCE, encoding='utf-8')
              return str(path)
      - id: test_python_full_crud_roundtrip
        type: FunctionDef
        code: |-
          def test_python_full_crud_roundtrip(py_file):
              ast_replace(items=[ReplaceItem(path=py_file, source='def bar():\n    return 42', id='bar')])
              ast_edit_marks(
                  items=[
                      EditMarksItem(
                          path=py_file,
                          start_marker='x = 1',
                          end_marker='return x',
                          content='return 99',
                          id='A.foo')])
              ast_insert(items=[InsertItem(path=py_file, source='z = 5', id='bar', position='after')])
              import_id = ast_find(paths=[py_file], node_type='imports').files[0].nodes[0].id
              ast_delete(items=[DeleteItem(path=py_file, id=import_id)])
              assert ast_validate([py_file]).all_ok
              text = Path(py_file).read_text()
              assert 'return 42' in text
              assert 'return 99' in text
              assert 'z = 5' in text
              assert 'import os' not in text
```
/answer 0a99-ad-ac allow
Text:
 Zuerst noch eine kleine Korrektur: Ohne bestehende Imports sollte hinter dem Prolog keine doppelte Leerzeile entstehen.
Control Request:
```yaml
id: 0a99-ae-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: apply_treesitter
    old_text: |2-
                  if data[pos:].strip():
                      text += b'\n'
    new_text: |2-
                  if data[pos:].strip() and not data[pos:].startswith((b'\n', b'\r\n')):
                      text += b'\n'
  reason: Doppelte Leerzeile beim Einfügen ohne vorhandene Imports vermeiden
```
/answer 0a99-ae-0 allow
ReasoningToken: 1969
Control Request:
```yaml
id: 0a99-af-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    source: |
      """Tests for ``addImport``/``removeImport`` handling of the node-editing ``ast_*`` tools."""
      from __future__ import annotations

      import sys
      from pathlib import Path

      import pytest

      _SRC = Path(__file__).resolve().parents[1] / 'src'
      if _SRC.is_dir() and str(_SRC) not in sys.path:
          sys.path.insert(0, str(_SRC))

      from xy.ai.mcpc.tools.ast import core  # noqa: E402
      from xy.ai.mcpc.tools.ast.delete import DeleteItem, ast_delete  # noqa: E402
      from xy.ai.mcpc.tools.ast.find import ast_find  # noqa: E402
      from xy.ai.mcpc.tools.ast.imports import IMPORT_NOTICE, normalize_import, statement_spans  # noqa: E402
      from xy.ai.mcpc.tools.ast.insert import InsertItem, ast_insert  # noqa: E402
      from xy.ai.mcpc.tools.ast.replace import ReplaceItem, ast_replace  # noqa: E402

      PY_SOURCE = '"""Doc."""\nimport os\n\n\nclass A:\n    pass\n'
      JS_SOURCE = "import a from 'a';\nimport {b} from 'b';\n\nfunction f() {\n  return 1;\n}\n"
      GO_SOURCE = 'package main\n\nimport "fmt"\n\nfunc main() {\n\tfmt.Println(1)\n}\n'


      def _write(tmp_path: Path, name: str, source: str) -> str:
          path = tmp_path / name
          path.write_text(source, encoding='utf-8')
          return str(path)


      def _first_id(path: str, node_type: str) -> str:
          return ast_find(paths=[path], node_type=node_type).files[0].nodes[0].id


      # --------------------------------------------------------------------------- helpers

      def test_normalize_import_ignores_whitespace_quotes_and_semicolon():
          assert normalize_import("import  { a ,b }  from \"x\" ;") == normalize_import("import {a, b} from 'x'")
          assert normalize_import('use a::{b, c,};') == normalize_import('use a::{b,c};')


      def test_statement_spans_skip_comments_and_braces():
          data = b'// x; y\nuse a::{b, c};\n/* z; */ use d;\n'
          assert [data[s:e] for s, e in statement_spans(data)] == [b'use a::{b, c};', b'use d;']


      # --------------------------------------------------------------------------- python

      def test_python_adds_missing_imports_after_existing_ones(tmp_path: Path):
          path = _write(tmp_path, 'm.py', PY_SOURCE)
          result = ast_insert(items=[InsertItem(
              path=path, source='z = 5', id='A', add_import=['import sys', 'import os', 'from os import path'])])
          assert not result.errors and result.notice is None
          assert result.results[0].imports.added == ['import sys', 'from os import path']
          text = Path(path).read_text()
          assert text.index('import os') < text.index('import sys') < text.index('from os import path') < text.index('class A')
          assert text.startswith('"""Doc."""')


      def test_python_add_is_idempotent(tmp_path: Path):
          path = _write(tmp_path, 'm.py', PY_SOURCE)
          for _ in range(2):
              ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['import sys'])])
          assert Path(path).read_text().count('import sys') == 1


      def test_python_remove_import_and_single_alias(tmp_path: Path):
          path = _write(tmp_path, 'm.py', '"""Doc."""\nimport os, sys\nfrom a import b, c\n\n\nclass A:\n    pass\n')
          result = ast_insert(items=[InsertItem(
              path=path, source='z = 5', id='A', remove_import=['import sys', 'from a import c', 'import missing'])])
          assert result.results[0].imports.removed == ['import sys', 'from a import c']
          text = Path(path).read_text()
          assert 'import os' in text and 'from a import b' in text
          assert 'sys' not in text and 'import c' not in text


      def test_python_future_import_goes_first(tmp_path: Path):
          path = _write(tmp_path, 'm.py', PY_SOURCE)
          ast_insert(items=[InsertItem(
              path=path, source='z = 5', id='A', add_import=['from __future__ import annotations'])])
          text = Path(path).read_text()
          assert text.index('from __future__') < text.index('import os')


      def test_python_invalid_import_fails_without_writing(tmp_path: Path):
          path = _write(tmp_path, 'm.py', PY_SOURCE)
          result = ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['x = 1'])])
          assert len(result.errors) == 1 and 'Not a single import' in result.errors[0].error
          assert 'z = 5' not in Path(path).read_text()
          assert 'z = 5' not in core.CACHE.get_tree(Path(path)).source


      def test_delete_with_imports_removes_unused_import(tmp_path: Path):
          path = _write(tmp_path, 'm.py', PY_SOURCE)
          result = ast_delete(items=[DeleteItem(path=path, id='A', remove_import=['import os'])])
          assert not result.errors
          assert 'import os' not in Path(path).read_text()


      def test_delete_file_rejects_imports(tmp_path: Path):
          path = _write(tmp_path, 'm.py', PY_SOURCE)
          result = ast_delete(items=[DeleteItem(path=path, add_import=['import sys'])])
          assert len(result.errors) == 1
          assert Path(path).exists()


      # --------------------------------------------------------------------------- javascript

      def test_javascript_add_and_remove(tmp_path: Path):
          path = _write(tmp_path, 'm.js', JS_SOURCE)
          node = _first_id(path, 'function_declaration')
          result = ast_replace(items=[ReplaceItem(
              path=path, source='function f() {\n  return 2;\n}', id=node,
              add_import=['import { c } from "c";', 'import a from "a"'],
              remove_import=["import {b} from 'b';"])])
          assert not result.errors and result.notice is None
          assert result.results[0].imports.added == ['import { c } from "c";']
          assert result.results[0].imports.removed == ["import {b} from 'b';"]
          text = Path(path).read_text()
          assert text.startswith("import a from 'a';\nimport { c } from \"c\";\n")
          assert 'return 2' in text and "from 'b'" not in text


      def test_javascript_without_imports_inserts_after_directive(tmp_path: Path):
          path = _write(tmp_path, 'm.js', "'use strict';\n\nfunction f() {\n  return 1;\n}\n")
          node = _first_id(path, 'function_declaration')
          ast_replace(items=[ReplaceItem(
              path=path, source='function f() {\n  return 2;\n}', id=node, add_import=["import a from 'a';"])])
          text = Path(path).read_text()
          assert text.index('use strict') < text.index('import a') < text.index('function f')


      def test_javascript_rejects_non_import(tmp_path: Path):
          path = _write(tmp_path, 'm.js', JS_SOURCE)
          node = _first_id(path, 'function_declaration')
          result = ast_replace(items=[ReplaceItem(
              path=path, source='function f() {}', id=node, add_import=['const a = 1;'])])
          assert len(result.errors) == 1
          assert Path(path).read_text() == JS_SOURCE


      # --------------------------------------------------------------------------- generic fallback

      def test_generic_present_import_needs_no_notice(tmp_path: Path):
          path = _write(tmp_path, 'm.go', GO_SOURCE)
          node = core.locate_all(core.load(path)[1])[0]
          result = ast_replace(items=[ReplaceItem(
              path=path, source=node.node.text.decode(), id=node.node_id, add_import=['import "fmt"'])])
          assert not result.errors and result.notice is None


      def test_generic_missing_import_is_not_written_but_flagged(tmp_path: Path):
          path = _write(tmp_path, 'm.go', GO_SOURCE)
          node = core.locate_all(core.load(path)[1])[0]
          result = ast_replace(items=[ReplaceItem(
              path=path, source=node.node.text.decode(), id=node.node_id, add_import=['import "os"'])])
          assert not result.errors
          assert result.notice == IMPORT_NOTICE
          assert 'import "os"' not in Path(path).read_text()


      def test_generic_removes_whole_line_only(tmp_path: Path):
          path = _write(tmp_path, 'm.go', GO_SOURCE)
          node = core.locate_all(core.load(path)[1])[0]
          result = ast_replace(items=[ReplaceItem(
              path=path, source=node.node.text.decode(), id=node.node_id, remove_import=['import "fmt"'])])
          assert not result.errors and result.notice is None
          assert 'import "fmt"' not in Path(path).read_text()


      def test_generic_unsafe_remove_is_flagged(tmp_path: Path):
          path = _write(tmp_path, 'm.go', GO_SOURCE)
          node = core.locate_all(core.load(path)[1])[0]
          result = ast_replace(items=[ReplaceItem(
              path=path, source=node.node.text.decode(), id=node.node_id, remove_import=['fmt.Println'])])
          assert result.notice == IMPORT_NOTICE
          assert 'fmt.Println(1)' in Path(path).read_text()


      @pytest.mark.parametrize('bad', [[''], 'import os', [1]])
      def test_schema_level_validation_rejects_malformed_values(bad):
          from xy.ai.mcpc.tools.ast.imports import parse_import_args
          with pytest.raises(ValueError):
              parse_import_args({'addImport': bad})
  reason: "Tests für Python-, JS- und generisches Import-Handling anlegen"
```
/answer 0a99-af-0 allow
Text:
 Die Tests sind angelegt. Ich versuche, sie auszuführen. Das ist eine Ausführung und kein Dateizugriff, den die AST-Tools übernehmen könnten.
ReasoningToken: 650
Control Request:
```yaml
id: 0a99-b0-0
toolName: python
arguments:
  script: |-
    import subprocess, sys
    r = subprocess.run([sys.executable, "-I", "-m", "pytest", "-q", "-x", "--no-header", "-p", "no:cacheprovider",
                        "/home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py",
                        "/home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_engines.py"],
                       cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=600)
    print(r.stdout[-6000:]); print(r.stderr[-2000:]); print("exit", r.returncode)
  reason: "Neue Import-Tests und die bestehenden AST-Engine-Tests ausführen (reiner pytest-Lauf, keine Dateien lesen/ändern)"
```
/answer 0a99-b0-0 allow
Control Request:
```yaml
id: 0a99-b1-b0
result:
  structuredContent:
    exit_code: 0
    stdout: |2

      /usr/bin/python3: No module named pytest

      exit 1
```
/answer 0a99-b1-b0 allow
Text:
 Das System-Python hat kein `pytest`. Ich suche das Projekt-Venv.
ReasoningToken: 47
Control Request:
```yaml
id: 0a99-b2-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc
    pattern: "(^|/)(\\.venv|venv)/bin/python[0-9.]*$"
    recursive: true
  - path: /home/user/xyan/xy.ai.workbench
    pattern: "(^|/)(\\.venv|venv)/bin/python[0-9.]*$"
    recursive: false
  reason: Python-Interpreter des Projekt-Venv mit installiertem pytest finden
```
/answer 0a99-b2-0 deny kein venv vorhanden
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 310472, in: 92, out: 82742, reason: 239, read: 4411402, write: 227638, start: 1794264881356, end: 1794264895716, time: 14278, total: 576848, io: 0, control: 82, sent: 0, recv: 323
ReasoningToken: 239
Die Import-Unterstützung ist eingebaut, aber ich konnte sie nicht ausführen. Alle geänderten Dateien kompilieren (`ast_validate`). `pytest` ist im System-Python nicht installiert und es gibt kein Venv, deshalb sind die neuen Tests in `tests/test_ast_imports.py` ungelaufen. Bitte lass einmal `pytest tests/test_ast_imports.py tests/test_ast_engines.py` laufen. Auch ob die Tests mit Python, JS und Go in den Randfällen stimmen, ist ungeprüft.

## Was jetzt gilt

- **Felder:** `replace`, `insert`, `delete`, `edit_block` und `edit_marks` haben die Listen `addImport` und `removeImport`. `ast_create` ist wie besprochen ausgenommen. Die Schema-Beschreibung (`IMPORT_PROPS` in `common.py`) ist sprachneutral.
- **Reihenfolge pro Batch-Item:** Zuerst läuft der Node-Edit und die neue ID wird berechnet. Danach werden die Imports angewendet und die Datei gespeichert. `new_id` und `ids` stimmen also noch.
- **Fehlerfall:** Schlägt der Import-Teil fehl, wird der Cache verworfen. Die Meldung lautet „Import handling failed, nothing was written“, und der Node-Edit ist nicht auf der Platte.
- **Ergebnis:** Im Item-Ergebnis steht `imports: {added, removed}`, wenn sich etwas geändert hat. Konnte etwas nicht geprüft werden, steht einmal pro Batch ein generisches Feld `notice`. Dafür habe ich `serialize_batch_result` in `_tool_helpers.py` um dieses Feld erweitert.
- **`ast_delete` ohne `id`:** Löscht die ganze Datei und lehnt Import-Felder mit Fehler ab.

## Verhalten je Engine

| Engine | `addImport` | `removeImport` |
|---|---|---|
| Python | Semantisch über das `ast`. Es wird hinter dem letzten Import eingefügt, sonst hinter dem Docstring. `__future__`-Imports kommen an den Anfang. | Entfernt einzelne Aliasse. Das Statement verschwindet, wenn es leer wird. |
| JavaScript | Der Text wird geparst und muss genau ein `import_statement` sein. Quotes, Whitespace und `;` sind beim Vergleich egal. Eingefügt wird hinter dem letzten Import, sonst hinter Shebang, `'use strict'` und Header-Kommentar. | Entfernt die Zeile des Import-Knotens. |
| Java und Rust (RPC) | Eingefügt wird über die Engine, danach wird neu lokalisiert und geprüft, dass der Prozess es als Import erkannt hat. | Löscht den Statement-Bereich innerhalb der Import-Gruppe. Das Ergebnis wird mit `validate` geprüft. |
| Alles andere (generisch) | Es wird nur geprüft. Fehlt der Import, wird nichts geschrieben, und es gibt die `notice`. | Eine Zeile, die genau passt, wird entfernt. Kommt der Text woanders vor, passiert nichts und es gibt die `notice`. Kommt er gar nicht vor, passiert nichts. |

## Was du wissen solltest

- **Java/Rust-Pfad:** Er stützt sich darauf, dass beide Prozesse Import-Gruppen als `node_type == 'imports'` melden. Das habe ich im Quelltext von `NodeLocator.java` und `node_locator.rs` gesehen, aber nicht gegen einen laufenden Prozess geprüft. Wie `insert` auf eine Import-Gruppe reagiert, ist offen. Ein Fehlschlag würde als Fehler gemeldet und nicht still durchgehen.
- **Rust:** Verglichen wird nur der normalisierte Text. `use a::b;` und `use a::{b, c};` zählen also nicht als dasselbe.
- **TypeScript, TSX und CSS:** Sie laufen über die generische Basis. CSS hat zwar `_IMPORT_TYPES`, aber keinen Override. Nachrüsten wäre bei TypeScript und CSS je ein Einzeiler wie bei JavaScript.
- **`generic/_java.py`:** Sie wirkt ungenutzt, weil `.java` über die RPC-Engine läuft. Ich habe sie deshalb nicht angefasst.

Neu sind `ast/imports.py` und `ast/python/_imports.py`.
Pytest ist installiert
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Dann lag es vermutlich an meinem `-I`-Aufruf, denn der Isolated-Modus blendet das User-Site-Verzeichnis aus. Ich starte den Lauf noch einmal ohne `-I`.
ReasoningToken: 151
Control Request:
```yaml
id: 0a99-b3-0
toolName: python
arguments:
  script: |-
    import shutil, subprocess, sys
    print("pytest bin:", shutil.which("pytest"))
    cmd = [shutil.which("pytest")] if shutil.which("pytest") else [sys.executable, "-m", "pytest"]
    r = subprocess.run(cmd + ["-q", "--no-header", "-p", "no:cacheprovider",
                        "tests/test_ast_imports.py", "tests/test_ast_engines.py"],
                       cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=900)
    print(r.stdout[-8000:]); print(r.stderr[-2000:]); print("exit", r.returncode)
  reason: Neue Import-Tests und bestehende AST-Engine-Tests mit dem installierten pytest ausführen
```
/answer 0a99-b3-0 allow
Control Request:
```yaml
id: 0a99-b4-b3
result:
  structuredContent:
    exit_code: 0
    stdout: |
      pytest bin: /home/user/.local/bin/pytest
      .....................................                                    [100%]
      37 passed in 0.15s


      exit 0
```
/answer 0a99-b4-b3 allow
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 314108, in: 96, out: 83761, reason: 64, read: 4745787, write: 230251, start: 1794264936296, end: 1794264940828, time: 4478, total: 588226, io: 0, control: 54, sent: 0, recv: 94
ReasoningToken: 64
Alle 37 Tests sind grün, die neuen aus `test_ast_imports.py` und die bestehenden aus `test_ast_engines.py`. Mein vorheriger Fehlschlag lag am `-I`-Flag, das `~/.local/bin/pytest` ausgeblendet hat.

Damit sind Python, JavaScript und die generische Basis (am Beispiel Go) verifiziert. Das umfasst Hinzufügen, Idempotenz, Entfernen, Fehlerfall ohne Schreiben, den Batch-Hinweis `notice` und die Ablehnung bei Datei-Löschung.

Ungetestet bleibt der RPC-Pfad für Java und Rust. Dafür bräuchte es die laufenden Prozesse (`ast-java` und `xy_ai_ast_rust`). Wenn du sie startbereit hast, schreibe ich Integrationstests dafür. Sie würden vor allem klären, wie `insert` auf eine Import-Gruppe reagiert und wo ohne vorhandene Imports eingefügt wird.
Ich sehe keinen Grund eine Operation scheitern zu lassen nur weil der Import Teil nicht klappt. Genau für den Fall ist die Notice da. Darüber Hinaus sind Source und die import Parameter unabhängige Komponenten.
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Das ist ein berechtigter Einwand. Ein Import-Problem darf den Node-Edit nicht verwerfen, dafür gibt es die `notice`. Ich ändere `core.apply_imports` so, dass Fehler abgefangen und als „ungeprüft“ gemeldet werden, mit Rollback des Import-Teils. Außerdem lehnt `ast_delete` ohne `id` die Import-Felder nicht mehr ab, sondern löscht die Datei und meldet die `notice`.
Control Request:
```yaml
id: 0a99-b5-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  name: apply_imports
  reason: Aktuelle Fassung von core.apply_imports und Datei-Header für logging prüfen
```
/answer 0a99-b5-0 allow
Control Request:
```yaml
id: 0a99-b6-b5
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
      - id: apply_imports
        code: |-
          def apply_imports(path: Path, tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:
              """Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``.

              On failure the cached tree is dropped, so the item's unsaved node edit is not leaked.
              """
              if not add and (not remove):
                  return None
              try:
                  return tree.engine.apply_imports(tree, add or [], remove or [])
              except AstError as exc:
                  CACHE.invalidate(path)
                  raise AstError(f'Import handling failed, nothing was written: {exc}') from exc
```
/answer 0a99-b6-b5 allow
ReasoningToken: 1828
Control Request:
```yaml
id: 0a99-b7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: ^import (hashlib|logging)|^logger\b
    include:
    - core.py
    exclude:
    - '**/__pycache__/**'
    intent: editing
  reason: "Prüfen, ob core.py bereits einen Logger hat"
```
/answer 0a99-b7-0 allow
Control Request:
```yaml
id: 0a99-b8-b7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: core.py
        nodes:
        - id: CXxOCc|vQxBdo
          matches:
          - import hashlib
```
/answer 0a99-b8-b7 allow
ReasoningToken: 208
Control Request:
```yaml
id: 0a99-b9-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: apply_imports
    source: |-
      def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:
          """Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``.

          Independent of the node edit: if the import part fails, the engine's partial
          changes to the import region are rolled back, the node edit stays, and the
          outcome is flagged ``unchecked`` so the batch carries the notice.
          """
          if not add and not remove:
              return None
          source, raw = tree.source, tree.raw
          try:
              return tree.engine.apply_imports(tree, add or [], remove or [])
          except Exception:  # noqa: BLE001 - never fail the edit because of its imports
              logger.warning('Import handling failed for %s', tree.path, exc_info=True)
              tree.source, tree.raw = source, raw
              return ImportOutcome(unchecked=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: CXxOCc|vQxBdo
    source: |-
      import hashlib
      import logging
  reason: core.apply_imports auf nicht-fatales Verhalten mit Rollback und Notice umstellen
```
/answer 0a99-b9-0 allow
Text:
 Ich habe dort eine ganze Import-Gruppe ersetzt. Ich kontrolliere, ob dabei etwas verloren gegangen ist.
ReasoningToken: 189
Control Request:
```yaml
id: 0a99-ba-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  node_type: imports
  reason: Import-Block von core.py nach der Ersetzung kontrollieren
```
/answer 0a99-ba-0 allow
Control Request:
```yaml
id: 0a99-bb-ba
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
      - id: Vx2JFF|mvdSSl
        type: imports
        code: |-
          import hashlib
          import logging
```
/answer 0a99-bb-ba allow
ReasoningToken: 198
Control Request:
```yaml
id: 0a99-bc-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  lineno: 1
  end_lineno: 25
  reason: Kopf von core.py ansehen
```
/answer 0a99-bc-0 allow
Control Request:
```yaml
id: 0a99-bd-bc
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
      - id: V6fdSR|z5Q7Tq
        code: "'Facade the ``ast_*`` tools call, dispatching to a per-file engine.\\n\\nThe engine is chosen by file extension: Python files use the ``ast``-based\\n:mod:`.python` engine, everything else the generic tree-sitter :mod:`.generic`\\nengine, or - for extensions no grammar covers - its whole-file\\n``PlainTextEngine`` fallback. Snippets passed as raw ``code`` (no path)\\ndefault to Python.\\n\\nA single content-hash validated :class:`AstCache` – reused across engines –\\nholds parsed :class:`~.base.Tree` objects keyed by absolute path and validated\\non every access by ``st_mtime_ns`` and, on change, by a content hash.\\n\\nThe engine-agnostic model (``AstError``, ``Located``, ``OutlineNode`` …) is\\nre-exported here so the tools only ever import :mod:`.core`.\\n'"
      - id: Vx2JFF|mvdSSl
        code: |-
          import hashlib
          import logging
      - id: 85RFV8|RBFWwR
        code: "_PYTHON_EXTENSIONS = ('.py', '.pyi')"
      - id: engine_for_path.8cr5Dx|aR1Rpj
        code: |-
          'Return the engine responsible for ``path`` based on its extension.'
          ext = path.suffix.lower()
          if ext in _PYTHON_EXTENSIONS:
              return python.ENGINE
          symbol = generic.language_for_extension(ext)
          if symbol is None:
              return generic.fallback_engine()
          return generic.get_engine(symbol)
```
/answer 0a99-bd-bc allow
Thinking:
Die übrigen Imports von `core.py` sind dabei verloren gegangen, weil ich eine ganze Gruppe ersetzt habe, obwohl die Suche nur einen Teiltreffer geliefert hatte. Ich stelle den Block jetzt anhand des zuvor gelesenen Originals wieder her.


Control Request:
```yaml
id: 0a99-be-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: Vx2JFF|mvdSSl
    source: |-
      from __future__ import annotations
      import hashlib
      import logging
      import threading
      from dataclasses import dataclass
      from pathlib import Path
      from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, AstTextError, Engine, Located, OutlineNode, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, resolve_by_prefix, to_dict
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome
      from xy.ai.mcpc.tools.ast import generic
      from xy.ai.mcpc.tools.ast import python

      logger = logging.getLogger(__name__)
  reason: Versehentlich überschriebenen Import-Block von core.py wiederherstellen (plus logging/logger)
```
/answer 0a99-be-0 allow
ReasoningToken: 577
Control Request:
```yaml
id: 0a99-bf-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: core\.(CACHE|AstError|Located|OutlineNode|AstAmbiguous|AstTextError|Engine|Tree)|from xy.ai.mcpc.tools.ast.core import
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    - '**/node_modules/**'
    - '**/.git/**'
    limit: 5
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: "\\bcore\\.[A-Za-z_]+"
    include:
    - core.py
    exclude:
    - '**/__pycache__/**'
    limit: 5
  reason: "Sicherstellen, dass keine weiteren Namen aus dem ursprünglichen core-Header fehlen"
```
/answer 0a99-bf-0 allow
Control Request:
```yaml
id: 0a99-c0-bf
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
      matches:
      - path: tests/test_ast_engines.py
        matches:
        - first = core.CACHE.get_tree(path)
        - assert core.CACHE.get_tree(path) is first
        - core.CACHE.invalidate(path)
        - assert core.CACHE.get_tree(path) is not first
        - assert core.CACHE.get_tree(Path(py_file)) is py_tree
      warning: "Limit of 5 matches reached (total 99); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
```
/answer 0a99-c0-bf allow
Text:
 Nun die Aufrufer anpassen (Signatur ohne `path`) und den Datei-Löschpfad in `delete.py` auf „löschen und `notice`“ umstellen.
ReasoningToken: 82
Control Request:
```yaml
id: 0a99-c1-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/replace.py
    id: replace_one
    old_text: "core.apply_imports(file_path, tree,"
    new_text: "core.apply_imports(tree,"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/insert.py
    id: insert_one
    old_text: "core.apply_imports(file_path, tree,"
    new_text: "core.apply_imports(tree,"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py
    id: edit_block_one
    old_text: "core.apply_imports(file_path, tree,"
    new_text: "core.apply_imports(tree,"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_marks.py
    id: edit_marks_one
    old_text: "core.apply_imports(file_path, tree,"
    new_text: "core.apply_imports(tree,"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: delete_one
    old_text: "core.apply_imports(file_path, tree,"
    new_text: "core.apply_imports(tree,"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: delete_one
    old_text: |2
              if item.add_import or item.remove_import:
                  raise core.AstError(
                      "'addImport'/'removeImport' need a node 'id'; they cannot be used when deleting a file.")
    new_text: ""
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: delete_one
    old_text: "        return DeleteResult(path=item.path, id=None, result='success')"
    new_text: |2-
              unchecked = ImportOutcome(unchecked=True) if item.add_import or item.remove_import else None
              return DeleteResult(path=item.path, id=None, result='success', imports=unchecked)
  reason: Aufrufer von core.apply_imports anpassen und Datei-Löschen ohne Fehler bei Import-Feldern
```
/answer 0a99-c1-0 allow
Text:
 Jetzt passe ich die drei Tests an, die das alte Fehlverhalten erwartet haben, und ergänze einen Rollback-Test.
ReasoningToken: 484
Control Request:
```yaml
id: 0a99-c2-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: test_python_invalid_import_fails_without_writing
    source: |-
      def test_python_invalid_import_keeps_edit_and_flags_notice(tmp_path: Path):
          path = _write(tmp_path, 'm.py', PY_SOURCE)
          result = ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['x = 1'])])
          assert not result.errors
          assert result.notice == IMPORT_NOTICE
          text = Path(path).read_text()
          assert 'z = 5' in text and 'x = 1' not in text
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: test_delete_file_rejects_imports
    source: |-
      def test_delete_file_ignores_imports_but_flags_notice(tmp_path: Path):
          path = _write(tmp_path, 'm.py', PY_SOURCE)
          result = ast_delete(items=[DeleteItem(path=path, add_import=['import sys'])])
          assert not result.errors
          assert result.notice == IMPORT_NOTICE
          assert not Path(path).exists()
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: test_javascript_rejects_non_import
    source: |-
      def test_javascript_non_import_keeps_edit_and_flags_notice(tmp_path: Path):
          path = _write(tmp_path, 'm.js', JS_SOURCE)
          node = _first_id(path, 'function_declaration')
          result = ast_replace(items=[ReplaceItem(
              path=path, source='function f() {}', id=node, add_import=['const a = 1;'])])
          assert not result.errors
          assert result.notice == IMPORT_NOTICE
          text = Path(path).read_text()
          assert 'function f() {}' in text and 'const a' not in text


      def test_failing_engine_rolls_back_imports_only(tmp_path: Path, monkeypatch):
          path = _write(tmp_path, 'm.js', JS_SOURCE)
          node = _first_id(path, 'function_declaration')
          engine = core.load(path)[1].engine

          def broken(tree, add, remove):
              tree.source = tree.source + '\nGARBAGE'
              raise RuntimeError('boom')

          monkeypatch.setattr(engine, 'apply_imports', broken)
          result = ast_replace(items=[ReplaceItem(
              path=path, source='function f() {\n  return 2;\n}', id=node, add_import=["import c from 'c';"])])
          assert not result.errors
          assert result.notice == IMPORT_NOTICE
          text = Path(path).read_text()
          assert 'return 2' in text and 'GARBAGE' not in text
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    id: DeleteItem
    source: |-
      @dataclass(frozen=True)
      class DeleteItem:
          """One node (or file) to delete.

          Attributes:
              path: Absolute path to the file to modify.
              id: Unique id of the target node. Omit to delete the whole file.
              add_import: Import statements to add if missing, applied after the delete.
              remove_import: Import statements to remove if present, applied after the delete.
                  Both are ignored (and flagged in the batch notice) when a whole file is deleted.
          """
          path: str
          id: str | None = None
          add_import: list[str] | None = None
          remove_import: list[str] | None = None
  reason: "Tests an das neue nicht-fatale Import-Verhalten anpassen, Rollback-Test ergänzen, DeleteItem-Doku korrigieren"
```
/answer 0a99-c2-0 allow
Control Request:
```yaml
id: 0a99-c3-c2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
      result: success
      id: test_delete_file_rejects_imports
      new_id: test_delete_file_rejects_imports
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
      result: success
      id: test_javascript_rejects_non_import
      new_id: test_javascript_rejects_non_import
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      result: success
      id: DeleteItem
      new_id: DeleteItem
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: test_python_invalid_import_fails_without_writing
```
/answer 0a99-c3-c2 allow
Control Request:
```yaml
id: 0a99-c4-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
  regexp: invalid_import
  node_type: FunctionDef
  reason: ID des noch nicht angepassten Tests finden
```
/answer 0a99-c4-0 allow
ReasoningToken: 132
Control Request:
```yaml
id: 0a99-c5-c4
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
      nodes:
      - id: hec242bb5
        type: FunctionDef
        code: |-
          def test_python_invalid_import_fails_without_writing(tmp_path: Path):
              path = _write(tmp_path, 'm.py', PY_SOURCE)
              result = ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['x = 1'])])
              assert len(result.errors) == 1 and 'Not a single import' in result.errors[0].error
              assert 'z = 5' not in Path(path).read_text()
              assert 'z = 5' not in core.CACHE.get_tree(Path(path)).source
```
/answer 0a99-c5-c4 allow
Control Request:
```yaml
id: 0a99-c6-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
    ids:
    - DeleteItem
  reason: "Prüfen, dass der Dekorator von DeleteItem nicht doppelt ist"
```
/answer 0a99-c6-0 allow
Control Request:
```yaml
id: 0a99-c7-c6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/delete.py
      nodes:
      - id: DeleteItem
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class DeleteItem:
              """One node (or file) to delete.

              Attributes:
                  path: Absolute path to the file to modify.
                  id: Unique id of the target node. Omit to delete the whole file.
                  add_import: Import statements to add if missing, applied after the delete.
                  remove_import: Import statements to remove if present, applied after the delete.
                      Both are ignored (and flagged in the batch notice) when a whole file is deleted.
              """
              path: str
              id: str | None = None
              add_import: list[str] | None = None
              remove_import: list[str] | None = None
```
/answer 0a99-c7-c6 allow
Control Request:
```yaml
id: 0a99-c8-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: hec242bb5
    source: |-
      def test_python_invalid_import_keeps_edit_and_flags_notice(tmp_path: Path):
          path = _write(tmp_path, 'm.py', PY_SOURCE)
          result = ast_insert(items=[InsertItem(path=path, source='z = 5', id='A', add_import=['x = 1'])])
          assert not result.errors
          assert result.notice == IMPORT_NOTICE
          text = Path(path).read_text()
          assert 'z = 5' in text and 'x = 1' not in text
  reason: Letzten Test an das neue Verhalten anpassen
```
/answer 0a99-c8-0 allow
Control Request:
```yaml
id: 0a99-c9-0
toolName: python
arguments:
  script: |-
    import shutil, subprocess
    r = subprocess.run([shutil.which("pytest"), "-q", "--no-header", "-p", "no:cacheprovider", "tests"],
                       cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=1200)
    print(r.stdout[-6000:]); print(r.stderr[-2000:]); print("exit", r.returncode)
  reason: Gesamte Testsuite nach der Verhaltensänderung und der Header-Wiederherstellung ausführen
```
/answer 0a99-c9-0 allow
Control Request:
```yaml
id: 0a99-ca-c9
result:
  structuredContent:
    exit_code: 0
    stdout: |
      ........................................................................ [ 27%]
      ........................................................................ [ 54%]
      ........................................................................ [ 81%]
      ..........................................F.....                         [100%]
      =================================== FAILURES ===================================
      __________ test_malformed_insert_raises_plain_ast_error_not_ambiguous __________

      engine = <xy.ai.mcpc.tools.ast.rpc._java.JavaEngine object at 0x75b044eee510>

          def test_malformed_insert_raises_plain_ast_error_not_ambiguous(engine):
              """A 422 (malformed edit) must not be mistaken for a 409 (AstAmbiguous)."""
              tree = engine.parse(VALID_SOURCE)
              foo = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo'))
      >       with pytest.raises(AstError) as excinfo:
      E       Failed: DID NOT RAISE <class 'xy.ai.mcpc.tools.ast.base.AstError'>

      tests/test_rpc_java_engine.py:89: Failed
      =========================== short test summary info ============================
      FAILED tests/test_rpc_java_engine.py::test_malformed_insert_raises_plain_ast_error_not_ambiguous
      1 failed, 263 passed in 9.86s


      exit 1
```
/answer 0a99-ca-c9 allow
Thinking:
Die Suite läuft bis auf einen Fehler im Java-RPC-Test grün (263 bestanden), und der betrifft einen Pfad, den ich nicht angefasst habe. Weil diese Tests den echten Java-Prozess starten, schaue ich mir jetzt die Fixtures an, um den bisher ungetesteten RPC-Pfad zu prüfen.


ReasoningToken: 610
Control Request:
```yaml
id: 0a99-cb-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_rpc_java_engine.py
  reason: Fixture und Aufbau der Java-RPC-Tests für Integrationstests des Import-Pfads
```
/answer 0a99-cb-0 allow
Control Request:
```yaml
id: 0a99-cc-cb
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_rpc_java_engine.py
      nodes:
      - id: U5L197|fW1LQb
        signature: '''pytest tests for the RPC/PIC-based Java engine (xy.ai.mcpc.tools.ast.rpc).\n\n…'
      - id: 0ekC7u|9IXYUc
        signature: from __future__ import annotations
      - id: dcEdBS|j0BuJY
        signature: "_SRC = Path(__file__).resolve().parents[1] / 'src'"
      - id: fzPWql|drfHhJ
        signature: "from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError"
      - id: jqwo3C|2hARRN
        signature: "VALID_SOURCE = 'public class Foo {\\n    void bar() {\\n        System.out.printl…"
      - id: engine
        signature: "@pytest.fixture(scope='module') def engine():"
      - id: test_validate_accepts_well_formed_source
        signature: "def test_validate_accepts_well_formed_source(engine):"
      - id: test_validate_reports_malformed_source
        signature: "def test_validate_reports_malformed_source(engine):"
      - id: heef5b5b0
        signature: "def test_parse_raises_ast_error_on_malformed_source(engine):"
      - id: test_locate_all_reports_class_and_method
        signature: "def test_locate_all_reports_class_and_method(engine):"
      - id: test_node_code_matches_source_slice
        signature: "def test_node_code_matches_source_slice(engine):"
      - id: test_replace_updates_tree_source
        signature: "def test_replace_updates_tree_source(engine):"
      - id: test_insert_before_adds_a_sibling_field
        signature: "def test_insert_before_adds_a_sibling_field(engine):"
      - id: test_delete_removes_the_node
        signature: "def test_delete_removes_the_node(engine):"
      - id: test_append_adds_a_top_level_type
        signature: "def test_append_adds_a_top_level_type(engine):"
      - id: h875ddf1d
        signature: "def test_malformed_insert_raises_plain_ast_error_not_ambiguous(engine):"
        docstring: A 422 (malformed edit) must not be mistaken for a 409 (AstAmbiguous).
```
/answer 0a99-cc-cb allow
Control Request:
```yaml
id: 0a99-cd-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: rpc|rust|java
  reason: Vorhandene RPC-Tests (Java/Rust) auflisten
```
/answer 0a99-cd-0 allow
Control Request:
```yaml
id: 0a99-ce-cd
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests
      entries:
      - ".:"
      - test_ast_rust_bridge.py
      - test_rpc_java_engine.py
```
/answer 0a99-ce-cd allow
Control Request:
```yaml
id: 0a99-cf-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_rpc_java_engine.py
    ids:
    - engine
    - h875ddf1d
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_rust_bridge.py
  reason: Engine-Fixture und fehlschlagenden Test lesen; Rust-Bridge-Test als Vorlage für Rust-Integration
```
/answer 0a99-cf-0 allow
Control Request:
```yaml
id: 0a99-d0-cf
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_rpc_java_engine.py
      nodes:
      - id: engine
        type: FunctionDef
        code: |-
          @pytest.fixture(scope='module')
          def engine():
              return get_engine('java')
      - id: h875ddf1d
        type: FunctionDef
        code: |-
          def test_malformed_insert_raises_plain_ast_error_not_ambiguous(engine):
              """A 422 (malformed edit) must not be mistaken for a 409 (AstAmbiguous)."""
              tree = engine.parse(VALID_SOURCE)
              foo = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo'))
              with pytest.raises(AstError) as excinfo:
                  engine.insert(foo, 'int x;', 'before')
              assert not isinstance(excinfo.value, AstAmbiguous)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_rust_bridge.py
      nodes:
      - id: dU2g3m|CuR1rs
        signature: '''Eigenständiger Test für die Rust-RPC-Bridge (``xy.ai.mcpc.tools.ast.rpc._rust`…'
      - id: Mx1s7V|mOqPjE
        signature: from __future__ import annotations
      - id: WjQzAk|VtGINc
        signature: "SOURCE = 'struct Point {\\n    x: i32,\\n    y: i32,\\n}\\n\\nimpl Point {\\n    fn n…"
      - id: engine
        signature: "@pytest.fixture(scope='module') def engine() -> RustEngine:"
      - id: test_parse_and_serialize_roundtrip
        signature: "def test_parse_and_serialize_roundtrip(engine: RustEngine) -> None:"
      - id: test_locate_all_finds_struct_impl_and_fn
        signature: "def test_locate_all_finds_struct_impl_and_fn(engine: RustEngine) -> None:"
      - id: test_replace_mutates_and_stays_valid
        signature: "def test_replace_mutates_and_stays_valid(engine: RustEngine) -> None:"
      - id: test_validate_reports_syntax_error
        signature: "def test_validate_reports_syntax_error(engine: RustEngine) -> None:"
```
/answer 0a99-d0-cf allow
Text:
 Ich probiere den RPC-Pfad zuerst explorativ gegen die echten Prozesse aus, bevor ich Tests festschreibe.
ReasoningToken: 332
Control Request:
```yaml
id: 0a99-d1-0
toolName: python
arguments:
  script: |-
    import sys
    sys.path.insert(0, "/home/user/xyan/xy.ai.workbench/mcpc/src")
    from xy.ai.mcpc.tools.ast.generic import get_engine

    def show(title, tree):
        print("-----", title); print(tree.source)

    java = get_engine("java")
    src = "package p;\n\nimport java.util.List;\nimport java.util.Map;\n\npublic class Foo {\n    void bar() {}\n}\n"
    t = java.parse(src)
    print(java.apply_imports(t, ["import java.util.Set;", "import java.util.List"], ["import java.util.Map;"]))
    show("java add/remove", t)

    src2 = "package p;\n\npublic class Foo {\n}\n"
    t = java.parse(src2)
    print(java.apply_imports(t, ["import java.util.Set;"], []))
    show("java no imports", t)

    t = java.parse("public class Foo {\n}\n")
    print(java.apply_imports(t, ["import static java.lang.Math.max"], []))
    show("java no package no imports", t)

    t = java.parse(src)
    try:
        print(java.apply_imports(t, ["int x;"], []))
    except Exception as e:
        print("ERR", type(e).__name__, e)

    rust = get_engine("rust")
    rsrc = "use std::fmt;\nuse std::io::{self, Read};\n\nstruct P {\n    x: i32,\n}\n"
    t = rust.parse(rsrc)
    print(rust.apply_imports(t, ["use std::collections::HashMap;", "use std::fmt"], ["use std::io::{self, Read};"]))
    show("rust add/remove", t)

    t = rust.parse("//! doc\n\nstruct P {\n    x: i32,\n}\n")
    print(rust.apply_imports(t, ["use std::fmt;"], []))
    show("rust no imports", t)
  reason: Verhalten des Import-Pfads gegen die echten Java- und Rust-RPC-Prozesse explorativ prüfen
```
/answer 0a99-d1-0 allow
Control Request:
```yaml
id: 0a99-d2-d1
result:
  structuredContent:
    exit_code: 1
    stdout: ""
    stderr: |
      Traceback (most recent call last):
        File "<stdin>", line 11, in <module>
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py", line 202, in apply_imports
          return apply_rpc(self, tree, add, remove)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py", line 282, in apply_rpc
          raise AstError(f'Not recognized as an import by the parser: {missing!r}')
      xy.ai.mcpc.tools.ast.base.AstError: Not recognized as an import by the parser: ['import java.util.Set;', 'import java.util.List;']
```
/answer 0a99-d2-d1 allow
Text:
 Das Prüfen schlägt schon bei vorhandenen Imports fehl, also erkennt `_rpc_present` die Gruppen nicht richtig. Ich schaue mir an, was der Prozess tatsächlich meldet.
ReasoningToken: 109
Control Request:
```yaml
id: 0a99-d3-0
toolName: python
arguments:
  script: |-
    import sys
    sys.path.insert(0, "/home/user/xyan/xy.ai.workbench/mcpc/src")
    from xy.ai.mcpc.tools.ast.generic import get_engine
    for sym, src in [("java", "package p;\n\nimport java.util.List;\nimport java.util.Map;\n\npublic class Foo {\n    void bar() {}\n}\n"),
                     ("rust", "use std::fmt;\nuse std::io::{self, Read};\n\nstruct P {\n    x: i32,\n}\n")]:
        e = get_engine(sym); t = e.parse(src)
        for l in e.locate_all(t):
            print(sym, repr(l.node_id), l.node_type, l.name, l.lineno, l.end_lineno, l.parent_type, repr(e.node_code(l.node)))
  reason: Tatsächlich gemeldete Node-IDs/-Typen/Zeilenbereiche der RPC-Engines ansehen
```
/answer 0a99-d3-0 allow
Control Request:
```yaml
id: 0a99-d4-d3
result:
  structuredContent:
    exit_code: 0
    stdout: |
      java 'bG4Cjo|du6ALf' ImportGroup None 3 4 CompilationUnit 'import java.util.List;\nimport java.util.Map;\n'
      java 'Foo' ClassOrInterfaceDeclaration Foo 6 8 CompilationUnit 'public class Foo {\n    void bar() {}\n}\n'
      java 'Foo.bar' MethodDeclaration bar 7 7 ClassOrInterfaceDeclaration '    void bar() {}\n'
      rust 'zksMZE|zNFZo1' ImportGroup None 1 2 File 'use std::fmt;\nuse std::io::{self, Read};\n'
      rust 'P' ItemStruct P 4 6 File 'struct P {\n    x: i32,\n}\n'
```
/answer 0a99-d4-d3 allow
Text:
 Gut, dass ich getestet habe. Die Prozesse melden den Typ `ImportGroup` und nicht `imports`. Ich korrigiere die Erkennung und akzeptiere beide Schreibweisen.
Control Request:
```yaml
id: 0a99-d5-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: _rpc_present
    old_text: "if loc.node_type == 'imports' and '.' not in loc.node_id]"
    new_text: "if loc.node_type in _RPC_GROUP_TYPES and '.' not in loc.node_id]"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: _rpc_present
    old_text: '"""Top-level ``imports`` groups plus'
    new_text: '"""Top-level import groups plus'
  reason: Import-Gruppen der RPC-Engines mit dem tatsächlichen Typ ImportGroup erkennen
```
/answer 0a99-d5-0 allow
ReasoningToken: 127
Control Request:
```yaml
id: 0a99-d6-d5
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: _rpc_present
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: _rpc_present
```
/answer 0a99-d6-d5 allow
Control Request:
```yaml
id: 0a99-d7-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: _rpc_statement
    position: before
    source: |-
      '#: Node types under which the RPC processes report a run of imports.'
      _RPC_GROUP_TYPES = {'ImportGroup', 'imports'}
  reason: Konstante für RPC-Gruppentypen anlegen
```
/answer 0a99-d7-0 allow
Control Request:
```yaml
id: 0a99-d8-d7
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: _rpc_statement
```
/answer 0a99-d8-d7 allow
Text:
 Die IDs führen den Unterstrich nicht mit, daher `rpc_present` und `rpc_statement`.
Control Request:
```yaml
id: 0a99-d9-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: rpc_present
    old_text: "if loc.node_type == 'imports' and '.' not in loc.node_id]"
    new_text: "if loc.node_type in _RPC_GROUP_TYPES and '.' not in loc.node_id]"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: rpc_present
    old_text: '"""Top-level ``imports`` groups plus'
    new_text: '"""Top-level import groups plus'
  reason: Gruppen-Typ-Prüfung in _rpc_present korrigieren
```
/answer 0a99-d9-0 allow
ReasoningToken: 60
Control Request:
```yaml
id: 0a99-da-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: rpc_statement
    position: before
    source: |-
      '#: Node types under which the RPC processes report a run of imports.'
      _RPC_GROUP_TYPES = {'ImportGroup', 'imports'}
  reason: Konstante für RPC-Gruppentypen anlegen
```
/answer 0a99-da-0 allow
Text:
 Jetzt wiederhole ich die explorative Prüfung gegen beide Prozesse.
Control Request:
```yaml
id: 0a99-db-0
toolName: python
arguments:
  script: |-
    import sys
    sys.path.insert(0, "/home/user/xyan/xy.ai.workbench/mcpc/src")
    from xy.ai.mcpc.tools.ast.generic import get_engine

    def run(title, engine, src, add, remove):
        t = engine.parse(src)
        try:
            out = engine.apply_imports(t, add, remove)
            print("-----", title, out); print(t.source)
        except Exception as e:
            print("-----", title, "ERR", type(e).__name__, e)

    java = get_engine("java")
    src = "package p;\n\nimport java.util.List;\nimport java.util.Map;\n\npublic class Foo {\n    void bar() {}\n}\n"
    run("java add/remove", java, src, ["import java.util.Set;", "import java.util.List"], ["import java.util.Map;"])
    run("java no imports", java, "package p;\n\npublic class Foo {\n}\n", ["import java.util.Set;"], [])
    run("java no package/imports", java, "public class Foo {\n}\n", ["import static java.lang.Math.max"], [])
    run("java non-import", java, src, ["int x;"], [])
    rust = get_engine("rust")
    rsrc = "use std::fmt;\nuse std::io::{self, Read};\n\nstruct P {\n    x: i32,\n}\n"
    run("rust add/remove", rust, rsrc, ["use std::collections::HashMap;", "use std::fmt"], ["use std::io::{self, Read};"])
    run("rust no imports", rust, "//! doc\n\nstruct P {\n    x: i32,\n}\n", ["use std::fmt;"], [])
    run("rust non-import", rust, rsrc, ["fn x() {}"], [])
  reason: Import-Pfad gegen die echten Java- und Rust-Prozesse erneut prüfen
```
/answer 0a99-db-0 allow
Control Request:
```yaml
id: 0a99-dc-db
result:
  structuredContent:
    exit_code: 0
    stdout: |
      ----- java add/remove ImportOutcome(added=['import java.util.Set;'], removed=['import java.util.Map;'], unchecked=False)
      package p;

      import java.util.List;
      import java.util.Set;

      public class Foo {

        void bar() {
        }
      }

      ----- java no imports ERR AstError invalid type declaration: Encountered unexpected token: "import" "import"
          at line 1, column 1.

      Was expecting one of:

          "@"
          "class"
          "enum"
          "interface"

      ----- java no package/imports ERR AstError invalid type declaration: Encountered unexpected token: "import" "import"
          at line 1, column 1.

      Was expecting one of:

          "@"
          "class"
          "enum"
          "interface"

      ----- java non-import ERR AstError no import declarations found: int x;
      ----- rust add/remove ImportOutcome(added=['use std::collections::HashMap;'], removed=['use std::io::{self, Read};'], unchecked=False)
      use std::fmt;
      use std::collections::HashMap;
      struct P {
          x: i32,
      }

      ----- rust no imports ImportOutcome(added=['use std::fmt;'], removed=[], unchecked=False)
      //! doc
      use std::fmt;
      struct P {
          x: i32,
      }

      ----- rust non-import ERR AstError invalid items: expected one of: `fn`, `extern`, `use`, `static`, `const`, `unsafe`, `mod`, `type`, `struct`, `enum`, `union`, `impl`, `trait`, `auto`, `default`, `macro`, identifier, `self`, `super`, `crate`, `::`
```
/answer 0a99-dc-db allow
Thinking:
Der RPC-Pfad läuft für Java und Rust mit bestehenden Imports, aber Java ohne Imports braucht einen textuellen Fallback mit Validierung, und `apply_rpc` setzt den Quelltext bei Fehlern zurück. Rust verliert beim Einfügen eine Leerzeile, was an der Engine-Formatierung liegt.


ReasoningToken: 1963
Control Request:
```yaml
id: 0a99-dd-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    ids:
    - apply_rpc
  reason: Aktuelle Fassung von apply_rpc für den Umbau lesen
```
/answer 0a99-dd-0 allow
Control Request:
```yaml
id: 0a99-de-dd
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: apply_rpc
        type: FunctionDef
        code: |-
          def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
              """Structure-aware handling for RPC engines that report ``imports`` groups (Java ``import``, Rust ``use``).

              Statements are added through the engine's own ``insert``/``append`` (so the
              process parses them), then re-located to verify they were recognized as imports.
              Removal deletes the statement's span inside its group; the result is re-validated.
              """
              adds = _unique([_rpc_statement(s) for s in add])
              removes = _unique([_rpc_statement(s) for s in remove])
              outcome = ImportOutcome()
              if removes:
                  _, present = _rpc_present(engine, tree)
                  data = tree.source.encode('utf-8')
                  hits = [(present[normalize_import(s)], s) for s in removes if normalize_import(s) in present]
                  for (begin, end), _ in sorted(hits, key=lambda hit: hit[0][0], reverse=True):
                      data = delete_span(data, begin, end)
                  if hits:
                      source = data.decode('utf-8')
                      error = engine.validate(source)
                      if error:
                          raise AstError(f'The import change would leave the file invalid: {error}')
                      tree.source = source
                      outcome.removed = [s for _, s in hits]
              groups, present = _rpc_present(engine, tree)
              missing = [s for s in adds if normalize_import(s) not in present]
              if missing:
                  code = '\n'.join(missing)
                  if groups:
                      engine.insert(groups[-1], code, 'after')
                  else:
                      top = [loc for loc in engine.locate_all(tree) if '.' not in loc.node_id]
                      if top:
                          engine.insert(top[0], code, 'before')
                      else:
                          engine.append(tree, code)
                  _, present = _rpc_present(engine, tree)
                  if any((normalize_import(s) not in present for s in missing)):
                      raise AstError(f'Not recognized as an import by the parser: {missing!r}')
                  outcome.added = missing
              return outcome
```
/answer 0a99-de-dd allow
ReasoningToken: 84
Control Request:
```yaml
id: 0a99-df-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: apply_rpc
    source: |-
      def _insert_textually(engine: Engine, tree: Tree, code: str, first_top_level: Any | None) -> None:
          """Fallback when the process cannot insert imports next to a type: place them behind a leading
          ``package`` statement, else in front of the first top-level node, and validate the result."""
          data = tree.source.encode('utf-8')
          spans = statement_spans(data)
          block = code.encode('utf-8')
          if spans and data[spans[0][0]:spans[0][1]].startswith(b'package'):
              pos = _next_line_start(data, spans[0][1])
              block = b'\n' + block + b'\n'
          elif first_top_level is not None:
              pos = _line_starts(data)[min(first_top_level.lineno - 1, len(data))]
              block += b'\n\n'
          else:
              pos = len(data)
              block += b'\n'
          source = (data[:pos] + block + data[pos:]).decode('utf-8')
          error = engine.validate(source)
          if error:
              raise AstError(f'The import change would leave the file invalid: {error}')
          tree.source = source


      def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
          """Structure-aware handling for RPC engines that report import groups (Java ``import``, Rust ``use``).

          Statements are added through the engine's own ``insert``/``append`` (so the
          process parses them), then re-located to verify they were recognized as imports.
          Removal deletes the statement's span inside its group; the result is re-validated.
          On failure the tree's source is restored.
          """
          adds = _unique([_rpc_statement(s) for s in add])
          removes = _unique([_rpc_statement(s) for s in remove])
          original = tree.source
          outcome = ImportOutcome()
          try:
              if removes:
                  _, present = _rpc_present(engine, tree)
                  data = tree.source.encode('utf-8')
                  hits = [(present[normalize_import(s)], s) for s in removes if normalize_import(s) in present]
                  for (begin, end), _ in sorted(hits, key=lambda hit: hit[0][0], reverse=True):
                      data = delete_span(data, begin, end)
                  if hits:
                      source = data.decode('utf-8')
                      error = engine.validate(source)
                      if error:
                          raise AstError(f'The import change would leave the file invalid: {error}')
                      tree.source = source
                      outcome.removed = [s for _, s in hits]
              groups, present = _rpc_present(engine, tree)
              missing = [s for s in adds if normalize_import(s) not in present]
              if missing:
                  code = '\n'.join(missing)
                  if groups:
                      engine.insert(groups[-1], code, 'after')
                  else:
                      top = [loc for loc in engine.locate_all(tree) if '.' not in loc.node_id]
                      try:
                          if top:
                              engine.insert(top[0], code, 'before')
                          else:
                              engine.append(tree, code)
                      except AstError:
                          _insert_textually(engine, tree, code, top[0] if top else None)
                  _, present = _rpc_present(engine, tree)
                  if any(normalize_import(s) not in present for s in missing):
                      raise AstError(f'Not recognized as an import by the parser: {missing!r}')
                  outcome.added = missing
          except Exception:
              tree.source = original
              raise
          return outcome
  reason: apply_rpc um textuellen Fallback (Java ohne Imports) und Rollback des Quelltexts erweitern
```
/answer 0a99-df-0 allow
Control Request:
```yaml
id: 0a99-e0-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: insert_textually
    old_text: "        pos = _line_starts(data)[min(first_top_level.lineno - 1, len(data))]"
    new_text: |2-
              starts = _line_starts(data)
              pos = starts[min(first_top_level.lineno - 1, len(starts) - 1)]
  reason: Indexgrenze beim Zeilenstart im textuellen Fallback korrigieren
```
/answer 0a99-e0-0 allow
ReasoningToken: 111
Control Request:
```yaml
id: 0a99-e1-0
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
    source: |
      """Import handling against the real Java (JavaParser) and Rust (syn) RPC processes."""
      from __future__ import annotations

      import sys
      from pathlib import Path

      import pytest

      _SRC = Path(__file__).resolve().parents[1] / 'src'
      if _SRC.is_dir() and str(_SRC) not in sys.path:
          sys.path.insert(0, str(_SRC))

      from xy.ai.mcpc.tools.ast.base import AstError  # noqa: E402
      from xy.ai.mcpc.tools.ast.generic import get_engine  # noqa: E402

      JAVA = 'package p;\n\nimport java.util.List;\nimport java.util.Map;\n\npublic class Foo {\n    void bar() {}\n}\n'
      RUST = 'use std::fmt;\nuse std::io::{self, Read};\n\nstruct P {\n    x: i32,\n}\n'


      @pytest.fixture(scope='module')
      def java():
          return get_engine('java')


      @pytest.fixture(scope='module')
      def rust():
          return get_engine('rust')


      def test_java_add_present_and_remove(java):
          tree = java.parse(JAVA)
          outcome = java.apply_imports(
              tree, ['import java.util.Set;', 'import java.util.List'], ['import java.util.Map;'])
          assert outcome.added == ['import java.util.Set;']
          assert outcome.removed == ['import java.util.Map;']
          assert not outcome.unchecked
          assert 'import java.util.Set;' in tree.source and 'java.util.Map' not in tree.source
          assert tree.source.count('import java.util.List;') == 1
          assert java.validate(tree.source) is None


      def test_java_without_imports_goes_behind_package(java):
          tree = java.parse('package p;\n\npublic class Foo {\n}\n')
          outcome = java.apply_imports(tree, ['import java.util.Set;'], [])
          assert outcome.added == ['import java.util.Set;']
          assert tree.source.index('package p;') < tree.source.index('import java.util.Set;') < tree.source.index('class Foo')


      def test_java_without_package_and_imports(java):
          tree = java.parse('public class Foo {\n}\n')
          java.apply_imports(tree, ['import static java.lang.Math.max'], [])
          assert tree.source.startswith('import static java.lang.Math.max;')
          assert java.validate(tree.source) is None


      def test_java_non_import_raises_and_leaves_source(java):
          tree = java.parse(JAVA)
          with pytest.raises(AstError):
              java.apply_imports(tree, ['int x;'], [])
          assert tree.source == JAVA


      def test_rust_add_present_and_remove(rust):
          tree = rust.parse(RUST)
          outcome = rust.apply_imports(
              tree, ['use std::collections::HashMap;', 'use std::fmt'], ['use std::io::{self, Read};'])
          assert outcome.added == ['use std::collections::HashMap;']
          assert outcome.removed == ['use std::io::{self, Read};']
          assert tree.source.count('use std::fmt;') == 1
          assert 'std::io' not in tree.source
          assert rust.validate(tree.source) is None


      def test_rust_without_imports(rust):
          tree = rust.parse('struct P {\n    x: i32,\n}\n')
          outcome = rust.apply_imports(tree, ['use std::fmt;'], [])
          assert outcome.added == ['use std::fmt;']
          assert tree.source.index('use std::fmt;') < tree.source.index('struct P')


      def test_rust_non_import_raises_and_leaves_source(rust):
          tree = rust.parse(RUST)
          with pytest.raises(AstError):
              rust.apply_imports(tree, ['fn x() {}'], [])
          assert tree.source == RUST
  reason: Integrationstests für den Java- und Rust-RPC-Import-Pfad anlegen
```
/answer 0a99-e1-0 allow
Control Request:
```yaml
id: 0a99-e2-0
toolName: python
arguments:
  script: |-
    import shutil, subprocess
    r = subprocess.run([shutil.which("pytest"), "-q", "--no-header", "-p", "no:cacheprovider", "tests"],
                       cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=1200)
    print(r.stdout[-6000:]); print(r.stderr[-2000:]); print("exit", r.returncode)
  reason: Gesamte Testsuite inklusive neuer RPC-Integrationstests ausführen
```
/answer 0a99-e2-0 allow
Control Request:
```yaml
id: 0a99-e3-e2
result:
  structuredContent:
    exit_code: 0
    stdout: |
      ........................................................................ [ 26%]
      ........................................................................ [ 53%]
      ........................................................................ [ 79%]
      .................................................F.....                  [100%]
      =================================== FAILURES ===================================
      __________ test_malformed_insert_raises_plain_ast_error_not_ambiguous __________

      engine = <xy.ai.mcpc.tools.ast.rpc._java.JavaEngine object at 0x7f472c630c50>

          def test_malformed_insert_raises_plain_ast_error_not_ambiguous(engine):
              """A 422 (malformed edit) must not be mistaken for a 409 (AstAmbiguous)."""
              tree = engine.parse(VALID_SOURCE)
              foo = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo'))
      >       with pytest.raises(AstError) as excinfo:
      E       Failed: DID NOT RAISE <class 'xy.ai.mcpc.tools.ast.base.AstError'>

      tests/test_rpc_java_engine.py:89: Failed
      =========================== short test summary info ============================
      FAILED tests/test_rpc_java_engine.py::test_malformed_insert_raises_plain_ast_error_not_ambiguous
      1 failed, 270 passed in 10.21s


      exit 1
```
/answer 0a99-e3-e2 allow
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 368009, in: 138, out: 105992, reason: 294, read: 8597077, write: 261879, start: 1794265620753, end: 1794265631497, time: 10688, total: 780801, io: 0, control: 56, sent: 0, recv: 224
ReasoningToken: 294
Ein Import-Fehler verwirft den Node-Edit jetzt nicht mehr. Die Suite hat 270 bestandene Tests und einen Fehlschlag, `test_malformed_insert_raises_plain_ast_error_not_ambiguous` in `test_rpc_java_engine.py`. Der Test ruft `engine.insert` direkt mit `int x;` auf, einen Pfad, den ich nicht geändert habe. Gegen den Stand vor meinen Änderungen habe ich ihn nicht geprüft.

## Was sich geändert hat

- **Import-Fehler sind nicht mehr fatal:** `core.apply_imports` fängt jeden Fehler des Import-Teils ab, schreibt eine Warnung ins Log und meldet das Item als ungeprüft. Der Node-Edit bleibt erhalten und wird gespeichert. Der Batch enthält dann die `notice`. Den Grund für den Fehler sieht man nur im Log, nicht im Ergebnis.
- **Rollback nur für den Import-Teil:** Hat die Engine den Quelltext schon verändert, bevor sie scheiterte, wird er auf den Stand nach dem Node-Edit zurückgesetzt.
- **`ast_delete` ohne `id`:** Die Datei wird gelöscht. Die Import-Felder werden ignoriert und über die `notice` gemeldet.
- **Unabhängigkeit in die andere Richtung:** Scheitert der Node-Edit selbst, werden die Imports weiterhin nicht angewendet. Das habe ich bewusst so gelassen, weil ein Import ohne die zugehörige Änderung keinen Sinn ergibt. Sag Bescheid, wenn du das anders meinst.

## Fehler, den der Realtest aufgedeckt hat

Die Java- und Rust-Prozesse melden Import-Gruppen als `ImportGroup`, nicht als `imports`. Meine erste Fassung hätte dort nie etwas erkannt. Das ist korrigiert, und der RPC-Pfad ist jetzt gegen die echten Prozesse getestet.

- **Java:** Hinzufügen, Entfernen und das Erkennen vorhandener Imports funktionieren. Ohne bestehende Imports kann der Prozess keinen Import vor einen Typ einfügen. Dafür gibt es jetzt einen textuellen Fallback hinter der `package`-Zeile, mit anschließender Validierung.
- **Rust:** Hinzufügen und Entfernen funktionieren, auch ohne bestehende Imports.
- **Neuformatierung:** Die Engines drucken beim Einfügen neu. Bei Java ändert sich dabei die Formatierung des Klassenrumpfs, bei Rust fällt eine Leerzeile weg. Das liegt an der Engine und nicht an meiner Logik.

Neu sind die Tests in `tests/test_ast_imports_rpc.py`. In `tests/test_ast_imports.py` habe ich drei Tests auf das neue Verhalten umgestellt und einen Rollback-Test ergänzt.

Beim Umbau von `core.py` habe ich versehentlich den Import-Block überschrieben. Ich habe ihn aus dem zuvor gelesenen Original wiederhergestellt, und die Suite läuft damit.
Was meint "Hat die Engine den Quelltext schon verändert, bevor sie scheiterte"?
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 370787, in: 140, out: 107242, reason: 231, read: 8797546, write: 263405, start: 1794265786321, end: 1794265797014, time: 10616, total: 791417, io: 0, control: 77, sent: 1, recv: 228
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
ReasoningToken: 231
Gemeint ist ein Zwischenzustand: Der Import-Teil besteht aus mehreren Schritten, und ein späterer Schritt kann scheitern, nachdem ein früherer den Tree schon verändert hat.

## Konkretes Beispiel (Java/Rust über RPC)

`apply_rpc` arbeitet in Etappen:

1. **Entfernen:** Die Zeile wird gelöscht und `tree.source` sofort neu gesetzt.
2. **Einfügen:** `engine.insert(...)` schreibt den neuen Quelltext direkt in `tree.source`. Das macht der Prozess-Aufruf selbst.
3. **Prüfen:** Danach wird neu lokalisiert, ob der Import als Import erkannt wurde. Wenn nicht, löse ich einen Fehler aus.

Scheitert Schritt 3, steht in `tree.source` schon der halbfertige Zustand, etwa mit einer Zeile, die der Parser nicht als Import erkennt. Ohne Rollback würde `CACHE.save` diesen Zustand zusammen mit dem Node-Edit auf die Platte schreiben. Dann hätte der Import-Fehler doch die Datei beschädigt.

## Wie der Rollback das auffängt

Es gibt zwei Ebenen:

- **`apply_rpc`:** Es merkt sich `tree.source` vorher und setzt es bei jedem Fehler zurück.
- **`core.apply_imports`:** Es sichert `tree.source` und `tree.raw` nach dem Node-Edit und stellt beides bei einem Fehler wieder her. Das gilt für jede Engine, auch für künftige.

Zurückgesetzt wird nur der Stand nach dem Node-Edit, nicht davor. Der Node-Edit bleibt bestehen.

## Grenze der Absicherung

Bei Python wird der `ast`-Baum in `tree.raw` direkt verändert (`body.insert`, `del body[i]`). Eine gesicherte Referenz auf `raw` ist deshalb dasselbe Objekt und würde eine solche Änderung nicht zurückdrehen. Das ist aber nur theoretisch. Alle Fehlerquellen der Python-Variante (kein Import-Statement, Syntaxfehler) treten beim Parsen auf, bevor irgendetwas verändert wird. Bei JavaScript wird `tree.raw` erst nach erfolgreicher Validierung ersetzt. Ein echtes Rollback-Problem kann es deshalb derzeit nur bei den RPC-Engines geben, und dort greift die Sicherung.

Wenn du das für Python robuster willst, könnte ich vor dem Import-Teil eine `copy.deepcopy` des `ast.Module` anlegen. Das kostet bei großen Dateien etwas Zeit, die ich bisher nicht für nötig gehalten habe.
Das verstehe ich nicht. Die Engine kann alle Import Knoten holen, dann alle änderungen gesammelt anwenden und sie schreiben. Dabei gibt es weder Teilausführung noch Fallback.
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Sie haben recht, und mein Rollback ist ein Symptom einer schlechteren Konstruktion. Ich habe die RPC-Variante so gebaut, dass sie `engine.insert` aufruft. Dieser Aufruf schreibt selbst in `tree.source`, und dadurch gibt es Zwischenzustände. Dein Ansatz ist besser: alle Import-Knoten holen, alle Änderungen als Edits auf dem Original berechnen, einmal anwenden, validieren und erst dann schreiben. Dann entfallen auch der Rollback und der Java-Fallback. Außerdem würde die Neuformatierung durch die Engine wegfallen. Ich baue `apply_rpc` entsprechend um.
ReasoningToken: 5071
Control Request:
```yaml
id: 0a99-e4-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    ids:
    - delete_span
    - rpc_present
  reason: delete_span und _rpc_present vor dem Umbau in der aktuellen Fassung lesen
```
/answer 0a99-e4-0 allow
Control Request:
```yaml
id: 0a99-e5-e4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: delete_span
        type: FunctionDef
        code: |-
          def delete_span(data: bytes, start: int, end: int) -> bytes:
              """Remove ``data[start:end]``, including its line if nothing else stands on it."""
              line_start = data.rfind(b'\n', 0, start) + 1
              line_end = data.find(b'\n', end)
              line_end = len(data) if line_end < 0 else line_end
              if not data[line_start:start].strip() and (not data[end:line_end].strip()):
                  return data[:line_start] + data[min(line_end + 1, len(data)):]
              return data[:start] + data[end:]
      - id: rpc_present
        type: FunctionDef
        code: |-
          def _rpc_present(engine: Engine, tree: Tree) -> tuple[list[Any], dict[str, tuple[int, int]]]:
              """Top-level import groups plus every statement's byte span, keyed by normalized text."""
              data = tree.source.encode('utf-8')
              starts = _line_starts(data)
              groups = [loc for loc in engine.locate_all(tree) if loc.node_type in _RPC_GROUP_TYPES and '.' not in loc.node_id]
              present: dict[str, tuple[int, int]] = {}
              for loc in groups:
                  low = starts[min(loc.lineno - 1, len(starts) - 1)]
                  high = starts[min(loc.end_lineno, len(starts) - 1)]
                  for begin, end in statement_spans(data[low:high]):
                      present.setdefault(normalize_import(data[low + begin:low + end].decode('utf-8')), (low + begin, low + end))
              return (groups, present)
```
/answer 0a99-e5-e4 allow
ReasoningToken: 115
Control Request:
```yaml
id: 0a99-e6-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: delete_span
    source: |-
      def line_extent(data: bytes, start: int, end: int) -> tuple[int, int]:
          """``(start, end)`` widened to its whole line(s) if nothing else stands on them."""
          line_start = data.rfind(b'\n', 0, start) + 1
          line_end = data.find(b'\n', end)
          line_end = len(data) if line_end < 0 else line_end
          if not data[line_start:start].strip() and not data[end:line_end].strip():
              return line_start, min(line_end + 1, len(data))
          return start, end


      def delete_span(data: bytes, start: int, end: int) -> bytes:
          """Remove ``data[start:end]``, including its line if nothing else stands on it."""
          start, end = line_extent(data, start, end)
          return data[:start] + data[end:]
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: rpc_present
    source: |-
      def _rpc_present(
              engine: Engine, tree: Tree, source: str | None = None) -> tuple[dict[str, tuple[int, int]], list[Any]]:
          """Every top-level import statement's byte span (keyed by normalized text) and the top-level nodes.

          ``source`` inspects a candidate text instead of ``tree.source``, without touching the tree.
          """
          view = tree if source is None else Tree(engine, None, source, tree.path)
          data = view.source.encode('utf-8')
          starts = _line_starts(data)
          top = [loc for loc in engine.locate_all(view) if '.' not in loc.node_id]
          present: dict[str, tuple[int, int]] = {}
          for loc in top:
              if loc.node_type not in _RPC_GROUP_TYPES:
                  continue
              low = starts[min(loc.lineno - 1, len(starts) - 1)]
              high = starts[min(loc.end_lineno, len(starts) - 1)]
              for begin, end in statement_spans(data[low:high]):
                  present.setdefault(normalize_import(data[low + begin:low + end].decode('utf-8')), (low + begin, low + end))
          return present, top


      '#: Line prefixes that stay attached to the declaration below them (docs, annotations, attributes).'
      _ATTACHED_PREFIXES = (b'//', b'/*', b'*', b'#[', b'@')


      def _leading_start(data: bytes, starts: list[int], index: int) -> int:
          """Start offset of line ``index``, moved up over directly attached comment/annotation lines."""
          while index > 0:
              previous = data[starts[index - 1]:starts[index]].strip()
              if previous.startswith(_ATTACHED_PREFIXES) and not previous.startswith(b'//!'):
                  index -= 1
              else:
                  break
          return starts[index]


      def _rpc_insertion(
              data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:
          """Edit that places ``code``: behind the last import, else behind ``package``, else before the first node."""
          block = code.encode('utf-8')
          if present:
              pos = _next_line_start(data, max(end for _, end in present.values()))
              text = b'\n' + block if pos == len(data) and not data.endswith(b'\n') else block + b'\n'
              return pos, pos, text
          spans = statement_spans(data)
          if spans and data[spans[0][0]:spans[0][1]].startswith(b'package'):
              pos = _next_line_start(data, spans[0][1])
              return pos, pos, b'\n' + block + b'\n'
          if top:
              starts = _line_starts(data)
              pos = _leading_start(data, starts, min(top[0].lineno - 1, len(starts) - 1))
              return pos, pos, block + b'\n\n'
          pos = len(data)
          return pos, pos, (b'' if not data or data.endswith(b'\n') else b'\n') + block + b'\n'
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: apply_rpc
    source: |-
      def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
          """Structure-aware handling for RPC engines that report import groups (Java ``import``, Rust ``use``).

          The engine only reads: its locate result yields every import statement's span, all
          removals and the one insertion are computed as edits on the original text and applied
          together. The candidate is validated and re-located before ``tree.source`` is written once,
          so a failure leaves the tree untouched.
          """
          adds = _unique([_rpc_statement(s) for s in add])
          removes = _unique([_rpc_statement(s) for s in remove])
          data = tree.source.encode('utf-8')
          present, top = _rpc_present(engine, tree)
          outcome = ImportOutcome()
          edits: list[tuple[int, int, bytes]] = []
          removed_keys: set[str] = set()
          for statement in removes:
              key = normalize_import(statement)
              if key in present:
                  edits.append((*line_extent(data, *present[key]), b''))
                  removed_keys.add(key)
                  outcome.removed.append(statement)
          missing = [s for s in adds if normalize_import(s) not in present.keys() - removed_keys]
          if missing:
              edits.append(_rpc_insertion(data, present, top, '\n'.join(missing)))
              outcome.added = missing
          if not edits:
              return outcome
          for start, end, text in sorted(edits, key=lambda edit: (edit[0], edit[1]), reverse=True):
              data = data[:start] + text + data[end:]
          source = data.decode('utf-8')
          error = engine.validate(source)
          if error:
              raise AstError(f'The import change would leave the file invalid: {error}')
          after, _ = _rpc_present(engine, tree, source)
          if any(normalize_import(s) not in after for s in outcome.added):
              raise AstError(f'Not recognized as an import by the parser: {outcome.added!r}')
          tree.source = source
          return outcome
  reason: "RPC-Import-Pfad auf „alle Edits berechnen, validieren, einmal schreiben“ umstellen"
```
/answer 0a99-e6-0 allow
Control Request:
```yaml
id: 0a99-e7-0
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: insert_textually
  reason: Nicht mehr benötigten textuellen Fallback entfernen
```
/answer 0a99-e7-0 allow
Text:
 Jetzt vereinfache ich `core.apply_imports`, denn ohne Zwischenzustände braucht es keinen Snapshot mehr.
Control Request:
```yaml
id: 0a99-e8-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: apply_imports
    source: |-
      def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:
          """Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``.

          Independent of the node edit: engines compute all import changes first and write them
          once, so a failure leaves the tree as the node edit left it. It is then reported as
          ``unchecked``, which puts the notice on the batch.
          """
          if not add and not remove:
              return None
          try:
              return tree.engine.apply_imports(tree, add or [], remove or [])
          except Exception:  # noqa: BLE001 - never fail the edit because of its imports
              logger.warning('Import handling failed for %s', tree.path, exc_info=True)
              return ImportOutcome(unchecked=True)
  reason: Snapshot/Restore in core.apply_imports entfernen
```
/answer 0a99-e8-0 allow
ReasoningToken: 88
Control Request:
```yaml
id: 0a99-e9-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  reason: Struktur nach dem Umbau auf Duplikate/Reste prüfen
```
/answer 0a99-e9-0 allow
Control Request:
```yaml
id: 0a99-ea-e9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: foxd99|7aiScK
        signature: '"Import/include/use handling shared by the ``ast_*`` edit tools.\n\nEvery edit …'
      - id: AFajCW|f1RBFD
        signature: from __future__ import annotations
      - id: Rr1Rct|w5Es1O
        signature: "__all__ = ["
      - id: ImportOutcome
        signature: "@dataclass class ImportOutcome:"
        docstring: "What an engine did for one item's ``addImport``/``removeImport``. Attributes: a…"
      - id: string_list
        signature: "def _string_list(item: dict[str, Any], key: str) -> list[str] | None:"
      - id: parse_import_args
        signature: "def parse_import_args(item: dict[str, Any]) -> tuple[list[str] | None, list[str] | None]:"
        docstring: "``(addImport, removeImport)`` of a raw MCP item; ``ValueError`` on malformed va…"
      - id: import_notice
        signature: "def import_notice(results: list[Any]) -> str | None:"
        docstring: "The single batch-level hint, if any result carries unchecked imports."
      - id: import_entry
        signature: "def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:"
        docstring: "Serialized ``imports`` field of a result, or ``None`` if nothing changed."
      - id: S64wGg|AyaH5Z
        signature: _SPACE = re.compile('\\s+')
      - id: normalize_import
        signature: "def normalize_import(text: str) -> str:"
        docstring: "Whitespace-, quote- and trailing-``;``-insensitive form used to compare stateme…"
      - id: checked
        signature: "def _checked(statement: str) -> str:"
      - id: unique
        signature: "def _unique(statements: list[str]) -> list[str]:"
      - id: statement_spans
        signature: "def statement_spans(data: bytes) -> list[tuple[int, int]]:"
        docstring: Byte spans of the ``;``-terminated top-level statements in ``data`` (comments s…
      - id: line_extent
        signature: "def line_extent(data: bytes, start: int, end: int) -> tuple[int, int]:"
        docstring: "``(start, end)`` widened to its whole line(s) if nothing else stands on them."
      - id: delete_span
        signature: "def delete_span(data: bytes, start: int, end: int) -> bytes:"
        docstring: "Remove ``data[start:end]``, including its line if nothing else stands on it."
      - id: next_line_start
        signature: "def _next_line_start(data: bytes, offset: int) -> int:"
      - id: line_starts
        signature: "def _line_starts(data: bytes) -> list[int]:"
      - id: store
        signature: "def _store(engine: Engine, tree: Tree, data: bytes) -> None:"
      - id: apply_generic
        signature: "def apply_generic(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
        docstring: "Engine-agnostic fallback. ``add`` is only checked: a statement missing as a lin…"
      - id: prologue_end
        signature: "def _prologue_end(root: Any, data: bytes, is_prologue: Callable[[Any], bool] | None) -> int:"
        docstring: Offset after leading shebang/directives and header comments (comments followed …
      - id: snippet
        signature: "def _snippet(engine: Engine, statement: str, import_types: set[str]) -> str:"
      - id: apply_treesitter
        signature: "def apply_treesitter(engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str], is_prologue: Callable[[Any], bool] | None=None) -> ImportOutcome:"
        docstring: Structure-aware handling for tree-sitter engines whose top-level import nodes a…
      - id: 4Vz76R|AG3pif
        signature: "'#: Node types under which the RPC processes report a run of imports.'"
      - id: rpc_statement
        signature: "def _rpc_statement(statement: str) -> str:"
      - id: rpc_present
        signature: "def _rpc_present(engine: Engine, tree: Tree, source: str | None=None) -> tuple[dict[str, tuple[int, int]], list[Any]]:"
        docstring: Every top-level import statement's byte span (keyed by normalized text) and the…
      - id: g7FpPW|tkBF2z
        signature: "'#: Line prefixes that stay attached to the declaration below them (docs, annot…"
      - id: leading_start
        signature: "def _leading_start(data: bytes, starts: list[int], index: int) -> int:"
        docstring: "Start offset of line ``index``, moved up over directly attached comment/annotat…"
      - id: rpc_insertion
        signature: "def _rpc_insertion(data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:"
        docstring: "Edit that places ``code``: behind the last import, else behind ``package``, els…"
      - id: apply_rpc
        signature: "def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
        docstring: Structure-aware handling for RPC engines that report import groups (Java ``impo…
```
/answer 0a99-ea-e9 allow
Text:
 Ich ergänze zwei Tests: Formatierung bleibt erhalten, und ein Import landet nicht zwischen Doc-Kommentar und Deklaration. Danach läuft die ganze Suite.
Control Request:
```yaml
id: 0a99-eb-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
    id: test_rust_non_import_raises_and_leaves_source
    position: after
    source: |-
      def test_rust_keeps_surrounding_formatting(rust):
          tree = rust.parse(RUST)
          rust.apply_imports(tree, ['use std::collections::HashMap;'], [])
          assert tree.source == 'use std::fmt;\nuse std::io::{self, Read};\nuse std::collections::HashMap;\n\nstruct P {\n    x: i32,\n}\n'


      def test_rust_import_does_not_split_doc_comment_from_item(rust):
          tree = rust.parse('//! crate\n\n/// Point.\nstruct P {\n    x: i32,\n}\n')
          rust.apply_imports(tree, ['use std::fmt;'], [])
          assert tree.source.index('//! crate') < tree.source.index('use std::fmt;') < tree.source.index('/// Point.')


      def test_java_keeps_surrounding_formatting(java):
          tree = java.parse(JAVA)
          java.apply_imports(tree, ['import java.util.Set;'], [])
          assert tree.source == JAVA.replace('import java.util.Map;\n', 'import java.util.Map;\nimport java.util.Set;\n')


      def test_java_add_and_remove_in_one_pass(java):
          tree = java.parse(JAVA)
          outcome = java.apply_imports(tree, ['import java.util.Set;'], ['import java.util.Map;'])
          assert outcome.added and outcome.removed
          assert tree.source == JAVA.replace('import java.util.Map;\n', 'import java.util.Set;\n')
  reason: "Tests für erhaltene Formatierung, Doc-Kommentar-Bindung und gleichzeitiges Hinzufügen/Entfernen ergänzen"
```
/answer 0a99-eb-0 allow
ReasoningToken: 244
Control Request:
```yaml
id: 0a99-ec-eb
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: test_rust_non_import_raises_and_leaves_source
```
/answer 0a99-ec-eb allow
Control Request:
```yaml
id: 0a99-ed-0
toolName: python
arguments:
  script: |-
    import shutil, subprocess
    r = subprocess.run([shutil.which("pytest"), "-q", "--no-header", "-p", "no:cacheprovider", "tests"],
                       cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=1200)
    print(r.stdout[-7000:]); print(r.stderr[-2000:]); print("exit", r.returncode)
  reason: Gesamte Testsuite nach dem Umbau von apply_rpc ausführen
```
/answer 0a99-ed-0 allow
Control Request:
```yaml
id: 0a99-ee-ed
result:
  structuredContent:
    exit_code: 0
    stdout: |
      ..............................F......................................... [ 26%]
      ........................................................................ [ 53%]
      ........................................................................ [ 79%]
      .................................................F.....                  [100%]
      =================================== FAILURES ===================================
      _________________ test_failing_engine_rolls_back_imports_only __________________

      tmp_path = PosixPath('/tmp/pytest-of-user/pytest-3/test_failing_engine_rolls_back0')
      monkeypatch = <_pytest.monkeypatch.MonkeyPatch object at 0x71cb12854650>

          def test_failing_engine_rolls_back_imports_only(tmp_path: Path, monkeypatch):
              path = _write(tmp_path, 'm.js', JS_SOURCE)
              node = _first_id(path, 'function_declaration')
              engine = core.load(path)[1].engine

              def broken(tree, add, remove):
                  tree.source = tree.source + '\nGARBAGE'
                  raise RuntimeError('boom')
              monkeypatch.setattr(engine, 'apply_imports', broken)
              result = ast_replace(
                  items=[
                      ReplaceItem(
                          path=path,
                          source='function f() {\n  return 2;\n}',
                          id=node,
                          add_import=["import c from 'c';"])])
              assert not result.errors
              assert result.notice == IMPORT_NOTICE
              text = Path(path).read_text()
      >       assert 'return 2' in text and 'GARBAGE' not in text
      E       assert ('return 2' in "import a from 'a';\nimport {b} from 'b';\n\nfunction f() {\n  return 2;\n}\n\nGARBAGE" and 'GARBAGE' not in 'import a fr...}\n\nGARBAGE'
      E
      E         'GARBAGE' is contained here:
      E           turn 2;
      E           }
      E
      E           GARBAGE)

      tests/test_ast_imports.py:174: AssertionError
      ------------------------------ Captured log call -------------------------------
      WARNING  xy.ai.mcpc.tools.ast.core:core.py:146 Import handling failed for /tmp/pytest-of-user/pytest-3/test_failing_engine_rolls_back0/m.js
      Traceback (most recent call last):
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py", line 143, in apply_imports
          return tree.engine.apply_imports(tree, add or [], remove or [])
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        File "/home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py", line 162, in broken
          raise RuntimeError('boom')
      RuntimeError: boom
      __________ test_malformed_insert_raises_plain_ast_error_not_ambiguous __________

      engine = <xy.ai.mcpc.tools.ast.rpc._java.JavaEngine object at 0x71cb1260c740>

          def test_malformed_insert_raises_plain_ast_error_not_ambiguous(engine):
              """A 422 (malformed edit) must not be mistaken for a 409 (AstAmbiguous)."""
              tree = engine.parse(VALID_SOURCE)
              foo = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo'))
      >       with pytest.raises(AstError) as excinfo:
      E       Failed: DID NOT RAISE <class 'xy.ai.mcpc.tools.ast.base.AstError'>

      tests/test_rpc_java_engine.py:89: Failed
      =========================== short test summary info ============================
      FAILED tests/test_ast_imports.py::test_failing_engine_rolls_back_imports_only
      FAILED tests/test_rpc_java_engine.py::test_malformed_insert_raises_plain_ast_error_not_ambiguous
      2 failed, 269 passed in 9.87s


      exit 1
```
/answer 0a99-ee-ed allow
Thinking:
Mein Rollback-Test passt nicht mehr zum neuen Vertrag, daher ersetze ich ihn durch einen Fall, in dem die Engine schon vor dem Schreiben scheitert. Außerdem muss ich das Einfügen der neuen RPC-Tests an der ID noch einmal angehen.


ReasoningToken: 420
Control Request:
```yaml
id: 0a99-ef-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
  regexp: non_import_raises
  node_type: FunctionDef
  reason: Aktuelle Node-IDs der letzten RPC-Tests ermitteln
```
/answer 0a99-ef-0 allow
Control Request:
```yaml
id: 0a99-f0-ef
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
      nodes:
      - id: h836b0c56
        type: FunctionDef
        code: |-
          def test_java_non_import_raises_and_leaves_source(java):
              tree = java.parse(JAVA)
              with pytest.raises(AstError):
                  java.apply_imports(tree, ['int x;'], [])
              assert tree.source == JAVA
      - id: h240d4d24
        type: FunctionDef
        code: |-
          def test_rust_non_import_raises_and_leaves_source(rust):
              tree = rust.parse(RUST)
              with pytest.raises(AstError):
                  rust.apply_imports(tree, ['fn x() {}'], [])
              assert tree.source == RUST
```
/answer 0a99-f0-ef allow
Control Request:
```yaml
id: 0a99-f1-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports_rpc.py
    id: h240d4d24
    position: after
    source: |-
      def test_rust_keeps_surrounding_formatting(rust):
          tree = rust.parse(RUST)
          rust.apply_imports(tree, ['use std::collections::HashMap;'], [])
          assert tree.source == 'use std::fmt;\nuse std::io::{self, Read};\nuse std::collections::HashMap;\n\nstruct P {\n    x: i32,\n}\n'


      def test_rust_import_does_not_split_doc_comment_from_item(rust):
          tree = rust.parse('//! crate\n\n/// Point.\nstruct P {\n    x: i32,\n}\n')
          rust.apply_imports(tree, ['use std::fmt;'], [])
          assert tree.source.index('//! crate') < tree.source.index('use std::fmt;') < tree.source.index('/// Point.')


      def test_java_keeps_surrounding_formatting(java):
          tree = java.parse(JAVA)
          java.apply_imports(tree, ['import java.util.Set;'], [])
          assert tree.source == JAVA.replace('import java.util.Map;\n', 'import java.util.Map;\nimport java.util.Set;\n')


      def test_java_add_and_remove_in_one_pass(java):
          tree = java.parse(JAVA)
          outcome = java.apply_imports(tree, ['import java.util.Set;'], ['import java.util.Map;'])
          assert outcome.added and outcome.removed
          assert tree.source == JAVA.replace('import java.util.Map;\n', 'import java.util.Set;\n')
  reason: "Neue RPC-Tests (Formatierung, Doc-Kommentar, Add+Remove in einem Durchgang) einfügen"
```
/answer 0a99-f1-0 allow
Control Request:
```yaml
id: 0a99-f2-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
  regexp: rolls_back_imports_only
  node_type: FunctionDef
  reason: ID des zu ersetzenden Rollback-Tests ermitteln
```
/answer 0a99-f2-0 allow
Control Request:
```yaml
id: 0a99-f3-f2
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
      nodes:
      - id: h6431b01f
        type: FunctionDef
        code: |-
          def test_failing_engine_rolls_back_imports_only(tmp_path: Path, monkeypatch):
              path = _write(tmp_path, 'm.js', JS_SOURCE)
              node = _first_id(path, 'function_declaration')
              engine = core.load(path)[1].engine

              def broken(tree, add, remove):
                  tree.source = tree.source + '\nGARBAGE'
                  raise RuntimeError('boom')
              monkeypatch.setattr(engine, 'apply_imports', broken)
              result = ast_replace(
                  items=[
                      ReplaceItem(
                          path=path,
                          source='function f() {\n  return 2;\n}',
                          id=node,
                          add_import=["import c from 'c';"])])
              assert not result.errors
              assert result.notice == IMPORT_NOTICE
              text = Path(path).read_text()
              assert 'return 2' in text and 'GARBAGE' not in text
```
/answer 0a99-f3-f2 allow
Control Request:
```yaml
id: 0a99-f4-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_ast_imports.py
    id: h6431b01f
    source: |-
      def test_failing_engine_keeps_edit_and_flags_notice(tmp_path: Path, monkeypatch):
          path = _write(tmp_path, 'm.js', JS_SOURCE)
          node = _first_id(path, 'function_declaration')
          engine = core.load(path)[1].engine

          def broken(tree, add, remove):
              raise RuntimeError('boom')

          monkeypatch.setattr(engine, 'apply_imports', broken)
          result = ast_replace(items=[ReplaceItem(
              path=path, source='function f() {\n  return 2;\n}', id=node, add_import=["import c from 'c';"])])
          assert not result.errors
          assert result.notice == IMPORT_NOTICE
          assert Path(path).read_text() == JS_SOURCE.replace('return 1', 'return 2')
  reason: Rollback-Test durch einen Test für „Engine scheitert vor dem Schreiben“ ersetzen
```
/answer 0a99-f4-0 allow
Control Request:
```yaml
id: 0a99-f5-0
toolName: python
arguments:
  script: |-
    import shutil, subprocess
    r = subprocess.run([shutil.which("pytest"), "-q", "--no-header", "-p", "no:cacheprovider", "tests"],
                       cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=1200)
    print(r.stdout[-5000:]); print(r.stderr[-1500:]); print("exit", r.returncode)
  reason: Gesamte Testsuite erneut ausführen
```
/answer 0a99-f5-0 allow
Control Request:
```yaml
id: 0a99-f6-f5
result:
  structuredContent:
    exit_code: 0
    stdout: |
      ..............................................F......................... [ 26%]
      ........................................................................ [ 52%]
      ........................................................................ [ 78%]
      .....................................................F.....              [100%]
      =================================== FAILURES ===================================
      ____________ test_rust_import_does_not_split_doc_comment_from_item _____________

      rust = <xy.ai.mcpc.tools.ast.rpc._rust.RustEngine object at 0x77565813f5c0>

          def test_rust_import_does_not_split_doc_comment_from_item(rust):
              tree = rust.parse('//! crate\n\n/// Point.\nstruct P {\n    x: i32,\n}\n')
      >       rust.apply_imports(tree, ['use std::fmt;'], [])

      tests/test_ast_imports_rpc.py:81:
      _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
      src/xy/ai/mcpc/tools/ast/rpc/_engine.py:202: in apply_imports
          return apply_rpc(self, tree, add, remove)
      _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _

      engine = <xy.ai.mcpc.tools.ast.rpc._rust.RustEngine object at 0x77565813f5c0>
      tree = Tree(engine=<xy.ai.mcpc.tools.ast.rpc._rust.RustEngine object at 0x77565813f5c0>, raw=None, source='//! crate\n\n/// Point.\nstruct P {\n    x: i32,\n}\n', path=None)
      add = ['use std::fmt;'], remove = []

          def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
              """Structure-aware handling for RPC engines that report import groups (Java ``import``, Rust ``use``).

              The engine only reads: its locate result yields every import statement's span, all
              removals and the one insertion are computed as edits on the original text and applied
              together. The candidate is validated and re-located before ``tree.source`` is written once,
              so a failure leaves the tree untouched.
              """
              adds = _unique([_rpc_statement(s) for s in add])
              removes = _unique([_rpc_statement(s) for s in remove])
              data = tree.source.encode('utf-8')
              present, top = _rpc_present(engine, tree)
              outcome = ImportOutcome()
              edits: list[tuple[int, int, bytes]] = []
              removed_keys: set[str] = set()
              for statement in removes:
                  key = normalize_import(statement)
                  if key in present:
                      edits.append((*line_extent(data, *present[key]), b''))
                      removed_keys.add(key)
                      outcome.removed.append(statement)
              missing = [s for s in adds if normalize_import(s) not in present.keys() - removed_keys]
              if missing:
                  edits.append(_rpc_insertion(data, present, top, '\n'.join(missing)))
                  outcome.added = missing
              if not edits:
                  return outcome
              for start, end, text in sorted(edits, key=lambda edit: (edit[0], edit[1]), reverse=True):
                  data = data[:start] + text + data[end:]
              source = data.decode('utf-8')
              error = engine.validate(source)
              if error:
      >           raise AstError(f'The import change would leave the file invalid: {error}')
      E           xy.ai.mcpc.tools.ast.base.AstError: The import change would leave the file invalid: expected square brackets

      src/xy/ai/mcpc/tools/ast/imports.py:320: AstError
      __________ test_malformed_insert_raises_plain_ast_error_not_ambiguous __________

      engine = <xy.ai.mcpc.tools.ast.rpc._java.JavaEngine object at 0x77565813d430>

          def test_malformed_insert_raises_plain_ast_error_not_ambiguous(engine):
              """A 422 (malformed edit) must not be mistaken for a 409 (AstAmbiguous)."""
              tree = engine.parse(VALID_SOURCE)
              foo = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo'))
      >       with pytest.raises(AstError) as excinfo:
      E       Failed: DID NOT RAISE <class 'xy.ai.mcpc.tools.ast.base.AstError'>

      tests/test_rpc_java_engine.py:89: Failed
      =========================== short test summary info ============================
      FAILED tests/test_ast_imports_rpc.py::test_rust_import_does_not_split_doc_comment_from_item
      FAILED tests/test_rpc_java_engine.py::test_malformed_insert_raises_plain_ast_error_not_ambiguous
      2 failed, 273 passed in 11.21s


      exit 1
```
/answer 0a99-f6-f5 allow
Text:
 Die Validierung hat hier korrekt eine kaputte Änderung abgelehnt, also hat meine Einfüge-Position für diesen Rust-Fall nicht gestimmt. Ich schaue nach, was der Prozess für die erste Node meldet.
ReasoningToken: 236
Control Request:
```yaml
id: 0a99-f7-0
toolName: python
arguments:
  script: |-
    import sys
    sys.path.insert(0, "/home/user/xyan/xy.ai.workbench/mcpc/src")
    from xy.ai.mcpc.tools.ast.generic import get_engine
    e = get_engine("rust")
    for src in ['//! crate\n\n/// Point.\nstruct P {\n    x: i32,\n}\n', '//! crate\n\nstruct P {\n    x: i32,\n}\n']:
        t = e.parse(src)
        for l in e.locate_all(t):
            print(repr(l.node_id), l.node_type, l.lineno, l.end_lineno)
        print('---')
  reason: Gemeldete Zeilenbereiche der ersten Rust-Node mit Doc-Kommentaren prüfen
```
/answer 0a99-f7-0 allow
Control Request:
```yaml
id: 0a99-f8-f7
result:
  structuredContent:
    exit_code: 0
    stdout: |
      'ModuleDoc' ModuleAttrs 1 1
      'P' ItemStruct 3 6
      ---
      'ModuleDoc' ModuleAttrs 1 1
      'P' ItemStruct 3 5
      ---
```
/answer 0a99-f8-f7 allow
Thinking:
Die Rust-Engine liefert `//!`-Moduldokumentation als eigenen Top-Level-Knoten (`ModuleAttrs`), woran sich meine Einfüge-Logik orientiert hatte; die Validierung hat dabei keine kaputte Datei zugelassen. Ich behandle diesen Typ nun als Prolog.


Control Request:
```yaml
id: 0a99-f9-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: rpc_insertion
    source: |-
      def _rpc_insertion(
              data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:
          """Edit that places ``code``: behind the last import, else behind ``package``/module docs, else before the first node."""
          block = code.encode('utf-8')
          if present:
              pos = _next_line_start(data, max(end for _, end in present.values()))
              text = b'\n' + block if pos == len(data) and not data.endswith(b'\n') else block + b'\n'
              return pos, pos, text
          spans = statement_spans(data)
          if spans and data[spans[0][0]:spans[0][1]].startswith(b'package'):
              pos = _next_line_start(data, spans[0][1])
              return pos, pos, b'\n' + block + b'\n'
          starts = _line_starts(data)
          prologue = [n for n in top if n.node_type in _RPC_PROLOGUE_TYPES]
          body = [n for n in top if n.node_type not in _RPC_PROLOGUE_TYPES]
          if body:
              pos = _leading_start(data, starts, min(body[0].lineno - 1, len(starts) - 1))
              return pos, pos, block + b'\n\n'
          if prologue:
              pos = starts[min(prologue[-1].end_lineno, len(starts) - 1)]
              return pos, pos, b'\n' + block + b'\n'
          pos = len(data)
          return pos, pos, (b'' if not data or data.endswith(b'\n') else b'\n') + block + b'\n'
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: 4Vz76R|AG3pif
    source: |-
      '#: Node types under which the RPC processes report a run of imports.'
      _RPC_GROUP_TYPES = {'ImportGroup', 'imports'}
      '#: Top-level node types that must stay in front of any import (Rust ``//!`` module docs).'
      _RPC_PROLOGUE_TYPES = {'ModuleAttrs'}
  reason: Rust-Moduldoku (ModuleAttrs) als Prolog behandeln und Konstante ergänzen
```
/answer 0a99-f9-0 allow
ReasoningToken: 753
Control Request:
```yaml
id: 0a99-fa-0
toolName: python
arguments:
  script: |-
    import shutil, subprocess
    r = subprocess.run([shutil.which("pytest"), "-q", "--no-header", "-p", "no:cacheprovider", "tests"],
                       cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=1200)
    print(r.stdout[-3500:]); print(r.stderr[-1500:]); print("exit", r.returncode)
  reason: Gesamte Testsuite erneut ausführen
```
/answer 0a99-fa-0 allow
Control Request:
```yaml
id: 0a99-fb-fa
result:
  structuredContent:
    exit_code: 0
    stdout: |
      ........................................................................ [ 26%]
      ........................................................................ [ 52%]
      ........................................................................ [ 78%]
      .....................................................F.....              [100%]
      =================================== FAILURES ===================================
      __________ test_malformed_insert_raises_plain_ast_error_not_ambiguous __________

      engine = <xy.ai.mcpc.tools.ast.rpc._java.JavaEngine object at 0x7767985293d0>

          def test_malformed_insert_raises_plain_ast_error_not_ambiguous(engine):
              """A 422 (malformed edit) must not be mistaken for a 409 (AstAmbiguous)."""
              tree = engine.parse(VALID_SOURCE)
              foo = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo'))
      >       with pytest.raises(AstError) as excinfo:
      E       Failed: DID NOT RAISE <class 'xy.ai.mcpc.tools.ast.base.AstError'>

      tests/test_rpc_java_engine.py:89: Failed
      =========================== short test summary info ============================
      FAILED tests/test_rpc_java_engine.py::test_malformed_insert_raises_plain_ast_error_not_ambiguous
      1 failed, 274 passed in 11.54s


      exit 1
```
/answer 0a99-fb-fa allow
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 413077, in: 160, out: 123667, reason: 0, read: 10950467, write: 289250, start: 1794266185971, end: 1794266196541, time: 10513, total: 975035, io: 0, control: 57, sent: 0, recv: 212
Ihr Ansatz ist umgesetzt, und die Teilausführung samt Rollback ist weg. Die Suite zeigt 274 bestandene Tests und einen Fehlschlag, und der ist derselbe wie zuvor: `test_malformed_insert_raises_plain_ast_error_not_ambiguous`. Er ruft `engine.insert` direkt auf, einen Pfad, den ich nicht angefasst habe. Gegen den Stand vor meinen Änderungen habe ich ihn nicht geprüft.

## Was sich geändert hat

Mein Rollback war eine Folge davon, dass ich `engine.insert` aufgerufen habe, das selbst in `tree.source` schreibt. Jetzt gilt in jeder Engine derselbe Ablauf:

1. Die vorhandenen Import-Statements werden gelesen. Die RPC-Engine liefert nur Zeilenbereiche, den Rest berechnet MCPC.
2. Alle Entfernungen und die eine Einfügung werden als Edits auf dem Originaltext berechnet und gemeinsam angewendet.
3. Das Ergebnis wird validiert und neu lokalisiert.
4. Erst dann wird `tree.source` genau einmal geschrieben.

Scheitert ein Schritt vorher, ist der Tree unverändert und es entsteht kein halber Zustand. Der Snapshot in `core.apply_imports` ist deshalb entfernt. Dort bleibt nur das Abfangen mit Log und `notice`.

Das hat auch den Java-Fallback und die Neuformatierung durch die Engine beseitigt: Die Formatierung bleibt erhalten, und für Java ohne vorhandene Imports braucht es keinen Sonderweg mehr. Die Python-, JavaScript- und generischen Pfade hatten diesen Ablauf schon. Bei Python wird der `ast`-Baum erst verändert, nachdem alle Statements geparst und geprüft sind.

## Zwei Dinge, die die neuen Tests gefunden haben

- Die Rust-Engine meldet `//!`-Moduldokumentation als eigene Top-Level-Node (`ModuleAttrs`). Mein erster Wurf hätte den Import davor gesetzt. Die Validierung hat das als ungültig abgelehnt, bevor etwas geschrieben wurde. `ModuleAttrs` gilt jetzt als Prolog.
- Ohne Imports wandert die Einfügeposition über direkt anhängende Kommentar- und Annotationszeilen nach oben. Ein Import landet so nicht zwischen `/// Doc` und der Deklaration.

Den früheren Rollback-Test habe ich durch einen Test ersetzt, der eine Engine nachstellt, die vor dem Schreiben scheitert. Neu sind außerdem Tests, dass die Formatierung erhalten bleibt und dass Entfernen und Hinzufügen in einem Durchgang funktioniert.
Prüfe den letzten Fehlschlag
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Ich prüfe zuerst, was der Aufruf tatsächlich tut, und danach, ob der Test auch ohne meine Änderungen scheitert.
Control Request:
```yaml
id: 0a99-fc-0
toolName: python
arguments:
  script: |-
    import sys
    sys.path.insert(0, "/home/user/xyan/xy.ai.workbench/mcpc/src")
    from xy.ai.mcpc.tools.ast.generic import get_engine
    e = get_engine("java")
    src = open("/home/user/xyan/xy.ai.workbench/mcpc/tests/test_rpc_java_engine.py").read()
    import re
    m = re.search(r"VALID_SOURCE = (.+?)\n\n", src, re.S)
    print(m.group(1))
    VALID = eval(m.group(1))
    t = e.parse(VALID)
    foo = next(l for l in e.locate_all(t) if l.node_id == 'Foo')
    try:
        n = e.insert(foo, 'int x;', 'before')
        print("returned", n); print(t.source)
    except Exception as ex:
        print("raised", type(ex).__name__, ex)
  reason: "Verhalten von engine.insert('int x;', 'before') auf der Java-Klasse direkt beobachten"
```
/answer 0a99-fc-0 allow
ReasoningToken: 472
Control Request:
```yaml
id: 0a99-fd-fc
result:
  structuredContent:
    exit_code: 1
    stdout: ""
    stderr: |
      Traceback (most recent call last):
        File "<stdin>", line 3, in <module>
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py", line 6, in <module>
          from xy.ai.mcpc.tools.ast import register_ast_tools
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/__init__.py", line 15, in <module>
          from xy.ai.mcpc.tools.ast import create, delete, edit_block, edit_marks, find, insert, read, replace, script, validate
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/create.py", line 6, in <module>
          from xy.ai.mcpc.tools.ast import core
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py", line 22, in <module>
          from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, AstTextError, Engine, Located, OutlineNode, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, resolve_by_prefix, to_dict
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py", line 24, in <module>
          from xy.ai.mcpc.tools.ast.imports import apply_generic
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py", line 12, in <module>
          from xy.ai.mcpc.tools.ast.base import AstError, Engine, Tree, check_no_control_chars
      ImportError: cannot import name 'AstError' from partially initialized module 'xy.ai.mcpc.tools.ast.base' (most likely due to a circular import) (/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py)
```
/answer 0a99-fd-fc allow
Control Request:
```yaml
id: 0a99-fe-0
toolName: python
arguments:
  script: |-
    import subprocess
    def sh(*a, cwd="/home/user/xyan/xy.ai.workbench/mcpc"):
        r = subprocess.run(a, cwd=cwd, capture_output=True, text=True); return r.stdout + r.stderr
    print(sh("git", "status", "--short"))
    print(sh("git", "log", "-3", "--format=%h %ad %s", "--date=short"))
    print(sh("git", "diff", "--stat", "HEAD", "--", "src/xy/ai/mcpc/tools/ast/rpc", "tests/test_rpc_java_engine.py"))
  reason: "Git-Status: welche Dateien ich verändert habe und ob RPC-Insert-Code oder der Test betroffen sind"
```
/answer 0a99-fe-0 deny kein git
Text:
 Das ist unerwartet: Ein Import in `base.py` Zeile 24 erzeugt einen Zirkelbezug, den es in meinem Stand nicht geben sollte. Ich sehe mir den Kopf von `base.py` an.
Control Request:
```yaml
id: 0a99-ff-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
  lineno: 1
  end_lineno: 40
  reason: "Kopf von base.py ansehen, um die Herkunft des Modul-Imports von apply_generic zu klären"
```
/answer 0a99-ff-0 allow
Control Request:
```yaml
id: 0a99-100-ff
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: ZjoNfB|uJIfID
        code: "'Engine-agnostic model shared by every ``ast_*`` tool.\\n\\nThe tools address nodes by *selector* (id, type, name, line\\nrange or parent type) and never touch a concrete parser. Two engines implement\\n:class:`Engine`: a Python one built on the standard-library ``ast`` module and a\\ngeneric tree-sitter one for every other language/format. :mod:`.core` picks the\\nengine per file extension and exposes a thin facade the tools call.\\n\\nA :class:`Tree` carries a back-reference to the engine that produced it, so\\nevery helper here can dispatch to the right engine without the tools knowing\\nwhich one is in play.\\n'"
      - id: oRSlHi|kUrRiw
        code: |-
          from __future__ import annotations
          import difflib
          import hashlib
          import re
          import string
          from abc import ABC, abstractmethod
          from dataclasses import asdict, dataclass, field
          from pathlib import Path
          from typing import TYPE_CHECKING, Any
      - id: K1Mt8u|UQj1Lh
        code: |-
          if TYPE_CHECKING:
              from xy.ai.mcpc.tools.ast.imports import ImportOutcome
      - id: VkeGz2|sMKWME
        code: from xy.ai.mcpc.tools.ast.imports import apply_generic
      - id: AstError.RDaRob|BEUNOZ
        code: "'A user-facing, path-free error raised by the AST tools.'"
      - id: AstAmbiguous.FSy0jO|jchvnz
        code: '''Raised when a text-based node search (no id given) finds several\n    unrelated candidates instead of a single node.'''
      - id: AstAmbiguous.init.VltHfz|3ETIUM
        code: |-
          super().__init__(message)
          self.candidates = candidates
      - id: AstTextError.xHQ70j|YI3gjQ
        code: "\"Raised when a text/marker-based edit's search text could not be applied.\\n\\n    Carries the shared-matcher's diagnosis (see ``tools._text_match``): ``reason``\\n    classifies the cause (e.g. ``whitespace_mismatch``, ``content_changed``,\\n    ``guard_rejected``, ``marker_order``, ``ambiguous``, ``not_found``);\\n    ``corrected_text`` is a verified fix (whitespace-only difference), ``guess``\\n    an unverified single-candidate heuristic, ``next_step`` a fallback instruction.\\n    \""
```
/answer 0a99-100-ff allow
ReasoningToken: 622
Control Request:
```yaml
id: 0a99-101-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: apply_generic|ast\.imports import
    include:
    - '*.py'
    exclude:
    - '**/__pycache__/**'
    limit: 30
  reason: Alle Verwendungen von apply_generic und Importe des imports-Moduls auflisten
```
/answer 0a99-101-0 allow
Control Request:
```yaml
id: 0a99-102-101
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: base.py
        matches:
        - from xy.ai.mcpc.tools.ast.imports import ImportOutcome
        - from xy.ai.mcpc.tools.ast.imports import apply_generic
        - "return apply_generic(self, tree, add, remove)"
      - path: replace.py
        matches:
        - "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args"
      - path: delete.py
        matches:
        - "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args"
      - path: rpc/_engine.py
        matches:
        - "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, apply_rpc"
      - path: generic/_javascript.py
        matches:
        - "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, apply_treesitter"
      - path: edit_marks.py
        matches:
        - "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args"
      - path: edit_block.py
        matches:
        - "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args"
      - path: python/_imports.py
        matches:
        - from xy.ai.mcpc.tools.ast.imports import ImportOutcome
      - path: python/_engine.py
        matches:
        - from xy.ai.mcpc.tools.ast.imports import ImportOutcome
      - path: core.py
        matches:
        - from xy.ai.mcpc.tools.ast.imports import ImportOutcome
      - path: insert.py
        matches:
        - "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, import_entry, import_notice, parse_import_args"
      - path: imports.py
        matches:
        - ":func:`apply_generic`, which only verifies and removes whole lines."
        - "'apply_generic',"
        - "def apply_generic(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
```
/answer 0a99-102-101 allow
ReasoningToken: 975
Control Request:
```yaml
id: 0a99-103-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    ids:
    - K1Mt8u|UQj1Lh
    - VkeGz2|sMKWME
    - Engine.apply_imports
  reason: Modul-Import und die Methode Engine.apply_imports in base.py im Detail lesen
```
/answer 0a99-103-0 allow
Control Request:
```yaml
id: 0a99-104-103
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      nodes:
      - id: K1Mt8u|UQj1Lh
        type: statements
        code: |-
          if TYPE_CHECKING:
              from xy.ai.mcpc.tools.ast.imports import ImportOutcome
      - id: VkeGz2|sMKWME
        type: imports
        code: from xy.ai.mcpc.tools.ast.imports import apply_generic
      - id: Engine.apply_imports
        type: FunctionDef
        code: |-
          def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> 'ImportOutcome':
              """Add missing / remove present import statements (``addImport``/``removeImport``).

                  The default only verifies and removes whole lines; engines with structural
                  knowledge of their language's import syntax override this."""
              return apply_generic(self, tree, add, remove)
```
/answer 0a99-104-103 allow
Thinking:
Die Methode in `base.py` ist in Ordnung, aber der Modulimport von `apply_generic` steht vor `AstError` und bildet einen echten Zirkelbezug. Ich prüfe jetzt, ob die Suite im aktuellen Stand noch läuft.


ReasoningToken: 2434
Control Request:
```yaml
id: 0a99-105-0
toolName: python
arguments:
  script: |-
    import shutil, subprocess
    r = subprocess.run([shutil.which("pytest"), "-q", "--no-header", "-p", "no:cacheprovider", "tests", "-x", "-q"],
                       cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=1200)
    print(r.stdout[-2500:]); print(r.stderr[-800:]); print("exit", r.returncode)
    import os
    p = "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/"
    import time
    for f in ["base.py", "imports.py", "core.py"]:
        print(f, time.ctime(os.path.getmtime(p + f)))
    print(sorted(os.listdir(p + "__pycache__"))[:3])
  reason: Aktuellen Teststand und Änderungszeitpunkte der betroffenen Dateien prüfen
```
/answer 0a99-105-0 deny ich habe den import nach oben gezogen weil import nicht in methodenbodies gehören
Thinking:
Der Zirkelbezug `base` → `imports` → `base` ist die eigentliche Ursache, deshalb lasse ich `base.py` nicht mehr `imports.py` kennen. Stattdessen meldet die Basis-Engine „nicht unterstützt“, und `core` wählt dann den generischen Fallback.


Control Request:
```yaml
id: 0a99-106-0
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    id: VkeGz2|sMKWME
  reason: Zirkulären Modul-Import von apply_generic aus base.py entfernen
```
/answer 0a99-106-0 allow
Control Request:
```yaml
id: 0a99-106-0
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    id: VkeGz2|sMKWME
  reason: Zirkulären Modul-Import von apply_generic aus base.py entfernen
```
/answer 0a99-106-0 allow
ReasoningToken: 794
Control Request:
```yaml
id: 0a99-107-0
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
    id: Engine.apply_imports
    source: |-
      def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> 'ImportOutcome':
          """Add missing / remove present import statements (``addImport``/``removeImport``).

          Engines with structural knowledge of their language's import syntax override this,
          computing all changes first and writing ``tree`` once. The default raises
          :class:`NotImplementedError`, upon which :func:`core.apply_imports` uses the generic
          check-only fallback (kept out of this module to avoid an import cycle).
          """
          raise NotImplementedError(f'{self.name} has no import support')
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: apply_imports
    source: |-
      def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:
          """Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``.

          Independent of the node edit: engines compute all import changes first and write them
          once, so a failure leaves the tree as the node edit left it. It is then reported as
          ``unchecked``, which puts the notice on the batch. Engines without dedicated support
          get the generic check-only fallback.
          """
          if not add and not remove:
              return None
          try:
              try:
                  return tree.engine.apply_imports(tree, add or [], remove or [])
              except NotImplementedError:
                  return apply_generic(tree.engine, tree, add or [], remove or [])
          except Exception:  # noqa: BLE001 - never fail the edit because of its imports
              logger.warning('Import handling failed for %s', tree.path, exc_info=True)
              return ImportOutcome(unchecked=True)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: Vx2JFF|mvdSSl
    source: |-
      from __future__ import annotations
      import hashlib
      import logging
      import threading
      from dataclasses import dataclass
      from pathlib import Path
      from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, AstTextError, Engine, Located, OutlineNode, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, resolve_by_prefix, to_dict
      from xy.ai.mcpc.tools.ast.imports import ImportOutcome, apply_generic
      from xy.ai.mcpc.tools.ast import generic
      from xy.ai.mcpc.tools.ast import python

      logger = logging.getLogger(__name__)
  reason: Engine-Default auf NotImplementedError umstellen und Fallback in core.apply_imports verlagern
```
/answer 0a99-107-0 allow
Control Request:
```yaml
id: 0a99-108-107
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/base.py
      result: success
      id: Engine.apply_imports
      new_id: Engine.apply_imports
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      result: success
      id: apply_imports
      new_id: apply_imports
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      error: No node matched the selector. verify the node with `search` or `ast_find`
      id: Vx2JFF|mvdSSl
```
/answer 0a99-108-107 allow
Text:
 Die ID des Import-Blocks in `core.py` hat sich geändert. Ich suche sie neu und lese dabei den aktuellen Stand.
ReasoningToken: 35
Control Request:
```yaml
id: 0a99-109-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  node_type: imports
  reason: Aktuellen Import-Block von core.py lesen
```
/answer 0a99-109-0 allow
Control Request:
```yaml
id: 0a99-10a-109
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
      - id: OsZYRc|LC95ci
        type: imports
        code: |-
          from __future__ import annotations
          import hashlib
          import logging
          import threading
          from dataclasses import dataclass
          from pathlib import Path
          from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, AstTextError, Engine, Located, OutlineNode, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, resolve_by_prefix, to_dict
          from xy.ai.mcpc.tools.ast.imports import ImportOutcome
          from xy.ai.mcpc.tools.ast import generic
      - id: 64y9R1|MAcfZT
        type: imports
        code: from xy.ai.mcpc.tools.ast import python
```
/answer 0a99-10a-109 allow
Control Request:
```yaml
id: 0a99-10b-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    id: OsZYRc|LC95ci
    old_text: from xy.ai.mcpc.tools.ast.imports import ImportOutcome
    new_text: "from xy.ai.mcpc.tools.ast.imports import ImportOutcome, apply_generic"
  reason: apply_generic in core.py importieren
```
/answer 0a99-10b-0 allow
ReasoningToken: 51
Control Request:
```yaml
id: 0a99-10c-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: "^\\s+(from [\\w.]+ import|import [\\w.]+)"
    include:
    - imports.py
    - _imports.py
    - test_ast_imports.py
    - test_ast_imports_rpc.py
    - core.py
    - _javascript.py
    exclude:
    - '**/__pycache__/**'
    - '**/.venv/**'
    limit: 20
    intent: editing
  reason: Verbliebene Imports in Funktionsrümpfen in meinen Dateien finden
```
/answer 0a99-10c-0 allow
Control Request:
```yaml
id: 0a99-10d-10c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc
```
/answer 0a99-10d-10c allow
Text:
 Keine Imports mehr in Funktionsrümpfen in meinen Dateien. Jetzt die Suite.
ReasoningToken: 169
Control Request:
```yaml
id: 0a99-10e-0
toolName: python
arguments:
  script: |-
    import shutil, subprocess
    r = subprocess.run([shutil.which("pytest"), "-q", "--no-header", "-p", "no:cacheprovider", "tests"],
                       cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=1200)
    print(r.stdout[-3500:]); print(r.stderr[-800:]); print("exit", r.returncode)
  reason: Gesamte Testsuite nach Auflösung des Zirkelbezugs ausführen
```
/answer 0a99-10e-0 allow
Control Request:
```yaml
id: 0a99-10f-10e
result:
  structuredContent:
    exit_code: 0
    stdout: |
      ........................................................................ [ 26%]
      ........................................................................ [ 52%]
      ........................................................................ [ 78%]
      .....................................................F.....              [100%]
      =================================== FAILURES ===================================
      __________ test_malformed_insert_raises_plain_ast_error_not_ambiguous __________

      engine = <xy.ai.mcpc.tools.ast.rpc._java.JavaEngine object at 0x7472b29856d0>

          def test_malformed_insert_raises_plain_ast_error_not_ambiguous(engine):
              """A 422 (malformed edit) must not be mistaken for a 409 (AstAmbiguous)."""
              tree = engine.parse(VALID_SOURCE)
              foo = next((loc for loc in engine.locate_all(tree) if loc.node_id == 'Foo'))
      >       with pytest.raises(AstError) as excinfo:
      E       Failed: DID NOT RAISE <class 'xy.ai.mcpc.tools.ast.base.AstError'>

      tests/test_rpc_java_engine.py:89: Failed
      =========================== short test summary info ============================
      FAILED tests/test_rpc_java_engine.py::test_malformed_insert_raises_plain_ast_error_not_ambiguous
      1 failed, 274 passed in 10.11s


      exit 1
```
/answer 0a99-10f-10e allow
Text:
 Der Zirkelbezug ist beseitigt, und die Suite läuft wieder. Jetzt zum eigentlichen Fehlschlag. Ich beobachte direkt, was `engine.insert('int x;', 'before')` auf der Java-Klasse zurückgibt.
ReasoningToken: 69
Control Request:
```yaml
id: 0a99-110-0
toolName: python
arguments:
  script: |-
    import sys, re
    sys.path.insert(0, "/home/user/xyan/xy.ai.workbench/mcpc/src")
    from xy.ai.mcpc.tools.ast.generic import get_engine
    e = get_engine("java")
    src = open("/home/user/xyan/xy.ai.workbench/mcpc/tests/test_rpc_java_engine.py").read()
    VALID = eval(re.search(r"VALID_SOURCE = (.+?)\n\n", src, re.S).group(1))
    print(repr(VALID))
    t = e.parse(VALID)
    foo = next(l for l in e.locate_all(t) if l.node_id == 'Foo')
    for code, pos in [('int x;', 'before'), ('int x;', 'after'), ('void m() {', 'before'), ('this is not java', 'before')]:
        t = e.parse(VALID); foo = next(l for l in e.locate_all(t) if l.node_id == 'Foo')
        try:
            n = e.insert(foo, code, pos); print(code, pos, "-> returned", n, repr(t.source))
        except Exception as ex:
            print(code, pos, "-> raised", type(ex).__name__, str(ex)[:150])
  reason: Verhalten von engine.insert mit verschiedenen Eingaben vor/nach der Java-Klasse beobachten
```
/answer 0a99-110-0 allow
Control Request:
```yaml
id: 0a99-111-110
result:
  structuredContent:
    exit_code: 0
    stdout: |+
      'public class Foo {\n    void bar() {\n        System.out.println(1);\n    }\n}\n'
      int x; before -> returned 1 'int x;\n\npublic class Foo {\n\n  void bar() {\n    System.out.println(1);\n  }\n}\n'
      int x; after -> returned 1 'public class Foo {\n\n  void bar() {\n    System.out.println(1);\n  }\n}\n\nint x;\n'
      void m() { before -> raised AstError invalid type declaration: Encountered unexpected token: "void" "void"
          at line 1, column 1.

      Was expecting one of:

          "@"
          "class"
          "enum"

      this is not java before -> raised AstError invalid type declaration: Encountered unexpected token: "this" "this"
          at line 1, column 1.

      Was expecting one of:

          "@"
          "class"
          "enum"
```
/answer 0a99-111-110 allow
Thinking:
Der Java-Prozess lässt `int x;` vor einer Top-Level-Klasse durch, obwohl das kein gültiges Java ist, während er andere ungültige Eingaben korrekt ablehnt. Ich sehe mir jetzt die Java-Quellen an, wie Top-Level-Einfügungen behandelt werden.


ReasoningToken: 402
Control Request:
```yaml
id: 0a99-112-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
    pattern: invalid type declaration|parseTypeDeclaration|parseBodyDeclaration|insertRelative|KIND_IMPORTS|insertTopLevel
    include:
    - '*.java'
    exclude:
    - '**/test/**'
    limit: 25
  reason: "Stelle im Java-Engine-Quelltext finden, die Top-Level-Einfügungen parst und validiert"
```
/answer 0a99-112-0 allow
Control Request:
```yaml
id: 0a99-113-112
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java/src
      matches:
      - path: xy/ai/mcpc/ast/openapi/AppendInfoNodesValidateClient.java
        matches:
        - "xy.ai.mcpc.ast.openapi.response.nodes.nodeid.insert.NodesNodeIdInsertResponse insertRelativeToNode(String nodeId, xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest request);"
      - path: xy/ai/mcpc/ast/openapi/AppendInfoNodesValidateServer.java
        matches:
        - "protected abstract xy.ai.mcpc.ast.openapi.response.nodes.nodeid.insert.NodesNodeIdInsertResponse insertRelativeToNode(String nodeId, xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest request);"
        - "xy.ai.mcpc.ast.openapi.response.nodes.nodeid.insert.NodesNodeIdInsertResponse response = insertRelativeToNode(nodeId, request);"
      - path: xy/ai/mcpc/ast/openapi/AppendInfoNodesValidateClientImpl.java
        matches:
        - "public xy.ai.mcpc.ast.openapi.response.nodes.nodeid.insert.NodesNodeIdInsertResponse insertRelativeToNode(String nodeId, xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest request) {"
        - "throw new RuntimeException(\"request failed: insertRelativeToNode\", e);"
      - path: xy/ai/mcpc/ast/engine/JavaAstServer.java
        matches:
        - "protected NodesNodeIdInsertResponse insertRelativeToNode(String nodeId, InsertRequest request) {"
      - path: xy/ai/mcpc/ast/engine/NodeLocator.java
        matches:
        - "addGroup(imports, NodeGroup.KIND_IMPORTS, \"CompilationUnit\", \"\", start, i, used, out);"
      - path: xy/ai/mcpc/ast/engine/JavaAstEngine.java
        matches:
        - ParseResult<TypeDeclaration<?>> result = parser.parseTypeDeclaration(code);
        - "// parseTypeDeclaration does not accept records, the compilation unit grammar does"
        - "return unwrap(parser.parseBodyDeclaration(code), \"member declaration\");"
        - "return NodeGroup.KIND_IMPORTS.equals(kind) ? parseImportsFragment(code) : parseMembersFragment(code);"
        - "* Parses {@code code} as one or more import declarations, for an {@link NodeGroup#KIND_IMPORTS} group."
      - path: xy/ai/mcpc/ast/engine/NodeGroup.java
        matches:
        - public static final String KIND_IMPORTS = "ImportGroup";
```
/answer 0a99-113-112 allow
Control Request:
```yaml
id: 0a99-114-0
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
  regexp: parseTypeDeclaration|parseMembersFragment|parseFragment|isTopLevel|CompilationUnit
  node_type: method_declaration
  reason: "Methoden der Java-Engine lesen, die Fragmente je nach Zielknoten parsen (Top-Level vs. Member)"
```
/answer 0a99-114-0 allow
Control Request:
```yaml
id: 0a99-115-114
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes: []
```
/answer 0a99-115-114 allow
ReasoningToken: 49
Control Request:
```yaml
id: 0a99-116-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine
    pattern: parseTypeDeclaration|parseMembersFragment|parseBodyDeclaration|private .*parse\w*\(|insert\w*\(|isTopLevel|getParentNode|CompilationUnit\)
    include:
    - JavaAstEngine.java
    limit: 30
    intent: editing
  reason: Zeilen der Parse-/Insert-Logik in JavaAstEngine.java lokalisieren
```
/answer 0a99-116-0 allow
Control Request:
```yaml
id: 0a99-117-116
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine
      matches:
      - path: JavaAstEngine.java
        nodes:
        - id: JavaAstEngine.parseTypeFragment
          matches:
          - ParseResult<TypeDeclaration<?>> result = parser.parseTypeDeclaration(code);
          - "// parseTypeDeclaration does not accept records, the compilation unit grammar does"
        - id: JavaAstEngine.parseBodyFragment
          matches:
          - "return unwrap(parser.parseBodyDeclaration(code), \"member declaration\");"
        - id: JavaAstEngine.insert
          matches:
          - "public int insert(AddressableNode target, String code, String position) {"
          - "return insertGroup(group, code, position);"
        - id: JavaAstEngine.parseInsertionFragments
          matches:
          - "private List<Node> parseInsertionFragments(Node sample, String code) {"
          - return parseMembersFragment(code);
        - id: JavaAstEngine.insertGroup
          matches:
          - "private int insertGroup(NodeGroup group, String code, String position) {"
        - id: JavaAstEngine.parseGroupFragments
          matches:
          - "private List<Node> parseGroupFragments(String kind, String code) {"
          - "return NodeGroup.KIND_IMPORTS.equals(kind) ? parseImportsFragment(code) : parseMembersFragment(code);"
        - id: JavaAstEngine.parseMembersFragment
          matches:
          - "public List<Node> parseMembersFragment(String code) {"
        - id: JavaAstEngine.unwrapSingleStatementBlocks
          matches:
          - Node parent = block.getParentNode().orElse(null);
```
/answer 0a99-117-116 allow
Control Request:
```yaml
id: 0a99-118-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.parseInsertionFragments
    - JavaAstEngine.parseMembersFragment
    - JavaAstEngine.parseTypeFragment
  reason: "Logik lesen, die entscheidet, wie ein Einfüge-Fragment relativ zu einem Top-Level-Typ geparst wird"
```
/answer 0a99-118-0 allow
Control Request:
```yaml
id: 0a99-119-118
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: JavaAstEngine.parseInsertionFragments
        type: MethodDeclaration
        code: |2
            private List<Node> parseInsertionFragments(Node sample, String code) {
              if (sample instanceof BodyDeclaration<?> && !(sample instanceof TypeDeclaration<?>))
                try {
                  return List.of(parseReplacementFor(sample, code));
                } catch (AstEngineException e) {
                  if (e.kind() != AstEngineException.Kind.SYNTAX)
                    throw e;
                  return parseMembersFragment(code);
                }
              return List.of(parseReplacementFor(sample, code));
            }
      - id: JavaAstEngine.parseMembersFragment
        type: MethodDeclaration
        code: |2
            public List<Node> parseMembersFragment(String code) {
              CompilationUnit wrapper = unwrap(parser.parse("class __Wrapper__ { " + code + " }"), "member declarations");
              ClassOrInterfaceDeclaration decl = (ClassOrInterfaceDeclaration) wrapper.getType(0);
              List<Node> result = new ArrayList<>();
              for (BodyDeclaration<?> m : decl.getMembers()) result.add(m.clone());
              if (result.isEmpty())
                throw new AstEngineException(AstEngineException.Kind.SYNTAX, "no member declarations found: " + code);
              return result;
            }
      - id: JavaAstEngine.parseTypeFragment
        type: MethodDeclaration
        code: |2
            public TypeDeclaration<?> parseTypeFragment(String code) {
              ParseResult<TypeDeclaration<?>> result = parser.parseTypeDeclaration(code);
              if (result.isSuccessful() && result.getResult().isPresent())
                return unwrap(result, "type declaration");
              // parseTypeDeclaration does not accept records, the compilation unit grammar does
              ParseResult<CompilationUnit> unit = parser.parse(code);
              if (unit.isSuccessful() && unit.getResult().isPresent()) {
                CompilationUnit cu = unit.getResult().get();
                if (cu.getTypes().size() == 1 && cu.getImports().isEmpty() && cu.getPackageDeclaration().isEmpty()) {
                  TypeDeclaration<?> type = cu.getType(0).clone();
                  unwrapSingleStatementBlocks(type);
                  return type;
                }
              }
              return unwrap(result, "type declaration");
            }
```
/answer 0a99-119-118 allow
Control Request:
```yaml
id: 0a99-11a-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
    ids:
    - JavaAstEngine.insert
    - JavaAstEngine.parseReplacementFor
  reason: "insert und parseReplacementFor lesen, um zu sehen, warum \"int x;\" vor einem Top-Level-Typ akzeptiert wird"
```
/answer 0a99-11a-0 allow
Control Request:
```yaml
id: 0a99-11b-11a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java/src/xy/ai/mcpc/ast/engine/JavaAstEngine.java
      nodes:
      - id: JavaAstEngine.insert
        type: MethodDeclaration
        code: |2
            @SuppressWarnings("unchecked")
            public int insert(AddressableNode target, String code, String position) {
              if (target.astNode instanceof NodeGroup group)
                return insertGroup(group, code, position);
              Node node = (Node) target.astNode;
              List<Node> fragments = parseInsertionFragments(node, code);
              NodeList<Node> container = (NodeList<Node>) target.container;
              int idx = container.indexOf(node);
              if (idx < 0)
                throw new AstEngineException(AstEngineException.Kind.CONFLICT, "node no longer present: " + target.id);
              if ("after".equals(position))
                idx++;
              container.addAll(idx, fragments);
              return fragments.size();
            }
      - id: JavaAstEngine.parseReplacementFor
        type: MethodDeclaration
        code: |2
            public Node parseReplacementFor(Node sample, String code) {
              if (sample instanceof ImportDeclaration)
                return parseImportFragment(code);
              if (sample instanceof TypeDeclaration<?>)
                return parseTypeFragment(code);
              if (sample instanceof EnumConstantDeclaration)
                return parseEnumConstantFragment(code);
              return parseBodyFragment(code);
            }
```
/answer 0a99-11b-11a allow
ReasoningToken: 160
Control Request:
```yaml
id: 0a99-11c-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
    pattern: \.(jar|class)$|build|target
    recursive: true
  reason: "Gebautes Artefakt der Java-Engine lokalisieren, um Alter/Aktualität gegenüber dem Quelltext zu beurteilen"
```
/answer 0a99-11c-0 allow
Control Request:
```yaml
id: 0a99-11d-11c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/java
      entries:
      - "./.bin/xy/ai/mcpc/ast/engine: (15 files)"
      - AddressableNode.class
      - "./.bin/xy/ai/mcpc/ast/openapi: (4 files)"
      - AppendInfoNodesValidateClient.class
      - "./.bin/xy/ai/mcpc/ast/openapi/components: (6 files)"
      - CandidatesList.class
      - "./.bin/xy/ai/mcpc/ast/openapi/request/nodes/nodeid/insert/post/json: (2 files)"
      - InsertRequest.class
      - "./.bin/xy/ai/mcpc/ast/openapi/request/nodes/post/json: (1 file)"
      - LocateRequest.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/append: (1 file)"
      - AppendResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/append/code200/json: (2 files)"
      - AppendResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/append/code422/json: (1 file)"
      - AppendResponseCode422Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/info: (1 file)"
      - InfoResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/info/code200/json: (2 files)"
      - EngineInfo.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes: (1 file)"
      - NodesResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/code200/json: (3 files)"
      - LocateResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/code422/json: (1 file)"
      - NodesResponseCode422Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid: (1 file)"
      - NodesNodeIdResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code200/json: (1 file)"
      - NodesNodeIdResponseCode200Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code404/json: (1 file)"
      - NodesNodeIdResponseCode404Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code409/json: (1 file)"
      - NodesNodeIdResponseCode409Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/code422/json: (1 file)"
      - NodesNodeIdResponseCode422Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete: (1 file)"
      - NodesNodeIdDeleteResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code200/json: (1 file)"
      - NodesNodeIdDeleteResponseCode200Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code404/json: (1 file)"
      - NodesNodeIdDeleteResponseCode404Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code409/json: (1 file)"
      - NodesNodeIdDeleteResponseCode409Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/delete/code422/json: (1 file)"
      - NodesNodeIdDeleteResponseCode422Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert: (1 file)"
      - NodesNodeIdInsertResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code200/json: (2 files)"
      - InsertResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code404/json: (1 file)"
      - NodesNodeIdInsertResponseCode404Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code409/json: (1 file)"
      - NodesNodeIdInsertResponseCode409Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/insert/code422/json: (1 file)"
      - NodesNodeIdInsertResponseCode422Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace: (1 file)"
      - NodesNodeIdReplaceResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code200/json: (1 file)"
      - NodesNodeIdReplaceResponseCode200Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code404/json: (1 file)"
      - NodesNodeIdReplaceResponseCode404Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code409/json: (1 file)"
      - NodesNodeIdReplaceResponseCode409Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/nodes/nodeid/replace/code422/json: (1 file)"
      - NodesNodeIdReplaceResponseCode422Json.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/validate: (1 file)"
      - ValidateResponse.class
      - "./.bin/xy/ai/mcpc/ast/openapi/response/validate/code200/json: (2 files)"
      - ValidateResponse.class
      - "./libs: (6 files)"
      - jackson-annotations-3.0-rc5.jar
      warning: Result reduced because the total number of entries (70) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 0a99-11d-11c deny Keine informationen aus den RPC engine implementierungen.
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 450204, in: 190, out: 138130, reason: 739, read: 14551664, write: 311884, start: 1794266751621, end: 1794266767464, time: 15790, total: 1114902, io: 0, control: 53, sent: 0, recv: 244
ReasoningToken: 739
Der Fehlschlag liegt nicht an meinen Änderungen, sondern an einer Abweichung zwischen dem Java-Prozess und dem, was der Test erwartet. Die Ursache im Prozess konnte ich nicht klären.

## Beobachtung

`engine.insert(Foo, 'int x;', 'before')` liefert Erfolg (`1` Einheit eingefügt) und schreibt ungültiges Java:

```
int x;

public class Foo { ... }
```

Der Test erwartet an dieser Stelle einen `AstError`. Andere ungültige Eingaben lehnt der Prozess weiterhin ab, etwa `void m() {` oder `this is not java` mit „invalid type declaration“. Nur der Fall „Feld vor einer Top-Level-Klasse“ rutscht durch.

## Warum ich das nicht meinen Änderungen zuschreibe

- `RpcEngine.insert` reicht den Aufruf unverändert an den Prozess durch. In `rpc/_engine.py` habe ich nur `apply_imports` ergänzt.
- Der Fehlschlag trat schon im ersten Gesamtlauf auf, als mein Code im RPC-Teil nur diese eine Methode hinzugefügt hatte.
- Wie sich der Prozess zuvor verhalten hat, habe ich nicht geprüft. Einen Lauf gegen den Stand vor meinen Änderungen konnte ich ohne git nicht machen.

Den Prozess-Quelltext lese ich nicht weiter, weil du keine Informationen aus den RPC-Engine-Implementierungen willst. Ich habe vor deiner Ansage zwei Methoden von `JavaAstEngine.java` gelesen. Danach hätte die Quelle diese Eingabe eigentlich abweisen müssen. Das spricht dafür, dass der laufende Prozess nicht zur Quelle passt, etwa weil das Artefakt veraltet ist. Das ist eine Vermutung und kein Befund.

Der nächste Schritt wäre, die Java-Engine neu zu bauen (`run.sh --build`) und den Test zu wiederholen. Das habe ich nicht getan, weil es den Prozess betrifft, den du aus meinem Blickfeld nehmen willst.

## Gefunden beim Prüfen: ein Zirkelbezug, den ich verursacht hatte

Dein Hochziehen des Imports in `base.py` hat gezeigt, dass `base` → `imports` → `base` ein Zyklus war. Mein Import im Methodenrumpf hatte ihn nur verdeckt. Ich habe ihn an der Wurzel gelöst:

- `Engine.apply_imports` in `base.py` wirft jetzt `NotImplementedError` und kennt `imports.py` nicht mehr.
- `core.apply_imports` fängt das ab und nutzt den generischen Fallback `apply_generic`. `core` importiert `imports.py` ohnehin.
- In meinen Dateien steht kein Import mehr in einem Funktionsrumpf.

Die Suite zeigt 274 bestandene Tests und den einen Fehlschlag aus dem Java-Test.
Was ist der Grund für den verbelibenden Fehlschlag?
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Die Ursache kenne ich nicht. Ich habe sie bisher nur eingegrenzt. Eine Möglichkeit kann ich ohne Blick in die RPC-Implementierung noch ausschließen: dass meine Tests den gemeinsamen Java-Prozess beeinflussen. Dazu lasse ich den Test isoliert laufen.
ReasoningToken: 378
Control Request:
```yaml
id: 0a99-11e-0
toolName: python
arguments:
  script: |-
    import shutil, subprocess
    def run(*args):
        r = subprocess.run([shutil.which("pytest"), "-q", "--no-header", "-p", "no:cacheprovider", *args],
                           cwd="/home/user/xyan/xy.ai.workbench/mcpc", capture_output=True, text=True, timeout=600)
        return r.stdout.strip().splitlines()[-3:]
    print("alone      :", run("tests/test_rpc_java_engine.py"))
    print("single test:", run("tests/test_rpc_java_engine.py::test_malformed_insert_raises_plain_ast_error_not_ambiguous"))
  reason: "Prüfen, ob der Fehlschlag auch ohne meine Test-Dateien im selben Lauf auftritt (Ausschluss von Seiteneffekten)"
```
/answer 0a99-11e-0 allow
Control Request:
```yaml
id: 0a99-11f-11e
result:
  structuredContent:
    exit_code: 0
    stdout: |
      alone      : ['=========================== short test summary info ============================', 'FAILED tests/test_rpc_java_engine.py::test_malformed_insert_raises_plain_ast_error_not_ambiguous', '1 failed, 9 passed in 0.14s']
      single test: ['=========================== short test summary info ============================', 'FAILED tests/test_rpc_java_engine.py::test_malformed_insert_raises_plain_ast_error_not_ambiguous', '1 failed in 0.10s']
```
/answer 0a99-11f-11e allow
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 454750, in: 194, out: 139697, reason: 0, read: 15054518, write: 314859, start: 1794266899688, end: 1794266905801, time: 6062, total: 1131520, io: 0, control: 51, sent: 0, recv: 145
Den Grund kenne ich nicht, ich habe nur eingegrenzt, wo er liegt.

**Gesichert:**
- Der Test scheitert auch allein und in `test_rpc_java_engine.py` ohne meine Test-Dateien. Dass meine Tests den Java-Prozess beeinflussen, ist damit ausgeschlossen. Der Prozess bekommt den Quelltext ohnehin bei jedem Aufruf mit.
- Der Java-Prozess nimmt `engine.insert(Foo, 'int x;', 'before')` an und schreibt `int x;` vor die Klasse. Der Test erwartet an dieser Stelle einen `AstError`.
- `RpcEngine.insert` leitet den Aufruf unverändert weiter. Meine Änderungen an `rpc/_engine.py` beschränken sich auf die neue Methode `apply_imports`.

**Nicht gesichert:**
- **Ob der Test früher bestanden hat:** Einen Lauf gegen den Stand vor meinen Änderungen konnte ich nicht machen, weil ich ohne git keinen Vergleich habe.
- **Ob die Ursache ein veralteter Build ist:** Die Quelle, die ich vor deiner Ansage gelesen hatte, hätte diese Eingabe eigentlich ablehnen müssen. Das spricht für eine Abweichung zwischen laufendem Prozess und Quelle, belegt sie aber nicht.

Um das zu klären, bräuchte ich eines von beiden:
- einen Lauf, bei dem du die Java-Engine neu baust (`run.sh --build`) und den Test wiederholst, oder
- die Bestätigung, ob der Test auf deinem Stand vorher grün war.

Wenn er nach dem Neubau grün ist, war es ein veralteter Build. Wenn er rot bleibt, liegt die Abweichung in der Engine selbst, und der Test oder die Engine müsste angepasst werden. Das gehört dann nicht mehr zu diesem Import-Feature.
Alter du machst mich irre was ist der Fehlschlag des Tests und was bedeutet er
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 456582, in: 196, out: 140750, reason: 350, read: 15307967, write: 315636, start: 1794266928037, end: 1794266938091, time: 10008, total: 1141528, io: 0, control: 46, sent: 1, recv: 153
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
ReasoningToken: 350
Der Test prüft, ob der Java-Engine ungültiger Code abgelehnt wird, und das passiert in diesem einen Fall nicht.

## Was der Test macht

```python
engine.insert(foo, 'int x;', 'before')   # foo = die Top-Level-Klasse "Foo"
```

Er fügt die Felddeklaration `int x;` direkt vor die Klasse `Foo` ein. Auf der obersten Ebene einer Java-Datei dürfen nur Typen stehen (`class`, `interface`, `enum`), keine Felder. Der Test erwartet deshalb, dass die Engine das mit einem `AstError` ablehnt. Zusätzlich soll es ein normaler `AstError` sein und kein `AstAmbiguous`, also ein Fehler wegen fehlerhaftem Code und nicht wegen mehrdeutiger Zuordnung.

## Was tatsächlich passiert

Die Engine lehnt nichts ab. Sie meldet Erfolg und liefert diese Datei zurück:

```java
int x;

public class Foo { ... }
```

Das ist ungültiges Java. Der Test bricht mit `DID NOT RAISE AstError` ab, weil der erwartete Fehler ausbleibt.

## Was das bedeutet

Der Java-Prozess prüft Einfügungen vor einem Top-Level-Typ nicht ausreichend. Andere Fehleingaben wie `void m() {` lehnt er ab. Nur `int x;` kommt durch. Die Folge ist, dass `ast_insert` auf Java-Dateien in diesem Fall stillschweigend eine kaputte Datei schreiben würde.

Mit dem Import-Feature hat das nichts zu tun. Meine Import-Logik ruft `insert` nicht mehr auf und validiert das Ergebnis vor dem Schreiben selbst. Ich habe nicht geklärt, warum die Engine das durchlässt, und ich weiß nicht, ob der Test früher grün war.
Welche Engine?
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 458312, in: 198, out: 141391, reason: 183, read: 15562193, write: 316723, start: 1794267016045, end: 1794267021911, time: 5818, total: 1147346, io: 0, control: 48, sent: 1, recv: 107
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
ReasoningToken: 183
Es ist die Java-RPC-Engine, nicht die Tree-sitter-Variante.

- **Die Python-Klasse:** `JavaEngine` in `src/xy/ai/mcpc/tools/ast/rpc/_java.py`. Sie ist nur ein dünner Client, der die Aufrufe weiterleitet. Die Fixture `get_engine('java')` im Test liefert genau sie, weil `.java` in `generic/__init__.py` auf diese Klasse zeigt.
- **Der eigentliche Prüfer:** Der separate Prozess `ast-java` (JavaParser) unter `ast-engines/java`. `RpcEngine.insert` schickt den Aufruf dorthin (`insertRelativeToNode`). Dort entscheidet sich, ob `int x;` vor einer Top-Level-Klasse angenommen oder abgelehnt wird.
- **Nicht beteiligt:** `generic/_java.py` mit der Tree-sitter-Engine. Sie ist nicht registriert und hat mit dem Test nichts zu tun.

Der Fehler steckt also im Java-Prozess, nicht im MCPC-Code. Das Rust-Gegenstück (`xy_ai_ast_rust`) ist nicht betroffen, denn der Test läuft nur gegen Java.
Erstelle eine gleiche Fehlerhafte Datei damit ich sie mir anschauen kann
SystemInit:  id=3ce94156-2fe1-4592-9066-faa974d907e8, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Text:
 Ich erzeuge die Datei mit der echten Engine, damit es genau die Ausgabe ist und nicht meine Abschrift davon. Ich lege sie zusammen mit der Eingabe in ein eigenes Verzeichnis unter `/tmp/claude-1000`.
ReasoningToken: 222
Control Request:
```yaml
id: 0a99-120-0
toolName: python
arguments:
  script: |-
    import sys
    from pathlib import Path
    sys.path.insert(0, "/home/user/xyan/xy.ai.workbench/mcpc/src")
    from xy.ai.mcpc.tools.ast.generic import get_engine

    out = Path("/tmp/claude-1000/java_insert_repro")
    out.mkdir(parents=True, exist_ok=True)
    e = get_engine("java")
    source = 'public class Foo {\n    void bar() {\n        System.out.println(1);\n    }\n}\n'
    (out / "Input.java").write_text(source)

    tree = e.parse(source)
    foo = next(l for l in e.locate_all(tree) if l.node_id == "Foo")
    e.insert(foo, "int x;", "before")
    (out / "Foo.java").write_text(tree.source)

    print("validate(Input):", e.validate(source))
    print("validate(Foo)  :", e.validate(tree.source))
    print(*sorted(p.name for p in out.iterdir()))
    print("----- Foo.java -----"); print(tree.source)
  reason: Fehlerhafte Java-Ausgabe der Engine (int x; vor Top-Level-Klasse) als Datei zur Ansicht erzeugen
```
/answer 0a99-120-0 deny mit dem AST Tool
Text:
 Verstanden, dann stelle ich es mit den AST-Tools nach. Zuerst lege ich die gültige Datei an, danach füge ich `int x;` vor der Klasse ein.
ReasoningToken: 132
Control Request:
```yaml
id: 0a99-121-0
toolName: ast_create
arguments:
  items:
  - path: /tmp/claude-1000/java_insert_repro/Foo.java
    source: |
      public class Foo {
          void bar() {
              System.out.println(1);
          }
      }
  reason: Gültige Ausgangsdatei für die Reproduktion anlegen (neues Verzeichnis unter /tmp/claude-1000)
```
/answer 0a99-121-0 allow
Control Request:
```yaml
id: 0a99-122-0
toolName: ast_insert
arguments:
  items:
  - path: /tmp/claude-1000/java_insert_repro/Foo.java
    id: Foo
    position: before
    source: int x;
  reason: "Den Fehlerfall nachstellen: Feld int x; vor die Top-Level-Klasse Foo einfügen"
```
/answer 0a99-122-0 allow
Text:
 Die Rückgabe enthält etwas Neues: die IDs beginnen jetzt mit `_COMPACT_CLASS`. Ich lese die Datei, damit du sie ansehen kannst.
Control Request:
```yaml
id: 0a99-123-0
toolName: ast_outline
arguments:
  paths:
  - /tmp/claude-1000/java_insert_repro/Foo.java
  reason: Struktur der durch die Engine veränderten Datei ansehen
```
/answer 0a99-123-0 allow
Control Request:
```yaml
id: 0a99-124-123
result:
  structuredContent:
    results:
    - path: /tmp/claude-1000/java_insert_repro/Foo.java
      nodes:
      - id: _COMPACT_CLASS
        signature: "int x;  public class Foo {"
        children:
        - id: _COMPACT_CLASS.XRHekl|7mXl14
          signature: int x;
        - id: _COMPACT_CLASS.Foo
          signature: "public class Foo {"
          children:
          - id: _COMPACT_CLASS.Foo.bar
            signature: void bar()
```
/answer 0a99-124-123 allow
ReasoningToken: 379
Control Request:
```yaml
id: 0a99-125-0
toolName: ast_validate
arguments:
  paths:
  - /tmp/claude-1000/java_insert_repro/Foo.java
  reason: "Prüfen, ob die Engine die veränderte Datei selbst als gültig einstuft"
```
/answer 0a99-125-0 allow
Control Request:
```yaml
id: 0a99-126-0
toolName: ast_find
arguments:
  paths:
  - /tmp/claude-1000/java_insert_repro/Foo.java
  lineno: 1
  end_lineno: 20
  reason: Vollständigen Quelltext der veränderten Datei lesen
```
/answer 0a99-126-0 allow
Control Request:
```yaml
id: 0a99-127-126
result:
  structuredContent:
    files:
    - path: /tmp/claude-1000/java_insert_repro/Foo.java
      nodes:
      - id: _COMPACT_CLASS.XRHekl|7mXl14
        code: |
          int x;
      - id: _COMPACT_CLASS.Foo.bar
        code: |2
            void bar() {
              System.out.println(1);
            }
```
/answer 0a99-127-126 allow
ReasoningToken: 53
Control Request:
```yaml
id: 0a99-128-0
toolName: ast_read
arguments:
  items:
  - path: /tmp/claude-1000/java_insert_repro/Foo.java
    ids:
    - _COMPACT_CLASS
  reason: Gesamten Dateiinhalt über den Wurzelknoten lesen
```
/answer 0a99-128-0 allow
Control Request:
```yaml
id: 0a99-129-128
result:
  structuredContent:
    results:
    - path: /tmp/claude-1000/java_insert_repro/Foo.java
      nodes:
      - id: _COMPACT_CLASS
        type: ClassOrInterfaceDeclaration
        signature: "int x;  public class Foo {"
        children:
        - id: _COMPACT_CLASS.XRHekl|7mXl14
          type: StatementGroup
          code: |
            int x;
        - id: _COMPACT_CLASS.Foo
          type: ClassOrInterfaceDeclaration
          signature: "public class Foo {"
          children:
          - id: _COMPACT_CLASS.Foo.bar
            type: MethodDeclaration
            code: |2
                void bar() {
                  System.out.println(1);
                }
```
/answer 0a99-129-128 allow
Result Stats: id=3ce94156-2fe1-4592-9066-faa974d907e8, total: 465951, in: 210, out: 144754, reason: 0, read: 17104464, write: 320987, start: 1794267113949, end: 1794267123573, time: 9576, total: 1181740, io: 0, control: 48, sent: 0, recv: 188
Die Datei liegt unter `/tmp/claude-1000/java_insert_repro/Foo.java`. Sie ist so entstanden: Ich habe sie mit der gültigen Klasse angelegt und danach mit `ast_insert` die Zeile `int x;` vor `Foo` eingefügt. Der Inhalt ist jetzt:

```java
int x;

public class Foo {

  void bar() {
    System.out.println(1);
  }
}
```

Die Beobachtung hat meine Erklärung korrigiert. Ich hatte die Ausgabe als ungültiges Java bezeichnet, aber die Engine sieht das offenbar anders:

- `ast_validate` meldet die Datei als gültig (`all_ok: true`).
- Die Engine erkennt sie als **Compact Source File**. Der Wurzelknoten heißt jetzt `_COMPACT_CLASS`, und `int x;` sowie `Foo` hängen darunter. `Foo` ist eine verschachtelte Klasse, `bar` gehört zu `Foo`.

Das passt zu einem neueren Java-Sprachlevel, in dem Felder und Methoden auf oberster Ebene erlaubt sind und in eine implizite Klasse wandern. Das ist meine Schlussfolgerung aus der Ausgabe der Engine. Ich habe weder die Engine-Quellen noch die Java-Version dazu geprüft.

Falls das stimmt, ist der Test nicht mehr zeitgemäß: Er geht davon aus, dass `int x;` auf oberster Ebene ungültig ist. Die Engine ist dann nicht defekt, sondern hat ein moderneres Sprachverständnis. Das verändert meine frühere Aussage, dass `ast_insert` hier „eine kaputte Datei schreibt“. Die Datei ist für diese Engine gültig. Ob sie für euren Java-Compiler gültig ist, hängt von dessen Version ab.

Für den Import-Pfad ist das ebenfalls relevant, weil meine Java-Einfügung auf `ImportGroup` und `package` aufsetzt. Das ist hier nicht berührt, denn der Import-Pfad schreibt vor `Foo` nur Imports und validiert danach. Möglich wäre aber, dass ein „Import“, der kein Import ist, in einer Compact-Datei bei `validate` durchrutscht. Meine Prüfung `_rpc_statement` und das erneute Lokalisieren als `ImportGroup` fangen das ab, weil `int x;` dort nicht als Import erkannt würde.
Das ist dann ist neuen kompakten Javadateiformate
SystemInit:  id=60e2a49c-8888-497c-a8ba-f9bdb58b8f9c, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
ReasoningToken: 79
Control Request:
```yaml
id: 0a99-12a-0
toolName: ask_user
arguments:
  question: "Meinst du, dass ein neues kompaktes Java-Dateiformat eingeführt werden soll? Was genau soll ich tun: das Format entwerfen/spezifizieren, einen Parser/Writer implementieren oder bestehende Java-Dateien in dieses Format überführen? Gibt es dazu eine Beschreibung oder Beispieldatei im Projekt?"
  reason: "Die Nachricht beschreibt kein konkretes Ziel. Ich brauche Klarheit über die gewünschte Aufgabe, bevor ich Dateien im Repository ändere."
```
/answer 0a99-12a-0 allow
Selected session parameters are incompatible with the current configuration