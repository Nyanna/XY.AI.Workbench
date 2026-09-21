"""``openalex_search_results`` - stage 2: resolve openalex_search / openalex_semantic_search ids to full records."""
from typing import Any
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult
from xy.ai.mcpc.tools._tool_helpers import require_items
from ._common import logger, ok_result, search_cache
__all__ = ['openalex_search_results', 'OpenalexSearchResultsTool']
_DESCRIPTION = 'Resolve ids returned by openalex_search or openalex_semantic_search to their full record.'
_INPUT_SCHEMA: dict[str,
                    Any] = {'type': 'object',
                            'properties': {'ids': {'type': 'array',
                                                   'items': {'type': 'string'},
                                                   'minItems': 1,
                                                   'description': 'Result ids (OpenAlex ids) returned by openalex_search or openalex_semantic_search.'}},
                            'required': ['ids']}

def openalex_search_results(ids: list[str]) -> list[dict[str, Any]]:
    """Resolve ids from a prior openalex_search/openalex_semantic_search call to full records.

    Args:
        ids: OpenAlex ids returned by ``openalex_search`` or
            ``openalex_semantic_search``.

    Returns:
        One full record per known id.
    """
    items = search_cache.get_many(ids)
    found = {item['id'] for item in items}
    missing = [i for i in ids if i not in found]
    if missing:
        logger.warning('openalex_search_results: unknown or expired id(s): %s', missing)
    return items

class OpenalexSearchResultsTool(ToolDefinition):
    name = 'openalex_search_results'
    title = 'OpenAlex search results'
    description = _DESCRIPTION
    input_schema = _INPUT_SCHEMA

    def handle(self, ctx: ToolContext) -> ToolResult:
        ids, error = require_items(ctx, key='ids')
        if error is not None:
            return error
        return ok_result({'results': openalex_search_results(ids=ids)})