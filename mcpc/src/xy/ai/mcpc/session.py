"""In-memory, server-side session state.

The server is *stateful*: for every session id supplied via the
``X-MCPC-SESSION-ID`` header it keeps a :class:`Session` object that persists
the negotiated protocol version, the client-specific tool configuration and
arbitrary per-session state for the lifetime of the process.
"""

from __future__ import annotations

import logging
import threading
import time
import uuid
from dataclasses import dataclass, field
from typing import Any, Iterator

logger = logging.getLogger("xy.ai.mcpc.session")

def is_valid_uuid(value: str) -> bool:
    try:
        uuid.UUID(str(value))
    except (ValueError, AttributeError, TypeError):
        return False
    return True


@dataclass(slots=True)
class AgentSubSession:
    """Bookkeeping for a single sub-agent spawned from a session.

    A session may spawn an arbitrary number of sub-agents; each one is tracked
    here and keyed by its CLI-session id (which is also the id of the pre-created
    MCPC session the sub-agent connects back with).  The last-used timestamp
    drives the one-hour idle TTL used when a ``resume`` is requested.
    """

    cli_session_id: str
    created_at: float = field(default_factory=time.time)
    last_used_at: float = field(default_factory=time.time)
    model: str | None = None
    profile: str | None = None

    def touch(self) -> None:
        self.last_used_at = time.time()

    def is_valid(self, ttl_seconds: float, *, now: float | None = None) -> bool:
        now = time.time() if now is None else now
        return (now - self.last_used_at) <= ttl_seconds

    def summary(self) -> dict[str, Any]:
        return {
            "cliSessionId": self.cli_session_id,
            "createdAt": self.created_at,
            "lastUsedAt": self.last_used_at,
            "model": self.model,
            "profile": self.profile,
        }


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
    
    #: Names of tools enabled for this session. An empty set means 
    #: no tools are enabled.
    enabled_tools: set[str] = field(default_factory=set)

    #: Sub-agents spawned from this session, keyed by their CLI-session id.  A
    #: single session may drive an arbitrary number of sub-agents concurrently.
    agent_sessions: dict[str, AgentSubSession] = field(default_factory=dict)

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
        return name in self.enabled_tools

    def set_enabled_tools(self, names: "set[str] | list[str] | None") -> None:
        """Replace the set of enabled tools (``None`` or empty input clears it)."""
        logger.info("Enable tools for session: %s", names)
        with self.lock:
            self.enabled_tools = set() if names is None else set(names)

    def register_agent_session(
        self,
        cli_session_id: str,
        *,
        model: str | None = None,
        profile: str | None = None,
    ) -> AgentSubSession:
        """Record (or refresh) a sub-agent spawned from this session."""
        with self.lock:
            record = self.agent_sessions.get(cli_session_id)
            if record is None:
                record = AgentSubSession(
                    cli_session_id=cli_session_id, model=model, profile=profile
                )
                self.agent_sessions[cli_session_id] = record
            else:
                record.touch()
            return record

    def get_agent_session(self, cli_session_id: str) -> AgentSubSession | None:
        with self.lock:
            return self.agent_sessions.get(cli_session_id)

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
            "agentSessions": [r.summary() for r in self.agent_sessions.values()],
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

    def precreate(
        self,
        session_id: str,
        *,
        enabled_tools: "set[str] | list[str] | None" = None,
    ) -> Session:
        """Create (or fetch) a session *before* the client first connects.

        This is what the agent tool uses to stage a sub-agent's session with a
        pre-configured toolset: the sub-agent's CLI later connects with the same
        session id and never has to send the ``X-MCPC-TOOLS`` header itself.
        """
        with self._lock:
            session = self._sessions.get(session_id)
            if session is None:
                session = Session(id=session_id)
                self._sessions[session_id] = session
            if enabled_tools is not None:
                session.enabled_tools = set(enabled_tools)
            return session

    def set_enabled_tools(
        self,
        session_id: str,
        names: "set[str] | list[str] | None",
    ) -> Session:
        """Configure the active tools for *session_id*, creating it if needed.

        Convenience for use from within a tool implementation that needs to
        reconfigure the active toolset of an (existing or future) session id.
        """
        session = self.precreate(session_id)
        session.set_enabled_tools(names)
        return session

    def remove(self, session_id: str) -> bool:
        with self._lock:
            return self._sessions.pop(session_id, None) is not None

    def __len__(self) -> int:
        with self._lock:
            return len(self._sessions)

    def __iter__(self) -> Iterator[Session]:
        with self._lock:
            return iter(list(self._sessions.values()))
