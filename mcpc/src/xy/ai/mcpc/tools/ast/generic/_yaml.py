"""YAML tree-sitter engine: leaves collapsed to a size budget instead of the full grammar depth.

Tree-sitter's own YAML tree exposes every scalar, flow wrapper ('flow_node',
'block_node', ...) and punctuation token as its own node, which is far too
fine-grained to address individually on large files. This engine instead
rebuilds the tree from 'block_mapping_pair' / 'flow_pair' / 'block_sequence_item'
units - transparently unwrapping the purely structural 'document', 'block_node',
'flow_node', 'block_mapping', 'block_sequence', 'flow_mapping' and
'flow_sequence' containers in between (see :func:`_yaml_units`) - and collapses
each unit into a childless leaf as soon as its full text fits
:data:`_YAML_LEAF_LIMIT`, only recursing into its own nested units when it
doesn't (see :func:`_yaml_collapse`).
"""
from __future__ import annotations
from typing import Any
from xy.ai.mcpc.tools.ast.base import Located, Tree, id_segment
from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _RootHolder
__all__ = ['YamlEngine']
"#: Max size (characters) a collapsed leaf's full text may still have."
_YAML_LEAF_LIMIT = 1500
'#: Node types individually addressable - a mapping entry, a flow-mapping entry'
"#: or a sequence item; everything else is either a leaf's inner detail or one"
'#: of the wrapper types below.'
_YAML_UNIT_TYPES = {'block_mapping_pair', 'flow_pair', 'block_sequence_item'}
'#: Purely structural nodes, transparently unwrapped on the way to the next unit.'
_YAML_WRAPPER_TYPES = {
    'document',
    'block_node',
    'flow_node',
    'block_mapping',
    'block_sequence',
    'flow_mapping',
    'flow_sequence'}
_YAML_ADDRESSABLE_TYPES = _YAML_UNIT_TYPES | {'document'}

def _yaml_units(node: Any) -> list[Any]:
    """Direct logical children of ``node``: its own unit-type descendants, found by
    transparently unwrapping wrapper nodes (comments/anchors/punctuation in between are dropped)."""
    units: list[Any] = []
    for child in node.named_children:
        if child.type in _YAML_UNIT_TYPES:
            units.append(child)
        elif child.type in _YAML_WRAPPER_TYPES:
            units.extend(_yaml_units(child))
    return units

class _YamlUnit:
    """One collapsed YAML unit: either a childless leaf (its full text already fits
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

    def child_by_field_name(self, field: str) -> Any:
        """Delegate to the wrapped native node, so key/value based naming keeps working."""
        return self._node.child_by_field_name(field)

def _yaml_collapse(node: Any, source: bytes) -> _YamlUnit:
    """Collapse ``node`` into a leaf if its full text fits the budget (or it has no
    nested unit to expand into), else into a branch over its own collapsed sub-units."""
    sub_units = _yaml_units(node)
    size = len(node.text.decode('utf-8', 'replace'))
    children = [] if size <= _YAML_LEAF_LIMIT or not sub_units else [_yaml_collapse(unit, source) for unit in sub_units]
    return _YamlUnit(node, children, source)

def _yaml_root_children(root_node: Any, source: bytes) -> list[Any]:
    """Top-level children of a YAML file: one leaf/branch per unit of the (single)
    document, or one per '---'-separated document in a multi-document stream."""
    documents = [child for child in root_node.named_children if child.type == 'document']
    if len(documents) > 1:
        return [_yaml_collapse(document, source) for document in documents]
    if not documents:
        return []
    units = _yaml_units(documents[0])
    if units:
        return [_yaml_collapse(unit, source) for unit in units]
    return [_yaml_collapse(documents[0], source)] if documents[0].named_children else []

class YamlEngine(TreeSitterEngine):
    """Tree-sitter YAML restructured into the size-collapsed leaves above."""

    def __init__(self) -> None:
        super().__init__('yaml')

    def locate_all(self, tree: Tree) -> list[Located]:
        root = _RootHolder(_yaml_root_children(tree.raw.root_node, tree.source.encode('utf-8')))
        results: list[Located] = []

        def walk(node: Any, path: str, depth: int) -> None:
            used: dict[str, int] = {}
            for index, child in enumerate(node.named_children):
                if depth > 0 and child.type not in _YAML_ADDRESSABLE_TYPES:
                    continue
                name = self._name(child)
                '# Sequence items have no natural name: a stable positional index is'
                '# preferable to a content hash, which would move on every edit elsewhere.'
                content = None if child.type == 'block_sequence_item' else self.node_code(child)
                seg = id_segment(name, index, used, content=content)
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