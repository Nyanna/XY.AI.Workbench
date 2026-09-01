"""``ast_create`` tool: creates a file with source content."""


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


def ast_create(path: str, source: str) -> CreateNodeResult:
    """Create a file from ``source``.

    Args:
        path: Absolute path to the file to replace or create.
        code: source to write.

    Returns:
        CreateNodeResult: Success status and the number of statements appended.

    Raises:
        core.AstError: If ``path`` is not absolute, or ``source`` has a syntax error.
    """
    file_path = core.require_path(path, must_exist=False)
    tree = core.CACHE.get_tree(file_path) if file_path.exists() else core.empty_tree(file_path)
    created = core.append_nodes(tree, source)
    core.CACHE.save(file_path, tree)
    return CreateNodeResult(result="success", created=created)


class CreateNodeTool(ToolDefinition):
    name = "ast_create"
    title = "Create a file"
    description = "Create a file with source."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path to the file."},
            "source": {"type": "string", "description": "source of the statement(s) to append."},
        },
        "required": ["path", "source"],
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
