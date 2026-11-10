"""Semantic import add/remove on the top-level body of a Python module."""
from __future__ import annotations
import ast
from xy.ai.mcpc.tools.ast.imports import ImportOutcome
from xy.ai.mcpc.tools.ast.python._nodes import _IMPORT_TYPES
__all__ = ['apply_python_imports']
_Key = tuple[str, str, int, str, str | None]

def _key(node: ast.Import | ast.ImportFrom, alias: ast.alias) -> _Key:
    if isinstance(node, ast.Import):
        return ('import', '', 0, alias.name, alias.asname)
    return ('from', node.module or '', node.level, alias.name, alias.asname)

def _is_future(node: ast.stmt) -> bool:
    return isinstance(node, ast.ImportFrom) and node.module == '__future__' and (node.level == 0)

def _after_docstring(body: list[ast.stmt]) -> int:
    first = body[0] if body else None
    is_doc = isinstance(
        first,
        ast.Expr) and isinstance(
            first.value,
            ast.Constant) and isinstance(
                first.value.value,
        str)
    return 1 if is_doc else 0

def _insert(body: list[ast.stmt], node: ast.Import | ast.ImportFrom) -> None:
    last = last_future = None
    for i, stmt in enumerate(body):
        if isinstance(stmt, _IMPORT_TYPES):
            last = i
        if _is_future(stmt):
            last_future = i
    if _is_future(node):
        index = _after_docstring(body) if last_future is None else last_future + 1
    else:
        index = _after_docstring(body) if last is None else last + 1
    body.insert(index, node)

def apply_python_imports(body: list[ast.stmt], add: list[tuple[str, ast.Import | ast.ImportFrom]], remove: list[tuple[str, ast.Import | ast.ImportFrom]]) -> ImportOutcome:
    """Apply parsed ``(text, node)`` statements to ``body``; only top-level imports count."""
    outcome = ImportOutcome()
    for text, node in remove:
        wanted = {_key(node, alias) for alias in node.names}
        hit, i = (False, 0)
        while i < len(body):
            stmt = body[i]
            if isinstance(stmt, _IMPORT_TYPES):
                kept = [a for a in stmt.names if _key(stmt, a) not in wanted]
                if len(kept) != len(stmt.names):
                    hit = True
                    if kept:
                        stmt.names = kept
                    else:
                        del body[i]
                        continue
            i += 1
        if hit:
            outcome.removed.append(text)
    existing = {_key(s, a) for s in body if isinstance(s, _IMPORT_TYPES) for a in s.names}
    for text, node in add:
        missing = [a for a in node.names if _key(node, a) not in existing]
        if not missing:
            continue
        node.names = missing
        existing.update((_key(node, a) for a in missing))
        _insert(body, node)
        outcome.added.append(text)
    return outcome