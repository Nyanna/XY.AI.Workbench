"""Grep tool – recursive extended-regex search for retrieval."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
import re
__all__ = ['GrepError', 'GrepMatch', 'grep', 'GrepTool', 'register_grep_tool']
_MAX_STREAM_CHARS = 10000

class GrepError(Exception):
    """Raised when a grep search cannot be executed or its output cannot be parsed."""

@dataclass(frozen=True)
class GrepMatch:
    """A single grep match, parsed from a 'path:line:content' output line."""
    directory: str
    filename: str
    match: str

def _parse_grep_stdout(stdout: str) -> list[GrepMatch]:
    """Parse grep's 'path:line:content' stdout into :class:`GrepMatch` objects."""
    matches: list[GrepMatch] = []
    for line in stdout.splitlines():
        if not line:
            continue
        path, sep, rest = line.partition(':')
        if not sep:
            raise GrepError(f'Cannot parse grep output line: {line!r}')
        directory, _, filename = path.rpartition('/')
        matches.append(GrepMatch(directory=directory, filename=filename, match=rest))
    return matches

def _run_grep(directory: str, pattern: str, *, exclude: str | None=None, include: str | None=None) -> ProcessResult:
    """Recursively search ``directory`` for ``pattern`` (extended regexp).

    Args:
        directory: Absolute path to the directory to search (must exist and be a directory).
        pattern: Extended regular expression (grep -E syntax).
        exclude: Glob of file names to exclude from the search, if given.
        include: Glob of file names to include in the search, if given.

    Returns:
        ProcessResult with:
            exit_code: 0 if matches were found, 1 if none were found, >=2 on grep error.
            stdout: Matching lines as 'path:line:content', with ``path`` relative to
                ``directory``.
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
        result = run_process(cmd)
    except LaunchError as exc:
        raise GrepError(f'Failed to launch grep: {exc}') from exc
    prefix = str(directory_path).rstrip('/') + '/'
    stdout = re.sub(f'^{re.escape(prefix)}', '', result.stdout, flags=re.MULTILINE)
    return ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr)

def grep(directory: str, pattern: str, *, exclude: str | None=None, include: str | None=None) -> list[GrepMatch]:
    """Recursively search ``directory`` for ``pattern`` (extended regexp).

    Args:
        directory: Absolute path to the directory to search (must exist and be a directory).
        pattern: Extended regular expression (grep -E syntax).
        exclude: Glob of file names to exclude from the search, if given.
        include: Glob of file names to include in the search, if given.

    Returns:
        List of GrepMatch objects, each with the directory (relative to ``directory``),
        the filename and the match ('line:content'). Empty if no matches were found.

    Raises:
        GrepError: If directory is not absolute.
        GrepError: If directory does not exist or is not a directory.
        GrepError: If pattern is empty.
        GrepError: If grep binary cannot be launched.
        GrepError: If grep exits with an error (exit code >= 2).
        GrepError: If the grep output cannot be parsed into directory, filename and match.
    """
    result = _run_grep(directory, pattern, exclude=exclude, include=include)
    if result.exit_code >= 2:
        raise GrepError(f'grep failed (exit code {result.exit_code}): {result.stderr}')
    return _parse_grep_stdout(result.stdout)

class GrepTool(ToolDefinition):
    name = 'grep'
    title = 'Search files with grep'
    description = f"Recursively search a directory for lines matching an extended regular expression (grep -E). Returns matches as 'path:line:content' (path relative to the searched directory), the exit code (0 = matches found, 1 = none found) and, if present, standard error output.\n\nUSAGE GUIDANCE: Always use the 'include' and 'exclude' filters unless (a) the directory tree is already known to be small and relevant, or (b) the search pattern is expected to match very rarely across all file types. Unfiltered searches on large or unknown directory trees frequently exceed output limits and waste resources — filter first, then broaden only if needed."
    input_schema = {
        'type': 'object',
        'properties': {
            'directory': {
                'type': 'string',
                'description': 'Absolute path to the directory to search recursively. Prefer the narrowest subtree that is likely to contain the target files.'},
            'pattern': {
                'type': 'string',
                'description': 'Extended regular expression (grep -E syntax) to search for. Make the pattern as specific as possible to reduce noise.'},
            'exclude': {
                'type': 'string',
                        'description': "Glob of file names to exclude from the search, e.g. '*.min.js'. RECOMMENDED: always set this to exclude build artefacts, dependencies (e.g. 'node_modules/**'), and minified files unless you have a specific reason not to."},
            'include': {
                'type': 'string',
                'description': "Glob of file names to include in the search, e.g. '*.py'. RECOMMENDED: always set this to restrict the search to the relevant file types; omit only when the file type is unknown or intentionally broad."}},
        'required': [
            'directory',
            'pattern']}
    output_schema = {
        'type': 'object', 'properties': {
            'exit_code': {
                'type': 'integer'}, 'stdout': {
                    'type': 'string'}, 'stderr': {
                        'type': 'string'}, 'stdout_file': {
                            'type': 'string', 'description': 'Absolute path to a file containing the full STDOUT.'}, 'stderr_file': {
                                'type': 'string', 'description': 'Absolute path to a file containing the full STDERR.'}}, 'required': ['stdout']}
    annotations = {'readOnlyHint': True, 'idempotentHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`_run_grep` and pack the result into the MCP output schema."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = _run_grep(
                args['directory'],
                args['pattern'],
                exclude=args.get('exclude'),
                include=args.get('include'))
        except GrepError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return pack_process_result(
            result,
            normalize_output=True,
            omit_zero_exit_code=True,
            max_stream_chars=_MAX_STREAM_CHARS)

def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(GrepTool())
    functions.register(grep)