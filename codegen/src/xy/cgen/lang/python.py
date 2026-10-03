"""Python language support: types, literals, and code-fragment generation.

Generated proxies wrap a plain Python JSON value (dict/list/str/int/float/bool/
None, as produced by `json.loads`) instead of a typed tree library -- no runtime
dependency is needed. Cross-class references use the fully-qualified module path
directly (`import pkg.sub.Foo` then `pkg.sub.Foo.Foo(...)`), mirroring the
Java/PHP fqn-in-place approach instead of per-file `from ... import ...` lists.
"""
import re
from xy.cgen.lang.base import LanguageSupport
_PATH_PARAM = re.compile('\\{([^}]+)\\}')
'#: Optional raw-string -> scalar conversion, used where JSON already decoded'
'#: the value (str stays str, bool stays bool -- only int/float ever need a cast).'
_FACTORY_METHOD: dict = {}
_READ_METHOD = {'integer': 'int', 'number': 'float'}
'#: structural type-check expression per primitive type (bool is an int subclass'
'#: in Python, so integer/number checks must explicitly exclude it).'
_TYPE_CHECK_EXPR = {
    'string': 'isinstance(self._node, str)',
    'integer': 'isinstance(self._node, int) and not isinstance(self._node, bool)',
    'number': 'isinstance(self._node, (int, float)) and not isinstance(self._node, bool)',
    'boolean': 'isinstance(self._node, bool)'}
"#: expression parsing a decoded (already-unquoted) string into the parameter's scalar type."
_PARSE_EXPR = {'int': 'int({raw})', 'float': 'float({raw})', 'bool': "{raw}.lower() == 'true'", 'str': '{raw}'}

def _string_literal(text: str) -> str:
    return repr(text)

def _pyimports(fqns, self_fqn=None) -> list:
    """Jinja filter: dedup/sort the fqns among `fqns` that need an `import`
    statement -- i.e. drop None, scalars, `typing.Any`, and self-references."""
    seen = set()
    for fqn in fqns:
        if fqn and fqn != self_fqn and (fqn != 'typing.Any') and ('.' in fqn):
            seen.add(fqn)
    return sorted(seen)

class PythonSupport(LanguageSupport):
    name = 'python'
    extension = 'py'
    primitive_type = {'string': 'str', 'integer': 'int', 'number': 'float', 'boolean': 'bool'}
    any_dictionary_type = 'typing.Any'
    parameter_type = {'integer': 'int', 'number': 'float', 'boolean': 'bool'}
    parameter_default_type = 'str'
    read_method = _READ_METHOD
    factory_method = _FACTORY_METHOD
    jinja_filters = {'pyimports': _pyimports}

    def string_literal(self, text: str) -> str:
        return _string_literal(text)

    def literal(self, value, primitive_type: str) -> str:
        if primitive_type == 'boolean':
            return 'True' if value else 'False'
        if primitive_type == 'string':
            return _string_literal(str(value))
        if primitive_type == 'integer':
            return str(int(value))
        if primitive_type == 'number':
            return repr(float(value))
        raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')

    def type_hint(self, type_name: str) -> str:
        return type_name

    def parameter_declaration(self, type_hint: str, var_name: str, kind: str='path') -> str:
        return f'{var_name}: {type_hint}'

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
            parts.append(f'urllib.parse.quote(str({param.name}), safe="")')
            last = match.end()
        tail = path[last:]
        if tail or not parts:
            parts.append(_string_literal(tail))
        return ' + '.join(parts)

    def discriminator_literal_expr(self, property_name: str, value, primitive_type: str) -> str:
        """Guards with isinstance(..., dict) since self._node may be any JSON value."""
        accessor = f'self._node.get({_string_literal(property_name)})'
        literal = self.literal(value, primitive_type)
        return f'(isinstance(self._node, dict) and {accessor} == {literal})'

    def applies_always(self) -> str:
        return 'True'

    def applies_required(self, fields: list) -> str:
        return ' and '.join((f'{_string_literal(field_name)} in self._node' for field_name in fields))

    def applies_array(self) -> str:
        return 'isinstance(self._node, list)'

    def applies_object(self) -> str:
        return 'isinstance(self._node, dict)'

    def applies_type_check(self, primitive_type: str) -> str:
        return _TYPE_CHECK_EXPR[primitive_type]

    def applies_null(self) -> str:
        return 'self._node is None'

    def build_binding_lines(self, client_method, regex_order, body_json_support_fqn: str) -> tuple:
        lines = []
        for i, param in enumerate(regex_order, start=1):
            raw = f'urllib.parse.unquote(match.group({i}))'
            lines.append(f'{param.name} = {_PARSE_EXPR[param.java_type].format(raw=raw)}')
        for param in client_method.query_params:
            raw_var = f'{param.name}_raw'
            lines.append(f'{raw_var} = query_params.get({_string_literal(param.raw_name)})')
            lines.append(
                f'{param.name} = None if {raw_var} is None else {_PARSE_EXPR[param.java_type].format(raw=raw_var)}')
        if client_method.body_param is not None:
            fqn = client_method.body_param.java_type
            class_name = fqn.rsplit('.', 1)[-1]
            lines.append('raw_body = self._read_body()')
            lines.append(f'{client_method.body_param.name} = {fqn}.{class_name}({body_json_support_fqn}.parse(raw_body))')
        return tuple(lines)
PYTHON = PythonSupport()