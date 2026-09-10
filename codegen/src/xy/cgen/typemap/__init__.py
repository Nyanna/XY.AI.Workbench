"""Maps IR nodes to the Java type used at their use-site (getter/setter, list
element, dictionary value, ...). Used by naming/emit.

Primitives are always boxed (never `int`/`double`/...) since a field can be
absent or explicitly null. `format`/validators are ignored -- they never
affect the resolved type. `null` has no Java type of its own: absent vs.
explicit-null is a getter concern on the composition view that contains it
, never a standalone type.
"""

from xy.cgen.model.nodes import (
    AnyDictionaryNode,
    CompositionNode,
    DictionaryNode,
    EnumNode,
    ListNode,
    ObjectNode,
    PrimitiveNode,
    RefNode,
    UnsupportedNode,
)

JAVA_PRIMITIVE_TYPE = {
    "string": "String",
    "integer": "Long",
    "number": "Double",
    "boolean": "Boolean",
}

ANY_DICTIONARY_JAVA_TYPE = "com.fasterxml.jackson.databind.JsonNode"

# Node kinds whose Java type is the generated class assigned by naming.
_GENERATED_CLASS_KINDS = (EnumNode, ListNode, DictionaryNode, ObjectNode, CompositionNode)


def map_type(node, named_model) -> str:
    """Resolve the Java type for a given IR node.

    `named_model` supplies the generated class name/package for node kinds
    that are not primitives (RefNode resolves through the referenced named
    schema's own entry).
    """
    if isinstance(node, PrimitiveNode):
        return _map_primitive(node)
    if isinstance(node, AnyDictionaryNode):
        return ANY_DICTIONARY_JAVA_TYPE
    if isinstance(node, RefNode):
        return named_model.name_of_ref(node.name).fqn
    if isinstance(node, _GENERATED_CLASS_KINDS):
        return named_model.name_of(node).fqn
    if isinstance(node, UnsupportedNode):
        raise ValueError(f"unsupported node has no Java type (reason={node.reason!r})")
    raise TypeError(f"cannot map node kind to a Java type: {node.kind!r}")


def _map_primitive(node: PrimitiveNode) -> str:
    if node.primitive_type == "null":
        raise ValueError("'null' has no standalone Java type -- resolve via the enclosing composition view")
    return JAVA_PRIMITIVE_TYPE[node.primitive_type]
