"""Insert tool – inserts text at a character offset inside an existing file."""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
__all__ = ['InsertError', 'InsertResult', 'insert', 'InsertTool', 'register_insert_tool']

class InsertError(Exception):
    """Raised when an insert operation cannot be performed."""

@dataclass(frozen=True)
class InsertResult:
    result: str

def insert(path: str, offset: int, content: str) -> InsertResult:
    """Insert ``content`` at the zero-based character ``offset`` of the file at ``path``.
    
    Args:
        path: Absolute path to file to modify (must be a regular file).
        offset: Zero-based character offset where to insert (must be >= 0 and <= file length).
        content: Text to insert at the given offset.
    
    Returns:
        InsertResult with success status.
    
    Raises:
        InsertError: If path is not absolute.
        InsertError: If file not found or not a regular file.
        InsertError: If offset is beyond end of file.
        InsertError: If write operation fails.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise InsertError('Path must be absolute.')
    if not file_path.exists():
        raise InsertError('File not found.')
    if not file_path.is_file():
        raise InsertError('Not a regular file.')
    try:
        text = file_path.read_text(encoding='utf-8')
        if offset > len(text):
            raise InsertError('Offset is beyond end of file.')
        new_text = text[:offset] + content + text[offset:]
        file_path.write_text(new_text, encoding='utf-8')
    except OSError as exc:
        raise InsertError(f'Insert failed: {exc}') from exc
    return InsertResult(result='success')

class InsertTool(ToolDefinition):
    name = 'insert'
    title = 'Insert into file'
    description = 'Insert text at a specific character offset inside an existing file. The offset is zero-based and refers to the UTF-8 decoded content of the file. All existing content at and after the offset is shifted right.'
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the file to modify.'}, 'offset': {'type': 'integer', 'description': 'Zero-based character offset at which to insert the new content.', 'minimum': 0}, 'content': {'type': 'string', 'description': 'Text to insert at the given offset.'}}, 'required': ['path', 'offset', 'content']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string', 'description': '``success`` on success.'}}, 'required': ['result']}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`insert`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = insert(path=args['path'], offset=args['offset'], content=args['content'])
        except InsertError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_insert_tool(registry: ToolRegistry) -> None:
    registry.register(InsertTool())