"""Visibility filter based on ``.mcpcignore`` files.

A ``.mcpcignore`` is an *allow list* in ``.gitignore`` syntax with inverted meaning:
a matching pattern *allows* a path, a negated pattern (``!pattern``) *excludes* it.
Typically whole directories are listed (e.g. ``/src/resident``); a pattern without a
trailing slash matches the directory and everything below it, but not its siblings.
Everything that is not allowed behaves as if it did not exist.

Priority of rules (independent of their order and of the file they are in):
**deny > allow > abstain**. A path is visible only if no matching rule in the whole
chain excludes it and at least one allows it. Files and directories are judged alike.
Without any ``.mcpcignore`` in the chain a built-in default applies instead.

How :func:`is_allowed` judges a path (no request context, no distinction between
files and directories):

1. Starting at the path's directory and going upwards, the first directory that
   contains a ``.mcpcignore`` yields an :class:`IgnoreConfiguration`.
2. That configuration does all the work. It asks its parent configuration (the
   next ``.mcpcignore`` further up) first and then applies its own patterns to the
   full path (relative to its own directory). Rules of all files add up.
3. If there is no ``.mcpcignore`` at all, the built-in default applies: everything is
   allowed except paths containing one of :data:`DEFAULT_EXCLUDED_DIRS`.

:func:`is_excluded` is only a shortcut for directory walks: a directory that is
explicitly excluded can be skipped as a whole, since nothing below it can be visible.
A directory that is merely not allowed (e.g. ``src`` above an allowed ``src/resident``)
must still be entered.

Caches (all lazy, each re-check happens at most every :data:`CHECK_INTERVAL_SECONDS`):

* per directory level either its configuration or the knowledge that it has none;
  a missing configuration is looked for again after the interval,
* each configuration reloads its file when modification time or size changed,
* a configuration whose file was removed stays in place: it keeps delegating to its
  parents and contributes no patterns itself.
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
        for line in lines:
            if line.startswith('!'):
                deny.append(line[1:])
            else:
                allow.append(line)
        self._allow = PathSpec.from_lines('gitwildmatch', allow)
        self._deny = PathSpec.from_lines('gitwildmatch', deny)

    def verdict(self, relative: str) -> bool | None:
        """False if any deny rule matches, else True if any allow rule matches, else None."""
        if self._deny.match_file(relative):
            return False
        if self._allow.match_file(relative):
            return True
        return None
_DEFAULT_RULES = _Rules(['*', *(f'!{name}/' for name in DEFAULT_EXCLUDED_DIRS)])

def _default_verdict(target: Path, is_dir: bool) -> bool | None:
    """The built-in default, applied to the full absolute path."""
    relative = target.as_posix().lstrip('/')
    if not relative:
        return None
    return _DEFAULT_RULES.verdict(relative + '/' if is_dir else relative)

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

    def _verdict(self, path: Path, is_dir: bool) -> bool | None:
        """False if denied anywhere in the chain, else True if allowed anywhere, else None. Parent first."""
        parent = self._parent_configuration()
        verdict = parent._verdict(path, is_dir) if parent is not None else None
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
        own = rules.verdict(relative + '/' if is_dir else relative)
        return verdict if own is None else own

    def is_allowed(self, path: str | os.PathLike[str]) -> bool:
        """Whether ``path`` (file or directory alike) is visible: parent configurations first, then this one."""
        target = Path(os.path.abspath(path))
        if not self._has_rules():
            return _default_verdict(target, False) is not False
        return self._verdict(target, False) is True

    def is_excluded(self, directory: str | os.PathLike[str]) -> bool:
        """Whether ``directory`` is explicitly excluded, so that nothing below it can be visible."""
        target = Path(os.path.abspath(directory))
        if not self._has_rules():
            return _default_verdict(target, True) is False
        return self._verdict(target, True) is False
    isAllowed = is_allowed
_registry: dict[Path, IgnoreConfiguration] = {}
_absent: dict[Path, float] = {}
_registry_lock = threading.Lock()

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
        return _default_verdict(target, False) is not False
    return configuration.is_allowed(target)

def is_excluded(directory: str | os.PathLike[str]) -> bool:
    """Whether ``directory`` is explicitly excluded (see module docstring); a shortcut for directory walks."""
    target = Path(os.path.abspath(directory))
    configuration = ignore_for(target.parent)
    if configuration is None:
        return _default_verdict(target, True) is False
    return configuration.is_excluded(target)