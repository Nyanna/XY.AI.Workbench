"""Edit Marks tool – replaces the text strictly including two markers."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._text_match import replace_between, marks_line_preserving, TextMatchError
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = [
    'EditMarksError',
    'EditMarksResult',
    'edit_marks',
    'edit_marks_text',
    'EditMarksTool',
    'register_edit_mark_tool']

class EditMarksError(Exception):
    """Raised when a replace operation cannot be performed."""

@dataclass(frozen=True)
class EditMarksResult:
    result: str

def edit_marks_text(text: str, begin_marker: str, content: str, end_marker: str, exact: bool=False) -> str:
    """Replace everything between and including 'begin_marker' and 'end_marker' with content, in *text*.

    Both markers are included in the replacement. Matching escalates through
    whitespace/escape/quote tolerance; each marker match must keep its own line
    count, so tolerant matching never merges lines into a syntax error.

    Args:
        text: Source text to edit.
        begin_marker: Unique substring marking the beginning of the block.
        content: Replacement text.
        end_marker: Unique substring marking the end of the block.
        exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.

    Returns:
        The edited text.

    Raises:
        EditMarksError: If start or end markers are not found or appear more than once.
        EditMarksError: If end marker does not start after start marker ends.
    """
    begin, end = (begin_marker, end_marker) if exact else (begin_marker.strip(), end_marker.strip())
    try:
        return replace_between(
            text,
            begin,
            end,
            content,
            exact=exact,
            accept=marks_line_preserving(
                begin,
                end),
            max_level=2,
            where='file')
    except TextMatchError as exc:
        raise EditMarksError(str(exc)) from exc

def edit_marks(path: str, begin_marker: str, end_marker: str, content: str, exact: bool=False) -> EditMarksResult:
    """Replace everything between and including 'start' and 'end' with content.

    Both markers are included in the replacement.

    Args:
        path: Absolute path to target file.
        start: Unique substring marking the beginning of the block.
        end: Unique substring marking the end of the block.
        content: Replacement text.
        exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.

    Returns:
        EditMarksResult with success status.

    Raises:
        EditMarksError: If path is not absolute, not found, or not a regular file.
        EditMarksError: If start or end markers are not found or appear more than once.
        EditMarksError: If end marker does not start after start marker ends.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise EditMarksError('Path must be absolute.')
    if not file_path.exists():
        raise EditMarksError('File not found.')
    if not file_path.is_file():
        raise EditMarksError('Not a regular file.')
    text = file_path.read_text(encoding='utf-8')
    result_text = edit_marks_text(text, begin_marker, content, end_marker, exact=exact)
    try:
        file_path.write_text(result_text, encoding='utf-8')
    except OSError as exc:
        raise EditMarksError(f'Write failed: {exc}') from exc
    return EditMarksResult(result='success')

class EditMarksTool(ToolDefinition):
    name = 'edit_marks'
    title = 'Replace text between two marks'
    description = "Replace everything between and including the unique 'start_marker' and 'end_marker' markers with new 'content'."
    input_schema = {
        'type': 'object',
        'strict': True,
        'additionalProperties': False,
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the target file.'},
            'begin_marker': {
                'type': 'string',
                'minLength': 10,
                'maxLength': 30,
                'description': 'Unique 10-30 char substring marking the beginning of the text to replace.'},
            'content': {
                'type': 'string',
                        'description': 'Replacement source for the marked text.'},
            'end_marker': {
                'type': 'string',
                'minLength': 10,
                'maxLength': 30,
                'description': 'Unique 10-30 char substring marking the end of the text to replace'},
            'exact': {
                'type': 'boolean',
                'description': "If true, 'begin_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                'default': False}},
        'required': [
            'path',
            'begin_marker',
            'end_marker',
            'content']}
    output_schema = {
        'type': 'object',
        'properties': {
            'result': {
                'type': 'string',
                'description': '``success`` on success.'}},
        'required': []}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = edit_marks(
                path=args['path'],
                begin_marker=args['begin_marker'],
                end_marker=args['end_marker'],
                content=args['content'],
                exact=args.get(
                    'exact',
                    False))
        except EditMarksError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_edit_marks_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditMarksTool())
    functions.register(edit_marks)