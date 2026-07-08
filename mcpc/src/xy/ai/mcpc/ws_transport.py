"""WebSocket transport for the MCP endpoint.

This module adds a *second* network interface next to the Streamable-HTTP
transport (:mod:`.transport`): the very same server-side object graph
(:class:`~.protocol.McpProtocol`, :class:`~.session.SessionStore`,
:class:`~.registry.ToolRegistry`, :class:`~.logging_utils.CommunicationLog`,
...) is exposed over persistent WebSocket connections instead of discrete
HTTP ``POST`` requests. No tool or protocol logic is duplicated; this module
is purely a second transport.

Wire semantics
---------------
One physical WebSocket connection is bound to exactly one MCP session for
its whole lifetime:

* The session id is taken from the configured ``X-MCPC-SESSION-ID`` header
  (or, if the client library used cannot set custom headers on the opening
  handshake, from a same-named query parameter) of the *opening handshake*
  request — there is no per-message header, so it cannot change mid-connection.
* ``X-MCPC-TOOLS`` and ``X-MCPC-CC-PROFILE`` are likewise read once, from the
  handshake request, and applied to the session exactly like the HTTP
  transport applies them per-request.
* ``X-MCPC-CONTROL: off`` on the handshake disables tool-call interception
  for every request sent over this connection.
* Every text frame sent by the client carries exactly one JSON-RPC message:
  a *request* is answered with one JSON-RPC response frame; a *notification*
  produces no reply (only lifecycle side effects, mirroring the HTTP
  transport's ``202 Accepted``); a *response* (this server never issues
  requests of its own) is accepted and ignored.
* The server never emits unsolicited messages, so no broadcast/fan-out
  machinery is needed — this mirrors the Streamable-HTTP transport, which
  offers no server-to-client SSE stream either.

Implementation notes
---------------------
Built on the ``websockets`` package (asyncio-based). The server runs its own
event loop in a dedicated daemon thread so it can be started and stopped
independently of, while still sharing all state with, the synchronous
thread-per-connection :class:`~.server.McpHTTPServer`.

``protocol.handle_request`` is a *blocking* call (it may hold a session lock
or wait for a human-in-the-loop approval decision for up to 24h), so it must
never run directly on the asyncio event loop — doing so would stall every
other WebSocket connection. Each request is therefore dispatched to a worker
thread via ``loop.run_in_executor``.
"""

from __future__ import annotations

import asyncio
import logging
import threading
from typing import TYPE_CHECKING, Any
from urllib.parse import parse_qs, urlparse

from . import errors, jsonrpc
from .jsonrpc import JsonRpcRequest, MessageKind
from .logging_utils import EVENT, IN, OUT
from .session import Session, is_valid_uuid
from .transport import apply_ccprofile_header, apply_tools_header, is_origin_allowed

try:
    from websockets.asyncio.server import Server as _WsServer
    from websockets.asyncio.server import ServerConnection
    from websockets.asyncio.server import serve as ws_serve
    from websockets.exceptions import ConnectionClosed
except ImportError:  # pragma: no cover - exercised only when the optional
    # dependency is missing; WebSocketMcpServer() raises a clear error instead.
    _WsServer = None  # type: ignore[assignment]
    ServerConnection = Any  # type: ignore[assignment,misc]
    ws_serve = None  # type: ignore[assignment]
    ConnectionClosed = Exception  # type: ignore[assignment,misc]

if TYPE_CHECKING:
    from .config import ServerConfig
    from .context import AppServices
    from .logging_utils import CommunicationLog
    from .protocol import McpProtocol
    from .session import SessionStore

logger = logging.getLogger("xy.ai.mcpc.ws")

#: WebSocket close code for "policy violation" (RFC 6455 §7.4.1), used for
#: every handshake-time rejection (unknown endpoint, missing session id, ...).
_POLICY_VIOLATION = 1008


