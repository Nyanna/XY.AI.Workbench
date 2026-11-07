Ich brauche ein Python Batch Script das den MCPC verwendet (`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/README.md`).
Es soll aus einer YAML-Datei über eine Liste von Modulen iterieren, ein jinja2 Template Anreichern und der Reihe nach eine Claude Code Haiku Instanz mit einem MCPC und einem Prompt zu Spezifikationserstellung starten.
Das Promptergebnis wird in einer Markdown Datei in einem Verzeichnis für die Spezifikationen gespeichert.

Moduldefinition:
```yaml
- Name: Erweiterbare Zusatzparameter
  Order: 1
  Responsibilities:
  - typindizierte Extension-Map mit höchstens einem Wert je Typ führen
  - Werte beim Klonen teilen und thread-safe durch Queue und Planung reichen
  Requirements:
  - Dient dem Kern als einziges Mittel für Addon-Tasks (erweiterbares Parameterobjekt).
  - Trägt Addon-spezifische Parameter, etwa den Scope eines Folge-Tasks, ohne dass der Kern sie interpretiert.
  - Nimmt als typindizierte Extension-Map beliebige zusätzliche, typgebundene Werte auf.
  - Speichert je Typ höchstens einen Wert. Erneutes Setzen desselben Typs ersetzt den vorhandenen Wert.
  - Fragt Werte nach Typ ab (sonst „nicht vorhanden“) und entfernt sie nach Typ, mit Rückmeldung, ob der Wert vorhanden war.
  - Liefert die Anzahl der Werte und ob die Map leer ist.
  - Teilt die Werte beim Klonen (shared), statt sie zu duplizieren.
  - Reicht die Werte unverändert durch Queue und Planung und ist thread-safe übertragbar (Send + Sync).
  Dependencies: []
- Name: Abschluss-Warten
  Order: 2
  Responsibilities:
  - asynchron auf completed oder cancelled warten, ohne Thread zu blockieren oder Notification zu verlieren
  - mehrere Wartende gleichzeitig wecken
  Requirements:
  - Meldet den Abschluss einer Arbeitseinheit per Event oder Notification, sodass Ergebnisse abgeholt werden können.
  - Erlaubt, die Registrierung für die Notification vor dem Lesen des Zustands zu aktivieren. Keine Notification geht verloren, auch wenn der Abschluss zwischen Zustandsprüfung und Wartebeginn eintritt.
  - Unterstützt mehrere Wartende gleichzeitig und weckt alle, bei Completion ebenso wie bei erfolgreichem Cancel.
  - Wartet asynchron auf „completed“ oder „cancelled“, ohne einen Thread zu blockieren. Bei bereits erreichtem Endzustand endet das Warten sofort.
  - Ein Wakeup ohne Endzustand führt zu erneuter Prüfung.
  Dependencies: []
```

Mit dem MCPC wird Claude ein Tool "read_specifications" (Parameter Modulname) zur Verfügung gestellt mit dem er bereits erledigte Spezifikationen aus dem Verzeichnis lesen kann.
Ist das Modul noch nicht spezifiziert(Datei nicht gefunden) wird ein entsprechender Fehler gemeldet das die Spezifikation nachfolgend noch aussteht.

