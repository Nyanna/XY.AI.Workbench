"""List tool – returns the files below a directory, grouped by relative subdirectory, for a batch of items.

Walks each given absolute directory (recursively by default) and returns the file
names grouped by the directory they live in. An optional regular expression filters
the files (matched against each file's path relative to the requested directory).
Visibility is controlled by ``.mcpcignore`` files.

Styles:
    ``flat`` (default): every directory with entries is a header such as ``./src/pkg:``
    followed by its entries, like ``ls -R``.
    ``hierarchical``: directories are nested lists (``{'./src/': [{'pkg/': [...]}]}``);
    shared path prefixes are written once and chains of single subdirectories are
    consolidated into one key (``'./a/b/c/'``). Files of the requested directory itself
    are plain entries at the top level.

Limit: a result has at most ``_MAX_ENTRIES`` entries (counted in the flat form: headers,
files and pseudo entries). Beyond that, files are hidden first, one by one from the
deepest level upwards and evenly across the directories of the same depth, leaving a
``N files hidden`` entry. If that is not enough, the deepest directories are removed one
by one, leaving ``N subdirectories hidden`` (and the files hidden below) in their parent.
Use ``pattern`` to narrow down large directories instead.
"""
import os
import re
from dataclasses import dataclass, field
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
_STYLES = ('flat', 'hierarchical')
_NO_ENTRIES = 'No entries'
'# Extra characters a nested directory costs besides its key (braces, brackets, quotes, comma).'
_NEST_OVERHEAD = 8
Entry = str | dict[str, list[Any]]

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
        style: ``flat`` (default) or ``hierarchical`` (nested directory lists, more compact).
    """
    path: str
    pattern: str | None = None
    recursive: bool = True
    style: str = 'flat'

@dataclass(frozen=True)
class ListResult(ItemResult):
    """Result of listing a single directory, mirroring its input path for result association.

    ``entries`` holds strings (files, pseudo entries, flat headers) and, in the
    hierarchical style, directories as ``{key: [entries]}``.
    """
    path: str
    entries: list[Entry]

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

    def payload(self) -> dict[str, Any]:
        return {'entries': self.entries or _NO_ENTRIES}

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

@dataclass(eq=False, repr=False)
class _Node:
    """A directory of the result tree; ``rel`` is its path relative to the listed directory.

    ``hidden_files`` / ``hidden_dirs`` count what was removed from the node itself and
    from the branches removed below it.
    """
    name: str
    depth: int
    rel: str
    parent: '_Node | None' = None
    children: dict[str, '_Node'] = field(default_factory=dict)
    files: list[str] = field(default_factory=list)
    hidden_files: int = 0
    hidden_dirs: int = 0

    def size(self) -> int:
        """Entries of the flat form: header plus files and pseudo entries; 0 if there are none."""
        own = len(self.files) + (self.hidden_files > 0) + (self.hidden_dirs > 0)
        return own + 1 if own else 0

def _check_style(style: str) -> None:
    if style not in _STYLES:
        raise ListError(f'Invalid style {style!r}: expected one of {', '.join(_STYLES)}.')

def _node_at(tree: _Node, rel_dir: str) -> _Node:
    node = tree
    if rel_dir != '.':
        for part in rel_dir.split(os.sep):
            child = node.children.get(part)
            if child is None:
                child = node.children[part] = _Node(part, node.depth + 1, f'{node.rel}/{part}', node)
            node = child
    return node

def _preorder(tree: _Node) -> list[_Node]:
    order: list[_Node] = []
    stack = [tree]
    while stack:
        node = stack.pop()
        order.append(node)
        stack.extend((node.children[name] for name in sorted(node.children, reverse=True)))
    return order

def _reduce(order: list[_Node], limit: int) -> int | None:
    """Hide entries until at most ``limit`` remain; returns the original count if anything was hidden.

    Files go first: from the deepest level upwards, the last file of each directory of a
    level in turn. Then the deepest directories are removed one by one; their hidden
    counts move to the parent.
    """
    total = sum((node.size() for node in order))
    original = total
    if total <= limit:
        return None
    by_depth: dict[int, list[_Node]] = {}
    for node in order:
        if node.files:
            by_depth.setdefault(node.depth, []).append(node)
    for depth in sorted(by_depth, reverse=True):
        level = by_depth[depth]
        while level and total > limit:
            for node in level:
                if total <= limit:
                    break
                before = node.size()
                node.files.pop()
                node.hidden_files += 1
                total += node.size() - before
            level = [node for node in level if node.files]
    if total > limit:
        for node in sorted((n for n in order if n.parent is not None), key=lambda n: -n.depth):
            if total <= limit:
                break
            parent = node.parent
            before = node.size() + parent.size()
            parent.hidden_files += node.hidden_files
            parent.hidden_dirs += node.hidden_dirs + 1
            del parent.children[node.name]
            total += parent.size() - before
    return original

def _own_entries(node: _Node) -> list[Entry]:
    entries: list[Entry] = [*node.files]
    if node.hidden_files:
        entries.append(f'{node.hidden_files} file{('s' if node.hidden_files != 1 else '')} hidden')
    if node.hidden_dirs:
        entries.append(f'{node.hidden_dirs} {('subdirectories' if node.hidden_dirs != 1 else 'subdirectory')} hidden')
    return entries

def _render_flat(tree: _Node) -> list[Entry]:
    entries: list[Entry] = []
    for node in _preorder(tree):
        own = _own_entries(node)
        if own:
            entries.append(f'{node.rel}:')
            entries.extend(own)
    return entries

def _nest(prefix: str, count: int) -> bool:
    """Whether ``count`` subdirectories are shorter under one ``prefix`` key than each carrying the prefix."""
    return len(prefix) + _NEST_OVERHEAD < count * len(prefix)

def _render_hierarchical(tree: _Node) -> list[Entry]:
    """Nested form: own entries of a directory plus ``{'name/': [...]}`` per subdirectory.

    A directory without own entries that leads to a single subtree is consolidated into one
    key (``'a/b/c/'``). Where such a chain ends in several subdirectories, they are nested
    under the chain key only if that is shorter than repeating the chain in each key.
    """
    '# Per directory: own entries, nested subdirectories (keys relative to it) and, for a'
    '# directory without own entries and with several subdirectories, the chain prefix that'
    "# still awaits the nest-or-repeat decision (``''`` for the directory itself)."
    content: dict[int, tuple[list[Entry], list[dict[str, list[Entry]]], str | None]] = {}
    for node in reversed(_preorder(tree)):
        own = _own_entries(node)
        parts = [(name, content.pop(id(node.children[name]))) for name in sorted(node.children)]
        parts = [(name, state) for name, state in parts if state[0] or state[1]]
        if not own and len(parts) == 1 and (parts[0][1][2] is not None):
            name, (_, child_subs, chain) = parts[0]
            content[id(node)] = ([], child_subs, f'{name}/{chain}')
            continue
        subs: list[dict[str, list[Entry]]] = []
        for name, (child_own, child_subs, chain) in parts:
            prefix = f'{name}/'
            if chain is not None:
                prefix += chain
                subs.extend(_nested_or_repeated(prefix, child_subs))
            elif child_own:
                subs.append({prefix: [*child_own, *child_subs]})
            else:
                (key, value), = child_subs[0].items()
                subs.append({prefix + key: value})
        content[id(node)] = (own, subs, '' if not own and len(subs) > 1 else None)
    own, subs, chain = content[id(tree)]
    if chain is None:
        return [*own, *({f'./{key}': value} for sub in subs for key, value in sub.items())]
    return _nested_or_repeated(f'./{chain}', subs, allow_nesting=bool(chain))

def _nested_or_repeated(prefix: str, subs: list[dict[str, list[Entry]]], allow_nesting: bool=True) -> list[dict[str, list[Entry]]]:
    if allow_nesting and _nest(prefix, len(subs)):
        return [{prefix: subs}]
    return [{prefix + key: value} for sub in subs for key, value in sub.items()]

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
        _check_style(item.style)
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
        ListError: If the style is unknown.
    """
    item = items[0]
    _check_style(item.style)
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
    tree = _Node(name='.', depth=0, rel='.')
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
            _node_at(tree, rel_dir).files = matched_files
        if not item.recursive:
            dirs.clear()
    original = _reduce(_preorder(tree), _MAX_ENTRIES)
    entries = _render_hierarchical(tree) if item.style == 'hierarchical' else _render_flat(tree)
    if original is not None:
        notes += (f"Result reduced because the number of entries ({original}) exceeds the limit of {_MAX_ENTRIES}: files and subdirectories were hidden. Narrow down the result using the 'pattern' regular expression parameter.",)
    return ListResult(path=item.path, entries=entries, warnings=notes)

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
    description = "List all files below one or more absolute directory paths, recursively, for a batch of items. Filter each result with a regular expression. Limits apply per item, not per batch:"
    _ITEM_PROPERTIES = {
        'path': {
            'type': 'string',
            'description': 'Absolute directory path.'},
        'pattern': {
            'type': 'string',
            'description': 'Regular expression to filter the result.'},
        'recursive': {
            'type': 'boolean',
                    'description': 'List subdirectories recursively. Default true.'},
        'style': {
            'type': 'string',
            'enum': [
                *
                _STYLES],
            'description': "'flat' (default): one header per directory. 'hierarchical': directories as nested lists, more compact."}}
    _ITEM_REQUIRED = ['path']
    _ITEMS_DESCRIPTION = 'Directories to list.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`list`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> ListItem:
            return ListItem(
                path=it['path'],
                pattern=it.get('pattern'),
                recursive=it.get(
                    'recursive',
                    True),
                style=it.get(
                    'style',
                    'flat'))
        return handle_batch_tool(ctx, item_factory, list, ListError)

def register_list_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ListTool())
    functions.register(list)