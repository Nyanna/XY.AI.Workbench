"""Run-command tool – executes predefined, directory-specific commands from ``commands.json`` by ID, in parallel batches."""
import json
import re
import shlex
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any
from xy.ai.mcpc.tools._tool_helpers import batch_schema, require_items
from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
__all__ = [
    'COMMANDS_FILE',
    'RunCommandError',
    'CommandSpec',
    'RunCommandItem',
    'RunCommandItemError',
    'load_commands',
    'render_command',
    'run_command',
    'RunCommandTool',
    'register_run_command_tool']
COMMANDS_FILE = 'commands.json'
_MAX_STREAM_CHARS = 3000
_PLACEHOLDER = re.compile('\\{\\{\\s*(\\w+)\\s*\\}\\}')

class RunCommandError(Exception):
    """Raised when a command cannot be resolved or executed."""

@dataclass(frozen=True)
class CommandSpec:
    """One entry of ``commands.json``.

    File format; an entry is either a script string or an object with per-command options::

        {
          "spill_to_disk": true,
          "commands": {
            "runTests": {"script": "cargo test", "spill_to_disk": false},
            "readFile": "cat {{file}}"
          }
        }

    Attributes:
        script: Bash script with optional ``{{name}}`` placeholders.
        spill_to_disk: If false, oversized output is returned inline instead of being written to a file.
            Falls back to the top-level ``spill_to_disk`` (default true) when not set on the entry.
    """
    script: str
    spill_to_disk: bool = True

@dataclass(frozen=True)
class RunCommandItem:
    """One command run.

    Attributes:
        cwd: Absolute path to the working directory containing ``commands.json``.
        id: Command ID defined in ``commands.json``.
        values: Placeholder name to replacement value.
    """
    cwd: str
    id: str
    values: dict[str, str] = field(default_factory=dict)

@dataclass(frozen=True)
class RunCommandItemError:
    """Error running a single item."""
    error: str

def _parse_spec(key: str, raw: Any, default_spill: bool) -> CommandSpec:
    if isinstance(raw, str):
        return CommandSpec(script=raw, spill_to_disk=default_spill)
    if isinstance(raw, dict):
        script = raw.get('script')
        spill = raw.get('spill_to_disk', default_spill)
        if isinstance(script, str) and isinstance(spill, bool):
            return CommandSpec(script=script, spill_to_disk=spill)
    raise RunCommandError(
        f"'{COMMANDS_FILE}': command '{key}' must be a string or an object with 'script' (string) and optional 'spill_to_disk' (boolean).")

def load_commands(cwd: Path) -> dict[str, CommandSpec]:
    path = cwd / COMMANDS_FILE
    if not path.is_file():
        raise RunCommandError(f"'{COMMANDS_FILE}' not found in working directory.")
    try:
        data = json.loads(path.read_text(encoding='utf-8'))
    except (OSError, ValueError) as exc:
        raise RunCommandError(f"Failed to read '{COMMANDS_FILE}': {exc}") from exc
    if not isinstance(data, dict):
        raise RunCommandError(f"'{COMMANDS_FILE}' must contain a JSON object.")
    commands = data.get('commands')
    if not isinstance(commands, dict):
        raise RunCommandError(f"'{COMMANDS_FILE}': 'commands' must be an object mapping IDs to commands.")
    default_spill = data.get('spill_to_disk', True)
    if not isinstance(default_spill, bool):
        raise RunCommandError(f"'{COMMANDS_FILE}': 'spill_to_disk' must be a boolean.")
    return {key: _parse_spec(key, raw, default_spill) for key, raw in commands.items()}

def render_command(script: str, values: dict[str, str]) -> str:
    """Replace ``{{name}}`` placeholders with shell-quoted values (single pass)."""
    required = set(_PLACEHOLDER.findall(script))
    missing = required - values.keys()
    if missing:
        raise RunCommandError(f'Missing values for placeholders: {sorted(missing)}')
    unused = values.keys() - required
    if unused:
        raise RunCommandError(f'Unknown placeholders: {sorted(unused)}')
    return _PLACEHOLDER.sub(lambda m: shlex.quote(values[m.group(1)]), script)

