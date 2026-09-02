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

def ast_edit_marks(path: str, block_start: str, block_end: str, content: str, *, exact: bool=False, id: str | None=None) -> EditMarksNodeResult:
    """Replace everything between the 'block_start' and 'block_end' markers inside a node addressed by id.

    The addressed node's source is unparsed, edited between the two markers (both
    included) as with ``edit_marks``, re-parsed, and used to replace the node.

    Args:
        path: Absolute path to the file to modify.
        block_start: Unique 10-30 char substring marking the beginning of the block, within the node's source.
        block_end: Unique 10-30 char substring marking the end of the block, within the node's source.
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
    try:
        new_source = edit_marks_text(node_source, block_start, block_end, content, exact=exact)
    except EditMarksError as exc:
        raise core.AstError(str(exc)) from exc
    new_id = core.replace_node(target, new_source)
    core.CACHE.save(file_path, tree)
    return EditMarksNodeResult(result='success', id=new_id)

class EditMarksNodeTool(ToolDefinition):
    name = 'ast_edit_marks'
    title = 'Edit AST node between markers'
    description = "In-node marker edit: replace everything strictly between and including the unique 'block_start' and 'block_end' markers, found within the node addressed by id, with 'content'. Ideal for focused in-section changes."
    input_schema = {
        'type': 'object',
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the file.'},
            'block_start': {
                'type': 'string',
                'minLength': 10,
                'maxLength': 30,
                'description': "Unique 10-30 char substring marking the beginning of the block, within the node's source."},
            'block_end': {
                'type': 'string',
                        'minLength': 10,
                        'maxLength': 30,
                        'description': "Unique 10-30 char substring marking the end of the block, within the node's source."},
            'content': {
                'type': 'string',
                'description': 'Replacement source for the marked block.'},
            'exact': {
                'type': 'boolean',
                'description': "If true, 'block_start'/'block_end' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                'default': False},
            **PATH_SELECTOR_PROPS},
        'required': [
            'path',
            'block_start',
            'block_end',
            'content']}
    output_schema = {
        'type': 'object',
        'properties': {
            'result': {
                'type': 'string'},
            'id': {
                'type': 'string',
                'description': "The node's new id, if the edit changed it."}},
        'required': ['result']}
    annotations = {'readOnlyHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ast_edit_marks`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_edit_marks(
                args['path'],
                args['block_start'],
                args['block_end'],
                args['content'],
                exact=args.get(
                    'exact',
                    False),
                id=args.get('id'))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        content = {'result': result.result}
        if result.id is not None:
            content['id'] = result.id
        return ToolResult(structured_content=content, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditMarksNodeTool())
    functions.register(ast_edit_marks)