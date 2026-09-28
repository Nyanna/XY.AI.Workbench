"""Builds the Jinja2 template context for request/response root serialization.

Root objects encapsulate their own (de-)serialization: a request root
gets toString()/fromString() added directly to its already-generated model
class (see model_emit.py); a response root is its own small class here,
since status code and content type are transport metadata that never live on
a shared model type -- they only exist on this operation-specific root.
"""
from dataclasses import dataclass
from xy.cgen.emit.model_context import classify
from xy.cgen.lang import get_language
from xy.cgen.model.nodes import RefNode
from xy.cgen.naming.identifiers import content_type_short_name, to_pascal_case
from xy.cgen.typemap import map_type

def request_root_node(operation_model, named_nodes):
    """The structural node whose generated class becomes this operation's request root.

    A $ref body IS the request root: the already-generated class for that
    named schema gets serialization added, no wrapper class. An inline body's
    own node gets it directly for the same reason -- there is exactly one
    class per request body, never a duplicate.
    """
    request_node = operation_model.request
    if request_node is None or request_node.body is None:
        return None
    target = request_node.body.target
    return named_nodes[target.name] if isinstance(target, RefNode) else target

def request_root_node_ids(model) -> set:
    """id() of every node that must render toString()/fromString()."""
    ids = set()
    for operation_model in model.operations:
        node = request_root_node(operation_model, model.named_nodes)
        if node is not None:
            ids.add(id(node))
    return ids

def json_support_fqn(base_package: str) -> str:
    return f'{base_package}.JsonSupport'

@dataclass(frozen=True)
class ContentTypeBranch:
    """One content-type view of a CodeNode: is<Ct>()/get<Ct>(), discriminated by header."""
    short_name: str
    content_type: str
    java_type: str
    category: str
    read_method: str | None
    node_factory_method: str | None
    description: str | None

def build_content_type_branches(code_node, named_model) -> list:
    """One branch per ContentTypeView, in declaration order."""
    lang = get_language(named_model.language)
    branches = []
    for content_type_view in code_node.content_types:
        edge = content_type_view.body
        category, primitive_type = classify(edge.target, named_model.named_nodes)
        if category == 'unsupported':
            continue
        has_read = category in ('primitive', 'enum')
        branches.append(
            ContentTypeBranch(
                short_name=to_pascal_case(
                    content_type_short_name(
                        content_type_view.content_type)),
                content_type=content_type_view.content_type,
                java_type=map_type(
                    edge.target,
                    named_model),
                category=category,
                read_method=lang.read_method.get(primitive_type) if has_read else None,
                node_factory_method=lang.factory_method.get(primitive_type) if category == 'primitive' else None,
                description=edge.description))
    return branches

@dataclass(frozen=True)
class CodeBranch:
    """One status-code view of a ResponseNode: getCode<code>() -- null unless it matches."""
    status_code: str
    java_type: str

def build_code_branches(response_node, named_model) -> list:
    return [CodeBranch(status_code=code_node.status_code, java_type=named_model.name_of(code_node).fqn)
            for code_node in response_node.codes]

@dataclass(frozen=True)
class ResponseSetter:
    """One setCode<NNN>[<ContentTypeSuffix>](...) constructor-style setter on a
    ResponseNode: status code and content type are implied by the spec, the
    server implementer only supplies the already-typed body value."""
    status_code: str
    method_name: str
    content_type: str
    java_type: str
    category: str
    node_factory_method: str | None

def build_response_setters(response_node, named_model) -> list:
    """One ResponseSetter per (status code, content type) combination. The
    content-type suffix is only appended when a status code has more than one
    content type (declaration order, same as build_content_type_branches)."""
    setters = []
    for code_node in response_node.codes:
        branches = build_content_type_branches(code_node, named_model)
        multiple = len(branches) > 1
        for branch in branches:
            suffix = branch.short_name if multiple else ''
            setters.append(
                ResponseSetter(
                    status_code=code_node.status_code,
                    method_name=f'setCode{
                        code_node.status_code}{suffix}',
                    content_type=branch.content_type,
                    java_type=branch.java_type,
                    category=branch.category,
                    node_factory_method=branch.node_factory_method))
    return setters