"""Edit-block tool – edits an exact block of text (old -> new) in a file, for a batch of items."""
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
from xy.ai.mcpc.tools._text_match import replace_in_block, line_preserving, TextMatchError
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = [
    'EditBlockError',
    'EditBlockItem',
    'EditBlockResult',
    'EditBlockItemError',
    'EditBlockBatchResult',
    'edit_block',
    'EditBlockTool',
    'register_edit_block_tool']

class EditBlockError(Exception):
    """Raised when a edit-block operation cannot be performed."""

    def __init__(self, message: str, *, reason: str | None=None, position: str | None=None, corrected_text: str | None=None, guess: str | None=None, next_step: str | None=None) -> None:
        super().__init__(message)
        self.reason = reason
        self.position = position
        self.corrected_text = corrected_text
        self.guess = guess
        self.next_step = next_step

@dataclass(frozen=True)
class EditBlockItem:
    """One block edit to apply.

    Attributes:
        path: Absolute path to file (must be a regular file).
        old_text: Unique text to find and replace (must occur exactly once, unless replace_all).
        new_text: Replacement text (may be empty to perform a deletion).
        exact: If False (default), whitespace in old_text is matched tolerantly.
               If True, whitespace must match exactly.
        replace_all: If True, replace every occurrence of old_text instead of requiring
                     a single unique match.
    """
    path: str
    old_text: str
    new_text: str
    exact: bool = False
    replace_all: bool = False

@dataclass(frozen=True)
class EditBlockResult:
    """Result of applying a single block edit, mirroring its input path for result association."""
    path: str
    result: str

@dataclass(frozen=True)
class EditBlockItemError:
    """Error applying a single block edit, mirroring its input path for result association."""
    path: str
    error: str
    reason: str | None = None
    corrected_text: str | None = None
    guess: str | None = None
    next_step: str | None = None

@dataclass(frozen=True)
class EditBlockBatchResult:
    """Result of :func:`edit_block`.

    Attributes:
        results: One :class:`EditBlockResult` per successfully applied edit.
        errors: One :class:`EditBlockItemError` per edit that failed.
    """
    results: list[EditBlockResult]
    errors: list[EditBlockItemError]

def _edit_block_one(item: EditBlockItem) -> EditBlockResult:
    file_path = Path(item.path)
    if not file_path.is_absolute():
        raise EditBlockError('Path must be absolute.')
    if not file_path.exists():
        raise EditBlockError('File not found.')
    if not file_path.is_file():
        raise EditBlockError('Not a regular file.')
    text = file_path.read_text(encoding='utf-8')
    try:
        result_text = replace_in_block(
            text,
            item.old_text,
            item.new_text,
            exact=item.exact,
            replace_all=item.replace_all,
            accept=line_preserving(
                item.old_text),
            max_level=2,
            where='file')
    except TextMatchError as exc:
        raise EditBlockError(
            str(exc), reason=getattr(
                exc, 'reason', None), position=getattr(
                    exc, 'position', None), corrected_text=getattr(
                        exc, 'corrected_text', None), guess=getattr(
                            exc, 'guess', None), next_step=getattr(
                                exc, 'next_step', None)) from exc
    try:
        file_path.write_text(result_text, encoding='utf-8')
    except OSError as exc:
        raise EditBlockError(f'Write failed: {exc}') from exc
    return EditBlockResult(path=item.path, result='success')

def edit_block(items: list[EditBlockItem]) -> EditBlockBatchResult:
    """Replace occurrence(s) of ``old_text`` with ``new_text`` in one or more files.

    Matching escalates through whitespace/escape/quote tolerance; a candidate is
    only accepted when it preserves ``old_text``'s line structure, so no two lines
    are merged into a syntax error.

    Args:
        items: Block edits to apply. Must be non-empty.

    Returns:
        EditBlockBatchResult: one result per successfully applied edit, one error per failed edit.

    Raises:
        EditBlockError: If items is empty.
    """
    if not items:
        raise EditBlockError("'items' must be a non-empty list.")
    results: list[EditBlockResult] = []
    errors: list[EditBlockItemError] = []
    for item in items:
        try:
            results.append(_edit_block_one(item))
        except EditBlockError as exc:
            errors.append(
                EditBlockItemError(
                    path=item.path,
                    error=str(exc),
                    reason=exc.reason,
                    corrected_text=exc.corrected_text,
                    guess=exc.guess,
                    next_step=exc.next_step))
    return EditBlockBatchResult(results=results, errors=errors)

class EditBlockTool(ToolDefinition):
    name = 'edit_block'
    title = 'Replace text in file'
    description = "Replace a short text inside one or more files, for a batch of items. 'old_text' must occur exactly once, unless 'replaceAll' is set. By default whitespace (spaces, tabs, newlines) is matched tolerantly; set 'exact' to require exact whitespace matching."
    _ITEM_PROPERTIES = {
        'path': {
            'type': 'string',
            'description': 'Absolute path to the target file.'},
        'old_text': {
            'type': 'string',
            'minLength': 10,
            'maxLength': 100,
            'description': 'Text (10-100 chars) to find and replace. Must occur exactly once, unless replaceAll is set.'},
        'new_text': {
            'type': 'string',
                    'description': "Text that replaces 'old_text' (empty to perform a deletion)."},
        'exact': {
            'type': 'boolean',
            'description': "If true, 'old_text' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.",
            'default': False},
        'replaceAll': {
            'type': 'boolean',
            'description': "If true, replace every occurrence of 'old_text' instead of requiring a single unique match. Defaults to false.",
            'default': False}}
    _ITEM_REQUIRED = ['path', 'old_text', 'new_text']
    _ITEMS_DESCRIPTION = 'Block edits to apply.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`edit_block`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> EditBlockItem:
            return EditBlockItem(
                path=it['path'],
                old_text=it['old_text'],
                new_text=it['new_text'],
                exact=it.get(
                    'exact',
                    False),
                replace_all=it.get(
                    'replaceAll',
                    False))

        def error_serializer(e: EditBlockItemError) -> dict[str, Any]:
            return {k: v for k, v in asdict(e).items() if v is not None}
        return handle_batch_tool(
            ctx,
            item_factory,
            edit_block,
            EditBlockError,
            error_serializer=error_serializer,
            auto_approve=True)

def register_edit_block_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(EditBlockTool())
    functions.register(edit_block)