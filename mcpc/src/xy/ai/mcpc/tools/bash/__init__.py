"""Bash tool – executes a shell script inside a specified working directory."""
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['BashError', 'bash', 'BashTool', 'register_bash_tool']
_MAX_STREAM_CHARS = 3000

class BashError(Exception):
    """Raised when a Bash script cannot be executed."""

def bash(cwd: str, script: str) -> ProcessResult:
    """Run ``script`` with ``bash -c`` inside the absolute directory ``cwd``.
    
    Args:
        cwd: Absolute path to working directory (must exist and be a directory).
        script: Bash script content to execute.
    
    Returns:
        ProcessResult with:
            exit_code: Exit code of bash process.
            stdout: Standard output (up to 3000 chars; see stdout_file if longer).
            stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
            stdout_file: Absolute path to temp file with full stdout if > 3000 chars.
            stderr_file: Absolute path to temp file with full stderr if > 3000 chars.
    
    Raises:
        BashError: If cwd is not absolute.
        BashError: If cwd does not exist or is not a directory.
        BashError: If bash binary cannot be launched.
    """
    cwd_path = Path(cwd)
    if not cwd_path.is_absolute():
        raise BashError('cwd must be an absolute path.')
    if not cwd_path.is_dir():
        raise BashError('Working directory not found or not a directory.')
    try:
        return run_process(['bash', '-c', script], cwd=cwd_path)
    except LaunchError as exc:
        raise BashError(f'Failed to launch bash: {exc}') from exc

class BashTool(ToolDefinition):
    name = 'bash'
    title = 'Run Bash script'
    description = f"Execute a Bash script in the specified working directory. Returns the exit code, standard output and, if present, standard error output. As a safety limit, STDOUT/STDERR longer than {_MAX_STREAM_CHARS} characters are written to a temp file."
    input_schema = {
        'type': 'object',
        'properties': {
            'cwd': {
                'type': 'string',
                'description': 'Absolute path to the working directory.'},
            'script': {
                'type': 'string',
                'description': 'Bash script content.'}},
        'required': [
            'cwd',
            'script']}
    output_schema = {
        'type': 'object',
        'properties': {
            'exit_code': {
                'type': 'integer'},
            'stdout': {
                'type': 'string'},
            'stderr': {
                'type': 'string'},
            'stdout_file': {
                'type': 'string',
                'description': 'Absolute path to a file containing the full STDOUT, if STDOUT exceeded the safety limit.'},
            'stderr_file': {
                'type': 'string',
                'description': 'Absolute path to a file containing the full STDERR, if STDERR exceeded the safety limit.'}},
        'required': ['stdout']}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': True}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`bash` and pack the result into the MCP output schema."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = bash(cwd=args['cwd'], script=args['script'])
        except BashError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return pack_process_result(
            result,
            normalize_output=True,
            omit_zero_exit_code=True,
            max_stream_chars=_MAX_STREAM_CHARS)

def register_bash_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(BashTool())
    functions.register(bash)