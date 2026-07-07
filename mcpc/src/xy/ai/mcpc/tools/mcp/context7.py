"""Context7 bridge – library documentation tools backed by the Context7 MCP server.

Exposes two tools:
  context7-resolve-library-id  →  resolveLibraryId
  context7-query-docs          →  queryDocs
"""

from __future__ import annotations

from typing import Any

from ...config import ServerConfig
from ...registry import ToolRegistry
from .bridge import McpBridge
from .client import McpClient

# ---------------------------------------------------------------------------
# Tool: resolveLibraryId
# ---------------------------------------------------------------------------

_RESOLVE_DESCRIPTION = (
    "Search Context7 for a library and return its canonical library ID.\n\n"
    "Best for: Resolving a library name to the ID needed by context7-query-docs.\n"
    "Returns: Ranked list of matching libraries with ID, title, description, "
    "snippet count, reputation, benchmark score, and available versions."
)
_RESOLVE_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "libraryName": {
            "type": "string",
            "description": "Library name to search for (e.g. 'react', 'next.js', 'vue').",
        },
        "query": {
            "type": "string",
            "description": (
                "User's original question or task – used for relevance ranking "
                "(e.g. 'How to manage state with hooks')."
            ),
        },
    },
    "required": ["libraryName", "query"],
}
_RESOLVE_OUTPUT: dict[str, Any] = {
    "type": "object",
    "properties": {
        "content": {
            "type": "string",
            "description": (
                "Ranked list of matching libraries. Each entry contains "
                "Context7-compatible library ID, title, description, code snippet count, "
                "source reputation, benchmark score, and available versions."
            ),
        },
    },
    "required": ["content"],
}

# ---------------------------------------------------------------------------
# Tool: queryDocs
# ---------------------------------------------------------------------------

_QUERY_DOCS_DESCRIPTION = (
    "Fetch documentation and code examples for a library from Context7.\n\n"
    "Best for: Retrieving accurate API docs, usage examples, and configuration guides "
    "for any library or framework.\n"
    "Use context7-resolve-library-id first to obtain the correct libraryId.\n"
    "Returns: Documentation snippets and code examples relevant to the query.\n\n"
    "Keep each query scoped to a single concept. For multi-concept questions, "
    "make separate calls per concept unless the question is about how the concepts interact.\n"
)
_QUERY_DOCS_SCHEMA: dict[str, Any] = {
    "type": "object",
    "properties": {
        "libraryId": {
            "type": "string",
            "description": (
                "Context7-compatible library ID as returned by context7-resolve-library-id "
                "(e.g. '/reactjs/react.dev', '/vercel/next.js'). "
                "Optionally suffix with a version: '/vercel/next.js/v14.3.0'."
            ),
        },
        "query": {
            "type": "string",
            "description": (
                "The question or task to find documentation for, scoped to a single concept. "
                "Be specific and include relevant details "
                "(e.g. 'React useEffect cleanup function examples')."
            ),
        },
    },
    "required": ["libraryId", "query"],
}
_QUERY_DOCS_OUTPUT: dict[str, Any] = {
    "type": "object",
    "properties": {
        "content": {
            "type": "string",
            "description": "Documentation snippets and code examples relevant to the query.",
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
    ``context7-query-docs`` tools."""
    bridge = bridge or Context7Bridge()

    bridge.register_tool(
        registry,
        name="context7-libraries",
        remote_tool="resolve-library-id",
        title="Context7 resolve library ID",
        description=_RESOLVE_DESCRIPTION,
        input_schema=_RESOLVE_SCHEMA,
        output_schema=_RESOLVE_OUTPUT,
        annotations=_RO,
    )
    bridge.register_tool(
        registry,
        name="context7-documentation",
        remote_tool="query-docs",
        title="Context7 query docs",
        description=_QUERY_DOCS_DESCRIPTION,
        input_schema=_QUERY_DOCS_SCHEMA,
        output_schema=_QUERY_DOCS_OUTPUT,
        annotations=_RO,
    )
