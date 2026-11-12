Analysiere und behebe die Fehler des Improt handlings.

1. Der Fehlschlag ist von der eigenen vorlaufenden Änderung verursacht:
2026-11-12 20:51:34,257 WARNING xy.ai.mcpc.tools.ast.core: Import handling failed for /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java
Traceback (most recent call last):
  File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py", line 145, in apply_imports
    return tree.engine.apply_imports(tree, add or [], remove or [])
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py", line 202, in apply_imports
    return apply_rpc(self, tree, add, remove)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py", line 328, in apply_rpc
    after, _ = _rpc_present(engine, tree, source)
               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py", line 249, in _rpc_present
    top = [loc for loc in engine.locate_all(view) if '.' not in loc.node_id]
                          ^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py", line 111, in locate_all
    payload = self._unwrap(self._rpc().listNodes(request))
              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py", line 81, in _unwrap
    raise AstError(message)
xy.ai.mcpc.tools.ast.base.AstError: file changed on disk since the given source was captured: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove/AutoApprover.java

2. Irgendwas hat die Improts komplett entfern. Ich konnte die Änderung aber nich rekonstruieren. Es wäre möglich das ein Fehler in Kombination einen AST gesichert hat indem die Imports bereits fehlten.
SystemInit:  id=07d84b84-b4d7-4538-a4cc-4c1c68592228, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Control Request:
```yaml
id: f500-1-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  reason: Read imports.py to analyze apply_rpc and _rpc_present
```
/answer f500-1-0 allow
Control Request:
```yaml
id: f500-2-1
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: foxd99|7aiScK
        excert: '"Import/include/use handling shared by the ``ast_*`` edit tools.\n\nEvery edit …'
      - id: AFajCW|f1RBFD
        excert: from __future__ import annotations
      - id: AiyIX3|8cD8JN
        excert: "__all__ = ["
      - id: ImportOutcome
        excert: "@dataclass class ImportOutcome:"
        docstring: "What an engine did for one item's ``addImport``/``removeImport``. Attributes: a…"
      - id: string_list
        excert: "def _string_list(item: dict[str, Any], key: str) -> list[str] | None:"
      - id: parse_import_args
        excert: "def parse_import_args(item: dict[str, Any]) -> tuple[list[str] | None, list[str] | None]:"
        docstring: "``(addImport, removeImport)`` of a raw MCP item; ``ValueError`` on malformed va…"
      - id: import_notice
        excert: "def import_notice(results: list[Any]) -> str | None:"
        docstring: "The single batch-level hint, if any result carries unchecked imports."
      - id: import_entry
        excert: "def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:"
        docstring: "Serialized ``imports`` field of a result, or ``None`` if nothing changed."
      - id: S64wGg|AyaH5Z
        excert: _SPACE = re.compile('\\s+')
      - id: normalize_import
        excert: "def normalize_import(text: str) -> str:"
        docstring: "Whitespace-, quote- and trailing-``;``-insensitive form used to compare stateme…"
      - id: checked
        excert: "def _checked(statement: str) -> str:"
      - id: unique
        excert: "def _unique(statements: list[str]) -> list[str]:"
      - id: statement_spans
        excert: "def statement_spans(data: bytes) -> list[tuple[int, int]]:"
        docstring: Byte spans of the ``;``-terminated top-level statements in ``data`` (comments s…
      - id: line_extent
        excert: "def line_extent(data: bytes, start: int, end: int) -> tuple[int, int]:"
        docstring: "``(start, end)`` widened to its whole line(s) if nothing else stands on them."
      - id: delete_span
        excert: "def delete_span(data: bytes, start: int, end: int) -> bytes:"
        docstring: "Remove ``data[start:end]``, including its line if nothing else stands on it."
      - id: next_line_start
        excert: "def _next_line_start(data: bytes, offset: int) -> int:"
      - id: line_starts
        excert: "def _line_starts(data: bytes) -> list[int]:"
      - id: store
        excert: "def _store(engine: Engine, tree: Tree, data: bytes) -> None:"
      - id: apply_generic
        excert: "def apply_generic(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
        docstring: "Engine-agnostic fallback. ``add`` is only checked: a statement missing as a lin…"
      - id: prologue_end
        excert: "def _prologue_end(root: Any, data: bytes, is_prologue: Callable[[Any], bool] | None) -> int:"
        docstring: Offset after leading shebang/directives and header comments (comments followed …
      - id: snippet
        excert: "def _snippet(engine: Engine, statement: str, import_types: set[str]) -> str:"
      - id: apply_treesitter
        excert: "def apply_treesitter(engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str], is_prologue: Callable[[Any], bool] | None=None) -> ImportOutcome:"
        docstring: Structure-aware handling for tree-sitter engines whose top-level import nodes a…
      - id: fs86A7|AnrSrF
        excert: "'#: Node types under which the RPC processes report a run of imports.'"
      - id: rpc_statement
        excert: "def _rpc_statement(statement: str) -> str:"
      - id: rpc_present
        excert: "def _rpc_present(engine: Engine, tree: Tree, source: str | None=None) -> tuple[dict[str, tuple[int, int]], list[Any]]:"
        docstring: Every top-level import statement's byte span (keyed by normalized text) and the…
      - id: g7FpPW|tkBF2z
        excert: "'#: Line prefixes that stay attached to the declaration below them (docs, annot…"
      - id: leading_start
        excert: "def _leading_start(data: bytes, starts: list[int], index: int) -> int:"
        docstring: "Start offset of line ``index``, moved up over directly attached comment/annotat…"
      - id: rpc_insertion
        excert: "def _rpc_insertion(data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:"
        docstring: "Edit that places ``code``: behind the last import, else behind ``package``/modu…"
      - id: apply_rpc
        excert: "def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
        docstring: Structure-aware handling for RPC engines that report import groups (Java ``impo…
```
/answer f500-2-1 allow
Control Request:
```yaml
id: f500-3-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
  reason: Get structure and node ids
```
/answer f500-3-0 allow
Control Request:
```yaml
id: f500-4-3
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: foxd99|7aiScK
        excert: '"Import/include/use handling shared by the ``ast_*`` edit tools.\n\nEvery edit …'
      - id: AFajCW|f1RBFD
        excert: from __future__ import annotations
      - id: AiyIX3|8cD8JN
        excert: "__all__ = ["
      - id: ImportOutcome
        excert: "@dataclass class ImportOutcome:"
        docstring: "What an engine did for one item's ``addImport``/``removeImport``. Attributes: a…"
      - id: string_list
        excert: "def _string_list(item: dict[str, Any], key: str) -> list[str] | None:"
      - id: parse_import_args
        excert: "def parse_import_args(item: dict[str, Any]) -> tuple[list[str] | None, list[str] | None]:"
        docstring: "``(addImport, removeImport)`` of a raw MCP item; ``ValueError`` on malformed va…"
      - id: import_notice
        excert: "def import_notice(results: list[Any]) -> str | None:"
        docstring: "The single batch-level hint, if any result carries unchecked imports."
      - id: import_entry
        excert: "def import_entry(outcome: ImportOutcome | None) -> dict[str, list[str]] | None:"
        docstring: "Serialized ``imports`` field of a result, or ``None`` if nothing changed."
      - id: S64wGg|AyaH5Z
        excert: _SPACE = re.compile('\\s+')
      - id: normalize_import
        excert: "def normalize_import(text: str) -> str:"
        docstring: "Whitespace-, quote- and trailing-``;``-insensitive form used to compare stateme…"
      - id: checked
        excert: "def _checked(statement: str) -> str:"
      - id: unique
        excert: "def _unique(statements: list[str]) -> list[str]:"
      - id: statement_spans
        excert: "def statement_spans(data: bytes) -> list[tuple[int, int]]:"
        docstring: Byte spans of the ``;``-terminated top-level statements in ``data`` (comments s…
      - id: line_extent
        excert: "def line_extent(data: bytes, start: int, end: int) -> tuple[int, int]:"
        docstring: "``(start, end)`` widened to its whole line(s) if nothing else stands on them."
      - id: delete_span
        excert: "def delete_span(data: bytes, start: int, end: int) -> bytes:"
        docstring: "Remove ``data[start:end]``, including its line if nothing else stands on it."
      - id: next_line_start
        excert: "def _next_line_start(data: bytes, offset: int) -> int:"
      - id: line_starts
        excert: "def _line_starts(data: bytes) -> list[int]:"
      - id: store
        excert: "def _store(engine: Engine, tree: Tree, data: bytes) -> None:"
      - id: apply_generic
        excert: "def apply_generic(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
        docstring: "Engine-agnostic fallback. ``add`` is only checked: a statement missing as a lin…"
      - id: prologue_end
        excert: "def _prologue_end(root: Any, data: bytes, is_prologue: Callable[[Any], bool] | None) -> int:"
        docstring: Offset after leading shebang/directives and header comments (comments followed …
      - id: snippet
        excert: "def _snippet(engine: Engine, statement: str, import_types: set[str]) -> str:"
      - id: apply_treesitter
        excert: "def apply_treesitter(engine: Engine, tree: Tree, add: list[str], remove: list[str], import_types: set[str], is_prologue: Callable[[Any], bool] | None=None) -> ImportOutcome:"
        docstring: Structure-aware handling for tree-sitter engines whose top-level import nodes a…
      - id: fs86A7|AnrSrF
        excert: "'#: Node types under which the RPC processes report a run of imports.'"
      - id: rpc_statement
        excert: "def _rpc_statement(statement: str) -> str:"
      - id: rpc_present
        excert: "def _rpc_present(engine: Engine, tree: Tree, source: str | None=None) -> tuple[dict[str, tuple[int, int]], list[Any]]:"
        docstring: Every top-level import statement's byte span (keyed by normalized text) and the…
      - id: g7FpPW|tkBF2z
        excert: "'#: Line prefixes that stay attached to the declaration below them (docs, annot…"
      - id: leading_start
        excert: "def _leading_start(data: bytes, starts: list[int], index: int) -> int:"
        docstring: "Start offset of line ``index``, moved up over directly attached comment/annotat…"
      - id: rpc_insertion
        excert: "def _rpc_insertion(data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:"
        docstring: "Edit that places ``code``: behind the last import, else behind ``package``/modu…"
      - id: apply_rpc
        excert: "def apply_rpc(engine: Engine, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
        docstring: Structure-aware handling for RPC engines that report import groups (Java ``impo…
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
      nodes:
      - id: BQIdfu|0WxhPP
        excert: '"Generic :class:`Engine` back-end for any parser exposed via the RPC/PIC\nHTTP …'
      - id: Da4IMM|t2aMCb
        excert: from __future__ import annotations
      - id: DLMY8E|Z2twV0
        excert: from xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest…
      - id: BshuIz|Kr3xsO
        excert: from xy.ai.mcpc.tools.ast.rpc._process import RpcProcess
      - id: ibsDCF|76lOk2
        excert: "__all__ = ['RpcEngine']"
      - id: RpcNode
        excert: "@dataclass class _RpcNode:"
        docstring: One node as reported by ``POST /nodes``. Carries everything :meth:`RpcEngine.si…
      - id: RpcEngine
        excert: "class RpcEngine(Engine):"
        docstring: One RPC/PIC engine process exposed through the common :class:`Engine` API. Stru…
        children:
        - id: RpcEngine.95ZeHe|fDr6kO
          excert: '"One RPC/PIC engine process exposed through the common :class:`Engine` API.\n\n…'
        - id: RpcEngine.init
          excert: "def __init__(self, symbol: str, process: RpcProcess) -> None:"
        - id: RpcEngine.rpc
          excert: "def _rpc(self) -> AppendInfoNodesValidateClientImpl:"
        - id: RpcEngine.unwrap
          excert: "@staticmethod def _unwrap(response: Any) -> Any:"
          docstring: "Return a 200 response's typed body, or raise on any other status."
        - id: RpcEngine.MltvBv|wKemBs
          excert: '''# -- Engine ------------------------------------------------------------'''
        - id: RpcEngine.parse
          excert: "def parse(self, source: str, path: Path | None=None) -> Tree:"
        - id: RpcEngine.empty_tree
          excert: "def empty_tree(self, path: Path | None=None) -> Tree:"
        - id: RpcEngine.serialize
          excert: "def serialize(self, tree: Tree) -> str:"
        - id: RpcEngine.validate
          excert: "def validate(self, source: str) -> str | None:"
        - id: RpcEngine.is_definition
          excert: "def is_definition(self, node_type: str) -> bool:"
        - id: RpcEngine.locate_all
          excert: "def locate_all(self, tree: Tree) -> list[Located]:"
        - id: RpcEngine.signature
          excert: "def signature(self, node: _RpcNode) -> str:"
        - id: RpcEngine.docstring
          excert: "def docstring(self, node: _RpcNode) -> str | None:"
        - id: RpcEngine.node_code
          excert: "def node_code(self, node: _RpcNode) -> str:"
        - id: RpcEngine.replace
          excert: "def replace(self, loc: Located, code: str) -> None:"
        - id: RpcEngine.insert
          excert: "def insert(self, loc: Located, code: str, position: str) -> int:"
        - id: RpcEngine.delete
          excert: "def delete(self, loc: Located) -> None:"
        - id: RpcEngine.append
          excert: "def append(self, tree: Tree, code: str) -> int:"
        - id: RpcEngine.apply_imports
          excert: "def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
