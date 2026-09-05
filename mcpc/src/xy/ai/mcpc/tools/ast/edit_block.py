"""``ast_edit_block`` tool: exact-block (old_text -> new_text) edit within a selected node."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.ast import core
from xy.ai.mcpc.tools.ast.common import PATH_SELECTOR_PROPS, select_by_path
from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, TextMatchError
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

def _node_guard(engine, reference):
    """Guard for tolerant node edits.

    Engines that flag malformed edits on replace need no extra check. Others
    (e.g. markup grammars) must not merge lines and must still re-parse cleanly.
    """
    if engine.validates_syntax:
        return None
    keep_lines = line_preserving(reference)
    return lambda span, result: keep_lines(span, result) and engine.validate(result) is None

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
    try:
        new_source = replace_in_block(
            node_source,
            old_text,
            new_text,
            exact=exact,
            replace_all=replace_all,
            accept=_node_guard(
                tree.engine,
                old_text),
            max_level=3 if tree.engine.validates_syntax else 2,
            where='node')
    except TextMatchError as exc:
        raise core.AstError(str(exc)) from exc
    new_id = core.replace_node(target, new_source)
    core.CACHE.save(file_path, tree)
    return EditBlockNodeResult(result='success', id=new_id)

class EditBlockNodeTool(ToolDefinition):
    name = 'ast_edit_block'
    title = 'Replace short text within AST node'
    description = "Replace occurrence of short 'old_text' with 'new_text', within the node addressed by id. Don't use for large edits, use ast_edit_marks instead."
    input_schema = {
        'type': 'object',
        'strict': True,
        'additionalProperties': False,
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the file.'},
            'old_text': {
                'type': 'string',
                'minLength': 10,
                'maxLength': 100,
                'description': 'Short text (10-100 chars) to replace within the node. Must occur exactly once, or replaceAll is set.'},
            'new_text': {
                'type': 'string',
                        'description': 'Replacement text, may be empty to remove the text.'},
            'exact': {
                'type': 'boolean',
                'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                'default': False},
            'replaceAll': {
                'type': 'boolean',
                'description': "If true, replace every occurrence of 'old_text' within the node instead of a single unique match.",
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
                'type': 'string',
                'description': 'Result status'},
            'id': {
                'type': 'string',
                'description': "The node's new id."}},
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
        if result.id is not None:
            message = f'Node {args.get('id')} was replaced with {result.id}'
        else:
            message = f'Node ID {args.get('id')} unchanged'
        content = {'result': message}
        if result.id is not None:
            content['id'] = result.id
        return ToolResult(content=[text_content(message)], structured_content=content, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditBlockNodeTool())
    functions.register(ast_edit_block)