"""``python_ast_outline`` – compact structural overview of Python files."""


import ast
from dataclasses import dataclass, field
from typing import Any

from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.file_stats import  compute_file_stats, FileStatsResult

__all__ = [
    "OutlineError",
    "FileOutline",
    "OutlineResult",
    "python_ast_outline",
    "OutlineTool",
    "register",
]

class OutlineError(Exception):
    """Raised when the outline operation cannot be performed at all."""


@dataclass(frozen=True)
class FileOutline:
    """Structural outline of a single file, as returned by :func:`python_ast_outline`.

    Attributes:
        path: The path exactly as given in the input.
        ok: Whether the file could be read and parsed.
        error: Error message if ``ok`` is ``False``, else ``None``.
        stats: File-metrics block (see the ``file-stats`` tool), only if ``ok``.
        imports: Top-level imports with ``names``/``lineno``, only if ``ok``.
        classes: Top-level classes with nested ``methods``, only if ``ok``.
        functions: Top-level functions, only if ``ok``.
    """

    path: str
    ok: bool
    error: str | None
    stats: FileStatsResult
    imports: list[dict[str, Any]] = field(default_factory=list)
    classes: list[dict[str, Any]] = field(default_factory=list)
    functions: list[dict[str, Any]] = field(default_factory=list)


@dataclass(frozen=True)
class OutlineResult:
    """Result of :func:`python_ast_outline`.

    Attributes:
        all_ok: Whether every file in ``files`` outlined successfully.
        files: One :class:`FileOutline` per input path, in the given order.
    """

    all_ok: bool
    files: list[FileOutline] = field(default_factory=list)


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


def _outline_one(path_str: str) -> FileOutline:
    try:
        path, tree = core.load(path_str)
    except core.AstError as exc:
        return FileOutline(path=path_str, ok=False, error=str(exc))
    outline = _build_outline(tree)
    return FileOutline(
        path=path_str,
        ok=True,
        error=None,
        stats=compute_file_stats(path),
        **outline,
    )


def python_ast_outline(paths: list[str]) -> OutlineResult:
    """Build a structural outline (imports, classes, functions, stats) for each of ``paths``.

    Per-file failures (e.g. a non-existent or unparsable file) are reported inside
    the corresponding :class:`FileOutline` rather than raised; only a malformed
    call (empty ``paths``) raises.

    Args:
        paths: Absolute paths of Python files to outline. Must be non-empty.

    Returns:
        OutlineResult: One :class:`FileOutline` per path, in order, plus an overall
        ``all_ok`` flag.

    Raises:
        OutlineError: If ``paths`` is empty.
    """
    if not paths:
        raise OutlineError("'paths' must be a non-empty list.")
    files = [_outline_one(p) for p in paths]
    return OutlineResult(all_ok=all(f.ok for f in files), files=files)


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


class OutlineTool(ToolDefinition):
    name = "python_ast_outline"
    title = "Python outline"
    description = (
        "Token-efficient structural overview of Python files: file metrics, "
        "imports, and a class/function hierarchy with line ranges and short "
        "docstrings. Accepts one or several files at once."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "paths": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Absolute paths of Python files to outline.",
            }
        },
        "required": ["paths"],
    }
    output_schema = {
        "type": "object",
        "properties": {
            "all_ok": {"type": "boolean"},
            "files": {"type": "array", "items": _OUTLINE_ITEM_SCHEMA},
        },
        "required": ["all_ok", "files"],
    }
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`python_ast_outline`, translating the MCP schema to/from the Python API."""
        paths = ctx.arguments["paths"]
        if not isinstance(paths, list):
            return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
        try:
            result = python_ast_outline(paths)
        except OutlineError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)

        return ToolResult(
            structured_content={
                "all_ok": result.all_ok,
                "files": [f.__dict__ for f in result.files],
            },
        )


def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(OutlineTool())
    functions.register(python_ast_outline)
