"""File stats tool – returns file metrics for access and processing planning.

Provides compact metrics including complexity, timestamps, size, line/word
counts, line length statistics, and average words per line.
"""

from __future__ import annotations

import hashlib
import re
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

__all__ = [
    "FileStatsError",
    "FileStatsResult",
    "compute_file_stats",
    "file_stats",
    "FileStatsTool",
    "register_file_stats_tool",
]


class FileStatsError(Exception):
    """Raised when file metrics cannot be computed."""


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


def _calculate_complexity(text: str) -> float:
    """Calculate data structure complexity (0.0 to 1.0).

    Based on character set diversity and pattern variation.
    """
    if not text:
        return 0.0

    has_alpha = bool(re.search(r'[a-zA-Z]', text))
    has_digit = bool(re.search(r'\d', text))
    has_punct = bool(re.search(r'[^\w\s]', text))
    has_space = bool(re.search(r'\s', text))
    has_upper = bool(re.search(r'[A-Z]', text))
    has_lower = bool(re.search(r'[a-z]', text))

    char_type_score = sum([has_alpha, has_digit, has_punct, has_space, has_upper, has_lower]) / 6.0

    unique_chars = len(set(text))
    entropy_score = min(1.0, unique_chars / 256.0)

    complexity = (char_type_score * 0.4) + (entropy_score * 0.6)
    return round(complexity, 3)


def compute_file_stats(path: Path) -> dict[str, Any]:
    """Compute the file-metrics block for *path* (also reused by the outline tool).

    Assumes *path* is an existing regular file.
    """
    raw_bytes = path.read_bytes()
    text = raw_bytes.decode("utf-8", errors="replace")
    lines = text.splitlines()

    size_bytes = len(raw_bytes)
    num_lines = len(lines)
    num_words = len(text.split())
    complexity = _calculate_complexity(text)
    checksum = hashlib.sha256(text.encode("utf-8")).hexdigest()

    line_lengths = [len(line) for line in lines] if lines else [0]
    line_length_max = max(line_lengths) if line_lengths else 0
    line_length_min = min(line_lengths) if line_lengths else 0
    line_length_avg = round(sum(line_lengths) / len(line_lengths), 2) if line_lengths else 0.0

    words_per_line = [len(line.split()) for line in lines]
    words_per_line_avg = (
        round(sum(words_per_line) / len(words_per_line), 2) if words_per_line else 0.0
    )

    stat = path.stat()
    created = datetime.fromtimestamp(
        stat.st_birthtime if hasattr(stat, "st_birthtime") else stat.st_mtime,
        tz=timezone.utc,
    ).isoformat()
    modified = datetime.fromtimestamp(stat.st_mtime, tz=timezone.utc).isoformat()
    accessed = datetime.fromtimestamp(stat.st_atime, tz=timezone.utc).isoformat()

    return {
        "path": str(path.resolve()),
        "size_bytes": size_bytes,
        "lines": num_lines,
        "words": num_words,
        "complexity": complexity,
        "created": created,
        "modified": modified,
        "accessed": accessed,
        "line_length_max": line_length_max,
        "line_length_min": line_length_min,
        "line_length_avg": line_length_avg,
        "words_per_line_avg": words_per_line_avg,
        "checksum": checksum,
    }


def file_stats(path: str) -> FileStatsResult:
    """Compute file metrics for the absolute path ``path``."""
    file_path = Path(path)
    if not file_path.is_absolute():
        raise FileStatsError("Path must be absolute.")
    if not file_path.exists():
        raise FileStatsError("File not found.")
    if not file_path.is_file():
        raise FileStatsError("Not a regular file.")

    return FileStatsResult(**compute_file_stats(file_path))


class FileStatsTool(ToolDefinition):
    name = "file-stats"
    title = "File stats"
    description = (
        "Get file metrics for access and processing planning: complexity, timestamps, "
        "size, line/word counts, and line length statistics."
    )
    input_schema = {
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "Absolute file path.",
            },
        },
        "required": ["path"],
    }
    output_schema = {
        "type": "object",
        "properties": {
            "path": {
                "type": "string",
                "description": "Absolute file path.",
            },
            "size_bytes": {
                "type": "integer",
                "description": "File size in bytes.",
            },
            "lines": {
                "type": "integer",
                "description": "Total number of lines.",
            },
            "words": {
                "type": "integer",
                "description": "Total number of words.",
            },
            "complexity": {
                "type": "number",
                "description": "Data structure complexity (0.0 to 1.0).",
            },
            "created": {
                "type": "string",
                "description": "Creation timestamp (ISO 8601).",
            },
            "modified": {
                "type": "string",
                "description": "Last modification timestamp (ISO 8601).",
            },
            "accessed": {
                "type": "string",
                "description": "Last access timestamp (ISO 8601).",
            },
            "line_length_max": {
                "type": "integer",
                "description": "Maximum line length in characters.",
            },
            "line_length_min": {
                "type": "integer",
                "description": "Minimum line length in characters.",
            },
            "line_length_avg": {
                "type": "number",
                "description": "Average line length in characters.",
            },
            "words_per_line_avg": {
                "type": "number",
                "description": "Average number of words per line.",
            },
            "checksum": {
                "type": "string",
                "description": "sha256 checksum of the file content.",
            },
        },
        "required": [
            "path", "size_bytes", "lines", "words", "complexity",
            "created", "modified", "accessed",
            "line_length_max", "line_length_min", "line_length_avg",
            "words_per_line_avg", "checksum"
        ],
    }
    annotations = {"readOnlyHint": True, "openWorldHint": False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`file_stats`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = file_stats(args["path"])
        except FileStatsError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)

        return ToolResult(
            content=[],
            structured_content=result.__dict__,
            auto_approve=True,
        )


def register_file_stats_tool(registry: ToolRegistry) -> None:
    registry.register(FileStatsTool())
