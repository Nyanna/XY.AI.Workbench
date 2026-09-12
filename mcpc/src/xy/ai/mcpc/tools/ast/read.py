"""``ast_read`` tool: read one or more node subtrees (with source) by id, across files."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.ast.list import ast_list
from xy.ai.mcpc.tools._tool_helpers import require_items
__all__ = ['ReadItem', 'ReadResult', 'ReadError', 'ReadBatchResult', 'ast_read', 'ReadNodeTool', 'register']
_ROOT_INTENT_IDS = {'root', '_module_', '__module__', 'module', '', '*'}

def _looks_like_root_intent(ids: list[str]) -> bool:
    """Whether ``ids`` is empty or consists solely of common root-id guesses."""
    return not ids or all((i.strip().lower() in _ROOT_INTENT_IDS for i in ids))

@dataclass(frozen=True)
class ReadItem:
    """One file's node ids to read.

    Attributes:
        path: Absolute path to the file to read.
        ids: Node ids to read. Must be non-empty.
    """
    path: str
    ids: list[str]

@dataclass(frozen=True)
class ReadResult:
    """Nodes read from a single file.

    Attributes:
        path: The path exactly as given in the input, for result association.
        nodes: One expanded subtree per resolved id, in the given order; same
            shape as :func:`ast_find`'s results (see :class:`core.OutlineNode`).
        errors: One message per requested id that could not be resolved (id
            unknown/ambiguous, and no unambiguous name/fuzzy match found).
    """
    path: str
    nodes: list[core.OutlineNode]
    errors: list[str]

@dataclass(frozen=True)
class ReadError:
    """Error reading a whole item (e.g. bad path or empty ``ids``).

    Attributes:
        path: The path exactly as given in the input, for result association.
        error: The error message.
    """
    path: str
    error: str

@dataclass(frozen=True)
class ReadBatchResult:
    """Result of :func:`ast_read`.

    Attributes:
        results: One :class:`ReadResult` per successfully read file.
        errors: One :class:`ReadError` per file that failed entirely.
    """
    results: list[ReadResult]
    errors: list[ReadError]

def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:
    if not item.ids:
        raise core.AstError("'ids' must be a non-empty list of node ids.")
    tree = core.load(item.path)[1]
    nodes, errs = core.read_subtrees(core.locate_all(tree), item.ids, with_lines=with_lines)
    return ReadResult(path=item.path, nodes=nodes, errors=errs)

def ast_read(items: list[ReadItem], *, with_lines: bool=True) -> ReadBatchResult:
    """Recursively read the subtree of each addressed node, across one or more files.

    Each id resolves to a subtree: a node whose body consists solely of nested
    classes/functions is expanded into ``children`` instead of source, so the agent
    can descend to the innermost editable block; any other node is returned whole,
    as ``code`` ready to hand back to ``ast_replace`` via its ``id``.

    An id that doesn't match any node is retried as a node *name* (exact, then a
    conservative fuzzy match) instead of failing the whole item; ids that still
    can't be resolved are reported in that item's ``errors``, not raised.

    Args:
        items: Per-file node ids to read. Must be non-empty.
        with_lines: Whether to populate each node's line range.

    Returns:
        ReadBatchResult: One result per file (with its own id-level errors), and
        one file-level error per file that failed entirely (bad path, empty
        ``ids``, or a syntax error).

    Raises:
        core.AstError: If ``items`` is empty.
    """
    if not items:
        raise core.AstError("'items' must be a non-empty list.")
    results: list[ReadResult] = []
    errors: list[ReadError] = []
    for item in items:
        try:
            results.append(_read_one(item, with_lines=with_lines))
        except core.AstError as exc:
            errors.append(ReadError(path=item.path, error=str(exc)))
    return ReadBatchResult(results=results, errors=errors)

class ReadNodeTool(ToolDefinition):
    name = 'ast_read'
    title = 'After using `ast_list` or `ast_find`, read AST subtrees for known node IDs'
    description = "After using `ast_list` or `ast_find`, recursively read the subtree of each ID-addressed AST node across one or more files, surfacing each node's children and source."
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
                        'ids': {
                            'type': 'array',
                            'items': {
                                    'type': 'string'},
                            'description': 'List of AST node ids to read.'}},
                    'required': [
                        'path',
                        'ids']},
                'description': 'Per-file node ids to read.'}},
        'required': ['items']}
    output_schema = {
        '$defs': {
            'outline_node': core.OUTLINE_NODE_SCHEMA}, 'type': 'object', 'properties': {
                'results': {
                    'type': 'array', 'items': {
                        'type': 'object', 'properties': {
                            'path': {
                                'type': 'string'}, 'nodes': {
                                    'type': 'array', 'items': {
                                        '$ref': '#/$defs/outline_node'}}, 'errors': {
                                            'type': 'array', 'items': {
                                                'type': 'string'}}}, 'required': [
                                                    'path', 'nodes']}}, 'errors': {
                                                        'type': 'array', 'items': {
                                                            'type': 'object', 'properties': {
                                                                'path': {
                                                                    'type': 'string'}, 'error': {
                                                                        'type': 'string'}}, 'required': [
                                                                            'path', 'error']}}}, 'required': [
                                                                                'results', 'errors']}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API.

        Per item, falls back to :func:`ast_list` when ``ids`` looks like agents
        habitually mis-guessing a root id (``root``/``_module_``/``module``/empty)
        and none of them resolve to a node: returns that file's full outline
        instead of an error, and notes the redirect in the item's ``errors``.
        """
        with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
        raw_items, error = require_items(ctx)
        if error is not None:
            return error
        results: list[dict[str, Any]] = []
        errors: list[dict[str, str]] = []
        for raw in raw_items:
            path = raw.get('path')
            ids: list[str] = raw.get('ids') or []
            item_result: ReadResult | None = None
            if ids:
                batch = ast_read([ReadItem(path=path, ids=ids)], with_lines=with_lines)
                if batch.errors:
                    errors.append({'path': batch.errors[0].path, 'error': batch.errors[0].error})
                    continue
                item_result = batch.results[0]
            if _looks_like_root_intent(ids) and (item_result is None or not item_result.nodes):
                list_batch = ast_list([path], with_lines=with_lines)
                for lr in list_batch.results:
                    results.append({'path': lr.path, 'nodes': [core.to_dict(n) for n in lr.nodes], 'errors': [
                                   f'ids {ids!r} resolved to no node; redirected to ast_list, returning the full outline instead.']})
                for le in list_batch.errors:
                    errors.append({'path': le.path, 'error': le.error})
                continue
            pathResult = {'path': item_result.path, 'nodes': [core.to_dict(n) for n in item_result.nodes]}
            if item_result.errors:
                pathResult['errors'] = item_result.errors
            results.append(pathResult)
        structured_content: dict[str, Any] = {}
        if results:
            structured_content['results'] = results
        if errors:
            structured_content['errors'] = errors
        return ToolResult(structured_content=structured_content)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReadNodeTool())
    functions.register(ast_read)