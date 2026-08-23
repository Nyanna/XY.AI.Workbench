"""``python-ast-outline`` – compact structural overview of Python files."""

from __future__ import annotations

import ast
import importlib
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from . import core

# ``file-stats`` uses a hyphenated (non-identifier) package name.
compute_file_stats = importlib.import_module(
    "xy.ai.mcpc.tools.file-stats"
).compute_file_stats


def _method_entry(loc: core.Located) -> dict[str, Any]:
    node = loc.node
    return {
        "name": loc.name,
        "qualified_name": loc.qualified_name,
        "lineno": node.lineno,
        "end_lineno": getattr(node, "end_lineno", node.lineno),
        "docstring": core.short_docstring(node),
    }


def _build_outline(tree: ast.Module) -> dict[str, Any]:
    located = core.locate_all(tree)

    imports = [
        {"names": loc.name, "lineno": loc.node.lineno}
        for loc in located
        if isinstance(loc.node, core._IMPORT_TYPES)
    ]

    classes: list[dict[str, Any]] = []
    functions: list[dict[str, Any]] = []
    for loc in located:
        node = loc.node
        if isinstance(node, ast.ClassDef):
            methods = [
                _method_entry(m)
                for m in located
                if m.parent is node and isinstance(m.node, (ast.FunctionDef, ast.AsyncFunctionDef))
            ]
            classes.append(
                {
                    "name": loc.name,
                    "qualified_name": loc.qualified_name,
                    "lineno": node.lineno,
                    "end_lineno": getattr(node, "end_lineno", node.lineno),
                    "docstring": core.short_docstring(node),
                    "methods": methods,
                }
            )
        elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and isinstance(
            loc.parent, ast.Module
        ):
            functions.append(_method_entry(loc))

    return {"imports": imports, "classes": classes, "functions": functions}


def _outline_one(path_str: str) -> dict[str, Any]:
    try:
        path, tree = core.load(path_str)
    except core.AstError as exc:
        return {"path": path_str, "ok": False, "error": str(exc)}
    outline = {"stats": compute_file_stats(path), **_build_outline(tree)}
    return {"path": path_str, "ok": True, "error": None, **outline}


_OUTLINE_ITEM_SCHEMA = {
    "type": "object",
    "properties": {
        "path": {"type": "string"},
        "ok": {"type": "boolean"},
        "error": {"type": ["string", "null"]},
        "stats": {"type": "object", "description": "File-metrics block."},
        "imports": {
            "type": "array",
            "description": "Imports with line numbers.",
            "items": {
                "type": "object",
                "properties": {
                    "names": {"type": "string"},
                    "lineno": {"type": "integer"},
                },
                "required": ["names", "lineno"],
            },
        },
        "classes": {
            "type": "array",
            "description": "Top-level classes with nested methods.",
            "items": {"type": "object"},
        },
        "functions": {
            "type": "array",
            "description": "Top-level functions.",
            "items": {"type": "object"},
        },
    },
    "required": ["path", "ok", "error"],
}


def register(registry: ToolRegistry) -> None:
    @registry.tool(
        "python-ast-outline",
        title="Python outline",
        description=(
            "Token-efficient structural overview of Python files: file metrics, "
            "imports, and a class/function hierarchy with line ranges and short "
            "docstrings. Accepts one or several files at once."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "paths": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Absolute paths of Python files to outline.",
                }
            },
            "required": ["paths"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "all_ok": {"type": "boolean"},
                "files": {"type": "array", "items": _OUTLINE_ITEM_SCHEMA},
            },
            "required": ["all_ok", "files"],
        },
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def outline(ctx: ToolContext) -> ToolResult:
        paths = ctx.arguments["paths"]
        if not isinstance(paths, list) or not paths:
            return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
        files = [_outline_one(p) for p in paths]
        return ToolResult(
            structured_content={"all_ok": all(f["ok"] for f in files), "files": files},
            auto_approve=True,
        )
