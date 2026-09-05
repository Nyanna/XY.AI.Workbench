"""``ast_read`` tool: read one or more node subtrees (with source) by id."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ReadNodeResult', 'ast_read', 'ReadNodeTool', 'register']

@dataclass(frozen=True)
class ReadNodeResult:
    """Result of :func:`ast_read`.

    Attributes:
        nodes: One expanded subtree per resolved id, in the given order; same
            shape as :func:`ast_find`'s results (see :class:`core.OutlineNode`).
        errors: One message per requested id that could not be resolved (id
            unknown/ambiguous, and no unambiguous name/fuzzy match found).
    """
    nodes: list[core.OutlineNode]
    errors: list[str]

def ast_read(ids: list[str], path: str, *, with_lines: bool=True) -> ReadNodeResult:
    """Recursively read the subtree of each addressed node for block-wise edit/replace.

    Each id resolves to a subtree: a node whose body consists solely of nested
    classes/functions is expanded into ``children`` instead of source, so the agent
    can descend to the innermost editable block; any other node is returned whole,
    as ``code`` ready to hand back to ``ast_replace`` via its ``id``.

    An id that doesn't match any node is retried as a node *name* (exact, then a
    conservative fuzzy match) instead of failing the whole call; ids that still
    can't be resolved are reported in ``errors``, not raised.

    Args:
        ids: Node ids to read. Must be non-empty.
        path: Absolute path to the file to read.
        with_lines: Whether to populate each node's line range.

    Returns:
        ReadNodeResult: One subtree per resolved entry in ``ids``, plus errors for
        the rest.

    Raises:
        core.AstError: If ``ids`` is empty, ``path`` is not absolute or not an existing
            regular file, or the source has a syntax error.
    """
    if not ids:
        raise core.AstError("'ids' must be a non-empty list of node ids.")
    tree = core.load(path)[1]
    nodes, errors = core.read_subtrees(core.locate_all(tree), ids, with_lines=with_lines)
    return ReadNodeResult(nodes=nodes, errors=errors)

class ReadNodeTool(ToolDefinition):
    name = 'ast_read'
    title = 'Read AST subtrees'
    description = "Recursively read the subtree of each addressed known AST node id, surfacing each node's id, children, and source."
    input_schema = {
        'type': 'object',
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the file.'},
            'ids': {
                'type': 'array',
                'items': {
                        'type': 'string'},
                'description': 'Use ``ast_list`` or ``ast_find`` to get the ids. List of AST node ids to read.'}},
        'required': [
            'ids',
            'path']}
    output_schema = {
        '$defs': {
            'outline_node': core.OUTLINE_NODE_SCHEMA},
        'type': 'object',
        'properties': {
                'nodes': {
                    'type': 'array',
                    'items': {
                        '$ref': '#/$defs/outline_node'}},
            'errors': {
                    'type': 'array',
                            'items': {
                                'type': 'string'}}},
        'required': ['nodes']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
        try:
            result = ast_read(ids=args.get('ids') or [], path=args.get('path'), with_lines=with_lines)
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        structured_content: dict[str, Any] = {'nodes': [core.to_dict(n) for n in result.nodes]}
        if result.errors:
            structured_content['errors'] = result.errors
        return ToolResult(structured_content=structured_content)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReadNodeTool())
    functions.register(ast_read)