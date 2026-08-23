"""``python-ast-replace-block`` – text replace scoped to a single AST node.

Like the top-level ``replace-block`` tool but constrained to the line range of a
selected node (method/class/function), so ``old_text`` only has to be unique
within that node rather than the whole file. Shares the whitespace-tolerant
matcher with the file-level tool.
"""

from __future__ import annotations

from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from .._text_match import find as find_text
from . import core

_SELECTOR_PROPS = {
    "qualified_name": {"type": "string", "description": "Python-style FQN of the enclosing node."},
    "name": {"type": "string", "description": "Simple node name."},
    "node_type": {"type": "string", "description": "AST node class name filter."},
    "lineno": {"type": "integer", "description": "Start line of the node."},
}


def _select(tree, args: dict[str, Any]) -> core.Located:
    sel = {k: args.get(k) for k in _SELECTOR_PROPS}
    hits = core.find(tree, **sel)
    if not hits:
        raise core.AstError("No node matched the selector.")
    if len(hits) > 1:
        raise core.AstError(f"Selector is ambiguous – {len(hits)} nodes matched.")
    return hits[0]


def register(registry: ToolRegistry) -> None:
    @registry.tool(
        "python-ast-replace-block",
        title="Replace block within node",
        description=(
            "Replace a text block inside the line range of a selected AST node. "
            "'old_text' must occur exactly once within that node; whitespace is "
            "matched tolerantly unless 'exact' is set."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the Python file."},
                "old_text": {"type": "string", "description": "Text to find within the node."},
                "new_text": {"type": "string", "description": "Replacement text."},
                "exact": {
                    "type": "boolean",
                    "description": "Require exact whitespace matching.",
                    "default": False,
                },
                **_SELECTOR_PROPS,
            },
            "required": ["path", "old_text", "new_text"],
        },
        output_schema={
            "type": "object",
            "properties": {"result": {"type": "string"}},
            "required": ["result"],
        },
        annotations={"readOnlyHint": False, "openWorldHint": False},
    )
    def replace_block(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        old_text = args["old_text"]
        if old_text == "":
            return ToolResult(content=[text_content("'old_text' must not be empty.")], is_error=True)
        try:
            path = core.require_path(args["path"])
            tree = core.CACHE.get_tree(path)
            target = _select(tree, args)
            node = target.node

            text = path.read_text(encoding="utf-8")
            lines = text.splitlines(keepends=True)
            start = node.lineno - 1
            end = getattr(node, "end_lineno", node.lineno)
            scope_start = len("".join(lines[:start]))
            scope_end = len("".join(lines[:end]))
            scope = text[scope_start:scope_end]

            match = find_text(scope, old_text, exact=args.get("exact", False))
            if match.count == 0:
                raise core.AstError("Text not found within node.")
            if match.count > 1:
                raise core.AstError(f"Text is ambiguous – {match.count} occurrences within node.")

            abs_start = scope_start + match.start
            abs_end = scope_start + match.end
            new_full = text[:abs_start] + args["new_text"] + text[abs_end:]

            # Validate the result before persisting; refresh cache from the file.
            core.parse_source(new_full)
            path.write_text(new_full, encoding="utf-8")
            core.CACHE.invalidate(path)
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)

        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
