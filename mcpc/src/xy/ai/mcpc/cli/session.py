"""A single managed CLI process and its stream-json conversation."""

from __future__ import annotations

import os
import queue
import subprocess
import threading
import time
import uuid
from dataclasses import dataclass
from pathlib import Path
from typing import TYPE_CHECKING, Any, Callable, Protocol

from ..codec import JsonCodec
from .parameters import CliParameters

if TYPE_CHECKING:
    from .manager import CliSessionManager


class Process(Protocol):
    """The subset of :class:`subprocess.Popen` the session relies on."""

    stdin: Any
    stdout: Any
    stderr: Any

    def poll(self) -> int | None: ...
    def terminate(self) -> None: ...
    def kill(self) -> None: ...
    def wait(self, timeout: float | None = None) -> int: ...


#: A launcher turns a command line + environment into a running process.  It is
#: injectable so tests can supply a fake CLI without spawning ``claude``.
Launcher = Callable[[list[str], dict[str, str]], Process]

#: Sentinel pushed onto the output queue when the process stream reaches EOF.
_EOF = object()


def default_launcher(cmd: list[str], env: dict[str, str]) -> Process:
    """Start a real CLI process wired for line-buffered stream-json stdio."""
    return subprocess.Popen(  # type: ignore[return-value]
        cmd,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env=env,
        text=True,
        bufsize=1,
    )


class CliSessionError(RuntimeError):
    """Raised when a prompt cannot be served (expired, timed out, crashed…)."""


@dataclass(slots=True)
class CliResult:
    """The outcome of a single prompt/response exchange."""

    text: str
    is_error: bool = False
    subtype: str | None = None


