"""Search tool – recursive extended-regex search for retrieval, for a batch of items."""
from dataclasses import dataclass, replace
from types import SimpleNamespace
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools._directories import normalize_directories
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
import re
from xy.ai.mcpc.tools.ast import core
__all__ = [
    'SearchError',
    'SearchLineMatch',
    'SearchNodeMatch',
    'SearchMatch',
    'SearchItem',
    'SearchResult',
    'SearchItemError',
    'SearchBatchResult',
    'search',
    'SearchTool',
    'register_search_tool']
_DEFAULT_LIMIT = 15
_INTENTS = ('locate', 'line_numbers', 'ast_node_ids')
_DEFAULT_INTENT = 'locate'
_INTENT_DESCRIPTION = "What you will do with the results. 'locate': only need to know which files/matches exist. 'line_numbers': you will address by line numbers. 'ast_node_ids'(recommended): you will continue with ast_* tools."
_MAX_LIMIT = 50

class SearchError(Exception):
    """Raised when a search cannot be executed or its output cannot be parsed."""

@dataclass(frozen=True)
class SearchLineMatch:
    """A single matching line within a file."""
    lineno: int
    match: str

@dataclass(frozen=True)
class SearchNodeMatch:
    """Matching lines attributed to the most specific enclosing AST node."""
    node_id: str
    matches: list[SearchLineMatch]

@dataclass(frozen=True)
class SearchMatch:
    """All matches found within a single file.

    ``nodes`` is set instead of relying on ``matches`` only if the file could be
    resolved to AST nodes; lines outside any node are kept under ``matches``.
    """
    directory: str
    filename: str
    matches: list[SearchLineMatch]
    nodes: list[SearchNodeMatch] | None = None

@dataclass(frozen=True)
class SearchItem:
    """One independent search to run.

    Attributes:
        directory: Absolute paths of the directories to search recursively.
        pattern: Extended regular expression (grep -E syntax) to search for.
        exclude: Globs of file names to exclude from the search, if given.
        include: Globs of file names to include in the search, if given.
        limit: Maximum number of matches to return (1..``_MAX_LIMIT``). Applies per item, not per batch.
        intent: Overrides the batch-level intent for this item, if given.
    """
    directory: list[str]
    pattern: str
    exclude: list[str] | None = None
    include: list[str] | None = None
    limit: int = _DEFAULT_LIMIT
    intent: str | None = None

@dataclass(frozen=True)
class SearchResult:
    """Result of a single search.

    Attributes:
        directory: The directory list exactly as given in the input.
        matches: The matches found, grouped by file (empty if none).
        warning: Set if ``limit`` was reached and further matches may exist.
    """
    directory: list[str]
    matches: list[SearchMatch]
    warning: str | None = None

@dataclass(frozen=True)
class SearchItemError:
    """Error running a single search, mirroring its input for result association."""
    directory: list[str]
    pattern: str
    error: str

@dataclass(frozen=True)
class SearchBatchResult:
    """Result of :func:`search`.

    Attributes:
        results: One :class:`SearchResult` per successfully executed search.
        errors: One :class:`SearchItemError` per search that failed.
    """
    results: list[SearchResult]
    errors: list[SearchItemError]

def _parse_search_stdout(stdout: str) -> list[SearchMatch]:
    """Parse 'path:line:content' stdout into :class:`SearchMatch` objects, grouped by file."""
    grouped: dict[tuple[str, str], list[SearchLineMatch]] = {}
    order: list[tuple[str, str]] = []
    for line in stdout.splitlines():
        if not line:
            continue
        path, sep, rest = line.partition(':')
        if not sep:
            raise SearchError(f'Cannot parse search output line: {line!r}')
        lineno_str, sep, match = rest.partition(':')
        if not sep or not lineno_str.isdigit():
            raise SearchError(f'Cannot parse search output line: {line!r}')
        directory, _, filename = path.rpartition('/')
        key = (directory, filename)
        if key not in grouped:
            grouped[key] = []
            order.append(key)
        grouped[key].append(SearchLineMatch(lineno=int(lineno_str), match=match.strip()))
    return [SearchMatch(directory=d, filename=f, matches=grouped[d, f]) for d, f in order]

def _as_list(value: list[str] | None) -> list[str]:
    """Normalize an optional list into a list (empty if ``None``)."""
    return list(value) if value is not None else []

