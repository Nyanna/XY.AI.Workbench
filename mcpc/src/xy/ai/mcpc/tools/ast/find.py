"""``ast_find`` tool: find AST nodes by type, name, id, line range or parent type."""
import re
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import SELECTOR_PROPS, list_output_schema
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['FindNodesResult', 'ast_find', 'FindNodesTool', 'register']

@dataclass(frozen=True)
class FindNodesResult:
    """Result of :func:`ast_find`.

    Attributes:
        nodes: Outline-style node descriptions (see :class:`core.OutlineNode`)
            matching the given selectors, suited for retrieval and navigation.
        count: Number of entries in ``nodes``.
    """
    nodes: list[core.OutlineNode]
    count: int

def ast_find(path: str, *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:
    """Find nodes by id, type, name, line range, parent type, text or regexp.

    ``ast_find`` is the single retrieval point that restricts on node properties;
    every other tool addresses nodes purely by ``id``. ``text``/``regexp`` are
    matched against the whole file, and each match is attributed to the most
    specific (smallest) enclosing node rather than to every ancestor whose
    source happens to contain it. Matches are returned with their full source.
    Called with no selector at all, ``ast_find`` returns the whole node tree,
    nested like ``ast_list`` but including source.

    Args:
        path: Absolute path to the file to read.
        id: Engine-independent unique node id (primarily name-based path).
        name: Exact simple name a node's ``name`` must equal.
        node_type: Node type name a node must match (case-insensitive).
        lineno: Line the target node must contain; selects the most specific
            (smallest) matching node. Combined with ``end_lineno``, selects the
            most specific node fully covering ``[lineno, end_lineno]``.
        end_lineno: End line of the target range; see ``lineno``. May be given
            alone to select the most specific node containing that single line.
        parent_type: Node type name of the enclosing container (case-insensitive).
        text: Case-insensitive substring to search for in the file.
        regexp: Regular expression to search for in the file (``re.finditer``).
        with_lines: Whether to populate each match's line range.

    Returns:
        FindNodesResult: The matching node summaries (with source) and their count.
        Any number of matches (including zero) is a normal, successful result.

    Raises:
        core.AstError: If ``path`` is not absolute or does not point to an existing
            regular file, if the source has a syntax error, or if ``regexp`` is not
            a valid regular expression.
    """
    tree = core.load(path)[1]
    exact = dict(id=id, name=name, node_type=node_type, parent_type=parent_type)
    structural = dict(exact, lineno=lineno, end_lineno=end_lineno)
    no_selector = not any(structural.values()) and text is None and (regexp is None)
    if no_selector:
        nodes = core.build_outline(core.locate_all(tree), with_code=True, with_lines=with_lines)
        return FindNodesResult(nodes=nodes, count=len(nodes))
    candidates = core.find(tree, **exact)
    if lineno is not None or end_lineno is not None:
        start = lineno if lineno is not None else end_lineno
        end = end_lineno if end_lineno is not None else lineno
        hit = core.most_specific(candidates, start, end)
        candidates = [hit] if hit is not None else []
    if text is None and regexp is None:
        return FindNodesResult(nodes=[core.node_outline(h, with_code=True, with_lines=with_lines) for h in candidates], count=len(candidates))
    if regexp is not None:
        try:
            pattern = re.compile(regexp)
        except re.error as exc:
            raise core.AstError(f'Invalid regexp: {exc}') from exc
    else:
        pattern = re.compile(re.escape(text), re.IGNORECASE)
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
    return FindNodesResult(nodes=[core.node_outline(h, with_code=True, with_lines=with_lines) for h in ordered], count=len(ordered))

class FindNodesTool(ToolDefinition):
    name = 'ast_find'
    title = 'Find AST nodes'
    description = 'Filter the AST-node tree by type, name, id, line range, parent type, text substring or regexp. Returns matches with their full source.'
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the file.'}, **SELECTOR_PROPS, 'text': {'type': 'string', 'description': "Case-insensitive substring the node's source must contain."}, 'regexp': {'type': 'string', 'description': "Regular expression the node's source must match (re.search)."}}, 'required': ['path']}
    output_schema = list_output_schema()
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_find`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        with_lines = bool({'tools', 'edit-lines'} & ctx.session.enabled_tools)
        try:
            result = ast_find(path=args.get('path'), id=args.get('id'), name=args.get('name'), node_type=args.get('node_type'), lineno=args.get('lineno'), end_lineno=args.get('end_lineno'), parent_type=args.get('parent_type'), text=args.get('text'), regexp=args.get('regexp'), with_lines=with_lines)
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'nodes': [core.to_dict(n) for n in result.nodes], 'count': result.count})

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(FindNodesTool())
    functions.register(ast_find)