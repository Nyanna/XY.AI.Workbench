"""Shared client state, dataclasses and helpers for the OpenAlex tools."""
import logging
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.openalex import OpenAlexAPIError, OpenAlexClient, Work, project_results
from xy.ai.mcpc.tools.tool_registry import ToolResult, text_content
from xy.ai.mcpc.utils.text_sanitize import sanitize_value
logger = logging.getLogger('xy.ai.mcpc.tools.openalex')
'#: Hard caps that mirror the OpenAlex API limits.'
MAX_PER_PAGE = 50
MAX_SEMANTIC_RESULTS = 50
DEFAULT_SEARCH_LIMIT = 25
DEFAULT_SEMANTIC_LIMIT = 10
_client: OpenAlexClient | None = None

class ResultCache:
    """In-memory store of full OpenAlex records, keyed by their OpenAlex id."""

    def __init__(self) -> None:
        self._items: dict[str, dict[str, Any]] = {}

    def put(self, item: dict[str, Any]) -> None:
        self._items[item['id']] = item

    def get_many(self, ids: list[str]) -> list[dict[str, Any]]:
        return [self._items[item_id] for item_id in ids if item_id in self._items]
'#: Shared between openalex_search / openalex_semantic_search and openalex_search_results.'
search_cache = ResultCache()
'#: Identity fields kept in the stage-1 overview; the full record (whatever'
'#: fields the chosen preset selected) is cached and available via'
'#: ``openalex_search_results``.'
_OVERVIEW_FIELDS = ('id', 'display_name', 'title', 'type', 'publication_year', 'relevance_score')

def _overview(item: Any) -> Any:
    if not isinstance(item, dict):
        return item
    return {k: item[k] for k in _OVERVIEW_FIELDS if k in item}

def build_client(config: ServerConfig) -> OpenAlexClient:
    """Build the OpenAlex client once, at registration time."""
    return OpenAlexClient(
        api_key=config.openalex_api_key,
        base_url=config.openalex_base_url,
        mailto=config.openalex_mailto)

def set_client(client: OpenAlexClient) -> None:
    global _client
    _client = client

def get_client() -> OpenAlexClient:
    return _client

def clamp(value: Any, default: int, maximum: int) -> int:
    try:
        number = int(value)
    except (TypeError, ValueError):
        return default
    return max(1, min(number, maximum))

def error_result(exc: Exception) -> ToolResult:
    message = str(exc)
    if isinstance(exc, OpenAlexAPIError) and exc.status is not None:
        message = f'{message} (status {exc.status})'
    logger.error('OpenAlex request failed: %s', message, exc_info=exc)
    return ToolResult(content=[text_content(message)], is_error=True)

def ok_result(structured: dict[str, Any]) -> ToolResult:
    """# OpenAlex occasionally returns fields (titles, abstracts, ...) that"""
    '# contain raw non-printable control characters; strip them so downstream'
    '# consumers (notably YAML block-scalar rendering) never choke on them.'
    return ToolResult(structured_content=sanitize_value(structured))

def summarise_list(data: dict[str, Any]) -> dict[str, Any]:
    meta = data.get('meta') or {}
    results = project_results(data.get('results') or [])
    structured: dict[str, Any] = {'count': meta.get('count'), 'returned': len(
        results), 'page': meta.get('page'), 'per_page': meta.get('per_page'), 'results': results}
    if data.get('group_by'):
        structured['group_by'] = data['group_by']
    return structured

@dataclass(frozen=True, slots=True)
class SearchResult:
    """Overview of an ``openalex_search``/``openalex_semantic_search`` call.

    ``results`` holds only identity fields (id, display_name/title, ...); the
    full record for each result is cached and resolved via
    ``openalex_search_results``.
    """
    count: int | None
    returned: int
    page: int | None
    per_page: int | None
    results: list[dict[str, Any]]
    group_by: list[dict[str, Any]] | None = None

@dataclass(frozen=True, slots=True)
class WorkResult:
    """Parsed ``openalex_work`` response."""
    work: Work

def to_search_overview(structured: dict[str, Any]) -> SearchResult:
    """Cache each full result and build the stage-1 overview response.

    ``structured`` is the output of ``summarise_list``. Each result is cached
    under its OpenAlex id in ``search_cache``; resolve full records via
    ``openalex_search_results``.
    """
    results = []
    for item in structured.get('results', []):
        if isinstance(item, dict) and item.get('id'):
            search_cache.put(item)
        results.append(_overview(item))
    return SearchResult(count=structured.get('count'), returned=structured.get('returned', 0), page=structured.get(
        'page'), per_page=structured.get('per_page'), results=results, group_by=structured.get('group_by'))

def overview_structured(result: SearchResult) -> dict[str, Any]:
    """Flatten a ``SearchResult`` back into ``structured_content`` shape."""
    structured: dict[str, Any] = {'count': result.count, 'returned': result.returned,
                                  'page': result.page, 'per_page': result.per_page, 'results': result.results}
    if result.group_by:
        structured['group_by'] = result.group_by
    return structured