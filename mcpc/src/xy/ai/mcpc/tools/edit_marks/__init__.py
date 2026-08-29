"""Edit Marks tool – replaces the text strictly including two markers."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._text_match import find as find_text
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['EditMarksError', 'EditMarksResult', 'edit_marks', 'EditMarksTool', 'register_edit_mark_tool']

class EditMarksError(Exception):
    """Raised when a replace operation cannot be performed."""

@dataclass(frozen=True)
class EditMarksResult:
    result: str

def edit_marks(path: str, start: str, content: str, end: str | None=None, exact: bool=False) -> EditMarksResult:
    """Replace text at/around the unique 'start' marker with 'content'.

    If 'end' is given, everything between and including 'start' and 'end' is
    replaced (both markers included). If 'end' is omitted, only the 'start'
    marker itself is replaced, which allows replacing a single line, inserting
    content before/after it (by including the marker text in 'content'), or
    deleting it (by passing empty 'content').

    Args:
        path: Absolute path to target file.
        start: Unique substring marking the beginning of the block.
        content: Replacement text.
        end: Optional unique substring marking the end of the block.
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

    start_match = find_text(text, start, exact=exact)
    if start_match.count == 0:
        raise EditMarksError('Start marker not found in file.')
    if start_match.count > 1:
        raise EditMarksError(
            f'Start marker is ambiguous – found {start_match.count} occurrences in file.'
        )

    if end is None:
        result_text = text[:start_match.start] + content + text[start_match.end:]
    else:
        end_match = find_text(text, end, exact=exact)
        if end_match.count == 0:
            raise EditMarksError('End marker not found in file.')
        if end_match.count > 1:
            raise EditMarksError(
                f'End marker is ambiguous – found {end_match.count} occurrences in file.'
            )

        if end_match.start < start_match.end:
            raise EditMarksError('End marker must start after start marker ends.')

        result_text = text[:start_match.start] + content + text[end_match.end:]

    try:
        file_path.write_text(result_text, encoding='utf-8')
    except OSError as exc:
        raise EditMarksError(f'Write failed: {exc}') from exc

    return EditMarksResult(result='success')


class EditMarksTool(ToolDefinition):
    name = 'edit_marks'
    title = 'Edit marked file block'
    description = "Replace everything strictly between and including the unique 'start' and 'end' markers with 'content'. If 'end' is omitted, only the 'start' marker itself is replaced – useful for replacing a single line, inserting content before/after it, or deleting it (empty 'content'). If given, 'end' must occur exactly once after 'start'. By default whitespace in markers is matched tolerantly; set 'exact' to require exact whitespace matching."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'start': {'type': 'string', 'description': "Unique substring marking the beginning of the block (must occur exactly once)."}, 'end': {'type': 'string', 'description': "Optional unique substring marking the end of the block (must occur exactly once, after 'start'). If omitted, only 'start' is replaced."}, 'content': {'type': 'string', 'description': "Replacement block"}, 'exact': {'type': 'boolean', 'description': "If true, 'start'/'end' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}}, 'required': ['path', 'start', 'content']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string', 'description': '``success`` on success.'}}, 'required': []}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = edit_marks(path=args['path'], start=args['start'], content=args['content'], end=args.get('end'), exact=args.get('exact', False))
        except EditMarksError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_edit_marks_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditMarksTool())
    functions.register(edit_marks)