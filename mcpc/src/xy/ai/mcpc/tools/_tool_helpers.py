"""Common helpers for tool definition and request handling."""
from typing import Any, Callable, TypeVar
from xy.ai.mcpc.tools.tool_registry import ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.results import BatchResult
__all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serialize_batch_result', 'batch_schema']
T = TypeVar('T')
ResultPayload = Callable[[Any], dict[str, Any]]

def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:
    """Build the standard ``{items: [...]}`` input schema shared by all batch tools.

    Neither the top level nor the items accept properties beyond the declared ones.

    Args:
        item_properties: The ``properties`` of a single item.
        required: The ``required`` keys of a single item.
        description: Description of the ``items`` array.
    """
    item_schema: dict[str, Any] = {'type': 'object', 'properties': item_properties,
                                   'required': required, 'additionalProperties': False}
    return {
        'type': 'object',
        'properties': {
            'items': {
                'type': 'array',
                'minItems': 1,
                'items': item_schema,
                'description': description}},
        'required': ['items'],
        'additionalProperties': False}

def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':
    """Return the non-empty list at *key* in ``ctx.arguments``, or an error result.

    Returns:
        ``(values, None)`` on success, or ``([], error_result)`` if *key* is missing or
        empty; callers must return ``error_result`` immediately in that case.
    """
    values = ctx.arguments.get(key) or []
    if not values:
        return ([], ToolResult.failure(f"'{key}' must be a non-empty list."))
    return (values, None)

def serialize_batch_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None) -> dict[str, Any]:
    """Serialize the populated ``results``/``errors`` lists of a batch result.

    Warnings are not part of this dict; they travel via :attr:`ToolResult.warnings`.
    """
    if not isinstance(batch_result, BatchResult):
        raise TypeError(f'Expected a BatchResult, got {type(batch_result).__name__}')
    return batch_result.to_dict(result_payload)

def batch_tool_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:
    """Wrap a batch result into a :class:`ToolResult` carrying its call-wide warnings."""
    return ToolResult.ok(
        batch_result,
        auto_approve=auto_approve,
        structured=serialize_batch_result(
            batch_result,
            result_payload))

def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], BatchResult[Any, Any]], error_class: type, result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:
    """Common handler for batch-processing tools.

    Args:
        ctx: Tool context with arguments.
        item_factory: Function to convert raw dict to typed item.
        batch_fn: Function to process list of items, returning batch result.
        error_class: Exception class raised by `batch_fn` for whole-batch failures.
        result_payload: Optional context-dependent payload for result items (default: ``payload()``).
        auto_approve: Approve the call automatically if no item failed.

    Returns:
        ToolResult with serialized batch result and its warnings.
    """
    raw_items, error = require_items(ctx)
    if error is not None:
        return error
    items: list[T] = []
    for it in raw_items:
        try:
            items.append(item_factory(it))
        except KeyError as exc:
            key = exc.args[0] if exc.args else exc
            return ToolResult.failure(f'Invalid item: missing required property {key!r}')
        except (ValueError, TypeError) as exc:
            return ToolResult.failure(f'Invalid item: {exc}')
    try:
        batch_result = batch_fn(items)
    except error_class as exc:
        return ToolResult.failure(str(exc))
    return batch_tool_result(batch_result, result_payload, auto_approve)