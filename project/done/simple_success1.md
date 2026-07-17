In `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py` wird ein Simple "success" automatisch approved.
Ändere die Implementierung.
Das Autoapprove soll nicht mehr an die Struktur gebunden sein, sondern ein Flag in Request oder Response aktivieren das Auto-Approve von Seiten des Tools aus. Das Tool bestimmt nun welche Anforderungen ein Approval benötigen.

Durchsuche danach die Tools in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools` auf das Success-Pattern und migriere das Flag.

Führe das Flag auch im Read-Tool in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py` hinzu für die Rückgabe die eine volle Datei uneingeschränkt zurückgibt.
Control Request:
```yaml
id: c3c4082c-b42a-4096-9a42-cd0819b4632f
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
```
/answer c3c4082c-b42a-4096-9a42-cd0819b4632f allow
Control Request:
```yaml
id: e283f38e-c8b8-4f87-80f3-d80fa9de6b2c
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
```
/answer e283f38e-c8b8-4f87-80f3-d80fa9de6b2c allow
Control Request:
```yaml
id: 2ec770c7-d7c1-41d8-9afe-c7d412a1f5a2
result:
  structuredContent:
    error: Not a regular file.
  isError: true
```
/answer 2ec770c7-d7c1-41d8-9afe-c7d412a1f5a2 allow
Control Request:
```yaml
id: fd25773b-253e-4ae4-ae84-c5f9f5c66789
result:
  structuredContent:
    content: |
      """Human-in-the-loop tool control manager.

      Implements the Manager Pattern for intercepting tool calls at two points:

      1. *Before* execution (``phase="request"``): the interceptor may approve,
         modify arguments, or reject the call entirely.
      2. *After* execution (``phase="result"``): the interceptor may approve,
         replace the tool output, or inject instructions as the result.

      The control endpoint at ``/control/tool`` lets an external client poll for
      pending items and post approval decisions.  The intercepting threads block on
      per-item ``threading.Event`` objects until a decision arrives.
      """

      from __future__ import annotations

      import logging
      import threading
      import uuid
      from dataclasses import dataclass, field
      from typing import Any
      from xy.ai.mcpc.session import Session

      logger = logging.getLogger("xy.ai.mcpc.control")

      # How long (seconds) an intercepted call waits before timing out and auto-approving.
      _DEFAULT_TIMEOUT = 24 * 60 * 60.0  # 24 h — matches agent MCP timeout


      # ---------------------------------------------------------------------------
      # Decision DTO
      # ---------------------------------------------------------------------------

      @dataclass(slots=True)
      class ControlDecision:
          """The outcome of a human review, produced by :meth:`ToolControlManager.process_approvals`."""

          approved: bool
          """True when the call should proceed (possibly with modified data)."""

          rejection_reason: str | None = None
          """Human-readable hint for the agent when ``approved`` is False."""

          modified_arguments: dict[str, Any] | None = None
          """Replacement arguments for the ``request`` phase (``None`` → keep originals)."""

          modified_result: dict[str, Any] | None = None
          """Replacement result dict for the ``result`` phase (``None`` → keep original)."""


      # ---------------------------------------------------------------------------
      # Internal pending item
      # ---------------------------------------------------------------------------

      @dataclass
      class _PendingItem:
          id: str
          phase: str          # "request" | "result"
          tool_name: str
          arguments: dict[str, Any] | None       # populated in request phase
          result: dict[str, Any] | None          # populated in result phase
          _event: threading.Event = field(default_factory=threading.Event, repr=False)
          _decision: ControlDecision | None = field(default=None, repr=False)

          def to_dict(self) -> dict[str, Any]:
              item: dict[str, Any] = {"id": self.id}
              if self.arguments is not None:
                  item["toolName"] = self.tool_name
                  item["arguments"] = self.arguments
              if self.result is not None:
                  item["result"] = self.result
              return item


      # ---------------------------------------------------------------------------
      # Manager
      # ---------------------------------------------------------------------------

      class ToolControlManager:
          """Thread-safe manager for human-in-the-loop tool interception.

          Usage::

              manager = ToolControlManager()

              # In the interceptor thread (blocks until a decision is received):
              decision = manager.submit_request("bash", {"command": "rm -rf /"})

              # In the control endpoint handler (non-blocking):
              pending = manager.get_pending()
              manager.process_approvals([{"id": "…", "approved": True}])
          """

          def __init__(self, timeout: float = _DEFAULT_TIMEOUT) -> None:
              self._timeout = timeout
              self._pending: dict[str, _PendingItem] = {}
              self._lock = threading.Lock()

          # ------------------------------------------------------------------
          # Interceptor-facing API (blocking)
          # ------------------------------------------------------------------

          def submit_request(
              self,
              session: Session,
              tool_name: str,
              arguments: dict[str, Any],
          ) -> ControlDecision:
              """Block until the controller approves/rejects a tool-call *request*.

              Returns a :class:`ControlDecision`.  If the decision includes
              ``modified_arguments``, the caller should substitute them before
              invoking the tool handler.
              """
              item = self._enqueue(session, "request", tool_name, arguments=arguments, result=None)
              return self._wait(item)

          def submit_result(
              self,
              session: Session,
              tool_name: str,
              result: dict[str, Any],
          ) -> ControlDecision:
              """Block until the controller approves/replaces a tool-call *result*.

              Returns a :class:`ControlDecision`.  If the decision includes
              ``modified_result``, the caller should use that instead of the
              original result.

              Simple "success" results (empty ``content`` and a
              ``structuredContent`` of exactly ``{"result": "success"}``) are
              auto-approved without involving the controller, since there is
              nothing meaningful for a human to review.
              """
              if self._is_simple_success_result(result):
                  logger.info(
                      "Auto-approving simple success result for %s [%s]",
                      tool_name, session.id,
                  )
                  return ControlDecision(approved=True)

              item = self._enqueue(session, "result", tool_name, arguments=None, result=result)
              return self._wait(item)

          # ------------------------------------------------------------------
          # Control-endpoint-facing API (non-blocking)
          # ------------------------------------------------------------------

          def get_pending(self) -> list[dict[str, Any]]:
              """Return serialisable snapshots of all items still awaiting a decision."""
              with self._lock:
                  return [item.to_dict() for item in self._pending.values()]

          def process_approvals(self, approvals: list[dict[str, Any]]) -> None:
              """Apply a batch of approval decisions from the control client.

              Each entry in *approvals* must have at minimum an ``"id"`` key.
              Accepted forms:

              * ``{"id": "…"}`` — simple approval, keep original data.
              * ``{"id": "…", "rejected": true, "reason": "…"}`` — rejection.
              * ``{"id": "…", "arguments": {…}}`` — approve with modified arguments.
              * ``{"id": "…", "result": {…}}`` — approve with replaced result.
              """
              for approval in approvals:
                  item_id = approval.get("id")
                  if not isinstance(item_id, str):
                      logger.warning("Approval entry missing 'id', skipped: %s", approval)
                      continue
                  with self._lock:
                      item = self._pending.get(item_id)
                  if item is None:
                      logger.warning("Unknown approval id %s, skipped", item_id)
                      continue

                  if approval.get("rejected"):
                      decision = ControlDecision(
                          approved=False,
                          rejection_reason=approval.get("reason") or "Rejected by controller",
                      )
                  else:
                      decision = ControlDecision(
                          approved=True,
                          modified_arguments=approval.get("arguments"),
                          modified_result=approval.get("result"),
                      )

                  item._decision = decision
                  with self._lock:
                      self._pending.pop(item_id, None)
                  logger.info(
                      "Dequeued control item %s [%s/%s]: approved=%s",
                      item.tool_name, item.phase, item_id, decision.approved,
                  )
                  item._event.set()

          # ------------------------------------------------------------------
          # Internal helpers
          # ------------------------------------------------------------------

          @staticmethod
          def _is_simple_success_result(result: dict[str, Any]) -> bool:
              """Return ``True`` for a plain success result with nothing to review.

              Matches results of the shape::

                  {
                      "content": [],
                      "structuredContent": {"result": "success"}
                  }

              Any additional content items, extra keys, or a differing
              ``structuredContent`` payload disqualify the result from
              auto-approval.
              """
              if not isinstance(result, dict):
                  return False
              if set(result.keys()) - {"content", "structuredContent"}:
                  return False
              if result.get("content") not in ([], None):
                  return False
              return result.get("structuredContent") == {"result": "success"}

          def _enqueue(
              self,
              session: Session,
              phase: str,
              tool_name: str,
              arguments: dict[str, Any] | None,
              result: dict[str, Any] | None,
          ) -> _PendingItem:
              item_id = str(uuid.uuid4())
              item = _PendingItem(
                  id=item_id,
                  phase=phase,
                  tool_name=tool_name,
                  arguments=arguments,
                  result=result,
              )
              with self._lock:
                  self._pending[item_id] = item
              logger.info("Enqueued control item %s [%s/%s/%s]", tool_name, phase, session.id, item_id)
              return item

          def _wait(self, item: _PendingItem) -> ControlDecision:
              signalled = item._event.wait(timeout=self._timeout)
              with self._lock:
                  self._pending.pop(item.id, None)

              if not signalled or item._decision is None:
                  # Timeout — auto-approve to avoid hanging the agent forever.
                  logger.warning(
                      "Control item %s [%s/%s] timed out, auto-approving",
                      item.tool_name, item.phase, item.id,
                  )
                  return ControlDecision(approved=True)

              return item._decision
```
/answer fd25773b-253e-4ae4-ae84-c5f9f5c66789 allow
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=f6bb7d0c-e5f2-41d8-9746-9abe85a55169 | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: b35b530c-82a1-4dbf-bb6e-83514bcafccc
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/
```
/answer b35b530c-82a1-4dbf-bb6e-83514bcafccc allow
Control Request:
```yaml
id: 90d49362-6bdc-47a4-8e0d-8ee70d738477
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: grep -rn "submit_request\|submit_result\|_is_simple_success_result\|structuredContent" src/xy/ai/mcpc --include=*.py | grep -v "/tools/"
```
/answer 90d49362-6bdc-47a4-8e0d-8ee70d738477 allow
Control Request:
```yaml
id: c8f1915b-add9-440e-9671-c0daeaa9e621
result:
  structuredContent:
    error: Not a regular file.
  isError: true
