"""XML tree-sitter engine: elements collapsed to a size budget, mirroring
:mod:`xy.ai.mcpc.tools.ast.generic._html` - unlike HTML the grammar wraps an
element's body in an anonymous 'content' node and a document's prolog in an
anonymous 'prolog' node, so :func:`_xml_units` drills through those container
types to find the actual addressable units (elements, comments, processing
instructions, CDATA sections, the doctype declaration). Every unit collapses
into a childless leaf (its full text already fits :data:`_XML_LEAF_LIMIT`, see
:func:`_xml_collapse`) unless still too large, in which case it becomes a
branch over its own (recursively collapsed) child units - attributes/text
content never get their own node, they're too fine-grained to be
independently useful.
"""
from __future__ import annotations
from typing import Any
from xy.ai.mcpc.tools.ast.base import Located, Tree, id_segment
from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _RootHolder
__all__ = ['XmlEngine']
"#: Max size (characters) a collapsed leaf's full text may still have."
_XML_LEAF_LIMIT = 1500
'#: Individually addressable node types.'
_XML_UNIT_TYPES = {'element', 'Comment', 'CDSect', 'PI', 'doctypedecl'}
'#: Anonymous wrapper types the grammar interposes between a node and its'
'#: actual unit children - drilled through, never addressable themselves.'
_XML_CONTAINER_TYPES = {'document', 'prolog', 'content'}

def _xml_units(node: Any) -> list[Any]:
    units = []
    for child in node.named_children:
        if child.type in _XML_UNIT_TYPES:
            units.append(child)
        elif child.type in _XML_CONTAINER_TYPES:
            units.extend(_xml_units(child))
    return units

class _XmlUnit:
    """One collapsed XML unit: either a childless leaf (its full text already fits
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
        """The wrapped native node's own 'STag'/'EmptyElemTag' child, looked up on
        the *native* tree - never among the collapsed unit children, which only
        ever hold nested elements."""
        return next((c for c in self._node.named_children if c.type in ('STag', 'EmptyElemTag')), None)

def _xml_collapse(node: Any, source: bytes) -> _XmlUnit:
    """Collapse ``node`` into a leaf if its full text fits the budget (or it has no
    nested unit to expand into), else into a branch over its own collapsed sub-units."""
    sub_units = _xml_units(node)
    size = len(node.text.decode('utf-8', 'replace'))
    children = [] if size <= _XML_LEAF_LIMIT or not sub_units else [_xml_collapse(unit, source) for unit in sub_units]
    return _XmlUnit(node, children, source)

def _xml_root_children(root_node: Any, source: bytes) -> list[Any]:
    """Top-level children of an XML file: prolog declarations (doctype, comments,
    processing instructions), the root element, and any trailing misc."""
    return [_xml_collapse(unit, source) for unit in _xml_units(root_node)]

def _xml_attr(start_tag: Any, attr_name: str) -> str | None:
    """Value of ``attr_name`` on ``start_tag``, or ``None`` if absent."""
    for attr in start_tag.named_children:
        if attr.type != 'Attribute':
            continue
        name_node = next((c for c in attr.named_children if c.type == 'Name'), None)
        if name_node is None or name_node.text.decode('utf-8', 'replace') != attr_name:
            continue
        value_node = next((c for c in attr.named_children if c.type == 'AttValue'), None)
        if value_node is None:
            return ''
        return value_node.text.decode('utf-8', 'replace')[1:-1].strip()
    return None

class XmlEngine(TreeSitterEngine):
    """Tree-sitter XML restructured into the size-collapsed leaves above."""

    def __init__(self) -> None:
        super().__init__('xml')

    def is_definition(self, node_type: str) -> bool:
        return node_type in _XML_UNIT_TYPES

    def signature(self, node: Any, limit: int=80) -> str:
        start_tag = node.start_tag if isinstance(node, _XmlUnit) else None
        raw = start_tag.text if start_tag is not None else node.text
        text = ' '.join(raw.decode('utf-8', 'replace').split())
        return text if len(text) <= limit else text[:limit - 1] + '…'

    def _xml_name(self, node: Any) -> str | None:
        start_tag = node.start_tag if isinstance(node, _XmlUnit) else None
        if start_tag is None:
            return None
        name_node = next((c for c in start_tag.named_children if c.type == 'Name'), None)
        if name_node is None:
            return None
        tag = name_node.text.decode('utf-8', 'replace')
        ident = _xml_attr(start_tag, 'id')
        return f'{tag}#{ident}' if ident else tag

    def locate_all(self, tree: Tree) -> list[Located]:
        root = _RootHolder(_xml_root_children(tree.raw.root_node, tree.source.encode('utf-8')))
        results: list[Located] = []

        def walk(node: Any, path: str, depth: int) -> None:
            used: dict[str, int] = {}
            for index, child in enumerate(node.named_children):
                name = self._xml_name(child)
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