"""``python-ast-replace-block`` – text replace scoped to a single AST node.

Like the top-level ``replace-block`` tool but constrained to the line range of a
selected node (method/class/function), so ``old_text`` only has to be unique
within that node rather than the whole file. Shares the whitespace-tolerant
matcher with the file-level tool.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._text_match import find as find_text
from xy.ai.mcpc.tools.ast import core

__all__ = ["ReplaceBlockResult", "replace_block_in_node", "NodeReplaceBlockTool", "register"]

_SELECTOR_PROPS = {
    "qualified_name": {"type": "string", "description": "Python-style FQN of the enclosing node."},
    "name": {"type": "string", "description": "Simple node name."},
    "node_type": {"type": "string", "description": "AST node class name filter."},
    "lineno": {"type": "integer", "description": "Start line of the node."},
}


def _select(tree, **selectors: Any) -> core.Located:
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


@dataclass(frozen=True)
class ReplaceBlockResult:
    """Result of :func:`replace_block_in_node`.

    Attributes:
        result: Always ``"success"``.
    """

    result: str


def replace_block_in_node(
    path: str,
    old_text: str,
    new_text: str,
    *,
    exact: bool = False,
    qualified_name: str | None = None,
    name: str | None = None,
    node_type: str | None = None,
    lineno: int | None = None,
) -> ReplaceBlockResult:
    """Replace a text block inside the line range of a selected AST node.

    Args:
        path: Absolute path to the Python file.
        old_text: Text to find within the selected node; must occur exactly once
            there and must not be empty.
        new_text: Replacement text.
        exact: If ``True``, require exact whitespace matching; if ``False``
            (default), whitespace runs match any amount/kind of whitespace.
        qualified_name: Selector – exact Python-style FQN of the enclosing node.
        name: Selector – exact simple name of the enclosing node.
        node_type: Selector – AST node class name of the enclosing node.
        lineno: Selector – exact start line of the enclosing node.

    Returns:
        ReplaceBlockResult: Success status.

    Raises:
        core.AstError: If ``old_text`` is empty, ``path`` is invalid, the selector
            matches zero or more than one node, ``old_text`` occurs zero or more
            than once within the selected node's line range, or the file is not
            valid Python after the replacement.
    """
    if old_text == "":
        raise core.AstError("'old_text' must not be empty.")

    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    target = _select(tree, qualified_name=qualified_name, name=name, node_type=node_type, lineno=lineno)
    node = target.node

    text = file_path.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)
    start = node.lineno - 1
    end = getattr(node, "end_lineno", node.lineno)
    scope_start = len("".join(lines[:start]))
    scope_end = len("".join(lines[:end]))
    scope = text[scope_start:scope_end]

    match = find_text(scope, old_text, exact=exact)
    if match.count == 0:
        raise core.AstError("Text not found within node.")
    if match.count > 1:
        raise core.AstError(f"Text is ambiguous – {match.count} occurrences within node.")

    abs_start = scope_start + match.start
    abs_end = scope_start + match.end
    new_full = text[:abs_start] + new_text + text[abs_end:]

    # Validate the result before persisting; refresh cache from the file.
    core.parse_source(new_full)
    file_path.write_text(new_full, encoding="utf-8")
    core.CACHE.invalidate(file_path)
    return ReplaceBlockResult(result="success")


class NodeReplaceBlockTool(ToolDefinition):
    name = "python-ast-replace-block"
    title = "Replace block within node"
    description = (
        "Replace a text block inside the line range of a selected AST node. "
        "'old_text' must occur exactly once within that node; whitespace is "
        "matched tolerantly unless 'exact' is set."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "old_text": {"type": "string", "description": "Text to find within the node."},
            "new_text": {"type": "string", "description": "Replacement text."},
            "exact": {
                "type": "boolean",
                "description": "Require exact whitespace matching.",
                "default": False,
            },
            **_SELECTOR_PROPS,
        },
        "required": ["path", "old_text", "new_text"],
    }
    output_schema = {
        "type": "object",
        "properties": {"result": {"type": "string"}},
        "required": ["result"],
    }
    annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`replace_block_in_node`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = replace_block_in_node(
                args["path"],
                args["old_text"],
                args["new_text"],
                exact=args.get("exact", False),
                qualified_name=args.get("qualified_name"),
                name=args.get("name"),
                node_type=args.get("node_type"),
                lineno=args.get("lineno"),
            )
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result}, auto_approve=True)


def register(registry: ToolRegistry) -> None:
    registry.register(NodeReplaceBlockTool())