class CliSession:
    """Container around a CLI process that serves prompts over stdin/stdout.

    The session owns the process lifecycle and reuses the CLI's prompt cache by
    passing ``--session-id`` on first start and ``--resume`` on restart.  It
    replicates every line sent and received to a per-session NDJSON log and
    enforces a one-hour idle TTL keyed on the last *sent* prompt.
    """

    def __init__(
        self,
        manager: "CliSessionManager",
        parameters: CliParameters,
        *,
        session_id: str | None = None,
        log_dir: Path | None = None,
        ttl_seconds: float = 3600.0,
        response_timeout: float = 300.0,
        launcher: Launcher = default_launcher,
    ) -> None:
        self.manager = manager
        self.parameters = parameters
        self.id = session_id or str(uuid.uuid4())
        self.ttl_seconds = ttl_seconds
        self.response_timeout = response_timeout
        self._launcher = launcher
        self._log_path = Path(log_dir) / f"{self.id}.json.log" if log_dir else None

        self.created_at = time.time()
        self.started_at: float | None = None
        self.last_sent_at: float | None = None
        self.last_received_at: float | None = None

        self._process: Process | None = None
        self._started_once = False
        self._reader: threading.Thread | None = None
        self._out: "queue.Queue[Any]" = queue.Queue()
        self._lock = threading.RLock()
        self._log_lock = threading.Lock()

    # -- validity -----------------------------------------------------------
    @property
    def running(self) -> bool:
        return self._process is not None and self._process.poll() is None

    def _ttl_reference(self) -> float:
        # The last *sent* message drives the TTL; before any prompt, use the
        # creation time so a freshly minted session is valid.
        return self.last_sent_at if self.last_sent_at is not None else self.created_at

    def is_valid(self, *, now: float | None = None) -> bool:
        now = time.time() if now is None else now
        return (now - self._ttl_reference()) <= self.ttl_seconds

    # -- process lifecycle --------------------------------------------------
    def _start(self) -> None:
        cmd = self.parameters.build_command(self.id, resume=self._started_once)
        env = self.parameters.build_environment(dict(os.environ))
        self._process = self._launcher(cmd, env)
        self.started_at = time.time()
        self._started_once = True
        self._out = queue.Queue()
        self._reader = threading.Thread(
            target=self._read_loop, args=(self._process,), daemon=True
        )
        self._reader.start()

    def _read_loop(self, process: Process) -> None:
        stdout = process.stdout
        try:
            for line in iter(stdout.readline, ""):
                if line == "":
                    break
                self.last_received_at = time.time()
                self._replicate("out", line.rstrip("\n"))
                obj = JsonCodec.decode_line(line)
                if obj is None:
                    continue
                self._out.put(obj)
        finally:
            self._out.put(_EOF)

    def terminate(self) -> None:
        """Terminate a running CLI process and release its streams."""
        with self._lock:
            process = self._process
            self._process = None
            if process is None:
                return
            if process.poll() is None:
                try:
                    process.terminate()
                    try:
                        process.wait(timeout=5)
                    except Exception:  # noqa: BLE001 - fall through to kill
                        process.kill()
                except Exception:  # noqa: BLE001 - best-effort teardown
                    pass
            for stream in (process.stdin, process.stdout, process.stderr):
                try:
                    if stream is not None:
                        stream.close()
                except Exception:  # noqa: BLE001
                    pass

    # -- prompting ----------------------------------------------------------
    def prompt(self, text: str) -> CliResult:
        """Send *text* to the CLI and return its final result.

        A prompt sent to an invalid (expired) session terminates any lingering
        process and raises; otherwise it lazily (re)starts the process, sending
        ``--resume`` when reattaching to a previously started CLI session.
        """
        with self._lock:
            if not self.is_valid():
                self.terminate()
                raise CliSessionError(f"CLI session {self.id} has expired")

            if not self.running:
                self._start()

            self._send_user_message(text)
            return self._collect_result()

    def _send_user_message(self, text: str) -> None:
        assert self._process is not None and self._process.stdin is not None
        message = {
            "type": "user",
            "message": {"role": "user", "content": [{"type": "text", "text": text}]},
        }
        self._replicate("in", JsonCodec.encode(message, compact=True))
        try:
            JsonCodec.write_line(self._process.stdin, message)
        except (BrokenPipeError, ValueError) as exc:
            raise CliSessionError(f"CLI session {self.id} is not accepting input: {exc}")
        # A sent prompt resets the remaining lifetime to the full TTL.
        self.last_sent_at = time.time()

    def _collect_result(self) -> CliResult:
        deadline = time.time() + self.response_timeout
        assistant_text: list[str] = []
        while True:
            remaining = deadline - time.time()
            if remaining <= 0:
                raise CliSessionError(
                    f"CLI session {self.id} timed out after {self.response_timeout}s"
                )
            try:
                obj = self._out.get(timeout=remaining)
            except queue.Empty:
                raise CliSessionError(f"CLI session {self.id} timed out")

            if obj is _EOF:
                raise CliSessionError(
                    f"CLI session {self.id} ended before returning a result"
                )
            if not isinstance(obj, dict):
                continue

            kind = obj.get("type")
            if kind == "assistant":
                assistant_text.append(_extract_text(obj.get("message", {})))
            elif kind == "result":
                subtype = obj.get("subtype")
                is_error = bool(obj.get("is_error")) or (
                    isinstance(subtype, str) and subtype != "success"
                )
                text = obj.get("result")
                if not isinstance(text, str):
                    text = "".join(assistant_text)
                return CliResult(text=text, is_error=is_error, subtype=subtype)

    # -- replication --------------------------------------------------------
    def _replicate(self, direction: str, line: str) -> None:
        if self._log_path is None:
            return
        entry = {
            "ts": time.time(),
            "cliSessionId": self.id,
            "direction": direction,
            "line": line,
        }
        record = JsonCodec.encode(entry)
        with self._log_lock:
            self._log_path.parent.mkdir(parents=True, exist_ok=True)
            with self._log_path.open("a", encoding="utf-8") as fh:
                fh.write(record + "\n")

    # -- diagnostics --------------------------------------------------------
    def summary(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "running": self.running,
            "createdAt": self.created_at,
            "startedAt": self.started_at,
            "lastSentAt": self.last_sent_at,
            "lastReceivedAt": self.last_received_at,
            "valid": self.is_valid(),
        }


def _extract_text(message: dict[str, Any]) -> str:
    """Concatenate text blocks from an assistant stream-json message."""
    content = message.get("content")
    if isinstance(content, str):
        return content
    if not isinstance(content, list):
        return ""
    parts = [
        block.get("text", "")
        for block in content
        if isinstance(block, dict) and block.get("type") == "text"
    ]
    return "".join(parts)
