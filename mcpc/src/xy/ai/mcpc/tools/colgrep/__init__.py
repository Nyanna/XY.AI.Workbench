"""colgrep tool - semantic + keyword code search over a pre-built colgrep index.

Search-only wrapper around the `colgrep` CLI. Never creates, initializes or
otherwise modifies an index; that remains the user's responsibility (see the
`colgrep-init.sh` setup script). Given a directory, the tool climbs up through
parent directories until it finds a colgrep index (built with
XDG_DATA_HOME/XDG_CONFIG_HOME pointed at the project root) and runs the
search from there, scoped back to the originally requested directory.
"""
from __future__ import annotations
import json
import os
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any
from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

__all__ = [
    "ColgrepError",
    "ColgrepResult",
    "colgrep_search",
    "ColgrepTool",
    "register_colgrep_tool",
]

_COLGREP_BIN = '/home/user/.cargo/bin/colgrep'
_CONTEXT_LINES = '2'
_DEFAULT_RESULTS = 15
_MAX_RESULTS = 50
_MAX_CODE_LEN = 100
_DROPPED_KEYS = frozenset({'language', 'signature', 'qualified_name', 'unit_type', 'complexity', 'has_loops', 'has_branches', 'has_error_handling', 'extends', 'parent_class', 'variables', 'name', 'return_type', 'calls', 'imports', 'parameters'})


class ColgrepError(Exception):
    """Raised when a colgrep search cannot be performed."""


@dataclass(frozen=True)
class ColgrepResult:
    results: list[Any] = field(default_factory=list)
    count: int = 0


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
            if key == 'code' and isinstance(item, str) and len(item) > _MAX_CODE_LEN:
                item = item[:_MAX_CODE_LEN]
            cleaned_item = _clean_result(item)
            if cleaned_item is False or cleaned_item == '' or cleaned_item is None or cleaned_item == []:
                continue
            cleaned[key] = cleaned_item
        return cleaned
    if isinstance(value, list):
        return [_clean_result(item) for item in value]
    return value


def colgrep_search(
    path: str,
    query: str,
    results: int = _DEFAULT_RESULTS,
    semantic_only: bool = False,
    code_only: bool = False,
    files_only: bool = False,
    full_content: bool = False,
    include: list[str] | None = None,
    exclude: list[str] | None = None,
    exclude_dir: list[str] | None = None,
) -> ColgrepResult:
    """Search the colgrep index covering ``path`` for ``query``."""
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
    if files_only and full_content:
        raise ColgrepError('files_only and full_content are mutually exclusive.')
    if not 1 <= results <= _MAX_RESULTS:
        raise ColgrepError(f'results must be between 1 and {_MAX_RESULTS}.')

    search_dir = search_path.resolve()
    index_root = _find_index_root(search_dir)
    if index_root is None:
        raise ColgrepError('No colgrep index found for this directory or any parent directory.')

    cmd = [_COLGREP_BIN, query, str(search_dir), '--json', '-n', _CONTEXT_LINES, '-k', str(results)]
    if files_only:
        cmd.append('-l')
    if full_content:
        cmd.append('-c')
    if code_only:
        cmd.append('--code-only')
    if semantic_only:
        cmd.append('--semantic-only')
    for pattern in include:
        cmd.append(f'--include={pattern}')
    for pattern in exclude:
        cmd.append(f'--exclude={pattern}')
    for name in exclude_dir:
        cmd.append(f'--exclude-dir={name}')

    env = dict(os.environ)
    env['XDG_DATA_HOME'] = str(index_root)
    env['XDG_CONFIG_HOME'] = str(index_root)

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
        return ColgrepResult(results=parsed, count=len(parsed))
    return ColgrepResult(results=[parsed], count=1)


class ColgrepTool(ToolDefinition):
    name = 'colgrep'
    title = 'Search code with colgrep'
    description = "Search a project's codebase with colgrep."
    input_schema = {
        'type': 'object',
        'properties': {
            'path': {'type': 'string', 'description': 'Absolute directory to search in.'},
            'query': {'type': 'string', 'description': 'Search query: natural language and/or identifiers/keywords.'},
            'results': {'type': 'integer', 'minimum': 1, 'maximum': _MAX_RESULTS, 'default': _DEFAULT_RESULTS, 'description': 'Maximum number of results to return.'},
            'semantic_only': {'type': 'boolean', 'default': False, 'description': 'Disable keyword fusion; pure semantic ranking only.'},
            'code_only': {'type': 'boolean', 'default': False, 'description': 'Skip documentation/config files; search source code only.'},
            'files_only': {'type': 'boolean', 'default': False, 'description': 'Return matching file paths only, without snippets.'},
            'full_content': {'type': 'boolean', 'default': False, 'description': 'Return the full matched function/class body instead of a short snippet.'},
            'include': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Glob patterns a file must match, e.g. "*.py", "src/**/*.rs".'},
            'exclude': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Glob patterns of files to exclude, e.g. "*.test.ts".'},
            'exclude_dir': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Directory names to exclude, e.g. "vendor", "node_modules".'},
        },
        'required': ['path', 'query']
    }
    output_schema = {
        'type': 'object',
        'properties': {
            'results': {'type': 'array', 'items': {'type': 'object'}, 'description': 'Result objects as produced by `colgrep'},
            'count': {'type': 'integer'}
        },
        'required': ['results']
    }
    annotations = {'readOnlyHint': True, 'openWorldHint': False}

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`colgrep_search`, translating the MCP schema to/from the Python API."""
        args: dict[str, Any] = ctx.arguments
        try:
            result = colgrep_search(
                path=args['path'],
                query=args['query'],
                results=args.get('results', _DEFAULT_RESULTS),
                semantic_only=args.get('semantic_only', False),
                code_only=args.get('code_only', False),
                files_only=args.get('files_only', False),
                full_content=args.get('full_content', False),
                include=args.get('include') or [],
                exclude=args.get('exclude') or [],
                exclude_dir=args.get('exclude_dir') or [],
            )
        except ColgrepError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)

        return ToolResult(structured_content={'results': result.results, 'count': result.count})


def register_colgrep_tool(registry: ToolRegistry) -> None:
    registry.register(ColgrepTool())
