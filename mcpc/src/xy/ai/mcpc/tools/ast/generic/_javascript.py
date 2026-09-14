"""JavaScript tree-sitter engine: Classes/Functions -> Methods, mirroring the
Java engine's shape but driven by semantics rather than fixed node types -
JS has no single "this is a definition" grammar rule, since a function/class
can just as well be a ``const``'s or a class field's value as a standalone
declaration, and any of that can be wrapped in an ``export``. :func:`_is_def`,
:func:`_def_name` and :func:`_def_body` share one recursive unwrapping so all
of that - ``function foo() {}``, ``export function foo() {}``, ``const foo =
() => {}``, ``export const foo = () => {}``, ``handleClick = () => {}`` class
fields, ``export default class {}`` - resolves to the same definition. Plain
non-function statements (``const x = 1``, control flow, bare ``import``/re-
``export``, ...) are far too fine-grained to be useful individually and
collapse into 'imports'/'statements' segments capped at ``SEGMENT_MAX_CHARS``,
exactly like the Java engine.
"""
from __future__ import annotations
from typing import Any
from xy.ai.mcpc.tools.ast.base import SEGMENT_MAX_CHARS, Located, Tree, id_segment
from xy.ai.mcpc.tools.ast.generic._engine import TreeSitterEngine, _SynthNode
__all__ = ['JavaScriptEngine']
"#: Declaration/definition node types with their own 'name' (and usually 'body') field."
_CORE_DEF_TYPES = {'function_declaration', 'generator_function_declaration', 'class_declaration', 'method_definition'}
'#: Function/class *expression* types - only a definition once assigned to a name (below).'
_FUNC_EXPR_TYPES = {'arrow_function', 'function_expression', 'generator_function', 'class'}
'#: ``let``/``const``/``var`` wrappers - a definition only with exactly one'
'#: declarator whose value is one of ``_FUNC_EXPR_TYPES``.'
_DECL_WRAPPER_TYPES = {'lexical_declaration', 'variable_declaration'}
'#: Node types an individually addressable definition may end up as; used by ``is_definition``.'
_DEF_NODE_TYPES = _CORE_DEF_TYPES | {'field_definition'} | _DECL_WRAPPER_TYPES | {'export_statement'}
_IMPORT_TYPES = {'import_statement'}

def _is_def(node: Any) -> bool:
    """Whether ``node`` is - or wraps, through a single ``export``/``const``/field
    layer - a function, method or class definition."""
    t = node.type
    if t in _CORE_DEF_TYPES:
        return True
    if t == 'field_definition':
        value = node.child_by_field_name('value')
        return value is not None and value.type in _FUNC_EXPR_TYPES
    if t in _DECL_WRAPPER_TYPES:
        declarators = [c for c in node.named_children if c.type == 'variable_declarator']
        if len(declarators) != 1:
            return False
        value = declarators[0].child_by_field_name('value')
        return value is not None and value.type in _FUNC_EXPR_TYPES
    if t == 'export_statement':
        candidates = node.named_children
        if len(candidates) == 1:
            inner = candidates[0]
            return inner.type in _CORE_DEF_TYPES or inner.type in _FUNC_EXPR_TYPES or (
                inner.type in _DECL_WRAPPER_TYPES and _is_def(inner))
    return False

def _def_name(node: Any) -> str | None:
    """Name of a node ``_is_def`` accepted: the declaration's own identifier, or -
    unwrapping one ``export``/``const``/field layer - the name it was assigned to."""
    name = node.child_by_field_name('name')
    if name is not None:
        return name.text.decode('utf-8', 'replace').strip()
    if node.type == 'field_definition':
        prop = node.child_by_field_name('property')
        if prop is not None:
            return prop.text.decode('utf-8', 'replace').strip()
    value = node.child_by_field_name('value')
    if value is not None:
        found = _def_name(value)
        if found:
            return found
    if node.type in _DECL_WRAPPER_TYPES:
        declarators = [c for c in node.named_children if c.type == 'variable_declarator']
        if len(declarators) == 1:
            return _def_name(declarators[0])
    if node.type == 'export_statement' and len(node.named_children) == 1:
        return _def_name(node.named_children[0])
    return None

