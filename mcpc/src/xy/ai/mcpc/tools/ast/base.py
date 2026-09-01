"""Engine-agnostic model shared by every ``ast_*`` tool.

The tools address nodes by *selector* (id, type, name, qualified name, line
range or parent type) and never touch a concrete parser. Two engines implement
:class:`Engine`: a Python one built on the standard-library ``ast`` module and a
generic tree-sitter one for every other language/format. :mod:`.core` picks the
engine per file extension and exposes a thin facade the tools call.

A :class:`Tree` carries a back-reference to the engine that produced it, so
every helper here can dispatch to the right engine without the tools knowing
which one is in play.
"""


from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
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

    engine: "Engine"
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
        node_id: Stable dotted index path from the root, e.g. ``"3.1"``.
        node_type: Engine-reported node type name.
        name: Simple name, if the node carries one.
        qualified_name: Dotted path of enclosing names, if any.
        lineno / end_lineno: 1-based inclusive line span.
        parent_type: Type name of ``parent``, or ``None`` at the top level.
    """

    tree: Tree
    node: Any
    parent: Any
    index: int
    node_id: str
    node_type: str
    name: str | None
    qualified_name: str | None
    lineno: int
    end_lineno: int
    parent_type: str | None


@dataclass(frozen=True)
class OutlineNode:
    """One node in a structural (outline/list/find) result."""

    type: str
    qualified_name: str | None
    lines: str
    signature: str
    docstring: str | None
    children: list["OutlineNode"] = field(default_factory=list)


@dataclass(frozen=True)
class ReadNode:
    """One node in a subtree read for block-wise edit/replace.

    ``code`` holds the node's full source unless it is a pure container of
    nested addressable nodes, in which case it is ``None`` and ``children`` is
    populated so the agent can descend to the innermost editable block.
    """

    type: str
    qualified_name: str | None
    lines: str
    code: str | None
    children: list["ReadNode"] = field(default_factory=list)


def line_range(loc: Located) -> str:
    """Return ``loc``'s start line, or a ``"start-end"`` range if it spans several."""
    if loc.end_lineno == loc.lineno:
        return str(loc.lineno)
    return f"{loc.lineno}-{loc.end_lineno}"


def node_outline(loc: Located) -> OutlineNode:
    """Build a flat (childless) :class:`OutlineNode` describing ``loc``."""
    engine = loc.tree.engine
    return OutlineNode(
        type=loc.node_type,
        qualified_name=loc.qualified_name,
        lines=line_range(loc),
        signature=engine.signature(loc.node),
        docstring=engine.docstring(loc.node),
    )


def matches(
    loc: Located,
    *,
    id: str | None = None,
    node_type: str | None = None,
    name: str | None = None,
    qualified_name: str | None = None,
    lineno: int | None = None,
    end_lineno: int | None = None,
    parent_type: str | None = None,
) -> bool:
    if id is not None and loc.node_id != id:
        return False
    if node_type is not None and loc.node_type.lower() != node_type.lower():
        return False
    if name is not None and loc.name != name:
        return False
    if qualified_name is not None and loc.qualified_name != qualified_name:
        return False
    if lineno is not None and loc.lineno != lineno:
        return False
    if end_lineno is not None and loc.end_lineno != end_lineno:
        return False
    if parent_type is not None and (loc.parent_type or "").lower() != parent_type.lower():
        return False
    return True


def find(tree: Tree, **filters: object) -> list[Located]:
    active = {k: v for k, v in filters.items() if v is not None}
    return [loc for loc in tree.engine.locate_all(tree) if matches(loc, **active)]  # type: ignore[arg-type]


class Engine(ABC):
    """A parser back-end turning source into an addressable, mutable tree.

    Structural mutations differ fundamentally between back-ends: the Python
    engine edits the ``ast`` object graph and re-serialises it via ``unparse``,
    whereas generic engines splice source text at node byte-ranges and re-parse.
    Both, however, expose the same node-oriented operations below.
    """

    #: Human-readable engine name (used e.g. to guard Python-only tools).
    name: str = "engine"

    @abstractmethod
    def parse(self, source: str, path: Path | None = None) -> Tree:
        """Parse ``source`` into a :class:`Tree`, raising :class:`AstError` on error."""

    @abstractmethod
    def empty_tree(self, path: Path | None = None) -> Tree:
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
    def outline_nodes(self, tree: Tree) -> list[OutlineNode]:
        """Build the nested structural outline of ``tree``."""

    @abstractmethod
    def read_node(self, loc: Located) -> ReadNode:
        """Read ``loc``'s subtree, expanding pure containers into children."""

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


def require_path(path_str: str, *, must_exist: bool = True) -> Path:
    """Validate a mandatory absolute path, raising :class:`AstError` on failure."""
    path = Path(path_str)
    if not path.is_absolute():
        raise AstError("Path must be absolute.")
    if must_exist:
        if not path.exists():
            raise AstError("File not found.")
        if not path.is_file():
            raise AstError("Not a regular file.")
    return path


#: JSON-Schema fragment for :class:`OutlineNode`, shared by outline/list/find.
OUTLINE_NODE_SCHEMA = {
    "type": "object",
    "properties": {
        "type": {"type": "string"},
        "qualified_name": {"type": ["string", "null"]},
        "lines": {
            "type": "string",
            "description": "Line number, or 'start-end' if the node spans multiple lines.",
        },
        "signature": {"type": "string"},
        "docstring": {"type": ["string", "null"]},
        "children": {"type": "array", "items": {"$ref": "#/$defs/outline_node"}},
    },
    "required": ["type", "qualified_name", "lines", "signature", "docstring", "children"],
}
