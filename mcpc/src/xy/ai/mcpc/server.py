"""Assembling and running the MCP Controller HTTP server."""

from __future__ import annotations

import logging
from http.server import ThreadingHTTPServer
from typing import Any

from .cli import CliSessionManager
from .config import ServerConfig
from .context import AppServices
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

    @property
    def endpoint_url(self) -> str:
        host, port = self.server_address[0], self.server_address[1]
        return f"http://{host}:{port}/{self.config.path}"


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
    logger.info("Aquiring config")
    config = config or ServerConfig()

    logger.info("Reading profiles")
    profiles = ProfileRegistry(list(DEFAULT_PROFILES))


    logger.info("Initialising Tool-Registry")
    if registry is None:
        registry = ToolRegistry()
        if register_builtin:
            from .tools import register_builtin_tools
            from .tools.agent import register_agent_tools

            register_builtin_tools(registry)
            register_agent_tools(registry, profiles)

    logger.info("Initialising Session-Store")
    sessions = SessionStore()
    logger.info("Initialising CLI-Manager")
    cli_manager = CliSessionManager(
        log_dir=config.cli_log_dir,
        ttl_seconds=config.agent_session_ttl_seconds,
        response_timeout=config.agent_response_timeout_seconds,
    )
    services = AppServices(
        config=config,
        registry=registry,
        sessions=sessions,
        cli_manager=cli_manager,
        profiles=profiles,
    )
    protocol = McpProtocol(config, registry, services)
    logger.info("Initialising Communikation-Log")
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
