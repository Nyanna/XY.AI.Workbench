Analog zu `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py` soll `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py` stark typisierte Result Objekte bekommen. Die MCP Rückgabe wird daraus abgeleitet. Die Rückgaben von "List" und "Find" sollen den outline Knoten gleichen und dienen dem Retrieval und der Navigation.

Zusätzlich soll es ein "read" Tool geben, das einen Teilbaum rekursiv ausgibt. Gegenüber der Outline soll Read den Qualified Name(soweit anwendbar und eindeutig) und den Code ausgeben. Das Ziel ist dem Agenten ein blockweises Edit oder Replace zu ermöglichen. Der Agent kann nach dem Lesen dann den Block per Qualified Name referenzieren und Ersetzen. Wenn, Beispielweise, ein Objekt direkt statements enhält, umfasst der code das ganze objekt. Enthält es dagegen nur Functions als direkte Kinder so ist der Code Leer aber der Knoten enthält die Functions als Kindknoten.
SystemInit:  id=e6d3b44f-b856-479e-ad5d-3298dbdc81bb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 46c8-1
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  reason: "Verstehen der bestehenden stark typisierten Result-Objekte für Outline, um analoges Muster für crud.py zu übernehmen"
```
/answer 46c8-1 allow
Control Request:
```yaml
id: 46c8-2
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  reason: "Aktuellen Stand von crud.py lesen, um die Result-Objekte und List/Find/Read Tools zu implementieren"
```
/answer 46c8-2 allow
ReasoningToken: 21
Control Request:
```yaml
id: 46c8-3
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  reason: "Verstehen der core-Hilfsfunktionen (Located, node_summary, find, locate_all, tree_from_input) für die Implementierung der stark typisierten CRUD Result-Objekte und des neuen Read-Tools"
```
/answer 46c8-3 allow
ReasoningToken: 39
Control Request:
```yaml
id: 46c8-4
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast
  reason: "Verzeichnisinhalt des ast-Tools-Ordners auflisten, um alle Konsumenten von node_summary/ListNodesResult/FindNodesResult zu finden"
```
/answer 46c8-4 deny es gibt keine Konsumenten
ReasoningToken: 3983
Control Request:
```yaml
id: 46c8-5
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  start: from dataclasses import dataclass
  content: "from dataclasses import dataclass, field"
  reason: field() wird für den neuen rekursiven OutlineNode-Default in core.py benötigt
