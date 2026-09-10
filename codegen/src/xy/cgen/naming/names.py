"""Class-name derivation for the shared type graph and the per-operation transport roots.

Rules:
- Named schema -> its key, sanitized. Kept even when the node is a composition.
- Inline composition -> <Keyword><Branch1><Branch2>...; a branch is the target
  ref/primitive name, else (any other anonymous kind) '<Composite>Part<n>'
  (n = 1-based branch index; <Composite> = nearest enclosing named/class name).
- Plain anonymous object outside a composition branch -> '<Composite>Part<n>'
  too, n = 1-based position among its parent's child edges.
- List -> '<Element>List' (structural element name only, never key-based).
- Enum: named -> key; inline -> PascalCase(values...)+'Enum', deterministic order.
- Dictionary -> '<Value>Dictionary'; AnyDictionary -> literal 'AnyDictionary'.
- RequestNode: $ref body -> the named schema's own name IS the request root.
  Inline/anonymous body -> synthetic '<Path>Request<Method>'.
- ResponseNode root: always synthetic '<Path>Response'.
- CodeNode/ContentTypeView: always synthetic '<Path>ResponseCode<code><Ct>'; the
  body payload underneath is named independently (context = that synthetic name).

Name derivation never depends on discovery order for a given input document:
named_nodes/operations are walked in a fixed order, so two runs over the same
schema produce identical names.
"""

from dataclasses import dataclass, field

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
from xy.cgen.naming.identifiers import class_identifier, content_type_short_name, sanitize_identifier, to_pascal_case
from xy.cgen.naming.paths import method_to_class_fragment, path_to_class_fragment

PRIMITIVE_BRANCH_NAME = {
    "string": "String",
    "integer": "Integer",
    "number": "Number",
    "boolean": "Boolean",
    "null": "Null",
}
COMPOSITION_KEYWORD_NAME = {"allOf": "AllOf", "anyOf": "AnyOf", "oneOf": "OneOf"}
ENUM_LABEL_MAX_LEN = 48


@dataclass(frozen=True)
class ClassNames:
    """Raw (pre-collision) class names.

    - `named`: schema key -> class name.
    - `anonymous`: id(node) -> class name, for anonymous shared-type nodes.
    - `anonymous_nodes`: id(node) -> node object, same keys as `anonymous`.
    - `transport`: id(node) -> class name, for per-operation RequestNode/
      ResponseNode/CodeNode/ContentTypeView instances (never shared/deduped,
      so identity by id() is safe and stable within one generator run).
    - `transport_nodes`: id(node) -> node object, same keys as `transport`.
    """

    named: dict = field(default_factory=dict)
    anonymous: dict = field(default_factory=dict)
    anonymous_nodes: dict = field(default_factory=dict)
    transport: dict = field(default_factory=dict)
    transport_nodes: dict = field(default_factory=dict)


def derive_class_names(identified_model) -> ClassNames:
    """Assign a structural class name to every named schema, anonymous node,
    and per-operation transport root."""
    named = {key: class_identifier(key) for key in identified_model.named_nodes}
    anon_memo: dict = {}
    anon_nodes: dict = {}
    transport: dict = {}
    transport_nodes: dict = {}

    def resolve(node, context, index=None, force_name=None):
        if isinstance(node, RefNode):
            return named[node.name]
        if isinstance(node, PrimitiveNode):
            return PRIMITIVE_BRANCH_NAME[node.primitive_type]
        cached = anon_memo.get(id(node))
        if cached is not None:
            return cached
        if force_name is not None:
            name = force_name
            anon_memo[id(node)] = name
            anon_nodes[id(node)] = node
            _walk_children(node, name, resolve, branch_name)
            return name
        name = _derive(node, context, index, resolve, branch_name)
        anon_memo[id(node)] = name
        anon_nodes[id(node)] = node
        if isinstance(node, ObjectNode):
            _walk_children(node, name, resolve, branch_name)
        return name

    def branch_name(target, context, index):
        if isinstance(target, RefNode):
            return named[target.name]
        if isinstance(target, PrimitiveNode):
            return PRIMITIVE_BRANCH_NAME[target.primitive_type]
        return resolve(target, context, index, force_name=f"{context}Part{index}")

    for key, node in identified_model.named_nodes.items():
        _walk_children(node, named[key], resolve, branch_name)

    for operation_model in identified_model.operations:
        _name_operation(operation_model, resolve, transport, transport_nodes)

    return ClassNames(
        named=named,
        anonymous=anon_memo,
        anonymous_nodes=anon_nodes,
        transport=transport,
        transport_nodes=transport_nodes,
    )


