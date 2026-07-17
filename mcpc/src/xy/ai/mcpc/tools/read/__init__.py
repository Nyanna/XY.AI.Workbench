"""Read tool – returns file contents, optionally sliced by line or unique marker.

Range: start = min_line | start-marker | file start; end = max_line | end-marker
| file end (all inclusive). Markers must be unique substrings.

Per-session cache (key ``_read_cache`` in ``Session.state``, keyed by the call
arguments plus the session id): the sha256 checksum of every read is recorded.
If a subsequent read with identical parameters yields the same checksum,
``content`` is omitted from ``structured_content`` and replaced by an
explanatory text content block; only the metrics (and the checksum itself)
are still returned. ``structured_content`` always carries the ``checksum``.
"""

from __future__ import annotations

import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content

#: Key under which the per-session read cache is kept in ``Session.state``.
_CACHE_STATE_KEY = "_read_cache"


def _cache_key(session_id: str, arguments: dict[str, Any]) -> str:
    """Derive a stable cache key from the session id and the call arguments."""
    payload = json.dumps({"session": session_id, "arguments": arguments}, sort_keys=True)
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()

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
                "path": {
                    "type": "string",
                    "description": "Absolute file path (only set for unrestricted reads).",
                },
                "modified": {
                    "type": "string",
                    "description": (
                        "Last modification timestamp in ISO 8601 format "
                        "(only set for unrestricted reads)."
                    ),
                },
                "size": {
                    "type": "integer",
                    "description": "File size in bytes (only set for unrestricted reads).",
                },
                "lines": {
                    "type": "integer",
                    "description": "Total number of lines (only set for unrestricted reads).",
                },
                "checksum": {
                    "type": "string",
                    "description": (
                        "sha256 checksum of the read content."
                    ),
                },
                "unchanged": {
                    "type": "boolean",
                    "description": (
                        "True if the content is identical to a previous read with the "
                        "same parameters"
                    ),
                },
            },
            "required": ["checksum"],
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
                content=[text_content("``min_line`` and ``start`` are mutually exclusive.")],
                is_error=True,
            )
        if max_line is not None and end_marker is not None:
            return ToolResult(
                content=[text_content("``max_line`` and ``end`` are mutually exclusive.")],
                is_error=True,
            )

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
                    content=[text_content("Start marker not found in file.")],
                    is_error=True,
                )
            if start_count > 1:
                return ToolResult(
                    content=[text_content(f"Start marker is ambiguous – found {start_count} occurrences in file.")],
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
                    content=[text_content("End marker not found in file.")],
                    is_error=True,
                )
            if end_count > 1:
                return ToolResult(
                    content=[text_content(f"End marker is ambiguous – found {end_count} occurrences in file.")],
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
                content=[text_content(
                    "Resolved end position must not lie before "
                    "the resolved start position."
                )],
                is_error=True,
            )

        sliced = text[region_start:region_end]
        checksum = hashlib.sha256(sliced.encode("utf-8")).hexdigest()

        # --- per-session cache lookup ---
        session = ctx.session
        key = _cache_key(session.id, args)
        with session.lock:
            cache: dict[str, str] = session.state.setdefault(_CACHE_STATE_KEY, {})
            previous_checksum = cache.get(key)
            cache[key] = checksum

        unchanged = previous_checksum == checksum

        structured: dict[str, Any] = {"checksum": checksum}
        if unchanged:
            structured["unchanged"] = True
        else:
            structured["content"] = sliced

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

        if is_full_file:
            stat = path.stat()
            structured["path"] = str(path.resolve())
            structured["modified"] = datetime.fromtimestamp(
                stat.st_mtime, tz=timezone.utc
            ).isoformat()
            structured["size"] = stat.st_size
            structured["lines"] = total_lines

        content: list[dict[str, Any]] = []
        if unchanged:
            content.append(
                text_content(
                    "Content unchanged since the last identical read. Use the former read result."
                )
            )

        return ToolResult(
            content=content,
            structured_content=structured,
            auto_approve=is_full_file,
        )
