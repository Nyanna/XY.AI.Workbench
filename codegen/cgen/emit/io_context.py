"""Builds the Jinja2 template context for request/response root serialization.

Root objects encapsulate their own (de-)serialization: a request root
gets toString()/fromString() added directly to its already-generated model
class (see model_emit.py); a response root is its own small class here,
since status code and content type are transport metadata that never live on
a shared model type -- they only exist on this operation-specific root.
"""

from dataclasses import dataclass

from cgen.emit.model_context import PRIMITIVE_READ_METHOD, classify
from cgen.model.nodes import RefNode
from cgen.naming.identifiers import content_type_short_name, to_pascal_case
from cgen.typemap import map_type


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
    return f"{base_package}.io.JsonSupport"


@dataclass(frozen=True)
class ContentTypeBranch:
    """One content-type view of a CodeNode: is<Ct>()/get<Ct>(), discriminated by header."""

    short_name: str
    content_type: str
    java_type: str
    category: str
    read_method: str | None
    description: str | None


def build_content_type_branches(code_node, named_model) -> list:
    """One branch per ContentTypeView, in declaration order."""
    branches = []
    for content_type_view in code_node.content_types:
        edge = content_type_view.body
        category, primitive_type = classify(edge.target, named_model.named_nodes)
        if category == "unsupported":
            continue
        branches.append(
            ContentTypeBranch(
                short_name=to_pascal_case(content_type_short_name(content_type_view.content_type)),
                content_type=content_type_view.content_type,
                java_type=map_type(edge.target, named_model),
                category=category,
                read_method=PRIMITIVE_READ_METHOD.get(primitive_type) if category in ("primitive", "enum") else None,
                description=edge.description,
            )
        )
    return branches


@dataclass(frozen=True)
class CodeBranch:
    """One status-code view of a ResponseNode: getCode<code>() -- null unless it matches."""

    status_code: str
    java_type: str


def build_code_branches(response_node, named_model) -> list:
    return [
        CodeBranch(status_code=code_node.status_code, java_type=named_model.name_of(code_node).fqn)
        for code_node in response_node.codes
    ]
