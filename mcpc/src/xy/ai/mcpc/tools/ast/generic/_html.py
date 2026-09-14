"""HTML tree-sitter engine: elements collapsed to a size budget, mirroring
:mod:`xy.ai.mcpc.tools.ast.generic._yaml` - unlike Java's fixed def-node types,
an HTML document's addressable unit (an 'element') nests to arbitrary, content-
dependent depth (a '<li>' vs. a '<body>' wrapping a whole page), so there is
no fixed "definition" level to cut at. Every element/script/style/comment/
doctype collapses into a childless leaf (its full text already fits
:data:`_HTML_LEAF_LIMIT`, see :func:`_html_collapse`) unless still too large,
in which case it becomes a branch over its own (recursively collapsed) child
elements - attributes/text content never get their own node, they're too fine-
grained to be independently useful.
"""
from __future__ import annotations
from typing import Any
from xy.ai.mcpc.tools.ast.base import Located, Tree, id_segment
from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _RootHolder
__all__ = ['HtmlEngine']
"#: Max size (characters) a collapsed leaf's full text may still have."
_HTML_LEAF_LIMIT = 1500
'#: Individually addressable node types - everything with its own start/end tag,'
'#: plus comments/doctype, which are already always leaves.'
_HTML_UNIT_TYPES = {'element', 'script_element', 'style_element', 'comment', 'doctype'}

def _html_units(node: Any) -> list[Any]:
    return [child for child in node.named_children if child.type in _HTML_UNIT_TYPES]

class _HtmlUnit:
    """One collapsed HTML unit: either a childless leaf (its full text already fits
    the leaf budget, or it has no nested unit to expand into anyway) or a branch
    whose children are its own recursively collapsed sub-units."""
    __slots__ = ('type', 'named_children', 'start_byte', 'end_byte', 'start_point', 'end_point', '_node', '_source')

    def __init__(self, node: Any, children: list[Any], source: bytes) -> None:
        self.type = node.type
        self.named_children = children
        self.start_byte = node.start_byte
        self.end_byte = node.end_byte
        self.start_point = node.start_point
        self.end_point = node.end_point
        self._node = node
        self._source = source

    @property
    def text(self) -> bytes:
        return self._source[self.start_byte:self.end_byte]

    @property
    def start_tag(self) -> Any | None:
        """The wrapped native node's own 'start_tag' child (tag name + attributes),
        looked up on the *native* tree - never among the collapsed unit children,
        which only ever hold nested elements."""
        return next((c for c in self._node.named_children if c.type == 'start_tag'), None)

def _html_collapse(node: Any, source: bytes) -> _HtmlUnit:
    """Collapse ``node`` into a leaf if its full text fits the budget (or it has no
    nested unit to expand into), else into a branch over its own collapsed sub-units."""
    sub_units = _html_units(node)
    size = len(node.text.decode('utf-8', 'replace'))
    children = [] if size <= _HTML_LEAF_LIMIT or not sub_units else [_html_collapse(unit, source) for unit in sub_units]
    return _HtmlUnit(node, children, source)

def _html_root_children(root_node: Any, source: bytes) -> list[Any]:
    """Top-level children of an HTML file: the doctype plus the root element(s)."""
    return [_html_collapse(unit, source) for unit in _html_units(root_node)]

def _html_attr(start_tag: Any, attr_name: str) -> str | None:
    """Value of ``attr_name`` on ``start_tag``, or ``None`` if absent/valueless."""
    for attr in start_tag.named_children:
        if attr.type != 'attribute':
            continue
        name_node = next((c for c in attr.named_children if c.type == 'attribute_name'), None)
        if name_node is None or name_node.text.decode('utf-8', 'replace') != attr_name:
            continue
        value_node = next((c for c in attr.named_children if c.type in (
            'quoted_attribute_value', 'attribute_value')), None)
        if value_node is not None and value_node.type == 'quoted_attribute_value':
            value_node = next((c for c in value_node.named_children if c.type == 'attribute_value'), None)
        return value_node.text.decode('utf-8', 'replace').strip() if value_node is not None else ''
    return None

class HtmlEngine(TreeSitterEngine):
    """Tree-sitter HTML restructured into the size-collapsed leaves above."""

    def __init__(self) -> None:
        super().__init__('html')

    def is_definition(self, node_type: str) -> bool:
        return node_type in _HTML_UNIT_TYPES

    def signature(self, node: Any, limit: int=80) -> str:
        start_tag = node.start_tag if isinstance(node, _HtmlUnit) else None
        raw = start_tag.text if start_tag is not None else node.text
        text = ' '.join(raw.decode('utf-8', 'replace').split())
        return text if len(text) <= limit else text[:limit - 1] + '…'

    def _html_name(self, node: Any) -> str | None:
        start_tag = node.start_tag if isinstance(node, _HtmlUnit) else None
        if start_tag is None:
            return None
        tag_node = next((c for c in start_tag.named_children if c.type == 'tag_name'), None)
        if tag_node is None:
            return None
        tag = tag_node.text.decode('utf-8', 'replace')
        ident = _html_attr(start_tag, 'id')
        if ident:
            return f'{tag}#{ident}'
        css_class = _html_attr(start_tag, 'class')
        if css_class:
            return f'{tag}.{css_class.split()[0]}'
        return tag

    def locate_all(self, tree: Tree) -> list[Located]:
        root = _RootHolder(_html_root_children(tree.raw.root_node, tree.source.encode('utf-8')))
        results: list[Located] = []

        def walk(node: Any, path: str, depth: int) -> None:
            used: dict[str, int] = {}
            for index, child in enumerate(node.named_children):
                name = self._html_name(child)
                seg = id_segment(name, index, used, content=self.node_code(child))
                nid = f'{path}.{seg}' if path else seg
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
                        expandable=bool(
                            child.named_children)))
                walk(child, nid, depth + 1)
        walk(root, '', 0)
        return results