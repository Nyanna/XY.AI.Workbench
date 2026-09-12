"""``ast_edit_marks`` tool: mark-based edits within the source of selected nodes."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, PATH_PROP, select_by_text
from xy.ai.mcpc.tools._tool_helpers import batch_schema
from xy.ai.mcpc.tools._text_match import replace_between, marks_line_preserving, TextMatchError
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
__all__ = [
    'EditMarksItem',
    'EditMarksResult',
    'EditMarksError',
    'EditMarksBatchResult',
    'ast_edit_marks',
    'EditMarksNodeTool',
    'register']

@dataclass(frozen=True)
class EditMarksItem:
    """One marker-based edit to apply.

    Attributes:
        path: Absolute path to the file to modify.
        start_marker: Unique 10-30 char substring marking the beginning of the block, within the node's source.
        end_marker: Unique 10-30 char substring marking the end of the block, within the node's source.
        content: Replacement source for the marked block.
        exact: If False (default), whitespace in start/end is matched tolerantly.
        id: Unique id of the target node. If omitted, the node is searched for by the markers.
    """
    path: str
    start_marker: str
    end_marker: str
    content: str
    exact: bool = False
    id: str | None = None

@dataclass(frozen=True)
class EditMarksResult:
    """Result of applying a single marker-based edit.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        result: Always ``"success"``.
        new_id: The node's new id, only set if the edit changed it.
    """
    path: str
    id: str | None
    result: str
    new_id: str | None = None

@dataclass(frozen=True)
class EditMarksError:
    """Error applying a single marker-based edit.

    Attributes:
        path: The path exactly as given in the input, for result association.
        id: The id exactly as given in the input, for result association.
        error: The error message.
        candidates: On ambiguity (id omitted, several nodes matched), the candidate node ids.
    """
    path: str
    id: str | None
    error: str
    candidates: list[str] | None = None

@dataclass(frozen=True)
class EditMarksBatchResult:
    """Result of :func:`ast_edit_marks`.

    Attributes:
        results: One :class:`EditMarksResult` per successfully applied edit.
        errors: One :class:`EditMarksError` per edit that failed.
    """
    results: list[EditMarksResult]
    errors: list[EditMarksError]

def _node_marks_guard(engine, begin_marker, end_marker):
    """Guard for tolerant marker edits (see ``ast.edit_block._node_guard``)."""
    if engine.validates_syntax:
        return None
    keep_lines = marks_line_preserving(begin_marker, end_marker)
    return lambda begin, end, result: keep_lines(begin, end, result) and engine.validate(result) is None

def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:
    file_path = core.require_path(item.path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_text(tree, [item.start_marker, item.end_marker], id=item.id)
    node_source = core.edit_node_source(target)
    begin, end = (item.start_marker, item.end_marker) if item.exact else (
        item.start_marker.strip(), item.end_marker.strip())
    try:
        new_source = replace_between(
            node_source,
            begin,
            end,
            item.content,
            exact=item.exact,
            accept=_node_marks_guard(
                tree.engine,
                begin,
                end),
            max_level=3 if tree.engine.validates_syntax else 2,
            where='node')
    except TextMatchError as exc:
        raise core.AstError(str(exc)) from exc
    new_id = core.replace_node(target, new_source)
    core.CACHE.save(file_path, tree)
    return EditMarksResult(path=item.path, id=item.id, result='success', new_id=new_id)

def ast_edit_marks(items: list[EditMarksItem]) -> EditMarksBatchResult:
    """Replace everything between the start/end markers inside one or more nodes.

    Each addressed node's source is unparsed, edited between the two markers
    (both included) with escalating whitespace/escape/quote tolerance,
    re-parsed, and used to replace the node.

    Args:
        items: Marker-based edits to apply. Must be non-empty.

    Returns:
        EditMarksBatchResult: One result per successful edit, one error per failed edit.

    Raises:
        core.AstError: If ``items`` is empty.
    """
    if not items:
        raise core.AstError("'items' must be a non-empty list.")
    results: list[EditMarksResult] = []
    errors: list[EditMarksError] = []
    for item in items:
        try:
            results.append(_edit_marks_one(item))
        except core.AstAmbiguous as exc:
            errors.append(EditMarksError(path=item.path, id=item.id, error=str(exc), candidates=exc.candidates))
        except core.AstError as exc:
            errors.append(EditMarksError(path=item.path, id=item.id, error=str(exc)))
    return EditMarksBatchResult(results=results, errors=errors)

class EditMarksNodeTool(ToolDefinition):
    name = 'ast_edit_marks'
    title = 'Replace large text regions within AST nodes between markers'
    description = "Replace everything between and including the unique 'start_marker' and 'end_marker' markers, found within nodes addressed by id, with new 'content', for a batch of items. Returns changed IDs in the result."
    _ITEM_PROPERTIES = {
        'path': PATH_PROP,
        'start_marker': {
            'type': 'string',
            'minLength': 10,
            'maxLength': 30,
            'description': "Unique 10-30 char substring marking the beginning of the text to replace, within the node's source."},
        'end_marker': {
            'type': 'string',
            'minLength': 10,
            'maxLength': 30,
            'description': "Unique 10-30 char substring marking the end of the text to replace, within the node's source."},
        'content': {
            'type': 'string',
                    'description': 'Replacement source for the marked text.'},
        'exact': {
            'type': 'boolean',
            'description': "If true, 'start_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
            'default': False},
        **PATH_SELECTOR_PROPS}
    _ITEM_REQUIRED = ['path', 'start_marker', 'end_marker', 'content']
    _ITEMS_DESCRIPTION = 'Marker-based edits to apply.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST API."""

        def item_factory(it: dict[str, Any]) -> EditMarksItem:
            return EditMarksItem(
                path=it['path'],
                start_marker=it['start_marker'],
                end_marker=it['end_marker'],
                content=it['content'],
                exact=it.get(
                    'exact',
                    False),
                id=it.get('id'))

        def result_serializer(r: EditMarksResult) -> dict[str, Any]:
            entry = {'path': r.path, 'id': r.id, 'result': r.result}
            if r.new_id is not None:
                entry['new_id'] = r.new_id
            return entry

        def error_serializer(e: EditMarksError) -> dict[str, Any]:
            entry = {'path': e.path, 'id': e.id, 'error': e.error}
            if e.candidates is not None:
                entry['candidates'] = e.candidates
            return entry
        return handle_batch_tool(
            ctx,
            item_factory,
            ast_edit_marks,
            core.AstError,
            result_serializer,
            error_serializer,
            auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditMarksNodeTool())
    functions.register(ast_edit_marks)