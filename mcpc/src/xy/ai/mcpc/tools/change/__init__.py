"""Change tool – replaces a block inside a file delimited by start/end markers.

Modes
-----
``include_start``
    The replaced region starts **at** the start marker (inclusive) and ends
    **before** the end marker (exclusive).  The end marker is preserved in the
    output.

``include_end``
    The replaced region starts **after** the start marker (exclusive) and ends
    **at** the end of the end marker (inclusive).  The start marker is
    preserved in the output.

``full``
    Both markers are included in the replaced region (both inclusive).

Error conditions
----------------
* Path is not absolute, does not exist or is not a regular file.
* A marker string does not occur in the file at all.
* A marker string occurs more than once (ambiguous); the exact count is
  reported so the caller can supply a more specific marker.
* The first occurrence of the end marker lies before (or at the same position
  as) the first occurrence of the start marker.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult


def register_change_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "change",
        title="Change file block",
        description=(
            "Replace a block of text inside a file that is delimited by a "
            "start string and an end string.  The ``mode`` parameter controls "
            "whether each marker is itself included in or excluded from the "
            "replaced region:\n"
            "* ``include_start`` – replace from the start marker (inclusive) "
            "to just before the end marker (exclusive); the end marker is kept.\n"
            "* ``include_end`` – replace from just after the start marker "
            "(exclusive) to the end of the end marker (inclusive); the start "
            "marker is kept.\n"
            "* ``full`` – replace the entire range including both markers.\n\n"
            "Both markers must appear exactly once in the file and the end "
            "marker must follow the start marker.  A precise error is returned "
            "otherwise."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the target file.",
                },
                "mode": {
                    "type": "string",
                    "enum": ["include_start", "include_end", "full"],
                    "description": (
                        "``include_start`` – start marker inclusive, end marker exclusive. "
                        "``include_end`` – start marker exclusive, end marker inclusive. "
                        "``full`` – both markers inclusive."
                    ),
                },
                "start": {
                    "type": "string",
                    "description": "Exact string that marks the beginning of the block.",
                },
                "end": {
                    "type": "string",
                    "description": "Exact string that marks the end of the block.",
                },
                "content": {
                    "type": "string",
                    "description": "Replacement text that will be written in place of the matched block.",
                },
            },
            "required": ["path", "mode", "start", "end", "content"],
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
        mode: str = args["mode"]
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

        # --- apply replacement based on mode ---
        if mode == "include_start":
            # replace from start_pos (inclusive) to end_pos (exclusive)
            result_text = text[:start_pos] + new_content + text[end_pos:]
        elif mode == "include_end":
            # replace from start_pos + len(start) (exclusive) to end_pos + len(end) (inclusive)
            result_text = text[:start_pos + len(start_marker)] + new_content + text[end_pos + len(end_marker):]
        else:  # full
            # replace from start_pos (inclusive) to end_pos + len(end) (inclusive)
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
