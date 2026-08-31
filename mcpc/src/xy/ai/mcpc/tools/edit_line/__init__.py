"""Edit-line tool – Replaces a single line with one or more lines in a file.
"""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.edit_block import EditBlockError, edit_block
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['EditLineError', 'EditLineResult', 'edit_line', 'EditLineTool', 'register_edit_line_tool']

class EditLineError(Exception):
    """Raised when an edit-line operation cannot be performed."""

@dataclass(frozen=True)
class EditLineResult:
    result: str

def edit_line(path: str, old_line: str, new_lines: str, exact: bool=False, replace_all: bool=False) -> EditLineResult:
    """Replace a single line ``old_line`` in the file at ``path`` with ``new_lines``.

    Delegates to :func:`edit_block`; ``old_line`` must be a single line (no newline
    characters). ``new_lines`` may be a single line or multiple lines joined by ``\\n``.

    Args:
        path: Absolute path to file (must be a regular file).
        old_line: The single line to find and replace (must occur exactly once,
                  unless replace_all). Must not contain a newline character.
        new_lines: Replacement content; either a single line or multiple lines
                   (joined with '\\n') to replace old_line with.
        exact: If False (default), whitespace in old_line is matched tolerantly.
               If True, whitespace must match exactly.
        replace_all: If True, replace every occurrence of old_line instead of
                     requiring a single unique match.

    Returns:
        EditLineResult with success status.

    Raises:
        EditLineError: If old_line contains a newline character.
        EditLineError: Wraps any EditBlockError raised while delegating.
    """
    if '\n' in old_line or '\r' in old_line:
        raise EditLineError('old_line must be a single line without newline characters.')
    try:
        result = edit_block(path=path, old_text=old_line, new_text=new_lines, exact=exact, replace_all=replace_all)
    except EditBlockError as exc:
        raise EditLineError(str(exc)) from exc
    return EditLineResult(result=result.result)

class EditLineTool(ToolDefinition):
    name = 'edit_line'
    title = 'Edit single line in file'
    description = "Replace exactly one line inside an existing file with one or more lines. IMPORTANT: 'old_line' MUST be a single line — it MUST NOT contain any newline character ('\\n'). Choose 'old_line' to be short and unique within the file. 'old_line' must occur exactly once, unless 'replaceAll' is set. By default whitespace is matched tolerantly; set 'exact' to require exact whitespace matching."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'old_line': {'type': 'string', 'description': 'A SINGLE line to find and replace. MUST NOT contain a newline character. Keep it short and distinct enough to match uniquely. Must occur exactly once unless replaceAll is set. Do NOT pass multiple lines here.'}, 'new_lines': {'type': 'string', 'description': "Replacement content for 'old_line'. Either a single line, or multiple lines joined with '\\n' (may be empty to delete the line)."}, 'exact': {'type': 'boolean', 'description': "If true, 'old_line' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}, 'replaceAll': {'type': 'boolean', 'description': "If true, replace every occurrence of 'old_line' instead of requiring a single unique match. Defaults to false.", 'default': False}}, 'required': ['path', 'old_line', 'new_lines']}
    output_schema = {'type': 'object', 'properties': {'result': {'type': 'string'}}, 'required': []}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_line`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = edit_line(path=args['path'], old_line=args['old_line'], new_lines=args['new_lines'], exact=args.get('exact', False), replace_all=args.get('replaceAll', False))
        except EditLineError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'result': result.result}, auto_approve=True)

def register_edit_line_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditLineTool())
    functions.register(edit_line)
