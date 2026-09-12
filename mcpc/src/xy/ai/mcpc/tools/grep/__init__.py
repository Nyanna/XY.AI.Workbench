"""Grep tool – recursive extended-regex search for retrieval, for a batch of items."""
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools._directories import normalize_directories
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, serialize_batch_result
import re
__all__ = [
    'GrepError',
    'GrepMatch',
    'GrepItem',
    'GrepResult',
    'GrepItemError',
    'GrepBatchResult',
    'grep',
    'GrepTool',
    'register_grep_tool']
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

@dataclass(frozen=True)
class GrepItem:
    """One independent grep search to run.

    Attributes:
        directory: Absolute paths of the directories to search recursively.
        pattern: Extended regular expression (grep -E syntax) to search for.
        exclude: Globs of file names to exclude from the search, if given.
        include: Globs of file names to include in the search, if given.
        limit: Maximum number of matches to return (1..``_MAX_LIMIT``). Applies per item, not per batch.
    """
    directory: list[str]
    pattern: str
    exclude: list[str] | None = None
    include: list[str] | None = None
    limit: int = _DEFAULT_LIMIT

@dataclass(frozen=True)
class GrepResult:
    """Result of a single grep search, mirroring its input for result association.

    Attributes:
        directory: The directory list exactly as given in the input.
        pattern: The pattern exactly as given in the input.
        matches: The matches found (empty if none).
        warning: Set if ``limit`` was reached and further matches may exist.
    """
    directory: list[str]
    pattern: str
    matches: list[GrepMatch]
    warning: str | None = None

@dataclass(frozen=True)
class GrepItemError:
    """Error running a single grep search, mirroring its input for result association."""
    directory: list[str]
    pattern: str
    error: str

@dataclass(frozen=True)
class GrepBatchResult:
    """Result of :func:`grep`.

    Attributes:
        results: One :class:`GrepResult` per successfully executed search.
        errors: One :class:`GrepItemError` per search that failed.
    """
    results: list[GrepResult]
    errors: list[GrepItemError]

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

def _grep_one(item: GrepItem) -> GrepResult:
    result = _run_grep(item.directory, item.pattern, exclude=item.exclude, include=item.include, limit=item.limit)
    if result.exit_code >= 2:
        raise GrepError(f'grep failed (exit code {result.exit_code}): {result.stderr}')
    matches = _parse_grep_stdout(result.stdout)
    warning = None
    if len(matches) >= item.limit:
        warning = f'Limit of {
            item.limit} matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit.'
    return GrepResult(directory=item.directory, pattern=item.pattern, matches=matches, warning=warning)

def grep(items: list[GrepItem]) -> GrepBatchResult:
    """Run one or more independent grep searches. Limits apply per item, not per batch.

    Args:
        items: Grep searches to run. Must be non-empty.

    Returns:
        GrepBatchResult: one result per successful search, one error per failed search.

    Raises:
        GrepError: If items is empty.
    """
    if not items:
        raise GrepError("'items' must be a non-empty list.")
    results: list[GrepResult] = []
    errors: list[GrepItemError] = []
    for item in items:
        try:
            results.append(_grep_one(item))
        except GrepError as exc:
            errors.append(GrepItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
    return GrepBatchResult(results=results, errors=errors)

class GrepTool(ToolDefinition):
    name = 'grep'
    title = 'Search files with grep'
    description = "Run one or more independent grep searches for lines matching an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters. Limits apply per item, not per batch."
    input_schema = {
        'type': 'object',
        'properties': {
            'items': {
                'type': 'array',
                'minItems': 1,
                'items': {
                    'type': 'object',
                    'additionalProperties': False,
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
                            'description': 'Maximum number of matching lines to return for this item.',
                            'default': _DEFAULT_LIMIT,
                            'minimum': 1,
                            'maximum': _MAX_LIMIT}},
                    'required': [
                        'directory',
                        'pattern']},
                'description': 'Independent grep searches to run.'}},
        'required': ['items']}
    output_schema = {
        'type': 'object', 'properties': {
            'results': {
                'type': 'array', 'items': {
                    'type': 'object', 'properties': {
                        'directory': {
                            'type': 'array', 'items': {
                                'type': 'string'}}, 'pattern': {
                                    'type': 'string'}, 'matches': {
                                        'type': 'array', 'items': {
                                            'type': 'object', 'properties': {
                                                'path': {
                                                    'type': 'string'}, 'lineno': {
                                                        'type': 'integer'}, 'match': {
                                                            'type': 'string'}}, 'required': [
                                                                'path', 'lineno', 'match']}}, 'warning': {
                                                                    'type': 'string'}}, 'required': [
                                                                        'directory', 'pattern', 'matches']}}, 'errors': {
                                                                            'type': 'array', 'items': {
                                                                                'type': 'object', 'properties': {
                                                                                    'directory': {
                                                                                        'type': 'array', 'items': {
                                                                                            'type': 'string'}}, 'pattern': {
                                                                                                'type': 'string'}, 'error': {
                                                                                                    'type': 'string'}}, 'required': [
                                                                                                        'directory', 'pattern', 'error']}}}}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`grep`, translating the MCP schema to/from the Python API."""

        def item_factory(it: dict[str, Any]) -> GrepItem:
            return GrepItem(
                directory=it['directory'],
                pattern=it['pattern'],
                exclude=it.get('exclude'),
                include=it.get('include'),
                limit=int(
                    it.get(
                        'limit',
                        _DEFAULT_LIMIT)))

        def result_serializer(r: GrepResult) -> dict[str, Any]:
            entry: dict[str,
                        Any] = {'directory': r.directory,
                                'pattern': r.pattern,
                                'matches': [{'path': f'{m.directory}/{m.filename}' if m.directory else m.filename,
                                             'lineno': m.lineno,
                                             'match': m.match} for m in r.matches]}
            if r.warning is not None:
                entry['warning'] = r.warning
            return entry
        args: dict[str, Any] = ctx.arguments
        raw_items = args.get('items') or []
        if not raw_items:
            return ToolResult(content=[text_content("'items' must be a non-empty list.")], is_error=True)
        items = [item_factory(it) for it in raw_items]
        batch = grep(items)
        error_serializer = lambda e: {'directory': e.directory, 'pattern': e.pattern, 'error': e.error}
        content = serialize_batch_result(batch, result_serializer, error_serializer)
        has_error = bool(batch.errors)
        return ToolResult(structured_content=content, auto_approve=not has_error)

def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(GrepTool())
    functions.register(grep)