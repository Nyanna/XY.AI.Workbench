"""``tool_call`` – restricted Python context with on-demand tool injection.

(:mod:`xy.ai.mcpc.tools.ast.script`), but general-purpose: instead of a
file's AST, the sandbox is handed the functions/methods named by ``tool_ids``
(looked up in the ``FunctionRegistry`` instance the tool was registered
with). The namespace persists per session across calls, so a script can stash
objects (including large outputs, see :data:`STREAM_SPILL_THRESHOLD`) for
later reuse.
"""
from __future__ import annotations
import contextlib
import io
from dataclasses import dataclass, field
from typing import Any
from xy.ai.mcpc.tools.registry import ToolDefinition, ToolRegistry, ToolResult, text_content
from xy.ai.mcpc.tools.tool_context import ToolContext
from xy.ai.mcpc.tools.function_registry import FunctionRegistry
__all__ = ['ToolCallError', 'ToolCallExecution', 'fresh_namespace', 'inject_tools', 'run_tool_call', 'ToolCallTool', 'register']
'#: Per-session state key holding the persistent exec namespace (globals dict).'
_NAMESPACE_STATE_KEY = 'tool_call_namespace'
'#: STDOUT/STDERR beyond this many characters is spilled into the persistent'
'#: namespace under a dynamic variable name instead of being returned inline.'
STREAM_SPILL_THRESHOLD = 4000
_SAFE_BUILTINS = {name: getattr(__builtins__, name, None) if not isinstance(__builtins__, dict) else __builtins__.get(name) for name in ('print', 'isinstance', 'issubclass', 'getattr', 'setattr', 'hasattr', 'delattr', 'len', 'list', 'dict', 'set', 'tuple', 'str', 'int', 'float', 'bool', 'enumerate', 'range', 'sorted', 'reversed', 'zip', 'map', 'filter', 'any', 'all', 'min', 'max', 'sum', 'type', 'repr', 'abs', 'round', 'Exception', 'ValueError', 'TypeError', 'KeyError', 'IndexError', 'StopIteration', 'RuntimeError')}

class ToolCallError(Exception):
    """Raised when *tool_ids* cannot be resolved/injected."""

@dataclass(frozen=True)
class ToolCallExecution:
    """Result of :func:`run_tool_call`."""
    stdout: str
    stderr: str
    error: str | None = None

def fresh_namespace() -> dict[str, Any]:
    """Return a new, empty persistent exec namespace (globals dict)."""
    return {'__builtins__': _SAFE_BUILTINS}

def _sanitize_identifier(tool_id: str) -> str:
    chars = [c if c.isalnum() or c == '_' else '_' for c in tool_id]
    ident = ''.join(chars)
    if not ident or ident[0].isdigit():
        ident = f'_{ident}'
    return ident

def inject_tools(functions: FunctionRegistry, namespace: dict[str, Any], tool_ids: list[str]) -> dict[str, str]:
    """Bind every id in *tool_ids* into *namespace* under a valid identifier.

    Args:
        functions: Registry to resolve *tool_ids* against.
        namespace: Persistent exec namespace to mutate.
        tool_ids: Ids of functions registered in *functions*.

    Returns:
        Mapping of the variable name each tool was bound to, to its id.

    Raises:
        ToolCallError: If any id in *tool_ids* is not registered.
    """
    bound: dict[str, str] = {}
    for tool_id in tool_ids:
        entry = functions.get(tool_id)
        if entry is None:
            raise ToolCallError(f'No function registered under id: {tool_id}')
        var_name = _sanitize_identifier(tool_id)
        namespace[var_name] = entry.func
        bound[var_name] = tool_id
    return bound

def _spill(namespace: dict[str, Any], text: str, label: str) -> str:
    """Store *text* under a fresh variable name in *namespace*; return that name."""
    counter = namespace.get('_tool_call_spill_counter', 0) + 1
    namespace['_tool_call_spill_counter'] = counter
    var_name = f'_{label}_spill_{counter}'
    namespace[var_name] = text
    return var_name

