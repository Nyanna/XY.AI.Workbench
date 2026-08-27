`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py`

Der ToolContext soll separiert werden in wirklichen Kontext (mit Session) und Environment (Services). Services sind etwas, das bei der Instanziierung eines Tools eine Rolle spielt sich aber nicht zwischen Aufrufen unterscheidet. Baue das entsprechend in den Tools um. Bei der Registrierung soll das Environment bereist injected werden für Tools, die dies benötigen.
SystemInit:  id=71a6ca6e-b14a-491c-a0fd-f9a15e07b8fb, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7396-1
toolName: python-ast-outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
  reason: Struktur von tool_context.py verstehen
```
/answer 7396-1 allow
Control Request:
```yaml
id: 7396-2
result:
  structuredContent:
    all_ok: true
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
      ok: true
      error: null
      stats:
        path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/tool_context.py
        size_bytes: 1032
        lines: 29
        words: 98
        complexity: 0.453
        created: 2026-08-27T15:43:57.145509+00:00
        modified: 2026-08-27T15:43:57.145509+00:00
        accessed: 2026-08-27T15:46:22.610213+00:00
        line_length_max: 70
        line_length_min: 0
        line_length_avg: 34.62
        words_per_line_avg: 3.38
        checksum: 1fcc13bc4b10368e5efaa3ab5018b039b759f0c03d7659a843ebaa9086347ea2
      imports:
      - names: __future__:annotations
        lineno: 3
      - names: dataclasses:dataclass
        lineno: 4
      - names: "typing:TYPE_CHECKING, Any"
        lineno: 5
      classes:
      - name: ToolContext
        qualified_name: ToolContext
        lineno: 15
        end_lineno: 19
        docstring: Context handed to a tool handler on invocation.
        methods: []
      - name: AppServices
        qualified_name: AppServices
        lineno: 22
        end_lineno: 29
        docstring: Container for the shared components a tool handler may need.
        methods: []
      functions: []
```
/answer 7396-2 allow
ReasoningToken: 30
Control Request:
```yaml
id: 7396-3
toolName: grep
arguments:
  directory: /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: ToolContext|AppServices
  reason: "Alle Verwendungen von ToolContext und AppServices finden, um die Umstrukturierung durchzuführen"
```
/answer 7396-3 allow
Control Request:
```yaml
id: 7396-4
result:
  content:
  - type: text
    text: "Full output written to file (11035 characters). Before loading the file, reduce the content to what is strictly needed: use targeted commands (grep, head, tail, awk) to extract only the relevant parts.Only load the file with `file-read` once the output is already narrowed down to the essential information."
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-p6bvco27.log
```
/answer 7396-4 allow
ReasoningToken: 9
Control Request:
```yaml
id: 7396-5
toolName: grep
arguments:
  directory: /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: ToolContext|AppServices
  reason: Alle Verwendungen anzeigen (voller Output)
  include: '*.py'
```
/answer 7396-5 allow