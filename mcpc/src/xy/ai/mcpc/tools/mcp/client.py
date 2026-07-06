"""A minimal outbound MCP client (Streamable HTTP, JSON-RPC 2.0).

MCPC acts as an MCP *client* towards an external server, forwarding a small set
of hard-coded calls.  The connection is established and initialised lazily on
first use.  Server-initiated notifications / SSE streams are not consumed — only
request/response exchanges are performed.  Both ``application/json`` and
``text/event-stream`` responses are understood.
"""

from __future__ import annotations

import json
import threading
import urllib.error
import urllib.request
from typing import Any

#: Protocol revision advertised on ``initialize`` (server may negotiate down).
DEFAULT_PROTOCOL_VERSION = "2025-06-18"


class McpClientError(RuntimeError):
    """Raised for transport, protocol, or remote JSON-RPC errors."""


class McpClient:
    """Talks JSON-RPC over Streamable HTTP to a single external MCP server."""

    def __init__(
        self,
        endpoint: str,
        *,
        headers: dict[str, str] | None = None,
        protocol_version: str = DEFAULT_PROTOCOL_VERSION,
        client_name: str = "xy.ai.mcpc",
        client_version: str = "0.1.0",
        timeout: float = 60.0,
    ) -> None:
        self.endpoint = endpoint
        self._static_headers = dict(headers or {})
        self.protocol_version = protocol_version
        self.client_name = client_name
        self.client_version = client_version
        self.timeout = timeout

        self._session_id: str | None = None
        self._negotiated_version: str | None = None
        self._initialized = False
        self._id = 0
        self._lock = threading.RLock()

    # -- lifecycle ----------------------------------------------------------
    def ensure_initialized(self) -> None:
        """Connect and run the ``initialize`` handshake once (idempotent)."""
        with self._lock:
            if self._initialized:
                return
            self._initialize()
            self._initialized = True

    def _initialize(self) -> None:
        result = self._result_or_raise(
            self._send(
                {
                    "jsonrpc": "2.0",
                    "id": self._next_id(),
                    "method": "initialize",
                    "params": {
                        "protocolVersion": self.protocol_version,
                        "capabilities": {},
                        "clientInfo": {
                            "name": self.client_name,
                            "version": self.client_version,
                        },
                    },
                },
                expect_response=True,
            )
        )
        self._negotiated_version = result.get("protocolVersion", self.protocol_version)
        # Complete the handshake; the server never streams notifications back.
        self._send(
            {"jsonrpc": "2.0", "method": "notifications/initialized"},
            expect_response=False,
        )

    # -- calls --------------------------------------------------------------
    def call_tool(self, name: str, arguments: dict[str, Any]) -> dict[str, Any]:
        """Invoke ``tools/call`` and return the raw ``CallToolResult``."""
        self.ensure_initialized()
        with self._lock:
            message = self._send(
                {
                    "jsonrpc": "2.0",
                    "id": self._next_id(),
                    "method": "tools/call",
                    "params": {"name": name, "arguments": arguments},
                },
                expect_response=True,
            )
        return self._result_or_raise(message)

    # -- transport ----------------------------------------------------------
    def _next_id(self) -> int:
        self._id += 1
        return self._id

    def _headers(self) -> dict[str, str]:
        headers = {
            "Content-Type": "application/json",
            "Accept": "application/json, text/event-stream",
        }
        headers.update(self._static_headers)
        if self._negotiated_version:
            headers["MCP-Protocol-Version"] = self._negotiated_version
        if self._session_id:
            headers["Mcp-Session-Id"] = self._session_id
        return headers

    def _send(self, payload: dict[str, Any], *, expect_response: bool) -> dict[str, Any] | None:
        data = json.dumps(payload).encode("utf-8")
        request = urllib.request.Request(
            self.endpoint, data=data, method="POST", headers=self._headers()
        )
        try:
            with urllib.request.urlopen(request, timeout=self.timeout) as resp:
                sid = resp.headers.get("Mcp-Session-Id")
                if sid:
                    self._session_id = sid
                body = resp.read()
                content_type = resp.headers.get("Content-Type", "")
        except urllib.error.HTTPError as exc:
            detail = exc.read().decode("utf-8", "replace")[:500]
            raise McpClientError(f"HTTP {exc.code} from {self.endpoint}: {detail}")
        except (urllib.error.URLError, OSError) as exc:
            raise McpClientError(f"Cannot reach {self.endpoint}: {exc}")

        if not expect_response:
            return None
        return self._parse_body(body, content_type)

    @staticmethod
    def _parse_body(body: bytes, content_type: str) -> dict[str, Any] | None:
        text = body.decode("utf-8", "replace").strip()
        if not text:
            return None
        if "text/event-stream" in content_type:
            messages = []
            for line in text.splitlines():
                line = line.strip()
                if line.startswith("data:"):
                    chunk = line[len("data:"):].strip()
                    if chunk and chunk != "[DONE]":
                        try:
                            messages.append(json.loads(chunk))
                        except json.JSONDecodeError:
                            continue
            for message in messages:
                if isinstance(message, dict) and ("result" in message or "error" in message):
                    return message
            return messages[-1] if messages else None
        try:
            return json.loads(text)
        except json.JSONDecodeError as exc:
            raise McpClientError(f"Malformed response from {content_type or 'server'}: {exc}")

    @staticmethod
    def _result_or_raise(message: dict[str, Any] | None) -> dict[str, Any]:
        if message is None:
            raise McpClientError("Empty response from MCP server")
        error = message.get("error")
        if error:
            raise McpClientError(
                f"MCP error {error.get('code')}: {error.get('message')}"
            )
        result = message.get("result")
        if not isinstance(result, dict):
            raise McpClientError("MCP response is missing a result object")
        return result
