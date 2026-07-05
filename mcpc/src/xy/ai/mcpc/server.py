"""Assembling and running the MCP Controller HTTP server."""

from __future__ import annotations

import logging
from http.server import ThreadingHTTPServer
from typing import Any

from .config import ServerConfig
from .logging_utils import CommunicationLog
from .protocol import McpProtocol
from .registry import ToolRegistry
from .session import SessionStore
from .transport import StreamableHttpHandler

logger = logging.getLogger("xy.ai.mcpc")


class McpHTTPServer(ThreadingHTTPServer):
    """Threaded HTTP server carrying the shared MCP component graph."""

    daemon_threads = True
    allow_reuse_address = True

    def __init__(
        self,
        config: ServerConfig,
        protocol: McpProtocol,
        sessions: SessionStore,
        comm_log: CommunicationLog,
    ) -> None:
        self.config = config
        self.protocol = protocol
        self.sessions = sessions
        self.comm_log = comm_log
        self.logger = logger
        super().__init__((config.host, config.port), StreamableHttpHandler)

    @property
    def endpoint_url(self) -> str:
        host, port = self.server_address[0], self.server_address[1]
        return f"http://{host}:{port}{self.config.path}"


def build_server(
    config: ServerConfig | None = None,
    registry: ToolRegistry | None = None,
    *,
    register_builtin: bool = True,
) -> McpHTTPServer:
    """Construct (but do not start) an :class:`McpHTTPServer`.

    If no *registry* is supplied a fresh one is created; unless
    ``register_builtin`` is false the built-in example tools are registered.
    """
    config = config or ServerConfig()

    if registry is None:
        registry = ToolRegistry()
        if register_builtin:
            from .tools import register_builtin_tools

            register_builtin_tools(registry)

    protocol = McpProtocol(config, registry)
    sessions = SessionStore()
    comm_log = CommunicationLog(config.log_dir)
    return McpHTTPServer(config, protocol, sessions, comm_log)


def run(config: ServerConfig | None = None, **build_kwargs: Any) -> None:
    """Build a server from *config* and serve until interrupted."""
    server = build_server(config, **build_kwargs)
    logger.info("MCP Controller listening on %s", server.endpoint_url)
    logger.info("Session header: %s | log dir: %s",
                server.config.session_header, server.comm_log.directory)
    try:
        server.serve_forever()
    except KeyboardInterrupt:  # pragma: no cover - interactive
        logger.info("Shutting down")
    finally:
        server.shutdown()
        server.server_close()
