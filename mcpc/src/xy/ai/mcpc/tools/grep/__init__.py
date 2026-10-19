"""Grep tool – recursive extended-regex search for retrieval, for a batch of items."""
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
    'GrepError',
    'GrepLineMatch',
    'GrepNodeMatch',
    'GrepMatch',
    'GrepItem',
    'GrepResult',
    'GrepItemError',
    'GrepBatchResult',
    'grep',
    'GrepTool',
    'register_grep_tool']
_DEFAULT_LIMIT = 15
_INTENTS = ('locate', 'read_lines', 'ast_navigate')
_DEFAULT_INTENT = 'locate'
_INTENT_DESCRIPTION = "What you will do with the results. 'locate': only need to know which files/matches exist (paths + text). 'read_lines': you will read surrounding code next with the read tool; returns path + line numbers + text. 'ast_navigate': you will continue with ast_* tools; returns path + nodeId (matches outside any AST node keep their line numbers)."
_MAX_LIMIT = 50

class GrepError(Exception):
    """Raised when a grep search cannot be executed or its output cannot be parsed."""

@dataclass(frozen=True)
class GrepLineMatch:
    """A single matching line within a file."""
    lineno: int
    match: str

@dataclass(frozen=True)
class GrepNodeMatch:
    """Matching lines attributed to the most specific enclosing AST node."""
    node_id: str
    matches: list[GrepLineMatch]

@dataclass(frozen=True)
class GrepMatch:
    """All matches found within a single file.

    ``nodes`` is set instead of relying on ``matches`` only if the file could be
    resolved to AST nodes; lines outside any node are kept under ``matches``.
    """
    directory: str
    filename: str
    matches: list[GrepLineMatch]
    nodes: list[GrepNodeMatch] | None = None

@dataclass(frozen=True)
class GrepItem:
    """One independent grep search to run.

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
class GrepResult:
    """Result of a single grep search.

    Attributes:
        directory: The directory list exactly as given in the input.
        matches: The matches found, grouped by file (empty if none).
        warning: Set if ``limit`` was reached and further matches may exist.
    """
    directory: list[str]
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
    """Parse grep's 'path:line:content' stdout into :class:`GrepMatch` objects, grouped by file."""
    grouped: dict[tuple[str, str], list[GrepLineMatch]] = {}
    order: list[tuple[str, str]] = []
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
        key = (directory, filename)
        if key not in grouped:
            grouped[key] = []
            order.append(key)
        grouped[key].append(GrepLineMatch(lineno=int(lineno_str), match=match.strip()))
    return [GrepMatch(directory=d, filename=f, matches=grouped[d, f]) for d, f in order]

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
    total_matches = sum((len(m.matches) for m in matches))
    warning = None
    if total_matches >= item.limit:
        warning = f'Limit of {
            item.limit} matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit.'
    return GrepResult(directory=item.directory, matches=matches, warning=warning)

def _resolve_nodes(match: GrepMatch, roots: list[str]) -> GrepMatch:
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
    by_node: dict[str, list[GrepLineMatch]] = {}
    unresolved: list[GrepLineMatch] = []
    for lm in match.matches:
        loc = core.most_specific(located, lm.lineno, lm.lineno)
        if loc is None:
            unresolved.append(lm)
        else:
            by_node.setdefault(loc.node_id, []).append(lm)
    nodes = [GrepNodeMatch(node_id=nid, matches=lms) for nid, lms in by_node.items()]
    return GrepMatch(directory=match.directory, filename=match.filename, matches=unresolved, nodes=nodes)

