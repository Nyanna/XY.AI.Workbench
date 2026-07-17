"""Ask-user tool – lets an agent ask the human a clarifying question.

This tool exists to give agents a back-channel to the user so they can ask
questions that improve session efficiency instead of, e.g., exhaustively
searching whole directory hierarchies when the user might already know the
answer or can find it far more easily.

The permission system already allows the user to intercept tool calls and
their outputs, so this implementation is intentionally a simple dummy: it
always reports that the user did not answer, leaving it up to the agent to
proceed on its own (e.g. by falling back to exploration). The main value of
this module is exposing a well-defined API/MCP tool for the interaction, not
an actual answering mechanism.
"""

from __future__ import annotations

from typing import Any

from ...registry import ToolContext, ToolRegistry, ToolResult, text_content

#: Answer returned whenever the user has not (yet) responded.
_NOT_ANSWERED = "The user did not answer. Proceed on your own."


def register_ask_user_tool(registry: ToolRegistry) -> None:
    @registry.tool(
        "ask-user",
        title="Ask user",
        description=(
            "Ask the user a clarifying question, in the user's language, to "
            "improve session efficiency (e.g. instead of searching an entire "
            "file hierarchy when the user likely knows the answer already). "
        ),
        input_schema={
            "type": "object",
            "properties": {
                "question": {
                    "type": "string",
                    "description": "The question to ask the user, in the user's language.",
                },
            },
            "required": ["question"],
        },
        output_schema={
            "type": "object",
            "properties": {
                "answer": {"type": "string"},
            },
            "required": ["answer"],
        },
        annotations={"readOnlyHint": True, "openWorldHint": False},
    )
    def ask_user(ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        question: str = args["question"]
        if not question or not question.strip():
            return ToolResult(
                content=[text_content("``question`` must not be empty.")],
                is_error=True,
            )

        return ToolResult(structured_content={"answer": _NOT_ANSWERED})
