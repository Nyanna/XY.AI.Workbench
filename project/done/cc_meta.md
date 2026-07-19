Implementiere für alle Tool-Outputs generisch die Anthropic Meta Annotation "anthropic/maxResultSizeChars".

* Einstiegspunkt `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py`

```json
{
  "content": [
    {
      "type": "text",
      "text": "... dein großer Datei-Inhalt / Output ..."
    }
  ],
  "_meta": {
    "anthropic/maxResultSizeChars": 500000
  }
}
```
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=23a7936c-0d4b-4bfb-9b0b-4fecd1252a2d | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 77b2c079-4b97-4038-8469-55f5fd89464e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  reason: Analyse für Meta-Annotation Implementierung
```
/answer 77b2c079-4b97-4038-8469-55f5fd89464e allow
Control Request:
```yaml
id: 14b45823-f1cc-46a9-9c59-01ef6436ea38
toolName: bash
arguments:
  script: grep -rn "to_dict\|ToolResult(\|_meta\|maxResultSizeChars" /home/user/xyan/xy.ai.workbench/mcpc/src --include=*.py
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Finde alle Verwendungen von ToolResult.to_dict und _meta
```
/answer 14b45823-f1cc-46a9-9c59-01ef6436ea38 allow
Control Request:
```yaml
id: 5de49470-1db0-413f-ac85-926f5c76524f
result:
  content:
  - type: text
    text: Full output written to file 10299 characters). Read only relevant excerpts (e.g. via grep/head/tail).
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-8bc1gfbu.log
```
/answer 5de49470-1db0-413f-ac85-926f5c76524f allow
ReasoningToken: 160
Control Request:
```yaml
id: 76a776dd-e76e-4a6f-aab4-7d8f196d31ad
toolName: bash
arguments:
  script: cat /tmp/mcpc-stdout-8bc1gfbu.log
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Log ausgeben
```
/answer 76a776dd-e76e-4a6f-aab4-7d8f196d31ad allow
Control Request:
```yaml
id: 51b65089-a153-4414-aa3c-180192863b78
result:
  content:
  - type: text
    text: Full output written to file 10299 characters). Read only relevant excerpts (e.g. via grep/head/tail).
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-i6dmc3h2.log
```
/answer 51b65089-a153-4414-aa3c-180192863b78 allow
Control Request:
```yaml
id: 2ad0eafc-b678-4412-a173-228c5196acf1
toolName: bash
arguments:
  script: wc -l /tmp/mcpc-stdout-8bc1gfbu.log; head -c 3000 /tmp/mcpc-stdout-8bc1gfbu.log
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Log anzeigen
```
/answer 2ad0eafc-b678-4412-a173-228c5196acf1 allow
Control Request:
```yaml
id: 43a3cdc8-3d13-4085-a81a-3d521051ae27
result:
  content:
  - type: text
    text: Full output written to file 3033 characters). Read only relevant excerpts (e.g. via grep/head/tail).
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-5ksp__yc.log
```
/answer 43a3cdc8-3d13-4085-a81a-3d521051ae27 allow
ReasoningToken: 25
Control Request:
```yaml
id: c8a133ad-24d5-4013-87d0-aa3d35809f5e
toolName: read-file
arguments:
  path: /tmp/mcpc-stdout-8bc1gfbu.log
  reason: Log lesen
```
/answer c8a133ad-24d5-4013-87d0-aa3d35809f5e allow
ReasoningToken: 42
Control Request:
```yaml
id: 0aeacd42-c6cb-4081-93bc-ab22c40fb1e8
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  reason: Kontext für ToolResult.to_dict Verwendung prüfen
  min_line: 150
  max_line: 230
```
/answer 0aeacd42-c6cb-4081-93bc-ab22c40fb1e8 allow
Control Request:
```yaml
id: 08b9cb0b-416b-4355-9e96-984048377d14
result:
  structuredContent:
    checksum: 8382268c248edd1dec7fbbdf13dc77d039963b24faf67c1daf027c394ce18402
    content: |2+

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

    path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
    modified: 2026-07-19T11:58:16.315764+00:00
    size: 10528
    lines: 269
