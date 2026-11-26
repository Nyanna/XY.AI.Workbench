"""Facade the ``ast_*`` tools call, dispatching to a per-file engine.

The engine is chosen by file extension: Python files use the ``ast``-based
:mod:`.python` engine, everything else the generic tree-sitter :mod:`.generic`
engine, or - for extensions no grammar covers - its whole-file
``PlainTextEngine`` fallback. Snippets passed as raw ``code`` (no path)
default to Python.

A single content-hash validated :class:`AstCache` – reused across engines –
holds parsed :class:`~.base.Tree` objects keyed by absolute path and validated
on every access by ``st_mtime_ns`` and, on change, by a content hash.

The engine-agnostic model (``AstError``, ``Located``, ``OutlineNode`` …) is
re-exported here so the tools only ever import :mod:`.core`.
"""
from __future__ import annotations
import hashlib
import logging
import threading
from dataclasses import dataclass
from pathlib import Path
from xy.ai.mcpc.tools.ast.base import AstAmbiguous, AstError, AstTextError, Engine, Located, OutlineNode, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, resolve_by_prefix, to_dict
from xy.ai.mcpc.tools.ast.imports import ImportOutcome, apply_generic
from xy.ai.mcpc.tools.ast import generic
from xy.ai.mcpc.tools.ast import python
logger = logging.getLogger(__name__)
_PYTHON_EXTENSIONS = ('.py', '.pyi')

def engine_for_path(path: Path) -> Engine:
    """Return the engine responsible for ``path`` based on its extension."""
    ext = path.suffix.lower()
    if ext in _PYTHON_EXTENSIONS:
        return python.ENGINE
    symbol = generic.language_for_extension(ext)
    if symbol is None:
        return generic.fallback_engine()
    return generic.get_engine(symbol)

@dataclass
class _CacheEntry:
    mtime_ns: int
    content_hash: str
    tree: Tree

class AstCache:
    """Content-hash validated cache of parsed trees keyed by absolute path."""

    def __init__(self) -> None:
        self._entries: dict[str, _CacheEntry] = {}
        self._lock = threading.RLock()

    def get_tree(self, path: Path) -> Tree:
        key = str(path)
        engine = engine_for_path(path)
        with self._lock:
            entry = self._entries.get(key)
            mtime_ns = path.stat().st_mtime_ns
            if entry is not None and entry.mtime_ns == mtime_ns:
                return entry.tree
            source = path.read_text(encoding='utf-8')
            digest = hashlib.sha256(source.encode('utf-8')).hexdigest()
            if entry is not None and entry.content_hash == digest:
                entry.mtime_ns = mtime_ns
                return entry.tree
            tree = engine.parse(source, path)
            self._entries[key] = _CacheEntry(mtime_ns, digest, tree)
            return tree

    def save(self, path: Path, tree: Tree) -> str:
        """Serialise *tree*, write it to *path* and refresh the cache entry."""
        source = tree.engine.serialize(tree)
        path.write_text(source, encoding='utf-8')
        '# Re-parse so cached positions match the file exactly.'
        normalized = tree.engine.parse(source, path)
        digest = hashlib.sha256(source.encode('utf-8')).hexdigest()
        with self._lock:
            self._entries[str(path)] = _CacheEntry(path.stat().st_mtime_ns, digest, normalized)
        return source

    def invalidate(self, path: Path) -> None:
        with self._lock:
            self._entries.pop(str(path), None)
'#: Process-wide shared cache instance.'
CACHE = AstCache()

def load(path_str: str) -> tuple[Path, Tree]:
    """Resolve *path_str* and return it together with its cached tree."""
    path = require_path(path_str)
    return (path, CACHE.get_tree(path))

def parse_source(source: str, engine: Engine | None=None) -> Tree:
    """Parse *source* with *engine* (Python by default for path-less snippets)."""
    return (engine or python.ENGINE).parse(source)

def parse_for(path_str: str, code: str) -> Tree:
    """Parse *code* with the engine selected for *path_str*'s extension."""
    path = require_path(path_str, must_exist=False)
    return engine_for_path(path).parse(code, path)

def locate_all(tree: Tree) -> list[Located]:
    return tree.engine.locate_all(tree)

def edit_node_source(loc: Located) -> str:
    return loc.tree.engine.node_code(loc.node)

