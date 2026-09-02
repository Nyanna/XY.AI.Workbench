"""Node classification, formatting and statement-grouping helpers for the Python engine.

``import ast`` here resolves to the standard-library module (absolute import),
not the ``ast`` tool package.
"""
from __future__ import annotations
import ast
from dataclasses import dataclass
_DEF_TYPES = (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)
_IMPORT_TYPES = (ast.Import, ast.ImportFrom)

def import_names(node: ast.Import | ast.ImportFrom) -> str:
    """Return a compact, canonical description of an import statement."""
    if isinstance(node, ast.Import):
        return ', '.join((a.name + (f' as {a.asname}' if a.asname else '') for a in node.names))
    module = '.' * node.level + (node.module or '')
    imported = ', '.join((a.name + (f' as {a.asname}' if a.asname else '') for a in node.names))
    return f'{module}:{imported}'

def _only_defs(body: list[ast.stmt]) -> bool:
    return bool(body) and all((isinstance(n, _DEF_TYPES) for n in body))

def _is_expandable(node: ast.FunctionDef | ast.AsyncFunctionDef | ast.ClassDef) -> bool:
    """Whether ``read``/``list`` should descend into ``node`` instead of returning its full source.

    A class is worth descending into as soon as it nests a def, even alongside
    plain attributes/statements; a function only if its body is nothing but
    nested defs (otherwise it's a small enough unit to show whole).
    """
    if isinstance(node, ast.ClassDef):
        return any((isinstance(n, _DEF_TYPES) for n in node.body))
    return _only_defs(node.body)

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
