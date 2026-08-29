"""Ask-user tool – lets an agent ask the human a clarifying question.

This tool exists to give agents a back-channel to the user so they can ask
questions that improve session efficiency instead of, e.g., exhaustively
searching whole directory hierarchies when the user might already know the
answer or can find it far more easily.
"""
from dataclasses import dataclass
from typing import Any
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
__all__ = ['AskUserError', 'AskUserResult', 'ask_user', 'AskUserTool', 'register_ask_user_tool', "TOOLNAME_ASK_USER"]
_NOT_ANSWERED = 'The user did not answer. Proceed on your own.'
TOOLNAME_ASK_USER = "ask_user"

class AskUserError(Exception):
    """Raised when a question cannot be asked."""

@dataclass(frozen=True)
class AskUserResult:
    answer: str

def ask_user(question: str) -> str:
    """Ask the user ``question``; always returns the "not answered" placeholder.
    
    Args:
        question: Question to display to the user.
    
    Returns:
        Always returns the literal string "[User did not answer]". The actual user
        response (if any) is not captured or returned by this function.
    
    Raises:
        AskUserError: If the question cannot be asked (no user interaction possible).
    
    Note:
        This is a placeholder implementation. The actual user interaction is handled
        at the MCP transport level. This function always returns a fixed string
        indicating the question was not answered.
    """
    return '[User did not answer]'

class AskUserTool(ToolDefinition):
    name = TOOLNAME_ASK_USER
    title = 'Ask user'
    description = "Ask the user a clarifying question, in the user's language, to improve session efficiency (e.g. instead of searching an entire file hierarchy when the user likely knows the answer already). "
    input_schema = {'type': 'object', 'properties': {'question': {'type': 'string', 'description': "The question to ask the user, in the user's language."}}, 'required': ['question']}
    output_schema = {'type': 'object', 'properties': {'answer': {'type': 'string'}}, 'required': ['answer']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`ask_user`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = ask_user(args['question'])
        except AskUserError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'answer': result})

def register_ask_user_tool(registry: ToolRegistry) -> None:
    registry.register(AskUserTool())