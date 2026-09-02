"""Context7 bridge – library documentation tools backed by the Context7 MCP server.

Exposes two tools:
  context7_libraries      →  resolveLibraryId
  context7_documentation  →  queryDocs
"""
import re
from dataclasses import asdict, dataclass
from typing import Any
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
from xy.ai.mcpc.tools.mcp.bridge import McpBridge, McpBridgeError, compact
from xy.ai.mcpc.tools.mcp.client import McpClient
__all__ = [
    'Context7Bridge',
    'Library',
    'DocumentationSection',
    'context7_libraries',
    'context7_documentation',
    'Context7LibrariesTool',
    'Context7DocumentationTool',
    'register_context7_tools']
_RESOLVE_DESCRIPTION = 'Search Context7 for a library and return its canonical library ID.\n\nBest for: Resolving a library name to the ID needed by context7_documentation.\nReturns: Ranked list of matching libraries with library ID, title, and description.'
_RESOLVE_SCHEMA: dict[str,
                      Any] = {'type': 'object',
                              'properties': {'libraryName': {'type': 'string',
                                                             'description': "Library name to search for (e.g. 'react', 'next.js', 'vue')."},
                                             'query': {'type': 'string',
                                                       'description': "User's original question or task – used for relevance ranking (e.g. 'How to manage state with hooks')."}},
                              'required': ['libraryName',
                                           'query']}
_RESOLVE_OUTPUT: dict[str,
                      Any] = {'type': 'object',
                              'properties': {'libraries': {'type': 'array',
                                                           'description': 'Ranked list of matching libraries.',
                                                           'items': {'type': 'object',
                                                                     'properties': {'title': {'type': 'string'},
                                                                                    'library_id': {'type': 'string',
                                                                                                   'description': 'Context7-compatible library ID.'},
                                                                                    'description': {'type': 'string'}}}}},
                              'required': ['libraries']}
_QUERY_DOCS_DESCRIPTION = 'Fetch documentation and code examples for a library from Context7.\n\nBest for: Retrieving accurate API docs, usage examples, and configuration guides for any library or framework.\nUse context7_libraries first to obtain the correct libraryId.\nReturns: Documentation snippets and code examples relevant to the query.\n\nKeep each query scoped to a single concept. For multi-concept questions, make separate calls per concept unless the question is about how the concepts interact.\n'
_QUERY_DOCS_SCHEMA: dict[str,
                         Any] = {'type': 'object',
                                 'properties': {'libraryId': {'type': 'string',
                                                              'description': "Context7-compatible library ID as returned by context7_libraries (e.g. '/reactjs/react.dev', '/vercel/next.js'). Optionally suffix with a version: '/vercel/next.js/v14.3.0'."},
                                                'query': {'type': 'string',
                                                          'description': "The question or task to find documentation for, scoped to a single concept. Be specific and include relevant details (e.g. 'React useEffect cleanup function examples')."}},
                                 'required': ['libraryId',
                                              'query']}
_QUERY_DOCS_OUTPUT: dict[str,
                         Any] = {'type': 'object',
                                 'properties': {'sections': {'type': 'array',
                                                             'description': 'Documentation sections',
                                                             'items': {'type': 'object',
                                                                       'properties': {'content': {'type': 'string'}},
                                                                       'required': ['content']}}},
                                 'required': ['sections']}
_RO: dict[str, Any] = {'readOnlyHint': True, 'openWorldHint': True}
_NOT_FOUND_TRIGGER = 'No documentation found for library'
_BLOCK_SEPARATOR = re.compile('(?m)^-{3,}\\s*$')
_LIBRARY_FIELD = re.compile('(?m)^-\\s*(.+?):\\s*(.*)$')

@dataclass(frozen=True, slots=True)
class Library:
    """One Context7 library search result."""
    title: str | None = None
    library_id: str | None = None
    description: str | None = None

