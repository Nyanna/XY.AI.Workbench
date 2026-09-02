"""``PythonEngine``: comment-preserving parse and ``unparse``-based serialisation/mutation.

Mutations edit the ``ast`` object graph in place and are re-serialised via
``unparse``.
"""
from __future__ import annotations
import ast
import autopep8
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, SEGMENT_MAX_CHARS, Tree, id_segment
from xy.ai.mcpc.tools.ast.python._comments import comments_to_annotations
from xy.ai.mcpc.tools.ast.python._nodes import _DEF_TYPES, _IMPORT_TYPES, _StatementGroup, _decorators, _is_expandable

class _FormattingUnparser(ast._Unparser):
    """``ast.unparse`` variant that reflows overlong single-line literals.

    ``ast.unparse`` always renders collections (dicts, lists, ...) on one
    line. For a top-level node of ``INTERCEPT_TYPES`` whose single-line
    rendering exceeds ``MAX_LINE_LENGTH``, the whole rendered subtree is
    reformatted at once via autopep8, using the real prefix already written
    on the line so continuation lines get correctly aligned. Nested
    ``INTERCEPT_TYPES`` nodes are not reformatted individually: autopep8
    already reflows them as part of their enclosing literal.
    """
    MAX_LINE_LENGTH = 100
    INTERCEPT_TYPES = (ast.Dict, ast.List, ast.Set, ast.Tuple, ast.Call)

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._formatting = False

    def traverse(self, node):
        if isinstance(node, list) or self._formatting or (not isinstance(node, self.INTERCEPT_TYPES)):
            super().traverse(node)
            return
        start = len(self._source)
        self._formatting = True
        try:
            super().traverse(node)
        finally:
            self._formatting = False
        text = ''.join(self._source[start:])
        if '\n' in text:
            return
        line_so_far = ''.join(self._source[:start]).rsplit('\n', 1)[-1]
        indent = line_so_far[:len(line_so_far) - len(line_so_far.lstrip(' '))]
        prefix = line_so_far[len(indent):]
        if len(indent) + len(prefix) + len(text) <= self.MAX_LINE_LENGTH:
            return
        formatted = autopep8.fix_code(prefix + text, options={'max_line_length': max(1, self.MAX_LINE_LENGTH - len(indent)), 'aggressive': 1}).rstrip('\n')
        first_line, _, rest = formatted.partition('\n')
        if not first_line.startswith(prefix):
            return
        continuation = ''.join((f'\n{indent}{line}' for line in rest.split('\n'))) if rest else ''
        self._source[start:] = [first_line[len(prefix):] + continuation]

def _unparse(node: ast.AST) -> str:
    return _FormattingUnparser().visit(node)

class PythonEngine(Engine):
    """``ast``-based engine: comment-preserving parse, ``unparse`` serialisation."""
    name = 'python'

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
        return Located(tree=tree, node=node, parent=parent, index=index, node_id=nid, node_type=node_type, name=name, lineno=node.lineno, end_lineno=getattr(node, 'end_lineno', node.lineno), parent_type=type(parent).__name__, expandable=expandable)

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