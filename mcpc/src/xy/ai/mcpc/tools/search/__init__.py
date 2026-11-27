"""Search tool – recursive extended-regex search for retrieval, for a batch of items."""
from dataclasses import dataclass, replace
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools._directories import normalize_directories
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolFeature, ToolRegistry, ToolResult
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, run_process
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
from xy.ai.mcpc.tools._tool_helpers import handle_batch_tool, batch_schema
from xy.ai.mcpc.tools._mcpcignore import is_allowed
from xy.ai.mcpc.tools.results import BatchResult, ItemError, ItemResult
import re
from xy.ai.mcpc.tools.ast import core
from pathspec import PathSpec
__all__ = [
    'SearchError',
    'SearchLineMatch',
    'SearchNodeMatch',
    'SearchMatch',
    'SearchItem',
    'SearchResult',
    'SearchItemError',
    'SearchBatchResult',
    'SEARCH_DESCRIPTION',
    'search',
    'SearchTool',
    'register_search_tool']
_DEFAULT_LIMIT = 15
_INTENTS_WITHOUT_LOCATION = ('probe', 'survey', 'sample', 'check_existence', 'find_examples_or_conventions')
_INTENTS_WITH_LOCATION = ('locate_for_edit', 'retrieve_context', 'locate_error', 'edit')
_INTENTS = _INTENTS_WITHOUT_LOCATION + _INTENTS_WITH_LOCATION
_DEFAULT_INTENT = 'survey'
_MAX_LIMIT = 50
_MAX_LINE_LENGTH = 300

class SearchError(Exception):
    """Raised when a search cannot be executed or its output cannot be parsed."""

