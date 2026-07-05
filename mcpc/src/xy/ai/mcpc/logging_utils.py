"""Per-session communication logging.

Every JSON message exchanged with a client is appended, one JSON object per
line (JSON Lines / NDJSON), to ``<log_dir>/<session-id>.log``.  This gives a
complete, replayable audit trail keyed by the session id.
"""

from __future__ import annotations

import json
import threading
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

#: Log entry directions.
IN = "in"        # client -> server
OUT = "out"      # server -> client
EVENT = "event"  # server-side lifecycle / diagnostic entry


def _now_iso() -> str:
    return datetime.now(timezone.utc).isoformat()


def _safe_name(session_id: str) -> str:
    """Sanitise a session id for safe use as a filename.

    Keeps only characters that are safe on common filesystems; anything else is
    replaced with ``_`` so a malicious session id cannot escape ``log_dir``.
    """
    return "".join(c if (c.isalnum() or c in "-_.") else "_" for c in session_id) or "unknown"


class CommunicationLog:
    """Thread-safe, append-only NDJSON logger, one file per session id."""

    def __init__(self, log_dir: Path) -> None:
        self._dir = Path(log_dir)
        self._dir.mkdir(parents=True, exist_ok=True)
        self._locks: dict[str, threading.Lock] = {}
        self._guard = threading.Lock()

    @property
    def directory(self) -> Path:
        return self._dir

    def path_for(self, session_id: str) -> Path:
        return self._dir / f"{_safe_name(session_id)}.log"

    def _lock_for(self, key: str) -> threading.Lock:
        with self._guard:
            lock = self._locks.get(key)
            if lock is None:
                lock = threading.Lock()
                self._locks[key] = lock
            return lock

    def log(
        self,
        session_id: str,
        direction: str,
        payload: Any,
        **meta: Any,
    ) -> None:
        """Append a single log entry for *session_id*.

        ``payload`` is typically the JSON-RPC message; ``meta`` carries extra
        context such as the HTTP method or status code.
        """
        entry: dict[str, Any] = {
            "ts": _now_iso(),
            "session": session_id,
            "direction": direction,
        }
        if meta:
            entry.update(meta)
        entry["message"] = payload
        line = json.dumps(entry, ensure_ascii=False, default=str)
        key = _safe_name(session_id)
        path = self._dir / f"{key}.log"
        with self._lock_for(key):
            with path.open("a", encoding="utf-8") as fh:
                fh.write(line)
                fh.write("\n")
