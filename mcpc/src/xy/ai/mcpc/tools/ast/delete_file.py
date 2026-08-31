"""Whole-file operation: ``ast_delete_file``."""


from dataclasses import dataclass

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = [
    "AstFileResult",
    "ast_delete_file",
    "DeleteFileTool",
    "register",
]


@dataclass(frozen=True)
class AstFileResult:
    """Result of :func:`ast_delete_file`.

    Attributes:
        result: Always ``"success"``.
    """

    result: str


def ast_delete_file(path: str) -> AstFileResult:
    """Delete the Python file at ``path`` and drop it from the AST cache.

    Args:
        path: Absolute path of the file to delete.

    Returns:
        AstFileResult: Success status.

    Raises:
        core.AstError: If ``path`` is not absolute, does not point to an existing
            regular file, or the deletion fails at the OS level.
    """
    file_path = core.require_path(path)
    try:
        file_path.unlink()
    except OSError as exc:
        raise core.AstError("Delete failed.") from exc
    core.CACHE.invalidate(file_path)
    return AstFileResult(result="success")


class DeleteFileTool(ToolDefinition):
    name = "ast_delete_file"
    title = "Delete Python file"
    description = "Delete a Python file and drop it from the AST cache."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path of the file to delete."}
        },
        "required": ["path"],
    }
    output_schema = {
        "type": "object",
        "properties": {"result": {"type": "string"}},
        "required": ["result"],
    }
    annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_delete_file`, translating the MCP schema to/from the Python API."""
        try:
            result = ast_delete_file(ctx.arguments["path"])
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result}, auto_approve=True)


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(DeleteFileTool())
    functions.register(ast_delete_file)
