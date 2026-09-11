"""Universal whole-file fallback for extensions no tree-sitter grammar claims
(see :mod:`xy.ai.mcpc.tools.ast.generic._engine` for the engine powering every
language/format that *is* supported).

Rather than reject the file, :class:`PlainTextEngine` exposes it as a single
addressable node named ``"file"`` spanning the whole content; every ``ast_*``
tool keeps working on it, just without structural granularity below the file
itself (replace/insert/delete/append all act on the entire text).
"""
from __future__ import annotations
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.ast.base import Engine, Located, Tree
__all__ = ['PlainTextEngine']
'#: Fixed id/type of the single node a whole file collapses into.'
_NODE_ID = 'file'

def _end_lineno(source: str) -> int:
    if not source:
        return 1
    return source.count('\n') + (0 if source.endswith('\n') else 1)

class PlainTextEngine(Engine):
    """Treats an entire file as one opaque node; the fallback for every
    extension neither the Python nor a tree-sitter engine handles."""
    name = 'text'

    def parse(self, source: str, path: Path | None=None) -> Tree:
        return Tree(self, source, source, path)

    def empty_tree(self, path: Path | None=None) -> Tree:
        return Tree(self, '', '', path)

    def serialize(self, tree: Tree) -> str:
        return tree.source

    def validate(self, source: str) -> str | None:
        return None

    def locate_all(self, tree: Tree) -> list[Located]:
        if not tree.source:
            return []
        return [
            Located(
                tree=tree,
                node=tree.source,
                parent=None,
                index=0,
                node_id=_NODE_ID,
                node_type=_NODE_ID,
                name=tree.path.name if tree.path else None,
                lineno=1,
                end_lineno=_end_lineno(
                    tree.source),
                parent_type=None,
                expandable=False)]

    def signature(self, node: Any, limit: int=80) -> str:
        first_line = node.splitlines()[0].strip() if node else ''
        return first_line if len(first_line) <= limit else first_line[:limit - 1] + '…'

    def docstring(self, node: Any) -> str | None:
        return None

    def node_code(self, node: Any) -> str:
        return node

    def replace(self, loc: Located, code: str) -> None:
        loc.tree.source = code
        loc.tree.raw = code

    def insert(self, loc: Located, code: str, position: str) -> int:
        if position == 'before':
            loc.tree.source = code + '\n' + loc.tree.source
        else:
            loc.tree.source = loc.tree.source + '\n' + code
        loc.tree.raw = loc.tree.source
        return 1

    def delete(self, loc: Located) -> None:
        loc.tree.source = ''
        loc.tree.raw = ''

    def append(self, tree: Tree, code: str) -> int:
        sep = '' if not tree.source or tree.source.endswith('\n') else '\n'
        tree.source = tree.source + sep + code
        tree.raw = tree.source
        return 1