"""``ast_create`` tool: append statement(s) to a Python file's top level."""


from dataclasses import dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = ["CreateNodeResult", "ast_create", "CreateNodeTool", "register"]


@dataclass(frozen=True)
class CreateNodeResult:
    """Result of :func:`ast_create`.

    Attributes:
        result: Always ``"success"``.
        created: Number of top-level statements parsed from ``code`` and appended.
    """

    result: str
    created: int


def ast_create(path: str, code: str) -> CreateNodeResult:
    """Append statement(s) parsed from ``code`` to a Python file's top level.

    Args:
        path: Absolute path to the Python file to modify or create (its parent
            directory must already exist).
        code: Python source of the statement(s) to append.

    Returns:
        CreateNodeResult: Success status and the number of statements appended.

    Raises:
        core.AstError: If ``path`` is not absolute, or ``code`` has a syntax error.
    """
    file_path = core.require_path(path, must_exist=False)
    new_nodes = core.parse_snippet(code)
    tree = core.CACHE.get_tree(file_path) if file_path.exists() else ast.Module(body=[], type_ignores=[])
    tree.body.extend(new_nodes)
    core.CACHE.save(file_path, tree)
    return CreateNodeResult(result="success", created=len(new_nodes))


class CreateNodeTool(ToolDefinition):
    name = "ast_create"
    title = "Create AST node"
    description = "Append statement(s) parsed from code to a Python file's top level (creating the file if needed)."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the Python file."},
            "code": {"type": "string", "description": "Python source of the statement(s) to append."},
        },
        "required": ["path", "code"],
    }
    output_schema = {
        "type": "object",
        "properties": {"result": {"type": "string"}, "created": {"type": "integer"}},
        "required": ["result", "created"],
    }
    annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_create`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_create(args["path"], args["code"])
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result, "created": result.created}, auto_approve=True)


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(CreateNodeTool())
    functions.register(ast_create)
