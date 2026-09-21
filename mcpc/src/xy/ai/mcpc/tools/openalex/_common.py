"""Shared client state, dataclasses and helpers for the OpenAlex tools."""
import logging
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.openalex import GroupByItem, OpenAlexAPIError, OpenAlexClient, OpenAlexRecord, Work, parse_entity, parse_group_by, project_results
from xy.ai.mcpc.tools.tool_registry import ToolResult, text_content
from xy.ai.mcpc.utils.text_sanitize import sanitize_value
logger = logging.getLogger('xy.ai.mcpc.tools.openalex')
'#: Hard caps that mirror the OpenAlex API limits.'
MAX_PER_PAGE = 50
MAX_SEMANTIC_RESULTS = 50
DEFAULT_SEARCH_LIMIT = 25
DEFAULT_SEMANTIC_LIMIT = 10
_client: OpenAlexClient | None = None

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
    """Parsed ``openalex_search`` / ``openalex_semantic_search`` response."""
    count: int | None
    returned: int
    page: int | None
    per_page: int | None
    results: list[OpenAlexRecord]
    group_by: list[GroupByItem] | None = None

@dataclass(frozen=True, slots=True)
class WorkResult:
    """Parsed ``openalex_work`` response."""
    work: Work

def to_search_result(structured: dict[str, Any], entity: str) -> SearchResult:
    group_by = structured.get('group_by')
    return SearchResult(
        count=structured.get('count'),
        returned=structured.get(
            'returned',
            0),
        page=structured.get('page'),
        per_page=structured.get('per_page'),
        results=[
            parse_entity(
                entity,
                item) for item in structured.get(
                'results',
                [])],
        group_by=parse_group_by(group_by) if group_by else None)