```
/answer 46c8-5 allow
Control Request:
```yaml
id: 46c8-6
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/core.py
  start: |-
    def node_summary(loc: Located) -> dict[str, object]:
        node = loc.node
        return {
            "type": type(node).__name__,
            "name": loc.name,
            "qualified_name": loc.qualified_name,
            "lineno": node.lineno,
            "end_lineno": getattr(node, "end_lineno", node.lineno),
            "parent_type": type(loc.parent).__name__,
            "docstring": short_docstring(node),
        }
  content: |-
    @dataclass(frozen=True)
    class OutlineNode:
        """One AST statement in a structural (outline/list/find) result.

        Attributes:
            type: The node's exact AST type, e.g. ``"ClassDef"`` or ``"Import"``.
            qualified_name: Dotted path, for classes/functions/imports only; ``None`` otherwise.
            lines: Line number, or a ``"start-end"`` range if the node spans several lines.
            signature: One-line rendering of the node's header (or the statement itself).
            docstring: Short docstring, only possible for classes/functions.
            children: Nested entries for a class's body; empty for flat (list/find) results.
        """

        type: str
        qualified_name: str | None
        lines: str
        signature: str
        docstring: str | None
        children: list["OutlineNode"] = field(default_factory=list)


    def line_range(node: ast.stmt) -> str:
        """Return *node*'s start line, or a ``"start-end"`` range if it spans several lines."""
        end = getattr(node, "end_lineno", node.lineno)
        return str(node.lineno) if end == node.lineno else f"{node.lineno}-{end}"


    def _decorators(node: ast.FunctionDef | ast.AsyncFunctionDef | ast.ClassDef) -> str:
        return "".join(f"@{ast.unparse(d)} " for d in node.decorator_list)


    def node_signature(node: ast.stmt, limit: int = 80) -> str:
        """Return a one-line rendering of *node*'s header (or the statement itself)."""
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            keyword = "async def" if isinstance(node, ast.AsyncFunctionDef) else "def"
            returns = f" -> {ast.unparse(node.returns)}" if node.returns is not None else ""
            return f"{_decorators(node)}{keyword} {node.name}({ast.unparse(node.args)}){returns}:"
        if isinstance(node, ast.ClassDef):
            bases = [ast.unparse(b) for b in node.bases] + [
                f"{kw.arg}={ast.unparse(kw.value)}" for kw in node.keywords
            ]
            bases_str = f"({', '.join(bases)})" if bases else ""
            return f"{_decorators(node)}class {node.name}{bases_str}:"
        first_line = ast.unparse(node).splitlines()[0]
        return first_line if len(first_line) <= limit else first_line[: limit - 1] + "…"


    def node_outline(loc: Located) -> OutlineNode:
        """Build a flat (childless) :class:`OutlineNode` describing *loc*."""
        node = loc.node
        return OutlineNode(
            type=type(node).__name__,
            qualified_name=loc.qualified_name,
            lines=line_range(node),
            signature=node_signature(node),
            docstring=short_docstring(node),
        )


    #: JSON-Schema fragment for :class:`OutlineNode`, shared by the outline/list/find tools.
    OUTLINE_NODE_SCHEMA = {
        "type": "object",
        "properties": {
            "type": {"type": "string"},
            "qualified_name": {"type": ["string", "null"]},
            "lines": {
                "type": "string",
                "description": "Line number, or 'start-end' if the node spans multiple lines.",
            },
            "signature": {"type": "string"},
            "docstring": {"type": ["string", "null"]},
            "children": {"type": "array", "items": {"$ref": "#/$defs/outline_node"}},
        },
        "required": ["type", "qualified_name", "lines", "signature", "docstring", "children"],
    }
  reason: "node_summary durch das stark typisierte, mit outline.py geteilte OutlineNode-Modell samt Hilfsfunktionen ersetzen, damit List/Find outline-gleiche Knoten liefern"
```
/answer 46c8-6 allow
Control Request:
```yaml
id: 46c8-7
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  start: from xy.ai.mcpc.tools.ast import core
  content: |-
    from xy.ai.mcpc.tools.ast import core
    from xy.ai.mcpc.tools.ast.core import OutlineNode
  reason: OutlineNode wird jetzt zentral in core.py definiert und von outline.py wiederverwendet/re-exportiert
```
/answer 46c8-7 allow
Control Request:
```yaml
id: 46c8-8
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  start: |-
    @dataclass(frozen=True)
    class OutlineNode:
        """One AST statement in the outline tree.

        Attributes:
            type: The node's exact AST type, e.g. ``"ClassDef"`` or ``"Import"``.
            qualified_name: Dotted path, for classes/functions only; ``None`` otherwise.
            lines: Line number, or a ``"start-end"`` range if the node spans several lines.
            signature: One-line rendering of the node's header (or the statement itself).
            docstring: Short docstring, only possible for classes/functions.
            children: Nested outline entries for a class's body; empty otherwise.
        """

        type: str
        qualified_name: str | None
        lines: str
        signature: str
        docstring: str | None
        children: list["OutlineNode"] = field(default_factory=list)


    @dataclass(frozen=True)
    class FileOutline:
  content: |-
    @dataclass(frozen=True)
    class FileOutline:
  reason: "Lokale OutlineNode-Definition entfernen, da sie jetzt aus core.py importiert wird"
