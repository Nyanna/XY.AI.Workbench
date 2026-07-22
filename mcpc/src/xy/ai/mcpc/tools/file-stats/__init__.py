"""File stats tool – returns file metrics for access and processing planning.

Provides compact metrics including complexity, timestamps, size, line/word
counts, line length statistics, and average words per line.
"""

from __future__ import annotations

import hashlib
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content


def _calculate_complexity(text: str) -> float:
    """Calculate data structure complexity (0.0 to 1.0).
    
    Based on character set diversity and pattern variation.
    """
    if not text:
        return 0.0
    
    # Count unique character types
    has_alpha = bool(re.search(r'[a-zA-Z]', text))
    has_digit = bool(re.search(r'\d', text))
    has_punct = bool(re.search(r'[^\w\s]', text))
    has_space = bool(re.search(r'\s', text))
    has_upper = bool(re.search(r'[A-Z]', text))
    has_lower = bool(re.search(r'[a-z]', text))
    
    char_type_score = sum([has_alpha, has_digit, has_punct, has_space, has_upper, has_lower]) / 6.0
    
    # Entropy-like measure based on unique characters
    unique_chars = len(set(text))
    entropy_score = min(1.0, unique_chars / 256.0)
    
    complexity = (char_type_score * 0.4) + (entropy_score * 0.6)
    return round(complexity, 3)


def register_file_stats_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "file-stats",
        title="File stats",
        description=(
            "Get file metrics for access and processing planning: complexity, timestamps, "
            "size, line/word counts, and line length statistics."
        ),
        input_schema={
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute file path.",
                },
            },
            "required": ["path"],
        },
        output_schema={
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
        },
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def file_stats(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        path_str: str = args["path"]

        path = Path(path_str)
        if not path.is_absolute():
            return ToolResult(
                content=[text_content("Path must be absolute.")],
                is_error=True,
            )
        if not path.exists():
            return ToolResult(
                content=[text_content("File not found.")],
                is_error=True,
            )
        if not path.is_file():
            return ToolResult(
                content=[text_content("Not a regular file.")],
                is_error=True,
            )

        # --- Read file ---
        raw_bytes = path.read_bytes()
        text = raw_bytes.decode("utf-8", errors="replace")
        lines = text.splitlines()

        # --- Calculate metrics ---
        size_bytes = len(raw_bytes)
        num_lines = len(lines)
        
        # Word count
        words = text.split()
        num_words = len(words)
        
        # Complexity
        complexity = _calculate_complexity(text)

        # Checksum
        checksum = hashlib.sha256(text.encode("utf-8")).hexdigest()
        
        # Line length stats
        line_lengths = [len(line) for line in lines] if lines else [0]
        line_length_max = max(line_lengths) if line_lengths else 0
        line_length_min = min(line_lengths) if line_lengths else 0
        line_length_avg = sum(line_lengths) / len(line_lengths) if line_lengths else 0.0
        line_length_avg = round(line_length_avg, 2)
        
        # Words per line
        words_per_line = []
        for line in lines:
            line_words = len(line.split())
            words_per_line.append(line_words)
        words_per_line_avg = (sum(words_per_line) / len(words_per_line)) if words_per_line else 0.0
        words_per_line_avg = round(words_per_line_avg, 2)
        
        # Timestamps
        stat = path.stat()
        created = datetime.fromtimestamp(
            stat.st_birthtime if hasattr(stat, 'st_birthtime') else stat.st_mtime,
            tz=timezone.utc
        ).isoformat()
        modified = datetime.fromtimestamp(
            stat.st_mtime, tz=timezone.utc
        ).isoformat()
        accessed = datetime.fromtimestamp(
            stat.st_atime, tz=timezone.utc
        ).isoformat()

        structured: dict[str, Any] = {
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

        return ToolResult(
            content=[],
            structured_content=structured,
            auto_approve=True,
        )