def _derive(node, context, index, resolve, branch_name):
    if isinstance(node, EnumNode):
        return _enum_class_name(node)
    if isinstance(node, ListNode):
        parts = [resolve(edge.target, context, i) for i, edge in enumerate(node.elements, 1)]
        return "".join(parts) + "List"
    if isinstance(node, DictionaryNode):
        return resolve(node.value.target, context, 1) + "Dictionary"
    if isinstance(node, AnyDictionaryNode):
        return "AnyDictionary"
    if isinstance(node, CompositionNode):
        parts = [branch_name(edge.target, context, i) for i, edge in enumerate(node.branches, 1)]
        return COMPOSITION_KEYWORD_NAME[node.keyword] + "".join(parts)
    if isinstance(node, ObjectNode):
        return f"{context}Part{index}"
    if isinstance(node, UnsupportedNode):
        return "Unsupported"
    raise TypeError(f"cannot name node kind: {node.kind!r}")


def _walk_children(node, context, resolve, branch_name):
    """Visit direct children needing their own name, using `context` as the new prefix.

    Only reachable for ObjectNode (rebase point) and named-schema entry points;
    List/Dictionary/Composition children are already resolved while building
    their parent's own name string (see _derive), so they are not re-walked here.
    """
    if isinstance(node, ObjectNode):
        for i, edge in enumerate(node.properties, 1):
            resolve(edge.target, context, i)
    elif isinstance(node, ListNode):
        for i, edge in enumerate(node.elements, 1):
            resolve(edge.target, context, i)
    elif isinstance(node, DictionaryNode):
        resolve(node.value.target, context, 1)
    elif isinstance(node, CompositionNode):
        for i, edge in enumerate(node.branches, 1):
            branch_name(edge.target, context, i)


def _name_operation(operation_model, resolve, transport: dict, transport_nodes: dict) -> None:
    operation = operation_model.operation
    path_fragment = path_to_class_fragment(operation.path)

    request_node = operation_model.request
    if request_node is not None and request_node.body is not None:
        context = f"{path_fragment}Request{method_to_class_fragment(operation.method)}"
        target = request_node.body.target
        force = None if isinstance(target, RefNode) else context
        transport[id(request_node)] = resolve(target, context, force_name=force)
        transport_nodes[id(request_node)] = request_node

    response_node = operation_model.response
    response_context = f"{path_fragment}Response"
    transport[id(response_node)] = response_context
    transport_nodes[id(response_node)] = response_node
    for code_node in response_node.codes:
        for content_type_view in code_node.content_types:
            code_context = (
                f"{response_context}Code{code_node.status_code}"
                f"{to_pascal_case(content_type_short_name(content_type_view.content_type))}"
            )
            transport[id(code_node)] = code_context
            transport_nodes[id(code_node)] = code_node
            transport[id(content_type_view)] = code_context
            transport_nodes[id(content_type_view)] = content_type_view
            resolve(content_type_view.body.target, code_context)


def _enum_class_name(node: EnumNode) -> str:
    tokens = _sorted_enum_tokens(node.values)
    label = "".join(tokens) or "Empty"
    if len(label) > ENUM_LABEL_MAX_LEN:
        label = _cap_enum_label(tokens)
    return sanitize_identifier(label) + "Enum"


def _sorted_enum_tokens(values: tuple) -> list:
    """Enum values are a set (identity), sorted by (type, repr) for determinism."""
    pairs = sorted(((type(v).__name__, v) for v in values), key=lambda pair: (pair[0], repr(pair[1])))
    return [to_pascal_case(str(value)) for _, value in pairs]


def _cap_enum_label(tokens: list) -> str:
    """Keep leading tokens until the length budget is exhausted (deterministic cap)."""
    kept, length = [], 0
    for token in tokens:
        if kept and length + len(token) > ENUM_LABEL_MAX_LEN:
            break
        kept.append(token)
        length += len(token)
    return "".join(kept) or tokens[0][:ENUM_LABEL_MAX_LEN]
