"""``ast_read`` tool: read one or more node subtrees (with source) by id."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.core import ReadNode
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ReadNode', 'ReadNodeResult', 'ast_read', 'ReadNodeTool', 'register']

@dataclass(frozen=True)
class ReadNodeResult:
    """Result of :func:`ast_read`.

    Attributes:
        nodes: One expanded subtree per requested id, in the given order.
    """
    nodes: list[ReadNode]

def ast_read(ids: list[str], path: str) -> ReadNodeResult:
    """Recursively read the subtree of each addressed node for block-wise edit/replace.

    Each id resolves to a subtree: a node whose body consists solely of nested
    classes/functions is expanded into ``children`` instead of source, so the agent
    can descend to the innermost editable block; any other node is returned whole,
    as ``code`` ready to hand back to ``ast_replace`` via its ``id``.

    Args:
        ids: Node ids to read. Must be non-empty.
        path: Absolute path to the file to read.

    Returns:
        ReadNodeResult: One subtree per entry in ``ids``.

    Raises:
        core.AstError: If ``ids`` is empty, ``path`` is not absolute or not an existing
            regular file, the source has a syntax error, or an id matches no node.
    """
    if not ids:
        raise core.AstError("'ids' must be a non-empty list of node ids.")
    tree = core.load(path)[1]
    nodes = core.read_subtrees(core.locate_all(tree), ids)
    return ReadNodeResult(nodes=nodes)
_READ_NODE_SCHEMA = {'type': 'object', 'properties': {'id': {'type': 'string', 'description': 'Unique node id; the address for ast_replace/edit.'}, 'type': {'type': 'string'}, 'lines': {'type': 'string', 'description': "Line number, or 'start-end' if the node spans multiple lines."}, 'code': {'type': 'string', 'description': "Full source of this node, ready for ast_replace; omitted if the node consists solely of the nested classes/functions listed in 'children'."}, 'children': {'type': 'array', 'items': {'$ref': '#/$defs/read_node'}}}, 'required': ['id', 'type', 'lines']}

class ReadNodeTool(ToolDefinition):
    name = 'ast_read'
    title = 'Read AST subtrees'
    description = "Recursively read the subtree of each addressed node (by id), surfacing each block's id and source so it can be handed to ast_replace/ast_edit_marks/ast_edit_block. Nodes whose body consists solely of nested classes/functions are expanded into 'children' instead of source, letting the agent descend to the innermost block that needs editing."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the file.'}, 'ids': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Node ids to read.'}}, 'required': ['ids', 'path']}
    output_schema = {'$defs': {'read_node': _READ_NODE_SCHEMA}, 'type': 'object', 'properties': {'nodes': {'type': 'array', 'items': {'$ref': '#/$defs/read_node'}}}, 'required': ['nodes']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_read(ids=args.get('ids') or [], path=args.get('path'))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'nodes': [core.to_dict(n) for n in result.nodes]})

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReadNodeTool())
    functions.register(ast_read)