"""CLI-session management.

A :class:`CliSessionManager` owns a pool of :class:`CliSession` objects, each of
which wraps a single ``claude`` CLI process driven over a stream-json stdio
protocol.  The manager creates, resolves, and expires sessions; the individual
session object owns the process lifecycle and replicates its I/O to disk.

See ``project/sessionmanager.md`` for the specification.
"""

from __future__ import annotations

from .manager import CliSessionError, CliSessionManager
from .parameters import CliParameters, Effort, Model
from .session import CliSession

__all__ = [
    "CliParameters",
    "CliSession",
    "CliSessionError",
    "CliSessionManager",
    "Effort",
    "Model",
]
