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
from xy.cgen.emit.io_context import request_root_node
from xy.cgen.lang import get_language
from xy.cgen.model.nodes import MISSING
from xy.cgen.naming.identifiers import class_identifier, property_accessor_name, sanitize_identifier, to_camel_case
from xy.cgen.naming.paths import path_to_class_fragment
_PATH_PARAM = re.compile('\\{([^}]+)\\}')

def _parameter_type(schema: dict | None, language: str) -> str:
    """Path/query parameters never enter the body tree; map their
    raw JSON-Schema type directly to a scalar type (string is the default)."""
    lang = get_language(language)
    return lang.parameter_type.get((schema or {}).get('type'), lang.parameter_default_type)

@dataclass(frozen=True)
class MethodParameter:
    """One path/query parameter, or the synthetic body parameter."""
    '# Java identifier used in the method signature'
    name: str
    java_type: str
    '# original OpenAPI parameter name (the actual URL token/query key)'
    raw_name: str
    "# 'path' | 'query' | 'body'"
    kind: str

def _build_parameter(param, kind: str, language: str) -> MethodParameter:
    return MethodParameter(
        name=property_accessor_name(
            param.name), java_type=_parameter_type(
                param.schema, language), raw_name=param.name, kind=kind)

def _signature(parameters: tuple, language: str) -> str:
    lang = get_language(language)
    return ', '.join((lang.parameter_declaration(lang.type_hint(p.java_type), p.name, p.kind) for p in parameters))

def _path_url_expression(path: str, path_params: tuple, language: str) -> str:
    """A string-concatenation expression rebuilding the URL path, with
    every '{param}' token replaced by its URL-encoded argument value."""
    return get_language(language).path_url_expression(path, path_params)

def _method_name(operation) -> str:
    """operationId wins verbatim (D-decision); otherwise '<method><Path>' (e.g. postResponses)."""
    if operation.operation_id:
        return sanitize_identifier(operation.operation_id)
    return to_camel_case(f'{operation.method}{path_to_class_fragment(operation.path)}')

@dataclass(frozen=True)
class ClientMethod:
    """One operation's client-facing method: interface signature plus everything
    the implementation needs to build and send the HTTP request."""
    name: str
    "# upper-case, e.g. 'POST'"
    http_method: str
    path: str
    "# ready-made Java expression for the request URL's path part"
    path_url_expression: str
    '# MethodParameter, in path-declaration order'
    path_params: tuple
    '# MethodParameter'
    query_params: tuple
    '# None if the operation has no request body'
    body_param: MethodParameter | None
    '# path_params + query_params + ([body_param]), for the method signature'
    parameters: tuple
    '# precomputed "Type name, Type name, ..." parameter list'
    signature: str
    '# FQN of the already-generated ResponseNode root class'
    response_type: str
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
        "# Defensive collision guard (structurally shouldn't happen: distinct"
        '# (path, method) pairs only collide in name if operationIds collide,'
        '# which is itself an invalid OpenAPI document) -- numbered by the same'
        '# deterministic (path, method) order used above, never discovery order.'
        used_names[name] = used_names.get(name, 0) + 1
        final_name = name if used_names[name] == 1 else f'{name}{used_names[name]}'
        language = named_model.language
        path_params = tuple((_build_parameter(p, 'path', language)
                            for p in operation.parameters if p.location == 'path'))
        query_params = tuple((_build_parameter(p, 'query', language)
                             for p in operation.parameters if p.location == 'query'))
        body_node = request_root_node(operation_model, named_model.named_nodes)
        body_param = MethodParameter(
            name='request',
            java_type=named_model.name_of(body_node).fqn,
            raw_name='request',
            kind='body') if body_node is not None else None
        request_edge = operation_model.request.body if operation_model.request is not None else None
        example = None if request_edge is None or request_edge.example is MISSING else repr(request_edge.example)
        parameters = path_params + query_params + ((body_param,) if body_param is not None else ())
        methods.append(
            ClientMethod(
                name=final_name,
                http_method=operation.method.upper(),
                path=operation.path,
                path_url_expression=_path_url_expression(
                    operation.path,
                    path_params,
                    language),
                path_params=path_params,
                query_params=query_params,
                body_param=body_param,
                parameters=parameters,
                signature=_signature(
                    parameters,
                    language),
                response_type=named_model.name_of(
                    operation_model.response).fqn,
                description=operation.description,
                example_repr=example))
    return methods

def _top_path_segment(path: str) -> str:
    for part in (path or '').split('/'):
        part = part.strip().strip('{}')
        if part:
            return part
    return 'api'

def client_interface_name(named_model) -> str:
    """Deterministic name from the sorted set of top-level path segments
    (single path '/responses' -> 'ResponsesClient', per acceptance)."""
    top_segments = sorted({_top_path_segment(om.operation.path) for om in named_model.operations})
    fragment = ''.join((class_identifier(segment) for segment in top_segments)) if top_segments else 'Api'
    return f'{fragment}Client'