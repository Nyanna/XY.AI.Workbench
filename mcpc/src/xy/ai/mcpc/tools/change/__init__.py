"""Change tool – replaces the block between start/end markers (both inclusive)."""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._text_match import find as find_text
__all__ = ['ChangeError', 'ChangeResult', 'change', 'ChangeTool', 'register_change_tool']

class ChangeError(Exception):
    """Raised when a change operation cannot be performed."""

@dataclass(frozen=True)
class ChangeResult:
    result: str

def change(path: str, start: str, end: str, content: str, exact: bool=False) -> ChangeResult:
    """Replace text between start/end markers with content.
    
    Args:
        path: Absolute path to target file (must be a regular file).
        start: Unique substring marking the block's start (must occur exactly once).
        end: Unique substring marking the block's end (must occur exactly once, after start).
        content: Replacement text. Repeat a marker inside content to keep it.
        exact: If False (default), whitespace in start/end is matched tolerantly
               (any whitespace run matches any other). If True, whitespace must match exactly.
    
    Returns:
        ChangeResult with success status.
    
    Raises:
        ChangeError: If path is not absolute, not found, or not a regular file.
        ChangeError: If start or end markers are not found or appear more than once.
        ChangeError: If end marker does not appear after start marker.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise ChangeError('Path must be absolute.')
    if not file_path.exists():
        raise ChangeError('File not found.')
    if not file_path.is_file():
        raise ChangeError('Not a regular file.')
    text = file_path.read_text(encoding='utf-8')
    start_match = find_text(text, start, exact=exact)
    if start_match.count == 0:
        raise ChangeError('Start marker not found in file.')
    if start_match.count > 1:
        raise ChangeError(f'Start marker is ambiguous – found {start_match.count} occurrences in file.')
    end_match = find_text(text, end, exact=exact)
    if end_match.count == 0:
        raise ChangeError('End marker not found in file.')
    if end_match.count > 1:
        raise ChangeError(f'End marker is ambiguous – found {end_match.count} occurrences in file.')
    if end_match.start <= start_match.start:
        raise ChangeError('End marker must appear after start marker.')
    result_text = text[:start_match.start] + content + text[end_match.end:]
    try:
        file_path.write_text(result_text, encoding='utf-8')
    except OSError as exc:
        raise ChangeError(f'Write failed: {exc}') from exc
    return ChangeResult(result='success')

class ChangeTool(ToolDefinition):
    name = 'change'
    title = 'Change file block'
    description = "Replace the text between 'start' and 'end' (both included) with 'content'. Each marker must occur exactly once in the file; 'end' must come after 'start'. Repeat a marker inside 'content' to keep it. By default whitespace in 'start'/'end' is matched tolerantly; set 'exact' to require exact whitespace matching."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'start': {'type': 'string', 'description': "Unique substring marking the block's start (must occur exactly once)."}, 'end': {'type': 'string', 'description': "Unique substring marking the block's end (must occur exactly once, after 'start')."}, 'content': {'type': 'string', 'description': "Text that replaces the block, including where 'start'/'end' were."}, 'exact': {'type': 'boolean', 'description': "If true, 'start'/'end' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}}, 'required': ['path', 'start', 'end', 'content']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string', 'description': '``success`` on success.'}}, 'required': []}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`change`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = change(path=args['path'], start=args['start'], end=args['end'], content=args['content'], exact=args.get('exact', False))
        except ChangeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_change_tool(registry: ToolRegistry) -> None:
    registry.register(ChangeTool())