"""Engine-agnostic model shared by every ``ast_*`` tool.

The tools address nodes by *selector* (id, type, name, line
range or parent type) and never touch a concrete parser. Two engines implement
:class:`Engine`: a Python one built on the standard-library ``ast`` module and a
generic tree-sitter one for every other language/format. :mod:`.core` picks the
engine per file extension and exposes a thin facade the tools call.

A :class:`Tree` carries a back-reference to the engine that produced it, so
every helper here can dispatch to the right engine without the tools knowing
which one is in play.
"""
from __future__ import annotations
import hashlib
import re
from abc import ABC, abstractmethod
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

class AstError(Exception):
    """A user-facing, path-free error raised by the AST tools."""

@dataclass
class Tree:
    """A parsed file/snippet plus the engine that owns it.

    Attributes:
        engine: The engine that parsed ``raw`` and knows how to mutate it.
        raw: Engine-native tree (``ast.Module`` or ``tree_sitter.Tree``).
        source: Current source text; the single source of truth for tree-sitter
            engines, refreshed by the Python engine only on save.
        path: Absolute path the tree was loaded from, or ``None`` for snippets.
    """
    engine: 'Engine'
    raw: Any
    source: str
    path: Path | None = None

@dataclass
class Located:
    """A node with the engine-independent metadata the selectors match on.

    Attributes:
        tree: The owning tree (for engine dispatch).
        node: Engine-native node object.
        parent: Engine-native container node.
        index: Position of ``node`` among its parent's addressable children.
        node_id: The node's unique ``id`` — its fully-qualified path from the
            root (e.g. ``"MyClass.method"``), in name/hash form or, for nameless
            nodes/segments, a numeric fallback. There is no separate FQN.
        node_type: Engine-reported node type name.
        name: Simple name, if the node carries one.
        lineno / end_lineno: 1-based inclusive line span.
        parent_type: Type name of ``parent``, or ``None`` at the top level.
        expandable: Whether ``read`` should descend into children instead of
            returning the node's full source (a pure container of nested defs).
    """
    tree: Tree
    node: Any
    parent: Any
    index: int
    node_id: str
    node_type: str
    name: str | None
    lineno: int
    end_lineno: int
    parent_type: str | None
    expandable: bool = False

@dataclass(frozen=True)
class OutlineNode:
    """One node in a structural (list/find) result.

    ``id`` is the node's unique, primarily name-based path used by every tool to
    address it. ``code`` carries the node's full source and is populated only by
    ``find`` – ``list`` always leaves it ``None``. ``signature`` is only set for
    class/function nodes. Serialization drops ``None``/empty fields, see
    :func:`to_dict`.
    """
    id: str
    type: str
    lines: str | None
    signature: str | None
    docstring: str | None
    code: str | None = None
    children: list['OutlineNode'] = field(default_factory=list)

@dataclass(frozen=True)
class ReadNode:
    """One node in a subtree read for block-wise edit/replace.

    ``code`` holds the node's full source unless it is a pure container of
    nested addressable nodes, in which case it is ``None`` and ``children`` is
    populated so the agent can descend to the innermost editable block.
    """
    id: str
    type: str
    lines: str
    code: str | None
    children: list['ReadNode'] = field(default_factory=list)

def line_range(loc: Located) -> str:
    """Return ``loc``'s start line, or a ``"start-end"`` range if it spans several."""
    if loc.end_lineno == loc.lineno:
        return str(loc.lineno)
    return f'{loc.lineno}-{loc.end_lineno}'
_ID_CLEAN_RE = re.compile('\\W+')
'#: A statement/anonymous segment keeps accumulating siblings until adding the'
'#: next one would push its source past this many characters (then it splits).'
SEGMENT_MAX_CHARS = 500

def _hash(name: str, length: int) -> str:
    return hashlib.sha1(name.encode('utf-8')).hexdigest()[:length]

