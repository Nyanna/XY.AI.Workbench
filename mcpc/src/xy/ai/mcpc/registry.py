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


#: Default value for the Anthropic-specific ``anthropic/maxResultSizeChars``
#: meta annotation, applied generically to every tool result (see
#: :meth:`ToolResult.to_dict`). This tells Anthropic-compatible MCP clients
#: how many characters of the result they may render/keep before truncating.
ANTHROPIC_MAX_RESULT_SIZE_CHARS = 500_000


@dataclass(slots=True)
class ToolResult:
    """The result of a tool call (maps onto MCP ``CallToolResult``)."""

    content: list[dict[str, Any]] = field(default_factory=list)
    structured_content: dict[str, Any] | None = None
    is_error: bool = False
    auto_approve: bool = False
    control_hint: str | None = None
    """Optional hint attached by the controller on approval (``/allow <id> <hint>``).

    Rendered as an independent top-level field (see :data:`CONTROL_HINT_PROPERTY`)
    that never touches ``content``/``structuredContent``/``isError`` — the actual
    tool result is left untouched, the hint merely rides along for the agent.
    """

    def to_dict(self) -> dict[str, Any]:
        result: dict[str, Any] = {}
        if self.content:
            result["content"] = list(self.content)
        if self.structured_content:
            result["structuredContent"] = self.structured_content
        if self.is_error:
            result["isError"] = True
        if self.control_hint:
            result[CONTROL_HINT_PROPERTY] = self.control_hint
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
    meta: dict[str, Any] | None = None

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
        if self.meta is not None:
            spec["_meta"] = self.meta
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


#: Name of the mandatory reason property injected into every tool's input
#: schema (see :func:`_inject_property`).
REASON_PROPERTY = "reason"

#: Name of the independent hint property injected into every tool's output
#: schema (see :func:`_inject_property`). Populated at call time from an
#: ``/allow <id> <hint>`` control decision (see ``ControlDecision.approval_hint``
#: in ``control/manager.py``); never required and never part of the actual
#: ``content``/``structuredContent`` payload, so it cannot interfere with it.
CONTROL_HINT_PROPERTY = "controlHint"


def _inject_property(
    schema: dict[str, Any],
    name: str,
    description: str,
    *,
    required: bool,
) -> dict[str, Any]:
    """Return *schema* with an additional property generically injected.

    Used both for the mandatory ``reason`` property on every tool's input
    schema and for the optional ``controlHint`` property on every tool's
    output schema — the same generic mechanism, applied at registration time
    so individual tool modules never need to declare either themselves.
    """
    schema = dict(schema)
    properties = dict(schema.get("properties", {}))
    properties[name] = {"type": "string", "description": description}
    schema["properties"] = properties
    if required:
        required_list = list(schema.get("required", []))
        if name not in required_list:
            required_list.append(name)
        schema["required"] = required_list
    return schema


def _with_mandatory_reason(schema: dict[str, Any]) -> dict[str, Any]:
    """Return *schema* with a mandatory, short ``reason`` property injected.

    Every tool call must carry an extremely short reason/goal for the call so
    the authorizing user can review it (e.g. via the human-in-the-loop
    control layer) before or while it executes.
    """
    return _inject_property(
        schema,
        REASON_PROPERTY,
        (
            "Precise, specific reason for this tool call (what exactly is being retrieved"
            "and why it is needed now), shown to the authorizing user."
        ),
        required=True,
    )


def _with_optional_control_hint(schema: dict[str, Any]) -> dict[str, Any]:
    """Return *schema* with the optional ``controlHint`` output property injected.

    Documents the independent, optional field that may accompany a tool
    result when the authorizing user attached a hint to an ``/allow``
    decision. Does not affect ``content``/``structuredContent``.
    """
    return _inject_property(
        schema,
        CONTROL_HINT_PROPERTY,
        (
            "Optional hint or question from the authorizing user"
        ),
        required=False,
    )


class ToolRegistry:
    """Process-wide registry of available tools."""

    def __init__(self) -> None:
        self._tools: dict[str, Tool] = {}

    def register(self, tool: Tool) -> Tool:
        if tool.name in self._tools:
            raise ValueError(f"Tool already registered: {tool.name}")
        tool.input_schema = _with_mandatory_reason(tool.input_schema)
        if tool.output_schema is not None:
            tool.output_schema = _with_optional_control_hint(tool.output_schema)
        
        meta: dict[str, Any] = {
            "anthropic/maxResultSizeChars": ANTHROPIC_MAX_RESULT_SIZE_CHARS
        }
        tool.meta = meta
        
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
