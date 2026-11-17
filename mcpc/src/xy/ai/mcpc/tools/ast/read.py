"""``ast_read`` tool: read one or more node subtrees (with source) by id, across files."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_PROP
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.ast.outline import ast_outline
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
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
class ReadResult(ItemResult):
    """Nodes read from a single file.

    Attributes:
        path: The path exactly as given in the input, for result association.
        nodes: One expanded subtree per resolved id, in the given order; same
            shape as :func:`ast_find`'s results (see :class:`core.OutlineNode`).
        warnings: One message per requested id that could not be resolved (id
            unknown/ambiguous, and no unambiguous name/fuzzy match found), or the note that
            the file's outline was returned instead.
    """
    path: str
    nodes: list[core.OutlineNode]

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

    def payload(self) -> dict[str, Any]:
        return {'nodes': [core.to_dict(n) for n in self.nodes]}

@dataclass(frozen=True)
class ReadError(ItemError):
    """Error reading a whole item (e.g. bad path or a syntax error).

    Attributes:
        path: The path exactly as given in the input, for result association.
        error: The error message.
    """
    path: str

    def key(self) -> dict[str, Any]:
        return {'path': self.path}

class ReadBatchResult(BatchResult[ReadResult, ReadError]):
    """Result of :func:`ast_read`.

    Attributes:
        results: One :class:`ReadResult` per successfully read file.
        errors: One :class:`ReadError` per file that failed entirely.
    """

def _read_one(item: ReadItem, *, with_lines: bool) -> ReadResult:
    tree = core.load(item.path)[1]
    nodes, unresolved = core.read_subtrees(core.locate_all(tree), item.ids, with_lines=with_lines)
    return ReadResult(path=item.path, nodes=nodes, warnings=tuple(unresolved))

def _outline_instead(item: ReadItem, *, with_lines: bool) -> tuple[list[ReadResult], list[ReadError]]:
    """The file's full outline, noting the redirect if ids were given but none resolved."""
    outline = ast_outline([item.path], with_lines=with_lines)
    results = [
        ReadResult(
            path=r.path,
            nodes=r.nodes,
            warnings=(
                f'ast_read fell back to ast_outline: ids {
                    item.ids!r} resolved to no node; returning the full outline of {
                        r.path} instead.',
            ) if item.ids else ()) for r in outline.results]
    return (results, [ReadError(path=e.path, error=e.error) for e in outline.errors])

def ast_read(items: list[ReadItem], *, with_lines: bool=True) -> ReadBatchResult:
    """Recursively read the subtree of each addressed node, across one or more files.

    Each id resolves to a subtree: a node whose body consists solely of nested
    classes/functions is expanded into ``children`` instead of source, so the agent
    can descend to the innermost editable block; any other node is returned whole,
    as ``code`` ready to hand back to ``ast_replace`` via its ``id``.

    An id that doesn't match any node is retried as a node *name* (exact, then a
    conservative fuzzy match) instead of failing the whole item; ids that still
    can't be resolved are reported in that item's ``warnings``, not raised.

    Per item, falls back to :func:`ast_outline` when ``ids`` is empty or looks like
    agents habitually mis-guessing a root id (``root``/``_module_``/``module``) and
    none of them resolve to a node: returns that file's full outline instead of an
    error, and notes the redirect in the item's ``warnings`` (if ids were given).

    Args:
        items: Per-file node ids to read. Must be non-empty.
        with_lines: Whether to populate each node's line range.

    Returns:
        ReadBatchResult: One result per file (with its own id-level warnings), and
        one file-level error per file that failed entirely (bad path or a syntax error).

    Raises:
        core.AstError: If ``items`` is empty.
    """
    if not items:
        raise core.AstError("'items' must be a non-empty list.")
    results: list[ReadResult] = []
    errors: list[ReadError] = []
    for item in items:
        read: ReadResult | None = None
        if item.ids:
            try:
                read = _read_one(item, with_lines=with_lines)
            except core.AstError as exc:
                errors.append(ReadError(path=item.path, error=str(exc)))
                continue
        if read is None or (not read.nodes and _looks_like_root_intent(item.ids)):
            outlined, outline_errors = _outline_instead(item, with_lines=with_lines)
            results += outlined
            errors += outline_errors
            continue
        results.append(read)
    return ReadBatchResult(results=results, errors=errors)

class ReadNodeTool(ToolDefinition):
    name = 'ast_read'
    title = 'Reads the code, text, and content of the nodes identified by the given IDs within the specified files.'
    description = 'After `ast_outline` or `ast_find`, reads the code, text, content, and children of the given AST node IDs within the given list of absolute file paths (never directories). Use this when you already know the IDs.'
    _ITEM_PROPERTIES = {
        'path': PATH_PROP,
        'ids': {
            'type': 'array',
            'items': {
                'type': 'string'},
            'description': 'List of AST node IDs to read.'}}
    _ITEM_REQUIRED = ['path']
    _ITEMS_DESCRIPTION = 'Per-file node IDs to read.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_read`, translating the MCP schema to/from the AST API."""
        with_lines = bool({'tools', 'edit_lines'} & ctx.session.enabled_tools)

        def item_factory(it: dict[str, Any]) -> ReadItem:
            return ReadItem(path=it['path'], ids=it.get('ids') or [])
        return handle_batch_tool(ctx, item_factory, lambda items: ast_read(items, with_lines=with_lines), core.AstError)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReadNodeTool())
    functions.register(ast_read)