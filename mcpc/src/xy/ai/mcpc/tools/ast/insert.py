"""``ast_insert`` tool: insert statement(s) relative to a selected node."""


from dataclasses import dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["InsertNodeResult", "ast_insert", "InsertNodeTool", "register"]


@dataclass(frozen=True)
class InsertNodeResult:
    """Result of :func:`ast_insert`.

    Attributes:
        result: Always ``"success"``.
        inserted: Number of top-level statements parsed from ``code`` and inserted.
    """

    result: str
    inserted: int


def ast_insert(
    path: str,
    code: str,
    *,
    position: str = "after",
    id: str | None = None,
    qualified_name: str | None = None,
) -> InsertNodeResult:
    """Insert statement(s) parsed from ``code`` relative to a selected node.

    Args:
        path: Absolute path to the file to modify.
        code: Source of the statement(s) to insert.
        position: ``"before"`` or ``"after"`` the selected node. Defaults to ``"after"``.
        id: Node id (primarily name-based path).
        qualified_name: Exact qualified name of the target node.

    Returns:
        InsertNodeResult: Success status and the number of statements inserted.

    Raises:
        core.AstError: If ``path`` is invalid, ``code`` has a syntax error, neither
            ``id`` nor ``qualified_name`` is given, or the path matches zero or more
            than one node.
    """
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=id, qualified_name=qualified_name)
    inserted = core.insert_node(target, code, position)
    core.CACHE.save(file_path, tree)
    return InsertNodeResult(result="success", inserted=inserted)


class InsertNodeTool(ToolDefinition):
    name = "ast_insert"
    title = "Insert AST node"
    description = "Insert statement(s) parsed from code relative to a selected node ('before' or 'after')."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the file."},
            "code": {"type": "string", "description": "Source of the statement(s) to insert."},
            "position": {
                "type": "string",
                "enum": ["before", "after"],
                "description": "Placement relative to the selected node.",
                "default": "after",
            },
            **PATH_SELECTOR_PROPS,
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
        """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_insert(
                args["path"],
                args["code"],
                position=args.get("position", "after"),
                id=args.get("id"),
                qualified_name=args.get("qualified_name"),
            )
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result, "inserted": result.inserted}, auto_approve=True)


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(InsertNodeTool())
    functions.register(ast_insert)
