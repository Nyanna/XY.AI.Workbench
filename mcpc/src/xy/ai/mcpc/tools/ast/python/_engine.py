"""``PythonEngine``: comment-preserving parse and ``unparse``-based serialisation/mutation.

Mutations edit the ``ast`` object graph in place and are re-serialised via
``unparse``.
"""
from __future__ import annotations
import ast
import autopep8
import logging
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, SEGMENT_MAX_CHARS, Tree, id_segment
from xy.ai.mcpc.tools.ast.python._comments import comments_to_annotations
from xy.ai.mcpc.tools.ast.python._nodes import _DEF_TYPES, _IMPORT_TYPES, _StatementGroup, _decorators, _is_expandable
logger = logging.getLogger('xy.ai.mcpc.tools.ast.python')

class _FormattingUnparser(ast._Unparser):
    """``ast.unparse`` variant that reflows overlong single-line statements.

    ``ast.unparse`` always renders simple statements (assignments, returns,
    ...) on one line. Before writing a statement, it is unparsed in
    isolation (independent of the live buffer) to measure its real, final
    width at the current indent depth and, if too long, reformatted as a
    whole via autopep8. Compound statements (``if``/``def``/``class``/...)
    are left untouched: their own rendering already spans multiple lines,
    so the "single line" check naturally excludes them.
    """
    MAX_LINE_LENGTH = 120

    def traverse(self, node):
        if isinstance(node, list) or not isinstance(node, ast.stmt):
            super().traverse(node)
            return
        rendered = ast.unparse(node)
        if '\n' in rendered:
            super().traverse(node)
            return
        indent = '    ' * self._indent
        if len(indent) + len(rendered) <= self.MAX_LINE_LENGTH:
            super().traverse(node)
            return
        formatted = self._fix_code(rendered, max(1, self.MAX_LINE_LENGTH - len(indent)), node)
        if formatted is None:
            super().traverse(node)
            return
        lines = formatted.split('\n')
        self.fill(lines[0])
        for line in lines[1:]:
            self.write('\n' + indent + line)

    def _fix_code(self, code: str, max_line_length: int, node: ast.AST) -> str | None:
        options = {'max_line_length': max_line_length, 'indent_size': 2}
        for aggressive in (2, 1, 0):
            try:
                return autopep8.fix_code(code, options={**options, 'aggressive': aggressive}).rstrip('\n')
            except Exception:
                continue
        logger.error('autopep8 failed to format node at line %s, col %s; leaving unformatted',
                     getattr(node, 'lineno', '?'), getattr(node, 'col_offset', '?'))
        return None

def _unparse(node: ast.AST) -> str:
    return _FormattingUnparser().visit(node)

