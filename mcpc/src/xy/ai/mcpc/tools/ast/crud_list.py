"""``python_ast_list`` tool: list AST nodes of a file or source snippet."""


from dataclasses import asdict, dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.crud_common import list_output_schema
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["ListNodesResult", "python_ast_list", "ListNodesTool", "register"]


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
    output_schema = list_output_schema()
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`python_ast_list`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = python_ast_list(path=args.get("path"), code=args.get("code"), node_type=args.get("node_type"))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"nodes": [asdict(n) for n in result.nodes], "count": result.count})


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ListNodesTool())
    functions.register(python_ast_list)