```
/answer 46c8-8 allow
Control Request:
```yaml
id: 46c8-9
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  start: |-
    def _line_range(node: ast.stmt) -> str:
        end = getattr(node, "end_lineno", node.lineno)
        return str(node.lineno) if end == node.lineno else f"{node.lineno}-{end}"


    def _decorators(node: ast.FunctionDef | ast.AsyncFunctionDef | ast.ClassDef) -> str:
        return "".join(f"@{ast.unparse(d)} " for d in node.decorator_list)


    def _signature(node: ast.stmt, limit: int = 80) -> str:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            keyword = "async def" if isinstance(node, ast.AsyncFunctionDef) else "def"
            returns = f" -> {ast.unparse(node.returns)}" if node.returns is not None else ""
            return f"{_decorators(node)}{keyword} {node.name}({ast.unparse(node.args)}){returns}:"
        if isinstance(node, ast.ClassDef):
            bases = [ast.unparse(b) for b in node.bases] + [
                f"{kw.arg}={ast.unparse(kw.value)}" for kw in node.keywords
            ]
            bases_str = f"({', '.join(bases)})" if bases else ""
            return f"{_decorators(node)}class {node.name}{bases_str}:"
        first_line = ast.unparse(node).splitlines()[0]
        return first_line if len(first_line) <= limit else first_line[: limit - 1] + "…"


    def _outline_body(body: list[ast.stmt], qualified_name: str | None) -> list[OutlineNode]:
  content: "def _outline_body(body: list[ast.stmt], qualified_name: str | None) -> list[OutlineNode]:"
  reason: "Duplizierte Zeilen/Signatur-Hilfsfunktionen entfernen, jetzt zentral in core.py als line_range/node_signature verfügbar"
```
/answer 46c8-9 allow
Control Request:
```yaml
id: 46c8-a
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  start: |2-
                    lines=_line_range(node),
                    signature=_signature(node),
  content: |2-
                    lines=core.line_range(node),
                    signature=core.node_signature(node),
  reason: Aufrufe auf die neuen zentralen core-Hilfsfunktionen umstellen
