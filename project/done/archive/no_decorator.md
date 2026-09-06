Ich möchte das es neben dem Decorator noch eine syntaktisch saubere Art gibt ein Tool mittels eines Objektes zu registrieren.

- Tool: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py`
- Registry: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py`

SystemInit:  id=bf9f4957-92c2-4443-aab0-3503e6092918, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 613c-1
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  reason: "Aktuelle Registry-Implementierung verstehen, um Objekt-basierte Registrierung zu ergänzen"
```
/answer 613c-1 allow
Control Request:
```yaml
id: 613c-2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py
  reason: Beispiel-Tool-Implementierung mit Decorator verstehen
```
/answer 613c-2 allow
ReasoningToken: 29
Control Request:
```yaml
id: 613c-3
toolName: python-ast-outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  reason: Struktur der Registry-Klasse für gezielten Einbau prüfen
```
/answer 613c-3 allow
Control Request:
```yaml
id: 613c-4
result:
  structuredContent:
    all_ok: true
    files:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
      ok: true
      error: null
      stats:
        path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
        size_bytes: 11211
        lines: 305
        words: 1179
        complexity: 0.585
        created: 2026-08-25T19:52:30.683625+00:00
        modified: 2026-08-25T19:52:30.683625+00:00
        accessed: 2026-08-25T19:52:35.848602+00:00
        line_length_max: 90
        line_length_min: 0
        line_length_avg: 35.74
        words_per_line_avg: 3.87
        checksum: 5c4bdbe70dca92b1cc11072c1dc5c159ff40a6b71c85ffcc0d3b1638010043ba
      imports:
      - names: __future__:annotations
        lineno: 8
      - names: "dataclasses:dataclass, field"
        lineno: 10
      - names: "typing:TYPE_CHECKING, Any, Callable, Iterable"
        lineno: 11
      - names: .codec:JsonCodec
        lineno: 13
      - names: .session:Session
        lineno: 14
      classes:
      - name: ToolContext
        qualified_name: ToolContext
        lineno: 21
        end_lineno: 28
        docstring: Context handed to a tool handler on invocation.
        methods: []
      - name: ToolResult
        qualified_name: ToolResult
        lineno: 39
        end_lineno: 67
        docstring: The result of a tool call (maps onto MCP ``CallToolResult``).
        methods:
        - name: to_dict
          qualified_name: ToolResult.to_dict
          lineno: 56
          end_lineno: 67
          docstring: null
      - name: Tool
        qualified_name: Tool
        lineno: 83
        end_lineno: 110
        docstring: A registered tool and its MCP metadata.
        methods:
        - name: to_spec
          qualified_name: Tool.to_spec
          lineno: 95
          end_lineno: 110
          docstring: Return the MCP ``Tool`` object advertised via ``tools/list``.
      - name: ToolRegistry
        qualified_name: ToolRegistry
        lineno: 206
        end_lineno: 305
        docstring: Process-wide registry of available tools.
        methods:
        - name: __init__
          qualified_name: ToolRegistry.__init__
          lineno: 209
          end_lineno: 213
          docstring: null
        - name: register_alias
          qualified_name: ToolRegistry.register_alias
          lineno: 215
          end_lineno: 221
          docstring: "Define (or extend) a tool-set alias expanding to *members*. Generic mechanism: …"
        - name: expand_aliases
          qualified_name: ToolRegistry.expand_aliases
          lineno: 223
          end_lineno: 232
          docstring: Expand any alias names in *names* to their member tool names.
        - name: is_enabled
          qualified_name: ToolRegistry.is_enabled
          lineno: 234
          end_lineno: 236
          docstring: "Whether *name* is enabled for *session*, honouring tool-set aliases."
        - name: register
          qualified_name: ToolRegistry.register
          lineno: 238
          end_lineno: 254
          docstring: null
        - name: tool
          qualified_name: ToolRegistry.tool
          lineno: 256
          end_lineno: 282
          docstring: Decorator registering the decorated function as a tool handler.
        - name: get
          qualified_name: ToolRegistry.get
          lineno: 284
          end_lineno: 285
          docstring: null
        - name: __contains__
          qualified_name: ToolRegistry.__contains__
          lineno: 287
          end_lineno: 288
          docstring: null
        - name: __len__
          qualified_name: ToolRegistry.__len__
          lineno: 290
          end_lineno: 291
          docstring: null
        - name: names
          qualified_name: ToolRegistry.names
          lineno: 293
          end_lineno: 294
          docstring: null
        - name: list_for_session
          qualified_name: ToolRegistry.list_for_session
          lineno: 296
          end_lineno: 305
          docstring: Reconcile the registry with the session's enabled-tool configuration. Returns t…
      functions:
      - name: text_content
        qualified_name: text_content
        lineno: 70
        end_lineno: 72
        docstring: Build a ``TextContent`` block.
      - name: normalize_result
        qualified_name: normalize_result
        lineno: 113
        end_lineno: 132
        docstring: Coerce whatever a handler returned into a :class:`ToolResult`.
      - name: _inject_property
        qualified_name: _inject_property
        lineno: 146
        end_lineno: 169
        docstring: Return *schema* with an additional property generically injected. Used both for…
      - name: _with_mandatory_reason
        qualified_name: _with_mandatory_reason
        lineno: 172
        end_lineno: 187
        docstring: "Return *schema* with a mandatory, short ``reason`` property injected. Every too…"
      - name: _with_optional_control_hint
        qualified_name: _with_optional_control_hint
        lineno: 190
        end_lineno: 203
        docstring: Return *schema* with the optional ``controlHint`` output property injected. Doc…
