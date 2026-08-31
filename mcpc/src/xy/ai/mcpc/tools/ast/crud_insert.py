"""``python_ast_insert`` tool: insert statement(s) relative to a selected node."""


from dataclasses import dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.crud_common import SELECTOR_PROPS, select_one
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["InsertNodeResult", "python_ast_insert", "InsertNodeTool", "register"]


@dataclass(frozen=True)
class InsertNodeResult:
    """Result of :func:`python_ast_insert`.

    Attributes:
        result: Always ``"success"``.
        inserted: Number of top-level statements parsed from ``code`` and inserted.
    """

    result: str
    inserted: int


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
    target = select_one(
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
            **SELECTOR_PROPS,
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


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(InsertNodeTool())
    functions.register(python_ast_insert)
