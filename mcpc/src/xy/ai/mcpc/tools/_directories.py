"""Shared directory-path normalization for tools that accept directory arguments.

Agents occasionally hallucinate a file path where a directory is expected (e.g.
passing a specific module file instead of its containing package directory).
Reducing such a path to its parent directory keeps the request working instead
of failing outright.
"""
from pathlib import Path
__all__ = ['normalize_directory', 'normalize_directories']

def normalize_directory(path: Path) -> Path:
    """Reduce ``path`` to its parent directory if it names an existing file."""
    return path.parent if path.is_file() else path

def normalize_directories(paths: list[Path]) -> list[Path]:
    """Apply :func:`normalize_directory` to each of ``paths``.

    Preserves order and de-duplicates directories that collapse onto each other.
    """
    seen: dict[Path, None] = {}
    for path in paths:
        seen.setdefault(normalize_directory(path), None)
    return list(seen)