```
/answer 613c-4 allow
ReasoningToken: 935
Control Request:
```yaml
id: 613c-5
toolName: python-ast-imports
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  operation: add
  items:
  - code: "from abc import ABC, abstractmethod"
  reason: ABC für objektbasierte Tool-Definition ergänzen
```
/answer 613c-5 allow
ReasoningToken: 198
Text:
 Jetzt die `ToolDefinition`-Basisklasse nach `Tool` einfügen:
Control Request:
```yaml
id: 613c-6
toolName: python-ast-insert
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  qualified_name: Tool
  position: after
  code: |
    class ToolDefinition(ABC):
        """Base class for registering a tool as an object instead of via ``@registry.tool``.

        Subclasses declare the MCP metadata as class attributes and implement
        :meth:`handle`; an instance is itself a callable :data:`ToolHandler` and
        can be passed directly to :meth:`ToolRegistry.register`::

            class MyTool(ToolDefinition):
                name = "my_tool"
                description = "..."
                input_schema = {...}

                def handle(self, ctx: ToolContext) -> ToolResult:
                    ...

            registry.register(MyTool())
        """

        name: str
        description: str
        input_schema: dict[str, Any]
        title: str | None = None
        output_schema: dict[str, Any] | None = None
        annotations: dict[str, Any] | None = None

        @abstractmethod
        def handle(self, ctx: ToolContext) -> "ToolResult | str | dict[str, Any]":
            ...

        def __call__(self, ctx: ToolContext) -> "ToolResult | str | dict[str, Any]":
            return self.handle(ctx)

        def to_tool(self) -> Tool:
            return Tool(
                name=self.name,
                description=self.description,
                input_schema=self.input_schema,
                handler=self,
                title=self.title,
                output_schema=self.output_schema,
                annotations=self.annotations,
            )
  reason: Objektbasierte Tool-Registrierung als saubere Alternative zum Decorator anbieten
```
/answer 613c-6 allow
Control Request:
```yaml
id: 613c-7
toolName: python-ast-replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  qualified_name: ToolRegistry.register
  old_text: |2-
        def register(self, tool: Tool) -> Tool:
            if tool.name in self._tools:
  new_text: |2-
        def register(self, tool: "Tool | ToolDefinition") -> Tool:
            if isinstance(tool, ToolDefinition):
                tool = tool.to_tool()
            if tool.name in self._tools:
  reason: register() akzeptiert nun sowohl Tool-Instanzen als auch ToolDefinition-Objekte
