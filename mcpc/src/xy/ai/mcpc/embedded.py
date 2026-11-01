"""Run an MCPC server inline from a Python script.

Typical batch usage::

    class MyTools(ToolProvider):
        def tools(self, environment):
            return [FunctionTool("lookup", "Look up a record", {"type": "object", "properties": {}}, lambda ctx: "ok")]

    with EmbeddedMcpc(builtin_tools=["read", "bash"], tools=[MyTools()], auto_approve=True) as mcpc:
        sid = mcpc.open_session(tools={"lookup", "read"})
        # point the agent at mcpc.url with header ``mcpc.headers(sid)``

One instance can be shared by many parallel agents: every agent uses its own
session id (``open_session``) and therefore its own toolset.
"""
from __future__ import annotations
import logging
import threading
import uuid
from abc import ABC, abstractmethod
from collections.abc import Callable, Iterable
from typing import TYPE_CHECKING, Any
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.server.server import McpHTTPServer, build_server, build_ws_server
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
if TYPE_CHECKING:
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
__all__ = ['EmbeddedMcpc', 'FunctionTool', 'ToolProvider']
logger = logging.getLogger('xy.ai.mcpc.embedded')
ToolHandlerFn = Callable[['ToolContext'], 'ToolResult | str | dict[str, Any]']

class ToolProvider(ABC):
    """Subclass and implement :meth:`tools` to contribute custom tools to a server."""

    @abstractmethod
    def tools(self, environment: AppEnvironment) -> Iterable[ToolDefinition]:
        """Return the tools to register. *environment* gives access to config, sessions, etc."""

class FunctionTool(ToolDefinition):
    """A tool backed by a plain callable taking a ``ToolContext``."""

    def __init__(self, name: str, description: str, input_schema: dict[str, Any], handler: ToolHandlerFn, *, title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> None:
        self.name = name
        self.description = description
        self.input_schema = input_schema
        self.title = title
        self.output_schema = output_schema if output_schema is not None else {'type': 'object'}
        self.annotations = annotations
        self._handler = handler

    def handle(self, ctx: ToolContext) -> ToolResult | str | dict[str, Any]:
        return self._handler(ctx)

class EmbeddedMcpc:
    """An MCPC server running on a background thread inside the current process.

    :param builtin_tools: ``True`` for all built-in tools, ``False`` for none, or
        an iterable of tool/alias names to keep.
    :param tools: extra :class:`ToolDefinition` instances or :class:`ToolProvider`\\ s.
    :param auto_approve: approve every tool request and result without a controller.
    :param ignore_tools_header: ignore the client's tools header; sessions get all
        registered tools unless configured via :meth:`open_session`.
    """

    def __init__(self, config: ServerConfig | None=None, *, builtin_tools: bool | Iterable[str]=True, tools: Iterable[ToolDefinition | ToolProvider]=(), auto_approve: bool | None=None, ignore_tools_header: bool | None=None, enable_control: bool=True) -> None:
        if config is None:
            config = ServerConfig(host='127.0.0.1', port=0, ws_enabled=True, ws_port=0)
        overrides = {}
        if auto_approve is not None:
            overrides['auto_approve'] = auto_approve
        if ignore_tools_header is not None:
            overrides['ignore_tools_header'] = ignore_tools_header
        self.config = config.with_overrides(**overrides) if overrides else config
        self._builtin_tools = builtin_tools if isinstance(builtin_tools, bool) else frozenset(builtin_tools)
        self._tools: list[ToolDefinition | ToolProvider] = list(tools)
        self._enable_control = enable_control
        self._server: McpHTTPServer | None = None
        self._ws_server = None
        self._thread: threading.Thread | None = None

    def add_tool(self, tool: ToolDefinition | ToolProvider) -> None:
        """Register a tool or provider; before :meth:`start` it is queued, afterwards registered live."""
        if self._server is None:
            self._tools.append(tool)
            return
        self._register(tool, self._server.environment.registry, self._server.environment)

    def tool(self, name: str, description: str, input_schema: dict[str, Any] | None=None, **kwargs: Any) -> Callable[[ToolHandlerFn], ToolHandlerFn]:
        """Decorator registering a function ``(ctx) -> result`` as a tool."""
        schema = input_schema if input_schema is not None else {'type': 'object', 'properties': {}}

        def decorator(fn: ToolHandlerFn) -> ToolHandlerFn:
            self.add_tool(FunctionTool(name, description, schema, fn, **kwargs))
            return fn
        return decorator

    @staticmethod
    def _register(tool: ToolDefinition | ToolProvider, registry: ToolRegistry, environment: AppEnvironment) -> None:
        if isinstance(tool, ToolProvider):
            for definition in tool.tools(environment):
                registry.register(definition)
        else:
            registry.register(tool)

    def _register_all(self, registry: ToolRegistry, environment: AppEnvironment) -> None:
        for tool in self._tools:
            self._register(tool, registry, environment)
        self._tools.clear()

    def start(self) -> EmbeddedMcpc:
        if self._server is not None:
            raise RuntimeError('Server already started')
        server = build_server(
            self.config,
            enable_control=self._enable_control,
            builtin_tools=self._builtin_tools,
            extra_tools=self._register_all)
        self._server = server
        self._ws_server = build_ws_server(server)
        if self._ws_server is not None:
            self._ws_server.start()
        self._thread = threading.Thread(
            target=server.serve_forever,
            kwargs={
                'poll_interval': 0.1},
            name='embedded-mcpc',
            daemon=True)
        self._thread.start()
        logger.info('Embedded MCPC listening on %s', server.endpoint_url)
        return self

    def stop(self) -> None:
        server = self._server
        if server is None:
            return
        self._server = None
        if self._ws_server is not None:
            self._ws_server.stop()
            self._ws_server = None
        server.environment.cli_manager.shutdown()
        server.shutdown()
        server.server_close()
        if self._thread is not None:
            self._thread.join(timeout=5)
            self._thread = None

    def __enter__(self) -> EmbeddedMcpc:
        return self.start()

    def __exit__(self, *exc_info: object) -> None:
        self.stop()

    @property
    def server(self) -> McpHTTPServer:
        if self._server is None:
            raise RuntimeError('Server not started')
        return self._server

    @property
    def url(self) -> str:
        return self.server.endpoint_url

    @property
    def ws_url(self) -> str | None:
        return self._ws_server.endpoint_url if self._ws_server is not None else None

    @property
    def tool_names(self) -> list[str]:
        return self.server.environment.registry.names()

    def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None) -> str:
        """Create a session with its own toolset (default: all registered tools).

        Sessions configured here are never overridden by the client's tools header.
        """
        sid = session_id or str(uuid.uuid4())
        names = {'all'} if tools is None else set(tools)
        session = self.server.sessions.precreate(sid, enabled_tools=names, cc_profile='none')
        session.state['tools_preset'] = True
        return sid

    def close_session(self, session_id: str) -> bool:
        return self.server.sessions.remove(session_id)

    def headers(self, session_id: str) -> dict[str, str]:
        """HTTP headers an agent must send to use *session_id*."""
        return {self.config.session_header: session_id}