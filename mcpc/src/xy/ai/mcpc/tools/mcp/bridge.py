"""Utility for forwarding hard-coded calls of an external MCP server.
"""

from __future__ import annotations

import threading
from typing import Any

from xy.ai.mcpc.server.json_codec import JsonCodec
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.utils.text_sanitize import sanitize_text, sanitize_value
from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError


def compact(**kwargs: Any) -> dict[str, Any]:
    """Build a remote-call argument dict, dropping keys whose value is ``None``.

    Shared helper for the core functions in ``context7``, ``exa`` and
    ``github``, which forward only the arguments the caller actually
    supplied.
    """
    return {k: v for k, v in kwargs.items() if v is not None}


class McpBridgeError(RuntimeError):
    """Raised when a forwarded call fails, at transport level or because the
    remote tool itself reported ``isError``."""


class McpBridge:
    """Lazily connects to one external MCP server and forwards ``tools/call``."""

    def __init__(self, config: ServerConfig | None = None) -> None:
        self.config = config or ServerConfig()
        self._client: McpClient | None = None
        self._lock = threading.Lock()

    def build_client(self, config: ServerConfig) -> McpClient:
        """Create the client for the target server (called once, lazily)."""
        raise NotImplementedError

    def get_client(self) -> McpClient:
        with self._lock:
            if self._client is None:
                self._client = self.build_client(self.config)
            return self._client

    def call(self, remote_tool: str, arguments: dict[str, Any]) -> dict[str, Any]:
        """Forward a call and return the remote's structured result.

        Raises:
            McpBridgeError: if the transport/protocol fails, or the remote
                tool call itself reports ``isError``.
        """
        try:
            client = self.get_client()
            result = client.call_tool(remote_tool, arguments)
        except McpClientError as exc:
            raise McpBridgeError(f"'{remote_tool}' failed: {exc}") from exc
        return _extract_result(remote_tool, result)


def _extract_result(remote_tool: str, result: dict[str, Any]) -> dict[str, Any]:
    """Resolve a remote ``CallToolResult`` into structured data, or raise."""
    # Extract the text blocks from the remote content array; on error this is
    # the only material the agent gets to see, so it also becomes the
    # McpBridgeError message.
    raw_blocks = result.get("content")
    if isinstance(raw_blocks, list):
        texts = [
            block.get("text", "")
            for block in raw_blocks
            if isinstance(block, dict) and block.get("type") == "text"
        ]
        text = "\n".join(texts)
    else:
        text = ""
    # Some remote servers leak raw non-printable control bytes (e.g. an
    # unescaped 0x02) into text content; strip them so downstream consumers
    # (notably YAML block-scalar rendering) never choke on them.
    text = sanitize_text(text)

    if result.get("isError", False):
        raise McpBridgeError(text or f"'{remote_tool}' failed")

    # Use structuredContent from the remote server when present. Otherwise
    # recover it from the text: some servers only ever fill in the text
    # block, and that text is frequently a JSON document that was serialised
    # to a string rather than left as real structure. Parsing it here keeps
    # that substructure intact instead of swallowing it into a flat
    # ``{"content": text}`` string.
    structured = result.get("structuredContent")
    if isinstance(structured, dict):
        return sanitize_value(structured)
    parsed = JsonCodec.try_decode(text)
    return sanitize_value(parsed) if isinstance(parsed, dict) else {"content": text}
