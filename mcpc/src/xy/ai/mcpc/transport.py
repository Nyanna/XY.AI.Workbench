"""Streamable HTTP transport (MCP) built on ``http.server``.

Implements the client-facing half of the MCP *Streamable HTTP* transport:

* ``POST`` to the MCP endpoint carries a single JSON-RPC message.
    * A *request* is answered with ``Content-Type: application/json`` (a single
      response object).  SSE streaming is intentionally not used because this
      server emits no server-initiated notifications.
    * A *notification* / *response* is acknowledged with ``202 Accepted``.
* ``GET`` returns ``405`` — the server offers no server-to-client SSE stream.
* ``DELETE`` terminates the session (``204``).

The session id is taken from the configured ``X-MCPC-SESSION-ID`` header, which
the client must send on every request.
"""
from __future__ import annotations

import logging

from http import HTTPStatus
from http.server import BaseHTTPRequestHandler
from typing import Any
from urllib.parse import urlparse

from . import errors, jsonrpc
from .jsonrpc import MessageKind
from .logging_utils import EVENT, IN, OUT
from .session import is_valid_uuid
from aptdaemon import logger

logger = logging.getLogger("xy.ai.mcpc.transport")


def _origin_host(origin: str) -> str:
    try:
        return urlparse(origin).hostname or ""
    except ValueError:
        return ""


