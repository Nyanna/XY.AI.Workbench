"""Custom tool definitions for an embedded MCPC server."""
from __future__ import annotations
from abc import ABC, abstractmethod
from collections.abc import Callable, Iterable
from typing import TYPE_CHECKING, Any
from xy.ai.mcpc.server.json_codec import JsonCodec
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
if TYPE_CHECKING:
    from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
__all__ = ['FunctionTool', 'ToolHandlerFn', 'ToolProvider']
ToolHandlerFn = Callable[['ToolContext'], 'ToolResult | str | dict[str, Any]']

class ToolProvider(ABC):
    """Subclass and implement :meth:`tools` to contribute custom tools to a server."""

    @abstractmethod
    def tools(self, environment: AppEnvironment) -> Iterable[ToolDefinition]:
        """Return the tools to register. *environment* gives access to config, sessions, etc."""

class FunctionTool(ToolDefinition):
    """A tool backed by a plain callable taking a ``ToolContext``."""

    def __init__(self, name: str, description: str, input_schema: dict[str, Any], handler: ToolHandlerFn, *, title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> None:
        self.name = name
        self.description = description
        self.input_schema = input_schema
        self.title = title
        self.output_schema = output_schema if output_schema is not None else {'type': 'object'}
        self.annotations = annotations
        self._handler = handler

    def handle(self, ctx: ToolContext) -> ToolResult:
        return _coerce(self._handler(ctx))

def _coerce(value: ToolResult | str | dict[str, Any] | None) -> ToolResult:
    """Adapt the convenience return types of embedded handlers to :class:`ToolResult`."""
    if isinstance(value, ToolResult):
        return value
    if value is None:
        return ToolResult()
    if isinstance(value, str):
        return ToolResult(content=[text_content(value)])
    if isinstance(value, dict):
        return ToolResult(content=[text_content(JsonCodec.encode(value))], structured_content=value)
    raise TypeError(f'Unsupported tool return type: {type(value)!r}')

def register_tool(tool: ToolDefinition | ToolProvider, registry: ToolRegistry, environment: AppEnvironment) -> None:
    if isinstance(tool, ToolProvider):
        for definition in tool.tools(environment):
            registry.register(definition)
    else:
        registry.register(tool)