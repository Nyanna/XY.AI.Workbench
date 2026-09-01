"""Python back-end for the ``ast_*`` tools, built on the standard-library ``ast``.

Existing ``#`` comments are converted into standalone string-literal annotations
(:func:`comments_to_annotations`) before parsing so they survive the round-trip
through :func:`ast.parse` / :func:`ast.unparse`. Mutations edit the ``ast``
object graph in place and are re-serialised via ``unparse``.

``import ast`` here resolves to the standard library module (absolute import),
not the ``ast`` tool package.
"""
from __future__ import annotations
import ast
import io
import re
import tokenize
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.ast.base import AstError, Engine, Located, SEGMENT_MAX_CHARS, Tree, id_segment
_DEF_TYPES = (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)
_IMPORT_TYPES = (ast.Import, ast.ImportFrom)
_CONTINUATION_HEADER_RE = re.compile('^\\s*(elif|else|except|finally)\\b')
_CASE_HEADER_RE = re.compile('^\\s*case\\b.*:\\s*(#.*)?$')

def _annotation_literal(comment: str) -> str:
    return repr(comment.rstrip())

def _is_continuation_header(line: str) -> bool:
    return bool(_CONTINUATION_HEADER_RE.match(line) or _CASE_HEADER_RE.match(line))

def _next_code_line_index(lines: list[str], start: int) -> int | None:
    for i in range(start, len(lines)):
        stripped = lines[i].strip()
        if stripped == '' or stripped.startswith('#'):
            continue
        return i
    return None

def _suite_indent(lines: list[str], header_lineno: int) -> str:
    header_line = lines[header_lineno - 1]
    header_indent = header_line[:len(header_line) - len(header_line.lstrip())]
    for line in lines[header_lineno:]:
        if line.strip() == '':
            continue
        return line[:len(line) - len(line.lstrip())]
    return header_indent + '    '

def comments_to_annotations(source: str) -> str:
    """Rewrite ``#`` comments into standalone string-literal statements.

    A comment on its own line becomes an equally-indented string literal; a
    trailing comment is lifted onto its own literal line in front of the
    statement it belonged to. Comments inside brackets/continuations cannot be
    represented as standalone literals without breaking syntax and are dropped.
    Style and exact placement are explicitly *not* preserved – only semantics
    plus the recovered annotation text. Comments preceding or trailing a
    continuation header (``elif``/``else``/``except``/``finally``/``case``) are
    moved into the suite that header opens.
    """
    if '#' not in source:
        return source
    lines = source.splitlines(keepends=True)
    replaces: dict[int, str] = {}
    strips: dict[int, int] = {}
    inserts: dict[int, list[str]] = {}
    depth = 0
    logical_start: int | None = None
    try:
        for tok in tokenize.generate_tokens(io.StringIO(source).readline):
            ttype = tok.type
            if ttype == tokenize.NEWLINE:
                logical_start = None
                continue
            if ttype in (tokenize.NL, tokenize.INDENT, tokenize.DEDENT, tokenize.ENCODING, tokenize.ENDMARKER):
                continue
            if ttype == tokenize.COMMENT:
                lineno, col = tok.start
                prefix = lines[lineno - 1][:col]
                standalone = prefix.strip() == ''
                literal = _annotation_literal(tok.string)
                if depth == 0 and standalone and (logical_start is None):
                    next_idx = _next_code_line_index(lines, lineno)
                    if next_idx is not None and _is_continuation_header(lines[next_idx]):
                        header_lineno = next_idx + 1
                        target_indent = _suite_indent(lines, header_lineno)
                        inserts.setdefault(header_lineno + 1, []).append(f'{target_indent}{literal}\n')
                        replaces[lineno] = '\n'
                    else:
                        replaces[lineno] = f'{prefix}{literal}\n'
                elif depth == 0 and (not standalone) and (logical_start is not None):
                    stmt_line = lines[logical_start - 1]
                    if _is_continuation_header(stmt_line):
                        target_indent = _suite_indent(lines, lineno)
                        inserts.setdefault(lineno + 1, []).append(f'{target_indent}{literal}\n')
                    else:
                        indent = stmt_line[:len(stmt_line) - len(stmt_line.lstrip())]
                        inserts.setdefault(logical_start, []).append(f'{indent}{literal}\n')
                    strips[lineno] = col
                elif standalone:
                    replaces[lineno] = '\n'
                else:
                    strips[lineno] = col
                continue
            if logical_start is None:
                logical_start = tok.start[0]
            if ttype == tokenize.OP:
                if tok.string in '([{':
                    depth += 1
                elif tok.string in ')]}':
                    depth = max(0, depth - 1)
    except (tokenize.TokenError, IndentationError):
        return source
    out: list[str] = []
    for i, line in enumerate(lines, start=1):
        if i in inserts:
            out.extend(inserts[i])
        if i in replaces:
            out.append(replaces[i])
        elif i in strips:
            out.append(line[:strips[i]].rstrip() + '\n')
        else:
            out.append(line)
    return ''.join(out)

def import_names(node: ast.Import | ast.ImportFrom) -> str:
    """Return a compact, canonical description of an import statement."""
    if isinstance(node, ast.Import):
        return ', '.join((a.name + (f' as {a.asname}' if a.asname else '') for a in node.names))
    module = '.' * node.level + (node.module or '')
    imported = ', '.join((a.name + (f' as {a.asname}' if a.asname else '') for a in node.names))
    return f'{module}:{imported}'

def _only_defs(body: list[ast.stmt]) -> bool:
    return bool(body) and all((isinstance(n, _DEF_TYPES) for n in body))

def _decorators(node: ast.FunctionDef | ast.AsyncFunctionDef | ast.ClassDef) -> str:
    return ''.join((f'@{ast.unparse(d)} ' for d in node.decorator_list))

@dataclass
class _StatementGroup:
    """A run of consecutive same-kind statements addressed as a single node.

    Individual statements are never addressable on their own: consecutive imports
    collapse into one ``imports`` segment, all other statements into ``statements``
    segments (split once their source would exceed ``SEGMENT_MAX_CHARS``). The group
    stands in for a real ``ast`` node wherever the engine expects one.
    """
    parent: ast.AST
    start: int
    stop: int
    kind: str

    @property
    def stmts(self) -> list[ast.stmt]:
        return self.parent.body[self.start:self.stop]

    @property
    def lineno(self) -> int:
        return self.stmts[0].lineno

    @property
    def end_lineno(self) -> int:
        last = self.stmts[-1]
        return getattr(last, 'end_lineno', last.lineno)

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
        return ast.unparse(ast.fix_missing_locations(tree.raw))

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
                    results.append(self._loc(tree, node, container, i, node.name, nid, _only_defs(node.body)))
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
            return '\n'.join((ast.unparse(ast.fix_missing_locations(s)) for s in node.stmts))
        return ast.unparse(ast.fix_missing_locations(node))

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
'#: Shared instance; the Python engine is stateless.'
ENGINE = PythonEngine()