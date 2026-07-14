"""Write tool – writes a file completely or appends lines to it."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult


def register_write_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "write",
        title="Write file",
        description=(
            "Write content to a file. "
            "In ``replace`` mode the file is overwritten with the supplied content. "
            "In ``append`` mode the content is added at the end of the existing file "
            "(the file is created if it does not yet exist)."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the file to write.",
                },
                "mode": {
                    "type": "string",
                    "enum": ["replace", "append"],
                    "description": (
                        "``replace`` – overwrite the file with the new content. "
                        "``append`` – add the new content after the existing content."
                    ),
                },
                "content": {
                    "type": "string",
                    "description": "Text to write to the file.",
                },
            },
            "required": ["path", "mode", "content"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
                    "description": "``success`` on success.",
                },
            },
            "required": ["result"],
        },
        annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
    )
    def write(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]
        mode: str = args["mode"]
        content: str = args["content"]

        path = Path(path_str)
        if not path.is_absolute():
            return ToolResult(
                structured_content={"error": "Path must be absolute."},
                is_error=True,
            )

        try:
            path.parent.mkdir(parents=True, exist_ok=True)
            file_mode = "a" if mode == "append" else "w"
            with path.open(file_mode, encoding="utf-8") as fh:
                fh.write(content)
        except OSError as exc:
            return ToolResult(
                structured_content={"error": f"Write failed: {exc}"},
                is_error=True,
            )

        return ToolResult(structured_content={"result": "success"})
