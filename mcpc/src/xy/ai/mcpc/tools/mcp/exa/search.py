"""``web_search_exa`` - stage 1 of the two-stage Exa search retrieval.

Runs a search and caches each full result (incl. text and url) by id;
returns only an overview list without text/url. Call ``web_search_exa_results``
with the returned ids to resolve url and full text.
"""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.mcp.bridge import McpBridgeError, compact
from xy.ai.mcpc.tools.mcp.exa.bridge import get_bridge
from xy.ai.mcpc.tools.mcp.exa.core import normalize_item, search_cache, strip_empty
__all__ = ['WebSearchResult', 'web_search_exa', 'WebSearchExaTool', 'register']
_DESCRIPTION = 'Search the web for any topic and get clean, ready-to-use content.\n\nBest for: Finding current information, facts, or answering questions about any topic.\nReturns: an overview per result (id, title, author, excerpt) without text or url; call web_search_exa_results with the ids to get url and full text.'
_INPUT_SCHEMA: dict[str,
                    Any] = {'type': 'object',
                            'properties': {'query': {'type': 'string',
                                                     'description': 'Natural language search query. Should be a semantically rich description of the ideal page.'},
                                           'numResults': {'type': 'integer',
                                                          'description': 'Number of search results to return (default: 10).',
                                                          'minimum': 1}},
                            'required': ['query']}
_ITEM_SCHEMA: dict[str,
                   Any] = {'type': 'object',
                           'properties': {'id': {'type': 'string',
                                                 'description': 'Result id; pass to web_search_exa_results for url and text.'},
                                          'title': {'type': 'string'},
                                          'author': {'type': 'string'},
                                          'excerpt': {'type': 'array',
                                                      'items': {'type': 'string'},
                                                      'description': 'Short excerpt(s) of the page text.'}},
                           'required': ['id']}
_OUTPUT_SCHEMA: dict[str, Any] = {'type': 'object', 'properties': {'results': {
    'type': 'array', 'items': _ITEM_SCHEMA}, 'autoprompt_string': {'type': 'string'}}, 'required': ['results']}
_ANNOTATIONS = {'readOnlyHint': True, 'openWorldHint': True}

@dataclass(frozen=True, slots=True)
class WebSearchResult:
    """Overview of a ``web_search_exa`` call; url/text via ``web_search_exa_results``."""
    results: list[dict[str, Any]]
    autoprompt_string: str | None = None

def _web_search_exa_raw(query: str, numResults: int | None=None) -> dict[str, Any]:
    return get_bridge().call('web_search_exa', compact(query=query, numResults=numResults))

def web_search_exa(query: str, numResults: int | None=None) -> WebSearchResult:
    """Search the web for any topic and get clean, ready-to-use content.

    Best for: Finding current information, facts, or answering questions
    about any topic.

    Args:
        query: Natural language search query; should be a semantically
            rich description of the ideal page.
        numResults: Number of search results to return (default: 10).

    Returns:
        Overview per result (no text/url); resolve ids via
        ``web_search_exa_results`` for the full text.

    Raises:
        McpBridgeError: if the Exa call fails.
    """
    raw = _web_search_exa_raw(query, numResults)
    items = [normalize_item(item) for item in raw.get('results', [])]
    for item in items:
        search_cache.put(item)
    overview = [strip_empty({k: v for k, v in item.items() if k not in ('text', 'url')}) for item in items]
    return WebSearchResult(results=overview, autoprompt_string=raw.get('autoprompt_string'))

class WebSearchExaTool(ToolDefinition):
    name = 'web_search_exa'
    title = 'Exa web search'
    description = _DESCRIPTION
    input_schema = _INPUT_SCHEMA
    output_schema = _OUTPUT_SCHEMA
    annotations = _ANNOTATIONS

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            result = web_search_exa(query=args['query'], numResults=args.get('numResults'))
        except McpBridgeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        structured = strip_empty({'results': result.results, 'autoprompt_string': result.autoprompt_string})
        return ToolResult(structured_content=structured)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(WebSearchExaTool())
    functions.register(web_search_exa)