def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False) -> str:
    """Return a unique-within-siblings id segment, name-based when feasible.

    A clean, short name becomes the segment verbatim; a long/awkward name collapses
    to a short hash; a nameless node falls back to its numeric ``index``. With
    ``hash_only`` the name is *always* reduced to a 6-char hex hash (used for
    Markdown headings, whose id must never be the literal heading text). Collisions
    among siblings get a numeric suffix.
    """
    seg: str | None = None
    if name:
        if hash_only:
            seg = _hash(name, 6)
        else:
            cleaned = _ID_CLEAN_RE.sub('_', name).strip('_')
            seg = cleaned if cleaned and len(cleaned) <= 40 else 'h' + _hash(name, 8)
    if not seg:
        seg = str(index)
    count = used.get(seg, 0)
    used[seg] = count + 1
    return seg if count == 0 else f'{seg}_{count}'
'#: Node-type substrings (case-insensitive) that identify a class/function'
'#: definition across engines, the only nodes a "signature" makes sense for.'
_SIGNATURE_TYPE_RE = re.compile('class|function', re.IGNORECASE)

def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:
    """Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``, lines only if ``with_lines``)."""
    engine = loc.tree.engine
    signature = engine.signature(loc.node) if _SIGNATURE_TYPE_RE.search(loc.node_type) else None
    return OutlineNode(id=loc.node_id, type=loc.node_type, lines=line_range(loc) if with_lines else None, signature=signature, docstring=engine.docstring(loc.node), code=engine.node_code(loc.node) if with_code else None, children=children or [])

def _compact(value: Any) -> Any:
    """Recursively drop ``None`` values and empty lists from a dataclass-derived structure."""
    if isinstance(value, dict):
        return {k: _compact(v) for k, v in value.items() if v is not None and v != []}
    if isinstance(value, list):
        return [_compact(v) for v in value]
    return value

def to_dict(node: OutlineNode | ReadNode) -> dict:
    """Serialize an :class:`OutlineNode`/:class:`ReadNode` to MCP output, omitting empty fields."""
    return _compact(asdict(node))

@dataclass
class _TreeNode:
    loc: Located
    children: list['_TreeNode'] = field(default_factory=list)

def _build_forest(located: list[Located]) -> list[_TreeNode]:
    """Nest a pre-order list of ``Located`` into a forest via ``node_id`` prefixes."""
    roots: list[_TreeNode] = []
    stack: list[_TreeNode] = []
    for loc in located:
        node = _TreeNode(loc)
        while stack and (not loc.node_id.startswith(stack[-1].loc.node_id + '.')):
            stack.pop()
        (stack[-1].children if stack else roots).append(node)
        stack.append(node)
    return roots

def build_outline(located: list[Located], *, with_code: bool=False, with_lines: bool=True) -> list[OutlineNode]:
    """Build the nested outline of ``located`` (source only if ``with_code``, lines only if ``with_lines``)."""

    def convert(nodes: list[_TreeNode]) -> list[OutlineNode]:
        return [node_outline(t.loc, with_code=with_code, with_lines=with_lines, children=convert(t.children)) for t in nodes]
    return convert(_build_forest(located))

def _to_read(t: _TreeNode) -> ReadNode:
    loc = t.loc
    if loc.expandable and t.children:
        return ReadNode(id=loc.node_id, type=loc.node_type, lines=line_range(loc), code=None, children=[_to_read(c) for c in t.children])
    return ReadNode(id=loc.node_id, type=loc.node_type, lines=line_range(loc), code=loc.tree.engine.node_code(loc.node), children=[])

def read_subtrees(located: list[Located], keys: list[str]) -> list[ReadNode]:
    """Return one read subtree per ``keys`` entry, matched by ``id``.

    Raises:
        AstError: If any key matches no node.
    """
    index: dict[str, _TreeNode] = {}

    def collect(nodes: list[_TreeNode]) -> None:
        for t in nodes:
            index.setdefault(t.loc.node_id, t)
            collect(t.children)
    collect(_build_forest(located))
    result: list[ReadNode] = []
    for key in keys:
        target = index.get(key)
        if target is None:
            raise AstError(f"No node matched '{key}'.")
        result.append(_to_read(target))
    return result

def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None) -> bool:
    if id is not None and loc.node_id != id:
        return False
    if node_type is not None and loc.node_type.lower() != node_type.lower():
        return False
    if name is not None and loc.name != name:
        return False
    if lineno is not None and loc.lineno != lineno:
        return False
    if end_lineno is not None and loc.end_lineno != end_lineno:
        return False
    if parent_type is not None and (loc.parent_type or '').lower() != parent_type.lower():
        return False
    return True

