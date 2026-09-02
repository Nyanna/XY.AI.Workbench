"""Edit-chars tool – replaces a character range inside an existing file."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['EditCharsError', 'EditCharsResult', 'edit_chars', 'EditCharsTool', 'register_edit_chars_tool']

class EditCharsError(Exception):
    """Raised when a edit-chars operation cannot be performed."""

@dataclass(frozen=True)
class EditCharsResult:
    result: str

def edit_chars(path: str, offset: int, length: int, content: str) -> EditCharsResult:
    """Replace ``length`` characters starting at ``offset`` in the file at ``path`` with ``content``.
    
    Args:
        path: Absolute path to file (must be a regular file).
        offset: Zero-based character offset where to start replacement (must be >= 0).
        length: Number of characters to replace (must be >= 0).
        content: Replacement text.
    
    Returns:
        EditCharsResult with success status.
    
    Raises:
        EditCharsError: If path is not absolute, not found, or not a regular file.
        EditCharsError: If offset or length are out of bounds.
        EditCharsError: If write operation fails.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise EditCharsError('Path must be absolute.')
    if not file_path.exists():
        raise EditCharsError('File not found.')
    if not file_path.is_file():
        raise EditCharsError('Not a regular file.')
    try:
        text = file_path.read_text(encoding='utf-8')
        if offset < 0 or offset > len(text):
            raise EditCharsError('Offset is out of bounds.')
        if length < 0 or offset + length > len(text):
            raise EditCharsError('Length is out of bounds.')
        new_text = text[:offset] + content + text[offset + length:]
        file_path.write_text(new_text, encoding='utf-8')
    except OSError as exc:
        raise EditCharsError(f'Replace failed: {exc}') from exc
    return EditCharsResult(result='success')

class EditCharsTool(ToolDefinition):
    name = 'edit_chars'
    title = 'Replace characters in file'
    description = 'Replace a range of characters inside an existing file with new content. The range is defined by a zero-based character ``offset`` and a ``length`` (number of characters to remove starting at the offset). The supplied ``content`` is written in place of the removed range.'
    input_schema = {
        'type': 'object',
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the file to modify.'},
            'offset': {
                'type': 'integer',
                'description': 'Zero-based character offset of the first character to replace.',
                'minimum': 0},
            'length': {
                'type': 'integer',
                        'description': 'Number of characters to remove starting at ``offset``.',
                        'minimum': 0},
            'content': {
                'type': 'string',
                'description': 'Replacement text (may be empty to perform a pure deletion).'}},
        'required': [
            'path',
            'offset',
            'length',
            'content']}
    output_schema = {
        'type': 'object',
        'properties': {
            'result': {
                'type': 'string',
                'description': '``success`` on success.'}},
        'required': ['result']}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_chars`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = edit_chars(
                path=args['path'],
                offset=args['offset'],
                length=args['length'],
                content=args['content'])
        except EditCharsError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_edit_chars_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditCharsTool())
    functions.register(edit_chars)