class PythonEngine(Engine):
    """``ast``-based engine: comment-preserving parse, ``unparse`` serialisation."""
    name = 'python'
    validates_syntax = True

    def parse(self, source: str, path: Path | None=None) -> Tree:
        return Tree(self, self._parse_module(source), source, path)

    def _parse_module(self, source: str) -> ast.Module:
        try:
            return ast.parse(comments_to_annotations(source))
        except SyntaxError as exc:
            raise AstError(f'Syntax error: {exc.msg} (line {exc.lineno})') from exc

    def _parse_fragment(self, code: str) -> list[ast.stmt]:
        return self._parse_module(code).body

    def empty_tree(self, path: Path | None=None) -> Tree:
        return Tree(self, ast.Module(body=[], type_ignores=[]), '', path)

    def serialize(self, tree: Tree) -> str:
        return _unparse(ast.fix_missing_locations(tree.raw))

    def validate(self, source: str) -> str | None:
        try:
            compile(source, '<validate>', 'exec')
        except SyntaxError as exc:
            return f'{exc.msg} (line {exc.lineno})'
        return None

    def _loc(self, tree, node, parent, index, name, nid, expandable=False) -> Located:
        node_type = node.kind if isinstance(node, _StatementGroup) else type(node).__name__
        return Located(
            tree=tree,
            node=node,
            parent=parent,
            index=index,
            node_id=nid,
            node_type=node_type,
            name=name,
            lineno=node.lineno,
            end_lineno=getattr(
                node,
                'end_lineno',
                node.lineno),
            parent_type=type(parent).__name__,
            expandable=expandable)

    def locate_all(self, tree: Tree) -> list[Located]:
        results: list[Located] = []

        def walk(container: ast.AST, path: str) -> None:
            used: dict[str, int] = {}
            body = getattr(container, 'body', [])
            i = 0
            while i < len(body):
                node = body[i]
                if isinstance(node, _DEF_TYPES):
                    seg = id_segment(node.name, i, used)
                    nid = f'{path}.{seg}' if path else seg
                    results.append(self._loc(tree, node, container, i, node.name, nid, _is_expandable(node)))
                    walk(node, nid)
                    i += 1
                    continue
                start = i
                kind = 'imports' if isinstance(node, _IMPORT_TYPES) else 'statements'
                length = 0
                while i < len(body):
                    current = body[i]
                    if isinstance(current, _DEF_TYPES):
                        break
                    current_kind = 'imports' if isinstance(current, _IMPORT_TYPES) else 'statements'
                    if current_kind != kind:
                        break
                    piece = len(self.node_code(current))
                    if i > start and length + piece > SEGMENT_MAX_CHARS:
                        break
                    length += piece
                    i += 1
                group = _StatementGroup(container, start, i, kind)
                seg = id_segment(None, start, used, content=self.node_code(group))
                nid = f'{path}.{seg}' if path else seg
                results.append(self._loc(tree, group, container, start, None, nid))
        walk(tree.raw, '')
        return results

    def is_definition(self, node_type: str) -> bool:
        return node_type in ('FunctionDef', 'AsyncFunctionDef', 'ClassDef')

    def signature(self, node: Any, limit: int=80) -> str:
        if isinstance(node, _StatementGroup):
            first_line = (self.node_code(node).splitlines() or [''])[0]
            return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            keyword = 'async def' if isinstance(node, ast.AsyncFunctionDef) else 'def'
            returns = f' -> {ast.unparse(node.returns)}' if node.returns is not None else ''
            return f'{_decorators(node)}{keyword} {node.name}({ast.unparse(node.args)}){returns}:'
        if isinstance(node, ast.ClassDef):
            bases = [ast.unparse(b) for b in node.bases] + [f'{kw.arg}={ast.unparse(kw.value)}' for kw in node.keywords]
            bases_str = f'({', '.join(bases)})' if bases else ''
            return f'{_decorators(node)}class {node.name}{bases_str}:'
        first_line = ast.unparse(node).splitlines()[0]
        return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'

    def docstring(self, node: Any, limit: int=80) -> str | None:
        if not isinstance(node, (ast.Module, ast.ClassDef, *_DEF_TYPES)):
            return None
        doc = ast.get_docstring(node, clean=True)
        if doc is None:
            return None
        doc = ' '.join(doc.split())
        return doc if len(doc) <= limit else doc[:limit - 1] + '…'

    def node_code(self, node: Any) -> str:
        if isinstance(node, _StatementGroup):
            return '\n'.join((_unparse(ast.fix_missing_locations(s)) for s in node.stmts))
        return _unparse(ast.fix_missing_locations(node))

    def replace(self, loc: Located, code: str) -> None:
        node = loc.node
        if isinstance(node, _StatementGroup):
            node.parent.body[node.start:node.stop] = self._parse_fragment(code)
        else:
            loc.parent.body[loc.index:loc.index + 1] = self._parse_fragment(code)

    def insert(self, loc: Located, code: str, position: str) -> int:
        stmts = self._parse_fragment(code)
        node = loc.node
        if isinstance(node, _StatementGroup):
            body = node.parent.body
            index = node.stop if position == 'after' else node.start
        else:
            body = loc.parent.body
            offset = 1 if position == 'after' else 0
            index = body.index(loc.node) + offset
        body[index:index] = stmts
        return len(stmts)

    def delete(self, loc: Located) -> None:
        node = loc.node
        if isinstance(node, _StatementGroup):
            del node.parent.body[node.start:node.stop]
        else:
            del loc.parent.body[loc.index]

    def append(self, tree: Tree, code: str) -> int:
        stmts = self._parse_fragment(code)
        tree.raw.body.extend(stmts)
        return len(stmts)
'# Shared instance; the Python engine is stateless.'
ENGINE = PythonEngine()