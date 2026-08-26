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
from pathlib import Path
from typing import Any
from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content

_COLGREP_BIN = '/home/user/.cargo/bin/colgrep'
_CONTEXT_LINES = '2'
_DEFAULT_RESULTS = 15
_MAX_RESULTS = 50
_MAX_CODE_LEN = 100
_DROPPED_KEYS = frozenset({'language', 'signature', 'qualified_name', 'unit_type', 'complexity', 'has_loops', 'has_branches', 'has_error_handling', 'extends', 'parent_class', 'variables', 'name', 'return_type', 'calls', 'imports', 'parameters'})


def _find_index_root(start: Path) -> Path | None:
    """Climb from *start* up to the filesystem root looking for a colgrep index.

    A directory ``D`` is considered a colgrep project root if
    ``D/colgrep/indices`` exists and is non-empty - the layout produced when
    colgrep is run with ``XDG_DATA_HOME=D`` (see ``colgrep-init.sh``).
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
        args: dict[str, Any] = ctx.arguments
        path_str: str = args['path']
        query: str = args['query']
        results: int = args.get('results', _DEFAULT_RESULTS)
        semantic_only: bool = args.get('semantic_only', False)
        code_only: bool = args.get('code_only', False)
        files_only: bool = args.get('files_only', False)
        full_content: bool = args.get('full_content', False)
        include: list[str] = args.get('include') or []
        exclude: list[str] = args.get('exclude') or []
        exclude_dir: list[str] = args.get('exclude_dir') or []
        if not query.strip():
            return ToolResult(content=[text_content('query must not be empty.')], is_error=True)
        path = Path(path_str)
        if not path.is_absolute():
            return ToolResult(content=[text_content('path must be an absolute path.')], is_error=True)
        if not path.is_dir():
            return ToolResult(content=[text_content('Directory not found.')], is_error=True)
        if files_only and full_content:
            return ToolResult(content=[text_content('files_only and full_content are mutually exclusive.')], is_error=True)
        if not 1 <= results <= _MAX_RESULTS:
            return ToolResult(content=[text_content(f'results must be between 1 and {_MAX_RESULTS}.')], is_error=True)
        search_dir = path.resolve()
        index_root = _find_index_root(search_dir)
        if index_root is None:
            return ToolResult(content=[text_content('No colgrep index found for this directory or any parent directory.')], is_error=True)
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
            return ToolResult(content=[text_content(f'Failed to launch colgrep: {exc}')], is_error=True)
        if proc.returncode != 0:
            message = proc.stderr.strip() or proc.stdout.strip() or f'colgrep exited with code {proc.returncode}.'
            return ToolResult(content=[text_content(message)], is_error=True)
        try:
            parsed = json.loads(proc.stdout) if proc.stdout.strip() else []
        except json.JSONDecodeError:
            return ToolResult(content=[text_content('colgrep returned output that could not be parsed as JSON.')], is_error=True)
        parsed = _clean_result(parsed)
        payload = {'results': parsed, 'count': len(parsed)} if isinstance(parsed, list) else {'results': [parsed], 'count': 1}
        return ToolResult(structured_content=payload)


def register_colgrep_tool(registry: ToolRegistry) -> None:
    registry.register(ColgrepTool())
