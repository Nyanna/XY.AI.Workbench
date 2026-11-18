"""Visibility filter based on ``.mcpcignore`` files.

A ``.mcpcignore`` is an *allow list* in ``.gitignore`` syntax with inverted meaning:
a matching pattern *allows* a path, a negated pattern (``!pattern``) *excludes* it.
A pattern covers a directory and everything below it, but not its siblings. A trailing
slash is ignored: files and directories are not told apart. What is not allowed behaves as if it did not exist.

Within one file deny beats allow beats no match, regardless of order. Every
``.mcpcignore`` in the chain of parent directories is a filter of its own: a path is
visible only if all of them allow it, so a deeper file can narrow but never widen.
Directories without a file are no filter. Without any file in the chain a built-in
default applies: everything except paths containing one of
:data:`DEFAULT_EXCLUDED_DIRS`, judged on the full absolute path.

:func:`is_excluded` is a shortcut for directory walks: only an explicitly excluded
directory can be skipped. A directory that is merely not allowed (``src`` above an
allowed ``src/resident``) must still be entered.

State is cached lazily and re-checked at most every :data:`CHECK_INTERVAL_SECONDS`.
A configuration whose file was removed stays in the chain and contributes no rules.
"""
from __future__ import annotations
import os
import threading
import time
from pathlib import Path
from pathspec import PathSpec
__all__ = [
    'CHECK_INTERVAL_SECONDS',
    'DEFAULT_EXCLUDED_DIRS',
    'IGNORE_FILE_NAME',
    'IgnoreConfiguration',
    'ignore_for',
    'is_allowed',
    'is_excluded']
IGNORE_FILE_NAME = '.mcpcignore'
CHECK_INTERVAL_SECONDS = 5.0
_MAX_ABSENT_CACHE = 8192
_MAX_REGISTRY = 1024
_NEVER = float('-inf')
DEFAULT_EXCLUDED_DIRS = (
    '.git',
    '.hg',
    '.svn',
    '__pycache__',
    '.mypy_cache',
    '.pytest_cache',
    '.ruff_cache',
    '.tox',
    '.venv',
    'venv',
    'node_modules',
    '.idea',
    '.vscode',
    'dist',
    'build',
    '.cache')

class _Rules:
    """The rules of one file, split into allow and deny patterns (gitwildmatch, full relative path)."""

    def __init__(self, lines: list[str]) -> None:
        allow: list[str] = []
        deny: list[str] = []
        '# a trailing slash is dropped: files and directories are not told apart'
        for line in lines:
            if line.startswith('!'):
                deny.append(line[1:].rstrip('/'))
            else:
                allow.append(line.rstrip('/'))
        self._allow = PathSpec.from_lines('gitwildmatch', allow)
        self._deny = PathSpec.from_lines('gitwildmatch', deny)

    def verdict(self, relative: str) -> bool | None:
        """False if any deny rule matches, else True if any allow rule matches, else None."""
        if self._deny.match_file(relative):
            return False
        if self._allow.match_file(relative):
            return True
        return None
_DEFAULT_RULES = _Rules(['*', *(f'!{name}' for name in DEFAULT_EXCLUDED_DIRS)])

def _default_verdict(target: Path) -> bool | None:
    """The built-in default, applied to the full absolute path."""
    relative = target.as_posix().lstrip('/')
    if not relative:
        return None
    return _DEFAULT_RULES.verdict(relative)

