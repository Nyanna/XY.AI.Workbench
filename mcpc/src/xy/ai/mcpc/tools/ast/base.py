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
import difflib
import hashlib
import re
import string
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
            nodes/segments, a stable content-hash fallback. There is no separate FQN.
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
    """One node in a structural (list/find/read) result.

    ``id`` is the node's unique, primarily name-based path used by every tool to
    address it. ``code`` carries the node's full source and is populated by
    ``find``/``read`` – ``list`` always leaves it ``None``. ``signature``/
    ``docstring`` are only set for class/function nodes whose ``code`` is
    *not* included, since the full source already makes them visible.
    Serialization drops ``None``/empty fields, see :func:`to_dict`.
    """
    id: str
    type: str
    lines: str | None
    signature: str | None
    docstring: str | None
    code: str | None = None
    children: list['OutlineNode'] = field(default_factory=list)

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
_ID_HASH_ALPHABET = string.digits + string.ascii_letters

def _content_hash(content: str, length: int=6) -> str:
    """Base62 (0-9a-zA-Z) digest of ``content``, stable across unrelated tree edits."""
    digest = int.from_bytes(hashlib.sha1(content.encode('utf-8')).digest(), 'big')
    base = len(_ID_HASH_ALPHABET)
    chars = []
    for _ in range(length):
        digest, rem = divmod(digest, base)
        chars.append(_ID_HASH_ALPHABET[rem])
    return ''.join(chars)

def id_segment(name: str | None, index: int, used: dict[str, int], *, hash_only: bool=False, content: str | None=None) -> str:
    """Return a unique-within-siblings id segment, name-based when feasible.

    A clean, short name becomes the segment verbatim; a long/awkward name collapses
    to a short hash; a nameless node falls back to a 6-char content hash (derived
    from ``content``, stable across edits made elsewhere in the file) or, lacking
    that, its numeric ``index``. With ``hash_only`` the name is *always* reduced to
    a 6-char hex hash (used for Markdown headings, whose id must never be the
    literal heading text). Collisions among siblings get a numeric suffix.
    """
    seg: str | None = None
    if name:
        if hash_only:
            seg = _hash(name, 6)
        else:
            cleaned = _ID_CLEAN_RE.sub('_', name).strip('_')
            seg = cleaned if cleaned and len(cleaned) <= 40 else 'h' + _hash(name, 8)
    if not seg:
        seg = _content_hash(content, 6) if content else str(index)
    count = used.get(seg, 0)
    used[seg] = count + 1
    return seg if count == 0 else f'{seg}_{count}'
'#: Node-type substrings (case-insensitive) that identify a class/function'
'#: definition across engines, the only nodes a "signature" makes sense for.'
_SIGNATURE_TYPE_RE = re.compile('class|function|method|constructor|interface|enum|record', re.IGNORECASE)

def node_outline(loc: Located, *, with_code: bool=False, with_lines: bool=True, children: list[OutlineNode] | None=None) -> OutlineNode:
    """Build an :class:`OutlineNode` describing ``loc`` (source only if ``with_code``, lines only if ``with_lines``).

    ``signature``/``docstring`` are only computed when ``code`` is not, since the
    full source already makes them visible.
    """
    engine = loc.tree.engine
    if with_code:
        signature = docstring = None
        code = engine.node_code(loc.node)
    else:
        signature = engine.signature(loc.node) if engine.is_definition(loc.node_type) else None
        docstring = engine.docstring(loc.node)
        code = None
    return OutlineNode(
        id=loc.node_id,
        type=loc.node_type,
        lines=line_range(loc) if with_lines else None,
        signature=signature,
        docstring=docstring,
        code=code,
        children=children or [])

def _compact(value: Any) -> Any:
    """Recursively drop ``None`` values and empty lists from a dataclass-derived structure."""
    if isinstance(value, dict):
        return {k: _compact(v) for k, v in value.items() if v is not None and v != []}
    if isinstance(value, list):
        return [_compact(v) for v in value]
    return value

def to_dict(node: OutlineNode) -> dict:
    """Serialize an :class:`OutlineNode` to MCP output, omitting empty fields."""
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
    """Build the nested outline of ``located`` (source only if ``with_code``, lines only if ``with_lines``).

    Non-expandable nodes (no nested defs worth descending into) are rendered with
    their full source instead of being fragmented into ``children``.
    """
    return _outline_nodes(_build_forest(located), with_code=with_code, with_lines=with_lines)

def _outline_nodes(nodes: list['_TreeNode'], *, with_code: bool, with_lines: bool=True) -> list[OutlineNode]:
    """Convert a forest into OutlineNodes, collapsing non-expandable nodes to full source instead of ``children``."""
    result: list[OutlineNode] = []
    for t in nodes:
        if t.loc.expandable and t.children:
            result.append(
                node_outline(
                    t.loc,
                    with_code=False,
                    with_lines=with_lines,
                    children=_outline_nodes(
                        t.children,
                        with_code=with_code,
                        with_lines=with_lines)))
        else:
            result.append(node_outline(t.loc, with_code=with_code, with_lines=with_lines))
    return result

