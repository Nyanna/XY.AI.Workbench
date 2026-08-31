"""``python_ast_replace`` tool: replace the single selected node with new source."""


from dataclasses import dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.crud_common import SELECTOR_PROPS, select_one
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["ReplaceNodeResult", "python_ast_replace", "ReplaceNodeTool", "register"]


@dataclass(frozen=True)
class ReplaceNodeResult:
    """Result of :func:`python_ast_replace`.

    Attributes:
        result: Always ``"success"``.
    """

    result: str


def python_ast_replace(
    path: str,
    code: str,
    *,
    qualified_name: str | None = None,
    name: str | None = None,
    node_type: str | None = None,
    lineno: int | None = None,
    end_lineno: int | None = None,
    parent_type: str | None = None,
) -> ReplaceNodeResult:
    """Replace the single selected node with statement(s) parsed from ``code``.

    Args:
        path: Absolute path to the Python file to modify.
        code: Replacement Python source.
        qualified_name: Selector – exact Python-style FQN of the target node.
        name: Selector – exact simple name of the target node.
        node_type: Selector – AST node class name of the target node.
        lineno: Selector – exact start line of the target node.
        end_lineno: Selector – exact end line of the target node.
        parent_type: Selector – AST class name of the target node's container.

    Returns:
        ReplaceNodeResult: Success status.

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
    core.replace_in_body(target, new_nodes)
    core.CACHE.save(file_path, tree)
    return ReplaceNodeResult(result="success")


class ReplaceNodeTool(ToolDefinition):
    name = "python_ast_replace"
    title = "Replace AST node"
    description = "Replace the single selected node with statement(s) parsed from code."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Replacement Python source."},
            **SELECTOR_PROPS,
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
        """Delegate to :func:`python_ast_replace`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = python_ast_replace(
                args["path"],
                args["code"],
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
    registry.register(ReplaceNodeTool())
    functions.register(python_ast_replace)
