"""Move tool – moves files or directories, or sends them to the trash, for a batch of items."""
import os
import shutil
import urllib.parse
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
from xy.ai.mcpc.tools.results import BatchResult, Done, ItemError, ItemResult
__all__ = ['MoveError', 'MoveItem', 'MoveItemError', 'MoveBatchResult', 'move', 'MoveTool', 'register_move_tool']
TRASH_KEYWORD = 'trash'

class MoveError(Exception):
    """Raised when a move operation cannot be performed."""

@dataclass(frozen=True)
class MoveItem:
    """One file or directory to move.

    Attributes:
        source: Absolute path of the file or directory to move.
        target: Absolute destination path, or empty / ``"trash"`` to move to the trash.
    """
    source: str
    target: str = ''

@dataclass(frozen=True)
class MoveItemError(ItemError):
    """Error moving a single item, mirroring its source path for result association."""
    source: str

    def key(self) -> dict[str, Any]:
        return {'source': self.source}

class MoveBatchResult(BatchResult[ItemResult, MoveItemError]):
    """Result of :func:`move`; successes are not reported individually.

    Attributes:
        errors: One :class:`MoveItemError` per item that failed.
    """

def _trash_dir() -> Path:
    data_home = os.environ.get('XDG_DATA_HOME') or str(Path.home() / '.local' / 'share')
    return Path(data_home) / 'Trash'

def _unique_name(files_dir: Path, name: str) -> str:
    candidate = name
    counter = 1
    while os.path.lexists(files_dir / candidate) or os.path.lexists(files_dir.parent / 'info' / f'{candidate}.trashinfo'):
        counter += 1
        candidate = f'{name}.{counter}'
    return candidate

def _move_to_trash(source: Path) -> None:
    trash = _trash_dir()
    files_dir = trash / 'files'
    info_dir = trash / 'info'
    files_dir.mkdir(parents=True, exist_ok=True)
    info_dir.mkdir(parents=True, exist_ok=True)
    name = _unique_name(files_dir, source.name)
    info = info_dir / f'{name}.trashinfo'
    stamp = datetime.now().strftime('%Y-%m-%dT%H:%M:%S')
    info.write_text(f'[Trash Info]\nPath={urllib.parse.quote(str(source))}\nDeletionDate={stamp}\n', encoding='utf-8')
    try:
        shutil.move(str(source), str(files_dir / name))
    except BaseException:
        info.unlink(missing_ok=True)
        raise

def _move_one(item: MoveItem) -> None:
    source = Path(item.source)
    if not source.is_absolute():
        raise MoveError('Source must be an absolute path.')
    if not os.path.lexists(source):
        raise MoveError('Source does not exist.')
    target_text = item.target.strip()
    to_trash = not target_text or target_text.lower() == TRASH_KEYWORD
    try:
        if to_trash:
            _move_to_trash(source)
        else:
            target = Path(target_text)
            if not target.is_absolute():
                raise MoveError('Target must be an absolute path, empty or "trash".')
            if os.path.lexists(target):
                raise MoveError('Target already exists.')
            if source.is_dir() and (not source.is_symlink()) and target.is_relative_to(source):
                raise MoveError('Cannot move a directory into itself.')
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.move(str(source), str(target))
    except (OSError, shutil.Error) as exc:
        raise MoveError(f'Move failed: {exc}') from exc

def move(items: list[MoveItem]) -> MoveBatchResult:
    """Move or trash one or more files or directories, for a batch of items.

    Args:
        items: Items to move. Must be non-empty.

    Returns:
        MoveBatchResult: one error per failed item; no errors means all items were moved.

    Raises:
        MoveError: If items is empty.
    """
    if not items:
        raise MoveError("'items' must be a non-empty list.")
    errors: list[MoveItemError] = []
    for item in items:
        try:
            _move_one(item)
        except MoveError as exc:
            errors.append(MoveItemError(source=item.source, error=str(exc)))
    return MoveBatchResult(errors=errors)

class MoveTool(ToolDefinition):
    name = 'move'
    title = 'Move file or directory'
    description = 'Move or rename files and directories, for a batch of items. Parent directories of the target are created. With the keyword ``trash`` the item is moved to the trash .'
    _ITEM_PROPERTIES = {
        'source': {
            'type': 'string',
            'description': 'Absolute path of the file or directory to move.'},
        'target': {
            'type': 'string',
            'description': 'Absolute destination path. ``trash`` moves the source to the trash.'}}
    _ITEM_REQUIRED = ['source']
    _ITEMS_DESCRIPTION = 'Files or directories to move.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`move`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> MoveItem:
            return MoveItem(source=it['source'], target=it.get('target') or '')
        result = handle_batch_tool(ctx, item_factory, move, MoveError)
        if result.is_error or result.structured_content:
            return result
        return ToolResult.ok(Done(), structured=Done().to_dict())

def register_move_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(MoveTool())
    functions.register(move)