class IgnoreConfiguration:
    """The rules of one ``.mcpcignore`` file plus delegation to its parent configuration.

    Paths are evaluated lexically (no filesystem access, symlinks are not followed).
    """

    def __init__(self, directory: Path) -> None:
        self.directory = directory
        self._file = directory / IGNORE_FILE_NAME
        self._lock = threading.RLock()
        self._rules: _Rules | None = None
        self._stamp: tuple[int, int] | None = None
        self._last_check = _NEVER
        self._parent: IgnoreConfiguration | None = None
        self._parent_checked = _NEVER
    '# -- lazy state ---------------------------------------------------------'

    def _current_rules(self) -> _Rules | None:
        """Own rules; re-reads the file if (mtime, size) changed, checked at most every 5 s."""
        with self._lock:
            now = time.monotonic()
            if now - self._last_check >= CHECK_INTERVAL_SECONDS:
                self._last_check = now
                self._reload_if_changed()
            return self._rules

    def _reload_if_changed(self) -> None:
        try:
            stat = self._file.stat()
            stamp: tuple[int, int] | None = (stat.st_mtime_ns, stat.st_size)
        except OSError:
            stamp = None
        if stamp == self._stamp:
            return
        '# file removed: stay in the hierarchy, contribute nothing'
        if stamp is None:
            self._rules = None
            self._stamp = None
            return
        try:
            lines = self._file.read_text(encoding='utf-8').splitlines()
            self._rules = _Rules(lines)
            self._stamp = stamp
        except (OSError, ValueError):
            '# unreadable/invalid: retry at the next check'
            self._rules = None
            self._stamp = None

    def _parent_configuration(self) -> IgnoreConfiguration | None:
        """Nearest configuration above this one; searched again at most every 5 s."""
        with self._lock:
            now = time.monotonic()
            if now - self._parent_checked >= CHECK_INTERVAL_SECONDS:
                self._parent_checked = now
                above = self.directory.parent
                self._parent = None if above == self.directory else ignore_for(above)
            return self._parent
    '# -- evaluation ---------------------------------------------------------'

    def _has_rules(self) -> bool:
        if self._current_rules() is not None:
            return True
        parent = self._parent_configuration()
        return parent is not None and parent._has_rules()

    def _verdict(self, path: Path, strict: bool) -> bool | None:
        """Parent first. ``strict``: a file with rules that neither allows nor denies the path counts as denying it.

    Without ``strict`` only explicit denials yield False; None means no file had an opinion.
    """
        parent = self._parent_configuration()
        verdict = parent._verdict(path, strict) if parent is not None else None
        if verdict is False:
            return False
        rules = self._current_rules()
        if rules is None:
            return verdict
        try:
            relative = path.relative_to(self.directory).as_posix()
        except ValueError:
            return verdict
        if relative == '.':
            return verdict
        own = rules.verdict(relative)
        if own is None:
            return False if strict else verdict
        return own

    def is_allowed(self, path: str | os.PathLike[str]) -> bool:
        """Whether ``path`` (file or directory alike) is visible: every file in the chain must allow it."""
        target = Path(os.path.abspath(path))
        if not self._has_rules():
            return _default_verdict(target) is not False
        return self._verdict(target, True) is True

    def is_excluded(self, directory: str | os.PathLike[str]) -> bool:
        """Whether ``directory`` is explicitly excluded, so that nothing below it can be visible."""
        target = Path(os.path.abspath(directory))
        if not self._has_rules():
            return _default_verdict(target) is False
        return self._verdict(target, False) is False
_registry: dict[Path, IgnoreConfiguration] = {}
_absent: dict[Path, float] = {}
_registry_lock = threading.Lock()

def _prune_registry() -> None:
    """Drops configurations whose file is gone, or all of them if that frees nothing. Needs the registry lock."""
    for directory in [d for d, configuration in _registry.items() if not configuration._file.is_file()]:
        del _registry[directory]
    if len(_registry) >= _MAX_REGISTRY:
        _registry.clear()

def _level(directory: Path) -> IgnoreConfiguration | None:
    """The configuration of exactly this directory, or ``None`` (re-checked after the interval)."""
    with _registry_lock:
        known = _registry.get(directory)
        if known is not None:
            return known
        checked = _absent.get(directory)
    now = time.monotonic()
    if checked is not None and now - checked < CHECK_INTERVAL_SECONDS:
        return None
    if (directory / IGNORE_FILE_NAME).is_file():
        with _registry_lock:
            _absent.pop(directory, None)
            if directory not in _registry and len(_registry) >= _MAX_REGISTRY:
                _prune_registry()
            return _registry.setdefault(directory, IgnoreConfiguration(directory))
    with _registry_lock:
        if len(_absent) >= _MAX_ABSENT_CACHE:
            _absent.clear()
        _absent[directory] = now
    return None

def ignore_for(directory: str | os.PathLike[str]) -> IgnoreConfiguration | None:
    """First configuration at or above ``directory``, or ``None`` if there is no ``.mcpcignore``."""
    base = Path(os.path.abspath(directory))
    for candidate in (base, *base.parents):
        configuration = _level(candidate)
        if configuration is not None:
            return configuration
    return None

def is_allowed(path: str | os.PathLike[str]) -> bool:
    """Whether ``path`` (a file or a directory, no difference) is visible."""
    target = Path(os.path.abspath(path))
    configuration = ignore_for(target.parent)
    if configuration is None:
        return _default_verdict(target) is not False
    return configuration.is_allowed(target)

def is_excluded(directory: str | os.PathLike[str]) -> bool:
    """Whether ``directory`` is explicitly excluded (see module docstring); a shortcut for directory walks."""
    target = Path(os.path.abspath(directory))
    configuration = ignore_for(target.parent)
    if configuration is None:
        return _default_verdict(target) is False
    return configuration.is_excluded(target)