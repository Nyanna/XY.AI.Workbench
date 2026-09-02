"""``ast_delete`` tool: delete a selected node, or the whole file if none is selected."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['DeleteResult', 'ast_delete', 'DeleteTool', 'register']

@dataclass(frozen=True)
class DeleteResult:
    """Result of :func:`ast_delete`.

    Attributes:
        result: Always ``"success"``.
    """
    result: str

def ast_delete(path: str, *, id: str | None=None) -> DeleteResult:
    """Delete the single selected node, or the whole file if the root is selected.

    The whole file is deleted by omitting the ``id`` selector – there is no other way
    to address the root, since it is never itself an addressable child. Deleting the
    file also removes it from the AST cache and, if its parent directory becomes
    empty as a result, removes that directory too.

    Args:
        path: Absolute path to the file to modify.
        id: Unique id of the target node.

    Returns:
        DeleteResult: Success status.

    Raises:
        core.AstError: If ``path`` is invalid, or a selector is given but matches
            zero or more than one node.
    """
    file_path = core.require_path(path)
    if id is None:
        try:
            file_path.unlink()
        except OSError as exc:
            raise core.AstError('Delete failed.') from exc
        core.CACHE.invalidate(file_path)
        parent = file_path.parent
        if not any(parent.iterdir()):
            parent.rmdir()
        return DeleteResult(result='success')
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=id)
    core.delete_node(target)
    core.CACHE.save(file_path, tree)
    return DeleteResult(result='success')

class DeleteTool(ToolDefinition):
    name = 'ast_delete'
    title = 'Delete AST node or file'
    description = 'Delete the single selected node from a file, or the whole file – and its directory if no selector is given.'
    input_schema = {
        'type': 'object',
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the file.'},
            **PATH_SELECTOR_PROPS},
        'required': ['path']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': ['result']}
    annotations = {'readOnlyHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_delete(args['path'], id=args.get('id'))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(DeleteTool())
    functions.register(ast_delete)