```
/answer 613c-7 allow
Control Request:
```yaml
id: 613c-8
toolName: python-ast-validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  reason: Syntaxprüfung nach Erweiterung der Registry
```
/answer 613c-8 allow
Text:
 Jetzt `ToolRegistry.register` so anpassen, dass es auch `ToolDefinition`-Instanzen akzeptiert:
Text:
 Jetzt validieren und das `insert`-Tool als Beispiel auf die objektbasierte Registrierung umstellen:
Control Request:
```yaml
id: 613c-9
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py
  mode: replace
  content: |
    """Insert tool – inserts text at a character offset inside an existing file."""

    from __future__ import annotations

    from pathlib import Path
    from typing import Any

    from ...registry import ToolContext, ToolDefinition, ToolRegistry, ToolResult, text_content


    class InsertTool(ToolDefinition):
        name = "insert"
        title = "Insert into file"
        description = (
            "Insert text at a specific character offset inside an existing file. "
            "The offset is zero-based and refers to the UTF-8 decoded content of the file. "
            "All existing content at and after the offset is shifted right."
        )
        input_schema = {
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Absolute path to the file to modify.",
                },
                "offset": {
                    "type": "integer",
                    "description": "Zero-based character offset at which to insert the new content.",
                    "minimum": 0,
                },
                "content": {
                    "type": "string",
                    "description": "Text to insert at the given offset.",
                },
            },
            "required": ["path", "offset", "content"],
        }
        output_schema = {
            "type": "object",
            "properties": {
                "result": {
                    "type": "string",
                    "description": "``success`` on success.",
                },
            },
            "required": ["result"],
        }
        annotations = {"readOnlyHint": False, "idempotentHint": False, "openWorldHint": False}

        def handle(self, ctx: ToolContext) -> ToolResult:
            args: dict[str, Any] = ctx.arguments
            path_str: str = args["path"]
            offset: int = args["offset"]
            new_content: str = args["content"]

            path = Path(path_str)
            if not path.is_absolute():
                return ToolResult(
                    content=[text_content("Path must be absolute.")],
                    is_error=True,
                )
            if not path.exists():
                return ToolResult(
                    content=[text_content("File not found.")],
                    is_error=True,
                )
            if not path.is_file():
                return ToolResult(
                    content=[text_content("Not a regular file.")],
                    is_error=True,
                )

            try:
                text = path.read_text(encoding="utf-8")
                if offset > len(text):
                    return ToolResult(
                        content=[text_content("Offset is beyond end of file.")],
                        is_error=True,
                    )
                result = text[:offset] + new_content + text[offset:]
                path.write_text(result, encoding="utf-8")
            except OSError as exc:
                return ToolResult(
                    content=[text_content(f"Insert failed: {exc}")],
                    is_error=True,
                )

            return ToolResult(structured_content={"result": "success"}, auto_approve=True)


    def register_insert_tool(registry: ToolRegistry) -> None:
        registry.register(InsertTool())
  reason: Insert-Tool als Beispiel auf objektbasierte ToolDefinition-Registrierung umstellen
```
/answer 613c-9 allow
Control Request:
```yaml
id: 613c-a
toolName: python-ast-validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/insert/__init__.py
  reason: Syntaxprüfung des umgestellten insert-Tools
```
/answer 613c-a allow
Control Request:
```yaml
id: 613c-b
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |
    cd /home/user/xyan/xy.ai.workbench/mcpc && python -c "
    import sys
    sys.path.insert(0, 'src')
    from xy.ai.mcpc.registry import ToolRegistry
    from xy.ai.mcpc.tools.insert import register_insert_tool
    r = ToolRegistry()
    register_insert_tool(r)
    print(r.get('insert').to_spec())
    "
  reason: "Laufzeitprüfung: objektbasierte Registrierung funktioniert end-to-end"
```
/answer 613c-b allow
Control Request:
```yaml
id: 613c-c
result:
  structuredContent:
    stdout: |
      {'name': 'insert', 'description': 'Insert text at a specific character offset inside an existing file. The offset is zero-based and refers to the UTF-8 decoded content of the file. All existing content at and after the offset is shifted right.', 'inputSchema': {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the file to modify.'}, 'offset': {'type': 'integer', 'description': 'Zero-based character offset at which to insert the new content.', 'minimum': 0}, 'content': {'type': 'string', 'description': 'Text to insert at the given offset.'}, 'reason': {'type': 'string', 'description': 'Precise, specific reason for this tool call (what exactly is being retrievedand why it is needed now), shown to the authorizing user.'}}, 'required': ['path', 'offset', 'content', 'reason']}, 'title': 'Insert into file', 'outputSchema': {'type': 'object', 'properties': {'result': {'type': 'string', 'description': '``success`` on success.'}, 'controlHint': {'type': 'string', 'description': 'Optional hint or question from the authorizing user'}}, 'required': ['result']}, '_meta': {'anthropic/maxResultSizeChars': 500000}, 'annotations': {'readOnlyHint': False, 'idempotentHint': False, 'openWorldHint': False}}
```
/answer 613c-c allow
ReasoningToken: 24
ReasoningToken: 85
Funktioniert einwandfrei end-to-end.

**Zusammenfassung:**

- **`registry.py`**: Neue abstrakte Basisklasse `ToolDefinition` hinzugefügt. Subklassen deklarieren die MCP-Metadaten (`name`, `description`, `input_schema`, …) als Klassenattribute und implementieren `handle(ctx)`. Die Instanz ist selbst callable (`__call__` delegiert an `handle`) und über `to_tool()` in ein `Tool` konvertierbar. `ToolRegistry.register()` akzeptiert nun `Tool | ToolDefinition` und konvertiert automatisch.
- **`tools/insert/__init__.py`**: Als Beispiel auf die neue, objektbasierte Registrierung umgestellt (`InsertTool(ToolDefinition)`, registriert via `registry.register(InsertTool())`), funktional identisch zur bisherigen Decorator-Variante.

Der Decorator (`@registry.tool(...)`) bleibt unverändert nutzbar – beide Wege sind jetzt gleichwertige Optionen.
