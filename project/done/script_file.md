Der Source/Code Parameter der Scripting Tools soll modifiziert werden. Statt einem Script soll er auch einen absoluten oder relativen Dateipfad zu einer Script/Code Datei erlauben.

Das Tool prüft dann, ob nur ein Pfad und nicht mehr im Feld enthalten ist und ob dieser zu einer existierenden Datei auflöst. Eine Dateiendung wird nicht geprüft.

Tools:

- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py`
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py`
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_call/__init__.py`
- `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py`

Tests: `run_command` mit `runTests`
SystemInit:  id=f8bd9ce1-8614-4c23-94d9-d9343baef710, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
Control Request:
```yaml
id: d13c-2a8-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_call/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
  reason: Outline the four scripting tool files to understand how source/code parameter is handled
```
/answer d13c-2a8-0 allow
Control Request:
```yaml
id: d13c-2a9-2a8
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
        nodes:
        - id: qLJRy6|ALMpGp
          excert: '''Python tool – executes a Python script directly from context (no file).'''
        - id: Qow1uS|e9N7uW
          excert: import sys
        - id: lw7nNq|8ewVws
          excert: "__all__ = ['PythonError', 'python', 'PythonTool', 'register_python_tool']"
        - id: PythonError
          excert: "class PythonError(Exception):"
          docstring: Raised when a Python script cannot be executed.
        - id: python
          excert: "def python(script: str) -> ProcessResult:"
          docstring: "Feed ``script`` to a fresh Python interpreter on standard input. Args: script: …"
        - id: PythonTool
          excert: "class PythonTool(ToolDefinition):"
          children:
          - id: PythonTool.aSVAHa|CK4iaM
            excert: name = 'python'
          - id: PythonTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: Delegate to :func:`python` and pack the result into the MCP output schema.
        - id: register_python_tool
          excert: "def register_python_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
        nodes:
        - id: P3QdKz|6v1eAI
          excert: "'Bash tool – executes shell scripts, in parallel batches, each inside its own w…"
        - id: dTLvN6|XGQSft
          excert: from concurrent.futures import ThreadPoolExecutor
        - id: 04tLNt|nyjCCe
          excert: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
        - id: p7mDiR|xZMd5C
          excert: "__all__ = ['BashError', 'BashItem', 'BashItemError', 'bash', 'BashTool', 'regis…"
        - id: BashError
          excert: "class BashError(Exception):"
          docstring: Raised when a Bash script cannot be executed.
        - id: BashItem
          excert: "@dataclass(frozen=True) class BashItem:"
          docstring: "One Bash run. Attributes: cwd: Absolute path to working directory (must exist a…"
        - id: BashItemError
          excert: "@dataclass(frozen=True) class BashItemError:"
          docstring: Error running a single item.
        - id: bash_one
          excert: "def _bash_one(item: BashItem) -> ProcessResult:"
        - id: bash_one_safe
          excert: "def _bash_one_safe(item: BashItem | dict[str, Any]) -> ProcessResult | BashItemError:"
        - id: bash
          excert: "def bash(items: list[BashItem | dict[str, Any]]) -> list[ProcessResult | BashItemError]:"
          docstring: "Run each item with ``bash -c`` in its own working directory, in parallel. Args:…"
        - id: BashTool
          excert: "class BashTool(ToolDefinition):"
          children:
          - id: BashTool.dfhcZ0|rokhJy
            excert: name = 'bash'
          - id: BashTool.DrNd5W|24oi1E
            excert: "_ITEM_PROPERTIES = {"
          - id: BashTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: Delegate to :func:`bash` and pack each result into the MCP output schema.
        - id: register_bash_tool
          excert: "def register_bash_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_call/__init__.py
        nodes:
        - id: LGEYeC|KoQDTY
          excert: '"``tool_call`` – restricted Python context with on-demand tool injection.\n\n(:…'
        - id: IwsbCv|vh2IZc
          excert: import contextlib
        - id: YqZKuQ|ozT4qG
          excert: "__all__ = ["
        - id: ndDeFA|BTFIw1
          excert: "_SAFE_BUILTINS = {"
        - id: ToolCallError
          excert: "class ToolCallError(Exception):"
          docstring: Raised when *tool_ids* cannot be resolved/injected.
        - id: ToolCallExecution
          excert: "@dataclass(frozen=True) class ToolCallExecution:"
        - id: fresh_namespace
          excert: "def fresh_namespace() -> dict[str, Any]:"
        - id: sanitize_identifier
          excert: "def _sanitize_identifier(tool_id: str) -> str:"
        - id: inject_tools
          excert: "def inject_tools(functions: FunctionRegistry, namespace: dict[str, Any], tool_ids: list[str]) -> dict[str, str]:"
          docstring: "Bind every id in *tool_ids* into *namespace* under a valid identifier. Args: fu…"
        - id: spill
          excert: "def _spill(namespace: dict[str, Any], text: str, label: str) -> str:"
          docstring: Store *text* under a fresh variable name in *namespace*; return that name.
        - id: run_tool_call
          excert: "def run_tool_call(namespace: dict[str, Any], code: str) -> ToolCallExecution:"
          docstring: "Execute *code* against the persistent *namespace*, capturing STDOUT/STDERR. *na…"
        - id: ToolCallOutput
          excert: "@dataclass(frozen=True, kw_only=True) class ToolCallOutput(SinglePayload):"
          docstring: Captured output of a script; spilled streams are referenced by variable name. `…
        - id: ToolCallTool
          excert: "class ToolCallTool(ToolDefinition):"
          children:
          - id: ToolCallTool.GnfnyX|6fMqma
            excert: name = 'tool_call'
          - id: ToolCallTool.uJ3A02|eBCj54
            excert: "description = 'Run Python code against a restricted, session-persistent context…"
          - id: ToolCallTool.8dBrjf|BwbXB1
            excert: "input_schema = {"
          - id: ToolCallTool.init
            excert: "def __init__(self, functions: FunctionRegistry) -> None:"
          - id: ToolCallTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
        nodes:
        - id: wlNo2J|amT2KR
          excert: '"``ast_script`` – run restricted Python against the parsed AST.\n\nFor complex …'
        - id: e3mMSQ|stMXj1
          excert: import ast
        - id: l7jic4|D86RaJ
          excert: "__all__ = ['ScriptTree', 'ScriptError', 'AstScriptResult', 'ast_script', 'Scrip…"
        - id: Za6t1t|HuXBMZ
          excert: "_SAFE_BUILTINS = {"
        - id: ScriptTree
          excert: "class ScriptTree:"
          docstring: Engine-agnostic ``tree`` handle exposed to sandboxed scripts. Wraps a :class:`c…
          children:
          - id: ScriptTree.0KadZG|R2UlTU
            excert: '"Engine-agnostic ``tree`` handle exposed to sandboxed scripts.\n\n    Wraps a :…'
          - id: ScriptTree.init
            excert: "def __init__(self, tree: core.Tree) -> None:"
          - id: ScriptTree.raw
            excert: "@property def raw(self) -> Any:"
          - id: ScriptTree.source
            excert: "@property def source(self) -> str:"
          - id: ScriptTree.path
            excert: "@property def path(self) -> Any:"
          - id: ScriptTree.find
            excert: "def find(self, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> list[core.Located]:"
          - id: ScriptTree.locate_all
            excert: "def locate_all(self) -> list[core.Located]:"
          - id: ScriptTree.node_code
            excert: "def node_code(self, loc: core.Located) -> str:"
          - id: ScriptTree.replace
            excert: "def replace(self, loc: core.Located, code: str) -> str | None:"
          - id: ScriptTree.insert
            excert: "def insert(self, loc: core.Located, code: str, position: str='after') -> int:"
          - id: ScriptTree.delete
            excert: "def delete(self, loc: core.Located) -> None:"
          - id: ScriptTree.append
            excert: "def append(self, code: str) -> int:"
        - id: ScriptError
          excert: "class ScriptError(Exception):"
          docstring: Raised when an AST script cannot be run to completion.
        - id: AstScriptResult
          excert: "@dataclass(frozen=True) class AstScriptResult:"
          docstring: "Result of :func:`ast_script`. Attributes: result: Always ``\"success\"``. value: …"
        - id: ast_script
          excert: "def ast_script(path: str, code: str) -> AstScriptResult:"
          docstring: Execute ``code`` in a restricted sandbox exposing the file's tree as ``tree``. …
        - id: ScriptDone
          excert: "@dataclass(frozen=True, kw_only=True) class ScriptDone(Done):"
        - id: ScriptTool
          excert: "class ScriptTool(ToolDefinition):"
          children:
          - id: ScriptTool.dNdM46|IeCbts
            excert: name = 'ast_script'
          - id: ScriptTool.LwzTKu|ZSL46i
            excert: "input_schema = {"
          - id: ScriptTool.handle
            excert: "def handle(self, ctx: ToolContext) -> ToolResult:"
            docstring: "Delegate to :func:`ast_script`, translating the MCP schema to/from the Python A…"
        - id: register
          excert: "def register(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer d13c-2a9-2a8 allow
