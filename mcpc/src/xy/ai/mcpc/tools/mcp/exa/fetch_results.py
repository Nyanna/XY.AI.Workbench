"""``web_fetch_exa_results`` - stage 2: resolve ``web_fetch_exa`` ids to full text.

Optionally filters the text line-wise with an extended regular expression
(``grep -E`` semantics), including a configurable amount of context lines.
"""
import re
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.mcp.exa.core import fetch_cache, logger, strip_empty
__all__ = ['web_fetch_exa_results', 'WebFetchExaResultsTool', 'register']
_DESCRIPTION = 'Resolve ids returned by web_fetch_exa to their full text, optionally filter long pages.'
_INPUT_SCHEMA: dict[str,
                    Any] = {'type': 'object',
                            'properties': {'ids': {'type': 'array',
                                                   'items': {'type': 'string'},
                                                   'description': 'Result ids returned by web_fetch_exa.'},
                                           'pattern': {'type': 'string',
                                                       'description': 'Extended regular expression (grep -E semantics) to filter text lines.'},
                                           'context': {'type': 'integer',
                                                       'description': 'Context lines kept before/after each match (default: 1); only used with pattern.',
                                                       'minimum': 0}},
                            'required': ['ids']}
_OUTPUT_SCHEMA: dict[str,
                     Any] = {'type': 'object',
                             'properties': {'results': {'type': 'array',
                                                        'items': {'type': 'object',
                                                                  'properties': {'id': {'type': 'string'},
                                                                                 'text': {'type': 'string'}},
                                                                  'required': ['id']}}},
                             'required': ['results']}
_ANNOTATIONS = {'readOnlyHint': True, 'openWorldHint': False}

def _grep_lines(text: str, pattern: str, context: int) -> str:
    """Keep lines matching *pattern* plus *context* lines around each match ('grep -E' style)."""
    lines = text.splitlines()
    regex = re.compile(pattern)
    matched = [i for i, line in enumerate(lines) if regex.search(line)]
    if not matched:
        return ''
    keep = sorted({j for i in matched for j in range(max(0, i - context), min(len(lines), i + context + 1))})
    grouped: list[list[int]] = []
    for i in keep:
        if grouped and i == grouped[-1][-1] + 1:
            grouped[-1].append(i)
        else:
            grouped.append([i])
    return '\n--\n'.join(('\n'.join((lines[i] for i in group)) for group in grouped))

def web_fetch_exa_results(ids: list[str], pattern: str | None=None, context: int=1) -> list[dict[str, Any]]:
    """Resolve ids from a prior ``web_fetch_exa`` call to url and full text.

    Args:
        ids: Result ids returned by ``web_fetch_exa``.
        pattern: Optional extended regular expression to filter text lines
            (``grep -E`` semantics); non-adjacent matching blocks are
            separated by a ``--`` line, like ``grep``.
        context: Context lines kept before/after each match (default: 1).

    Returns:
        One entry per known id, with ``id`` and ``text``.
    """
    items = fetch_cache.get_many(ids)
    missing = [i for i in ids if i not in {item['id'] for item in items}]
    if missing:
        logger.warning('web_fetch_exa_results: unknown or expired id(s): %s', missing)
    results = []
    for item in items:
        text = item.get('text') or ''
        if pattern:
            text = _grep_lines(text, pattern, context)
        results.append(strip_empty({'id': item['id'], 'text': text}))
    return results

class WebFetchExaResultsTool(ToolDefinition):
    name = 'web_fetch_exa_results'
    title = 'Exa web fetch results'
    description = _DESCRIPTION
    input_schema = _INPUT_SCHEMA
    output_schema = _OUTPUT_SCHEMA
    annotations = _ANNOTATIONS

    def handle(self, ctx: ToolContext) -> ToolResult:
        args = ctx.arguments
        try:
            results = web_fetch_exa_results(
                ids=args['ids'],
                pattern=args.get('pattern'),
                context=args.get(
                    'context',
                    1))
        except re.error as exc:
            logger.warning('web_fetch_exa_results: invalid pattern %r: %s', args.get('pattern'), exc)
            return ToolResult(content=[text_content(f'Invalid pattern: {exc}')], is_error=True)
        except Exception as exc:
            logger.exception('web_fetch_exa_results failed')
            return ToolResult(content=[text_content(f'Error resolving web_fetch_exa results: {exc}')], is_error=True)
        return ToolResult(structured_content={'results': results}, auto_approve=False)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(WebFetchExaResultsTool())
    functions.register(web_fetch_exa_results)