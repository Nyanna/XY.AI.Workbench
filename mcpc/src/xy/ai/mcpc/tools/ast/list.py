"""``ast_list`` tool: list AST nodes of one or more files."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ListNodesResult', 'ListNodesError', 'ListNodesBatchResult', 'ast_list', 'ListNodesTool', 'register']
_MAX_DIR_EXPANSION = 5

@dataclass(frozen=True)
class ListNodesResult:
    """Nodes of a single file.

    Attributes:
        path: The resolved path of the listed file (see ``ast_list``'s
            directory-expansion note), for result association.
        nodes: Outline-style node descriptions (see :class:`core.OutlineNode`), in
            document order, suited for retrieval and navigation.
    """
    path: str
    nodes: list[core.OutlineNode]

@dataclass(frozen=True)
class ListNodesError:
    """Error listing a single file.

    Attributes:
        path: The resolved path that failed, for result association.
        error: The error message.
    """
    path: str
    error: str

@dataclass(frozen=True)
class ListNodesBatchResult:
    """Result of :func:`ast_list`.

    Attributes:
        results: One :class:`ListNodesResult` per successfully listed file.
        errors: One :class:`ListNodesError` per file that failed.
    """
    results: list[ListNodesResult]
    errors: list[ListNodesError]

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

def _list_one(path: str, *, with_lines: bool) -> ListNodesResult:
    tree = core.load(path)[1]
    nodes = core.build_outline(core.locate_all(tree), with_lines=with_lines)
    return ListNodesResult(path=path, nodes=nodes)

def ast_list(paths: list[str], *, with_lines: bool=True) -> ListNodesBatchResult:
    """List the hierarchical AST-node tree of one or more files.

    The tree is the foundation every other tool builds on: each node carries its
    unique, primarily name-based ``id`` and line range, but never its source –
    use ``ast_find`` (property/text filtering) or ``ast_read`` (by id) to
    retrieve source.

    A path naming a directory without subdirectories and holding at most
    5 files is transparently expanded to those files; any other directory
    fails normally.

    Args:
        paths: Absolute paths of the files to list. Must be non-empty.
        with_lines: Whether to populate each node's line range.

    Returns:
        ListNodesBatchResult: One result per listed file, one error per failed file.

    Raises:
        core.AstError: If ``paths`` is empty.
    """
    if not paths:
        raise core.AstError("'paths' must be a non-empty list.")
    results: list[ListNodesResult] = []
    errors: list[ListNodesError] = []
    for path in paths:
        for real_path in _expand_path(path):
            try:
                results.append(_list_one(real_path, with_lines=with_lines))
            except core.AstError as exc:
                errors.append(ListNodesError(path=real_path, error=str(exc)))
    return ListNodesBatchResult(results=results, errors=errors)

class ListNodesTool(ToolDefinition):
    name = 'ast_list'
    title = 'List AST nodes'
    description = "Hierarchical tree of one or more files' AST nodes (import/statement segments, classes, functions, sections) with id and optional line range – no source. A directory without subdirectories and with at most 5 files is transparently expanded to those files."
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
    output_schema = {
        '$defs': {
            'outline_node': core.OUTLINE_NODE_SCHEMA}, 'type': 'object', 'properties': {
                'results': {
                    'type': 'array', 'items': {
                        'type': 'object', 'properties': {
                            'path': {
                                'type': 'string'}, 'nodes': {
                                    'type': 'array', 'items': {
                                        '$ref': '#/$defs/outline_node'}}}, 'required': [
                                            'path', 'nodes']}}, 'errors': {
                                                'type': 'array', 'items': {
                                                    'type': 'object', 'properties': {
                                                        'path': {
                                                            'type': 'string'}, 'error': {
                                                                'type': 'string'}}, 'required': [
                                                                    'path', 'error']}}}, 'required': [
                                                                        'results', 'errors']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_list`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
        paths = args.get('paths') or []
        if not paths:
            return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
        batch = ast_list(paths=paths, with_lines=with_lines)
        content = {'results': [{'path': r.path, 'nodes': [core.to_dict(n) for n in r.nodes]} for r in batch.results], 'errors': [
            {'path': e.path, 'error': e.error} for e in batch.errors]}
        is_error = bool(batch.errors) and (not batch.results)
        return ToolResult(structured_content=content, is_error=is_error)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ListNodesTool())
    functions.register(ast_list)