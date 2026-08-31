"""``python_ast_find`` tool: find AST nodes by type, name, qualified name, line range or parent type."""


from dataclasses import asdict, dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.crud_common import SELECTOR_PROPS, list_output_schema
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["FindNodesResult", "python_ast_find", "FindNodesTool", "register"]


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


class FindNodesTool(ToolDefinition):
    name = "python_ast_find"
    title = "Find AST nodes"
    description = "Find AST nodes by type, name, qualified name, line range or parent type."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Python source to parse instead of a file."},
            **SELECTOR_PROPS,
        },
        "required": [],
    }
    output_schema = list_output_schema()
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


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(FindNodesTool())
    functions.register(python_ast_find)
