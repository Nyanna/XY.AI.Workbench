"""Write tool – writes a file completely or appends lines to it."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['WriteError', 'WriteResult', 'write', 'WriteTool', 'register_write_tool']

class WriteError(Exception):
    """Raised when a write operation cannot be performed."""

@dataclass(frozen=True)
class WriteResult:
    result: str

def write(path: str, mode: str, content: str) -> WriteResult:
    """Write ``content`` to ``path``; ``mode`` is ``replace`` or ``append``.
    
    Args:
        path: Absolute path to file to write (created if not exists).
        mode: Write mode: "replace" overwrites entire file, "append" adds content at end.
        content: Text content to write.
    
    Returns:
        WriteResult with success status.
    
    Raises:
        WriteError: If path is not absolute.
        WriteError: If write operation fails (permission, disk full, etc.).
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise WriteError('Path must be absolute.')
    try:
        file_path.parent.mkdir(parents=True, exist_ok=True)
        file_mode = 'a' if mode == 'append' else 'w'
        with file_path.open(file_mode, encoding='utf-8') as fh:
            fh.write(content)
    except OSError as exc:
        raise WriteError(f'Write failed: {exc}') from exc
    return WriteResult(result='success')

class WriteTool(ToolDefinition):
    name = 'write'
    title = 'Write file'
    description = 'Write content to a file. In ``replace`` mode the file is overwritten with the supplied content. In ``append`` mode the content is added at the end of the existing file (the file is created if it does not yet exist).'
    input_schema = {
        'type': 'object',
        'properties': {
            'path': {
                'type': 'string',
                'description': 'Absolute path to the file to write.'},
            'mode': {
                'type': 'string',
                'enum': [
                        'replace',
                        'append'],
                'description': '``replace`` – overwrite the file with the new content. ``append`` – add the new content after the existing content.'},
            'content': {
                'type': 'string',
                'description': 'Text to write to the file.'}},
        'required': [
            'path',
            'mode',
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
        """Delegate to :func:`write`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = write(path=args['path'], mode=args['mode'], content=args['content'])
        except WriteError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_write_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(WriteTool())
    functions.register(write)