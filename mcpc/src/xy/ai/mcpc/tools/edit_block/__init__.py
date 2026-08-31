"""Edit-block tool – Edits an exact block of text (old -> new) in a file."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._text_match import find as find_text, find_all as find_all_text
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['EditBlockError', 'EditBlockResult', 'edit_block', 'EditBlockTool', 'register_edit_block_tool']

class EditBlockError(Exception):
    """Raised when a edit-block operation cannot be performed."""

@dataclass(frozen=True)
class EditBlockResult:
    result: str

def edit_block(path: str, old_text: str, new_text: str, exact: bool=False, replace_all: bool=False) -> EditBlockResult:
    """Replace occurrence(s) of ``old_text`` in the file at ``path`` with ``new_text``.

    Args:
        path: Absolute path to file (must be a regular file).
        old_text: Unique text to find and replace (must occur exactly once, unless replace_all).
        new_text: replacement text.
        exact: If False (default), whitespace in old_text is matched tolerantly.
               If True, whitespace must match exactly.
        replace_all: If True, replace every occurrence of old_text instead of requiring
                     a single unique match.

    Returns:
        EditBlockResult with success status.

    Raises:
        EditBlockError: If path is not absolute, not found, or not a regular file.
        EditBlockError: If old_text not found, or appears more than once in file (when
                        replace_all is False).
        EditBlockError: If write operation fails.
    """
    file_path = Path(path)
    if not file_path.is_absolute():
        raise EditBlockError('Path must be absolute.')
    if not file_path.exists():
        raise EditBlockError('File not found.')
    if not file_path.is_file():
        raise EditBlockError('Not a regular file.')
    text = file_path.read_text(encoding='utf-8')
    if replace_all:
        matches = find_all_text(text, old_text, exact=exact)
        if not matches:
            raise EditBlockError('Text not found in file.')
        result_text = text
        for match in sorted(matches, key=lambda m: m.start, reverse=True):
            result_text = result_text[:match.start] + new_text + result_text[match.end:]
    else:
        match = find_text(text, old_text, exact=exact)
        if match.count == 0:
            raise EditBlockError('Text not found in file.')
        if match.count > 1:
            raise EditBlockError(f'Text is ambiguous – found {match.count} occurrences in file.')
        result_text = text[:match.start] + new_text + text[match.end:]
    try:
        file_path.write_text(result_text, encoding='utf-8')
    except OSError as exc:
        raise EditBlockError(f'Write failed: {exc}') from exc
    return EditBlockResult(result='success')

class EditBlockTool(ToolDefinition):
    name = 'edit_block'
    title = 'Edit text block in file'
    description = "Replace a complete block of text inside an existing file. 'old_text' must occur exactly once, unless 'replaceAll' is set. By default whitespace (spaces, tabs, newlines) is matched tolerantly; set 'exact' to require exact whitespace matching."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'old_text': {'type': 'string', 'description': 'Text to find and replace. Must occur exactly once, unless replaceAll is set.'}, 'new_text': {'type': 'string', 'description': "Text that replace 'old_text (may be empty to perform a pure deletion)'."}, 'exact': {'type': 'boolean', 'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}, 'replaceAll': {'type': 'boolean', 'description': "If true, replace every occurrence of 'old_text' instead of requiring a single unique match. Defaults to false.", 'default': False}}, 'required': ['path', 'old_text', 'new_text']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': []}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_block`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = edit_block(path=args['path'], old_text=args['old_text'], new_text=args['new_text'], exact=args.get('exact', False), replace_all=args.get('replaceAll', False))
        except EditBlockError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_edit_block_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditBlockTool())
    functions.register(edit_block)