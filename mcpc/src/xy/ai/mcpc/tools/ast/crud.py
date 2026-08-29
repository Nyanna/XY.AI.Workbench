"""Node-level CRUD tools: ``python_ast_{list,find,insert,replace,delete,create}``.

These operate on the typed AST directly and are the foundation the ``imports``,
``classes`` and ``functions`` convenience layers build on.
"""


import ast
from dataclasses import asdict, dataclass, field
from typing import Any

from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

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

#: Shared JSON-Schema fragment for the node selectors accepted by find/insert/replace/delete.
_SELECTOR_PROPS = {
    "qualified_name": {"type": "string", "description": "Python-style FQN of the target node."},
    "name": {"type": "string", "description": "Simple node name."},
    "node_type": {"type": "string", "description": "AST node class name, e.g. 'FunctionDef'."},
    "lineno": {"type": "integer", "description": "Start line of the target node."},
    "end_lineno": {"type": "integer", "description": "End line of the target node."},
    "parent_type": {"type": "string", "description": "AST class name of the container."},
}


def _select_one(tree: ast.Module, **selectors: Any) -> core.Located:
    """Return the single node in *tree* matching *selectors*.

    Raises:
        core.AstError: If no node matches, or more than one node matches.
    """
    hits = core.find(tree, **selectors)
    if not hits:
        raise core.AstError("No node matched the selector.")
    if len(hits) > 1:
        raise core.AstError(f"Selector is ambiguous – {len(hits)} nodes matched.")
    return hits[0]


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


@dataclass(frozen=True)
class InsertNodeResult:
    """Result of :func:`python_ast_insert`.

    Attributes:
        result: Always ``"success"``.
        inserted: Number of top-level statements parsed from ``code`` and inserted.
    """

    result: str
    inserted: int


@dataclass(frozen=True)
class ReplaceNodeResult:
    """Result of :func:`python_ast_replace`.

    Attributes:
        result: Always ``"success"``.
    """

    result: str


@dataclass(frozen=True)
class DeleteNodeResult:
    """Result of :func:`python_ast_delete`.

    Attributes:
        result: Always ``"success"``.
    """

    result: str


@dataclass(frozen=True)
class CreateNodeResult:
    """Result of :func:`python_ast_create`.

    Attributes:
        result: Always ``"success"``.
        created: Number of top-level statements parsed from ``code`` and appended.
    """

    result: str
    created: int


def python_ast_list(path: str | None = None, code: str | None = None, node_type: str | None = None) -> ListNodesResult:
    """List AST nodes (imports, classes, functions, statements) of a file or source snippet.

    Args:
        path: Absolute path to the Python file to read. Mutually usable with ``code``;
            exactly one of the two must be given.
        code: Python source to parse instead of reading ``path``.
        node_type: Restrict the result to this AST node class name (case-insensitive),
            e.g. ``"FunctionDef"``. ``None`` returns every node.

    Returns:
        ListNodesResult: The matching node summaries and their count.

    Raises:
        core.AstError: If neither ``path`` nor ``code`` is given, if ``path`` is not
            absolute or does not point to an existing regular file, or if the source
            has a syntax error.
    """
    tree = core.tree_from_input(path, code)
    located = core.locate_all(tree)
    nodes = [
        core.node_outline(loc)
        for loc in located
        if node_type is None or type(loc.node).__name__.lower() == node_type.lower()
    ]
    return ListNodesResult(nodes=nodes, count=len(nodes))


