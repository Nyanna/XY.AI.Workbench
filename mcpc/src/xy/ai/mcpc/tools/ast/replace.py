"""``ast_replace`` tool: replace selected nodes with new source."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
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
    """
    path: str
    source: str
    id: str | None = None

@dataclass(frozen=True)
class ReplaceResult:
    """Result of a single node replacement.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        result: Always ``"success"``.
        new_id: The node's new id, only set if the replacement changed it.
    """
    path: str
    id: str | None
    result: str
    new_id: str | None = None

@dataclass(frozen=True)
class ReplaceError:
    """Error applying a single node replacement.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        error: The error message.
    """
    path: str
    id: str | None
    error: str

@dataclass(frozen=True)
class ReplaceBatchResult:
    """Result of :func:`ast_replace`.

    Attributes:
        results: One :class:`ReplaceResult` per successful replacement.
        errors: One :class:`ReplaceError` per replacement that failed.
    """
    results: list[ReplaceResult]
    errors: list[ReplaceError]

def _replace_one(item: ReplaceItem) -> ReplaceResult:
    file_path = core.require_path(item.path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=item.id)
    new_id = core.replace_node(target, item.source)
    core.CACHE.save(file_path, tree)
    return ReplaceResult(path=item.path, id=item.id, result='success', new_id=new_id)

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
    title = 'Replace AST nodes'
    description = 'Replace selected nodes with source or text, for a batch of items; several operations may target the same or different files.'
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
                        'source': {
                            'type': 'string',
                            'description': 'Replacement source.'},
                        **PATH_SELECTOR_PROPS},
                    'required': [
                        'path',
                        'source']},
                'description': 'Node replacements to apply.'}},
        'required': ['items']}
    output_schema = {
        'type': 'object', 'properties': {
            'results': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'path': {
                            'type': 'string'}, 'id': {
                                'type': 'string'}, 'result': {
                                    'type': 'string', 'description': 'Result status'}, 'new_id': {
                                        'type': 'string', 'description': "The node's new id."}}, 'required': [
                                            'path', 'result']}}, 'errors': {
                                                'type': 'array', 'items': {
                                                    'type': 'object', 'properties': {
                                                        'path': {
                                                            'type': 'string'}, 'id': {
                                                                'type': 'string'}, 'error': {
                                                                    'type': 'string'}}, 'required': [
                                                                        'path', 'error']}}}}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_replace`, translating the MCP schema to/from the AST API."""

        def item_factory(it: dict[str, Any]) -> ReplaceItem:
            return ReplaceItem(path=it['path'], source=it['source'], id=it.get('id'))

        def result_serializer(r: ReplaceResult) -> dict[str, Any]:
            entry = {'path': r.path, 'id': r.id, 'result': r.result}
            if r.new_id is not None:
                entry['new_id'] = r.new_id
            return entry

        def error_serializer(e: ReplaceError) -> dict[str, Any]:
            return {'path': e.path, 'id': e.id, 'error': e.error}
        return handle_batch_tool(ctx, item_factory, ast_replace, core.AstError, result_serializer, error_serializer, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReplaceNodeTool())
    functions.register(ast_replace)