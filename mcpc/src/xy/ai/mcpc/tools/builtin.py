"""Built-in example tools registered onto a :class:`ToolRegistry`.

The set is deliberately small but exercises every feature of the skeleton:

* ``echo``          – trivial text round-trip.
* ``add``           – numeric args, ``structuredContent`` + ``outputSchema``.
* ``server_time``   – no arguments.
* ``session_set``   – writes to per-session state (statefulness).
* ``session_get``   – reads per-session state.
* ``session_info``  – reflects the current session context.
"""

from __future__ import annotations

from datetime import datetime, timezone

from ..registry import ToolContext, ToolRegistry, ToolResult, text_content


def register_builtin_tools(registry: ToolRegistry) -> None:
    @registry.tool(
        "echo",
        title="Echo",
        description="Return the supplied message unchanged.",
        input_schema={
            "type": "object",
            "properties": {"message": {"type": "string", "description": "Text to echo back."}},
            "required": ["message"],
        },
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def echo(ctx: ToolContext):
        message = ctx.arguments["message"]
        return ToolResult(
            content=[text_content(message)],
            structured_content={"message": message},
        )

    @registry.tool(
        "add",
        title="Add",
        description="Add two numbers and return the sum.",
        input_schema={
            "type": "object",
            "properties": {
                "a": {"type": "number", "description": "First addend."},
                "b": {"type": "number", "description": "Second addend."},
            },
            "required": ["a", "b"],
        },
        output_schema={
            "type": "object",
            "properties": {"sum": {"type": "number"}},
            "required": ["sum"],
        },
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def add(ctx: ToolContext):
        total = ctx.arguments["a"] + ctx.arguments["b"]
        return ToolResult(
            content=[text_content(f"{total}")],
            structured_content={"sum": total},
        )

    @registry.tool(
        "server_time",
        title="Server time",
        description="Return the current server time in ISO-8601 (UTC).",
        input_schema={"type": "object", "properties": {}},
        annotations={"readOnlyHint": True},
    )
    def server_time(ctx: ToolContext):
        now = datetime.now(timezone.utc).isoformat()
        return ToolResult(content=[text_content(now)], structured_content={"now": now})

    @registry.tool(
        "session_set",
        title="Set session value",
        description="Store a value under a key in the current session's state.",
        input_schema={
            "type": "object",
            "properties": {
                "key": {"type": "string"},
                "value": {"type": "string"},
            },
            "required": ["key", "value"],
        },
        annotations={"readOnlyHint": False, "idempotentHint": True},
    )
    def session_set(ctx: ToolContext):
        key = ctx.arguments["key"]
        ctx.session.state[key] = ctx.arguments["value"]
        return ToolResult(
            content=[text_content(f"Stored '{key}'.")],
            structured_content={"key": key, "stored": True},
        )

    @registry.tool(
        "session_get",
        title="Get session value",
        description="Read a value previously stored in the current session's state.",
        input_schema={
            "type": "object",
            "properties": {"key": {"type": "string"}},
            "required": ["key"],
        },
        annotations={"readOnlyHint": True},
    )
    def session_get(ctx: ToolContext):
        key = ctx.arguments["key"]
        if key not in ctx.session.state:
            return ToolResult(
                content=[text_content(f"No value stored for '{key}'.")],
                is_error=True,
            )
        value = ctx.session.state[key]
        return ToolResult(
            content=[text_content(str(value))],
            structured_content={"key": key, "value": value},
        )

    @registry.tool(
        "session_info",
        title="Session info",
        description="Return metadata about the current session.",
        input_schema={"type": "object", "properties": {}},
        annotations={"readOnlyHint": True},
    )
    def session_info(ctx: ToolContext):
        return ctx.session.summary()
