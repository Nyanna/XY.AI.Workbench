Prüfe den Code und API.
Ich möchte die Instantiierung von Agenten auch unabhängig vom MCPC-Server erlauben (dort als Subagents genutzt).
Ziel ist den Use Case einer Batch Verarbeitung mittels Iteration und integrierter Verwendung des MCPC zu ermöglichen.
Das umfasst eine Reihe von Agenten parallel gegen denselben MCPC(Alle Agenten nutzen eine MCPC Instanz mit vorkonfigurierten Tools) zu schalten, als auch Agenten jeweils eine eigens konfigurierte Instanz bereitzustellen (Agenten bekommen Profilspeiche aufgaben und eine spezialisiert konfigurierten MCPC).

- MCPC ist ein MCP Server der einem Agenten fokussierte Tools für die Erfüllung einer Aufgabe bereitstellt (Der Agent ist auf die Verwendung des MCPC konfiguriert).
- Stelle in der Embeddid API ein Interface für den Start und die Verwendung von Agenten bereit

CLI für Subagenten:
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/__init__.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/manager.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/parameters.py`
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/session.py`

Embedded MCPC:
`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py`
SystemInit:  id=68b4dce6-08bc-4bda-a5ff-b28695588b43, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich beginne mit der Analyse der angegebenen Dateien.
Control Request:
```yaml
id: 86ae-1
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/manager.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/parameters.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/session.py
  reason: Überblick über Struktur der Embedded API und der CLI-Subagent-Module
```
/answer 86ae-1 allow
Control Request:
```yaml
id: 86ae-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
      nodes:
      - id: 6XGU1h|V9QjkU
        signature: '''Run an MCPC server inline from a Python script.\n\nTypical batch usage::\n\n  …'
      - id: zTYD23|YL1yml
        signature: from __future__ import annotations
      - id: 7Pdehz|CzgsbJ
        signature: "if TYPE_CHECKING:"
      - id: ToolProvider
        signature: "class ToolProvider(ABC):"
        docstring: Subclass and implement :meth:`tools` to contribute custom tools to a server.
        children:
        - id: ToolProvider.Go7Vy4|1l0D1Q
          signature: '''Subclass and implement :meth:`tools` to contribute custom tools to a server.'''
        - id: ToolProvider.tools
          signature: "@abstractmethod def tools(self, environment: AppEnvironment) -> Iterable[ToolDefinition]:"
          docstring: "Return the tools to register. *environment* gives access to config, sessions, e…"
      - id: FunctionTool
        signature: "class FunctionTool(ToolDefinition):"
        docstring: A tool backed by a plain callable taking a ``ToolContext``.
        children:
        - id: FunctionTool.VwPilR|QMm9kb
          signature: '''A tool backed by a plain callable taking a ``ToolContext``.'''
        - id: FunctionTool.init
          signature: "def __init__(self, name: str, description: str, input_schema: dict[str, Any], handler: ToolHandlerFn, *, title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> None:"
        - id: FunctionTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult | str | dict[str, Any]:"
      - id: EmbeddedMcpc
        signature: "class EmbeddedMcpc:"
        docstring: An MCPC server running on a background thread inside the current process. :para…
        children:
        - id: EmbeddedMcpc.QggrSD|YSNAXv
          signature: '"An MCPC server running on a background thread inside the current process.\n\n …'
        - id: EmbeddedMcpc.init
          signature: "def __init__(self, config: ServerConfig | None=None, *, builtin_tools: bool | Iterable[str]=True, tools: Iterable[ToolDefinition | ToolProvider]=(), auto_approve: bool | None=None, ignore_tools_header: bool | None=None, enable_control: bool=True) -> None:"
        - id: EmbeddedMcpc.add_tool
          signature: "def add_tool(self, tool: ToolDefinition | ToolProvider) -> None:"
          docstring: "Register a tool or provider; before :meth:`start` it is queued, afterwards regi…"
        - id: EmbeddedMcpc.tool
          signature: "def tool(self, name: str, description: str, input_schema: dict[str, Any] | None=None, **kwargs: Any) -> Callable[[ToolHandlerFn], ToolHandlerFn]:"
          docstring: Decorator registering a function ``(ctx) -> result`` as a tool.
        - id: EmbeddedMcpc.register
          signature: "@staticmethod def _register(tool: ToolDefinition | ToolProvider, registry: ToolRegistry, environment: AppEnvironment) -> None:"
        - id: EmbeddedMcpc.register_all
          signature: "def _register_all(self, registry: ToolRegistry, environment: AppEnvironment) -> None:"
        - id: EmbeddedMcpc.start
          signature: "def start(self) -> EmbeddedMcpc:"
        - id: EmbeddedMcpc.stop
          signature: "def stop(self) -> None:"
        - id: EmbeddedMcpc.enter
          signature: "def __enter__(self) -> EmbeddedMcpc:"
        - id: EmbeddedMcpc.exit
          signature: "def __exit__(self, *exc_info: object) -> None:"
        - id: EmbeddedMcpc.server
          signature: "@property def server(self) -> McpHTTPServer:"
        - id: EmbeddedMcpc.url
          signature: "@property def url(self) -> str:"
        - id: EmbeddedMcpc.ws_url
          signature: "@property def ws_url(self) -> str | None:"
        - id: EmbeddedMcpc.tool_names
          signature: "@property def tool_names(self) -> list[str]:"
        - id: EmbeddedMcpc.open_session
          signature: "def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None) -> str:"
          docstring: "Create a session with its own toolset (default: all registered tools). Sessions…"
        - id: EmbeddedMcpc.close_session
          signature: "def close_session(self, session_id: str) -> bool:"
        - id: EmbeddedMcpc.headers
          signature: "def headers(self, session_id: str) -> dict[str, str]:"
          docstring: HTTP headers an agent must send to use *session_id*.
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/__init__.py
      nodes:
      - id: a43gxC|jrmLRh
        signature: '''CLI-session management.\n\nA :class:`CliSessionManager` owns a pool of :class:…'
      - id: iiJehU|MVlTP0
        signature: "from xy.ai.mcpc.cli.manager import CliSessionError, CliSessionManager"
      - id: wLxR0u|0BVBQy
        signature: "__all__ = []"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/manager.py
      nodes:
      - id: O6Q2GY|oWnx6Q
        signature: "\"Central registry that creates, resolves, and expires CLI sessions.\\n\\nThe mana…"
      - id: iVBKNp|x3a0P8
        signature: import threading
      - id: Vtx6DV|1sHaLP
        signature: "__all__ = ['CliSessionError', 'CliSessionManager']"
      - id: CliSessionManager
        signature: "class CliSessionManager:"
        docstring: Thread-safe pool of :class:`CliSession` objects keyed by CLI-session id.
        children:
        - id: CliSessionManager.T95S1L|d7wM0X
          signature: '''Thread-safe pool of :class:`CliSession` objects keyed by CLI-session id.'''
        - id: CliSessionManager.init
          signature: "def __init__(self, *, log_dir: Path | str='.claude/logs', ttl_seconds: float=3600.0, response_timeout: float=24 * 60 * 60.0, launcher: Launcher=default_launcher) -> None:"
        - id: CliSessionManager.E2O0Wa|4cfd0w
          signature: '''# -- requests -----------------------------------------------------------'''
        - id: CliSessionManager.request
          signature: "def request(self, parameters: CliParameters | None=None, *, resume: str | None=None, session_id: str | None=None) -> CliSession:"
          docstring: "Obtain a CLI session, creating a new one unless *resume* is given. When *resume…"
        - id: CliSessionManager.resume
          signature: "def _resume(self, cli_session_id: str) -> CliSession:"
        - id: CliSessionManager.resolve
          signature: "def resolve(self, cli_session_id: str) -> CliSession | None:"
          docstring: Resolve a session by id without validity/expiry side effects.
        - id: CliSessionManager.sweep
          signature: "def _sweep(self) -> None:"
          docstring: Terminate and drop every expired session (caller holds the lock).
        - id: CliSessionManager.sweep_1
          signature: "def sweep(self) -> None:"
        - id: CliSessionManager.shutdown
          signature: "def shutdown(self) -> None:"
          docstring: Terminate every managed CLI process and clear the index.
        - id: CliSessionManager.len
          signature: "def __len__(self) -> int:"
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/parameters.py
      nodes:
      - id: jmlmCO|lPZTCm
        signature: '''Construction of the ``claude`` CLI command line and process environment.\n'''
      - id: qPztP8|2lQH5K
        signature: import os
      - id: Model
        signature: "class Model(str, Enum):"
        docstring: Models the agent tool may request.
      - id: Effort
        signature: "class Effort(str, Enum):"
        docstring: Reasoning-effort levels the agent tool may request.
      - id: CacheMode
        signature: "class CacheMode(str, Enum):"
        docstring: Prompt-cache behaviour of the spawned CLI.
      - id: mnuaiY|JOVate
        signature: "_CACHE_ENV = {CacheMode.DISABLED.value: 'DISABLE_PROMPT_CACHING',"
      - id: CliParameters
        signature: "@dataclass(slots=True) class CliParameters:"
        docstring: Everything needed to (re)start a single CLI process. The MCP-server and tool-ho…
        children:
        - id: CliParameters.wwK6DO|5sKV0x
          signature: '"Everything needed to (re)start a single CLI process.\n\n    The MCP-server and…'
        - id: CliParameters.yTWjXa|CgmiNh
          signature: "cache_mode: str = CacheMode.DEFAULT.value"
        - id: CliParameters.base_url
          signature: "@property def base_url(self) -> str:"
        - id: CliParameters.mcp_url
          signature: "@property def mcp_url(self) -> str:"
        - id: CliParameters.ws_mcp_url
          signature: "@property def ws_mcp_url(self) -> str:"
        - id: CliParameters.hook_url
          signature: "@property def hook_url(self) -> str:"
        - id: CliParameters.permission_hook_url
          signature: "@property def permission_hook_url(self) -> str:"
        - id: CliParameters.build_base_command
          signature: "def build_base_command(self, cli_session_id: str) -> list[str]:"
          docstring: Build the command line shared by initial-start and resume.
        - id: CliParameters.build_command
          signature: "def build_command(self, cli_session_id: str, *, resume: bool) -> list[str]:"
          docstring: Return the full command line for a given CLI session id. ``resume`` selects ``-…
        - id: CliParameters.zHjMoJ|vafbjY
          signature: '''# -- Environment --------------------------------------------------------'''
        - id: CliParameters.build_environment
          signature: "def build_environment(self, env: dict[str, str]) -> dict[str, str]:"
          docstring: Populate *env* (mutated in place) for the CLI process.
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/session.py
      nodes:
      - id: r4U6yD|jAohK1
        signature: '''A single managed CLI process and its stream-json conversation.'''
      - id: gxMglT|GObpRn
        signature: import collections
      - id: UIiKk1|bIaBD3
        signature: "if TYPE_CHECKING:"
      - id: Process
        signature: "class Process(Protocol):"
        docstring: The subset of :class:`subprocess.Popen` the session relies on.
        children:
        - id: Process.DZrCo3|Dhqls2
          signature: '''The subset of :class:`subprocess.Popen` the session relies on.'''
        - id: Process.poll
          signature: "def poll(self) -> int | None:"
        - id: Process.terminate
          signature: "def terminate(self) -> None:"
        - id: Process.kill
          signature: "def kill(self) -> None:"
        - id: Process.wait
          signature: "def wait(self, timeout: float | None=None) -> int:"
      - id: AQJ5Jf|ldBSL7
        signature: "'#: A launcher turns a command line + environment into a running process.  It i…"
      - id: default_launcher
        signature: "def default_launcher(cmd: list[str], env: dict[str, str]) -> Process:"
        docstring: Start a real CLI process wired for line-buffered stream-json stdio.
      - id: CliSessionError
        signature: "class CliSessionError(RuntimeError):"
        docstring: "Raised when a prompt cannot be served (expired, timed out, crashed…)."
      - id: CliResult
        signature: "@dataclass(slots=True) class CliResult:"
        docstring: The outcome of a single prompt/response exchange.
      - id: CliSession
        signature: "class CliSession:"
        docstring: Container around a CLI process that serves prompts over stdin/stdout. The sessi…
        children:
        - id: CliSession.xoB2k1|33EVDu
          signature: '"Container around a CLI process that serves prompts over stdin/stdout.\n\n    T…'
        - id: CliSession.init
          signature: "def __init__(self, manager: 'CliSessionManager', parameters: CliParameters, *, session_id: str | None=None, log_dir: Path | None=None, ttl_seconds: float=3600.0, response_timeout: float=24 * 60 * 60.0, launcher: Launcher=default_launcher) -> None:"
        - id: CliSession.running
          signature: "@property def running(self) -> bool:"
        - id: CliSession.ttl_reference
          signature: "def _ttl_reference(self) -> float:"
          docstring: The latest activity (sent or received) drives the TTL; a fresh session counts f…
        - id: CliSession.is_valid
          signature: "def is_valid(self, *, now: float | None=None) -> bool:"
        - id: CliSession.start
          signature: "def _start(self) -> None:"
        - id: CliSession.read_loop
          signature: "def _read_loop(self, process: Process) -> None:"
        - id: CliSession.drain_stderr
          signature: "def _drain_stderr(self, process: Process) -> None:"
        - id: CliSession.stderr_hint
          signature: "def _stderr_hint(self) -> str:"
        - id: CliSession.terminate
          signature: "def terminate(self) -> None:"
          docstring: Terminate a running CLI process and release its streams.
        - id: CliSession.prompt
          signature: "def prompt(self, text: str) -> CliResult:"
          docstring: Send *text* to the CLI and return its final result. A prompt sent to an invalid…
        - id: CliSession.discard_stale_output
          signature: "def _discard_stale_output(self) -> bool:"
          docstring: Drop events left over from an earlier exchange; return True if the stream ended.
        - id: CliSession.send_user_message
          signature: "def _send_user_message(self, text: str) -> None:"
        - id: CliSession.collect_result
          signature: "def _collect_result(self) -> CliResult:"
        - id: CliSession.oBomwo|yhI5nV
          signature: '''# -- replication --------------------------------------------------------'''
        - id: CliSession.replicate
          signature: "def _replicate(self, direction: str, line: str) -> None:"
        - id: CliSession.summary
          signature: "def summary(self) -> dict[str, Any]:"
      - id: extract_text
        signature: "def _extract_text(message: dict[str, Any]) -> str:"
        docstring: Concatenate text blocks from an assistant stream-json message.
      - id: join_errors
        signature: "def _join_errors(errors: Any) -> str:"
        docstring: Flatten the ``errors`` array of failed ``result`` events.
```
/answer 86ae-2 allow
Control Request:
```yaml
id: 86ae-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/parameters.py
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/manager.py
  reason: "Vollständigen Quelltext lesen, um API und Parameter zu verstehen"
```
/answer 86ae-3 allow
Control Request:
```yaml
id: 86ae-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
      nodes:
      - id: 6XGU1h|V9QjkU
        signature: '''Run an MCPC server inline from a Python script.\n\nTypical batch usage::\n\n  …'
      - id: zTYD23|YL1yml
        signature: from __future__ import annotations
      - id: 7Pdehz|CzgsbJ
        signature: "if TYPE_CHECKING:"
      - id: ToolProvider
        signature: "class ToolProvider(ABC):"
        docstring: Subclass and implement :meth:`tools` to contribute custom tools to a server.
        children:
        - id: ToolProvider.Go7Vy4|1l0D1Q
          signature: '''Subclass and implement :meth:`tools` to contribute custom tools to a server.'''
        - id: ToolProvider.tools
          signature: "@abstractmethod def tools(self, environment: AppEnvironment) -> Iterable[ToolDefinition]:"
          docstring: "Return the tools to register. *environment* gives access to config, sessions, e…"
      - id: FunctionTool
        signature: "class FunctionTool(ToolDefinition):"
        docstring: A tool backed by a plain callable taking a ``ToolContext``.
        children:
        - id: FunctionTool.VwPilR|QMm9kb
          signature: '''A tool backed by a plain callable taking a ``ToolContext``.'''
        - id: FunctionTool.init
          signature: "def __init__(self, name: str, description: str, input_schema: dict[str, Any], handler: ToolHandlerFn, *, title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> None:"
        - id: FunctionTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult | str | dict[str, Any]:"
      - id: EmbeddedMcpc
        signature: "class EmbeddedMcpc:"
        docstring: An MCPC server running on a background thread inside the current process. :para…
        children:
        - id: EmbeddedMcpc.QggrSD|YSNAXv
          signature: '"An MCPC server running on a background thread inside the current process.\n\n …'
        - id: EmbeddedMcpc.init
          signature: "def __init__(self, config: ServerConfig | None=None, *, builtin_tools: bool | Iterable[str]=True, tools: Iterable[ToolDefinition | ToolProvider]=(), auto_approve: bool | None=None, ignore_tools_header: bool | None=None, enable_control: bool=True) -> None:"
        - id: EmbeddedMcpc.add_tool
          signature: "def add_tool(self, tool: ToolDefinition | ToolProvider) -> None:"
          docstring: "Register a tool or provider; before :meth:`start` it is queued, afterwards regi…"
        - id: EmbeddedMcpc.tool
          signature: "def tool(self, name: str, description: str, input_schema: dict[str, Any] | None=None, **kwargs: Any) -> Callable[[ToolHandlerFn], ToolHandlerFn]:"
          docstring: Decorator registering a function ``(ctx) -> result`` as a tool.
        - id: EmbeddedMcpc.register
          signature: "@staticmethod def _register(tool: ToolDefinition | ToolProvider, registry: ToolRegistry, environment: AppEnvironment) -> None:"
        - id: EmbeddedMcpc.register_all
          signature: "def _register_all(self, registry: ToolRegistry, environment: AppEnvironment) -> None:"
        - id: EmbeddedMcpc.start
          signature: "def start(self) -> EmbeddedMcpc:"
        - id: EmbeddedMcpc.stop
          signature: "def stop(self) -> None:"
        - id: EmbeddedMcpc.enter
          signature: "def __enter__(self) -> EmbeddedMcpc:"
        - id: EmbeddedMcpc.exit
          signature: "def __exit__(self, *exc_info: object) -> None:"
        - id: EmbeddedMcpc.server
          signature: "@property def server(self) -> McpHTTPServer:"
        - id: EmbeddedMcpc.url
          signature: "@property def url(self) -> str:"
        - id: EmbeddedMcpc.ws_url
          signature: "@property def ws_url(self) -> str | None:"
        - id: EmbeddedMcpc.tool_names
          signature: "@property def tool_names(self) -> list[str]:"
        - id: EmbeddedMcpc.open_session
          signature: "def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None) -> str:"
          docstring: "Create a session with its own toolset (default: all registered tools). Sessions…"
        - id: EmbeddedMcpc.close_session
          signature: "def close_session(self, session_id: str) -> bool:"
        - id: EmbeddedMcpc.headers
          signature: "def headers(self, session_id: str) -> dict[str, str]:"
          docstring: HTTP headers an agent must send to use *session_id*.
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/parameters.py
      nodes:
      - id: jmlmCO|lPZTCm
        signature: '''Construction of the ``claude`` CLI command line and process environment.\n'''
      - id: qPztP8|2lQH5K
        signature: import os
      - id: Model
        signature: "class Model(str, Enum):"
        docstring: Models the agent tool may request.
      - id: Effort
        signature: "class Effort(str, Enum):"
        docstring: Reasoning-effort levels the agent tool may request.
      - id: CacheMode
        signature: "class CacheMode(str, Enum):"
        docstring: Prompt-cache behaviour of the spawned CLI.
      - id: mnuaiY|JOVate
        signature: "_CACHE_ENV = {CacheMode.DISABLED.value: 'DISABLE_PROMPT_CACHING',"
      - id: CliParameters
        signature: "@dataclass(slots=True) class CliParameters:"
        docstring: Everything needed to (re)start a single CLI process. The MCP-server and tool-ho…
        children:
        - id: CliParameters.wwK6DO|5sKV0x
          signature: '"Everything needed to (re)start a single CLI process.\n\n    The MCP-server and…'
        - id: CliParameters.yTWjXa|CgmiNh
          signature: "cache_mode: str = CacheMode.DEFAULT.value"
        - id: CliParameters.base_url
          signature: "@property def base_url(self) -> str:"
        - id: CliParameters.mcp_url
          signature: "@property def mcp_url(self) -> str:"
        - id: CliParameters.ws_mcp_url
          signature: "@property def ws_mcp_url(self) -> str:"
        - id: CliParameters.hook_url
          signature: "@property def hook_url(self) -> str:"
        - id: CliParameters.permission_hook_url
          signature: "@property def permission_hook_url(self) -> str:"
        - id: CliParameters.build_base_command
          signature: "def build_base_command(self, cli_session_id: str) -> list[str]:"
          docstring: Build the command line shared by initial-start and resume.
        - id: CliParameters.build_command
          signature: "def build_command(self, cli_session_id: str, *, resume: bool) -> list[str]:"
          docstring: Return the full command line for a given CLI session id. ``resume`` selects ``-…
        - id: CliParameters.zHjMoJ|vafbjY
          signature: '''# -- Environment --------------------------------------------------------'''
        - id: CliParameters.build_environment
          signature: "def build_environment(self, env: dict[str, str]) -> dict[str, str]:"
          docstring: Populate *env* (mutated in place) for the CLI process.
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/manager.py
      nodes:
      - id: O6Q2GY|oWnx6Q
        signature: "\"Central registry that creates, resolves, and expires CLI sessions.\\n\\nThe mana…"
      - id: iVBKNp|x3a0P8
        signature: import threading
      - id: Vtx6DV|1sHaLP
        signature: "__all__ = ['CliSessionError', 'CliSessionManager']"
      - id: CliSessionManager
        signature: "class CliSessionManager:"
        docstring: Thread-safe pool of :class:`CliSession` objects keyed by CLI-session id.
        children:
        - id: CliSessionManager.T95S1L|d7wM0X
          signature: '''Thread-safe pool of :class:`CliSession` objects keyed by CLI-session id.'''
        - id: CliSessionManager.init
          signature: "def __init__(self, *, log_dir: Path | str='.claude/logs', ttl_seconds: float=3600.0, response_timeout: float=24 * 60 * 60.0, launcher: Launcher=default_launcher) -> None:"
        - id: CliSessionManager.E2O0Wa|4cfd0w
          signature: '''# -- requests -----------------------------------------------------------'''
        - id: CliSessionManager.request
          signature: "def request(self, parameters: CliParameters | None=None, *, resume: str | None=None, session_id: str | None=None) -> CliSession:"
          docstring: "Obtain a CLI session, creating a new one unless *resume* is given. When *resume…"
        - id: CliSessionManager.resume
          signature: "def _resume(self, cli_session_id: str) -> CliSession:"
        - id: CliSessionManager.resolve
          signature: "def resolve(self, cli_session_id: str) -> CliSession | None:"
          docstring: Resolve a session by id without validity/expiry side effects.
        - id: CliSessionManager.sweep
          signature: "def _sweep(self) -> None:"
          docstring: Terminate and drop every expired session (caller holds the lock).
        - id: CliSessionManager.sweep_1
          signature: "def sweep(self) -> None:"
        - id: CliSessionManager.shutdown
          signature: "def shutdown(self) -> None:"
          docstring: Terminate every managed CLI process and clear the index.
        - id: CliSessionManager.len
          signature: "def __len__(self) -> int:"
```
/answer 86ae-4 allow
ReasoningToken: 29
Control Request:
```yaml
id: 86ae-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
    ids:
    - 6XGU1h|V9QjkU
    - 7Pdehz|CzgsbJ
    - EmbeddedMcpc
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/parameters.py
    ids:
    - Model
    - Effort
    - CacheMode
    - CliParameters
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/manager.py
    ids:
    - O6Q2GY|oWnx6Q
    - CliSessionManager
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/session.py
    ids:
    - CliResult
    - CliSession
  reason: Quelltext der Klassen mit konkreten IDs lesen
```
/answer 86ae-5 allow
Control Request:
```yaml
id: 86ae-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
      nodes:
      - id: 6XGU1h|V9QjkU
        type: statements
        code: "'Run an MCPC server inline from a Python script.\\n\\nTypical batch usage::\\n\\n    class MyTools(ToolProvider):\\n        def tools(self, environment):\\n            return [FunctionTool(\"lookup\", \"Look up a record\", {\"type\": \"object\", \"properties\": {}}, lambda ctx: \"ok\")]\\n\\n    with EmbeddedMcpc(builtin_tools=[\"read\", \"bash\"], tools=[MyTools()], auto_approve=True) as mcpc:\\n        sid = mcpc.open_session(tools={\"lookup\", \"read\"})\\n        # point the agent at mcpc.url with header ``mcpc.headers(sid)``\\n\\nOne instance can be shared by many parallel agents: every agent uses its own\\nsession id (``open_session``) and therefore its own toolset.\\n'"
      - id: 7Pdehz|CzgsbJ
        type: statements
        code: |-
          if TYPE_CHECKING:
              from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
          __all__ = ['EmbeddedMcpc', 'FunctionTool', 'ToolProvider']
          logger = logging.getLogger('xy.ai.mcpc.embedded')
          ToolHandlerFn = Callable[['ToolContext'], 'ToolResult | str | dict[str, Any]']
      - id: EmbeddedMcpc
        type: ClassDef
        signature: "class EmbeddedMcpc:"
        docstring: An MCPC server running on a background thread inside the current process. :para…
        children:
        - id: EmbeddedMcpc.QggrSD|YSNAXv
          type: statements
          code: "\"An MCPC server running on a background thread inside the current process.\\n\\n    :param builtin_tools: ``True`` for all built-in tools, ``False`` for none, or\\n        an iterable of tool/alias names to keep.\\n    :param tools: extra :class:`ToolDefinition` instances or :class:`ToolProvider`\\\\ s.\\n    :param auto_approve: approve every tool request and result without a controller.\\n    :param ignore_tools_header: ignore the client's tools header; sessions get all\\n        registered tools unless configured via :meth:`open_session`.\\n    \""
        - id: EmbeddedMcpc.init
          type: FunctionDef
          code: |-
            def __init__(self, config: ServerConfig | None=None, *, builtin_tools: bool | Iterable[str]=True, tools: Iterable[ToolDefinition | ToolProvider]=(), auto_approve: bool | None=None, ignore_tools_header: bool | None=None, enable_control: bool=True) -> None:
                if config is None:
                    config = ServerConfig(host='127.0.0.1', port=0, ws_enabled=True, ws_port=0)
                overrides = {}
                if auto_approve is not None:
                    overrides['auto_approve'] = auto_approve
                if ignore_tools_header is not None:
                    overrides['ignore_tools_header'] = ignore_tools_header
                self.config = config.with_overrides(**overrides) if overrides else config
                self._builtin_tools = builtin_tools if isinstance(builtin_tools, bool) else frozenset(builtin_tools)
                self._tools: list[ToolDefinition | ToolProvider] = list(tools)
                self._enable_control = enable_control
                self._server: McpHTTPServer | None = None
                self._ws_server = None
                self._thread: threading.Thread | None = None
        - id: EmbeddedMcpc.add_tool
          type: FunctionDef
          code: |-
            def add_tool(self, tool: ToolDefinition | ToolProvider) -> None:
                """Register a tool or provider; before :meth:`start` it is queued, afterwards registered live."""
                if self._server is None:
                    self._tools.append(tool)
                    return
                self._register(tool, self._server.environment.registry, self._server.environment)
        - id: EmbeddedMcpc.tool
          type: FunctionDef
          code: |-
            def tool(self, name: str, description: str, input_schema: dict[str, Any] | None=None, **kwargs: Any) -> Callable[[ToolHandlerFn], ToolHandlerFn]:
                """Decorator registering a function ``(ctx) -> result`` as a tool."""
                schema = input_schema if input_schema is not None else {'type': 'object', 'properties': {}}

                def decorator(fn: ToolHandlerFn) -> ToolHandlerFn:
                    self.add_tool(FunctionTool(name, description, schema, fn, **kwargs))
                    return fn
                return decorator
        - id: EmbeddedMcpc.register
          type: FunctionDef
          code: |-
            @staticmethod
            def _register(tool: ToolDefinition | ToolProvider, registry: ToolRegistry, environment: AppEnvironment) -> None:
                if isinstance(tool, ToolProvider):
                    for definition in tool.tools(environment):
                        registry.register(definition)
                else:
                    registry.register(tool)
        - id: EmbeddedMcpc.register_all
          type: FunctionDef
          code: |-
            def _register_all(self, registry: ToolRegistry, environment: AppEnvironment) -> None:
                for tool in self._tools:
                    self._register(tool, registry, environment)
                self._tools.clear()
        - id: EmbeddedMcpc.start
          type: FunctionDef
          code: |-
            def start(self) -> EmbeddedMcpc:
                if self._server is not None:
                    raise RuntimeError('Server already started')
                server = build_server(
                    self.config,
                    enable_control=self._enable_control,
                    builtin_tools=self._builtin_tools,
                    extra_tools=self._register_all)
                self._server = server
                self._ws_server = build_ws_server(server)
                if self._ws_server is not None:
                    self._ws_server.start()
                self._thread = threading.Thread(
                    target=server.serve_forever,
                    kwargs={
                        'poll_interval': 0.1},
                    name='embedded-mcpc',
                    daemon=True)
                self._thread.start()
                logger.info('Embedded MCPC listening on %s', server.endpoint_url)
                return self
        - id: EmbeddedMcpc.stop
          type: FunctionDef
          code: |-
            def stop(self) -> None:
                server = self._server
                if server is None:
                    return
                self._server = None
                if self._ws_server is not None:
                    self._ws_server.stop()
                    self._ws_server = None
                server.environment.cli_manager.shutdown()
                server.shutdown()
                server.server_close()
                if self._thread is not None:
                    self._thread.join(timeout=5)
                    self._thread = None
        - id: EmbeddedMcpc.enter
          type: FunctionDef
          code: |-
            def __enter__(self) -> EmbeddedMcpc:
                return self.start()
        - id: EmbeddedMcpc.exit
          type: FunctionDef
          code: |-
            def __exit__(self, *exc_info: object) -> None:
                self.stop()
        - id: EmbeddedMcpc.server
          type: FunctionDef
          code: |-
            @property
            def server(self) -> McpHTTPServer:
                if self._server is None:
                    raise RuntimeError('Server not started')
                return self._server
        - id: EmbeddedMcpc.url
          type: FunctionDef
          code: |-
            @property
            def url(self) -> str:
                return self.server.endpoint_url
        - id: EmbeddedMcpc.ws_url
          type: FunctionDef
          code: |-
            @property
            def ws_url(self) -> str | None:
                return self._ws_server.endpoint_url if self._ws_server is not None else None
        - id: EmbeddedMcpc.tool_names
          type: FunctionDef
          code: |-
            @property
            def tool_names(self) -> list[str]:
                return self.server.environment.registry.names()
        - id: EmbeddedMcpc.open_session
          type: FunctionDef
          code: |-
            def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None) -> str:
                """Create a session with its own toolset (default: all registered tools).

                    Sessions configured here are never overridden by the client's tools header.
                    """
                sid = session_id or str(uuid.uuid4())
                names = {'all'} if tools is None else set(tools)
                session = self.server.sessions.precreate(sid, enabled_tools=names, cc_profile='none')
                session.state['tools_preset'] = True
                return sid
        - id: EmbeddedMcpc.close_session
          type: FunctionDef
          code: |-
            def close_session(self, session_id: str) -> bool:
                return self.server.sessions.remove(session_id)
        - id: EmbeddedMcpc.headers
          type: FunctionDef
          code: |-
            def headers(self, session_id: str) -> dict[str, str]:
                """HTTP headers an agent must send to use *session_id*."""
                return {self.config.session_header: session_id}
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/parameters.py
      nodes:
      - id: Model
        type: ClassDef
        code: |-
          class Model(str, Enum):
              """Models the agent tool may request."""
              HAIKU = 'haiku'
              SONNET = 'sonnet'
              OPUS = 'opus'
      - id: Effort
        type: ClassDef
        code: |-
          class Effort(str, Enum):
              """Reasoning-effort levels the agent tool may request."""
              DISABLED = 'disabled'
              LOW = 'low'
              MEDIUM = 'medium'
              HIGH = 'high'
              XHIGH = 'xhigh'
              MAX = 'max'
      - id: CacheMode
        type: ClassDef
        code: |-
          class CacheMode(str, Enum):
              """Prompt-cache behaviour of the spawned CLI."""
              DEFAULT = 'default'
              DISABLED = 'disabled'
              MINUTES_5 = '5m'
              HOURS_1 = '1h'
      - id: CliParameters
        type: ClassDef
        signature: "@dataclass(slots=True) class CliParameters:"
        docstring: Everything needed to (re)start a single CLI process. The MCP-server and tool-ho…
        children:
        - id: CliParameters.wwK6DO|5sKV0x
          type: statements
          code: |-
            "Everything needed to (re)start a single CLI process.\n\n    The MCP-server and tool-hook endpoints the spawned CLI connects back to are\n    taken straight from this server's own :class:`ServerConfig` (host, port, MCP\n    endpoint path and hook path).\n    "
            config: ServerConfig
            model: str
            system_prompt: str
            mcpc_session_id: str
            effort: str = Effort.MEDIUM.value
            cc_profile: str = 'none'
            executable: str = 'claude'
            extra_env: dict[str, str] = field(default_factory=dict)
        - id: CliParameters.yTWjXa|CgmiNh
          type: statements
          code: "cache_mode: str = CacheMode.DEFAULT.value"
        - id: CliParameters.base_url
          type: FunctionDef
          code: |-
            @property
            def base_url(self) -> str:
                return f'http://{self.config.host}:{self.config.port}'
        - id: CliParameters.mcp_url
          type: FunctionDef
          code: |-
            @property
            def mcp_url(self) -> str:
                return f'{self.base_url}{self.config.path}'
        - id: CliParameters.ws_mcp_url
          type: FunctionDef
          code: |-
            @property
            def ws_mcp_url(self) -> str:
                config = self.config
                return f'http://{config.resolved_ws_host}:{config.ws_port}{config.ws_path}'
        - id: CliParameters.hook_url
          type: FunctionDef
          code: |-
            @property
            def hook_url(self) -> str:
                return f'{self.base_url}{self.config.hook_path}'
        - id: CliParameters.permission_hook_url
          type: FunctionDef
          code: |-
            @property
            def permission_hook_url(self) -> str:
                return f'{self.base_url}{self.config.permission_hook_path}'
        - id: CliParameters.build_base_command
          type: FunctionDef
          code: |-
            def build_base_command(self, cli_session_id: str) -> list[str]:
                """Build the command line shared by initial-start and resume."""
                session_header = self.config.session_header
                ccprofile_header = self.config.ccprofile_header
                hook_headers = {session_header: cli_session_id}
                settings = {'hooks': {'PreToolUse': [{'hooks': [{'type': 'http',
                                                                 'url': self.hook_url,
                                                                 'headers': hook_headers,
                                                                 'timeout': 86400}]}],
                                      'PermissionRequest': [{'hooks': [{'type': 'http',
                                                                        'url': self.permission_hook_url,
                                                                        'headers': hook_headers,
                                                                        'timeout': 86400}]}]}}
                mcp_config = {
                    'mcpServers': {
                        'mcpc': {
                            'type': 'ws',
                            'url': self.ws_mcp_url,
                            'timeout': 86400000,
                            'alwaysLoad': True,
                            'headers': {
                                session_header: cli_session_id,
                                ccprofile_header: self.cc_profile}}}}
                cmd = [
                    self.executable,
                    '--system-prompt',
                    self.system_prompt,
                    '--tools',
                    '',
                    '--settings',
                    JsonCodec.encode(settings),
                    '--mcp-config',
                    JsonCodec.encode(mcp_config),
                    '--verbose',
                    '--input-format',
                    'stream-json',
                    '--output-format',
                    'stream-json',
                    '--model',
                    self.model]
                if self.effort != Effort.DISABLED.value:
                    cmd += ['--effort', self.effort]
                '# Until interactive permission prompts are handled, skip them.'
                cmd.append('--dangerously-skip-permissions')
                return cmd
        - id: CliParameters.build_command
          type: FunctionDef
          code: |-
            def build_command(self, cli_session_id: str, *, resume: bool) -> list[str]:
                """Return the full command line for a given CLI session id.

                    ``resume`` selects ``--resume`` over ``--session-id`` to reattach to an
                    existing CLI-side session while reusing its prompt cache.
                    """
                cmd = self.build_base_command(cli_session_id)
                cmd.append('--resume' if resume else '--session-id')
                cmd.append(cli_session_id)
                return cmd
        - id: CliParameters.zHjMoJ|vafbjY
          type: statements
          code: '''# -- Environment --------------------------------------------------------'''
        - id: CliParameters.build_environment
          type: FunctionDef
          code: |-
            def build_environment(self, env: dict[str, str]) -> dict[str, str]:
                """Populate *env* (mutated in place) for the CLI process."""
                home = os.path.expanduser('~')
                env['CLAUDE_CONFIG_DIR'] = f'{home}/.claude-{self.cc_profile}'
                env[MCPC_SESSION_ENV] = self.mcpc_session_id
                env['CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS'] = '1'
                env['CLAUDE_CODE_DISABLE_SPELLCHECK'] = 'true'
                '# Thinking is only forcibly disabled when reasoning effort is disabled.'
                if self.effort == Effort.DISABLED.value:
                    env['CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING'] = '1'
                    env['MAX_THINKING_TOKENS'] = '0'
                env['CLAUDE_CODE_DISABLE_AGENT_VIEW'] = '1'
                env['CLAUDE_CODE_DISABLE_BACKGROUND_TASKS'] = '1'
                env['CLAUDE_CODE_DISABLE_BUNDLED_SKILLS'] = '1'
                env['CLAUDE_CODE_DISABLE_CLAUDE_MDS'] = '1'
                env['CLAUDE_CODE_DISABLE_CRON'] = '1'
                env['CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS'] = '1'
                env['CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS'] = '1'
                env['CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC'] = '1'
                env['CLAUDE_CODE_DISABLE_POLICY_SKILLS'] = '1'
                env['CLAUDE_CODE_DISABLE_WEB_FETCH'] = '1'
                env['CLAUDE_CODE_DISABLE_WORKFLOWS'] = '1'
                env['CLAUDE_CODE_ENABLE_AWAY_SUMMARY'] = '0'
                env['CLAUDE_CODE_ENABLE_BACKGROUND_PLUGIN_REFRESH'] = '1'
                env['CLAUDE_CODE_FORK_SUBAGENT'] = '0'
                env['CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY'] = '1'
                env['CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS'] = '0'
                env['CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT'] = '0'
                env['MAX_MCP_OUTPUT_TOKENS'] = '250000'
                env['MCP_TOOL_TIMEOUT'] = '86400000'
                env['MCP_TIMEOUT'] = '86400000'
                env['CLAUDE_ENABLE_STREAM_WATCHDOG'] = '0'
                env['CLAUDE_ENABLE_BYTE_WATCHDOG'] = '0'
                env['CLAUDE_STREAM_IDLE_TIMEOUT_MS'] = '86400000'
                env['API_FORCE_IDLE_TIMEOUT'] = '0'
                env['ENABLE_TOOL_SEARCH'] = 'false'
                env['CLAUDE_CODE_DISABLE_ADVISOR_TOOL'] = '1'
                cache_env = _CACHE_ENV.get(self.cache_mode)
                if cache_env is not None:
                    env[cache_env] = '1'
                env.update(self.extra_env)
                return env
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/manager.py
      nodes:
      - id: O6Q2GY|oWnx6Q
        type: statements
        code: "\"Central registry that creates, resolves, and expires CLI sessions.\\n\\nThe manager itself never starts or stops a CLI process — that is the session's\\nresponsibility.  It maintains the index, hands out session objects on request,\\nand, on every request.\\n\""
      - id: CliSessionManager
        type: ClassDef
        signature: "class CliSessionManager:"
        docstring: Thread-safe pool of :class:`CliSession` objects keyed by CLI-session id.
        children:
        - id: CliSessionManager.T95S1L|d7wM0X
          type: statements
          code: '''Thread-safe pool of :class:`CliSession` objects keyed by CLI-session id.'''
        - id: CliSessionManager.init
          type: FunctionDef
          code: |-
            def __init__(self, *, log_dir: Path | str='.claude/logs', ttl_seconds: float=3600.0, response_timeout: float=24 * 60 * 60.0, launcher: Launcher=default_launcher) -> None:
                self.log_dir = Path(log_dir)
                self.ttl_seconds = ttl_seconds
                self.response_timeout = response_timeout
                self._launcher = launcher
                self._sessions: dict[str, CliSession] = {}
                self._lock = threading.RLock()
        - id: CliSessionManager.E2O0Wa|4cfd0w
          type: statements
          code: '''# -- requests -----------------------------------------------------------'''
        - id: CliSessionManager.request
          type: FunctionDef
          code: |-
            def request(self, parameters: CliParameters | None=None, *, resume: str | None=None, session_id: str | None=None) -> CliSession:
                """Obtain a CLI session, creating a new one unless *resume* is given.

                    When *resume* is provided the existing session is looked up and its
                    validity is verified; an unknown or expired session raises
                    :class:`CliSessionError` (after terminating any lingering process).
                    Otherwise a brand new session is created and indexed under *session_id*
                    (a fresh UUID when omitted).
                    """
                with self._lock:
                    self._sweep()
                    if resume is not None:
                        return self._resume(resume)
                    if parameters is None:
                        raise CliSessionError('parameters are required to start a new CLI session')
                    if session_id is not None and session_id in self._sessions:
                        raise CliSessionError(f'CLI session already exists: {session_id}')
                    session = CliSession(
                        self,
                        parameters,
                        session_id=session_id,
                        log_dir=self.log_dir,
                        ttl_seconds=self.ttl_seconds,
                        response_timeout=self.response_timeout,
                        launcher=self._launcher)
                    self._sessions[session.id] = session
                    return session
        - id: CliSessionManager.resume
          type: FunctionDef
          code: |-
            def _resume(self, cli_session_id: str) -> CliSession:
                session = self._sessions.get(cli_session_id)
                if session is None:
                    raise CliSessionError(f'Unknown CLI session: {cli_session_id}')
                if not session.is_valid():
                    session.terminate()
                    self._sessions.pop(cli_session_id, None)
                    raise CliSessionError(f'CLI session {cli_session_id} has expired')
                return session
        - id: CliSessionManager.resolve
          type: FunctionDef
          code: |-
            def resolve(self, cli_session_id: str) -> CliSession | None:
                """Resolve a session by id without validity/expiry side effects."""
                with self._lock:
                    return self._sessions.get(cli_session_id)
        - id: CliSessionManager.sweep
          type: FunctionDef
          code: |-
            def _sweep(self) -> None:
                """Terminate and drop every expired session (caller holds the lock)."""
                expired = [s for s in self._sessions.values() if not s.is_valid()]
                for session in expired:
                    session.terminate()
                    self._sessions.pop(session.id, None)
        - id: CliSessionManager.sweep_1
          type: FunctionDef
          code: |-
            def sweep(self) -> None:
                with self._lock:
                    self._sweep()
        - id: CliSessionManager.shutdown
          type: FunctionDef
          code: |-
            def shutdown(self) -> None:
                """Terminate every managed CLI process and clear the index."""
                with self._lock:
                    for session in list(self._sessions.values()):
                        session.terminate()
                    self._sessions.clear()
        - id: CliSessionManager.len
          type: FunctionDef
          code: |-
            def __len__(self) -> int:
                with self._lock:
                    return len(self._sessions)
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/session.py
      nodes:
      - id: CliResult
        type: ClassDef
        code: |-
          @dataclass(slots=True)
          class CliResult:
              """The outcome of a single prompt/response exchange."""
              text: str
              is_error: bool = False
              subtype: str | None = None
      - id: CliSession
        type: ClassDef
        signature: "class CliSession:"
        docstring: Container around a CLI process that serves prompts over stdin/stdout. The sessi…
        children:
        - id: CliSession.xoB2k1|33EVDu
          type: statements
          code: '"Container around a CLI process that serves prompts over stdin/stdout.\n\n    The session owns the process lifecycle and reuses the CLI''s prompt cache by\n    passing ``--session-id`` on first start and ``--resume`` on restart.\n    "'
        - id: CliSession.init
          type: FunctionDef
          code: |-
            def __init__(self, manager: 'CliSessionManager', parameters: CliParameters, *, session_id: str | None=None, log_dir: Path | None=None, ttl_seconds: float=3600.0, response_timeout: float=24 * 60 * 60.0, launcher: Launcher=default_launcher) -> None:
                self.manager = manager
                self.parameters = parameters
                self.id = session_id or str(uuid.uuid4())
                self.ttl_seconds = ttl_seconds
                self.response_timeout = response_timeout
                self._launcher = launcher
                self._log_path = Path(log_dir) / f'{self.id}.json.log' if log_dir else None
                self.created_at = time.time()
                self.started_at: float | None = None
                self.last_sent_at: float | None = None
                self.last_received_at: float | None = None
                self._process: Process | None = None
                self._started_once = False
                self._reader: threading.Thread | None = None
                self._out: 'queue.Queue[Any]' = queue.Queue()
                self._stderr_tail: collections.deque[str] = collections.deque(maxlen=40)
                self._in_prompt = False
                self._stderr_reader: threading.Thread | None = None
                self._lock = threading.RLock()
                self._log_lock = threading.Lock()
        - id: CliSession.running
          type: FunctionDef
          code: |-
            @property
            def running(self) -> bool:
                return self._process is not None and self._process.poll() is None
        - id: CliSession.ttl_reference
          type: FunctionDef
          code: |-
            def _ttl_reference(self) -> float:
                """The latest activity (sent or received) drives the TTL; a fresh session counts from creation."""
                return max(self.last_sent_at or self.created_at, self.last_received_at or 0.0)
        - id: CliSession.is_valid
          type: FunctionDef
          code: |-
            def is_valid(self, *, now: float | None=None) -> bool:
                if self._in_prompt:
                    return True
                now = time.time() if now is None else now
                return now - self._ttl_reference() <= self.ttl_seconds
        - id: CliSession.start
          type: FunctionDef
          code: |-
            def _start(self) -> None:
                cmd = self.parameters.build_command(self.id, resume=self._started_once)
                env = self.parameters.build_environment(dict(os.environ))
                self._process = self._launcher(cmd, env)
                self.started_at = time.time()
                self._started_once = True
                self._out = queue.Queue()
                self._stderr_tail.clear()
                self._reader = threading.Thread(target=self._read_loop, args=(self._process,), daemon=True)
                self._reader.start()
                if getattr(self._process, 'stderr', None) is not None:
                    self._stderr_reader = threading.Thread(target=self._drain_stderr, args=(self._process,), daemon=True)
                    self._stderr_reader.start()
        - id: CliSession.read_loop
          type: FunctionDef
          code: |-
            def _read_loop(self, process: Process) -> None:
                stdout = process.stdout
                try:
                    for line in iter(stdout.readline, ''):
                        if line == '':
                            break
                        self.last_received_at = time.time()
                        self._replicate('out', line.rstrip('\n'))
                        obj = JsonCodec.decode_line(line)
                        if obj is None:
                            continue
                        self._out.put(obj)
                finally:
                    self._out.put(_EOF)
        - id: CliSession.drain_stderr
          type: FunctionDef
          code: |-
            def _drain_stderr(self, process: Process) -> None:
                try:
                    for line in iter(process.stderr.readline, ''):
                        line = line.rstrip('\n')
                        self._stderr_tail.append(line)
                        self._replicate('err', line)
                except Exception:
                    pass
        - id: CliSession.stderr_hint
          type: FunctionDef
          code: |-
            def _stderr_hint(self) -> str:
                if self._stderr_reader is not None:
                    self._stderr_reader.join(timeout=1)
                tail = '\n'.join(self._stderr_tail)
                return f'\nstderr:\n{tail}' if tail else ''
        - id: CliSession.terminate
          type: FunctionDef
          code: |-
            def terminate(self) -> None:
                """Terminate a running CLI process and release its streams."""
                with self._lock:
                    process = self._process
                    self._process = None
                    if process is None:
                        return
                    if process.poll() is None:
                        try:
                            process.terminate()
                            try:
                                process.wait(timeout=5)
                            except Exception:
                                '# noqa: BLE001 - fall through to kill'
                                process.kill()
                        except Exception:
                            '# noqa: BLE001 - best-effort teardown'
                            pass
                    for stream in (process.stdin, process.stdout, process.stderr):
                        try:
                            if stream is not None:
                                stream.close()
                        except Exception:
                            '# noqa: BLE001'
                            pass
        - id: CliSession.prompt
          type: FunctionDef
          code: |-
            def prompt(self, text: str) -> CliResult:
                """Send *text* to the CLI and return its final result.

                    A prompt sent to an invalid (expired) session terminates any lingering
                    process and raises; otherwise it lazily (re)starts the process, sending
                    ``--resume`` when reattaching to a previously started CLI session.
                    While the prompt is in flight the session never expires, so tool calls
                    blocked by a control agent can wait as long as the response timeout allows.
                    """
                with self._lock:
                    if not self.is_valid():
                        self.terminate()
                        raise CliSessionError(f'CLI session {self.id} has expired')
                    self._in_prompt = True
                    try:
                        if self.running and self._discard_stale_output():
                            self.terminate()
                        if not self.running:
                            self._start()
                        self._send_user_message(text)
                        return self._collect_result()
                    finally:
                        self._in_prompt = False
                        self.last_received_at = time.time()
        - id: CliSession.discard_stale_output
          type: FunctionDef
          code: |-
            def _discard_stale_output(self) -> bool:
                """Drop events left over from an earlier exchange; return True if the stream ended."""
                ended = False
                while True:
                    try:
                        if self._out.get_nowait() is _EOF:
                            ended = True
                    except queue.Empty:
                        return ended
        - id: CliSession.send_user_message
          type: FunctionDef
          code: |-
            def _send_user_message(self, text: str) -> None:
                assert self._process is not None and self._process.stdin is not None
                message = {'type': 'user', 'message': {'role': 'user', 'content': [{'type': 'text', 'text': text}]}}
                self._replicate('in', JsonCodec.encode(message, compact=True))
                try:
                    JsonCodec.write_line(self._process.stdin, message)
                except (BrokenPipeError, ValueError) as exc:
                    raise CliSessionError(f'CLI session {self.id} is not accepting input: {exc}')
                '# A sent prompt resets the remaining lifetime to the full TTL.'
                self.last_sent_at = time.time()
        - id: CliSession.collect_result
          type: FunctionDef
          code: |-
            def _collect_result(self) -> CliResult:
                deadline = time.time() + self.response_timeout
                assistant_text: list[str] = []
                while True:
                    remaining = deadline - time.time()
                    if remaining <= 0:
                        self.terminate()
                        raise CliSessionError(f'CLI session {self.id} timed out after {self.response_timeout}s')
                    try:
                        obj = self._out.get(timeout=remaining)
                    except queue.Empty:
                        self.terminate()
                        raise CliSessionError(f'CLI session {self.id} timed out after {self.response_timeout}s')
                    if obj is _EOF:
                        raise CliSessionError(f'CLI session {self.id} ended before returning a result{self._stderr_hint()}')
                    if not isinstance(obj, dict):
                        continue
                    kind = obj.get('type')
                    if kind == 'assistant':
                        assistant_text.append(_extract_text(obj.get('message', {})))
                    elif kind == 'result':
                        subtype = obj.get('subtype')
                        errors = _join_errors(obj.get('errors'))
                        is_error = bool(obj.get('is_error')) or bool(errors) or (isinstance(subtype, str) and subtype != 'success')
                        text = obj.get('result')
                        if not isinstance(text, str):
                            text = '\n'.join((t for t in assistant_text if t))
                        if errors:
                            text = f'{text}\n{errors}'.strip()
                        return CliResult(text=text, is_error=is_error, subtype=subtype)
        - id: CliSession.oBomwo|yhI5nV
          type: statements
          code: '''# -- replication --------------------------------------------------------'''
        - id: CliSession.replicate
          type: FunctionDef
          code: |-
            def _replicate(self, direction: str, line: str) -> None:
                if self._log_path is None:
                    return
                entry = {'ts': time.time(), 'cliSessionId': self.id, 'direction': direction, 'line': line}
                record = JsonCodec.encode(entry)
                with self._log_lock:
                    self._log_path.parent.mkdir(parents=True, exist_ok=True)
                    with self._log_path.open('a', encoding='utf-8') as fh:
                        fh.write(record + '\n')
        - id: CliSession.summary
          type: FunctionDef
          code: |-
            def summary(self) -> dict[str, Any]:
                return {'id': self.id, 'running': self.running, 'createdAt': self.created_at, 'startedAt': self.started_at,
                        'lastSentAt': self.last_sent_at, 'lastReceivedAt': self.last_received_at, 'valid': self.is_valid()}
```
/answer 86ae-6 allow
ReasoningToken: 157
Control Request:
```yaml
id: 86ae-7
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: def build_server|def build_ws_server|cli_manager|endpoint_url|MCPC_SESSION_ENV\s*=|CliParameters\(
    include:
    - '*.py'
    exclude:
    - tests/**
    limit: 40
  reason: "Klären, wie Ports (port=0) und cli_manager/CliParameters im Server verdrahtet sind"
```
/answer 86ae-7 allow
Control Request:
```yaml
id: 86ae-8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: server/server.py
        matches:
        - "def endpoint_url(self) -> str:"
        - "def build_server(config: ServerConfig | None=None, registry: ToolRegistry | None=None, *, enable_control: bool=True, builtin_tools: 'bool | Iterable[str]'=True, extra_tools: 'Callable[[ToolRegistry, AppEnvironment], None] | None'=None) -> McpHTTPServer:"
        - cli_manager = CliSessionManager(
        - "cli_manager=cli_manager,"
        - "def build_ws_server(server: McpHTTPServer) -> WebSocketMcpServer | None:"
        - "logger.info('MCP Controller listening on %s', server.endpoint_url)"
        - "logger.info('MCP Controller (WebSocket) listening on %s', ws_server.endpoint_url)"
        - server.environment.cli_manager.shutdown()
      - path: server/ws_transport.py
        matches:
        - "def endpoint_url(self) -> str:"
      - path: cli/parameters.py
        matches:
        - MCPC_SESSION_ENV = 'MCPC_SESSION_ID'
      - path: tools/tool_context.py
        matches:
        - "cli_manager: 'CliSessionManager'"
      - path: tools/agent/__init__.py
        matches:
        - params = CliParameters(
        - "cli = services.cli_manager.request(parameters=params, session_id=sub_id)"
        - cli = services.cli_manager.request(resume=resume_id)
      - path: embedded.py
        matches:
        - "logger.info('Embedded MCPC listening on %s', server.endpoint_url)"
        - server.environment.cli_manager.shutdown()
        - return self.server.endpoint_url
        - return self._ws_server.endpoint_url if self._ws_server is not None else None
```
/answer 86ae-8 allow
Control Request:
```yaml
id: 86ae-9
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/agent/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/server.py
  regexp: CliParameters\(|def build_server|def build_ws_server|def endpoint_url|server_address|server_port
  reason: "Sehen, wie der Agent-Tool-Code CliParameters baut und wie reale Ports ermittelt werden"
```
/answer 86ae-9 allow
Control Request:
```yaml
id: 86ae-a
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/agent/__init__.py
      nodes:
      - id: run_agent.7nS8wl|gmSpzm
        code: |-
          sub_id = str(uuid.uuid4())
          cc_profile = ctx.session.cc_profile
          services.sessions.precreate(sub_id, enabled_tools=set(tools), cc_profile=cc_profile)
          params = CliParameters(
              config=services.config,
              model=model,
              system_prompt=system_prompt,
              mcpc_session_id=sub_id,
              effort=effort,
              cc_profile=cc_profile,
              cache_mode=_cache_mode(
                  profile,
                  tools))
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/server.py
      nodes:
      - id: McpHTTPServer.endpoint_url
        code: |-
          @property
          def endpoint_url(self) -> str:
              host, port = (self.server_address[0], self.server_address[1])
              return f'http://{host}:{port}/{self.config.path}'
      - id: build_server
        code: |-
          def build_server(config: ServerConfig | None=None, registry: ToolRegistry | None=None, *, enable_control: bool=True, builtin_tools: 'bool | Iterable[str]'=True, extra_tools: 'Callable[[ToolRegistry, AppEnvironment], None] | None'=None) -> McpHTTPServer:
              """Construct (but do not start) an :class:`McpHTTPServer`.
              """
              logger.debug('Aquiring config')
              config = config or ServerConfig()
              logger.debug('Reading profiles')
              profiles = ProfileRegistry(list(DEFAULT_PROFILES))
              logger.debug('Initialising Session-Store')
              sessions = SessionStore()
              logger.debug('Initialising CLI-Manager')
              cli_manager = CliSessionManager(
                  log_dir=config.cli_log_dir,
                  ttl_seconds=config.agent_session_ttl_seconds,
                  response_timeout=config.agent_response_timeout_seconds)
              control_manager: ToolControlManager | None = None
              if enable_control:
                  logger.debug('Initialising Tool-Control-Manager')
                  control_manager = ToolControlManager(timeout=config.agent_response_timeout_seconds)
              logger.debug('Initialising Tool-Registry')
              registry_given = registry is not None
              if registry is None:
                  registry = ToolRegistry()
              logger.debug('Initialising Function-Registry')
              functions = FunctionRegistry()
              '# The environment is built before tools are registered so registration'
              '# can inject it into the handlers that need it (see register_tools()).'
              environment = AppEnvironment(
                  config=config,
                  registry=registry,
                  functions=functions,
                  sessions=sessions,
                  cli_manager=cli_manager,
                  profiles=profiles,
                  control_manager=control_manager)
              if not registry_given and builtin_tools is not False:
                  if builtin_tools is not True:
                      patterns = tuple(builtin_tools)
                      registry.registration_filter = lambda name: any((fnmatchcase(name, p) for p in patterns))
                  try:
                      register_tools(registry, environment)
                  finally:
                      registry.registration_filter = None
              if extra_tools is not None:
                  extra_tools(registry, environment)
              protocol = McpProtocol(config, registry, environment)
              logger.debug('Initialising Communication-Log')
              comm_log = CommunicationLog(config.log_dir)
              return McpHTTPServer(config, protocol, sessions, comm_log, environment)
      - id: build_ws_server
        code: |-
          def build_ws_server(server: McpHTTPServer) -> WebSocketMcpServer | None:
              """Build the WebSocket transport sharing *server*'s component graph.

              Returns ``None`` (after logging a warning) when the transport is disabled
              via configuration, or when the optional ``websockets`` dependency is not
              installed — the HTTP transport keeps working either way.
              """
              if not server.config.ws_enabled:
                  logger.info('WebSocket transport disabled (ws_enabled=False)')
                  return None
              try:
                  return WebSocketMcpServer(server.config, server.protocol, server.sessions, server.comm_log, server.environment)
              except RuntimeError as exc:
                  logger.warning('WebSocket transport unavailable: %s', exc)
                  return None
```
/answer 86ae-a allow
ReasoningToken: 38
Control Request:
```yaml
id: 86ae-b
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
    pattern: port|config\s*=|replace\(|with_overrides
    include:
    - server.py
    - ws_transport.py
    exclude:
    - tests/**
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: def with_overrides|resolved_ws_host|frozen
    include:
    - config.py
    - '*config*.py'
    exclude:
    - tests/**
  reason: "Prüfen, ob die tatsächlich gebundenen Ports (port=0) in die Config zurückgeschrieben werden"
```
/answer 86ae-b allow
Control Request:
```yaml
id: 86ae-c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
      matches:
      - path: server.py
        matches:
        - import logging
        - import socket
        - from fnmatch import fnmatchcase
        - from http.server import ThreadingHTTPServer
        - "from collections.abc import Callable, Iterable"
        - from typing import Any
        - from xy.ai.mcpc.cli import CliSessionManager
        - from xy.ai.mcpc.config import ServerConfig
        - from xy.ai.mcpc.tools.tool_context import AppEnvironment
        - from xy.ai.mcpc.control import ToolControlManager
        - from xy.ai.mcpc.utils.logging_utils import CommunicationLog
        - from xy.ai.mcpc.server.mcp_protocol import McpProtocol
        - from xy.ai.mcpc.tools import register_tools
        - from xy.ai.mcpc.tools.tool_registry import ToolRegistry
        - from xy.ai.mcpc.tools.function_registry import FunctionRegistry
        - from xy.ai.mcpc.server.session import SessionStore
        - "from xy.ai.mcpc.tools.agent.profiles import DEFAULT_PROFILES, ProfileRegistry"
        - from xy.ai.mcpc.server.http_transport import StreamableHttpHandler
        - from xy.ai.mcpc.server.ws_transport import WebSocketMcpServer
        - self.config = config
        - "super().__init__((config.host, config.port), StreamableHttpHandler)"
        - "host, port = (self.server_address[0], self.server_address[1])"
        - "return f'http://{host}:{port}/{self.config.path}'"
        - config = config or ServerConfig()
        - "config=config,"
        - '"""Build the WebSocket transport sharing *server*''s component graph.'
        - Returns ``None`` (after logging a warning) when the transport is disabled
        - installed — the HTTP transport keeps working either way.
        - logger.info('WebSocket transport disabled (ws_enabled=False)')
        - "logger.warning('WebSocket transport unavailable: %s', exc)"
      - path: ws_transport.py
        matches:
        - '"""WebSocket transport for the MCP endpoint.'
        - "transport (:mod:`.transport`): the very same server-side object graph"
        - is purely a second transport.
        - transport applies them per-request.
        - transport's ``202 Accepted``); a *response* (this server never issues
        - "machinery is needed — this mirrors the Streamable-HTTP transport, which"
        - import asyncio
        - import logging
        - import threading
        - from typing import Any
        - from collections.abc import Sequence
        - "from urllib.parse import parse_qs, urlparse"
        - "from xy.ai.mcpc.server import errors, jsonrpc"
        - from xy.ai.mcpc.server.json_codec import JsonCodec
        - "from xy.ai.mcpc.server.jsonrpc import JsonRpcRequest, MessageKind"
        - "from xy.ai.mcpc.utils.logging_utils import EVENT, IN, OUT"
        - "from xy.ai.mcpc.server.session import Session, is_valid_uuid"
        - "from xy.ai.mcpc.server.http_transport import apply_ccprofile_header, apply_tools_header, is_origin_allowed"
        - from websockets.asyncio.server import Server as _WsServer
        - from websockets.asyncio.server import ServerConnection
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: config.py
        matches:
        - "@dataclass(frozen=True, slots=True)"
        - "def resolved_ws_host(self) -> str:"
        - "def with_overrides(self, **changes) -> 'ServerConfig':"
```
/answer 86ae-c allow
ReasoningToken: 50
Control Request:
```yaml
id: 86ae-d
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
    pattern: \.port|_port|bound|serve\(|sockets
    include:
    - ws_transport.py
    exclude:
    - tests/**
    limit: 30
  reason: "Ermitteln, wie der WS-Server seinen tatsächlichen Port bestimmt"
```
/answer 86ae-d allow
Control Request:
```yaml
id: 86ae-e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
      matches:
      - path: ws_transport.py
        matches:
        - One physical WebSocket connection is bound to exactly one MCP session for
        - Built on the ``websockets`` package (asyncio-based). The server runs its own
        - from websockets.asyncio.server import Server as _WsServer
        - from websockets.asyncio.server import ServerConnection
        - from websockets.asyncio.server import serve as ws_serve
        - from websockets.exceptions import ConnectionClosed
        - '"The ''websockets'' package is required for the WebSocket transport; install it with `pip install websockets`.")'
        - port = self.config.ws_port
        - "if self._server is not None and getattr(self._server, 'sockets', None):"
        - "port = self._server.sockets[0].getsockname()[1]"
        - asyncio.run(self._serve())
        - "async def _serve(self) -> None:"
        - "async with ws_serve(self._handle_connection, self.config.resolved_ws_host, self.config.ws_port, subprotocols=['mcp'], select_subprotocol=_select_subprotocol) as server:"
```
/answer 86ae-e allow
Control Request:
```yaml
id: 86ae-f
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/ws_transport.py
  name: start
  reason: "Prüfen, ob start() bis zum Binden wartet, damit der reale WS-Port verfügbar ist"
```
/answer 86ae-f allow
ReasoningToken: 149
Control Request:
```yaml
id: 86ae-10
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/ws_transport.py
      nodes:
      - id: WebSocketMcpServer.start
        code: |-
          def start(self) -> None:
              """Start the WebSocket server in a background thread and block until ready."""
              self._thread = threading.Thread(target=self._run, name='mcpc-ws', daemon=True)
              self._thread.start()
              self._ready.wait()
              if self._startup_error is not None:
                  raise self._startup_error
```
/answer 86ae-10 allow
Control Request:
```yaml
id: 86ae-11
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
  node_type: Import|ImportFrom
  reason: Bestehende Imports in embedded.py für die Erweiterung ermitteln
```
/answer 86ae-11 allow
Control Request:
```yaml
id: 86ae-12
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
      nodes:
      - id: zTYD23|YL1yml
        type: imports
        code: |-
          from __future__ import annotations
          import logging
          import threading
          import uuid
          from abc import ABC, abstractmethod
          from collections.abc import Callable, Iterable
          from typing import TYPE_CHECKING, Any
          from xy.ai.mcpc.config import ServerConfig
          from xy.ai.mcpc.server.server import McpHTTPServer, build_server, build_ws_server
          from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
```
/answer 86ae-12 allow
ReasoningToken: 1917
Control Request:
```yaml
id: 86ae-13
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
    id: zTYD23|YL1yml
    start_marker: import logging
    end_marker: "ToolRegistry, ToolResult"
    content: |-
      import logging
      import threading
      import uuid
      from abc import ABC, abstractmethod
      from collections.abc import Callable, Iterable, Sequence
      from concurrent.futures import ThreadPoolExecutor
      from enum import Enum
      from typing import TYPE_CHECKING, Any
      from urllib.parse import urlparse
      from xy.ai.mcpc.cli.parameters import CacheMode, CliParameters, Effort, Model
      from xy.ai.mcpc.cli.session import CliResult, CliSession, CliSessionError
      from xy.ai.mcpc.config import ServerConfig
      from xy.ai.mcpc.server.server import McpHTTPServer, build_server, build_ws_server
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
    id: 7Pdehz|CzgsbJ
    start_marker: "__all__ = ['EmbeddedMcpc', 'FunctionTool', 'ToolProvider']"
    end_marker: "__all__ = ['EmbeddedMcpc', 'FunctionTool', 'ToolProvider']"
    content: "__all__ = ['Agent', 'EmbeddedMcpc', 'FunctionTool', 'ToolProvider', 'launch_agent']"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
    id: EmbeddedMcpc.open_session
    start_marker: "def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None) -> str:"
    end_marker: cc_profile='none')
    content: |-
      def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None, cc_profile: str='none') -> str:
              """Create a session with its own toolset (default: all registered tools).

              Sessions configured here are never overridden by the client's tools header.
              """
              sid = session_id or str(uuid.uuid4())
              names = {'all'} if tools is None else set(tools)
              session = self.server.sessions.precreate(sid, enabled_tools=names, cc_profile=cc_profile)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
    id: EmbeddedMcpc.headers
    start_marker: "def headers(self, session_id: str) -> dict[str, str]:"
    end_marker: "return {self.config.session_header: session_id}"
    content: |-
      def headers(self, session_id: str) -> dict[str, str]:
              """HTTP headers an agent must send to use *session_id*."""
              return {self.config.session_header: session_id}

          @property
          def agent_config(self) -> ServerConfig:
              """The configuration with the actually bound ports (relevant when configured with port 0)."""
              ws_url = self.ws_url
              if ws_url is None:
                  raise RuntimeError('Agents require the WebSocket transport (ws_enabled=True and `websockets` installed)')
              return self.config.with_overrides(port=self.server.server_address[1], ws_port=urlparse(ws_url).port)

          def start_agent(
                  self,
                  system_prompt: str,
                  *,
                  model: Model | str=Model.SONNET,
                  effort: Effort | str=Effort.MEDIUM,
                  tools: Iterable[str] | None=None,
                  cc_profile: str='none',
                  cache_mode: CacheMode | str=CacheMode.DEFAULT,
                  executable: str='claude',
                  extra_env: dict[str, str] | None=None,
                  agent_id: str | None=None) -> Agent:
              """Start an agent bound to this server with its own session and toolset (*tools*, default: all).

              Any number of agents may share one instance; the CLI process starts lazily on the first prompt.
              """
              sid = self.open_session(tools=tools, cc_profile=cc_profile)
              parameters = CliParameters(
                  config=self.agent_config,
                  model=_enum_value(model),
                  system_prompt=system_prompt,
                  mcpc_session_id=sid,
                  effort=_enum_value(effort),
                  cc_profile=cc_profile,
                  executable=executable,
                  extra_env=dict(extra_env or {}),
                  cache_mode=_enum_value(cache_mode))
              try:
                  cli = self.server.environment.cli_manager.request(parameters=parameters, session_id=agent_id)
              except Exception:
                  self.close_session(sid)
                  raise
              return Agent(self, cli, sid)

          def run_batch(
                  self,
                  prompts: Sequence[str],
                  system_prompt: str,
                  *,
                  max_workers: int=4,
                  **agent_options: Any) -> list[CliResult]:
              """Run each prompt on a fresh agent (parallel, sharing this server); results keep the prompt order.

              *agent_options* are passed to :meth:`start_agent`. Failures are returned as error results.
              """
              def run(prompt: str) -> CliResult:
                  try:
                      with self.start_agent(system_prompt, **agent_options) as agent:
                          return agent.prompt(prompt)
                  except CliSessionError as exc:
                      return CliResult(text=str(exc), is_error=True, subtype='session_error')

              with ThreadPoolExecutor(max_workers=max_workers, thread_name_prefix='mcpc-batch') as pool:
                  return list(pool.map(run, prompts))
  reason: "Embedded API um Agent-Start, Batch-Ausführung und korrekte Port-Auflösung erweitern"
```
/answer 86ae-13 allow
Control Request:
```yaml
id: 86ae-14
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
      result: success
      id: zTYD23|YL1yml
      new_id: zTYD23|YL1yml
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
      result: success
      id: 7Pdehz|CzgsbJ
      new_id: 7Pdehz|CzgsbJ
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
      error: "Syntax error: unindent does not match any outer indentation level (line 9)"
      id: EmbeddedMcpc.open_session
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
      error: "Syntax error: unindent does not match any outer indentation level (line 5)"
      id: EmbeddedMcpc.headers
```
/answer 86ae-14 allow
ReasoningToken: 60
Control Request:
```yaml
id: 86ae-15
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
    id: EmbeddedMcpc.open_session
    start_marker: "def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None) -> str:"
    end_marker: cc_profile='none')
    content: |-
      def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None, cc_profile: str='none') -> str:
          """Create a session with its own toolset (default: all registered tools).

          Sessions configured here are never overridden by the client's tools header.
          """
          sid = session_id or str(uuid.uuid4())
          names = {'all'} if tools is None else set(tools)
          session = self.server.sessions.precreate(sid, enabled_tools=names, cc_profile=cc_profile)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
    id: EmbeddedMcpc.headers
    start_marker: "def headers(self, session_id: str) -> dict[str, str]:"
    end_marker: "return {self.config.session_header: session_id}"
    content: |-
      def headers(self, session_id: str) -> dict[str, str]:
          """HTTP headers an agent must send to use *session_id*."""
          return {self.config.session_header: session_id}

      @property
      def agent_config(self) -> ServerConfig:
          """The configuration with the actually bound ports (relevant when configured with port 0)."""
          ws_url = self.ws_url
          if ws_url is None:
              raise RuntimeError('Agents require the WebSocket transport (ws_enabled=True and `websockets` installed)')
          return self.config.with_overrides(port=self.server.server_address[1], ws_port=urlparse(ws_url).port)

      def start_agent(
              self,
              system_prompt: str,
              *,
              model: Model | str=Model.SONNET,
              effort: Effort | str=Effort.MEDIUM,
              tools: Iterable[str] | None=None,
              cc_profile: str='none',
              cache_mode: CacheMode | str=CacheMode.DEFAULT,
              executable: str='claude',
              extra_env: dict[str, str] | None=None,
              agent_id: str | None=None) -> Agent:
          """Start an agent bound to this server with its own session and toolset (*tools*, default: all).

          Any number of agents may share one instance; the CLI process starts lazily on the first prompt.
          """
          sid = self.open_session(tools=tools, cc_profile=cc_profile)
          parameters = CliParameters(
              config=self.agent_config,
              model=_enum_value(model),
              system_prompt=system_prompt,
              mcpc_session_id=sid,
              effort=_enum_value(effort),
              cc_profile=cc_profile,
              executable=executable,
              extra_env=dict(extra_env or {}),
              cache_mode=_enum_value(cache_mode))
          try:
              cli = self.server.environment.cli_manager.request(parameters=parameters, session_id=agent_id)
          except Exception:
              self.close_session(sid)
              raise
          return Agent(self, cli, sid)

      def run_batch(
              self,
              prompts: Sequence[str],
              system_prompt: str,
              *,
              max_workers: int=4,
              **agent_options: Any) -> list[CliResult]:
          """Run each prompt on a fresh agent (parallel, sharing this server); results keep the prompt order.

          *agent_options* are passed to :meth:`start_agent`. Failures are returned as error results.
          """
          def run(prompt: str) -> CliResult:
              try:
                  with self.start_agent(system_prompt, **agent_options) as agent:
                      return agent.prompt(prompt)
              except CliSessionError as exc:
                  return CliResult(text=str(exc), is_error=True, subtype='session_error')

          with ThreadPoolExecutor(max_workers=max_workers, thread_name_prefix='mcpc-batch') as pool:
              return list(pool.map(run, prompts))
  reason: Einrückung der Methoden korrigieren (Knoten-Quelltext ist dedentiert)
```
/answer 86ae-15 allow
ReasoningToken: 58
Control Request:
```yaml
id: 86ae-16
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
    id: EmbeddedMcpc
    position: after
    source: |-
      def _enum_value(value: Enum | str) -> str:
          return value.value if isinstance(value, Enum) else value


      class Agent:
          """A Claude CLI agent wired to an :class:`EmbeddedMcpc` session.

          Created via :meth:`EmbeddedMcpc.start_agent` or :func:`launch_agent`. The
          conversation persists across :meth:`prompt` calls. Closing releases the CLI
          process and the MCPC session (and the server itself when the agent owns it).
          """

          def __init__(self, mcpc: EmbeddedMcpc, cli: CliSession, mcpc_session_id: str, *, owns_mcpc: bool=False) -> None:
              self.mcpc = mcpc
              self.cli = cli
              self.mcpc_session_id = mcpc_session_id
              self._owns_mcpc = owns_mcpc

          @property
          def id(self) -> str:
              return self.cli.id

          def prompt(self, text: str) -> CliResult:
              return self.cli.prompt(text)

          def ask(self, text: str) -> str:
              """Like :meth:`prompt` but return only the text and raise :class:`CliSessionError` on an error result."""
              result = self.prompt(text)
              if result.is_error:
                  raise CliSessionError(result.text)
              return result.text

          def close(self) -> None:
              self.cli.terminate()
              if self._owns_mcpc:
                  self.mcpc.stop()
                  return
              if self.mcpc._server is not None:
                  self.mcpc.server.environment.cli_manager.discard(self.cli.id)
                  self.mcpc.close_session(self.mcpc_session_id)

          def __enter__(self) -> Agent:
              return self

          def __exit__(self, *exc_info: object) -> None:
              self.close()


      def launch_agent(
              system_prompt: str,
              *,
              builtin_tools: bool | Iterable[str]=True,
              tools: Iterable[ToolDefinition | ToolProvider]=(),
              config: ServerConfig | None=None,
              auto_approve: bool=True,
              **agent_options: Any) -> Agent:
          """Start an agent with a dedicated, specially configured MCPC instance that it owns.

          *agent_options* are passed to :meth:`EmbeddedMcpc.start_agent`; closing the agent stops its server.
          """
          mcpc = EmbeddedMcpc(config, builtin_tools=builtin_tools, tools=tools, auto_approve=auto_approve).start()
          try:
              agent = mcpc.start_agent(system_prompt, **agent_options)
          except Exception:
              mcpc.stop()
              raise
          agent._owns_mcpc = True
          return agent
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/manager.py
    id: CliSessionManager.resolve
    position: after
    source: |-
      def discard(self, cli_session_id: str) -> None:
          """Terminate and forget a session; unknown ids are ignored."""
          with self._lock:
              session = self._sessions.pop(cli_session_id, None)
          if session is not None:
              session.terminate()
  reason: "Agent-Klasse, launch_agent und Manager.discard ergänzen"
```
/answer 86ae-16 allow
Control Request:
```yaml
id: 86ae-17
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/cli/manager.py
  reason: Syntax der geänderten Dateien prüfen
```
/answer 86ae-17 allow
Control Request:
```yaml
id: 86ae-18
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: ^from xy.ai.mcpc.embedded|import embedded|CliResult
    include:
    - __init__.py
    - session.py
    exclude:
    - tests/**
  reason: Zirkuläre Imports und Export von CliResult prüfen
```
/answer 86ae-18 allow
Control Request:
```yaml
id: 86ae-19
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: cli/session.py
        matches:
        - "class CliResult:"
        - "def prompt(self, text: str) -> CliResult:"
        - "def _collect_result(self) -> CliResult:"
        - "return CliResult(text=text, is_error=is_error, subtype=subtype)"
```
/answer 86ae-19 allow
Result Stats: id=68b4dce6-08bc-4bda-a5ff-b28695588b43, total: 57885, in: 26, out: 12116, reason: 0, read: 363380, write: 45743, start: 1793544263789, end: 1793544273795, time: 9953, total: 97694, io: 0, control: 53, sent: 0, recv: 251
Die Embedded API kann jetzt Agenten starten, ohne dass der MCPC-Server sie als Subagents aufruft. Ich habe den Code nicht ausgeführt und keinen echten `claude`-Lauf gemacht. Beide geänderten Dateien kompilieren, Tests gibt es dafür noch keine.

