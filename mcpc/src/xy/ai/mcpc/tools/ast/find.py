"""``ast_find`` tool: find AST nodes by type, name, id, line range or parent type."""


import re
from dataclasses import asdict, dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS, list_output_schema
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["FindNodesResult", "ast_find", "FindNodesTool", "register"]


@dataclass(frozen=True)
class FindNodesResult:
    """Result of :func:`ast_find`.

    Attributes:
        nodes: Outline-style node descriptions (see :class:`core.OutlineNode`)
            matching the given selectors, suited for retrieval and navigation.
        count: Number of entries in ``nodes``.
    """

    nodes: list[core.OutlineNode]
    count: int


def ast_find(
    path: str,
    *,
    id: str | None = None,
    name: str | None = None,
    node_type: str | None = None,
    lineno: int | None = None,
    end_lineno: int | None = None,
    parent_type: str | None = None,
    text: str | None = None,
    regexp: str | None = None,
) -> FindNodesResult:
    """Find nodes by id, type, name, line range, parent type, text or regexp.

    ``ast_find`` is the single retrieval point that restricts on node properties;
    every other tool addresses nodes purely by ``id``. Matches are returned with
    their full source.

    Args:
        path: Absolute path to the file to read.
        id: Engine-independent unique node id (primarily name-based path).
        name: Exact simple name a node's ``name`` must equal.
        node_type: Node type name a node must match (case-insensitive).
        lineno: Exact start line a node must match.
        end_lineno: Exact end line a node must match.
        parent_type: Node type name of the enclosing container (case-insensitive).
        text: Case-insensitive substring the node's source must contain.
        regexp: Regular expression the node's source must match (``re.search``).

    Returns:
        FindNodesResult: The matching node summaries (with source) and their count.
        Any number of matches (including zero) is a normal, successful result.

    Raises:
        core.AstError: If ``path`` is not absolute or does not point to an existing
            regular file, if the source has a syntax error, or if ``regexp`` is not
            a valid regular expression.
    """
    tree = core.load(path)[1]
    hits = core.find(
        tree,
        id=id,
        name=name,
        node_type=node_type,
        lineno=lineno,
        end_lineno=end_lineno,
        parent_type=parent_type,
    )
    if text is not None:
        needle = text.lower()
        hits = [h for h in hits if needle in tree.engine.node_code(h.node).lower()]
    if regexp is not None:
        try:
            pattern = re.compile(regexp)
        except re.error as exc:
            raise core.AstError(f"Invalid regexp: {exc}") from exc
        hits = [h for h in hits if pattern.search(tree.engine.node_code(h.node))]
    return FindNodesResult(nodes=[core.node_outline(h, with_code=True) for h in hits], count=len(hits))


class FindNodesTool(ToolDefinition):
    name = "ast_find"
    title = "Find AST nodes"
    description = (
        "Filter the AST-node tree by type, name, id, line range, parent type, "
        "text substring or regexp – the only retrieval point with property/text "
        "restriction. Returns matches with their full source."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the file."},
            **SELECTOR_PROPS,
            "text": {"type": "string", "description": "Case-insensitive substring the node's source must contain."},
            "regexp": {"type": "string", "description": "Regular expression the node's source must match (re.search)."},
        },
        "required": ["path"],
    }
    output_schema = list_output_schema()
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_find(
                path=args.get("path"),

                id=args.get("id"),

                name=args.get("name"),
                node_type=args.get("node_type"),
                lineno=args.get("lineno"),
                end_lineno=args.get("end_lineno"),
                parent_type=args.get("parent_type"),
                text=args.get("text"),
                regexp=args.get("regexp"),
            )
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"nodes": [asdict(n) for n in result.nodes], "count": result.count})


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(FindNodesTool())
    functions.register(ast_find)
