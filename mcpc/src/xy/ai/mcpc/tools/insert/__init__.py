"""Insert tool – inserts text at a character offset inside an existing file."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult


def register_insert_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "insert",
        title="Insert into file",
        description=(
            "Insert text at a specific character offset inside an existing file. "
            "The offset is zero-based and refers to the UTF-8 decoded content of the file. "
            "All existing content at and after the offset is shifted right."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the file to modify.",
                },
                "offset": {
                    "type": "integer",
                    "description": "Zero-based character offset at which to insert the new content.",
                    "minimum": 0,
                },
                "content": {
                    "type": "string",
                    "description": "Text to insert at the given offset.",
                },
            },
            "required": ["path", "offset", "content"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "result": {"type": "string"},
                "description": "``success`` on success.",
            },
            "required": ["path"],
        },
        annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
    )
    def insert(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]
        offset: int = args["offset"]
        new_content: str = args["content"]

        path = Path(path_str)
        if not path.is_absolute():
            return ToolResult(
                structured_content={"error": f"Path must be absolute: {path_str}"},
                is_error=True,
            )
        if not path.exists():
            return ToolResult(
                structured_content={"error": f"File not found: {path_str}"},
                is_error=True,
            )
        if not path.is_file():
            return ToolResult(
                structured_content={"error": f"Not a regular file: {path_str}"},
                is_error=True,
            )

        try:
            text = path.read_text(encoding="utf-8")
            if offset > len(text):
                return ToolResult(
                    structured_content={
                        "error": (
                            f"Offset {offset} is beyond end of file "
                            f"(file length: {len(text)} characters)."
                        )
                    },
                    is_error=True,
                )
            result = text[:offset] + new_content + text[offset:]
            path.write_text(result, encoding="utf-8")
        except OSError as exc:
            return ToolResult(
                structured_content={"error": f"Insert failed: {exc}"},
                is_error=True,
            )

        return ToolResult(structured_content={"result": "success"})