**Befund:** `CliParameters` liest Host und Ports aus der `ServerConfig`. `EmbeddedMcpc` startet aber standardmäßig mit `port=0` und `ws_port=0`. Die Config ist frozen, und die tatsächlich gebundenen Ports wurden nie zurückgeschrieben. Ein Agent, der direkt aus der Config gebaut wird, würde sich also mit Port 0 verbinden. Das behebt die neue Property `agent_config`, die die echten Ports einsetzt.

**Änderungen in `embedded.py`:**
- **`Agent`:** Wrapper um eine CLI-Session mit den Methoden `prompt()` (gibt ein `CliResult` zurück), `ask()` (gibt nur den Text zurück und wirft bei einem Fehler), `close()` und Context-Manager. Die Konversation bleibt über mehrere Prompts erhalten.
- **`EmbeddedMcpc.start_agent(system_prompt, *, model, effort, tools, cc_profile, cache_mode, executable, extra_env, agent_id)`:** Für den Fall "viele Agenten gegen einen MCPC". Jeder Agent bekommt eine eigene MCPC-Session mit eigenem Toolset (`tools`).
- **`EmbeddedMcpc.run_batch(prompts, system_prompt, *, max_workers=4, **agent_options)`:** Pro Prompt ein frischer Agent, parallel im Thread-Pool. Die Ergebnisse behalten die Reihenfolge der Prompts. Ein `CliSessionError` wird als `CliResult(is_error=True)` zurückgegeben und bricht die übrigen Prompts nicht ab.
- **`launch_agent(system_prompt, *, builtin_tools, tools, config, auto_approve=True, **agent_options)`:** Für den Fall "jeder Agent bekommt seinen eigenen MCPC". Der Agent besitzt die Instanz und stoppt sie in `close()`. Damit lassen sich Agenten mit unterschiedlichen Profilen und Toolsets bauen.
- **`open_session`:** Hat jetzt den Parameter `cc_profile`. Vorher war `'none'` fest eingetragen.
- `Agent` und `launch_agent` stehen in `__all__`.