```
/answer 08b9cb0b-416b-4355-9e96-984048377d14 allow
ReasoningToken: 108
Control Request:
```yaml
id: 184529ad-32dc-4e4b-9d0d-040a0e122406
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  old_text: |-
    @dataclass(slots=True)
    class ToolResult:
        """The result of a tool call (maps onto MCP ``CallToolResult``)."""

        content: list[dict[str, Any]] = field(default_factory=list)
        structured_content: dict[str, Any] | None = None
        is_error: bool = False
        auto_approve: bool = False

        def to_dict(self) -> dict[str, Any]:
            result: dict[str, Any] = {}
            if self.content:
                result["content"] = list(self.content)
            if self.structured_content:
                result["structuredContent"] = self.structured_content
            if self.is_error:
                result["isError"] = True
            return result
  new_text: |-
    #: Default value for the Anthropic-specific ``anthropic/maxResultSizeChars``
    #: meta annotation, applied generically to every tool result (see
    #: :meth:`ToolResult.to_dict`). This tells Anthropic-compatible MCP clients
    #: how many characters of the result they may render/keep before truncating.
    ANTHROPIC_MAX_RESULT_SIZE_CHARS = 500_000


    @dataclass(slots=True)
    class ToolResult:
        """The result of a tool call (maps onto MCP ``CallToolResult``)."""

        content: list[dict[str, Any]] = field(default_factory=list)
        structured_content: dict[str, Any] | None = None
        is_error: bool = False
        auto_approve: bool = False
        #: Extra ``_meta`` entries to merge on top of the generic defaults (e.g.
        #: to override ``anthropic/maxResultSizeChars`` for a specific tool).
        meta: dict[str, Any] | None = None

        def to_dict(self) -> dict[str, Any]:
            result: dict[str, Any] = {}
            if self.content:
                result["content"] = list(self.content)
            if self.structured_content:
                result["structuredContent"] = self.structured_content
            if self.is_error:
                result["isError"] = True
            meta: dict[str, Any] = {
                "anthropic/maxResultSizeChars": ANTHROPIC_MAX_RESULT_SIZE_CHARS
            }
            if self.meta:
                meta.update(self.meta)
            result["_meta"] = meta
            return result
  reason: _meta mit anthropic/maxResultSizeChars generisch für alle Tool-Outputs hinzufügen
```
/answer 184529ad-32dc-4e4b-9d0d-040a0e122406 allow
ReasoningToken: 255
Control Request:
```yaml
id: c6928f2f-d38b-4a98-9ae9-de4c9f4402fa
toolName: bash
arguments:
  script: grep -rln "to_dict\|CallToolResult\|isError" /home/user/xyan/xy.ai.workbench/mcpc/tests 2>/dev/null
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: "Tests finden, die von to_dict abhängen"
```
/answer c6928f2f-d38b-4a98-9ae9-de4c9f4402fa allow
Control Request:
```yaml
id: 3c65076f-82cd-4bf8-be71-7e7ad7769df0
result:
  structuredContent:
    exit_code: 2
    stdout: ""
```
/answer 3c65076f-82cd-4bf8-be71-7e7ad7769df0 allow
Control Request:
```yaml
id: b404ec10-954c-419b-9dbb-af3a53d9c2c8
toolName: bash
arguments:
  script: find /home/user/xyan/xy.ai.workbench/mcpc -iname "*test*" -maxdepth 3
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Testverzeichnis finden
```
/answer b404ec10-954c-419b-9dbb-af3a53d9c2c8 deny Es gibt keine Tests
Control Request:
```yaml
id: e0ebfcfb-fab2-4a5e-bcce-91efedf79b79
toolName: bash
arguments:
  script: |
    cd /home/user/xyan/xy.ai.workbench/mcpc && python3 -c "
    import sys; sys.path.insert(0, 'src')
    from xy.ai.mcpc.registry import ToolResult, text_content
    r = ToolResult(content=[text_content('hallo')])
    print(r.to_dict())
    r2 = ToolResult(content=[text_content('x')], meta={'anthropic/maxResultSizeChars': 100})
    print(r2.to_dict())
    "
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  reason: Syntax und Funktion verifizieren
```
/answer e0ebfcfb-fab2-4a5e-bcce-91efedf79b79 allow
Control Request:
```yaml
id: c2369206-ca4d-4cce-ad90-8cee16dec5b6
result:
  structuredContent:
    stdout: |
      {'content': [{'type': 'text', 'text': 'hallo'}], '_meta': {'anthropic/maxResultSizeChars': 500000}}
      {'content': [{'type': 'text', 'text': 'x'}], '_meta': {'anthropic/maxResultSizeChars': 100}}
```
/answer c2369206-ca4d-4cce-ad90-8cee16dec5b6 allow
Text:
 Jetzt prüfen, ob `decision.modified_result` (Control-Manager-Pfad) auch damit umgehen kann, und ob es Tests gibt, die auf die exakte `to_dict()`-Struktur ohne `_meta` prüfen.
Text:
 Keine Tests vorhanden. Jetzt kurz verifizieren, dass das Modul syntaktisch korrekt ist.
Fertig. Zusammenfassung der Änderung in `registry.py`:

- Neue Konstante `ANTHROPIC_MAX_RESULT_SIZE_CHARS = 500_000`.
- `ToolResult` hat ein neues optionales Feld `meta: dict[str, Any] | None`, mit dem einzelne Tools den Wert pro Ergebnis überschreiben können.
- `ToolResult.to_dict()` fügt jetzt generisch für **jeden** Tool-Output einen `_meta`-Block mit `"anthropic/maxResultSizeChars": 500000` hinzu (per-Tool via `meta=` überschreibbar), genau wie im gewünschten JSON-Beispiel.

Da `to_dict()` der zentrale Serialisierungspunkt in `protocol.py` (`_handle_tools_call`) für alle Tool-Antworten ist – inkl. der DENIED-/Fehlerpfade – gilt die Annotation automatisch für sämtliche Tools (bash, read, write, change, insert, list, agent, mcp-bridge, openalex usw.), ohne dass jedes Tool einzeln angepasst werden muss.
