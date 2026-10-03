"""Rust language support: types, literals, and code-fragment generation.

Model/io classes wrap a serde_json::Value (no field-level structs), mirroring
the Python target's dynamic-node approach but with typed, Option-returning
accessors. Class/package names are shared, Java-identifier-derived names (see
xy.cgen.naming); they are rendered through the 'rustfqn' filter, which turns a
dotted fqn into a fully-qualified 'crate::...' path -- no 'use' imports are
ever generated, every reference is written out in full.
"""
import re
from xy.cgen.lang.base import LanguageSupport
_PATH_PARAM = re.compile('\\{([^}]+)\\}')
_SCALAR_TYPES = frozenset({'String', 'i64', 'f64', 'bool', 'serde_json::Value'})
_PARSE_EXPR = {
    'i64': '{raw}.as_str().parse::<i64>().unwrap_or_default()',
    'f64': '{raw}.as_str().parse::<f64>().unwrap_or_default()',
    'bool': '({raw}.as_str() == "true")',
    'String': '{raw}.as_str().to_string()'}

def _string_literal(text: str) -> str:
    escaped = text.replace(chr(92), chr(92) * 2).replace('"', chr(92) + '"')
    escaped = escaped.replace(chr(10), chr(92) + 'n').replace(chr(13), chr(92) + 'r').replace(chr(9), chr(92) + 't')
    return '"' + escaped + '"'

def _to_rust_fqn(fqn: str) -> str:
    """Dotted fqn ('pkg.sub.Class') -> fully-qualified Rust path to the generated
    *type*. Scalars (no dot, e.g. 'String'/'i64'/'serde_json::Value') pass through
    unchanged. Every generated class is its own module (one `pub mod Class;` per
    file, see emit._emit_rust_module_tree), so the item itself sits one level
    below its own module of the same name: 'crate::pkg::sub::Class::Class'."""
    if '.' not in fqn:
        return fqn
    segments = fqn.split('.')
    return 'crate::' + '::'.join(segments) + '::' + segments[-1]

def _to_rust_mod(fqn: str) -> str:
    """Dotted fqn -> fully-qualified Rust path to a *module* (e.g. the JsonSupport
    helper, whose free functions live directly in its own module, with no
    same-named type nested below it -- unlike `_to_rust_fqn`, no segment is
    duplicated."""
    return 'crate::' + fqn.replace('.', '::')

