"""``tool_usage`` – full signature, docstring and type sources for one function.
"""
import inspect
import typing
from dataclasses import dataclass, field
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ToolUsageError', 'ToolUsageInfo', 'describe_function', 'ToolUsageTool', 'register']
'#: Per-session state key: ids already fully described by tool_usage.'
_SEEN_STATE_KEY = 'tool_usage_seen'
'#: Project package prefix identifying a "self-declared" (non-stdlib) type.'
_PROJECT_PREFIX = 'xy.ai.mcpc'

class ToolUsageError(Exception):
    """Raised when usage information cannot be produced for a requested function."""

@dataclass(frozen=True)
class ToolUsageInfo:
    """Result of :func:`describe_function`."""
    signature: str
    docstring: str
    type_sources: list[str] = field(default_factory=list)

def _flatten_annotation(annotation: Any) -> list[Any]:
    """Unwrap generics (``list[X]``, ``X | None``, ``dict[K, V]``, ...) down to their leaf types."""
    origin = typing.get_origin(annotation)
    if origin is None:
        return [annotation]
    leaves: list[Any] = []
    for arg in typing.get_args(annotation):
        leaves.extend(_flatten_annotation(arg))
    return leaves

def _is_project_type(tp: Any) -> bool:
    return inspect.isclass(tp) and getattr(tp, '__module__', '').startswith(_PROJECT_PREFIX)

def _hints_of(obj: Any) -> dict[str, Any]:
    try:
        return typing.get_type_hints(obj)
    except Exception:
        return dict(getattr(obj, '__annotations__', {}))

def _collect_project_types(func: Any) -> list[type]:
    """Collect every project-local type referenced by *func*, recursively.

    Starts from the function's own parameter/return annotations, then walks
    into each found type's own annotations (dataclass fields, attributes,
    ...) so nested self-declared objects are included too.
    """
    found: dict[str, type] = {}
    stack: list[Any] = []
    for annotation in _hints_of(func).values():
        stack.extend(_flatten_annotation(annotation))
    while stack:
        candidate = stack.pop()
        if not _is_project_type(candidate):
            continue
        key = f'{candidate.__module__}.{candidate.__qualname__}'
        if key in found:
            continue
        found[key] = candidate
        for annotation in _hints_of(candidate).values():
            stack.extend(_flatten_annotation(annotation))
    return sorted(found.values(), key=lambda t: (t.__module__, t.__qualname__))

def describe_function(functions: FunctionRegistry, function_id: str) -> ToolUsageInfo:
    """Describe the function *function_id* registered in *functions* for type-safe use.

    Args:
        functions: Registry the function was registered into.
        function_id: Id under which the function was registered (see
            :meth:`~xy.ai.mcpc.tools.function_registry.FunctionRegistry.register`).

    Returns:
        ToolUsageInfo: Signature, full docstring, and the source of every
        project-local (non-stdlib) type referenced by the function, including
        types referenced only by a referenced type (nested).

    Raises:
        ToolUsageError: If no function is registered under *function_id*.
    """
    entry = functions.get(function_id)
    if entry is None:
        raise ToolUsageError(f'No function registered under id: {function_id}')
    try:
        signature = f'{entry.name}{inspect.signature(entry.func)}'
    except (TypeError, ValueError):
        signature = entry.name
    sources: list[str] = []
    for tp in _collect_project_types(entry.func):
        try:
            sources.append(inspect.getsource(tp))
        except (OSError, TypeError):
            continue
    return ToolUsageInfo(signature=signature, docstring=entry.doc, type_sources=sources)

class ToolUsageTool(ToolDefinition):
    name = 'tool_usage'
    title = 'Show function-based tool usage'
    description = 'Get usage and information for one function-based tool: its signature and the source.'
    input_schema = {
        'type': 'object',
        'properties': {
            'name': {
                'type': 'string',
                'description': 'Id/name of the function, as returned by tool_search.'}},
        'required': ['name']}
    output_schema = {
        'type': 'object', 'properties': {
            'signature': {
                'type': 'string'}, 'docstring': {
                    'type': 'string'}, 'type_sources': {
                        'type': 'array', 'items': {
                            'type': 'string'}}}}
    annotations = {'readOnlyHint': True, 'idempotentHint': False, 'openWorldHint': False}

    def __init__(self, functions: FunctionRegistry) -> None:
        self._functions = functions

    def handle(self, ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        name = args['name']
        seen: set[str] = ctx.session.state.setdefault(_SEEN_STATE_KEY, set())
        if name in seen:
            return ToolResult(
                content=[
                    text_content(
                        f"Usage for '{name}' was already returned earlier in this session; refer to that earlier result.")])
        try:
            info = describe_function(self._functions, name)
        except ToolUsageError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        seen.add(name)
        return ToolResult(
            structured_content={
                'signature': info.signature,
                'docstring': info.docstring,
                'type_sources': info.type_sources})

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ToolUsageTool(functions))