def grep(items: list[GrepItem], *, intent: str=_DEFAULT_INTENT) -> GrepBatchResult:
    """Run one or more independent grep searches. Limits apply per item, not per batch.

    Args:
        items: Grep searches to run. Must be non-empty.
        intent: Default intent for all items; ``GrepItem.intent`` takes precedence.
            ``'ast_navigate'`` resolves matched lines to AST node ids (see :class:`GrepMatch`).

    Returns:
        GrepBatchResult: one result per successful search, one error per failed search.

    Raises:
        GrepError: If items is empty or the batch-level intent is invalid.
    """
    if not items:
        raise GrepError("'items' must be a non-empty list.")
    _check_intent(intent)
    results: list[GrepResult] = []
    errors: list[GrepItemError] = []
    for item in items:
        try:
            effective = item.intent if item.intent is not None else intent
            _check_intent(effective)
            result = _grep_one(item)
            if effective == 'ast_navigate':
                result = replace(result, matches=[_resolve_nodes(m, item.directory) for m in result.matches])
            results.append(result)
        except GrepError as exc:
            errors.append(GrepItemError(directory=item.directory, pattern=item.pattern, error=str(exc)))
    return GrepBatchResult(results=results, errors=errors)

def _check_intent(intent: str) -> None:
    if intent not in _INTENTS:
        raise GrepError(f'Invalid intent {intent!r}; expected one of {', '.join(_INTENTS)}.')

class GrepTool(ToolDefinition):
    name = 'grep'
    title = 'Search files with grep'
    description = "Run one or more independent grep searches for lines matching an extended regular expression, for a batch of items. Always use the 'include' and 'exclude' filters. Limits apply per item, not per batch."
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
    _ITEMS_DESCRIPTION = 'Independent grep searches to run.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION, additional_properties=False)
    input_schema['properties']['intent'] = {
        'type': 'string',
        'enum': list(_INTENTS),
        'default': _DEFAULT_INTENT,
        'description': 'Default intent for all items; an item-level intent takes precedence. ' + _INTENT_DESCRIPTION}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`grep`, translating the MCP schema to/from the Python API."""
        intent = ctx.arguments.get('intent', _DEFAULT_INTENT)

        def item_factory(it: dict[str, Any]) -> GrepItem:
            return GrepItem(
                directory=it['directory'],
                pattern=it['pattern'],
                exclude=it.get('exclude'),
                include=it.get('include'),
                limit=int(
                    it.get(
                        'limit',
                        _DEFAULT_LIMIT)),
                intent=it.get('intent'))

        def line_dicts(lms: list[GrepLineMatch], with_lineno: bool) -> list[Any]:
            if with_lineno:
                return [{'lineno': lm.lineno, 'match': lm.match} for lm in lms]
            return [lm.match for lm in lms]

        def file_dict(m: GrepMatch, item_intent: str) -> dict[str, Any]:
            entry: dict[str, Any] = {'path': f'{m.directory}/{m.filename}' if m.directory else m.filename}
            if m.nodes:
                entry['nodes'] = [{'id': n.node_id, 'matches': line_dicts(
                    n.matches, item_intent == 'read_lines')} for n in m.nodes]
            if m.matches:
                entry['matches'] = line_dicts(m.matches, item_intent != 'locate')
            return entry

        def result_serializer(rendered: tuple[GrepResult, str]) -> dict[str, Any]:
            r, item_intent = rendered
            entry: dict[str, Any] = {'directory': r.directory}
            if r.matches:
                entry['matches'] = [file_dict(m, item_intent) for m in r.matches]
            if r.warning is not None:
                entry['warning'] = r.warning
            return entry

        def error_serializer(e: GrepItemError) -> dict[str, Any]:
            return {'directory': e.directory, 'pattern': e.pattern, 'error': e.error}

        def run(items: list[GrepItem]) -> SimpleNamespace:
            results: list[tuple[GrepResult, str]] = []
            errors: list[GrepItemError] = []
            for item in items:
                batch = grep([item], intent=intent)
                effective = item.intent if item.intent is not None else intent
                results += [(r, effective) for r in batch.results]
                errors += batch.errors
            return SimpleNamespace(results=results, errors=errors)
        return handle_batch_tool(ctx, item_factory, run, GrepError, result_serializer, error_serializer)

def register_grep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(GrepTool())
    functions.register(grep)