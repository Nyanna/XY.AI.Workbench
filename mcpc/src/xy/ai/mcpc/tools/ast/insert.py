"""``ast_insert`` tool: insert statement(s) relative to a selected node."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['InsertNodeResult', 'ast_insert', 'InsertNodeTool', 'register']

@dataclass(frozen=True)
class InsertNodeResult:
    """Result of :func:`ast_insert`.

    Attributes:
        result: Always ``"success"``.
        inserted: Number of top-level statements parsed from ``code`` and inserted.
        ids: The newly inserted top-level node(s)' ids.
    """
    result: str
    inserted: int
    ids: list[str] | None = None

def ast_insert(path: str, code: str, *, position: str='after', id: str | None=None) -> InsertNodeResult:
    """Insert statement(s) parsed from ``code`` relative to a selected node.

    Args:
        path: Absolute path to the file to modify.
        code: Source of the statement(s) to insert.
        position: ``"before"`` or ``"after"`` the selected node. Defaults to ``"after"``.
        id: Unique id of the target node.


    Returns:
        InsertNodeResult: Success status, the number of statements inserted, and
            their new ids.

    Raises:
        core.AstError: If ``path`` is invalid, ``code`` has a syntax error, ``id`` is
            not given, or it matches zero or more than one node.
    """
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=id)
    before_ids = {loc.node_id for loc in core.locate_all(tree)}
    inserted = core.insert_node(target, code, position)
    new_ids = [loc.node_id for loc in core.locate_all(tree) if loc.node_id not in before_ids]
    core.CACHE.save(file_path, tree)
    return InsertNodeResult(result='success', inserted=inserted, ids=new_ids or None)

class InsertNodeTool(ToolDefinition):
    name = 'ast_insert'
    title = 'Insert AST node'
    description = "Insert statement(s) parsed from code relative to a selected node ('before' or 'after')."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the file.'}, 'code': {'type': 'string', 'description': 'Source of the statement(s) to insert.'}, 'position': {'type': 'string', 'enum': ['before', 'after'], 'description': 'Placement relative to the selected node.', 'default': 'after'}, **PATH_SELECTOR_PROPS}, 'required': ['path', 'code']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}, 'inserted': {'type': 'integer'}, 'ids': {'type': 'array', 'items': {'type': 'string'}, 'description': 'The newly inserted node(s) ids.'}}, 'required': ['result', 'inserted']}
    annotations = {'readOnlyHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_insert(args['path'], args['code'], position=args.get('position', 'after'), id=args.get('id'))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        content = {'result': result.result, 'inserted': result.inserted}
        if result.ids is not None:
            content['ids'] = result.ids
        return ToolResult(structured_content=content, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(InsertNodeTool())
    functions.register(ast_insert)