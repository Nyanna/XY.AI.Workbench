"""HTTP handler for the human-in-the-loop control endpoint."""

from __future__ import annotations

import logging
from http import HTTPStatus
from typing import TYPE_CHECKING, Any
from urllib.parse import urlparse

from .. import jsonrpc

if TYPE_CHECKING:
    from ..transport import StreamableHttpHandler

logger = logging.getLogger("xy.ai.mcpc.control")


class ControlHandler:
    """Handles POST requests to the tool-control endpoint (``/control/tool``).

    Instantiate with the active :class:`StreamableHttpHandler` and call
    :meth:`matches` to check the request path, then :meth:`handle` to process
    it.

    Request body (JSON)::

        {"approvals": [...]}

    Each approval entry:

    * ``{"id": "…"}``                              — simple approval
    * ``{"id": "…", "rejected": true, "reason": "…"}``  — rejection

    Response body (JSON)::

        {"pending": [...]}
    """

    def __init__(self, http: "StreamableHttpHandler") -> None:
        self._http = http

    def matches(self) -> bool:
        """Return ``True`` when the request path equals ``config.control_path``."""
        return urlparse(self._http.path).path == self._http.config.control_path

    def handle(self) -> None:
        """Process a poll/approval request from the control client."""
        logger.debug("Control endpoint reached")
        control = self._http.server.services.control_manager  # type: ignore[attr-defined]
        if control is None:
            logger.warning("Control: manager not enabled, returning 404")
            self._http._send_http_error(HTTPStatus.NOT_FOUND, "Tool control is not enabled")
            return

        raw = self._http._read_body()
        logger.debug("Control: body read, length=%d", len(raw) if raw is not None else -1)
        if raw is None:
            return

        if raw:
            try:
                body = jsonrpc.parse_body(raw)
            except Exception as exc:
                logger.warning("Control: invalid JSON body: %s", exc)
                self._http._send_http_error(HTTPStatus.BAD_REQUEST, "Invalid JSON body")
                return
            approvals = body.get("approvals", [])
            if not isinstance(approvals, list):
                logger.warning("Control: 'approvals' is not a list: %r", approvals)
                self._http._send_http_error(HTTPStatus.BAD_REQUEST, '"approvals" must be an array')
                return
            logger.debug("Control: processing %d approval(s)", len(approvals))
            # Process decisions first so callers can be unblocked before
            # the next pending list is assembled.
            control.process_approvals(approvals)
        else:
            logger.debug("Control: empty body, poll only")

        pending = control.get_pending()
        logger.debug("Control: returning %d pending item(s)", len(pending))
        response: dict[str, Any] = {"pending": pending}
        self._http._send_json(HTTPStatus.OK, jsonrpc.dumps(response), session_id=None)
