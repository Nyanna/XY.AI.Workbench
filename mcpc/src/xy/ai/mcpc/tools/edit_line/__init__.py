"""Edit-line tool – replaces a single line with one or more lines in a file, for a batch of items."""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.edit_block import EditBlockItem, edit_block
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = [
    'EditLineError',
    'EditLineItem',
    'EditLineResult',
    'EditLineItemError',
    'EditLineBatchResult',
    'edit_line',
    'EditLineTool',
    'register_edit_line_tool']

class EditLineError(Exception):
    """Raised when an edit-line operation cannot be performed."""

@dataclass(frozen=True)
class EditLineItem:
    """One single-line edit to apply.

    Attributes:
        path: Absolute path to file (must be a regular file).
        old_line: The single line to find and replace (must occur exactly once,
                  unless replace_all). Must not contain a newline character.
        new_lines: Replacement content; either a single line or multiple lines
                   (joined with '\\n') to replace old_line with.
        exact: If False (default), whitespace in old_line is matched tolerantly.
               If True, whitespace must match exactly.
        replace_all: If True, replace every occurrence of old_line instead of
                     requiring a single unique match.
    """
    path: str
    old_line: str
    new_lines: str
    exact: bool = False
    replace_all: bool = False

@dataclass(frozen=True)
class EditLineResult:
    """Result of applying a single line edit, mirroring its input path for result association."""
    path: str
    result: str

@dataclass(frozen=True)
class EditLineItemError:
    """Error applying a single line edit, mirroring its input path for result association."""
    path: str
    error: str

@dataclass(frozen=True)
class EditLineBatchResult:
    """Result of :func:`edit_line`.

    Attributes:
        results: One :class:`EditLineResult` per successfully applied edit.
        errors: One :class:`EditLineItemError` per edit that failed.
    """
    results: list[EditLineResult]
    errors: list[EditLineItemError]

def _edit_line_one(item: EditLineItem) -> EditLineResult:
    if '\n' in item.old_line or '\r' in item.old_line:
        raise EditLineError('old_line must be a single line without newline characters.')
    block_item = EditBlockItem(
        path=item.path,
        old_text=item.old_line,
        new_text=item.new_lines,
        exact=item.exact,
        replace_all=item.replace_all)
    batch = edit_block([block_item])
    if batch.errors:
        raise EditLineError(batch.errors[0].error)
    return EditLineResult(path=item.path, result=batch.results[0].result)

def edit_line(items: list[EditLineItem]) -> EditLineBatchResult:
    """Replace a single line with one or more lines, in one or more files.

    Delegates to :func:`edit_block`; ``old_line`` must be a single line (no newline
    characters). ``new_lines`` may be a single line or multiple lines joined by ``\\n``.

    Args:
        items: Single-line edits to apply. Must be non-empty.

    Returns:
        EditLineBatchResult: one result per successfully applied edit, one error per failed edit.

    Raises:
        EditLineError: If items is empty.
    """
    if not items:
        raise EditLineError("'items' must be a non-empty list.")
    results: list[EditLineResult] = []
    errors: list[EditLineItemError] = []
    for item in items:
        try:
            results.append(_edit_line_one(item))
        except EditLineError as exc:
            errors.append(EditLineItemError(path=item.path, error=str(exc)))
    return EditLineBatchResult(results=results, errors=errors)

class EditLineTool(ToolDefinition):
    name = 'edit_line'
    title = 'Replace a single line in file'
    description = "Replace exactly one line inside one or more files with one or more lines, for a batch of items. 'old_line' must be a single line without a newline character. Choose 'old_line' to be unique within the file. 'old_line' must occur exactly once, unless 'replaceAll' is set. By default whitespace is matched tolerantly; set 'exact' to require exact whitespace matching."
    input_schema = {
        'type': 'object',
        'properties': {
            'items': {
                'type': 'array',
                'minItems': 1,
                'items': {
                    'type': 'object',
                    'additionalProperties': False,
                    'properties': {
                        'path': {
                            'type': 'string',
                            'description': 'Absolute path to the target file.'},
                        'old_line': {
                            'type': 'string',
                            'description': 'A single line to find and replace without a newline character. Keep it short and distinct enough to match uniquely. Must occur exactly once unless replaceAll is set. Never pass multiple lines here.'},
                        'new_lines': {
                            'type': 'string',
                                    'description': "Replacement content for 'old_line', may be empty to delete the line."},
                        'exact': {
                            'type': 'boolean',
                            'description': "If true, 'old_line' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
                            'default': False},
                        'replaceAll': {
                            'type': 'boolean',
                            'description': "If true, replace every occurrence of 'old_line' instead of requiring a single unique match. Defaults to false.",
                            'default': False}},
                    'required': [
                        'path',
                        'old_line',
                        'new_lines']},
                'description': 'Single-line edits to apply.'}},
        'required': ['items']}
    output_schema = {
        'type': 'object', 'properties': {
            'results': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'path': {
                            'type': 'string'}, 'result': {
                                'type': 'string'}}, 'required': [
                                    'path', 'result']}}, 'errors': {
                                        'type': 'array', 'items': {
                                            'type': 'object', 'properties': {
                                                'path': {
                                                    'type': 'string'}, 'error': {
                                                        'type': 'string'}}, 'required': [
                                                            'path', 'error']}}}, 'required': [
                                                                'results', 'errors']}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_line`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        raw_items = args.get('items') or []
        if not raw_items:
            return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
        items = [
            EditLineItem(
                path=it['path'],
                old_line=it['old_line'],
                new_lines=it['new_lines'],
                exact=it.get(
                    'exact',
                    False),
                replace_all=it.get(
                    'replaceAll',
                    False)) for it in raw_items]
        batch = edit_line(items)
        results = [{'path': r.path, 'result': r.result} for r in batch.results]
        errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
        is_error = bool(batch.errors) and (not batch.results)
        return ToolResult(
            structured_content={
                'results': results,
                'errors': errors},
            is_error=is_error,
            auto_approve=not is_error)

def register_edit_line_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditLineTool())
    functions.register(edit_line)