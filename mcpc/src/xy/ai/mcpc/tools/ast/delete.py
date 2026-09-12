"""``ast_delete`` tool: delete selected nodes, or whole files if none selected."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['DeleteItem', 'DeleteResult', 'DeleteError', 'DeleteBatchResult', 'ast_delete', 'DeleteTool', 'register']

@dataclass(frozen=True)
class DeleteItem:
    """One node (or file) to delete.

    Attributes:
        path: Absolute path to the file to modify.
        id: Unique id of the target node. Omit to delete the whole file.
    """
    path: str
    id: str | None = None

@dataclass(frozen=True)
class DeleteResult:
    """Result of deleting a single node or file.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        result: Always ``"success"``.
    """
    path: str
    id: str | None
    result: str

@dataclass(frozen=True)
class DeleteError:
    """Error deleting a single node or file.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        error: The error message.
    """
    path: str
    id: str | None
    error: str

@dataclass(frozen=True)
class DeleteBatchResult:
    """Result of :func:`ast_delete`.

    Attributes:
        results: One :class:`DeleteResult` per successfully deleted item.
        errors: One :class:`DeleteError` per item that failed.
    """
    results: list[DeleteResult]
    errors: list[DeleteError]

def _delete_one(item: DeleteItem) -> DeleteResult:
    file_path = core.require_path(item.path)
    if item.id is None:
        try:
            file_path.unlink()
        except OSError as exc:
            raise core.AstError('Delete failed.') from exc
        core.CACHE.invalidate(file_path)
        parent = file_path.parent
        if not any(parent.iterdir()):
            parent.rmdir()
        return DeleteResult(path=item.path, id=None, result='success')
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=item.id)
    core.delete_node(target)
    core.CACHE.save(file_path, tree)
    return DeleteResult(path=item.path, id=item.id, result='success')

def ast_delete(items: list[DeleteItem]) -> DeleteBatchResult:
    """Delete one or more selected nodes, or whole files if their ``id`` is omitted.

    A whole file is deleted by omitting its ``id`` selector – there is no other
    way to address the root, since it is never itself an addressable child.
    Deleting a file also removes it from the AST cache and, if its parent
    directory becomes empty as a result, removes that directory too.

    Args:
        items: Nodes/files to delete. Must be non-empty.

    Returns:
        DeleteBatchResult: One result per deleted item, one error per failed item.

    Raises:
        core.AstError: If ``items`` is empty.
    """
    if not items:
        raise core.AstError("'items' must be a non-empty list.")
    results: list[DeleteResult] = []
    errors: list[DeleteError] = []
    for item in items:
        try:
            results.append(_delete_one(item))
        except core.AstError as exc:
            errors.append(DeleteError(path=item.path, id=item.id, error=str(exc)))
    return DeleteBatchResult(results=results, errors=errors)

class DeleteTool(ToolDefinition):
    name = 'ast_delete'
    title = 'Delete AST nodes or files'
    description = 'Delete selected nodes from files, or whole files if no selector is given, for a batch of items.'
    input_schema = {
        'type': 'object',
        'properties': {
            'items': {
                'type': 'array',
                'minItems': 1,
                'items': {
                    'type': 'object',
                    'properties': {
                        'path': {
                            'type': 'string',
                            'description': 'Absolute path to the file.'},
                        **PATH_SELECTOR_PROPS},
                    'required': ['path']},
                'description': 'Nodes/files to delete.'}},
        'required': ['items']}
    output_schema = {
        'type': 'object', 'properties': {
            'results': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'path': {
                            'type': 'string'}, 'id': {
                                'type': 'string'}, 'result': {
                                    'type': 'string'}}, 'required': [
                                        'path', 'result']}}, 'errors': {
                                            'type': 'array', 'items': {
                                                'type': 'object', 'properties': {
                                                    'path': {
                                                        'type': 'string'}, 'id': {
                                                            'type': 'string'}, 'error': {
                                                                'type': 'string'}}, 'required': [
                                                                    'path', 'error']}}}, 'required': [
                                                                        'results', 'errors']}
    annotations = {'readOnlyHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        raw_items = args.get('items') or []
        if not raw_items:
            return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
        items = [DeleteItem(path=it['path'], id=it.get('id')) for it in raw_items]
        batch = ast_delete(items)
        content = {'results': [{'path': r.path, 'id': r.id, 'result': r.result} for r in batch.results], 'errors': [
            {'path': e.path, 'id': e.id, 'error': e.error} for e in batch.errors]}
        is_error = bool(batch.errors) and (not batch.results)
        return ToolResult(structured_content=content, auto_approve=not is_error)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(DeleteTool())
    functions.register(ast_delete)