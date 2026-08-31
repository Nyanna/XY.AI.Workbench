"""Edit-lines tool – replaces a range of lines inside an existing file.
"""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['EditLinesError', 'EditLinesResult', 'edit_lines', 'EditLinesTool', 'register_edit_lines_tool']

class EditLinesError(Exception):
    """Raised when a edit-lines operation cannot be performed."""

@dataclass(frozen=True)
class EditLinesResult:
    result: str

def edit_lines(path: str, offset: int, amount: int, content: str) -> EditLinesResult:
    """Replace ``amount`` lines starting at line ``offset`` in the file at ``path`` with ``content``.
    
    Args:
        path: Absolute path to file (must be a regular file).
        offset: Zero-based line offset where to start replacement (must be >= 0).
        length: Number of lines to replace (must be >= 0).
        content: Replacement text (should include its own trailing newline if a line break is wanted).
    
    Returns:
        EditLinesResult with success status.
    
    Raises:
        EditLinesError: If path is not absolute, not found, or not a regular file.
        EditLinesError: If offset or length are out of bounds.
        EditLinesError: If write operation fails.
    
    Note:
        Lines are 0-based. content may be empty to perform pure deletion.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise EditLinesError('Path must be absolute.')
    if not file_path.exists():
        raise EditLinesError('File not found.')
    if not file_path.is_file():
        raise EditLinesError('Not a regular file.')
    try:
        text = file_path.read_text(encoding='utf-8')
        lines = text.splitlines(keepends=True)
        if offset < 0 or offset > len(lines):
            raise EditLinesError('Offset is out of bounds.')
        if amount < 0 or offset + amount > len(lines):
            raise EditLinesError('Length is out of bounds.')
        new_lines = lines[:offset] + [content] + lines[offset + amount:]
        new_text = ''.join(new_lines)
        file_path.write_text(new_text, encoding='utf-8')
    except OSError as exc:
        raise EditLinesError(f'Replace failed: {exc}') from exc
    return EditLinesResult(result='success')

class EditLinesTool(ToolDefinition):
    name = 'edit_lines'
    title = 'Replace lines in file'
    description = 'Replace a range of lines inside an existing file with new content. The range is defined by a zero-based line ``offset`` and a ``length`` (number of lines to remove starting at the offset). The supplied ``content`` is written in place of the removed lines; it should include its own trailing newline if a line break is wanted.'
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the file to modify.'}, 'offset': {'type': 'integer', 'description': 'Zero-based line offset of the first line to replace.', 'minimum': 0}, 'amount': {'type': 'integer', 'description': 'Number of lines to remove starting at ``offset``.', 'minimum': 0}, 'content': {'type': 'string', 'description': 'Replacement text (may be empty to perform a pure deletion).'}}, 'required': ['path', 'offset', 'length', 'content']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string', 'description': '``success`` on success.'}}, 'required': ['result']}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_lines`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = edit_lines(path=args['path'], offset=args['offset'], amount=args['amount'], content=args['content'])
        except EditLinesError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_edit_lines_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditLinesTool())
    functions.register(edit_lines)