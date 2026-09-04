"""``ast_edit_marks`` tool: mark-based edit within the source of a node addressed by id."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
from xy.ai.mcpc.tools.edit_marks import EditMarksError, edit_marks_text
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['EditMarksNodeResult', 'ast_edit_marks', 'EditMarksNodeTool', 'register']

@dataclass(frozen=True)
class EditMarksNodeResult:
    """Result of :func:`ast_edit_marks`.

    Attributes:
        result: Always ``"success"``.
        id: The node's new id, only set if the edit changed it.
    """
    result: str
    id: str | None = None

def ast_edit_marks(path: str, start_marker: str, end_marker: str, content: str, *, exact: bool=False, id: str | None=None) -> EditMarksNodeResult:
    """Replace everything between the 'start_marker' and 'end_marker' markers inside a node addressed by id.

    The addressed node's source is unparsed, edited between the two markers (both
    included) as with ``edit_marks``, re-parsed, and used to replace the node.

    Args:
        path: Absolute path to the file to modify.
        start_marker: Unique 10-30 char substring marking the beginning of the block, within the node's source.
        end_marker: Unique 10-30 char substring marking the end of the block, within the node's source.
        content: Replacement source for the marked block.
        exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.
        id: Unique id of the target node.

    Returns:
        EditMarksNodeResult: Success status.

    Raises:
        core.AstError: If ``path`` is invalid, ``id`` is not
            given, the path matches zero or more than one node, the markers are not
            found or ambiguous within the node's source, or the edited source has a
            syntax error.
    """
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=id)
    node_source = core.edit_node_source(target)
    stripped_start, stripped_end = (start_marker.strip(), end_marker.strip())
    can_retry = not exact and (stripped_start != start_marker or stripped_end != end_marker)
    try:
        new_source = edit_marks_text(node_source, start_marker, end_marker, content, exact=exact)
    except EditMarksError as exc:
        if not can_retry:
            raise core.AstError(str(exc)) from exc
        '# below, retrying with stripped markers is safe here (unlike for plain text).'
        try:
            new_source = edit_marks_text(node_source, stripped_start, stripped_end, content, exact=exact)
        except EditMarksError:
            raise core.AstError(str(exc)) from exc
    new_id = core.replace_node(target, new_source)
    core.CACHE.save(file_path, tree)
    return EditMarksNodeResult(result='success', id=new_id)

class EditMarksNodeTool(ToolDefinition):
    name = 'ast_edit_marks'
    title = 'Replace large text regions within a AST node between markers'
    description = "Replace everything between and including the unique 'start_marker' and 'end_marker' markers, found within the node addressed by id, with new 'content'."
    input_schema = {
        'type': 'object',
        'strict': True,
        'additionalProperties': False,
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the file.'},
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
            **PATH_SELECTOR_PROPS},
        'required': [
            'path',
            'start_marker',
            'end_marker',
            'content']}
    output_schema = {
        'type': 'object',
        'properties': {
            'result': {
                'type': 'string',
                'description': 'Result status'},
            'id': {
                'type': 'string',
                'description': "The node's new id."}},
        'required': ['result']}
    annotations = {'readOnlyHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_edit_marks(
                args['path'],
                args['start_marker'],
                args['end_marker'],
                args['content'],
                exact=args.get(
                    'exact',
                    False),
                id=args.get('id'))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        if result.id is not None:
            message = f'Node {args.get('id')} was replaced with {result.id}'
        else:
            message = f'Node ID {args.get('id')} unchanged'
        content = {'result': message}
        if result.id is not None:
            content['id'] = result.id
        return ToolResult(content=[text_content(message)], structured_content=content, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditMarksNodeTool())
    functions.register(ast_edit_marks)