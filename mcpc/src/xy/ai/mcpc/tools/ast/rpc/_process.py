"""Base class for lazily-started AST engine subprocesses speaking the RPC/PIC
HTTP protocol described by ``ast-engines/openapi.yaml``.

Subclasses only need to supply the port and the argv that starts their
engine (JVM, .NET runtime, ...); everything else -- lazy startup, readiness
polling, keeping the process alive for the server's lifetime -- lives here so
future engines (C#, Go, ...) only add their own :class:`RpcProcess` subclass.
"""
from __future__ import annotations
import atexit
import logging
import socket
import subprocess
import threading
import time
from abc import ABC, abstractmethod
logger = logging.getLogger('xy.ai.mcpc.tools.ast.rpc')
__all__ = ['RpcProcess']

class RpcProcess(ABC):
    """Supervises a single long-lived RPC engine subprocess, started on first use.

    Thread-safe: concurrent callers are serialized so only one subprocess is
    ever spawned. Once started, the process is left running (registered via
    :mod:`atexit`) for the lifetime of this Python process.
    """
    host = '127.0.0.1'
    startup_timeout = 30.0

    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._process: subprocess.Popen | None = None
        self._port: int | None = None

    @abstractmethod
    def default_port(self) -> int:
        """TCP port the engine should listen on."""

    @abstractmethod
    def command(self, port: int) -> list[str]:
        """Argv that launches the engine, listening on ``port``."""

    def cwd(self) -> str | None:
        """Working directory for the subprocess; ``None`` inherits the caller's."""
        return None

    def base_url(self) -> str:
        """URL of the (lazily started) engine's API root, e.g. ``http://127.0.0.1:8787``.

        Note: unlike the openapi document's example server URL, ``ast-java``
        mounts its routes at the root, not under ``/v1``.
        """
        self._ensure_running()
        return f'http://{self.host}:{self._port}'

    def _ensure_running(self) -> None:
        with self._lock:
            if self._process is not None and self._process.poll() is None:
                return
            port = self.default_port()
            argv = self.command(port)
            logger.info('%s: starting %s', self.__class__.__name__, ' '.join(argv))
            self._process = subprocess.Popen(
                argv,
                cwd=self.cwd(),
                stdout=subprocess.DEVNULL,
                stderr=subprocess.PIPE,
                text=True)
            self._port = port
            atexit.register(self._terminate)
            self._wait_until_ready(port)
            logger.info('%s: ready on port %d (pid %d)', self.__class__.__name__, port, self._process.pid)

    def _wait_until_ready(self, port: int) -> None:
        deadline = time.monotonic() + self.startup_timeout
        while time.monotonic() < deadline:
            if self._process.poll() is not None:
                stderr = self._process.stderr.read() if self._process.stderr else ''
                logger.error(
                    '%s: exited during startup (code %s): %s',
                    self.__class__.__name__,
                    self._process.returncode,
                    stderr.strip())
                raise RuntimeError(
                    f'{self.__class__.__name__} exited during startup (code {self._process.returncode}): {stderr.strip()}')
            try:
                with socket.create_connection((self.host, port), timeout=0.5):
                    return
            except OSError:
                time.sleep(0.1)
        logger.error('%s: did not become ready on port %d within %ss',
                     self.__class__.__name__, port, self.startup_timeout)
        self._terminate()
        raise TimeoutError(
            f'{self.__class__.__name__} did not become ready on port {port} within {self.startup_timeout}s.')

    def _terminate(self) -> None:
        process, self._process = (self._process, None)
        if process is not None and process.poll() is None:
            logger.info('%s: terminating pid %d', self.__class__.__name__, process.pid)
            process.terminate()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()