```
/answer f500-4-3 allow
Control Request:
```yaml
id: f500-5-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    ids:
    - store
    - rpc_present
    - apply_rpc
    - rpc_insertion
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
    ids:
    - RpcEngine.parse
    - RpcEngine.serialize
    - RpcEngine.locate_all
    - RpcEngine.replace
    - RpcEngine.insert
    - RpcEngine.delete
    - RpcEngine.append
    - RpcEngine.apply_imports
    - RpcEngine.init
    - RpcNode
  reason: Read the relevant functions
```
/answer f500-5-0 allow
Control Request:
```yaml
id: f500-6-5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
      nodes:
      - id: store
        type: FunctionDef
        code: |-
          def _store(engine: Engine, tree: Tree, data: bytes) -> None:
              source = data.decode('utf-8')
              error = engine.validate(source) if engine.validates_syntax else None
              if error:
                  raise AstError(f'The import change would leave the file invalid: {error}')
              tree.raw = engine.parse(source, tree.path).raw
              tree.source = source
      - id: rpc_present
        type: FunctionDef
        code: |-
          def _rpc_present(engine: Engine, tree: Tree, source: str | None=None) -> tuple[dict[str, tuple[int, int]], list[Any]]:
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
              return (present, top)
      - id: apply_rpc
        type: FunctionDef
        code: |-
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
              if any((normalize_import(s) not in after for s in outcome.added)):
                  raise AstError(f'Not recognized as an import by the parser: {outcome.added!r}')
              tree.source = source
              return outcome
      - id: rpc_insertion
        type: FunctionDef
        code: |-
          def _rpc_insertion(data: bytes, present: dict[str, tuple[int, int]], top: list[Any], code: str) -> tuple[int, int, bytes]:
              """Edit that places ``code``: behind the last import, else behind ``package``/module docs, else before the first node."""
              block = code.encode('utf-8')
              if present:
                  pos = _next_line_start(data, max((end for _, end in present.values())))
                  text = b'\n' + block if pos == len(data) and (not data.endswith(b'\n')) else block + b'\n'
                  return (pos, pos, text)
              spans = statement_spans(data)
              if spans and data[spans[0][0]:spans[0][1]].startswith(b'package'):
                  pos = _next_line_start(data, spans[0][1])
                  return (pos, pos, b'\n' + block + b'\n')
              starts = _line_starts(data)
              prologue = [n for n in top if n.node_type in _RPC_PROLOGUE_TYPES]
              body = [n for n in top if n.node_type not in _RPC_PROLOGUE_TYPES]
              if body:
                  pos = _leading_start(data, starts, min(body[0].lineno - 1, len(starts) - 1))
                  return (pos, pos, block + b'\n\n')
              if prologue:
                  pos = starts[min(prologue[-1].end_lineno, len(starts) - 1)]
                  return (pos, pos, b'\n' + block + b'\n')
              pos = len(data)
              return (pos, pos, (b'' if not data or data.endswith(b'\n') else b'\n') + block + b'\n')
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/rpc/_engine.py
      nodes:
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
      - id: RpcEngine.apply_imports
        type: FunctionDef
        code: |-
          def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:
              return apply_rpc(self, tree, add, remove)
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
      - id: RpcNode
        type: ClassDef
        code: |-
          @dataclass
          class _RpcNode:
              """One node as reported by ``POST /nodes``.

              Carries everything :meth:`RpcEngine.signature`/``docstring``/``node_code``
              need without a further round-trip: signature/docstring come straight from
              the server, and code -- unless eagerly included -- is sliced from the
              already-known ``tree.source`` by line range.
              """
              tree: Tree
              lineno: int
              end_lineno: int
              signature_text: str | None
              docstring_text: str | None
              code: str | None
