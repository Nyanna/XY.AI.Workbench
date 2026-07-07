"""Read tool – reads a file and returns its contents.

Features
--------
* Optional line-range restriction (``min_line`` / ``max_line``, 1-based inclusive).
* Content-hash caching per session: if the client requests the same file again
  and the file on disk has not changed, an error is returned to avoid redundant
  transfers.
* The cache is stored under the ``_read_cache`` key in the session's ``state``
  dict as ``{absolute_path: sha256_hex}``.
"""

from __future__ import annotations

import hashlib
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult

#: Key used inside ``Session.state`` to persist the per-session file cache.
_CACHE_KEY = "_read_cache"


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def register_read_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "read",
        title="Read file",
        description=(
            "Read the contents of a file and return them as text. "
            "Optionally restrict the result to a line range (1-based, inclusive). "
            "Results are cached per session by content hash; if the file has not "
            "changed since the last read an error is returned indicating so."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the file to read.",
                },
                "min_line": {
                    "type": "integer",
                    "description": "First line to return (1-based, inclusive). Omit to start from the beginning.",
                    "minimum": 1,
                },
                "max_line": {
                    "type": "integer",
                    "description": "Last line to return (1-based, inclusive). Omit to read to the end of the file.",
                    "minimum": 1,
                },
            },
            "required": ["path"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string"},
                "sha256": {"type": "string"},
                "total_lines": {"type": "integer"},
                "returned_lines": {"type": "integer"},
                "content": {"type": "string"},
                "min_line": {"type": "integer"},
                "max_line": {"type": "integer"},
            },
            "required": ["path", "sha256", "total_lines", "returned_lines", "content"],
        },
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def read(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]
        min_line: int | None = args.get("min_line")
        max_line: int | None = args.get("max_line")

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

        raw_bytes = path.read_bytes()
        current_hash = _sha256(raw_bytes)

        # --- session cache check ---
        cache: dict[str, str] = ctx.session.state.setdefault(_CACHE_KEY, {})
        key = str(path.resolve())
        if cache.get(key) == current_hash:
            return ToolResult(
                structured_content={
                    "error": f"File has not changed since the last read (sha256={current_hash}): {path_str}"
                },
                is_error=True,
            )
        cache[key] = current_hash

        # --- decode and slice ---
        text = raw_bytes.decode("utf-8", errors="replace")
        lines = text.splitlines(keepends=True)
        total_lines = len(lines)

        lo = (min_line - 1) if min_line is not None else 0
        hi = max_line if max_line is not None else total_lines
        lo = max(0, lo)
        hi = min(total_lines, hi)

        sliced = "".join(lines[lo:hi])

        structured: dict[str, Any] = {
            "path": key,
            "sha256": current_hash,
            "total_lines": total_lines,
            "returned_lines": hi - lo,
            "content": sliced,
        }
        if min_line is not None:
            structured["min_line"] = min_line
        if max_line is not None:
            structured["max_line"] = max_line

        return ToolResult(structured_content=structured)
