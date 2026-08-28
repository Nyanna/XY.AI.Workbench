"""Replace tool – replaces the text matched by 'start', optionally extended through 'end' (both inclusive), with given content."""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._text_match import find as find_text
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ReplaceError', 'ReplaceResult', 'replace', 'ReplaceTool', 'register_replace_tool']

class ReplaceError(Exception):
    """Raised when a replace operation cannot be performed."""

@dataclass(frozen=True)
class ReplaceResult:
    result: str

def replace(path: str, start: str, content: str, end: str | None=None, exact: bool=False) -> ReplaceResult:
    """Replace the text matched by 'start', optionally extended through 'end', with content.
    
    Args:
        path: Absolute path to target file (must be a regular file).
        start: Unique substring marking the block's start (must occur exactly once).
               If 'end' is omitted, this substring alone is what gets replaced.
        content: Replacement text. Repeat a marker inside content to keep it.
        end: Unique substring marking the block's end (must occur exactly once, after start).
             If omitted, only the 'start' match itself is replaced.
        exact: If False (default), whitespace in start/end is matched tolerantly
               (any whitespace run matches any other). If True, whitespace must match exactly.
    
    Returns:
        ReplaceResult with success status.
    
    Raises:
        ReplaceError: If path is not absolute, not found, or not a regular file.
        ReplaceError: If start or end markers are not found or appear more than once.
        ReplaceError: If end marker does not appear after start marker.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise ReplaceError('Path must be absolute.')
    if not file_path.exists():
        raise ReplaceError('File not found.')
    if not file_path.is_file():
        raise ReplaceError('Not a regular file.')
    text = file_path.read_text(encoding='utf-8')
    start_match = find_text(text, start, exact=exact)
    if start_match.count == 0:
        raise ReplaceError('Start marker not found in file.')
    if start_match.count > 1:
        raise ReplaceError(f'Start marker is ambiguous – found {start_match.count} occurrences in file.')
    if end is None:
        end_match = start_match
    else:
        end_match = find_text(text, end, exact=exact)
        if end_match.count == 0:
            raise ReplaceError('End marker not found in file.')
        if end_match.count > 1:
            raise ReplaceError(f'End marker is ambiguous – found {end_match.count} occurrences in file.')
        if end_match.start < start_match.start:
            raise ReplaceError('End marker must appear after start marker.')
    result_text = text[:start_match.start] + content + text[end_match.end:]
    try:
        file_path.write_text(result_text, encoding='utf-8')
    except OSError as exc:
        raise ReplaceError(f'Write failed: {exc}') from exc
    return ReplaceResult(result='success')

class ReplaceTool(ToolDefinition):
    name = 'replace'
    title = 'Replace file block'
    description = "Replace the text matched by 'start' with 'content'. If 'end' is also given, everything from 'start' through 'end' (both included) is replaced instead. Each marker must occur exactly once in the file; if given, 'end' must come after 'start'. Repeat a marker inside 'content' to keep it. By default whitespace in 'start'/'end' is matched tolerantly; set 'exact' to require exact whitespace matching."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'start': {'type': 'string', 'description': "Unique substring marking the block's start (must occur exactly once). If 'end' is omitted, this substring alone is what gets replaced."}, 'end': {'type': 'string', 'description': "Optional unique substring marking the block's end (must occur exactly once, after 'start'). If omitted, only the 'start' match is replaced."}, 'content': {'type': 'string', 'description': "Text that replaces the matched block, including where 'start'/'end' were."}, 'exact': {'type': 'boolean', 'description': "If true, 'start'/'end' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}}, 'required': ['path', 'start', 'content']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string', 'description': '``success`` on success.'}}, 'required': []}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`replace`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = replace(path=args['path'], start=args['start'], end=args.get('end'), content=args['content'], exact=args.get('exact', False))
        except ReplaceError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_replace_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReplaceTool())
    functions.register(replace)