def _run_one(item: RunCommandItem) -> tuple[ProcessResult, CommandSpec]:
    cwd_path = Path(item.cwd)
    if not cwd_path.is_absolute():
        raise RunCommandError('cwd must be an absolute path.')
    if not cwd_path.is_dir():
        raise RunCommandError('Working directory not found or not a directory.')
    spec = load_commands(cwd_path).get(item.id)
    if spec is None:
        raise RunCommandError(f"Unknown command id '{item.id}'.")
    script = render_command(spec.script, item.values)
    try:
        return (run_process(['bash', '-c', script], cwd=cwd_path), spec)
    except LaunchError as exc:
        raise RunCommandError(f'Failed to launch bash: {exc}') from exc

def _run_one_safe(item: RunCommandItem | dict[str, Any]) -> tuple[ProcessResult, CommandSpec] | RunCommandItemError:
    try:
        if isinstance(item, dict):
            try:
                item = RunCommandItem(cwd=item['cwd'], id=item['id'], values=item.get('values') or {})
            except KeyError as exc:
                raise RunCommandError(f'Missing item key: {exc}') from exc
        return _run_one(item)
    except RunCommandError as exc:
        return RunCommandItemError(error=str(exc))

def run_command(items: list[RunCommandItem | dict[str, Any]]) -> list[tuple[ProcessResult, CommandSpec] | RunCommandItemError]:
    """Run each command from its directory's ``commands.json`` with ``bash -c``, in parallel.

    Args:
        items: Runs to execute (RunCommandItem or dict with the same keys). Must be non-empty.

    Returns:
        One entry per item, in input order: ``(ProcessResult, CommandSpec)`` or a RunCommandItemError
        (invalid cwd, missing/invalid file, unknown id, placeholder mismatch, launch failure).

    Raises:
        RunCommandError: If items is empty.
    """
    if not items:
        raise RunCommandError("'items' must be a non-empty list.")
    with ThreadPoolExecutor(max_workers=len(items)) as pool:
        return list(pool.map(_run_one_safe, items))

class RunCommandTool(ToolDefinition):
    name = 'run_command'
    title = 'Run predefined commands'
    description = f"Execute predefined commands by ID, as configured in '{COMMANDS_FILE}' of each working directory. Items run in parallel; placeholder values are passed as name/value pairs. Result holds exit code, stdout and stderr, or an error."
    _ITEM_PROPERTIES = {
        'cwd': {
            'type': 'string', 'description': f"Absolute path to the working directory containing '{COMMANDS_FILE}'."}, 'id': {
                'type': 'string', 'description': 'ID of the command.'}, 'values': {
                    'type': 'array', 'description': 'Placeholder replacements.', 'items': {
                        'type': 'object', 'properties': {
                            'name': {
                                'type': 'string'}, 'value': {
                                    'type': 'string'}}, 'required': [
                                        'name', 'value']}}}
    _ITEM_REQUIRED = ['cwd', 'id']
    input_schema = batch_schema(_ITEM_PROPERTIES, _ITEM_REQUIRED, 'Command runs, executed in parallel.')

    def handle(self, ctx: ToolContext) -> ToolResult:
        raw_items, error = require_items(ctx)
        if error is not None:
            return error
        items: list[RunCommandItem] = []
        for it in raw_items:
            try:
                values = {v['name']: v['value'] for v in it.get('values') or []}
                items.append(RunCommandItem(cwd=it['cwd'], id=it['id'], values=values))
            except (KeyError, TypeError, AttributeError) as exc:
                return ToolResult(content=[text_content(f'Invalid item: {exc!r}')], is_error=True)
        try:
            outcomes = run_command(items)
        except RunCommandError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        results: list[dict[str, Any]] = []
        content: list[dict[str, Any]] = []
        for outcome in outcomes:
            if isinstance(outcome, RunCommandItemError):
                results.append({'error': outcome.error})
                continue
            result, spec = outcome
            packed = pack_process_result(
                result,
                ctx,
                normalize_output=True,
                omit_zero_exit_code=True,
                max_stream_chars=_MAX_STREAM_CHARS if spec.spill_to_disk else None)
            results.append(packed.structured_content or {})
            content.extend(packed.content or [])
        return ToolResult(content=content, structured_content={'results': results})

def register_run_command_tool(registry: ToolRegistry) -> None:
    registry.register(RunCommandTool())