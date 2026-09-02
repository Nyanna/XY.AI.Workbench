"""``tool_search`` – keyword search over the :class:`FunctionRegistry`.
"""
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionEntry, FunctionRegistry
__all__ = ['search_functions', 'ToolSearchTool', 'register']
'#: Per-session state key: ids of functions already surfaced by tool_search.'
_SEEN_STATE_KEY = 'tool_search_seen'

def _first_doc_line(doc: str) -> str:
    for line in doc.splitlines():
        stripped = line.strip()
        if stripped:
            return stripped
    return ''

def search_functions(functions: FunctionRegistry, keywords: str, seen: set[str]) -> list[FunctionEntry]:
    """Find functions registered in *functions* matching *keywords*, excluding *seen* ids.

    Each function is matched against its name first, then (if the name did
    not match) its docstring. *seen* is mutated in place to include every
    id returned, so a repeated search never surfaces the same function twice
    for the caller that owns *seen*.

    Args:
        functions: Registry of candidate functions/methods to search.
        keywords: Space-separated, lower-cased or mixed-case English keywords.
        seen: Ids already returned to the caller in the past; updated with
            the ids returned by this call.

    Returns:
        Matching entries not previously in *seen*, sorted alphabetically by name.
    """
    words = [w.lower() for w in keywords.split() if w]
    matches: list[FunctionEntry] = []
    for entry in functions.all():
        if entry.id in seen:
            continue
        name_lower = entry.name.lower()
        doc_lower = entry.doc.lower()
        if any((w in name_lower for w in words)) or any((w in doc_lower for w in words)):
            matches.append(entry)
    matches.sort(key=lambda e: e.name)
    seen.update((e.id for e in matches))
    return matches

class ToolSearchTool(ToolDefinition):
    name = 'tool_search'
    title = 'Search function-based tools'
    description = 'Search function-based tools by space-separated English keywords, matched against each tool. Returns name + first docstring line, alphabetically sorted. Each function is only ever returned once per session.'
    input_schema = {
        'type': 'object',
        'properties': {
            'keywords': {
                'type': 'string',
                'description': 'Space-separated English keywords.'}},
        'required': ['keywords']}
    output_schema = {
        'type': 'object', 'properties': {
            'tools': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'name': {
                            'type': 'string'}, 'docstring': {
                                'type': 'string'}}, 'required': ['name']}}}, 'required': ['tools']}
    annotations = {'readOnlyHint': True, 'idempotentHint': False, 'openWorldHint': False}

    def __init__(self, functions: FunctionRegistry) -> None:
        self._functions = functions

    def handle(self, ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        seen: set[str] = ctx.session.state.setdefault(_SEEN_STATE_KEY, set())
        matches = search_functions(self._functions, args['keywords'], seen)
        tools = [{'name': e.name, 'docstring': _first_doc_line(e.doc)} for e in matches]
        return ToolResult(structured_content={'tools': tools})

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ToolSearchTool(functions))