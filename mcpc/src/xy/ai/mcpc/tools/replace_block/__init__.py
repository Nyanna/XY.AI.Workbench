"""Replace-block tool – replaces an exact block of text (old -> new) in a file."""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from .._text_match import find as find_text


def register_replace_block_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "replace-block",
        title="Replace text block in file",
        description=(
            "Replace a complete block of text inside an existing file. "
            "'old_text' must occur exactly once. By default whitespace "
            "(spaces, tabs, newlines) is matched tolerantly; set 'exact' to "
            "require exact whitespace matching."
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
                    "description": "Text to find and replace. Must occur exactly once.",
                },
                "new_text": {
                    "type": "string",
                    "description": "Text that replaces 'old_text'.",
                },
                "exact": {
                    "type": "boolean",
                    "description": (
                        "If true, 'old_text' must match whitespace exactly. "
                        "If false (default), whitespace runs match any amount/kind of whitespace."
                    ),
                    "default": False,
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
        if old_text == "":
            return ToolResult(
                content=[text_content("'old_text' must not be empty.")],
                is_error=True,
            )

        text = path.read_text(encoding="utf-8")

        # --- locate and validate old_text ---
        match = find_text(text, old_text, exact=exact)
        if match.count == 0:
            return ToolResult(
                content=[text_content("Text not found in file.")],
                is_error=True,
            )
        if match.count > 1:
            return ToolResult(
                content=[text_content(f"Text is ambiguous – found {match.count} occurrences in file.")],
                is_error=True,
            )

        result_text = text[: match.start] + new_text + text[match.end :]

        # --- write back ---
        try:
            path.write_text(result_text, encoding="utf-8")
        except OSError as exc:
            return ToolResult(
                content=[text_content(f"Write failed: {exc}")],
                is_error=True,
            )

        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