**Änderung in `cli/manager.py`:** Neue Methode `CliSessionManager.discard(id)`, die eine Session beendet und aus dem Index entfernt. `Agent.close()` braucht sie, damit geschlossene Agenten im Manager nicht liegen bleiben.

**Beispiel:**
```python
with EmbeddedMcpc(builtin_tools=["read", "bash"], tools=[MyTools()], auto_approve=True) as mcpc:
    results = mcpc.run_batch(prompts, "You process records.", tools={"lookup", "read"}, max_workers=8)

with launch_agent("You are a reviewer.", builtin_tools=["read"], cc_profile="review") as agent:
    print(agent.ask("Review foo.py"))
```

**Einschränkungen:**
- Agenten brauchen den WebSocket-Transport (`ws_enabled=True` und das Paket `websockets`). Fehlt er, wirft `agent_config` einen `RuntimeError`.
- Mit `auto_approve=False` und ohne Controller blockieren Tool-Aufrufe. `launch_agent` setzt deshalb `auto_approve=True` als Default, `start_agent` übernimmt die Einstellung des `EmbeddedMcpc`.
Lässt sich `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py` etwas reduzieren und Teile Auslagern die von einem Agenten beim schreiben eines Scriptes nicht unmittelbar benötigt werden?
Result Stats: id=68b4dce6-08bc-4bda-a5ff-b28695588b43, total: 60743, in: 28, out: 13470, reason: 383, read: 409123, write: 47245, start: 1793544448211, end: 1793544460591, time: 12321, total: 110015, io: 0, control: 59, sent: 1, recv: 231
SystemInit:  id=68b4dce6-08bc-4bda-a5ff-b28695588b43, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 383
Ja. Ich habe noch nichts geändert, weil dabei die Importpfade berührt werden. Ein Agent, der ein Script schreibt, braucht aus `embedded.py` nur die öffentliche API. Das sind `EmbeddedMcpc` mit `start`/`stop`, `tool`/`add_tool`, `start_agent`, `run_batch` und `open_session`/`headers`, dazu `launch_agent`, `Agent`, `ToolProvider` und `FunctionTool`. Alles andere liest er nicht.

