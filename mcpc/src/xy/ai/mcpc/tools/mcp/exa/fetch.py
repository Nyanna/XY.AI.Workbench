"""``web_fetch_exa`` - stage 1 of the two-stage Exa fetch retrieval.

Fetches page content and caches each full result (incl. text) by id; returns
an overview with url and file_stats-style text metrics, but no text. Call
``web_fetch_exa_results`` with the returned ids to resolve the full text.
"""
import re
from dataclasses import asdict, dataclass
from typing import Any
from xy.ai.mcpc.tools.file_stats import compute_text_stats
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.mcp.bridge import McpBridgeError, compact
from xy.ai.mcpc.tools.mcp.exa.bridge import get_bridge
from xy.ai.mcpc.tools.mcp.exa.core import extract_results, fetch_cache, logger, normalize_item, strip_empty
__all__ = ['WebFetchResult', 'web_fetch_exa', 'WebFetchExaTool', 'register']
_DESCRIPTION = "Read a webpage's full content as clean markdown. Use to read any URL."
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
                                                 'description': 'Result id; pass to web_fetch_exa_results for text.'},
                                          'url': {'type': 'string'},
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
'#: Matches the start of one item in the markdown-ish plain-text format the'
'#: Exa remote MCP server sends for ``web_fetch_exa`` when it does not return'
'#: ``structuredContent`` (observed in practice on ``mcp.exa.ai``: it never'
'#: sends ``structuredContent`` for this tool). Each fetched url renders as a'
'#: markdown H1 title line immediately followed by a ``URL:`` line, a blank'
"#: line, then the page's extracted markdown content; consecutive urls are"
'#: simply concatenated with no other separator.'
_FETCH_ITEM_RE = re.compile(
    '^# (?P<title>[^\\n]*)\\nURL:[ \\t]*(?P<url>\\S+)\\n(?:Published:[ \\t]*[^\\n]*\\n)?\\n', re.M)

def _parse_fetch_text(text: str) -> list[dict[str, Any]]:
    """Parse Exa's plain-text ``web_fetch_exa`` fallback format.

    Each fetched url renders as::

        # <title>
        URL: <url>

        <page content as markdown>

    with consecutive urls simply concatenated one after another (no
    delimiter beyond the next ``# <title>\\nURL: ...`` header). If no such
    header is found at all, this returns an empty list so the caller can
    report a proper error instead of guessing.
    """
    matches = list(_FETCH_ITEM_RE.finditer(text))
    if not matches:
        logger.warning('web_fetch_exa: no "# title / URL:" markers found in fallback text, cannot split into per-url items')
        return []
    items: list[dict[str, Any]] = []
    for i, match in enumerate(matches):
        start = match.end()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        body = text[start:end].strip('\n')
        items.append({'title': match.group('title').strip() or None,
                     'url': match.group('url').strip() or None, 'text': body})
    return items

def web_fetch_exa(urls: list[str], maxCharacters: int | None=None) -> WebFetchResult:
    """Read one or more webpages' full content as clean markdown.

    Best for: Extracting full content from known URLs. Batch multiple
    URLs in one call.

    Args:
        urls: URLs to fetch.
        maxCharacters: Maximum characters to extract per page (default: 3000).

    Returns:
        Overview per url (url, file_stats-style text metrics, no text);
        resolve ids via ``web_fetch_exa_results`` for the full text.

    Raises:
        McpBridgeError: if the Exa call fails, or the remote response has an
            unexpected shape (see ``extract_results``).
    """
    raw = _web_fetch_exa_raw(urls, maxCharacters)
    items = []
    for raw_item in extract_results(raw, 'web_fetch_exa', text_parser=_parse_fetch_text):
        try:
            items.append(normalize_item(raw_item))
        except Exception:
            logger.exception('web_fetch_exa: failed to normalize result item: %r', raw_item)
    for item in items:
        fetch_cache.put(item)
    overview = []
    for item in items:
        try:
            metrics = asdict(compute_text_stats(item.get('text') or ''))
        except Exception:
            logger.exception('web_fetch_exa: failed to compute text stats for item %s', item.get('id'))
            metrics = {}
        entry = {k: v for k, v in item.items() if k != 'text'}
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
            logger.warning('web_fetch_exa failed: %s', exc)
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        except Exception as exc:
            logger.exception('web_fetch_exa: unexpected error')
            return ToolResult(content=[text_content(f'Unexpected error in web_fetch_exa: {exc}')], is_error=True)
        return ToolResult(structured_content={'results': result.results})

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(WebFetchExaTool())
    functions.register(web_fetch_exa)