"""Exa bridge – forwards ``web_search_exa`` and ``web_fetch_exa`` to Exa's MCP.

Exa is reached through its remote MCP server; the API key is taken from the
server configuration.  MCPC advertises its own descriptions and input schemas.
"""

from __future__ import annotations

import json
from typing import Any

from ...config import ServerConfig
from ...registry import ToolRegistry
from .bridge import McpBridge
from .client import McpClient, McpClientError

_WEB_SEARCH_DESCRIPTION = (
    "Search the web for any topic and get clean, ready-to-use content.\n\n"
    "Best for: Finding current information, news, facts, people, companies, or "
    "answering questions about any topic.\n"
    "Returns: Clean text content from top search results.\n\n"
    "Query tips:\n"
    "describe the ideal page, not keywords. \"blog post comparing React and Vue "
    "performance\" not \"React vs Vue\".\n"
    "Use category:people / category:company to search through Linkedin profiles "
    "/ companies respectively.\n"
    "If highlights are insufficient, follow up with web_fetch_exa on the best URLs."
)

_WEB_FETCH_DESCRIPTION = (
    "Read a webpage's full content as clean markdown. Use after web_search_exa "
    "when highlights are insufficient or to read any URL.\n\n"
    "Best for: Extracting full content from known URLs. Batch multiple URLs in "
    "one call.\n"
    "Returns: Clean text content and metadata from the page(s)."
)

_WEB_SEARCH_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "query": {
            "type": "string",
            "description": (
                "Natural language search query. Should be a semantically rich "
                "description of the ideal page, not just keywords. Optionally "
                "include category:<type> (company, people) to focus results — "
                "e.g. 'category:people John Doe software engineer'."
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


def _coerce_urls(arguments: dict[str, Any]) -> dict[str, Any]:
    """Accept a single URL or a JSON-encoded list for ``urls`` leniently."""
    urls = arguments.get("urls")
    if isinstance(urls, str):
        try:
            parsed = json.loads(urls)
            arguments["urls"] = parsed if isinstance(parsed, list) else [urls]
        except json.JSONDecodeError:
            arguments["urls"] = [urls]
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


def register_exa_tools(registry: ToolRegistry, bridge: "ExaBridge | None" = None) -> None:
    """Register the Exa-backed ``web_search_exa`` and ``web_fetch_exa`` tools."""
    bridge = bridge or ExaBridge()
    bridge.register_tool(
        registry,
        name="web_search_exa",
        title="Exa web search",
        description=_WEB_SEARCH_DESCRIPTION,
        input_schema=_WEB_SEARCH_SCHEMA,
    )
    bridge.register_tool(
        registry,
        name="web_fetch_exa",
        title="Exa web fetch",
        description=_WEB_FETCH_DESCRIPTION,
        input_schema=_WEB_FETCH_SCHEMA,
        transform=_coerce_urls,
    )
