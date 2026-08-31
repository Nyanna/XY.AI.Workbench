"""colgrep tool - semantic + keyword code search over a pre-built colgrep index.

Search-only wrapper around the `colgrep` CLI.
"""
import json
import os
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ColgrepError', 'ColgrepResult', 'colgrep', 'ColgrepTool', 'register_colgrep_tool']
_COLGREP_BIN = '/home/user/.cargo/bin/colgrep'
_CONTEXT_LINES = '2'
_DEFAULT_RESULTS = 15
_MAX_RESULTS = 50
_MAX_CODE_LEN = 100
_SEMANTIC_ONLY_MAX_LEN = 4
_ALPHA_RAMP_MIN_LEN = 5
_ALPHA_RAMP_MAX_LEN = 10
_ALPHA_RAMP_START = 0.9
_ALPHA_RAMP_END = 0.6
_DROPPED_KEYS = frozenset({'language', 'signature', 'qualified_name', 'unit_type', 'complexity', 'has_loops', 'has_branches', 'has_error_handling', 'extends', 'parent_class', 'variables', 'name', 'return_type', 'calls', 'imports', 'parameters'})

class ColgrepError(Exception):
    """Raised when a colgrep search cannot be performed."""

@dataclass(frozen=True)
class ColgrepResult:
    results: list[Any] = field(default_factory=list)

def _find_index_root(start: Path) -> Path | None:
    """Climb from *start* up to the filesystem root looking for a colgrep index.

    A directory ``D`` is considered a colgrep project root if
    ``D/.colgrep/colgrep/indices`` exists and is non-empty - the layout
    produced when colgrep is run with ``XDG_DATA_HOME=D`` (see
    ``colgrep-init.sh``).
    """
    current = start
    while True:
        candidate = current / '.colgrep' / 'colgrep' / 'indices'
        if candidate.is_dir() and any(candidate.iterdir()):
            return current
        if current.parent == current:
            return None
        current = current.parent

def _clean_result(value: Any) -> Any:
    """Recursively drop empty components (``False``, ``""``, ``None``, ``[]``) and
    unwanted keys (``score`` plus the fields listed in ``_DROPPED_KEYS``) from
    colgrep JSON output. The ``code`` field is truncated to ``_MAX_CODE_LEN``
    characters.
    """
    if isinstance(value, dict):
        cleaned = {}
        for key, item in value.items():
            if key == 'score' or key in _DROPPED_KEYS:
                continue
            if key == 'code' and isinstance(item, str) and (len(item) > _MAX_CODE_LEN):
                item = item[:_MAX_CODE_LEN]
            cleaned_item = _clean_result(item)
            if cleaned_item is False or cleaned_item == '' or cleaned_item is None or (cleaned_item == []):
                continue
            cleaned[key] = cleaned_item
        return cleaned
    if isinstance(value, list):
        return [_clean_result(item) for item in value]
    return value

def _search_mode_for_query(query: str) -> tuple[bool, float | None]:
    """Derive (semantic_only, alpha) from the query length.

    <= _SEMANTIC_ONLY_MAX_LEN chars: semantic-only search, no alpha.
    _ALPHA_RAMP_MIN_LEN.._ALPHA_RAMP_MAX_LEN chars: alpha ramps linearly from
    _ALPHA_RAMP_START down to _ALPHA_RAMP_END.
    Longer queries: alpha fixed at _ALPHA_RAMP_END.
    """
    length = len(query)
    if length <= _SEMANTIC_ONLY_MAX_LEN:
        return (True, None)
    if length >= _ALPHA_RAMP_MAX_LEN:
        return (False, _ALPHA_RAMP_END)
    span = _ALPHA_RAMP_MAX_LEN - _ALPHA_RAMP_MIN_LEN
    fraction = (_ALPHA_RAMP_MAX_LEN - length) / span
    alpha = _ALPHA_RAMP_END + fraction * (_ALPHA_RAMP_START - _ALPHA_RAMP_END)
    return (False, round(alpha, 2))

