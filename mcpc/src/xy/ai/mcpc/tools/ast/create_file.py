"""Whole-file operation: ``ast_create_file``."""


from dataclasses import dataclass
from typing import Any

from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry

__all__ = [
    "AstFileResult",
    "ast_create_file",
    "CreateFileTool",
    "register",
]


@dataclass(frozen=True)
class AstFileResult:
    """Result of :func:`ast_create_file`.

    Attributes:
        result: Always ``"success"``.
    """

    result: str


def ast_create_file(path: str, code: str, overwrite: bool = False) -> AstFileResult:
    """Create a new Python file at ``path`` from ``code`` (validated by parsing it).

    Args:
        path: Absolute path of the file to create.
        code: Python source for the new file.
        overwrite: Allow replacing an existing file. Defaults to ``False``.

    Returns:
        AstFileResult: Success status.

    Raises:
        core.AstError: If ``path`` is not absolute, if the file already exists and
            ``overwrite`` is ``False``, or if ``code`` has a syntax error.
    """
    file_path = core.require_path(path, must_exist=False)
    if file_path.exists() and not overwrite:
        raise core.AstError("File already exists.")
    tree = core.parse_source(code)
    file_path.parent.mkdir(parents=True, exist_ok=True)
    core.CACHE.save(file_path, tree)
    return AstFileResult(result="success")


class CreateFileTool(ToolDefinition):
    name = "ast_create_file"
    title = "Create Python file"
    description = "Create a new Python file from source text (validated by parsing it through the AST)."
    input_schema = {
        "type": "object",
        "properties": {
            "path": {"type": "string", "description": "Absolute path of the file to create."},
            "code": {"type": "string", "description": "Python source for the new file."},
            "overwrite": {
                "type": "boolean",
                "description": "Allow replacing an existing file.",
                "default": False,
            },
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
        """Delegate to :func:`ast_create_file`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_create_file(
                path=args["path"], code=args["code"], overwrite=args.get("overwrite", False)
            )
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={"result": result.result}, auto_approve=True)


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(CreateFileTool())
    functions.register(ast_create_file)
