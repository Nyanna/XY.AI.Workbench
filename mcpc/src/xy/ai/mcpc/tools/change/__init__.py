"""Change tool – replaces the block between start/end markers (both inclusive)."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from .._text_match import find as find_text


def register_change_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "change",
        title="Change file block",
        description=(
            "Replace the text between 'start' and 'end' (both included) with "
            "'content'. Each marker must occur exactly once in the file; "
            "'end' must come after 'start'. Repeat a marker inside 'content' "
            "to keep it. By default whitespace in 'start'/'end' is matched "
            "tolerantly; set 'exact' to require exact whitespace matching."
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
                "exact": {
                    "type": "boolean",
                    "description": (
                        "If true, 'start'/'end' must match whitespace exactly. "
                        "If false (default), whitespace runs match any amount/kind of whitespace."
                    ),
                    "default": False,
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
        exact: bool = args.get("exact", False)

        # --- path validation ---
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

        text = path.read_text(encoding="utf-8")

        # --- locate and validate start marker ---
        start_match = find_text(text, start_marker, exact=exact)
        if start_match.count == 0:
            return ToolResult(
                content=[text_content("Start marker not found in file.")],
                is_error=True,
            )
        if start_match.count > 1:
            return ToolResult(
                content=[text_content(f"Start marker is ambiguous – found {start_match.count} occurrences in file.")],
                is_error=True,
            )

        # --- locate and validate end marker ---
        end_match = find_text(text, end_marker, exact=exact)
        if end_match.count == 0:
            return ToolResult(
                content=[text_content("End marker not found in file.")],
                is_error=True,
            )
        if end_match.count > 1:
            return ToolResult(
                content=[text_content(f"End marker is ambiguous – found {end_match.count} occurrences in file.")],
                is_error=True,
            )

        # --- order validation ---
        if end_match.start <= start_match.start:
            return ToolResult(
                content=[text_content("End marker must appear after start marker.")],
                is_error=True,
            )

        # --- apply replacement: both markers included (full range) ---
        result_text = text[: start_match.start] + new_content + text[end_match.end :]

        # --- write back ---
        try:
            path.write_text(result_text, encoding="utf-8")
        except OSError as exc:
            return ToolResult(
                content=[text_content(f"Write failed: {exc}")],
                is_error=True,
            )

        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
