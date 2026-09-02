"""CLI-session management.

A :class:`CliSessionManager` owns a pool of :class:`CliSession` objects, each of
which wraps a single ``claude`` CLI process driven over a stream-json stdio
protocol.  The manager creates, resolves, and expires sessions; the individual
session object owns the process lifecycle and replicates its I/O to disk.
"""
from xy.ai.mcpc.cli.manager import CliSessionError, CliSessionManager
from xy.ai.mcpc.cli.parameters import CliParameters, Effort, Model
from xy.ai.mcpc.cli.session import CliSession
__all__ = []