class WebSocketMcpServer:
    """Runs the MCP JSON-RPC protocol over WebSocket, next to the HTTP server.

    Shares *config*, *protocol*, *sessions* and *comm_log* with the HTTP
    transport so both interfaces operate on the same sessions and tools.
    """

    def __init__(
        self,
        config: "ServerConfig",
        protocol: "McpProtocol",
        sessions: "SessionStore",
        comm_log: "CommunicationLog",
        services: "AppServices | None" = None,
    ) -> None:
        if ws_serve is None:  # pragma: no cover - environment without the dep
            raise RuntimeError(
                "The 'websockets' package is required for the WebSocket "
                "transport; install it with `pip install websockets`."
            )
        self.config = config
        self.protocol = protocol
        self.sessions = sessions
        self.comm_log = comm_log
        self.services = services

        self._loop: asyncio.AbstractEventLoop | None = None
        self._thread: threading.Thread | None = None
        self._server: "_WsServer | None" = None
        self._ready = threading.Event()
        self._startup_error: BaseException | None = None

    @property
    def endpoint_url(self) -> str:
        return f"ws://{self.config.resolved_ws_host}:{self.config.ws_port}{self.config.ws_path}"

    # -- lifecycle ------------------------------------------------------------
    def start(self) -> None:
        """Start the WebSocket server in a background thread and block until ready."""
        self._thread = threading.Thread(target=self._run, name="mcpc-ws", daemon=True)
        self._thread.start()
        self._ready.wait()
        if self._startup_error is not None:
            raise self._startup_error

    def stop(self) -> None:
        """Stop the WebSocket server and join its thread."""
        if self._loop is not None and self._server is not None:
            self._loop.call_soon_threadsafe(self._server.close)
        if self._thread is not None:
            self._thread.join(timeout=10)

    def _run(self) -> None:
        try:
            asyncio.run(self._serve())
        except BaseException as exc:  # noqa: BLE001 - surfaced to start()
            self._startup_error = exc
            self._ready.set()

    async def _serve(self) -> None:
        self._loop = asyncio.get_running_loop()
        async with ws_serve(
            self._handle_connection,
            self.config.resolved_ws_host,
            self.config.ws_port,
        ) as server:
            self._server = server
            self._ready.set()
            await server.serve_forever()

    # -- connection handling ---------------------------------------------------
    async def _handle_connection(self, connection: "ServerConnection") -> None:
        request = connection.request
        headers = request.headers
        parsed = urlparse(request.path)
        query = parse_qs(parsed.query)

        if parsed.path != self.config.path:
            logger.warning("WS: unknown endpoint %s", parsed.path)
            await connection.close(_POLICY_VIOLATION, "Unknown endpoint")
            return

        origin = headers.get("Origin")
        if not is_origin_allowed(self.config, origin):
            logger.warning("WS: origin forbidden: %s", origin)
            await connection.close(_POLICY_VIOLATION, f"Origin not allowed: {origin}")
            return

        session_id = headers.get(self.config.session_header)
        if not session_id:
            session_id = (query.get(self.config.session_header) or [None])[0]
        if not session_id:
            logger.warning("WS: session id missing")
            await connection.close(
                _POLICY_VIOLATION,
                f"Missing required header: {self.config.session_header}",
            )
            return
        if self.config.require_uuid_session and not is_valid_uuid(session_id):
            logger.warning("WS: session id is not a valid UUID")
            await connection.close(
                _POLICY_VIOLATION, f"{self.config.session_header} must be a valid UUID"
            )
            return

        skip_control = (headers.get(self.config.control_header, "") or "").lower() == "off"

        session, created = self.sessions.get_or_create(session_id)
        if created:
            self.comm_log.log(session_id, EVENT, {"event": "session.created", "transport": "ws"})
        session.touch()
        apply_tools_header(
            self.config, self.comm_log, session_id, session, headers.get(self.config.tools_header)
        )
        apply_ccprofile_header(
            self.comm_log, session_id, session, headers.get(self.config.ccprofile_header)
        )
        self.comm_log.log(session_id, EVENT, {"event": "session.ws_connected"})

        try:
            async for raw in connection:
                await self._handle_message(session_id, session, connection, raw, skip_control)
        except ConnectionClosed:
            pass
        except Exception:  # noqa: BLE001 - never let one connection kill the loop
            logger.exception("WS: unhandled error on connection for session %s", session_id)
        finally:
            self.comm_log.log(session_id, EVENT, {"event": "session.ws_disconnected"})

    async def _handle_message(
        self,
        session_id: str,
        session: Session,
        connection: "ServerConnection",
        raw: "str | bytes",
        skip_control: bool,
    ) -> None:
        text = raw.decode("utf-8", "replace") if isinstance(raw, (bytes, bytearray)) else raw
        try:
            message = jsonrpc.parse_body(text.encode("utf-8"))
        except errors.JsonRpcError as exc:
            self.comm_log.log(session_id, IN, text, transport="ws", note="unparseable")
            await self._send(connection, session_id, jsonrpc.error_response(None, exc))
            return

        self.comm_log.log(session_id, IN, message, transport="ws")

        try:
            kind = jsonrpc.classify(message)
            request = jsonrpc.to_request(message) if kind is not MessageKind.RESPONSE else None
        except errors.JsonRpcError as exc:
            response = jsonrpc.error_response(message.get("id"), exc)
            await self._send(connection, session_id, response)
            return

        session.touch()

        if kind is MessageKind.REQUEST:
            await self._handle_request(session_id, session, request, connection, skip_control)  # type: ignore[arg-type]
        elif kind is MessageKind.NOTIFICATION:
            try:
                self.protocol.handle_notification(session, request)  # type: ignore[arg-type]
            except errors.JsonRpcError:
                pass  # notifications never produce a response
        # MessageKind.RESPONSE: this server issues no requests of its own,
        # so a client-sent "response" has nothing to correlate to; ignore it.

    async def _handle_request(
        self,
        session_id: str,
        session: Session,
        request: JsonRpcRequest,
        connection: "ServerConnection",
        skip_control: bool,
    ) -> None:
        loop = asyncio.get_running_loop()

        def _run() -> dict[str, Any]:
            try:
                with session.lock:
                    result = self.protocol.handle_request(session, request, skip_control=skip_control)
                return jsonrpc.success_response(request.id, result)
            except errors.JsonRpcError as exc:
                return jsonrpc.error_response(request.id, exc)
            except Exception as exc:  # noqa: BLE001 - never leak a stack trace
                logger.exception("WS: unhandled error processing request")
                return jsonrpc.error_response(request.id, errors.internal_error(str(exc)))

        # Runs on a worker thread: `handle_request` blocks (session lock,
        # human-in-the-loop approval) and must not stall the event loop.
        response = await loop.run_in_executor(None, _run)
        await self._send(connection, session_id, response)

    async def _send(
        self, connection: "ServerConnection", session_id: str, response: dict[str, Any]
    ) -> None:
        self.comm_log.log(session_id, OUT, response, transport="ws")
        try:
            await connection.send(jsonrpc.dumps(response).decode("utf-8"))
        except ConnectionClosed:
            logger.debug("WS: client disconnected before response could be sent")
