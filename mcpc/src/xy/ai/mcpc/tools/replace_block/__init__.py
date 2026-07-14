"""Replace-block tool – replaces an exact block of text (old -> new) in a file."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult


def register_replace_block_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "replace-block",
        title="Replace text block in file",
        description=(
            "Replace a complete block of text inside an existing file. "
            "'old_text' must occur exactly once in the file and is replaced "
            "in full by 'new_text'. Use this when you know the exact text to "
            "be replaced rather than an offset/length or start/end markers."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the target file.",
                },
                "old_text": {
                    "type": "string",
                    "description": (
                        "Exact text to find and replace. Must occur exactly once "
                        "in the file."
                    ),
                },
                "new_text": {
                    "type": "string",
                    "description": "Text that replaces 'old_text'.",
                },
            },
            "required": ["path", "old_text", "new_text"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
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
    def replace_block(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]
        old_text: str = args["old_text"]
        new_text: str = args["new_text"]

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
        if old_text == "":
            return ToolResult(
                structured_content={"error": "'old_text' must not be empty."},
                is_error=True,
            )

        text = path.read_text(encoding="utf-8")

        # --- locate and validate old_text ---
        occurrences = text.count(old_text)
        if occurrences == 0:
            return ToolResult(
                structured_content={"error": f"Text not found in file: {old_text!r}"},
                is_error=True,
            )
        if occurrences > 1:
            return ToolResult(
                structured_content={
                    "error": (
                        f"Text is ambiguous – found {occurrences} occurrences "
                        f"in file: {old_text!r}"
                    )
                },
                is_error=True,
            )

        result_text = text.replace(old_text, new_text, 1)

        # --- write back ---
        try:
            path.write_text(result_text, encoding="utf-8")
        except OSError as exc:
            return ToolResult(
                structured_content={"error": f"Write failed: {exc}"},
                is_error=True,
            )

        return ToolResult(structured_content={"result": "success"})
