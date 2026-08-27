"""``python-ast-*`` tool family built on the standard-library ``ast`` module.

A content-hash validated cache (:mod:`.core`) holds parsed modules; comments are
converted to standalone string-literal annotations on import so they survive the
``parse``/``unparse`` round-trip. The tools cover a structural ``outline``,
node-level CRUD, whole-file create/delete, the imports/classes/functions
convenience layers, a node-scoped ``replace-block``, a restricted ``script`` and
a ``validate`` compile check.

Call :func:`register_ast_tools` to register the whole family and expose it under
the generic tool-set alias ``python-ast``.
"""

from __future__ import annotations

from xy.ai.mcpc.tools.registry import ToolRegistry
from . import crud, file_ops, layers, node_replace_block, outline, script, validate

#: Alias name that activates the whole family in one go.
ALIAS = "python-ast"


def register_ast_tools(registry: ToolRegistry) -> None:
    """Register every ``python-ast-*`` tool and the ``python-ast`` alias."""
    before = set(registry.names())

    outline.register(registry)
    crud.register(registry)
    file_ops.register(registry)
    layers.register(registry)
    node_replace_block.register(registry)
    script.register(registry)
    validate.register(registry)

    added = [n for n in registry.names() if n not in before]
    registry.register_alias(ALIAS, added)


__all__ = ["register_ast_tools", "ALIAS"]
