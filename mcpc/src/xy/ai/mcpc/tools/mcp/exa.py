"""Exa bridge – forwards ``web_search_exa`` and ``web_fetch_exa`` to Exa's MCP.

Exa is reached through its remote MCP server; the API key is taken from the
server configuration.  MCPC advertises its own descriptions and input schemas.
"""

from __future__ import annotations

from typing import Any

from xy.ai.mcpc.server.json_codec import JsonCodec
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
from xy.ai.mcpc.tools.mcp.client import McpClient, McpClientError

__all__ = [
    "ExaBridge",
    "web_search_exa",
    "web_fetch_exa",
    "WebSearchExaTool",
    "WebFetchExaTool",
    "register_exa_tools",
]

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

_RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}


def _coerce_urls(urls: list[str] | str) -> list[str]:
    """Accept a single URL or a JSON-encoded list for ``urls`` leniently."""
    if isinstance(urls, str):
        # Accept a JSON-encoded list carried as a string; a plain URL (or any
        # non-list) is wrapped as a single-element list.
        parsed = JsonCodec.try_decode(urls)
        return parsed if isinstance(parsed, list) else [urls]
    return urls


class ExaBridge(McpBridge):
    """Bridge to the Exa remote MCP server."""

    def build_client(self, config: ServerConfig) -> McpClient:
        api_key = config.exa_api_key
        if not api_key:
            raise McpClientError(
                "Exa API key is not configured (set MCPC_EXA_API_KEY / EXA_API_KEY)."
            )
        return McpClient(config.exa_mcp_url, headers={"x-api-key": api_key})


#: Module-level bridge, built by :func:`register_exa_tools`.
_bridge: ExaBridge | None = None


def _get_bridge() -> ExaBridge:
    """Return the module-level Exa bridge configured by :func:`register_exa_tools`."""
    if _bridge is None:
        raise McpBridgeError("Exa tools used before register_exa_tools() was called.")
    return _bridge


def web_search_exa(query: str, numResults: int | None = None) -> dict:
    """Search the web for any topic and get clean, ready-to-use content.

    Best for: Finding current information, facts, or answering questions
    about any topic.

    Args:
        query: Natural language search query; should be a semantically
            rich description of the ideal page.
        numResults: Number of search results to return (default: 10).

    Returns:
        Clean text content from the top results.

    Raises:
        McpBridgeError: if the Exa call fails.
    """
    return _get_bridge().call("web_search_exa", compact(query=query, numResults=numResults))


def web_fetch_exa(urls: list[str] | str, maxCharacters: int | None = None) -> dict:
    """Read a webpage's full content as clean markdown.

    Best for: Extracting full content from known URLs. Batch multiple
    URLs in one call.

    Args:
        urls: URL(s) to read; a single URL or a list of URLs.
        maxCharacters: Maximum characters to extract per page (default: 3000).

    Returns:
        Clean text content and metadata from the page(s).

    Raises:
        McpBridgeError: if the Exa call fails.
    """
    arguments = compact(urls=_coerce_urls(urls), maxCharacters=maxCharacters)
    return _get_bridge().call("web_fetch_exa", arguments)


class WebSearchExaTool(ToolDefinition):
    name = "web_search_exa"
    title = "Exa web search"
    description = _WEB_SEARCH_DESCRIPTION
    input_schema = _WEB_SEARCH_SCHEMA
    output_schema = _SEARCH_OUTPUT_SCHEMA
    annotations = _RO

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            result = web_search_exa(query=args["query"], numResults=args.get("numResults"))
        except McpBridgeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content=result)


class WebFetchExaTool(ToolDefinition):
    name = "web_fetch_exa"
    title = "Exa web fetch"
    description = _WEB_FETCH_DESCRIPTION
    input_schema = _WEB_FETCH_SCHEMA
    output_schema = _FETCH_OUTPUT_SCHEMA
    annotations = _RO

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            result = web_fetch_exa(urls=args["urls"], maxCharacters=args.get("maxCharacters"))
        except McpBridgeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content=result)


def register_exa_tools(
    registry: ToolRegistry,
    environment: AppEnvironment,
) -> None:
    """Register the Exa-backed ``web_search_exa`` and ``web_fetch_exa`` tools."""
    global _bridge
    _bridge = ExaBridge(environment.config)
    registry.register(WebSearchExaTool())
    registry.register(WebFetchExaTool())
    environment.functions.register(web_search_exa)
    environment.functions.register(web_fetch_exa)
