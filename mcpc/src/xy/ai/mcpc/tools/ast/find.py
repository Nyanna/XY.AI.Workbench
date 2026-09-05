"""``ast_find`` tool: find AST nodes by type, name, id, line range or parent type."""
import re
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['FileNodesResult', 'FindNodesResult', 'ast_find', 'FindNodesTool', 'register']

@dataclass(frozen=True)
class FileNodesResult:
    """Nodes matching the given selectors within a single file.

    Attributes:
        path: The path exactly as given in the input.
        nodes: Outline-style node descriptions (see :class:`core.OutlineNode`)
            matching the given selectors, suited for retrieval and navigation.
    """
    path: str
    nodes: list[core.OutlineNode]

@dataclass(frozen=True)
class FindNodesResult:
    """Result of :func:`ast_find`.

    Attributes:
        files: One :class:`FileNodesResult` per input path, in order.
    """
    files: list[FileNodesResult]

def _find_in_file(path: str, *, exact: dict[str, Any], lineno: int | None, end_lineno: int | None, no_selector: bool, pattern: re.Pattern[str] | None, with_lines: bool) -> FileNodesResult:
    tree = core.load(path)[1]
    if no_selector:
        nodes = core.build_outline(core.locate_all(tree), with_code=True, with_lines=with_lines)
        return FileNodesResult(path=path, nodes=nodes)
    candidates = core.find(tree, **exact)
    if lineno is not None or end_lineno is not None:
        start = lineno if lineno is not None else end_lineno
        end = end_lineno if end_lineno is not None else lineno
        hit = core.most_specific(candidates, start, end)
        candidates = [hit] if hit is not None else []
    if pattern is None:
        nodes = [core.node_outline(h, with_code=True, with_lines=with_lines) for h in candidates]
        return FileNodesResult(path=path, nodes=nodes)
    source = tree.source
    seen: set[str] = set()
    ordered: list[core.Located] = []
    for m in pattern.finditer(source):
        start_line = source.count('\n', 0, m.start()) + 1
        end_line = source.count('\n', 0, max(m.end() - 1, m.start())) + 1
        loc = core.most_specific(candidates, start_line, end_line)
        if loc is not None and loc.node_id not in seen:
            seen.add(loc.node_id)
            ordered.append(loc)
    nodes = [core.node_outline(h, with_code=True, with_lines=with_lines) for h in ordered]
    return FileNodesResult(path=path, nodes=nodes)

def ast_find(paths: list[str], *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:
    """Find nodes by id, type, name, line range, parent type, text or regexp.

    ``ast_find`` is the single retrieval point that restricts on node properties;
    every other tool addresses nodes purely by ``id``. ``text``/``regexp`` are
    matched against each file's whole source, and each match is attributed to the
    most specific (smallest) enclosing node rather than to every ancestor whose
    source happens to contain it. Matches are returned with their full source.
    Called with no selector at all, ``ast_find`` returns the whole node tree per
    file, nested like ``ast_list`` but including source.

    Args:
        paths: Absolute paths of the files to search. Must be non-empty.
        id: Engine-independent unique node id (primarily name-based path).
        name: Exact simple name a node's ``name`` must equal.
        node_type: Node type name a node must match (case-insensitive).
        lineno: Line the target node must contain; selects the most specific
            (smallest) matching node. Combined with ``end_lineno``, selects the
            most specific node fully covering ``[lineno, end_lineno]``.
        end_lineno: End line of the target range; see ``lineno``. May be given
            alone to select the most specific node containing that single line.
        parent_type: Node type name of the enclosing container (case-insensitive).
        text: Case-insensitive substring to search for in each file.
        regexp: Regular expression to search for in each file (``re.finditer``).
        with_lines: Whether to populate each match's line range.

    Returns:
        FindNodesResult: One :class:`FileNodesResult` per path, in order. Any
        number of matches per file (including zero) is a normal, successful result.

    Raises:
        core.AstError: If ``paths`` is empty, if any path is not absolute or does
            not point to an existing regular file, if a file has a syntax error, or
            if ``regexp`` is not a valid regular expression.
    """
    if not paths:
        raise core.AstError("'paths' must be a non-empty list.")
    exact = dict(id=id, name=name, node_type=node_type, parent_type=parent_type)
    structural = dict(exact, lineno=lineno, end_lineno=end_lineno)
    no_selector = not any(structural.values()) and text is None and (regexp is None)
    pattern: re.Pattern[str] | None = None
    if not no_selector and (text is not None or regexp is not None):
        if regexp is not None:
            try:
                pattern = re.compile(regexp)
            except re.error as exc:
                raise core.AstError(f'Invalid regexp: {exc}') from exc
        else:
            pattern = re.compile(re.escape(text), re.IGNORECASE)
    files = [
        _find_in_file(
            p,
            exact=exact,
            lineno=lineno,
            end_lineno=end_lineno,
            no_selector=no_selector,
            pattern=pattern,
            with_lines=with_lines) for p in paths]
    return FindNodesResult(files=files)

class FindNodesTool(ToolDefinition):
    name = 'ast_find'
    title = 'Find AST nodes'
    description = 'Filter the AST-node tree of a list of files by type, name, id, line range, parent type, text substring or regexp. Returns matches per file with their full source.'
    input_schema = {
        'type': 'object',
        'properties': {
            'paths': {
                'type': 'array',
                'items': {
                    'type': 'string'},
                'description': 'Absolute paths of the files to search.'},
            **SELECTOR_PROPS,
            'text': {
                'type': 'string',
                        'description': "Case-insensitive substring the node's source must contain."},
            'regexp': {
                'type': 'string',
                'description': "Regular expression the node's source must match (re.search)."}},
        'required': ['paths']}
    output_schema = {
        '$defs': {
            'outline_node': core.OUTLINE_NODE_SCHEMA},
        'type': 'object',
        'properties': {
                'files': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'path': {
                                'type': 'string'},
                            'nodes': {
                                'type': 'array',
                                'items': {
                                        '$ref': '#/$defs/outline_node'}}},
                        'required': [
                            'path',
                            'nodes']}}},
        'required': ['files']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        paths = args.get('paths')
        if not isinstance(paths, list):
            return ToolResult(content=[text_content("'paths' must be a non-empty list.")], is_error=True)
        with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
        try:
            result = ast_find(
                paths=paths,
                id=args.get('id'),
                name=args.get('name'),
                node_type=args.get('node_type'),
                lineno=args.get('lineno'),
                end_lineno=args.get('end_lineno'),
                parent_type=args.get('parent_type'),
                text=args.get('text'),
                regexp=args.get('regexp'),
                with_lines=with_lines)
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'files': [{'path': f.path, 'nodes': [
                          core.to_dict(n) for n in f.nodes]} for f in result.files]})

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(FindNodesTool())
    functions.register(ast_find)