**Auslagern**
- **`FunctionTool` und `ToolProvider`:** Beide wandern nach `embedded_tools.py` und werden aus `embedded.py` re-exportiert, sodass `from xy.ai.mcpc.embedded import FunctionTool` weiter funktioniert. Beide sind reine Tool-Definitionen und haben mit dem Serverlebenszyklus nichts zu tun.
- **Server-Interna:** `_register`, `_register_all`, die Thread-Verwaltung in `start`/`stop` und der Port-Auflösungsteil von `agent_config` kommen in einen privaten Helfer, zum Beispiel `_embedded_runtime.py`. Das sind Implementierungsdetails, keine API.
- **`_enum_value` und das Bauen der `CliParameters`:** Das gehört nach `cli/parameters.py` als Factory, zum Beispiel `CliParameters.for_embedded(...)`. So steht der CLI-Kram nicht im Embedded-Modul.

**Reduzieren**
- **Modul-Docstring:** Er wird zur Kurzanleitung mit je einem Beispiel für "geteilter MCPC" (`run_batch`) und "eigener MCPC pro Agent" (`launch_agent`). Das ist das Erste, was ein Agent liest.
- **Property `server`:** Sie bleibt, weil sie als Escape-Hatch nützlich ist. Ob `tool_names` und `ws_url` bleiben sollen, entscheide ich nach Prüfung der Aufrufer.
- **`Agent`:** `ask()` und `prompt()` überlappen. Ich würde beide behalten, weil `prompt()` das `CliResult` liefert und `ask()` für einfache Scripte gedacht ist.

