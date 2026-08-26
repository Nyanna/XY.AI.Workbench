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
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

__all__ = ["ListError", "ListResult", "list_files", "ListTool", "register_list_tool"]

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


class ListError(Exception):
    """Raised when a directory listing cannot be performed."""


@dataclass(frozen=True)
class ListResult:
    entries: list[str]


def list_files(path: str, pattern: str | None = None) -> ListResult:
    """List all files below the absolute directory ``path``, optionally filtered by ``pattern``."""
    dir_path = Path(path)
    if not dir_path.is_absolute():
        raise ListError("Path must be absolute.")
    if not dir_path.exists():
        raise ListError("Directory not found.")
    if not dir_path.is_dir():
        raise ListError("Not a directory.")

    regex: re.Pattern[str] | None = None
    if pattern is not None:
        try:
            regex = re.compile(pattern)
        except re.error as exc:
            raise ListError(f"Invalid regular expression: {exc}") from exc

    root = dir_path.resolve()
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
        raise ListError(
            f"Too many entries ({len(entries)}) exceed the limit of "
            f"{_MAX_ENTRIES}. Narrow down the result using the "
            "'pattern' regular expression parameter."
        )

    return ListResult(entries=entries)


class ListTool(ToolDefinition):
    name = "list"
    title = "List directory"
    description = (
        "List all files below an absolute directory path, recursively, "
        "as a flat list. "
        "Optionally filter the result with a regular expression."
    )
    input_schema = {
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
    }
    output_schema = {
        "type": "object",
        "properties": {
            "entries": {
                "type": "array",
                "items": {"type": "string"},
            },
        },
        "required": ["entries"],
    }
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`list_files`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = list_files(path=args["path"], pattern=args.get("pattern"))
        except ListError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)

        return ToolResult(structured_content={"entries": result.entries})


def register_list_tool(registry: ToolRegistry) -> None:
    registry.register(ListTool())
