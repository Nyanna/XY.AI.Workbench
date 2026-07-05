"""In-memory, server-side session state.

The server is *stateful*: for every session id supplied via the
``X-MCPC-SESSION-ID`` header it keeps a :class:`Session` object that persists
the negotiated protocol version, the client-specific tool configuration and
arbitrary per-session state for the lifetime of the process.
"""

from __future__ import annotations

import threading
import time
import uuid
from dataclasses import dataclass, field
from typing import Any, Iterator


def is_valid_uuid(value: str) -> bool:
    try:
        uuid.UUID(str(value))
    except (ValueError, AttributeError, TypeError):
        return False
    return True


@dataclass(slots=True)
class Session:
    """Server-side state for a single ``X-MCPC-SESSION-ID``."""

    id: str
    created_at: float = field(default_factory=time.time)
    last_seen_at: float = field(default_factory=time.time)

    #: Set once the ``initialize`` request has been processed.
    protocol_version: str | None = None
    #: Set once the ``notifications/initialized`` notification has arrived.
    initialized: bool = False

    client_info: dict[str, Any] | None = None
    client_capabilities: dict[str, Any] | None = None

    #: Names of tools enabled for this session.  ``None`` means "all tools
    #: currently in the registry" — the registry is reconciled against this set
    #: on every ``tools/list`` / ``tools/call``.
    enabled_tools: set[str] | None = None

    #: Arbitrary per-session key/value state persisted across requests.
    state: dict[str, Any] = field(default_factory=dict)

    #: Serialises request handling for this session so state stays consistent
    #: even though the HTTP server is multi-threaded.
    lock: threading.RLock = field(default_factory=threading.RLock, repr=False)

    @property
    def handshake_complete(self) -> bool:
        """True once ``initialize`` has been answered (operation may begin)."""
        return self.protocol_version is not None

    def touch(self) -> None:
        self.last_seen_at = time.time()

    def is_tool_enabled(self, name: str) -> bool:
        return self.enabled_tools is None or name in self.enabled_tools

    def summary(self) -> dict[str, Any]:
        """A JSON-serialisable snapshot, e.g. for diagnostics tools."""
        return {
            "id": self.id,
            "createdAt": self.created_at,
            "lastSeenAt": self.last_seen_at,
            "protocolVersion": self.protocol_version,
            "initialized": self.initialized,
            "clientInfo": self.client_info,
            "enabledTools": sorted(self.enabled_tools) if self.enabled_tools is not None else None,
            "stateKeys": sorted(self.state),
        }


class SessionStore:
    """Thread-safe registry of live sessions keyed by session id."""

    def __init__(self) -> None:
        self._sessions: dict[str, Session] = {}
        self._lock = threading.RLock()

    def get(self, session_id: str) -> Session | None:
        with self._lock:
            return self._sessions.get(session_id)

    def get_or_create(self, session_id: str) -> tuple[Session, bool]:
        """Return ``(session, created)`` for *session_id*."""
        with self._lock:
            session = self._sessions.get(session_id)
            if session is not None:
                return session, False
            session = Session(id=session_id)
            self._sessions[session_id] = session
            return session, True

    def remove(self, session_id: str) -> bool:
        with self._lock:
            return self._sessions.pop(session_id, None) is not None

    def __len__(self) -> int:
        with self._lock:
            return len(self._sessions)

    def __iter__(self) -> Iterator[Session]:
        with self._lock:
            return iter(list(self._sessions.values()))
