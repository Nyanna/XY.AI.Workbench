"""Convenience layers ``python-ast-{imports,classes,functions}``.

Thin wrappers over :func:`convenience.build_bulk_tool`; each restricts the
generic bulk CRUD machinery to a node kind.
"""

from __future__ import annotations

import ast

from ...registry import ToolRegistry
from .convenience import _import_insert_index, build_bulk_tool


def register(registry: ToolRegistry) -> None:
    build_bulk_tool(
        registry,
        name="python-ast-imports",
        title="Python imports",
        description="Bulk CRUD for imports/modules of a Python file (list/add/remove/replace).",
        node_types=(ast.Import, ast.ImportFrom),
        kind_label="import",
        insert_index=_import_insert_index,
    )
    build_bulk_tool(
        registry,
        name="python-ast-classes",
        title="Python classes",
        description="Bulk CRUD for classes of a Python file from source text (list/add/remove/replace).",
        node_types=(ast.ClassDef,),
        kind_label="class",
    )
    build_bulk_tool(
        registry,
        name="python-ast-functions",
        title="Python functions",
        description="Bulk CRUD for functions/methods of a Python file from source text (list/add/remove/replace).",
        node_types=(ast.FunctionDef, ast.AsyncFunctionDef),
        kind_label="function",
    )
