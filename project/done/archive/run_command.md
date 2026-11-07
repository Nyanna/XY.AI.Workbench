Implementiere ein neues Tool "run_command" analog `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py` im Package `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash`.
Dieses basiert auf dem Bash Tool aber unter speziellen Voraussetzungen.

Eingabe:
- cwd: CWD analog dem Bash Aufruf
- id: eine eindeutige ID eines Kommandos
- values: Eine Liste von Platzhalternamen mit Replacements Values

## Funktionsweise
Der Nutzer hinterlegt im Prompt manuelle Anweisungen zur Verwendung des MCP-Tools. Er weist beispielweise den Agenten an "run_command" mit einer bestimmten ID und bestimmten Values Aufzurufen.

1. Das Tool prüft zunächst, ob im CWD eine Datei "commands.json" liegt.
2. Diese wird eingelesen und wenn nciht ein Fehler zurückgegeben
3. Die Datei enthält Mapping Liste von ID auf ein String der ein Bashkommando oder Skript darstellt und mit optionalen Platzhaltern versehen ist.
4. Die Platzhalter werden ersetzt und das Kommando via Bash ausgeführt.
5. STDOUT und STDERR werden analog dem Bash Tool zurückgegeben

## Zielsetzung
Das Ziel ist dem Agenten ein spezifisches Subset von Bashkommandos zu erlauben, ohne das dies für ihn Transparent ist.
Beispielweise kann ID:"runTests" definiert werden das unsichtbar ein "cargo test" ausführt.
Der Agent muss die spezifische Teststrategie nicht in Erfahrung bringen und es abstrahiert Projektübergreifend.
Beispielweise kann ID:"Cargo.toml" definiert werden das intern zu "cat Cargo.toml" gemapped wird.
Es wird ebenso ein Flag in der JSON Datei benötigt mit der, der Output Spill-to-Disk deaktiviert werden kann.

- Das Mapping ist Verzeichnis spezifisch da vom CWD Abhängig
SystemInit:  id=2a97172e-ec44-438b-8363-70b065eb2b0b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: b622-35
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash
  reason: Inspect bash tool package
```
/answer b622-35 allow
Control Request:
```yaml
id: b622-36
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash
      entries:
      - ".:"
      - __init__.py
```
/answer b622-36 allow
Control Request:
```yaml
id: b622-37
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash
    script:
    - cat __init__.py
  reason: Read the existing bash tool
```
/answer b622-37 deny benutze AST
Control Request:
```yaml
id: b622-38
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
  reason: Read structure of bash tool
```
/answer b622-38 allow
Control Request:
```yaml
id: b622-39
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
      nodes:
      - id: P3QdKz|6v1eAI
        signature: "'Bash tool – executes shell scripts, in parallel batches, each inside its own w…"
      - id: dTLvN6|XGQSft
        signature: from concurrent.futures import ThreadPoolExecutor
      - id: 53fjeN|DGVP0G
        signature: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: p7mDiR|xZMd5C
        signature: "__all__ = ['BashError', 'BashItem', 'BashItemError', 'bash', 'BashTool', 'regis…"
      - id: BashError
        signature: "class BashError(Exception):"
        docstring: Raised when a Bash script cannot be executed.
      - id: BashItem
        signature: "@dataclass(frozen=True) class BashItem:"
        docstring: "One Bash run. Attributes: cwd: Absolute path to working directory (must exist a…"
      - id: BashItemError
        signature: "@dataclass(frozen=True) class BashItemError:"
        docstring: Error running a single item.
      - id: bash_one
        signature: "def _bash_one(item: BashItem) -> ProcessResult:"
      - id: bash_one_safe
        signature: "def _bash_one_safe(item: BashItem | dict[str, Any]) -> ProcessResult | BashItemError:"
      - id: bash
        signature: "def bash(items: list[BashItem | dict[str, Any]]) -> list[ProcessResult | BashItemError]:"
        docstring: "Run each item with ``bash -c`` in its own working directory, in parallel. Args:…"
      - id: BashTool
        signature: "class BashTool(ToolDefinition):"
        children:
        - id: BashTool.dfhcZ0|rokhJy
          signature: name = 'bash'
        - id: BashTool.DrNd5W|24oi1E
          signature: "_ITEM_PROPERTIES = {"
        - id: BashTool.handle
          signature: "def handle(self, ctx: ToolContext) -> ToolResult:"
          docstring: Delegate to :func:`bash` and pack each result into the MCP output schema.
      - id: register_bash_tool
        signature: "def register_bash_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
```
/answer b622-39 allow
Control Request:
```yaml
id: b622-3a
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
  regexp: .
  node_type: Module
  reason: Read the full source
```
/answer b622-3a allow
Control Request:
```yaml
id: b622-3b
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
      nodes: []
```
/answer b622-3b allow
Control Request:
```yaml
id: b622-3c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
    ids:
    - P3QdKz|6v1eAI
    - dTLvN6|XGQSft
    - 53fjeN|DGVP0G
    - p7mDiR|xZMd5C
    - BashError
    - BashItem
    - BashItemError
    - bash_one
    - bash_one_safe
    - bash
    - BashTool
    - register_bash_tool
  reason: Read all nodes of bash tool