def relocate(tree: Tree, old: Located) -> Located | None:
    """Find the node now occupying ``old``'s former slot after an in-place edit.

    Matched by start line and parent type (unaffected by the edit itself), then
    disambiguated by sibling index if several candidates share that line.
    """
    candidates = [loc for loc in locate_all(tree) if loc.lineno == old.lineno and loc.parent_type == old.parent_type]
    for loc in candidates:
        if loc.index == old.index:
            return loc
    return candidates[0] if candidates else None

def replace_node(loc: Located, code: str) -> str | None:
    """Replace ``loc``'s node with ``code``; return its new id."""
    loc.tree.engine.replace(loc, code)
    new_loc = relocate(loc.tree, loc)
    return new_loc.node_id if new_loc and new_loc.node_id != loc.node_id else loc.node_id

def insert_node(loc: Located, code: str, position: str) -> int:
    return loc.tree.engine.insert(loc, code, position)

def delete_node(loc: Located) -> None:
    loc.tree.engine.delete(loc)

def append_nodes(tree: Tree, code: str) -> int:
    return tree.engine.append(tree, code)

@dataclass(frozen=True)
class Placement:
    """Where an insert/append landed, in :func:`locate_all` order.

    Attributes:
        start: Index of the first inserted node after the edit.
        count: Number of addressable nodes the edit added.
        units: Units the engine reported as inserted/appended.
        exact: Whether the nodes outside ``[start, start + count)`` kept their structure
            (node types). If not (e.g. merged statement groups), ``start``/``count``
            cannot be trusted.
    """
    start: int
    count: int
    units: int
    exact: bool

def top_level_ids(ids: list[str]) -> list[str]:
    """Those ``ids`` without an ancestor among ``ids``, in the given order."""
    return [i for i in ids if not any((i.startswith(other + '.') for other in ids if other != i))]

def place_node(tree: Tree, loc: Located | None, code: str, position: str) -> Placement:
    """Insert ``code`` ``position`` ``loc`` (append to ``tree`` if ``loc`` is ``None``) and track where it landed.

    The inserted nodes are found by position, not by an id diff: inserting may rename
    existing nodes (a second ``foo`` turns the old one into ``foo_1``), so neither a
    new id nor a changed id proves a node new. In document order the inserted nodes
    start at the target's index (``before``), right behind its subtree (``after``) or
    at the end (append).
    """
    before = locate_all(tree)
    if loc is None:
        start = len(before)
        units = append_nodes(tree, code)
    else:
        index = next((i for i, other in enumerate(before) if other.node_id == loc.node_id), None)
        if index is None:
            raise AstError('The target node is no longer present.')
        start = index
        if position != 'before':
            start += 1 + sum((1 for other in before if other.node_id.startswith(loc.node_id + '.')))
        units = insert_node(loc, code, position)
    after = locate_all(tree)
    count = len(after) - len(before)
    rest = after[:start] + after[start + count:]
    exact = count > 0 and [other.node_type for other in rest] == [other.node_type for other in before]
    return Placement(start=start, count=count, units=units, exact=exact)

def placed_ids(tree: Tree, placement: Placement) -> list[str] | None:
    """Top-level ids of the nodes ``placement`` inserted, or ``None`` if it is not exact."""
    if not placement.exact:
        return None
    located = locate_all(tree)[placement.start:placement.start + placement.count]
    return top_level_ids([loc.node_id for loc in located])

def apply_imports(tree: Tree, add: list[str] | None, remove: list[str] | None) -> ImportOutcome | None:
    """Apply an item's ``addImport``/``removeImport`` to the already edited ``tree``.

    Independent of the node edit: engines compute all import changes first and write them
    once, so a failure leaves the tree as the node edit left it. It is then reported as
    ``unchecked``, which adds a warning to the batch. Engines without dedicated support
    get the generic check-only fallback.
    """
    if not add and (not remove):
        return None
    try:
        try:
            return tree.engine.apply_imports(tree, add or [], remove or [])
        except NotImplementedError:
            return apply_generic(tree.engine, tree, add or [], remove or [])
    except Exception as exc:
        '# noqa: BLE001 - never fail the edit because of its imports'
        logger.warning('Import handling failed for %s', tree.path, exc_info=True)
        return ImportOutcome(unchecked=True, reason=str(exc) or type(exc).__name__)

def empty_tree(path: Path) -> Tree:
    return engine_for_path(path).empty_tree(path)

def validate_source(path: Path, source: str) -> str | None:
    return engine_for_path(path).validate(source)