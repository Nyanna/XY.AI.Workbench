"""List tool – returns a flat, sorted list of absolute file paths below a directory.

Walks the given absolute directory recursively and returns all file paths
(files only, no directories) as an alphabetically sorted flat list of
absolute path strings. An optional regular expression can be supplied to
filter the resulting list (matched against each absolute file path).
"""

from __future__ import annotations

import re
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult


def register_list_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "list",
        title="List directory",
        description=(
            "List all files below an absolute directory path, recursively, "
            "as a flat list. "
            "Optionally filter the result with a regular expression."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute directory path.",
                },
                "pattern": {
                    "type": "string",
                    "description": (
                        "Optional regular expression used to filter the result."
                    ),
                },
            },
            "required": ["path"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "entries": {
                    "type": "array",
                    "items": {"type": "string"},
                },
            },
            "required": ["entries"],
        },
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def list_dir(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]
        pattern: str | None = args.get("pattern")

        path = Path(path_str)
        if not path.is_absolute():
            return ToolResult(
                structured_content={"error": f"Path must be absolute: {path_str}"},
                is_error=True,
            )
        if not path.exists():
            return ToolResult(
                structured_content={"error": f"Directory not found: {path_str}"},
                is_error=True,
            )
        if not path.is_dir():
            return ToolResult(
                structured_content={"error": f"Not a directory: {path_str}"},
                is_error=True,
            )

        regex: re.Pattern[str] | None = None
        if pattern is not None:
            try:
                regex = re.compile(pattern)
            except re.error as exc:
                return ToolResult(
                    structured_content={"error": f"Invalid regular expression: {exc}"},
                    is_error=True,
                )

        entries: list[str] = []
        for file_path in path.rglob("*"):
            if not file_path.is_file():
                continue
            abs_path = str(file_path.resolve())
            if regex is not None and not regex.search(abs_path):
                continue
            entries.append(abs_path)

        entries.sort()

        return ToolResult(structured_content={"entries": entries})
