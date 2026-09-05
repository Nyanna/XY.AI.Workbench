"""Generic tree-sitter back-end for every non-Python language/format.

Grammars come from ``tree_sitter_language_pack`` (~370 languages, compatible
with tree-sitter 0.26+). Unlike the Python engine there is no ``unparse``:
mutations are plain source-text operations - splice a node's byte-range, or
concatenate - followed by a re-parse, matching how these formats are edited
in practice.

Nodes are addressed engine-independently by a single dotted ``id`` over the
*named* child hierarchy, name/hash based (a Markdown heading always collapses
to a 6-char hex hash) or numeric where no name exists. By default every named
child is addressable, exposing the grammar's native structure as-is; a
language whose native tree doesn't match how it's actually edited overrides
:meth:`TreeSitterEngine.locate_all` in its own subclass (see
:mod:`xy.ai.mcpc.tools.ast.generic._markdown` for Markdown's Page/Heading
model) instead of special-casing this base engine.
"""
from __future__ import annotations
from pathlib import Path
from typing import Any, Callable
from tree_sitter_language_pack import get_parser
from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, Tree, id_segment
__all__ = ['TreeSitterEngine']
"#: Named child types that usually carry a node's identifier/key."
_NAME_TYPES = {
    'identifier',
    'property_identifier',
    'field_identifier',
    'type_identifier',
    'constant',
    'key',
    'string',
    'bare_key',
    'dotted_key',
    'flow_node',
    'plain_scalar',
    'tag'}

class _SynthNode:
    """Minimal tree-sitter-node stand-in for a node an engine subclass rebuilds
    itself (a contiguous run of real children treated as one unit)."""
    __slots__ = ('type', 'named_children', 'start_byte', 'end_byte', 'start_point', 'end_point', '_source')

    def __init__(self, node_type: str, children: list[Any], source: bytes) -> None:
        self.type = node_type
        self.named_children = children
        self.start_byte = children[0].start_byte
        self.end_byte = children[-1].end_byte
        self.start_point = children[0].start_point
        self.end_point = children[-1].end_point
        self._source = source

    @property
    def text(self) -> bytes:
        return self._source[self.start_byte:self.end_byte]

    @staticmethod
    def child_by_field_name(_field: str) -> None:
        return None

class _RootHolder:
    """Fake container so the shared node-walker can start from a plain child list."""
    __slots__ = ('named_children', 'type')

    def __init__(self, children: list[Any]) -> None:
        self.named_children = children
        self.type = None

class TreeSitterEngine(Engine):
    """One tree-sitter grammar exposed through the common :class:`Engine` API.

    Instances are per-language and cached by :func:`get_engine`; each mutation
    edits ``Tree.source`` and re-parses, so :meth:`serialize` just returns that
    text.
    """

    def __init__(self, symbol: str) -> None:
        self.symbol = symbol
        self.name = f'tree-sitter:{symbol}'
        self._parser = None

    def _parse(self, data: bytes):
        if self._parser is None:
            try:
                self._parser = get_parser(self.symbol)
            except Exception as exc:
                '# noqa: BLE001'
                raise AstError(f"Tree-sitter grammar '{self.symbol}' is unavailable.") from exc
        return self._parser.parse(data)

    def parse(self, source: str, path: Path | None=None) -> Tree:
        return Tree(self, self._parse(source.encode('utf-8')), source, path)

    def empty_tree(self, path: Path | None=None) -> Tree:
        return Tree(self, self._parse(b''), '', path)

    def serialize(self, tree: Tree) -> str:
        return tree.source

    def validate(self, source: str) -> str | None:
        root = self._parse(source.encode('utf-8')).root_node
        if not root.has_error:
            return None
        stack = [root]
        while stack:
            node = stack.pop()
            if node.type == 'ERROR' or node.is_missing:
                return f'Parse error near line {node.start_point[0] + 1}.'
            stack.extend(node.children)
        return 'Parse error.'

    def _name(self, node: Any) -> str | None:
        if node.type == 'section':
            for child in node.named_children:
                if child.type.endswith('heading'):
                    return self._clean_heading(child.text)
        for field in ('name', 'key', 'tag'):
            child = node.child_by_field_name(field)
            if child is not None:
                return self._clean(child.text)
        for child in node.named_children:
            if child.type in _NAME_TYPES:
                return self._clean(child.text)
        return None

    @staticmethod
    def _clean_heading(raw: bytes) -> str:
        return raw.decode('utf-8', 'replace').strip().lstrip('#').strip()

    @staticmethod
    def _clean(raw: bytes) -> str:
        return raw.decode('utf-8', 'replace').strip().strip('"\'')

    def locate_all(self, tree: Tree) -> list[Located]:
        return self._locate_from(tree, tree.raw.root_node, lambda child, depth: True)

    def _locate_from(self, tree: Tree, root: Any, addressable: Callable[[Any, int], bool]) -> list[Located]:
        """Shared node-walker: ``root`` and ``addressable`` let subclasses curate a
        different (possibly synthetic, see ``_markdown._RootHolder``) node set."""
        results: list[Located] = []

        def walk(node: Any, path: str, depth: int) -> None:
            used: dict[str, int] = {}
            for index, child in enumerate(node.named_children):
                if not addressable(child, depth):
                    continue
                is_hashed = child.type in ('section', 'page')
                name = self._name(child)
                seg = id_segment(name, index, used, hash_only=is_hashed, content=self.node_code(child))
                nid = f'{path}.{seg}' if path else seg
                addr_children = [c for c in child.named_children if addressable(c, depth + 1)]
                results.append(
                    Located(
                        tree=tree,
                        node=child,
                        parent=node,
                        index=index,
                        node_id=nid,
                        node_type=child.type,
                        name=name,
                        lineno=child.start_point[0] + 1,
                        end_lineno=child.end_point[0] + 1,
                        parent_type=node.type,
                        expandable=bool(addr_children)))
                walk(child, nid, depth + 1)
        walk(root, '', 0)
        return results

    def signature(self, node: Any, limit: int=80) -> str:
        first_line = node.text.decode('utf-8', 'replace').splitlines()[0] if node.text else ''
        first_line = first_line.strip()
        return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'

    def docstring(self, node: Any) -> str | None:
        return None

    def node_code(self, node: Any) -> str:
        return node.text.decode('utf-8', 'replace')

    def _splice(self, tree: Tree, start: int, end: int, text: str) -> None:
        data = tree.source.encode('utf-8')
        new = data[:start] + text.encode('utf-8') + data[end:]
        tree.source = new.decode('utf-8')
        tree.raw = self._parse(new)

    def replace(self, loc: Located, code: str) -> None:
        self._splice(loc.tree, loc.node.start_byte, loc.node.end_byte, code)

    def insert(self, loc: Located, code: str, position: str) -> int:
        if position == 'before':
            self._splice(loc.tree, loc.node.start_byte, loc.node.start_byte, code + '\n')
        else:
            self._splice(loc.tree, loc.node.end_byte, loc.node.end_byte, '\n' + code)
        return 1

    def delete(self, loc: Located) -> None:
        self._splice(loc.tree, loc.node.start_byte, loc.node.end_byte, '')

    def append(self, tree: Tree, code: str) -> int:
        sep = '' if not tree.source or tree.source.endswith('\n') else '\n'
        self._splice(tree, len(tree.source.encode('utf-8')), len(tree.source.encode('utf-8')), sep + code)
        return 1