def _def_body(node: Any) -> Any | None:
    """The innermost 'body' field reachable through the same unwrapping as
    ``_def_name`` - a function/method's block, or a class's body."""
    body = node.child_by_field_name('body')
    if body is not None:
        return body
    value = node.child_by_field_name('value')
    if value is not None and value.type in _FUNC_EXPR_TYPES:
        return _def_body(value) or value
    if node.type in _DECL_WRAPPER_TYPES:
        declarators = [c for c in node.named_children if c.type == 'variable_declarator']
        if len(declarators) == 1:
            return _def_body(declarators[0])
    if node.type == 'export_statement' and len(node.named_children) == 1:
        return _def_body(node.named_children[0])
    return None

class JavaScriptEngine(TreeSitterEngine):
    """Tree-sitter JavaScript restructured like the Python/Java engines:
    functions/classes/methods (however they're declared or exported) as real
    nodes, everything else grouped into statement/import segments."""

    def __init__(self) -> None:
        super().__init__('javascript')

    def is_definition(self, node_type: str) -> bool:
        return node_type in _DEF_NODE_TYPES

    def signature(self, node: Any, limit: int=80) -> str:
        body = _def_body(node)
        end = body.start_byte if body is not None else node.end_byte
        raw = node.text[:end - node.start_byte]
        text = ' '.join(raw.decode('utf-8', 'replace').split())
        return text if len(text) <= limit else text[:limit - 1] + '…'

    def locate_all(self, tree: Tree) -> list[Located]:
        results: list[Located] = []
        source = tree.source.encode('utf-8')

        def walk(children: list[Any], container: Any, path: str) -> None:
            used: dict[str, int] = {}
            i, n = (0, len(children))
            while i < n:
                node = children[i]
                if _is_def(node):
                    name = _def_name(node)
                    seg = id_segment(name, i, used, content=self.node_code(node))
                    nid = f'{path}.{seg}' if path else seg
                    body = _def_body(node)
                    '#: Only class bodies nest further definitions (methods/fields);'
                    "#: a function/method's own statement_block is left as one leaf,"
                    "#: exactly like the Java engine never splits a method's body."
                    inner = body if body is not None and body.type == 'class_body' else None
                    expandable = bool(inner) and any((_is_def(c) for c in inner.named_children))
                    results.append(
                        Located(
                            tree=tree,
                            node=node,
                            parent=container,
                            index=i,
                            node_id=nid,
                            node_type=node.type,
                            name=name,
                            lineno=node.start_point[0] + 1,
                            end_lineno=node.end_point[0] + 1,
                            parent_type=container.type,
                            expandable=expandable))
                    if inner is not None:
                        walk(inner.named_children, inner, nid)
                    i += 1
                    continue
                start = i
                kind = 'imports' if node.type in _IMPORT_TYPES else 'statements'
                length = 0
                while i < n:
                    current = children[i]
                    if _is_def(current):
                        break
                    current_kind = 'imports' if current.type in _IMPORT_TYPES else 'statements'
                    if current_kind != kind:
                        break
                    piece = current.end_byte - current.start_byte
                    if i > start and length + piece > SEGMENT_MAX_CHARS:
                        break
                    length += piece
                    i += 1
                group = _SynthNode(kind, children[start:i], source)
                seg = id_segment(None, start, used, content=self.node_code(group))
                nid = f'{path}.{seg}' if path else seg
                results.append(
                    Located(
                        tree=tree,
                        node=group,
                        parent=container,
                        index=start,
                        node_id=nid,
                        node_type=kind,
                        name=None,
                        lineno=group.start_point[0] + 1,
                        end_lineno=group.end_point[0] + 1,
                        parent_type=container.type,
                        expandable=False))
        walk(tree.raw.root_node.named_children, tree.raw.root_node, '')
        return results