def find(tree: Tree, **filters: object) -> list[Located]:
    active = {k: v for k, v in filters.items() if v is not None}
    '# type: ignore[arg-type]'
    return [loc for loc in tree.engine.locate_all(tree) if matches(loc, **active)]

def most_specific(located: list[Located], lineno: int, end_lineno: int) -> Located | None:
    """Return the smallest node in *located* fully containing lines [lineno, end_lineno]."""
    best = None
    for loc in located:
        if loc.lineno <= lineno and loc.end_lineno >= end_lineno:
            if best is None or loc.end_lineno - loc.lineno < best.end_lineno - best.lineno:
                best = loc
    return best

class Engine(ABC):
    """A parser back-end turning source into an addressable, mutable tree.

    Structural mutations differ fundamentally between back-ends: the Python
    engine edits the ``ast`` object graph and re-serialises it via ``unparse``,
    whereas generic engines splice source text at node byte-ranges and re-parse.
    Both, however, expose the same node-oriented operations below.
    """
    '#: Human-readable engine name (used e.g. to guard Python-only tools).'
    name: str = 'engine'

    @abstractmethod
    def parse(self, source: str, path: Path | None=None) -> Tree:
        """Parse ``source`` into a :class:`Tree`, raising :class:`AstError` on error."""

    @abstractmethod
    def empty_tree(self, path: Path | None=None) -> Tree:
        """Return an empty tree, used when appending to a not-yet-existing file."""

    @abstractmethod
    def serialize(self, tree: Tree) -> str:
        """Render ``tree`` back to source text for writing to disk."""

    @abstractmethod
    def validate(self, source: str) -> str | None:
        """Return an error message if ``source`` is malformed, else ``None``."""

    @abstractmethod
    def locate_all(self, tree: Tree) -> list[Located]:
        """Flatten ``tree`` into every addressable node, in document order."""

    @abstractmethod
    def signature(self, node: Any) -> str:
        """One-line rendering of ``node``'s header (or the node itself)."""

    @abstractmethod
    def docstring(self, node: Any) -> str | None:
        """Short docstring of ``node``, if the format has such a concept."""

    @abstractmethod
    def node_code(self, node: Any) -> str:
        """Full source of a single ``node``."""

    @abstractmethod
    def replace(self, loc: Located, code: str) -> None:
        """Replace ``loc``'s node with ``code``."""

    @abstractmethod
    def insert(self, loc: Located, code: str, position: str) -> int:
        """Insert ``code`` ``"before"``/``"after"`` ``loc``; return units inserted."""

    @abstractmethod
    def delete(self, loc: Located) -> None:
        """Delete ``loc``'s node from its container."""

    @abstractmethod
    def append(self, tree: Tree, code: str) -> int:
        """Append ``code`` at ``tree``'s top level; return units appended."""

def require_path(path_str: str, *, must_exist: bool=True) -> Path:
    """Validate a mandatory absolute path, raising :class:`AstError` on failure."""
    path = Path(path_str)
    if not path.is_absolute():
        raise AstError('Path must be absolute.')
    if must_exist:
        if not path.exists():
            raise AstError('File not found.')
        if not path.is_file():
            raise AstError('Not a regular file.')
    return path
'#: JSON-Schema fragment for :class:`OutlineNode`, shared by list/find.'
OUTLINE_NODE_SCHEMA = {'type': 'object', 'properties': {'id': {'type': 'string', 'description': 'Unique, primarily name-based node id (numeric fallback for nameless segments); the sole address for every tool.'}, 'type': {'type': 'string'}, 'lines': {'type': 'string', 'description': "Line number, or 'start-end' if the node spans multiple lines; omitted unless the 'tools' or 'edit-lines' tool is enabled in the session."}, 'signature': {'type': 'string', 'description': 'One-line header; present only for class/function nodes.'}, 'docstring': {'type': 'string'}, 'code': {'type': 'string', 'description': 'Full node source; populated by find, omitted in list.'}, 'children': {'type': 'array', 'items': {'$ref': '#/$defs/outline_node'}}}, 'required': ['id', 'type']}