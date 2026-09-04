"""``web_fetch_exa`` - stage 1 of the two-stage Exa fetch retrieval.

Fetches page content and caches each full result (incl. text and url) by id;
returns only an overview with file_stats-style text metrics, no text/url.
Call ``web_fetch_exa_results`` with the returned ids to resolve url and text.
"""
from dataclasses import asdict, dataclass
from typing import Any
from xy.ai.mcpc.tools.file_stats import compute_text_stats
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.mcp.bridge import McpBridgeError, compact
from xy.ai.mcpc.tools.mcp.exa.bridge import get_bridge
from xy.ai.mcpc.tools.mcp.exa.core import fetch_cache, normalize_item, strip_empty
__all__ = ['WebFetchResult', 'web_fetch_exa', 'WebFetchExaTool', 'register']
_DESCRIPTION = "Read a webpage's full content as clean markdown. Use to read any URL.\n\nBest for: Extracting full content from known URLs. Batch multiple URLs in one call.\nReturns: an overview per url (id, title, text metrics) without text/url; call web_fetch_exa_results with the ids to get url and full text."
_INPUT_SCHEMA: dict[str,
                    Any] = {'type': 'object',
                            'properties': {'urls': {'type': 'array',
                                                    'items': {'type': 'string'},
                                                    'description': 'URLs to fetch. Batch multiple URLs in one call.'},
                                           'maxCharacters': {'type': 'integer',
                                                             'description': 'Maximum characters to extract per page (default: 3000).',
                                                             'minimum': 1}},
                            'required': ['urls']}
_METRICS_SCHEMA: dict[str,
                      Any] = {'size_bytes': {'type': 'integer'},
                              'lines': {'type': 'integer'},
                              'words': {'type': 'integer'},
                              'complexity': {'type': 'number'},
                              'line_length_max': {'type': 'integer'},
                              'line_length_min': {'type': 'integer'},
                              'line_length_avg': {'type': 'number'},
                              'words_per_line_avg': {'type': 'number'},
                              'checksum': {'type': 'string'}}
_ITEM_SCHEMA: dict[str,
                   Any] = {'type': 'object',
                           'properties': {'id': {'type': 'string',
                                                 'description': 'Result id; pass to web_fetch_exa_results for url and text.'},
                                          'title': {'type': 'string'},
                                          'author': {'type': 'string'},
                                          'summary': {'type': 'string'},
                                          'excerpt': {'type': 'array',
                                                      'items': {'type': 'string'},
                                                      'description': 'Short excerpt(s) of the page text.'},
                                          **_METRICS_SCHEMA},
                           'required': ['id']}
_OUTPUT_SCHEMA: dict[str, Any] = {'type': 'object', 'properties': {
    'results': {'type': 'array', 'items': _ITEM_SCHEMA}}, 'required': ['results']}
_ANNOTATIONS = {'readOnlyHint': True, 'openWorldHint': True}

@dataclass(frozen=True, slots=True)
class WebFetchResult:
    """Overview of a ``web_fetch_exa`` call; url/text via ``web_fetch_exa_results``."""
    results: list[dict[str, Any]]

def _web_fetch_exa_raw(urls: list[str], maxCharacters: int | None=None) -> dict[str, Any]:
    return get_bridge().call('web_fetch_exa', compact(urls=urls, maxCharacters=maxCharacters))

def web_fetch_exa(urls: list[str], maxCharacters: int | None=None) -> WebFetchResult:
    """Read one or more webpages' full content as clean markdown.

    Best for: Extracting full content from known URLs. Batch multiple
    URLs in one call.

    Args:
        urls: URLs to fetch.
        maxCharacters: Maximum characters to extract per page (default: 3000).

    Returns:
        Overview per url (file_stats-style text metrics, no text/url);
        resolve ids via ``web_fetch_exa_results`` for the full text.

    Raises:
        McpBridgeError: if the Exa call fails.
    """
    raw = _web_fetch_exa_raw(urls, maxCharacters)
    items = [normalize_item(item) for item in raw.get('results', [])]
    for item in items:
        fetch_cache.put(item)
    overview = []
    for item in items:
        metrics = asdict(compute_text_stats(item.get('text') or ''))
        entry = {k: v for k, v in item.items() if k not in ('text', 'url')}
        overview.append(strip_empty({**entry, **metrics}))
    return WebFetchResult(results=overview)

class WebFetchExaTool(ToolDefinition):
    name = 'web_fetch_exa'
    title = 'Exa web fetch'
    description = _DESCRIPTION
    input_schema = _INPUT_SCHEMA
    output_schema = _OUTPUT_SCHEMA
    annotations = _ANNOTATIONS

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            result = web_fetch_exa(urls=args['urls'], maxCharacters=args.get('maxCharacters'))
        except McpBridgeError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'results': result.results})

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(WebFetchExaTool())
    functions.register(web_fetch_exa)