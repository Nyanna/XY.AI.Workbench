"""Whole-file operations: ``python-ast-create-file`` and ``python-ast-delete-file``."""

from __future__ import annotations

from typing import Any

from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
from . import core


def _err(exc: core.AstError) -> ToolResult:
    return ToolResult(content=[text_content(str(exc))], is_error=True)


class CreateFileTool(ToolDefinition):
    name = "python-ast-create-file"
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
        args: dict[str, Any] = ctx.arguments
        try:
            path = core.require_path(args["path"], must_exist=False)
            if path.exists() and not args.get("overwrite", False):
                raise core.AstError("File already exists.")
            tree = core.parse_source(args["code"])
            path.parent.mkdir(parents=True, exist_ok=True)
            core.CACHE.save(path, tree)
        except core.AstError as exc:
            return _err(exc)
        return ToolResult(structured_content={"result": "success"}, auto_approve=True)


class DeleteFileTool(ToolDefinition):
    name = "python-ast-delete-file"
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
        try:
            path = core.require_path(ctx.arguments["path"])
            path.unlink()
            core.CACHE.invalidate(path)
        except core.AstError as exc:
            return _err(exc)
        except OSError:
            return ToolResult(content=[text_content("Delete failed.")], is_error=True)
        return ToolResult(structured_content={"result": "success"}, auto_approve=True)


def register(registry: ToolRegistry) -> None:
    registry.register(CreateFileTool())
    registry.register(DeleteFileTool())