def colgrep(path: str, query: str, results: int=_DEFAULT_RESULTS, code_only: bool=False, files_only: bool=False, include: list[str] | None=None, exclude: list[str] | None=None, exclude_dir: list[str] | None=None) -> ColgrepResult:
    """Search colgrep index for code matching query.

    Searches the colgrep index covering the given path using semantic and/or
    keyword matching. The tool climbs up from path to find the index root.
    The search mode (semantic-only vs. keyword/semantic fusion via alpha) is
    derived automatically from the query length, see ``_search_mode_for_query``.

    Args:
        path: Absolute directory path to search within (must be a directory).
        query: Non-empty search query (semantic, keyword, or combined).
        results: Number of results to return (minimum 1, maximum 50). Default: 15.
        code_only: If True, search only code (skip comments, docs).
        files_only: If True, return only file paths without content; otherwise
                    each result includes a code snippet truncated to 100 chars.
        include: Optional list of glob patterns to include in search.
        exclude: Optional list of glob patterns to exclude from search.
        exclude_dir: Optional list of directory names to exclude from search.

    Returns:
        ColgrepResult with results: list of matched results (each cleaned to
        max 100 char code snippets, unless files_only).

    Raises:
        ColgrepError: If path is not absolute or not a directory.
        ColgrepError: If query is empty.
        ColgrepError: If results not in range [1, 50].
        ColgrepError: If no colgrep index found in path or parent directories.
        ColgrepError: If colgrep binary fails or returns unparseable JSON.
    """
    include = include or []
    exclude = exclude or []
    exclude_dir = exclude_dir or []
    if not query.strip():
        raise ColgrepError('query must not be empty.')
    search_path = Path(path)
    if not search_path.is_absolute():
        raise ColgrepError('path must be an absolute path.')
    if not search_path.is_dir():
        raise ColgrepError('Directory not found.')
    if not 1 <= results <= _MAX_RESULTS:
        raise ColgrepError(f'results must be between 1 and {_MAX_RESULTS}.')
    search_dir = search_path.resolve()
    index_root = _find_index_root(search_dir)
    if index_root is None:
        raise ColgrepError('No colgrep index found for this directory or any parent directory.')
    semantic_only, alpha = _search_mode_for_query(query)
    cmd = [_COLGREP_BIN, query, str(search_dir), '--json', '-n', _CONTEXT_LINES, '-k', str(results)]
    if files_only:
        cmd.append('-l')
    if code_only:
        cmd.append('--code-only')
    if semantic_only:
        cmd.append('--semantic-only')
    elif alpha is not None:
        cmd.extend(['--alpha', str(alpha)])
    for pattern in include:
        cmd.append(f'--include={pattern}')
    for pattern in exclude:
        cmd.append(f'--exclude={pattern}')
    for name in exclude_dir:
        cmd.append(f'--exclude-dir={name}')
    cmd.append('--no-update')
    env = dict(os.environ)
    env['XDG_DATA_HOME'] = str(index_root / '.colgrep')
    env['XDG_CONFIG_HOME'] = str(index_root / '.colgrep')
    try:
        proc = subprocess.run(cmd, cwd=str(index_root), env=env, input='', capture_output=True, encoding='utf-8', errors='replace')
    except OSError as exc:
        raise ColgrepError(f'Failed to launch colgrep: {exc}') from exc
    if proc.returncode != 0:
        message = proc.stderr.strip() or proc.stdout.strip() or f'colgrep exited with code {proc.returncode}.'
        raise ColgrepError(message)
    try:
        parsed = json.loads(proc.stdout) if proc.stdout.strip() else []
    except json.JSONDecodeError as exc:
        raise ColgrepError('colgrep returned output that could not be parsed as JSON.') from exc
    parsed = _clean_result(parsed)
    if isinstance(parsed, list):
        return ColgrepResult(results=parsed)
    return ColgrepResult(results=[parsed])

class ColgrepTool(ToolDefinition):
    name = 'colgrep'
    title = 'Search code with colgrep'
    description = "Search a project's codebase with colgrep."
    input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute directory to search in.'}, 'query': {'type': 'string', 'description': 'Search query: natural language and/or identifiers/keywords.'}, 'results': {'type': 'integer', 'minimum': 1, 'maximum': _MAX_RESULTS, 'default': _DEFAULT_RESULTS, 'description': 'Maximum number of results to return.'}, 'code_only': {'type': 'boolean', 'default': False, 'description': 'Skip documentation/config files; search source code only.'}, 'files_only': {'type': 'boolean', 'default': False, 'description': 'Return matching file paths only, without snippets.'}, 'include': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Glob patterns a file must match, e.g. "*.py", "src/**/*.rs".'}, 'exclude': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Glob patterns of files to exclude, e.g. "*.test.ts".'}, 'exclude_dir': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Directory names to exclude, e.g. "vendor", "node_modules".'}}, 'required': ['path', 'query']}
    output_schema = {'type': 'object', 'properties': {'results': {'type': 'array', 'items': {'type': 'object'}, 'description': 'Result objects as produced by `colgrep'}}, 'required': ['results']}
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`colgrep`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = colgrep(path=args['path'], query=args['query'], results=args.get('results', _DEFAULT_RESULTS), code_only=args.get('code_only', False), files_only=args.get('files_only', False), include=args.get('include') or [], exclude=args.get('exclude') or [], exclude_dir=args.get('exclude_dir') or [])
        except ColgrepError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        return ToolResult(structured_content={'results': result.results})

def register_colgrep_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ColgrepTool())
    functions.register(colgrep)