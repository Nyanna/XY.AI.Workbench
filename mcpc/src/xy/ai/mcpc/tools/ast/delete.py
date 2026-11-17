"""``ast_delete`` tool: delete selected nodes, or whole files if none selected."""
from dataclasses import dataclass
from xy.ai.mcpc.tools.ast.imports import ImportCheckedBatchResult, ImportOutcome, parse_import_args
from xy.ai.mcpc.tools.ast.outcomes import NodeError, edit_payload, node_key
from xy.ai.mcpc.tools.results import ItemResult
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import IMPORT_PROPS, PATH_SELECTOR_PROPS, PATH_PROP, select_by_path
from xy.ai.mcpc.tools._tool_helpers import batch_schema
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
__all__ = ['DeleteItem', 'DeleteResult', 'DeleteError', 'DeleteBatchResult', 'ast_delete', 'DeleteTool', 'register']

@dataclass(frozen=True)
class DeleteItem:
    """One node (or file) to delete.

    Attributes:
        path: Absolute path to the file to modify.
        id: Unique id of the target node. Omit to delete the whole file.
        add_import: Import statements to add if missing, applied after the delete.
        remove_import: Import statements to remove if present, applied after the delete.
            Both are ignored (and flagged in the batch warnings) when a whole file is deleted.
    """
    path: str
    id: str | None = None
    add_import: list[str] | None = None
    remove_import: list[str] | None = None

@dataclass(frozen=True)
class DeleteResult(ItemResult):
    """Result of deleting a single node or file.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
    """
    path: str
    id: str | None
    imports: ImportOutcome | None = None

    def key(self) -> dict[str, Any]:
        return node_key(self.path, self.id)

    def payload(self) -> dict[str, Any]:
        return edit_payload(imports=self.imports)

class DeleteError(NodeError):
    """Error deleting a single node or file.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        error: The error message.
    """

class DeleteBatchResult(ImportCheckedBatchResult[DeleteResult, DeleteError]):
    """Result of :func:`ast_delete`.

    Attributes:
        results: One :class:`DeleteResult` per successfully deleted item.
        errors: One :class:`DeleteError` per item that failed.
    """

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
        unchecked = ImportOutcome(unchecked=True) if item.add_import or item.remove_import else None
        return DeleteResult(path=item.path, id=None, imports=unchecked)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=item.id)
    core.delete_node(target)
    imports = core.apply_imports(tree, item.add_import, item.remove_import)
    core.CACHE.save(file_path, tree)
    return DeleteResult(path=item.path, id=item.id, imports=imports)

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
    _ITEM_PROPERTIES = {'path': PATH_PROP, **PATH_SELECTOR_PROPS, **IMPORT_PROPS}
    _ITEM_REQUIRED = ['path']
    _ITEMS_DESCRIPTION = 'Nodes/files to delete.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_delete`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> DeleteItem:
            add_import, remove_import = parse_import_args(it)
            return DeleteItem(path=it['path'], id=it.get('id'), add_import=add_import, remove_import=remove_import)
        return handle_batch_tool(ctx, item_factory, ast_delete, core.AstError, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(DeleteTool())
    functions.register(ast_delete)