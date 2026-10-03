"""Java language support: types, literals, and code-fragment generation."""
import re
from xy.cgen.lang.base import LanguageSupport
_PATH_PARAM = re.compile('\\{([^}]+)\\}')
'#: JsonNodeFactory typed-constructor names, used only where ArrayNode has no'
'#: typed set() overload (mixed/tuple lists, see list_mixed.jinja).'
_FACTORY_METHOD = {'string': 'textNode', 'integer': 'numberNode', 'number': 'numberNode', 'boolean': 'booleanNode'}
'#: JsonNode.is*() method per primitive type, used for structural type checks.'
_TYPE_CHECK_METHOD = {
    'string': 'isTextual',
    'integer': 'isIntegralNumber',
    'number': 'isNumber',
    'boolean': 'isBoolean'}
"#: expression parsing a decoded String into the parameter's scalar type."
_PARSE_EXPR = {
    'Long': 'Long.valueOf({raw})',
    'Double': 'Double.valueOf({raw})',
    'Boolean': 'Boolean.valueOf({raw})',
    'String': '{raw}'}

def _string_literal(text: str) -> str:
    escaped = text.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n').replace('\r', '\\r')
    return f'"{escaped}"'

class JavaSupport(LanguageSupport):
    name = 'java'
    extension = 'java'
    primitive_type = {'string': 'String', 'integer': 'Long', 'number': 'Double', 'boolean': 'Boolean'}
    any_dictionary_type = 'com.fasterxml.jackson.databind.JsonNode'
    parameter_type = {'integer': 'Long', 'number': 'Double', 'boolean': 'Boolean'}
    parameter_default_type = 'String'
    read_method = {'string': 'asText', 'integer': 'asLong', 'number': 'asDouble', 'boolean': 'asBoolean'}
    factory_method = _FACTORY_METHOD
    jinja_filters = {}

    def string_literal(self, text: str) -> str:
        return _string_literal(text)

    def literal(self, value, primitive_type: str) -> str:
        if primitive_type == 'boolean':
            return 'true' if value else 'false'
        if primitive_type == 'string':
            return _string_literal(str(value))
        if primitive_type == 'integer':
            return f'{int(value)}L'
        if primitive_type == 'number':
            return f'{float(value)}d'
        raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')

    def type_hint(self, type_name: str) -> str:
        return type_name

    def parameter_declaration(self, type_hint: str, var_name: str, kind: str='path') -> str:
        return f'{type_hint} {var_name}'

    def reference_expr(self, name: str) -> str:
        return name

    def path_url_expression(self, path: str, path_params) -> str:
        by_raw_name = {p.raw_name: p for p in path_params}
        parts, last = ([], 0)
        for match in _PATH_PARAM.finditer(path):
            literal = path[last:match.start()]
            if literal:
                parts.append(_string_literal(literal))
            param = by_raw_name[match.group(1)]
            parts.append(
                f'java.net.URLEncoder.encode(String.valueOf({
                    param.name}), java.nio.charset.StandardCharsets.UTF_8)')
            last = match.end()
        tail = path[last:]
        if tail or not parts:
            parts.append(_string_literal(tail))
        return ' + '.join(parts)

    def discriminator_literal_expr(self, property_name: str, value, primitive_type: str) -> str:
        """# Uses JsonNode.path() (never MissingNode == null) so no separate absence guard is needed here."""
        accessor = f'node.path("{property_name}")'
        if primitive_type == 'string':
            return f'{_string_literal(str(value))}.equals({accessor}.asText())'
        if primitive_type == 'integer':
            return f'{accessor}.asLong() == {int(value)}L'
        if primitive_type == 'number':
            return f'{accessor}.asDouble() == {float(value)}d'
        if primitive_type == 'boolean':
            return f'{accessor}.asBoolean() == {('true' if value else 'false')}'
        raise ValueError(f'discriminator property has no supported primitive type: {primitive_type!r}')

    def applies_always(self) -> str:
        return 'true'

    def applies_required(self, fields: list) -> str:
        return ' && '.join((f'node.has("{field_name}")' for field_name in fields))

    def applies_array(self) -> str:
        return 'node.isArray()'

    def applies_object(self) -> str:
        return 'node.isObject()'

    def applies_type_check(self, primitive_type: str) -> str:
        return f'node.{_TYPE_CHECK_METHOD[primitive_type]}()'

    def applies_null(self) -> str:
        return 'node.isNull()'

    def build_binding_lines(self, client_method, regex_order, body_json_support_fqn: str) -> tuple:
        lines = []
        for i, param in enumerate(regex_order, start=1):
            raw = f'URLDecoder.decode(matcher.group({i}), StandardCharsets.UTF_8)'
            lines.append(f'{param.java_type} {param.name} = {_PARSE_EXPR[param.java_type].format(raw=raw)};')
        for param in client_method.query_params:
            raw_var = f'{param.name}Raw'
            lines.append(f'String {raw_var} = queryParams.get("{param.raw_name}");')
            lines.append(
                f'{param.java_type} {param.name} = {raw_var} == null ? null : {_PARSE_EXPR[param.java_type].format(raw=raw_var)};')
        if client_method.body_param is not None:
            lines.append('String rawBody = readBody(exchange);')
            lines.append(f'{client_method.body_param.java_type} {client_method.body_param.name} = new {client_method.body_param.java_type}({body_json_support_fqn}.parse(rawBody));')
        return tuple(lines)
JAVA = JavaSupport()