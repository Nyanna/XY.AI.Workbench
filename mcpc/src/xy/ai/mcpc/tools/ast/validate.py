"""``python-ast-validate`` – compile a list of Python files and report results."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content


def _check(path_str: str) -> dict[str, Any]:
    path = Path(path_str)
    if not path.is_absolute():
        return {"path": path_str, "ok": False, "error": "Path must be absolute."}
    try:
        source = path.read_text(encoding="utf-8")
    except OSError:
        return {"path": path_str, "ok": False, "error": "File not readable."}
    try:
        compile(source, str(path), "exec")
    except SyntaxError as exc:
        return {"path": path_str, "ok": False, "error": f"{exc.msg} (line {exc.lineno})"}
    return {"path": path_str, "ok": True, "error": None}


class ValidateTool(ToolDefinition):
    name = "python-ast-validate"
    title = "Validate Python files"
    description = "Check that each of a list of Python files compiles; report success/error per file."
    input_schema = {
        "type": "object",
        "properties": {
            "paths": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Absolute paths of Python files to validate.",
            }
        },
        "required": ["paths"],
    }
    output_schema = {
        "type": "object",
        "properties": {
            "all_ok": {"type": "boolean"},
            "files": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "path": {"type": "string"},
                        "ok": {"type": "boolean"},
                        "error": {"type": ["string", "null"]},
                    },
                    "required": ["path", "ok", "error"],
                },
            },
        },
        "required": ["all_ok", "files"],
    }
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        paths = ctx.arguments["paths"]
        if not isinstance(paths, list) or not paths:
            return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
        files = [_check(p) for p in paths]
        allOk = all(f["ok"] for f in files)
        return ToolResult(
            structured_content={"all_ok": allOk, "files": files},
            auto_approve=allOk,
        )


def register(registry: ToolRegistry) -> None:
    registry.register(ValidateTool())
