"""``python_ast_*`` tool family built on the standard-library ``ast`` module.

A content-hash validated cache (:mod:`.core`) holds parsed modules; comments are
converted to standalone string-literal annotations on import so they survive the
``parse``/``unparse`` round-trip. The tools cover a structural ``outline``,
node-level CRUD, whole-file create/delete, the imports/classes/functions
convenience layers, a node-scoped ``replace_block``, a restricted ``script`` and
a ``validate`` compile check.

Call :func:`register_ast_tools` to register the whole family and expose it under
the generic tool-set alias ``python-ast``.
"""


from xy.ai.mcpc.tools.registry import ToolRegistry
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.ast import crud, file_ops, outline, script, validate

__all__ = ["register_ast_tools", "ALIAS"]
#: Alias name that activates the whole family in one go.
ALIAS = "python-ast"
_ALIAS_MEMBERS = ("python_ast_outline", "python_ast_list", "python_ast_find", "python_ast_insert", "python_ast_replace", "python_ast_delete", "python_ast_create","python_ast_create_file","python_ast_delete_file",
                  "python_ast_imports"), "python_ast_classes", "python_ast_functions", "python_ast_replace_block", "python_ast_validate"


def register_ast_tools(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    """Register every ``python_ast_*`` tool and the ``python-ast`` alias."""

    outline.register(registry, functions)
    crud.register(registry, functions)
    file_ops.register(registry, functions)
    script.register(registry, functions)
    validate.register(registry, functions)

    registry.register_alias(ALIAS, _ALIAS_MEMBERS)
