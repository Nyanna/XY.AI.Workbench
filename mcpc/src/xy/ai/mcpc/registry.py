"""Central tool registry and tool result helpers.

Tools are registered once in a process-wide :class:`ToolRegistry`.  What a
given client actually sees is derived by reconciling the registry against the
per-session configuration (:attr:`Session.enabled_tools`).
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import TYPE_CHECKING, Any, Callable

from .codec import JsonCodec
from .session import Session

if TYPE_CHECKING:
    from .context import AppServices


@dataclass(slots=True)
class ToolContext:
    """Context handed to a tool handler on invocation."""

    session: Session
    arguments: dict[str, Any]
    #: Shared process-wide services (session store, CLI manager, profiles).
    #: ``None`` for tools that never orchestrate other sessions.
    services: "AppServices | None" = None


@dataclass(slots=True)
class ToolResult:
    """The result of a tool call (maps onto MCP ``CallToolResult``)."""

    content: list[dict[str, Any]] = field(default_factory=list)
    structured_content: dict[str, Any] | None = None
    is_error: bool = False

    def to_dict(self) -> dict[str, Any]:
        result: dict[str, Any] = {"content": list(self.content)}
        if self.structured_content is not None:
            result["structuredContent"] = self.structured_content
        if self.is_error:
            result["isError"] = True
        return result


def text_content(text: str) -> dict[str, Any]:
    """Build a ``TextContent`` block."""
    return {"type": "text", "text": text}


#: A handler receives the invocation context and returns one of:
#: * a :class:`ToolResult`,
#: * a ``str`` (wrapped as a single text content block),
#: * a ``dict`` (treated as structured content, also rendered as JSON text).
ToolHandler = Callable[[ToolContext], "ToolResult | str | dict[str, Any]"]


@dataclass(slots=True)
class Tool:
    """A registered tool and its MCP metadata."""

    name: str
    description: str
    input_schema: dict[str, Any]
    handler: ToolHandler
    title: str | None = None
    output_schema: dict[str, Any] | None = None
    annotations: dict[str, Any] | None = None

    def to_spec(self) -> dict[str, Any]:
        """Return the MCP ``Tool`` object advertised via ``tools/list``."""
        spec: dict[str, Any] = {
            "name": self.name,
            "description": self.description,
            "inputSchema": self.input_schema,
        }
        if self.title is not None:
            spec["title"] = self.title
        if self.output_schema is not None:
            spec["outputSchema"] = self.output_schema
        if self.annotations is not None:
            spec["annotations"] = self.annotations
        return spec


def normalize_result(value: "ToolResult | str | dict[str, Any] | None") -> ToolResult:
    """Coerce whatever a handler returned into a :class:`ToolResult`."""
    if isinstance(value, ToolResult):
        return value
    if value is None:
        return ToolResult(content=[])
    if isinstance(value, str):
        return ToolResult(content=[text_content(value)])
    if isinstance(value, dict):
        # A dict already shaped like a CallToolResult is passed through.
        if "content" in value and isinstance(value["content"], list):
            return ToolResult(
                content=value["content"],
                structured_content=value.get("structuredContent"),
                is_error=bool(value.get("isError", False)),
            )
        # Otherwise treat the dict as structured content.
        rendered = JsonCodec.encode(value)
        return ToolResult(content=[text_content(rendered)], structured_content=value)
    raise TypeError(f"Unsupported tool return type: {type(value)!r}")


class ToolRegistry:
    """Process-wide registry of available tools."""

    def __init__(self) -> None:
        self._tools: dict[str, Tool] = {}

    def register(self, tool: Tool) -> Tool:
        if tool.name in self._tools:
            raise ValueError(f"Tool already registered: {tool.name}")
        self._tools[tool.name] = tool
        return tool

    def tool(
        self,
        name: str,
        *,
        description: str,
        input_schema: dict[str, Any],
        title: str | None = None,
        output_schema: dict[str, Any] | None = None,
        annotations: dict[str, Any] | None = None,
    ) -> Callable[[ToolHandler], ToolHandler]:
        """Decorator registering the decorated function as a tool handler."""

        def decorator(handler: ToolHandler) -> ToolHandler:
            self.register(
                Tool(
                    name=name,
                    description=description,
                    input_schema=input_schema,
                    handler=handler,
                    title=title,
                    output_schema=output_schema,
                    annotations=annotations,
                )
            )
            return handler

        return decorator

    def get(self, name: str) -> Tool | None:
        return self._tools.get(name)

    def __contains__(self, name: object) -> bool:
        return name in self._tools

    def __len__(self) -> int:
        return len(self._tools)

    def names(self) -> list[str]:
        return list(self._tools)

    def list_for_session(self, session: Session) -> list[Tool]:
        """Reconcile the registry with the session's enabled-tool configuration.

        Returns the tools the session is allowed to see, sorted by name for a
        stable pagination order.
        """
        tools = [t for t in self._tools.values() if session.is_tool_enabled(t.name)]
        tools.sort(key=lambda t: t.name)
        return tools