def run_tool_call(namespace: dict[str, Any], code: str) -> ToolCallExecution:
    """Execute *code* against the persistent *namespace*, capturing STDOUT/STDERR.

    *namespace* is used as both globals and locals, so assignments made by
    *code* persist in it for later calls. Only ``print``/error output is
    captured; ``code`` may hold arbitrary objects in *namespace* for reuse.

    Args:
        namespace: Persistent exec namespace (see :func:`fresh_namespace`).
        code: Python source to execute; imports are not available (restricted
            builtins only, see ``_SAFE_BUILTINS``).

    Returns:
        ToolCallExecution: Captured STDOUT/STDERR and, if execution failed, an
        error message (STDOUT/STDERR captured up to the point of failure).
    """
    stdout_buf, stderr_buf = (io.StringIO(), io.StringIO())
    error: str | None = None
    try:
        with contextlib.redirect_stdout(stdout_buf), contextlib.redirect_stderr(stderr_buf):
            '# noqa: S102'
            exec(compile(code, '<tool-call>', 'exec'), namespace)
    except SyntaxError as exc:
        error = f'Script syntax error: {exc.msg}'
    except Exception as exc:
        '# noqa: BLE001'
        error = f'Script failed: {type(exc).__name__}: {exc}'
    return ToolCallExecution(stdout=stdout_buf.getvalue(), stderr=stderr_buf.getvalue(), error=error)

class ToolCallTool(ToolDefinition):
    name = 'tool_call'
    title = 'Run a script against injected tools'
    description = f"Run Python code against a restricted, session-persistent context. The context persists across calls in this session."
    input_schema = {'type': 'object', 'properties': {'tool_ids': {'type': 'array', 'items': {'type': 'string'}, 'description': "Ids of functions to inject into 'code' as same-named variables."}, 'code': {'type': 'string', 'description': 'Python script; restricted builtins, no imports.'}}, 'required': ['tool_ids', 'code']}
    output_schema = {'type': 'object', 'properties': {'stdout': {'type': 'string'}, 'stderr': {'type': 'string'}, 'stdout_var': {'type': 'string', 'description': 'Namespace variable holding full STDOUT if it was spilled.'}, 'stderr_var': {'type': 'string', 'description': 'Namespace variable holding full STDERR if it was spilled.'}, 'error': {'type': 'string'}}}
    annotations = {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}

    def __init__(self, functions: FunctionRegistry) -> None:
        self._functions = functions

    def handle(self, ctx: ToolContext) -> ToolResult:
        args: dict[str, Any] = ctx.arguments
        namespace: dict[str, Any] = ctx.session.state.setdefault(_NAMESPACE_STATE_KEY, fresh_namespace())
        try:
            inject_tools(self._functions, namespace, list(args.get('tool_ids', [])))
        except ToolCallError as exc:
            return ToolResult(content=[text_content(str(exc))], is_error=True)
        execution = run_tool_call(namespace, args['code'])
        structured: dict[str, Any] = {}
        notices: list[str] = []
        if len(execution.stdout) > STREAM_SPILL_THRESHOLD:
            var_name = _spill(namespace, execution.stdout, 'stdout')
            structured['stdout_var'] = var_name
            notices.append(f"STDOUT exceeded {STREAM_SPILL_THRESHOLD} characters and was stored as '{var_name}' in the persistent context. Filter it (e.g. slicing, splitlines(), grep-like logic) and print only what's needed via a follow-up tool_call using '{var_name}'.")
        else:
            structured['stdout'] = execution.stdout
        if execution.stderr:
            if len(execution.stderr) > STREAM_SPILL_THRESHOLD:
                var_name = _spill(namespace, execution.stderr, 'stderr')
                structured['stderr_var'] = var_name
                notices.append(f"STDERR exceeded {STREAM_SPILL_THRESHOLD} characters and was stored as '{var_name}' in the persistent context. Filter it and re-print via a follow-up tool_call using '{var_name}'.")
            else:
                structured['stderr'] = execution.stderr
        if execution.error is not None:
            structured['error'] = execution.error
        content = [text_content(n) for n in notices]
        return ToolResult(content=content, structured_content=structured, is_error=execution.error is not None)

def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:
    registry.register(ToolCallTool(functions))