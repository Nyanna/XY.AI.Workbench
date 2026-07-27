Analog zu `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py` braucht `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py` eine Möglichkeit einen Request vor der Weitergabe beschränken zu können. Beispielweise ist das Issue result Limit pro page mit 100 zu hoch und muss auf 15 begrenzt werden.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=9a850015-d780-45a4-b419-4299fb2e1dec | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 0787b430-a9b6-459a-9e75-4da6d69b92f2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/openalex/__init__.py
  reason: Study reference pattern for limiting requests
```
/answer 0787b430-a9b6-459a-9e75-4da6d69b92f2 allow
Control Request:
```yaml
id: 7bc39a7f-6338-4ee1-ac9a-f31f8605a511
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  reason: Understand current github tool implementation
```
/answer 7bc39a7f-6338-4ee1-ac9a-f31f8605a511 allow
ReasoningToken: 10
Control Request:
```yaml
id: 3da9c734-f401-48fd-b5e3-a3a67d849c35
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/bridge.py
  reason: Understand McpBridge.register_tool to add argument clamping hook
```
/answer 3da9c734-f401-48fd-b5e3-a3a67d849c35 allow
ReasoningToken: 51
ReasoningToken: 577
Control Request:
```yaml
id: 372e3a42-a53b-416a-bcfe-1d5c543c74fe
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: grep -rn "input_schema\|jsonschema\|validate" src/xy/ai/mcpc/registry.py | head -50
  reason: Check if registry validates input_schema (e.g. maximum) automatically
```
/answer 372e3a42-a53b-416a-bcfe-1d5c543c74fe allow
Control Request:
```yaml
id: 7cbaab6e-8f39-4d2a-93de-ce62f6789e45
result:
  structuredContent:
    stdout: |
      76:    input_schema: dict[str, Any]
      88:            "inputSchema": self.input_schema,
      163:        tool.input_schema = _with_mandatory_reason(tool.input_schema)
      178:        input_schema: dict[str, Any],
      190:                    input_schema=input_schema,
