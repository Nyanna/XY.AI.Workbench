"""Assembling and running the MCP Controller HTTP server."""

from __future__ import annotations

import logging
import socket
from http.server import ThreadingHTTPServer
from typing import Any

from .cli import CliSessionManager
from .config import ServerConfig
from .context import AppServices
from .control import ToolControlManager
from .logging_utils import CommunicationLog
from .protocol import McpProtocol
from .registry import ToolRegistry
from .session import SessionStore
from .tools.agent.profiles import DEFAULT_PROFILES, ProfileRegistry
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
        services: AppServices,
    ) -> None:
        self.config = config
        self.protocol = protocol
        self.sessions = sessions
        self.comm_log = comm_log
        self.services = services
        self.logger = logger
        super().__init__((config.host, config.port), StreamableHttpHandler)

    def get_request(self):
        """Accept a connection and enable TCP keepalive.

        Long-blocking tool-call requests (waiting for human approval) keep the
        HTTP connection open for up to 24 h.  Without keepalive, NAT gateways
        and proxies typically drop idle TCP connections after 5–15 minutes,
        causing ``ConnectionResetError`` on the server when it eventually
        tries to write the response.
        """
        conn, addr = super().get_request()
        conn.setsockopt(socket.SOL_SOCKET, socket.SO_KEEPALIVE, 1)
        # Start probing after 60 s of inactivity, retry every 10 s, drop
        # after 6 consecutive failures (= ~1 minute of unresponsiveness).
        if hasattr(socket, "TCP_KEEPIDLE"):
            conn.setsockopt(socket.IPPROTO_TCP, socket.TCP_KEEPIDLE, 60)
        if hasattr(socket, "TCP_KEEPINTVL"):
            conn.setsockopt(socket.IPPROTO_TCP, socket.TCP_KEEPINTVL, 10)
        if hasattr(socket, "TCP_KEEPCNT"):
            conn.setsockopt(socket.IPPROTO_TCP, socket.TCP_KEEPCNT, 6)
        return conn, addr

    @property
    def endpoint_url(self) -> str:
        host, port = self.server_address[0], self.server_address[1]
        return f"http://{host}:{port}/{self.config.path}"


def build_server(
    config: ServerConfig | None = None,
    registry: ToolRegistry | None = None,
    *,
    register_builtin: bool = True,
    enable_control: bool = True,
) -> McpHTTPServer:
    """Construct (but do not start) an :class:`McpHTTPServer`.

    If no *registry* is supplied a fresh one is created; unless
    ``register_builtin`` is false the built-in example tools are registered.
    """
    logger.debug("Aquiring config")
    config = config or ServerConfig()

    logger.debug("Reading profiles")
    profiles = ProfileRegistry(list(DEFAULT_PROFILES))


    logger.debug("Initialising Tool-Registry")
    if registry is None:
        registry = ToolRegistry()
        if register_builtin:
            from .tools import register_builtin_tools
            from .tools.agent import register_agent_tools

            register_builtin_tools(registry)
            register_agent_tools(registry, profiles)

    logger.debug("Initialising Session-Store")
    sessions = SessionStore()
    logger.debug("Initialising CLI-Manager")
    cli_manager = CliSessionManager(
        log_dir=config.cli_log_dir,
        ttl_seconds=config.agent_session_ttl_seconds,
        response_timeout=config.agent_response_timeout_seconds,
    )
    control_manager: ToolControlManager | None = None
    if enable_control:
        logger.debug("Initialising Tool-Control-Manager")
        control_manager = ToolControlManager(
            timeout=config.agent_response_timeout_seconds,
        )
    services = AppServices(
        config=config,
        registry=registry,
        sessions=sessions,
        cli_manager=cli_manager,
        profiles=profiles,
        control_manager=control_manager,
    )
    protocol = McpProtocol(config, registry, services)
    logger.debug("Initialising Communikation-Log")
    comm_log = CommunicationLog(config.log_dir)
    return McpHTTPServer(config, protocol, sessions, comm_log, services)


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
        server.services.cli_manager.shutdown()
        server.shutdown()
        server.server_close()
