"""Read tool – returns file contents, optionally sliced by line, character offset, or marker.

Range: start = min_line | min_char | start-marker | file start;
end = max_line | max_char | end-marker | file end (all inclusive).
Markers must be unique substrings. Line and char ranges are mutually exclusive.

Per-session cache (key ``_read_cache`` in ``Session.state``, keyed by the call
arguments plus the session id): the sha256 checksum of every read is recorded.
If a subsequent read with identical parameters yields the same checksum,
``content`` is omitted from ``structured_content`` and replaced by an
explanatory text content block; only the checksum is still returned.
``structured_content`` always carries the ``checksum``.
"""
import hashlib
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ReadError', 'ReadResult', 'read_file', 'ReadTool', 'register_read_tool']
_CACHE_STATE_KEY = '_read_cache'

class ReadError(Exception):
    """Raised when a file cannot be read or the requested range is invalid."""

@dataclass(frozen=True)
class ReadResult:
    content: str
    checksum: str
    is_full_file: bool

def _cache_key(session_id: str, arguments: dict[str, Any]) -> str:
    payload = json.dumps({'session': session_id, 'arguments': arguments}, sort_keys=True)
    return hashlib.sha256(payload.encode('utf-8')).hexdigest()

def read_file(path: str, min_line: int | None=None, max_line: int | None=None, min_char: int | None=None, max_char: int | None=None, start: str | None=None, end: str | None=None) -> ReadResult:
    """Read file contents, optionally sliced to a range.
    
    Args:
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
    
    Returns:
        ReadResult with:
            content: The file content or requested slice (UTF-8 decoded, errors replaced).
            checksum: SHA256 checksum of the content.
            is_full_file: True if entire file was read (no range specified).
    
    Raises:
        ReadError: If path is not absolute, not found, or not a regular file.
        ReadError: If conflicting range parameters provided (e.g., min_line AND min_char).
        ReadError: If start/end markers not found or appear more than once.
        ReadError: If end position resolves before start position.
    
    Note:
        Line numbering is 1-based (first line is 1). Character offsets are 0-based.
        Session-level change detection via cache: identical reads return checksum only.
    """
    if min_line is not None and min_char is not None:
        raise ReadError('``min_line`` and ``min_char`` are mutually exclusive.')
    if max_line is not None and max_char is not None:
        raise ReadError('``max_line`` and ``max_char`` are mutually exclusive.')
    if min_line is not None and start is not None:
        raise ReadError('``min_line`` and ``start`` are mutually exclusive.')
    if min_char is not None and start is not None:
        raise ReadError('``min_char`` and ``start`` are mutually exclusive.')
    if max_line is not None and end is not None:
        raise ReadError('``max_line`` and ``end`` are mutually exclusive.')
    if max_char is not None and end is not None:
        raise ReadError('``max_char`` and ``end`` are mutually exclusive.')
    file_path = Path(path)
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
    if start is not None:
        start_count = text.count(start)
        if start_count == 0:
            raise ReadError('Start marker not found in file.')
        if start_count > 1:
            raise ReadError(f'Start marker is ambiguous – found {start_count} occurrences in file.')
        region_start = text.index(start)
    elif min_line is not None:
        region_start = line_start_offset(min_line)
    elif min_char is not None:
        region_start = min_char
    else:
        region_start = 0
    if end is not None:
        end_count = text.count(end)
        if end_count == 0:
            raise ReadError('End marker not found in file.')
        if end_count > 1:
            raise ReadError(f'End marker is ambiguous – found {end_count} occurrences in file.')
        region_end = text.index(end) + len(end)
    elif max_line is not None:
        region_end = line_end_offset(max_line)
    elif max_char is not None:
        region_end = max_char
    else:
        region_end = len(text)
    if region_end < region_start:
        raise ReadError('Resolved end position must not lie before the resolved start position.')
    is_full_file = min_line is None and max_line is None and (
        min_char is None) and (
            max_char is None) and (
                start is None) and (
                    end is None)
    if not is_full_file and len(text) and (region_end - region_start > 0.7 * len(text)):
        raise ReadError('The requested range selects more than 70% of the file. Read the whole file instead (omit the range parameters) and rely on the checksum-based conditional read to detect unchanged content.')
    sliced = text[region_start:region_end]
    checksum = hashlib.sha256(sliced.encode('utf-8')).hexdigest()
    return ReadResult(content=sliced, checksum=checksum, is_full_file=is_full_file)

class ReadTool(ToolDefinition):
    name = 'read_file'
    title = 'Read file content'
    description = "Read a file as text, optionally sliced to a range."
    input_schema = {
        'type': 'object',
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
        'required': ['path']}
    output_schema = {
        'type': 'object',
        'properties': {
            'content': {
                'type': 'string'},
            'checksum': {
                'type': 'string',
                'description': 'sha256 checksum of the read content.'},
            'unchanged': {
                'type': 'boolean',
                        'description': 'True if the content is identical to a previous read with the same parameters'}},
        'required': ['checksum']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`read_file`, then apply session-level change detection and MCP packing."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = read_file(
                path=args['path'],
                min_line=args.get('min_line'),
                max_line=args.get('max_line'),
                min_char=args.get('min_char'),
                max_char=args.get('max_char'),
                start=args.get('start'),
                end=args.get('end'))
        except ReadError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        session = ctx.session
        key = _cache_key(session.id, args)
        with session.lock:
            cache: dict[str, str] = session.state.setdefault(_CACHE_STATE_KEY, {})
            previous_checksum = cache.get(key)
            cache[key] = result.checksum
        unchanged = previous_checksum == result.checksum
        structured: dict[str, Any] = {'checksum': result.checksum}
        if unchanged:
            structured['unchanged'] = True
        else:
            structured['content'] = result.content
        content: list[dict[str, Any]] = []
        if unchanged:
            content.append(text_content('Content unchanged since the last identical read. Use the former read result.'))
        return ToolResult(content=content, structured_content=structured, auto_approve=result.is_full_file)

def register_read_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ReadTool())
    functions.register(read_file)