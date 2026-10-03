"""``ast_outline`` tool: list AST nodes of one or more files."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import require_items, serialize_batch_result
from xy.ai.mcpc.tools.list import ListError, ListItem
from xy.ai.mcpc.tools.list import list as _list_dirs
__all__ = [
    'OutlineNodesResult',
    'OutlineNodesError',
    'OutlineNodesBatchResult',
    'ast_outline',
    'OutlineNodesTool',
    'register']
_MAX_DIR_EXPANSION = 5

@dataclass(frozen=True)
class OutlineNodesResult:
    """Nodes of a single file.

    Attributes:
        path: The resolved path of the listed file (see ``ast_outline``'s
            directory-expansion note), for result association.
        nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
            document order, suited for retrieval and navigation.
    """
    path: str
    nodes: list[core.OutlineNode]

@dataclass(frozen=True)
class OutlineNodesError:
    """Error listing a single file.

    Attributes:
        path: The resolved path that failed, for result association.
        error: The error message.
    """
    path: str
    error: str

@dataclass(frozen=True)
class OutlineNodesBatchResult:
    """Result of :func:`ast_outline`.

    Attributes:
        results: One :class:`OutlineNodesResult` per successfully listed file.
        errors: One :class:`OutlineNodesError` per file that failed.
    """
    results: list[OutlineNodesResult]
    errors: list[OutlineNodesError]

def _expand_path(path_str: str) -> list[str]:
    """Expand *path_str* to its own single-element list, unless it names a
    directory that can be unambiguously expanded to its files.

    Expansion only succeeds if the directory has no subdirectories and holds
    at most :data:`_MAX_DIR_EXPANSION` files; otherwise *path_str* is returned
    unchanged and left to fail normally.
    """
    p = Path(path_str)
    if not p.is_dir():
        return [path_str]
    entries = list(p.iterdir())
    if any((e.is_dir() for e in entries)):
        return [path_str]
    files = sorted((e for e in entries if e.is_file()), key=lambda e: e.name)
    if not files or len(files) > _MAX_DIR_EXPANSION:
        return [path_str]
    return [str(f) for f in files]

def _outline_one(path: str, *, with_lines: bool) -> OutlineNodesResult:
    tree = core.load(path)[1]
    nodes = core.build_outline(core.locate_all(tree), with_lines=with_lines, with_type=False)
    return OutlineNodesResult(path=path, nodes=nodes)

def _directory_listing_hint(path: str) -> str:
    """Build an error message for a path that turned out to be a directory,
    anticipating the likely intent: list its contents (via the ``list`` tool)
    instead of outlining it as a file.
    """
    hint = 'Path is a directory, not a file; listing its contents instead (intent anticipated).'
    try:
        batch = _list_dirs([ListItem(path=path)])
    except ListError as exc:
        return f'{hint} Listing failed: {exc}'
    if batch.errors:
        return f'{hint} Listing failed: {batch.errors[0].error}'
    result = batch.results[0]
    parts = [hint]
    if result.warning:
        parts.append(result.warning)
    parts.extend(result.entries)
    return '\n'.join(parts)

def ast_outline(paths: list[str], *, with_lines: bool=True) -> OutlineNodesBatchResult:
    """List the hierarchical AST-node tree of one or more files.

    The tree is the foundation every other tool builds on: each node carries its
    unique, primarily name-based ``id`` and line range, but never its source –
    use ``ast_find`` (property/text filtering) or ``ast_read`` (by id) to
    retrieve source.

    A path naming a directory without subdirectories and holding at most
    5 files is transparently expanded to those files; any other directory
    is reported as an error whose message contains the directory's listing
    (via the ``list`` tool), anticipating the likely intent.

    Args:
        paths: Absolute paths of the files to list. Must be non-empty.
        with_lines: Whether to populate each node's line range.

    Returns:
        OutlineNodesBatchResult: One result per listed file, one error per failed file.

    Raises:
        core.AstError: If ``paths`` is empty.
    """
    if not paths:
        raise core.AstError("'paths' must be a non-empty list.")
    results: list[OutlineNodesResult] = []
    errors: list[OutlineNodesError] = []
    for path in paths:
        for real_path in _expand_path(path):
            if Path(real_path).is_dir():
                errors.append(OutlineNodesError(path=real_path, error=_directory_listing_hint(real_path)))
                continue
            try:
                results.append(_outline_one(real_path, with_lines=with_lines))
            except core.AstError as exc:
                errors.append(OutlineNodesError(path=real_path, error=str(exc)))
    return OutlineNodesBatchResult(results=results, errors=errors)

class OutlineNodesTool(ToolDefinition):
    name = 'ast_outline'
    title = 'List AST nodes of files'
    description = "Hierarchical tree of one or more files' AST nodes (import/statement segments, classes, functions, sections) with id and optional line range – no source. A directory without subdirectories and with at most 5 files is transparently expanded to those files; any other directory is reported as an error containing its listing. " + core.OUTLINE_NODE_DESCRIPTION
    input_schema = {
        'type': 'object',
        'properties': {
            'paths': {
                'type': 'array',
                'minItems': 1,
                'items': {
                    'type': 'string'},
                'description': 'Absolute paths of the files to list.'}},
        'required': ['paths']}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_list`, translating the MCP schema to/from the AST API."""
        paths, error = require_items(ctx, key='paths')
        if error is not None:
            return error
        with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
        batch = ast_outline(paths=paths, with_lines=with_lines)
        result_serializer = lambda r: {'path': r.path, 'nodes': [core.to_dict(n) for n in r.nodes]}
        error_serializer = lambda e: {'path': e.path, 'error': e.error}
        content = serialize_batch_result(batch, result_serializer, error_serializer)
        return ToolResult(structured_content=content)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(OutlineNodesTool())
    functions.register(ast_outline)