@dataclass(frozen=True)
class SearchLineMatch:
    """A single matching line within a file."""
    lineno: int
    match: str
    truncated: bool = False

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
        file_exclude_mask: Globs on the file base name, passed to grep --exclude; files whose base name matches one are skipped.
        file_include_mask: Globs on the file base name, passed to grep --include; if given, only files whose base name matches one are searched.
        dir_exclude_mask: Gitignore-style glob masks; files whose absolute directory path matches one are skipped.
        dir_include_mask: Gitignore-style glob masks; if given, only files whose absolute directory path matches one are searched.
        limit: Maximum number of matches to return (1..``_MAX_LIMIT``). Applies per item, not per batch.
        intent: Purpose of this search (one of ``_INTENTS``); intents with location information also
            return line numbers / AST node ids. Defaults to ``_DEFAULT_INTENT``.
    """
    directory: list[str]
    pattern: str
    file_exclude_mask: list[str] | None = None
    file_include_mask: list[str] | None = None
    dir_exclude_mask: list[str] | None = None
    dir_include_mask: list[str] | None = None
    limit: int = _DEFAULT_LIMIT
    intent: str | None = None

@dataclass(frozen=True)
class SearchResult(ItemResult):
    """Result of a single search.

    Attributes:
        directory: The directory list exactly as given in the input.
        matches: The matches found, grouped by file (empty if none).
        warnings: Set if ``limit`` was reached and further matches may exist, and, separately,
            if match lines exceeded the maximum length and were shortened with an ellipsis.
    """
    directory: list[str]
    matches: list[SearchMatch]

    def key(self) -> dict[str, Any]:
        return {'directory': self.directory}

@dataclass(frozen=True)
class SearchItemError(ItemError):
    """Error running a single search, mirroring its input for result association."""
    directory: list[str]
    pattern: str

    def key(self) -> dict[str, Any]:
        return {'directory': self.directory, 'pattern': self.pattern}

class SearchBatchResult(BatchResult[SearchResult, SearchItemError]):
    """Result of :func:`search`.

    Attributes:
        results: One :class:`SearchResult` per successfully executed search.
        errors: One :class:`SearchItemError` per search that failed.
    """

def _make_line_match(lineno: int, text: str) -> SearchLineMatch:
    if len(text) <= _MAX_LINE_LENGTH:
        return SearchLineMatch(lineno=lineno, match=text)
    half = _MAX_LINE_LENGTH // 2
    return SearchLineMatch(lineno=lineno, match=f'{text[:half]}…{text[-half:]}', truncated=True)

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
        grouped[key].append(_make_line_match(int(lineno_str), match.strip()))
    return [SearchMatch(directory=d, filename=f, matches=grouped[d, f]) for d, f in order]

def _as_list(value: list[str] | str | None) -> list[str]:
    """Normalize an optional list into a list (empty if ``None``).

    A bare string is tolerated and treated as a single list item, even though
    this violates the schema.
    """
    if value is None:
        return []
    if isinstance(value, str):
        return [value]
    return list(value)

def _drop_permission_errors(result: ProcessResult) -> ProcessResult:
    """Remove grep's ``Permission denied`` stderr lines; they are noise for the caller.

    If nothing but such lines was reported on stderr, grep's exit code 2 is mapped to
    0 (matches found) or 1 (no matches), so unreadable entries do not fail the search.
    """
    if not result.stderr:
        return result
    kept = [line for line in result.stderr.splitlines() if not line.endswith(': Permission denied')]
    if len(kept) == len(result.stderr.splitlines()):
        return result
    stderr = '\n'.join(kept)
    exit_code = result.exit_code
    if not kept and exit_code == 2:
        exit_code = 0 if result.stdout.strip() else 1
    return ProcessResult(exit_code=exit_code, stdout=result.stdout, stderr=stderr)

class _DirMasks:
    """Gitignore-style glob masks (gitwildmatch) on the absolute directory path of a file.

    A file is selected if no exclude mask matches and, if include masks are given, one of them matches.
    """

    def __init__(self, dir_exclude: list[str] | None, dir_include: list[str] | None) -> None:
        self._dir_exclude = self._spec(dir_exclude)
        self._dir_include = self._spec(dir_include)

    @staticmethod
    def _spec(masks: list[str] | str | None) -> PathSpec | None:
        lines = _as_list(masks)
        return PathSpec.from_lines('gitwildmatch', lines) if lines else None

    def selects(self, path: Path) -> bool:
        key = path.parent.as_posix().lstrip('/')
        if self._dir_exclude and self._dir_exclude.match_file(key):
            return False
        return not self._dir_include or self._dir_include.match_file(key)

def _run_search(directory: list[str], pattern: str, *, file_exclude_mask: list[str] | None=None, file_include_mask: list[str] | None=None, dir_exclude_mask: list[str] | None=None, dir_include_mask: list[str] | None=None, limit: int=_DEFAULT_LIMIT) -> tuple[ProcessResult, int]:
    """Recursively search one or more directories for ``pattern`` (extended regexp).

    Args:
        directory: Absolute paths of the directories to search (each must exist and
            be a directory).
        pattern: Extended regular expression (grep -E syntax).
        file_exclude_mask: Globs on the file base name, passed to grep --exclude; files whose base name matches one are skipped.
        file_include_mask: Globs on the file base name, passed to grep --include; if given, only files whose base name matches one are searched.
        dir_exclude_mask: Gitignore-style glob masks; files whose absolute directory path matches one are skipped.
        dir_include_mask: Gitignore-style glob masks; if given, only files whose absolute directory path matches one are searched.
        limit: Maximum number of matching lines to return (1..``_MAX_LIMIT``).

    Returns:
        Tuple ``(result, total_lines)`` where ``total_lines`` is the number of matching
        lines found before truncation to ``limit``, and ``result`` is a ProcessResult with:
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
    for pattern_glob in _as_list(file_include_mask):
        cmd.append(f'--include={pattern_glob}')
    for pattern_glob in _as_list(file_exclude_mask):
        cmd.append(f'--exclude={pattern_glob}')
    cmd += ['--', pattern, *(str(p) for p in directory_paths)]
    try:
        result = run_process(cmd)
    except LaunchError as exc:
        raise SearchError(f'Failed to launch: {exc}') from exc
    result = _drop_permission_errors(result)
    masks = _DirMasks(dir_exclude_mask, dir_include_mask)
    allowed_lines = []
    for line in result.stdout.splitlines():
        path = Path(line.partition(':')[0])
        if masks.selects(path) and is_allowed(path):
            allowed_lines.append(line)
    result = ProcessResult(exit_code=result.exit_code if allowed_lines else min(
        result.exit_code, 1), stdout='\n'.join(allowed_lines), stderr=result.stderr)
    prefixes = sorted((str(p).rstrip('/') + '/' for p in directory_paths), key=len, reverse=True)
    prefix_pattern = '|'.join((re.escape(p) for p in prefixes))
    stdout = re.sub(f'^(?:{prefix_pattern})', '', result.stdout, flags=re.MULTILINE)
    lines = stdout.splitlines()
    stdout = '\n'.join(lines[:limit])
    return (ProcessResult(exit_code=result.exit_code, stdout=stdout, stderr=result.stderr), len(lines))

def _search_one(item: SearchItem) -> SearchResult:
    result, total_lines = _run_search(item.directory, item.pattern, file_exclude_mask=item.file_exclude_mask, file_include_mask=item.file_include_mask,
                                      dir_exclude_mask=item.dir_exclude_mask, dir_include_mask=item.dir_include_mask, limit=item.limit)
    if result.exit_code >= 2:
        raise SearchError(f'search failed (exit code {result.exit_code}): {result.stderr}')
    matches = _parse_search_stdout(result.stdout)
    warnings: list[str] = []
    if total_lines > item.limit:
        hint = 'narrow pattern, directory or the file/dir masks'
        if item.limit < _MAX_LIMIT:
            hint += f', or raise limit (max {_MAX_LIMIT})'
        warnings.append(f'Limit of {item.limit} matches reached (total {total_lines}); {hint}.')
    shortened = sum((lm.truncated for m in matches for lm in m.matches))
    if shortened:
        warnings.append(
            f'{shortened} match line(s) exceeded {_MAX_LINE_LENGTH} characters and were shortened in the middle.')
    return SearchResult(directory=item.directory, matches=matches, warnings=tuple(warnings))

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

