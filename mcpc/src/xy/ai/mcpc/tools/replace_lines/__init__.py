"""Replace-lines tool – replaces a range of lines inside an existing file.

This is the line-oriented analogue of ``replace-chars``: the range is given as a
zero-based *line* offset and a *line* count instead of character offsets.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content


def register_replace_lines_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "replace-lines",
        title="Replace lines in file",
        description=(
            "Replace a range of lines inside an existing file with new content. "
            "The range is defined by a zero-based line ``offset`` and a ``length`` "
            "(number of lines to remove starting at the offset). "
            "The supplied ``content`` is written in place of the removed lines; "
            "it should include its own trailing newline if a line break is wanted. "
            "To replace an arbitrary character range instead, use ``replace-chars``."
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
                    "description": "Zero-based line offset of the first line to replace.",
                    "minimum": 0,
                },
                "length": {
                    "type": "integer",
                    "description": "Number of lines to remove starting at ``offset``.",
                    "minimum": 0,
                },
                "content": {
                    "type": "string",
                    "description": "Replacement text (may be empty to perform a pure deletion).",
                },
            },
            "required": ["path", "offset", "length", "content"],
        },
        annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
    )
    def replace_lines(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]
        offset: int = args["offset"]
        length: int = args["length"]
        new_content: str = args["content"]

        path = Path(path_str)
        if not path.is_absolute():
            return ToolResult(
                content=[text_content(f"Path must be absolute: {path_str}")],
                is_error=True,
            )
        if not path.exists():
            return ToolResult(
                content=[text_content(f"File not found: {path_str}")],
                is_error=True,
            )
        if not path.is_file():
            return ToolResult(
                content=[text_content(f"Not a regular file: {path_str}")],
                is_error=True,
            )

        try:
            text = path.read_text(encoding="utf-8")
            lines = text.splitlines(keepends=True)
            line_count = len(lines)
            if offset > line_count:
                return ToolResult(
                    content=[
                        text_content(
                            f"Offset {offset} is beyond end of file "
                            f"(file length: {line_count} lines)."
                        )
                    ],
                    is_error=True,
                )
            end = min(offset + length, line_count)
            result = "".join(lines[:offset]) + new_content + "".join(lines[end:])
            path.write_text(result, encoding="utf-8")
        except OSError as exc:
            return ToolResult(
                content=[text_content(f"Replace failed: {exc}")],
                is_error=True,
            )

        return ToolResult(content=[text_content(f"OK: {path_str}")])