class StreamableHttpHandler(BaseHTTPRequestHandler):
    """Handles a single HTTP connection for the MCP endpoint."""

    protocol_version = "HTTP/1.1"
    server_version = "xy.ai.mcpc"

    # ``server`` is our McpHTTPServer instance; expose its parts for brevity.
    @property
    def config(self):
        return self.server.config  # type: ignore[attr-defined]

    @property
    def protocol(self):
        return self.server.protocol  # type: ignore[attr-defined]

    @property
    def sessions(self):
        return self.server.sessions  # type: ignore[attr-defined]

    @property
    def comm_log(self):
        return self.server.comm_log  # type: ignore[attr-defined]

    # -- logging ------------------------------------------------------------
    def log_message(self, fmt: str, *args: Any) -> None:  # noqa: A003
        # Route access logs through the standard logging framework instead of
        # writing straight to stderr.
        self.server.logger.info("%s - %s", self.address_string(), fmt % args)  # type: ignore[attr-defined]

    # -- HTTP verbs ---------------------------------------------------------
    def do_POST(self) -> None:  # noqa: N802
        logger.debug("Accept POST")
        if self._hook_path_matches():
            self._handle_hook()
            return
        if self._control_path_matches():
            logger.debug("Control path matched: %s", urlparse(self.path).path)
            self._handle_control()
            return
        if not self._path_matches():
            logger.error("Unknown endpoint %s != %s", urlparse(self.path).path, self.config.path)
            self._send_http_error(HTTPStatus.NOT_FOUND, "Unknown endpoint")
            return
        if not self._check_origin():
            logger.error("Origin forbidden")
            return
        session_id = self._require_session_id()
        if session_id is None:
            logger.error("Session id missing [%s]", self.headers)
            return
        if not self._check_protocol_version_header():
            logger.error("Protocol version missing")
            return

        raw = self._read_body()
        if raw is None:
            return

        try:
            message = jsonrpc.parse_body(raw)
        except errors.JsonRpcError as exc:
            self.comm_log.log(session_id, IN, raw.decode("utf-8", "replace"), http="POST", note="unparseable")
            self._send_jsonrpc_error(HTTPStatus.BAD_REQUEST, None, exc, session_id)
            return

        self.comm_log.log(session_id, IN, message, http="POST")

        try:
            kind = jsonrpc.classify(message)
            request = jsonrpc.to_request(message) if kind is not MessageKind.RESPONSE else None
        except errors.JsonRpcError as exc:
            self._send_jsonrpc_error(HTTPStatus.BAD_REQUEST, message.get("id"), exc, session_id)
            return

        session, created = self.sessions.get_or_create(session_id)
        if created:
            self.comm_log.log(session_id, EVENT, {"event": "session.created"})
        session.touch()
        self._apply_tools_header(session_id, session)
        self._apply_ccprofile_header(session_id, session)

        if kind is MessageKind.REQUEST:
            self._handle_request(session_id, session, request)  # type: ignore[arg-type]
        else:
            # Notification or response: acknowledge, act on lifecycle only.
            if kind is MessageKind.NOTIFICATION:
                try:
                    self.protocol.handle_notification(session, request)  # type: ignore[arg-type]
                except errors.JsonRpcError:
                    pass  # notifications never produce a response
            self._send_accepted(session_id)

    def do_GET(self) -> None:  # noqa: N802
        if not self._path_matches():
            self._send_http_error(HTTPStatus.NOT_FOUND, "Unknown endpoint")
            return
        # No server-initiated SSE stream is offered (notifications unsupported).
        self._send_http_error(
            HTTPStatus.METHOD_NOT_ALLOWED,
            "This endpoint does not provide a server-to-client SSE stream",
            extra_headers={"Allow": "POST, DELETE"},
        )

    def do_DELETE(self) -> None:  # noqa: N802
        if not self._path_matches():
            self._send_http_error(HTTPStatus.NOT_FOUND, "Unknown endpoint")
            return
        if not self._check_origin():
            return
        session_id = self._require_session_id()
        if session_id is None:
            return
        removed = self.sessions.remove(session_id)
        if removed:
            self.comm_log.log(session_id, EVENT, {"event": "session.terminated"})
            self._send_empty(HTTPStatus.NO_CONTENT, session_id)
        else:
            self._send_http_error(HTTPStatus.NOT_FOUND, "Unknown session", session_id=session_id)

    def _apply_tools_header(self, session_id: str, session) -> None:
        """Reconcile the session's active toolset with the ``X-MCPC-TOOLS`` header.

        The header carries a comma-separated list of tool names and is honoured
        on every request.  When it is *absent* the session's configuration is
        left untouched — this is deliberate: a spawned sub-agent inherits a
        pre-configured toolset and never sends the header itself.  A present but
        empty header activates the empty toolset (no tools).
        """
        raw = self.headers.get(self.config.tools_header)
        if raw is None:
            return
        logger.debug("Process tool header: %s", raw)
        names = {part.strip() for part in raw.split(",") if part.strip()}
        if session.enabled_tools != names:
            session.set_enabled_tools(names)
            self.comm_log.log(
                session_id,
                EVENT,
                {"event": "session.tools", "tools": sorted(names)},
            )
            
    def _apply_ccprofile_header(self, session_id: str, session) -> None:
        """Reconcile the session's active CC-profile with the ``X-MCPC-CC-PROFILE`` header.
        """
        raw = self.headers.get(self.config.ccprofile_header)
        if raw is None:
            return
        logger.debug("Process CC-profile header: %s", raw)
        if session.cc_profile != raw:
            session.cc_profile = raw
            self.comm_log.log(
                session_id,
                EVENT,
                {"event": "session.cc_profile", "cc_profile": raw},
            )

    # -- request processing -------------------------------------------------
    def _handle_request(self, session_id: str, session, request) -> None:
        skip_control = self.headers.get(self.config.control_header, "").lower() == "off"
        try:
            with session.lock:
                result = self.protocol.handle_request(session, request, skip_control=skip_control)
            response = jsonrpc.success_response(request.id, result)
        except errors.JsonRpcError as exc:
            response = jsonrpc.error_response(request.id, exc)
        except Exception as exc:  # noqa: BLE001 - never leak a stack trace
            self.server.logger.exception("Unhandled error processing request")  # type: ignore[attr-defined]
            response = jsonrpc.error_response(
                request.id, errors.internal_error(str(exc))
            )

        self.comm_log.log(session_id, OUT, response, http="POST")
        self._send_json(HTTPStatus.OK, jsonrpc.dumps(response), session_id)

    # -- control handler ----------------------------------------------------
    def _control_path_matches(self) -> bool:
        return urlparse(self.path).path == self.config.control_path

    def _handle_control(self) -> None:
        """Handle a poll request from the human-in-the-loop control client.

        Request body (JSON):
          ``{"approvals": [...]}``

        Each approval entry:
          * ``{"id": "…"}``                              — simple approval
          * ``{"id": "…", "rejected": true, "reason": "…"}``  — rejection
          * ``{"id": "…", "arguments": {…}}``           — approve with modified args
          * ``{"id": "…", "result": {…}}``              — approve with replaced result

        Response body (JSON):
          ``{"pending": [...]}``  — items still waiting for a decision
        """
        logger.debug("Control endpoint reached")
        control = self.server.services.control_manager  # type: ignore[attr-defined]
        if control is None:
            logger.warning("Control: manager not enabled, returning 404")
            self._send_http_error(HTTPStatus.NOT_FOUND, "Tool control is not enabled")
            return

        raw = self._read_body()
        logger.debug("Control: body read, length=%d", len(raw) if raw is not None else -1)
        if raw is None:
            return

        if raw:
            try:
                body = jsonrpc.parse_body(raw)
            except Exception as exc:
                logger.warning("Control: invalid JSON body: %s", exc)
                self._send_http_error(HTTPStatus.BAD_REQUEST, "Invalid JSON body")
                return
            approvals = body.get("approvals", [])
            if not isinstance(approvals, list):
                logger.warning("Control: 'approvals' is not a list: %r", approvals)
                self._send_http_error(HTTPStatus.BAD_REQUEST, '"approvals" must be an array')
                return
            logger.debug("Control: processing %d approval(s)", len(approvals))
            # 1. Process decisions first so callers can be unblocked before
            #    the next pending list is assembled.
            control.process_approvals(approvals)
        else:
            logger.error("Control: empty body, poll only")

        # 2. Return the remaining (or new) pending items.
        pending = control.get_pending()
        logger.debug("Control: returning %d pending item(s)", len(pending))
        response: dict[str, Any] = {"pending": pending}
        self._send_json(HTTPStatus.OK, jsonrpc.dumps(response), session_id=None)

    # -logger.debugler -------------------------------------------------------
    def _hook_path_matches(self) -> bool:
        return urlparse(self.path).path == self.config.hook_path

    def _handle_hook(self) -> None:
        """Handle a PreToolUse hook call from a spawned CLI process.

        Reads the JSON body (ignored for now), and always responds with
        ``{"continue": true, "suppressOutput": false}`` — i.e. every tool
        call is allowed unconditionally.
        Intended for use in headless subagents. The usual tool use will
        handle permission implicite.
        """
        raw = self._read_body()
        if raw is None:
            return
        logger.debug("PreToolUse hook called: %s", raw.decode("utf-8", "replace"))
        response: dict[str, Any] = {"continue": True, "suppressOutput": False}
        body = jsonrpc.dumps(response)
        self._send_json(HTTPStatus.OK, body, session_id=None)

    # -- validation helpers -------------------------------------------------
    def _path_matches(self) -> bool:
        return urlparse(self.path).path == self.config.path

    def _check_origin(self) -> bool:
        origin = self.headers.get("Origin")
        if origin is None:
            return True  # non-browser client
        host = _origin_host(origin)
        allowed = {"localhost", "127.0.0.1", "::1", "[::1]", self.config.host}
        if self.config.allowed_origins:
            allowed.update(self.config.allowed_origins)
        if host in allowed:
            return True
        self._send_http_error(HTTPStatus.FORBIDDEN, f"Origin not allowed: {origin}")
        return False

    def _require_session_id(self) -> str | None:
        session_id = self.headers.get(self.config.session_header)
        if not session_id:
            logger.error("Session id header not found")
            self._send_jsonrpc_error(
                HTTPStatus.BAD_REQUEST,
                None,
                errors.invalid_request(
                    f"Missing required header: {self.config.session_header}"
                ),
                session_id=None,
            )
            return None
        if self.config.require_uuid_session and not is_valid_uuid(session_id):
            logger.error("Session id is invalid UUID")
            self._send_jsonrpc_error(
                HTTPStatus.BAD_REQUEST,
                None,
                errors.invalid_request(
                    f"{self.config.session_header} must be a valid UUID"
                ),
                session_id=session_id,
            )
            return None
        logger.debug("Session id: %s", session_id)
        return session_id

    def _check_protocol_version_header(self) -> bool:
        version = self.headers.get("MCP-Protocol-Version")
        if version is None:
            return True  # absent on the initial 'initialize' request
        if version in self.config.supported_protocol_versions:
            return True
        self._send_http_error(
            HTTPStatus.BAD_REQUEST, f"Unsupported MCP-Protocol-Version: {version}"
        )
        return False

    def _read_body(self) -> bytes | None:
        try:
            length = int(self.headers.get("Content-Length", 0))
        except ValueError:
            self._send_http_error(HTTPStatus.BAD_REQUEST, "Invalid Content-Length")
            return None
        if length > self.config.max_body_bytes:
            self._send_http_error(
                HTTPStatus.REQUEST_ENTITY_TOO_LARGE, "Request body too large"
            )
            return None
        return self.rfile.read(length) if length else b""

    # -- response helpers ---------------------------------------------------
    def _base_headers(self, session_id: str | None) -> None:
        if session_id is not None:
            self.send_header(self.config.session_header, session_id)

    def _send_json(self, status: HTTPStatus, body: bytes, session_id: str | None) -> None:
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self._base_headers(session_id)
        self.end_headers()
        self.wfile.write(body)

    def _send_accepted(self, session_id: str | None) -> None:
        self._send_empty(HTTPStatus.ACCEPTED, session_id)

    def _send_empty(self, status: HTTPStatus, session_id: str | None) -> None:
        self.send_response(status)
        self.send_header("Content-Length", "0")
        self._base_headers(session_id)
        self.end_headers()

    def _send_json_object(self, status: HTTPStatus, obj: dict[str, Any], session_id: str | None) -> None:
        self._send_json(status, jsonrpc.dumps(obj), session_id)

    def _send_jsonrpc_error(
        self,
        status: HTTPStatus,
        request_id,
        error: errors.JsonRpcError,
        session_id: str | None,
    ) -> None:
        response = jsonrpc.error_response(request_id, error)
        if session_id is not None:
            self.comm_log.log(session_id, OUT, response, http=self.command, status=int(status))
        self._send_json_object(status, response, session_id)

    def _send_http_error(
        self,
        status: HTTPStatus,
        message: str,
        session_id: str | None = None,
        extra_headers: dict[str, str] | None = None,
    ) -> None:
        # Body is a JSON-RPC error response with a null id, per the transport spec.
        response = jsonrpc.error_response(None, errors.JsonRpcError(-32600, message))
        body = jsonrpc.dumps(response)
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        if extra_headers:
            for key, value in extra_headers.items():
                self.send_header(key, value)
        self._base_headers(session_id)
        self.end_headers()
        self.wfile.write(body)
