Das Tool implementiert in `/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py` braucht als Eingabe eine Liste damit mehrere Module auf einmal abgefragt werden können. Die Ausgabe sollte dann eine entsprechende Liste sein mit Fehlern bei Absenz.
SystemInit:  id=43a598bb-28ed-4179-b2b8-0c22cedf0da8, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: f0c0-3e
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
  reason: Understand current structure of the tool
```
/answer f0c0-3e allow
Control Request:
```yaml
id: f0c0-3f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
      nodes:
      - id: fwGBve|36FcS4
        signature: '''#!/usr/bin/env python3'''
      - id: H74vFF|3PSClU
        signature: import argparse
      - id: nnxSG4|36su44
        signature: BASE_DIR = Path('/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch')
      - id: slugify
        signature: "def slugify(name: str) -> str:"
      - id: spec_path
        signature: "def spec_path(spec_dir: Path, name: str) -> Path:"
      - id: format_stats
        signature: "def format_stats(stats: TokenStats, elapsed: float | None=None) -> str:"
      - id: Reporter
        signature: "class Reporter:"
        docstring: Writes each line to STDOUT (STDERR for errors) and to the log file.
        children:
        - id: Reporter.qgjaym|iMxXms
          signature: '''Writes each line to STDOUT (STDERR for errors) and to the log file.'''
        - id: Reporter.init
          signature: "def __init__(self, log_file) -> None:"
        - id: Reporter.call
          signature: "def __call__(self, message: str, *, error: bool=False) -> None:"
      - id: main
        signature: "def main() -> int:"
      - id: run
        signature: "def run(args, report: Reporter) -> int:"
      - id: ehLeJq|07g9mi
        signature: "if __name__ == '__main__':"
```
/answer f0c0-3f allow
Control Request:
```yaml
id: f0c0-40
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    ids:
    - slugify
    - spec_path
    - main
    - run
  reason: Read argument parsing and run logic
```
/answer f0c0-40 allow
Control Request:
```yaml
id: f0c0-41
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
      nodes:
      - id: slugify
        type: FunctionDef
        code: |-
          def slugify(name: str) -> str:
              slug = re.sub('[^\\w]+', '_', name.strip(), flags=re.UNICODE).strip('_')
              return slug.lower() or 'module'
      - id: spec_path
        type: FunctionDef
        code: |-
          def spec_path(spec_dir: Path, name: str) -> Path:
              return spec_dir / f'{slugify(name)}.md'
      - id: main
        type: FunctionDef
        code: |-
          def main() -> int:
              parser = argparse.ArgumentParser(description=__doc__)
              parser.add_argument('--modules', type=Path, default=BASE_DIR / 'modules.yaml')
              parser.add_argument('--template', type=Path, default=BASE_DIR / 'prompt.md.j2')
              parser.add_argument('--spec-dir', type=Path, default=BASE_DIR / 'specs')
              parser.add_argument(
                  '--log-file',
                  type=Path,
                  default=None,
                  help='log file (default: <base>/logs/batch_<timestamp>.log)')
              parser.add_argument('--force', action='store_true', help='regenerate existing specifications')
              parser.add_argument('--profile', default='personal', help='Claude config profile (~/.claude-<profile>)')
              args = parser.parse_args()
              log_path = args.log_file or BASE_DIR / 'logs' / f'batch_{datetime.now():%Y%m%d_%H%M%S}.log'
              log_path.parent.mkdir(parents=True, exist_ok=True)
              with log_path.open('a', encoding='utf-8') as log_file:
                  return run(args, Reporter(log_file))
      - id: run
        type: FunctionDef
        code: |-
          def run(args, report: Reporter) -> int:
              modules = yaml.safe_load(args.modules.read_text(encoding='utf-8'))
              modules.sort(key=lambda m: m.get('Order', 0))
              args.spec_dir.mkdir(parents=True, exist_ok=True)
              env = Environment(loader=FileSystemLoader(str(args.template.parent)),
                                undefined=StrictUndefined, keep_trailing_newline=True)
              template = env.get_template(args.template.name)
              total = TokenStats()
              started = time.monotonic()

              def finish(code: int) -> int:
                  report(f'[stat] TOTAL | {format_stats(total, time.monotonic() - started)}')
                  return code
              with EmbeddedMcpc(builtin_tools=[], auto_approve=True) as mcpc:

                  @mcpc.tool('read_specifications', 'Read the finished specification of a module by name.', READ_SPEC_SCHEMA)
                  def read_specifications(ctx):
                      name = str(ctx.arguments.get('module', ''))
                      path = spec_path(args.spec_dir, name)
                      if not path.is_file():
                          return ToolResult(
                              content=[{'type': 'text', 'text': f"Specification for module '{name}' is still pending."}], is_error=True)
                      return path.read_text(encoding='utf-8')
                  done = []
                  for module in modules:
                      name = module['Name']
                      target = spec_path(args.spec_dir, name)
                      if target.exists() and (not args.force):
                          report(f'[skip] {name}')
                          done.append(name)
                          continue
                      module.setdefault('Dependencies', [])
                      prompt = template.render(module=module, previous_modules=list(done))
                      report(f'[run ] {name}')
                      module_started = time.monotonic()
                      with mcpc.start_agent(SYSTEM_PROMPT, tools={'read_specifications'}, model=Model.HAIKU, effort=Effort.LOW, cc_profile=args.profile) as agent:
                          result = agent.prompt(prompt)
                      total.add(result.stats)
                      report(f'[stat] {name} | {format_stats(result.stats, time.monotonic() - module_started)}')
                      if result.is_error or not result.text.strip():
                          report(f'[fail] {name}: {result.text}', error=True)
                          return finish(1)
                      target.write_text(result.text, encoding='utf-8')
                      done.append(name)
                      report(f'[ok  ] {target}')
                  return finish(0)
