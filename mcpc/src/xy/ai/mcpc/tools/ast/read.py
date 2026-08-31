"""``ast_read`` tool: recursively read a node's subtree for block-wise edit/replace."""

import ast
from dataclasses import asdict, dataclass, field
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS, select_one
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["ReadNode", "ReadNodeResult", "ast_read", "ReadNodeTool", "register"]


@dataclass(frozen=True)
class ReadNode:
    """One node in a subtree read for block-wise edit/replace.

    Attributes:
        type: The node's exact AST type, e.g. ``"ClassDef"`` or ``"FunctionDef"``.
        qualified_name: Dotted path, for classes/functions/imports only; ``None`` otherwise.
        lines: Line number, or a ``"start-end"`` range if the node spans several lines.
        code: The node's full source, usable as-is with ``ast_replace``; ``None``
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
    """Result of :func:`ast_read`.

    Attributes:
        node: The selected node, expanded recursively.
    """

    node: ReadNode


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


def ast_read(
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
    to hand back to ``ast_replace`` via its ``qualified_name``.

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
    target = select_one(
        tree,
        qualified_name=qualified_name,
        name=name,
        node_type=node_type,
        lineno=lineno,
        end_lineno=end_lineno,
        parent_type=parent_type,
    )
    return ReadNodeResult(node=_read_node(target.node, target.qualified_name))


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
                "Full source of this node, ready for ast_replace; null if the node "
                "consists solely of the nested classes/functions listed in 'children'."
            ),
        },
        "children": {"type": "array", "items": {"$ref": "#/$defs/read_node"}},
    },
    "required": ["type", "qualified_name", "lines", "code", "children"],
}


class ReadNodeTool(ToolDefinition):
    name = "ast_read"
    title = "Read AST subtree"
    description = (
        "Recursively read the selected node's subtree, surfacing each block's qualified "
        "name and source so it can be handed back to ast_replace. Nodes whose body "
        "consists solely of nested classes/functions are expanded into 'children' instead "
        "of source, letting the agent descend to the innermost block that needs editing."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Python source to parse instead of a file."},
            **SELECTOR_PROPS,
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
        """Delegate to :func:`ast_read`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_read(
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


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReadNodeTool())
    functions.register(ast_read)