def python_ast_find(
    path: str | None = None,
    code: str | None = None,
    *,
    qualified_name: str | None = None,
    name: str | None = None,
    node_type: str | None = None,
    lineno: int | None = None,
    end_lineno: int | None = None,
    parent_type: str | None = None,
) -> FindNodesResult:
    """Find AST nodes by type, name, qualified name, line range or parent type.

    Args:
        path: Absolute path to the Python file to read. Mutually usable with ``code``;
            exactly one of the two must be given.
        code: Python source to parse instead of reading ``path``.
        qualified_name: Exact Python-style FQN a node's ``qualified_name`` must equal.
        name: Exact simple name a node's ``name`` must equal.
        node_type: AST node class name a node must match (case-insensitive).
        lineno: Exact start line a node must match.
        end_lineno: Exact end line a node must match.
        parent_type: AST class name of the enclosing container a node must match
            (case-insensitive).

    Returns:
        FindNodesResult: The matching node summaries and their count. Any number of
        matches (including zero) is a normal, successful result.

    Raises:
        core.AstError: If neither ``path`` nor ``code`` is given, if ``path`` is not
            absolute or does not point to an existing regular file, or if the source
            has a syntax error.
    """
    tree = core.tree_from_input(path, code)
    hits = core.find(
        tree,
        qualified_name=qualified_name,
        name=name,
        node_type=node_type,
        lineno=lineno,
        end_lineno=end_lineno,
        parent_type=parent_type,
    )
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
    path: str,
    code: str,
    *,
    position: str = "after",
    qualified_name: str | None = None,
    name: str | None = None,
    node_type: str | None = None,
    lineno: int | None = None,
    end_lineno: int | None = None,
    parent_type: str | None = None,
) -> InsertNodeResult:
    """Insert statement(s) parsed from ``code`` relative to a selected node.

    Args:
        path: Absolute path to the Python file to modify.
        code: Python source of the statement(s) to insert.
        position: ``"before"`` or ``"after"`` the selected node. Defaults to ``"after"``.
        qualified_name: Selector – exact Python-style FQN of the target node.
        name: Selector – exact simple name of the target node.
        node_type: Selector – AST node class name of the target node.
        lineno: Selector – exact start line of the target node.
        end_lineno: Selector – exact end line of the target node.
        parent_type: Selector – AST class name of the target node's container.

    Returns:
        InsertNodeResult: Success status and the number of statements inserted.

    Raises:
        core.AstError: If ``path`` is invalid, ``code`` has a syntax error, or the
            selector matches zero or more than one node.
    """
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    new_nodes = core.parse_snippet(code)
    target = _select_one(
        tree,
        qualified_name=qualified_name,
        name=name,
        node_type=node_type,
        lineno=lineno,
        end_lineno=end_lineno,
        parent_type=parent_type,
    )
    body = target.parent.body  # type: ignore[attr-defined]
    offset = 1 if position == "after" else 0
    index = body.index(target.node) + offset
    body[index:index] = new_nodes
    core.CACHE.save(file_path, tree)
    return InsertNodeResult(result="success", inserted=len(new_nodes))


def python_ast_replace(
    path: str,
    code: str,
    *,
    qualified_name: str | None = None,
    name: str | None = None,
    node_type: str | None = None,
    lineno: int | None = None,
    end_lineno: int | None = None,
    parent_type: str | None = None,
) -> ReplaceNodeResult:
    """Replace the single selected node with statement(s) parsed from ``code``.

    Args:
        path: Absolute path to the Python file to modify.
        code: Replacement Python source.
        qualified_name: Selector – exact Python-style FQN of the target node.
        name: Selector – exact simple name of the target node.
        node_type: Selector – AST node class name of the target node.
        lineno: Selector – exact start line of the target node.
        end_lineno: Selector – exact end line of the target node.
        parent_type: Selector – AST class name of the target node's container.

    Returns:
        ReplaceNodeResult: Success status.

    Raises:
        core.AstError: If ``path`` is invalid, ``code`` has a syntax error, or the
            selector matches zero or more than one node.
    """
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    new_nodes = core.parse_snippet(code)
    target = _select_one(
        tree,
        qualified_name=qualified_name,
        name=name,
        node_type=node_type,
        lineno=lineno,
        end_lineno=end_lineno,
        parent_type=parent_type,
    )
    core.replace_in_body(target, new_nodes)
    core.CACHE.save(file_path, tree)
    return ReplaceNodeResult(result="success")


def python_ast_delete(
    path: str,
    *,
    qualified_name: str | None = None,
    name: str | None = None,
    node_type: str | None = None,
    lineno: int | None = None,
    end_lineno: int | None = None,
    parent_type: str | None = None,
) -> DeleteNodeResult:
    """Delete the single selected node from a Python file.

    Args:
        path: Absolute path to the Python file to modify.
        qualified_name: Selector – exact Python-style FQN of the target node.
        name: Selector – exact simple name of the target node.
        node_type: Selector – AST node class name of the target node.
        lineno: Selector – exact start line of the target node.
        end_lineno: Selector – exact end line of the target node.
        parent_type: Selector – AST class name of the target node's container.

    Returns:
        DeleteNodeResult: Success status.

    Raises:
        core.AstError: If ``path`` is invalid, or the selector matches zero or more
            than one node.
    """
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    target = _select_one(
        tree,
        qualified_name=qualified_name,
        name=name,
        node_type=node_type,
        lineno=lineno,
        end_lineno=end_lineno,
        parent_type=parent_type,
    )
    core.delete_from_body(target)
    core.CACHE.save(file_path, tree)
    return DeleteNodeResult(result="success")


