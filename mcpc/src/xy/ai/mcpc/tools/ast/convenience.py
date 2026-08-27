"""Generic bulk CRUD convenience layer used by imports / classes / functions.

All three tools share the same shape: an ``operation`` plus a list of ``items``.
They are thin wrappers that manipulate the typed AST through :mod:`core`, so a
single generic :class:`BulkCrudTool` here keeps them consistent and DRY (see
:mod:`layers` for the three concrete instantiations).
"""

from __future__ import annotations

import ast
from dataclasses import dataclass
from typing import Any, Callable, Sequence

from xy.ai.mcpc.tools.registry import ToolDefinition, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from . import core

__all__ = ["BulkCrudResult", "run_bulk_operation", "BulkCrudTool"]

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


@dataclass(frozen=True)
class BulkCrudResult:
    """Result of :func:`run_bulk_operation`.

    Attributes:
        result: Always ``"success"``.
        nodes: Node summaries; set only when ``operation`` is ``"list"``.
        changed: Number of items added/removed/replaced; set for every operation
            except ``"list"``.
    """

    result: str
    nodes: list[dict[str, Any]] | None = None
    changed: int | None = None


def run_bulk_operation(
    path: str,
    operation: str,
    items: Sequence[dict[str, Any]] | None = None,
    *,
    node_types: tuple[type, ...],
    kind_label: str,
    insert_index: Callable[[ast.Module], int] = _default_insert_index,
) -> BulkCrudResult:
    """Apply a bulk ``list``/``add``/``remove``/``replace`` operation restricted to ``node_types``.

    Args:
        path: Absolute path to the Python file.
        operation: One of ``"list"``, ``"add"``, ``"remove"``, ``"replace"``.
        items: Items to add/remove/replace (ignored for ``"list"``); each item is a
            mapping that may carry ``code`` (required for ``"add"``/``"replace"``) plus
            any of the node selectors ``qualified_name``, ``name``, ``node_type``,
            ``lineno``, ``parent_type`` (required to uniquely identify the target for
            ``"remove"``/``"replace"``).
        node_types: AST node classes this operation is restricted to, e.g.
            ``(ast.ClassDef,)``.
        kind_label: Human-readable label used in error messages, e.g. ``"class"``.
        insert_index: Computes the insertion index used by ``"add"``; defaults to
            appending at the end of the module body.

    Returns:
        BulkCrudResult: ``nodes`` is populated for ``"list"``; ``changed`` for the
        other three operations.

    Raises:
        core.AstError: If ``path`` is invalid, an item is missing ``code`` (for
            ``"add"``/``"replace"``), an item's ``code`` does not parse to a node of
            ``node_types``, a selector matches zero or more than one node (for
            ``"remove"``/``"replace"``), or ``operation`` is not one of the four
            supported values.
    """
    items = list(items or [])

    def _is_kind(node: ast.AST) -> bool:
        return isinstance(node, node_types)

    def _list(tree: ast.Module) -> list[dict[str, Any]]:
        return [core.node_summary(loc) for loc in core.locate_all(tree) if _is_kind(loc.node)]

    def _resolve(tree: ast.Module, item: dict[str, Any]) -> core.Located:
        hits = [h for h in core.find(tree, **_selectors(item)) if _is_kind(h.node)]
        if not hits:
            raise core.AstError(f"No {kind_label} matched a selector.")
        if len(hits) > 1:
            raise core.AstError(f"A {kind_label} selector is ambiguous.")
        return hits[0]

    def _parse_items(subset: Sequence[dict[str, Any]]) -> list[ast.stmt]:
        nodes: list[ast.stmt] = []
        for item in subset:
            code = item.get("code")
            if not code:
                raise core.AstError("Item is missing 'code'.")
            parsed = core.parse_snippet(code)
            for node in parsed:
                if not _is_kind(node):
                    raise core.AstError(f"Item 'code' is not a {kind_label}.")
            nodes.extend(parsed)
        return nodes

    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)

    if operation == "list":
        return BulkCrudResult(result="success", nodes=_list(tree))

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
    else:
        raise core.AstError("Unknown operation.")

    core.CACHE.save(file_path, tree)
    return BulkCrudResult(result="success", changed=changed)


class BulkCrudTool(ToolDefinition):
    """Generic ``list``/``add``/``remove``/``replace`` tool restricted to a node kind.

    One instance is created per node kind (see :mod:`layers`); every instance
    delegates to :func:`run_bulk_operation` with its own ``node_types``,
    ``kind_label`` and ``insert_index``.
    """

    def __init__(
        self,
        *,
        name: str,
        title: str,
        description: str,
        node_types: tuple[type, ...],
        kind_label: str,
        insert_index: Callable[[ast.Module], int] = _default_insert_index,
    ) -> None:
        self.name = name
        self.title = title
        self.description = description
        self._node_types = node_types
        self._kind_label = kind_label
        self._insert_index = insert_index
        self.input_schema = {
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
        }
        self.output_schema = {
            "type": "object",
            "properties": {
                "result": {"type": "string"},
                "nodes": {"type": "array", "items": {"type": "object"}},
                "changed": {"type": "integer"},
            },
            "required": ["result"],
        }
        self.annotations = {"readOnlyHint": False, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`run_bulk_operation`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = run_bulk_operation(
                args["path"],
                args["operation"],
                args.get("items"),
                node_types=self._node_types,
                kind_label=self._kind_label,
                insert_index=self._insert_index,
            )
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)

        structured: dict[str, Any] = {"result": result.result}
        if result.nodes is not None:
            structured["nodes"] = result.nodes
        if result.changed is not None:
            structured["changed"] = result.changed
        return ToolResult(structured_content=structured, auto_approve=result.nodes is None)
