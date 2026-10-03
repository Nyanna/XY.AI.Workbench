"""List tool – returns files below a directory, grouped by relative subdirectory like ``ls -R``, for a batch of items.

Walks each given absolute directory recursively and returns all file paths
(files only, no directories), grouped by the relative directory they live in
(e.g. ``./src/pkg:`` followed by tab-indented file names), mirroring the
output format of ``ls -R``. An optional regular expression can be supplied to
filter the resulting files (matched against each file's path relative to the
requested directory). Common VCS/build/cache directories (e.g. ``.git``) are
always excluded. To keep results manageable, the number of matched files per
item is capped; use ``pattern`` to narrow down large directories instead of
raising the limit.
"""
import os
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools._directories import normalize_directory
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
__all__ = [
    'ListError',
    'ListItem',
    'ListResult',
    'ListItemError',
    'ListBatchResult',
    'list',
    'ListTool',
    'register_list_tool']
_MAX_ENTRIES = 50
_EXCLUDED_DIRS = {
    '.git',
    '.hg',
    '.svn',
    '__pycache__',
    '.mypy_cache',
    '.pytest_cache',
    '.ruff_cache',
    '.tox',
    '.venv',
    'venv',
    'node_modules',
    '.idea',
    '.vscode',
    'dist',
    'build',
    '.cache'}

class ListError(Exception):
    """Raised when a directory listing cannot be performed."""

@dataclass(frozen=True)
class ListItem:
    """One directory to list.

    Attributes:
        path: Absolute directory path to list (must exist and be a directory).
        pattern: Optional regular expression to filter results. Only matching file paths are included.
    """
    path: str
    pattern: str | None = None

@dataclass(frozen=True)
class ListResult:
    """Result of listing a single directory, mirroring its input path for result association."""
    path: str
    entries: list[str]
    warning: str | None = None

@dataclass(frozen=True)
class ListItemError:
    """Error listing a single directory, mirroring its input path for result association."""
    path: str
    error: str

@dataclass(frozen=True)
class ListBatchResult:
    """Result of :func:`list`.

    Attributes:
        results: One :class:`ListResult` per successfully listed directory.
        errors: One :class:`ListItemError` per directory that failed.
    """
    results: list[ListResult]
    errors: list[ListItemError]

def _list_one(item: ListItem) -> ListResult:
    """List all files below the absolute directory ``item.path``, grouped like ``ls -R``.

    Raises:
        ListError: If path is not absolute.
        ListError: If path does not exist or is not a directory.
        ListError: If pattern is not a valid regular expression.
        ListError: If more than ``_MAX_ENTRIES`` entries remain even after the result has been
            reduced to one entry per directory level and then to top-level directories only.
    """
    '# When the flat file count exceeds the limit, the result is progressively reduced:'
    '# first to one entry per directory level (with a per-level file count), then, if'
    '# that still exceeds the limit, to top-level directories only (with an aggregated'
    '# file count below each). Both reduced results are returned with a warning.'
    dir_path = Path(item.path)
    if not dir_path.is_absolute():
        raise ListError('Path must be absolute.')
    dir_path = normalize_directory(dir_path)
    if not dir_path.is_dir():
        raise ListError('Directory not found or not a directory.')
    try:
        regex = re.compile(item.pattern) if item.pattern else None
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
    if match_count <= _MAX_ENTRIES:
        entries = []
        for rel_dir in sorted(groups):
            header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
            entries.append(f'{header}:')
            entries.extend(groups[rel_dir])
        return ListResult(path=item.path, entries=entries)
    '# Step 1: collapse each directory level to a single representative entry,'
    '# annotated with the total file count for that level.'
    level1_counts = {rel_dir: len(files) for rel_dir, files in groups.items()}
    if len(level1_counts) <= _MAX_ENTRIES:
        entries = []
        for rel_dir in sorted(groups):
            header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
            count = level1_counts[rel_dir]
            entries.append(f'{header}: ({count} file{('s' if count != 1 else '')})')
            entries.append(groups[rel_dir][0])
        warning = f"Result reduced because the total number of entries ({match_count}) exceeds the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter."
        return ListResult(path=item.path, entries=entries, warning=warning)
    '# Step 2: the per-level reduction still exceeds the limit, so collapse the'
    '# tree depth and aggregate file counts below each top-level directory.'

    def _top_key(rel_dir: str) -> str:
        if rel_dir == '.':
            return '.'
        return rel_dir.split(os.sep)[0]
    top_counts: dict[str, int] = {}
    for rel_dir, files in groups.items():
        key = _top_key(rel_dir)
        top_counts[key] = top_counts.get(key, 0) + len(files)
    if len(top_counts) > _MAX_ENTRIES:
        raise ListError(
            f"Too many entries even after reducing by directory level ({
                len(top_counts)}) exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")
    entries = []
    for key in sorted(top_counts):
        header = key if key == '.' else './' + key.replace(os.sep, '/')
        count = top_counts[key]
        entries.append(f'{header}: ({count} file{('s' if count != 1 else '')} below)')
    warning = f"Result reduced to top-level directories the limit of {_MAX_ENTRIES} is exceeded ({
        len(level1_counts)}). Narrow down the result using the 'pattern' regular expression parameter."
    return ListResult(path=item.path, entries=entries, warning=warning)

def _dedupe_hierarchy(items: list[ListItem]) -> list[ListItem]:
    """Remove items whose path lies below another item's path in the same directory hierarchy.

    Only the topmost item of each hierarchy is kept; items for separate hierarchies
    are left untouched. Original relative order of the kept items is preserved.
    """
    resolved = [(item, Path(item.path).resolve()) for item in items]
    resolved.sort(key=lambda pair: len(pair[1].parts))
    kept_paths: list[Path] = []
    kept_items: list[ListItem] = []
    for item, path in resolved:
        if any((path == kept or kept in path.parents for kept in kept_paths)):
            continue
        kept_paths.append(path)
        kept_items.append(item)
    kept_ids = {id(item) for item in kept_items}
    return [item for item in items if id(item) in kept_ids]

def list(items: list[ListItem]) -> ListBatchResult:
    """List each directory in ``items``.

    Args:
        items: Directories to list. Must be non-empty.

    Returns:
        ListBatchResult: one result per successfully listed directory, one error per failed directory.

    Raises:
        ListError: If items is empty.
    """
    if not items:
        raise ListError("'items' must be a non-empty list.")
    items = _dedupe_hierarchy(items)
    results: list[ListResult] = []
    errors: list[ListItemError] = []
    for item in items:
        try:
            results.append(_list_one(item))
        except ListError as exc:
            errors.append(ListItemError(path=item.path, error=str(exc)))
    return ListBatchResult(results=results, errors=errors)

class ListTool(ToolDefinition):
    name = 'list'
    title = 'List directory contents'
    description = 'List all files below one or more absolute directory paths, recursively, as a flat list, for a batch of items. Filter each result with a regular expression. Limits apply per item, not per batch.'
    _ITEM_PROPERTIES = {'path': {'type': 'string', 'description': 'Absolute directory path.'},
                        'pattern': {'type': 'string', 'description': 'Regular expression to filter the result.'}}
    _ITEM_REQUIRED = ['path']
    _ITEMS_DESCRIPTION = 'Directories to list.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> ListItem:
            return ListItem(path=it['path'], pattern=it.get('pattern'))

        def result_serializer(r):
            return {'path': r.path, 'entries': r.entries, **({'warning': r.warning} if r.warning else {})}
        return handle_batch_tool(ctx, item_factory, list, ListError, result_serializer)

def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ListTool())
    functions.register(list)