def python_ast_create(path: str, code: str) -> CreateNodeResult:
    """Append statement(s) parsed from ``code`` to a Python file's top level.

    Args:
        path: Absolute path to the Python file to modify or create (its parent
            directory must already exist).
        code: Python source of the statement(s) to append.

    Returns:
        CreateNodeResult: Success status and the number of statements appended.

    Raises:
        core.AstError: If ``path`` is not absolute, or ``code`` has a syntax error.
    """
    file_path = core.require_path(path, must_exist=False)
    new_nodes = core.parse_snippet(code)
    tree = core.CACHE.get_tree(file_path) if file_path.exists() else ast.Module(body=[], type_ignores=[])
    tree.body.extend(new_nodes)
    core.CACHE.save(file_path, tree)
    return CreateNodeResult(result="success", created=len(new_nodes))


class ListNodesTool(ToolDefinition):
    name = "python_ast_list"
    title = "List AST nodes"
    description = "List AST nodes (imports, classes, functions, statements) of a Python file, optionally filtered by type."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Python source to parse instead of a file."},
            "node_type": {"type": "string", "description": "Restrict to this AST node class name."},
        },
        "required": [],
    }
    output_schema = _list_output_schema()
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`python_ast_list`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = python_ast_list(path=args.get("path"), code=args.get("code"), node_type=args.get("node_type"))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"nodes": [asdict(n) for n in result.nodes], "count": result.count})


class FindNodesTool(ToolDefinition):
    name = "python_ast_find"
    title = "Find AST nodes"
    description = "Find AST nodes by type, name, qualified name, line range or parent type."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Python source to parse instead of a file."},
            **_SELECTOR_PROPS,
        },
        "required": [],
    }
    output_schema = _list_output_schema()
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`python_ast_find`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = python_ast_find(
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
    name = "python_ast_insert"
    title = "Insert AST node"
    description = "Insert statement(s) parsed from code relative to a selected node ('before' or 'after')."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Python source of the statement(s) to insert."},
            "position": {
                "type": "string",
                "enum": ["before", "after"],
                "description": "Placement relative to the selected node.",
                "default": "after",
            },
            **_SELECTOR_PROPS,
        },
        "required": ["path", "code"],
    }
    output_schema = {
        "type": "object",
        "properties": {"result": {"type": "string"}, "inserted": {"type": "integer"}},
        "required": ["result", "inserted"],
    }
    annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`python_ast_insert`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = python_ast_insert(
                args["path"],
                args["code"],
                position=args.get("position", "after"),
                qualified_name=args.get("qualified_name"),
                name=args.get("name"),
                node_type=args.get("node_type"),
                lineno=args.get("lineno"),
                end_lineno=args.get("end_lineno"),
                parent_type=args.get("parent_type"),
            )
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result, "inserted": result.inserted}, auto_approve=True)


class ReplaceNodeTool(ToolDefinition):
    name = "python_ast_replace"
    title = "Replace AST node"
    description = "Replace the single selected node with statement(s) parsed from code."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Replacement Python source."},
            **_SELECTOR_PROPS,
        },
        "required": ["path", "code"],
    }
    output_schema = {
        "type": "object",
        "properties": {"result": {"type": "string"}},
        "required": ["result"],
    }
    annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`python_ast_replace`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = python_ast_replace(
                args["path"],
                args["code"],
                qualified_name=args.get("qualified_name"),
                name=args.get("name"),
                node_type=args.get("node_type"),
                lineno=args.get("lineno"),
                end_lineno=args.get("end_lineno"),
                parent_type=args.get("parent_type"),
            )
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result}, auto_approve=True)


class DeleteNodeTool(ToolDefinition):
    name = "python_ast_delete"
    title = "Delete AST node"
    description = "Delete the single selected node from a Python file."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            **_SELECTOR_PROPS,
        },
        "required": ["path"],
    }
    output_schema = {
        "type": "object",
        "properties": {"result": {"type": "string"}},
        "required": ["result"],
    }
    annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`python_ast_delete`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = python_ast_delete(
                args["path"],
                qualified_name=args.get("qualified_name"),
                name=args.get("name"),
                node_type=args.get("node_type"),
                lineno=args.get("lineno"),
                end_lineno=args.get("end_lineno"),
                parent_type=args.get("parent_type"),
            )
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result}, auto_approve=True)


class CreateNodeTool(ToolDefinition):
    name = "python_ast_create"
    title = "Create AST node"
    description = "Append statement(s) parsed from code to a Python file's top level (creating the file if needed)."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Python source of the statement(s) to append."},
        },
        "required": ["path", "code"],
    }
    output_schema = {
        "type": "object",
        "properties": {"result": {"type": "string"}, "created": {"type": "integer"}},
        "required": ["result", "created"],
    }
    annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`python_ast_create`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = python_ast_create(args["path"], args["code"])
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result, "created": result.created}, auto_approve=True)


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
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
    functions.register(python_ast_replace)
    functions.register(python_ast_delete)
    functions.register(python_ast_create)
