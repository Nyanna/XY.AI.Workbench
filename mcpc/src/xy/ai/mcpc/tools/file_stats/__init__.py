"""File stats tool – returns file metrics for access and processing planning, for a batch of items.

Provides compact metrics including complexity, timestamps, size, line/word
counts, line length statistics, and average words per line.
"""
import hashlib
import re
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = [
    'FileStatsError',
    'FileStatsItem',
    'FileStatsResult',
    'FileStatsItemError',
    'FileStatsBatchResult',
    'TextStatsResult',
    'compute_text_stats',
    'compute_file_stats',
    'file_stats',
    'FileStatsTool',
    'register_file_stats_tool']

class FileStatsError(Exception):
    """Raised when file metrics cannot be computed."""

@dataclass(frozen=True)
class FileStatsItem:
    """One file to compute metrics for.

    Attributes:
        path: Absolute path to file (must exist and be a regular file).
    """
    path: str

@dataclass(frozen=True)
class FileStatsResult:
    path: str
    size_bytes: int
    lines: int
    words: int
    complexity: float
    created: str
    modified: str
    accessed: str
    line_length_max: int
    line_length_min: int
    line_length_avg: float
    words_per_line_avg: float
    checksum: str

@dataclass(frozen=True)
class FileStatsItemError:
    """Error computing metrics for a single file, mirroring its input path for result association."""
    path: str
    error: str

@dataclass(frozen=True)
class FileStatsBatchResult:
    """Result of :func:`file_stats`.

    Attributes:
        results: One :class:`FileStatsResult` per file whose metrics could be computed.
        errors: One :class:`FileStatsItemError` per file that failed.
    """
    results: list[FileStatsResult]
    errors: list[FileStatsItemError]

def _calculate_complexity(text: str) -> float:
    """Calculate data structure complexity (0.0 to 1.0).

    Based on character set diversity and pattern variation.
    """
    if not text:
        return 0.0
    has_alpha = bool(re.search('[a-zA-Z]', text))
    has_digit = bool(re.search('\\d', text))
    has_punct = bool(re.search('[^\\w\\s]', text))
    has_space = bool(re.search('\\s', text))
    has_upper = bool(re.search('[A-Z]', text))
    has_lower = bool(re.search('[a-z]', text))
    char_type_score = sum([has_alpha, has_digit, has_punct, has_space, has_upper, has_lower]) / 6.0
    unique_chars = len(set(text))
    entropy_score = min(1.0, unique_chars / 256.0)
    complexity = char_type_score * 0.4 + entropy_score * 0.6
    return round(complexity, 3)

@dataclass(frozen=True)
class TextStatsResult:
    size_bytes: int
    lines: int
    words: int
    complexity: float
    line_length_max: int
    line_length_min: int
    line_length_avg: float
    words_per_line_avg: float
    checksum: str

def compute_text_stats(text: str) -> TextStatsResult:
    """Compute size/line/word/complexity metrics for *text*.

    Extracted from :func:`compute_file_stats` so string content (e.g. fetched
    web pages) can be scored the same way without touching the filesystem.
    """
    lines = text.splitlines()
    size_bytes = len(text.encode('utf-8'))
    num_lines = len(lines)
    num_words = len(text.split())
    complexity = _calculate_complexity(text)
    checksum = hashlib.sha256(text.encode('utf-8')).hexdigest()
    line_lengths = [len(line) for line in lines] if lines else [0]
    line_length_max = max(line_lengths)
    line_length_min = min(line_lengths)
    line_length_avg = round(sum(line_lengths) / len(line_lengths), 2)
    words_per_line = [len(line.split()) for line in lines]
    words_per_line_avg = round(sum(words_per_line) / len(words_per_line), 2) if words_per_line else 0.0
    return TextStatsResult(
        size_bytes=size_bytes,
        lines=num_lines,
        words=num_words,
        complexity=complexity,
        line_length_max=line_length_max,
        line_length_min=line_length_min,
        line_length_avg=line_length_avg,
        words_per_line_avg=words_per_line_avg,
        checksum=checksum)

def compute_file_stats(path: Path) -> FileStatsResult:
    """Compute the file-metrics block for *path* (also reused by the outline tool).

    Assumes *path* is an existing regular file.
    """
    raw_bytes = path.read_bytes()
    text = raw_bytes.decode('utf-8', errors='replace')
    text_stats = asdict(compute_text_stats(text))
    text_stats['size_bytes'] = len(raw_bytes)
    stat = path.stat()
    created = datetime.fromtimestamp(stat.st_birthtime if hasattr(stat, 'st_birthtime')
                                     else stat.st_mtime, tz=timezone.utc).isoformat()
    modified = datetime.fromtimestamp(stat.st_mtime, tz=timezone.utc).isoformat()
    accessed = datetime.fromtimestamp(stat.st_atime, tz=timezone.utc).isoformat()
    return FileStatsResult(
        path=str(
            path.resolve()),
        created=created,
        modified=modified,
        accessed=accessed,
        **text_stats)

