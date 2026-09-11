"""Read tool – returns file contents, optionally sliced by line, character offset, or marker, for a batch of items."""
import hashlib
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = [
    'ReadError',
    'ReadItem',
    'ReadResult',
    'ReadItemError',
    'ReadBatchResult',
    'read_file',
    'ReadTool',
    'register_read_tool']
_CACHE_STATE_KEY = '_read_cache'

class ReadError(Exception):
    """Raised when a file cannot be read or the requested range is invalid."""

@dataclass(frozen=True)
class ReadItem:
    """One file (optionally sliced to a range) to read.

    Attributes:
        path: Absolute path to file (must exist and be readable).
        min_line: Range start as line number, inclusive, 1-based. Mutually exclusive with
                  min_char and start.
        max_line: Range end as line number, inclusive, 1-based. Mutually exclusive with
                  max_char and end.
        min_char: Range start as character offset, inclusive, 0-based. Mutually exclusive with
                  min_line and start.
        max_char: Range end as character offset, exclusive, 0-based. Mutually exclusive with
                  max_line and end.
        start: Range start as unique substring marker (inclusive). Mutually exclusive with
               min_line and min_char. Marker must occur exactly once in file.
        end: Range end as unique substring marker (inclusive). Mutually exclusive with
             max_line and max_char. Marker must occur exactly once in file.
    """
    path: str
    min_line: int | None = None
    max_line: int | None = None
    min_char: int | None = None
    max_char: int | None = None
    start: str | None = None
    end: str | None = None

@dataclass(frozen=True)
class ReadResult:
    """Result of reading a single file, mirroring its input path for result association."""
    path: str
    content: str
    checksum: str
    is_full_file: bool

@dataclass(frozen=True)
class ReadItemError:
    """Error reading a single file, mirroring its input path for result association."""
    path: str
    error: str

@dataclass(frozen=True)
class ReadBatchResult:
    """Result of :func:`read_file`.

    Attributes:
        results: One :class:`ReadResult` per successfully read file.
        errors: One :class:`ReadItemError` per file that failed.
    """
    results: list[ReadResult]
    errors: list[ReadItemError]

def _cache_key(session_id: str, item: ReadItem) -> str:
    payload = json.dumps({'session': session_id, 'item': item.__dict__}, sort_keys=True)
    return hashlib.sha256(payload.encode('utf-8')).hexdigest()

def _read_one(item: ReadItem) -> ReadResult:
    if item.min_line is not None and item.min_char is not None:
        raise ReadError('``min_line`` and ``min_char`` are mutually exclusive.')
    if item.max_line is not None and item.max_char is not None:
        raise ReadError('``max_line`` and ``max_char`` are mutually exclusive.')
    if item.min_line is not None and item.start is not None:
        raise ReadError('``min_line`` and ``start`` are mutually exclusive.')
    if item.min_char is not None and item.start is not None:
        raise ReadError('``min_char`` and ``start`` are mutually exclusive.')
    if item.max_line is not None and item.end is not None:
        raise ReadError('``max_line`` and ``end`` are mutually exclusive.')
    if item.max_char is not None and item.end is not None:
        raise ReadError('``max_char`` and ``end`` are mutually exclusive.')
    file_path = Path(item.path)
    if not file_path.is_absolute():
        raise ReadError('Path must be absolute.')
    if not file_path.exists():
        raise ReadError('File not found.')
    if not file_path.is_file():
        raise ReadError("Not a regular file. Don't read directories with this tool!")
    raw_bytes = file_path.read_bytes()
    text = raw_bytes.decode('utf-8', errors='replace')
    lines = text.splitlines(keepends=True)
    total_lines = len(lines)

    def line_start_offset(line_num: int) -> int:
        n = max(0, min(line_num - 1, total_lines))
        return sum((len(l) for l in lines[:n]))

    def line_end_offset(line_num: int) -> int:
        n = max(0, min(line_num, total_lines))
        return sum((len(l) for l in lines[:n]))
    if item.start is not None:
        start_count = text.count(item.start)
        if start_count == 0:
            raise ReadError('Start marker not found in file.')
        if start_count > 1:
            raise ReadError(f'Start marker is ambiguous – found {start_count} occurrences in file.')
        region_start = text.index(item.start)
    elif item.min_line is not None:
        region_start = line_start_offset(item.min_line)
    elif item.min_char is not None:
        region_start = item.min_char
    else:
        region_start = 0
    if item.end is not None:
        end_count = text.count(item.end)
        if end_count == 0:
            raise ReadError('End marker not found in file.')
        if end_count > 1:
            raise ReadError(f'End marker is ambiguous – found {end_count} occurrences in file.')
        region_end = text.index(item.end) + len(item.end)
    elif item.max_line is not None:
        region_end = line_end_offset(item.max_line)
    elif item.max_char is not None:
        region_end = item.max_char
    else:
        region_end = len(text)
    if region_end < region_start:
        raise ReadError('Resolved end position must not lie before the resolved start position.')
    is_full_file = item.min_line is None and item.max_line is None and (
        item.min_char is None) and (
            item.max_char is None) and (
                item.start is None) and (
                    item.end is None)
    if not is_full_file and len(text) and (region_end - region_start > 0.7 * len(text)):
        raise ReadError('The requested range selects more than 70% of the file. Read the whole file instead (omit the range parameters) and rely on the checksum-based conditional read to detect unchanged content.')
    sliced = text[region_start:region_end]
    checksum = hashlib.sha256(sliced.encode('utf-8')).hexdigest()
    return ReadResult(path=item.path, content=sliced, checksum=checksum, is_full_file=is_full_file)

