"""``openalex_search_results`` - stage 2: resolve openalex_search / openalex_semantic_search ids to full records."""
from typing import Any
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolResult
from xy.ai.mcpc.tools._tool_helpers import require_items
from xy.ai.mcpc.tools.results import unknown_ids_warnings
from ._common import logger, ok_result, record_document, search_cache
from xy.ai.mcpc.tools.persist import PATH_PROPERTY, persist_result
__all__ = ['openalex_search_results', 'OpenalexSearchResultsTool']
_DESCRIPTION = 'Resolve ids returned by openalex_search or openalex_semantic_search to their full record. With `path`, the records are saved as Markdown.'
_INPUT_SCHEMA: dict[str,
                    Any] = {'type': 'object',
                            'properties': {'ids': {'type': 'array',
                                                   'items': {'type': 'string'},
                                                   'minItems': 1,
                                                   'description': 'Result ids (OpenAlex ids) returned by openalex_search or openalex_semantic_search.'}},
                            'required': ['ids']}
_INPUT_SCHEMA['properties']['path'] = PATH_PROPERTY

def _resolve(ids: list[str]) -> tuple[list[dict[str, Any]], list[str]]:
    items = search_cache.get_many(ids)
    found = {item['id'] for item in items}
    return (items, [i for i in ids if i not in found])

def openalex_search_results(ids: list[str]) -> list[dict[str, Any]]:
    """Resolve ids from a prior openalex_search/openalex_semantic_search call to full records.

    Args:
        ids: OpenAlex ids returned by ``openalex_search`` or
            ``openalex_semantic_search``.

    Returns:
        One full record per known id.
    """
    items, missing = _resolve(ids)
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
        items, missing = _resolve(ids)
        if ctx.arguments.get('path'):
            return persist_result([record_document(item) for item in items], ctx.arguments['path'],
                                  ctx, reason='file outlines of persisted results', warnings=unknown_ids_warnings(missing))
        return ok_result({'results': items}, warnings=unknown_ids_warnings(missing))