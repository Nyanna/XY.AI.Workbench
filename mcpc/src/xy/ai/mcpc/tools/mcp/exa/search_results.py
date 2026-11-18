"""``web_search_exa_results`` - stage 2: resolve ``web_search_exa`` ids to url and text."""
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.mcp.exa.core import logger, search_cache, strip_empty
from xy.ai.mcpc.tools._tool_helpers import require_items
from xy.ai.mcpc.tools.results import unknown_ids_warnings
from xy.ai.mcpc.tools.results import RawPayload
from xy.ai.mcpc.tools.persist import PATH_PROPERTY, persist_result
from xy.ai.mcpc.tools.mcp.exa.core import exa_document
__all__ = ['web_search_exa_results', 'WebSearchExaResultsTool', 'register']
_DESCRIPTION = 'Resolve ids returned by web_search_exa to their full text. With `path`, the results are saved as Markdown.'
_INPUT_SCHEMA: dict[str, Any] = {'type': 'object', 'properties': {'ids': {'type': 'array', 'items': {
    'type': 'string'}, 'minItems': 1, 'description': 'Result ids returned by web_search_exa.'}}, 'required': ['ids']}

def _resolve(ids: list[str]) -> tuple[list[dict[str, Any]], list[str]]:
    items = search_cache.get_many(ids)
    known = {item['id'] for item in items}
    results = [strip_empty({'id': item['id'], 'url': item.get('url'), 'text': item.get('text')}) for item in items]
    return (results, [i for i in ids if i not in known])
_INPUT_SCHEMA['properties']['path'] = PATH_PROPERTY

def web_search_exa_results(ids: list[str]) -> list[dict[str, Any]]:
    """Resolve ids from a prior ``web_search_exa`` call to url and full text.

    Args:
        ids: Result ids returned by ``web_search_exa``.

    Returns:
        One entry per known id, with ``id``, ``url`` and ``text``.
    """
    results, missing = _resolve(ids)
    if missing:
        logger.warning('web_search_exa_results: unknown or expired id(s): %s', missing)
    return results

class WebSearchExaResultsTool(ToolDefinition):
    name = 'web_search_exa_results'
    title = 'Exa web search results'
    description = _DESCRIPTION
    input_schema = _INPUT_SCHEMA

    def handle(self, ctx: ToolContext) -> ToolResult:
        ids, error = require_items(ctx, key='ids')
        if error is not None:
            return error
        path = ctx.arguments.get('path')
        try:
            if path:
                items = search_cache.get_many(ids)
                known = {item['id'] for item in items}
                return persist_result([exa_document(item) for item in items],
                                      path,
                                      ctx,
                                      reason='file outlines of persisted results',
                                      warnings=unknown_ids_warnings([i for i in ids if i not in known]))
            results, missing = _resolve(ids)
        except Exception as exc:
            logger.exception('web_search_exa_results failed')
            return ToolResult.failure(f'Error resolving web_search_exa results: {exc}')
        return ToolResult.ok(RawPayload({'results': results},
                                        reason='shape defined by the remote Exa MCP server',
                                        warnings=tuple(unknown_ids_warnings(missing))))

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(WebSearchExaResultsTool())
    functions.register(web_search_exa_results)