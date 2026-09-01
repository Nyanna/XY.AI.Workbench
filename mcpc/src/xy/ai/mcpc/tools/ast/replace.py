"""``ast_replace`` tool: replace the single selected node with new source."""


from dataclasses import dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["ReplaceNodeResult", "ast_replace", "ReplaceNodeTool", "register"]


@dataclass(frozen=True)
class ReplaceNodeResult:
    """Result of :func:`ast_replace`.

    Attributes:
        result: Always ``"success"``.
    """

    result: str


def ast_replace(
    path: str,
    code: str,
    *,
    id: str | None = None,
) -> ReplaceNodeResult:
    """Replace the single selected node with ``code``.

    Args:
        path: Absolute path to the file to modify.
        code: Replacement source.
        id: Unique id of the target node.

    Returns:
        ReplaceNodeResult: Success status.

    Raises:
        core.AstError: If ``path`` is invalid, ``code`` has a syntax error, ``id`` is
            not given, or it matches zero or more than one node.
    """
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=id)
    core.replace_node(target, code)
    core.CACHE.save(file_path, tree)
    return ReplaceNodeResult(result="success")


class ReplaceNodeTool(ToolDefinition):
    name = "ast_replace"
    title = "Replace AST node"
    description = "Replace the single selected node with statement(s) parsed from code."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the file."},
            "code": {"type": "string", "description": "Replacement source."},
            **PATH_SELECTOR_PROPS,
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
        """Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_replace(
                args["path"],
                args["code"],
                id=args.get("id"),
            )
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result}, auto_approve=True)


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReplaceNodeTool())
    functions.register(ast_replace)