```
/answer 46c8-a allow
Control Request:
```yaml
id: 46c8-b
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  start: |-
    _OUTLINE_NODE_SCHEMA = {
        "type": "object",
        "properties": {
            "type": {"type": "string"},
            "qualified_name": {"type": ["string", "null"]},
            "lines": {
                "type": "string",
                "description": "Line number, or 'start-end' if the node spans multiple lines.",
            },
            "signature": {"type": "string"},
            "docstring": {"type": ["string", "null"]},
            "children": {"type": "array", "items": {"$ref": "#/$defs/outline_node"}},
        },
        "required": ["type", "qualified_name", "lines", "signature", "docstring", "children"],
    }

    _FILE_OUTLINE_SCHEMA = {
  content: "_FILE_OUTLINE_SCHEMA = {"
  reason: "Doppelte Schema-Definition entfernen, wird jetzt zentral als core.OUTLINE_NODE_SCHEMA bereitgestellt"
```
/answer 46c8-b allow
Control Request:
```yaml
id: 46c8-c
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  start: |2-
        output_schema = {
            "$defs": {"outline_node": _OUTLINE_NODE_SCHEMA},
  content: |2-
        output_schema = {
            "$defs": {"outline_node": core.OUTLINE_NODE_SCHEMA},
  reason: OutlineTool referenziert jetzt die zentrale Schema-Konstante aus core.py
```
/answer 46c8-c allow
Control Request:
```yaml
id: 46c8-d
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/outline.py
  reason: Kontrolle des Ergebnisses nach den Refactoring-Edits in outline.py
```
/answer 46c8-d allow
Control Request:
```yaml
id: 46c8-e
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  start: |-
    import ast
    from dataclasses import dataclass
    from typing import Any
  content: |-
    import ast
    from dataclasses import asdict, dataclass, field
    from typing import Any
  reason: "asdict/field werden für die neuen stark typisierten Node-Ergebnisse (OutlineNode, ReadNode) benötigt"
```
/answer 46c8-e allow
Control Request:
```yaml
id: 46c8-f
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  start: |-
    __all__ = [
        "ListNodesResult",
        "FindNodesResult",
        "InsertNodeResult",
        "ReplaceNodeResult",
        "DeleteNodeResult",
        "CreateNodeResult",
        "python_ast_list",
        "python_ast_find",
        "python_ast_insert",
        "python_ast_replace",
        "python_ast_delete",
        "python_ast_create",
        "ListNodesTool",
        "FindNodesTool",
        "InsertNodeTool",
        "ReplaceNodeTool",
        "DeleteNodeTool",
        "CreateNodeTool",
        "register",
    ]
  content: |-
    __all__ = [
        "ListNodesResult",
        "FindNodesResult",
        "ReadNode",
        "ReadNodeResult",
        "InsertNodeResult",
        "ReplaceNodeResult",
        "DeleteNodeResult",
        "CreateNodeResult",
        "python_ast_list",
        "python_ast_find",
        "python_ast_read",
        "python_ast_insert",
        "python_ast_replace",
        "python_ast_delete",
        "python_ast_create",
        "ListNodesTool",
        "FindNodesTool",
        "ReadNodeTool",
        "InsertNodeTool",
        "ReplaceNodeTool",
        "DeleteNodeTool",
        "CreateNodeTool",
        "register",
    ]
  reason: Neue Read-Bausteine im öffentlichen API der crud.py ergänzen
```
/answer 46c8-f allow
Control Request:
```yaml
id: 46c8-10
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  start: |-
    def _list_output_schema() -> dict[str, Any]:
        return {
            "type": "object",
            "properties": {
                "nodes": {"type": "array", "items": {"type": "object"}, "description": "Only structural information"},
                "count": {"type": "integer"},
            },
            "required": ["nodes", "count"],
        }


    @dataclass(frozen=True)
    class ListNodesResult:
        """Result of :func:`python_ast_list`.

        Attributes:
            nodes: Node summaries (see :func:`core.node_summary`) in document order.
            count: Number of entries in ``nodes``.
        """

        nodes: list[dict[str, Any]]
        count: int


    @dataclass(frozen=True)
    class FindNodesResult:
        """Result of :func:`python_ast_find`.

        Attributes:
            nodes: Node summaries matching the given selectors.
            count: Number of entries in ``nodes``.
        """

        nodes: list[dict[str, Any]]
        count: int
  content: |-
    def _list_output_schema() -> dict[str, Any]:
        return {
            "$defs": {"outline_node": core.OUTLINE_NODE_SCHEMA},
            "type": "object",
            "properties": {
                "nodes": {"type": "array", "items": {"$ref": "#/$defs/outline_node"}},
                "count": {"type": "integer"},
            },
            "required": ["nodes", "count"],
        }


    @dataclass(frozen=True)
    class ListNodesResult:
        """Result of :func:`python_ast_list`.

        Attributes:
            nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
                document order, suited for retrieval and navigation.
            count: Number of entries in ``nodes``.
        """

        nodes: list[core.OutlineNode]
        count: int


    @dataclass(frozen=True)
    class FindNodesResult:
        """Result of :func:`python_ast_find`.

        Attributes:
            nodes: Outline-style node descriptions (see :class:`core.OutlineNode`)
                matching the given selectors, suited for retrieval and navigation.
            count: Number of entries in ``nodes``.
        """

        nodes: list[core.OutlineNode]
        count: int


    @dataclass(frozen=True)
    class ReadNode:
        """One node in a subtree read for block-wise edit/replace.

        Attributes:
            type: The node's exact AST type, e.g. ``"ClassDef"`` or ``"FunctionDef"``.
            qualified_name: Dotted path, for classes/functions/imports only; ``None`` otherwise.
            lines: Line number, or a ``"start-end"`` range if the node spans several lines.
            code: The node's full source, usable as-is with ``python_ast_replace``; ``None``
                if the node's body consists solely of the nested classes/functions listed
                in ``children`` (whose source is then given by those children instead).
            children: Nested read entries, populated only when ``code`` is ``None``.
        """

        type: str
        qualified_name: str | None
        lines: str
        code: str | None
        children: list["ReadNode"] = field(default_factory=list)


    @dataclass(frozen=True)
    class ReadNodeResult:
        """Result of :func:`python_ast_read`.

        Attributes:
            node: The selected node, expanded recursively.
        """

        node: ReadNode
  reason: "List/Find auf outline-gleiche, stark typisierte Knoten umstellen und die neuen Read-Result-Typen ergänzen"
```
/answer 46c8-10 allow
ReasoningToken: 11100
Text:
 Jetzt `outline.py` anpassen, um `OutlineNode` und die Hilfsfunktionen aus `core` zu beziehen:
Control Request:
```yaml
id: 46c8-11
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  start: |2-
        tree = core.tree_from_input(path, code)
        located = core.locate_all(tree)
        summaries = [
            core.node_summary(loc)
            for loc in located
            if node_type is None or type(loc.node).__name__.lower() == node_type.lower()
        ]
        return ListNodesResult(nodes=summaries, count=len(summaries))
  content: |2-
        tree = core.tree_from_input(path, code)
        located = core.locate_all(tree)
        nodes = [
            core.node_outline(loc)
            for loc in located
            if node_type is None or type(loc.node).__name__.lower() == node_type.lower()
        ]
        return ListNodesResult(nodes=nodes, count=len(nodes))
  reason: python_ast_list liefert jetzt outline-gleiche OutlineNode-Objekte statt loser dicts
```
/answer 46c8-11 allow
Control Request:
```yaml
id: 46c8-12
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  start: |2-
            parent_type=parent_type,
        )
        return FindNodesResult(nodes=[core.node_summary(h) for h in hits], count=len(hits))
  content: |2-
            parent_type=parent_type,
        )
        return FindNodesResult(nodes=[core.node_outline(h) for h in hits], count=len(hits))
  reason: python_ast_find liefert jetzt outline-gleiche OutlineNode-Objekte statt loser dicts
```
/answer 46c8-12 allow
Control Request:
```yaml
id: 46c8-13
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  start: |2-
        return FindNodesResult(nodes=[core.node_outline(h) for h in hits], count=len(hits))


    def python_ast_insert(
  content: |2-
        return FindNodesResult(nodes=[core.node_outline(h) for h in hits], count=len(hits))


    def _only_defs(body: list[ast.stmt]) -> bool:
        """Whether *body* is non-empty and consists solely of nested classes/functions."""
        return bool(body) and all(isinstance(n, (ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef)) for n in body)


    def _read_node(node: ast.stmt, qualified_name: str | None) -> ReadNode:
        body = getattr(node, "body", None)
        if isinstance(body, list) and _only_defs(body):
            children = [
                _read_node(child, f"{qualified_name}.{child.name}" if qualified_name else child.name)
                for child in body
            ]
            return ReadNode(
                type=type(node).__name__,
                qualified_name=qualified_name,
                lines=core.line_range(node),
                code=None,
                children=children,
            )
        return ReadNode(
            type=type(node).__name__,
            qualified_name=qualified_name,
            lines=core.line_range(node),
            code=core.unparse(node),
            children=[],
        )


    def python_ast_read(
        path: str | None = None,
        code: str | None = None,
        *,
        qualified_name: str | None = None,
        name: str | None = None,
        node_type: str | None = None,
        lineno: int | None = None,
        end_lineno: int | None = None,
        parent_type: str | None = None,
    ) -> ReadNodeResult:
        """Recursively read the selected node's subtree for block-wise edit/replace.

        A node whose body consists solely of nested classes/functions is expanded into
        ``children`` instead of source, so the agent can descend to the innermost block
        that actually needs editing; any other node is returned whole, as ``code`` ready
        to hand back to ``python_ast_replace`` via its ``qualified_name``.

        Args:
            path: Absolute path to the Python file to read. Mutually usable with ``code``;
                exactly one of the two must be given.
            code: Python source to parse instead of reading ``path``.
            qualified_name: Selector – exact Python-style FQN of the target node.
            name: Selector – exact simple name of the target node.
            node_type: Selector – AST node class name of the target node.
            lineno: Selector – exact start line of the target node.
            end_lineno: Selector – exact end line of the target node.
            parent_type: Selector – AST class name of the target node's container.

        Returns:
            ReadNodeResult: The selected node's subtree.

        Raises:
            core.AstError: If neither ``path`` nor ``code`` is given, if ``path`` is not
                absolute or does not point to an existing regular file, the source has a
                syntax error, or the selector matches zero or more than one node.
        """
        tree = core.tree_from_input(path, code)
        target = _select_one(
            tree,
            qualified_name=qualified_name,
            name=name,
            node_type=node_type,
            lineno=lineno,
            end_lineno=end_lineno,
            parent_type=parent_type,
        )
        return ReadNodeResult(node=_read_node(target.node, target.qualified_name))


    def python_ast_insert(
  reason: "Neues Read-Tool implementieren: rekursives Ausgeben eines Teilbaums mit Qualified Name und Code für blockweises Edit/Replace"
```
/answer 46c8-13 allow
Control Request:
```yaml
id: 46c8-14
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  start: |2-
                result = python_ast_list(path=args.get("path"), code=args.get("code"), node_type=args.get("node_type"))
            except core.AstError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={"nodes": result.nodes, "count": result.count})
  content: |2-
                result = python_ast_list(path=args.get("path"), code=args.get("code"), node_type=args.get("node_type"))
            except core.AstError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={"nodes": [asdict(n) for n in result.nodes], "count": result.count})
  reason: ListNodesTool serialisiert jetzt OutlineNode-Dataclasses statt bereits vorliegender dicts
```
/answer 46c8-14 allow
Control Request:
```yaml
id: 46c8-15
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  start: |2-
                    parent_type=args.get("parent_type"),
                )
            except core.AstError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={"nodes": result.nodes, "count": result.count})


    class InsertNodeTool(ToolDefinition):
  content: |2-
                    parent_type=args.get("parent_type"),
                )
            except core.AstError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={"nodes": [asdict(n) for n in result.nodes], "count": result.count})


    _READ_NODE_SCHEMA = {
        "type": "object",
        "properties": {
            "type": {"type": "string"},
            "qualified_name": {"type": ["string", "null"]},
            "lines": {
                "type": "string",
                "description": "Line number, or 'start-end' if the node spans multiple lines.",
            },
            "code": {
                "type": ["string", "null"],
                "description": (
                    "Full source of this node, ready for python_ast_replace; null if the node "
                    "consists solely of the nested classes/functions listed in 'children'."
                ),
            },
            "children": {"type": "array", "items": {"$ref": "#/$defs/read_node"}},
        },
        "required": ["type", "qualified_name", "lines", "code", "children"],
    }


    class ReadNodeTool(ToolDefinition):
        name = "python_ast_read"
        title = "Read AST subtree"
        description = (
            "Recursively read the selected node's subtree, surfacing each block's qualified "
            "name and source so it can be handed back to python_ast_replace. Nodes whose body "
            "consists solely of nested classes/functions are expanded into 'children' instead "
            "of source, letting the agent descend to the innermost block that needs editing."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "code": {"type": "string", "description": "Python source to parse instead of a file."},
                **_SELECTOR_PROPS,
            },
            "required": [],
        }
        output_schema = {
            "$defs": {"read_node": _READ_NODE_SCHEMA},
            "type": "object",
            "properties": {"node": {"$ref": "#/$defs/read_node"}},
            "required": ["node"],
        }
        annotations = {"readOnlyHint": True, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            """Delegate to :func:`python_ast_read`, translating the MCP schema to/from the Python API."""
            args: dict[str, Any] = ctx.arguments
            try:
                result = python_ast_read(
                    path=args.get("path"),
                    code=args.get("code"),
                    qualified_name=args.get("qualified_name"),
                    name=args.get("name"),
                    node_type=args.get("node_type"),
                    lineno=args.get("lineno"),
                    end_lineno=args.get("end_lineno"),
                    parent_type=args.get("parent_type"),
                )
            except core.AstError as exc:
                return ToolResult(content=[text_content(str(exc))], is_error=True)
            return ToolResult(structured_content={"node": asdict(result.node)})


    class InsertNodeTool(ToolDefinition):
  reason: FindNodesTool-Serialisierung anpassen und neues ReadNodeTool mit rekursivem JSON-Schema hinzufügen
```
/answer 46c8-15 allow
Control Request:
```yaml
id: 46c8-16
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  start: |2-
        registry.register(ListNodesTool())
        registry.register(FindNodesTool())
        registry.register(InsertNodeTool())
        registry.register(ReplaceNodeTool())
        registry.register(DeleteNodeTool())
        registry.register(CreateNodeTool())
        functions.register(python_ast_list)
        functions.register(python_ast_find)
        functions.register(python_ast_insert)
  content: |2-
        registry.register(ListNodesTool())
        registry.register(FindNodesTool())
        registry.register(ReadNodeTool())
        registry.register(InsertNodeTool())
        registry.register(ReplaceNodeTool())
        registry.register(DeleteNodeTool())
        registry.register(CreateNodeTool())
        functions.register(python_ast_list)
        functions.register(python_ast_find)
        functions.register(python_ast_read)
        functions.register(python_ast_insert)
  reason: Neues Read-Tool und -Funktion in der Registry registrieren
```
/answer 46c8-16 allow
Control Request:
```yaml
id: 46c8-17
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/crud.py
  reason: Vollständige Überprüfung der Datei nach allen Edits auf Konsistenz und Korrektheit
```
/answer 46c8-17 allow
ReasoningToken: 23
Text:
 Gut, outline.py sieht korrekt aus. Jetzt crud.py umbauen: typisierte List/Find-Ergebnisse und das neue Read-Tool ergänzen.
Text:
 Nun die Result-Dataclasses (List/Find) auf `core.OutlineNode` umstellen und `ReadNode`/`ReadNodeResult` ergänzen:
Text:
 Jetzt die Implementierungen von `python_ast_list`/`python_ast_find` anpassen und `python_ast_read` neu hinzufügen:
ReasoningToken: 79
Text:
 Nun die Tool-Klassen anpassen: `ListNodesTool`/`FindNodesTool` müssen die Dataclass-Ergebnisse serialisieren, und `ReadNodeTool` muss ergänzt werden.
Text:
 Jetzt noch die `register`-Funktion erweitern:
Text:
 Jetzt kontrolliere ich das vollständige, aktualisierte `crud.py`, um Konsistenz sicherzustellen:
Alles konsistent. Zusammenfassung der Änderungen:

- **`core.py`**: `node_summary` durch das gemeinsam genutzte, stark typisierte `OutlineNode` ersetzt (samt `line_range`, `node_signature`, `node_outline` und `OUTLINE_NODE_SCHEMA`), damit outline.py und crud.py dieselbe Struktur verwenden.
- **`outline.py`**: nutzt jetzt `OutlineNode`/`line_range`/`node_signature`/`OUTLINE_NODE_SCHEMA` aus `core` statt eigener Duplikate.
- **`crud.py`**:
  - `ListNodesResult`/`FindNodesResult` liefern jetzt `list[core.OutlineNode]` statt loser Dicts – die Rückgaben gleichen der Outline und eignen sich für Retrieval/Navigation.
  - Neu: `ReadNode`/`ReadNodeResult`, `python_ast_read` und `ReadNodeTool`. Das Read-Tool selektiert wie `find`/`replace`/`delete` genau einen Knoten und gibt ihn rekursiv aus: Besteht der Körper eines Knotens ausschließlich aus verschachtelten Klassen/Funktionen, wird `code` auf `None` gesetzt und stattdessen in `children` weiter aufgeschlüsselt; sobald der Körper mindestens ein anderes Statement enthält, wird der komplette Knoten als `code` (unparsed) zurückgegeben – jeweils mit `qualified_name`, damit der Agent den Block direkt per `python_ast_replace` ersetzen kann.
