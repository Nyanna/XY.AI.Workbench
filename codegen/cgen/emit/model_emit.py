"""Renders Java model classes from named IR nodes. No dependency on client emission.

One Jinja2 template per node kind: the generator never writes Java text
itself, it only builds a context and renders a template. Every rendered class
is a proxy holding a single JsonNode reference plus a statically known child
list derived from the IR -- no client dependency, no data copies.
"""

from pathlib import Path

from jinja2 import Environment, FileSystemLoader, StrictUndefined

from cgen.emit.io_context import json_support_fqn, request_root_node_ids
from cgen.emit.model_context import build_accessor, build_branches, enum_constants, enum_raw_type
from cgen.model.nodes import (
    AnyDictionaryNode,
    CompositionNode,
    DictionaryNode,
    EnumNode,
    ListNode,
    ObjectNode,
    RefNode,
)
from cgen.naming.traverse import iter_child_edges

TEMPLATES_DIR = Path(__file__).resolve().parent.parent / "templates"

_ENV = Environment(
    loader=FileSystemLoader(str(TEMPLATES_DIR)),
    trim_blocks=True,
    lstrip_blocks=True,
    keep_trailing_newline=True,
    undefined=StrictUndefined,
)

# Node kinds that become a generated model class. UnsupportedNode never gets a
# class; RefNode/PrimitiveNode are never classes. AnyDictionaryNode gets
# a (rarely used) raw-passthrough class too: map_type only collapses it to
# plain JsonNode at a *direct* use site -- a named additionalProperties:
# true schema reached through a RefNode still resolves to this class's own
# FQN, so it must exist and compile even though most call sites
# never construct it. CompositionNode is one node -> one proxy-view class
# over allOf/anyOf/oneOf branches.
_MODEL_CLASS_KINDS = (ObjectNode, ListNode, DictionaryNode, EnumNode, AnyDictionaryNode, CompositionNode)


def emit_model(model, writer) -> None:
    """Render and write one .java file per reachable Object/List/Dictionary/Enum/AnyDictionary node."""
    request_root_ids = request_root_node_ids(model)
    for node in _reachable_nodes(model):
        if not isinstance(node, _MODEL_CLASS_KINDS):
            continue
        name = model.name_of(node)
        if name is None:  # not a class (e.g. structurally unreachable, defensive)
            continue
        content = _render(node, name, model, id(node) in request_root_ids)
        relative_path = Path(*name.package.split(".")) / f"{name.class_name}.java"
        writer.write(relative_path, content)


def _render(node, name, model, is_request_root: bool) -> str:
    if isinstance(node, ObjectNode):
        return _render_object(node, name, model, is_request_root)
    if isinstance(node, ListNode):
        return _render_list(node, name, model, is_request_root)
    if isinstance(node, DictionaryNode):
        return _render_dictionary(node, name, model, is_request_root)
    if isinstance(node, EnumNode):
        # A bare top-level enum request body is not supported as a request root:
        # a Java enum cannot hold the bound JsonNode a toString()/fromString()
        # pair would need.
        return _render_enum(node, name)
    if isinstance(node, AnyDictionaryNode):
        return _render_any_dictionary(name, model, is_request_root)
    if isinstance(node, CompositionNode):
        return _render_composition(node, name, model, is_request_root)
    raise TypeError(f"no template for node kind: {node.kind!r}")  # pragma: no cover -- guarded by _MODEL_CLASS_KINDS


def _render_object(node: ObjectNode, name, model, is_request_root: bool) -> str:
    accessors = [a for a in (build_accessor(edge.label, edge, model) for edge in node.properties) if a is not None]
    template = _ENV.get_template("model/object.java.jinja")
    return template.render(
        package=name.package,
        class_name=name.class_name,
        accessors=accessors,
        is_request_root=is_request_root,
        json_support_fqn=json_support_fqn(model.base_package),
    )


def _render_list(node: ListNode, name, model, is_request_root: bool) -> str:
    if node.mixed:
        elements = [
            a for a in (build_accessor(edge.label, edge, model) for edge in node.elements) if a is not None
        ]
        template = _ENV.get_template("model/list_mixed.java.jinja")
        return template.render(
            package=name.package,
            class_name=name.class_name,
            elements=elements,
            is_request_root=is_request_root,
            json_support_fqn=json_support_fqn(model.base_package),
        )
    (element_edge,) = node.elements
    element = build_accessor(element_edge.label, element_edge, model)
    template = _ENV.get_template("model/list.java.jinja")
    return template.render(
        package=name.package,
        class_name=name.class_name,
        element=element,
        is_request_root=is_request_root,
        json_support_fqn=json_support_fqn(model.base_package),
    )


def _render_dictionary(node: DictionaryNode, name, model, is_request_root: bool) -> str:
    value = build_accessor(node.value.label, node.value, model)
    template = _ENV.get_template("model/dictionary.java.jinja")
    return template.render(
        package=name.package,
        class_name=name.class_name,
        value=value,
        is_request_root=is_request_root,
        json_support_fqn=json_support_fqn(model.base_package),
    )


def _render_enum(node: EnumNode, name) -> str:
    template = _ENV.get_template("model/enum.java.jinja")
    return template.render(
        package=name.package,
        class_name=name.class_name,
        primitive_type=node.primitive_type,
        raw_type=enum_raw_type(node),
        constants=enum_constants(node),
    )


def _render_any_dictionary(name, model, is_request_root: bool) -> str:
    template = _ENV.get_template("model/any_dictionary.java.jinja")
    return template.render(
        package=name.package,
        class_name=name.class_name,
        is_request_root=is_request_root,
        json_support_fqn=json_support_fqn(model.base_package),
    )


def _render_composition(node: CompositionNode, name, model, is_request_root: bool) -> str:
    branches = build_branches(node, model)
    template = _ENV.get_template("model/composition.java.jinja")
    return template.render(
        package=name.package,
        class_name=name.class_name,
        keyword=node.keyword,
        branches=branches,
        is_request_root=is_request_root,
        json_support_fqn=json_support_fqn(model.base_package),
    )


# --- Graph traversal: every node reachable from named schemas or an operation's
# --- request/response tree, visited exactly once (dedup already collapsed
# --- structurally identical anonymous nodes onto one object).


def _reachable_nodes(model):
    visited: set = set()
    for node in model.named_nodes.values():
        yield from _walk(node, model.named_nodes, visited)
    for operation_model in model.operations:
        request_node = operation_model.request
        if request_node is not None and request_node.body is not None:
            yield from _walk(request_node.body.target, model.named_nodes, visited)
        for code_node in operation_model.response.codes:
            for content_type_view in code_node.content_types:
                yield from _walk(content_type_view.body.target, model.named_nodes, visited)


def _walk(node, named_nodes: dict, visited: set):
    if id(node) in visited:
        return
    visited.add(id(node))
    yield node
    if isinstance(node, RefNode):
        yield from _walk(named_nodes[node.name], named_nodes, visited)
        return
    for _, edge in iter_child_edges(node):
        yield from _walk(edge.target, named_nodes, visited)
