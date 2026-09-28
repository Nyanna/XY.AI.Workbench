"""Builds the Jinja2 template context for the transport-agnostic abstract server base class.

One abstract method per operation, with the exact same signature (parameters
AND response type) as the corresponding client-interface method -- server-side
implementers never see HTTP types, only the already-generated XxxResponse
class, built via its no-arg constructor plus setCode<NNN>(...) (see
io_context.build_response_setters). Routing, path/query decoding and response
writing are generated once, in the concrete `handle(HttpExchange)` method.
"""
import re
from dataclasses import dataclass
from xy.cgen.emit.client_context import MethodParameter, _php_type_hint, _top_path_segment, build_client_methods
from xy.cgen.emit.io_context import json_support_fqn
from xy.cgen.naming.identifiers import class_identifier
_PATH_PARAM = re.compile('\\{([^}]+)\\}')
"# Java expression parsing a decoded String into the parameter's scalar type."
_JAVA_PARSE_EXPR = {
    'Long': 'Long.valueOf({raw})',
    'Double': 'Double.valueOf({raw})',
    'Boolean': 'Boolean.valueOf({raw})',
    'String': '{raw}'}
'# PHP expression parsing a decoded string into the parameter scalar type.'
_PHP_PARSE_EXPR = {
    'int': '(int) {raw}',
    'float': '(float) {raw}',
    'bool': 'filter_var({raw}, FILTER_VALIDATE_BOOLEAN)',
    'string': '{raw}'}
_PARSE_EXPR = {'java': _JAVA_PARSE_EXPR, 'php': _PHP_PARSE_EXPR}

@dataclass(frozen=True)
class ServerMethod:
    """One operation's abstract server method plus everything the generated
    `handle(HttpExchange)` dispatcher needs to route to it."""
    name: str
    "# upper-case, e.g. 'POST'"
    http_method: str
    response_type: str
    "# identical to the client method's signature (D-verbindlich)"
    signature: str
    description: str | None
    example_repr: str | None
    "# unique Java identifier for this operation's compiled path Pattern"
    pattern_name: str
    '# regex, one capturing group per path parameter, in path-appearance order'
    pattern_regex: str
    '# Java statements decoding matcher groups / query params / body into locals'
    binding_lines: tuple
    '# ready-made "a, b, c" argument list for calling the abstract method'
    call_args: str
    has_body: bool

def _path_regex(path: str, path_params: tuple) -> tuple[str, tuple]:
    """Regex pattern string plus the path parameters in regex-group
    (path-appearance) order, which need not match declaration order."""
    by_raw_name = {p.raw_name: p for p in path_params}
    parts, order, last = ([], [], 0)
    for match in _PATH_PARAM.finditer(path):
        parts.append(re.escape(path[last:match.start()]))
        parts.append('([^/]+)')
        order.append(by_raw_name[match.group(1)])
        last = match.end()
    parts.append(re.escape(path[last:]))
    return (''.join(parts), tuple(order))

def _binding_lines(client_method, regex_order: tuple, body_json_support_fqn: str, language: str) -> tuple:
    if language == 'php':
        return _binding_lines_php(client_method, regex_order)
    lines = []
    for i, param in enumerate(regex_order, start=1):
        raw = f'URLDecoder.decode(matcher.group({i}), StandardCharsets.UTF_8)'
        lines.append(f'{param.java_type} {param.name} = {_PARSE_EXPR['java'][param.java_type].format(raw=raw)};')
    for param in client_method.query_params:
        raw_var = f'{param.name}Raw'
        lines.append(f'String {raw_var} = queryParams.get("{param.raw_name}");')
        lines.append(
            f'{param.java_type} {param.name} = {raw_var} == null ? null : {_PARSE_EXPR['java'][param.java_type].format(raw=raw_var)};')
    if client_method.body_param is not None:
        lines.append('String rawBody = readBody(exchange);')
        lines.append(f'{client_method.body_param.java_type} {client_method.body_param.name} = new {client_method.body_param.java_type}({body_json_support_fqn}.parse(rawBody));')
    return tuple(lines)

def _binding_lines_php(client_method, regex_order: tuple) -> tuple:
    lines = []
    for i, param in enumerate(regex_order, start=1):
        raw = f'urldecode($matches[{i}])'
        lines.append(f'${param.name} = {_PARSE_EXPR['php'][param.java_type].format(raw=raw)};')
    for param in client_method.query_params:
        raw_var = f'${param.name}Raw'
        lines.append(f"{raw_var} = $queryParams['{param.raw_name}'] ?? null;")
        lines.append(
            f'${param.name} = {raw_var} === null ? null : {_PARSE_EXPR['php'][param.java_type].format(raw=raw_var)};')
    if client_method.body_param is not None:
        lines.append("$rawBody = file_get_contents('php://input');")
        lines.append(
            f'${
                client_method.body_param.name} = new {
                    _php_type_hint(
                        client_method.body_param.java_type)}(json_decode($rawBody, true));')
    return tuple(lines)

def build_server_methods(named_model) -> list[ServerMethod]:
    """One ServerMethod per operation, same order as build_client_methods."""
    support_fqn = json_support_fqn(named_model.base_package)
    server_methods = []
    for client_method in build_client_methods(named_model):
        pattern_regex, regex_order = _path_regex(client_method.path, client_method.path_params)
        binding_lines = _binding_lines(client_method, regex_order, support_fqn, named_model.language)
        call_args = ', '.join((f'${p.name}' if named_model.language ==
                              'php' else p.name for p in client_method.parameters))
        server_methods.append(
            ServerMethod(
                name=client_method.name,
                http_method=client_method.http_method,
                response_type=client_method.response_type,
                signature=client_method.signature,
                description=client_method.description,
                example_repr=client_method.example_repr,
                pattern_name=f'PATTERN_{
                    client_method.name.upper()}',
                pattern_regex=pattern_regex,
                binding_lines=binding_lines,
                call_args=call_args,
                has_body=client_method.body_param is not None))
    return server_methods

def server_class_name(named_model) -> str:
    """Deterministic name mirroring client_interface_name (e.g. 'ResponsesServer')."""
    top_segments = sorted({_top_path_segment(om.operation.path) for om in named_model.operations})
    fragment = ''.join((class_identifier(segment) for segment in top_segments)) if top_segments else 'Api'
    return f'{fragment}Server'