def read_file(items: list[ReadItem]) -> ReadBatchResult:
    """Read one or more files, each optionally sliced to a range.

    Args:
        items: Files to read. Must be non-empty.

    Returns:
        ReadBatchResult: one result per successfully read file, one error per failed file.

    Raises:
        ReadError: If items is empty.

    Note:
        Line numbering is 1-based (first line is 1). Character offsets are 0-based.
    """
    if not items:
        raise ReadError("'items' must be a non-empty list.")
    results: list[ReadResult] = []
    errors: list[ReadItemError] = []
    for item in items:
        try:
            results.append(_read_one(item))
        except ReadError as exc:
            errors.append(ReadItemError(path=item.path, error=str(exc)))
    return ReadBatchResult(results=results, errors=errors)

class ReadTool(ToolDefinition):
    name = 'read_file'
    title = 'Read file content'
    description = 'Read one or more files as text, optionally sliced to a range, for a batch of items.'
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
                            'description': 'Absolute file path.'},
                        'min_line': {
                            'type': 'integer',
                            'description': 'Range start: line number, inclusive, 1-based. Excludes start and min_char.',
                            'minimum': 1},
                        'max_line': {
                            'type': 'integer',
                                    'description': 'Range end: line number, inclusive, 1-based. Excludes end and max_char.',
                                    'minimum': 1},
                        'min_char': {
                            'type': 'integer',
                            'description': 'Range start: character offset, inclusive, 0-based. Excludes min_line.',
                            'minimum': 0},
                        'max_char': {
                            'type': 'integer',
                            'description': 'Range end: character offset, exclusive, 0-based. Excludes max_line.',
                            'minimum': 0},
                        'start': {
                            'type': 'string',
                            'description': 'Range start: unique marker substring, inclusive. Excludes min_line and min_char.'},
                        'end': {
                            'type': 'string',
                            'description': 'Range end: unique marker substring, inclusive. Excludes max_line and max_char.'}},
                    'required': ['path']},
                'description': 'Files to read.'}},
        'required': ['items']}
    output_schema = {
        'type': 'object', 'properties': {
            'results': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'path': {
                            'type': 'string'}, 'content': {
                                'type': 'string'}, 'checksum': {
                                    'type': 'string', 'description': 'sha256 checksum of the read content.'}, 'unchanged': {
                                        'type': 'boolean', 'description': 'True if the content is identical to a previous read with the same parameters.'}}, 'required': [
                                            'path', 'checksum']}}, 'errors': {
                                                'type': 'array', 'items': {
                                                    'type': 'object', 'properties': {
                                                        'path': {
                                                            'type': 'string'}, 'error': {
                                                                'type': 'string'}}, 'required': [
                                                                    'path', 'error']}}}, 'required': [
                                                                        'results', 'errors']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`read_file`, then apply session-level change detection and MCP packing."""
        args: dict[str, Any] = ctx.arguments
        raw_items = args.get('items') or []
        if not raw_items:
            return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
        items = [
            ReadItem(
                path=it['path'],
                min_line=it.get('min_line'),
                max_line=it.get('max_line'),
                min_char=it.get('min_char'),
                max_char=it.get('max_char'),
                start=it.get('start'),
                end=it.get('end')) for it in raw_items]
        batch = read_file(items)
        session = ctx.session
        results: list[dict[str, Any]] = []
        content: list[dict[str, Any]] = []
        all_full_file = True
        with session.lock:
            cache: dict[str, str] = session.state.setdefault(_CACHE_STATE_KEY, {})
            for item, result in zip(items, batch.results):
                key = _cache_key(session.id, item)
                previous_checksum = cache.get(key)
                cache[key] = result.checksum
                unchanged = previous_checksum == result.checksum
                entry: dict[str, Any] = {'path': result.path, 'checksum': result.checksum}
                if unchanged:
                    entry['unchanged'] = True
                    content.append(text_content(
                        f'{result.path}: content unchanged since the last identical read. Use the former read result.'))
                else:
                    entry['content'] = result.content
                if not result.is_full_file:
                    all_full_file = False
                results.append(entry)
        errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
        is_error = bool(batch.errors) and (not batch.results)
        return ToolResult(
            content=content,
            structured_content={
                'results': results,
                'errors': errors},
            is_error=is_error,
            auto_approve=not is_error and all_full_file)

def register_read_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReadTool())
    functions.register(read_file)