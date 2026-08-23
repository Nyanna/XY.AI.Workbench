"""Node-level CRUD tools: ``python-ast-{list,find,insert,replace,delete,create}``.

These operate on the typed AST directly and are the foundation the ``imports``,
``classes`` and ``functions`` convenience layers build on.
"""

from __future__ import annotations

import ast
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from . import core

_SELECTOR_PROPS = {
    "qualified_name": {"type": "string", "description": "Python-style FQN of the target node."},
    "name": {"type": "string", "description": "Simple node name."},
    "node_type": {"type": "string", "description": "AST node class name, e.g. 'FunctionDef'."},
    "lineno": {"type": "integer", "description": "Start line of the target node."},
    "end_lineno": {"type": "integer", "description": "End line of the target node."},
    "parent_type": {"type": "string", "description": "AST class name of the container."},
}


def _selectors(args: dict[str, Any]) -> dict[str, Any]:
    return {k: args.get(k) for k in _SELECTOR_PROPS}


def _select_one(tree: ast.Module, args: dict[str, Any]) -> core.Located:
    hits = core.find(tree, **_selectors(args))
    if not hits:
        raise core.AstError("No node matched the selector.")
    if len(hits) > 1:
        raise core.AstError(f"Selector is ambiguous – {len(hits)} nodes matched.")
    return hits[0]


def _err(exc: core.AstError) -> ToolResult:
    return ToolResult(content=[text_content(str(exc))], is_error=True)


def _ok(structured: dict[str, Any]) -> ToolResult:
    return ToolResult(structured_content=structured, auto_approve=True)


def _list_output() -> dict[str, Any]:
    return {
        "type": "object",
        "properties": {
            "nodes": {"type": "array", "items": {"type": "object"}},
            "count": {"type": "integer"},
        },
        "required": ["nodes", "count"],
    }


def register(registry: ToolRegistry) -> None:
    _register_list(registry)
    _register_find(registry)
    _register_insert(registry)
    _register_replace(registry)
    _register_delete(registry)
    _register_create(registry)


def _register_list(registry: ToolRegistry) -> None:
    @registry.tool(
        "python-ast-list",
        title="List AST nodes",
        description="List AST nodes (imports, classes, functions, statements) of a Python file, optionally filtered by type.",
        input_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "code": {"type": "string", "description": "Python source to parse instead of a file."},
                "node_type": {"type": "string", "description": "Restrict to this AST node class name."},
            },
            "required": [],
        },
        output_schema=_list_output(),
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def list_nodes(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            tree = core.tree_from_input(args.get("path"), args.get("code"))
        except core.AstError as exc:
            return _err(exc)
        node_type = args.get("node_type")
        located = core.locate_all(tree)
        summaries = [
            core.node_summary(loc)
            for loc in located
            if node_type is None or type(loc.node).__name__.lower() == node_type.lower()
        ]
        return _ok({"nodes": summaries, "count": len(summaries)})


def _register_find(registry: ToolRegistry) -> None:
    @registry.tool(
        "python-ast-find",
        title="Find AST nodes",
        description="Find AST nodes by type, name, qualified name, line range or parent type.",
        input_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "code": {"type": "string", "description": "Python source to parse instead of a file."},
                **_SELECTOR_PROPS,
            },
            "required": [],
        },
        output_schema=_list_output(),
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def find_nodes(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            tree = core.tree_from_input(args.get("path"), args.get("code"))
        except core.AstError as exc:
            return _err(exc)
        hits = core.find(tree, **_selectors(args))
        return _ok({"nodes": [core.node_summary(h) for h in hits], "count": len(hits)})


def _register_insert(registry: ToolRegistry) -> None:
    @registry.tool(
        "python-ast-insert",
        title="Insert AST node",
        description="Insert statement(s) parsed from code relative to a selected node ('before' or 'after').",
        input_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "code": {"type": "string", "description": "Python source of the statement(s) to insert."},
                "position": {
                    "type": "string",
                    "enum": ["before", "after"],
                    "description": "Placement relative to the selected node.",
                    "default": "after",
                },
                **_SELECTOR_PROPS,
            },
            "required": ["path", "code"],
        },
        output_schema={
            "type": "object",
            "properties": {"result": {"type": "string"}, "inserted": {"type": "integer"}},
            "required": ["result", "inserted"],
        },
        annotations={"readOnlyHint": False, "openWorldHint": False},
    )
    def insert_node(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            path = core.require_path(args["path"])
            tree = core.CACHE.get_tree(path)
            new_nodes = core.parse_snippet(args["code"])
            target = _select_one(tree, args)
            body = target.parent.body  # type: ignore[attr-defined]
            offset = 1 if args.get("position", "after") == "after" else 0
            index = body.index(target.node) + offset
            body[index:index] = new_nodes
            core.CACHE.save(path, tree)
        except core.AstError as exc:
            return _err(exc)
        return _ok({"result": "success", "inserted": len(new_nodes)})


def _register_replace(registry: ToolRegistry) -> None:
    @registry.tool(
        "python-ast-replace",
        title="Replace AST node",
        description="Replace the single selected node with statement(s) parsed from code.",
        input_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "code": {"type": "string", "description": "Replacement Python source."},
                **_SELECTOR_PROPS,
            },
            "required": ["path", "code"],
        },
        output_schema={
            "type": "object",
            "properties": {"result": {"type": "string"}},
            "required": ["result"],
        },
        annotations={"readOnlyHint": False, "openWorldHint": False},
    )
    def replace_node(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            path = core.require_path(args["path"])
            tree = core.CACHE.get_tree(path)
            new_nodes = core.parse_snippet(args["code"])
            target = _select_one(tree, args)
            core.replace_in_body(target, new_nodes)
            core.CACHE.save(path, tree)
        except core.AstError as exc:
            return _err(exc)
        return _ok({"result": "success"})


def _register_delete(registry: ToolRegistry) -> None:
    @registry.tool(
        "python-ast-delete",
        title="Delete AST node",
        description="Delete the single selected node from a Python file.",
        input_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                **_SELECTOR_PROPS,
            },
            "required": ["path"],
        },
        output_schema={
            "type": "object",
            "properties": {"result": {"type": "string"}},
            "required": ["result"],
        },
        annotations={"readOnlyHint": False, "openWorldHint": False},
    )
    def delete_node(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            path = core.require_path(args["path"])
            tree = core.CACHE.get_tree(path)
            target = _select_one(tree, args)
            core.delete_from_body(target)
            core.CACHE.save(path, tree)
        except core.AstError as exc:
            return _err(exc)
        return _ok({"result": "success"})


def _register_create(registry: ToolRegistry) -> None:
    @registry.tool(
        "python-ast-create",
        title="Create AST node",
        description="Append statement(s) parsed from code to a Python file's top level (creating the file if needed).",
        input_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "code": {"type": "string", "description": "Python source of the statement(s) to append."},
            },
            "required": ["path", "code"],
        },
        output_schema={
            "type": "object",
            "properties": {"result": {"type": "string"}, "created": {"type": "integer"}},
            "required": ["result", "created"],
        },
        annotations={"readOnlyHint": False, "openWorldHint": False},
    )
    def create_node(ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            path = core.require_path(args["path"], must_exist=False)
            new_nodes = core.parse_snippet(args["code"])
            tree = core.CACHE.get_tree(path) if path.exists() else ast.Module(body=[], type_ignores=[])
            tree.body.extend(new_nodes)
            core.CACHE.save(path, tree)
        except core.AstError as exc:
            return _err(exc)
        return _ok({"result": "success", "created": len(new_nodes)})
