"""Convenience layers ``python_ast_{imports,classes,functions}``.

Thin instantiations of :class:`convenience.BulkCrudTool`; each restricts the
generic bulk CRUD machinery to a node kind. Each MCP tool is backed by a real,
explicitly written module-level function of the same name (not a dynamically
generated wrapper), so ``tool_usage`` can report its real signature/docstring
and ``tool_call`` can inject it as-is.
"""


import ast
from typing import Any, Sequence

from xy.ai.mcpc.tools.registry import ToolRegistry
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.ast.convenience import BulkCrudResult, BulkCrudTool, run_bulk_operation, _import_insert_index


def python_ast_imports(path: str, operation: str, items: Sequence[dict[str, Any]] | None = None) -> BulkCrudResult:
    """Bulk CRUD for imports/modules of a Python file (list/add/remove/replace).

    Args:
        path: Absolute path to the Python file.
        operation: One of ``"list"``, ``"add"``, ``"remove"``, ``"replace"``.
        items: Items to add/remove/replace (ignored for ``"list"``); see
            :func:`~xy.ai.mcpc.tools.ast.convenience.run_bulk_operation` for the
            item shape.

    Returns:
        BulkCrudResult: ``nodes`` for ``"list"``; ``changed`` otherwise.

    Raises:
        core.AstError: See :func:`~xy.ai.mcpc.tools.ast.convenience.run_bulk_operation`.
    """
    return run_bulk_operation(
        path, operation, items, node_types=(ast.Import, ast.ImportFrom), kind_label="import",
        insert_index=_import_insert_index,
    )


def python_ast_classes(path: str, operation: str, items: Sequence[dict[str, Any]] | None = None) -> BulkCrudResult:
    """Bulk CRUD for classes of a Python file from source text (list/add/remove/replace).

    Args:
        path: Absolute path to the Python file.
        operation: One of ``"list"``, ``"add"``, ``"remove"``, ``"replace"``.
        items: Items to add/remove/replace (ignored for ``"list"``); see
            :func:`~xy.ai.mcpc.tools.ast.convenience.run_bulk_operation` for the
            item shape.

    Returns:
        BulkCrudResult: ``nodes`` for ``"list"``; ``changed`` otherwise.

    Raises:
        core.AstError: See :func:`~xy.ai.mcpc.tools.ast.convenience.run_bulk_operation`.
    """
    return run_bulk_operation(path, operation, items, node_types=(ast.ClassDef,), kind_label="class")


def python_ast_functions(path: str, operation: str, items: Sequence[dict[str, Any]] | None = None) -> BulkCrudResult:
    """Bulk CRUD for functions/methods of a Python file from source text (list/add/remove/replace).

    Args:
        path: Absolute path to the Python file.
        operation: One of ``"list"``, ``"add"``, ``"remove"``, ``"replace"``.
        items: Items to add/remove/replace (ignored for ``"list"``); see
            :func:`~xy.ai.mcpc.tools.ast.convenience.run_bulk_operation` for the
            item shape.

    Returns:
        BulkCrudResult: ``nodes`` for ``"list"``; ``changed`` otherwise.

    Raises:
        core.AstError: See :func:`~xy.ai.mcpc.tools.ast.convenience.run_bulk_operation`.
    """
    return run_bulk_operation(
        path, operation, items, node_types=(ast.FunctionDef, ast.AsyncFunctionDef), kind_label="function"
    )


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(
        BulkCrudTool(
            name="python_ast_imports",
            title="Python imports",
            description="Bulk CRUD for imports/modules of a Python file (list/add/remove/replace).",
            node_types=(ast.Import, ast.ImportFrom),
            kind_label="import",
            insert_index=_import_insert_index,
        )
    )
    registry.register(
        BulkCrudTool(
            name="python_ast_classes",
            title="Python classes",
            description="Bulk CRUD for classes of a Python file from source text (list/add/remove/replace).",
            node_types=(ast.ClassDef,),
            kind_label="class",
        )
    )
    registry.register(
        BulkCrudTool(
            name="python_ast_functions",
            title="Python functions",
            description="Bulk CRUD for functions/methods of a Python file from source text (list/add/remove/replace).",
            node_types=(ast.FunctionDef, ast.AsyncFunctionDef),
            kind_label="function",
        )
    )
    functions.register(python_ast_imports)
    functions.register(python_ast_classes)
    functions.register(python_ast_functions)