```
/answer f0c0-41 allow
Control Request:
```yaml
id: f0c0-42
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
    pattern: READ_SPEC_SCHEMA
    include:
    - '*.py'
    exclude:
    - node_modules/**
    intent: editing
  reason: Locate the schema of the read_specifications tool
```
/answer f0c0-42 allow
Control Request:
```yaml
id: f0c0-43
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
      matches:
      - path: batch_specs.py
        nodes:
        - id: nnxSG4|36su44
          matches:
          - "READ_SPEC_SCHEMA = {"
        - id: run.xWKi2p|Mo6YYf
          matches:
          - "@mcpc.tool('read_specifications', 'Read the finished specification of a module by name.', READ_SPEC_SCHEMA)"
```
/answer f0c0-43 allow
Control Request:
```yaml
id: f0c0-44
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    ids:
    - nnxSG4|36su44
  reason: Read schema and constants
```
/answer f0c0-44 allow
Control Request:
```yaml
id: f0c0-45
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
      nodes:
      - id: nnxSG4|36su44
        type: statements
        code: |-
          BASE_DIR = Path('/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch')
          SYSTEM_PROMPT = 'You are a software architect writing precise module specifications. Use the tool `read_specifications` to read specifications of other modules.'
          READ_SPEC_SCHEMA = {
              'type': 'object',
              'properties': {
                  'module': {
                      'type': 'string',
                      'description': 'Module name'}},
              'required': ['module']}
```
/answer f0c0-45 allow
ReasoningToken: 162
Control Request:
```yaml
id: f0c0-46
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
    pattern: ^(import|from)
    include:
    - batch_specs.py
    exclude:
    - node_modules/**
    intent: editing
  reason: Check whether json is imported
```
/answer f0c0-46 allow
Control Request:
```yaml
id: f0c0-47
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
      matches:
      - path: batch_specs.py
        nodes:
        - id: H74vFF|3PSClU
          matches:
          - import argparse
          - import re
          - import time
          - from datetime import datetime
          - import sys
          - from pathlib import Path
          - import yaml
          - "from jinja2 import Environment, FileSystemLoader, StrictUndefined"
          - "from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model"
          - from xy.ai.mcpc.cli.session import TokenStats
          - from xy.ai.mcpc.tools.tool_registry import ToolResult
```
/answer f0c0-47 allow
Control Request:
```yaml
id: f0c0-48
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    id: H74vFF|3PSClU
    old_text: import argparse
    new_text: |-
      import argparse
      import json
  reason: Add json import for list output
```
/answer f0c0-48 allow
Control Request:
```yaml
id: f0c0-49
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    id: nnxSG4|36su44
    start_marker: SYSTEM_PROMPT = 'You
    end_marker: "'required': ['module']}"
    content: |-
      SYSTEM_PROMPT = 'You are a software architect writing precise module specifications. Use the tool `read_specifications` to read specifications of other modules; pass all names you need in one call.'
      READ_SPEC_SCHEMA = {
          'type': 'object',
          'properties': {
              'modules': {
                  'type': 'array',
                  'items': {'type': 'string'},
                  'minItems': 1,
                  'description': 'List of module names'}},
          'required': ['modules']}
  reason: Change schema to accept a list of modules
```
/answer f0c0-49 allow
Control Request:
```yaml
id: f0c0-4a
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    id: run
    start_marker: "name = str(ctx.arguments.get('module', ''))"
    end_marker: return path.read_text(encoding='utf-8')
    content: |-
      names = ctx.arguments.get('modules', [])
                  if isinstance(names, str):
                      names = [names]
                  results = []
                  for name in map(str, names):
                      path = spec_path(args.spec_dir, name)
                      if path.is_file():
                          results.append({'module': name, 'specification': path.read_text(encoding='utf-8')})
                      else:
                          results.append({'module': name, 'error': f"Specification for module '{name}' is still pending."})
                  all_missing = not results or all('error' in r for r in results)
                  text = json.dumps(results, ensure_ascii=False, indent=2)
                  if all_missing:
                      return ToolResult(content=[{'type': 'text', 'text': text}], is_error=True)
                  return text
  reason: Return a list of results with per-module errors
```
/answer f0c0-4a allow
Control Request:
```yaml
id: f0c0-4b
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
  reason: Verify file compiles after edits
```
/answer f0c0-4b allow
Result Stats: id=43a598bb-28ed-4179-b2b8-0c22cedf0da8, total: 14105, in: 16, out: 2662, reason: 0, read: 59460, write: 11427, start: 1793552195279, end: 1793552199850, time: 4528, total: 25989, io: 0, control: 43, sent: 0, recv: 118
Das Tool `read_specifications` in `batch_specs.py` nimmt jetzt eine Liste von Modulnamen entgegen. Die Datei lässt sich kompilieren; ausgeführt oder mit einem echten Agenten getestet habe ich es nicht.

- **Eingabe:** Der Parameter heißt jetzt `modules` und ist ein Array von Strings (mindestens ein Eintrag). Einen einzelnen String akzeptiert das Tool weiterhin und behandelt ihn wie eine Liste mit einem Eintrag.
- **Ausgabe:** Das Tool liefert eine JSON-Liste mit einem Eintrag pro angefragtem Modul.
  - Existiert die Spezifikation, lautet der Eintrag `{"module": ..., "specification": ...}`.
  - Fehlt sie, lautet er `{"module": ..., "error": "Specification for module '...' is still pending."}`.
- **Fehlerstatus:** Das Tool meldet nur dann einen Fehler (`is_error`), wenn gar keine Spezifikation gefunden wurde. Sobald mindestens ein Modul vorhanden ist, gilt der Aufruf als Erfolg, und die fehlenden Module stehen als Einträge mit `error` in der Liste.
- **Systemprompt:** Ich habe ihn ergänzt, damit der Agent alle benötigten Namen in einem Aufruf übergibt.
- **Import:** Dafür kam `import json` dazu.

Soll das Tool auch bei teilweise fehlenden Modulen einen Fehlerstatus melden, ist das eine kleine Änderung.
