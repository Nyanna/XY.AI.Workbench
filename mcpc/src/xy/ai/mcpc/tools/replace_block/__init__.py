"""Replace-block tool – replaces an exact block of text (old -> new) in a file."""
from __future__ import annotations
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content
from .._text_match import find as find_text
__all__ = ['ReplaceBlockError', 'ReplaceBlockResult', 'replace_block', 'ReplaceBlockTool', 'register_replace_block_tool']

class ReplaceBlockError(Exception):
    """Raised when a replace-block operation cannot be performed."""

@dataclass(frozen=True)
class ReplaceBlockResult:
    result: str

def replace_block(path: str, old_text: str, new_text: str, exact: bool=False) -> ReplaceBlockResult:
    """Replace the unique occurrence of ``old_text`` in the file at ``path`` with ``new_text``.
    
    Args:
        path: Absolute path to file (must be a regular file).
        old_text: Unique text to find and replace (must occur exactly once).
        new_text: Replacement text.
        exact: If False (default), whitespace in old_text is matched tolerantly.
               If True, whitespace must match exactly.
    
    Returns:
        ReplaceBlockResult with success status.
    
    Raises:
        ReplaceBlockError: If path is not absolute, not found, or not a regular file.
        ReplaceBlockError: If old_text not found or appears more than once in file.
        ReplaceBlockError: If write operation fails.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise ReplaceBlockError('Path must be absolute.')
    if not file_path.exists():
        raise ReplaceBlockError('File not found.')
    if not file_path.is_file():
        raise ReplaceBlockError('Not a regular file.')
    text = file_path.read_text(encoding='utf-8')
    match = find_text(text, old_text, exact=exact)
    if match.count == 0:
        raise ReplaceBlockError('Text not found in file.')
    if match.count > 1:
        raise ReplaceBlockError(f'Text is ambiguous – found {match.count} occurrences in file.')
    result_text = text[:match.start] + new_text + text[match.end:]
    try:
        file_path.write_text(result_text, encoding='utf-8')
    except OSError as exc:
        raise ReplaceBlockError(f'Write failed: {exc}') from exc
    return ReplaceBlockResult(result='success')

class ReplaceBlockTool(ToolDefinition):
    name = 'replace-block'
    title = 'Replace text block in file'
    description = "Replace a complete block of text inside an existing file. 'old_text' must occur exactly once. By default whitespace (spaces, tabs, newlines) is matched tolerantly; set 'exact' to require exact whitespace matching."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'old_text': {'type': 'string', 'description': 'Text to find and replace. Must occur exactly once.'}, 'new_text': {'type': 'string', 'description': "Text that replaces 'old_text'."}, 'exact': {'type': 'boolean', 'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}}, 'required': ['path', 'old_text', 'new_text']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': []}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`replace_block`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = replace_block(path=args['path'], old_text=args['old_text'], new_text=args['new_text'], exact=args.get('exact', False))
        except ReplaceBlockError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_replace_block_tool(registry: ToolRegistry) -> None:
    registry.register(ReplaceBlockTool())