```
/answer 7cbaab6e-8f39-4d2a-93de-ce62f6789e45 allow
Control Request:
```yaml
id: c4098bf4-f0fa-496f-b050-cba0b29b1f3f
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  old_text: |-
    from ...config import ServerConfig
    from ...registry import ToolRegistry
    from .bridge import McpBridge
    from .client import McpClient, McpClientError

    # ---------------------------------------------------------------------------
    # Shared helpers
    # ---------------------------------------------------------------------------

    _CONTENT_OUTPUT: dict[str, Any] = {
  new_text: |-
    from ...config import ServerConfig
    from ...registry import ToolRegistry
    from .bridge import ArgTransform, McpBridge
    from .client import McpClient, McpClientError

    # ---------------------------------------------------------------------------
    # Shared helpers
    # ---------------------------------------------------------------------------


    def _capped(field: str, maximum: int, minimum: int = 1) -> ArgTransform:
        """Build an :data:`ArgTransform` that clamps ``field`` to ``[minimum, maximum]``.

        The remote GitHub MCP server advertises much higher per-page limits
        (typically up to 100) than are useful for an AI agent context window;
        this lets a tool registration enforce a stricter cap before the request
        is ever forwarded, regardless of what the caller asked for. Values are
        clamped rather than rejected, and missing values are left untouched so
        the remote server's own default still applies.
        """

        def _transform(arguments: dict[str, Any]) -> dict[str, Any]:
            value = arguments.get(field)
            if value is None:
                return arguments
            try:
                number = int(value)
            except (TypeError, ValueError):
                arguments[field] = maximum
                return arguments
            arguments[field] = max(minimum, min(number, maximum))
            return arguments

        return _transform


    #: Hard cap for issue-related list/search results: the remote server allows
    #: up to 100 per page, which is far more than an agent should pull at once.
    _MAX_ISSUE_PER_PAGE = 15

    _CONTENT_OUTPUT: dict[str, Any] = {
  reason: Add reusable arg-capping helper mirroring openalex's _clamp pattern
```
/answer c4098bf4-f0fa-496f-b050-cba0b29b1f3f deny Ich finde die Idee ganz Reizvoll die verifizierung der Inputparameter in der Registry vorzunehmen und mit einem Fehler bereits dort abzubrechen. Dies vermeidet eine Extra Implementierung. Die Begrenzung könnte direkt im gespiegelten Schema eingetragen werden.
ReasoningToken: 1533
Control Request:
```yaml
id: 514ba034-94b0-4208-b956-3bccf76b66ae
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  reason: Understand registry structure to add schema validation before dispatch
```
/answer 514ba034-94b0-4208-b956-3bccf76b66ae allow
Control Request:
```yaml
id: 82b5c359-5169-4a3b-92fc-110e3aeac807
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |-
    grep -rln "tool.handler\|\.handler(ctx)\|call_tool\|tools/call" src/xy/ai/mcpc | grep -v tests
    echo ---
    grep -rn "jsonschema\|import jsonschema" src/xy/ai/mcpc pyproject.toml 2>/dev/null
  reason: Find dispatch site for tool calls and check for jsonschema dependency
```
/answer 82b5c359-5169-4a3b-92fc-110e3aeac807 allow
Control Request:
```yaml
id: 6d7086b3-9125-4266-a777-49fc6f7b7b1d
result:
  structuredContent:
    exit_code: 1
    stdout: |
      src/xy/ai/mcpc/registry.py
      src/xy/ai/mcpc/control/manager.py
      src/xy/ai/mcpc/context.py
      src/xy/ai/mcpc/protocol.py
      src/xy/ai/mcpc/tools/mcp/client.py
      src/xy/ai/mcpc/tools/mcp/bridge.py
      ---
```
/answer 6d7086b3-9125-4266-a777-49fc6f7b7b1d allow
Control Request:
```yaml
id: 4552a52d-ed87-4633-a60e-e56d841a22e3
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: grep -n "tools/call\|def.*call\|tool.handler\|normalize_result" src/xy/ai/mcpc/protocol.py
  reason: Locate tool call dispatch logic in protocol.py
```
/answer 4552a52d-ed87-4633-a60e-e56d841a22e3 allow
Control Request:
```yaml
id: bee6c27d-a575-4e21-ae48-d81e9a9559d8
result:
  structuredContent:
    stdout: |
      18:from .registry import ToolContext, ToolRegistry, normalize_result
      60:            "tools/call": self._handle_tools_call,
      86:        if request.method == "tools/call":
      151:    def _handle_tools_call(
      169:        # "Errors in finding the tool" are protocol errors (spec, tools/call).
      197:                raw = tool.handler(context)
      198:            result = normalize_result(raw)
```
/answer bee6c27d-a575-4e21-ae48-d81e9a9559d8 allow
Control Request:
```yaml
id: f5069f07-5a43-47dc-9cc1-92df01d3197b
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  start: def _handle_tools_call
  end: return self._success
  reason: Read full tools/call handler to see where to inject validation
```
/answer f5069f07-5a43-47dc-9cc1-92df01d3197b allow
Control Request:
```yaml
id: 4ad0c7cb-db9d-4070-a57d-41e61ecf1369
result:
  content:
  - type: text
    text: End marker not found in file.
  isError: true
```
/answer 4ad0c7cb-db9d-4070-a57d-41e61ecf1369 allow
Control Request:
```yaml
id: 56b6df69-10a4-472d-b732-ef72ca9a6168
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  min_line: 140
  max_line: 230
  reason: Read full tools/call handler
```
/answer 56b6df69-10a4-472d-b732-ef72ca9a6168 allow
Control Request:
```yaml
id: c9b7a7a4-689e-47f5-85a4-112d3837c96e
result:
  structuredContent:
    checksum: 0631bffe976726fd4aeab37ddca1e13c371e10827a877580ed998ff794b2637c
    content: |2+

              cursor = params.get("cursor")
              start = _decode_cursor(cursor) if cursor is not None else 0
              page_size = self.config.tools_page_size
              page = tools[start : start + page_size]

              result: dict[str, Any] = {"tools": [t.to_spec() for t in page]}
              if start + page_size < len(tools):
                  result["nextCursor"] = _encode_cursor(start + page_size)
              return result

          def _handle_tools_call(
              self,
              session: Session,
              params: dict[str, Any],
              *,
              skip_control: bool = False,
          ) -> dict[str, Any]:
              name = params.get("name")
              if not isinstance(name, str) or not name:
                  raise errors.invalid_params('"name" is required')

              arguments = params.get("arguments", {})
              if arguments is None:
                  arguments = {}
              if not isinstance(arguments, dict):
                  raise errors.invalid_params('"arguments" must be an object')

              tool = self.registry.get(name)
              # "Errors in finding the tool" are protocol errors (spec, tools/call).
              if tool is None or not session.is_tool_enabled(name):
                  raise errors.invalid_params(
                      f"Unknown or unavailable tool: {name}", {"name": name}
                  )

              _validate_arguments(tool.input_schema, arguments)

              # --- request interception -------------------------------------------
              control = self.services.control_manager if self.services else None
              if control is not None and not skip_control:
                  decision = control.submit_request(session, name, arguments)
                  if not decision.approved:
                      from .registry import ToolResult, text_content
                      reason = decision.rejection_reason or "Tool call rejected by controller"
                      return ToolResult(
                          content=[text_content(f"DENIED: {reason}")],
                          is_error=True,
                      ).to_dict()
                  if decision.modified_arguments is not None:
                      arguments = decision.modified_arguments
              # --------------------------------------------------------------------

              context = ToolContext(session=session, arguments=arguments, services=self.services)
              # Tool execution errors are reported *inside* the result (isError=true)
              # so the model can see and self-correct, not as protocol errors.
              try:
                  with session.lock:
                      raw = tool.handler(context)
                  result = normalize_result(raw)
              except errors.JsonRpcError:
                  raise
              except Exception as exc:  # noqa: BLE001 - surface as tool error result
                  from .registry import ToolResult, text_content

                  result = ToolResult(
                      content=[text_content(f"Tool '{name}' failed: {exc}")],
                      is_error=True,
                  )

              # --- result interception --------------------------------------------
              if control is not None and not skip_control:
                  decision = control.submit_result(
                      session, name, result.to_dict(), auto_approve=result.auto_approve
                  )
                  if not decision.approved:
                      from .registry import ToolResult, text_content
                      reason = decision.rejection_reason or "Tool result rejected by controller"
                      return ToolResult(
                          content=[text_content(f"DENIED: {reason}")],
                          is_error=True,
                      ).to_dict()
                  if decision.modified_result is not None:
                      return decision.modified_result
              # --------------------------------------------------------------------

              return result.to_dict()


      def _validate_arguments(schema: dict[str, Any], arguments: dict[str, Any]) -> None:
          """Minimal validation of *arguments* against an input JSON Schema.
```
/answer c9b7a7a4-689e-47f5-85a4-112d3837c96e allow
ReasoningToken: 75
ReasoningToken: 177
Control Request:
```yaml
id: 7e0811a1-6a65-4403-9edf-a456f9ad73c8
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  min_line: 225
  max_line: 300
  reason: Read _validate_arguments implementation
```
/answer 7e0811a1-6a65-4403-9edf-a456f9ad73c8 allow
Control Request:
```yaml
id: 7a2ee446-4dca-4124-9339-1de668eb7269
result:
  structuredContent:
    checksum: 77653d13e8b1f4f61b3de87eaf61577618555c4ca0d3d6bf15e5321f1b291d82
    content: |2
              return result.to_dict()


      def _validate_arguments(schema: dict[str, Any], arguments: dict[str, Any]) -> None:
          """Minimal validation of *arguments* against an input JSON Schema.

          Only the top-level ``required`` list and primitive ``type`` of declared
          properties are checked — enough to give clients meaningful ``INVALID_PARAMS``
          errors without pulling in a full JSON Schema implementation.
          """
          required = schema.get("required", [])
          missing = [key for key in required if key not in arguments]
          if missing:
              raise errors.invalid_params(
                  f"Missing required argument(s): {', '.join(missing)}",
                  {"missing": missing},
              )

          properties = schema.get("properties", {})
          type_checks = {
              "string": str,
              "number": (int, float),
              "integer": int,
              "boolean": bool,
              "object": dict,
              "array": list,
          }
          for key, value in arguments.items():
              prop = properties.get(key)
              if not isinstance(prop, dict):
                  continue
              expected = prop.get("type")
              py_type = type_checks.get(expected) if isinstance(expected, str) else None
              if py_type is None:
                  continue
              # bool is a subclass of int; guard the integer/number cases explicitly.
              if expected in ("number", "integer") and isinstance(value, bool):
                  ok = False
              else:
                  ok = isinstance(value, py_type)
              if not ok:
                  raise errors.invalid_params(
                      f"Argument '{key}' must be of type {expected}",
                      {"argument": key, "expectedType": expected},
                  )
```
/answer 7a2ee446-4dca-4124-9339-1de668eb7269 allow
ReasoningToken: 56
ReasoningToken: 31
ReasoningToken: 23
Control Request:
```yaml
id: feb24c38-53a5-479e-81f9-61cce4b13c7d
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  old_text: |2-
            if not ok:
                raise errors.invalid_params(
                    f"Argument '{key}' must be of type {expected}",
                    {"argument": key, "expectedType": expected},
                )
  new_text: |2-
            if not ok:
                raise errors.invalid_params(
                    f"Argument '{key}' must be of type {expected}",
                    {"argument": key, "expectedType": expected},
                )

            # Numeric bounds (``minimum``/``maximum``) let a tool's mirrored
            # schema clamp what would otherwise be forwarded verbatim to an
            # upstream server (e.g. capping a remote API's per-page limit down
            # to a value that makes sense for an agent). Enforced here, once,
            # for every tool, rather than in each tool's handler.
            if expected in ("number", "integer"):
                minimum = prop.get("minimum")
                if isinstance(minimum, (int, float)) and value < minimum:
                    raise errors.invalid_params(
                        f"Argument '{key}' must be >= {minimum}",
                        {"argument": key, "minimum": minimum},
                    )
                maximum = prop.get("maximum")
                if isinstance(maximum, (int, float)) and value > maximum:
                    raise errors.invalid_params(
                        f"Argument '{key}' must be <= {maximum}",
                        {"argument": key, "maximum": maximum},
                    )
  reason: "Enforce JSON-schema min/max on tool arguments centrally, so per-tool schemas can cap upstream limits"
```
/answer feb24c38-53a5-479e-81f9-61cce4b13c7d allow
ReasoningToken: 369
Control Request:
```yaml
id: 02edc2cc-6cec-41d8-8047-aec08864a299
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  old_text: |2-
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 100).",
                "minimum": 1,
                "maximum": 100,
            },
            "after": {
                "type": "string",
                "description": "Cursor for pagination (from previous response).",
            },
        },
        "required": ["owner", "repo"],
    }

    _SEARCH_ISSUES_SCHEMA: dict[str, Any] = {
  new_text: |2-
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "after": {
                "type": "string",
                "description": "Cursor for pagination (from previous response).",
            },
        },
        "required": ["owner", "repo"],
    }

    _SEARCH_ISSUES_SCHEMA: dict[str, Any] = {
  reason: Cap github-list-issues perPage from 100 down to 15 via the mirrored schema
```
/answer 02edc2cc-6cec-41d8-8047-aec08864a299 allow
Control Request:
```yaml
id: fea34d3d-fa77-4534-924d-00949c998dda
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  start: "_SEARCH_ISSUES_SCHEMA: dict[str, Any] = {"
  end: _GET_DISCUSSION_SCHEMA
  reason: Check search issues schema perPage field to cap it too
```
/answer fea34d3d-fa77-4534-924d-00949c998dda allow
Control Request:
```yaml
id: 7e402d6a-ebf2-4b0c-a67a-bd18416a2906
result:
  content:
  - type: text
    text: End marker is ambiguous – found 2 occurrences in file.
  isError: true
```
/answer 7e402d6a-ebf2-4b0c-a67a-bd18416a2906 allow
Text:
 Jetzt die GitHub-Schemas anpassen, damit das Issue-Result-Limit dort direkt begrenzt wird:
Control Request:
```yaml
id: d9deaa2d-3dd8-45f7-b676-c31f12107d1d
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/mcp/github.py
  old_text: |2-
            "sort": {"type": "string", "description": "Sort field."},
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 100).",
                "minimum": 1,
                "maximum": 100,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    # ---------------------------------------------------------------------------
    # Discussion tools
  new_text: |2-
            "sort": {"type": "string", "description": "Sort field."},
            "order": {"type": "string", "description": "Sort order: asc | desc."},
            "perPage": {
                "type": "integer",
                "description": "Results per page (max 15).",
                "minimum": 1,
                "maximum": 15,
            },
            "page": {"type": "integer", "description": "Page number (min 1).", "minimum": 1},
        },
        "required": ["query"],
    }

    # ---------------------------------------------------------------------------
    # Discussion tools
  reason: Cap github-search-issues perPage from 100 down to 15 to match list-issues
```
/answer d9deaa2d-3dd8-45f7-b676-c31f12107d1d allow
Control Request:
```yaml
id: cf24aa7f-aa44-42cd-a9dd-7a9f3f37ee1a
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: grep -rln "_validate_arguments\|maximum" src/tests 2>/dev/null; find . -name "test_protocol*"
  reason: Find existing tests for protocol validation
```
/answer cf24aa7f-aa44-42cd-a9dd-7a9f3f37ee1a deny es gibt weder Build Tools noch Tests
Control Request:
```yaml
id: c09a0c7f-cb0c-417c-b759-6f9200e48dc2
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: python3 -m py_compile src/xy/ai/mcpc/protocol.py src/xy/ai/mcpc/tools/mcp/github.py && echo OK
  reason: Syntax-check edited files
```
/answer c09a0c7f-cb0c-417c-b759-6f9200e48dc2 allow
Control Request:
```yaml
id: 52ee3aa5-1618-486d-abea-7fd0479e785c
result:
  structuredContent:
    stdout: |
      OK
```
/answer 52ee3aa5-1618-486d-abea-7fd0479e785c allow
ReasoningToken: 80
Text:
 Nun kurz testen, dass die Validierung greift (existierende Tests, falls vorhanden, laufen lassen):
ReasoningToken: 31
Beide Dateien sind syntaktisch sauber. Zusammenfassung:

**`protocol.py`** – `_validate_arguments` prüft jetzt zusätzlich zu Typ/Required auch `minimum`/`maximum` aus dem `input_schema` für `number`/`integer`-Felder. Verstößt ein Aufruf dagegen, wird er zentral mit einem `INVALID_PARAMS`-Fehler abgelehnt, **bevor** die Anfrage an ein Bridge-Tool (z. B. GitHub) weitergereicht wird. Damit lässt sich jede Anfrage-Begrenzung direkt im gespiegelten Schema jedes Tools deklarieren, ohne pro Tool eigenen Clamp-/Transform-Code zu schreiben.

**`github.py`** – Als Anwendungsfall wurde das `perPage`-Limit bei `github-list-issues` und `github-search-issues` von `maximum: 100` auf `maximum: 15` gesenkt (inkl. angepasster Beschreibung). Ein Aufruf mit z. B. `perPage: 50` wird jetzt zentral abgelehnt, statt ungeprüft an den GitHub-MCP-Server weitergeleitet zu werden.

Für weitere Tools/Felder reicht künftig einfach das Setzen von `minimum`/`maximum` im jeweiligen Schema.