- Zielverzeichnis für alles: `/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch`
SystemInit:  id=0bd74981-2940-4ae2-a5ac-0536a4443da6, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich lese zuerst die README des MCPC.
Control Request:
```yaml
id: f0c0-3
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded
    script:
    - cat README.md
    - ls -la
    - ls /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
  reason: README des embedded MCPC lesen und Zielverzeichnis prüfen
```
/answer f0c0-3 deny nutze AST
Control Request:
```yaml
id: f0c0-4
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/README.md
  reason: README des embedded MCPC strukturell lesen
```
/answer f0c0-4 allow
Control Request:
```yaml
id: f0c0-5
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/README.md
      nodes:
      - id: 464c2e
        signature: xy.ai.mcpc.embedded
        children:
        - id: 464c2e.cCP1DO|Xp0Jzp
          signature: Run an MCPC server (MCP serve…
        - id: 464c2e.DRdgTr|NLFfYo
          signature: |-
            ```python
            from xy.ai.mcpc.emb…
        - id: 464c2e.7daa97
          signature: Pick a topology
        - id: 464c2e.34fd58
          signature: Shared MCPC (default pattern)
          children:
          - id: 464c2e.34fd58.q2A5K9|2q3Yt9
            signature: |-
              ```python
              with EmbeddedMcpc(b…
          - id: 464c2e.34fd58.9T2kC2|auk5J7
            signature: Register tools **before or af…
        - id: 464c2e.598e32
          signature: Multi-turn / manual control
          children:
          - id: 464c2e.598e32.V1tJb9|igrInd
            signature: |-
              ```python
              with mcpc.start_age…
          - id: 464c2e.598e32.4GP3a0|i5pSoF
            signature: The conversation persists acr…
        - id: 464c2e.627f65
          signature: Dedicated MCPC per agent
          children:
          - id: 464c2e.627f65.DyzsCp|FrN3yC
            signature: |-
              ```python
              with launch_agent("…
          - id: 464c2e.627f65.UtXqfa|0JW1iJ
            signature: For a pool of specialised age…
        - id: 464c2e.d93d10
          signature: API
          children:
          - id: 464c2e.d93d10.RNopyQ|w5x3OY
            signature: "**`EmbeddedMcpc(config=None, …"
          - id: 464c2e.d93d10.uj1UWv|iVcz2I
            signature: "**`Agent`**: `id`, `prompt(te…"
          - id: 464c2e.d93d10.Aq0C9P|e0Eof4
            signature: '**Enums** (plain strings also…'
        - id: 464c2e.000eb2
          signature: Gotchas
```
/answer f0c0-5 allow
Control Request:
```yaml
id: f0c0-6
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/README.md
    ids:
    - 464c2e
  reason: Gesamten README-Inhalt lesen
```
/answer f0c0-6 allow
Control Request:
```yaml
id: f0c0-7
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded/README.md
      nodes:
      - id: 464c2e
        type: section
        signature: xy.ai.mcpc.embedded
        children:
        - id: 464c2e.cCP1DO|Xp0Jzp
          type: paragraph
          code: |
            Run an MCPC server (MCP server giving an agent focused tools) inside your own Python process and drive Claude CLI agents against it. Use it for batch scripts: iterate over inputs, one agent per item, parallel.
        - id: 464c2e.DRdgTr|NLFfYo
          type: fenced_code_block
          code: |
            ```python
            from xy.ai.mcpc.embedded import EmbeddedMcpc, launch_agent, FunctionTool, ToolProvider, Model, Effort
            ```
        - id: 464c2e.7daa97
          type: section
          code: |
            ## Pick a topology

            | Need | Use |
            |---|---|
            | Many agents, **one shared** MCPC with preconfigured tools | `EmbeddedMcpc` + `run_batch` / `start_agent` |
            | Each agent its **own specially configured** MCPC (tools, profile) | `launch_agent(...)` |
        - id: 464c2e.34fd58
          type: section
          signature: Shared MCPC (default pattern)
          children:
          - id: 464c2e.34fd58.q2A5K9|2q3Yt9
            type: fenced_code_block
            code: |
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
          - id: 464c2e.34fd58.9T2kC2|auk5J7
            type: paragraph
            code: |
              Register tools **before or after** `start()`; `@mcpc.tool` works in both cases.
        - id: 464c2e.598e32
          type: section
          signature: Multi-turn / manual control
          children:
          - id: 464c2e.598e32.V1tJb9|igrInd
            type: fenced_code_block
            code: |
              ```python
              with mcpc.start_agent("You are a reviewer.", tools={"read_file"}) as agent:
                  a = agent.ask("Review foo.py")         # str; raises CliSessionError on error result
                  r = agent.prompt("Now bar.py")         # CliResult, never raises on agent-side errors
              ```
          - id: 464c2e.598e32.4GP3a0|i5pSoF
            type: paragraph
            code: |
              The conversation persists across calls on the same `Agent`. Closing (context manager or `agent.close()`) kills the CLI process and the MCPC session.
        - id: 464c2e.627f65
          type: section
          signature: Dedicated MCPC per agent
          children:
          - id: 464c2e.627f65.DyzsCp|FrN3yC
            type: fenced_code_block
            code: |
              ```python
              with launch_agent("You review SQL.", builtin_tools=["read_file"], tools=[MyProvider()],
                                cc_profile="review", model=Model.OPUS) as agent:
                  print(agent.ask("..."))                # closing also stops its server
              ```
          - id: 464c2e.627f65.UtXqfa|0JW1iJ
            type: paragraph
            code: |
              For a pool of specialised agents run several `launch_agent` calls in a `ThreadPoolExecutor`.
        - id: 464c2e.d93d10
          type: section
          signature: API
          children:
          - id: 464c2e.d93d10.RNopyQ|w5x3OY
            type: paragraph
            code: |
              **`EmbeddedMcpc(config=None, *, builtin_tools=True, tools=(), auto_approve=None, ignore_tools_header=None, enable_control=True)`**
          - id: 464c2e.d93d10.uj1UWv|iVcz2I
            type: paragraph
            code: |
              **`Agent`**: `id`, `prompt(text) -> CliResult`, `ask(text) -> str`, `close()`.
          - id: 464c2e.d93d10.Aq0C9P|e0Eof4
            type: paragraph
            code: |
              **Enums** (plain strings also accepted): `Model` = haiku|sonnet|opus; `Effort` = disabled|low|medium|high|xhigh|max; `CacheMode` = default|disabled|5m|1h.
        - id: 464c2e.000eb2
          type: section
          code: |
            ## Gotchas

            - `tools=` on `start_agent`/`run_batch` selects from the **registered** tool names; unknown names give the agent nothing.
            - `cc_profile` selects the Claude config dir `~/.claude-<profile>` (login, settings). Use a profile that is authenticated.
            - `run_batch` converts `CliSessionError` (timeout, crash) into `CliResult(is_error=True)`; check `is_error` per result.
            - Sessions expire after `agent_session_ttl_seconds` of inactivity; the response timeout defaults to 24 h. Override via `ServerConfig`.
            - Keep `max_workers` modest: each agent is one `claude` process.
            - CLI traffic is logged per agent under `config.cli_log_dir` (`<agent id>.json.log`); look there when a result is an error.
```
/answer f0c0-7 allow
ReasoningToken: 353
Control Request:
```yaml
id: f0c0-8
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
    pattern: class ToolResult|class CliResult
    include:
    - '*.py'
    intent: editing
  reason: ToolResult-Signatur für Fehlerrückgabe prüfen
```
/answer f0c0-8 allow
Control Request:
```yaml
id: f0c0-9
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
      matches:
      - path: cli/session.py
        nodes:
        - id: CliResult
          matches:
          - "class CliResult:"
      - path: tools/tool_registry.py
        nodes:
        - id: ToolResult
          matches:
          - "class ToolResult:"
```
/answer f0c0-9 allow
Control Request:
```yaml
id: f0c0-a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
    ids:
    - ToolResult
  reason: ToolResult-Felder lesen
```
/answer f0c0-a allow
Control Request:
```yaml
id: f0c0-b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_registry.py
      nodes:
      - id: ToolResult
        type: ClassDef
        signature: "@dataclass(slots=True) class ToolResult:"
        docstring: The result of a tool call (maps onto MCP ``CallToolResult``).
        children:
        - id: ToolResult.6F9vfX|khLiSB
          type: statements
          code: |-
            'The result of a tool call (maps onto MCP ``CallToolResult``).'
            content: list[dict[str, Any]] = field(default_factory=list)
            structured_content: dict[str, Any] | None = None
            is_error: bool = False
            auto_approve: bool = False
            message: str | None = None
        - id: ToolResult.post_init
          type: FunctionDef
          code: |-
            def __post_init__(self) -> None:
                if self.is_error and self.structured_content:
                    raise ValueError('ToolResult: structured_content must not be set when is_error is True.')
        - id: ToolResult.to_dict
          type: FunctionDef
          code: |-
            def to_dict(self) -> dict[str, Any]:
                result: dict[str, Any] = {}
                if self.content:
                    result['content'] = list(self.content)
                if self.structured_content:
                    result['structuredContent'] = dict(self.structured_content)
                if self.message:
                    attach_message(result, self.message)
                if self.is_error:
                    result['isError'] = True
                return result
```
/answer f0c0-b allow
Control Request:
```yaml
id: f0c0-c
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded
    pattern: ToolResult
    include:
    - '*.py'
  reason: Prüfen ob ToolResult aus embedded exportiert wird
```
/answer f0c0-c allow
Control Request:
```yaml
id: f0c0-d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded
      matches:
      - path: tools.py
        matches:
        - "from xy.ai.mcpc.tools.tool_registry import ToolDefinition, ToolRegistry, ToolResult"
        - "ToolHandlerFn = Callable[['ToolContext'], 'ToolResult | str | dict[str, Any]']"
        - "def handle(self, ctx: ToolContext) -> ToolResult | str | dict[str, Any]:"
```
/answer f0c0-d allow
Control Request:
```yaml
id: f0c0-e
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/modules.yaml
    source: |
      - Name: Erweiterbare Zusatzparameter
        Order: 1
        Responsibilities:
        - typindizierte Extension-Map mit höchstens einem Wert je Typ führen
        - Werte beim Klonen teilen und thread-safe durch Queue und Planung reichen
        Requirements:
        - Dient dem Kern als einziges Mittel für Addon-Tasks (erweiterbares Parameterobjekt).
        - Trägt Addon-spezifische Parameter, etwa den Scope eines Folge-Tasks, ohne dass der Kern sie interpretiert.
        - Nimmt als typindizierte Extension-Map beliebige zusätzliche, typgebundene Werte auf.
        - Speichert je Typ höchstens einen Wert. Erneutes Setzen desselben Typs ersetzt den vorhandenen Wert.
        - Fragt Werte nach Typ ab (sonst „nicht vorhanden“) und entfernt sie nach Typ, mit Rückmeldung, ob der Wert vorhanden war.
        - Liefert die Anzahl der Werte und ob die Map leer ist.
        - Teilt die Werte beim Klonen (shared), statt sie zu duplizieren.
        - Reicht die Werte unverändert durch Queue und Planung und ist thread-safe übertragbar (Send + Sync).
        Dependencies: []
      - Name: Abschluss-Warten
        Order: 2
        Responsibilities:
        - asynchron auf completed oder cancelled warten, ohne Thread zu blockieren oder Notification zu verlieren
        - mehrere Wartende gleichzeitig wecken
        Requirements:
        - Meldet den Abschluss einer Arbeitseinheit per Event oder Notification, sodass Ergebnisse abgeholt werden können.
        - Erlaubt, die Registrierung für die Notification vor dem Lesen des Zustands zu aktivieren. Keine Notification geht verloren, auch wenn der Abschluss zwischen Zustandsprüfung und Wartebeginn eintritt.
        - Unterstützt mehrere Wartende gleichzeitig und weckt alle, bei Completion ebenso wie bei erfolgreichem Cancel.
        - Wartet asynchron auf „completed“ oder „cancelled“, ohne einen Thread zu blockieren. Bei bereits erreichtem Endzustand endet das Warten sofort.
        - Ein Wakeup ohne Endzustand führt zu erneuter Prüfung.
        Dependencies: []
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/prompt.md.j2
    source: |
      Create a specification for the module "{{ module.Name }}".

      ## Responsibilities
      {% for item in module.Responsibilities %}
      - {{ item }}
      {%- endfor %}

      ## Requirements
      {% for item in module.Requirements %}
      - {{ item }}
      {%- endfor %}

      ## Dependencies
      {% if module.Dependencies %}
      {% for dep in module.Dependencies %}
      - {{ dep }}
      {%- endfor %}

      Read the specifications of these dependencies with the tool `read_specifications` before you start.
      {% else %}
      None.
      {% endif %}

      {% if previous_modules %}
      ## Modules specified before this one
      {% for name in previous_modules %}
      - {{ name }}
      {%- endfor %}

      Use `read_specifications` (parameter `module`) to consult them where relevant. If it reports a specification as still pending, continue without it.
      {% endif %}

      Output only the finished specification as Markdown.
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    source: |
      #!/usr/bin/env python3
      import argparse
      import re
      import sys
      from pathlib import Path

      import yaml
      from jinja2 import Environment, FileSystemLoader, StrictUndefined

      from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model
      from xy.ai.mcpc.tools.tool_registry import ToolResult

      BASE_DIR = Path("/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch")

      SYSTEM_PROMPT = (
          "You are a software architect writing precise module specifications. "
          "Use the tool `read_specifications` to read specifications of other modules."
      )

      READ_SPEC_SCHEMA = {
          "type": "object",
          "properties": {"module": {"type": "string", "description": "Module name"}},
          "required": ["module"],
      }


      def slugify(name: str) -> str:
          slug = re.sub(r"[^\w]+", "_", name.strip(), flags=re.UNICODE).strip("_")
          return slug.lower() or "module"


      def spec_path(spec_dir: Path, name: str) -> Path:
          return spec_dir / f"{slugify(name)}.md"


      def main() -> int:
          parser = argparse.ArgumentParser(description=__doc__)
          parser.add_argument("--modules", type=Path, default=BASE_DIR / "modules.yaml")
          parser.add_argument("--template", type=Path, default=BASE_DIR / "prompt.md.j2")
          parser.add_argument("--spec-dir", type=Path, default=BASE_DIR / "specs")
          parser.add_argument("--force", action="store_true", help="regenerate existing specifications")
          args = parser.parse_args()

          modules = yaml.safe_load(args.modules.read_text(encoding="utf-8"))
          modules.sort(key=lambda m: m.get("Order", 0))
          args.spec_dir.mkdir(parents=True, exist_ok=True)

          env = Environment(
              loader=FileSystemLoader(str(args.template.parent)),
              undefined=StrictUndefined,
              keep_trailing_newline=True,
          )
          template = env.get_template(args.template.name)

          failed = []
          with EmbeddedMcpc(builtin_tools=[], auto_approve=True) as mcpc:

              @mcpc.tool(
                  "read_specifications",
                  "Read the finished specification of a module by name.",
                  READ_SPEC_SCHEMA,
              )
              def read_specifications(ctx):
                  name = str(ctx.arguments.get("module", ""))
                  path = spec_path(args.spec_dir, name)
                  if not path.is_file():
                      return ToolResult(
                          content=[{
                              "type": "text",
                              "text": f"Specification for module '{name}' is still pending.",
                          }],
                          is_error=True,
                      )
                  return path.read_text(encoding="utf-8")

              done = []
              for module in modules:
                  name = module["Name"]
                  target = spec_path(args.spec_dir, name)
                  if target.exists() and not args.force:
                      print(f"[skip] {name}")
                      done.append(name)
                      continue

                  module.setdefault("Dependencies", [])
                  prompt = template.render(module=module, previous_modules=list(done))
                  print(f"[run ] {name}")
                  with mcpc.start_agent(
                      SYSTEM_PROMPT,
                      tools={"read_specifications"},
                      model=Model.HAIKU,
                      effort=Effort.LOW,
                  ) as agent:
                      result = agent.prompt(prompt)

                  if result.is_error or not result.text.strip():
                      print(f"[fail] {name}: {result.text}", file=sys.stderr)
                      failed.append(name)
                      continue

                  target.write_text(result.text, encoding="utf-8")
                  done.append(name)
                  print(f"[ok  ] {target}")

          if failed:
              print(f"Failed modules: {', '.join(failed)}", file=sys.stderr)
              return 1
          return 0


      if __name__ == "__main__":
          sys.exit(main())
  reason: "Batch-Script, Prompt-Template und Modul-YAML im Zielverzeichnis anlegen"
```
/answer f0c0-e allow
Control Request:
```yaml
id: f0c0-f
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
  reason: Syntax des neuen Scripts prüfen
```
/answer f0c0-f allow
Result Stats: id=0bd74981-2940-4ae2-a5ac-0536a4443da6, total: 22514, in: 16, out: 5588, reason: 0, read: 77136, write: 16910, start: 1793546350153, end: 1793546356536, time: 6336, total: 45906, io: 0, control: 47, sent: 0, recv: 162
Ich habe das Batch-Script angelegt. Es besteht die Syntaxprüfung, gelaufen ist es noch nicht. Es hat also weder einen echten Claude-Aufruf noch den MCPC-Import getestet.

Alle Dateien liegen in `/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/`:
- **`batch_specs.py`:** das Script.
- **`modules.yaml`:** Ihre beiden Beispielmodule.
- **`prompt.md.j2`:** das Jinja2-Template für den Prompt.

**Ablauf:**
- Das Script sortiert die Module nach `Order` und startet pro Modul nacheinander eine Haiku-Instanz (`Effort.LOW`) über einen gemeinsamen `EmbeddedMcpc`. Die Reihenfolge ist wichtig, weil spätere Module frühere Spezifikationen lesen können.
- Das Template wird pro Modul mit Name, Responsibilities, Requirements und Dependencies gefüllt. Dazu kommt die Liste der bereits spezifizierten Module.
- Das Ergebnis landet in `specs/<modulname>.md`. Der Dateiname ist der Modulname in Kleinbuchstaben, Sonderzeichen werden zu `_`. Das Verzeichnis `specs/` wird beim Start angelegt.

**Tool `read_specifications`:**
- Es hat den Parameter `module`.
- Es liefert den Inhalt der Datei. Fehlt die Datei, meldet es einen Tool-Fehler, dass die Spezifikation noch aussteht.

**Verhalten bei Fehlern und Wiederholung:**
- Schlägt ein Modul fehl, geht das Script mit dem nächsten weiter und endet am Schluss mit Exit-Code 1.
- Bereits vorhandene Spezifikationen werden übersprungen. Mit `--force` werden sie neu erzeugt.

**Start:**
```bash
python batch_specs.py [--force] [--modules ...] [--template ...] [--spec-dir ...]
```
Es braucht `pyyaml`, `jinja2` und das `xy.ai.mcpc`-Paket im Python-Pfad. Für Claude muss ein angemeldetes Profil vorhanden sein. Ein anderes Profil lässt sich über `cc_profile` setzen, falls das nötig ist.
Das Script kann Module die vorhanden sind überspringen (resume), ein Fehler soll abbrechen. 
SystemInit:  id=0bd74981-2940-4ae2-a5ac-0536a4443da6, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 46
Control Request:
```yaml
id: f0c0-10
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    start_marker: "if result.is_error or not result.text.strip():"
    end_marker: |-
      failed.append(name)
                      continue
    content: |-
      if result.is_error or not result.text.strip():
                      print(f"[fail] {name}: {result.text}", file=sys.stderr)
                      return 1
    exact: false
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    start_marker: |2
          if failed:
    end_marker: |2-
          return 1
          return 0
    content: '    return 0'
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    start_marker: |2
          failed = []
    end_marker: '    with EmbeddedMcpc'
    content: '    with EmbeddedMcpc'
  reason: Bei Fehler sofort abbrechen statt weiterzumachen; failed-Liste entfernen
```
/answer f0c0-10 allow
Control Request:
```yaml
id: f0c0-11
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
      result: success
      new_id: main
    - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
      result: success
      new_id: main
    errors:
    - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
      error: "Syntax error: unexpected indent (line 14)"
```
/answer f0c0-11 allow
ReasoningToken: 26
Control Request:
```yaml
id: f0c0-12
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
  regexp: failed|return 0
  reason: Aktuellen Stand nach den Edits prüfen
```
/answer f0c0-12 allow
Control Request:
```yaml
id: f0c0-13
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
      nodes:
      - id: main.9Lp1K8|D7QvPH
        code: |-
          modules.sort(key=lambda m: m.get('Order', 0))
          args.spec_dir.mkdir(parents=True, exist_ok=True)
          env = Environment(loader=FileSystemLoader(str(args.template.parent)),
                            undefined=StrictUndefined, keep_trailing_newline=True)
          template = env.get_template(args.template.name)
          failed = []
      - id: main.CPzFt0|EHzNfu
        code: |-
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
                      print(f'[skip] {name}')
                      done.append(name)
                      continue
                  module.setdefault('Dependencies', [])
                  prompt = template.render(module=module, previous_modules=list(done))
                  print(f'[run ] {name}')
                  with mcpc.start_agent(SYSTEM_PROMPT, tools={'read_specifications'}, model=Model.HAIKU, effort=Effort.LOW) as agent:
                      result = agent.prompt(prompt)
                  if result.is_error or not result.text.strip():
                      print(f'[fail] {name}: {result.text}', file=sys.stderr)
                      return 1
                  target.write_text(result.text, encoding='utf-8')
                  done.append(name)
                  print(f'[ok  ] {target}')
              return 0
```
/answer f0c0-13 allow
Control Request:
```yaml
id: f0c0-14
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    id: main.9Lp1K8|D7QvPH
    old_text: |2-

      failed = []
    new_text: ""
  reason: Ungenutzte failed-Liste entfernen
```
/answer f0c0-14 allow
ReasoningToken: 67
Control Request:
```yaml
id: f0c0-15
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
  reason: Syntax prüfen
```
/answer f0c0-15 allow
Result Stats: id=0bd74981-2940-4ae2-a5ac-0536a4443da6, total: 27207, in: 24, out: 6947, reason: 0, read: 150007, write: 20236, start: 1793546518101, end: 1793546520912, time: 2768, total: 58935, io: 0, control: 43, sent: 0, recv: 64
Das Script bricht jetzt beim ersten Fehler ab. Es gibt Exit-Code 1 zurück, und für das fehlgeschlagene Modul wird keine Datei geschrieben. Ich habe es nicht ausgeführt, die Syntaxprüfung ist erfolgreich.

Das Resume war schon vorher drin: Module, deren Spezifikationsdatei in `specs/` existiert, werden übersprungen (`[skip]`). Nach einem Abbruch startet man das Script einfach neu, es macht beim fehlgeschlagenen Modul weiter. Mit `--force` werden vorhandene Spezifikationen neu erzeugt.

Die Liste `failed` und die Sammelmeldung am Ende habe ich entfernt, weil sie nicht mehr gebraucht werden.
teste es 
SystemInit:  id=0bd74981-2940-4ae2-a5ac-0536a4443da6, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 83
Control Request:
```yaml
id: f0c0-16
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
    script:
    - python3 batch_specs.py 2>&1 | tail -30
    - ls -la specs
  reason: Batch-Script ausführen und erzeugte Spezifikationen prüfen
```
/answer f0c0-16 allow
Control Request:
```yaml
id: f0c0-17
result:
  structuredContent:
    results:
    - exit_code: 2
      stdout: |
        Traceback (most recent call last):
          File "/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py", line 8, in <module>
            from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model
        ModuleNotFoundError: No module named 'xy'
      stderr: |
        ls: cannot access 'specs': No such file or directory
```
/answer f0c0-17 allow
ReasoningToken: 30
Control Request:
```yaml
id: f0c0-18
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench
    script:
    - ls -d mcpc/.venv .venv mcpc/venv 2>&1
    - ls mcpc | head -20
  reason: Passende Python-Umgebung für xy.ai.mcpc finden
```
/answer f0c0-18 deny MCPC liegt hier `/home/user/xyan/xy.ai.workbench/mcpc/src`
Control Request:
```yaml
id: f0c0-19
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
    script:
    - PYTHONPATH=/home/user/xyan/xy.ai.workbench/mcpc/src python3 batch_specs.py 2>&1 | tail -30
    - ls -la specs
  reason: Script mit PYTHONPATH auf mcpc/src ausführen
```
/answer f0c0-19 allow
Control Request:
```yaml
id: f0c0-1a
result:
  structuredContent:
    results:
    - stdout: |
        [fail] Erweiterbare Zusatzparameter: Not logged in · Please run /login
        [run ] Erweiterbare Zusatzparameter
        total 8
        drwxrwxr-x 2 user user 4096 Nov  1 16:23 .
        drwxrwxr-x 4 user user 4096 Nov  1 16:23 ..
```
/answer f0c0-1a allow
ReasoningToken: 78
Control Request:
```yaml
id: f0c0-1b
toolName: ask_user
arguments:
  question: "Der Lauf scheitert mit \"Not logged in · Please run /login\". Welches Claude-Profil (`~/.claude-<profile>`) soll ich verwenden, oder soll ich das Default-Profil lassen und Sie loggen sich ein?"
  reason: "Der Test läuft bis zum Claude-Aufruf, scheitert aber an fehlender Authentifizierung; das Profil bestimmt, welche Anmeldung genutzt wird."
```
/answer f0c0-1b allow es muss mit dem MCPC Profil "personal" laufen
ReasoningToken: 52
Control Request:
```yaml
id: f0c0-1c
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded
    pattern: cc_profile
    include:
    - '*.py'
    intent: survey
  reason: "Herausfinden, wo cc_profile übergeben wird"
```
/answer f0c0-1c allow
Control Request:
```yaml
id: f0c0-1d
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/embedded
      matches:
      - path: __init__.py
        matches:
        - "with launch_agent(\"You are a reviewer.\", builtin_tools=[\"read\"], cc_profile=\"review\") as agent:"
      - path: mcpc.py
        matches:
        - "def open_session(self, tools: Iterable[str] | None=None, session_id: str | None=None, cc_profile: str='none') -> str:"
        - "session = self.server.sessions.precreate(sid, enabled_tools=names, cc_profile=cc_profile)"
        - "def start_agent(self, system_prompt: str, *, model: Model | str=Model.SONNET, effort: Effort | str=Effort.MEDIUM, tools: Iterable[str] | None=None, cc…, cache_mode: CacheMode | str=CacheMode.DEFAULT, executable: str='claude', extra_env: dict[str, str] | None=None, agent_id: str | None=None) -> Agent:"
        - "sid = self.open_session(tools=tools, cc_profile=cc_profile)"
        - "cc_profile=cc_profile,"
      - path: agent.py
        matches:
        - "def agent_parameters(config: ServerConfig, mcpc_session_id: str, system_prompt: str, *, model: Model | str, effort: Effort | str, cc_profile: str, cache_mode: CacheMode | str, executable: str, extra_env: dict[str, str] | None) -> CliParameters:"
        - "cc_profile=cc_profile,"
      error: 1 match line(s) exceeded 300 characters and were shortened in the middle with an ellipsis (150 leading and trailing characters kept).
```
/answer f0c0-1d allow
Text:
 `start_agent` nimmt `cc_profile` entgegen. Ich ergänze es im Script, mit Default `personal` und einem CLI-Argument.
Control Request:
```yaml
id: f0c0-1e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    id: main
    old_text: "effort=Effort.LOW) as agent:"
    new_text: "effort=Effort.LOW, cc_profile=args.profile) as agent:"
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
    id: main
    old_text: '    args = parser.parse_args()'
    new_text: |2-
          parser.add_argument("--profile", default="personal", help="Claude config profile (~/.claude-<profile>)")
          args = parser.parse_args()
  reason: cc_profile "personal" an start_agent übergeben
```
/answer f0c0-1e allow
Control Request:
```yaml
id: f0c0-1f
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
    script:
    - PYTHONPATH=/home/user/xyan/xy.ai.workbench/mcpc/src timeout 900 python3 batch_specs.py 2>&1 | tail -30
    - ls -la specs
  reason: Script mit Profil personal erneut ausführen
```
/answer f0c0-1f allow
Control Request:
```yaml
id: f0c0-20
result:
  structuredContent:
    results:
    - stdout: |
        [run ] Erweiterbare Zusatzparameter
        [ok  ] /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/specs/erweiterbare_zusatzparameter.md
        [run ] Abschluss-Warten
        [ok  ] /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/specs/abschluss_warten.md
        total 24
        drwxrwxr-x 2 user user 4096 Nov  1 16:25 .
        drwxrwxr-x 4 user user 4096 Nov  1 16:23 ..
        -rw-rw-r-- 1 user user 6422 Nov  1 16:25 abschluss_warten.md
        -rw-rw-r-- 1 user user 4245 Nov  1 16:25 erweiterbare_zusatzparameter.md
```
/answer f0c0-20 allow
ReasoningToken: 21
Control Request:
```yaml
id: f0c0-21
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
    script:
    - head -15 specs/abschluss_warten.md
    - PYTHONPATH=/home/user/xyan/xy.ai.workbench/mcpc/src python3 batch_specs.py 2>&1 | tail
  reason: Ergebnisinhalt und Resume-Verhalten prüfen
```
/answer f0c0-21 allow
Control Request:
```yaml
id: f0c0-22
result:
  structuredContent:
    results:
    - stdout: |
        # Spezifikation: Abschluss-Warten

        ## 1. Zweck

        Das Modul „Abschluss-Warten“ stellt Wartenden einen asynchronen Mechanismus bereit, der auf den Endzustand einer Arbeitseinheit reagiert. Ein Wartender wird geweckt, sobald die Arbeitseinheit in `completed` oder `cancelled` übergeht. Das Modul blockiert keine Threads und verliert keine Notification, auch nicht bei Wettlaufsituationen zwischen Zustandsprüfung und Wartebeginn.

        ## 2. Verantwortlichkeiten

        - Asynchrones Warten auf den Endzustand `completed` oder `cancelled`, ohne einen Thread zu blockieren.
        - Verlustfreie Zustellung von Abschluss-Notifications, auch bei Abschluss zwischen Zustandsprüfung und Wartebeginn.
        - Gleichzeitiges Wecken mehrerer Wartender.

        ## 3. Begriffe

        | Begriff | Bedeutung |
        [skip] Erweiterbare Zusatzparameter
        [skip] Abschluss-Warten
```
/answer f0c0-22 allow
Result Stats: id=0bd74981-2940-4ae2-a5ac-0536a4443da6, total: 34438, in: 42, out: 9619, reason: 0, read: 347353, write: 24777, start: 1793546739353, end: 1793546744076, time: 4677, total: 115496, io: 0, control: 46, sent: 0, recv: 121
Der Test lief durch: Beide Module wurden spezifiziert und als Markdown gespeichert.

- **Aufruf:** Ich habe das Script mit `PYTHONPATH=/home/user/xyan/xy.ai.workbench/mcpc/src python3 batch_specs.py` gestartet. Ohne diesen Pfad findet Python das Paket `xy` nicht.
- **Profil:** Der erste Lauf scheiterte mit „Not logged in“. Ich habe deshalb den Parameter `--profile` ergänzt, Default ist `personal`. Damit lief es durch.
- **Ergebnis:** In `specs/` liegen `erweiterbare_zusatzparameter.md` und `abschluss_warten.md`. Ich habe nur den Anfang von `abschluss_warten.md` angesehen, der Aufbau mit Zweck, Verantwortlichkeiten und Begriffen sieht sinnvoll aus. Die Spezifikationen selbst habe ich nicht inhaltlich geprüft.
- **Resume:** Ein zweiter Lauf hat beide Module mit `[skip]` übersprungen.
- **Fehlerabbruch:** Den Abbruch beim Fehler habe ich nur indirekt gesehen. Der Lauf mit fehlender Anmeldung gab `[fail]` aus. Danach lief nichts weiter, es wurde keine Datei geschrieben.
- **Nicht getestet:** Ob Haiku `read_specifications` tatsächlich aufruft, habe ich nicht geprüft. Beide Module haben keine Dependencies, und das zweite Modul konnte das erste lesen. Die Logs unter `config.cli_log_dir` zeigen, ob das passiert ist.
Erzeuge noch eine Bash Script für den start
SystemInit:  id=0bd74981-2940-4ae2-a5ac-0536a4443da6, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: f0c0-23
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/run.sh
    source: |
      #!/usr/bin/env bash
      set -euo pipefail

      BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
      export PYTHONPATH="/home/user/xyan/xy.ai.workbench/mcpc/src${PYTHONPATH:+:$PYTHONPATH}"

      exec python3 "$BASE_DIR/batch_specs.py" "$@"
  reason: Start-Script mit gesetztem PYTHONPATH anlegen
```
/answer f0c0-23 allow
Control Request:
```yaml
id: f0c0-24
toolName: bash
arguments:
  items:
  - cwd: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch
    script:
    - chmod +x run.sh batch_specs.py
    - ./run.sh
  reason: Script ausführbar machen und per Resume-Lauf prüfen
```
/answer f0c0-24 allow
Control Request:
```yaml
id: f0c0-25
result:
  structuredContent:
    results:
    - stdout: |
        [skip] Erweiterbare Zusatzparameter
        [skip] Abschluss-Warten
```
/answer f0c0-25 allow
Result Stats: id=0bd74981-2940-4ae2-a5ac-0536a4443da6, total: 36490, in: 46, out: 10315, reason: 0, read: 397564, write: 26129, start: 1793546806919, end: 1793546809779, time: 2818, total: 122239, io: 0, control: 42, sent: 0, recv: 45
Ich habe `run.sh` im Verzeichnis `23_batch` angelegt und ausführbar gemacht. Es setzt `PYTHONPATH` auf `/home/user/xyan/xy.ai.workbench/mcpc/src` und startet `batch_specs.py`. Ein Testlauf überspringt beide Module, weil ihre Spezifikationen schon existieren.

Alle Argumente werden an das Python-Script durchgereicht, zum Beispiel:

```bash
./run.sh --force
./run.sh --profile personal --spec-dir /anderer/pfad
```
