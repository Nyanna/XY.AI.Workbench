"""``web_search_exa_results`` - stage 2: resolve ``web_search_exa`` ids to url and text."""
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.mcp.exa.core import search_cache, strip_empty
__all__ = ['web_search_exa_results', 'WebSearchExaResultsTool', 'register']
_DESCRIPTION = 'Resolve ids returned by web_search_exa to their url and full text.\n\nBest for: reading the full content of specific web_search_exa results.'
_INPUT_SCHEMA: dict[str, Any] = {'type': 'object', 'properties': {'ids': {'type': 'array', 'items': {
    'type': 'string'}, 'description': 'Result ids returned by web_search_exa.'}}, 'required': ['ids']}
_OUTPUT_SCHEMA: dict[str,
                     Any] = {'type': 'object',
                             'properties': {'results': {'type': 'array',
                                                        'items': {'type': 'object',
                                                                  'properties': {'id': {'type': 'string'},
                                                                                 'url': {'type': 'string'},
                                                                                 'text': {'type': 'string'}},
                                                                  'required': ['id']}}},
                             'required': ['results']}
_ANNOTATIONS = {'readOnlyHint': True, 'openWorldHint': False}

def web_search_exa_results(ids: list[str]) -> list[dict[str, Any]]:
    """Resolve ids from a prior ``web_search_exa`` call to url and full text.

    Args:
        ids: Result ids returned by ``web_search_exa``.

    Returns:
        One entry per known id, with ``id``, ``url`` and ``text``.
    """
    items = search_cache.get_many(ids)
    return [strip_empty({'id': item['id'], 'url': item.get('url'), 'text': item.get('text')}) for item in items]

class WebSearchExaResultsTool(ToolDefinition):
    name = 'web_search_exa_results'
    title = 'Exa web search results'
    description = _DESCRIPTION
    input_schema = _INPUT_SCHEMA
    output_schema = _OUTPUT_SCHEMA
    annotations = _ANNOTATIONS

    def handle(self, ctx: ToolContext) -> ToolResult:
        results = web_search_exa_results(ids=ctx.arguments['ids'])
        return ToolResult(structured_content={'results': results}, auto_approve=True)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(WebSearchExaResultsTool())
    functions.register(web_search_exa_results)