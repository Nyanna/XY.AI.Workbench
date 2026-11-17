"""Visibility filter based on ``.mcpcignore`` files.

Syntax is that of ``.gitignore`` but with inverted meaning: a matching pattern
*allows* a path, a negated pattern (``!pattern``) *excludes* it. Everything that
is not allowed behaves as if it did not exist.
"""
from __future__ import annotations
from pathlib import Path
from pathspec import GitIgnoreSpec
IGNORE_FILE_NAME = '.mcpcignore'

class McpcIgnore:
    """Evaluates the ``.mcpcignore`` files from ``/`` down to a base directory.

    Files are combined root first, so deeper files override shallower ones and,
    as in gitignore, the last matching pattern wins. Patterns are relative to the
    directory containing their ``.mcpcignore``. Without any ``.mcpcignore``
    everything is visible.

    Directories are visible unless explicitly excluded, so that allowed files
    below them (e.g. ``*.py``) stay reachable. Files must be explicitly allowed.
    """

    def __init__(self, base: Path) -> None:
        base = Path(base).resolve()
        self._base = base
        self._specs: list[tuple[Path, GitIgnoreSpec]] = []
        for directory in reversed([base, *base.parents]):
            ignore_file = directory / IGNORE_FILE_NAME
            try:
                lines = ignore_file.read_text(encoding='utf-8').splitlines()
            except OSError:
                continue
            self._specs.append((directory, GitIgnoreSpec.from_lines(lines)))

    def _verdict(self, path: Path, is_dir: bool) -> bool | None:
        """Return True if allowed, False if excluded, None if no pattern matched."""
        verdict: bool | None = None
        for directory, spec in self._specs:
            try:
                relative = path.relative_to(directory).as_posix()
            except ValueError:
                continue
            if is_dir:
                relative += '/'
            result = spec.check_file(relative)
            if result.include is not None:
                verdict = result.include
        return verdict

    def is_file_allowed(self, path: Path) -> bool:
        if not self._specs:
            return True
        resolved = Path(path).resolve()
        if self._base in resolved.parents:
            for parent in resolved.parents:
                if parent == self._base:
                    break
                if not self.is_dir_visible(parent):
                    return False
        return self._verdict(resolved, is_dir=False) is True

    def is_dir_visible(self, path: Path) -> bool:
        if not self._specs:
            return True
        return self._verdict(Path(path).resolve(), is_dir=True) is not False