**Ergebnis**
`embedded.py` bliebe eine kurze Fassade mit `EmbeddedMcpc`, `Agent` und `launch_agent`. Die Tool-Klassen und die Interna lägen in eigenen Modulen. Den Importpfad ändert das für Bestandscode nicht.

Soll ich das so umsetzen? Dann suche ich vorher noch nach Aufrufern von `_register*` und `embedded.*` im Repo.
Erstelle ein Package in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded`
SystemInit:  id=68b4dce6-08bc-4bda-a5ff-b28695588b43, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 86ae-1a
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: mcpc\.embedded|mcpc import embedded|_register_all|_register\(
    include:
    - '*.py'
    - '*.md'
    - '*.toml'
    - '*.cfg'
    exclude:
    - .git/**
    - node_modules/**
    - .venv/**
    limit: 40
  reason: "Aufrufer des Moduls embedded und der internen Register-Methoden finden, bevor es zum Package wird"
```
/answer 86ae-1a deny embedded wird noch nicht verwendet
ReasoningToken: 59
Control Request:
```yaml
id: 86ae-1b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
    ids:
    - ToolProvider
    - FunctionTool
    - 7Pdehz|CzgsbJ
  reason: Quelltext von ToolProvider und FunctionTool für die Auslagerung lesen
```
/answer 86ae-1b allow
Control Request:
```yaml
id: 86ae-1c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
      nodes:
      - id: ToolProvider
        type: ClassDef
        signature: "class ToolProvider(ABC):"
        docstring: Subclass and implement :meth:`tools` to contribute custom tools to a server.
        children:
        - id: ToolProvider.Go7Vy4|1l0D1Q
          type: statements
          code: '''Subclass and implement :meth:`tools` to contribute custom tools to a server.'''
        - id: ToolProvider.tools
          type: FunctionDef
          code: |-
            @abstractmethod
            def tools(self, environment: AppEnvironment) -> Iterable[ToolDefinition]:
                """Return the tools to register. *environment* gives access to config, sessions, etc."""
      - id: FunctionTool
        type: ClassDef
        signature: "class FunctionTool(ToolDefinition):"
        docstring: A tool backed by a plain callable taking a ``ToolContext``.
        children:
        - id: FunctionTool.VwPilR|QMm9kb
          type: statements
          code: '''A tool backed by a plain callable taking a ``ToolContext``.'''
        - id: FunctionTool.init
          type: FunctionDef
          code: |-
            def __init__(self, name: str, description: str, input_schema: dict[str, Any], handler: ToolHandlerFn, *, title: str | None=None, output_schema: dict[str, Any] | None=None, annotations: dict[str, Any] | None=None) -> None:
                self.name = name
                self.description = description
                self.input_schema = input_schema
                self.title = title
                self.output_schema = output_schema if output_schema is not None else {'type': 'object'}
                self.annotations = annotations
                self._handler = handler
        - id: FunctionTool.handle
          type: FunctionDef
          code: |-
            def handle(self, ctx: ToolContext) -> ToolResult | str | dict[str, Any]:
                return self._handler(ctx)
      - id: 7Pdehz|G3UOct
        type: statements
        code: |-
          if TYPE_CHECKING:
              from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext
          __all__ = ['Agent', 'EmbeddedMcpc', 'FunctionTool', 'ToolProvider', 'launch_agent']
          logger = logging.getLogger('xy.ai.mcpc.embedded')
          ToolHandlerFn = Callable[['ToolContext'], 'ToolResult | str | dict[str, Any]']
```
/answer 86ae-1c allow
ReasoningToken: 787
Control Request:
```yaml
id: 86ae-1d
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/tools.py
    source: |
      """Custom tool definitions for an embedded MCPC server."""
      from __future__ import annotations

      from abc import ABC, abstractmethod
      from collections.abc import Callable, Iterable
      from typing import TYPE_CHECKING, Any

      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult

      if TYPE_CHECKING:
          from xy.ai.mcpc.tools.tool_context import AppEnvironment, ToolContext

      __all__ = ['FunctionTool', 'ToolHandlerFn', 'ToolProvider']

      ToolHandlerFn = Callable[['ToolContext'], 'ToolResult | str | dict[str, Any]']


      class ToolProvider(ABC):
          """Subclass and implement :meth:`tools` to contribute custom tools to a server."""

          @abstractmethod
          def tools(self, environment: AppEnvironment) -> Iterable[ToolDefinition]:
              """Return the tools to register. *environment* gives access to config, sessions, etc."""


      class FunctionTool(ToolDefinition):
          """A tool backed by a plain callable taking a ``ToolContext``."""

          def __init__(
                  self,
                  name: str,
                  description: str,
                  input_schema: dict[str, Any],
                  handler: ToolHandlerFn,
                  *,
                  title: str | None=None,
                  output_schema: dict[str, Any] | None=None,
                  annotations: dict[str, Any] | None=None) -> None:
              self.name = name
              self.description = description
              self.input_schema = input_schema
              self.title = title
              self.output_schema = output_schema if output_schema is not None else {'type': 'object'}
              self.annotations = annotations
              self._handler = handler

          def handle(self, ctx: ToolContext) -> ToolResult | str | dict[str, Any]:
              return self._handler(ctx)


      def register_tool(tool: ToolDefinition | ToolProvider, registry: ToolRegistry, environment: AppEnvironment) -> None:
          if isinstance(tool, ToolProvider):
              for definition in tool.tools(environment):
                  registry.register(definition)
          else:
              registry.register(tool)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/agent.py
    source: |
      """Agents driven from a script against an embedded MCPC."""
      from __future__ import annotations

      from collections.abc import Iterable
      from enum import Enum
      from typing import TYPE_CHECKING

      from xy.ai.mcpc.cli.parameters import CacheMode, CliParameters, Effort, Model
      from xy.ai.mcpc.cli.session import CliResult, CliSession, CliSessionError

      if TYPE_CHECKING:
          from xy.ai.mcpc.config import ServerConfig
          from xy.ai.mcpc.embedded.mcpc import EmbeddedMcpc

      __all__ = ['Agent', 'CacheMode', 'CliResult', 'CliSessionError', 'Effort', 'Model']


      def _value(value: Enum | str) -> str:
          return value.value if isinstance(value, Enum) else value


      def agent_parameters(
              config: ServerConfig,
              mcpc_session_id: str,
              system_prompt: str,
              *,
              model: Model | str,
              effort: Effort | str,
              cc_profile: str,
              cache_mode: CacheMode | str,
              executable: str,
              extra_env: dict[str, str] | None) -> CliParameters:
          return CliParameters(
              config=config,
              model=_value(model),
              system_prompt=system_prompt,
              mcpc_session_id=mcpc_session_id,
              effort=_value(effort),
              cc_profile=cc_profile,
              executable=executable,
              extra_env=dict(extra_env or {}),
              cache_mode=_value(cache_mode))


      class Agent:
          """A Claude CLI agent wired to an :class:`EmbeddedMcpc` session.

          Created via :meth:`EmbeddedMcpc.start_agent` or :func:`launch_agent`. The
          conversation persists across :meth:`prompt` calls. Closing releases the CLI
          process and the MCPC session (and the server itself when the agent owns it).
          """

          def __init__(self, mcpc: EmbeddedMcpc, cli: CliSession, mcpc_session_id: str, *, owns_mcpc: bool=False) -> None:
              self.mcpc = mcpc
              self.cli = cli
              self.mcpc_session_id = mcpc_session_id
              self.owns_mcpc = owns_mcpc

          @property
          def id(self) -> str:
              return self.cli.id

          def prompt(self, text: str) -> CliResult:
              return self.cli.prompt(text)

          def ask(self, text: str) -> str:
              """Like :meth:`prompt` but return only the text; raise :class:`CliSessionError` on an error result."""
              result = self.prompt(text)
              if result.is_error:
                  raise CliSessionError(result.text)
              return result.text

          def close(self) -> None:
              self.cli.terminate()
              if self.owns_mcpc:
                  self.mcpc.stop()
              elif self.mcpc.running:
                  self.mcpc.server.environment.cli_manager.discard(self.cli.id)
                  self.mcpc.close_session(self.mcpc_session_id)

          def __enter__(self) -> Agent:
              return self

          def __exit__(self, *exc_info: object) -> None:
              self.close()
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/mcpc.py
    source: |
      """An MCPC server running on a background thread inside the current process."""
      from __future__ import annotations

      import logging
      import threading
      import uuid
      from collections.abc import Callable, Iterable, Sequence
      from concurrent.futures import ThreadPoolExecutor
      from typing import Any
      from urllib.parse import urlparse

      from xy.ai.mcpc.config import ServerConfig
      from xy.ai.mcpc.embedded.agent import Agent, CacheMode, CliResult, CliSessionError, Effort, Model, agent_parameters
      from xy.ai.mcpc.embedded.tools import FunctionTool, ToolHandlerFn, ToolProvider, register_tool
      from xy.ai.mcpc.server.server import McpHTTPServer, build_server, build_ws_server
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry

      __all__ = ['EmbeddedMcpc', 'launch_agent']

      logger = logging.getLogger('xy.ai.mcpc.embedded')


      class EmbeddedMcpc:
          """An MCPC server running on a background thread inside the current process.

          :param builtin_tools: ``True`` for all built-in tools, ``False`` for none, or
              an iterable of tool/alias names to keep.
          :param tools: extra :class:`ToolDefinition` instances or :class:`ToolProvider`\\ s.
          :param auto_approve: approve every tool request and result without a controller.
          :param ignore_tools_header: ignore the client's tools header; sessions get all
              registered tools unless configured via :meth:`open_session`.
          """

          def __init__(
                  self,
                  config: ServerConfig | None=None,
                  *,
                  builtin_tools: bool | Iterable[str]=True,
                  tools: Iterable[ToolDefinition | ToolProvider]=(),
                  auto_approve: bool | None=None,
                  ignore_tools_header: bool | None=None,
                  enable_control: bool=True) -> None:
              if config is None:
                  config = ServerConfig(host='127.0.0.1', port=0, ws_enabled=True, ws_port=0)
              overrides = {}
              if auto_approve is not None:
                  overrides['auto_approve'] = auto_approve
              if ignore_tools_header is not None:
                  overrides['ignore_tools_header'] = ignore_tools_header
              self.config = config.with_overrides(**overrides) if overrides else config
              self._builtin_tools = builtin_tools if isinstance(builtin_tools, bool) else frozenset(builtin_tools)
              self._tools: list[ToolDefinition | ToolProvider] = list(tools)
              self._enable_control = enable_control
              self._server: McpHTTPServer | None = None
              self._ws_server = None
              self._thread: threading.Thread | None = None

          def add_tool(self, tool: ToolDefinition | ToolProvider) -> None:
              """Register a tool or provider; before :meth:`start` it is queued, afterwards registered live."""
              if self._server is None:
                  self._tools.append(tool)
                  return
              register_tool(tool, self._server.environment.registry, self._server.environment)

          def tool(
                  self,
                  name: str,
                  description: str,
                  input_schema: dict[str, Any] | None=None,
                  **kwargs: Any) -> Callable[[ToolHandlerFn], ToolHandlerFn]:
              """Decorator registering a function ``(ctx) -> result`` as a tool."""
              schema = input_schema if input_schema is not None else {'type': 'object', 'properties': {}}

              def decorator(fn: ToolHandlerFn) -> ToolHandlerFn:
                  self.add_tool(FunctionTool(name, description, schema, fn, **kwargs))
                  return fn
              return decorator

          def _register_all(self, registry: ToolRegistry, environment: Any) -> None:
              for tool in self._tools:
                  register_tool(tool, registry, environment)
              self._tools.clear()

          def start(self) -> EmbeddedMcpc:
              if self._server is not None:
                  raise RuntimeError('Server already started')
              server = build_server(
                  self.config,
                  enable_control=self._enable_control,
                  builtin_tools=self._builtin_tools,
                  extra_tools=self._register_all)
              self._server = server
              self._ws_server = build_ws_server(server)
              if self._ws_server is not None:
                  self._ws_server.start()
              self._thread = threading.Thread(
                  target=server.serve_forever,
                  kwargs={'poll_interval': 0.1},
                  name='embedded-mcpc',
                  daemon=True)
              self._thread.start()
              logger.info('Embedded MCPC listening on %s', server.endpoint_url)
              return self

          def stop(self) -> None:
              server = self._server
              if server is None:
                  return
              self._server = None
              if self._ws_server is not None:
                  self._ws_server.stop()
                  self._ws_server = None
              server.environment.cli_manager.shutdown()
              server.shutdown()
              server.server_close()
              if self._thread is not None:
                  self._thread.join(timeout=5)
                  self._thread = None

          def __enter__(self) -> EmbeddedMcpc:
              return self.start()

          def __exit__(self, *exc_info: object) -> None:
              self.stop()

          @property
          def running(self) -> bool:
              return self._server is not None

          @property
          def server(self) -> McpHTTPServer:
              if self._server is None:
                  raise RuntimeError('Server not started')
              return self._server

          @property
          def url(self) -> str:
              return self.server.endpoint_url

          @property
          def ws_url(self) -> str | None:
              return self._ws_server.endpoint_url if self._ws_server is not None else None

          @property
          def tool_names(self) -> list[str]:
              return self.server.environment.registry.names()

          def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None, cc_profile: str='none') -> str:
              """Create a session with its own toolset (default: all registered tools).

              Sessions configured here are never overridden by the client's tools header.
              """
              sid = session_id or str(uuid.uuid4())
              names = {'all'} if tools is None else set(tools)
              session = self.server.sessions.precreate(sid, enabled_tools=names, cc_profile=cc_profile)
              session.state['tools_preset'] = True
              return sid

          def close_session(self, session_id: str) -> bool:
              return self.server.sessions.remove(session_id)

          def headers(self, session_id: str) -> dict[str, str]:
              """HTTP headers an agent must send to use *session_id*."""
              return {self.config.session_header: session_id}

          @property
          def agent_config(self) -> ServerConfig:
              """The configuration with the actually bound ports (relevant when configured with port 0)."""
              ws_url = self.ws_url
              if ws_url is None:
                  raise RuntimeError('Agents require the WebSocket transport (ws_enabled=True and `websockets` installed)')
              return self.config.with_overrides(port=self.server.server_address[1], ws_port=urlparse(ws_url).port)

          def start_agent(
                  self,
                  system_prompt: str,
                  *,
                  model: Model | str=Model.SONNET,
                  effort: Effort | str=Effort.MEDIUM,
                  tools: Iterable[str] | None=None,
                  cc_profile: str='none',
                  cache_mode: CacheMode | str=CacheMode.DEFAULT,
                  executable: str='claude',
                  extra_env: dict[str, str] | None=None,
                  agent_id: str | None=None) -> Agent:
              """Start an agent bound to this server with its own session and toolset (*tools*, default: all).

              Any number of agents may share one instance; the CLI process starts lazily on the first prompt.
              """
              sid = self.open_session(tools=tools, cc_profile=cc_profile)
              try:
                  parameters = agent_parameters(
                      self.agent_config,
                      sid,
                      system_prompt,
                      model=model,
                      effort=effort,
                      cc_profile=cc_profile,
                      cache_mode=cache_mode,
                      executable=executable,
                      extra_env=extra_env)
                  cli = self.server.environment.cli_manager.request(parameters=parameters, session_id=agent_id)
              except Exception:
                  self.close_session(sid)
                  raise
              return Agent(self, cli, sid)

          def run_batch(
                  self,
                  prompts: Sequence[str],
                  system_prompt: str,
                  *,
                  max_workers: int=4,
                  **agent_options: Any) -> list[CliResult]:
              """Run each prompt on a fresh agent (parallel, sharing this server); results keep the prompt order.

              *agent_options* are passed to :meth:`start_agent`. Failures are returned as error results.
              """
              def run(prompt: str) -> CliResult:
                  try:
                      with self.start_agent(system_prompt, **agent_options) as agent:
                          return agent.prompt(prompt)
                  except CliSessionError as exc:
                      return CliResult(text=str(exc), is_error=True, subtype='session_error')

              with ThreadPoolExecutor(max_workers=max_workers, thread_name_prefix='mcpc-batch') as pool:
                  return list(pool.map(run, prompts))


      def launch_agent(
              system_prompt: str,
              *,
              builtin_tools: bool | Iterable[str]=True,
              tools: Iterable[ToolDefinition | ToolProvider]=(),
              config: ServerConfig | None=None,
              auto_approve: bool=True,
              **agent_options: Any) -> Agent:
          """Start an agent with a dedicated, specially configured MCPC instance that it owns.

          *agent_options* are passed to :meth:`EmbeddedMcpc.start_agent`; closing the agent stops its server.
          """
          mcpc = EmbeddedMcpc(config, builtin_tools=builtin_tools, tools=tools, auto_approve=auto_approve).start()
          try:
              agent = mcpc.start_agent(system_prompt, **agent_options)
          except Exception:
              mcpc.stop()
              raise
          agent.owns_mcpc = True
          return agent
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/__init__.py
    source: |
      """Run an MCPC server inline from a Python script and drive agents against it.

      One MCPC shared by parallel agents (each agent gets its own session and toolset)::

          class MyTools(ToolProvider):
              def tools(self, environment):
                  return [FunctionTool("lookup", "Look up a record", {"type": "object", "properties": {}}, lambda ctx: "ok")]

          with EmbeddedMcpc(builtin_tools=["read"], tools=[MyTools()], auto_approve=True) as mcpc:
              results = mcpc.run_batch(prompts, "You process records.", tools={"lookup", "read"}, max_workers=8)
              with mcpc.start_agent("You are a reviewer.") as agent:
                  text = agent.ask("Review foo.py")

      A dedicated, specially configured MCPC per agent::

          with launch_agent("You are a reviewer.", builtin_tools=["read"], cc_profile="review") as agent:
              text = agent.ask("Review foo.py")
      """
      from xy.ai.mcpc.embedded.agent import Agent, CacheMode, CliResult, CliSessionError, Effort, Model
      from xy.ai.mcpc.embedded.mcpc import EmbeddedMcpc, launch_agent
      from xy.ai.mcpc.embedded.tools import FunctionTool, ToolProvider

      __all__ = [
          'Agent',
          'CacheMode',
          'CliResult',
          'CliSessionError',
          'EmbeddedMcpc',
          'Effort',
          'FunctionTool',
          'Model',
          'ToolProvider',
          'launch_agent',
      ]
  reason: "Package embedded/ mit tools.py, agent.py, mcpc.py und __init__.py anlegen"
```
/answer 86ae-1d allow
Text:
 Das alte Modul `embedded.py` würde vom Package überdeckt, deshalb entferne ich es und validiere die neuen Dateien.
