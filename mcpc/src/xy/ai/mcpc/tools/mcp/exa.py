"""Exa bridge – forwards ``web_search_exa`` and ``web_fetch_exa`` to Exa's MCP.

Exa is reached through its remote MCP server; the API key is taken from the
server configuration.  MCPC advertises its own descriptions and input schemas.
"""

from __future__ import annotations

from typing import Any

from ...codec import JsonCodec
from ...config import ServerConfig
from ...registry import ToolRegistry
from .bridge import McpBridge
from .client import McpClient, McpClientError

_WEB_SEARCH_DESCRIPTION = (
    "Search the web for any topic and get clean, ready-to-use content.\n\n"
    "Best for: Finding current information, facts, or "
    "answering questions about any topic.\n"
    "Returns: Clean text content from top search results."
)
_WEB_SEARCH_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "query": {
            "type": "string",
            "description": (
                "Natural language search query. Should be a semantically rich "
                "description of the ideal page."
            ),
        },
        "numResults": {
            "type": "integer",
            "description": "Number of search results to return (default: 10).",
            "minimum": 1,
        },
    },
    "required": ["query"],
}
_SEARCH_OUTPUT_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "content": {
            "type": "string",
            "description": "Clean text content from the top search results.",
        },
    },
    "required": ["content"],
}

_WEB_FETCH_DESCRIPTION = (
    "Read a webpage's full content as clean markdown. Use to read any URL.\n\n"
    "Best for: Extracting full content from known URLs. Batch multiple URLs in "
    "one call.\n"
    "Returns: Clean text content and metadata from the page(s)."
)
_WEB_FETCH_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "urls": {
            "type": "array",
            "items": {"type": "string"},
            "description": "URLs to read. Batch multiple URLs in one call.",
        },
        "maxCharacters": {
            "type": "integer",
            "description": "Maximum characters to extract per page (default: 3000)",
            "minimum": 1,
        },
    },
    "required": ["urls"],
}
_FETCH_OUTPUT_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "content": {
            "type": "string",
            "description": "Clean text content extracted from the requested page(s).",
        },
    },
    "required": ["content"],
}


def _coerce_urls(arguments: dict[str, Any]) -> dict[str, Any]:
    """Accept a single URL or a JSON-encoded list for ``urls`` leniently."""
    urls = arguments.get("urls")
    if isinstance(urls, str):
        # Accept a JSON-encoded list carried as a string; a plain URL (or any
        # non-list) is wrapped as a single-element list.
        parsed = JsonCodec.try_decode(urls)
        arguments["urls"] = parsed if isinstance(parsed, list) else [urls]
    return arguments


class ExaBridge(McpBridge):
    """Bridge to the Exa remote MCP server."""

    def build_client(self, config: ServerConfig) -> McpClient:
        api_key = config.exa_api_key
        if not api_key:
            raise McpClientError(
                "Exa API key is not configured (set MCPC_EXA_API_KEY / EXA_API_KEY)."
            )
        return McpClient(config.exa_mcp_url, headers={"x-api-key": api_key})


def register_exa_tools(registry: ToolRegistry, bridge: "ExaBridge | None"=None) -> None:
    """Register the Exa-backed ``web_search_exa`` and ``web_fetch_exa`` tools."""
    bridge = bridge or ExaBridge()
    bridge.register_tool(
        registry,
        name="web-search-exa",
        remote_tool="web_search_exa",
        title="Exa web search",
        description=_WEB_SEARCH_DESCRIPTION,
        input_schema=_WEB_SEARCH_SCHEMA,
        output_schema=_SEARCH_OUTPUT_SCHEMA,
    )
    bridge.register_tool(
        registry,
        name="web-fetch-exa",
        remote_tool="web_fetch_exa",
        title="Exa web fetch",
        description=_WEB_FETCH_DESCRIPTION,
        input_schema=_WEB_FETCH_SCHEMA,
        output_schema=_FETCH_OUTPUT_SCHEMA,
        transform=_coerce_urls,
    )
