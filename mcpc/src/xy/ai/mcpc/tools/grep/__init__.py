"""Grep tool – recursive extended-regex search for retrieval."""
from __future__ import annotations
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
__all__ = ['GrepError', 'grep', 'GrepTool', 'register_grep_tool']
_MAX_STREAM_CHARS = 4000

class GrepError(Exception):
    """Raised when a grep search cannot be executed."""

def grep(directory: str, pattern: str, *, exclude: str | None = None, include: str | None = None) -> ProcessResult:
    """Recursively search ``directory`` for ``pattern`` (extended regexp).

    Args:
        directory: Absolute path to the directory to search (must exist and be a directory).
        pattern: Extended regular expression (grep -E syntax).
        exclude: Glob of file names to exclude from the search, if given.
        include: Glob of file names to include in the search, if given.

    Returns:
        ProcessResult with:
            exit_code: 0 if matches were found, 1 if none were found, >=2 on grep error.
            stdout: Matching lines as 'path:line:content' (up to 3000 chars; see stdout_file if longer).
            stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
            stdout_file: Absolute path to temp file with full stdout if to large.
            stderr_file: Absolute path to temp file with full stderr if to large.

    Raises:
        GrepError: If directory is not absolute.
        GrepError: If directory does not exist or is not a directory.
        GrepError: If pattern is empty.
        GrepError: If grep binary cannot be launched.
    """
    directory_path = Path(directory)
    if not directory_path.is_absolute():
        raise GrepError('directory must be an absolute path.')
    if not directory_path.is_dir():
        raise GrepError('Directory not found or not a directory.')
    if not pattern:
        raise GrepError('pattern must not be empty.')

    cmd = ['grep', '--recursive', '--line-number', '--extended-regexp', '--binary-files=without-match', '--color=never']
    if include:
        cmd.append(f'--include={include}')
    if exclude:
        cmd.append(f'--exclude={exclude}')
    cmd += ['--', pattern, str(directory_path)]

    try:
        return run_process(cmd)
    except LaunchError as exc:
        raise GrepError(f'Failed to launch grep: {exc}') from exc

class GrepTool(ToolDefinition):
    name = 'grep'
    title = 'Search files with grep'
    description = f"Recursively search a directory for lines matching an extended regular expression (grep -E). Returns matches as 'path:line:content', the exit code (0 = matches found, 1 = none found) and, if present, standard error output."
    input_schema = {
        'type': 'object',
        'properties': {
            'directory': {'type': 'string', 'description': 'Absolute path to the directory to search recursively.'},
            'pattern': {'type': 'string', 'description': 'Extended regular expression (grep -E syntax) to search for.'},
            'exclude': {'type': 'string', 'description': "Glob of file names to exclude from the search, e.g. '*.min.js'."},
            'include': {'type': 'string', 'description': "Glob of file names to include in the search, e.g. '*.py'."},
        },
        'required': ['directory', 'pattern'],
    }
    output_schema = {
        'type': 'object',
        'properties': {
            'exit_code': {'type': 'integer'},
            'stdout': {'type': 'string'},
            'stderr': {'type': 'string'},
            'stdout_file': {'type': 'string', 'description': 'Absolute path to a file containing the full STDOUT.'},
            'stderr_file': {'type': 'string', 'description': 'Absolute path to a file containing the full STDERR.'},
        },
        'required': ['stdout'],
    }
    annotations = {'readOnlyHint': True, 'idempotentHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`grep` and pack the result into the MCP output schema."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = grep(args['directory'], args['pattern'], exclude=args.get('exclude'), include=args.get('include'))
        except GrepError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return pack_process_result(result, normalize_output=True, omit_zero_exit_code=True, max_stream_chars=_MAX_STREAM_CHARS)

def register_grep_tool(registry: ToolRegistry) -> None:
    registry.register(GrepTool())