Control Request:
```yaml
id: 86ae-1e
toolName: ast_delete
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded.py
  reason: Altes Modul embedded.py durch das gleichnamige Package ersetzen (Namenskonflikt)
```
/answer 86ae-1e allow
Control Request:
```yaml
id: 86ae-1f
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/tools.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/agent.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/mcpc.py
  reason: Syntax des neuen Packages prüfen
```
/answer 86ae-1f allow
Result Stats: id=68b4dce6-08bc-4bda-a5ff-b28695588b43, total: 82511, in: 36, out: 23139, reason: 0, read: 613696, write: 59336, start: 1793544595924, end: 1793544601754, time: 5784, total: 164736, io: 0, control: 46, sent: 0, recv: 141
Das Package `xy/ai/mcpc/embedded/` ist angelegt und ersetzt das alte `embedded.py`, das ich gelöscht habe. Die vier neuen Dateien kompilieren. Ausgeführt oder getestet habe ich nichts, auch keinen echten `claude`-Lauf.

| Datei | Inhalt |
|---|---|
| `__init__.py` | Kurzanleitung mit zwei Beispielen (geteilter MCPC mit `run_batch`, eigener MCPC pro Agent mit `launch_agent`). Re-exportiert `EmbeddedMcpc`, `launch_agent`, `Agent`, `ToolProvider`, `FunctionTool`, `CliResult`, `CliSessionError`, `Model`, `Effort` und `CacheMode`. |
| `mcpc.py` | `EmbeddedMcpc` (Serverlebenszyklus, Sessions, `start_agent`, `run_batch`) und `launch_agent`. |
| `agent.py` | `Agent` und die Funktion `agent_parameters`, die die `CliParameters` baut. |
| `tools.py` | `ToolProvider`, `FunctionTool`, `ToolHandlerFn` und `register_tool`. |