```
/answer f500-6-5 allow
ReasoningToken: 276
Control Request:
```yaml
id: f500-7-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
    pattern: apply_imports|changed on disk|def _write|\.source =
    include:
    - '*.py'
    exclude:
    - '*test*'
    limit: 50
  reason: Find import application call sites and the disk-check
```
/answer f500-7-0 allow
Control Request:
```yaml
id: f500-8-7
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
      matches:
      - path: base.py
        matches:
        - "def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> 'ImportOutcome':"
        - ":class:`NotImplementedError`, upon which :func:`core.apply_imports` uses the generic"
      - path: replace.py
        matches:
        - "imports = core.apply_imports(tree, item.add_import, item.remove_import)"
      - path: delete.py
        matches:
        - "imports = core.apply_imports(tree, item.add_import, item.remove_import)"
      - path: rpc/_engine.py
        matches:
        - loc.tree.source = payload.get_Source()
        - tree.source = payload.get_Source()
        - "def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
      - path: generic/_text.py
        matches:
        - loc.tree.source = code
        - loc.tree.source = code + '\n' + loc.tree.source
        - loc.tree.source = loc.tree.source + '\n' + code
        - loc.tree.source = ''
        - tree.source = tree.source + sep + code
      - path: generic/_javascript.py
        matches:
        - "def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
      - path: generic/_engine.py
        matches:
        - tree.source = new.decode('utf-8')
      - path: edit_marks.py
        matches:
        - "imports = core.apply_imports(tree, item.add_import, item.remove_import)"
      - path: edit_block.py
        matches:
        - "imports = core.apply_imports(tree, item.add_import, item.remove_import)"
      - path: python/_engine.py
        matches:
        - "def apply_imports(self, tree: Tree, add: list[str], remove: list[str]) -> ImportOutcome:"
      - path: core.py
        matches:
        - "def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:"
        - "return tree.engine.apply_imports(tree, add or [], remove or [])"
      - path: insert.py
        matches:
        - "imports = core.apply_imports(tree, item.add_import, item.remove_import)"
      - path: imports.py
        matches:
        - :meth:`Engine.apply_imports` after the item's node edit. Support is purely
        - tree.source = source
