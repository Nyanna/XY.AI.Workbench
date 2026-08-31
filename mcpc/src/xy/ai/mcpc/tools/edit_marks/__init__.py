"""Edit Marks tool – replaces the text strictly including two markers."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._text_match import find as find_text
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['EditMarksError', 'EditMarksResult', 'edit_marks', 'edit_marks_text', 'EditMarksTool', 'register_edit_mark_tool']

class EditMarksError(Exception):
    """Raised when a replace operation cannot be performed."""

@dataclass(frozen=True)
class EditMarksResult:
    result: str

def edit_marks_text(text: str, start: str, end: str, content: str, exact: bool = False) -> str:
    """Replace everything between and including 'start' and 'end' with content, in *text*.

    Both markers are included in the replacement.

    Args:
        text: Source text to edit.
        start: Unique substring marking the beginning of the block.
        end: Unique substring marking the end of the block.
        content: Replacement text.
        exact: If False (default), whitespace in start/end is matched tolerantly. If True, whitespace must match exactly.

    Returns:
        The edited text.

    Raises:
        EditMarksError: If start or end markers are not found or appear more than once.
        EditMarksError: If end marker does not start after start marker ends.
    """
    start_match = find_text(text, start, exact=exact)
    if start_match.count == 0:
        raise EditMarksError('Start marker not found in file.')
    if start_match.count > 1:
        raise EditMarksError(
            f'Start marker is ambiguous – found {start_match.count} occurrences in file.'
        )

    end_match = find_text(text, end, exact=exact)
    if end_match.count == 0:
        raise EditMarksError('End marker not found in file.')
    if end_match.count > 1:
        raise EditMarksError(
            f'End marker is ambiguous – found {end_match.count} occurrences in file.'
        )

    if end_match.start < start_match.end:
        raise EditMarksError('End marker must start after start marker ends.')

    return text[:start_match.start] + content + text[end_match.end:]


def edit_marks(path: str, start: str, end: str, content: str, exact: bool=False) -> EditMarksResult:
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
    result_text = edit_marks_text(text, start, end, content, exact=exact)

    try:
        file_path.write_text(result_text, encoding='utf-8')
    except OSError as exc:
        raise EditMarksError(f'Write failed: {exc}') from exc

    return EditMarksResult(result='success')


class EditMarksTool(ToolDefinition):
    name = 'edit_marks'
    title = 'Edit marked file block'
    description = "Replace everything strictly between and including the unique 'start' and 'end' markers (both markers included) with 'content'. Rules: (1) 'start' and 'end' must each appear exactly once in the file. (2) 'end' must begin after 'start' ends — they must not overlap and not duplicate. (3) Choose markers that are multicharacter and span a distinctive substring, ideally a full line or phrase, never a single word or whitespace only or big block. (4) The replaced region should be focused — a few lines at most, not the entire file. (5) Do not use this tool to replace a single line; the block must span at least a meaningful multi-line region. (6) By default whitespace in markers is matched tolerantly; set 'exact' to require exact whitespace matching. (7) Prefer distinct, short start/end lines over reusing a large block as both markers — 'start' should mark only the opening boundary, 'end' only the closing boundary."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'start': {'type': 'string', 'description': "Unique substring marking the beginning of the block. Must occur exactly once in the file. Must end before 'end' begins (no overlap, no duplicate). Choose a distinctive multi-character phrase, e.g. a full line of code or text."}, 'end': {'type': 'string', 'description': "Unique substring marking the end of the block. Must occur exactly once in the file, at a position after 'start' ends. Choose a distinctive multi-character phrase, e.g. a full line of code or text."}, 'content': {'type': 'string', 'description': "Replacement text that will replace everything from the start of 'start' to the end of 'end', inclusive."}, 'exact': {'type': 'boolean', 'description': "If true, 'start'/'end' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}}, 'required': ['path', 'start', 'end', 'content']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string', 'description': '``success`` on success.'}}, 'required': []}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_marks`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = edit_marks(path=args['path'], start=args['start'], end=args['end'], content=args['content'], exact=args.get('exact', False))
        except EditMarksError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_edit_marks_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditMarksTool())
    functions.register(edit_marks)