"""Central tool registry and tool result helpers.

Tools are registered once in a process-wide :class:`ToolRegistry`.  What a
given client actually sees is derived by reconciling the registry against the
per-session configuration (:attr:`Session.enabled_tools`).
"""
from __future__ import annotations
from dataclasses import dataclass, field
from typing import Any, Callable
from xy.ai.mcpc.server.json_codec import JsonCodec
from xy.ai.mcpc.server.session import Session
from xy.ai.mcpc.tools.tool_context import ToolContext
from abc import ABC, abstractmethod
'#: Default value for the Anthropic-specific ``anthropic/maxResultSizeChars``'
'#: meta annotation, applied generically to every tool result (see'
'#: :meth:`ToolResult.to_dict`). This tells Anthropic-compatible MCP clients'
'#: how many characters of the result they may render/keep before truncating.'
ANTHROPIC_MAX_RESULT_SIZE_CHARS = 500000

@dataclass(slots=True)
class ToolResult:
    """The result of a tool call (maps onto MCP ``CallToolResult``)."""
    content: list[dict[str, Any]] = field(default_factory=list)
    structured_content: dict[str, Any] | None = None
    is_error: bool = False
    auto_approve: bool = False
    control_hint: str | None = None
    'Optional hint attached by the controller on approval (``/allow <id> <hint>``).\n\n    Embedded as :data:`CONTROL_HINT_PROPERTY` *inside* ``structuredContent``\n    (see :meth:`to_dict`) rather than as a top-level ``CallToolResult`` field:\n    MCP clients only surface ``content``/``structuredContent``/``isError`` to\n    the model, so a sibling top-level key would silently be dropped before\n    ever reaching the agent.\n    '

    def to_dict(self) -> dict[str, Any]:
        result: dict[str, Any] = {}
        if self.content:
            result['content'] = list(self.content)
        structured = dict(self.structured_content) if self.structured_content else {}
        if self.control_hint:
            structured[CONTROL_HINT_PROPERTY] = self.control_hint
        if structured:
            result['structuredContent'] = structured
        if self.is_error:
            result['isError'] = True
        return result

def text_content(text: str) -> dict[str, Any]:
    """Build a ``TextContent`` block."""
    return {'type': 'text', 'text': text}
'#: A handler receives the invocation context and returns one of:'
'#: * a :class:`ToolResult`,'
'#: * a ``str`` (wrapped as a single text content block),'
'#: * a ``dict`` (treated as structured content, also rendered as JSON text).'
ToolHandler = Callable[[ToolContext], 'ToolResult | str | dict[str, Any]']

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
        spec: dict[str, Any] = {'name': self.name, 'description': self.description, 'inputSchema': self.input_schema}
        if self.title is not None:
            spec['title'] = self.title
        if self.output_schema is not None:
            spec['outputSchema'] = self.output_schema
        if self.meta is not None:
            spec['_meta'] = self.meta
        if self.annotations is not None:
            spec['annotations'] = self.annotations
        return spec

