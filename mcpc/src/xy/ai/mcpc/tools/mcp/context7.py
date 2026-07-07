"""Context7 bridge – library documentation tools backed by the Context7 MCP server.

Exposes two tools:
  context7-resolve-library-id  →  resolve-library-id
  context7-get-library-docs    →  get-library-docs
"""

from __future__ import annotations

from typing import Any

from ...config import ServerConfig
from ...registry import ToolRegistry
from .bridge import McpBridge
from .client import McpClient, McpClientError

# ---------------------------------------------------------------------------
# Tool: resolve-library-id
# ---------------------------------------------------------------------------

_RESOLVE_LIBRARY_DESCRIPTION = (
    "Search Context7 for a library and return its canonical library ID.\n\n"
    "Best for: Resolving a library name to the ID needed by context7-get-library-docs.\n"
    "Returns: Ranked list of matching libraries with metadata (id, title, description, "
    "stars, versions)."
)
_RESOLVE_LIBRARY_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "libraryName": {
            "type": "string",
            "description": "Library name to search for (e.g. 'react', 'nextjs', 'express').",
        },
        "query": {
            "type": "string",
            "description": (
                "User's original question or task – used for intelligent relevance ranking "
                "(e.g. 'How to manage state with hooks')."
            ),
        },
    },
    "required": ["libraryName", "query"],
}
_RESOLVE_LIBRARY_OUTPUT: dict[str, Any] = {
    "type": "object",
    "properties": {
        "content": {
            "type": "string",
            "description": (
                "Ranked list of matching libraries. Each entry contains "
                "id, title, description, stars, trustScore, and available versions."
            ),
        },
    },
    "required": ["content"],
}

# ---------------------------------------------------------------------------
# Tool: get-library-docs
# ---------------------------------------------------------------------------

_GET_LIBRARY_DOCS_DESCRIPTION = (
    "Fetch documentation and code examples for a library from Context7.\n\n"
    "Best for: Retrieving accurate API docs, usage examples, and configuration guides "
    "for any library or framework.\n"
    "Use context7-resolve-library-id first to obtain the correct libraryId.\n"
    "Returns: Curated documentation snippets relevant to the given topic."
)
_GET_LIBRARY_DOCS_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "context7CompatibleLibraryID": {
            "type": "string",
            "description": (
                "Context7 library ID as returned by context7-resolve-library-id "
                "(e.g. '/vercel/next.js', '/facebook/react')."
            ),
        },
        "topic": {
            "type": "string",
            "description": (
                "Topic or question to focus the documentation on "
                "(e.g. 'hooks', 'server components', 'authentication')."
            ),
        },
        "tokens": {
            "type": "integer",
            "description": "Maximum number of tokens to return (default: 10000).",
            "minimum": 1,
        },
    },
    "required": ["context7CompatibleLibraryID", "topic"],
}
_GET_LIBRARY_DOCS_OUTPUT: dict[str, Any] = {
    "type": "object",
    "properties": {
        "content": {
            "type": "string",
            "description": "Documentation snippets and code examples relevant to the topic.",
        },
    },
    "required": ["content"],
}

# ---------------------------------------------------------------------------
# Bridge
# ---------------------------------------------------------------------------

_RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}


class Context7Bridge(McpBridge):
    """Bridge to the Context7 remote MCP server."""

    def build_client(self, config: ServerConfig) -> McpClient:
        headers: dict[str, str] = {}
        if config.context7_api_key:
            headers["CONTEXT7_API_KEY"] = config.context7_api_key
        return McpClient(config.context7_mcp_url, headers=headers)


def register_context7_tools(
    registry: ToolRegistry, bridge: "Context7Bridge | None" = None
) -> None:
    """Register the Context7-backed ``context7-resolve-library-id`` and
    ``context7-get-library-docs`` tools."""
    bridge = bridge or Context7Bridge()

    bridge.register_tool(
        registry,
        name="context7-libraries",
        remote_tool="resolve-library-id",
        title="Context7 resolve library ID",
        description=_RESOLVE_LIBRARY_DESCRIPTION,
        input_schema=_RESOLVE_LIBRARY_SCHEMA,
        output_schema=_RESOLVE_LIBRARY_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="context7-documentation",
        remote_tool="get-library-docs",
        title="Context7 get library docs",
        description=_GET_LIBRARY_DOCS_DESCRIPTION,
        input_schema=_GET_LIBRARY_DOCS_SCHEMA,
        output_schema=_GET_LIBRARY_DOCS_OUTPUT,
        annotations=_RO,
    )
