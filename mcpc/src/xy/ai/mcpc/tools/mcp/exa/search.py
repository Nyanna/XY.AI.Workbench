"""``web_search_exa`` - stage 1 of the two-stage Exa search retrieval.

Runs a search and caches each full result (incl. text and url) by id;
returns only an overview list without text/url. Call ``web_search_exa_results``
with the returned ids to resolve url and full text.
"""
import re
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.mcp.bridge import McpBridgeError, compact
from xy.ai.mcpc.tools.mcp.exa.bridge import get_bridge
from xy.ai.mcpc.tools.mcp.exa.core import extract_results, logger, normalize_item, search_cache, strip_empty
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
'#: Matches one "Title:/URL:/Published:/Author:/Highlights:" block of the'
'#: markdown-ish plain-text format the Exa remote MCP server sends when it'
'#: does not return ``structuredContent`` (observed in practice on'
'#: ``mcp.exa.ai``: it never sends ``structuredContent`` for this tool).'
'#: Consecutive blocks are separated by a line containing only ``---``.'
_SEARCH_BLOCK_RE = re.compile(
    '^Title:[ \\t]*(?P<title>[^\\n]*)\\nURL:[ \\t]*(?P<url>[^\\n]*)\\nPublished:[ \\t]*(?P<published>[^\\n]*)\\nAuthor:[ \\t]*(?P<author>[^\\n]*)\\nHighlights:\\n(?P<highlights>.*)\\Z',
    re.S)
_SEARCH_BLOCK_SEP = re.compile('\\n\\n---\\n\\n')
_HIGHLIGHT_SEP = re.compile('\\n\\.\\.\\.\\n')

def _parse_search_text(text: str) -> list[dict[str, Any]]:
    """Parse Exa's plain-text ``web_search_exa`` fallback format.

    Each result renders as::

        Title: <title>
        URL: <url>
        Published: <date or "N/A">
        Author: <author or "N/A">
        Highlights:
        <highlight 1>
        ...
        <highlight 2>

    with consecutive results separated by a blank line, ``---``, blank line.
    Blocks that do not match this shape are skipped (and logged), since the
    remote's exact wording is not covered by any spec and may change without
    notice.
    """
    items: list[dict[str, Any]] = []
    for block in _SEARCH_BLOCK_SEP.split(text.strip()):
        block = block.strip('\n')
        if not block:
            continue
        match = _SEARCH_BLOCK_RE.match(block)
        if match is None:
            logger.warning(
                'web_search_exa: could not parse a result block from fallback text, skipping: %r', block[:200])
            continue
        highlights_raw = match.group('highlights').strip()
        highlights = [h.strip() for h in _HIGHLIGHT_SEP.split(highlights_raw) if h.strip()] if highlights_raw else []
        published = match.group('published').strip()
        author = match.group('author').strip()
        items.append({'title': match.group('title').strip() or None,
                      'url': match.group('url').strip() or None,
                      'published_date': None if published in ('',
                                                              'N/A') else published,
                      'author': None if author in ('',
                                                   'N/A') else author,
                      'text': highlights_raw,
                      'highlights': highlights})
    return items

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
        McpBridgeError: if the Exa call fails, or the remote response has an
            unexpected shape (see ``extract_results``).
    """
    raw = _web_search_exa_raw(query, numResults)
    items = []
    for raw_item in extract_results(raw, 'web_search_exa', text_parser=_parse_search_text):
        try:
            items.append(normalize_item(raw_item))
        except Exception:
            logger.exception('web_search_exa: failed to normalize result item: %r', raw_item)
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
            logger.warning('web_search_exa failed: %s', exc)
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        except Exception as exc:
            logger.exception('web_search_exa: unexpected error')
            return ToolResult(content=[text_content(f'Unexpected error in web_search_exa: {exc}')], is_error=True)
        "# Keep 'results' present even when empty: it is a required output_schema"
        '# field, and strip_empty()-ing the whole dict here previously dropped it'
        '# entirely on empty results, producing a schema-violating, effectively'
        '# content-less ToolResult.'
        structured: dict[str, Any] = {'results': result.results}
        if result.autoprompt_string:
            structured['autoprompt_string'] = result.autoprompt_string
        return ToolResult(structured_content=structured)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(WebSearchExaTool())
    functions.register(web_search_exa)