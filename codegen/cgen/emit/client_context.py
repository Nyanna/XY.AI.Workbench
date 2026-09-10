"""Builds the Jinja2 template context for the client interface/implementation.

One client per generator run: a single interface
with one method per operation, backed by one HttpClient-based implementation
(base URL/auth are runtime concerns, never generated into the method
bodies themselves). Path/query parameters shape the method signature and the
request URL; the JSON body -- if any -- is the already-generated request-root
class, which encapsulates its own serialization.
"""

import re
from dataclasses import dataclass

from cgen.emit.io_context import request_root_node
from cgen.model.nodes import MISSING
from cgen.naming.identifiers import class_identifier, property_accessor_name, sanitize_identifier, to_camel_case
from cgen.naming.paths import path_to_class_fragment

PARAMETER_JAVA_TYPE = {"integer": "Long", "number": "Double", "boolean": "Boolean"}
_PATH_PARAM = re.compile(r"\{([^}]+)\}")


def _parameter_java_type(schema: dict | None) -> str:
    """Path/query parameters never enter the body tree; map their
    raw JSON-Schema type directly to a scalar Java type (string is the default)."""
    return PARAMETER_JAVA_TYPE.get((schema or {}).get("type"), "String")


@dataclass(frozen=True)
class MethodParameter:
    """One path/query parameter, or the synthetic body parameter."""

    name: str  # Java identifier used in the method signature
    java_type: str
    raw_name: str  # original OpenAPI parameter name (the actual URL token/query key)
    kind: str  # 'path' | 'query' | 'body'


def _build_parameter(param, kind: str) -> MethodParameter:
    return MethodParameter(
        name=property_accessor_name(param.name), java_type=_parameter_java_type(param.schema), raw_name=param.name, kind=kind
    )


def _java_string_literal(text: str) -> str:
    escaped = text.replace("\\", "\\\\").replace('"', '\\"')
    return f'"{escaped}"'


def _path_url_expression(path: str, path_params: tuple) -> str:
    """A Java string-concatenation expression rebuilding the URL path, with
    every '{param}' token replaced by its URL-encoded argument value."""
    by_raw_name = {p.raw_name: p for p in path_params}
    parts, last = [], 0
    for match in _PATH_PARAM.finditer(path):
        literal = path[last : match.start()]
        if literal:
            parts.append(_java_string_literal(literal))
        param = by_raw_name[match.group(1)]
        parts.append(f"java.net.URLEncoder.encode(String.valueOf({param.name}), java.nio.charset.StandardCharsets.UTF_8)")
        last = match.end()
    tail = path[last:]
    if tail or not parts:
        parts.append(_java_string_literal(tail))
    return " + ".join(parts)


def _method_name(operation) -> str:
    """operationId wins verbatim (D-decision); otherwise '<method><Path>' (e.g. postResponses)."""
    if operation.operation_id:
        return sanitize_identifier(operation.operation_id)
    return to_camel_case(f"{operation.method}{path_to_class_fragment(operation.path)}")


@dataclass(frozen=True)
class ClientMethod:
    """One operation's client-facing method: interface signature plus everything
    the implementation needs to build and send the HTTP request."""

    name: str
    http_method: str  # upper-case, e.g. 'POST'
    path: str
    path_url_expression: str  # ready-made Java expression for the request URL's path part
    path_params: tuple  # MethodParameter, in path-declaration order
    query_params: tuple  # MethodParameter
    body_param: MethodParameter | None  # None if the operation has no request body
    parameters: tuple  # path_params + query_params + ([body_param]), for the method signature
    signature: str  # precomputed "Type name, Type name, ..." parameter list
    response_type: str  # FQN of the already-generated ResponseNode root class
    description: str | None
    example_repr: str | None


def build_client_methods(named_model) -> list[ClientMethod]:
    """One ClientMethod per operation, in deterministic (path, method) order."""
    methods = []
    used_names: dict[str, int] = {}
    ordered = sorted(named_model.operations, key=lambda om: (om.operation.path, om.operation.method))
    for operation_model in ordered:
        operation = operation_model.operation
        name = _method_name(operation)
        # Defensive collision guard (structurally shouldn't happen: distinct
        # (path, method) pairs only collide in name if operationIds collide,
        # which is itself an invalid OpenAPI document) -- numbered by the same
        # deterministic (path, method) order used above, never discovery order.
        used_names[name] = used_names.get(name, 0) + 1
        final_name = name if used_names[name] == 1 else f"{name}{used_names[name]}"

        path_params = tuple(_build_parameter(p, "path") for p in operation.parameters if p.location == "path")
        query_params = tuple(_build_parameter(p, "query") for p in operation.parameters if p.location == "query")

        body_node = request_root_node(operation_model, named_model.named_nodes)
        body_param = MethodParameter(name="request", java_type=named_model.name_of(body_node).fqn, raw_name="request", kind="body") if body_node is not None else None

        request_edge = operation_model.request.body if operation_model.request is not None else None
        example = None if request_edge is None or request_edge.example is MISSING else repr(request_edge.example)

        parameters = path_params + query_params + ((body_param,) if body_param is not None else ())
        methods.append(
            ClientMethod(
                name=final_name,
                http_method=operation.method.upper(),
                path=operation.path,
                path_url_expression=_path_url_expression(operation.path, path_params),
                path_params=path_params,
                query_params=query_params,
                body_param=body_param,
                parameters=parameters,
                signature=", ".join(f"{p.java_type} {p.name}" for p in parameters),
                response_type=named_model.name_of(operation_model.response).fqn,
                description=operation.description,
                example_repr=example,
            )
        )
    return methods


def _top_path_segment(path: str) -> str:
    for part in (path or "").split("/"):
        part = part.strip().strip("{}")
        if part:
            return part
    return "api"


def client_interface_name(named_model) -> str:
    """Deterministic name from the sorted set of top-level path segments
    (single path '/responses' -> 'ResponsesClient', per acceptance)."""
    top_segments = sorted({_top_path_segment(om.operation.path) for om in named_model.operations})
    fragment = "".join(class_identifier(segment) for segment in top_segments) if top_segments else "Api"
    return f"{fragment}Client"