```
/answer b622-3c allow
Control Request:
```yaml
id: b622-3d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
      nodes:
      - id: P3QdKz|6v1eAI
        type: statements
        code: "'Bash tool – executes shell scripts, in parallel batches, each inside its own working directory.'"
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
      - id: 53fjeN|DGVP0G
        type: imports
        code: from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      - id: p7mDiR|xZMd5C
        type: statements
        code: |-
          __all__ = ['BashError', 'BashItem', 'BashItemError', 'bash', 'BashTool', 'register_bash_tool']
          _MAX_STREAM_CHARS = 3000
      - id: BashError
        type: ClassDef
        code: |-
          class BashError(Exception):
              """Raised when a Bash script cannot be executed."""
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
      - id: BashItemError
        type: ClassDef
        code: |-
          @dataclass(frozen=True)
          class BashItemError:
              """Error running a single item."""
              error: str
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
      - id: bash_one_safe
        type: FunctionDef
        code: |-
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
      - id: bash
        type: FunctionDef
        code: |-
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
      - id: BashTool
        type: ClassDef
        signature: "class BashTool(ToolDefinition):"
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
      - id: register_bash_tool
        type: FunctionDef
        code: |-
          def register_bash_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
              registry.register(BashTool())
              functions.register(bash)
```
/answer b622-3d allow
ReasoningToken: 42
Control Request:
```yaml
id: b622-3e
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
    pattern: def pack_process_result|def batch_schema|def require_items|register_bash_tool|spill
    include:
    - '*.py'
    exclude:
    - __pycache__/**
    limit: 30
  reason: "Find pack_process_result, helpers, and registration sites"
```
/answer b622-3e allow
Control Request:
```yaml
id: b622-3f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools
      matches:
      - path: process.py
        matches:
        - "(stream normalisation, spill-to-file safety limit, ``exit_code`` omission)."
        - "def _spill_to_file(text: str, label: str) -> str:"
        - "def _spill_extra_info(path: str, ctx: ToolContext) -> dict[str, Any]:"
        - '"""Compute the extra structured info attached for a spilled-to-disk stream.'
        - "def pack_process_result(result: ProcessResult, ctx: ToolContext, *, normalize_output: bool=False, omit_zero_exit_code: bool=False, max_stream_chars: int | None=None) -> ToolResult:"
        - "stdout_file = _spill_to_file(stdout, 'stdout')"
        - "for key, value in _spill_extra_info(stdout_file, ctx).items():"
        - "stderr_file = _spill_to_file(stderr, 'stderr')"
        - "for key, value in _spill_extra_info(stderr_file, ctx).items():"
      - path: bash/__init__.py
        matches:
        - "__all__ = ['BashError', 'BashItem', 'BashItemError', 'bash', 'BashTool', 'register_bash_tool']"
        - "def register_bash_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:"
      - path: _tool_helpers.py
        matches:
        - "def batch_schema(item_properties: dict[str, Any], required: list[str], description: str, *, additional_properties: bool | None=None) -> dict[str, Any]:"
        - "def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':"
      - path: __init__.py
        matches:
        - from xy.ai.mcpc.tools.bash import register_bash_tool
        - "register_bash_tool(registry, functions)"
      - path: tool_call/__init__.py
        matches:
        - "def _spill(namespace: dict[str, Any], text: str, label: str) -> str:"
        - "counter = namespace.get('_tool_call_spill_counter', 0) + 1"
        - "namespace['_tool_call_spill_counter'] = counter"
        - "var_name = f'_{label}_spill_{counter}'"
        - "var_name = _spill(namespace, execution.stdout, 'stdout')"
        - "var_name = _spill(namespace, execution.stderr, 'stderr')"
```
/answer b622-3f allow
Control Request:
```yaml
id: b622-40
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py
  name: pack_process_result
  reason: Read pack_process_result to see how to disable spill
```
/answer b622-40 allow
Control Request:
```yaml
id: b622-41
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py
      nodes:
      - id: pack_process_result
        code: |-
          def pack_process_result(result: ProcessResult, ctx: ToolContext, *, normalize_output: bool=False, omit_zero_exit_code: bool=False, max_stream_chars: int | None=None) -> ToolResult:
              """Pack a :class:`ProcessResult` into the MCP output schema.

              * ``normalize_output`` — when ``True``, post-process STDOUT/STDERR to
                improve YAML block-scalar compatibility (see :func:`_normalize_stream`).
              * ``omit_zero_exit_code`` — when ``True``, ``exit_code`` is left out of the
                result entirely if the process exited with code ``0``.
              * ``max_stream_chars`` — safety limit on the number of characters of
                STDOUT/STDERR returned inline.  When a stream exceeds this limit, its
                full content is written to a temp file instead; the structured
                result then carries the absolute path (``stdout_file``/``stderr_file``)
                in place of the raw text, plus ``stdout_file_stats``/``stderr_file_stats``
                (and, when the ``ast_outline`` tool is enabled, ``stdout_file_outline``/
                ``stderr_file_outline``) describing that file, so the caller can keep
                operating on it (e.g. with the ``read`` tool) without the oversized
                content ever passing through the result payload.  ``None`` (the
                default) disables the limit.

              ``stdout`` is always present; ``stderr`` is included whenever it is
              non-empty. The result carries no separate text content block —
              ``structured_content`` alone conveys STDOUT/STDERR, avoiding duplication.

              A non-zero ``exit_code`` is a normal outcome of running *cmd* (the tool
              call itself succeeded), not an MCP-level failure — ``is_error`` is
              therefore never set here.
              """
              stdout = result.stdout
              stderr = result.stderr
              if normalize_output:
                  stdout = _normalize_stream(stdout)
                  stderr = _normalize_stream(stderr)
              content: list[dict[str, Any]] = []
              structured: dict[str, Any] = {}
              if not omit_zero_exit_code or result.exit_code != 0:
                  structured['exit_code'] = result.exit_code
              if max_stream_chars is not None and len(stdout) > max_stream_chars:
                  stdout_file = _spill_to_file(stdout, 'stdout')
                  content.append(
                      text_content(
                          f'STDOUT limit of characters exceeded, output saved to `{stdout_file}`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline)'))
                  structured['stdout_file'] = stdout_file
                  for key, value in _spill_extra_info(stdout_file, ctx).items():
                      structured[f'stdout_file_{key}'] = value
              else:
                  structured['stdout'] = stdout
              if stderr:
                  if max_stream_chars is not None and len(stderr) > max_stream_chars:
                      stderr_file = _spill_to_file(stderr, 'stderr')
                      content.append(
                          text_content(
                              f'STDERR limit of characters exceeded, output saved to `{stderr_file}`. Reduce the resulting token amount: filter it with targeted commands (grep, head, tail, awk, ast_outline).'))
                      structured['stderr_file'] = stderr_file
                      for key, value in _spill_extra_info(stderr_file, ctx).items():
                          structured[f'stderr_file_{key}'] = value
                  else:
                      structured['stderr'] = stderr
              '# Simple success with auto_approve when exit code is 0 and both streams are empty'
              if result.exit_code == 0 and (not stdout) and (not stderr):
                  return ToolResult(structured_content={'result': 'success'}, auto_approve=False)
              return ToolResult(content=content, structured_content=structured)
```
/answer b622-41 allow
Control Request:
```yaml
id: b622-42
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py
  text: bash
  reason: See registration site
```
/answer b622-42 allow
Control Request:
```yaml
id: b622-43
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py
      nodes:
      - id: lYo10t|5akFp5
        code: |-
          from xy.ai.mcpc.tools.tool_registry import ToolRegistry
          from xy.ai.mcpc.tools.tool_context import AppEnvironment
          from xy.ai.mcpc.tools.function_registry import FunctionRegistry
          from xy.ai.mcpc.tools.ast import register_ast_tools
          from xy.ai.mcpc.tools.bash import register_bash_tool
          from xy.ai.mcpc.tools.colgrep import register_colgrep_tool
          from xy.ai.mcpc.tools.list import register_list_tool
          from xy.ai.mcpc.tools.markdown import register_markdown_tool
      - id: register_tools.LpqV1K|lT9580
        code: |-
          register_bash_tool(registry, functions)
          register_python_tool(registry, functions)
          register_markdown_tool(registry, environment)
          register_ast_tools(registry, functions)
          register_ask_user_tool(registry)
          register_colgrep_tool(registry, functions)
          register_skills(registry, environment)
          register_exa_tools(registry, environment)
          register_github_tools(registry, environment)
          register_context7_tools(registry, environment)
          register_openalex_tools(registry, environment)
          register_agent_tools(registry, environment)
```
/answer b622-43 allow
ReasoningToken: 712
Control Request:
```yaml
id: b622-44
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
  name: require_items
  reason: See how ctx exposes arguments
```
/answer b622-44 allow
Control Request:
```yaml
id: b622-45
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/_tool_helpers.py
      nodes:
      - id: require_items
        code: |-
          def require_items(ctx: ToolContext, key: str='items') -> 'tuple[list[Any], ToolResult | None]':
              """Return the non-empty list at *key* in ``ctx.arguments``, or an error result.

              Returns:
                  ``(values, None)`` on success, or ``([], error_result)`` if *key* is missing or
                  empty; callers must return ``error_result`` immediately in that case.
              """
              values = ctx.arguments.get(key) or []
              if not values:
                  return ([], ToolResult(content=[text_content(f"'{key}' must be a non-empty list.")], is_error=True))
              return (values, None)
```
/answer b622-45 allow
Control Request:
```yaml
id: b622-46
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py
    source: |
      """Run-command tool – executes predefined, directory-specific commands from ``commands.json`` by ID."""
      import json
      import re
      import shlex
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process

      __all__ = ['COMMANDS_FILE', 'RunCommandError', 'RunCommandConfig', 'load_commands', 'render_command', 'run_command', 'RunCommandTool', 'register_run_command_tool']

      COMMANDS_FILE = 'commands.json'
      _MAX_STREAM_CHARS = 3000
      _PLACEHOLDER = re.compile(r'\{\{\s*(\w+)\s*\}\}')


      class RunCommandError(Exception):
          """Raised when a command cannot be resolved or executed."""


      @dataclass(frozen=True)
      class RunCommandConfig:
          """Parsed ``commands.json``.

          File format::

              {"spill_to_disk": false, "commands": {"runTests": "cargo test", "read": "cat {{file}}"}}

          Attributes:
              commands: Mapping of command ID to bash script with optional ``{{name}}`` placeholders.
              spill_to_disk: If false, oversized output is returned inline instead of being written to a file.
          """
          commands: dict[str, str]
          spill_to_disk: bool = True


      def load_commands(cwd: Path) -> RunCommandConfig:
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
          if not isinstance(commands, dict) or not all(isinstance(k, str) and isinstance(v, str) for k, v in commands.items()):
              raise RunCommandError(f"'{COMMANDS_FILE}': 'commands' must be an object mapping IDs to strings.")
          spill = data.get('spill_to_disk', True)
          if not isinstance(spill, bool):
              raise RunCommandError(f"'{COMMANDS_FILE}': 'spill_to_disk' must be a boolean.")
          return RunCommandConfig(commands=commands, spill_to_disk=spill)


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


      def run_command(cwd: str, id: str, values: dict[str, str] | None = None) -> tuple[ProcessResult, RunCommandConfig]:
          """Run the command registered under *id* in ``<cwd>/commands.json`` with ``bash -c``.

          Raises:
              RunCommandError: If cwd is invalid, the file or ID is missing, placeholders do not match, or launch fails.
          """
          cwd_path = Path(cwd)
          if not cwd_path.is_absolute():
              raise RunCommandError('cwd must be an absolute path.')
          if not cwd_path.is_dir():
              raise RunCommandError('Working directory not found or not a directory.')
          config = load_commands(cwd_path)
          if id not in config.commands:
              raise RunCommandError(f"Unknown command id '{id}'. Available: {sorted(config.commands)}")
          script = render_command(config.commands[id], values or {})
          try:
              return run_process(['bash', '-c', script], cwd=cwd_path), config
          except LaunchError as exc:
              raise RunCommandError(f'Failed to launch bash: {exc}') from exc


      class RunCommandTool(ToolDefinition):
          name = 'run_command'
          title = 'Run predefined command'
          description = (
              f"Execute a predefined command by ID, as configured in '{COMMANDS_FILE}' of the working directory. "
              'Placeholder values are passed as name/value pairs. Result holds exit code, stdout and stderr, or an error.')
          input_schema = {
              'type': 'object',
              'properties': {
                  'cwd': {'type': 'string', 'description': f"Absolute path to the working directory containing '{COMMANDS_FILE}'."},
                  'id': {'type': 'string', 'description': 'ID of the command.'},
                  'values': {
                      'type': 'array',
                      'description': 'Placeholder replacements.',
                      'items': {
                          'type': 'object',
                          'properties': {'name': {'type': 'string'}, 'value': {'type': 'string'}},
                          'required': ['name', 'value']}}},
              'required': ['cwd', 'id']}

          def handle(self, ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              cwd, cmd_id = args.get('cwd'), args.get('id')
              raw_values = args.get('values') or []
              if not isinstance(cwd, str) or not isinstance(cmd_id, str):
                  return ToolResult(content=[text_content("'cwd' and 'id' must be strings.")], is_error=True)
              try:
                  values = {v['name']: v['value'] for v in raw_values}
              except (KeyError, TypeError) as exc:
                  return ToolResult(content=[text_content(f'Invalid values: {exc!r}')], is_error=True)
              try:
                  result, config = run_command(cwd, cmd_id, values)
              except RunCommandError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              return pack_process_result(
                  result,
                  ctx,
                  normalize_output=True,
                  omit_zero_exit_code=True,
                  max_stream_chars=_MAX_STREAM_CHARS if config.spill_to_disk else None)


      def register_run_command_tool(registry: ToolRegistry) -> None:
          registry.register(RunCommandTool())
  reason: Create run_command module in bash package
```
/answer b622-46 allow
Control Request:
```yaml
id: b622-47
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
    id: register_bash_tool
    source: |-
      def register_bash_tool(registry: ToolRegistry, functions: FunctionRegistry) -> None:
          registry.register(BashTool())
          register_run_command_tool(registry)
          functions.register(bash)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
    id: 53fjeN|DGVP0G
    source: |-
      from xy.ai.mcpc.tools.function_registry import FunctionRegistry
      from xy.ai.mcpc.tools.bash.run_command import register_run_command_tool
  reason: Register run_command alongside bash tool
```
/answer b622-47 allow
Control Request:
```yaml
id: b622-48
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/__init__.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py
  reason: Check both files compile
```
/answer b622-48 allow
Result Stats: id=2a97172e-ec44-438b-8363-70b065eb2b0b, total: 27741, in: 20, out: 6666, reason: 0, read: 112676, write: 21055, start: 1793915256269, end: 1793915264012, time: 7692, total: 75247, io: 0, control: 51, sent: 0, recv: 187
Ich habe das Tool `run_command` angelegt. Beide Dateien kompilieren. Ausgeführt oder getestet habe ich das Tool noch nicht.

**Dateien**
- Neu ist `tools/bash/run_command.py`.
- In `tools/bash/__init__.py` ruft `register_bash_tool` jetzt zusätzlich `register_run_command_tool` auf. Dadurch musste ich `tools/__init__.py` nicht ändern.

**Eingabe**
- `cwd` ist ein absoluter Pfad, `id` die ID des Kommandos.
- `values` ist optional, eine Liste von `{name, value}`.

**Aufbau von `commands.json` im CWD**
```json
{
  "spill_to_disk": false,
  "commands": {
    "runTests": "cargo test",
    "Cargo.toml": "cat Cargo.toml",
    "readFile": "cat {{file}}"
  }
}
```
`spill_to_disk` ist optional und standardmäßig `true`.

**Ablauf**
1. Das Tool liest `commands.json` aus dem CWD. Fehlt die Datei oder ist sie ungültig, gibt es einen Fehler zurück.
2. Es sucht die ID heraus. Ist sie unbekannt, nennt der Fehler die verfügbaren IDs. Damit sieht der Agent die Kommandos doch, wenn er eine falsche ID verwendet. Wenn du das nicht willst, entferne ich die Liste aus der Meldung.
3. Es ersetzt die Platzhalter `{{name}}` und führt das Ergebnis mit `bash -c` im CWD aus.
4. STDOUT und STDERR kommen genau wie beim Bash-Tool zurück, mit normalisierter Ausgabe und ohne `exit_code` bei 0. Das Limit liegt bei 3000 Zeichen.
5. Mit `spill_to_disk: false` wird dieses Limit abgeschaltet und die Ausgabe immer vollständig inline zurückgegeben.

**Festlegungen**
- **Quoting:** Die Werte werden mit `shlex.quote` eingesetzt, damit der Agent keine Shell-Syntax einschleusen kann. Steht ein Platzhalter schon in Anführungszeichen (`"{{x}}"`), würde das doppelt gequotet. Schreib ihn dann ohne Anführungszeichen.
- **Platzhalter:** Fehlende oder überzählige Platzhalter ergeben einen Fehler.
- **Kein Batch:** Das Tool führt pro Aufruf genau ein Kommando aus, nicht mehrere parallel wie das Bash-Tool.
- **Funktionsregistry:** Ich habe `run_command` nicht in der `FunctionRegistry` registriert, nur als Tool.
"spill_to_disk" muss pro Mapping konfiguriert werden können
SystemInit:  id=2a97172e-ec44-438b-8363-70b065eb2b0b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: b622-49
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py
    overwrite: true
    source: |
      """Run-command tool – executes predefined, directory-specific commands from ``commands.json`` by ID."""
      import json
      import re
      import shlex
      from dataclasses import dataclass
      from pathlib import Path
      from typing import Any
      from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult, text_content
      from xy.ai.mcpc.tools.tool_context import ToolContext
      from xy.ai.mcpc.tools.process import LaunchError, ProcessResult, pack_process_result, run_process

      __all__ = ['COMMANDS_FILE', 'RunCommandError', 'CommandSpec', 'load_commands', 'render_command', 'run_command', 'RunCommandTool', 'register_run_command_tool']

      COMMANDS_FILE = 'commands.json'
      _MAX_STREAM_CHARS = 3000
      _PLACEHOLDER = re.compile(r'\{\{\s*(\w+)\s*\}\}')


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


      def run_command(cwd: str, id: str, values: dict[str, str] | None = None) -> tuple[ProcessResult, CommandSpec]:
          """Run the command registered under *id* in ``<cwd>/commands.json`` with ``bash -c``.

          Raises:
              RunCommandError: If cwd is invalid, the file or ID is missing, placeholders do not match, or launch fails.
          """
          cwd_path = Path(cwd)
          if not cwd_path.is_absolute():
              raise RunCommandError('cwd must be an absolute path.')
          if not cwd_path.is_dir():
              raise RunCommandError('Working directory not found or not a directory.')
          commands = load_commands(cwd_path)
          spec = commands.get(id)
          if spec is None:
              raise RunCommandError(f"Unknown command id '{id}'. Available: {sorted(commands)}")
          script = render_command(spec.script, values or {})
          try:
              return run_process(['bash', '-c', script], cwd=cwd_path), spec
          except LaunchError as exc:
              raise RunCommandError(f'Failed to launch bash: {exc}') from exc


      class RunCommandTool(ToolDefinition):
          name = 'run_command'
          title = 'Run predefined command'
          description = (
              f"Execute a predefined command by ID, as configured in '{COMMANDS_FILE}' of the working directory. "
              'Placeholder values are passed as name/value pairs. Result holds exit code, stdout and stderr, or an error.')
          input_schema = {
              'type': 'object',
              'properties': {
                  'cwd': {'type': 'string', 'description': f"Absolute path to the working directory containing '{COMMANDS_FILE}'."},
                  'id': {'type': 'string', 'description': 'ID of the command.'},
                  'values': {
                      'type': 'array',
                      'description': 'Placeholder replacements.',
                      'items': {
                          'type': 'object',
                          'properties': {'name': {'type': 'string'}, 'value': {'type': 'string'}},
                          'required': ['name', 'value']}}},
              'required': ['cwd', 'id']}

          def handle(self, ctx: ToolContext) -> ToolResult:
              args: dict[str, Any] = ctx.arguments
              cwd, cmd_id = args.get('cwd'), args.get('id')
              raw_values = args.get('values') or []
              if not isinstance(cwd, str) or not isinstance(cmd_id, str):
                  return ToolResult(content=[text_content("'cwd' and 'id' must be strings.")], is_error=True)
              try:
                  values = {v['name']: v['value'] for v in raw_values}
              except (KeyError, TypeError) as exc:
                  return ToolResult(content=[text_content(f'Invalid values: {exc!r}')], is_error=True)
              try:
                  result, spec = run_command(cwd, cmd_id, values)
              except RunCommandError as exc:
                  return ToolResult(content=[text_content(str(exc))], is_error=True)
              return pack_process_result(
                  result,
                  ctx,
                  normalize_output=True,
                  omit_zero_exit_code=True,
                  max_stream_chars=_MAX_STREAM_CHARS if spec.spill_to_disk else None)


      def register_run_command_tool(registry: ToolRegistry) -> None:
          registry.register(RunCommandTool())
  reason: Rewrite run_command with per-command spill_to_disk
```
/answer b622-49 allow
ReasoningToken: 143
Control Request:
```yaml
id: b622-4a
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py
  reason: Check file compiles
```
/answer b622-4a allow
Result Stats: id=2a97172e-ec44-438b-8363-70b065eb2b0b, total: 36331, in: 24, out: 10528, reason: 0, read: 155834, write: 25779, start: 1793915337296, end: 1793915342674, time: 5329, total: 98073, io: 0, control: 49, sent: 0, recv: 86
`spill_to_disk` lässt sich jetzt pro Mapping einstellen. Die Datei kompiliert, ausgeführt oder getestet habe ich sie nicht.

Ein Eintrag in `commands.json` ist entweder ein String oder ein Objekt:

```json
{
  "spill_to_disk": true,
  "commands": {
    "runTests": {"script": "cargo test", "spill_to_disk": false},
    "readFile": "cat {{file}}"
  }
}
```

- **String:** Ein Eintrag als String nutzt den Standardwert.
- **Objekt:** Ein Eintrag als Objekt hat `script` und optional `spill_to_disk`. Der Wert gilt nur für dieses Kommando.
- **Standardwert:** Das optionale `spill_to_disk` auf oberster Ebene gilt für alle Einträge ohne eigenen Wert. Ohne Angabe ist es `true`.
- **Fehler:** Ein Eintrag mit falschem Typ ergibt beim Aufruf einen Fehler.

Intern ersetzt `CommandSpec` (`script`, `spill_to_disk`) die frühere `RunCommandConfig`. Der Rest bleibt wie zuvor.
Die IDs sollen nicht im Fhlerfall gemeldet werden. Das diese nur vom Prompt exponiert werden ist Absicht. Ein guter Punkt, das Tool braucht natürlich Batch Support um mehrere IDs gleichzeitig ausführen zu können.
SystemInit:  id=2a97172e-ec44-438b-8363-70b065eb2b0b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: b622-4b
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py
    overwrite: true
    source: |
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
          'COMMANDS_FILE', 'RunCommandError', 'CommandSpec', 'RunCommandItem', 'RunCommandItemError',
          'load_commands', 'render_command', 'run_command', 'RunCommandTool', 'register_run_command_tool']

      COMMANDS_FILE = 'commands.json'
      _MAX_STREAM_CHARS = 3000
      _PLACEHOLDER = re.compile(r'\{\{\s*(\w+)\s*\}\}')


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
              return run_process(['bash', '-c', script], cwd=cwd_path), spec
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
          description = (
              f"Execute predefined commands by ID, as configured in '{COMMANDS_FILE}' of each working directory. "
              'Items run in parallel; placeholder values are passed as name/value pairs. '
              'Result holds exit code, stdout and stderr, or an error.')
          _ITEM_PROPERTIES = {
              'cwd': {'type': 'string', 'description': f"Absolute path to the working directory containing '{COMMANDS_FILE}'."},
              'id': {'type': 'string', 'description': 'ID of the command.'},
              'values': {
                  'type': 'array',
                  'description': 'Placeholder replacements.',
                  'items': {
                      'type': 'object',
                      'properties': {'name': {'type': 'string'}, 'value': {'type': 'string'}},
                      'required': ['name', 'value']}}}
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
  reason: Add batch support and remove ID listing from errors
```
/answer b622-4b allow
ReasoningToken: 178
Control Request:
```yaml
id: b622-4c
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py
  reason: Check file compiles
```
/answer b622-4c allow
Result Stats: id=2a97172e-ec44-438b-8363-70b065eb2b0b, total: 46469, in: 28, out: 15448, reason: 0, read: 207953, write: 30993, start: 1793915503487, end: 1793915509319, time: 5781, total: 126488, io: 0, control: 51, sent: 0, recv: 105
Beide Änderungen sind in `run_command.py` umgesetzt. Die Datei kompiliert, ausgeführt oder getestet habe ich sie nicht.

- **IDs im Fehlerfall:** Bei einer unbekannten ID lautet der Fehler nur noch `Unknown command id '<id>'.`. Die verfügbaren IDs stehen nicht mehr darin.
- **Batch:** Das Tool nimmt jetzt wie das Bash-Tool eine Liste `items`. Jedes Item hat `cwd`, `id` und optional `values` als `{name, value}`-Paare. Die Items laufen parallel in einem `ThreadPoolExecutor`.
- **Ergebnis:** Pro Item steht in der Antwort, in Eingabereihenfolge, entweder das Prozessergebnis oder `{error: ...}`. Ein fehlerhaftes Item bricht die anderen nicht ab.
- **Spill-Einstellung:** Jedes Item wird mit der `spill_to_disk`-Einstellung seines eigenen Kommandos verpackt. Ein Batch kann also Kommandos mit und ohne Spill mischen.
- **Funktionsname:** `run_command(items)` hat jetzt die Batch-Signatur und gibt Paare aus `ProcessResult` und `CommandSpec` oder einen `RunCommandItemError` zurück. Die Registrierung in `bash/__init__.py` bleibt unverändert.
Teste das Tool
SystemInit:  id=2a97172e-ec44-438b-8363-70b065eb2b0b, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 130
Control Request:
```yaml
id: b622-4d
toolName: python
arguments:
  script: |-
    import sys, json, tempfile, os
    sys.path.insert(0, '/home/user/xyan/xy.ai.workbench/mcpc/src')
    from types import SimpleNamespace
    from xy.ai.mcpc.tools.bash.run_command import run_command, RunCommandTool, RunCommandItemError

    d = tempfile.mkdtemp(); e = tempfile.mkdtemp()
    open(os.path.join(d, 'Cargo.toml'), 'w').write('[package]\nname="x"\n')
    json.dump({"spill_to_disk": True, "commands": {
        "hello": "echo hello",
        "Cargo.toml": "cat Cargo.toml",
        "readFile": "cat {{file}}",
        "big": {"script": "head -c 5000 /dev/zero | tr '\\0' a", "spill_to_disk": False},
        "bigSpill": "head -c 5000 /dev/zero | tr '\\0' a",
        "fail": "echo oops >&2; exit 3",
        "inj": "echo {{x}}"}}, open(os.path.join(d, 'commands.json'), 'w'))

    items = [
     {"cwd": d, "id": "hello"},
     {"cwd": d, "id": "Cargo.toml"},
     {"cwd": d, "id": "readFile", "values": {"file": "Cargo.toml"}},
     {"cwd": d, "id": "readFile"},
     {"cwd": d, "id": "hello", "values": {"z": "1"}},
     {"cwd": d, "id": "nope"},
     {"cwd": e, "id": "hello"},
     {"cwd": "rel", "id": "hello"},
     {"cwd": d, "id": "fail"},
     {"cwd": d, "id": "inj", "values": {"x": "a; echo PWNED"}},
     {"cwd": d},
    ]
    for it, o in zip(items, run_command(items)):
        if isinstance(o, RunCommandItemError): print(it.get('id'), 'ERR', o.error)
        else: print(it.get('id'), o[0].exit_code, repr(o[0].stdout[:50]), repr(o[0].stderr[:30]))

    ctx = SimpleNamespace(arguments={"items": [
      {"cwd": d, "id": "hello"},
      {"cwd": d, "id": "readFile", "values": [{"name": "file", "value": "Cargo.toml"}]},
      {"cwd": d, "id": "big"}, {"cwd": d, "id": "bigSpill"}, {"cwd": d, "id": "nope"}]})
    try:
        r = RunCommandTool().handle(ctx)
        for x in r.structured_content['results']:
            print({k: (v if len(str(v)) < 80 else f'<{len(v)} chars>') for k, v in x.items()})
    except Exception as ex:
        import traceback; traceback.print_exc()
  reason: Exercise run_command function and tool handler against a temp commands.json
```
/answer b622-4d allow
Control Request:
```yaml
id: b622-4e
result:
  structuredContent:
    exit_code: 0
    stdout: |
      hello 0 'hello\n' ''
      Cargo.toml 0 '[package]\nname="x"\n' ''
      readFile 0 '[package]\nname="x"\n' ''
      readFile ERR Missing values for placeholders: ['file']
      hello ERR Unknown placeholders: ['z']
      nope ERR Unknown command id 'nope'.
      hello ERR 'commands.json' not found in working directory.
      hello ERR cwd must be an absolute path.
      fail 3 '' 'oops\n'
      inj 0 'a; echo PWNED\n' ''
      None ERR Missing item key: 'id'
    stderr: |
      Traceback (most recent call last):
        File "<stdin>", line 39, in <module>
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/bash/run_command.py", line 194, in handle
          packed = pack_process_result(
                   ^^^^^^^^^^^^^^^^^^^^
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py", line 151, in pack_process_result
          for key, value in _spill_extra_info(stdout_file, ctx).items():
                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        File "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/process.py", line 102, in _spill_extra_info
          if {'tools', 'ast_outline'} & ctx.session.enabled_tools:
                                        ^^^^^^^^^^^
      AttributeError: 'types.SimpleNamespace' object has no attribute 'session'
```
/answer b622-4e allow
Text:
 Das Traceback kommt von meinem Fake-Kontext: Ihm fehlt `session`, und das braucht der Spill-Pfad. Ich ergänze es und wiederhole den Handler-Test.
Control Request:
```yaml
id: b622-4f
toolName: python
arguments:
  script: |-
    import sys, json, tempfile, os
    sys.path.insert(0, '/home/user/xyan/xy.ai.workbench/mcpc/src')
    from types import SimpleNamespace
    from xy.ai.mcpc.tools.bash.run_command import RunCommandTool

    d = tempfile.mkdtemp()
    open(os.path.join(d, 'Cargo.toml'), 'w').write('[package]\n')
    json.dump({"commands": {
        "hello": "echo hello",
        "readFile": "cat {{file}}",
        "big": {"script": "head -c 5000 /dev/zero | tr '\\0' a", "spill_to_disk": False},
        "bigSpill": "head -c 5000 /dev/zero | tr '\\0' a"}}, open(os.path.join(d, 'commands.json'), 'w'))
    ctx = SimpleNamespace(session=SimpleNamespace(enabled_tools=set()), arguments={"items": [
      {"cwd": d, "id": "hello"},
      {"cwd": d, "id": "readFile", "values": [{"name": "file", "value": "Cargo.toml"}]},
      {"cwd": d, "id": "big"}, {"cwd": d, "id": "bigSpill"}, {"cwd": d, "id": "nope"}]})
    r = RunCommandTool().handle(ctx)
    print(r.is_error)
    for x in r.structured_content['results']:
        print({k: (v if len(str(v)) < 80 else f'<{len(v)} chars>') for k, v in x.items()})
    print([c.get('text', '')[:60] for c in r.content])
  reason: Re-run handler test with a ctx that has a session
```
/answer b622-4f allow
Control Request:
```yaml
id: b622-50
result:
  structuredContent:
    exit_code: 0
    stdout: |
      False
      {'stdout': 'hello\n'}
      {'stdout': '[package]\n'}
      {'stdout': '<5000 chars>'}
      {'stdout_file': '/tmp/mcpc-stdout-cziqej_v.log', 'stdout_file_file_stats': '<13 chars>'}
      {'error': "Unknown command id 'nope'."}
      ['STDOUT limit of characters exceeded, output saved to `/tmp/m']
```
/answer b622-50 allow
Result Stats: id=2a97172e-ec44-438b-8363-70b065eb2b0b, total: 52668, in: 34, out: 18260, reason: 0, read: 303938, write: 34374, start: 1793915577787, end: 1793915588448, time: 10607, total: 151277, io: 0, control: 54, sent: 0, recv: 173
Das Tool funktioniert. Ich habe die Funktion `run_command` und den Handler `RunCommandTool.handle` gegen eine temporäre `commands.json` getestet. Den Handler habe ich mit einem selbstgebauten Kontext aufgerufen, nicht über den echten MCP-Server. Die Tests sind einmalige Skripte; im Repo liegen keine Testdateien.

**Funktion `run_command`**

| Fall | Ergebnis |
|---|---|
| `echo hello` | stdout `hello` |
| ID `Cargo.toml` (`cat Cargo.toml`) | Dateiinhalt wird ausgegeben |
| Platzhalter `{{file}}` mit Wert | Datei wird gelesen |
| Platzhalter ohne Wert | Fehler `Missing values for placeholders` |
| Wert ohne passenden Platzhalter | Fehler `Unknown placeholders` |
| Unbekannte ID | Fehler `Unknown command id 'nope'.`, ohne Liste der IDs |
| CWD ohne `commands.json` | Fehler, Datei nicht gefunden |
| Relativer CWD | Fehler, CWD muss absolut sein |
| Item ohne `id` | Fehler `Missing item key` |
| `exit 3` mit Ausgabe auf stderr | Exit-Code 3, stderr `oops` |
| Wert `a; echo PWNED` | Wird als reiner Text ausgegeben, kein Shell-Ausbruch |

Alle Items liefen im selben Batch, und die Fehler eines Items haben die anderen nicht beeinflusst.

**Handler**
- Ein Kommando mit `"spill_to_disk": false` und 5000 Zeichen Ausgabe kam vollständig inline zurück.
- Dasselbe Kommando mit Spill (Standard) wurde in `/tmp/mcpc-stdout-….log` ausgelagert, und der Hinweistext erschien.
- Beide Fälle liefen im selben Batch und verhielten sich unabhängig voneinander.
- Das Mapping von `values` als `{name, value}`-Liste und der Fehler bei unbekannter ID im Batch funktionieren.

Mein erster Handler-Test schlug fehl, weil meinem Fake-Kontext `session` fehlte. Das war ein Fehler im Test, nicht im Tool.

Nicht geprüft habe ich die Registrierung über `register_tools` und den Aufruf über den echten MCP-Server.