class ToolDefinition(ABC):
    """Base class for registering a tool as an object instead of via ``@registry.tool``.

    Subclasses declare the MCP metadata as class attributes and implement
    :meth:`handle`; an instance is itself a callable :data:`ToolHandler` and
    can be passed directly to :meth:`ToolRegistry.register`::

        class MyTool(ToolDefinition):
            name = "my_tool"
            description = "..."
            input_schema = {...}

            def handle(self, ctx: ToolContext) -> ToolResult:
                ...

        registry.register(MyTool())
    """
    name: str
    description: str
    input_schema: dict[str, Any]
    title: str | None = None
    output_schema: dict[str, Any] | None = None
    annotations: dict[str, Any] | None = None

    @abstractmethod
    def handle(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':
        ...

    def __call__(self, ctx: ToolContext) -> 'ToolResult | str | dict[str, Any]':
        return self.handle(ctx)

    def to_tool(self) -> Tool:
        return Tool(name=self.name, description=self.description, input_schema=self.input_schema, handler=self, title=self.title, output_schema=self.output_schema, annotations=self.annotations)

def normalize_result(value: 'ToolResult | str | dict[str, Any] | None') -> ToolResult:
    """Coerce whatever a handler returned into a :class:`ToolResult`."""
    if isinstance(value, ToolResult):
        return value
    if value is None:
        return ToolResult(content=[])
    if isinstance(value, str):
        return ToolResult(content=[text_content(value)])
    if isinstance(value, dict):
        '# A dict already shaped like a CallToolResult is passed through.'
        if 'content' in value and isinstance(value['content'], list):
            return ToolResult(content=value['content'], structured_content=value.get('structuredContent'), is_error=bool(value.get('isError', False)))
        '# Otherwise treat the dict as structured content.'
        rendered = JsonCodec.encode(value)
        return ToolResult(content=[text_content(rendered)], structured_content=value)
    raise TypeError(f'Unsupported tool return type: {type(value)!r}')
"#: Name of the mandatory reason property injected into every tool's input"
'#: schema (see :func:`_inject_property`).'
REASON_PROPERTY = 'reason'
"#: Name of the optional hint property injected into every tool's output"
"#: schema and, at call time, into the result's ``structuredContent`` (see"
'#: :meth:`ToolResult.to_dict`) — must live there, not top-level, since MCP'
'#: clients drop unknown top-level ``CallToolResult`` fields silently.'
CONTROL_HINT_PROPERTY = 'controlHint'

def _inject_property(schema: dict[str, Any], name: str, description: str, *, required: bool) -> dict[str, Any]:
    """Return *schema* with an additional property generically injected.

    Used both for the mandatory ``reason`` property on every tool's input
    schema and for the optional ``controlHint`` property on every tool's
    output schema — the same generic mechanism, applied at registration time
    so individual tool modules never need to declare either themselves.
    """
    schema = dict(schema)
    properties = dict(schema.get('properties', {}))
    properties[name] = {'type': 'string', 'description': description}
    schema['properties'] = properties
    if required:
        required_list = list(schema.get('required', []))
        if name not in required_list:
            required_list.append(name)
        schema['required'] = required_list
    return schema

def _with_mandatory_reason(schema: dict[str, Any]) -> dict[str, Any]:
    """Return *schema* with a mandatory, short ``reason`` property injected.

    Every tool call must carry an extremely short reason/goal for the call so
    the authorizing user can review it (e.g. via the human-in-the-loop
    control layer) before or while it executes.
    """
    return _inject_property(schema, REASON_PROPERTY, 'Precise, specific reason for this tool call (what exactly is being retrievedand why it is needed now), shown to the authorizing user.', required=True)

def _with_optional_control_hint(schema: dict[str, Any]) -> dict[str, Any]:
    """Return *schema* with the optional ``controlHint`` output property injected.

    Documents the field that may appear inside ``structuredContent`` when the
    authorizing user attached a hint to an ``/allow`` decision.
    """
    return _inject_property(schema, CONTROL_HINT_PROPERTY, 'Optional hint or question from the authorizing user', required=False)

class ToolRegistry:
    """Process-wide registry of available tools."""

    def __init__(self) -> None:
        self._tools: dict[str, Tool] = {}
        '#: Generic tool-set aliases: an alias name expands to a set of tool'
        '#: names. A session may enable an alias instead of listing every member.'
        self._aliases: dict[str, set[str]] = {}

    def register_alias(self, alias: str, members: 'Iterable[str]') -> None:
        """Define (or extend) a tool-set alias expanding to *members*.

        Generic mechanism: any group of tools can be activated together by
        enabling a single alias name in a session's tool configuration.
        """
        self._aliases.setdefault(alias, set()).update(members)

    def expand_aliases(self, names: 'Iterable[str]') -> set[str]:
        """Expand any alias names in *names* to their member tool names."""
        expanded: set[str] = set()
        for name in names:
            members = self._aliases.get(name)
            if members is None:
                expanded.add(name)
            else:
                expanded.update(members)
        return expanded

    def is_enabled(self, session: Session, name: str) -> bool:
        """Whether *name* is enabled for *session*, honouring tool-set aliases."""
        return name in self.expand_aliases(session.enabled_tools)

    def register(self, tool: 'Tool | ToolDefinition') -> Tool:
        if isinstance(tool, ToolDefinition):
            tool = tool.to_tool()
        if tool.name in self._tools:
            raise ValueError(f'Tool already registered: {tool.name}')
        tool.input_schema = _with_mandatory_reason(tool.input_schema)
        '# Applied unconditionally: ToolResult.to_dict() may attach controlHint'
        '# to *any* result regardless of whether the tool declared an'
        '# outputSchema, so the schema must always document it too.'
        base_output_schema = tool.output_schema or {'type': 'object', 'properties': {}}
        tool.output_schema = _with_optional_control_hint(base_output_schema)
        meta: dict[str, Any] = {'anthropic/maxResultSizeChars': ANTHROPIC_MAX_RESULT_SIZE_CHARS}
        tool.meta = meta
        self._tools[tool.name] = tool
        return tool

    def tool(self, name: str, *, description: str, input_schema: dict[str, Any], title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> Callable[[ToolHandler], ToolHandler]:
        """Decorator registering the decorated function as a tool handler."""

        def decorator(handler: ToolHandler) -> ToolHandler:
            self.register(Tool(name=name, description=description, input_schema=input_schema, handler=handler, title=title, output_schema=output_schema, annotations=annotations))
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
        enabled = self.expand_aliases(session.enabled_tools)
        tools = [t for t in self._tools.values() if t.name in enabled]
        tools.sort(key=lambda t: t.name)
        return tools