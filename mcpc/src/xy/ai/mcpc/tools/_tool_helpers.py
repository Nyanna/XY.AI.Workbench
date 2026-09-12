"""Common helpers for tool definition and request handling."""
from dataclasses import dataclass, asdict
from typing import Any, Callable, TypeVar
from xy.ai.mcpc.tools.tool_registry import ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
__all__ = ['BatchError', 'require_items', 'handle_batch_tool', 'serialize_batch_result']

@dataclass
class BatchError:
    """Error from a single batch item."""
    error: str
T = TypeVar('T')
R = TypeVar('R')

def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':
    """Return the non-empty list at *key* in ``ctx.arguments``, or an error result.

    Returns:
        ``(values, None)`` on success, or ``([], error_result)`` if *key* is missing or
        empty; callers must return ``error_result`` immediately in that case.
    """
    values = ctx.arguments.get(key) or []
    if not values:
        return ([], ToolResult(content=[text_content(f"'{key}' must be a non-empty list.")], is_error=True))
    return (values, None)

def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], Any], error_class: type, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> ToolResult:
    """Common handler for batch-processing tools.

    Args:
        ctx: Tool context with arguments.
        item_factory: Function to convert raw dict to typed item.
        batch_fn: Function to process list of items, returning batch result.
        error_class: Exception class raised by `batch_fn` for whole-batch failures.
        result_serializer: Optional custom serializer for result items (default: asdict).
        error_serializer: Optional custom serializer for error items (default: asdict).

    Returns:
        ToolResult with serialized batch result.
    """
    raw_items, error = require_items(ctx)
    if error is not None:
        return error
    items: list[T] = []
    for it in raw_items:
        try:
            items.append(item_factory(it))
        except (KeyError, ValueError, TypeError) as exc:
            return ToolResult(content=[text_content(f'Invalid item: {exc}')], is_error=True)
    try:
        batch_result = batch_fn(items)
    except error_class as exc:
        return ToolResult(content=[text_content(str(exc))], is_error=True)
    content = serialize_batch_result(
        batch_result,
        result_serializer=result_serializer,
        error_serializer=error_serializer)
    has_error = hasattr(batch_result, 'errors') and batch_result.errors
    return ToolResult(structured_content=content, auto_approve=not has_error)

def serialize_batch_result(batch_result: Any, result_serializer: Callable[[Any], dict[str, Any]] | None=None, error_serializer: Callable[[Any], dict[str, Any]] | None=None) -> dict[str, Any]:
    """Serialize a batch result, only including non-empty result/error lists.
    
    Args:
        batch_result: Object with optional `results` and `errors` attributes.
        result_serializer: Optional custom serializer for result items (default: asdict).
        error_serializer: Optional custom serializer for error items (default: asdict).
    
    Returns:
        Dict with only populated fields.
    """
    if result_serializer is None:
        result_serializer = lambda r: asdict(r) if hasattr(r, '__dataclass_fields__') else r
    if error_serializer is None:
        error_serializer = lambda e: asdict(e) if hasattr(e, '__dataclass_fields__') else e
    content: dict[str, Any] = {}
    if hasattr(batch_result, 'results') and batch_result.results:
        content['results'] = [result_serializer(r) for r in batch_result.results]
    if hasattr(batch_result, 'errors') and batch_result.errors:
        content['errors'] = [error_serializer(e) for e in batch_result.errors]
    return content