**Änderungen gegenüber der alten Datei:**
- `_register` und `_register_all` sind weg. Die Logik steckt jetzt in `register_tool` in `tools.py`.
- `EmbeddedMcpc` hat die neue Property `running`. `Agent.close()` nutzt sie statt des privaten `_server`.
- `Agent.owns_mcpc` ist jetzt öffentlich.
- `agent.py` importiert `EmbeddedMcpc` nur unter `TYPE_CHECKING`, damit kein Importzyklus mit `mcpc.py` entsteht. Deshalb liegt `launch_agent` in `mcpc.py`.

Den Importpfad `xy.ai.mcpc.embedded` habe ich beibehalten. Aufrufer habe ich nicht gesucht, weil du die Suche abgelehnt hast. Du sagtest dazu, `embedded` werde noch nicht verwendet.
Kannst du im Package eine englischsprache READE.md erzeugen die allen notwendigen Kontext enthält um einem Agenten das einfache erstellen eines Batch Skriptes ermöglicht. Kompakt, optimiert auf Salienz.
SystemInit:  id=68b4dce6-08bc-4bda-a5ff-b28695588b43, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 86ae-20
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
  name: ToolContext
  reason: "Wie Tool-Handler an Argumente kommen, für korrekte README-Beispiele"
```
/answer 86ae-20 allow
ReasoningToken: 87
Control Request:
```yaml
id: 86ae-21
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
      nodes:
      - id: ToolContext
        code: |-
          @dataclass(slots=True)
          class ToolContext:
              """Context handed to a tool handler on invocation.
              """
              session: 'Session'
              arguments: dict[str, Any]
