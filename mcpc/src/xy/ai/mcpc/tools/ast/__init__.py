"""``ast_*`` tool family built on the standard-library ``ast`` module.

A content-hash validated cache (:mod:`.core`) holds parsed modules; comments are
converted to standalone string-literal annotations on import so they survive the
``parse``/``unparse`` round-trip. The tools cover a structural ``outline``,
node-level CRUD (each tool in its own ``*`` module, ``ast_create``/``ast_delete``
covering the whole-file case too), the imports/classes/functions convenience
layers, a node-scoped ``replace_block``, a restricted ``script`` and a
``validate`` compile check.
"""


from xy.ai.mcpc.tools.tool_registry import ToolRegistry
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.ast import delete, edit, validate, read, create, insert, script, replace, outline, find, list

__all__ = ["register_ast_tools", "ALIAS"]
#: Alias name that activates the whole family in one go.
ALIAS = "ast"
_ALIAS_MEMBERS = (
    "ast_outline",
    "ast_list",
    "ast_find",
    "ast_read",
    "ast_insert",
    "ast_edit",
    "ast_replace",
    "ast_delete",
    "ast_create",
    "ast_imports",
    "ast_classes",
    "ast_functions",
    "ast_replace_block",
    "ast_validate",
)


def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    """Register every ``ast_*`` tool and the ``ast`` alias."""

    outline.register(registry, functions)
    list.register(registry, functions)
    find.register(registry, functions)
    read.register(registry, functions)
    insert.register(registry, functions)
    edit.register(registry, functions)
    replace.register(registry, functions)
    delete.register(registry, functions)
    create.register(registry, functions)
    script.register(registry, functions)
    validate.register(registry, functions)

    registry.register_alias(ALIAS, _ALIAS_MEMBERS)
