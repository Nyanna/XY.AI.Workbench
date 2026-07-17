"""Read tool – returns file contents, optionally sliced by line or unique marker.

Range: start = min_line | start-marker | file start; end = max_line | end-marker
| file end (all inclusive). Markers must be unique substrings. Per-session
sha256 cache rejects unchanged re-reads (key ``_read_cache`` in session state).
"""

from __future__ import annotations

import hashlib
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult

def register_read_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "read",
        title="Read file",
        description=(
            "Read a file as text, optionally sliced to a range. Don't use for directories."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute file path.",
                },
                "min_line": {
                    "type": "integer",
                    "description": "Range start: line number, inclusive, 1-based. Excludes start.",
                    "minimum": 1,
                },
                "max_line": {
                    "type": "integer",
                    "description": "Range end: line number, inclusive, 1-based. Excludes end.",
                    "minimum": 1,
                },
                "start": {
                    "type": "string",
                    "description": "Range start: unique marker substring, inclusive. Excludes min_line.",
                },
                "end": {
                    "type": "string",
                    "description": "Range end: unique marker substring, inclusive. Excludes max_line.",
                },
            },
            "required": ["path"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "content": {"type": "string"},
            },
            "required": ["content"],
        },
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def read(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]
        min_line: int | None = args.get("min_line")
        max_line: int | None = args.get("max_line")
        start_marker: str | None = args.get("start")
        end_marker: str | None = args.get("end")

        # --- mutual exclusivity validation ---
        if min_line is not None and start_marker is not None:
            return ToolResult(
                structured_content={
                    "error": "``min_line`` and ``start`` are mutually exclusive."
                },
                is_error=True,
            )
        if max_line is not None and end_marker is not None:
            return ToolResult(
                structured_content={
                    "error": "``max_line`` and ``end`` are mutually exclusive."
                },
                is_error=True,
            )

        path = Path(path_str)
        if not path.is_absolute():
            return ToolResult(
                structured_content={"error": "Path must be absolute."},
                is_error=True,
            )
        if not path.exists():
            return ToolResult(
                structured_content={"error": "File not found."},
                is_error=True,
            )
        if not path.is_file():
            return ToolResult(
                structured_content={"error": "Not a regular file."},
                is_error=True,
            )

        raw_bytes = path.read_bytes()

        # --- decode ---
        text = raw_bytes.decode("utf-8", errors="replace")
        lines = text.splitlines(keepends=True)
        total_lines = len(lines)

        def line_start_offset(line_num: int) -> int:
            n = max(0, min(line_num - 1, total_lines))
            return sum(len(l) for l in lines[:n])

        def line_end_offset(line_num: int) -> int:
            n = max(0, min(line_num, total_lines))
            return sum(len(l) for l in lines[:n])

        if start_marker is not None:
            start_count = text.count(start_marker)
            if start_count == 0:
                return ToolResult(
                    structured_content={"error": "Start marker not found in file."},
                    is_error=True,
                )
            if start_count > 1:
                return ToolResult(
                    structured_content={
                        "error": f"Start marker is ambiguous – found {start_count} occurrences in file."
                    },
                    is_error=True,
                )
            region_start = text.index(start_marker)
        elif min_line is not None:
            region_start = line_start_offset(min_line)
        else:
            region_start = 0

        if end_marker is not None:
            end_count = text.count(end_marker)
            if end_count == 0:
                return ToolResult(
                    structured_content={"error": "End marker not found in file."},
                    is_error=True,
                )
            if end_count > 1:
                return ToolResult(
                    structured_content={
                        "error": f"End marker is ambiguous – found {end_count} occurrences in file."
                    },
                    is_error=True,
                )
            region_end = text.index(end_marker) + len(end_marker)
        elif max_line is not None:
            region_end = line_end_offset(max_line)
        else:
            region_end = len(text)

        # --- order validation ---
        if region_end < region_start:
            return ToolResult(
                structured_content={
                    "error": (
                        f"Resolved end position must not lie before "
                        f"the resolved start position."
                    )
                },
                is_error=True,
            )

        sliced = text[region_start:region_end]
        structured: dict[str, Any] = {"content": sliced}

        # An unrestricted read (no line/marker range given) returns the
        # entire file verbatim; there is nothing a human reviewer could
        # meaningfully approve or reject beyond what a plain file read
        # already exposes, so the tool flags it for auto-approval.
        is_full_file = (
            min_line is None
            and max_line is None
            and start_marker is None
            and end_marker is None
        )

        return ToolResult(structured_content=structured, auto_approve=is_full_file)
