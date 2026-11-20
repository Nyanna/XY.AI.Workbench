"""TypeScript language support: types, literals, and code-fragment generation.

Generated proxies wrap a plain JSON value (as produced by `JSON.parse`), so no
runtime dependency is needed; the client uses the global `fetch`, the server
`node:http`. The output only uses erasable TypeScript syntax (no enums, no
parameter properties) and relative imports with `.ts` extensions, so Node can
run it directly via type stripping. Cross-class references are imported once
per file under a collision-free alias derived from the fully-qualified name
(`import { Foo as pkg_sub_Foo } from "../sub/Foo.ts"`).
"""
import json
import posixpath
import re
from xy.cgen.lang.base import LanguageSupport
_PATH_PARAM = re.compile('\\{([^}]+)\\}')
_NON_IDENTIFIER = re.compile('\\W')
_OBJECT_CHECK = 'typeof this.node === "object" && this.node !== null && !Array.isArray(this.node)'
_TYPE_CHECK_EXPR = {
    'string': 'typeof this.node === "string"',
    'integer': 'Number.isInteger(this.node)',
    'number': 'typeof this.node === "number"',
    'boolean': 'typeof this.node === "boolean"'}
_PARSE_EXPR = {'number': 'Number({raw})', 'boolean': '{raw}.toLowerCase() === "true"', 'string': '{raw}'}

def _string_literal(text: str) -> str:
    return json.dumps(text)

def _alias(fqn: str) -> str:
    return _NON_IDENTIFIER.sub('_', fqn)

def _ref(fqn: str, self_fqn: str | None=None) -> str:
    """Jinja filter: a type-position reference to `fqn` -- scalars pass through,
    the file's own class by its simple name, everything else by its import alias."""
    if '.' not in fqn:
        return fqn
    if fqn == self_fqn:
        return fqn.rsplit('.', 1)[-1]
    return _alias(fqn)

def _imports(fqns, self_fqn: str | None=None) -> list:
    """Jinja filter: dedup/sort the fqns among `fqns` that need an import statement."""
    return sorted({fqn for fqn in fqns if fqn and '.' in fqn and (fqn != self_fqn)})

def _path(fqn: str, package: str) -> str:
    """Jinja filter: relative module specifier of `fqn`'s file as seen from `package`."""
    target = fqn.replace('.', '/') + '.ts'
    relative = posixpath.relpath(target, package.replace('.', '/'))
    return relative if relative.startswith('.') else f'./{relative}'

class TypeScriptSupport(LanguageSupport):
    name = 'typescript'
    extension = 'ts'
    primitive_type = {'string': 'string', 'integer': 'number', 'number': 'number', 'boolean': 'boolean'}
    any_dictionary_type = 'unknown'
    parameter_type = {'integer': 'number', 'number': 'number', 'boolean': 'boolean'}
    parameter_default_type = 'string'
    read_method = {}
    factory_method = {}
    jinja_filters = {'tsref': _ref, 'tsimports': _imports, 'tspath': _path}

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
            return json.dumps(float(value))
        raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')

    def type_hint(self, type_name: str) -> str:
        return _ref(type_name)

    def parameter_declaration(self, type_hint: str, var_name: str, kind: str='path') -> str:
        if kind == 'query':
            return f'{var_name}: {type_hint} | null'
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
            parts.append(f'encodeURIComponent(String({param.name}))')
            last = match.end()
        tail = path[last:]
        if tail or not parts:
            parts.append(_string_literal(tail))
        return ' + '.join(parts)

    def discriminator_literal_expr(self, property_name: str, value, primitive_type: str) -> str:
        key = _string_literal(property_name)
        accessor = f'(this.node as Record<string, unknown>)[{key}]'
        literal = self.literal(value, primitive_type)
        return f'({_OBJECT_CHECK} && Object.hasOwn(this.node as object, {key}) && {accessor} === {literal})'

    def applies_always(self) -> str:
        return 'true'

    def applies_required(self, fields: list) -> str:
        checks = ' && '.join((f'Object.hasOwn(this.node as object, {_string_literal(f)})' for f in fields))
        return f'({_OBJECT_CHECK} && {checks})' if checks else f'({_OBJECT_CHECK})'

    def applies_array(self) -> str:
        return 'Array.isArray(this.node)'

    def applies_object(self) -> str:
        return f'({_OBJECT_CHECK})'

    def applies_type_check(self, primitive_type: str) -> str:
        return _TYPE_CHECK_EXPR[primitive_type]

    def applies_null(self) -> str:
        return 'this.node === null'

    def build_binding_lines(self, client_method, regex_order, body_json_support_fqn: str) -> tuple:
        lines = []
        for i, param in enumerate(regex_order, start=1):
            raw = f'decodeURIComponent(_match[{i}] as string)'
            lines.append(f'const {param.name} = {_PARSE_EXPR[param.java_type].format(raw=raw)};')
        for param in client_method.query_params:
            raw_var = f'_{param.name}Raw'
            lines.append(f'const {raw_var} = _query.get({_string_literal(param.raw_name)});')
            parsed = _PARSE_EXPR[param.java_type].format(raw=raw_var)
            lines.append(f'const {param.name} = {raw_var} === null ? null : {parsed};')
        if client_method.body_param is not None:
            body_type = self.type_hint(client_method.body_param.java_type)
            lines.append(f'const {client_method.body_param.name} = {body_type}.fromString(await _readBody(_req));')
        return tuple(lines)
TYPESCRIPT = TypeScriptSupport()
