"""Facade the ``ast_*`` tools call, dispatching to a per-file engine.

The engine is chosen by file extension: Python files use the ``ast``-based
:mod:`.python` engine, everything else the generic tree-sitter :mod:`.generic`
engine. Snippets passed as raw ``code`` (no path) default to Python.

A single content-hash validated :class:`AstCache` – reused across engines –
holds parsed :class:`~.base.Tree` objects keyed by absolute path and validated
on every access by ``st_mtime_ns`` and, on change, by a content hash.

The engine-agnostic model (``AstError``, ``Located``, ``OutlineNode`` …) is
re-exported here so the tools only ever import :mod:`.core`.
"""
from __future__ import annotations
import hashlib
import threading
from dataclasses import dataclass
from pathlib import Path
from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, OutlineNode, OUTLINE_NODE_SCHEMA, Tree, build_outline, find, line_range, matches, most_specific, node_outline, read_subtrees, require_path, to_dict
from xy.ai.mcpc.tools.ast import generic
from xy.ai.mcpc.tools.ast import python
_PYTHON_EXTENSIONS = ('.py', '.pyi')

def engine_for_path(path: Path) -> Engine:
    """Return the engine responsible for ``path`` based on its extension."""
    ext = path.suffix.lower()
    if ext in _PYTHON_EXTENSIONS:
        return python.ENGINE
    symbol = generic.language_for_extension(ext)
    if symbol is None:
        raise AstError(f"No AST engine available for '{ext or path.name}' files.")
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

def replace_node(loc: Located, code: str) -> None:
    loc.tree.engine.replace(loc, code)

def insert_node(loc: Located, code: str, position: str) -> int:
    return loc.tree.engine.insert(loc, code, position)

def delete_node(loc: Located) -> None:
    loc.tree.engine.delete(loc)

def append_nodes(tree: Tree, code: str) -> int:
    return tree.engine.append(tree, code)

def empty_tree(path: Path) -> Tree:
    return engine_for_path(path).empty_tree(path)

def validate_source(path: Path, source: str) -> str | None:
    return engine_for_path(path).validate(source)