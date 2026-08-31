"""Python tool – executes a Python script directly from context (no file)."""
import sys
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['PythonError', 'python', 'PythonTool', 'register_python_tool']

class PythonError(Exception):
    """Raised when a Python script cannot be executed."""

def python(script: str) -> ProcessResult:
    """Feed ``script`` to a fresh Python interpreter on standard input.
    
    Args:
        script: Python script content to execute.
    
    Returns:
        ProcessResult with:
            exit_code: Exit code of Python process.
            stdout: Standard output (up to 3000 chars; see stdout_file if longer).
            stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
            stdout_file: Absolute path to temp file with full stdout if > 3000 chars.
            stderr_file: Absolute path to temp file with full stderr if > 3000 chars.
    
    Raises:
        PythonError: If Python binary cannot be launched.
    """
    try:
        return run_process([sys.executable, '-'], stdin=script)
    except LaunchError as exc:
        raise PythonError(f'Failed to launch Python: {exc}') from exc

class PythonTool(ToolDefinition):
    name = 'python'
    title = 'Run Python script'
    description = 'Execute a Python script passed directly as content, without writing a script file. The script is fed to the interpreter on standard input. Returns the exit code, standard output and, if present, standard error output.'
    input_schema = {'type': 'object', 'properties': {'script': {'type': 'string', 'description': 'Python script content to execute.'}}, 'required': ['script']}
    output_schema = {'type': 'object', 'properties': {'exit_code': {'type': 'integer'}, 'stdout': {'type': 'string'}, 'stderr': {'type': 'string'}}, 'required': ['exit_code', 'stdout']}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': True}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`python` and pack the result into the MCP output schema."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = python(args['script'])
        except PythonError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return pack_process_result(result)

def register_python_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(PythonTool())
    functions.register(python)