def search(items: list[SearchItem], *, with_nodes: bool=False) -> SearchBatchResult:
    """Run one or more independent searches. Limits apply per item, not per batch.

    Args:
        items: Searches to run. Must be non-empty.
        with_nodes: If true, items with a location intent resolve matched lines to AST node ids
            (see :class:`SearchMatch`).

    Returns:
        SearchBatchResult: one result per successful search, one error per failed search.

    Raises:
        SearchError: If items is empty.
    """
    if not items:
        raise SearchError("'items' must be a non-empty list.")
    results: list[SearchResult] = []
    errors: list[SearchItemError] = []
    for item in items:
        try:
            effective = item.intent if item.intent is not None else _DEFAULT_INTENT
            _check_intent(effective)
            result = _search_one(item)
            if effective in _INTENTS_WITH_LOCATION and with_nodes:
                result = replace(result, matches=[_resolve_nodes(m, item.directory) for m in result.matches])
            results.append(result)
        except SearchError as exc:
            errors.append(SearchItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
    return SearchBatchResult(results=results, errors=errors)

def _check_intent(intent: str) -> None:
    if intent not in _INTENTS:
        raise SearchError(f'Invalid intent {intent!r}; expected one of {', '.join(_INTENTS)}.')
SEARCH_DESCRIPTION = 'Run one or more independent searches with an extended regular expression, for a batch of items. Always use the file/dir include and exclude masks.'

class SearchTool(ToolDefinition):
    name = 'search'
    title = 'Search code, text, id, and content within directories'
    description = SEARCH_DESCRIPTION
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
        'file_exclude_mask': {
            'type': 'array',
            'items': {
                'type': 'string'},
            'description': "Globs passed to grep --exclude; matching files are skipped, e.g. '*.min.js'. Always set this to exclude minified and generated files."},
        'file_include_mask': {
            'type': 'array',
            'items': {
                'type': 'string'},
            'description': "Globs passed to grep --include; if given, only matching files are searched, e.g. '*.py'. Always set this to restrict the search to the relevant file types; omit only when the file type is unknown."},
        'dir_exclude_mask': {
            'type': 'array',
            'items': {
                'type': 'string'},
            'description': "Gitignore-style globs on the absolute path of the directory containing the file; files in matching directories are skipped, e.g. 'node_modules', '**/build/output'. Always set this to exclude build artefacts and dependencies."},
        'dir_include_mask': {
            'type': 'array',
            'items': {
                'type': 'string'},
            'description': "Gitignore-style globs on the absolute path of the directory containing the file; if given, only files in matching directories are searched, e.g. 'src'."},
        'limit': {
            'type': 'integer',
            'description': 'Maximum number of matching lines to return for this item.',
            'default': _DEFAULT_LIMIT,
            'minimum': 1,
            'maximum': _MAX_LIMIT},
        'intent': {
            'type': 'string',
            'enum': list(_INTENTS),
            'default': _DEFAULT_INTENT}}
    _ITEM_REQUIRED = ['directory', 'pattern']
    _ITEMS_DESCRIPTION = 'Independent searches to run.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def __init__(self, registry: ToolRegistry) -> None:
        self._registry = registry

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func: translating the MCP schema to/from the Python API."""
        with_lines = self._registry.session_has_feature(ctx.session, ToolFeature.INPUT_LINENO)
        with_nodes = self._registry.session_has_feature(ctx.session, ToolFeature.INPUT_ASTID)
        intents: dict[int, str] = {}

        def item_factory(it: dict[str, Any]) -> SearchItem:
            return SearchItem(
                directory=it['directory'],
                pattern=it['pattern'],
                file_exclude_mask=it.get('file_exclude_mask'),
                file_include_mask=it.get('file_include_mask'),
                dir_exclude_mask=it.get('dir_exclude_mask'),
                dir_include_mask=it.get('dir_include_mask'),
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
            located = item_intent in _INTENTS_WITH_LOCATION
            entry: dict[str, Any] = {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename}
            if m.nodes:
                entry['nodes'] = [{'id': n.node_id, 'matches': line_dicts(
                    n.matches, located and with_lines)} for n in m.nodes]
            if m.matches:
                lines = line_dicts(m.matches, located and (with_lines or with_nodes))
                entry['matches'] = lines if located else list(dict.fromkeys(lines))
            return entry

        def result_payload(r: SearchResult) -> dict[str, Any]:
            if not r.matches:
                return {}
            return {'matches': [file_dict(m, intents[id(r)]) for m in r.matches]}

        def run(items: list[SearchItem]) -> SearchBatchResult:
            results: list[SearchResult] = []
            errors: list[SearchItemError] = []
            for item in items:
                batch = search([item], with_nodes=with_nodes)
                effective = item.intent if item.intent is not None else _DEFAULT_INTENT
                for r in batch.results:
                    intents[id(r)] = effective
                results += batch.results
                errors += batch.errors
            return SearchBatchResult(results=results, errors=errors)
        return handle_batch_tool(ctx, item_factory, run, SearchError, result_payload)

def register_search_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(SearchTool(registry))
    functions.register(search)