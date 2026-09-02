"""``ast_edit_block`` tool: exact-block (old_text -> new_text) edit within a selected node."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
from xy.ai.mcpc.tools._text_match import find as find_text, find_all as find_all_text
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['EditBlockNodeResult', 'ast_edit_block', 'EditBlockNodeTool', 'register']

@dataclass(frozen=True)
class EditBlockNodeResult:
    """Result of :func:`ast_edit_block`.

    Attributes:
        result: Always ``"success"``.
        id: The node's new id, only set if the edit changed it.
    """
    result: str
    id: str | None = None

def _replace_block(source: str, old_text: str, new_text: str, *, exact: bool, replace_all: bool) -> str:
    if replace_all:
        matches = find_all_text(source, old_text, exact=exact)
        if not matches:
            raise core.AstError('Text not found in node.')
        result = source
        for match in sorted(matches, key=lambda m: m.start, reverse=True):
            result = result[:match.start] + new_text + result[match.end:]
        return result
    match = find_text(source, old_text, exact=exact)
    if match.count == 0:
        raise core.AstError('Text not found in node.')
    if match.count > 1:
        raise core.AstError(f'Text is ambiguous – found {match.count} occurrences in node.')
    return source[:match.start] + new_text + source[match.end:]

def ast_edit_block(path: str, old_text: str, new_text: str, *, exact: bool=False, replace_all: bool=False, id: str | None=None) -> EditBlockNodeResult:
    """Replace occurrence(s) of ``old_text`` with ``new_text`` inside a node addressed by id.

    The addressed node's source is unparsed, its ``old_text`` block replaced (as with
    ``edit_block``), re-parsed, and used to replace the node.

    Args:
        path: Absolute path to the file to modify.
        old_text: Unique 10-100 char block to find within the node's source (unless ``replace_all``).
        new_text: Replacement text (may be empty to delete the block).
        exact: If False (default), whitespace in ``old_text`` is matched tolerantly.
        replace_all: If True, replace every occurrence instead of requiring a single match.
        id: Unique id of the target node.

    Returns:
        EditBlockNodeResult: Success status.

    Raises:
        core.AstError: If ``path`` is invalid, ``id`` is not
            given, the path matches zero or more than one node, ``old_text`` is not
            found or (without ``replace_all``) ambiguous within the node's source, or
            the edited source has a syntax error.
    """
    file_path = core.require_path(path)
    tree = core.CACHE.get_tree(file_path)
    target = select_by_path(tree, id=id)
    node_source = core.edit_node_source(target)
    new_source = _replace_block(node_source, old_text, new_text, exact=exact, replace_all=replace_all)
    new_id = core.replace_node(target, new_source)
    core.CACHE.save(file_path, tree)
    return EditBlockNodeResult(result='success', id=new_id)

class EditBlockNodeTool(ToolDefinition):
    name = 'ast_edit_block'
    title = 'Edit text block in AST node'
    description = "In-node block edit: replace occurrence(s) of 'old_text' with 'new_text' within the node addressed by id. Use for a single, self-contained block; prefer ast_edit_marks for larger, marker-delimited regions."
    input_schema = {
        'type': 'object',
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the file.'},
            'old_text': {
                'type': 'string',
                'minLength': 10,
                'maxLength': 100,
                'description': "Text (10-100 chars) to find within the node's source. Must occur exactly once, unless replaceAll is set."},
            'new_text': {
                'type': 'string',
                        'description': 'Replacement text (may be empty to delete the block).'},
            'exact': {
                'type': 'boolean',
                'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                'default': False},
            'replaceAll': {
                'type': 'boolean',
                'description': "If true, replace every occurrence of 'old_text' within the node instead of requiring a single unique match.",
                'default': False},
            **PATH_SELECTOR_PROPS},
        'required': [
            'path',
            'old_text',
            'new_text']}
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
        """Delegate to :func:`ast_edit_block`, translating the MCP schema to/from the AST API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ast_edit_block(
                args['path'], args['old_text'], args['new_text'], exact=args.get(
                    'exact', False), replace_all=args.get(
                        'replaceAll', False), id=args.get('id'))
        except core.AstError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        content = {'result': result.result}
        if result.id is not None:
            content['id'] = result.id
        return ToolResult(structured_content=content, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditBlockNodeTool())
    functions.register(ast_edit_block)