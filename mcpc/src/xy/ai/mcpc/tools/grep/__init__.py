"""Grep tool – recursive extended-regex search for retrieval."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools._directories import normalize_directories
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
import re
__all__ = ['GrepError', 'GrepMatch', 'grep', 'GrepTool', 'register_grep_tool']
_DEFAULT_LIMIT = 15
_MAX_LIMIT = 50

class GrepError(Exception):
    """Raised when a grep search cannot be executed or its output cannot be parsed."""

@dataclass(frozen=True)
class GrepMatch:
    """A single grep match, parsed from a 'path:line:content' output line."""
    directory: str
    filename: str
    lineno: int
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
        lineno_str, sep, match = rest.partition(':')
        if not sep or not lineno_str.isdigit():
            raise GrepError(f'Cannot parse grep output line: {line!r}')
        directory, _, filename = path.rpartition('/')
        matches.append(GrepMatch(directory=directory, filename=filename, lineno=int(lineno_str), match=match))
    return matches

def _as_list(value: list[str] | None) -> list[str]:
    """Normalize an optional list into a list (empty if ``None``)."""
    return list(value) if value is not None else []

def _run_grep(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:
    """Recursively search one or more directories for ``pattern`` (extended regexp).

    Args:
        directory: Absolute paths of the directories to search (each must exist and
            be a directory).
        pattern: Extended regular expression (grep -E syntax).
        exclude: Globs of file names to exclude from the search, if given.
        include: Globs of file names to include in the search, if given.
        limit: Maximum number of matching lines to return (1..``_MAX_LIMIT``).

    Returns:
        ProcessResult with:
            exit_code: 0 if matches were found, 1 if none were found, >=2 on grep error.
            stdout: Matching lines as 'path:line:content', with ``path`` relative to
                whichever searched directory it was found under, truncated to at most
                ``limit`` lines.
            stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
            stdout_file: Absolute path to temp file with full stdout if to large.
            stderr_file: Absolute path to temp file with full stderr if to large.

    Raises:
        GrepError: If a directory is not absolute.
        GrepError: If a directory does not exist or is not a directory.
        GrepError: If no directory is given.
        GrepError: If pattern is empty.
        GrepError: If limit is not between 1 and ``_MAX_LIMIT``.
        GrepError: If grep binary cannot be launched.
    """
    directory_paths = normalize_directories([Path(d) for d in _as_list(directory)])
    if not directory_paths:
        raise GrepError('At least one directory is required.')
    for directory_path in directory_paths:
        if not directory_path.is_absolute():
            raise GrepError('directory must be an absolute path.')
        if not directory_path.is_dir():
            raise GrepError('Directory not found or not a directory.')
    if not pattern:
        raise GrepError('pattern must not be empty.')
    if not 1 <= limit <= _MAX_LIMIT:
        raise GrepError(f'limit must be between 1 and {_MAX_LIMIT}.')
    cmd = ['grep', '--recursive', '--line-number', '--extended-regexp', '--binary-files=without-match', '--color=never']
    for pattern_glob in _as_list(include):
        cmd.append(f'--include={pattern_glob}')
    for pattern_glob in _as_list(exclude):
        cmd.append(f'--exclude={pattern_glob}')
    cmd += ['--', pattern, *(str(p) for p in directory_paths)]
    try:
        result = run_process(cmd)
    except LaunchError as exc:
        raise GrepError(f'Failed to launch grep: {exc}') from exc
    prefixes = sorted((str(p).rstrip('/') + '/' for p in directory_paths), key=len, reverse=True)
    prefix_pattern = '|'.join((re.escape(p) for p in prefixes))
    stdout = re.sub(f'^(?:{prefix_pattern})', '', result.stdout, flags=re.MULTILINE)
    lines = stdout.splitlines()
    stdout = '\n'.join(lines[:limit])
    return ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr)

def grep(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> list[GrepMatch]:
    """Recursively search one or more directories for ``pattern`` (extended regexp).

    Args:
        directory: Absolute paths of the directories to search (each must exist and
            be a directory).
        pattern: Extended regular expression (grep -E syntax).
        exclude: Globs of file names to exclude from the search, if given.
        include: Globs of file names to include in the search, if given.
        limit: Maximum number of matches to return (1..``_MAX_LIMIT``).

    Returns:
        List of GrepMatch objects, each with the directory (relative to whichever
        searched directory it was found under), the filename and the match
        ('line:content'). Empty if no matches were found.

    Raises:
        GrepError: If a directory is not absolute.
        GrepError: If a directory does not exist or is not a directory.
        GrepError: If no directory is given.
        GrepError: If pattern is empty.
        GrepError: If limit is not between 1 and ``_MAX_LIMIT``.
        GrepError: If grep binary cannot be launched.
        GrepError: If grep exits with an error (exit code >= 2).
        GrepError: If the grep output cannot be parsed into directory, filename and match.
    """
    result = _run_grep(directory, pattern, exclude=exclude, include=include, limit=limit)
    if result.exit_code >= 2:
        raise GrepError(f'grep failed (exit code {result.exit_code}): {result.stderr}')
    return _parse_grep_stdout(result.stdout)

class GrepTool(ToolDefinition):
    name = 'grep'
    title = 'Search files with grep'
    description = f"Recursively search a directory for lines matching an extended regular expression. Always use the 'include' and 'exclude' filters."
    input_schema = {
        'type': 'object',
        'properties': {
            'directory': {
                'type': 'array',
                'items': {
                    'type': 'string'},
                'minItems': 1,
                'description': 'Absolute paths of the directories to search recursively. Always use the narrowest subtree(s) that are likely to contain the target files.'},
            'pattern': {
                'type': 'string',
                        'description': 'Extended regular expression to search for. Make the pattern as specific as possible to reduce noise.'},
            'exclude': {
                'type': 'array',
                'items': {
                    'type': 'string'},
                'description': "Globs of file names to exclude from the search, e.g. '*.min.js'. Always set this to exclude build artefacts, dependencies (e.g. 'node_modules/**'), and minified files."},
            'include': {
                'type': 'array',
                'items': {
                    'type': 'string'},
                'description': "Globs of file names to include in the search, e.g. '*.py'. Always set this to restrict the search to the relevant file types; omit only when the file type is unknown."},
            'limit': {
                'type': 'integer',
                'description': f'Maximum number of matching lines to return.',
                'default': _DEFAULT_LIMIT,
                'minimum': 1,
                'maximum': _MAX_LIMIT}},
        'required': [
            'directory',
            'pattern']}
    output_schema = {
        'type': 'object', 'properties': {
            'matches': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'path': {
                            'type': 'string'}, 'lineno': {
                                'type': 'integer'}, 'match': {
                                    'type': 'string'}}, 'required': [
                                        'path', 'lineno', 'match']}}, 'warning': {
                                            'type': 'string'}}, 'required': ['matches']}
    annotations = {'readOnlyHint': True, 'idempotentHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`grep`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        limit = int(args.get('limit', _DEFAULT_LIMIT))
        try:
            matches = grep(
                args['directory'],
                args['pattern'],
                exclude=args.get('exclude'),
                include=args.get('include'),
                limit=limit)
        except GrepError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        structured_content: dict[str,
                                 Any] = {'matches': [{'path': f'{match.directory}/{match.filename}' if match.directory else match.filename,
                                                      'lineno': match.lineno,
                                                      'match': match.match} for match in matches]}
        content = []
        if len(matches) >= limit:
            warning = f'Limit of {limit} matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit.'
            structured_content['warning'] = warning
            content.append(text_content(warning))
        return ToolResult(content=content, structured_content=structured_content)

def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(GrepTool())
    functions.register(grep)