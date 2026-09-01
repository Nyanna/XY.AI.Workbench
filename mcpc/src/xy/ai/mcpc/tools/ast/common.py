"""Selector machinery shared by the ``ast_*`` tools.

``ast_find`` is the only tool that restricts on diverse node properties, so it
uses the full :data:`SELECTOR_PROPS`. Every mutation tool addresses a node purely
by its path (id/qualified name) and uses the reduced :data:`PATH_SELECTOR_PROPS`.
"""


from typing import Any

from xy.ai.mcpc.tools.ast import core

__all__ = ["SELECTOR_PROPS", "PATH_SELECTOR_PROPS", "select_one", "select_by_path", "list_output_schema"]

#: Full node selectors – only ``ast_find`` may restrict on node properties.
SELECTOR_PROPS = {
    "id": {"type": "string", "description": "Node id: primarily name-based dotted path (e.g. 'MyClass.method'), numeric only as fallback."},
    "qualified_name": {"type": "string", "description": "Qualified name of the target node."},
    "name": {"type": "string", "description": "Simple node name."},
    "node_type": {"type": "string", "description": "Node type name, e.g. 'FunctionDef' or 'pair'."},
    "lineno": {"type": "integer", "description": "Start line of the target node."},
    "end_lineno": {"type": "integer", "description": "End line of the target node."},
    "parent_type": {"type": "string", "description": "Node type name of the container."},
}

#: Path-only selectors used by every mutation tool (replace/insert/delete/edit_*).
PATH_SELECTOR_PROPS = {
    "id": SELECTOR_PROPS["id"],
    "qualified_name": SELECTOR_PROPS["qualified_name"],
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


def select_by_path(tree, *, id: str | None = None, qualified_name: str | None = None) -> core.Located:
    """Return the single node in *tree* addressed by ``id`` or ``qualified_name``.

    Raises:
        core.AstError: If neither is given, or the path matches zero/many nodes.
    """
    if id is None and qualified_name is None:
        raise core.AstError("A node selector (id or qualified_name) is required.")
    return select_one(tree, id=id, qualified_name=qualified_name)


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
