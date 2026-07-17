"""Replace-chars tool – replaces a character range inside an existing file."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content


def register_replace_chars_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "replace-chars",
        title="Replace characters in file",
        description=(
            "Replace a range of characters inside an existing file with new content. "
            "The range is defined by a zero-based character ``offset`` and a ``length`` "
            "(number of characters to remove starting at the offset). "
            "The supplied ``content`` is written in place of the removed range. "
            "To replace whole lines instead, use ``replace-lines``."
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
                    "description": "Zero-based character offset of the first character to replace.",
                    "minimum": 0,
                },
                "length": {
                    "type": "integer",
                    "description": "Number of characters to remove starting at ``offset``.",
                    "minimum": 0,
                },
                "content": {
                    "type": "string",
                    "description": "Replacement text (may be empty to perform a pure deletion).",
                },
            },
            "required": ["path", "offset", "length", "content"],
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
    def replace_chars(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]
        offset: int = args["offset"]
        length: int = args["length"]
        new_content: str = args["content"]

        path = Path(path_str)
        if not path.is_absolute():
            return ToolResult(
                content=[text_content("Path must be absolute.")],
                is_error=True,
            )
        if not path.exists():
            return ToolResult(
                content=[text_content("File not found.")],
                is_error=True,
            )
        if not path.is_file():
            return ToolResult(
                content=[text_content("Not a regular file.")],
                is_error=True,
            )

        try:
            text = path.read_text(encoding="utf-8")
            file_len = len(text)
            if offset > file_len:
                return ToolResult(
                    content=[text_content(
                        f"Offset {offset} is beyond end of file "
                        f"(file length: {file_len} characters)."
                    )],
                    is_error=True,
                )
            end = min(offset + length, file_len)
            result = text[:offset] + new_content + text[end:]
            path.write_text(result, encoding="utf-8")
        except OSError as exc:
            return ToolResult(
                content=[text_content(f"Replace failed: {exc}")],
                is_error=True,
            )

        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