```
/answer 86ae-21 allow
Control Request:
```yaml
id: 86ae-22
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: class ToolResult|def (text|error|ok)\(|name = '
    include:
    - tool_registry.py
    - __init__.py
    exclude:
    - tests/**
    limit: 30
  reason: ToolResult-Konstruktion und Namen der Built-in-Tools für README
```
/answer 86ae-22 allow
Control Request:
```yaml
id: 86ae-23
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: tool_usage/__init__.py
        matches:
        - name = 'tool_usage'
      - path: edit_chars/__init__.py
        matches:
        - name = 'edit_chars'
      - path: skills/__init__.py
        matches:
        - name = 'markdown_format'
        - name = 'skills'
      - path: agent/__init__.py
        matches:
        - name = 'agent'
      - path: bash/__init__.py
        matches:
        - name = 'bash'
      - path: edit_lines/__init__.py
        matches:
        - name = 'edit_lines'
      - path: tool_registry.py
        matches:
        - "class ToolResult:"
      - path: tool_search/__init__.py
        matches:
        - name = 'tool_search'
      - path: tool_call/__init__.py
        matches:
        - name = 'tool_call'
      - path: edit_block/__init__.py
        matches:
        - name = 'edit_block'
      - path: colgrep/__init__.py
        matches:
        - name = 'colgrep'
      - path: file_stats/__init__.py
        matches:
        - name = 'file_stats'
      - path: markdown/__init__.py
        matches:
        - name = 'markdown'
      - path: python/__init__.py
        matches:
        - name = 'python'
      - path: edit_marks/__init__.py
        matches:
        - name = 'edit_marks'
      - path: list/__init__.py
        matches:
        - name = 'list'
      - path: edit_line/__init__.py
        matches:
        - name = 'edit_line'
      - path: write/__init__.py
        matches:
        - name = 'write'
      - path: read/__init__.py
        matches:
        - name = 'read_file'
      - path: search/__init__.py
        matches:
        - name = 'search'
```
/answer 86ae-23 allow
ReasoningToken: 83
Control Request:
```yaml
id: 86ae-24
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/README.md
    source: |
      # xy.ai.mcpc.embedded

      Run an MCPC server (MCP server giving an agent focused tools) inside your own Python process and drive Claude CLI agents against it. Use it for batch scripts: iterate over inputs, one agent per item, parallel.

      ```python
      from xy.ai.mcpc.embedded import EmbeddedMcpc, launch_agent, FunctionTool, ToolProvider, Model, Effort
      ```

      Requirements: `claude` CLI on `PATH`, `websockets` installed. Agents need the WebSocket transport (enabled by default here).

      ## Pick a topology

      | Need | Use |
      |---|---|
      | Many agents, **one shared** MCPC with preconfigured tools | `EmbeddedMcpc` + `run_batch` / `start_agent` |
      | Each agent its **own specially configured** MCPC (tools, profile) | `launch_agent(...)` |

      ## Shared MCPC (default pattern)

      ```python
      with EmbeddedMcpc(builtin_tools=["read_file", "search"], auto_approve=True) as mcpc:

          @mcpc.tool("lookup", "Look up a record by id",
                     {"type": "object", "properties": {"id": {"type": "string"}}, "required": ["id"]})
          def lookup(ctx):                       # ctx.arguments: dict, ctx.session
              return db[ctx.arguments["id"]]     # str | dict | ToolResult

          results = mcpc.run_batch(
              prompts,                           # one fresh agent per prompt, results in prompt order
              "You classify records. Use `lookup`.",   # system prompt
              tools={"lookup", "read_file"},     # per-agent toolset (None = all registered)
              model=Model.HAIKU, effort=Effort.LOW,
              max_workers=8)

          for r in results:                      # CliResult(text, is_error, subtype)
              print("ERR" if r.is_error else "ok", r.text)
      ```

      Register tools **before or after** `start()`; `@mcpc.tool` works in both cases.

      ## Multi-turn / manual control

      ```python
      with mcpc.start_agent("You are a reviewer.", tools={"read_file"}) as agent:
          a = agent.ask("Review foo.py")         # str; raises CliSessionError on error result
          r = agent.prompt("Now bar.py")         # CliResult, never raises on agent-side errors
      ```

      The conversation persists across calls on the same `Agent`. Closing (context manager or `agent.close()`) kills the CLI process and the MCPC session.

      ## Dedicated MCPC per agent

      ```python
      with launch_agent("You review SQL.", builtin_tools=["read_file"], tools=[MyProvider()],
                        cc_profile="review", model=Model.OPUS) as agent:
          print(agent.ask("..."))                # closing also stops its server
      ```

      For a pool of specialised agents run several `launch_agent` calls in a `ThreadPoolExecutor`.

      ## API

      **`EmbeddedMcpc(config=None, *, builtin_tools=True, tools=(), auto_approve=None, ignore_tools_header=None, enable_control=True)`**
      - `builtin_tools`: `True` all, `False` none, or names to keep. Run `mcpc.tool_names` after `start()` to see what is registered.
      - `tools`: `ToolDefinition`s or `ToolProvider`s. `FunctionTool(name, description, input_schema, handler)` wraps a callable `(ctx) -> result`.
      - `auto_approve=True`: required in scripts, otherwise tool calls wait for a controller that does not exist.
      - Default config: `127.0.0.1`, ephemeral HTTP and WS ports.
      - `start()`/`stop()`/context manager, `running`, `url`, `ws_url`, `server`.
      - `add_tool(tool)`, `@tool(name, description, input_schema=None)`.
      - `open_session(tools=None, session_id=None, cc_profile="none") -> sid`, `close_session(sid)`, `headers(sid)`: for an external MCP client instead of the built-in agent.
      - `start_agent(system_prompt, *, model=Model.SONNET, effort=Effort.MEDIUM, tools=None, cc_profile="none", cache_mode=CacheMode.DEFAULT, executable="claude", extra_env=None, agent_id=None) -> Agent`
      - `run_batch(prompts, system_prompt, *, max_workers=4, **start_agent_options) -> list[CliResult]`

      **`Agent`**: `id`, `prompt(text) -> CliResult`, `ask(text) -> str`, `close()`.

      **Enums** (plain strings also accepted): `Model` = haiku|sonnet|opus; `Effort` = disabled|low|medium|high|xhigh|max; `CacheMode` = default|disabled|5m|1h.

      ## Gotchas

      - `tools=` on `start_agent`/`run_batch` selects from the **registered** tool names; unknown names give the agent nothing.
      - `cc_profile` selects the Claude config dir `~/.claude-<profile>` (login, settings). Use a profile that is authenticated.
      - `run_batch` converts `CliSessionError` (timeout, crash) into `CliResult(is_error=True)`; check `is_error` per result.
      - Sessions expire after `agent_session_ttl_seconds` of inactivity; the response timeout defaults to 24 h. Override via `ServerConfig`.
      - Keep `max_workers` modest: each agent is one `claude` process.
      - CLI traffic is logged per agent under `config.cli_log_dir` (`<agent id>.json.log`); look there when a result is an error.
  reason: Kompakte englische README im Package erzeugen
```
/answer 86ae-24 allow
Result Stats: id=68b4dce6-08bc-4bda-a5ff-b28695588b43, total: 90655, in: 42, out: 26524, reason: 0, read: 794885, write: 64089, start: 1793544735947, end: 1793544740771, time: 4779, total: 190043, io: 0, control: 45, sent: 0, recv: 106
Die README liegt unter `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/README.md`. Ich habe sie als `README.md` angelegt, nicht als `READE.md`. Ich habe sie nicht gegen einen echten Lauf geprüft.

Sie enthält:
- Eine Tabelle zur Wahl der Topologie: geteilter MCPC oder eigener MCPC pro Agent.
- Ein vollständiges Batch-Beispiel mit `@mcpc.tool` und `run_batch`.
- Ein Beispiel für Mehr-Runden-Gespräche mit `start_agent`, `ask()` und `prompt()`.
- Ein Beispiel für `launch_agent`.
- Die kompakte API-Referenz mit Signaturen und Enums.
- Fallstricke: `auto_approve`, `cc_profile`, Fehlerbehandlung in `run_batch`, Timeouts, `max_workers` und der Ort der CLI-Logs.

Offene Punkte:
- Die README nennt `read_file` und `search` als Tool-Namen. In der Registry heißt das Lese-Tool `read_file`, während der alte Docstring den Alias `read` benutzte. Ob Aliase in `tools={...}` bei Sessions greifen, habe ich nicht geprüft.
- Die README nennt `agent_session_ttl_seconds` und `cli_log_dir` als Config-Felder. Die Namen stammen aus `build_server`.
Erstelle einen Test in `/home/user/xyan/xy.ai.workbench/mcpc/tests` der mittels Haiku über drei Prompts iteriert. 
SystemInit:  id=68b4dce6-08bc-4bda-a5ff-b28695588b43, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 86ae-25
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: pytest|markers|testpaths|asyncio_mode
    include:
    - pyproject.toml
    - pytest.ini
    - conftest.py
    - setup.cfg
    exclude:
    - .git/**
    - .venv/**
    limit: 20
  reason: "Test-Konventionen (pytest, Marker) prüfen"
```
/answer 86ae-25 deny PyTest, vorhandene Konventionen sind nicht relevant
ReasoningToken: 14
Control Request:
```yaml
id: 86ae-26
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: ^import |^from |^def test_|skipif|mark\.
    include:
    - '*.py'
    exclude:
    - __pycache__/**
    limit: 25
  reason: Stil bestehender Tests ansehen
```
/answer 86ae-26 deny PyTest, vorhandene Konventionen sind nicht relevant
ReasoningToken: 60
Control Request:
```yaml
id: 86ae-27
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_embedded_batch.py
    source: |
      import shutil

      import pytest

      pytest.importorskip("websockets")

      from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model

      pytestmark = pytest.mark.skipif(shutil.which("claude") is None, reason="claude CLI not available")

      PROMPTS = [
          "What is 2+3? Answer with the number only.",
          "What is the capital of France? Answer with one word.",
          "Spell the word 'cat' backwards. Answer with the word only.",
      ]
      EXPECTED = ["5", "paris", "tac"]


      def test_haiku_batch_over_three_prompts():
          with EmbeddedMcpc(builtin_tools=False, auto_approve=True) as mcpc:
              results = mcpc.run_batch(
                  PROMPTS,
                  "Answer as briefly as possible.",
                  model=Model.HAIKU,
                  effort=Effort.DISABLED,
                  max_workers=3)

          assert len(results) == len(PROMPTS)
          for result, expected in zip(results, EXPECTED):
              assert not result.is_error, result.text
              assert expected in result.text.lower()
  reason: Pytest-Test mit Haiku über drei Prompts anlegen
```
/answer 86ae-27 allow
Result Stats: id=68b4dce6-08bc-4bda-a5ff-b28695588b43, total: 94103, in: 48, out: 28023, reason: 0, read: 988961, write: 66032, start: 1793545000843, end: 1793545004709, time: 3819, total: 214669, io: 0, control: 47, sent: 0, recv: 83
Ich habe den Test unter `/home/user/xyan/xy.ai.workbench/mcpc/tests/test_embedded_batch.py` angelegt, aber nicht ausgeführt. Er ruft Haiku wirklich auf.

Er startet einen `EmbeddedMcpc` ohne Built-in-Tools und lässt `run_batch` drei Prompts mit `Model.HAIKU` und `Effort.DISABLED` abarbeiten. Die Prompts sind 2+3, die Hauptstadt von Frankreich und das Wort "cat" rückwärts. Es laufen bis zu drei Agenten parallel. Geprüft wird, dass drei Ergebnisse zurückkommen, keines ein Fehler ist und jeder Text die erwartete Antwort enthält (`5`, `paris`, `tac`).

Der Test wird übersprungen, wenn das Paket `websockets` fehlt oder `claude` nicht im `PATH` liegt. Er braucht außerdem ein angemeldetes `claude` für das Default-Profil `none`, also `~/.claude-none`. Ohne Anmeldung schlägt er mit einem Fehlerergebnis fehl.
Auch ohne claude im path soll er fehlschalgen
SystemInit:  id=68b4dce6-08bc-4bda-a5ff-b28695588b43, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 86ae-28
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_embedded_batch.py
    start_marker: import shutil
    end_marker: "from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model"
    content: |-
      import pytest

      pytest.importorskip("websockets")

      from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model
    id: null
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_embedded_batch.py
    start_marker: pytestmark = pytest.mark.skipif
    end_marker: not available")
    content: ""
  reason: Skip-Bedingung für fehlendes claude entfernen
```
/answer 86ae-28 allow
Control Request:
```yaml
id: 86ae-29
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_embedded_batch.py
      error: No node matched the given text; a node selector (id) is required.
    - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_embedded_batch.py
      error: No node matched the given text; a node selector (id) is required.
```
/answer 86ae-29 allow
ReasoningToken: 141
Control Request:
```yaml
id: 86ae-2a
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_embedded_batch.py
    overwrite: true
    source: |
      import pytest

      pytest.importorskip("websockets")

      from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model

      PROMPTS = [
          "What is 2+3? Answer with the number only.",
          "What is the capital of France? Answer with one word.",
          "Spell the word 'cat' backwards. Answer with the word only.",
      ]
      EXPECTED = ["5", "paris", "tac"]


      def test_haiku_batch_over_three_prompts():
          with EmbeddedMcpc(builtin_tools=False, auto_approve=True) as mcpc:
              results = mcpc.run_batch(
                  PROMPTS,
                  "Answer as briefly as possible.",
                  model=Model.HAIKU,
                  effort=Effort.DISABLED,
                  max_workers=3)

          assert len(results) == len(PROMPTS)
          for result, expected in zip(results, EXPECTED):
              assert not result.is_error, result.text
              assert expected in result.text.lower()
  reason: Test ohne claude-Skip neu schreiben
```
/answer 86ae-2a allow
Result Stats: id=68b4dce6-08bc-4bda-a5ff-b28695588b43, total: 97137, in: 54, out: 29286, reason: 0, read: 1188506, write: 67797, start: 1793545083863, end: 1793545088734, time: 4828, total: 229155, io: 0, control: 43, sent: 0, recv: 47
Ich habe die Skip-Bedingung für fehlendes `claude` entfernt. Ohne `claude` im `PATH` schlägt der Test jetzt fehl. Dann bekommt jeder der drei Prompts ein Fehlerergebnis, und `assert not result.is_error` schlägt an. Ausgeführt habe ich den Test nicht.

Der Skip für fehlendes `websockets` ist noch drin. Ich kann ihn ebenfalls entfernen. Ohne `websockets` würde der Test dann mit einem `RuntimeError` aus `agent_config` fehlschlagen.