def _run_search(directory: list[str], pattern: str, *, exclude: list[str] | None=None, include: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> ProcessResult:
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
            exit_code: 0 if matches were found, 1 if none were found, >=2 on search error.
            stdout: Matching lines as 'path:line:content', with ``path`` relative to
                whichever searched directory it was found under, truncated to at most
                ``limit`` lines.
            stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
            stdout_file: Absolute path to temp file with full stdout if to large.
            stderr_file: Absolute path to temp file with full stderr if to large.

    Raises:
        SearchError: If a directory is not absolute.
        SearchError: If a directory does not exist or is not a directory.
        SearchError: If no directory is given.
        SearchError: If pattern is empty.
        SearchError: If limit is not between 1 and ``_MAX_LIMIT``.
        SearchError: If search binary cannot be launched.
    """
    directory_paths = normalize_directories([Path(d) for d in _as_list(directory)])
    if not directory_paths:
        raise SearchError('At least one directory is required.')
    for directory_path in directory_paths:
        if not directory_path.is_absolute():
            raise SearchError('directory must be an absolute path.')
        if not directory_path.is_dir():
            raise SearchError('Directory not found or not a directory.')
    if not pattern:
        raise SearchError('pattern must not be empty.')
    if not 1 <= limit <= _MAX_LIMIT:
        raise SearchError(f'limit must be between 1 and {_MAX_LIMIT}.')
    cmd = ['grep', '--recursive', '--line-number', '--extended-regexp', '--binary-files=without-match', '--color=never']
    for pattern_glob in _as_list(include):
        cmd.append(f'--include={pattern_glob}')
    for pattern_glob in _as_list(exclude):
        cmd.append(f'--exclude={pattern_glob}')
    cmd += ['--', pattern, *(str(p) for p in directory_paths)]
    try:
        result = run_process(cmd)
    except LaunchError as exc:
        raise SearchError(f'Failed to launch: {exc}') from exc
    prefixes = sorted((str(p).rstrip('/') + '/' for p in directory_paths), key=len, reverse=True)
    prefix_pattern = '|'.join((re.escape(p) for p in prefixes))
    stdout = re.sub(f'^(?:{prefix_pattern})', '', result.stdout, flags=re.MULTILINE)
    lines = stdout.splitlines()
    stdout = '\n'.join(lines[:limit])
    return ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr)

def _search_one(item: SearchItem) -> SearchResult:
    result = _run_search(item.directory, item.pattern, exclude=item.exclude, include=item.include, limit=item.limit)
    if result.exit_code >= 2:
        raise SearchError(f'search failed (exit code {result.exit_code}): {result.stderr}')
    matches = _parse_search_stdout(result.stdout)
    total_matches = sum((len(m.matches) for m in matches))
    warning = None
    if total_matches >= item.limit:
        warning = f'Limit of {
            item.limit} matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit.'
    return SearchResult(directory=item.directory, matches=matches, warning=warning)

def _resolve_nodes(match: SearchMatch, roots: list[str]) -> SearchMatch:
    """Regroup a file's line matches by the most specific enclosing AST node.

    ``match`` paths are relative to one of ``roots``. Files that cannot be parsed
    are returned unchanged. Lines not covered by any node stay in ``matches``.
    """
    relative = f'{match.directory}/{match.filename}' if match.directory else match.filename
    path = next((str(Path(r) / relative) for r in roots if (Path(r) / relative).is_file()), None)
    if path is None:
        return match
    try:
        located = core.locate_all(core.load(path)[1])
    except (core.AstError, OSError):
        return match
    by_node: dict[str, list[SearchLineMatch]] = {}
    unresolved: list[SearchLineMatch] = []
    for lm in match.matches:
        loc = core.most_specific(located, lm.lineno, lm.lineno)
        if loc is None:
            unresolved.append(lm)
        else:
            by_node.setdefault(loc.node_id, []).append(lm)
    nodes = [SearchNodeMatch(node_id=nid, matches=lms) for nid, lms in by_node.items()]
    return SearchMatch(directory=match.directory, filename=match.filename, matches=unresolved, nodes=nodes)

def search(items: list[SearchItem], *, intent: str=_DEFAULT_INTENT) -> SearchBatchResult:
    """Run one or more independent searches. Limits apply per item, not per batch.

    Args:
        items: Searches to run. Must be non-empty.
        intent: Default intent for all items; ``SearchItem.intent`` takes precedence.
            ``'ast_node_ids'`` resolves matched lines to AST node ids (see :class:`SearchMatch`).

    Returns:
        SearchBatchResult: one result per successful search, one error per failed search.

    Raises:
        SearchError: If items is empty or the batch-level intent is invalid.
    """
    if not items:
        raise SearchError("'items' must be a non-empty list.")
    _check_intent(intent)
    results: list[SearchResult] = []
    errors: list[SearchItemError] = []
    for item in items:
        try:
            effective = item.intent if item.intent is not None else intent
            _check_intent(effective)
            result = _search_one(item)
            if effective == 'ast_node_ids':
                result = replace(result, matches=[_resolve_nodes(m, item.directory) for m in result.matches])
            results.append(result)
        except SearchError as exc:
            errors.append(SearchItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
    return SearchBatchResult(results=results, errors=errors)

def _check_intent(intent: str) -> None:
    if intent not in _INTENTS:
        raise SearchError(f'Invalid intent {intent!r}; expected one of {', '.join(_INTENTS)}.')

class SearchTool(ToolDefinition):
    name = 'search'
    title = 'Search in directories'
    description = "Run one or more independent searches with an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters."
    _ITEM_PROPERTIES = {
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
            'maximum': _MAX_LIMIT},
        'intent': {
            'type': 'string',
            'enum': list(_INTENTS),
            'description': 'Intent for this item; overrides the batch-level intent.'}}
    _ITEM_REQUIRED = ['directory', 'pattern']
    _ITEMS_DESCRIPTION = 'Independent searches to run.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)
    input_schema['properties']['intent'] = {
        'type': 'string',
        'enum': list(_INTENTS),
        'default': _DEFAULT_INTENT,
        'description': 'Default intent for all items.' + _INTENT_DESCRIPTION}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func: translating the MCP schema to/from the Python API."""
        intent = ctx.arguments.get('intent', _DEFAULT_INTENT)

        def item_factory(it: dict[str, Any]) -> SearchItem:
            return SearchItem(
                directory=it['directory'],
                pattern=it['pattern'],
                exclude=it.get('exclude'),
                include=it.get('include'),
                limit=int(
                    it.get(
                        'limit',
                        _DEFAULT_LIMIT)),
                intent=it.get('intent'))

        def line_dicts(lms: list[SearchLineMatch], with_lineno: bool) -> list[Any]:
            if with_lineno:
                return [{'lineno': lm.lineno, 'match': lm.match} for lm in lms]
            return [lm.match for lm in lms]

        def file_dict(m: SearchMatch, item_intent: str) -> dict[str, Any]:
            entry: dict[str, Any] = {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename}
            if m.nodes:
                entry['nodes'] = [{'id': n.node_id, 'matches': line_dicts(
                    n.matches, item_intent == 'line_numbers')} for n in m.nodes]
            if m.matches:
                lines = line_dicts(m.matches, item_intent != 'locate')
                entry['matches'] = list(dict.fromkeys(lines)) if item_intent == 'locate' else lines
            return entry

        def result_serializer(rendered: tuple[SearchResult, str]) -> dict[str, Any]:
            r, item_intent = rendered
            entry: dict[str, Any] = {'directory': r.directory}
            if r.matches:
                entry['matches'] = [file_dict(m, item_intent) for m in r.matches]
            if r.warning is not None:
                entry['warning'] = r.warning
            return entry

        def error_serializer(e: SearchItemError) -> dict[str, Any]:
            return {'directory': e.directory, 'pattern': e.pattern, 'error': e.error}

        def run(items: list[SearchItem]) -> SimpleNamespace:
            results: list[tuple[SearchResult, str]] = []
            errors: list[SearchItemError] = []
            for item in items:
                batch = search([item], intent=intent)
                effective = item.intent if item.intent is not None else intent
                results += [(r, effective) for r in batch.results]
                errors += batch.errors
            return SimpleNamespace(results=results, errors=errors)
        return handle_batch_tool(ctx, item_factory, run, SearchError, result_serializer, error_serializer)

def register_search_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(SearchTool())
    functions.register(search)