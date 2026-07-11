"""Change tool – replaces the block between start/end markers (both inclusive)."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult


def register_change_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "change",
        title="Change file block",
        description=(
            "Replace the text between 'start' and 'end' (both included) with "
            "'content'. Each marker must occur exactly once in the file; "
            "'end' must come after 'start'. Repeat a marker inside 'content' "
            "to keep it."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the target file.",
                },
                "start": {
                    "type": "string",
                    "description": "Unique substring marking the block's start (must occur exactly once).",
                },
                "end": {
                    "type": "string",
                    "description": "Unique substring marking the block's end (must occur exactly once, after 'start').",
                },
                "content": {
                    "type": "string",
                    "description": "Text that replaces the block, including where 'start'/'end' were.",
                },
            },
            "required": ["path", "start", "end", "content"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
                    "description": "``success`` on success.",
                },
                "error": {
                    "type": "string",
                    "description": "Human-readable error message (only present when is_error is true).",
                },
            },
            "required": [],
        },
        annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
    )
    def change(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]
        start_marker: str = args["start"]
        end_marker: str = args["end"]
        new_content: str = args["content"]

        # --- path validation ---
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

        text = path.read_text(encoding="utf-8")

        # --- locate and validate start marker ---
        start_count = text.count(start_marker)
        if start_count == 0:
            return ToolResult(
                structured_content={"error": f"Start marker not found in file: {start_marker!r}"},
                is_error=True,
            )
        if start_count > 1:
            return ToolResult(
                structured_content={
                    "error": (
                        f"Start marker is ambiguous – found {start_count} occurrences "
                        f"in file: {start_marker!r}"
                    )
                },
                is_error=True,
            )

        # --- locate and validate end marker ---
        end_count = text.count(end_marker)
        if end_count == 0:
            return ToolResult(
                structured_content={"error": f"End marker not found in file: {end_marker!r}"},
                is_error=True,
            )
        if end_count > 1:
            return ToolResult(
                structured_content={
                    "error": (
                        f"End marker is ambiguous – found {end_count} occurrences "
                        f"in file: {end_marker!r}"
                    )
                },
                is_error=True,
            )

        start_pos = text.index(start_marker)
        end_pos = text.index(end_marker)

        # --- order validation ---
        if end_pos <= start_pos:
            return ToolResult(
                structured_content={
                    "error": (
                        f"End marker must appear after start marker, but end marker "
                        f"starts at position {end_pos} while start marker starts at "
                        f"position {start_pos}."
                    )
                },
                is_error=True,
            )

        # --- apply replacement: both markers included (full range) ---
        result_text = text[:start_pos] + new_content + text[end_pos + len(end_marker):]

        # --- write back ---
        try:
            path.write_text(result_text, encoding="utf-8")
        except OSError as exc:
            return ToolResult(
                structured_content={"error": f"Write failed: {exc}"},
                is_error=True,
            )

        return ToolResult(structured_content={"result": "success"})
