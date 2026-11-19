"""``ast_retrieve`` tool: all reading AST tools (plus search) in a single call.

Combines ``search``, ``ast_outline``, ``ast_find`` and ``ast_read``. Search is
special: with a location intent it yields AST node ids, which can be consumed
by ``read`` in the same call only if known upfront; otherwise use the ids from
the response in a follow-up call.
"""
from __future__ import annotations
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.ast.find import FindNodesTool
from xy.ai.mcpc.tools.ast.outline import OutlineNodesTool
from xy.ai.mcpc.tools.ast.read import ReadNodeTool
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools.results import ItemError, RawPayload
from xy.ai.mcpc.tools.search import SearchTool
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolRegistry, ToolResult
__all__ = ['RetrieveTool', 'register']

@dataclass(frozen=True, kw_only=True)
class _SectionError(ItemError):

    def key(self) -> dict[str, Any]:
        return {}

def _error_text(result: ToolResult) -> str:
    return ' '.join((str(c.get('text', '')) for c in result.content)) or 'Unknown error'

def _merge(parts: list[dict[str, Any]]) -> dict[str, Any]:
    """Merge several payloads of the same tool: list values are concatenated."""
    if len(parts) == 1:
        return parts[0]
    merged: dict[str, Any] = {}
    for part in parts:
        for key, value in part.items():
            if isinstance(value, list):
                merged.setdefault(key, []).extend(value)
            else:
                merged.setdefault(key, value)
    return merged

class RetrieveTool(ToolDefinition):
    name = 'ast_retrieve'
    features = frozenset({ToolFeature.INPUT_ASTID})
    title = 'Batch of retrieval operations (search, outline, find, read)'
    description = 'Run search, outline, find and read operations in one call. search: items of the tool search. outline: absolute file paths, as in ast_outline. find: list of queries, each in the format of ast_find. read: items of the tool ast_read.'
    input_schema = {
        'type': 'object',
        'properties': {
            'search': SearchTool.input_schema['properties']['items'],
            'outline': OutlineNodesTool.input_schema['properties']['paths'],
            'find': {
                'type': 'array',
                'items': FindNodesTool.input_schema,
                'description': 'Independent ast_find queries.'},
            'read': ReadNodeTool.input_schema['properties']['items']},
        'additionalProperties': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        sections: dict[str, Any] = {}
        warnings: list[str] = []
        approve = True
        '# (key, tool, list of argument dicts for that tool)'
        calls: list[tuple[str, ToolDefinition, list[dict[str, Any]]]] = []
        if ctx.arguments.get('search'):
            calls.append(('search', SearchTool(ctx.registry), [{'items': ctx.arguments['search']}]))
        if ctx.arguments.get('outline'):
            calls.append(('outline', OutlineNodesTool(), [{'paths': ctx.arguments['outline']}]))
        if ctx.arguments.get('find'):
            calls.append(('find', FindNodesTool(), list(ctx.arguments['find'])))
        if ctx.arguments.get('read'):
            calls.append(('read', ReadNodeTool(), [{'items': ctx.arguments['read']}]))
        for key, tool, arg_list in calls:
            parts: list[dict[str, Any]] = []
            for arguments in arg_list:
                sub = tool.handle(ToolContext(session=ctx.session, arguments=arguments, registry=ctx.registry))
                if sub.is_error:
                    parts = []
                    sections[key] = _SectionError(error=_error_text(sub)).to_dict()
                    approve = False
                    break
                parts.append(sub.structured_content or {})
                warnings.extend(sub.warnings)
                approve = approve and sub.auto_approve
            if parts:
                sections[key] = _merge(parts)
        if not sections:
            return ToolResult.failure('At least one non-empty retrieval list is required.')
        payload = RawPayload(
            sections,
            reason='composes the payloads of the individual retrieval tools',
            warnings=tuple(warnings))
        return ToolResult.ok(payload, auto_approve=approve)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(RetrieveTool())