```
/answer c8f1915b-add9-440e-9671-c0daeaa9e621 allow
Control Request:
```yaml
id: 1ae506db-1f0c-4b66-b105-2c56424d622d
result:
  structuredContent:
    stdout: |
      src/xy/ai/mcpc/registry.py:44:            result["structuredContent"] = self.structured_content
      src/xy/ai/mcpc/registry.py:103:                structured_content=value.get("structuredContent"),
      src/xy/ai/mcpc/control/manager.py:87:        decision = manager.submit_request("bash", {"command": "rm -rf /"})
      src/xy/ai/mcpc/control/manager.py:103:    def submit_request(
      src/xy/ai/mcpc/control/manager.py:118:    def submit_result(
      src/xy/ai/mcpc/control/manager.py:131:        ``structuredContent`` of exactly ``{"result": "success"}``) are
      src/xy/ai/mcpc/control/manager.py:135:        if self._is_simple_success_result(result):
      src/xy/ai/mcpc/control/manager.py:202:    def _is_simple_success_result(result: dict[str, Any]) -> bool:
      src/xy/ai/mcpc/control/manager.py:209:                "structuredContent": {"result": "success"}
      src/xy/ai/mcpc/control/manager.py:213:        ``structuredContent`` payload disqualify the result from
      src/xy/ai/mcpc/control/manager.py:218:        if set(result.keys()) - {"content", "structuredContent"}:
      src/xy/ai/mcpc/control/manager.py:222:        return result.get("structuredContent") == {"result": "success"}
      src/xy/ai/mcpc/protocol.py:180:            decision = control.submit_request(session, name, arguments)
      src/xy/ai/mcpc/protocol.py:211:            decision = control.submit_result(session, name, result.to_dict())
```
/answer 1ae506db-1f0c-4b66-b105-2c56424d622d allow
Control Request:
```yaml
id: 83346f92-0ee4-44da-8cfe-ab586347c22e
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server.py
```
/answer 83346f92-0ee4-44da-8cfe-ab586347c22e allow
Control Request:
```yaml
id: fd9e221a-02ef-46b0-8e42-25b6b6cd95e9
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
```
/answer fd9e221a-02ef-46b0-8e42-25b6b6cd95e9 allow
Control Request:
```yaml
id: 6225bbd6-4ff0-41e2-b391-d7343336c252
result:
  structuredContent:
    content: |
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
      from .ws_transport import WebSocketMcpServer

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


      def build_ws_server(server: McpHTTPServer) -> WebSocketMcpServer | None:
          """Build the WebSocket transport sharing *server*'s component graph.

          Returns ``None`` (after logging a warning) when the transport is disabled
          via configuration, or when the optional ``websockets`` dependency is not
          installed — the HTTP transport keeps working either way.
          """
          if not server.config.ws_enabled:
              logger.info("WebSocket transport disabled (ws_enabled=False)")
              return None
          try:
              return WebSocketMcpServer(
                  server.config, server.protocol, server.sessions, server.comm_log, server.services
              )
          except RuntimeError as exc:
              logger.warning("WebSocket transport unavailable: %s", exc)
              return None


      def run(config: ServerConfig | None = None, **build_kwargs: Any) -> None:
          """Build a server from *config* and serve until interrupted."""
          server = build_server(config, **build_kwargs)
          logger.info("MCP Controller listening on %s", server.endpoint_url)
          logger.info("Session header: %s | log dir: %s",
                      server.config.session_header, server.comm_log.directory)

          ws_server = build_ws_server(server)
          if ws_server is not None:
              ws_server.start()
              logger.info("MCP Controller (WebSocket) listening on %s", ws_server.endpoint_url)

          try:
              server.serve_forever()
          except KeyboardInterrupt:  # pragma: no cover - interactive
              logger.info("Shutting down")
          finally:
              if ws_server is not None:
                  ws_server.stop()
              server.services.cli_manager.shutdown()
              server.shutdown()
              server.server_close()
```
/answer 6225bbd6-4ff0-41e2-b391-d7343336c252 allow
Control Request:
```yaml
id: baef76a8-8dbf-444d-b543-faade630a147
result:
  structuredContent:
    content: |
      """Central tool registry and tool result helpers.

      Tools are registered once in a process-wide :class:`ToolRegistry`.  What a
      given client actually sees is derived by reconciling the registry against the
      per-session configuration (:attr:`Session.enabled_tools`).
      """

      from __future__ import annotations

      from dataclasses import dataclass, field
      from typing import TYPE_CHECKING, Any, Callable

      from .codec import JsonCodec
      from .session import Session

      if TYPE_CHECKING:
          from .context import AppServices


      @dataclass(slots=True)
      class ToolContext:
          """Context handed to a tool handler on invocation."""

          session: Session
          arguments: dict[str, Any]
          #: Shared process-wide services (session store, CLI manager, profiles).
          #: ``None`` for tools that never orchestrate other sessions.
          services: "AppServices | None" = None


      @dataclass(slots=True)
      class ToolResult:
          """The result of a tool call (maps onto MCP ``CallToolResult``)."""

          content: list[dict[str, Any]] = field(default_factory=list)
          structured_content: dict[str, Any] | None = None
          is_error: bool = False

          def to_dict(self) -> dict[str, Any]:
              result: dict[str, Any] = {}
              if self.content:
                  result["content"] = list(self.content)
              if self.structured_content:
                  result["structuredContent"] = self.structured_content
              if self.is_error:
                  result["isError"] = True
              return result


      def text_content(text: str) -> dict[str, Any]:
          """Build a ``TextContent`` block."""
          return {"type": "text", "text": text}


      #: A handler receives the invocation context and returns one of:
      #: * a :class:`ToolResult`,
      #: * a ``str`` (wrapped as a single text content block),
      #: * a ``dict`` (treated as structured content, also rendered as JSON text).
      ToolHandler = Callable[[ToolContext], "ToolResult | str | dict[str, Any]"]


      @dataclass(slots=True)
      class Tool:
          """A registered tool and its MCP metadata."""

          name: str
          description: str
          input_schema: dict[str, Any]
          handler: ToolHandler
          title: str | None = None
          output_schema: dict[str, Any] | None = None
          annotations: dict[str, Any] | None = None

          def to_spec(self) -> dict[str, Any]:
              """Return the MCP ``Tool`` object advertised via ``tools/list``."""
              spec: dict[str, Any] = {
                  "name": self.name,
                  "description": self.description,
                  "inputSchema": self.input_schema,
              }
              if self.title is not None:
                  spec["title"] = self.title
              if self.output_schema is not None:
                  spec["outputSchema"] = self.output_schema
              if self.annotations is not None:
                  spec["annotations"] = self.annotations
              return spec


      def normalize_result(value: "ToolResult | str | dict[str, Any] | None") -> ToolResult:
          """Coerce whatever a handler returned into a :class:`ToolResult`."""
          if isinstance(value, ToolResult):
              return value
          if value is None:
              return ToolResult(content=[])
          if isinstance(value, str):
              return ToolResult(content=[text_content(value)])
          if isinstance(value, dict):
              # A dict already shaped like a CallToolResult is passed through.
              if "content" in value and isinstance(value["content"], list):
                  return ToolResult(
                      content=value["content"],
                      structured_content=value.get("structuredContent"),
                      is_error=bool(value.get("isError", False)),
                  )
              # Otherwise treat the dict as structured content.
              rendered = JsonCodec.encode(value)
              return ToolResult(content=[text_content(rendered)], structured_content=value)
          raise TypeError(f"Unsupported tool return type: {type(value)!r}")


      class ToolRegistry:
          """Process-wide registry of available tools."""

          def __init__(self) -> None:
              self._tools: dict[str, Tool] = {}

          def register(self, tool: Tool) -> Tool:
              if tool.name in self._tools:
                  raise ValueError(f"Tool already registered: {tool.name}")
              self._tools[tool.name] = tool
              return tool

          def tool(
              self,
              name: str,
              *,
              description: str,
              input_schema: dict[str, Any],
              title: str | None = None,
              output_schema: dict[str, Any] | None = None,
              annotations: dict[str, Any] | None = None,
          ) -> Callable[[ToolHandler], ToolHandler]:
              """Decorator registering the decorated function as a tool handler."""

              def decorator(handler: ToolHandler) -> ToolHandler:
                  self.register(
                      Tool(
                          name=name,
                          description=description,
                          input_schema=input_schema,
                          handler=handler,
                          title=title,
                          output_schema=output_schema,
                          annotations=annotations,
                      )
                  )
                  return handler

              return decorator

          def get(self, name: str) -> Tool | None:
              return self._tools.get(name)

          def __contains__(self, name: object) -> bool:
              return name in self._tools

          def __len__(self) -> int:
              return len(self._tools)

          def names(self) -> list[str]:
              return list(self._tools)

          def list_for_session(self, session: Session) -> list[Tool]:
              """Reconcile the registry with the session's enabled-tool configuration.

              Returns the tools the session is allowed to see, sorted by name for a
              stable pagination order.
              """
              tools = [t for t in self._tools.values() if session.is_tool_enabled(t.name)]
              tools.sort(key=lambda t: t.name)
              return tools
```
/answer baef76a8-8dbf-444d-b543-faade630a147 allow
Control Request:
```yaml
id: 1f840c35-d9bb-4876-a064-6fc3435737af
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/session.py
```
/answer 1f840c35-d9bb-4876-a064-6fc3435737af allow
Control Request:
```yaml
id: 14a63cc0-cbbd-42bb-88c5-01956328d896
result:
  structuredContent:
    content: |
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

          #: Selects the ``CLAUDE_CONFIG_DIR`` (``~/.claude-<profile>``) so different
          #: agent profiles keep isolated credentials/caches.
          cc_profile: str = "none"

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
              logger.info("Enable tools for session %s: %s", self.id, names)
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
              cc_profile: str,
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
                  if cc_profile is not None:
                      session.cc_profile = cc_profile
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
```
/answer 14a63cc0-cbbd-42bb-88c5-01956328d896 allow
Control Request:
```yaml
id: 26c62e92-9860-4e49-ad22-7356636a7ef1
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
```
/answer 26c62e92-9860-4e49-ad22-7356636a7ef1 allow
Control Request:
```yaml
id: 004e9fac-3932-43d0-88d5-f6d4b79fae20
result:
  structuredContent:
    content: |
      """MCP protocol logic (lifecycle + tools feature).

      This module is transport-agnostic: it takes a parsed JSON-RPC request plus the
      owning :class:`Session` and returns a result payload (or raises
      :class:`JsonRpcError`).  The Streamable HTTP transport wraps the return value
      in a JSON-RPC envelope.
      """

      from __future__ import annotations

      import logging
      import base64
      from typing import TYPE_CHECKING, Any

      from . import errors
      from .config import ServerConfig
      from .jsonrpc import JsonRpcRequest
      from .registry import ToolContext, ToolRegistry, normalize_result
      from .session import Session

      logger = logging.getLogger("xy.ai.mcpc.protocol")

      if TYPE_CHECKING:
          from .context import AppServices

      # Methods a client may call before the initialize handshake has completed.
      _PRE_INIT_METHODS = {"initialize", "ping"}


      def _encode_cursor(offset: int) -> str:
          return base64.urlsafe_b64encode(str(offset).encode("ascii")).decode("ascii")


      def _decode_cursor(cursor: str) -> int:
          try:
              offset = int(base64.urlsafe_b64decode(cursor.encode("ascii")).decode("ascii"))
          except (ValueError, TypeError):
              raise errors.invalid_params("Invalid pagination cursor", {"cursor": cursor})
          if offset < 0:
              raise errors.invalid_params("Invalid pagination cursor", {"cursor": cursor})
          return offset


      class McpProtocol:
          """Dispatches MCP methods against a session."""

          def __init__(
              self,
              config: ServerConfig,
              registry: ToolRegistry,
              services: "AppServices | None" = None,
          ) -> None:
              self.config = config
              self.registry = registry
              self.services = services
              self._handlers = {
                  "initialize": self._handle_initialize,
                  "ping": self._handle_ping,
                  "tools/list": self._handle_tools_list,
                  "tools/call": self._handle_tools_call,
              }

          # -- Request handling ---------------------------------------------------
          def handle_request(
              self,
              session: Session,
              request: JsonRpcRequest,
              *,
              skip_control: bool = False,
          ) -> Any:
              """Handle a JSON-RPC *request* and return its ``result`` payload.

              ``skip_control`` suppresses tool interception for this request,
              regardless of whether a :class:`ToolControlManager` is configured.
              It is set when the caller sends ``X-MCPC-CONTROL: off``.
              """
              handler = self._handlers.get(request.method)
              if handler is None:
                  raise errors.method_not_found(request.method)

              if request.method not in _PRE_INIT_METHODS and not session.handshake_complete:
                  raise errors.JsonRpcError(
                      errors.NOT_INITIALIZED,
                      "Session is not initialized; send an 'initialize' request first",
                  )
              if request.method == "tools/call":
                  return self._handle_tools_call(session, request.params, skip_control=skip_control)
              return handler(session, request.params)

          def handle_notification(self, session: Session, request: JsonRpcRequest) -> None:
              """Handle a JSON-RPC *notification*.

              Notifications are not "supported" in the sense that the server never
              acts on arbitrary ones and never emits any; the lifecycle
              ``notifications/initialized`` is accepted to complete the handshake.
              """
              if request.method == "notifications/initialized":
                  with session.lock:
                      session.initialized = True
              # All other notifications are silently accepted and ignored.

          # -- Lifecycle ----------------------------------------------------------
          def _handle_initialize(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
              requested = params.get("protocolVersion")
              if not isinstance(requested, str):
                  raise errors.invalid_params('"protocolVersion" is required')

              if requested in self.config.supported_protocol_versions:
                  negotiated = requested
              else:
                  negotiated = self.config.preferred_protocol_version

              with session.lock:
                  session.protocol_version = negotiated
                  session.client_info = params.get("clientInfo")
                  session.client_capabilities = params.get("capabilities")
                  session.touch()

              return {
                  "protocolVersion": negotiated,
                  "capabilities": {
                      # Only the tools feature is offered; listChanged is false since
                      # notifications are unsupported.
                      "tools": {"listChanged": False},
                  },
                  "serverInfo": {
                      "name": self.config.server_name,
                      "title": self.config.server_title,
                      "version": self.config.server_version,
                  },
                  "instructions": self.config.instructions,
              }

          def _handle_ping(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
              return {}

          # -- Tools --------------------------------------------------------------
          def _handle_tools_list(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
              tools = self.registry.list_for_session(session)

              cursor = params.get("cursor")
              start = _decode_cursor(cursor) if cursor is not None else 0
              page_size = self.config.tools_page_size
              page = tools[start : start + page_size]

              result: dict[str, Any] = {"tools": [t.to_spec() for t in page]}
              if start + page_size < len(tools):
                  result["nextCursor"] = _encode_cursor(start + page_size)
              return result

          def _handle_tools_call(
              self,
              session: Session,
              params: dict[str, Any],
              *,
              skip_control: bool = False,
          ) -> dict[str, Any]:
              name = params.get("name")
              if not isinstance(name, str) or not name:
                  raise errors.invalid_params('"name" is required')

              arguments = params.get("arguments", {})
              if arguments is None:
                  arguments = {}
              if not isinstance(arguments, dict):
                  raise errors.invalid_params('"arguments" must be an object')

              tool = self.registry.get(name)
              # "Errors in finding the tool" are protocol errors (spec, tools/call).
              if tool is None or not session.is_tool_enabled(name):
                  raise errors.invalid_params(
                      f"Unknown or unavailable tool: {name}", {"name": name}
                  )

              _validate_arguments(tool.input_schema, arguments)

              # --- request interception -------------------------------------------
              control = self.services.control_manager if self.services else None
              if control is not None and not skip_control:
                  decision = control.submit_request(session, name, arguments)
                  if not decision.approved:
                      from .registry import ToolResult, text_content
                      reason = decision.rejection_reason or "Tool call rejected by controller"
                      return ToolResult(
                          content=[text_content(reason)],
                          is_error=True,
                      ).to_dict()
                  if decision.modified_arguments is not None:
                      arguments = decision.modified_arguments
              # --------------------------------------------------------------------

              context = ToolContext(session=session, arguments=arguments, services=self.services)
              # Tool execution errors are reported *inside* the result (isError=true)
              # so the model can see and self-correct, not as protocol errors.
              try:
                  with session.lock:
                      raw = tool.handler(context)
                  result = normalize_result(raw)
              except errors.JsonRpcError:
                  raise
              except Exception as exc:  # noqa: BLE001 - surface as tool error result
                  from .registry import ToolResult, text_content

                  result = ToolResult(
                      content=[text_content(f"Tool '{name}' failed: {exc}")],
                      is_error=True,
                  )

              # --- result interception --------------------------------------------
              if control is not None and not skip_control:
                  decision = control.submit_result(session, name, result.to_dict())
                  if not decision.approved:
                      from .registry import ToolResult, text_content
                      reason = decision.rejection_reason or "Tool result rejected by controller"
                      return ToolResult(
                          content=[text_content(reason)],
                          is_error=True,
                      ).to_dict()
                  if decision.modified_result is not None:
                      return decision.modified_result
              # --------------------------------------------------------------------

              return result.to_dict()


      def _validate_arguments(schema: dict[str, Any], arguments: dict[str, Any]) -> None:
          """Minimal validation of *arguments* against an input JSON Schema.

          Only the top-level ``required`` list and primitive ``type`` of declared
          properties are checked — enough to give clients meaningful ``INVALID_PARAMS``
          errors without pulling in a full JSON Schema implementation.
          """
          required = schema.get("required", [])
          missing = [key for key in required if key not in arguments]
          if missing:
              raise errors.invalid_params(
                  f"Missing required argument(s): {', '.join(missing)}",
                  {"missing": missing},
              )

          properties = schema.get("properties", {})
          type_checks = {
              "string": str,
              "number": (int, float),
              "integer": int,
              "boolean": bool,
              "object": dict,
              "array": list,
          }
          for key, value in arguments.items():
              prop = properties.get(key)
              if not isinstance(prop, dict):
                  continue
              expected = prop.get("type")
              py_type = type_checks.get(expected) if isinstance(expected, str) else None
              if py_type is None:
                  continue
              # bool is a subclass of int; guard the integer/number cases explicitly.
              if expected in ("number", "integer") and isinstance(value, bool):
                  ok = False
              else:
                  ok = isinstance(value, py_type)
              if not ok:
                  raise errors.invalid_params(
                      f"Argument '{key}' must be of type {expected}",
                      {"argument": key, "expectedType": expected},
                  )
```
/answer 004e9fac-3932-43d0-88d5-f6d4b79fae20 allow
ReasoningToken: 15
Control Request:
```yaml
id: 769b70f7-8057-42cb-ad0a-92e7233f1aab
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
```
/answer 769b70f7-8057-42cb-ad0a-92e7233f1aab allow
Control Request:
```yaml
id: a3188868-8a93-438a-93a1-061d0d6876e2
result:
  structuredContent:
    content: |
      """MCP protocol logic (lifecycle + tools feature).

      This module is transport-agnostic: it takes a parsed JSON-RPC request plus the
      owning :class:`Session` and returns a result payload (or raises
      :class:`JsonRpcError`).  The Streamable HTTP transport wraps the return value
      in a JSON-RPC envelope.
      """

      from __future__ import annotations

      import logging
      import base64
      from typing import TYPE_CHECKING, Any

      from . import errors
      from .config import ServerConfig
      from .jsonrpc import JsonRpcRequest
      from .registry import ToolContext, ToolRegistry, normalize_result
      from .session import Session

      logger = logging.getLogger("xy.ai.mcpc.protocol")

      if TYPE_CHECKING:
          from .context import AppServices

      # Methods a client may call before the initialize handshake has completed.
      _PRE_INIT_METHODS = {"initialize", "ping"}


      def _encode_cursor(offset: int) -> str:
          return base64.urlsafe_b64encode(str(offset).encode("ascii")).decode("ascii")


      def _decode_cursor(cursor: str) -> int:
          try:
              offset = int(base64.urlsafe_b64decode(cursor.encode("ascii")).decode("ascii"))
          except (ValueError, TypeError):
              raise errors.invalid_params("Invalid pagination cursor", {"cursor": cursor})
          if offset < 0:
              raise errors.invalid_params("Invalid pagination cursor", {"cursor": cursor})
          return offset


      class McpProtocol:
          """Dispatches MCP methods against a session."""

          def __init__(
              self,
              config: ServerConfig,
              registry: ToolRegistry,
              services: "AppServices | None" = None,
          ) -> None:
              self.config = config
              self.registry = registry
              self.services = services
              self._handlers = {
                  "initialize": self._handle_initialize,
                  "ping": self._handle_ping,
                  "tools/list": self._handle_tools_list,
                  "tools/call": self._handle_tools_call,
              }

          # -- Request handling ---------------------------------------------------
          def handle_request(
              self,
              session: Session,
              request: JsonRpcRequest,
              *,
              skip_control: bool = False,
          ) -> Any:
              """Handle a JSON-RPC *request* and return its ``result`` payload.

              ``skip_control`` suppresses tool interception for this request,
              regardless of whether a :class:`ToolControlManager` is configured.
              It is set when the caller sends ``X-MCPC-CONTROL: off``.
              """
              handler = self._handlers.get(request.method)
              if handler is None:
                  raise errors.method_not_found(request.method)

              if request.method not in _PRE_INIT_METHODS and not session.handshake_complete:
                  raise errors.JsonRpcError(
                      errors.NOT_INITIALIZED,
                      "Session is not initialized; send an 'initialize' request first",
                  )
              if request.method == "tools/call":
                  return self._handle_tools_call(session, request.params, skip_control=skip_control)
              return handler(session, request.params)

          def handle_notification(self, session: Session, request: JsonRpcRequest) -> None:
              """Handle a JSON-RPC *notification*.

              Notifications are not "supported" in the sense that the server never
              acts on arbitrary ones and never emits any; the lifecycle
              ``notifications/initialized`` is accepted to complete the handshake.
              """
              if request.method == "notifications/initialized":
                  with session.lock:
                      session.initialized = True
              # All other notifications are silently accepted and ignored.

          # -- Lifecycle ----------------------------------------------------------
          def _handle_initialize(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
              requested = params.get("protocolVersion")
              if not isinstance(requested, str):
                  raise errors.invalid_params('"protocolVersion" is required')

              if requested in self.config.supported_protocol_versions:
                  negotiated = requested
              else:
                  negotiated = self.config.preferred_protocol_version

              with session.lock:
                  session.protocol_version = negotiated
                  session.client_info = params.get("clientInfo")
                  session.client_capabilities = params.get("capabilities")
                  session.touch()

              return {
                  "protocolVersion": negotiated,
                  "capabilities": {
                      # Only the tools feature is offered; listChanged is false since
                      # notifications are unsupported.
                      "tools": {"listChanged": False},
                  },
                  "serverInfo": {
                      "name": self.config.server_name,
                      "title": self.config.server_title,
                      "version": self.config.server_version,
                  },
                  "instructions": self.config.instructions,
              }

          def _handle_ping(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
              return {}

          # -- Tools --------------------------------------------------------------
          def _handle_tools_list(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
              tools = self.registry.list_for_session(session)

              cursor = params.get("cursor")
              start = _decode_cursor(cursor) if cursor is not None else 0
              page_size = self.config.tools_page_size
              page = tools[start : start + page_size]

              result: dict[str, Any] = {"tools": [t.to_spec() for t in page]}
              if start + page_size < len(tools):
                  result["nextCursor"] = _encode_cursor(start + page_size)
              return result

          def _handle_tools_call(
              self,
              session: Session,
              params: dict[str, Any],
              *,
              skip_control: bool = False,
          ) -> dict[str, Any]:
              name = params.get("name")
              if not isinstance(name, str) or not name:
                  raise errors.invalid_params('"name" is required')

              arguments = params.get("arguments", {})
              if arguments is None:
                  arguments = {}
              if not isinstance(arguments, dict):
                  raise errors.invalid_params('"arguments" must be an object')

              tool = self.registry.get(name)
              # "Errors in finding the tool" are protocol errors (spec, tools/call).
              if tool is None or not session.is_tool_enabled(name):
                  raise errors.invalid_params(
                      f"Unknown or unavailable tool: {name}", {"name": name}
                  )

              _validate_arguments(tool.input_schema, arguments)

              # --- request interception -------------------------------------------
              control = self.services.control_manager if self.services else None
              if control is not None and not skip_control:
                  decision = control.submit_request(session, name, arguments)
                  if not decision.approved:
                      from .registry import ToolResult, text_content
                      reason = decision.rejection_reason or "Tool call rejected by controller"
                      return ToolResult(
                          content=[text_content(reason)],
                          is_error=True,
                      ).to_dict()
                  if decision.modified_arguments is not None:
                      arguments = decision.modified_arguments
              # --------------------------------------------------------------------

              context = ToolContext(session=session, arguments=arguments, services=self.services)
              # Tool execution errors are reported *inside* the result (isError=true)
              # so the model can see and self-correct, not as protocol errors.
              try:
                  with session.lock:
                      raw = tool.handler(context)
                  result = normalize_result(raw)
              except errors.JsonRpcError:
                  raise
              except Exception as exc:  # noqa: BLE001 - surface as tool error result
                  from .registry import ToolResult, text_content

                  result = ToolResult(
                      content=[text_content(f"Tool '{name}' failed: {exc}")],
                      is_error=True,
                  )

              # --- result interception --------------------------------------------
              if control is not None and not skip_control:
                  decision = control.submit_result(session, name, result.to_dict())
                  if not decision.approved:
                      from .registry import ToolResult, text_content
                      reason = decision.rejection_reason or "Tool result rejected by controller"
                      return ToolResult(
                          content=[text_content(reason)],
                          is_error=True,
                      ).to_dict()
                  if decision.modified_result is not None:
                      return decision.modified_result
              # --------------------------------------------------------------------

              return result.to_dict()


      def _validate_arguments(schema: dict[str, Any], arguments: dict[str, Any]) -> None:
          """Minimal validation of *arguments* against an input JSON Schema.

          Only the top-level ``required`` list and primitive ``type`` of declared
          properties are checked — enough to give clients meaningful ``INVALID_PARAMS``
          errors without pulling in a full JSON Schema implementation.
          """
          required = schema.get("required", [])
          missing = [key for key in required if key not in arguments]
          if missing:
              raise errors.invalid_params(
                  f"Missing required argument(s): {', '.join(missing)}",
                  {"missing": missing},
              )

          properties = schema.get("properties", {})
          type_checks = {
              "string": str,
              "number": (int, float),
              "integer": int,
              "boolean": bool,
              "object": dict,
              "array": list,
          }
          for key, value in arguments.items():
              prop = properties.get(key)
              if not isinstance(prop, dict):
                  continue
              expected = prop.get("type")
              py_type = type_checks.get(expected) if isinstance(expected, str) else None
              if py_type is None:
                  continue
              # bool is a subclass of int; guard the integer/number cases explicitly.
              if expected in ("number", "integer") and isinstance(value, bool):
                  ok = False
              else:
                  ok = isinstance(value, py_type)
              if not ok:
                  raise errors.invalid_params(
                      f"Argument '{key}' must be of type {expected}",
                      {"argument": key, "expectedType": expected},
                  )
```
/answer a3188868-8a93-438a-93a1-061d0d6876e2 allow
ReasoningToken: 65
ReasoningToken: 169
Control Request:
```yaml
id: 2f7c909a-7bff-4f50-823f-6d8c136c5aa5
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
```
/answer 2f7c909a-7bff-4f50-823f-6d8c136c5aa5 allow
Control Request:
```yaml
id: a1ed46c4-3e37-48af-9a17-6c48cdde3a28
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: "grep -rln '\"result\": \"success\"\\|result.*success\\|structuredContent' src/xy/ai/mcpc/tools --include=*.py"
```
/answer a1ed46c4-3e37-48af-9a17-6c48cdde3a28 allow
Control Request:
```yaml
id: bfd942e5-4cfc-47a3-a60f-51857748fd25
result:
  structuredContent:
    content: |
      """Central tool registry and tool result helpers.

      Tools are registered once in a process-wide :class:`ToolRegistry`.  What a
      given client actually sees is derived by reconciling the registry against the
      per-session configuration (:attr:`Session.enabled_tools`).
      """

      from __future__ import annotations

      from dataclasses import dataclass, field
      from typing import TYPE_CHECKING, Any, Callable

      from .codec import JsonCodec
      from .session import Session

      if TYPE_CHECKING:
          from .context import AppServices


      @dataclass(slots=True)
      class ToolContext:
          """Context handed to a tool handler on invocation."""

          session: Session
          arguments: dict[str, Any]
          #: Shared process-wide services (session store, CLI manager, profiles).
          #: ``None`` for tools that never orchestrate other sessions.
          services: "AppServices | None" = None


      @dataclass(slots=True)
      class ToolResult:
          """The result of a tool call (maps onto MCP ``CallToolResult``)."""

          content: list[dict[str, Any]] = field(default_factory=list)
          structured_content: dict[str, Any] | None = None
          is_error: bool = False

          def to_dict(self) -> dict[str, Any]:
              result: dict[str, Any] = {}
              if self.content:
                  result["content"] = list(self.content)
              if self.structured_content:
                  result["structuredContent"] = self.structured_content
              if self.is_error:
                  result["isError"] = True
              return result


      def text_content(text: str) -> dict[str, Any]:
          """Build a ``TextContent`` block."""
          return {"type": "text", "text": text}


      #: A handler receives the invocation context and returns one of:
      #: * a :class:`ToolResult`,
      #: * a ``str`` (wrapped as a single text content block),
      #: * a ``dict`` (treated as structured content, also rendered as JSON text).
      ToolHandler = Callable[[ToolContext], "ToolResult | str | dict[str, Any]"]


      @dataclass(slots=True)
      class Tool:
          """A registered tool and its MCP metadata."""

          name: str
          description: str
          input_schema: dict[str, Any]
          handler: ToolHandler
          title: str | None = None
          output_schema: dict[str, Any] | None = None
          annotations: dict[str, Any] | None = None

          def to_spec(self) -> dict[str, Any]:
              """Return the MCP ``Tool`` object advertised via ``tools/list``."""
              spec: dict[str, Any] = {
                  "name": self.name,
                  "description": self.description,
                  "inputSchema": self.input_schema,
              }
              if self.title is not None:
                  spec["title"] = self.title
              if self.output_schema is not None:
                  spec["outputSchema"] = self.output_schema
              if self.annotations is not None:
                  spec["annotations"] = self.annotations
              return spec


      def normalize_result(value: "ToolResult | str | dict[str, Any] | None") -> ToolResult:
          """Coerce whatever a handler returned into a :class:`ToolResult`."""
          if isinstance(value, ToolResult):
              return value
          if value is None:
              return ToolResult(content=[])
          if isinstance(value, str):
              return ToolResult(content=[text_content(value)])
          if isinstance(value, dict):
              # A dict already shaped like a CallToolResult is passed through.
              if "content" in value and isinstance(value["content"], list):
                  return ToolResult(
                      content=value["content"],
                      structured_content=value.get("structuredContent"),
                      is_error=bool(value.get("isError", False)),
                  )
              # Otherwise treat the dict as structured content.
              rendered = JsonCodec.encode(value)
              return ToolResult(content=[text_content(rendered)], structured_content=value)
          raise TypeError(f"Unsupported tool return type: {type(value)!r}")


      class ToolRegistry:
          """Process-wide registry of available tools."""

          def __init__(self) -> None:
              self._tools: dict[str, Tool] = {}

          def register(self, tool: Tool) -> Tool:
              if tool.name in self._tools:
                  raise ValueError(f"Tool already registered: {tool.name}")
              self._tools[tool.name] = tool
              return tool

          def tool(
              self,
              name: str,
              *,
              description: str,
              input_schema: dict[str, Any],
              title: str | None = None,
              output_schema: dict[str, Any] | None = None,
              annotations: dict[str, Any] | None = None,
          ) -> Callable[[ToolHandler], ToolHandler]:
              """Decorator registering the decorated function as a tool handler."""

              def decorator(handler: ToolHandler) -> ToolHandler:
                  self.register(
                      Tool(
                          name=name,
                          description=description,
                          input_schema=input_schema,
                          handler=handler,
                          title=title,
                          output_schema=output_schema,
                          annotations=annotations,
                      )
                  )
                  return handler

              return decorator

          def get(self, name: str) -> Tool | None:
              return self._tools.get(name)

          def __contains__(self, name: object) -> bool:
              return name in self._tools

          def __len__(self) -> int:
              return len(self._tools)

          def names(self) -> list[str]:
              return list(self._tools)

          def list_for_session(self, session: Session) -> list[Tool]:
              """Reconcile the registry with the session's enabled-tool configuration.

              Returns the tools the session is allowed to see, sorted by name for a
              stable pagination order.
              """
              tools = [t for t in self._tools.values() if session.is_tool_enabled(t.name)]
              tools.sort(key=lambda t: t.name)
              return tools
```
/answer bfd942e5-4cfc-47a3-a60f-51857748fd25 allow
Control Request:
```yaml
id: 85097926-8c82-4899-a60b-a30784a72632
result:
  structuredContent:
    stdout: |
      src/xy/ai/mcpc/tools/mcp/bridge.py
      src/xy/ai/mcpc/tools/change/__init__.py
      src/xy/ai/mcpc/tools/replace_chars/__init__.py
      src/xy/ai/mcpc/tools/insert/__init__.py
      src/xy/ai/mcpc/tools/replace_lines/__init__.py
      src/xy/ai/mcpc/tools/replace_block/__init__.py
      src/xy/ai/mcpc/tools/write/__init__.py
```
/answer 85097926-8c82-4899-a60b-a30784a72632 allow
ReasoningToken: 1152
Control Request:
```yaml
id: aa4b3547-5bf1-47a4-a7bf-bc680cd193da
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: for f in src/xy/ai/mcpc/tools/mcp/bridge.py src/xy/ai/mcpc/tools/change/__init__.py src/xy/ai/mcpc/tools/replace_chars/__init__.py src/xy/ai/mcpc/tools/insert/__init__.py src/xy/ai/mcpc/tools/replace_lines/__init__.py src/xy/ai/mcpc/tools/replace_block/__init__.py src/xy/ai/mcpc/tools/write/__init__.py; do echo "=== $f ==="; grep -n "success\|ToolResult\|structured" "$f"; done
```
/answer aa4b3547-5bf1-47a4-a7bf-bc680cd193da allow
Control Request:
```yaml
id: 2447f536-5dcc-4852-b33a-eab8bad373ff
result:
  structuredContent:
    stdout: STDOUT exceeded the 2000-character limit (6247 characters). Full output written to file.
    stdout_file: /tmp/mcpc-stdout-ud57j97n.log
```
/answer 2447f536-5dcc-4852-b33a-eab8bad373ff allow
Control Request:
```yaml
id: a8a5d281-0037-4cd3-9543-d8daee2324a4
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: cat /tmp/mcpc-stdout-ud57j97n.log
```
/answer a8a5d281-0037-4cd3-9543-d8daee2324a4 allow
Control Request:
```yaml
id: 4e2660bf-14b4-4498-a4c4-5032fc3ed053
result:
  structuredContent:
    stdout: STDOUT exceeded the 2000-character limit (6247 characters). Full output written to file.
    stdout_file: /tmp/mcpc-stdout-loq9dgdg.log
```
/answer 4e2660bf-14b4-4498-a4c4-5032fc3ed053 allow
Control Request:
```yaml
id: 013f840a-85ca-473f-b949-889a7b137b54
toolName: read
arguments:
  path: /tmp/mcpc-stdout-ud57j97n.log
```
/answer 013f840a-85ca-473f-b949-889a7b137b54 allow
Control Request:
```yaml
id: baaea1ae-712f-4024-bb9f-525e07c9be89
result:
  structuredContent:
    content: |
      === src/xy/ai/mcpc/tools/mcp/bridge.py ===
      17:from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
      45:    ) -> ToolResult:
      46:        """Forward a call and translate the outcome into a :class:`ToolResult`."""
      52:            return ToolResult(
      54:                structured_content={"error": msg},
      85:        def handler(ctx: ToolContext) -> ToolResult:
      93:def _to_tool_result(result: dict[str, Any]) -> ToolResult:
      94:    """Mirror a remote ``CallToolResult`` into an MCPC :class:`ToolResult`."""
      99:    # emitting it a second time on success is what used to duplicate the
      100:    # remote's answer next to structuredContent.
      112:    # Use structuredContent from the remote server when present. Otherwise
      118:    structured = result.get("structuredContent")
      119:    if isinstance(structured, dict):
      120:        structured_content = structured
      123:        structured_content = parsed if isinstance(parsed, dict) else {"content": text}
      126:    # successful result relies on structuredContent alone; only errors need a
      131:    return ToolResult(
      133:        structured_content=structured_content,
      === src/xy/ai/mcpc/tools/change/__init__.py ===
      8:from ...registry import ToolContext, ToolRegistry, ToolResult
      48:                    "description": "``success`` on success.",
      59:    def change(ctx: ToolContext) -> ToolResult:
      69:            return ToolResult(
      70:                structured_content={"error": "Path must be absolute."},
      74:            return ToolResult(
      75:                structured_content={"error": "File not found."},
      79:            return ToolResult(
      80:                structured_content={"error": "Not a regular file."},
      89:            return ToolResult(
      90:                structured_content={"error": "Start marker not found in file."},
      94:            return ToolResult(
      95:                structured_content={
      104:            return ToolResult(
      105:                structured_content={"error": "End marker not found in file."},
      109:            return ToolResult(
      110:                structured_content={
      121:            return ToolResult(
      122:                structured_content={
      137:            return ToolResult(
      138:                structured_content={"error": f"Write failed: {exc}"},
      142:        return ToolResult(structured_content={"result": "success"})
      === src/xy/ai/mcpc/tools/replace_chars/__init__.py ===
      8:from ...registry import ToolContext, ToolRegistry, ToolResult
      51:                    "description": "``success`` on success.",
      58:    def replace_chars(ctx: ToolContext) -> ToolResult:
      67:            return ToolResult(
      68:                structured_content={"error": "Path must be absolute."},
      72:            return ToolResult(
      73:                structured_content={"error": "File not found."},
      77:            return ToolResult(
      78:                structured_content={"error": "Not a regular file."},
      86:                return ToolResult(
      87:                    structured_content={
      99:            return ToolResult(
      100:                structured_content={"error": f"Replace failed: {exc}"},
      104:        return ToolResult(structured_content={"result": "success"})
      === src/xy/ai/mcpc/tools/insert/__init__.py ===
      8:from ...registry import ToolContext, ToolRegistry, ToolResult
      44:                    "description": "``success`` on success.",
      51:    def insert(ctx: ToolContext) -> ToolResult:
      59:            return ToolResult(
      60:                structured_content={"error": "Path must be absolute."},
      64:            return ToolResult(
      65:                structured_content={"error": "File not found."},
      69:            return ToolResult(
      70:                structured_content={"error": "Not a regular file."},
      77:                return ToolResult(
      78:                    structured_content={
      88:            return ToolResult(
      89:                structured_content={"error": f"Insert failed: {exc}"},
      93:        return ToolResult(structured_content={"result": "success"})
      === src/xy/ai/mcpc/tools/replace_lines/__init__.py ===
      12:from ...registry import ToolContext, ToolRegistry, ToolResult
      56:                    "description": "``success`` on success.",
      63:    def replace_lines(ctx: ToolContext) -> ToolResult:
      72:            return ToolResult(
      73:                structured_content={"error": "Path must be absolute."},
      77:            return ToolResult(
      78:                structured_content={"error": "File not found."},
      82:            return ToolResult(
      83:                structured_content={"error": "Not a regular file."},
      92:                return ToolResult(
      93:                    structured_content={
      105:            return ToolResult(
      106:                structured_content={"error": f"Replace failed: {exc}"},
      110:        return ToolResult(structured_content={"result": "success"})
      === src/xy/ai/mcpc/tools/replace_block/__init__.py ===
      8:from ...registry import ToolContext, ToolRegistry, ToolResult
      57:    def replace_block(ctx: ToolContext) -> ToolResult:
      66:            return ToolResult(
      67:                structured_content={"error": "Path must be absolute."},
      71:            return ToolResult(
      72:                structured_content={"error": "File not found."},
      76:            return ToolResult(
      77:                structured_content={"error": "Not a regular file."},
      81:            return ToolResult(
      82:                structured_content={"error": "'old_text' must not be empty."},
      91:            return ToolResult(
      92:                structured_content={"error": "Text not found in file."},
      96:            return ToolResult(
      97:                structured_content={
      109:            return ToolResult(
      110:                structured_content={"error": f"Write failed: {exc}"},
      114:        return ToolResult(structured_content={"result": "success"})
      === src/xy/ai/mcpc/tools/write/__init__.py ===
      8:from ...registry import ToolContext, ToolRegistry, ToolResult
      48:                    "description": "``success`` on success.",
      55:    def write(ctx: ToolContext) -> ToolResult:
      63:            return ToolResult(
      64:                structured_content={"error": "Path must be absolute."},
      74:            return ToolResult(
      75:                structured_content={"error": f"Write failed: {exc}"},
      79:        return ToolResult(structured_content={"result": "success"})
```
/answer baaea1ae-712f-4024-bb9f-525e07c9be89 allow
Control Request:
```yaml
id: d87fa16c-314e-4457-a109-b243949e2ef5
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
```
/answer d87fa16c-314e-4457-a109-b243949e2ef5 allow
Control Request:
```yaml
id: 1305b30e-371d-4296-abd2-eaa87d032b41
result:
  structuredContent:
    content: |
      """Read tool – returns file contents, optionally sliced by line or unique marker.

      Range: start = min_line | start-marker | file start; end = max_line | end-marker
      | file end (all inclusive). Markers must be unique substrings. Per-session
      sha256 cache rejects unchanged re-reads (key ``_read_cache`` in session state).
      """

      from __future__ import annotations

      import hashlib
      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult

      def register_read_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "read",
              title="Read file",
              description=(
                  "Read a file as text, optionally sliced to a range. Repeated "
                  "unchanged reads return an error."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute file path.",
                      },
                      "min_line": {
                          "type": "integer",
                          "description": "Range start: line number, inclusive, 1-based. Excludes start.",
                          "minimum": 1,
                      },
                      "max_line": {
                          "type": "integer",
                          "description": "Range end: line number, inclusive, 1-based. Excludes end.",
                          "minimum": 1,
                      },
                      "start": {
                          "type": "string",
                          "description": "Range start: unique marker substring, inclusive. Excludes min_line.",
                      },
                      "end": {
                          "type": "string",
                          "description": "Range end: unique marker substring, inclusive. Excludes max_line.",
                      },
                  },
                  "required": ["path"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "content": {"type": "string"},
                  },
                  "required": ["content"],
              },
              annotations={"readOnlyHint": True, "openWorldHint": False},
          )
          def read(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              min_line: int | None = args.get("min_line")
              max_line: int | None = args.get("max_line")
              start_marker: str | None = args.get("start")
              end_marker: str | None = args.get("end")

              # --- mutual exclusivity validation ---
              if min_line is not None and start_marker is not None:
                  return ToolResult(
                      structured_content={
                          "error": "``min_line`` and ``start`` are mutually exclusive."
                      },
                      is_error=True,
                  )
              if max_line is not None and end_marker is not None:
                  return ToolResult(
                      structured_content={
                          "error": "``max_line`` and ``end`` are mutually exclusive."
                      },
                      is_error=True,
                  )

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              raw_bytes = path.read_bytes()

              # --- decode ---
              text = raw_bytes.decode("utf-8", errors="replace")
              lines = text.splitlines(keepends=True)
              total_lines = len(lines)

              def line_start_offset(line_num: int) -> int:
                  n = max(0, min(line_num - 1, total_lines))
                  return sum(len(l) for l in lines[:n])

              def line_end_offset(line_num: int) -> int:
                  n = max(0, min(line_num, total_lines))
                  return sum(len(l) for l in lines[:n])

              if start_marker is not None:
                  start_count = text.count(start_marker)
                  if start_count == 0:
                      return ToolResult(
                          structured_content={"error": "Start marker not found in file."},
                          is_error=True,
                      )
                  if start_count > 1:
                      return ToolResult(
                          structured_content={
                              "error": f"Start marker is ambiguous – found {start_count} occurrences in file."
                          },
                          is_error=True,
                      )
                  region_start = text.index(start_marker)
              elif min_line is not None:
                  region_start = line_start_offset(min_line)
              else:
                  region_start = 0

              if end_marker is not None:
                  end_count = text.count(end_marker)
                  if end_count == 0:
                      return ToolResult(
                          structured_content={"error": "End marker not found in file."},
                          is_error=True,
                      )
                  if end_count > 1:
                      return ToolResult(
                          structured_content={
                              "error": f"End marker is ambiguous – found {end_count} occurrences in file."
                          },
                          is_error=True,
                      )
                  region_end = text.index(end_marker) + len(end_marker)
              elif max_line is not None:
                  region_end = line_end_offset(max_line)
              else:
                  region_end = len(text)

              # --- order validation ---
              if region_end < region_start:
                  return ToolResult(
                      structured_content={
                          "error": (
                              f"Resolved end position must not lie before "
                              f"the resolved start position."
                          )
                      },
                      is_error=True,
                  )

              sliced = text[region_start:region_end]
              structured: dict[str, Any] = {"content": sliced}

              return ToolResult(structured_content=structured)
```
/answer 1305b30e-371d-4296-abd2-eaa87d032b41 allow
ReasoningToken: 261
Control Request:
```yaml
id: 61f74301-80d9-4378-9a3b-68b66bfbece0
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py
```
/answer 61f74301-80d9-4378-9a3b-68b66bfbece0 allow
Control Request:
```yaml
id: a2df95e6-07e6-407a-99b3-2d3378fb3291
result:
  structuredContent:
    content: |
      """File-system and shell tools for the MCPC server.

      Available tools
      ---------------
      * ``read``          – read a file (with session-level content-hash caching)
      * ``list``          – recursively list files below a directory
      * ``write``         – overwrite or append to a file
      * ``insert``        – insert text at a character offset
      * ``replace-chars`` – replace a character range with new text
      * ``replace-lines`` – replace a line range with new text
      * ``replace-block`` – replace an exact block of text (old text -> new text)
      * ``change``        – replace a delimited block identified by start/end markers
      * ``bash``          – run a Bash script in a given working directory
      * ``python``        – run a Python script directly from context
      * ``markdown``      – AST-based Markdown editing via a remark (Node.js) script
      * ``ask-user``      – ask the user a clarifying question (back-channel)

      Skills (on-demand hint tools) are registered from the ``skills`` sub-package.
      Bridges to external MCP servers (e.g. Exa) live in the ``mcp`` sub-package.
      OpenAlex scholarly-search tools live in the ``openalex`` sub-package.

      Call :func:`register_tools` to register all tools onto a
      :class:`~xy.ai.mcpc.registry.ToolRegistry` instance.
      """

      from __future__ import annotations

      import importlib

      from ..registry import ToolRegistry
      from .bash import register_bash_tool
      from .change import register_change_tool
      from .insert import register_insert_tool
      from .list import register_list_tool
      from .markdown import register_markdown_tool
      from .mcp import register_context7_tools, register_exa_tools, register_github_tools
      from .openalex import register_openalex_tools
      from .python import register_python_tool
      from .read import register_read_tool
      from xy.ai.mcpc.tools.replace_chars import register_replace_chars_tool
      from .replace_lines import register_replace_lines_tool
      from .replace_block import register_replace_block_tool
      from .skills import register_skills
      from .write import register_write_tool

      # ``ask-user`` uses a hyphenated directory name, which is not a valid Python
      # identifier, so it cannot be imported with a regular ``from .ask-user import``
      # statement. Use ``importlib`` instead.
      register_ask_user_tool = importlib.import_module(
          "xy.ai.mcpc.tools.ask-user"
      ).register_ask_user_tool


      def register_tools(registry: ToolRegistry) -> None:
          """Register all built-in file-system and shell tools onto *registry*."""
          register_read_tool(registry)
          register_list_tool(registry)
          register_write_tool(registry)
          register_insert_tool(registry)
          register_change_tool(registry)
          register_replace_chars_tool(registry)
          register_replace_lines_tool(registry)
          register_replace_block_tool(registry)
          register_bash_tool(registry)
          register_python_tool(registry)
          register_markdown_tool(registry)
          register_ask_user_tool(registry)
          register_skills(registry)
          register_exa_tools(registry)
          register_github_tools(registry)
          register_context7_tools(registry)
          register_openalex_tools(registry)


      # Keep the old name available so existing call sites don't break.
      register_builtin_tools = register_tools

      __all__ = [
          "register_tools",
          "register_builtin_tools",
      ]
```
/answer a2df95e6-07e6-407a-99b3-2d3378fb3291 allow
Text:
 Jetzt implementiere ich die Änderungen.
Control Request:
```yaml
id: 1ba2ab70-4ccb-4e5a-89e1-2d22d5128014
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  old_text: |2-
        content: list[dict[str, Any]] = field(default_factory=list)
        structured_content: dict[str, Any] | None = None
        is_error: bool = False

        def to_dict(self) -> dict[str, Any]:
  new_text: |2-
        content: list[dict[str, Any]] = field(default_factory=list)
        structured_content: dict[str, Any] | None = None
        is_error: bool = False
        auto_approve: bool = False
        """Tool-side hint: skip human review of this result during the
        ``result`` control phase, since the tool itself judges there is
        nothing meaningful to approve (e.g. a plain success acknowledgement,
        or a read that returns an unrestricted full file). Never serialised
        to the client; consumed only by :class:`ToolControlManager`."""

        def to_dict(self) -> dict[str, Any]:
```
/answer 1ba2ab70-4ccb-4e5a-89e1-2d22d5128014 allow
Control Request:
```yaml
id: 96061907-a191-4734-8e85-74b165d1b409
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
```
/answer 96061907-a191-4734-8e85-74b165d1b409 allow
Control Request:
```yaml
id: ae5298dd-a720-429e-afac-7d07ef6652c0
result:
  structuredContent:
    content: |
      """Write tool – writes a file completely or appends lines to it."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_write_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "write",
              title="Write file",
              description=(
                  "Write content to a file. "
                  "In ``replace`` mode the file is overwritten with the supplied content. "
                  "In ``append`` mode the content is added at the end of the existing file "
                  "(the file is created if it does not yet exist)."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the file to write.",
                      },
                      "mode": {
                          "type": "string",
                          "enum": ["replace", "append"],
                          "description": (
                              "``replace`` – overwrite the file with the new content. "
                              "``append`` – add the new content after the existing content."
                          ),
                      },
                      "content": {
                          "type": "string",
                          "description": "Text to write to the file.",
                      },
                  },
                  "required": ["path", "mode", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                  },
                  "required": ["result"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def write(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              mode: str = args["mode"]
              content: str = args["content"]

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )

              try:
                  path.parent.mkdir(parents=True, exist_ok=True)
                  file_mode = "a" if mode == "append" else "w"
                  with path.open(file_mode, encoding="utf-8") as fh:
                      fh.write(content)
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Write failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"})
```
/answer ae5298dd-a720-429e-afac-7d07ef6652c0 allow
Control Request:
```yaml
id: 8c3e001b-6a0a-458d-acbb-f2690d364868
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py
```
/answer 8c3e001b-6a0a-458d-acbb-f2690d364868 allow
Control Request:
```yaml
id: 2c39fb93-95b7-47a2-8eea-91378c002cb9
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: "grep -rn \"normalize_result\\|return {\" src/xy/ai/mcpc/tools --include=*.py | grep -v \"structured_content\\|ToolResult\""
```
/answer 2c39fb93-95b7-47a2-8eea-91378c002cb9 allow
Control Request:
```yaml
id: 5999ca67-fb5a-46af-9a44-bbb14f6c3202
result:
  structuredContent:
    content: |
      """Insert tool – inserts text at a character offset inside an existing file."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_insert_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "insert",
              title="Insert into file",
              description=(
                  "Insert text at a specific character offset inside an existing file. "
                  "The offset is zero-based and refers to the UTF-8 decoded content of the file. "
                  "All existing content at and after the offset is shifted right."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the file to modify.",
                      },
                      "offset": {
                          "type": "integer",
                          "description": "Zero-based character offset at which to insert the new content.",
                          "minimum": 0,
                      },
                      "content": {
                          "type": "string",
                          "description": "Text to insert at the given offset.",
                      },
                  },
                  "required": ["path", "offset", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                  },
                  "required": ["result"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def insert(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              offset: int = args["offset"]
              new_content: str = args["content"]

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              try:
                  text = path.read_text(encoding="utf-8")
                  if offset > len(text):
                      return ToolResult(
                          structured_content={
                              "error": (
                                  f"Offset is beyond end of file."
                              )
                          },
                          is_error=True,
                      )
                  result = text[:offset] + new_content + text[offset:]
                  path.write_text(result, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Insert failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"})
```
/answer 5999ca67-fb5a-46af-9a44-bbb14f6c3202 allow
Control Request:
```yaml
id: 43e0e469-3f76-44c8-a6f3-83c57d056791
result:
  structuredContent:
    exit_code: 1
    stdout: ""
  isError: true
```
/answer 43e0e469-3f76-44c8-a6f3-83c57d056791 allow
Control Request:
```yaml
id: b12f6ed9-5096-4bae-b8e7-ffe388886bbf
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/change/__init__.py
```
/answer b12f6ed9-5096-4bae-b8e7-ffe388886bbf allow
Control Request:
```yaml
id: 8bf3cf92-93c7-4038-8a32-64ca076317fa
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  old_text: |2-
        def submit_result(
            self,
            session: Session,
            tool_name: str,
            result: dict[str, Any],
        ) -> ControlDecision:
            \"\"\"Block until the controller approves/replaces a tool-call *result*.

            Returns a :class:`ControlDecision`.  If the decision includes
            ``modified_result``, the caller should use that instead of the
            original result.

            Simple \"success\" results (empty ``content`` and a
            ``structuredContent`` of exactly ``{\"result\": \"success\"}``) are
            auto-approved without involving the controller, since there is
            nothing meaningful for a human to review.
            \"\"\"
            if self._is_simple_success_result(result):
                logger.info(
                    \"Auto-approving simple success result for %s [%s]\",
                    tool_name, session.id,
                )
                return ControlDecision(approved=True)

            item = self._enqueue(session, \"result\", tool_name, arguments=None, result=result)
            return self._wait(item)
  new_text: |2-
        def submit_result(
            self,
            session: Session,
            tool_name: str,
            result: dict[str, Any],
            *,
            auto_approve: bool = False,
        ) -> ControlDecision:
            \"\"\"Block until the controller approves/replaces a tool-call *result*.

            Returns a :class:`ControlDecision`.  If the decision includes
            ``modified_result``, the caller should use that instead of the
            original result.

            ``auto_approve`` is a hint set by the *tool itself* (via
            :attr:`ToolResult.auto_approve`), not derived from the shape of the
            result.  A tool sets it when it judges its own result carries
            nothing meaningful for a human to review (e.g. a plain success
            acknowledgement). When set, the result is approved without
            involving the controller.
            \"\"\"
            if auto_approve:
                logger.info(
                    \"Auto-approving result for %s [%s] (tool-flagged)\",
                    tool_name, session.id,
                )
                return ControlDecision(approved=True)

            item = self._enqueue(session, \"result\", tool_name, arguments=None, result=result)
            return self._wait(item)
```
/answer 8bf3cf92-93c7-4038-8a32-64ca076317fa allow
Control Request:
```yaml
id: cd66212d-45a0-4417-92b5-11a553501967
result:
  structuredContent:
    content: |
      """Change tool – replaces the block between start/end markers (both inclusive)."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_change_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "change",
              title="Change file block",
              description=(
                  "Replace the text between 'start' and 'end' (both included) with "
                  "'content'. Each marker must occur exactly once in the file; "
                  "'end' must come after 'start'. Repeat a marker inside 'content' "
                  "to keep it."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the target file.",
                      },
                      "start": {
                          "type": "string",
                          "description": "Unique substring marking the block's start (must occur exactly once).",
                      },
                      "end": {
                          "type": "string",
                          "description": "Unique substring marking the block's end (must occur exactly once, after 'start').",
                      },
                      "content": {
                          "type": "string",
                          "description": "Text that replaces the block, including where 'start'/'end' were.",
                      },
                  },
                  "required": ["path", "start", "end", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                      "error": {
                          "type": "string",
                          "description": "Human-readable error message (only present when is_error is true).",
                      },
                  },
                  "required": [],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def change(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              start_marker: str = args["start"]
              end_marker: str = args["end"]
              new_content: str = args["content"]

              # --- path validation ---
              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              text = path.read_text(encoding="utf-8")

              # --- locate and validate start marker ---
              start_count = text.count(start_marker)
              if start_count == 0:
                  return ToolResult(
                      structured_content={"error": "Start marker not found in file."},
                      is_error=True,
                  )
              if start_count > 1:
                  return ToolResult(
                      structured_content={
                          "error": f"Start marker is ambiguous – found {start_count} occurrences in file."
                      },
                      is_error=True,
                  )

              # --- locate and validate end marker ---
              end_count = text.count(end_marker)
              if end_count == 0:
                  return ToolResult(
                      structured_content={"error": "End marker not found in file."},
                      is_error=True,
                  )
              if end_count > 1:
                  return ToolResult(
                      structured_content={
                          "error": f"End marker is ambiguous – found {end_count} occurrences in file."
                      },
                      is_error=True,
                  )

              start_pos = text.index(start_marker)
              end_pos = text.index(end_marker)

              # --- order validation ---
              if end_pos <= start_pos:
                  return ToolResult(
                      structured_content={
                          "error": (
                              f"End marker must appear after start marker."
                          )
                      },
                      is_error=True,
                  )

              # --- apply replacement: both markers included (full range) ---
              result_text = text[:start_pos] + new_content + text[end_pos + len(end_marker):]

              # --- write back ---
              try:
                  path.write_text(result_text, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Write failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"})
```
/answer cd66212d-45a0-4417-92b5-11a553501967 allow
Control Request:
```yaml
id: b3b0b695-ee19-4633-af92-1f47e87501f4
result:
  structuredContent:
    error: Text not found in file.
  isError: true
```
/answer b3b0b695-ee19-4633-af92-1f47e87501f4 allow
Control Request:
```yaml
id: 013b7010-8363-48a5-8197-276408b934e3
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_chars/__init__.py
```
/answer 013b7010-8363-48a5-8197-276408b934e3 allow
Control Request:
```yaml
id: 9c7904f8-c56b-48ca-85ea-895862519a58
result:
  structuredContent:
    content: |
      """Replace-chars tool – replaces a character range inside an existing file."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_replace_chars_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "replace-chars",
              title="Replace characters in file",
              description=(
                  "Replace a range of characters inside an existing file with new content. "
                  "The range is defined by a zero-based character ``offset`` and a ``length`` "
                  "(number of characters to remove starting at the offset). "
                  "The supplied ``content`` is written in place of the removed range. "
                  "To replace whole lines instead, use ``replace-lines``."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the file to modify.",
                      },
                      "offset": {
                          "type": "integer",
                          "description": "Zero-based character offset of the first character to replace.",
                          "minimum": 0,
                      },
                      "length": {
                          "type": "integer",
                          "description": "Number of characters to remove starting at ``offset``.",
                          "minimum": 0,
                      },
                      "content": {
                          "type": "string",
                          "description": "Replacement text (may be empty to perform a pure deletion).",
                      },
                  },
                  "required": ["path", "offset", "length", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                  },
                  "required": ["result"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def replace_chars(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              offset: int = args["offset"]
              length: int = args["length"]
              new_content: str = args["content"]

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              try:
                  text = path.read_text(encoding="utf-8")
                  file_len = len(text)
                  if offset > file_len:
                      return ToolResult(
                          structured_content={
                              "error": (
                                  f"Offset {offset} is beyond end of file "
                                  f"(file length: {file_len} characters)."
                              )
                          },
                          is_error=True,
                      )
                  end = min(offset + length, file_len)
                  result = text[:offset] + new_content + text[end:]
                  path.write_text(result, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Replace failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"})
```
/answer 9c7904f8-c56b-48ca-85ea-895862519a58 allow
Control Request:
```yaml
id: 55923734-0639-42b5-8087-3c7ab1dd1ef4
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_lines/__init__.py
```
/answer 55923734-0639-42b5-8087-3c7ab1dd1ef4 allow
Control Request:
```yaml
id: db8db29c-d63f-4a88-b6ce-698e22ec1d41
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  start: '    def submit_result('
  end: |2-
            return self._wait(item)

        # ------------------------------------------------------------------
        # Control-endpoint-facing API (non-blocking)
```
/answer db8db29c-d63f-4a88-b6ce-698e22ec1d41 allow
Control Request:
```yaml
id: ffae5123-f42d-492a-84e7-f616581ad8d6
result:
  structuredContent:
    content: |
      """Replace-lines tool – replaces a range of lines inside an existing file.

      This is the line-oriented analogue of ``replace-chars``: the range is given as a
      zero-based *line* offset and a *line* count instead of character offsets.
      """

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_replace_lines_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "replace-lines",
              title="Replace lines in file",
              description=(
                  "Replace a range of lines inside an existing file with new content. "
                  "The range is defined by a zero-based line ``offset`` and a ``length`` "
                  "(number of lines to remove starting at the offset). "
                  "The supplied ``content`` is written in place of the removed lines; "
                  "it should include its own trailing newline if a line break is wanted. "
                  "To replace an arbitrary character range instead, use ``replace-chars``."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the file to modify.",
                      },
                      "offset": {
                          "type": "integer",
                          "description": "Zero-based line offset of the first line to replace.",
                          "minimum": 0,
                      },
                      "length": {
                          "type": "integer",
                          "description": "Number of lines to remove starting at ``offset``.",
                          "minimum": 0,
                      },
                      "content": {
                          "type": "string",
                          "description": "Replacement text (may be empty to perform a pure deletion).",
                      },
                  },
                  "required": ["path", "offset", "length", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                  },
                  "required": ["result"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def replace_lines(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              offset: int = args["offset"]
              length: int = args["length"]
              new_content: str = args["content"]

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              try:
                  text = path.read_text(encoding="utf-8")
                  lines = text.splitlines(keepends=True)
                  line_count = len(lines)
                  if offset > line_count:
                      return ToolResult(
                          structured_content={
                              "error": (
                                  f"Offset {offset} is beyond end of file "
                                  f"(file length: {line_count} lines)."
                              )
                          },
                          is_error=True,
                      )
                  end = min(offset + length, line_count)
                  result = "".join(lines[:offset]) + new_content + "".join(lines[end:])
                  path.write_text(result, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Replace failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"})
```
/answer ffae5123-f42d-492a-84e7-f616581ad8d6 allow
Control Request:
```yaml
id: 91488182-7c42-4621-aeaa-a74cf013721e
result:
  structuredContent:
    content: |2-
          def submit_result(
              self,
              session: Session,
              tool_name: str,
              result: dict[str, Any],
          ) -> ControlDecision:
              """Block until the controller approves/replaces a tool-call *result*.

              Returns a :class:`ControlDecision`.  If the decision includes
              ``modified_result``, the caller should use that instead of the
              original result.

              Simple "success" results (empty ``content`` and a
              ``structuredContent`` of exactly ``{"result": "success"}``) are
              auto-approved without involving the controller, since there is
              nothing meaningful for a human to review.
              """
              if self._is_simple_success_result(result):
                  logger.info(
                      "Auto-approving simple success result for %s [%s]",
                      tool_name, session.id,
                  )
                  return ControlDecision(approved=True)

              item = self._enqueue(session, "result", tool_name, arguments=None, result=result)
              return self._wait(item)

          # ------------------------------------------------------------------
          # Control-endpoint-facing API (non-blocking)
```
/answer 91488182-7c42-4621-aeaa-a74cf013721e allow
Control Request:
```yaml
id: b1f7d75f-9ea1-45d6-b492-375f7d5e39ff
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py
```
/answer b1f7d75f-9ea1-45d6-b492-375f7d5e39ff allow
Control Request:
```yaml
id: a3104e7a-ccb0-465a-a069-b2d3da868376
result:
  structuredContent:
    content: |
      """Replace-block tool – replaces an exact block of text (old -> new) in a file."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_replace_block_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "replace-block",
              title="Replace text block in file",
              description=(
                  "Replace a complete block of text inside an existing file. "
                  "'old_text' must occur exactly once in the file and is replaced "
                  "in full by 'new_text'. Use this when you know the exact text to "
                  "be replaced rather than an offset/length or start/end markers."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the target file.",
                      },
                      "old_text": {
                          "type": "string",
                          "description": (
                              "Exact text to find and replace. Must occur exactly once "
                              "in the file."
                          ),
                      },
                      "new_text": {
                          "type": "string",
                          "description": "Text that replaces 'old_text'.",
                      },
                  },
                  "required": ["path", "old_text", "new_text"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                      },
                      "error": {
                          "type": "string",
                          "description": "Human-readable error message (only present when is_error is true).",
                      },
                  },
                  "required": [],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def replace_block(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              old_text: str = args["old_text"]
              new_text: str = args["new_text"]

              # --- path validation ---
              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )
              if old_text == "":
                  return ToolResult(
                      structured_content={"error": "'old_text' must not be empty."},
                      is_error=True,
                  )

              text = path.read_text(encoding="utf-8")

              # --- locate and validate old_text ---
              occurrences = text.count(old_text)
              if occurrences == 0:
                  return ToolResult(
                      structured_content={"error": "Text not found in file."},
                      is_error=True,
                  )
              if occurrences > 1:
                  return ToolResult(
                      structured_content={
                          "error": f"Text is ambiguous – found {occurrences} occurrences in file."
                      },
                      is_error=True,
                  )

              result_text = text.replace(old_text, new_text, 1)

              # --- write back ---
              try:
                  path.write_text(result_text, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Write failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"})
```
/answer a3104e7a-ccb0-465a-a069-b2d3da868376 allow
Control Request:
```yaml
id: fe2b8ec7-14ec-448b-bd43-68c802c94e57
toolName: replace-lines
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  offset: 117
  length: 28
  content: |2
        def submit_result(
            self,
            session: Session,
            tool_name: str,
            result: dict[str, Any],
            *,
            auto_approve: bool = False,
        ) -> ControlDecision:
            """Block until the controller approves/replaces a tool-call *result*.

            Returns a :class:`ControlDecision`.  If the decision includes
            ``modified_result``, the caller should use that instead of the
            original result.

            ``auto_approve`` is a hint set by the *tool itself* (via
            ``ToolResult.auto_approve``), not derived from the shape of the
            result. A tool sets it when it judges its own result carries
            nothing meaningful for a human to review (e.g. a plain success
            acknowledgement, or an unrestricted full-file read). When set, the
            result is approved without involving the controller.
            """
            if auto_approve:
                logger.info(
                    "Auto-approving result for %s [%s] (tool-flagged)",
                    tool_name, session.id,
                )
                return ControlDecision(approved=True)

            item = self._enqueue(session, "result", tool_name, arguments=None, result=result)
            return self._wait(item)

        # ------------------------------------------------------------------
        # Control-endpoint-facing API (non-blocking)
```
/answer fe2b8ec7-14ec-448b-bd43-68c802c94e57 allow
Control Request:
```yaml
id: 3cedceaf-8729-46ef-aba5-517dff7df16b
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/markdown/__init__.py
```
/answer 3cedceaf-8729-46ef-aba5-517dff7df16b allow
Control Request:
```yaml
id: 571d8b59-7810-46a3-9dec-50c4cee6ad25
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  start: |2-
        @staticmethod
        def _is_simple_success_result
  end: |2
            return result.get("structuredContent") == {"result": "success"}
```
/answer 571d8b59-7810-46a3-9dec-50c4cee6ad25 allow
Control Request:
```yaml
id: d6ef7ccb-4977-40d8-9f63-8759fe39faa6
result:
  structuredContent:
    content: |
      """Markdown tool – AST-based reading/writing/transforming of Markdown files.

      The tool runs a TypeScript (ESM) script inside a pre-provisioned Node.js package
      environment that exposes ``remark``, ``remark-behead`` and ``remark-frontmatter``
      (via a local ``createRemark`` helper).  The script is handed to
      ``node --input-type=module`` on standard input and executed with the environment
      directory as its working directory, so bare package imports resolve.
      """

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...config import ServerConfig
      from ...registry import ToolContext, ToolRegistry, ToolResult
      from ..process import run_capture

      #: Example script surfaced in the tool description.
      _EXAMPLE = """\
      import { read, write } from 'to-vfile';
      import { createRemark } from './remark.js';
      import { visit } from 'unist-util-visit';

      const processor = createRemark({
        // frontmatter: true, // if required
        // behead: { depth: 1 }, // if required
      });

      processor.use(() => (tree, file) => {
        // insert code here
      });

      // read file – replace 'path/to/file.md' with the actual file path
      const file = await read('path/to/file.md');

      // parse to AST
      const tree = await processor.run(processor.parse(file), file);

      // Extract headings
      const headings = [];
      visit(tree, 'heading', (node) => {
          headings.push({
          depth: node.depth,
          text: node.children.map(c => c.value || c.children?.map(x => x.value).join('') || '').join('').trim()
          });
      });

      // format output
      await processor.process(file);
      file.path = 'path/to/file.md';
      await write(file);

      console.log(String("Done"));
      """

      _DESCRIPTION = (
          "AST-based reading, writing, modifying and transforming of Markdown files. "
          "Provide a TypeScript (ESM) script that uses `remark` (with `remark-behead` "
          "and `remark-frontmatter` available) to operate on Markdown. "
          "Returns the exit code, standard output and, if present, standard error.\n\n"
          "Follow this pattern:\n\n```typescript\n" + _EXAMPLE + "```"
      )


      def register_markdown_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "markdown",
              title="Run Markdown (remark) script",
              description=_DESCRIPTION,
              input_schema={
                  "type": "object",
                  "properties": {
                      "script": {
                          "type": "string",
                          "description": (
                              "TypeScript (ESM) script content to execute against the "
                              "remark environment."
                          ),
                      },
                  },
                  "required": ["script"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "exit_code": {"type": "integer"},
                      "stdout": {"type": "string"},
                      "stderr": {"type": "string"},
                  },
                  "required": ["exit_code", "stdout"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": True},
          )
          def markdown(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              script: str = args["script"]

              config = ctx.services.config if ctx.services is not None else ServerConfig()
              cwd = Path(config.markdown_env_dir)
              if not cwd.is_dir():
                  return ToolResult(
                      structured_content={"error": f"Markdown environment not found: {cwd}"},
                      is_error=True,
                  )

              return run_capture(
                  ["node", "--input-type=module"],
                  cwd=cwd,
                  stdin=script,
                  launch_error="Failed to launch node",
              )
```
/answer d6ef7ccb-4977-40d8-9f63-8759fe39faa6 allow
Control Request:
```yaml
id: a026bf0a-8d04-441c-9d50-4db6612c0ecc
result:
  structuredContent:
    content: |2
          @staticmethod
          def _is_simple_success_result(result: dict[str, Any]) -> bool:
              """Return ``True`` for a plain success result with nothing to review.

              Matches results of the shape::

                  {
                      "content": [],
                      "structuredContent": {"result": "success"}
                  }

              Any additional content items, extra keys, or a differing
              ``structuredContent`` payload disqualify the result from
              auto-approval.
              """
              if not isinstance(result, dict):
                  return False
              if set(result.keys()) - {"content", "structuredContent"}:
                  return False
              if result.get("content") not in ([], None):
                  return False
              return result.get("structuredContent") == {"result": "success"}
```
/answer a026bf0a-8d04-441c-9d50-4db6612c0ecc allow
Control Request:
```yaml
id: ad9c55e1-e844-487d-925b-4026ecd81981
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
```
/answer ad9c55e1-e844-487d-925b-4026ecd81981 allow
Control Request:
```yaml
id: 99890875-0576-4d49-bd99-f7b339a9b564
result:
  structuredContent:
    content: |
      """Read tool – returns file contents, optionally sliced by line or unique marker.

      Range: start = min_line | start-marker | file start; end = max_line | end-marker
      | file end (all inclusive). Markers must be unique substrings. Per-session
      sha256 cache rejects unchanged re-reads (key ``_read_cache`` in session state).
      """

      from __future__ import annotations

      import hashlib
      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult

      def register_read_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "read",
              title="Read file",
              description=(
                  "Read a file as text, optionally sliced to a range. Repeated "
                  "unchanged reads return an error."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute file path.",
                      },
                      "min_line": {
                          "type": "integer",
                          "description": "Range start: line number, inclusive, 1-based. Excludes start.",
                          "minimum": 1,
                      },
                      "max_line": {
                          "type": "integer",
                          "description": "Range end: line number, inclusive, 1-based. Excludes end.",
                          "minimum": 1,
                      },
                      "start": {
                          "type": "string",
                          "description": "Range start: unique marker substring, inclusive. Excludes min_line.",
                      },
                      "end": {
                          "type": "string",
                          "description": "Range end: unique marker substring, inclusive. Excludes max_line.",
                      },
                  },
                  "required": ["path"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "content": {"type": "string"},
                  },
                  "required": ["content"],
              },
              annotations={"readOnlyHint": True, "openWorldHint": False},
          )
          def read(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              min_line: int | None = args.get("min_line")
              max_line: int | None = args.get("max_line")
              start_marker: str | None = args.get("start")
              end_marker: str | None = args.get("end")

              # --- mutual exclusivity validation ---
              if min_line is not None and start_marker is not None:
                  return ToolResult(
                      structured_content={
                          "error": "``min_line`` and ``start`` are mutually exclusive."
                      },
                      is_error=True,
                  )
              if max_line is not None and end_marker is not None:
                  return ToolResult(
                      structured_content={
                          "error": "``max_line`` and ``end`` are mutually exclusive."
                      },
                      is_error=True,
                  )

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              raw_bytes = path.read_bytes()

              # --- decode ---
              text = raw_bytes.decode("utf-8", errors="replace")
              lines = text.splitlines(keepends=True)
              total_lines = len(lines)

              def line_start_offset(line_num: int) -> int:
                  n = max(0, min(line_num - 1, total_lines))
                  return sum(len(l) for l in lines[:n])

              def line_end_offset(line_num: int) -> int:
                  n = max(0, min(line_num, total_lines))
                  return sum(len(l) for l in lines[:n])

              if start_marker is not None:
                  start_count = text.count(start_marker)
                  if start_count == 0:
                      return ToolResult(
                          structured_content={"error": "Start marker not found in file."},
                          is_error=True,
                      )
                  if start_count > 1:
                      return ToolResult(
                          structured_content={
                              "error": f"Start marker is ambiguous – found {start_count} occurrences in file."
                          },
                          is_error=True,
                      )
                  region_start = text.index(start_marker)
              elif min_line is not None:
                  region_start = line_start_offset(min_line)
              else:
                  region_start = 0

              if end_marker is not None:
                  end_count = text.count(end_marker)
                  if end_count == 0:
                      return ToolResult(
                          structured_content={"error": "End marker not found in file."},
                          is_error=True,
                      )
                  if end_count > 1:
                      return ToolResult(
                          structured_content={
                              "error": f"End marker is ambiguous – found {end_count} occurrences in file."
                          },
                          is_error=True,
                      )
                  region_end = text.index(end_marker) + len(end_marker)
              elif max_line is not None:
                  region_end = line_end_offset(max_line)
              else:
                  region_end = len(text)

              # --- order validation ---
              if region_end < region_start:
                  return ToolResult(
                      structured_content={
                          "error": (
                              f"Resolved end position must not lie before "
                              f"the resolved start position."
                          )
                      },
                      is_error=True,
                  )

              sliced = text[region_start:region_end]
              structured: dict[str, Any] = {"content": sliced}

              return ToolResult(structured_content=structured)
```
/answer 99890875-0576-4d49-bd99-f7b339a9b564 allow
Control Request:
```yaml
id: e0ee5178-2542-4e72-abbc-aa891e58fb58
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  old_text: |2
        @staticmethod
        def _is_simple_success_result(result: dict[str, Any]) -> bool:
            """Return ``True`` for a plain success result with nothing to review.

            Matches results of the shape::

                {
                    "content": [],
                    "structuredContent": {"result": "success"}
                }

            Any additional content items, extra keys, or a differing
            ``structuredContent`` payload disqualify the result from
            auto-approval.
            """
            if not isinstance(result, dict):
                return False
            if set(result.keys()) - {"content", "structuredContent"}:
                return False
            if result.get("content") not in ([], None):
                return False
            return result.get("structuredContent") == {"result": "success"}
  new_text: ""
```
/answer e0ee5178-2542-4e72-abbc-aa891e58fb58 allow
ReasoningToken: 113
ReasoningToken: 135
Text:
 Nicht relevant genug – ich belasse `normalize_result` unverändert, da alle Tools `ToolResult` direkt zurückgeben. Jetzt der Manager:
Control Request:
```yaml
id: 3006d3ac-67ec-468e-83fd-656a3d4494db
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  old_text: "            decision = control.submit_result(session, name, result.to_dict())"
  new_text: |2-
                decision = control.submit_result(
                    session, name, result.to_dict(), auto_approve=result.auto_approve
                )
```
/answer 3006d3ac-67ec-468e-83fd-656a3d4494db allow
Control Request:
```yaml
id: 0fb22763-b1d0-47d6-ba37-1333b9e43d14
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
```
/answer 0fb22763-b1d0-47d6-ba37-1333b9e43d14 allow
Control Request:
```yaml
id: 15a04d5e-962e-406e-bb45-edcc818e769b
result:
  structuredContent:
    content: |
      """Bash tool – executes a shell script inside a specified working directory."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult
      from ..process import run_capture

      #: Safety limit on inline STDOUT/STDERR size. Streams larger than this are
      #: written to a temp file instead, and the absolute path is returned so the
      #: caller can keep operating on the output (e.g. via the ``read`` tool).
      _MAX_STREAM_CHARS = 2000


      def register_bash_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "bash",
              title="Run Bash script",
              description=(
                  "Execute a Bash script in the specified working directory. "
                  "Returns the exit code, standard output and, if present, standard error output. "
                  f"As a safety limit, STDOUT/STDERR longer than {_MAX_STREAM_CHARS} characters are "
                  "written to a temp file instead; the absolute file path is returned "
                  "(as 'stdout_file'/'stderr_file') so it can be inspected further."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "cwd": {
                          "type": "string",
                          "description": "Absolute path to the working directory in which to run the script.",
                      },
                      "script": {
                          "type": "string",
                          "description": "Bash script content to execute.",
                      },
                  },
                  "required": ["cwd", "script"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "exit_code": {"type": "integer"},
                      "stdout": {"type": "string"},
                      "stderr": {"type": "string"},
                      "stdout_file": {
                          "type": "string",
                          "description": (
                              "Absolute path to a file containing the full STDOUT, "
                              "present only if STDOUT exceeded the safety limit."
                          ),
                      },
                      "stderr_file": {
                          "type": "string",
                          "description": (
                              "Absolute path to a file containing the full STDERR, "
                              "present only if STDERR exceeded the safety limit."
                          ),
                      },
                  },
                  "required": ["stdout"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": True},
          )
          def bash(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              cwd_str: str = args["cwd"]
              script: str = args["script"]

              cwd = Path(cwd_str)
              if not cwd.is_absolute():
                  return ToolResult(
                      structured_content={"error": f"cwd must be an absolute path."},
                      is_error=True,
                  )
              if not cwd.is_dir():
                  return ToolResult(
                      structured_content={"error": f"Working directory not found or not a directory."},
                      is_error=True,
                  )

              return run_capture(
                  ["bash", "-c", script],
                  cwd=cwd,
                  launch_error="Failed to launch bash",
                  normalize_output=True,
                  omit_zero_exit_code=True,
                  max_stream_chars=_MAX_STREAM_CHARS,
              )
```
/answer 15a04d5e-962e-406e-bb45-edcc818e769b allow
Control Request:
```yaml
id: 696c8b30-ae62-440e-94a7-3d3a64f591d3
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: "grep -rn 'ToolResult(structured_content={\"result\": \"success\"})' src/xy/ai/mcpc/tools"
```
/answer 696c8b30-ae62-440e-94a7-3d3a64f591d3 allow
Control Request:
```yaml
id: e8f4f3bb-a012-4224-8a05-3608729c0679
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
```
/answer e8f4f3bb-a012-4224-8a05-3608729c0679 allow
Control Request:
```yaml
id: c2d977ff-f48e-4e97-bb08-79bcddb57294
result:
  structuredContent:
    stdout: |
      src/xy/ai/mcpc/tools/change/__init__.py:142:        return ToolResult(structured_content={"result": "success"})
      src/xy/ai/mcpc/tools/replace_chars/__init__.py:104:        return ToolResult(structured_content={"result": "success"})
      src/xy/ai/mcpc/tools/insert/__init__.py:93:        return ToolResult(structured_content={"result": "success"})
      src/xy/ai/mcpc/tools/replace_lines/__init__.py:110:        return ToolResult(structured_content={"result": "success"})
      src/xy/ai/mcpc/tools/replace_block/__init__.py:114:        return ToolResult(structured_content={"result": "success"})
      src/xy/ai/mcpc/tools/write/__init__.py:79:        return ToolResult(structured_content={"result": "success"})
```
/answer c2d977ff-f48e-4e97-bb08-79bcddb57294 allow
Control Request:
```yaml
id: 0487aba4-2307-44cc-8765-385eca1bbca6
result:
  structuredContent:
    content: |
      """Python tool – executes a Python script directly from context (no file)."""

      from __future__ import annotations

      import sys
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult
      from ..process import run_capture


      def register_python_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "python",
              title="Run Python script",
              description=(
                  "Execute a Python script passed directly as content, without writing "
                  "a script file. The script is fed to the interpreter on standard input. "
                  "Returns the exit code, standard output and, if present, standard error output."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "script": {
                          "type": "string",
                          "description": "Python script content to execute.",
                      },
                  },
                  "required": ["script"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "exit_code": {"type": "integer"},
                      "stdout": {"type": "string"},
                      "stderr": {"type": "string"},
                  },
                  "required": ["exit_code", "stdout"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": True},
          )
          def python(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              script: str = args["script"]

              return run_capture(
                  [sys.executable, "-"],
                  stdin=script,
                  launch_error="Failed to launch Python",
              )
```
/answer 0487aba4-2307-44cc-8765-385eca1bbca6 allow
Control Request:
```yaml
id: 03052d3d-3e07-4b63-b587-080145b31ae0
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py
```
/answer 03052d3d-3e07-4b63-b587-080145b31ae0 allow
Control Request:
```yaml
id: a142e5d3-fabe-4ae9-9e55-4ab3911a4888
result:
  structuredContent:
    content: |
      """List tool – returns a flat, sorted list of relative file paths below a directory.

      Walks the given absolute directory recursively and returns all file paths
      (files only, no directories) as an alphabetically sorted flat list of paths
      relative to the requested directory. An optional regular expression can be
      supplied to filter the resulting list (matched against each relative file
      path). Common VCS/build/cache directories (e.g. ``.git``) are always excluded.
      To keep results manageable, the number of returned entries is capped; use
      ``pattern`` to narrow down large directories instead of raising the limit.
      """

      from __future__ import annotations

      import os
      import re
      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult

      _MAX_ENTRIES = 50

      _EXCLUDED_DIRS = {
          ".git",
          ".hg",
          ".svn",
          "__pycache__",
          ".mypy_cache",
          ".pytest_cache",
          ".ruff_cache",
          ".tox",
          ".venv",
          "venv",
          "node_modules",
          ".idea",
          ".vscode",
          "dist",
          "build",
          ".cache",
      }


      def register_list_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "list",
              title="List directory",
              description=(
                  "List all files below an absolute directory path, recursively, "
                  "as a flat list. "
                  "Optionally filter the result with a regular expression."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute directory path.",
                      },
                      "pattern": {
                          "type": "string",
                          "description": (
                              "Optional regular expression used to filter the result."
                          ),
                      },
                  },
                  "required": ["path"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "entries": {
                          "type": "array",
                          "items": {"type": "string"},
                      },
                  },
                  "required": ["entries"],
              },
              annotations={"readOnlyHint": True, "openWorldHint": False},
          )
          def list_dir(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              pattern: str | None = args.get("pattern")

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "Directory not found."},
                      is_error=True,
                  )
              if not path.is_dir():
                  return ToolResult(
                      structured_content={"error": "Not a directory."},
                      is_error=True,
                  )

              regex: re.Pattern[str] | None = None
              if pattern is not None:
                  try:
                      regex = re.compile(pattern)
                  except re.error as exc:
                      return ToolResult(
                          structured_content={"error": f"Invalid regular expression: {exc}"},
                          is_error=True,
                      )

              root = path.resolve()
              entries: list[str] = []
              for dirpath, dirnames, filenames in os.walk(root):
                  dirnames[:] = [d for d in dirnames if d not in _EXCLUDED_DIRS]
                  for filename in filenames:
                      file_path = Path(dirpath) / filename
                      rel_path = str(file_path.relative_to(root))
                      if regex is not None and not regex.search(rel_path):
                          continue
                      entries.append(rel_path)

              entries.sort()

              if len(entries) > _MAX_ENTRIES:
                  return ToolResult(
                      structured_content={
                          "error": (
                              f"Too many entries ({len(entries)}) exceed the limit of "
                              f"{_MAX_ENTRIES}. Narrow down the result using the "
                              "'pattern' regular expression parameter."
                          )
                      },
                      is_error=True,
                  )

              return ToolResult(structured_content={"entries": entries})
```
/answer a142e5d3-fabe-4ae9-9e55-4ab3911a4888 allow
Control Request:
```yaml
id: 6374a121-4186-456f-9255-374da9f66089
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |-
    for f in src/xy/ai/mcpc/tools/change/__init__.py src/xy/ai/mcpc/tools/replace_chars/__init__.py src/xy/ai/mcpc/tools/insert/__init__.py src/xy/ai/mcpc/tools/replace_lines/__init__.py src/xy/ai/mcpc/tools/replace_block/__init__.py src/xy/ai/mcpc/tools/write/__init__.py; do
      sed -i 's/return ToolResult(structured_content={"result": "success"})/return ToolResult(structured_content={"result": "success"}, auto_approve=True)/' "$f"
    done
    grep -rn 'auto_approve=True' src/xy/ai/mcpc/tools
```
/answer 6374a121-4186-456f-9255-374da9f66089 allow
Control Request:
```yaml
id: 401f069c-b84f-4cf5-bbf6-2041e6683b34
result:
  structuredContent:
    stdout: |
      src/xy/ai/mcpc/tools/change/__init__.py:142:        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
      src/xy/ai/mcpc/tools/replace_chars/__init__.py:104:        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
      src/xy/ai/mcpc/tools/insert/__init__.py:93:        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
      src/xy/ai/mcpc/tools/replace_lines/__init__.py:110:        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
      src/xy/ai/mcpc/tools/replace_block/__init__.py:114:        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
      src/xy/ai/mcpc/tools/write/__init__.py:79:        return ToolResult(structured_content={"result": "success"}, auto_approve=True)
```
/answer 401f069c-b84f-4cf5-bbf6-2041e6683b34 allow
Control Request:
```yaml
id: 4dbcfce1-7d19-44b6-9258-b9db5bcb747c
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  start: |-
    @dataclass(slots=True)
    class ToolResult:
  end: '        return result'
```
/answer 4dbcfce1-7d19-44b6-9258-b9db5bcb747c allow
Control Request:
```yaml
id: 54b95a52-a63e-481d-a77f-f157ac7599aa
result:
  structuredContent:
    content: |-
      @dataclass(slots=True)
      class ToolResult:
          """The result of a tool call (maps onto MCP ``CallToolResult``)."""

          content: list[dict[str, Any]] = field(default_factory=list)
          structured_content: dict[str, Any] | None = None
          is_error: bool = False
          auto_approve: bool = False
          """Tool-side hint: skip human review of this result during the
          ``result`` control phase, since the tool itself judges there is
          nothing meaningful to approve (e.g. a plain success acknowledgement,
          or a read that returns an unrestricted full file). Never serialised
          to the client; consumed only by :class:`ToolControlManager`."""

          def to_dict(self) -> dict[str, Any]:
              result: dict[str, Any] = {}
              if self.content:
                  result["content"] = list(self.content)
              if self.structured_content:
                  result["structuredContent"] = self.structured_content
              if self.is_error:
                  result["isError"] = True
              return result
```
/answer 54b95a52-a63e-481d-a77f-f157ac7599aa allow
Control Request:
```yaml
id: 3df4f4f8-1b22-4c1e-8ded-d1d923e44876
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  old_text: |2-
            sliced = text[region_start:region_end]
            structured: dict[str, Any] = {"content": sliced}

            return ToolResult(structured_content=structured)
  new_text: |2-
            sliced = text[region_start:region_end]
            structured: dict[str, Any] = {"content": sliced}

            # An unrestricted read (no line/marker range given) returns the
            # entire file verbatim; there is nothing a human reviewer could
            # meaningfully approve or reject beyond what a plain file read
            # already exposes, so the tool flags it for auto-approval.
            is_full_file = (
                min_line is None
                and max_line is None
                and start_marker is None
                and end_marker is None
            )

            return ToolResult(structured_content=structured, auto_approve=is_full_file)
```
/answer 3df4f4f8-1b22-4c1e-8ded-d1d923e44876 allow
Control Request:
```yaml
id: 0378da8a-4e79-43a1-a06b-4049976953e1
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
```
/answer 0378da8a-4e79-43a1-a06b-4049976953e1 allow
Control Request:
```yaml
id: 172b7f59-dcb8-447e-b145-224a806a6645
result:
  structuredContent:
    content: |
      """Central tool registry and tool result helpers.

      Tools are registered once in a process-wide :class:`ToolRegistry`.  What a
      given client actually sees is derived by reconciling the registry against the
      per-session configuration (:attr:`Session.enabled_tools`).
      """

      from __future__ import annotations

      from dataclasses import dataclass, field
      from typing import TYPE_CHECKING, Any, Callable

      from .codec import JsonCodec
      from .session import Session

      if TYPE_CHECKING:
          from .context import AppServices


      @dataclass(slots=True)
      class ToolContext:
          """Context handed to a tool handler on invocation."""

          session: Session
          arguments: dict[str, Any]
          #: Shared process-wide services (session store, CLI manager, profiles).
          #: ``None`` for tools that never orchestrate other sessions.
          services: "AppServices | None" = None


      @dataclass(slots=True)
      class ToolResult:
          """The result of a tool call (maps onto MCP ``CallToolResult``)."""

          content: list[dict[str, Any]] = field(default_factory=list)
          structured_content: dict[str, Any] | None = None
          is_error: bool = False
          auto_approve: bool = False
          """Tool-side hint: skip human review of this result during the
          ``result`` control phase, since the tool itself judges there is
          nothing meaningful to approve (e.g. a plain success acknowledgement,
          or a read that returns an unrestricted full file). Never serialised
          to the client; consumed only by :class:`ToolControlManager`."""

          def to_dict(self) -> dict[str, Any]:
              result: dict[str, Any] = {}
              if self.content:
                  result["content"] = list(self.content)
              if self.structured_content:
                  result["structuredContent"] = self.structured_content
              if self.is_error:
                  result["isError"] = True
              return result


      def text_content(text: str) -> dict[str, Any]:
          """Build a ``TextContent`` block."""
          return {"type": "text", "text": text}


      #: A handler receives the invocation context and returns one of:
      #: * a :class:`ToolResult`,
      #: * a ``str`` (wrapped as a single text content block),
      #: * a ``dict`` (treated as structured content, also rendered as JSON text).
      ToolHandler = Callable[[ToolContext], "ToolResult | str | dict[str, Any]"]


      @dataclass(slots=True)
      class Tool:
          """A registered tool and its MCP metadata."""

          name: str
          description: str
          input_schema: dict[str, Any]
          handler: ToolHandler
          title: str | None = None
          output_schema: dict[str, Any] | None = None
          annotations: dict[str, Any] | None = None

          def to_spec(self) -> dict[str, Any]:
              """Return the MCP ``Tool`` object advertised via ``tools/list``."""
              spec: dict[str, Any] = {
                  "name": self.name,
                  "description": self.description,
                  "inputSchema": self.input_schema,
              }
              if self.title is not None:
                  spec["title"] = self.title
              if self.output_schema is not None:
                  spec["outputSchema"] = self.output_schema
              if self.annotations is not None:
                  spec["annotations"] = self.annotations
              return spec


      def normalize_result(value: "ToolResult | str | dict[str, Any] | None") -> ToolResult:
          """Coerce whatever a handler returned into a :class:`ToolResult`."""
          if isinstance(value, ToolResult):
              return value
          if value is None:
              return ToolResult(content=[])
          if isinstance(value, str):
              return ToolResult(content=[text_content(value)])
          if isinstance(value, dict):
              # A dict already shaped like a CallToolResult is passed through.
              if "content" in value and isinstance(value["content"], list):
                  return ToolResult(
                      content=value["content"],
                      structured_content=value.get("structuredContent"),
                      is_error=bool(value.get("isError", False)),
                  )
              # Otherwise treat the dict as structured content.
              rendered = JsonCodec.encode(value)
              return ToolResult(content=[text_content(rendered)], structured_content=value)
          raise TypeError(f"Unsupported tool return type: {type(value)!r}")


      class ToolRegistry:
          """Process-wide registry of available tools."""

          def __init__(self) -> None:
              self._tools: dict[str, Tool] = {}

          def register(self, tool: Tool) -> Tool:
              if tool.name in self._tools:
                  raise ValueError(f"Tool already registered: {tool.name}")
              self._tools[tool.name] = tool
              return tool

          def tool(
              self,
              name: str,
              *,
              description: str,
              input_schema: dict[str, Any],
              title: str | None = None,
              output_schema: dict[str, Any] | None = None,
              annotations: dict[str, Any] | None = None,
          ) -> Callable[[ToolHandler], ToolHandler]:
              """Decorator registering the decorated function as a tool handler."""

              def decorator(handler: ToolHandler) -> ToolHandler:
                  self.register(
                      Tool(
                          name=name,
                          description=description,
                          input_schema=input_schema,
                          handler=handler,
                          title=title,
                          output_schema=output_schema,
                          annotations=annotations,
                      )
                  )
                  return handler

              return decorator

          def get(self, name: str) -> Tool | None:
              return self._tools.get(name)

          def __contains__(self, name: object) -> bool:
              return name in self._tools

          def __len__(self) -> int:
              return len(self._tools)

          def names(self) -> list[str]:
              return list(self._tools)

          def list_for_session(self, session: Session) -> list[Tool]:
              """Reconcile the registry with the session's enabled-tool configuration.

              Returns the tools the session is allowed to see, sorted by name for a
              stable pagination order.
              """
              tools = [t for t in self._tools.values() if session.is_tool_enabled(t.name)]
              tools.sort(key=lambda t: t.name)
              return tools
```
/answer 172b7f59-dcb8-447e-b145-224a806a6645 allow
Control Request:
```yaml
id: aad7d081-de22-40ef-bfbe-f9dc163ec304
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |-
    grep -rn "simple.*success\|_is_simple_success_result" src/xy/ai/mcpc --include=*.py
    python -m py_compile src/xy/ai/mcpc/control/manager.py src/xy/ai/mcpc/protocol.py src/xy/ai/mcpc/registry.py src/xy/ai/mcpc/tools/read/__init__.py src/xy/ai/mcpc/tools/change/__init__.py src/xy/ai/mcpc/tools/replace_chars/__init__.py src/xy/ai/mcpc/tools/insert/__init__.py src/xy/ai/mcpc/tools/replace_lines/__init__.py src/xy/ai/mcpc/tools/replace_block/__init__.py src/xy/ai/mcpc/tools/write/__init__.py && echo OK
```
/answer aad7d081-de22-40ef-bfbe-f9dc163ec304 allow
Control Request:
```yaml
id: 67232675-561d-4f53-aad7-8d585c21958a
result:
  structuredContent:
    stdout: |
      OK
```
/answer 67232675-561d-4f53-aad7-8d585c21958a allow
Control Request:
```yaml
id: 06b8d416-cd08-4603-a47b-d8d41df81d82
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: grep -rn "submit_result\|auto_approve" src/xy/ai/mcpc/control/manager.py src/xy/ai/mcpc/protocol.py src/xy/ai/mcpc/registry.py
```
/answer 06b8d416-cd08-4603-a47b-d8d41df81d82 allow
Control Request:
```yaml
id: 1a1ba265-943f-48a1-8f7f-c5e940c1897a
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
```
/answer 1a1ba265-943f-48a1-8f7f-c5e940c1897a allow
Control Request:
```yaml
id: 9f5bad48-e030-4ef0-8d7d-7fb8831cd817
result:
  structuredContent:
    stdout: |
      src/xy/ai/mcpc/control/manager.py:118:    def submit_result(
      src/xy/ai/mcpc/control/manager.py:124:        auto_approve: bool = False,
      src/xy/ai/mcpc/control/manager.py:132:        ``auto_approve`` is a hint set by the *tool itself* (via
      src/xy/ai/mcpc/control/manager.py:133:        ``ToolResult.auto_approve``), not derived from the shape of the
      src/xy/ai/mcpc/control/manager.py:139:        if auto_approve:
      src/xy/ai/mcpc/protocol.py:211:            decision = control.submit_result(
      src/xy/ai/mcpc/protocol.py:212:                session, name, result.to_dict(), auto_approve=result.auto_approve
      src/xy/ai/mcpc/registry.py:38:    auto_approve: bool = False
```
/answer 9f5bad48-e030-4ef0-8d7d-7fb8831cd817 allow
Control Request:
```yaml
id: 06b28752-5c55-4a0c-9e3c-edde56b16eb7
result:
  structuredContent:
    content: |
      """Human-in-the-loop tool control manager.

      Implements the Manager Pattern for intercepting tool calls at two points:

      1. *Before* execution (``phase="request"``): the interceptor may approve,
         modify arguments, or reject the call entirely.
      2. *After* execution (``phase="result"``): the interceptor may approve,
         replace the tool output, or inject instructions as the result.

      The control endpoint at ``/control/tool`` lets an external client poll for
      pending items and post approval decisions.  The intercepting threads block on
      per-item ``threading.Event`` objects until a decision arrives.
      """

      from __future__ import annotations

      import logging
      import threading
      import uuid
      from dataclasses import dataclass, field
      from typing import Any
      from xy.ai.mcpc.session import Session

      logger = logging.getLogger("xy.ai.mcpc.control")

      # How long (seconds) an intercepted call waits before timing out and auto-approving.
      _DEFAULT_TIMEOUT = 24 * 60 * 60.0  # 24 h — matches agent MCP timeout


      # ---------------------------------------------------------------------------
      # Decision DTO
      # ---------------------------------------------------------------------------

      @dataclass(slots=True)
      class ControlDecision:
          """The outcome of a human review, produced by :meth:`ToolControlManager.process_approvals`."""

          approved: bool
          """True when the call should proceed (possibly with modified data)."""

          rejection_reason: str | None = None
          """Human-readable hint for the agent when ``approved`` is False."""

          modified_arguments: dict[str, Any] | None = None
          """Replacement arguments for the ``request`` phase (``None`` → keep originals)."""

          modified_result: dict[str, Any] | None = None
          """Replacement result dict for the ``result`` phase (``None`` → keep original)."""


      # ---------------------------------------------------------------------------
      # Internal pending item
      # ---------------------------------------------------------------------------

      @dataclass
      class _PendingItem:
          id: str
          phase: str          # "request" | "result"
          tool_name: str
          arguments: dict[str, Any] | None       # populated in request phase
          result: dict[str, Any] | None          # populated in result phase
          _event: threading.Event = field(default_factory=threading.Event, repr=False)
          _decision: ControlDecision | None = field(default=None, repr=False)

          def to_dict(self) -> dict[str, Any]:
              item: dict[str, Any] = {"id": self.id}
              if self.arguments is not None:
                  item["toolName"] = self.tool_name
                  item["arguments"] = self.arguments
              if self.result is not None:
                  item["result"] = self.result
              return item


      # ---------------------------------------------------------------------------
      # Manager
      # ---------------------------------------------------------------------------

      class ToolControlManager:
          """Thread-safe manager for human-in-the-loop tool interception.

          Usage::

              manager = ToolControlManager()

              # In the interceptor thread (blocks until a decision is received):
              decision = manager.submit_request("bash", {"command": "rm -rf /"})

              # In the control endpoint handler (non-blocking):
              pending = manager.get_pending()
              manager.process_approvals([{"id": "…", "approved": True}])
          """

          def __init__(self, timeout: float = _DEFAULT_TIMEOUT) -> None:
              self._timeout = timeout
              self._pending: dict[str, _PendingItem] = {}
              self._lock = threading.Lock()

          # ------------------------------------------------------------------
          # Interceptor-facing API (blocking)
          # ------------------------------------------------------------------

          def submit_request(
              self,
              session: Session,
              tool_name: str,
              arguments: dict[str, Any],
          ) -> ControlDecision:
              """Block until the controller approves/rejects a tool-call *request*.

              Returns a :class:`ControlDecision`.  If the decision includes
              ``modified_arguments``, the caller should substitute them before
              invoking the tool handler.
              """
              item = self._enqueue(session, "request", tool_name, arguments=arguments, result=None)
              return self._wait(item)

          def submit_result(
              self,
              session: Session,
              tool_name: str,
              result: dict[str, Any],
              *,
              auto_approve: bool = False,
          ) -> ControlDecision:
              """Block until the controller approves/replaces a tool-call *result*.

              Returns a :class:`ControlDecision`.  If the decision includes
              ``modified_result``, the caller should use that instead of the
              original result.

              ``auto_approve`` is a hint set by the *tool itself* (via
              ``ToolResult.auto_approve``), not derived from the shape of the
              result. A tool sets it when it judges its own result carries
              nothing meaningful for a human to review (e.g. a plain success
              acknowledgement, or an unrestricted full-file read). When set, the
              result is approved without involving the controller.
              """
              if auto_approve:
                  logger.info(
                      "Auto-approving result for %s [%s] (tool-flagged)",
                      tool_name, session.id,
                  )
                  return ControlDecision(approved=True)

              item = self._enqueue(session, "result", tool_name, arguments=None, result=result)
              return self._wait(item)

          # ------------------------------------------------------------------
          # Control-endpoint-facing API (non-blocking)
          # Control-endpoint-facing API (non-blocking)
          # ------------------------------------------------------------------

          def get_pending(self) -> list[dict[str, Any]]:
              """Return serialisable snapshots of all items still awaiting a decision."""
              with self._lock:
                  return [item.to_dict() for item in self._pending.values()]

          def process_approvals(self, approvals: list[dict[str, Any]]) -> None:
              """Apply a batch of approval decisions from the control client.

              Each entry in *approvals* must have at minimum an ``"id"`` key.
              Accepted forms:

              * ``{"id": "…"}`` — simple approval, keep original data.
              * ``{"id": "…", "rejected": true, "reason": "…"}`` — rejection.
              * ``{"id": "…", "arguments": {…}}`` — approve with modified arguments.
              * ``{"id": "…", "result": {…}}`` — approve with replaced result.
              """
              for approval in approvals:
                  item_id = approval.get("id")
                  if not isinstance(item_id, str):
                      logger.warning("Approval entry missing 'id', skipped: %s", approval)
                      continue
                  with self._lock:
                      item = self._pending.get(item_id)
                  if item is None:
                      logger.warning("Unknown approval id %s, skipped", item_id)
                      continue

                  if approval.get("rejected"):
                      decision = ControlDecision(
                          approved=False,
                          rejection_reason=approval.get("reason") or "Rejected by controller",
                      )
                  else:
                      decision = ControlDecision(
                          approved=True,
                          modified_arguments=approval.get("arguments"),
                          modified_result=approval.get("result"),
                      )

                  item._decision = decision
                  with self._lock:
                      self._pending.pop(item_id, None)
                  logger.info(
                      "Dequeued control item %s [%s/%s]: approved=%s",
                      item.tool_name, item.phase, item_id, decision.approved,
                  )
                  item._event.set()

          # ------------------------------------------------------------------
          # Internal helpers
          # ------------------------------------------------------------------


          def _enqueue(
              self,
              session: Session,
              phase: str,
              tool_name: str,
              arguments: dict[str, Any] | None,
              result: dict[str, Any] | None,
          ) -> _PendingItem:
              item_id = str(uuid.uuid4())
              item = _PendingItem(
                  id=item_id,
                  phase=phase,
                  tool_name=tool_name,
                  arguments=arguments,
                  result=result,
              )
              with self._lock:
                  self._pending[item_id] = item
              logger.info("Enqueued control item %s [%s/%s/%s]", tool_name, phase, session.id, item_id)
              return item

          def _wait(self, item: _PendingItem) -> ControlDecision:
              signalled = item._event.wait(timeout=self._timeout)
              with self._lock:
                  self._pending.pop(item.id, None)

              if not signalled or item._decision is None:
                  # Timeout — auto-approve to avoid hanging the agent forever.
                  logger.warning(
                      "Control item %s [%s/%s] timed out, auto-approving",
                      item.tool_name, item.phase, item.id,
                  )
                  return ControlDecision(approved=True)

              return item._decision
```
/answer 06b28752-5c55-4a0c-9e3c-edde56b16eb7 allow
Control Request:
```yaml
id: 47a29daa-6fdd-43d0-bb02-2a2cfba40447
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
```
/answer 47a29daa-6fdd-43d0-bb02-2a2cfba40447 allow
Control Request:
```yaml
id: f0ba4a6d-f629-40b0-bbba-58168f17aa2d
result:
  structuredContent:
    content: |
      """MCP protocol logic (lifecycle + tools feature).

      This module is transport-agnostic: it takes a parsed JSON-RPC request plus the
      owning :class:`Session` and returns a result payload (or raises
      :class:`JsonRpcError`).  The Streamable HTTP transport wraps the return value
      in a JSON-RPC envelope.
      """

      from __future__ import annotations

      import logging
      import base64
      from typing import TYPE_CHECKING, Any

      from . import errors
      from .config import ServerConfig
      from .jsonrpc import JsonRpcRequest
      from .registry import ToolContext, ToolRegistry, normalize_result
      from .session import Session

      logger = logging.getLogger("xy.ai.mcpc.protocol")

      if TYPE_CHECKING:
          from .context import AppServices

      # Methods a client may call before the initialize handshake has completed.
      _PRE_INIT_METHODS = {"initialize", "ping"}


      def _encode_cursor(offset: int) -> str:
          return base64.urlsafe_b64encode(str(offset).encode("ascii")).decode("ascii")


      def _decode_cursor(cursor: str) -> int:
          try:
              offset = int(base64.urlsafe_b64decode(cursor.encode("ascii")).decode("ascii"))
          except (ValueError, TypeError):
              raise errors.invalid_params("Invalid pagination cursor", {"cursor": cursor})
          if offset < 0:
              raise errors.invalid_params("Invalid pagination cursor", {"cursor": cursor})
          return offset


      class McpProtocol:
          """Dispatches MCP methods against a session."""

          def __init__(
              self,
              config: ServerConfig,
              registry: ToolRegistry,
              services: "AppServices | None" = None,
          ) -> None:
              self.config = config
              self.registry = registry
              self.services = services
              self._handlers = {
                  "initialize": self._handle_initialize,
                  "ping": self._handle_ping,
                  "tools/list": self._handle_tools_list,
                  "tools/call": self._handle_tools_call,
              }

          # -- Request handling ---------------------------------------------------
          def handle_request(
              self,
              session: Session,
              request: JsonRpcRequest,
              *,
              skip_control: bool = False,
          ) -> Any:
              """Handle a JSON-RPC *request* and return its ``result`` payload.

              ``skip_control`` suppresses tool interception for this request,
              regardless of whether a :class:`ToolControlManager` is configured.
              It is set when the caller sends ``X-MCPC-CONTROL: off``.
              """
              handler = self._handlers.get(request.method)
              if handler is None:
                  raise errors.method_not_found(request.method)

              if request.method not in _PRE_INIT_METHODS and not session.handshake_complete:
                  raise errors.JsonRpcError(
                      errors.NOT_INITIALIZED,
                      "Session is not initialized; send an 'initialize' request first",
                  )
              if request.method == "tools/call":
                  return self._handle_tools_call(session, request.params, skip_control=skip_control)
              return handler(session, request.params)

          def handle_notification(self, session: Session, request: JsonRpcRequest) -> None:
              """Handle a JSON-RPC *notification*.

              Notifications are not "supported" in the sense that the server never
              acts on arbitrary ones and never emits any; the lifecycle
              ``notifications/initialized`` is accepted to complete the handshake.
              """
              if request.method == "notifications/initialized":
                  with session.lock:
                      session.initialized = True
              # All other notifications are silently accepted and ignored.

          # -- Lifecycle ----------------------------------------------------------
          def _handle_initialize(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
              requested = params.get("protocolVersion")
              if not isinstance(requested, str):
                  raise errors.invalid_params('"protocolVersion" is required')

              if requested in self.config.supported_protocol_versions:
                  negotiated = requested
              else:
                  negotiated = self.config.preferred_protocol_version

              with session.lock:
                  session.protocol_version = negotiated
                  session.client_info = params.get("clientInfo")
                  session.client_capabilities = params.get("capabilities")
                  session.touch()

              return {
                  "protocolVersion": negotiated,
                  "capabilities": {
                      # Only the tools feature is offered; listChanged is false since
                      # notifications are unsupported.
                      "tools": {"listChanged": False},
                  },
                  "serverInfo": {
                      "name": self.config.server_name,
                      "title": self.config.server_title,
                      "version": self.config.server_version,
                  },
                  "instructions": self.config.instructions,
              }

          def _handle_ping(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
              return {}

          # -- Tools --------------------------------------------------------------
          def _handle_tools_list(self, session: Session, params: dict[str, Any]) -> dict[str, Any]:
              tools = self.registry.list_for_session(session)

              cursor = params.get("cursor")
              start = _decode_cursor(cursor) if cursor is not None else 0
              page_size = self.config.tools_page_size
              page = tools[start : start + page_size]

              result: dict[str, Any] = {"tools": [t.to_spec() for t in page]}
              if start + page_size < len(tools):
                  result["nextCursor"] = _encode_cursor(start + page_size)
              return result

          def _handle_tools_call(
              self,
              session: Session,
              params: dict[str, Any],
              *,
              skip_control: bool = False,
          ) -> dict[str, Any]:
              name = params.get("name")
              if not isinstance(name, str) or not name:
                  raise errors.invalid_params('"name" is required')

              arguments = params.get("arguments", {})
              if arguments is None:
                  arguments = {}
              if not isinstance(arguments, dict):
                  raise errors.invalid_params('"arguments" must be an object')

              tool = self.registry.get(name)
              # "Errors in finding the tool" are protocol errors (spec, tools/call).
              if tool is None or not session.is_tool_enabled(name):
                  raise errors.invalid_params(
                      f"Unknown or unavailable tool: {name}", {"name": name}
                  )

              _validate_arguments(tool.input_schema, arguments)

              # --- request interception -------------------------------------------
              control = self.services.control_manager if self.services else None
              if control is not None and not skip_control:
                  decision = control.submit_request(session, name, arguments)
                  if not decision.approved:
                      from .registry import ToolResult, text_content
                      reason = decision.rejection_reason or "Tool call rejected by controller"
                      return ToolResult(
                          content=[text_content(reason)],
                          is_error=True,
                      ).to_dict()
                  if decision.modified_arguments is not None:
                      arguments = decision.modified_arguments
              # --------------------------------------------------------------------

              context = ToolContext(session=session, arguments=arguments, services=self.services)
              # Tool execution errors are reported *inside* the result (isError=true)
              # so the model can see and self-correct, not as protocol errors.
              try:
                  with session.lock:
                      raw = tool.handler(context)
                  result = normalize_result(raw)
              except errors.JsonRpcError:
                  raise
              except Exception as exc:  # noqa: BLE001 - surface as tool error result
                  from .registry import ToolResult, text_content

                  result = ToolResult(
                      content=[text_content(f"Tool '{name}' failed: {exc}")],
                      is_error=True,
                  )

              # --- result interception --------------------------------------------
              if control is not None and not skip_control:
                  decision = control.submit_result(
                      session, name, result.to_dict(), auto_approve=result.auto_approve
                  )
                  if not decision.approved:
                      from .registry import ToolResult, text_content
                      reason = decision.rejection_reason or "Tool result rejected by controller"
                      return ToolResult(
                          content=[text_content(reason)],
                          is_error=True,
                      ).to_dict()
                  if decision.modified_result is not None:
                      return decision.modified_result
              # --------------------------------------------------------------------

              return result.to_dict()


      def _validate_arguments(schema: dict[str, Any], arguments: dict[str, Any]) -> None:
          """Minimal validation of *arguments* against an input JSON Schema.

          Only the top-level ``required`` list and primitive ``type`` of declared
          properties are checked — enough to give clients meaningful ``INVALID_PARAMS``
          errors without pulling in a full JSON Schema implementation.
          """
          required = schema.get("required", [])
          missing = [key for key in required if key not in arguments]
          if missing:
              raise errors.invalid_params(
                  f"Missing required argument(s): {', '.join(missing)}",
                  {"missing": missing},
              )

          properties = schema.get("properties", {})
          type_checks = {
              "string": str,
              "number": (int, float),
              "integer": int,
              "boolean": bool,
              "object": dict,
              "array": list,
          }
          for key, value in arguments.items():
              prop = properties.get(key)
              if not isinstance(prop, dict):
                  continue
              expected = prop.get("type")
              py_type = type_checks.get(expected) if isinstance(expected, str) else None
              if py_type is None:
                  continue
              # bool is a subclass of int; guard the integer/number cases explicitly.
              if expected in ("number", "integer") and isinstance(value, bool):
                  ok = False
              else:
                  ok = isinstance(value, py_type)
              if not ok:
                  raise errors.invalid_params(
                      f"Argument '{key}' must be of type {expected}",
                      {"argument": key, "expectedType": expected},
                  )
```
/answer f0ba4a6d-f629-40b0-bbba-58168f17aa2d allow
ReasoningToken: 105
Control Request:
```yaml
id: 08a27c9d-7cd5-4560-8a4e-df81ea77324d
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/write/__init__.py
```
/answer 08a27c9d-7cd5-4560-8a4e-df81ea77324d allow
Control Request:
```yaml
id: 8735cd9b-bc09-42b1-8eee-f20d5563da14
result:
  structuredContent:
    content: |
      """Write tool – writes a file completely or appends lines to it."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_write_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "write",
              title="Write file",
              description=(
                  "Write content to a file. "
                  "In ``replace`` mode the file is overwritten with the supplied content. "
                  "In ``append`` mode the content is added at the end of the existing file "
                  "(the file is created if it does not yet exist)."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the file to write.",
                      },
                      "mode": {
                          "type": "string",
                          "enum": ["replace", "append"],
                          "description": (
                              "``replace`` – overwrite the file with the new content. "
                              "``append`` – add the new content after the existing content."
                          ),
                      },
                      "content": {
                          "type": "string",
                          "description": "Text to write to the file.",
                      },
                  },
                  "required": ["path", "mode", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                  },
                  "required": ["result"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def write(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              mode: str = args["mode"]
              content: str = args["content"]

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )

              try:
                  path.parent.mkdir(parents=True, exist_ok=True)
                  file_mode = "a" if mode == "append" else "w"
                  with path.open(file_mode, encoding="utf-8") as fh:
                      fh.write(content)
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Write failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"}, auto_approve=True)
```
/answer 8735cd9b-bc09-42b1-8eee-f20d5563da14 allow
Control Request:
```yaml
id: cb3f3279-1a2a-43e8-b831-f18ad123e00f
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py
```
/answer cb3f3279-1a2a-43e8-b831-f18ad123e00f allow
Control Request:
```yaml
id: ac27219b-96cd-4086-8dc2-6c6aa2d43334
result:
  structuredContent:
    content: |
      """Insert tool – inserts text at a character offset inside an existing file."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_insert_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "insert",
              title="Insert into file",
              description=(
                  "Insert text at a specific character offset inside an existing file. "
                  "The offset is zero-based and refers to the UTF-8 decoded content of the file. "
                  "All existing content at and after the offset is shifted right."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the file to modify.",
                      },
                      "offset": {
                          "type": "integer",
                          "description": "Zero-based character offset at which to insert the new content.",
                          "minimum": 0,
                      },
                      "content": {
                          "type": "string",
                          "description": "Text to insert at the given offset.",
                      },
                  },
                  "required": ["path", "offset", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                  },
                  "required": ["result"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def insert(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              offset: int = args["offset"]
              new_content: str = args["content"]

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              try:
                  text = path.read_text(encoding="utf-8")
                  if offset > len(text):
                      return ToolResult(
                          structured_content={
                              "error": (
                                  f"Offset is beyond end of file."
                              )
                          },
                          is_error=True,
                      )
                  result = text[:offset] + new_content + text[offset:]
                  path.write_text(result, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Insert failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"}, auto_approve=True)
```
/answer ac27219b-96cd-4086-8dc2-6c6aa2d43334 allow
Control Request:
```yaml
id: 4e30f081-ba1e-4bb4-902c-e50ce76e23eb
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/change/__init__.py
```
/answer 4e30f081-ba1e-4bb4-902c-e50ce76e23eb allow
Control Request:
```yaml
id: 4f06225f-9662-4041-b8cd-56fe14748694
result:
  structuredContent:
    content: |
      """Change tool – replaces the block between start/end markers (both inclusive)."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_change_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "change",
              title="Change file block",
              description=(
                  "Replace the text between 'start' and 'end' (both included) with "
                  "'content'. Each marker must occur exactly once in the file; "
                  "'end' must come after 'start'. Repeat a marker inside 'content' "
                  "to keep it."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the target file.",
                      },
                      "start": {
                          "type": "string",
                          "description": "Unique substring marking the block's start (must occur exactly once).",
                      },
                      "end": {
                          "type": "string",
                          "description": "Unique substring marking the block's end (must occur exactly once, after 'start').",
                      },
                      "content": {
                          "type": "string",
                          "description": "Text that replaces the block, including where 'start'/'end' were.",
                      },
                  },
                  "required": ["path", "start", "end", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                      "error": {
                          "type": "string",
                          "description": "Human-readable error message (only present when is_error is true).",
                      },
                  },
                  "required": [],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def change(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              start_marker: str = args["start"]
              end_marker: str = args["end"]
              new_content: str = args["content"]

              # --- path validation ---
              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              text = path.read_text(encoding="utf-8")

              # --- locate and validate start marker ---
              start_count = text.count(start_marker)
              if start_count == 0:
                  return ToolResult(
                      structured_content={"error": "Start marker not found in file."},
                      is_error=True,
                  )
              if start_count > 1:
                  return ToolResult(
                      structured_content={
                          "error": f"Start marker is ambiguous – found {start_count} occurrences in file."
                      },
                      is_error=True,
                  )

              # --- locate and validate end marker ---
              end_count = text.count(end_marker)
              if end_count == 0:
                  return ToolResult(
                      structured_content={"error": "End marker not found in file."},
                      is_error=True,
                  )
              if end_count > 1:
                  return ToolResult(
                      structured_content={
                          "error": f"End marker is ambiguous – found {end_count} occurrences in file."
                      },
                      is_error=True,
                  )

              start_pos = text.index(start_marker)
              end_pos = text.index(end_marker)

              # --- order validation ---
              if end_pos <= start_pos:
                  return ToolResult(
                      structured_content={
                          "error": (
                              f"End marker must appear after start marker."
                          )
                      },
                      is_error=True,
                  )

              # --- apply replacement: both markers included (full range) ---
              result_text = text[:start_pos] + new_content + text[end_pos + len(end_marker):]

              # --- write back ---
              try:
                  path.write_text(result_text, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Write failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"}, auto_approve=True)
```
/answer 4f06225f-9662-4041-b8cd-56fe14748694 allow
Control Request:
```yaml
id: 05ffb3d9-0951-4259-83c2-dabcc7784e64
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_chars/__init__.py
```
/answer 05ffb3d9-0951-4259-83c2-dabcc7784e64 allow
Control Request:
```yaml
id: 04de1c87-1766-495d-b2ab-e3eb0f1e2f6f
result:
  structuredContent:
    content: |
      """Replace-chars tool – replaces a character range inside an existing file."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_replace_chars_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "replace-chars",
              title="Replace characters in file",
              description=(
                  "Replace a range of characters inside an existing file with new content. "
                  "The range is defined by a zero-based character ``offset`` and a ``length`` "
                  "(number of characters to remove starting at the offset). "
                  "The supplied ``content`` is written in place of the removed range. "
                  "To replace whole lines instead, use ``replace-lines``."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the file to modify.",
                      },
                      "offset": {
                          "type": "integer",
                          "description": "Zero-based character offset of the first character to replace.",
                          "minimum": 0,
                      },
                      "length": {
                          "type": "integer",
                          "description": "Number of characters to remove starting at ``offset``.",
                          "minimum": 0,
                      },
                      "content": {
                          "type": "string",
                          "description": "Replacement text (may be empty to perform a pure deletion).",
                      },
                  },
                  "required": ["path", "offset", "length", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                  },
                  "required": ["result"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def replace_chars(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              offset: int = args["offset"]
              length: int = args["length"]
              new_content: str = args["content"]

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              try:
                  text = path.read_text(encoding="utf-8")
                  file_len = len(text)
                  if offset > file_len:
                      return ToolResult(
                          structured_content={
                              "error": (
                                  f"Offset {offset} is beyond end of file "
                                  f"(file length: {file_len} characters)."
                              )
                          },
                          is_error=True,
                      )
                  end = min(offset + length, file_len)
                  result = text[:offset] + new_content + text[end:]
                  path.write_text(result, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Replace failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"}, auto_approve=True)
```
/answer 04de1c87-1766-495d-b2ab-e3eb0f1e2f6f allow
Control Request:
```yaml
id: f82d9f7e-3057-46ad-9c5c-aa0ab00e2a7a
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_lines/__init__.py
```
/answer f82d9f7e-3057-46ad-9c5c-aa0ab00e2a7a allow
Control Request:
```yaml
id: d7bf6309-331e-498b-acc7-4f77de5f6f93
result:
  structuredContent:
    content: |
      """Replace-lines tool – replaces a range of lines inside an existing file.

      This is the line-oriented analogue of ``replace-chars``: the range is given as a
      zero-based *line* offset and a *line* count instead of character offsets.
      """

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_replace_lines_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "replace-lines",
              title="Replace lines in file",
              description=(
                  "Replace a range of lines inside an existing file with new content. "
                  "The range is defined by a zero-based line ``offset`` and a ``length`` "
                  "(number of lines to remove starting at the offset). "
                  "The supplied ``content`` is written in place of the removed lines; "
                  "it should include its own trailing newline if a line break is wanted. "
                  "To replace an arbitrary character range instead, use ``replace-chars``."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the file to modify.",
                      },
                      "offset": {
                          "type": "integer",
                          "description": "Zero-based line offset of the first line to replace.",
                          "minimum": 0,
                      },
                      "length": {
                          "type": "integer",
                          "description": "Number of lines to remove starting at ``offset``.",
                          "minimum": 0,
                      },
                      "content": {
                          "type": "string",
                          "description": "Replacement text (may be empty to perform a pure deletion).",
                      },
                  },
                  "required": ["path", "offset", "length", "content"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                          "description": "``success`` on success.",
                      },
                  },
                  "required": ["result"],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def replace_lines(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              offset: int = args["offset"]
              length: int = args["length"]
              new_content: str = args["content"]

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              try:
                  text = path.read_text(encoding="utf-8")
                  lines = text.splitlines(keepends=True)
                  line_count = len(lines)
                  if offset > line_count:
                      return ToolResult(
                          structured_content={
                              "error": (
                                  f"Offset {offset} is beyond end of file "
                                  f"(file length: {line_count} lines)."
                              )
                          },
                          is_error=True,
                      )
                  end = min(offset + length, line_count)
                  result = "".join(lines[:offset]) + new_content + "".join(lines[end:])
                  path.write_text(result, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Replace failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"}, auto_approve=True)
```
/answer d7bf6309-331e-498b-acc7-4f77de5f6f93 allow
Control Request:
```yaml
id: 0f3c83c2-ed0c-4a1e-840c-f06cf5dc5b61
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace_block/__init__.py
```
/answer 0f3c83c2-ed0c-4a1e-840c-f06cf5dc5b61 allow
Control Request:
```yaml
id: 30a2ebb7-f106-4686-9ce4-ab20b2ee3d6e
result:
  structuredContent:
    content: |
      """Replace-block tool – replaces an exact block of text (old -> new) in a file."""

      from __future__ import annotations

      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult


      def register_replace_block_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "replace-block",
              title="Replace text block in file",
              description=(
                  "Replace a complete block of text inside an existing file. "
                  "'old_text' must occur exactly once in the file and is replaced "
                  "in full by 'new_text'. Use this when you know the exact text to "
                  "be replaced rather than an offset/length or start/end markers."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute path to the target file.",
                      },
                      "old_text": {
                          "type": "string",
                          "description": (
                              "Exact text to find and replace. Must occur exactly once "
                              "in the file."
                          ),
                      },
                      "new_text": {
                          "type": "string",
                          "description": "Text that replaces 'old_text'.",
                      },
                  },
                  "required": ["path", "old_text", "new_text"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "result": {
                          "type": "string",
                      },
                      "error": {
                          "type": "string",
                          "description": "Human-readable error message (only present when is_error is true).",
                      },
                  },
                  "required": [],
              },
              annotations={"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False},
          )
          def replace_block(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              old_text: str = args["old_text"]
              new_text: str = args["new_text"]

              # --- path validation ---
              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )
              if old_text == "":
                  return ToolResult(
                      structured_content={"error": "'old_text' must not be empty."},
                      is_error=True,
                  )

              text = path.read_text(encoding="utf-8")

              # --- locate and validate old_text ---
              occurrences = text.count(old_text)
              if occurrences == 0:
                  return ToolResult(
                      structured_content={"error": "Text not found in file."},
                      is_error=True,
                  )
              if occurrences > 1:
                  return ToolResult(
                      structured_content={
                          "error": f"Text is ambiguous – found {occurrences} occurrences in file."
                      },
                      is_error=True,
                  )

              result_text = text.replace(old_text, new_text, 1)

              # --- write back ---
              try:
                  path.write_text(result_text, encoding="utf-8")
              except OSError as exc:
                  return ToolResult(
                      structured_content={"error": f"Write failed: {exc}"},
                      is_error=True,
                  )

              return ToolResult(structured_content={"result": "success"}, auto_approve=True)
```
/answer 30a2ebb7-f106-4686-9ce4-ab20b2ee3d6e allow
Control Request:
```yaml
id: a30dae3c-ba06-41b0-b9a9-85edfd921886
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
```
/answer a30dae3c-ba06-41b0-b9a9-85edfd921886 allow
Control Request:
```yaml
id: 46363e14-c816-40a5-9086-84924ea2f9b0
result:
  structuredContent:
    content: |
      """Read tool – returns file contents, optionally sliced by line or unique marker.

      Range: start = min_line | start-marker | file start; end = max_line | end-marker
      | file end (all inclusive). Markers must be unique substrings. Per-session
      sha256 cache rejects unchanged re-reads (key ``_read_cache`` in session state).
      """

      from __future__ import annotations

      import hashlib
      from pathlib import Path
      from typing import Any

      from ...registry import ToolContext, ToolRegistry, ToolResult

      def register_read_tool(registry: ToolRegistry) -> None:
          @registry.tool(
              "read",
              title="Read file",
              description=(
                  "Read a file as text, optionally sliced to a range. Repeated "
                  "unchanged reads return an error."
              ),
              input_schema={
                  "type": "object",
                  "properties": {
                      "path": {
                          "type": "string",
                          "description": "Absolute file path.",
                      },
                      "min_line": {
                          "type": "integer",
                          "description": "Range start: line number, inclusive, 1-based. Excludes start.",
                          "minimum": 1,
                      },
                      "max_line": {
                          "type": "integer",
                          "description": "Range end: line number, inclusive, 1-based. Excludes end.",
                          "minimum": 1,
                      },
                      "start": {
                          "type": "string",
                          "description": "Range start: unique marker substring, inclusive. Excludes min_line.",
                      },
                      "end": {
                          "type": "string",
                          "description": "Range end: unique marker substring, inclusive. Excludes max_line.",
                      },
                  },
                  "required": ["path"],
              },
              output_schema={
                  "type": "object",
                  "properties": {
                      "content": {"type": "string"},
                  },
                  "required": ["content"],
              },
              annotations={"readOnlyHint": True, "openWorldHint": False},
          )
          def read(ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              path_str: str = args["path"]
              min_line: int | None = args.get("min_line")
              max_line: int | None = args.get("max_line")
              start_marker: str | None = args.get("start")
              end_marker: str | None = args.get("end")

              # --- mutual exclusivity validation ---
              if min_line is not None and start_marker is not None:
                  return ToolResult(
                      structured_content={
                          "error": "``min_line`` and ``start`` are mutually exclusive."
                      },
                      is_error=True,
                  )
              if max_line is not None and end_marker is not None:
                  return ToolResult(
                      structured_content={
                          "error": "``max_line`` and ``end`` are mutually exclusive."
                      },
                      is_error=True,
                  )

              path = Path(path_str)
              if not path.is_absolute():
                  return ToolResult(
                      structured_content={"error": "Path must be absolute."},
                      is_error=True,
                  )
              if not path.exists():
                  return ToolResult(
                      structured_content={"error": "File not found."},
                      is_error=True,
                  )
              if not path.is_file():
                  return ToolResult(
                      structured_content={"error": "Not a regular file."},
                      is_error=True,
                  )

              raw_bytes = path.read_bytes()

              # --- decode ---
              text = raw_bytes.decode("utf-8", errors="replace")
              lines = text.splitlines(keepends=True)
              total_lines = len(lines)

              def line_start_offset(line_num: int) -> int:
                  n = max(0, min(line_num - 1, total_lines))
                  return sum(len(l) for l in lines[:n])

              def line_end_offset(line_num: int) -> int:
                  n = max(0, min(line_num, total_lines))
                  return sum(len(l) for l in lines[:n])

              if start_marker is not None:
                  start_count = text.count(start_marker)
                  if start_count == 0:
                      return ToolResult(
                          structured_content={"error": "Start marker not found in file."},
                          is_error=True,
                      )
                  if start_count > 1:
                      return ToolResult(
                          structured_content={
                              "error": f"Start marker is ambiguous – found {start_count} occurrences in file."
                          },
                          is_error=True,
                      )
                  region_start = text.index(start_marker)
              elif min_line is not None:
                  region_start = line_start_offset(min_line)
              else:
                  region_start = 0

              if end_marker is not None:
                  end_count = text.count(end_marker)
                  if end_count == 0:
                      return ToolResult(
                          structured_content={"error": "End marker not found in file."},
                          is_error=True,
                      )
                  if end_count > 1:
                      return ToolResult(
                          structured_content={
                              "error": f"End marker is ambiguous – found {end_count} occurrences in file."
                          },
                          is_error=True,
                      )
                  region_end = text.index(end_marker) + len(end_marker)
              elif max_line is not None:
                  region_end = line_end_offset(max_line)
              else:
                  region_end = len(text)

              # --- order validation ---
              if region_end < region_start:
                  return ToolResult(
                      structured_content={
                          "error": (
                              f"Resolved end position must not lie before "
                              f"the resolved start position."
                          )
                      },
                      is_error=True,
                  )

              sliced = text[region_start:region_end]
              structured: dict[str, Any] = {"content": sliced}

              # An unrestricted read (no line/marker range given) returns the
              # entire file verbatim; there is nothing a human reviewer could
              # meaningfully approve or reject beyond what a plain file read
              # already exposes, so the tool flags it for auto-approval.
              is_full_file = (
                  min_line is None
                  and max_line is None
                  and start_marker is None
                  and end_marker is None
              )

              return ToolResult(structured_content=structured, auto_approve=is_full_file)
```
/answer 46363e14-c816-40a5-9086-84924ea2f9b0 allow
Control Request:
```yaml
id: 133ee89a-f983-406a-8cdb-94fff5ea4fb1
toolName: read
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  start: |2-
        # Control-endpoint-facing API (non-blocking)
        # Control-endpoint-facing API (non-blocking)
        # ------------------------------------------------------------------

        def get_pending
  end: "    def get_pending(self) -> list[dict[str, Any]]:"
```
/answer 133ee89a-f983-406a-8cdb-94fff5ea4fb1 allow
Control Request:
```yaml
id: 1ce525d8-5242-486c-88a1-40153cb6aa3b
result:
  structuredContent:
    content: |2-
          # Control-endpoint-facing API (non-blocking)
          # Control-endpoint-facing API (non-blocking)
          # ------------------------------------------------------------------

          def get_pending(self) -> list[dict[str, Any]]:
```
/answer 1ce525d8-5242-486c-88a1-40153cb6aa3b allow
Text:
 Jetzt entferne die `_is_simple_success_result` Methode:
Text:
 Jetzt `protocol.py` anpassen, damit der Flag durchgereicht wird:
Text:
 Nun die Docstring-Beispiele im Manager-Header anpassen, falls nötig, und die Tools migrieren:
Text:
 Nun das Read-Tool: Flag setzen, wenn der volle Dateiinhalt uneingeschränkt zurückgegeben wird (keine Range-Restriktion).
Text:
 Jetzt prüfen wir noch, ob im Manager-Docstring/Header sonst noch Referenzen auf das alte Verhalten stehen, und führen einen kompletten Check + Kompilierlauf durch.
Text:
 Alles kompiliert sauber, keine Reste der alten Struktur-Prüfung mehr vorhanden.
Control Request:
```yaml
id: b1733f98-3588-4036-b29c-0807fffe24de
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  mode: replace
  content: |
    """Human-in-the-loop tool control manager.

    Implements the Manager Pattern for intercepting tool calls at two points:

    1. *Before* execution (``phase="request"``): the interceptor may approve,
       modify arguments, or reject the call entirely.
    2. *After* execution (``phase="result"``): the interceptor may approve,
       replace the tool output, or inject instructions as the result.

    The control endpoint at ``/control/tool`` lets an external client poll for
    pending items and post approval decisions.  The intercepting threads block on
    per-item ``threading.Event`` objects until a decision arrives.
    """

    from __future__ import annotations

    import logging
    import threading
    import uuid
    from dataclasses import dataclass, field
    from typing import Any
    from xy.ai.mcpc.session import Session

    logger = logging.getLogger("xy.ai.mcpc.control")

    # How long (seconds) an intercepted call waits before timing out and auto-approving.
    _DEFAULT_TIMEOUT = 24 * 60 * 60.0  # 24 h — matches agent MCP timeout


    # ---------------------------------------------------------------------------
    # Decision DTO
    # ---------------------------------------------------------------------------

    @dataclass(slots=True)
    class ControlDecision:
        """The outcome of a human review, produced by :meth:`ToolControlManager.process_approvals`."""

        approved: bool
        """True when the call should proceed (possibly with modified data)."""

        rejection_reason: str | None = None
        """Human-readable hint for the agent when ``approved`` is False."""

        modified_arguments: dict[str, Any] | None = None
        """Replacement arguments for the ``request`` phase (``None`` → keep originals)."""

        modified_result: dict[str, Any] | None = None
        """Replacement result dict for the ``result`` phase (``None`` → keep original)."""


    # ---------------------------------------------------------------------------
    # Internal pending item
    # ---------------------------------------------------------------------------

    @dataclass
    class _PendingItem:
        id: str
        phase: str          # "request" | "result"
        tool_name: str
        arguments: dict[str, Any] | None       # populated in request phase
        result: dict[str, Any] | None          # populated in result phase
        _event: threading.Event = field(default_factory=threading.Event, repr=False)
        _decision: ControlDecision | None = field(default=None, repr=False)

        def to_dict(self) -> dict[str, Any]:
            item: dict[str, Any] = {"id": self.id}
            if self.arguments is not None:
                item["toolName"] = self.tool_name
                item["arguments"] = self.arguments
            if self.result is not None:
                item["result"] = self.result
            return item


    # ---------------------------------------------------------------------------
    # Manager
    # ---------------------------------------------------------------------------

    class ToolControlManager:
        """Thread-safe manager for human-in-the-loop tool interception.

        Usage::

            manager = ToolControlManager()

            # In the interceptor thread (blocks until a decision is received):
            decision = manager.submit_request("bash", {"command": "rm -rf /"})

            # In the control endpoint handler (non-blocking):
            pending = manager.get_pending()
            manager.process_approvals([{"id": "…", "approved": True}])
        """

        def __init__(self, timeout: float = _DEFAULT_TIMEOUT) -> None:
            self._timeout = timeout
            self._pending: dict[str, _PendingItem] = {}
            self._lock = threading.Lock()

        # ------------------------------------------------------------------
        # Interceptor-facing API (blocking)
        # ------------------------------------------------------------------

        def submit_request(
            self,
            session: Session,
            tool_name: str,
            arguments: dict[str, Any],
            *,
            auto_approve: bool = False,
        ) -> ControlDecision:
            """Block until the controller approves/rejects a tool-call *request*.

            Returns a :class:`ControlDecision`.  If the decision includes
            ``modified_arguments``, the caller should substitute them before
            invoking the tool handler.

            ``auto_approve`` lets a caller flag — on the tool's own authority —
            that this particular request needs no human review, bypassing the
            controller entirely. It is not derived from the shape of
            ``arguments``; the plumbing mirrors :meth:`submit_result` for
            symmetry, in case a tool ever wants to pre-approve its own request
            phase (e.g. based on previously-validated arguments).
            """
            if auto_approve:
                logger.info(
                    "Auto-approving request for %s [%s] (tool-flagged)",
                    tool_name, session.id,
                )
                return ControlDecision(approved=True)

            item = self._enqueue(session, "request", tool_name, arguments=arguments, result=None)
            return self._wait(item)

        def submit_result(
            self,
            session: Session,
            tool_name: str,
            result: dict[str, Any],
            *,
            auto_approve: bool = False,
        ) -> ControlDecision:
            """Block until the controller approves/replaces a tool-call *result*.

            Returns a :class:`ControlDecision`.  If the decision includes
            ``modified_result``, the caller should use that instead of the
            original result.

            ``auto_approve`` is a hint set by the *tool itself* (via
            ``ToolResult.auto_approve``), not derived from the shape of the
            result. A tool sets it when it judges its own result carries
            nothing meaningful for a human to review (e.g. a plain success
            acknowledgement, or an unrestricted full-file read). When set, the
            result is approved without involving the controller.
            """
            if auto_approve:
                logger.info(
                    "Auto-approving result for %s [%s] (tool-flagged)",
                    tool_name, session.id,
                )
                return ControlDecision(approved=True)

            item = self._enqueue(session, "result", tool_name, arguments=None, result=result)
            return self._wait(item)

        # ------------------------------------------------------------------
        # Control-endpoint-facing API (non-blocking)
        # ------------------------------------------------------------------

        def get_pending(self) -> list[dict[str, Any]]:
            """Return serialisable snapshots of all items still awaiting a decision."""
            with self._lock:
                return [item.to_dict() for item in self._pending.values()]

        def process_approvals(self, approvals: list[dict[str, Any]]) -> None:
            """Apply a batch of approval decisions from the control client.

            Each entry in *approvals* must have at minimum an ``"id"`` key.
            Accepted forms:

            * ``{"id": "…"}`` — simple approval, keep original data.
            * ``{"id": "…", "rejected": true, "reason": "…"}`` — rejection.
            * ``{"id": "…", "arguments": {…}}`` — approve with modified arguments.
            * ``{"id": "…", "result": {…}}`` — approve with replaced result.
            """
            for approval in approvals:
                item_id = approval.get("id")
                if not isinstance(item_id, str):
                    logger.warning("Approval entry missing 'id', skipped: %s", approval)
                    continue
                with self._lock:
                    item = self._pending.get(item_id)
                if item is None:
                    logger.warning("Unknown approval id %s, skipped", item_id)
                    continue

                if approval.get("rejected"):
                    decision = ControlDecision(
                        approved=False,
                        rejection_reason=approval.get("reason") or "Rejected by controller",
                    )
                else:
                    decision = ControlDecision(
                        approved=True,
                        modified_arguments=approval.get("arguments"),
                        modified_result=approval.get("result"),
                    )

                item._decision = decision
                with self._lock:
                    self._pending.pop(item_id, None)
                logger.info(
                    "Dequeued control item %s [%s/%s]: approved=%s",
                    item.tool_name, item.phase, item_id, decision.approved,
                )
                item._event.set()

        # ------------------------------------------------------------------
        # Internal helpers
        # ------------------------------------------------------------------

        def _enqueue(
            self,
            session: Session,
            phase: str,
            tool_name: str,
            arguments: dict[str, Any] | None,
            result: dict[str, Any] | None,
        ) -> _PendingItem:
            item_id = str(uuid.uuid4())
            item = _PendingItem(
                id=item_id,
                phase=phase,
                tool_name=tool_name,
                arguments=arguments,
                result=result,
            )
            with self._lock:
                self._pending[item_id] = item
            logger.info("Enqueued control item %s [%s/%s/%s]", tool_name, phase, session.id, item_id)
            return item

        def _wait(self, item: _PendingItem) -> ControlDecision:
            signalled = item._event.wait(timeout=self._timeout)
            with self._lock:
                self._pending.pop(item.id, None)

            if not signalled or item._decision is None:
                # Timeout — auto-approve to avoid hanging the agent forever.
                logger.warning(
                    "Control item %s [%s/%s] timed out, auto-approving",
                    item.tool_name, item.phase, item.id,
                )
                return ControlDecision(approved=True)

            return item._decision
```
/answer b1733f98-3588-4036-b29c-0807fffe24de allow
Zusammenfassung der Änderungen:

**1. `control/manager.py`**
- `submit_result()` bekommt einen neuen Keyword-Parameter `auto_approve: bool = False`. Ist dieser gesetzt, wird der Result-Approval-Schritt übersprungen — unabhängig von der Struktur des Ergebnisses.
- Die statische Methode `_is_simple_success_result()` (Struktur-basierte Erkennung von `{"result": "success"}`) wurde vollständig entfernt.

**2. `registry.py`**
- `ToolResult` hat ein neues Feld `auto_approve: bool = False`. Dieses Feld wird **nicht** in `to_dict()` serialisiert (also nie an den Client geschickt), sondern ist reines internes Signal für die Kontroll-Schicht.

**3. `protocol.py`**
- `_handle_tools_call` übergibt jetzt `auto_approve=result.auto_approve` an `control.submit_result(...)`.

**4. Tools mit Success-Pattern migriert** (alle `ToolResult(structured_content={"result": "success"})` → mit `auto_approve=True`):
- `change`, `replace_chars`, `insert`, `replace_lines`, `replace_block`, `write`

**5. `read/__init__.py`**
- Wenn kein Range-Filter (`min_line`, `max_line`, `start`, `end`) angegeben ist und somit die volle Datei uneingeschränkt zurückgegeben wird, wird `auto_approve=True` gesetzt.

Damit bestimmt jetzt jedes Tool selbst über sein `ToolResult`, ob sein Ergebnis ohne menschliche Prüfung freigegeben werden darf – die Auto-Approve-Logik ist nicht mehr an ein festes Struktur-Pattern im Manager gebunden.

Session ID: f6bb7d0c-e5f2-41d8-9746-9abe85a55169
Ttile: In `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py` wird ein Simple "success
TTL: 51 min
Model: CC_MCPC_SONNET
Effort: medium
Tools: read, list, change, insert, write, replace-block, replace-chars, replace-lines, bash
File: project/simple_success1.md
Stats: total: 29328, in: 115, out: 8795, reason: 2015, read: 452530, write: 29213

Systemprompt:

