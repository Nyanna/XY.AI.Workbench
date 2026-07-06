"""Central registry that creates, resolves, and expires CLI sessions.

The manager itself never starts or stops a CLI process — that is the session's
responsibility.  It maintains the index, hands out session objects on request,
and, on every request, sweeps expired sessions (asking them to terminate their
CLI and dropping them from the index).
"""

from __future__ import annotations

import threading
from pathlib import Path

from .parameters import CliParameters
from .session import CliSession, CliSessionError, Launcher, default_launcher

__all__ = ["CliSessionError", "CliSessionManager"]


class CliSessionManager:
    """Thread-safe pool of :class:`CliSession` objects keyed by CLI-session id."""

    def __init__(
        self,
        *,
        log_dir: Path | str = "logs/cli",
        ttl_seconds: float = 3600.0,
        response_timeout: float = 300.0,
        launcher: Launcher = default_launcher,
    ) -> None:
        self.log_dir = Path(log_dir)
        self.ttl_seconds = ttl_seconds
        self.response_timeout = response_timeout
        self._launcher = launcher
        self._sessions: dict[str, CliSession] = {}
        self._lock = threading.RLock()

    # -- requests -----------------------------------------------------------
    def request(
        self,
        parameters: CliParameters | None = None,
        *,
        resume: str | None = None,
        session_id: str | None = None,
    ) -> CliSession:
        """Obtain a CLI session, creating a new one unless *resume* is given.

        When *resume* is provided the existing session is looked up and its
        validity is verified; an unknown or expired session raises
        :class:`CliSessionError` (after terminating any lingering process).
        Otherwise a brand new session is created and indexed under *session_id*
        (a fresh UUID when omitted).
        """
        with self._lock:
            self._sweep()
            if resume is not None:
                return self._resume(resume)
            if parameters is None:
                raise CliSessionError("parameters are required to start a new CLI session")
            if session_id is not None and session_id in self._sessions:
                raise CliSessionError(f"CLI session already exists: {session_id}")
            session = CliSession(
                self,
                parameters,
                session_id=session_id,
                log_dir=self.log_dir,
                ttl_seconds=self.ttl_seconds,
                response_timeout=self.response_timeout,
                launcher=self._launcher,
            )
            self._sessions[session.id] = session
            return session

    def _resume(self, cli_session_id: str) -> CliSession:
        session = self._sessions.get(cli_session_id)
        if session is None:
            raise CliSessionError(f"Unknown CLI session: {cli_session_id}")
        if not session.is_valid():
            session.terminate()
            self._sessions.pop(cli_session_id, None)
            raise CliSessionError(f"CLI session {cli_session_id} has expired")
        return session

    # -- resolution / maintenance ------------------------------------------
    def resolve(self, cli_session_id: str) -> CliSession | None:
        """Resolve a session by id without validity/expiry side effects."""
        with self._lock:
            return self._sessions.get(cli_session_id)

    def _sweep(self) -> None:
        """Terminate and drop every expired session (caller holds the lock)."""
        expired = [s for s in self._sessions.values() if not s.is_valid()]
        for session in expired:
            session.terminate()
            self._sessions.pop(session.id, None)

    def sweep(self) -> None:
        with self._lock:
            self._sweep()

    def shutdown(self) -> None:
        """Terminate every managed CLI process and clear the index."""
        with self._lock:
            for session in list(self._sessions.values()):
                session.terminate()
            self._sessions.clear()

    def __len__(self) -> int:
        with self._lock:
            return len(self._sessions)