Control Request:
```yaml
id: d13c-2aa-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
    ids:
    - python
    - PythonTool
    - qLJRy6|ALMpGp
    - Qow1uS|e9N7uW
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
    ids:
    - BashItem
    - bash_one
    - BashTool
    - dTLvN6|XGQSft
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_call/__init__.py
    ids:
    - run_tool_call
    - ToolCallTool
    - IwsbCv|vh2IZc
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    ids:
    - ast_script
    - ScriptTool
    - e3mMSQ|stMXj1
  reason: Read relevant functions and tool classes
```
/answer d13c-2aa-0 allow
Control Request:
```yaml
id: d13c-2ab-2aa
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
        nodes:
        - id: python
          type: FunctionDef
          code: |-
            def python(script: str) -> ProcessResult:
                """Feed ``script`` to a fresh Python interpreter on standard input.

                Args:
                    script: Python script content to execute.

                Returns:
                    ProcessResult with:
                        exit_code: Exit code of Python process.
                        stdout: Standard output (up to 3000 chars; see stdout_file if longer).
                        stderr: Standard error output (up to 3000 chars; see stderr_file if longer).
                        stdout_file: Absolute path to temp file with full stdout if > 3000 chars.
                        stderr_file: Absolute path to temp file with full stderr if > 3000 chars.

                Raises:
                    PythonError: If Python binary cannot be launched.
                """
                try:
                    return run_process([sys.executable, '-'], stdin=script)
                except LaunchError as exc:
                    raise PythonError(f'Failed to launch Python: {exc}') from exc
        - id: PythonTool
          type: ClassDef
          excert: "class PythonTool(ToolDefinition):"
          children:
          - id: PythonTool.aSVAHa|CK4iaM
            type: statements
            code: |-
              name = 'python'
              title = 'Run Python script'
              description = 'Execute a Python script passed directly as content. Returns the exit code, standard output and, if present, standard error output.'
              input_schema = {
                  'type': 'object',
                  'properties': {
                      'script': {
                          'type': 'string',
                          'description': 'Python script content to execute.'}},
                  'required': ['script']}
          - id: PythonTool.handle
            type: FunctionDef
            code: |-
              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`python` and pack the result into the MCP output schema."""
                  args: dict[str, Any] = ctx.arguments
                  try:
                      result = python(args['script'])
                  except PythonError as exc:
                      return ToolResult.failure(str(exc))
                  return ToolResult.ok(ItemPayload(pack_process_result(result, ctx)))
        - id: qLJRy6|ALMpGp
          type: statements
          code: '''Python tool – executes a Python script directly from context (no file).'''
        - id: Qow1uS|e9N7uW
          type: imports
          code: |-
            import sys
            from typing import Any
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools.results import ItemPayload
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
        nodes:
        - id: BashItem
          type: ClassDef
          code: |-
            @dataclass(frozen=True)
            class BashItem:
                """One Bash run.

                Attributes:
                    cwd: Absolute path to working directory (must exist and be a directory).
                    script: Commands executed one after another in the same bash process.
                """
                cwd: str
                script: list[str]
        - id: bash_one
          type: FunctionDef
          code: |-
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
        - id: BashTool
          type: ClassDef
          excert: "class BashTool(ToolDefinition):"
          children:
          - id: BashTool.dfhcZ0|rokhJy
            type: statements
            code: |-
              name = 'bash'
              title = 'Run Bash scripts'
              description = f'Execute Bash scripts for a batch of items. Items run in parallel; Result holds exit code, stdout and stderr, or an error. Reduce the resulting token amount by filtering (grep, head, tail, wc -l). STDOUT and STDERR each have an output limit of {_MAX_STREAM_CHARS} characters per item.'
          - id: BashTool.DrNd5W|24oi1E
            type: statements
            code: |-
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
          - id: BashTool.handle
            type: FunctionDef
            code: |-
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
                          return ToolResult.failure(f'Invalid item: {exc!r}')
                  try:
                      outcomes = bash(items)
                  except BashError as exc:
                      return ToolResult.failure(str(exc))
                  results: list[ProcessOutput] = []
                  errors: list[ProcessItemError] = []
                  for index, outcome in enumerate(outcomes):
                      if isinstance(outcome, BashItemError):
                          errors.append(ProcessItemError(index=index, error=outcome.error))
                          continue
                      results.append(
                          pack_process_result(
                              outcome,
                              ctx,
                              normalize_output=True,
                              omit_zero_exit_code=True,
                              max_stream_chars=_MAX_STREAM_CHARS,
                              index=index))
                  return ToolResult.ok(ProcessBatchResult(results=results, errors=errors))
        - id: dTLvN6|XGQSft
          type: imports
          code: |-
            from concurrent.futures import ThreadPoolExecutor
            from dataclasses import dataclass
            from pathlib import Path
            from typing import Any
            from xy.ai.mcpc.tools._tool_helpers import batch_schema, require_items
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_call/__init__.py
        nodes:
        - id: run_tool_call
          type: FunctionDef
          code: |-
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
        - id: ToolCallTool
          type: ClassDef
          excert: "class ToolCallTool(ToolDefinition):"
          children:
          - id: ToolCallTool.GnfnyX|6fMqma
            type: statements
            code: |-
              name = 'tool_call'
              '# Generic executor of every registered function, hence all features are reachable through it.'
              features = frozenset({ToolFeature.INPUT_LINENO, ToolFeature.INPUT_ASTID, ToolFeature.PROVIDES_OUTLINE})
              title = 'Run a script against injected tools'
          - id: ToolCallTool.uJ3A02|eBCj54
            type: statements
            code: "description = 'Run Python code against a restricted, session-persistent context. Only text sent to print() is returned to the context — variable assignments alone (e.g. `r = tool(...)`) are persisted only. Complex objects are not auto-formatted: inspect their structure (e.g. via the type sources shown by tool_usage) and explicitly iterate/print the fields you need (e.g. `for x in r.results: print(x[\"title\"])`).'"
          - id: ToolCallTool.8dBrjf|BwbXB1
            type: statements
            code: |-
              input_schema = {
                  'type': 'object',
                  'properties': {
                      'tool_ids': {
                          'type': 'array',
                          'items': {
                              'type': 'string'},
                          'description': "Ids of functions to inject into 'code' as same-named variables."},
                      'code': {
                          'type': 'string',
                                  'description': 'Python script; restricted builtins, no imports. Must explicitly print() any result you want to see — assignments are session-persistent. iterate/extract fields yourself before printing.'}},
                  'required': [
                      'tool_ids',
                      'code']}
          - id: ToolCallTool.init
            type: FunctionDef
            code: |-
              def __init__(self, functions: FunctionRegistry) -> None:
                  self._functions = functions
          - id: ToolCallTool.handle
            type: FunctionDef
            code: |-
              def handle(self, ctx: ToolContext) -> ToolResult:
                  args: dict[str, Any] = ctx.arguments
                  namespace: dict[str, Any] = ctx.session.state.setdefault(_NAMESPACE_STATE_KEY, fresh_namespace())
                  try:
                      inject_tools(self._functions, namespace, list(args.get('tool_ids', [])))
                  except ToolCallError as exc:
                      return ToolResult.failure(str(exc))
                  execution = run_tool_call(namespace, args['code'])
                  fields: dict[str, str] = {}
                  warnings: list[str] = []
                  if len(execution.stdout) > STREAM_SPILL_THRESHOLD:
                      var_name = _spill(namespace, execution.stdout, 'stdout')
                      fields['stdout_var'] = var_name
                      warnings.append(f"STDOUT exceeded {STREAM_SPILL_THRESHOLD} characters and was stored as '{var_name}' in the persistent context. Filter it (e.g. slicing, splitlines(), grep-like logic) and print only what's needed via a follow-up tool_call using '{var_name}'.")
                  else:
                      fields['stdout'] = execution.stdout
                  if execution.stderr:
                      if len(execution.stderr) > STREAM_SPILL_THRESHOLD:
                          var_name = _spill(namespace, execution.stderr, 'stderr')
                          fields['stderr_var'] = var_name
                          warnings.append(
                              f"STDERR exceeded {STREAM_SPILL_THRESHOLD} characters and was stored as '{var_name}' in the persistent context. Filter it and re-print via a follow-up tool_call using '{var_name}'.")
                      else:
                          fields['stderr'] = execution.stderr
                  return ToolResult.ok(ToolCallOutput(exception=execution.error, warnings=tuple(warnings), **fields))
        - id: IwsbCv|vh2IZc
          type: imports
          code: |-
            import contextlib
            import io
            from dataclasses import dataclass, field
            from typing import Any
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools.results import SinglePayload
            from xy.ai.mcpc.tools.tool_registry import ToolFeature
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
        nodes:
        - id: ast_script
          type: FunctionDef
          code: |-
            def ast_script(path: str, code: str) -> AstScriptResult:
                """Execute ``code`` in a restricted sandbox exposing the file's tree as ``tree``.

                ``code`` runs with an empty ``__builtins__`` plus a small, curated set of safe
                names (see ``_SAFE_BUILTINS``); the only capabilities handed in are ``tree``
                (a :class:`ScriptTree`) and the standard-library ``ast`` module itself. Any
                mutation made through ``tree`` is persisted to ``path`` on success, for any
                file type the AST tools support (Python or tree-sitter).

                Args:
                    path: Absolute path to the file whose AST is exposed as ``tree``.
                    code: Python script to execute against ``tree``. May assign a module-level
                        name ``result`` to return an arbitrary value (reported as its ``repr()``).

                Returns:
                    AstScriptResult: Success status and, if the script set one, the ``repr()``
                    of its ``result`` variable.

                Raises:
                    core.AstError: If ``path`` is invalid.
                    ScriptError: If ``code`` has a syntax error, or raises during execution.
                """
                file_path = core.require_path(path)
                tree = core.CACHE.get_tree(file_path)
                env: dict[str, Any] = {'tree': ScriptTree(tree), 'ast': ast}
                sandbox_globals = {'__builtins__': _SAFE_BUILTINS}
                try:
                    '# noqa: S102'
                    exec(compile(code, '<ast-script>', 'exec'), sandbox_globals, env)
                except SyntaxError as exc:
                    raise ScriptError(f'Script syntax error: {exc.msg}') from exc
                except Exception as exc:
                    '# noqa: BLE001'
                    raise ScriptError(f'Script failed: {type(exc).__name__}: {exc}') from exc
                core.CACHE.save(file_path, tree)
                if 'result' in env:
                    return AstScriptResult(result='success', value=repr(env['result']))
                return AstScriptResult(result='success')
        - id: ScriptTool
          type: ClassDef
          excert: "class ScriptTool(ToolDefinition):"
          children:
          - id: ScriptTool.dNdM46|IeCbts
            type: statements
            code: |-
              name = 'ast_script'
              title = 'Run AST script'
              description = "Run a restricted Python script against a file's AST when the single-operation ast_* tools (list/find/read/replace/insert/delete) are insufficient for a complex, multi-step, or conditional transform. Sandboxed globals: 'tree' and 'ast'; no imports; assign 'result' to return data; edits made via 'tree' are saved automatically. See the 'code' parameter for the full tree API and node-attribute reference."
          - id: ScriptTool.LwzTKu|ZSL46i
            type: statements
            code: |-
              input_schema = {
                  'type': 'object',
                  'properties': {
                      'path': {
                          'type': 'string',
                          'description': 'Absolute path to the file.'},
                      'code': {
                          'type': 'string',
                          'description': "Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limited to isinstance/issubclass/getattr/setattr/hasattr/delattr/len/list/dict/set/tuple/str/int/float/bool/enumerate/range/sorted/reversed/zip/map/filter/any/all/min/max/sum/type/repr. tree API: find(id=None, node_type=None, name=None, parent_type=None)->list[Located]; locate_all()->list[Located] (every addressable node, document order); node_code(loc)->str; replace(loc, code)->str|None; insert(loc, code, position='after'|'before')->int; delete(loc)->None; append(code)->int; source->str; path. Located attrs (engine-independent, mirror the id/type/lines/code fields used by ast_outline/ast_find/ast_read/ast_edit_*/ast_replace/ast_insert/ast_delete): node_id (unique dotted name/hash path from root, e.g. 'MyClass.method' — the id used by every ast_* tool); node_type (reported type, e.g. 'FunctionDef'/'pair'); name (simple name or None); lineno/end_lineno (1-based inclusive span, i.e. the 'lines' field elsewhere); parent_type (enclosing node's type, None at top level); expandable (True = pure container of nested defs, i.e. what 'read' descends into instead of inlining code); node/parent (opaque node handles for use only as tree.* arguments, never introspect their internals); index (position among parent's addressable children); tree (owning Tree; use its .source/.path, nothing else). Note: 'signature'/'docstring' shown by list/find/read (one-line header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed."}},
                  'required': [
                      'path',
                      'code']}
          - id: ScriptTool.handle
            type: FunctionDef
            code: |-
              def handle(self, ctx: ToolContext) -> ToolResult:
                  """Delegate to :func:`ast_script`, translating the MCP schema to/from the Python API."""
                  args: dict[str, Any] = ctx.arguments
                  try:
                      result = ast_script(args['path'], args['code'])
                  except (core.AstError, ScriptError) as exc:
                      return ToolResult.failure(str(exc))
                  return ToolResult.ok(ScriptDone(result=result.result, value=result.value))
        - id: e3mMSQ|stMXj1
          type: imports
          code: |-
            import ast
            from dataclasses import dataclass
            from typing import Any
            from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.ast import core
            from xy.ai.mcpc.tools.function_registry import FunctionRegistry
            from xy.ai.mcpc.tools.results import Done
```
/answer d13c-2ab-2aa allow
ReasoningToken: 205
Control Request:
```yaml
id: d13c-2ac-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc
    pattern: run_tool_call|ast_script\(|python\(|_bash_one|bash\(
    include:
    - '*.py'
    exclude:
    - .venv/**
    - node_modules/**
    limit: 40
  reason: Inspect shared helpers module and existing tests for these tools
```
/answer d13c-2ac-0 allow
Control Request:
```yaml
id: d13c-2ad-2ac
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc
        matches:
        - path: tests/test_ast_engines.py
          matches:
          - "def test_snippet_defaults_to_python():"
        - path: tests/test_tools_family_smoke.py
          matches:
          - "def test_tool_search_to_tool_call_round_trip_with_bash(registry: ToolRegistry):"
          - "assert usage.structured_content['results'][0]['signature'].startswith('bash(')"
          - "code=\"r = bash([{'cwd': '/tmp', 'script': ['echo hi']}])[0]\\nprint(r.stdout.strip())\\nkept = r.exit_code\")"
        - path: tests/test_http_human_in_the_loop.py
          matches:
          - "def call_bash():"
        - path: src/xy/ai/mcpc/tools/ast/script.py
          matches:
          - "def ast_script(path: str, code: str) -> AstScriptResult:"
          - "result = ast_script(args['path'], args['code'])"
        - path: src/xy/ai/mcpc/tools/bash/__init__.py
          matches:
          - "def _bash_one(item: BashItem) -> ProcessResult:"
          - "def _bash_one_safe(item: BashItem | dict[str, Any]) -> ProcessResult | BashItemError:"
          - return _bash_one(item)
          - "def bash(items: list[BashItem | dict[str, Any]]) -> list[ProcessResult | BashItemError]:"
          - "return list(pool.map(_bash_one_safe, items))"
          - outcomes = bash(items)
        - path: src/xy/ai/mcpc/tools/tool_call/__init__.py
          matches:
          - "'run_tool_call',"
          - "def run_tool_call(namespace: dict[str, Any], code: str) -> ToolCallExecution:"
          - "execution = run_tool_call(namespace, args['code'])"
        - path: src/xy/ai/mcpc/tools/python/__init__.py
          matches:
          - "def python(script: str) -> ProcessResult:"
          - "result = python(args['script'])"
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
        nodes:
        - id: j2F2xB|e5OyD1
          excert: '''Common helpers for tool definition and request handling.'''
        - id: pXkVK0|aIkesy
          excert: "from typing import Any, Callable, TypeVar"
        - id: xQ71by|MKAujh
          excert: "__all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serializ…"
        - id: batch_schema
          excert: "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str) -> dict[str, Any]:"
          docstring: "Build the standard ``{items: [...]}`` input schema shared by all batch tools. N…"
        - id: require_items
          excert: "def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':"
          docstring: "Return the non-empty list at *key* in ``ctx.arguments``, or an error result. Re…"
        - id: serialize_batch_result
          excert: "def serialize_batch_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None) -> dict[str, Any]:"
          docstring: Serialize the populated ``results``/``errors`` lists of a batch result. Warning…
        - id: batch_tool_result
          excert: "def batch_tool_result(batch_result: BatchResult[Any, Any], result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:"
          docstring: Wrap a batch result into a :class:`ToolResult` carrying its call-wide warnings.
        - id: handle_batch_tool
          excert: "def handle_batch_tool(ctx: ToolContext, item_factory: Callable[[dict[str, Any]], T], batch_fn: Callable[[list[T]], BatchResult[Any, Any]], error_class: type, result_payload: ResultPayload | None=None, auto_approve: bool=False) -> ToolResult:"
          docstring: "Common handler for batch-processing tools. Args: ctx: Tool context with argumen…"
```
/answer d13c-2ad-2ac allow
ReasoningToken: 84
Control Request:
```yaml
id: d13c-2ae-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    ids:
    - pXkVK0|aIkesy
    - xQ71by|MKAujh
  reason: Read helper module header/imports and __all__
```
/answer d13c-2ae-0 allow
Control Request:
```yaml
id: d13c-2af-2ae
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
        nodes:
        - id: pXkVK0|aIkesy
          type: imports
          code: |-
            from typing import Any, Callable, TypeVar
            from xy.ai.mcpc.tools.tool_registry import ToolResult
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.results import BatchResult
        - id: xQ71by|MKAujh
          type: statements
          code: |-
            __all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serialize_batch_result', 'batch_schema']
            T = TypeVar('T')
            ResultPayload = Callable[[Any], dict[str, Any]]
```
/answer d13c-2af-2ae allow
ReasoningToken: 180
Control Request:
```yaml
id: d13c-2b0-0
toolName: ast_edit
arguments:
  edit_marks:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: xQ71by|MKAujh
    start_marker: "__all__ = ['require_items'"
    end_marker: "'batch_schema']"
    content: "__all__ = ['require_items', 'handle_batch_tool', 'batch_tool_result', 'serialize_batch_result', 'batch_schema', 'resolve_script_source']"
  insert:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: batch_schema
    position: before
    source: |+
      def resolve_script_source(source: str, base_dir: 'str | Path | None'=None) -> str:
          """Return script text, loading it from a file if *source* is just a file path.

          *source* is treated as a path only if, after stripping surrounding
          whitespace, it is a single line (nothing but the path) that resolves to an
          existing regular file. The file extension is not checked. Otherwise
          *source* is returned unchanged and treated as script content.

          Args:
              source: Script content, or an absolute/relative path to a script file.
              base_dir: Directory against which a relative path is resolved
                  (default: current working directory).

          Returns:
              The file's text content if *source* names an existing file, else
              *source* itself.

          Raises:
              OSError: If the file exists but cannot be read.
              UnicodeDecodeError: If the file is not valid UTF-8 text.
          """
          candidate = source.strip()
          if not candidate or '\n' in candidate or '\r' in candidate:
              return source
          try:
              path = Path(candidate)
              if not path.is_absolute() and base_dir is not None:
                  path = Path(base_dir) / path
              if not path.is_file():
                  return source
          except (OSError, ValueError):
              '# e.g. name too long or embedded NUL: not a path, so plain script content'
              return source
          return path.read_text(encoding='utf-8')


  reason: Add shared helper and wire it into the four scripting tools
```
/answer d13c-2b0-0 allow
ReasoningToken: 58
Control Request:
```yaml
id: d13c-2b1-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
    id: pXkVK0|aIkesy
    old_text: "from typing import Any, Callable, TypeVar"
    new_text: |-
      from pathlib import Path
      from typing import Any, Callable, TypeVar
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
    id: python
    old_text: |2
              script: Python script content to execute.
    new_text: |2
              script: Python script content to execute, or an absolute/relative path
                  to a script file (used if the value is only a path to an existing file).
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
    id: python
    old_text: |2-
          try:
              return run_process([sys.executable, '-'], stdin=script)
    new_text: |2-
          try:
              script = resolve_script_source(script)
          except (OSError, UnicodeDecodeError) as exc:
              raise PythonError(f'Failed to read script file: {exc}') from exc
          try:
              return run_process([sys.executable, '-'], stdin=script)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
    id: PythonTool.aSVAHa|CK4iaM
    old_text: '''Python script content to execute.'''
    new_text: "'Python script content to execute, or an absolute or relative path to a script file (only if the value consists of nothing but a path to an existing file).'"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
    id: Qow1uS|e9N7uW
    old_text: |
      import sys
    new_text: |
      import sys
      from xy.ai.mcpc.tools._tool_helpers import resolve_script_source
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
    id: dTLvN6|XGQSft
    old_text: "from xy.ai.mcpc.tools._tool_helpers import batch_schema, require_items"
    new_text: "from xy.ai.mcpc.tools._tool_helpers import batch_schema, require_items, resolve_script_source"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
    id: bash_one
    old_text: |2-
          try:
              return run_process(['bash', '-c', '\n'.join(item.script)], cwd=cwd_path)
    new_text: |2-
          script = item.script
          if len(script) == 1:
              try:
                  script = [resolve_script_source(script[0], cwd_path)]
              except (OSError, UnicodeDecodeError) as exc:
                  raise BashError(f'Failed to read script file: {exc}') from exc
          try:
              return run_process(['bash', '-c', '\n'.join(script)], cwd=cwd_path)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
    id: BashTool.DrNd5W|24oi1E
    old_text: '''Bash commands executed sequentially in the same process.'''
    new_text: "'Bash commands executed sequentially in the same process. Alternatively a single entry holding only an absolute path, or a path relative to cwd, of an existing script file, whose content is then executed.'"
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_call/__init__.py
    id: IwsbCv|vh2IZc
    old_text: |
      import contextlib
      import io
    new_text: |
      import contextlib
      import io
      from xy.ai.mcpc.tools._tool_helpers import resolve_script_source
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_call/__init__.py
    id: ToolCallTool.handle
    old_text: "    execution = run_tool_call(namespace, args['code'])"
    new_text: |2-
          try:
              code = resolve_script_source(args['code'])
          except (OSError, UnicodeDecodeError) as exc:
              return ToolResult.failure(f'Failed to read script file: {exc}')
          execution = run_tool_call(namespace, code)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_call/__init__.py
    id: ToolCallTool.8dBrjf|BwbXB1
    old_text: "'Python script; restricted builtins, no imports."
    new_text: "'Python script, or an absolute/relative path to a script file (only if the value is nothing but a path to an existing file); restricted builtins, no imports."
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    id: e3mMSQ|stMXj1
    old_text: |
      import ast
    new_text: |
      import ast
      from xy.ai.mcpc.tools._tool_helpers import resolve_script_source
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    id: ast_script
    old_text: "    env: dict[str, Any] = {'tree'"
    new_text: |2-
          try:
              code = resolve_script_source(code)
          except (OSError, UnicodeDecodeError) as exc:
              raise ScriptError(f'Failed to read script file: {exc}') from exc
          env: dict[str, Any] = {'tree'
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    id: ast_script
    old_text: "        code: Python script to execute against ``tree``. May"
    new_text: |2-
              code: Python script to execute against ``tree``, or an absolute/relative
                  path to a script file (used if the value is only a path to an existing
                  file). May
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/script.py
    id: ScriptTool.LwzTKu|ZSL46i
    old_text: '"Python source executed with ''tree'''
    new_text: "\"Python source (or an absolute/relative path to a script file, if the value is nothing but a path to an existing file) executed with 'tree'"
  reason: "Import Path in helpers; apply resolve_script_source in python, bash, tool_call, ast_script"
```
/answer d13c-2b1-0 allow
Control Request:
```yaml
id: d13c-2b2-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/mcpc
    id: runTests
  reason: Run the test suite to verify the changes
```
/answer d13c-2b2-0 allow
ReasoningToken: 45
Control Request:
```yaml
id: d13c-2b3-2b2
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 1
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 365 items

        tests/test_ast_engines.py ..................                             [  4%]
        tests/test_ast_imports.py ....................                           [ 10%]
        tests/test_ast_imports_rpc.py ...........                                [ 13%]
        tests/test_ast_retrieve.py ......                                        [ 15%]
        tests/test_ast_rust_bridge.py ....                                       [ 16%]
        tests/test_cli_session.py .................                              [ 20%]
        tests/test_edit_diagnostics.py .......................                   [ 27%]
        tests/test_edit_tolerance.py ...........................                 [ 34%]
        tests/test_embedded.py .............                                     [ 38%]
        tests/test_embedded_batch.py F                                           [ 38%]
        tests/test_grep_with_ast.py ...                                          [ 39%]
        tests/test_http_human_in_the_loop.py ..                                  [ 39%]
        tests/test_http_tool_calls.py ...                                        [ 40%]
        tests/test_import_all.py ............................................... [ 53%]
        ........................................................................ [ 73%]
        .....                                                                    [ 74%]
        tests/test_mcpcignore.py ........................                        [ 81%]
        tests/test_persist.py ..........                                         [ 83%]
        tests/test_result_contract.py ..................................         [ 93%]
        tests/test_rpc_java_engine.py ..........                                 [ 95%]
        tests/test_smoke.py ..                                                   [ 96%]
        tests/test_tools_family_smoke.py ...                                     [ 97%]
        tests/test_validate_arguments.py ..........                              [100%]

        =================================== FAILURES ===================================
        _____________________ test_haiku_batch_over_three_prompts ______________________

            def test_haiku_batch_over_three_prompts():
                with EmbeddedMcpc(builtin_tools=False, auto_approve=True) as mcpc:
                    results = mcpc.run_batch(
                        PROMPTS,
                        'Answer as briefly as possible.',
                        model=Model.HAIKU,
                        effort=Effort.DISABLED,
                        cc_profile='personal',
                        max_workers=3)
                assert len(results) == len(PROMPTS)
                for result, expected in zip(results, EXPECTED):
        >           assert not result.is_error, result.text
        E           AssertionError: You've hit your session limit · resets 10:10pm (Europe/Berlin)
        E           assert not True
        E            +  where True = CliResult(text="You've hit your session limit · resets 10:10pm (Europe/Berlin)", is_error=True, subtype='success', sta...t_tokens=0, output_tokens=0, cache_read=0, cache_create=0, reasoning_tokens=0, cost_usd=0.0, duration_ms=513, turns=1)).is_error

        tests/test_embedded_batch.py:20: AssertionError
        =========================== short test summary info ============================
        FAILED tests/test_embedded_batch.py::test_haiku_batch_over_three_prompts - As...
        ======================== 1 failed, 364 passed in 9.96s =========================
```
/answer d13c-2b3-2b2 allow
Control Request:
```yaml
id: d13c-2b4-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
    id: qLJRy6|ALMpGp
    old_text: executes a Python script directly from context (no file).
    new_text: executes a Python script given as content or as a path to a script file.
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/python/__init__.py
    id: PythonTool.aSVAHa|CK4iaM
    old_text: Execute a Python script passed directly as content.
    new_text: Execute a Python script passed as content or as a file path.
  reason: Update outdated Python tool docstring/description that says "no file"
```
/answer d13c-2b4-0 allow
Result Stats: id=f8bd9ce1-8614-4c23-94d9-d9343baef710, total: 35137, in: 16, out: 6615, reason: 0, read: 125341, write: 28506, start: 1795122480512, end: 1795122488102, time: 7542, total: 66321, io: 0, control: 48, sent: 0, recv: 169
Die vier Scripting Tools akzeptieren jetzt statt eines Scripts auch einen Dateipfad. Neue Tests dafür habe ich nicht geschrieben, und ein echter Lauf mit einer Script-Datei steht noch aus.

`runTests` ergibt 364 bestanden und 1 fehlgeschlagen. Der Fehler ist `test_embedded_batch.py::test_haiku_batch_over_three_prompts`. Er scheitert am Session-Limit des Claude-Zugangs («You've hit your session limit») und hat mit der Änderung nichts zu tun.

**Neuer Helper `resolve_script_source(source, base_dir=None)` in `tools/_tool_helpers.py`**
- Das Feld gilt als Pfad, wenn es nach `strip()` einzeilig ist und zu einer existierenden Datei auflöst. Eine Dateiendung wird nicht geprüft.
- Relative Pfade werden gegen `base_dir` aufgelöst, sonst gegen das aktuelle Arbeitsverzeichnis des Prozesses. `~` wird nicht expandiert.
- In allen anderen Fällen, also bei mehrzeiligem Text, einer nicht existierenden Datei oder einem Verzeichnis, bleibt der Wert unverändert und wird wie bisher als Script ausgeführt.
- Ist die Datei nicht lesbar oder kein UTF-8, gibt es einen Fehler pro Tool: `PythonError`, `BashError`, `ScriptError` bzw. `ToolResult.failure`.

**Anbindung der Tools**
- **`python`:** `python()` löst `script` auf. Die Beschreibung im Schema, das Docstring und das Modul-Docstring habe ich angepasst; der Hinweis «no file» ist weg.
- **`bash`:** Hat die `script`-Liste genau einen Eintrag, wird dieser geprüft. Relative Pfade werden gegen das `cwd` des jeweiligen Items aufgelöst, und der Dateiinhalt wird per `bash -c` ausgeführt. Die Schema-Beschreibung ist ergänzt.
- **`tool_call`:** `ToolCallTool.handle` löst `code` auf, bevor `run_tool_call` läuft. Die Schema-Beschreibung ist ergänzt.
- **`ast_script`:** `ast_script()` löst `code` auf. Docstring und Schema-Beschreibung sind ergänzt.

Soll ich Tests für den Helper und die vier Tools ergänzen?
Ja, Tests
SystemInit:  id=f8bd9ce1-8614-4c23-94d9-d9343baef710, cwd=/home/user/xyan/xy.ai.workbench/mcpc, model=claude-sonnet-5-5
ReasoningToken: 53
Control Request:
```yaml
id: d13c-2b5-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/mcpc/tests/test_tools_family_smoke.py
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/tests
    pattern: ast_script|ToolContext\(|def registry|PythonTool|ScriptTool
    include:
    - '*.py'
    limit: 30
  reason: "Look at existing test conventions (fixtures, how tools are invoked) before writing new tests"
```
/answer d13c-2b5-0 allow
Control Request:
```yaml
id: d13c-2b6-2b5
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/mcpc/tests
        matches:
        - path: test_ast_retrieve.py
          matches:
          - "return tool.handler(ToolContext(session=session, arguments=arguments, registry=registry))"
        - path: test_grep_with_ast.py
          matches:
          - "return tool.handler(ToolContext(session=session, arguments=arguments))"
        - path: conftest.py
          matches:
          - "def registry() -> ToolRegistry:"
        - path: test_tools_family_smoke.py
          matches:
          - "return tool.handler(ToolContext(session=session, arguments=arguments))"
        - path: test_persist.py
          matches:
          - "return ToolContext(session=Session(id='persist'), arguments={})"
          - "ctx = ToolContext(session=session, arguments={}, registry=registry)"
          - ToolContext(
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_tools_family_smoke.py
        nodes:
        - id: txcIR2|FU57Fp
          excert: '''Smoke test for the ``tools`` family (tool_search/tool_usage/tool_call).\n\nExe…'
        - id: RIgXMF|XZ5OPl
          excert: from __future__ import annotations
        - id: call
          excert: "def _call(registry: ToolRegistry, session: Session, tool_name: str, **arguments):"
        - id: test_tools_alias_groups_the_three_tools
          excert: "def test_tools_alias_groups_the_three_tools(registry: ToolRegistry):"
        - id: hf4b6e67f
          excert: "def test_tool_search_to_tool_call_round_trip_with_bash(registry: ToolRegistry):"
        - id: test_tool_call_rejects_unknown_tool_id
          excert: "def test_tool_call_rejects_unknown_tool_id(registry: ToolRegistry):"
```
/answer d13c-2b6-2b5 allow
Control Request:
```yaml
id: d13c-2b7-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_tools_family_smoke.py
    ids:
    - RIgXMF|XZ5OPl
    - call
    - hf4b6e67f
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/conftest.py
  reason: Read the smoke test file and conftest to mirror fixtures and result access patterns
```
/answer d13c-2b7-0 allow
Control Request:
```yaml
id: d13c-2b8-2b7
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_tools_family_smoke.py
        nodes:
        - id: RIgXMF|XZ5OPl
          type: imports
          code: |-
            from __future__ import annotations
            from xy.ai.mcpc.server.session import Session
            from xy.ai.mcpc.tools.tool_registry import ToolRegistry
            from xy.ai.mcpc.tools.tool_context import ToolContext
            from xy.ai.mcpc.tools.tool_call import STREAM_SPILL_THRESHOLD
        - id: call
          type: FunctionDef
          code: |-
            def _call(registry: ToolRegistry, session: Session, tool_name: str, **arguments):
                tool = registry.get(tool_name)
                assert tool is not None, f'tool not registered: {tool_name}'
                return tool.handler(ToolContext(session=session, arguments=arguments))
        - id: hf4b6e67f
          type: FunctionDef
          code: |-
            def test_tool_search_to_tool_call_round_trip_with_bash(registry: ToolRegistry):
                session = Session(id='smoke-tools')
                '# 1) discover `bash` by keyword; the same search must not repeat it.'
                search_result = _call(registry, session, 'tool_search', keywords='bash working directory')
                names = [t['name'] for t in search_result.structured_content['results']]
                assert 'bash' in names
                repeat = _call(registry, session, 'tool_search', keywords='bash working directory')
                assert not repeat.structured_content.get('results')
                '# 2) inspect its usage; repeating the same request yields a hint, not the info again.'
                usage = _call(registry, session, 'tool_usage', names=['bash'])
                assert usage.structured_content['results'][0]['signature'].startswith('bash(')
                usage_repeat = _call(registry, session, 'tool_usage', names=['bash'])
                assert 'already returned' in usage_repeat.structured_content['errors'][0]['error']
                '# 3) call `bash` via tool_call; result and a new variable persist in the session.'
                call1 = _call(
                    registry,
                    session,
                    'tool_call',
                    tool_ids=['bash'],
                    code="r = bash([{'cwd': '/tmp', 'script': ['echo hi']}])[0]\nprint(r.stdout.strip())\nkept = r.exit_code")
                assert call1.structured_content['stdout'] == 'hi\n'
                assert not call1.is_error
                '# 4) session persistence: `kept` survives into a fresh call without re-injecting tools.'
                call2 = _call(registry, session, 'tool_call', tool_ids=[], code='print(kept)')
                assert call2.structured_content['stdout'] == '0\n'
                '# 5) STDOUT spilling: oversized output is stored under a variable, not returned inline.'
                call3 = _call(registry, session, 'tool_call', tool_ids=[], code=f"print('x' * {STREAM_SPILL_THRESHOLD + 1})")
                assert 'stdout' not in call3.structured_content
                spill_var = call3.structured_content['stdout_var']
                assert spill_var in call3.warnings[0]
                '# 6) the spilled content is retrievable by name in a later call (context persists).'
                call4 = _call(registry, session, 'tool_call', tool_ids=[], code=f'print(len({spill_var}))')
                assert call4.structured_content['stdout'] == f'{STREAM_SPILL_THRESHOLD + 2}\n'
      - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/conftest.py
        nodes:
        - id: qmgX3W|tqzV3O
          excert: "'Test setup: make the ``src`` layout importable without installation.'"
        - id: 5VUlLh|r01CsA
          excert: from __future__ import annotations
        - id: iRf0tk|qB8MoX
          excert: SRC = Path(__file__).resolve().parent.parent / 'src'
        - id: x3hNJ6|DzQJSD
          excert: import pytest
        - id: 3Mpw8b|t0uVQE
          excert: "'# noqa: E402'"
        - id: Rvzd9A|64yxm4
          excert: from xy.ai.mcpc.cli.manager import CliSessionManager
        - id: 3Mpw8b|t0uVQE_1
          excert: "'# noqa: E402'"
        - id: ISA9QJ|HpQxxJ
          excert: from xy.ai.mcpc.config import ServerConfig
        - id: 3Mpw8b|t0uVQE_2
          excert: "'# noqa: E402'"
        - id: u9WjXo|amOnuc
          excert: from xy.ai.mcpc.server.session import SessionStore
        - id: 3Mpw8b|t0uVQE_3
          excert: "'# noqa: E402'"
        - id: CMkg8Q|JCVbVw
          excert: from xy.ai.mcpc.tools import register_tools
        - id: 3Mpw8b|t0uVQE_4
          excert: "'# noqa: E402'"
        - id: HuFMzr|NS1kqD
          excert: "from xy.ai.mcpc.tools.agent.profiles import DEFAULT_PROFILES, ProfileRegistry"
        - id: 3Mpw8b|t0uVQE_5
          excert: "'# noqa: E402'"
        - id: 53fjeN|DGVP0G
          excert: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
        - id: 3Mpw8b|t0uVQE_6
          excert: "'# noqa: E402'"
        - id: Sf6qeQ|UMWgY5
          excert: from xy.ai.mcpc.tools.tool_context import AppEnvironment
        - id: 3Mpw8b|t0uVQE_7
          excert: "'# noqa: E402'"
        - id: PQJTLs|q1ToOF
          excert: from xy.ai.mcpc.tools.tool_registry import ToolRegistry
        - id: registry
          excert: "@pytest.fixture def registry() -> ToolRegistry:"
          docstring: "A ``ToolRegistry`` populated by ``register_tools()``, wired like the real serve…"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/mcpc/tests/conftest.py instead."
```
/answer d13c-2b8-2b7 allow
ReasoningToken: 316
Control Request:
```yaml
id: d13c-2b9-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/tests/test_script_file_source.py
    source: |
      """Tests for passing a script file path instead of script content.

      Covers the shared helper ``resolve_script_source`` and its use by the
      ``python``, ``bash``, ``tool_call`` and ``ast_script`` tools.
      """
      from __future__ import annotations

      from pathlib import Path

      import pytest

      from xy.ai.mcpc.server.session import Session
      from xy.ai.mcpc.tools._tool_helpers import resolve_script_source
      from xy.ai.mcpc.tools.ast.script import ScriptError, ast_script
      from xy.ai.mcpc.tools.bash import BashError, bash
      from xy.ai.mcpc.tools.python import PythonError, python
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.tool_registry import ToolRegistry


      def _call(registry: ToolRegistry, tool_name: str, **arguments):
          tool = registry.get(tool_name)
          assert tool is not None, f'tool not registered: {tool_name}'
          return tool.handler(ToolContext(session=Session(id=f'script-file-{tool_name}'), arguments=arguments))


      # --- resolve_script_source ----------------------------------------------------


      def test_resolve_absolute_path_returns_file_content(tmp_path: Path):
          script = tmp_path / 'job'  # no extension on purpose: extensions are not checked
          script.write_text('print(1)\n', encoding='utf-8')
          assert resolve_script_source(str(script)) == 'print(1)\n'


      def test_resolve_tolerates_surrounding_whitespace(tmp_path: Path):
          script = tmp_path / 'a.txt'
          script.write_text('X', encoding='utf-8')
          assert resolve_script_source(f'  {script}\n') == 'X'


      def test_resolve_relative_path_uses_base_dir(tmp_path: Path):
          (tmp_path / 'rel.sh').write_text('echo rel', encoding='utf-8')
          assert resolve_script_source('rel.sh', tmp_path) == 'echo rel'


      def test_resolve_relative_path_uses_process_cwd_by_default(tmp_path: Path, monkeypatch: pytest.MonkeyPatch):
          (tmp_path / 'rel.py').write_text('print(2)', encoding='utf-8')
          monkeypatch.chdir(tmp_path)
          assert resolve_script_source('rel.py') == 'print(2)'


      def test_resolve_absolute_path_ignores_base_dir(tmp_path: Path):
          script = tmp_path / 'abs.txt'
          script.write_text('abs', encoding='utf-8')
          other = tmp_path / 'other'
          other.mkdir()
          assert resolve_script_source(str(script), other) == 'abs'


      @pytest.mark.parametrize(
          'source',
          [
              '',
              '   ',
              'print(1)',  # plain code, not an existing file
              '/definitely/not/existing/script.py',
              'x' * 5000,  # name too long for the file system
              'bad\x00name',  # embedded NUL
          ])
      def test_resolve_non_files_are_returned_unchanged(source: str):
          assert resolve_script_source(source) == source


      def test_resolve_directory_is_not_a_file(tmp_path: Path):
          assert resolve_script_source(str(tmp_path)) == str(tmp_path)


      def test_resolve_path_plus_more_content_is_script_content(tmp_path: Path):
          script = tmp_path / 'a.py'
          script.write_text('print(1)', encoding='utf-8')
          multi_line = f'{script}\nprint(2)'
          assert resolve_script_source(multi_line) == multi_line
          with_args = f'{script} --flag'
          assert resolve_script_source(with_args) == with_args


      def test_resolve_unreadable_encoding_raises(tmp_path: Path):
          script = tmp_path / 'bin'
          script.write_bytes(b'\xff\xfe\x00\x80')
          with pytest.raises(UnicodeDecodeError):
              resolve_script_source(str(script))


      # --- python -------------------------------------------------------------------


      def test_python_runs_script_file(tmp_path: Path):
          script = tmp_path / 'hello'
          script.write_text("print('from file')", encoding='utf-8')
          result = python(str(script))
          assert result.exit_code == 0
          assert result.stdout.strip() == 'from file'


      def test_python_still_runs_inline_content():
          assert python("print('inline')").stdout.strip() == 'inline'


      def test_python_non_utf8_file_raises_python_error(tmp_path: Path):
          script = tmp_path / 'bin'
          script.write_bytes(b'\xff\xfe\x00\x80')
          with pytest.raises(PythonError):
              python(str(script))


      def test_python_tool_handler_runs_script_file(registry: ToolRegistry, tmp_path: Path):
          script = tmp_path / 'tool.py'
          script.write_text("print('via tool')", encoding='utf-8')
          result = _call(registry, 'python', script=str(script))
          assert not result.is_error
          assert 'via tool' in str(result.structured_content)


      # --- bash ---------------------------------------------------------------------


      def test_bash_runs_single_entry_script_file_with_absolute_path(tmp_path: Path):
          script = tmp_path / 'run'
          script.write_text('echo one\necho two', encoding='utf-8')
          [result] = bash([{'cwd': str(tmp_path), 'script': [str(script)]}])
          assert result.exit_code == 0
          assert result.stdout.split() == ['one', 'two']


      def test_bash_resolves_relative_path_against_item_cwd(tmp_path: Path):
          (tmp_path / 'rel.sh').write_text('pwd', encoding='utf-8')
          [result] = bash([{'cwd': str(tmp_path), 'script': ['rel.sh']}])
          assert result.exit_code == 0
          assert Path(result.stdout.strip()).resolve() == tmp_path.resolve()


      def test_bash_multi_entry_list_is_never_treated_as_path(tmp_path: Path):
          script = tmp_path / 'run.sh'
          script.write_text('echo from-file', encoding='utf-8')
          [result] = bash([{'cwd': str(tmp_path), 'script': ['echo inline', f'# {script}']}])
          assert result.stdout.strip() == 'inline'


      def test_bash_single_inline_command_still_works(tmp_path: Path):
          [result] = bash([{'cwd': str(tmp_path), 'script': ['echo hi']}])
          assert result.stdout.strip() == 'hi'


      def test_bash_unreadable_script_file_reports_item_error(tmp_path: Path):
          script = tmp_path / 'bin'
          script.write_bytes(b'\xff\xfe\x00\x80')
          [outcome] = bash([{'cwd': str(tmp_path), 'script': [str(script)]}])
          assert not hasattr(outcome, 'exit_code')
          assert 'script file' in outcome.error


      def test_bash_error_type_is_exported():
          assert issubclass(BashError, Exception)


      # --- tool_call ----------------------------------------------------------------


      def test_tool_call_runs_code_file(registry: ToolRegistry, tmp_path: Path):
          script = tmp_path / 'code'
          script.write_text('answer = 6 * 7\nprint(answer)', encoding='utf-8')
          result = _call(registry, 'tool_call', tool_ids=[], code=str(script))
          assert not result.is_error
          assert result.structured_content['stdout'] == '42\n'


      def test_tool_call_inline_code_still_works(registry: ToolRegistry):
          result = _call(registry, 'tool_call', tool_ids=[], code='print(1 + 1)')
          assert result.structured_content['stdout'] == '2\n'


      def test_tool_call_unreadable_file_is_failure(registry: ToolRegistry, tmp_path: Path):
          script = tmp_path / 'bin'
          script.write_bytes(b'\xff\xfe\x00\x80')
          result = _call(registry, 'tool_call', tool_ids=[], code=str(script))
          assert result.is_error


      # --- ast_script ---------------------------------------------------------------


      def test_ast_script_runs_code_file(tmp_path: Path):
          target = tmp_path / 'target.py'
          target.write_text('x = 1\n', encoding='utf-8')
          script = tmp_path / 'edit_script'
          script.write_text('result = len(tree.source)', encoding='utf-8')
          outcome = ast_script(str(target), str(script))
          assert outcome.result == 'success'
          assert outcome.value == repr(len('x = 1\n'))


      def test_ast_script_inline_code_still_works(tmp_path: Path):
          target = tmp_path / 'target.py'
          target.write_text('x = 1\n', encoding='utf-8')
          outcome = ast_script(str(target), 'result = 1 + 1')
          assert outcome.value == '2'


      def test_ast_script_unreadable_file_raises_script_error(tmp_path: Path):
          target = tmp_path / 'target.py'
          target.write_text('x = 1\n', encoding='utf-8')
          script = tmp_path / 'bin'
          script.write_bytes(b'\xff\xfe\x00\x80')
          with pytest.raises(ScriptError):
              ast_script(str(target), str(script))
  reason: Create tests for file-path support of the scripting tools
```
/answer d13c-2b9-0 allow
Control Request:
```yaml
id: d13c-2ba-0
toolName: run_command
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/mcpc
    id: runTests
  reason: Run the test suite including the new tests
```
/answer d13c-2ba-0 allow
Control Request:
```yaml
id: d13c-2bb-2ba
result:
  structuredContent:
    results:
    - index: 0
      stdout: |
        ============================= test session starts ==============================
        platform linux -- Python 3.12.3, pytest-8.3.5, pluggy-1.6.0
        rootdir: /home/user/xyan/xy.ai.workbench/mcpc
        configfile: pyproject.toml
        plugins: anyio-4.8.0, langsmith-0.4.18, docker-3.1.2
        collected 395 items

        tests/test_ast_engines.py ..................                             [  4%]
        tests/test_ast_imports.py ....................                           [  9%]
        tests/test_ast_imports_rpc.py ...........                                [ 12%]
        tests/test_ast_retrieve.py ......                                        [ 13%]
        tests/test_ast_rust_bridge.py ....                                       [ 14%]
        tests/test_cli_session.py .................                              [ 19%]
        tests/test_edit_diagnostics.py .......................                   [ 25%]
        tests/test_edit_tolerance.py ...........................                 [ 31%]
        tests/test_embedded.py .............                                     [ 35%]
        tests/test_embedded_batch.py .                                           [ 35%]
        tests/test_grep_with_ast.py ...                                          [ 36%]
        tests/test_http_human_in_the_loop.py ..                                  [ 36%]
        tests/test_http_tool_calls.py ...                                        [ 37%]
        tests/test_import_all.py ............................................... [ 49%]
        ........................................................................ [ 67%]
        .....                                                                    [ 68%]
        tests/test_mcpcignore.py ........................                        [ 74%]
        tests/test_persist.py ..........                                         [ 77%]
        tests/test_result_contract.py ..................................         [ 86%]
        tests/test_rpc_java_engine.py ..........                                 [ 88%]
        tests/test_script_file_source.py ..............................          [ 96%]
        tests/test_smoke.py ..                                                   [ 96%]
        tests/test_tools_family_smoke.py ...                                     [ 97%]
        tests/test_validate_arguments.py ..........                              [100%]

        ============================= 395 passed in 10.35s =============================
```
/answer d13c-2bb-2ba allow
Result Stats: id=f8bd9ce1-8614-4c23-94d9-d9343baef710, total: 50627, in: 26, out: 12291, reason: 0, read: 284248, write: 38310, start: 1795122626409, end: 1795122632411, time: 5955, total: 115859, io: 0, control: 47, sent: 0, recv: 135
Ich habe 30 neue Tests in `tests/test_script_file_source.py` ergänzt. Der gesamte Lauf mit `runTests` ergibt 395 von 395 bestanden. Der vorher fehlgeschlagene Haiku-Test lief diesmal durch, das Session-Limit war also nur vorübergehend.

**Helper `resolve_script_source`**
- Ein absoluter Pfad liefert den Dateiinhalt, auch ohne Dateiendung.
- Whitespace um den Pfad herum wird ignoriert.
- Relative Pfade werden gegen `base_dir` aufgelöst, sonst gegen das aktuelle Arbeitsverzeichnis.
- Ein absoluter Pfad ignoriert `base_dir`.
- Diese Werte kommen unverändert zurück: leer, nur Whitespace, Code, nicht existierende Datei, zu langer Name, NUL-Zeichen, Verzeichnis.
- Dasselbe gilt für «Pfad plus mehr» (mehrzeilig oder mit Argumenten): Der Wert gilt dann als Script.
- Eine Datei, die kein UTF-8 ist, löst `UnicodeDecodeError` aus.

**`python`**
- Eine Script-Datei läuft.
- Inline-Code läuft weiterhin.
- Eine Datei, die kein UTF-8 ist, löst `PythonError` aus.
- Der Tool-Handler über die Registry läuft mit einer Script-Datei.

**`bash`**
- Ein Eintrag mit absolutem Pfad läuft.
- Ein relativer Pfad wird gegen das `cwd` des Items aufgelöst.
- Eine Liste mit mehreren Einträgen wird nie als Pfad behandelt.
- Ein einzelner Inline-Befehl läuft weiterhin.
- Eine nicht lesbare Datei erzeugt einen Item-Fehler.

**`tool_call`**
- Eine Code-Datei läuft.
- Inline-Code läuft weiterhin.
- Eine nicht lesbare Datei liefert ein `ToolResult.failure`.

**`ast_script`**
- Eine Code-Datei läuft.
- Inline-Code läuft weiterhin.
- Eine nicht lesbare Datei löst `ScriptError` aus.
