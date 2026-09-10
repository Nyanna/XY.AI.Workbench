"""Shared child-edge iteration over the IR node graph (naming-only helper).

Mirrors the child shapes already used by identity/dedup.py, but yields plain
(label, Edge) pairs instead of rebuilding nodes -- naming only ever reads.
"""

from xy.cgen.model.nodes import CompositionNode, DictionaryNode, ListNode, ObjectNode


def iter_child_edges(node):
    """Yield (label, edge) for every direct child edge of `node`.

    Leaf kinds (Ref/Primitive/Enum/AnyDictionary/Unsupported) yield nothing.
    """
    if isinstance(node, ObjectNode):
        for edge in node.properties:
            yield edge.label, edge
    elif isinstance(node, ListNode):
        for edge in node.elements:
            yield edge.label, edge
    elif isinstance(node, DictionaryNode):
        yield node.value.label, node.value
    elif isinstance(node, CompositionNode):
        for edge in node.branches:
            yield edge.label, edge
