"""List tool – returns a flat, sorted list of relative file paths below a directory.

Walks the given absolute directory recursively and returns all file paths
(files only, no directories) as an alphabetically sorted flat list of paths
relative to the requested directory. An optional regular expression can be
supplied to filter the resulting list (matched against each relative file
path). Common VCS/build/cache directories (e.g. ``.git``) are always excluded.
To keep results manageable, the number of returned entries is capped; use
``pattern`` to narrow down large directories instead of raising the limit.
"""

from __future__ import annotations

import os
import re
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult

_MAX_ENTRIES = 50

_EXCLUDED_DIRS = {
    ".git",
    ".hg",
    ".svn",
    "__pycache__",
    ".mypy_cache",
    ".pytest_cache",
    ".ruff_cache",
    ".tox",
    ".venv",
    "venv",
    "node_modules",
    ".idea",
    ".vscode",
    "dist",
    "build",
    ".cache",
}


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
                structured_content={"error": "Path must be absolute."},
                is_error=True,
            )
        if not path.exists():
            return ToolResult(
                structured_content={"error": "Directory not found."},
                is_error=True,
            )
        if not path.is_dir():
            return ToolResult(
                structured_content={"error": "Not a directory."},
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

        root = path.resolve()
        entries: list[str] = []
        for dirpath, dirnames, filenames in os.walk(root):
            dirnames[:] = [d for d in dirnames if d not in _EXCLUDED_DIRS]
            for filename in filenames:
                file_path = Path(dirpath) / filename
                rel_path = str(file_path.relative_to(root))
                if regex is not None and not regex.search(rel_path):
                    continue
                entries.append(rel_path)

        entries.sort()

        if len(entries) > _MAX_ENTRIES:
            return ToolResult(
                structured_content={
                    "error": (
                        f"Too many entries ({len(entries)}) exceed the limit of "
                        f"{_MAX_ENTRIES}. Narrow down the result using the "
                        "'pattern' regular expression parameter."
                    )
                },
                is_error=True,
            )

        return ToolResult(structured_content={"entries": entries})