def _resolve_by_name(key: str, by_name: dict[str, list['_TreeNode']]) -> tuple['_TreeNode | None', str | None]:
    """Resolve ``key`` against node names when it doesn't match an id directly.

    Tries an exact name match first (agents commonly pass a function/class name
    instead of its full id), then a single sufficiently close fuzzy match. The
    fuzzy cutoff scales with ``key``'s length so short names still require a
    near-exact match. Returns ``(None, reason)`` with a human-readable reason
    when a match exists but is ambiguous, or ``(None, None)`` when nothing is
    close enough.
    """
    exact = by_name.get(key)
    if exact:
        if len(exact) == 1:
            return (exact[0], None)
        return (None, f"'{key}' matches {len(exact)} nodes by name; use a specific id.")
    if not by_name:
        return (None, None)
    cutoff = 0.5 + min(0.35, 1.4 / max(len(key), 1))
    scored = sorted(((difflib.SequenceMatcher(None, key, name).ratio(), name) for name in by_name), reverse=True)
    best_score, best_name = scored[0]
    if best_score < cutoff:
        return (None, None)
    if len(scored) > 1 and scored[1][0] == best_score:
        return (None, f"'{key}' is ambiguous between similarly named nodes; use a specific id.")
    candidates = by_name[best_name]
    if len(candidates) != 1:
        return (None, f"'{key}' matches {len(candidates)} nodes named '{best_name}'; use a specific id.")
    return (candidates[0], None)

def read_subtrees(located: list[Located], keys: list[str], *, with_lines: bool=True) -> tuple[list[OutlineNode], list[str]]:
    """Return one read subtree per resolvable ``keys`` entry.

    Each key is matched, in order, by exact id, then by exact node name, then by
    a conservative fuzzy match on node name. Keys that cannot be resolved (or are
    ambiguous) are reported in the returned error list instead of aborting the
    whole read.

    Returns:
        Tuple of (subtrees for resolved keys, error messages for unresolved keys).
    """
    index: dict[str, _TreeNode] = {}
    by_name: dict[str, list[_TreeNode]] = {}

    def collect(nodes: list[_TreeNode]) -> None:
        for t in nodes:
            index.setdefault(t.loc.node_id, t)
            if t.loc.name:
                by_name.setdefault(t.loc.name, []).append(t)
            collect(t.children)
    collect(_build_forest(located))
    nodes: list[OutlineNode] = []
    errors: list[str] = []
    for key in keys:
        target = index.get(key)
        error: str | None = None
        if target is None:
            target, error = _resolve_by_name(key, by_name)
        if target is None:
            errors.append(error or f"No node matched '{key}'.")
            continue
        nodes.append(_outline_nodes([target], with_code=True, with_lines=with_lines)[0])
    return (nodes, errors)

def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> bool:
    if id is not None and loc.node_id != id:
        return False
    if node_type is not None and loc.node_type.lower() != node_type.lower():
        return False
    if name is not None and loc.name != name:
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
    '#: Whether ``validate``/``replace`` reliably reject malformed edits. Only then'
    '#: may callers rely on re-parse to catch corruption (false for markup grammars'
    '#: whose parser accepts almost any text without reporting errors).'
    validates_syntax: bool = False

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

    def is_definition(self, node_type: str) -> bool:
        """Whether ``node_type`` is "def-like" enough for a ``signature`` to make
        sense (as opposed to e.g. a statement/import segment or a Markdown
        paragraph). Engines with a precise, known node-type set (see
        :class:`xy.ai.mcpc.tools.ast.generic._java.JavaEngine`) should override
        this instead of relying on the substring-matching default."""
        return bool(_SIGNATURE_TYPE_RE.search(node_type))

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
OUTLINE_NODE_SCHEMA = {
    'type': 'object',
    'properties': {
        'id': {
            'type': 'string',
            'description': 'Unique, primarily name-based node id (numeric fallback for nameless segments); the sole address for every tool.'},
        'type': {
            'type': 'string'},
        'lines': {
            'type': 'string',
                    'description': "Line number, or 'start-end' if the node spans multiple lines; omitted unless the 'tools' or 'edit-lines' tool is enabled in the session."},
        'signature': {
            'type': 'string',
            'description': 'One-line header for class/function nodes; omitted when code is included.'},
        'docstring': {
            'type': 'string',
            'description': 'Omitted when code is included.'},
        'code': {
            'type': 'string',
            'description': 'Full node source; populated by find/read, omitted in list.'},
        'children': {
            'type': 'array',
            'items': {
                '$ref': '#/$defs/outline_node'}}},
    'required': [
        'id',
        'type']}