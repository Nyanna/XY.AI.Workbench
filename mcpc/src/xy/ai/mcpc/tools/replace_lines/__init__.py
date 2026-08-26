"""Replace-lines tool – replaces a range of lines inside an existing file.

This is the line-oriented analogue of ``replace-chars``: the range is given as a
zero-based *line* offset and a *line* count instead of character offsets.
"""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
__all__ = ['ReplaceLinesError', 'ReplaceLinesResult', 'replace_lines', 'ReplaceLinesTool', 'register_replace_lines_tool']

class ReplaceLinesError(Exception):
    """Raised when a replace-lines operation cannot be performed."""

@dataclass(frozen=True)
class ReplaceLinesResult:
    result: str

def replace_lines(path: str, offset: int, length: int, content: str) -> ReplaceLinesResult:
    """Replace ``length`` lines starting at line ``offset`` in the file at ``path`` with ``content``.
    
    Args:
        path: Absolute path to file (must be a regular file).
        offset: Zero-based line offset where to start replacement (must be >= 0).
        length: Number of lines to replace (must be >= 0).
        content: Replacement text (should include its own trailing newline if a line break is wanted).
    
    Returns:
        ReplaceLinesResult with success status.
    
    Raises:
        ReplaceLinesError: If path is not absolute, not found, or not a regular file.
        ReplaceLinesError: If offset or length are out of bounds.
        ReplaceLinesError: If write operation fails.
    
    Note:
        Lines are 0-based. content may be empty to perform pure deletion.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise ReplaceLinesError('Path must be absolute.')
    if not file_path.exists():
        raise ReplaceLinesError('File not found.')
    if not file_path.is_file():
        raise ReplaceLinesError('Not a regular file.')
    try:
        text = file_path.read_text(encoding='utf-8')
        lines = text.splitlines(keepends=True)
        if offset < 0 or offset > len(lines):
            raise ReplaceLinesError('Offset is out of bounds.')
        if length < 0 or offset + length > len(lines):
            raise ReplaceLinesError('Length is out of bounds.')
        new_lines = lines[:offset] + [content] + lines[offset + length:]
        new_text = ''.join(new_lines)
        file_path.write_text(new_text, encoding='utf-8')
    except OSError as exc:
        raise ReplaceLinesError(f'Replace failed: {exc}') from exc
    return ReplaceLinesResult(result='success')

class ReplaceLinesTool(ToolDefinition):
    name = 'replace-lines'
    title = 'Replace lines in file'
    description = 'Replace a range of lines inside an existing file with new content. The range is defined by a zero-based line ``offset`` and a ``length`` (number of lines to remove starting at the offset). The supplied ``content`` is written in place of the removed lines; it should include its own trailing newline if a line break is wanted. To replace an arbitrary character range instead, use ``replace-chars``.'
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the file to modify.'}, 'offset': {'type': 'integer', 'description': 'Zero-based line offset of the first line to replace.', 'minimum': 0}, 'length': {'type': 'integer', 'description': 'Number of lines to remove starting at ``offset``.', 'minimum': 0}, 'content': {'type': 'string', 'description': 'Replacement text (may be empty to perform a pure deletion).'}}, 'required': ['path', 'offset', 'length', 'content']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string', 'description': '``success`` on success.'}}, 'required': ['result']}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`replace_lines`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = replace_lines(path=args['path'], offset=args['offset'], length=args['length'], content=args['content'])
        except ReplaceLinesError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_replace_lines_tool(registry: ToolRegistry) -> None:
    registry.register(ReplaceLinesTool())