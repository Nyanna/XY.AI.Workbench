"""HTTP handlers for the CLI hook endpoints (PreToolUse and PermissionRequest)."""

from __future__ import annotations

import logging
from http import HTTPStatus
from typing import TYPE_CHECKING, Any
from urllib.parse import urlparse

from . import jsonrpc
from .codec import JsonCodec

if TYPE_CHECKING:
    from .transport import StreamableHttpHandler

logger = logging.getLogger("xy.ai.mcpc.transport")


class HookHandler:
    """Handles POST requests to the CLI PreToolUse hook endpoint (``/hooks/tool``).

    Instantiate with the active :class:`StreamableHttpHandler` and call
    :meth:`matches` to check the request path, then :meth:`handle` to process
    it.

    Always responds with ``{"continue": true, "suppressOutput": false}`` —
    headless sub-agents are approved unconditionally; the main session's tool
    interception handles the actual permission check.
    """

    def __init__(self, http: "StreamableHttpHandler") -> None:
        self._http = http

    def matches(self) -> bool:
        """Return ``True`` when the request path equals ``config.hook_path``."""
        return urlparse(self._http.path).path == self._http.config.hook_path

    def handle(self) -> None:
        """Approve the hook call unconditionally."""
        raw = self._http._read_body()
        if raw is None:
            return
        logger.debug("PreToolUse hook called: %s", JsonCodec.for_log(raw))
        response: dict[str, Any] = {"continue": True, "suppressOutput": False}
        self._http._send_json(HTTPStatus.OK, jsonrpc.dumps(response), session_id=None)


class PermissionHookHandler:
    """Handles POST requests to the CLI PermissionRequest hook endpoint (``/hooks/permission``).

    Instantiate with the active :class:`StreamableHttpHandler` and call
    :meth:`matches` to check the request path, then :meth:`handle` to process
    it.

    Always responds with ``behavior: "allow"`` — all permission requests are
    granted unconditionally.
    """

    def __init__(self, http: "StreamableHttpHandler") -> None:
        self._http = http

    def matches(self) -> bool:
        """Return ``True`` when the request path equals ``config.permission_hook_path``."""
        return urlparse(self._http.path).path == self._http.config.permission_hook_path

    def handle(self) -> None:
        """Allow the permission request unconditionally."""
        raw = self._http._read_body()
        if raw is None:
            return
        logger.debug("PermissionRequest hook called: %s", JsonCodec.for_log(raw))
        response: dict[str, Any] = {
            "continue": True,
            "suppressOutput": False,
            "hookSpecificOutput": {
                "hookEventName": "PermissionRequest",
                "decision": {
                    "behavior": "allow",
                },
            },
        }
        self._http._send_json(HTTPStatus.OK, jsonrpc.dumps(response), session_id=None)
