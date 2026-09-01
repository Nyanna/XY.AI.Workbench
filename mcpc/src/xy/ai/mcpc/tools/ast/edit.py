"""``ast_edit`` tool: mark-based edit within the source of a selected node."""


from dataclasses import dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS, select_one
from xy.ai.mcpc.tools.edit_marks import EditMarksError, edit_marks_text
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["EditNodeResult", "ast_edit", "EditNodeTool", "register"]


@dataclass(frozen=True)
class EditNodeResult:
    """Result of :func:`ast_edit`.

    Attributes:
        result: Always ``"success"``.
    """

    result: str


def ast_edit(
    path: str,
    block_start: str,
    block_end: str,
    content: str,
    *,
    exact: bool = False,
    id: str | None = None,
    qualified_name: str | None = None,
    name: str | None = None,
    node_type: str | None = None,
    lineno: int | None = None,
    end_lineno: int | None = None,
    parent_type: str | None = None,
) -> EditNodeResult:
    """Replace everything between the 'block_start' and 'block_end' markers inside a selected node's source.

    The selected node's source is unparsed, edited between the two markers (both
    included) as with ``edit_marks``, re-parsed, and used to replace the node.

    Args:
        path: Absolute path to the file to modify.
        block_start: Unique substring marking the beginning of the block, within the selected node's source.
        block_end: Unique substring marking the end of the block, within the selected node's source.
        content: Replacement source for the marked block.
        exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.
        qualified_name: Selector – exact FQN of the target node.
        name: Selector – exact simple name of the target node.
        node_type: Selector – AST node class name of the target node.
        lineno: Selector – exact start line of the target node.
        end_lineno: Selector – exact end line of the target node.
        parent_type: Selector – AST class name of the target node's container.

    Returns:
        EditNodeResult: Success status.

    Raises:
        core.AstError: If ``path`` is invalid, no selector is given, the selector
            matches zero or more than one node, the markers are not found or
            ambiguous within the node's source, or the edited source has a
            syntax error.
    """
    if not any((id, qualified_name, name, node_type, lineno, end_lineno, parent_type)):
        raise core.AstError("A node selector is required; ast_edit targets a node's content, not the whole file.")
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    target = select_one(
        tree,
        id=id,
        qualified_name=qualified_name,
        name=name,
        node_type=node_type,
        lineno=lineno,
        end_lineno=end_lineno,
        parent_type=parent_type,
    )
    node_source = core.edit_node_source(target)
    try:
        new_source = edit_marks_text(node_source, block_start, block_end, content, exact=exact)
    except EditMarksError as exc:
        raise core.AstError(str(exc)) from exc
    core.replace_node(target, new_source)
    core.CACHE.save(file_path, tree)
    return EditNodeResult(result="success")


class EditNodeTool(ToolDefinition):
    name = "ast_edit"
    title = "Edit AST node"
    description = (
        "Replace everything strictly between and including the unique 'block_start' and 'block_end' "
        "markers, found within the source of the selected node, with 'content'."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the file."},
            "block_start": {
                "type": "string",
                "description": "Unique substring marking the beginning of the block, within the selected node's source.",
            },
            "block_end": {
                "type": "string",
                "description": "Unique substring marking the end of the block, within the selected node's source.",
            },
            "content": {"type": "string", "description": "Replacement source for the marked block."},
            "exact": {
                "type": "boolean",
                "description": "If true, 'block_start'/'block_end' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                "default": False,
            },
            **SELECTOR_PROPS,
        },
        "required": ["path", "block_start", "block_end", "content"],
    }
    output_schema = {
        "type": "object",
        "properties": {"result": {"type": "string"}},
        "required": ["result"],
    }
    annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_edit`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_edit(
                args["path"],
                args["block_start"],
                args["block_end"],
                args["content"],
                exact=args.get("exact", False),
                id=args.get("id"),
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


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditNodeTool())
    functions.register(ast_edit)
