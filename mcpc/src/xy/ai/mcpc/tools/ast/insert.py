"""``ast_insert`` tool: insert statement(s) relative to selected nodes."""
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
from xy.ai.mcpc.tools.tool_registry import ToolFeature
__all__ = ['InsertItem', 'InsertResult', 'InsertError', 'InsertBatchResult', 'ast_insert', 'InsertNodeTool', 'register']

@dataclass(frozen=True)
class InsertItem:
    """One insert operation.

    Attributes:
        path: Absolute path to the file to modify.
        source: Source of the statement(s) to insert.
        position: ``"before"`` or ``"after"`` the selected node. Defaults to ``"after"``.
        id: Unique id of the target node.
        add_import: Import statements to add if missing, applied after the insert.
        remove_import: Import statements to remove if present, applied after the insert.
    """
    path: str
    source: str
    position: str = 'after'
    id: str | None = None
    add_import: list[str] | None = None
    remove_import: list[str] | None = None

@dataclass(frozen=True)
class InsertResult(ItemResult):
    """Result of a single insert operation.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        inserted: Number of top-level statements parsed from ``source`` and inserted.
        ids: The newly inserted top-level node(s)' ids.
    """
    path: str
    id: str | None
    inserted: int
    ids: list[str] | None = None
    imports: ImportOutcome | None = None

    def key(self) -> dict[str, Any]:
        return node_key(self.path, self.id)

    def payload(self) -> dict[str, Any]:
        return edit_payload(imports=self.imports, inserted=self.inserted, ids=self.ids)

class InsertError(NodeError):
    """Error applying a single insert operation.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        error: The error message.
    """

class InsertBatchResult(ImportCheckedBatchResult[InsertResult, InsertError]):
    """Result of :func:`ast_insert`.

    Attributes:
        results: One :class:`InsertResult` per successful insert.
        errors: One :class:`InsertError` per insert that failed.
    """

def _insert_one(item: InsertItem) -> InsertResult:
    file_path = core.require_path(item.path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=item.id)
    before_ids = {loc.node_id for loc in core.locate_all(tree)}
    inserted = core.insert_node(target, item.source, item.position)
    new_ids = [loc.node_id for loc in core.locate_all(tree) if loc.node_id not in before_ids]
    imports = core.apply_imports(tree, item.add_import, item.remove_import)
    core.CACHE.save(file_path, tree)
    return InsertResult(path=item.path, id=item.id, inserted=inserted, ids=new_ids or None, imports=imports)

def ast_insert(items: list[InsertItem]) -> InsertBatchResult:
    """Insert statement(s) parsed from ``source`` relative to one or more selected nodes.

    Several operations may target the same path (also across several paths);
    each item is applied in order.

    Args:
        items: Insert operations to apply. Must be non-empty.

    Returns:
        InsertBatchResult: One result per successful insert, one error per failed insert.

    Raises:
        core.AstError: If ``items`` is empty.
    """
    if not items:
        raise core.AstError("'items' must be a non-empty list.")
    results: list[InsertResult] = []
    errors: list[InsertError] = []
    for item in items:
        try:
            results.append(_insert_one(item))
        except core.AstError as exc:
            errors.append(InsertError(path=item.path, id=item.id, error=str(exc)))
    return InsertBatchResult(results=results, errors=errors)

class InsertNodeTool(ToolDefinition):
    name = 'ast_insert'
    features = frozenset({ToolFeature.INPUT_ASTID})
    title = 'Insert AST nodes'
    description = "Insert source relative to selected nodes ('before' or 'after'), for a batch of items; several operations may target the same or different files."
    _ITEM_PROPERTIES = {
        'path': PATH_PROP,
        'source': {
            'type': 'string',
            'description': 'Source to insert.'},
        'position': {
            'type': 'string',
            'enum': [
                    'before',
                    'after'],
            'description': 'Placement relative to the selected node.',
            'default': 'after'},
        **PATH_SELECTOR_PROPS,
        **IMPORT_PROPS}
    _ITEM_REQUIRED = ['path', 'source']
    _ITEMS_DESCRIPTION = 'Insert operations to apply.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_insert`, translating the MCP schema to/from the AST API."""

        def item_factory(it: dict[str, Any]) -> InsertItem:
            add_import, remove_import = parse_import_args(it)
            return InsertItem(
                path=it['path'],
                source=it['source'],
                position=it.get(
                    'position',
                    'after'),
                id=it.get('id'),
                add_import=add_import,
                remove_import=remove_import)
        return handle_batch_tool(ctx, item_factory, ast_insert, core.AstError, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(InsertNodeTool())
    functions.register(ast_insert)