"""An MCPC server running on a background thread inside the current process."""
from __future__ import annotations
import logging
import threading
import uuid
from collections.abc import Callable, Iterable, Sequence
from concurrent.futures import ThreadPoolExecutor
from typing import Any
from urllib.parse import urlparse
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.embedded.agent import Agent, CacheMode, CliResult, CliSessionError, Effort, Model, agent_parameters
from xy.ai.mcpc.embedded.tools import FunctionTool, ToolHandlerFn, ToolProvider, register_tool
from xy.ai.mcpc.server.server import McpHTTPServer, build_server, build_ws_server
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry
__all__ = ['EmbeddedMcpc', 'launch_agent']
logger = logging.getLogger('xy.ai.mcpc.embedded')

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
        register_tool(tool, self._server.environment.registry, self._server.environment)

    def tool(self, name: str, description: str, input_schema: dict[str, Any] | None=None, **kwargs: Any) -> Callable[[ToolHandlerFn], ToolHandlerFn]:
        """Decorator registering a function ``(ctx) -> result`` as a tool."""
        schema = input_schema if input_schema is not None else {'type': 'object', 'properties': {}}

        def decorator(fn: ToolHandlerFn) -> ToolHandlerFn:
            self.add_tool(FunctionTool(name, description, schema, fn, **kwargs))
            return fn
        return decorator

    def _register_all(self, registry: ToolRegistry, environment: Any) -> None:
        for tool in self._tools:
            register_tool(tool, registry, environment)
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
    def running(self) -> bool:
        return self._server is not None

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

    def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None, cc_profile: str='none') -> str:
        """Create a session with its own toolset (default: all registered tools).

        Sessions configured here are never overridden by the client's tools header.
        """
        sid = session_id or str(uuid.uuid4())
        names = {'all'} if tools is None else set(tools)
        session = self.server.sessions.precreate(sid, enabled_tools=names, cc_profile=cc_profile)
        session.state['tools_preset'] = True
        return sid

    def close_session(self, session_id: str) -> bool:
        return self.server.sessions.remove(session_id)

    def headers(self, session_id: str) -> dict[str, str]:
        """HTTP headers an agent must send to use *session_id*."""
        return {self.config.session_header: session_id}

    @property
    def agent_config(self) -> ServerConfig:
        """The configuration with the actually bound ports (relevant when configured with port 0)."""
        ws_url = self.ws_url
        if ws_url is None:
            raise RuntimeError('Agents require the WebSocket transport (ws_enabled=True and `websockets` installed)')
        return self.config.with_overrides(port=self.server.server_address[1], ws_port=urlparse(ws_url).port)

    def start_agent(self, system_prompt: str, *, model: Model | str=Model.SONNET, effort: Effort | str=Effort.MEDIUM, tools: Iterable[str] | None=None, cc_profile: str='none', cache_mode: CacheMode | str=CacheMode.DEFAULT, executable: str='claude', extra_env: dict[str, str] | None=None, agent_id: str | None=None) -> Agent:
        """Start an agent bound to this server with its own session and toolset (*tools*, default: all).

        Any number of agents may share one instance; the CLI process starts lazily on the first prompt.
        """
        sid = self.open_session(tools=tools, session_id=agent_id, cc_profile=cc_profile)
        try:
            parameters = agent_parameters(
                self.agent_config,
                sid,
                system_prompt,
                model=model,
                effort=effort,
                cc_profile=cc_profile,
                cache_mode=cache_mode,
                executable=executable,
                extra_env=extra_env)
            cli = self.server.environment.cli_manager.request(parameters=parameters, session_id=sid)
        except Exception:
            self.close_session(sid)
            raise
        return Agent(self, cli, sid)

    def run_batch(self, prompts: Sequence[str], system_prompt: str, *, max_workers: int=4, **agent_options: Any) -> list[CliResult]:
        """Run each prompt on a fresh agent (parallel, sharing this server); results keep the prompt order.

        *agent_options* are passed to :meth:`start_agent`. Failures are returned as error results.
        """

        def run(prompt: str) -> CliResult:
            try:
                with self.start_agent(system_prompt, **agent_options) as agent:
                    return agent.prompt(prompt)
            except CliSessionError as exc:
                return CliResult(text=str(exc), is_error=True, subtype='session_error')
        with ThreadPoolExecutor(max_workers=max_workers, thread_name_prefix='mcpc-batch') as pool:
            return list(pool.map(run, prompts))

def launch_agent(system_prompt: str, *, builtin_tools: bool | Iterable[str]=True, tools: Iterable[ToolDefinition | ToolProvider]=(), config: ServerConfig | None=None, auto_approve: bool=True, **agent_options: Any) -> Agent:
    """Start an agent with a dedicated, specially configured MCPC instance that it owns.

    *agent_options* are passed to :meth:`EmbeddedMcpc.start_agent`; closing the agent stops its server.
    """
    mcpc = EmbeddedMcpc(config, builtin_tools=builtin_tools, tools=tools, auto_approve=auto_approve).start()
    try:
        agent = mcpc.start_agent(system_prompt, **agent_options)
    except Exception:
        mcpc.stop()
        raise
    agent.owns_mcpc = True
    return agent