```
/answer f500-8-7 allow
Control Request:
```yaml
id: f500-9-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: Inspect core.apply_imports and save logic
```
/answer f500-9-0 allow
Control Request:
```yaml
id: f500-a-9
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
      - id: V6fdSR|z5Q7Tq
        excert: "'Facade the ``ast_*`` tools call, dispatching to a per-file engine.\\n\\nThe engi…"
      - id: OsZYRc|1ZfHhi
        excert: from __future__ import annotations
      - id: 64y9R1|MAcfZT
        excert: from xy.ai.mcpc.tools.ast import python
      - id: fr6KXO|uhDdPM
        excert: logger = logging.getLogger(__name__)
      - id: engine_for_path
        excert: "def engine_for_path(path: Path) -> Engine:"
        docstring: Return the engine responsible for ``path`` based on its extension.
      - id: CacheEntry
        excert: "@dataclass class _CacheEntry:"
      - id: AstCache
        excert: "class AstCache:"
        docstring: Content-hash validated cache of parsed trees keyed by absolute path.
        children:
        - id: AstCache.AzRmHX|ayni5v
          excert: '''Content-hash validated cache of parsed trees keyed by absolute path.'''
        - id: AstCache.init
          excert: "def __init__(self) -> None:"
        - id: AstCache.get_tree
          excert: "def get_tree(self, path: Path) -> Tree:"
        - id: AstCache.save
          excert: "def save(self, path: Path, tree: Tree) -> str:"
          docstring: "Serialise *tree*, write it to *path* and refresh the cache entry."
        - id: AstCache.invalidate
          excert: "def invalidate(self, path: Path) -> None:"
      - id: dRq1FJ|5EG9eJ
        excert: "'#: Process-wide shared cache instance.'"
      - id: load
        excert: "def load(path_str: str) -> tuple[Path, Tree]:"
        docstring: Resolve *path_str* and return it together with its cached tree.
      - id: parse_source
        excert: "def parse_source(source: str, engine: Engine | None=None) -> Tree:"
        docstring: Parse *source* with *engine* (Python by default for path-less snippets).
      - id: parse_for
        excert: "def parse_for(path_str: str, code: str) -> Tree:"
        docstring: Parse *code* with the engine selected for *path_str*'s extension.
      - id: locate_all
        excert: "def locate_all(tree: Tree) -> list[Located]:"
      - id: edit_node_source
        excert: "def edit_node_source(loc: Located) -> str:"
      - id: relocate
        excert: "def relocate(tree: Tree, old: Located) -> Located | None:"
        docstring: Find the node now occupying ``old``'s former slot after an in-place edit. Match…
      - id: replace_node
        excert: "def replace_node(loc: Located, code: str) -> str | None:"
        docstring: Replace ``loc``'s node with ``code``; return its new id.
      - id: insert_node
        excert: "def insert_node(loc: Located, code: str, position: str) -> int:"
      - id: delete_node
        excert: "def delete_node(loc: Located) -> None:"
      - id: append_nodes
        excert: "def append_nodes(tree: Tree, code: str) -> int:"
      - id: apply_imports
        excert: "def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:"
        docstring: Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``. …
      - id: empty_tree
        excert: "def empty_tree(path: Path) -> Tree:"
      - id: validate_source
        excert: "def validate_source(path: Path, source: str) -> str | None:"
