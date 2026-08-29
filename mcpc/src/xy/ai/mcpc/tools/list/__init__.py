"""List tool – returns files below a directory, grouped by relative subdirectory like ``ls -R``.

Walks the given absolute directory recursively and returns all file paths
(files only, no directories), grouped by the relative directory they live in
(e.g. ``./src/pkg:`` followed by tab-indented file names), mirroring the
output format of ``ls -R``. An optional regular expression can be supplied to
filter the resulting files (matched against each file's path relative to the
requested directory). Common VCS/build/cache directories (e.g. ``.git``) are
always excluded. To keep results manageable, the number of matched files is
capped; use ``pattern`` to narrow down large directories instead of raising
the limit.
"""
import os
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ListError', 'ListResult', 'list', 'ListTool', 'register_list_tool']
_MAX_ENTRIES = 50
_EXCLUDED_DIRS = {'.git', '.hg', '.svn', '__pycache__', '.mypy_cache', '.pytest_cache', '.ruff_cache', '.tox', '.venv', 'venv', 'node_modules', '.idea', '.vscode', 'dist', 'build', '.cache'}

class ListError(Exception):
    """Raised when a directory listing cannot be performed."""

@dataclass(frozen=True)
class ListResult:
    entries: list[str]

def list(path: str, pattern: str | None=None) -> ListResult:
    """List all files below the absolute directory ``path``, grouped like ``ls -R``.

    Args:
        path: Absolute directory path to list (must exist and be a directory).
        pattern: Optional regular expression to filter results. Only matching file paths are included.

    Returns:
        ListResult with:
            entries: Lines of output, one directory header (e.g. ``./sub:``)
                followed by its tab-indented file names, then a blank line
                before the next directory group. Directories without
                matching files are omitted.

    Raises:
        ListError: If path is not absolute.
        ListError: If path does not exist or is not a directory.
        ListError: If pattern is not a valid regular expression.
    """
    dir_path = Path(path)
    if not dir_path.is_absolute():
        raise ListError('Path must be absolute.')
    if not dir_path.is_dir():
        raise ListError('Directory not found or not a directory.')
    try:
        regex = re.compile(pattern) if pattern else None
    except re.error as exc:
        raise ListError(f'Invalid regex pattern: {exc}') from exc
    groups: dict[str, list[str]] = {}
    match_count = 0
    for root, dirs, files in os.walk(str(dir_path)):
        rel_dir = os.path.relpath(root, str(dir_path))
        matched_files = []
        for file in sorted(files):
            rel_path = os.path.normpath(os.path.join(rel_dir, file))
            if regex is None or regex.search(rel_path):
                matched_files.append(file)
        if matched_files:
            groups[rel_dir] = matched_files
            match_count += len(matched_files)
    if match_count > _MAX_ENTRIES:
        raise ListError(
            f"Too many entries ({match_count}) exceed the limit of "
            f"{_MAX_ENTRIES}. Narrow down the result using the "
            "'pattern' regular expression parameter."
        )
    entries = []
    for rel_dir in sorted(groups):
        header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
        if entries:
            entries.append('')
        entries.append(f'{header}:')
        entries.extend(f' {name}' for name in groups[rel_dir])
    return ListResult(entries=entries)

class ListTool(ToolDefinition):
    name = 'list'
    title = 'List directory'
    description = 'List all files below an absolute directory path, recursively, as a flat list. Optionally filter the result with a regular expression.'
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute directory path.'}, 'pattern': {'type': 'string', 'description': 'Optional regular expression used to filter the result.'}}, 'required': ['path']}
    output_schema = {'type': 'object', 'properties': {'entries': {'type': 'array', 'items': {'type': 'string'}}}, 'required': ['entries']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = list(path=args['path'], pattern=args.get('pattern'))
        except ListError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'entries': result.entries})

def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ListTool())
    functions.register(list)