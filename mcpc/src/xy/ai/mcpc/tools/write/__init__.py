"""Write tool – writes files completely or appends content to them, for a batch of items."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool
__all__ = [
    'WriteError',
    'WriteItem',
    'WriteResult',
    'WriteItemError',
    'WriteBatchResult',
    'write',
    'WriteTool',
    'register_write_tool']

class WriteError(Exception):
    """Raised when a write operation cannot be performed."""

@dataclass(frozen=True)
class WriteItem:
    """One file to write.

    Attributes:
        path: Absolute path to file to write (created if not exists).
        mode: Write mode: "replace" overwrites entire file, "append" adds content at end.
        content: Text content to write.
    """
    path: str
    mode: str
    content: str

@dataclass(frozen=True)
class WriteResult:
    """Result of writing a single file, mirroring its input path for result association."""
    path: str
    result: str

@dataclass(frozen=True)
class WriteItemError:
    """Error writing a single file, mirroring its input path for result association."""
    path: str
    error: str

@dataclass(frozen=True)
class WriteBatchResult:
    """Result of :func:`write`.

    Attributes:
        results: One :class:`WriteResult` per successfully written file.
        errors: One :class:`WriteItemError` per file that failed.
    """
    results: list[WriteResult]
    errors: list[WriteItemError]

def _write_one(item: WriteItem) -> WriteResult:
    file_path = Path(item.path)
    if not file_path.is_absolute():
        raise WriteError('Path must be absolute.')
    try:
        file_path.parent.mkdir(parents=True, exist_ok=True)
        file_mode = 'a' if item.mode == 'append' else 'w'
        with file_path.open(file_mode, encoding='utf-8') as fh:
            fh.write(item.content)
    except OSError as exc:
        raise WriteError(f'Write failed: {exc}') from exc
    return WriteResult(path=item.path, result='success')

def write(items: list[WriteItem]) -> WriteBatchResult:
    """Write or replace one or more target files, for a batch of items.

    Args:
        items: Files to write. Must be non-empty.

    Returns:
        WriteBatchResult: one result per successfully written file, one error per failed file.

    Raises:
        WriteError: If items is empty.
    """
    if not items:
        raise WriteError("'items' must be a non-empty list.")
    results: list[WriteResult] = []
    errors: list[WriteItemError] = []
    for item in items:
        try:
            results.append(_write_one(item))
        except WriteError as exc:
            errors.append(WriteItemError(path=item.path, error=str(exc)))
    return WriteBatchResult(results=results, errors=errors)

class WriteTool(ToolDefinition):
    name = 'write'
    title = 'Write file'
    description = 'Write content to one or more files, for a batch of items. In ``replace`` mode a file is overwritten with the supplied content. In ``append`` mode the content is added at the end of the existing file (the file is created if it does not yet exist).'
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
                            'description': 'Absolute path to the file to write.'},
                        'mode': {
                            'type': 'string',
                            'enum': [
                                    'replace',
                                    'append'],
                            'description': '``replace`` – overwrite the file with the new content. ``append`` – add the new content after the existing content.'},
                        'content': {
                            'type': 'string',
                            'description': 'Text to write to the file.'}},
                    'required': [
                        'path',
                        'mode',
                        'content']},
                'description': 'Files to write or replace.'}},
        'required': ['items']}
    output_schema = {
        'type': 'object', 'properties': {
            'results': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'path': {
                            'type': 'string'}, 'result': {
                                'type': 'string', 'description': '``success`` on success.'}}, 'required': [
                                    'path', 'result']}}, 'errors': {
                                        'type': 'array', 'items': {
                                            'type': 'object', 'properties': {
                                                'path': {
                                                    'type': 'string'}, 'error': {
                                                        'type': 'string'}}, 'required': [
                                                            'path', 'error']}}}}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`write`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> WriteItem:
            return WriteItem(path=it['path'], mode=it['mode'], content=it['content'])
        return handle_batch_tool(ctx, item_factory, write, WriteError)

def register_write_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(WriteTool())
    functions.register(write)