class RustSupport(LanguageSupport):
    name = 'rust'
    extension = 'rs'
    primitive_type = {'string': 'String', 'integer': 'i64', 'number': 'f64', 'boolean': 'bool'}
    any_dictionary_type = 'serde_json::Value'
    parameter_type = {'integer': 'i64', 'number': 'f64', 'boolean': 'bool'}
    parameter_default_type = 'String'
    read_method = {'string': 'as_str', 'integer': 'as_i64', 'number': 'as_f64', 'boolean': 'as_bool'}
    factory_method = {}
    jinja_filters = {'rustfqn': _to_rust_fqn, 'rustmod': _to_rust_mod}

    def string_literal(self, text: str) -> str:
        return _string_literal(text)

    def literal(self, value, primitive_type: str) -> str:
        if primitive_type == 'boolean':
            return 'true' if value else 'false'
        if primitive_type == 'string':
            return f'{_string_literal(str(value))}.to_string()'
        if primitive_type == 'integer':
            return f'{int(value)}i64'
        if primitive_type == 'number':
            return f'{float(value)}f64'
        raise ValueError(f'enum has no supported base primitive type: {primitive_type!r}')

    def type_hint(self, type_name: str) -> str:
        if type_name in _SCALAR_TYPES or '.' not in type_name:
            return type_name
        return _to_rust_fqn(type_name)

    def parameter_declaration(self, type_hint: str, var_name: str, kind: str='path') -> str:
        if kind == 'query':
            return f'{var_name}: Option<{type_hint}>'
        return f'{var_name}: {type_hint}'

    def reference_expr(self, name: str) -> str:
        return name

    def path_url_expression(self, path: str, path_params) -> str:
        by_raw_name = {p.raw_name: p for p in path_params}
        fmt_parts, args, last = ([], [], 0)
        for match in _PATH_PARAM.finditer(path):
            literal = path[last:match.start()]
            if literal:
                fmt_parts.append(literal.replace('{', '{{').replace('}', '}}'))
            param = by_raw_name[match.group(1)]
            fmt_parts.append('{}')
            args.append(f'percent_encode(&{param.name}.to_string())')
            last = match.end()
        tail = path[last:]
        fmt_parts.append(tail.replace('{', '{{').replace('}', '}}'))
        fmt_string = _string_literal(''.join(fmt_parts))
        if not args:
            return f'{fmt_string}.to_string()'
        return f'format!({fmt_string}, {', '.join(args)})'

    def discriminator_literal_expr(self, property_name: str, value, primitive_type: str) -> str:
        accessor = 'self.node.get(' + _string_literal(property_name) + ')'
        if primitive_type == 'string':
            return f'{accessor}.and_then(|v| v.as_str()) == Some({_string_literal(str(value))})'
        if primitive_type == 'integer':
            return f'{accessor}.and_then(|v| v.as_i64()) == Some({int(value)}i64)'
        if primitive_type == 'number':
            return f'{accessor}.and_then(|v| v.as_f64()) == Some({float(value)}f64)'
        if primitive_type == 'boolean':
            literal = 'true' if value else 'false'
            return f'{accessor}.and_then(|v| v.as_bool()) == Some({literal})'
        raise ValueError(f'discriminator property has no supported primitive type: {primitive_type!r}')

    def applies_always(self) -> str:
        return 'true'

    def applies_required(self, fields: list) -> str:
        return ' && '.join((f'self.node.get({_string_literal(field_name)}).is_some()' for field_name in fields))

    def applies_array(self) -> str:
        return 'self.node.is_array()'

    def applies_object(self) -> str:
        return 'self.node.is_object()'

    def applies_type_check(self, primitive_type: str) -> str:
        return {
            'string': 'self.node.is_string()',
            'integer': '(self.node.is_i64() || self.node.is_u64())',
            'number': 'self.node.is_number()',
            'boolean': 'self.node.is_boolean()'}[primitive_type]

    def applies_null(self) -> str:
        return 'self.node.is_null()'

    def build_binding_lines(self, client_method, regex_order, body_json_support_fqn: str) -> tuple:
        lines = []
        for i, param in enumerate(regex_order, start=1):
            raw = f'percent_decode(caps.get({i}).map(|m| m.as_str()).unwrap_or(""))'
            lines.append(f'let {param.name}: {param.java_type} = {_PARSE_EXPR[param.java_type].format(raw=raw)};')
        for param in client_method.query_params:
            mapper = _PARSE_EXPR[param.java_type].format(raw='r')
            lines.append(
                f'let {
                    param.name}: Option<{
                        param.java_type}> = query_params.get({
                            _string_literal(
                                param.raw_name)}).map(|r| {mapper});')
        if client_method.body_param is not None:
            fqn = self.type_hint(client_method.body_param.java_type)
            lines.append(
                f'let {
                    client_method.body_param.name} = {fqn}::new(serde_json::from_str(body).unwrap_or(serde_json::Value::Null));')
        return tuple(lines)

    def finalize_output(self, output_dir) -> None:
        """Rust has no implicit-namespace-package equivalent: every directory needs
    an explicit `mod.rs` declaring its child modules/files. The output
    directory itself is skipped -- it is not a module boundary, whatever file
    declares it as a module (e.g. main.rs: `mod generated;`) owns that
    declaration."""
        import os
        from pathlib import Path
        output_dir = Path(output_dir)
        for dirpath, dirnames, filenames in os.walk(output_dir, topdown=False):
            if Path(dirpath) == output_dir:
                continue
            rs_files = sorted((f[:-3] for f in filenames if f.endswith('.rs') and f != 'mod.rs'))
            if not rs_files and (not dirnames):
                continue
            lines = ['#![allow(non_snake_case, dead_code, unused)]']
            lines += [f'pub mod {d};' for d in sorted(dirnames)]
            lines += [f'pub mod {f};' for f in rs_files]
            (Path(dirpath) / 'mod.rs').write_text('\n'.join(lines) + '\n', encoding='utf-8')
RUST = RustSupport()