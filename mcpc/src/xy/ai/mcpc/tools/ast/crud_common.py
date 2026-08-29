"""Selector machinery shared by the ``python_ast_{find,read,insert,replace,delete}`` tools."""


from typing import Any

from xy.ai.mcpc.tools.ast import core

__all__ = ["SELECTOR_PROPS", "select_one", "list_output_schema"]

#: Shared JSON-Schema fragment for the node selectors accepted by find/read/insert/replace/delete.
SELECTOR_PROPS = {
    "qualified_name": {"type": "string", "description": "Python-style FQN of the target node."},
    "name": {"type": "string", "description": "Simple node name."},
    "node_type": {"type": "string", "description": "AST node class name, e.g. 'FunctionDef'."},
    "lineno": {"type": "integer", "description": "Start line of the target node."},
    "end_lineno": {"type": "integer", "description": "End line of the target node."},
    "parent_type": {"type": "string", "description": "AST class name of the container."},
}


def select_one(tree, **selectors: Any) -> core.Located:
    """Return the single node in *tree* matching *selectors*.

    Raises:
        core.AstError: If no node matches, or more than one node matches.
    """
    hits = core.find(tree, **selectors)
    if not hits:
        raise core.AstError("No node matched the selector.")
    if len(hits) > 1:
        raise core.AstError(f"Selector is ambiguous – {len(hits)} nodes matched.")
    return hits[0]


def list_output_schema() -> dict[str, Any]:
    return {
        "$defs": {"outline_node": core.OUTLINE_NODE_SCHEMA},
        "type": "object",
        "properties": {
            "nodes": {"type": "array", "items": {"$ref": "#/$defs/outline_node"}},
            "count": {"type": "integer"},
        },
        "required": ["nodes", "count"],
    }