```
/answer f500-a-9 allow
Control Request:
```yaml
id: f500-b-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: changed on disk
    include:
    - '*.py'
    - '*.java'
    exclude:
    - node_modules/**
    - target/**
    limit: 10
  reason: Find origin of the staleness check
```
/answer f500-b-0 allow
Control Request:
```yaml
id: f500-c-b
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
```
/answer f500-c-b allow
ReasoningToken: 39
Control Request:
```yaml
id: f500-d-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
    ids:
    - AstCache.get_tree
    - AstCache.save
    - apply_imports
    - load
  reason: Check cache/save and import wrapper for the lost-import cause
```
/answer f500-d-0 allow
Control Request:
```yaml
id: f500-e-d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
      nodes:
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
      - id: apply_imports
        type: FunctionDef
        code: |-
          def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:
              """Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``.

              Independent of the node edit: engines compute all import changes first and write them
              once, so a failure leaves the tree as the node edit left it. It is then reported as
              ``unchecked``, which puts the notice on the batch. Engines without dedicated support
              get the generic check-only fallback.
              """
              if not add and (not remove):
                  return None
              try:
                  try:
                      return tree.engine.apply_imports(tree, add or [], remove or [])
                  except NotImplementedError:
                      return apply_generic(tree.engine, tree, add or [], remove or [])
              except Exception:
                  '# noqa: BLE001 - never fail the edit because of its imports'
                  logger.warning('Import handling failed for %s', tree.path, exc_info=True)
                  return ImportOutcome(unchecked=True)
      - id: load
        type: FunctionDef
        code: |-
          def load(path_str: str) -> tuple[Path, Tree]:
              """Resolve *path_str* and return it together with its cached tree."""
              path = require_path(path_str)
              return (path, CACHE.get_tree(path))
```
/answer f500-e-d allow
ReasoningToken: 300
Control Request:
```yaml
id: f500-f-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: rpc_present
    old_text: "view = tree if source is None else Tree(engine, None, source, tree.path)"
    new_text: |-
      # A candidate has no file on disk: passing the path makes the engine reject it as stale.
          view = tree if source is None else Tree(engine, None, source, None)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
    id: apply_rpc
    old_text: |2-
          tree.source = source
          return outcome
    new_text: |2-
          # Never lose imports we were not asked to remove (e.g. a mis-reported span).
          kept = present.keys() - removed_keys
          lost = sorted(k for k in kept if k not in after)
          if lost:
              raise AstError(f'The import change would drop other imports: {lost!r}')
          tree.source = source
          return outcome
  reason: Fix stale-path check for candidate view and guard against silent import loss
```
/answer f500-f-0 allow
Control Request:
```yaml
id: f500-10-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/imports.py
  reason: Verify compile
```
/answer f500-10-0 allow
Result Stats: id=07d84b84-b4d7-4538-a4cc-4c1c68592228, total: 27666, in: 14, out: 3586, reason: 0, read: 103131, write: 24066, start: 1794514086731, end: 1794514095430, time: 8610, total: 45084, io: 0, control: 89, sent: 0, recv: 206
Ich habe den ersten Fehler behoben. Den zweiten, die komplett entfernten Imports, konnte ich nicht auf eine Ursache zurückführen. Stattdessen habe ich eine Absicherung eingebaut. Getestet habe ich nichts, die Datei lässt sich nur kompilieren (`ast_validate`).

**1. Fehler „file changed on disk“**

- **Ursache:** `_rpc_present` in `imports.py` prüft in `apply_rpc` den veränderten Quelltext vor dem Schreiben. Dafür hat es einen temporären `Tree` gebaut und dabei den echten Dateipfad übergeben. Der RPC-Server vergleicht bei einem Pfad den übergebenen Quelltext mit der Datei auf der Platte. Der Kandidat mit den Import-Änderungen weicht davon ab, deshalb kam die Fehlermeldung. Der Import-Schritt ist damit immer fehlgeschlagen und wurde nur als `unchecked` gemeldet.
- **Behebung:** Der Kandidaten-Tree wird jetzt ohne Pfad gebaut (`Tree(engine, None, source, None)`). Der Server führt dann keine Prüfung gegen die Platte durch. Der Pfad wird für die Sprachwahl nicht gebraucht, denn die Engine steht schon fest.

**2. Komplett verschwundene Imports**

- **Was ich geprüft habe:** Ich habe in `apply_rpc` und im Cache (`AstCache.get_tree`, `save`) keine Stelle gefunden, die Imports ohne Anforderung entfernt. Ein Fehler in `apply_rpc` lässt den Tree unverändert. Die Ursache kann ich deshalb nicht belegen.
- **Absicherung:** `apply_rpc` prüft jetzt nach der Änderung, ob alle Imports noch vorhanden sind, die nicht ausdrücklich entfernt werden sollten. Fehlt einer, wird mit `AstError` abgebrochen, und der Tree bleibt unverändert. So kann ein falsch gemeldeter Import-Bereich nicht mehr unbemerkt Imports löschen.
- **Offene Vermutung:** Dein Verdacht, dass ein Baum gesichert wurde, dem die Imports schon fehlten, bleibt möglich. `AstCache.save` schreibt den Tree ungeprüft auf die Platte. Der Cache wird nur über mtime und Hash validiert.
- **Zum Eingrenzen:** `git diff` oder `git log -p` auf `AutoApprover.java` zeigen, in welchem Commit die Imports verschwunden sind. Dazu kommt, ob zu dem Zeitpunkt ein `replace` oder `delete` mit `removeImport` lief.
