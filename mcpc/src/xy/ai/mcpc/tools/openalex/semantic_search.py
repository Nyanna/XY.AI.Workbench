"""``openalex_semantic_search`` tool: AI (embedding) search by meaning."""
from typing import Any
from xy.ai.mcpc.openalex import DEFAULT_SEARCH_PRESET, OpenAlexError, resolve_select
from xy.ai.mcpc.openalex.presets import WORK_PRESET_NAMES
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult
from ._common import DEFAULT_SEMANTIC_LIMIT, MAX_SEMANTIC_RESULTS, SearchResult, clamp, error_result, get_client, ok_result, summarise_list, to_search_result
_WORK_PRESETS = list(WORK_PRESET_NAMES)
_SEMANTIC_SEARCH_DESCRIPTION = 'AI-powered semantic search over OpenAlex works: finds works by meaning using embeddings, even when the wording differs. Best for paragraph-length input such as an abstract, a research question or a grant aim (up to ~2000 characters).\n\nSupports most filters, but NOT cited_by_count or country_code filters. Returns at most 50 works from the first page, ranked by semantic similarity.'
_SEMANTIC_INPUT_SCHEMA: dict[str,
                             Any] = {'type': 'object',
                                     'properties': {'query': {'type': 'string',
                                                              'description': 'Natural-language description of what you are looking for. Longer, richer input yields better matches (truncated at 2000 characters).'},
                                                    'fields': {'type': 'string',
                                                               'enum': _WORK_PRESETS,
                                                               'description': 'Field preset for each work (default: core).'},
                                                    'filter': {'type': 'string',
                                                               'description': "Optional OpenAlex filter string, e.g. 'publication_year:>2020,is_oa:true'. Note: cited_by_count and country_code filters are not supported by semantic search."},
                                                    'limit': {'type': 'integer',
                                                              'description': f'Max results (1-{MAX_SEMANTIC_RESULTS}, default {DEFAULT_SEMANTIC_LIMIT}).',
                                                              'minimum': 1,
                                                              'maximum': MAX_SEMANTIC_RESULTS}},
                                     'required': ['query']}

def _openalex_semantic_search_raw(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> dict[str, Any]:
    client = get_client()
    preset = fields or DEFAULT_SEARCH_PRESET
    resolved_limit = clamp(limit, DEFAULT_SEMANTIC_LIMIT, MAX_SEMANTIC_RESULTS)
    select = resolve_select(preset, 'works')
    data = client.semantic_search_works(query, filters=filter, select=select, per_page=resolved_limit, page=1)
    return summarise_list(data)

def openalex_semantic_search(query: str, fields: str | None=None, filter: str | None=None, limit: int | None=None) -> SearchResult:
    """AI-powered semantic search over OpenAlex works.

    Args:
        query: Natural-language description of what to look for.
        fields: Field preset for each work (default: core).
        filter: Optional OpenAlex filter string (no cited_by_count/country_code).
        limit: Max results (1-50, default 10).

    Returns:
        Parsed search results (:class:`~xy.ai.mcpc.openalex.Work` records).

    Raises:
        OpenAlexError: If the OpenAlex API request fails.
    """
    structured = _openalex_semantic_search_raw(query, fields=fields, filter=filter, limit=limit)
    return to_search_result(structured, 'works')

class OpenalexSemanticSearchTool(ToolDefinition):
    name = 'openalex_semantic_search'
    title = 'OpenAlex semantic search'
    description = _SEMANTIC_SEARCH_DESCRIPTION
    input_schema = _SEMANTIC_INPUT_SCHEMA

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            structured = _openalex_semantic_search_raw(
                query=args['query'],
                fields=args.get('fields'),
                filter=args.get('filter'),
                limit=args.get('limit'))
        except OpenAlexError as exc:
            return error_result(exc)
        return ok_result(structured)