"""List tool – returns files below a directory, grouped by relative subdirectory like ``ls -R``, for a batch of items.

Walks each given absolute directory recursively and returns all file paths
(files only, no directories), grouped by the relative directory they live in
(e.g. ``./src/pkg:`` followed by tab-indented file names), mirroring the
output format of ``ls -R``. An optional regular expression can be supplied to
filter the resulting files (matched against each file's path relative to the
requested directory). Visibility is controlled by ``.mcpcignore`` files. To keep results manageable, the number of matched files per
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
from xy.ai.mcpc.tools._mcpcignore import is_allowed, is_excluded
from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
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

class ListError(Exception):
    """Raised when a directory listing cannot be performed."""

@dataclass(frozen=True)
class ListItem:
    """One directory to list.

    Attributes:
        path: Absolute directory path to list (must exist and be a directory).
        pattern: Optional regular expression to filter results. If it contains '/', it is matched against the relative path (e.g. './dir/file.ext'); otherwise it is matched against the file name only.
        recursive: If True (default), list files in all subdirectories. If False, list only the
            immediate contents of ``path``.
    """
    path: str
    pattern: str | None = None
    recursive: bool = True

@dataclass(frozen=True)
class ListResult(ItemResult):
    """Result of listing a single directory, mirroring its input path for result association."""
    path: str
    entries: list[str]

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

    def payload(self) -> dict[str, Any]:
        return {'entries': self.entries}

@dataclass(frozen=True)
class ListItemError(ItemError):
    """Error listing a single directory, mirroring its input path for result association."""
    path: str

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

class ListBatchResult(BatchResult[ListResult, ListItemError]):
    """Result of :func:`list`.

    Attributes:
        results: One :class:`ListResult` per successfully listed directory.
        errors: One :class:`ListItemError` per directory that failed.
    """

@dataclass(frozen=True)
class _Selection:
    """One requested directory of a merged group: which files below it are wanted."""
    directory: Path
    regex: re.Pattern | None
    on_path: bool
    recursive: bool

def _compile(pattern: str | None) -> re.Pattern | None:
    try:
        return re.compile(pattern) if pattern else None
    except re.error as exc:
        raise ListError(f'Invalid regex pattern: {exc}') from exc

def _mergeable_directory(item: ListItem) -> Path | None:
    """The directory ``item`` lists, if it can take part in a merge; ``None`` if it is invalid.

    Invalid items (relative or missing path, bad pattern) are never merged so that they
    report their own error instead of breaking another item's result.
    """
    path = Path(item.path)
    if not path.is_absolute():
        return None
    directory = Path(os.path.abspath(normalize_directory(path)))
    if not directory.is_dir():
        return None
    try:
        _compile(item.pattern)
    except ListError:
        return None
    return directory

def _group_items(items: list[ListItem]) -> list[list[ListItem]]:
    """Merge items whose directory lies at or below a recursive item onto the topmost such item.

    Every group starts with its topmost item. Groups are returned in the order of their
    topmost items in ``items``. Nothing is dropped: the members of a group are
    combined later by applying each member's pattern to its own directory.
    """
    candidates = [(index, item, _mergeable_directory(item)) for index, item in enumerate(items)]
    candidates.sort(key=lambda c: len(c[2].parts) if c[2] is not None else 0)
    groups: dict[int, list[ListItem]] = {}
    tops: list[tuple[Path, int]] = []
    for index, item, directory in candidates:
        owner = None
        if directory is not None:
            owner = next((i for p, i in tops if p == directory or p in directory.parents), None)
        if owner is not None:
            groups[owner].append(item)
            continue
        groups[index] = [item]
        if directory is not None and item.recursive:
            tops.append((directory, index))
    return [groups[index] for index in sorted(groups)]

def _list_group(items: list[ListItem]) -> ListResult:
    """List the topmost item ``items[0]``, filtered by the patterns of all ``items`` (see :func:`_group_items`).

    Raises:
        ListError: If path is not absolute.
        ListError: If path does not exist or is not a directory.
        ListError: If pattern is not a valid regular expression.
        ListError: If more than ``_MAX_ENTRIES`` entries remain even after the result has been
            reduced to one entry per directory level and then by directory depth.
    """
    '# When the flat file count exceeds the limit, the result is progressively reduced:'
    '# first to one entry per directory level (with a per-level file count), then, if'
    '# that still exceeds the limit, by cutting the directory depth as late as possible'
    '# (aggregated file count below each remaining directory). Both reduced results are'
    '# returned with a warning.'
    item = items[0]
    dir_path = Path(item.path)
    if not dir_path.is_absolute():
        raise ListError('Path must be absolute.')
    dir_path = normalize_directory(dir_path)
    if not dir_path.is_dir():
        raise ListError('Directory not found or not a directory.')
    top = os.path.abspath(dir_path)
    selections = []
    for member in items:
        regex = _compile(member.pattern)
        selections.append(
            _Selection(
                directory=Path(
                    os.path.abspath(
                        normalize_directory(
                            Path(
                                member.path)))),
                regex=regex,
                on_path=regex is not None and '/' in member.pattern,
                recursive=member.recursive))
    notes: tuple[str, ...] = ()
    if len(items) > 1:
        merged = ', '.join((m.path + (f' (pattern {m.pattern!r})' if m.pattern else '') for m in items[1:]))
        notes = (
            f'Items below this directory were merged into this result: each pattern was applied to its own directory and the matches were combined: {merged}.',
        )
    groups: dict[str, list[str]] = {}
    match_count = 0
    for root, dirs, files in os.walk(top):
        dirs[:] = [d for d in dirs if not is_excluded(Path(root) / d)]
        rel_dir = os.path.relpath(root, top)
        root_path = Path(root)
        active = []
        for selection in selections:
            if root_path == selection.directory:
                active.append((selection, '.'))
            elif selection.recursive and selection.directory in root_path.parents:
                header = './' + os.path.relpath(root, selection.directory).replace(os.sep, '/')
                active.append((selection, header))
        matched_files = []
        for file in sorted(files):
            if not active or not is_allowed(root_path / file):
                continue
            for selection, header in active:
                candidate = f'{header}/{file}' if selection.on_path else file
                if selection.regex is None or selection.regex.search(candidate):
                    matched_files.append(file)
                    break
        if matched_files:
            groups[rel_dir] = matched_files
            match_count += len(matched_files)
        if not item.recursive:
            dirs.clear()
    if match_count <= _MAX_ENTRIES:
        entries = []
        for rel_dir in sorted(groups):
            header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
            entries.append(f'{header}:')
            entries.extend(groups[rel_dir])
        return ListResult(path=item.path, entries=entries, warnings=notes)
    '# Step 1: collapse each directory level to a single representative entry,'
    '# annotated with the total file count for that level.'
    if len(groups) <= _MAX_ENTRIES:
        entries = []
        for rel_dir in sorted(groups):
            header = rel_dir if rel_dir == '.' else './' + rel_dir.replace(os.sep, '/')
            count = len(groups[rel_dir])
            entries.append(f'{header}: ({count} file{('s' if count != 1 else '')})')
            entries.append(groups[rel_dir][0])
        warning = f"Result reduced because the total number of entries ({match_count}) exceeds the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter."
        return ListResult(path=item.path, entries=entries, warnings=(*notes, warning))
    '# Step 2: still too many directories. Cut the directory depth as late as possible,'
    '# i.e. keep the deepest depth whose number of directories fits the limit, and'
    '# aggregate the file counts below each remaining directory.'
    parts_by_dir = {rel_dir: [] if rel_dir == '.' else rel_dir.split(os.sep) for rel_dir in groups}
    max_depth = max((len(parts) for parts in parts_by_dir.values()))
    for depth in range(max_depth - 1, 0, -1):
        counts: dict[str, int] = {}
        for rel_dir, files in groups.items():
            key = '/'.join(parts_by_dir[rel_dir][:depth]) or '.'
            counts[key] = counts.get(key, 0) + len(files)
        if len(counts) <= _MAX_ENTRIES:
            entries = []
            for key in sorted(counts):
                header = key if key == '.' else './' + key
                count = counts[key]
                entries.append(f'{header}: ({count} file{('s' if count != 1 else '')} below)')
            warning = f"Result reduced to directories up to depth {depth} because the number of directories with matches ({
                len(groups)}) exceeds the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter."
            return ListResult(path=item.path, entries=entries, warnings=(*notes, warning))
    raise ListError(
        f"Too many entries even after reducing by directory depth: {
            len(groups)} directories with matches exceed the limit of {_MAX_ENTRIES}. Narrow down the result using the 'pattern' regular expression parameter.")

def list(items: list[ListItem]) -> ListBatchResult:
    """List each directory in ``items``.

    Items whose directory lies at or below another *recursive* item are merged into that
    topmost item to avoid listing the same files twice: its result covers the combined
    matches of all merged items (each pattern applies to its own directory) and its
    warnings name the merged items. Items are never dropped.

    Args:
        items: Directories to list. Must be non-empty.

    Returns:
        ListBatchResult: one result per topmost directory, one error per failed directory.

    Raises:
        ListError: If items is empty.
    """
    if not items:
        raise ListError("'items' must be a non-empty list.")
    results: list[ListResult] = []
    errors: list[ListItemError] = []
    for group in _group_items(items):
        try:
            results.append(_list_group(group))
        except ListError as exc:
            errors.append(ListItemError(path=group[0].path, error=str(exc)))
    return ListBatchResult(results=results, errors=errors)

class ListTool(ToolDefinition):
    name = 'list'
    title = 'List directory contents'
    description = 'List all files below one or more absolute directory paths, recursively, as a flat list, for a batch of items. Filter each result with a regular expression. Limits apply per item, not per batch.'
    _ITEM_PROPERTIES = {
        'path': {
            'type': 'string', 'description': 'Absolute directory path.'}, 'pattern': {
                'type': 'string', 'description': 'Regular expression to filter the result.'}, 'recursive': {
                    'type': 'boolean', 'description': 'List subdirectories recursively. Default true.'}}
    _ITEM_REQUIRED = ['path']
    _ITEMS_DESCRIPTION = 'Directories to list.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> ListItem:
            return ListItem(path=it['path'], pattern=it.get('pattern'), recursive=it.get('recursive', True))
        return handle_batch_tool(ctx, item_factory, list, ListError)

def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ListTool())
    functions.register(list)