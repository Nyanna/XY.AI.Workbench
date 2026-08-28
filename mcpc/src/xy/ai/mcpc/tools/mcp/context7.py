"""Context7 bridge – library documentation tools backed by the Context7 MCP server.

Exposes two tools:
  context7_libraries      →  resolveLibraryId
  context7_documentation  →  queryDocs
"""

from __future__ import annotations

from typing import Any

from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
from xy.ai.mcpc.tools.mcp.client import McpClient

__all__ = [
    "Context7Bridge",
    "context7_libraries",
    "context7_documentation",
    "Context7LibrariesTool",
    "Context7DocumentationTool",
    "register_context7_tools",
]

_RESOLVE_DESCRIPTION = (
    "Search Context7 for a library and return its canonical library ID.\n\n"
    "Best for: Resolving a library name to the ID needed by context7_documentation.\n"
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

_QUERY_DOCS_DESCRIPTION = (
    "Fetch documentation and code examples for a library from Context7.\n\n"
    "Best for: Retrieving accurate API docs, usage examples, and configuration guides "
    "for any library or framework.\n"
    "Use context7_libraries first to obtain the correct libraryId.\n"
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
                "Context7-compatible library ID as returned by context7_libraries "
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

_RO: dict[str, Any] = {"readOnlyHint": True, "openWorldHint": True}


class Context7Bridge(McpBridge):
    """Bridge to the Context7 remote MCP server."""

    def build_client(self, config: ServerConfig) -> McpClient:
        headers: dict[str, str] = {}
        if config.context7_api_key:
            headers["CONTEXT7_API_KEY"] = config.context7_api_key
        return McpClient(config.context7_mcp_url, headers=headers)


#: Module-level bridge, built by :func:`register_context7_tools`.
_bridge: Context7Bridge | None = None


def _get_bridge() -> Context7Bridge:
    """Return the module-level Context7 bridge configured by :func:`register_context7_tools`."""
    if _bridge is None:
        raise McpBridgeError("Context7 tools used before register_context7_tools() was called.")
    return _bridge


def context7_libraries(libraryName: str, query: str) -> dict:
    """Search Context7 for a library and return its canonical library ID.

    Best for: Resolving a library name to the ID needed by
    ``context7_documentation``.

    Args:
        libraryName: Library name to search for (e.g. 'react', 'next.js', 'vue').
        query: User's original question or task, used for relevance ranking.

    Returns:
        Ranked list of matching libraries (ID, title, description, snippet
        count, reputation, benchmark score, versions).

    Raises:
        McpBridgeError: if the Context7 call fails.
    """
    return _get_bridge().call("resolve-library-id", compact(libraryName=libraryName, query=query))


def context7_documentation(libraryId: str, query: str) -> dict:
    """Fetch documentation and code examples for a library from Context7.

    Best for: Retrieving accurate API docs, usage examples, and
    configuration guides for any library or framework. Use
    ``context7_libraries`` first to obtain the correct libraryId. Keep
    each query scoped to a single concept.

    Args:
        libraryId: Context7-compatible library ID as returned by
            ``context7_libraries`` (e.g. '/reactjs/react.dev'), optionally
            suffixed with a version.
        query: The question or task to find documentation for, scoped to
            a single concept.

    Returns:
        Documentation snippets and code examples relevant to the query.

    Raises:
        McpBridgeError: if the Context7 call fails.
    """
    return _get_bridge().call("query-docs", compact(libraryId=libraryId, query=query))


class Context7LibrariesTool(ToolDefinition):
    name = "context7_libraries"
    title = "Context7 resolve library ID"
    description = _RESOLVE_DESCRIPTION
    input_schema = _RESOLVE_SCHEMA
    output_schema = _RESOLVE_OUTPUT
    annotations = _RO

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            result = context7_libraries(libraryName=args["libraryName"], query=args["query"])
        except McpBridgeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content=result)


class Context7DocumentationTool(ToolDefinition):
    name = "context7_documentation"
    title = "Context7 query docs"
    description = _QUERY_DOCS_DESCRIPTION
    input_schema = _QUERY_DOCS_SCHEMA
    output_schema = _QUERY_DOCS_OUTPUT
    annotations = _RO

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            result = context7_documentation(libraryId=args["libraryId"], query=args["query"])
        except McpBridgeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content=result)


def register_context7_tools(
    registry: ToolRegistry,
    environment: AppEnvironment,
) -> None:
    """Register the Context7-backed ``context7_libraries`` and
    ``context7_documentation`` tools."""
    global _bridge
    _bridge = Context7Bridge(environment.config)
    registry.register(Context7LibrariesTool())
    registry.register(Context7DocumentationTool())
    environment.functions.register(context7_libraries)
    environment.functions.register(context7_documentation)
