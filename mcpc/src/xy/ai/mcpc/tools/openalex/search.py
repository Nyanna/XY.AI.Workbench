"""``openalex_search`` - stage 1 of the two-stage OpenAlex search retrieval.

Runs a keyword/boolean search and caches each full result by id; returns
only an identity overview (id, display_name/title, ...). Call
``openalex_search_results`` with the returned ids to resolve full records.
"""
from typing import Any
from xy.ai.mcpc.openalex import DEFAULT_SEARCH_PRESET, OpenAlexError, resolve_select
from xy.ai.mcpc.openalex.client import ENTITIES
from xy.ai.mcpc.openalex.presets import WORK_PRESET_NAMES
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult
from ._common import DEFAULT_SEARCH_LIMIT, MAX_PER_PAGE, SearchResult, clamp, error_result, get_client, ok_result, overview_structured, summarise_list, to_search_overview
_ENTITY_NAMES = sorted(ENTITIES)
_WORK_PRESETS = list(WORK_PRESET_NAMES)
_SEARCH_DESCRIPTION = 'Keyword and boolean full-text search across OpenAlex scholarly entities (works by default). Searches titles, abstracts and full text for works; names for authors, sources and institutions.\n\nQuery syntax: use uppercase AND / OR / NOT and double-quoted phrases, e.g. ("machine learning" OR "deep learning") NOT survey. Set exact=true for unstemmed matching and wildcards (machin*). Results are sorted by relevance and limited to the first page.\n\nReturns an identity overview per result (id, display_name/title, ...); call openalex_search_results with the ids to get the full record.'
_SEARCH_INPUT_SCHEMA: dict[str,
                           Any] = {'type': 'object',
                                   'properties': {'query': {'type': 'string',
                                                            'description': 'Full-text query. Supports boolean AND/OR/NOT (uppercase), quoted phrases and proximity ("a b"~5).'},
                                                  'entity': {'type': 'string',
                                                             'enum': _ENTITY_NAMES,
                                                             'description': 'Entity type to search (default: works).'},
                                                  'exact': {'type': 'boolean',
                                                            'description': 'Use exact (unstemmed) search; required for wildcards like machin*. Default: false.'},
                                                  'fields': {'type': 'string',
                                                             'enum': _WORK_PRESETS,
                                                             'description': 'Field preset controlling how much of each record is returned (works only). Default: core. Presets: minimal, core, bibliographic, authorship, access, metrics, topics, abstract, references, full.'},
                                                  'filter': {'type': 'string',
                                                             'description': "Optional OpenAlex filter string applied alongside the search, e.g. 'publication_year:>2020,is_oa:true'. Comma-separated key:value pairs."},
                                                  'sort': {'type': 'string',
                                                           'description': "Optional sort override, e.g. 'cited_by_count:desc' or 'publication_date:desc'. Defaults to relevance."},
                                                  'limit': {'type': 'integer',
                                                            'description': f'Max results from the first page (1-{MAX_PER_PAGE}, default {DEFAULT_SEARCH_LIMIT}).',
                                                            'minimum': 1,
                                                            'maximum': MAX_PER_PAGE}},
                                   'required': ['query']}

def _openalex_search_raw(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> dict[str, Any]:
    client = get_client()
    preset = fields or DEFAULT_SEARCH_PRESET
    resolved_limit = clamp(limit, DEFAULT_SEARCH_LIMIT, MAX_PER_PAGE)
    select = resolve_select(preset, entity)
    data = client.search_works(
        query,
        exact=exact,
        filters=filter,
        sort=sort,
        select=select,
        per_page=resolved_limit,
        page=1) if entity == 'works' else client.list_entities(
            entity,
            search_exact=query if exact else None,
            search=None if exact else query,
            filters=filter,
            sort=sort,
            select=select,
            per_page=resolved_limit,
        page=1)
    return summarise_list(data)

def openalex_search(query: str, entity: str='works', exact: bool=False, fields: str | None=None, filter: str | None=None, sort: str | None=None, limit: int | None=None) -> SearchResult:
    """Keyword/boolean full-text search across OpenAlex scholarly entities.

    Args:
        query: Full-text query (boolean AND/OR/NOT, quoted phrases, proximity).
        entity: Entity type to search (default: works).
        exact: Use exact (unstemmed) search; required for wildcards.
        fields: Field preset controlling how much of each record is returned.
        filter: Optional OpenAlex filter string.
        sort: Optional sort override; defaults to relevance.
        limit: Max results from the first page.

    Returns:
        Identity overview per result; resolve full records via
        ``openalex_search_results``.

    Raises:
        OpenAlexError: If the OpenAlex API request fails.
    """
    structured = _openalex_search_raw(
        query,
        entity=entity,
        exact=exact,
        fields=fields,
        filter=filter,
        sort=sort,
        limit=limit)
    return to_search_overview(structured)

class OpenalexSearchTool(ToolDefinition):
    name = 'openalex_search'
    title = 'OpenAlex search'
    description = _SEARCH_DESCRIPTION
    input_schema = _SEARCH_INPUT_SCHEMA

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            result = openalex_search(
                query=args['query'],
                entity=args.get(
                    'entity',
                    'works'),
                exact=bool(
                    args.get(
                        'exact',
                        False)),
                fields=args.get('fields'),
                filter=args.get('filter'),
                sort=args.get('sort'),
                limit=args.get('limit'))
        except OpenAlexError as exc:
            return error_result(exc)
        return ok_result(overview_structured(result))