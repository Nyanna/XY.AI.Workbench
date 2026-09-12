"""Edit-lines tool – replaces a range of lines inside existing files, for a batch of items."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = [
    'EditLinesError',
    'EditLinesItem',
    'EditLinesResult',
    'EditLinesItemError',
    'EditLinesBatchResult',
    'edit_lines',
    'EditLinesTool',
    'register_edit_lines_tool']

class EditLinesError(Exception):
    """Raised when a edit-lines operation cannot be performed."""

@dataclass(frozen=True)
class EditLinesItem:
    """One line-range replacement to apply.

    Attributes:
        path: Absolute path to file (must be a regular file).
        offset: Zero-based line offset where to start replacement (must be >= 0).
        amount: Number of lines to replace (must be >= 0).
        content: Replacement text (should include its own trailing newline if a line break is wanted).
    """
    path: str
    offset: int
    amount: int
    content: str

@dataclass(frozen=True)
class EditLinesResult:
    """Result of applying a single line-range replacement, mirroring its input path for result association."""
    path: str
    result: str

@dataclass(frozen=True)
class EditLinesItemError:
    """Error applying a single line-range replacement, mirroring its input path for result association."""
    path: str
    error: str

@dataclass(frozen=True)
class EditLinesBatchResult:
    """Result of :func:`edit_lines`.

    Attributes:
        results: One :class:`EditLinesResult` per successfully applied edit.
        errors: One :class:`EditLinesItemError` per edit that failed.
    """
    results: list[EditLinesResult]
    errors: list[EditLinesItemError]

def _edit_lines_one(item: EditLinesItem) -> EditLinesResult:
    file_path = Path(item.path)
    if not file_path.is_absolute():
        raise EditLinesError('Path must be absolute.')
    if not file_path.exists():
        raise EditLinesError('File not found.')
    if not file_path.is_file():
        raise EditLinesError('Not a regular file.')
    try:
        text = file_path.read_text(encoding='utf-8')
        lines = text.splitlines(keepends=True)
        if item.offset < 0 or item.offset > len(lines):
            raise EditLinesError('Offset is out of bounds.')
        if item.amount < 0 or item.offset + item.amount > len(lines):
            raise EditLinesError('Amount is out of bounds.')
        new_lines = lines[:item.offset] + [item.content] + lines[item.offset + item.amount:]
        new_text = ''.join(new_lines)
        file_path.write_text(new_text, encoding='utf-8')
    except OSError as exc:
        raise EditLinesError(f'Replace failed: {exc}') from exc
    return EditLinesResult(path=item.path, result='success')

def edit_lines(items: list[EditLinesItem]) -> EditLinesBatchResult:
    """Replace a range of lines in one or more files, for a batch of items.

    A given path may only be edited once per batch: since offsets can shift after
    each edit, every further item targeting an already-processed path fails.

    Args:
        items: Line-range replacements to apply. Must be non-empty.

    Returns:
        EditLinesBatchResult: one result per successfully applied edit, one error per failed edit.

    Raises:
        EditLinesError: If items is empty.

    Note:
        Lines are 0-based. content may be empty to perform pure deletion.
    """
    if not items:
        raise EditLinesError("'items' must be a non-empty list.")
    results: list[EditLinesResult] = []
    errors: list[EditLinesItemError] = []
    seen: set[str] = set()
    for item in items:
        if item.path in seen:
            errors.append(
                EditLinesItemError(
                    path=item.path,
                    error='Duplicate path in this batch; edit_lines allows only one edit per path per batch because offsets can shift after a prior edit.'))
            continue
        seen.add(item.path)
        try:
            results.append(_edit_lines_one(item))
        except EditLinesError as exc:
            errors.append(EditLinesItemError(path=item.path, error=str(exc)))
    return EditLinesBatchResult(results=results, errors=errors)

class EditLinesTool(ToolDefinition):
    name = 'edit_lines'
    title = 'Replace lines in file by line offsets'
    description = 'Replace a range of lines inside one or more existing files with new content, for a batch of items. The range is defined by a zero-based line ``offset`` and an ``amount`` (number of lines to remove starting at the offset). The supplied ``content`` is written in place of the removed lines; it should include its own trailing newline if a line break is wanted. A given path may only be edited once per batch, since offsets can shift after a prior edit.'
    _ITEM_PROPERTIES = {
        'path': {
            'type': 'string',
            'description': 'Absolute path to the file to modify.'},
        'offset': {
            'type': 'integer',
            'description': 'Zero-based line offset of the first line to replace.',
            'minimum': 0},
        'amount': {
            'type': 'integer',
                    'description': 'Number of lines to remove starting at ``offset``.',
                    'minimum': 0},
        'content': {
            'type': 'string',
            'description': 'Replacement text (may be empty to perform a pure deletion).'}}
    _ITEM_REQUIRED = ['path', 'offset', 'amount', 'content']
    _ITEMS_DESCRIPTION = 'Line-range replacements to apply. A given path may only appear once.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_lines`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> EditLinesItem:
            return EditLinesItem(path=it['path'], offset=it['offset'], amount=it['amount'], content=it['content'])
        return handle_batch_tool(ctx, item_factory, edit_lines, EditLinesError, auto_approve=True)

def register_edit_lines_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditLinesTool())
    functions.register(edit_lines)