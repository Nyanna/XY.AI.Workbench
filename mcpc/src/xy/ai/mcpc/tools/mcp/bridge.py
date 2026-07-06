"""Base class for exposing hard-coded calls of an external MCP server as tools.

A :class:`McpBridge` owns a single lazily-created :class:`McpClient` and registers
one MCPC tool per forwarded call.  MCPC supplies its own tool descriptions and
input schemas; the target server's tool list is never fetched.  Any error
returned by the target server (transport, protocol, or a tool-level
``isError`` result) is surfaced back to the agent.
"""

from __future__ import annotations

import threading
from typing import Any, Callable

from ...config import ServerConfig
from ...registry import ToolContext, ToolRegistry, ToolResult, text_content
from .client import McpClient, McpClientError

#: Optional hook to adapt MCPC's tool arguments to the remote tool's shape.
ArgTransform = Callable[[dict[str, Any]], dict[str, Any]]


class McpBridge:
    """Bridges selected calls of one external MCP server into the registry."""

    def __init__(self) -> None:
        self._client: McpClient | None = None
        self._lock = threading.Lock()

    # -- to be implemented by concrete bridges ------------------------------
    def build_client(self, config: ServerConfig) -> McpClient:
        """Create the client for the target server (called once, lazily)."""
        raise NotImplementedError

    # -- connection ---------------------------------------------------------
    def get_client(self, config: ServerConfig) -> McpClient:
        with self._lock:
            if self._client is None:
                self._client = self.build_client(config)
            return self._client

    def call(
        self, config: ServerConfig, remote_tool: str, arguments: dict[str, Any]
    ) -> ToolResult:
        """Forward a call and translate the outcome into a :class:`ToolResult`."""
        try:
            client = self.get_client(config)
            result = client.call_tool(remote_tool, arguments)
        except McpClientError as exc:
            return ToolResult(
                content=[text_content(f"'{remote_tool}' failed: {exc}")],
                is_error=True,
            )
        return _to_tool_result(result)

    # -- registration -------------------------------------------------------
    def register_tool(
        self,
        registry: ToolRegistry,
        *,
        name: str,
        description: str,
        input_schema: dict[str, Any],
        remote_tool: str | None = None,
        transform: ArgTransform | None = None,
        title: str | None = None,
        output_schema: dict[str, Any] | None = None,
        annotations: dict[str, Any] | None = None,
    ) -> None:
        """Register a single forwarded call as an MCPC tool."""
        remote = remote_tool or name
        bridge = self

        @registry.tool(
            name,
            title=title or name,
            description=description,
            input_schema=input_schema,
            output_schema=output_schema,
            annotations=annotations or {"readOnlyHint": True, "openWorldHint": True},
        )
        def handler(ctx: ToolContext) -> ToolResult:
            config = ctx.services.config if ctx.services is not None else ServerConfig()
            arguments = dict(ctx.arguments)
            if transform is not None:
                arguments = transform(arguments)
            return bridge.call(config, remote, arguments)


def _to_tool_result(result: dict[str, Any]) -> ToolResult:
    """Mirror a remote ``CallToolResult`` into an MCPC :class:`ToolResult`."""
    content = result.get("content")
    if not isinstance(content, list):
        content = [text_content("")]
    structured = result.get("structuredContent")
    return ToolResult(
        content=content,
        structured_content=structured if isinstance(structured, dict) else None,
        is_error=bool(result.get("isError", False)),
    )
