"""Bash tool – executes shell scripts, in parallel batches, each inside its own working directory."""
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools._tool_helpers import batch_schema, require_items
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['BashError', 'BashItem', 'BashItemError', 'bash', 'BashTool', 'register_bash_tool']
_MAX_STREAM_CHARS = 3000

class BashError(Exception):
    """Raised when a Bash script cannot be executed."""

@dataclass(frozen=True)
class BashItem:
    """One Bash run.

    Attributes:
        cwd: Absolute path to working directory (must exist and be a directory).
        script: Commands executed one after another in the same bash process.
    """
    cwd: str
    script: list[str]

@dataclass(frozen=True)
class BashItemError:
    """Error running a single item."""
    error: str

def _bash_one(item: BashItem) -> ProcessResult:
    cwd_path = Path(item.cwd)
    if not cwd_path.is_absolute():
        raise BashError('cwd must be an absolute path.')
    if not cwd_path.is_dir():
        raise BashError('Working directory not found or not a directory.')
    if not item.script:
        raise BashError("'script' must be a non-empty list.")
    try:
        return run_process(['bash', '-c', '\n'.join(item.script)], cwd=cwd_path)
    except LaunchError as exc:
        raise BashError(f'Failed to launch bash: {exc}') from exc

def _bash_one_safe(item: BashItem | dict[str, Any]) -> ProcessResult | BashItemError:
    try:
        if isinstance(item, dict):
            try:
                item = BashItem(cwd=item['cwd'], script=item['script'])
            except KeyError as exc:
                raise BashError(f'Missing item key: {exc}') from exc
        return _bash_one(item)
    except BashError as exc:
        return BashItemError(error=str(exc))

def bash(items: list[BashItem | dict[str, Any]]) -> list[ProcessResult | BashItemError]:
    """Run each item with ``bash -c`` in its own working directory, in parallel.

    Args:
        items: Runs to execute (BashItem or dict with the same keys). Must be non-empty. Every item has its own absolute ``cwd``
            (must exist and be a directory) and a ``script`` list whose commands run
            sequentially in one bash process, as if separated by newlines.

    Returns:
        One entry per item, in input order: a ProcessResult with
            exit_code: Exit code of bash process.
            stdout: Combined standard output of all commands.
            stderr: Combined standard error output of all commands.
        or a BashItemError if the item could not be run (invalid cwd, empty script, launch failure).

    Raises:
        BashError: If items is empty.
    """
    if not items:
        raise BashError("'items' must be a non-empty list.")
    with ThreadPoolExecutor(max_workers=len(items)) as pool:
        return list(pool.map(_bash_one_safe, items))

class BashTool(ToolDefinition):
    name = 'bash'
    title = 'Run Bash scripts'
    description = f"Execute Bash scripts for a batch of items. Items run in parallel; Result holds exit code, stdout and stderr, or an error. Reduce the resulting token amount by filtering (grep, head, tail, wc -l). STDOUT and STDERR each have an output limit of {_MAX_STREAM_CHARS} characters per item."
    _ITEM_PROPERTIES = {
        'cwd': {
            'type': 'string',
            'description': 'Absolute path to the working directory.'},
        'script': {
            'type': 'array',
            'minItems': 1,
            'items': {
                    'type': 'string'},
            'description': 'Bash commands executed sequentially in the same process.'}}
    _ITEM_REQUIRED = ['cwd', 'script']
    _ITEMS_DESCRIPTION = 'Bash runs, executed in parallel.'
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, _ITEMS_DESCRIPTION)

    def handle(self, ctx: ToolContext) -> ToolResult:
        """Delegate to :func:`bash` and pack each result into the MCP output schema."""
        raw_items, error = require_items(ctx)
        if error is not None:
            return error
        items: list[BashItem] = []
        for it in raw_items:
            try:
                script = it['script']
                if not isinstance(script, list) or not all((isinstance(s, str) for s in script)):
                    raise TypeError("'script' must be a list of strings")
                items.append(BashItem(cwd=it['cwd'], script=script))
            except (KeyError, TypeError) as exc:
                return ToolResult(content=[text_content(f'Invalid item: {exc!r}')], is_error=True)
        try:
            outcomes = bash(items)
        except BashError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        results: list[dict[str, Any]] = []
        content: list[dict[str, Any]] = []
        for outcome in outcomes:
            if isinstance(outcome, BashItemError):
                results.append({'error': outcome.error})
                continue
            packed = pack_process_result(
                outcome,
                ctx,
                normalize_output=True,
                omit_zero_exit_code=True,
                max_stream_chars=_MAX_STREAM_CHARS)
            results.append(packed.structured_content or {})
            content.extend(packed.content or [])
        return ToolResult(content=content, structured_content={'results': results})

def register_bash_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(BashTool())
    functions.register(bash)