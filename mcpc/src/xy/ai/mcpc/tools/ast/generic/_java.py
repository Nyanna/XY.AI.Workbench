"""Java tree-sitter engine: Classes/Interfaces/Enums -> Methods, mirroring the
Python engine's shape (def-like nodes are individually addressable, everything
else collapses into 'imports'/'statements' segments capped at
``SEGMENT_MAX_CHARS``) since the plain native tree-sitter grammar (every
expression/identifier addressable) is far too fine-grained to be useful.
"""
from __future__ import annotations
from typing import Any
from xy.ai.mcpc.tools.ast.base import SEGMENT_MAX_CHARS, Located, Tree, id_segment
from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _SynthNode
__all__ = ['JavaEngine']
'#: Individually addressable "def-like" node types (types + methods/constructors).'
_TYPE_DEF_TYPES = {
    'class_declaration',
    'interface_declaration',
    'enum_declaration',
    'record_declaration',
    'annotation_type_declaration'}
_DEF_TYPES = _TYPE_DEF_TYPES | {'method_declaration', 'constructor_declaration'}
_IMPORT_TYPES = {'package_declaration', 'import_declaration'}
"#: A type declaration's own body-container child, whose children are its members."
_BODY_TYPES = {'class_body', 'interface_body', 'annotation_type_body'}

def _body_of(def_node: Any) -> Any | None:
    for child in def_node.named_children:
        if child.type in _BODY_TYPES:
            return child
    return None

class JavaEngine(TreeSitterEngine):
    """Tree-sitter Java restructured like the Python engine: types/methods as
    real nodes, everything else grouped into statement/import segments."""

    def __init__(self) -> None:
        super().__init__('java')

    def is_definition(self, node_type: str) -> bool:
        return node_type in _DEF_TYPES

    def locate_all(self, tree: Tree) -> list[Located]:
        results: list[Located] = []
        source = tree.source.encode('utf-8')

        def walk(children: list[Any], container: Any, path: str) -> None:
            used: dict[str, int] = {}
            i, n = (0, len(children))
            while i < n:
                node = children[i]
                if node.type in _DEF_TYPES:
                    name = self._name(node)
                    seg = id_segment(name, i, used)
                    nid = f'{path}.{seg}' if path else seg
                    body = _body_of(node) if node.type in _TYPE_DEF_TYPES else None
                    expandable = bool(body) and any((c.type in _DEF_TYPES for c in body.named_children))
                    results.append(
                        Located(
                            tree=tree,
                            node=node,
                            parent=container,
                            index=i,
                            node_id=nid,
                            node_type=node.type,
                            name=name,
                            lineno=node.start_point[0] + 1,
                            end_lineno=node.end_point[0] + 1,
                            parent_type=container.type,
                            expandable=expandable))
                    if body is not None:
                        walk(body.named_children, body, nid)
                    i += 1
                    continue
                start = i
                kind = 'imports' if node.type in _IMPORT_TYPES else 'statements'
                length = 0
                while i < n:
                    current = children[i]
                    if current.type in _DEF_TYPES:
                        break
                    current_kind = 'imports' if current.type in _IMPORT_TYPES else 'statements'
                    if current_kind != kind:
                        break
                    piece = current.end_byte - current.start_byte
                    if i > start and length + piece > SEGMENT_MAX_CHARS:
                        break
                    length += piece
                    i += 1
                group = _SynthNode(kind, children[start:i], source)
                seg = id_segment(None, start, used, content=self.node_code(group))
                nid = f'{path}.{seg}' if path else seg
                results.append(
                    Located(
                        tree=tree,
                        node=group,
                        parent=container,
                        index=start,
                        node_id=nid,
                        node_type=kind,
                        name=None,
                        lineno=group.start_point[0] + 1,
                        end_lineno=group.end_point[0] + 1,
                        parent_type=container.type,
                        expandable=False))
        walk(tree.raw.root_node.named_children, tree.raw.root_node, '')
        return results