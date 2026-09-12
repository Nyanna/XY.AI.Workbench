"""Edit Marks tool – replaces the text strictly including two markers, for a batch of items."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
from xy.ai.mcpc.tools._text_match import replace_between, marks_line_preserving, TextMatchError
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = [
    'EditMarksError',
    'EditMarksItem',
    'EditMarksResult',
    'EditMarksItemError',
    'EditMarksBatchResult',
    'edit_marks',
    'edit_marks_text',
    'EditMarksTool',
    'register_edit_marks_tool']

class EditMarksError(Exception):
    """Raised when a replace operation cannot be performed."""

@dataclass(frozen=True)
class EditMarksItem:
    """One marker-based replacement to apply.

    Attributes:
        path: Absolute path to target file.
        begin_marker: Unique substring marking the beginning of the block.
        end_marker: Unique substring marking the end of the block.
        content: Replacement text.
        exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.
    """
    path: str
    begin_marker: str
    end_marker: str
    content: str
    exact: bool = False

@dataclass(frozen=True)
class EditMarksResult:
    """Result of applying a single marker-based replacement, mirroring its input path for result association."""
    path: str
    result: str

@dataclass(frozen=True)
class EditMarksItemError:
    """Error applying a single marker-based replacement, mirroring its input path for result association."""
    path: str
    error: str

@dataclass(frozen=True)
class EditMarksBatchResult:
    """Result of :func:`edit_marks`.

    Attributes:
        results: One :class:`EditMarksResult` per successfully applied edit.
        errors: One :class:`EditMarksItemError` per edit that failed.
    """
    results: list[EditMarksResult]
    errors: list[EditMarksItemError]

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

def _edit_marks_one(item: EditMarksItem) -> EditMarksResult:
    file_path = Path(item.path)
    if not file_path.is_absolute():
        raise EditMarksError('Path must be absolute.')
    if not file_path.exists():
        raise EditMarksError('File not found.')
    if not file_path.is_file():
        raise EditMarksError('Not a regular file.')
    text = file_path.read_text(encoding='utf-8')
    result_text = edit_marks_text(text, item.begin_marker, item.content, item.end_marker, exact=item.exact)
    try:
        file_path.write_text(result_text, encoding='utf-8')
    except OSError as exc:
        raise EditMarksError(f'Write failed: {exc}') from exc
    return EditMarksResult(path=item.path, result='success')

def edit_marks(items: list[EditMarksItem]) -> EditMarksBatchResult:
    """Replace everything between and including 'begin_marker' and 'end_marker' with content, in one or more files.

    Both markers are included in the replacement.

    Args:
        items: Marker-based edits to apply. Must be non-empty.

    Returns:
        EditMarksBatchResult: one result per successfully applied edit, one error per failed edit.

    Raises:
        EditMarksError: If items is empty.
    """
    if not items:
        raise EditMarksError("'items' must be a non-empty list.")
    results: list[EditMarksResult] = []
    errors: list[EditMarksItemError] = []
    for item in items:
        try:
            results.append(_edit_marks_one(item))
        except EditMarksError as exc:
            errors.append(EditMarksItemError(path=item.path, error=str(exc)))
    return EditMarksBatchResult(results=results, errors=errors)

class EditMarksTool(ToolDefinition):
    name = 'edit_marks'
    title = 'Replace text between two marks'
    description = "Replace everything between and including the unique 'begin_marker' and 'end_marker' markers, found in one or more files, with new 'content', for a batch of items."
    _ITEM_PROPERTIES = {
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
            'description': 'Unique 10-30 char substring marking the end of the text to replace.'},
        'exact': {
            'type': 'boolean',
            'description': "If true, 'begin_marker'/'end_marker' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
            'default': False}}
    _ITEM_REQUIRED = ['path', 'begin_marker', 'end_marker', 'content']
    _ITEMS_DESCRIPTION = 'Marker-based edits to apply.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> EditMarksItem:
            return EditMarksItem(
                path=it['path'],
                begin_marker=it['begin_marker'],
                end_marker=it['end_marker'],
                content=it['content'],
                exact=it.get(
                    'exact',
                    False))
        return handle_batch_tool(ctx, item_factory, edit_marks, EditMarksError, auto_approve=True)

def register_edit_marks_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditMarksTool())
    functions.register(edit_marks)