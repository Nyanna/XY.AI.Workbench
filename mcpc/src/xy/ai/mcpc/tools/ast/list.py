"""``ast_list`` tool: list AST nodes of a file or source snippet."""


from dataclasses import asdict, dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import list_output_schema
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["ListNodesResult", "ast_list", "ListNodesTool", "register"]


@dataclass(frozen=True)
class ListNodesResult:
    """Result of :func:`ast_list`.

    Attributes:
        nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
            document order, suited for retrieval and navigation.
        count: Number of entries in ``nodes``.
    """

    nodes: list[core.OutlineNode]
    count: int


def ast_list(path: str) -> ListNodesResult:
    """List the hierarchical AST-node tree of a file.

    The tree is the foundation every other tool builds on: each node carries its
    unique, primarily name-based ``id`` and line range, but never its source –
    use ``ast_find`` (property/text filtering) or ``ast_read`` (by id) to
    retrieve source.

    Args:
        path: Absolute path to the file to read.

    Returns:
        ListNodesResult: The nested node tree and the number of top-level nodes.

    Raises:
        core.AstError: If ``path`` is not absolute or does not point to an existing
            regular file, or if the source has a syntax error.
    """
    tree = core.load(path)[1]
    nodes = core.build_outline(core.locate_all(tree))
    return ListNodesResult(nodes=nodes, count=len(nodes))


class ListNodesTool(ToolDefinition):
    name = "ast_list"
    title = "List AST nodes"
    description = (
        "Hierarchical tree of a file's AST nodes (import/statement segments, classes, "
        "functions, sections) with id and line range – no source. Use ast_find to "
        "filter/search and get source, ast_read to read source by id."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the file."},
        },
        "required": ["path"],
    }
    output_schema = list_output_schema()
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_list`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_list(path=args.get("path"))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"nodes": [asdict(n) for n in result.nodes], "count": result.count})


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ListNodesTool())
    functions.register(ast_list)
