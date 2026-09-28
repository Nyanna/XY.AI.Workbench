"""PHP language support: types, literals, and code-fragment generation."""
import re
from xy.cgen.lang.base import LanguageSupport
_PATH_PARAM = re.compile('\\{([^}]+)\\}')
_SCALAR_TYPES = {'int', 'float', 'string', 'bool', 'mixed'}
'#: is_*() function per primitive type, used for structural type checks.'
_TYPE_CHECK = {'string': 'is_string', 'integer': 'is_int', 'number': 'is_float', 'boolean': 'is_bool'}
"#: expression parsing a decoded string into the parameter's scalar type."
_PARSE_EXPR = {
    'int': '(int) {raw}',
    'float': '(float) {raw}',
    'bool': 'filter_var({raw}, FILTER_VALIDATE_BOOLEAN)',
    'string': '{raw}'}

def _string_literal(text: str) -> str:
    escaped = text.replace('\\', '\\\\').replace("'", "\\'")
    return f"'{escaped}'"

def _to_php_fqn(fqn: str) -> str:
    """Dotted fqn ('pkg.sub.Class') -> fully-qualified PHP name ('\\pkg\\sub\\Class')."""
    return '\\' + fqn.replace('.', '\\')

def _to_namespace(package: str) -> str:
    return package.replace('.', '\\')

class PhpSupport(LanguageSupport):
    name = 'php'
    extension = 'php'
    primitive_type = {'string': 'string', 'integer': 'int', 'number': 'float', 'boolean': 'bool'}
    any_dictionary_type = 'mixed'
    parameter_type = {'integer': 'int', 'number': 'float', 'boolean': 'bool'}
    parameter_default_type = 'string'
    read_method = primitive_type
    factory_method = {}
    jinja_filters = {'phpfqn': _to_php_fqn, 'phpns': _to_namespace}

    def string_literal(self, text: str) -> str:
        return _string_literal(text)

    def literal(self, value, primitive_type: str) -> str:
        if primitive_type == 'boolean':
            return 'true' if value else 'false'
        if primitive_type == 'string':
            return _string_literal(str(value))
        if primitive_type == 'integer':
            return str(int(value))
        if primitive_type == 'number':
            return repr(float(value))
        raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')

    def type_hint(self, type_name: str) -> str:
        """A dotted fqn ('pkg.Class') becomes a fully-qualified PHP type hint; scalars pass through."""
        if type_name in _SCALAR_TYPES or '.' not in type_name:
            return type_name
        return _to_php_fqn(type_name)

    def parameter_declaration(self, type_hint: str, var_name: str) -> str:
        return f'{type_hint} ${var_name}'

    def reference_expr(self, name: str) -> str:
        return f'${name}'

    def path_url_expression(self, path: str, path_params) -> str:
        by_raw_name = {p.raw_name: p for p in path_params}
        parts, last = ([], 0)
        for match in _PATH_PARAM.finditer(path):
            literal = path[last:match.start()]
            if literal:
                parts.append(_string_literal(literal))
            param = by_raw_name[match.group(1)]
            parts.append(f'rawurlencode((string) ${param.name})')
            last = match.end()
        tail = path[last:]
        if tail or not parts:
            parts.append(_string_literal(tail))
        return ' . '.join(parts)

    def discriminator_literal_expr(self, property_name: str, value, primitive_type: str) -> str:
        accessor = f"($this->node['{property_name}'] ?? null)"
        if primitive_type == 'boolean':
            return f'{accessor} === {('true' if value else 'false')}'
        return f'{accessor} === {self.literal(value, primitive_type)}'

    def applies_always(self) -> str:
        return 'true'

    def applies_required(self, fields: list) -> str:
        return ' && '.join((f"array_key_exists('{field_name}', (array) $this->node)" for field_name in fields))

    def applies_array(self) -> str:
        return 'is_array($this->node)'

    def applies_object(self) -> str:
        return 'is_array($this->node) || is_object($this->node)'

    def applies_type_check(self, primitive_type: str) -> str:
        return f'{_TYPE_CHECK[primitive_type]}($this->node)'

    def applies_null(self) -> str:
        return '$this->node === null'

    def build_binding_lines(self, client_method, regex_order, body_json_support_fqn: str) -> tuple:
        lines = []
        for i, param in enumerate(regex_order, start=1):
            raw = f'urldecode($matches[{i}])'
            lines.append(f'${param.name} = {_PARSE_EXPR[param.java_type].format(raw=raw)};')
        for param in client_method.query_params:
            raw_var = f'${param.name}Raw'
            lines.append(f"{raw_var} = $queryParams['{param.raw_name}'] ?? null;")
            lines.append(
                f'${param.name} = {raw_var} === null ? null : {_PARSE_EXPR[param.java_type].format(raw=raw_var)};')
        if client_method.body_param is not None:
            lines.append("$rawBody = file_get_contents('php://input');")
            lines.append(
                f'${
                    client_method.body_param.name} = new {
                        self.type_hint(
                            client_method.body_param.java_type)}(json_decode($rawBody, true));')
        return tuple(lines)
PHP = PhpSupport()