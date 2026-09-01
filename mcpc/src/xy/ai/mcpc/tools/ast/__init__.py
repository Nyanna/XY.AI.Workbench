"""``ast_*`` tool family built on the standard-library ``ast`` module.

A content-hash validated cache (:mod:`.core`) holds parsed modules; comments are
converted to standalone string-literal annotations on import so they survive the
``parse``/``unparse`` round-trip. Retrieval is layered on a
single ``list`` tree (``ast_list`` structure, ``ast_find`` property/text/regexp
filtering with source, ``ast_read`` reads subtrees by id/FQN); mutation is
node-level CRUD, each tool in its own ``*`` module (``ast_create``/``ast_delete``
cover the whole-file case too), with two in-node editors ``ast_edit_marks``
(marker-delimited) and ``ast_edit_block`` (exact block), a restricted ``script``
and a ``validate`` compile check.
"""


from xy.ai.mcpc.tools.tool_registry import ToolRegistry
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.ast import (
    create,
    delete,
    edit_block,
    edit_marks,
    find,
    insert,
    list,
    read,
    replace,
    script,
    validate,
)

__all__ = ["register_ast_tools", "ALIAS"]
#: Alias name that activates the whole family in one go.
ALIAS = "ast"
_ALIAS_MEMBERS = (
    "ast_list",
    "ast_find",
    "ast_read",
    "ast_insert",
    "ast_edit_marks",
    "ast_edit_block",
    "ast_replace",
    "ast_delete",
    "ast_create",
    "ast_script",
    "ast_validate",
)


def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    """Register every ``ast_*`` tool and the ``ast`` alias."""


    list.register(registry, functions)
    find.register(registry, functions)
    read.register(registry, functions)
    insert.register(registry, functions)
    edit_marks.register(registry, functions)
    edit_block.register(registry, functions)
    replace.register(registry, functions)
    delete.register(registry, functions)
    create.register(registry, functions)
    script.register(registry, functions)
    validate.register(registry, functions)

    registry.register_alias(ALIAS, _ALIAS_MEMBERS)
