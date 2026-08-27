"""Replace-chars tool – replaces a character range inside an existing file."""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ReplaceCharsError', 'ReplaceCharsResult', 'replace_chars', 'ReplaceCharsTool', 'register_replace_chars_tool']

class ReplaceCharsError(Exception):
    """Raised when a replace-chars operation cannot be performed."""

@dataclass(frozen=True)
class ReplaceCharsResult:
    result: str

def replace_chars(path: str, offset: int, length: int, content: str) -> ReplaceCharsResult:
    """Replace ``length`` characters starting at ``offset`` in the file at ``path`` with ``content``.
    
    Args:
        path: Absolute path to file (must be a regular file).
        offset: Zero-based character offset where to start replacement (must be >= 0).
        length: Number of characters to replace (must be >= 0).
        content: Replacement text.
    
    Returns:
        ReplaceCharsResult with success status.
    
    Raises:
        ReplaceCharsError: If path is not absolute, not found, or not a regular file.
        ReplaceCharsError: If offset or length are out of bounds.
        ReplaceCharsError: If write operation fails.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise ReplaceCharsError('Path must be absolute.')
    if not file_path.exists():
        raise ReplaceCharsError('File not found.')
    if not file_path.is_file():
        raise ReplaceCharsError('Not a regular file.')
    try:
        text = file_path.read_text(encoding='utf-8')
        if offset < 0 or offset > len(text):
            raise ReplaceCharsError('Offset is out of bounds.')
        if length < 0 or offset + length > len(text):
            raise ReplaceCharsError('Length is out of bounds.')
        new_text = text[:offset] + content + text[offset + length:]
        file_path.write_text(new_text, encoding='utf-8')
    except OSError as exc:
        raise ReplaceCharsError(f'Replace failed: {exc}') from exc
    return ReplaceCharsResult(result='success')

class ReplaceCharsTool(ToolDefinition):
    name = 'replace-chars'
    title = 'Replace characters in file'
    description = 'Replace a range of characters inside an existing file with new content. The range is defined by a zero-based character ``offset`` and a ``length`` (number of characters to remove starting at the offset). The supplied ``content`` is written in place of the removed range. To replace whole lines instead, use ``replace-lines``.'
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the file to modify.'}, 'offset': {'type': 'integer', 'description': 'Zero-based character offset of the first character to replace.', 'minimum': 0}, 'length': {'type': 'integer', 'description': 'Number of characters to remove starting at ``offset``.', 'minimum': 0}, 'content': {'type': 'string', 'description': 'Replacement text (may be empty to perform a pure deletion).'}}, 'required': ['path', 'offset', 'length', 'content']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string', 'description': '``success`` on success.'}}, 'required': ['result']}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`replace_chars`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = replace_chars(path=args['path'], offset=args['offset'], length=args['length'], content=args['content'])
        except ReplaceCharsError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_replace_chars_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReplaceCharsTool())
    functions.register(replace_chars)