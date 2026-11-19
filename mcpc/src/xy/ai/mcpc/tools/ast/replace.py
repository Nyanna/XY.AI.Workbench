"""``ast_replace`` tool: replace selected nodes with new source."""
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
__all__ = [
    'ReplaceItem',
    'ReplaceResult',
    'ReplaceError',
    'ReplaceBatchResult',
    'ast_replace',
    'ReplaceNodeTool',
    'register']

@dataclass(frozen=True)
class ReplaceItem:
    """One node replacement to apply.

    Attributes:
        path: Absolute path to the file to modify.
        source: Replacement source.
        id: Unique id of the target node.
        add_import: Import statements to add if missing, applied after the node edit.
        remove_import: Import statements to remove if present, applied after the node edit.
    """
    path: str
    source: str
    id: str | None = None
    add_import: list[str] | None = None
    remove_import: list[str] | None = None

@dataclass(frozen=True)
class ReplaceResult(ItemResult):
    """Result of a single node replacement.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        new_id: The node's new id, only set if the replacement changed it.
    """
    path: str
    id: str | None
    new_id: str | None = None
    imports: ImportOutcome | None = None

    def key(self) -> dict[str, Any]:
        return node_key(self.path, self.id)

    def payload(self) -> dict[str, Any]:
        return edit_payload(new_id=self.new_id, imports=self.imports)

class ReplaceError(NodeError):
    """Error applying a single node replacement.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        error: The error message.
    """

class ReplaceBatchResult(ImportCheckedBatchResult[ReplaceResult, ReplaceError]):
    """Result of :func:`ast_replace`.

    Attributes:
        results: One :class:`ReplaceResult` per successful replacement.
        errors: One :class:`ReplaceError` per replacement that failed.
    """

def _replace_one(item: ReplaceItem) -> ReplaceResult:
    file_path = core.require_path(item.path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=item.id)
    new_id = core.replace_node(target, item.source)
    imports = core.apply_imports(tree, item.add_import, item.remove_import)
    core.CACHE.save(file_path, tree)
    return ReplaceResult(path=item.path, id=item.id, new_id=new_id, imports=imports)

def ast_replace(items: list[ReplaceItem]) -> ReplaceBatchResult:
    """Replace one or more selected nodes with new source.

    ``id`` and replacement ``source`` stay together per item, since replacing
    the same node twice makes no sense; several items may target the same or
    different files.

    Args:
        items: Node replacements to apply. Must be non-empty.

    Returns:
        ReplaceBatchResult: One result per successful replacement, one error per
        failed replacement.

    Raises:
        core.AstError: If ``items`` is empty.
    """
    if not items:
        raise core.AstError("'items' must be a non-empty list.")
    results: list[ReplaceResult] = []
    errors: list[ReplaceError] = []
    for item in items:
        try:
            results.append(_replace_one(item))
        except core.AstError as exc:
            errors.append(ReplaceError(path=item.path, id=item.id, error=str(exc)))
    return ReplaceBatchResult(results=results, errors=errors)

class ReplaceNodeTool(ToolDefinition):
    name = 'ast_replace'
    features = frozenset({ToolFeature.INPUT_ASTID})
    title = 'Replace the complete source of the AST nodes identified by the given IDs with new content.'
    description = 'Replace the source or text of selected AST nodes in a batch; operations may target the same or different files. Returns the updated IDs.'
    _ITEM_PROPERTIES = {
        'path': PATH_PROP,
        'source': {
            'type': 'string',
            'description': 'Replacement source.'},
        **PATH_SELECTOR_PROPS,
        **IMPORT_PROPS}
    _ITEM_REQUIRED = ['path', 'source', 'id']
    _ITEMS_DESCRIPTION = 'Node replacements to apply.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."""

        def item_factory(it: dict[str, Any]) -> ReplaceItem:
            add_import, remove_import = parse_import_args(it)
            return ReplaceItem(
                path=it['path'],
                source=it['source'],
                id=it.get('id'),
                add_import=add_import,
                remove_import=remove_import)
        return handle_batch_tool(ctx, item_factory, ast_replace, core.AstError, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReplaceNodeTool())
    functions.register(ast_replace)