@dataclass(frozen=True, slots=True)
class DocumentationSection:
    """One documentation section of a Context7 ``queryDocs`` response."""
    content: str

def _parse_libraries(text: str) -> list[Library]:
    libraries: list[Library] = []
    for block in _BLOCK_SEPARATOR.split(text):
        fields = {label.strip(): value.strip() for label, value in _LIBRARY_FIELD.findall(block)}
        if not fields:
            continue
        libraries.append(
            Library(
                title=fields.get('Title'),
                library_id=fields.get('Context7-compatible library ID'),
                description=fields.get('Description')))
    return libraries

def _parse_documentation(text: str) -> list[DocumentationSection]:
    if _NOT_FOUND_TRIGGER in text:
        raise McpBridgeError(text.strip())
    return [DocumentationSection(content=section.strip())
            for section in _BLOCK_SEPARATOR.split(text) if section.strip()]

class Context7Bridge(McpBridge):
    """Bridge to the Context7 remote MCP server."""

    def build_client(self, config: ServerConfig) -> McpClient:
        headers: dict[str, str] = {}
        if config.context7_api_key:
            headers['CONTEXT7_API_KEY'] = config.context7_api_key
        return McpClient(config.context7_mcp_url, headers=headers)
'#: Module-level bridge, built by :func:`register_context7_tools`.'
_bridge: Context7Bridge | None = None

def _get_bridge() -> Context7Bridge:
    """Return the module-level Context7 bridge configured by :func:`register_context7_tools`."""
    if _bridge is None:
        raise McpBridgeError('Context7 tools used before register_context7_tools() was called.')
    return _bridge

def context7_libraries(libraryName: str, query: str) -> list[Library]:
    """Search Context7 for a library and return its canonical library ID.

    Best for: Resolving a library name to the ID needed by
    ``context7_documentation``.

    Args:
        libraryName: Library name to search for (e.g. 'react', 'next.js', 'vue').
        query: User's original question or task, used for relevance ranking.

    Returns:
        Ranked list of matching libraries (title, library ID, description).

    Raises:
        McpBridgeError: if the Context7 call fails.
    """
    result = _get_bridge().call('resolve-library-id', compact(libraryName=libraryName, query=query))
    return _parse_libraries(result.get('content', ''))

def context7_documentation(libraryId: str, query: str) -> list[DocumentationSection]:
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
        Documentation, split into sections on '---' separators.

    Raises:
        McpBridgeError: if the Context7 call fails, or no documentation was found.
    """
    result = _get_bridge().call('query-docs', compact(libraryId=libraryId, query=query))
    return _parse_documentation(result.get('content', ''))

class Context7LibrariesTool(ToolDefinition):
    name = 'context7_libraries'
    title = 'Context7 resolve library ID'
    description = _RESOLVE_DESCRIPTION
    input_schema = _RESOLVE_SCHEMA
    output_schema = _RESOLVE_OUTPUT
    annotations = _RO

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            libraries = context7_libraries(libraryName=args['libraryName'], query=args['query'])
        except McpBridgeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'libraries': [asdict(library) for library in libraries]})

class Context7DocumentationTool(ToolDefinition):
    name = 'context7_documentation'
    title = 'Context7 query docs'
    description = _QUERY_DOCS_DESCRIPTION
    input_schema = _QUERY_DOCS_SCHEMA
    output_schema = _QUERY_DOCS_OUTPUT
    annotations = _RO

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            sections = context7_documentation(libraryId=args['libraryId'], query=args['query'])
        except McpBridgeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'sections': [asdict(section) for section in sections]})

def register_context7_tools(registry: ToolRegistry, environment: AppEnvironment) -> None:
    """Register the Context7-backed ``context7_libraries`` and
    ``context7_documentation`` tools."""
    global _bridge
    _bridge = Context7Bridge(environment.config)
    registry.register(Context7LibrariesTool())
    registry.register(Context7DocumentationTool())
    environment.functions.register(context7_libraries)
    environment.functions.register(context7_documentation)