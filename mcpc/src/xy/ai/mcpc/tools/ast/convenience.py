"""Generic bulk CRUD convenience layer used by imports / classes / functions.

All three tools share the same shape: an ``operation`` plus a list of ``items``.
They are thin wrappers that manipulate the typed AST through :mod:`core`, so a
single generic builder here keeps them consistent and DRY.
"""

from __future__ import annotations

import ast
from typing import Any, Callable, Sequence

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from . import core

#: Item selector fields (a subset of the node-level selectors) plus ``code``.
_ITEM_PROPS = {
    "code": {"type": "string", "description": "Python source (for 'add' / 'replace')."},
    "qualified_name": {"type": "string", "description": "Python-style FQN of the target."},
    "name": {"type": "string", "description": "Simple name of the target."},
    "node_type": {"type": "string", "description": "AST node class name filter."},
    "lineno": {"type": "integer", "description": "Start line of the target."},
    "parent_type": {"type": "string", "description": "AST class name of the container."},
}

_SELECTOR_KEYS = ("qualified_name", "name", "node_type", "lineno", "parent_type")


def _selectors(item: dict[str, Any]) -> dict[str, Any]:
    return {k: item.get(k) for k in _SELECTOR_KEYS}


def _err(exc: core.AstError) -> ToolResult:
    return ToolResult(content=[text_content(str(exc))], is_error=True)


def _default_insert_index(tree: ast.Module) -> int:
    """Append position: end of the module body."""
    return len(tree.body)


def _import_insert_index(tree: ast.Module) -> int:
    """Insert imports after any leading docstring and existing imports."""
    index = 0
    for i, node in enumerate(tree.body):
        if i == 0 and isinstance(node, ast.Expr) and isinstance(node.value, ast.Constant):
            index = 1
            continue
        if isinstance(node, core._IMPORT_TYPES):
            index = i + 1
        else:
            break
    return index


def build_bulk_tool(
    registry: ToolRegistry,
    *,
    name: str,
    title: str,
    description: str,
    node_types: tuple[type, ...],
    kind_label: str,
    insert_index: Callable[[ast.Module], int] = _default_insert_index,
) -> None:
    def _is_kind(node: ast.AST) -> bool:
        return isinstance(node, node_types)

    def _list(tree: ast.Module) -> list[dict[str, Any]]:
        return [
            core.node_summary(loc)
            for loc in core.locate_all(tree)
            if _is_kind(loc.node)
        ]

    def _resolve(tree: ast.Module, item: dict[str, Any]) -> core.Located:
        hits = [h for h in core.find(tree, **_selectors(item)) if _is_kind(h.node)]
        if not hits:
            raise core.AstError(f"No {kind_label} matched a selector.")
        if len(hits) > 1:
            raise core.AstError(f"A {kind_label} selector is ambiguous.")
        return hits[0]

    def _parse_items(items: Sequence[dict[str, Any]]) -> list[ast.stmt]:
        nodes: list[ast.stmt] = []
        for item in items:
            code = item.get("code")
            if not code:
                raise core.AstError("Item is missing 'code'.")
            parsed = core.parse_snippet(code)
            for node in parsed:
                if not _is_kind(node):
                    raise core.AstError(f"Item 'code' is not a {kind_label}.")
            nodes.extend(parsed)
        return nodes

    @registry.tool(
        name,
        title=title,
        description=description,
        input_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "operation": {
                    "type": "string",
                    "enum": ["list", "add", "remove", "replace"],
                    "description": "Bulk operation to apply.",
                },
                "items": {
                    "type": "array",
                    "description": "Items to add / remove / replace (ignored for 'list').",
                    "items": {"type": "object", "properties": _ITEM_PROPS},
                },
            },
            "required": ["path", "operation"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "result": {"type": "string"},
                "nodes": {"type": "array", "items": {"type": "object"}},
                "changed": {"type": "integer"},
            },
            "required": ["result"],
        },
        annotations={"readOnlyHint": False, "openWorldHint": False},
    )
    def handler(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        operation = args["operation"]
        items: list[dict[str, Any]] = args.get("items") or []
        try:
            path = core.require_path(args["path"])
            tree = core.CACHE.get_tree(path)

            if operation == "list":
                return ToolResult(
                    structured_content={"result": "success", "nodes": _list(tree)},
                )

            changed = 0
            if operation == "add":
                nodes = _parse_items(items)
                idx = insert_index(tree)
                tree.body[idx:idx] = nodes
                changed = len(nodes)
            elif operation == "remove":
                for item in items:
                    core.delete_from_body(_resolve(tree, item))
                    changed += 1
            elif operation == "replace":
                for item in items:
                    target = _resolve(tree, item)
                    core.replace_in_body(target, _parse_items([item]))
                    changed += 1
            else:  # pragma: no cover - guarded by enum
                raise core.AstError("Unknown operation.")

            core.CACHE.save(path, tree)
        except core.AstError as exc:
            return _err(exc)

        return ToolResult(
            structured_content={"result": "success", "changed": changed},
            auto_approve=True,
        )