def _file_stats_one(item: FileStatsItem) -> FileStatsResult:
    file_path = Path(item.path)
    if not file_path.is_absolute():
        raise FileStatsError('Path must be absolute.')
    if not file_path.exists():
        raise FileStatsError('File not found.')
    if not file_path.is_file():
        raise FileStatsError('Not a regular file.')
    return compute_file_stats(file_path)

def file_stats(items: list[FileStatsItem]) -> FileStatsBatchResult:
    """Compute file metrics for each absolute path in ``items``.

    Args:
        items: Files to compute metrics for. Must be non-empty.

    Returns:
        FileStatsBatchResult: one result per file whose metrics could be computed, one error
        per failed file. Each result includes:
            size_bytes: File size in bytes.
            lines: Total number of lines.
            words: Total number of words (whitespace-split).
            complexity: Data structure complexity score (0.0 to 1.0, based on character diversity).
            created: File creation timestamp (ISO format, UTC).
            modified: Last modification timestamp (ISO format, UTC).
            accessed: Last access timestamp (ISO format, UTC).
            line_length_max: Longest line length in characters.
            line_length_min: Shortest line length in characters.
            line_length_avg: Average line length (rounded to 2 decimals).
            words_per_line_avg: Average words per line (rounded to 2 decimals).
            checksum: SHA256 checksum of file content.

    Raises:
        FileStatsError: If items is empty.

    Note:
        Binary files are decoded as UTF-8 with error replacement.
        Timestamps use fallback to mtime if birthtime not available (Linux).
    """
    if not items:
        raise FileStatsError("'items' must be a non-empty list.")
    results: list[FileStatsResult] = []
    errors: list[FileStatsItemError] = []
    for item in items:
        try:
            results.append(_file_stats_one(item))
        except FileStatsError as exc:
            errors.append(FileStatsItemError(path=item.path, error=str(exc)))
    return FileStatsBatchResult(results=results, errors=errors)

class FileStatsTool(ToolDefinition):
    name = 'file_stats'
    title = 'File stats'
    description = 'Get file metrics for access and processing planning, for a batch of items: complexity, timestamps, size, line/word counts, and line length statistics.'
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
                            'description': 'Absolute file path.'}},
                    'required': ['path']},
                'description': 'Files to compute metrics for.'}},
        'required': ['items']}
    output_schema = {
        'type': 'object',
        'properties': {
            'results': {
                'type': 'array',
                'items': {
                    'type': 'object',
                    'properties': {
                        'path': {
                            'type': 'string',
                            'description': 'Absolute file path.'},
                        'size_bytes': {
                            'type': 'integer',
                            'description': 'File size in bytes.'},
                        'lines': {
                            'type': 'integer',
                                    'description': 'Total number of lines.'},
                        'words': {
                            'type': 'integer',
                            'description': 'Total number of words.'},
                        'complexity': {
                            'type': 'number',
                            'description': 'Data structure complexity (0.0 to 1.0).'},
                        'created': {
                            'type': 'string',
                            'description': 'Creation timestamp (ISO 8601).'},
                        'modified': {
                            'type': 'string',
                            'description': 'Last modification timestamp (ISO 8601).'},
                        'accessed': {
                            'type': 'string',
                            'description': 'Last access timestamp (ISO 8601).'},
                        'line_length_max': {
                            'type': 'integer',
                            'description': 'Maximum line length in characters.'},
                        'line_length_min': {
                            'type': 'integer',
                            'description': 'Minimum line length in characters.'},
                        'line_length_avg': {
                            'type': 'number',
                            'description': 'Average line length in characters.'},
                        'words_per_line_avg': {
                            'type': 'number',
                            'description': 'Average number of words per line.'},
                        'checksum': {
                            'type': 'string',
                            'description': 'sha256 checksum of the file content.'}},
                    'required': [
                        'path',
                        'size_bytes',
                        'lines',
                        'words',
                        'complexity',
                        'created',
                        'modified',
                        'accessed',
                        'line_length_max',
                        'line_length_min',
                        'line_length_avg',
                        'words_per_line_avg',
                        'checksum']}},
            'errors': {
                'type': 'array',
                'items': {
                    'type': 'object',
                    'properties': {
                        'path': {
                            'type': 'string'},
                        'error': {
                            'type': 'string'}},
                    'required': [
                        'path',
                        'error']}}},
        'required': [
            'results',
            'errors']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`file_stats`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        raw_items = args.get('items') or []
        if not raw_items:
            return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
        items = [FileStatsItem(path=it['path']) for it in raw_items]
        batch = file_stats(items)
        results = [asdict(r) for r in batch.results]
        errors = [{'path': e.path, 'error': e.error} for e in batch.errors]
        is_error = bool(batch.errors) and (not batch.results)
        return ToolResult(
            content=[],
            structured_content={
                'results': results,
                'errors': errors},
            is_error=is_error,
            auto_approve=not is_error)

def register_file_stats_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(FileStatsTool())
    functions.register(file_stats)