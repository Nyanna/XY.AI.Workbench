Ein häufiges Prompt Pattern ist, bei einem erlaubten Tool dem Agenten einen weiteren Hinweis für die Verarbeitung oder Folgefragen oder Hinweise mitzugeben.
- Dem Kontrollkommando `/allow <ID>` soll ein optionaler Hinweistext folgen, der dem Agenten mit dem Ergebnis zurückgegeben wird. Das ist analog dem deny mit dem Unterschied das der Hint den Toolaufruf nicht abbricht, sondern das Ergebnis anreichert.
- Dafür soll der generische Schema inject verwendet werden, der bereit für den "reason" genutzt wird
- Ein "allow" Hint für eine Anfrage und das Ergebnis zugleich wird im selben Feld kombiniert.
- Dem result Schema wird per generischem inject ein unabhängiges Feld hinzugefügt, dass das eigentliche Result nicht beeinträchtigt
- Ausnahme bildet das `ask-user` Tool, indem reason und allow hint gleich behandelt werden und dem Agenten eine Antwort liefern 

Server: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/handler.py`, `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py`, `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py`
Client: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java`
SystemInit:  id=3e7b3f1f-3006-4eb6-a976-003ce8133ada, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 5e4a-1
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/handler.py
  reason: Verstehen der aktuellen /allow und /deny Verarbeitung um Hint-Feature zu implementieren
```
/answer 5e4a-1 allow
Control Request:
```yaml
id: 5e4a-2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  reason: Verstehen der Manager-Logik für allow/deny und generisches inject Schema
```
/answer 5e4a-2 allow
Control Request:
```yaml
id: 5e4a-3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  reason: Verstehen wie Tool-Schemas (reason inject) aktuell generiert werden
```
/answer 5e4a-3 allow
ReasoningToken: 50
Control Request:
```yaml
id: 5e4a-4
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  reason: Verstehen der Client-seitigen Verarbeitung von /allow und /deny Kommandos
```
/answer 5e4a-4 allow
ReasoningToken: 383
Control Request:
```yaml
id: 5e4a-5
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "\"/allow\"\|/allow \|allow(" --include=*.java src | grep -i allow
  reason: Finden wo /allow Kommando geparst und approve() aufgerufen wird
```
/answer 5e4a-5 allow
Control Request:
```yaml
id: 5e4a-6
result:
  structuredContent:
    exit_code: 1
    stdout: ""
```
/answer 5e4a-6 allow
Control Request:
```yaml
id: 5e4a-7
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rln "ask-user\|ask_user\|askUser" --include=*.py /home/user/xyan/xy.ai.workbench/mcpc/src
  reason: Finden der ask-user Tool Implementierung im Server
```
/answer 5e4a-7 allow
Control Request:
```yaml
id: 5e4a-8
result:
  structuredContent:
    stdout: |
      /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
      /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py
      /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ask-user/__init__.py
```
/answer 5e4a-8 allow
ReasoningToken: 65
Control Request:
```yaml
id: 5e4a-9
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rln "allow" --include=*.java src | xargs grep -ln "deny\b"
  reason: "Finden der Klasse, die /allow und /deny Kommandos verarbeitet"
```
/answer 5e4a-9 allow
Control Request:
```yaml
id: 5e4a-a
result:
  structuredContent:
    stdout: |
      src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      src/xy/ai/workbench/connector/claudecode/CCConnector.java
```
/answer 5e4a-a allow
Control Request:
```yaml
id: 5e4a-b
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ask-user/__init__.py
  reason: Verstehen des ask-user Tools und wie reason/hint gehandhabt wird
```
/answer 5e4a-b allow
Control Request:
```yaml
id: 5e4a-c
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
  script: grep -rn "submit_request\|submit_result\|modified_result\|modified_arguments\|rejection_reason\|ControlDecision" --include=*.py .
  reason: "Finden aller Stellen, wo ControlDecision genutzt wird, um Injection-Punkte zu verstehen"
```
/answer 5e4a-c allow
Control Request:
```yaml
id: 5e4a-d
result:
  structuredContent:
    stdout: |
      ./control/manager.py:35:class ControlDecision:
      ./control/manager.py:41:    rejection_reason: str | None = None
      ./control/manager.py:44:    modified_arguments: dict[str, Any] | None = None
      ./control/manager.py:47:    modified_result: dict[str, Any] | None = None
      ./control/manager.py:64:    _decision: ControlDecision | None = field(default=None, repr=False)
      ./control/manager.py:88:        decision = manager.submit_request("bash", {"command": "rm -rf /"})
      ./control/manager.py:106:    def submit_request(
      ./control/manager.py:113:    ) -> ControlDecision:
      ./control/manager.py:116:        Returns a :class:`ControlDecision`.  If the decision includes
      ./control/manager.py:117:        ``modified_arguments``, the caller should substitute them before
      ./control/manager.py:125:            return ControlDecision(approved=True)
      ./control/manager.py:130:    def submit_result(
      ./control/manager.py:137:    ) -> ControlDecision:
      ./control/manager.py:140:        Returns a :class:`ControlDecision`.  If the decision includes
      ./control/manager.py:141:        ``modified_result``, the caller should use that instead of the
      ./control/manager.py:156:            return ControlDecision(approved=True)
      ./control/manager.py:193:                decision = ControlDecision(
      ./control/manager.py:195:                    rejection_reason=approval.get("reason") or "Rejected by controller",
      ./control/manager.py:198:                decision = ControlDecision(
      ./control/manager.py:200:                    modified_arguments=approval.get("arguments"),
      ./control/manager.py:201:                    modified_result=approval.get("result"),
      ./control/manager.py:218:        the interceptor thread blocked in :meth:`submit_request` /
      ./control/manager.py:219:        :meth:`submit_result` doesn't hang around until the (up to 24h)
      ./control/manager.py:231:            item._decision = ControlDecision(approved=False, rejection_reason=reason)
      ./control/manager.py:266:    def _wait(self, item: _PendingItem) -> ControlDecision:
      ./control/manager.py:277:            return ControlDecision(approved=True)
      ./control/__init__.py:4:from .manager import ToolControlManager, ControlDecision
      ./control/__init__.py:6:__all__ = ["ControlHandler", "ToolControlManager", "ControlDecision"]
      ./protocol.py:180:            decision = control.submit_request(session, name, arguments)
      ./protocol.py:183:                reason = decision.rejection_reason or "Tool call rejected by controller"
      ./protocol.py:190:            if decision.modified_arguments is not None:
      ./protocol.py:191:                arguments = decision.modified_arguments
      ./protocol.py:213:            decision = control.submit_result(
      ./protocol.py:218:                reason = decision.rejection_reason or "Tool result rejected by controller"
      ./protocol.py:225:            if decision.modified_result is not None:
      ./protocol.py:226:                return decision.modified_result
```
/answer 5e4a-d allow
ReasoningToken: 748
Control Request:
```yaml
id: 5e4a-e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  reason: Verstehen des vollen Kontexts der Tool-Call-Verarbeitung inkl. request/result Interception für ask-user Sonderbehandlung"
```
/answer 5e4a-e allow
Control Request:
```yaml
id: 5e4a-f
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  old_text: |2-
        rejection_reason: str | None = None
        """Human-readable hint for the agent when ``approved`` is False."""

        modified_arguments: dict[str, Any] | None = None
  new_text: |2-
        rejection_reason: str | None = None
        """Human-readable hint for the agent when ``approved`` is False."""

        approval_hint: str | None = None
        """Optional human-written hint attached to an *approval* (``/allow <id> <hint>``).

        Unlike :attr:`rejection_reason`, this never aborts the call — it enriches
        the eventual result instead. The same field is used for both the
        ``request`` and ``result`` phase; when a hint is supplied at both phases
        for the same call, the two are combined (see ``protocol.py``).
        """

        modified_arguments: dict[str, Any] | None = None
  reason: Feld für Allow-Hint zur ControlDecision hinzufügen
```
/answer 5e4a-f allow
Control Request:
```yaml
id: 5e4a-10
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  old_text: |2-
            * ``{"id": "…"}`` — simple approval, keep original data.
            * ``{"id": "…", "rejected": true, "reason": "…"}`` — rejection.
            * ``{"id": "…", "arguments": {…}}`` — approve with modified arguments.
            * ``{"id": "…", "result": {…}}`` — approve with replaced result.
            """
  new_text: |2-
            * ``{"id": "…"}`` — simple approval, keep original data.
            * ``{"id": "…", "rejected": true, "reason": "…"}`` — rejection.
            * ``{"id": "…", "arguments": {…}}`` — approve with modified arguments.
            * ``{"id": "…", "result": {…}}`` — approve with replaced result.
            * ``{"id": "…", "hint": "…"}`` — approve with a hint for the agent
              (``/allow <id> <hint>``); does not reject the call, only enriches
              the eventual result.
            """
  reason: Docstring der process_approvals um hint-Feld ergänzen
```
/answer 5e4a-10 allow
Control Request:
```yaml
id: 5e4a-11
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  old_text: |2-
                    decision = ControlDecision(
                        approved=True,
                        modified_arguments=approval.get("arguments"),
                        modified_result=approval.get("result"),
                    )
  new_text: |2-
                    decision = ControlDecision(
                        approved=True,
                        modified_arguments=approval.get("arguments"),
                        modified_result=approval.get("result"),
                        approval_hint=approval.get("hint"),
                    )
  reason: Hint-Wert aus der Approval-Nachricht in die ControlDecision übernehmen
```
/answer 5e4a-11 allow
ReasoningToken: 4570
Control Request:
```yaml
id: 5e4a-12
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
    @dataclass(slots=True)
    class ToolResult:
        """The result of a tool call (maps onto MCP ``CallToolResult``)."""

        content: list[dict[str, Any]] = field(default_factory=list)
        structured_content: dict[str, Any] | None = None
        is_error: bool = False
        auto_approve: bool = False
        control_hint: str | None = None
        """Optional hint attached by the controller on approval (``/allow <id> <hint>``).

        Rendered as an independent top-level field (see :data:`CONTROL_HINT_PROPERTY`)
        that never touches ``content``/``structuredContent``/``isError`` — the actual
        tool result is left untouched, the hint merely rides along for the agent.
        """

        def to_dict(self) -> dict[str, Any]:
            result: dict[str, Any] = {}
            if self.content:
                result["content"] = list(self.content)
            if self.structured_content:
                result["structuredContent"] = self.structured_content
            if self.is_error:
                result["isError"] = True
            if self.control_hint:
                result[CONTROL_HINT_PROPERTY] = self.control_hint
            return result
  reason: "ToolResult um unabhängiges control_hint Feld erweitern, das das eigentliche Ergebnis nicht beeinflusst"
```
/answer 5e4a-12 allow
Control Request:
```yaml
id: 5e4a-13
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  old_text: |-
    #: Name of the mandatory reason property injected into every tool's input
    #: schema (see :func:`_with_mandatory_reason`).
    REASON_PROPERTY = "reason"


    def _with_mandatory_reason(schema: dict[str, Any]) -> dict[str, Any]:
        """Return *schema* with a mandatory, short ``reason`` property injected.

        Every tool call must carry an extremely short reason/goal for the call so
        the authorizing user can review it (e.g. via the human-in-the-loop
        control layer) before or while it executes. This is applied centrally at
        registration time so individual tool modules never need to declare it
        themselves.
        """
        schema = dict(schema)
        properties = dict(schema.get("properties", {}))
        properties[REASON_PROPERTY] = {
            "type": "string",
            "reason": (
                "Precise, specific reason for this tool call (what exactly is being retrieved"
                "and why it is needed now), shown to the authorizing user."
            ),
        }
        schema["properties"] = properties
        required = list(schema.get("required", []))
        if REASON_PROPERTY not in required:
            required.append(REASON_PROPERTY)
        schema["required"] = required
        return schema
  new_text: |-
    #: Name of the mandatory reason property injected into every tool's input
    #: schema (see :func:`_inject_property`).
    REASON_PROPERTY = "reason"

    #: Name of the independent hint property injected into every tool's output
    #: schema (see :func:`_inject_property`). Populated at call time from an
    #: ``/allow <id> <hint>`` control decision (see ``ControlDecision.approval_hint``
    #: in ``control/manager.py``); never required and never part of the actual
    #: ``content``/``structuredContent`` payload, so it cannot interfere with it.
    CONTROL_HINT_PROPERTY = "controlHint"


    def _inject_property(
        schema: dict[str, Any],
        name: str,
        description: str,
        *,
        required: bool,
    ) -> dict[str, Any]:
        """Return *schema* with an additional property generically injected.

        Used both for the mandatory ``reason`` property on every tool's input
        schema and for the optional ``controlHint`` property on every tool's
        output schema — the same generic mechanism, applied at registration time
        so individual tool modules never need to declare either themselves.
        """
        schema = dict(schema)
        properties = dict(schema.get("properties", {}))
        properties[name] = {"type": "string", "description": description}
        schema["properties"] = properties
        if required:
            required_list = list(schema.get("required", []))
            if name not in required_list:
                required_list.append(name)
            schema["required"] = required_list
        return schema


    def _with_mandatory_reason(schema: dict[str, Any]) -> dict[str, Any]:
        """Return *schema* with a mandatory, short ``reason`` property injected.

        Every tool call must carry an extremely short reason/goal for the call so
        the authorizing user can review it (e.g. via the human-in-the-loop
        control layer) before or while it executes.
        """
        return _inject_property(
            schema,
            REASON_PROPERTY,
            (
                "Precise, specific reason for this tool call (what exactly is being retrieved"
                "and why it is needed now), shown to the authorizing user."
            ),
            required=True,
        )


    def _with_optional_control_hint(schema: dict[str, Any]) -> dict[str, Any]:
        """Return *schema* with the optional ``controlHint`` output property injected.

        Documents the independent, optional field that may accompany a tool
        result when the authorizing user attached a hint to an ``/allow``
        decision. Does not affect ``content``/``structuredContent``.
        """
        return _inject_property(
            schema,
            CONTROL_HINT_PROPERTY,
            (
                "Optional hint from the authorizing user, attached via an '/allow' "
                "decision. Does not represent tool output data."
            ),
            required=False,
        )
  reason: Generische inject-Funktion einführen und für reason sowie neues controlHint Output-Feld wiederverwenden
```
/answer 5e4a-13 allow
Control Request:
```yaml
id: 5e4a-14
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  old_text: |2
            tool.input_schema = _with_mandatory_reason(tool.input_schema)
  new_text: |2
            tool.input_schema = _with_mandatory_reason(tool.input_schema)
            if tool.output_schema is not None:
                tool.output_schema = _with_optional_control_hint(tool.output_schema)
  reason: Bei Registrierung auch das Output-Schema um das unabhängige controlHint Feld ergänzen
```
/answer 5e4a-14 allow
ReasoningToken: 91
Control Request:
```yaml
id: 5e4a-15
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  old_text: |2-
            # --- request interception -------------------------------------------
            control = self.services.control_manager if self.services else None
            if control is not None and not skip_control:
                decision = control.submit_request(session, name, arguments)
                if not decision.approved:
                    from .registry import ToolResult, text_content
                    reason = decision.rejection_reason or "Tool call rejected by controller"
                    if name == "ask-user":
                        return ToolResult(structured_content={"answer": reason}).to_dict()
                    return ToolResult(
                        content=[text_content(f"DENIED: {reason}")],
                        is_error=True,
                    ).to_dict()
                if decision.modified_arguments is not None:
                    arguments = decision.modified_arguments
            # --------------------------------------------------------------------
  new_text: |2-
            # --- request interception -------------------------------------------
            control = self.services.control_manager if self.services else None
            request_hint: str | None = None
            if control is not None and not skip_control:
                decision = control.submit_request(session, name, arguments)
                if not decision.approved:
                    from .registry import ToolResult, text_content
                    reason = decision.rejection_reason or "Tool call rejected by controller"
                    if name == "ask-user":
                        return ToolResult(structured_content={"answer": reason}).to_dict()
                    return ToolResult(
                        content=[text_content(f"DENIED: {reason}")],
                        is_error=True,
                    ).to_dict()
                if decision.modified_arguments is not None:
                    arguments = decision.modified_arguments
                if decision.approval_hint:
                    # For ask-user, an approval hint *is* the human's answer: the
                    # question is answered directly instead of merely annotating
                    # a result that would otherwise just be "not answered".
                    if name == "ask-user":
                        from .registry import ToolResult
                        return ToolResult(
                            structured_content={"answer": decision.approval_hint}
                        ).to_dict()
                    request_hint = decision.approval_hint
            # --------------------------------------------------------------------
  reason: "Request-Phase-Hint aus /allow verarbeiten, ask-user als Sonderfall direkt beantworten\""
```
/answer 5e4a-15 allow
Control Request:
```yaml
id: 5e4a-16
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  old_text: |2-
                if decision.modified_result is not None:
                    return decision.modified_result
            # --------------------------------------------------------------------

            return result.to_dict()
  new_text: |2-
                hint_parts = [h for h in (request_hint, decision.approval_hint) if h]
                combined_hint = "\n".join(hint_parts) if hint_parts else None

                if decision.modified_result is not None:
                    result_dict = decision.modified_result
                else:
                    result.control_hint = combined_hint
                    result_dict = result.to_dict()

                if combined_hint and name == "ask-user":
                    # Same exception as in the request phase: for ask-user the
                    # hint *is* the answer, not an independent side-channel field.
                    from .registry import ToolResult
                    return ToolResult(structured_content={"answer": combined_hint}).to_dict()
                if combined_hint and decision.modified_result is not None:
                    from .registry import CONTROL_HINT_PROPERTY
                    result_dict[CONTROL_HINT_PROPERTY] = combined_hint

                return result_dict
            # --------------------------------------------------------------------

            return result.to_dict()
  reason: "Result-Phase-Hint mit Request-Phase-Hint kombinieren, als unabhängiges Feld anhängen bzw. bei ask-user als tatsächliche Antwort verwenden\""
```
/answer 5e4a-16 allow
Control Request:
```yaml
id: 5e4a-17
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  start: '    def _handle_tools_call('
  end: '    def _handle_tools_list'
  reason: Kompletten aktualisierten _handle_tools_call Block prüfen"
```
/answer 5e4a-17 allow
Control Request:
```yaml
id: 5e4a-18
result:
  content:
  - type: text
    text: Resolved end position must not lie before the resolved start position.
  isError: true
```
/answer 5e4a-18 allow
Control Request:
```yaml
id: 5e4a-19
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  start: '    def _handle_tools_call('
  end: def _validate_arguments
  reason: Kompletten aktualisierten _handle_tools_call Block final prüfen
```
/answer 5e4a-19 allow
Control Request:
```yaml
id: 5e4a-1a
result:
  structuredContent:
    checksum: 4feda76b3ae5ce7adee9adbc6d0ad4922116612aff440108298f26563f3b930d
    content: |2-
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
              request_hint: str | None = None
              if control is not None and not skip_control:
                  decision = control.submit_request(session, name, arguments)
                  if not decision.approved:
                      from .registry import ToolResult, text_content
                      reason = decision.rejection_reason or "Tool call rejected by controller"
                      if name == "ask-user":
                          return ToolResult(structured_content={"answer": reason}).to_dict()
                      return ToolResult(
                          content=[text_content(f"DENIED: {reason}")],
                          is_error=True,
                      ).to_dict()
                  if decision.modified_arguments is not None:
                      arguments = decision.modified_arguments
                  if decision.approval_hint:
                      # For ask-user, an approval hint *is* the human's answer: the
                      # question is answered directly instead of merely annotating
                      # a result that would otherwise just be "not answered".
                      if name == "ask-user":
                          from .registry import ToolResult
                          return ToolResult(
                              structured_content={"answer": decision.approval_hint}
                          ).to_dict()
                      request_hint = decision.approval_hint
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
                      if name == "ask-user":
                          return ToolResult(structured_content={"answer": reason}).to_dict()
                      return ToolResult(
                          content=[text_content(f"DENIED: {reason}")],
                          is_error=True,
                      ).to_dict()
                  hint_parts = [h for h in (request_hint, decision.approval_hint) if h]
                  combined_hint = "\n".join(hint_parts) if hint_parts else None

                  if decision.modified_result is not None:
                      result_dict = decision.modified_result
                  else:
                      result.control_hint = combined_hint
                      result_dict = result.to_dict()

                  if combined_hint and name == "ask-user":
                      # Same exception as in the request phase: for ask-user the
                      # hint *is* the answer, not an independent side-channel field.
                      from .registry import ToolResult
                      return ToolResult(structured_content={"answer": combined_hint}).to_dict()
                  if combined_hint and decision.modified_result is not None:
                      from .registry import CONTROL_HINT_PROPERTY
                      result_dict[CONTROL_HINT_PROPERTY] = combined_hint

                  return result_dict
              # --------------------------------------------------------------------

              return result.to_dict()


      def _validate_arguments
```
/answer 5e4a-1a allow
Control Request:
```yaml
id: 5e4a-1b
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rln "allow\b" --include=*.java src
  reason: "Alle Java-Dateien finden, die 'allow' referenzieren, um Kommando-Parsing-Stelle zu lokalisieren"
```
/answer 5e4a-1b allow
Control Request:
```yaml
id: 5e4a-1c
result:
  structuredContent:
    stdout: |
      src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      src/xy/ai/workbench/connector/claudecode/CCConnector.java
```
/answer 5e4a-1c allow
Control Request:
```yaml
id: 5e4a-1d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  reason: Verstehen wo /allow und /deny Kommandos geparst und approve/deny aufgerufen werden"
```
/answer 5e4a-1d allow
Control Request:
```yaml
id: 5e4a-1e
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: "sed -n '1,200p' src/xy/ai/workbench/connector/claudecode/CCRequest.java"
  reason: "Command-Klasse und ihre Konstruktoren einsehen, um Hint-Parameter für Allow zu ergänzen\""
```
/answer 5e4a-1e allow
Control Request:
```yaml
id: 5e4a-1f
result:
  structuredContent:
    stdout: "package xy.ai.workbench.connector.claudecode;\n\nimport java.util.Collections;\nimport java.util.List;\nimport java.util.Objects;\n\nimport xy.ai.workbench.models.IModelRequest;\n\npublic class CCRequest implements IModelRequest {\n\n\tpublic final String id;\n\tpublic final String title;\n\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final Command cmd;\n\n\tpublic CCRequest(String id, String title, String systemPrompt, List<String> tools, Command cmd) {\n\t\tObjects.requireNonNull(cmd, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.title = title;\n\t\tthis.systemPrompt = systemPrompt;\n\t\tthis.tools = tools != null ? tools : Collections.emptyList();\n\t\tthis.cmd = cmd;\n\t}\n\n\t@Override\n\tpublic String getID() {\n\t\treturn id;\n\t}\n\n\tpublic static class Command {\n\t\tpublic final CommandType type;\n\t\tpublic final String parameter;\n\t\tpublic final String[] parameters;\n\n\t\tpublic Command(CommandType type, String... parameters) {\n\t\t\tObjects.requireNonNull(type, \"Type can't be null\");\n\t\t\tthis.type = type;\n\t\t\tthis.parameter = parameters[0];\n\t\t\tthis.parameters = parameters;\n\t\t}\n\t}\n}\n"
```
/answer 5e4a-1f allow
Control Request:
```yaml
id: 5e4a-20
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  old_text: "\t\t\t\tString[] parts = clean.split(\"\\\\s+\", 4);\n\t\t\t\tString id = parts[1];\n\t\t\t\tString action = parts[2].toLowerCase();\n\t\t\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));"
  new_text: "\t\t\t\tString[] parts = clean.split(\"\\\\s+\", 4);\n\t\t\t\tString id = parts[1];\n\t\t\t\tString action = parts[2].toLowerCase();\n\t\t\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id, reason));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));"
  reason: Optionalen Hinweistext nach /allow <ID> ebenfalls als Command-Parameter mitführen"
```
/answer 5e4a-20 allow
Control Request:
```yaml
id: 5e4a-21
result:
  content:
  - type: text
    text: Text not found in file.
  isError: true
```
/answer 5e4a-21 allow
Control Request:
```yaml
id: 5e4a-22
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  old_text: "\t\t\tcase Allow:\n\t\t\t\tcontrolClient.approve(req.cmd.parameter);\n\t\t\t\tbreak;"
  new_text: "\t\t\tcase Allow:\n\t\t\t\tcontrolClient.approve(req.cmd.parameters[0],\n\t\t\t\t\t\treq.cmd.parameters.length > 1 ? req.cmd.parameters[1] : null);\n\t\t\t\tbreak;"
  reason: Allow-Hint an den Control-Client zur Übermittlung an den Server weiterreichen"
```
/answer 5e4a-22 allow
ReasoningToken: 116
Control Request:
```yaml
id: 5e4a-23
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -n "Allow" src/xy/ai/workbench/connector/claudecode/CCConnector.java
  reason: "Aktuellen Stand des Allow-Case Blocks prüfen, da erster Replace fehlgeschlagen ist\""
```
/answer 5e4a-23 allow
Control Request:
```yaml
id: 5e4a-24
result:
  structuredContent:
    stdout: "56:\t\t// Preprocessing: extract Allow/Deny/exit/resume lines\n109:\t\tcase Allow:\n113:\t\t\tcase Allow:\n215:\t\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id));\n"
```
/answer 5e4a-24 allow
Text:
 Jetzt `register()` anpassen, damit auch das Output-Schema den `controlHint` erhält:
Control Request:
```yaml
id: 5e4a-25
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  min_line: 205
  max_line: 222
  reason: Aktuellen Stand des preprocessInput Blocks vor erneutem Patch prüfen"
```
/answer 5e4a-25 allow
Control Request:
```yaml
id: 5e4a-26
result:
  structuredContent:
    checksum: 371047ff613c942c0658a7c252489f1e7882d7e1571ab313456fd26126175db7
    content: "\t\t\t\tif (\"/exit\".equalsIgnoreCase(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Exit, \"\"));\n\t\t\t\telse if (clean.matches(\"(?i)/resume\\\\s+\\\\S+\"))\n\t\t\t\t\tcommands.add(new Command(CommandType.Resume, clean.split(\"\\\\s+\", 2)[1].strip()));\n\t\t\t\telse if (clean.matches(\"(?i)\" + CCControlClient.ANSWER + \"\\\\s+\\\\S+\\\\s+(allow|deny)(\\\\s+.*)?\")) {\n\t\t\t\t\tString[] parts = clean.split(\"\\\\s+\", 4);\n\t\t\t\t\tString id = parts[1];\n\t\t\t\t\tString action = parts[2].toLowerCase();\n\t\t\t\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id));\n\t\t\t\t\telse\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));\n\t\t\t\t} else if (controlClient.submitEdit(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Modification, \"\"));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Prompt, clean));\n\t\tif (commands.isEmpty())\n"
```
/answer 5e4a-26 allow
Control Request:
```yaml
id: 5e4a-27
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  old_text: "\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));"
  new_text: "\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id, reason));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));"
  reason: Optionalen Hinweistext nach /allow <ID> ebenfalls als Command-Parameter mitführen (nach fehlgeschlagenem ersten Versuch erneut)"
```
/answer 5e4a-27 allow
Control Request:
```yaml
id: 5e4a-28
result:
  content:
  - type: text
    text: Text not found in file.
  isError: true
```
/answer 5e4a-28 allow
Control Request:
```yaml
id: 5e4a-29
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -n "CommandType.Allow\|CommandType.Deny" src/xy/ai/workbench/connector/claudecode/CCConnector.java
  reason: Exakten aktuellen Text der betroffenen Zeilen ermitteln"
```
/answer 5e4a-29 deny grep liefert nicht garantiert die korrekten whitepaces, das tun nur die direkten Lese-Tools
Control Request:
```yaml
id: 5e4a-2a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  min_line: 205
  max_line: 222
  reason: Exakten aktuellen Text mit korrekten Whitespaces erneut lesen für replace-block"
```
/answer 5e4a-2a allow
Control Request:
```yaml
id: 5e4a-2b
result:
  structuredContent:
    checksum: 371047ff613c942c0658a7c252489f1e7882d7e1571ab313456fd26126175db7
    content: "\t\t\t\tif (\"/exit\".equalsIgnoreCase(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Exit, \"\"));\n\t\t\t\telse if (clean.matches(\"(?i)/resume\\\\s+\\\\S+\"))\n\t\t\t\t\tcommands.add(new Command(CommandType.Resume, clean.split(\"\\\\s+\", 2)[1].strip()));\n\t\t\t\telse if (clean.matches(\"(?i)\" + CCControlClient.ANSWER + \"\\\\s+\\\\S+\\\\s+(allow|deny)(\\\\s+.*)?\")) {\n\t\t\t\t\tString[] parts = clean.split(\"\\\\s+\", 4);\n\t\t\t\t\tString id = parts[1];\n\t\t\t\t\tString action = parts[2].toLowerCase();\n\t\t\t\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id));\n\t\t\t\t\telse\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));\n\t\t\t\t} else if (controlClient.submitEdit(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Modification, \"\"));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Prompt, clean));\n\t\tif (commands.isEmpty())\n"
```
/answer 5e4a-2b allow
ReasoningToken: 25
Control Request:
```yaml
id: 5e4a-2c
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  old_text: "\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id));"
  new_text: "\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id, reason));"
  reason: Optionalen Hinweistext nach /allow <ID> ebenfalls als Command-Parameter mitführen"
```
/answer 5e4a-2c allow
Control Request:
```yaml
id: 5e4a-2d
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  old_text: "\tpublic ArrayNode approve(String id) {\n\t\treturn submit(approvalNode(id, null, null, null));\n\t}\n\n\tpublic ArrayNode deny(String id, String reason) {\n\t\treturn submit(approvalNode(id, null, null, reason == null ? \"\" : reason));\n\t}"
  new_text: "\tpublic ArrayNode approve(String id) {\n\t\treturn approve(id, null);\n\t}\n\n\t/**\n\t * Approve the pending item {@code id}. If {@code hint} is non-blank, it is\n\t * sent along as an enriching hint for the agent (does not reject the\n\t * call, unlike {@link #deny(String, String)}).\n\t */\n\tpublic ArrayNode approve(String id, String hint) {\n\t\treturn submit(approvalNode(id, null, null, null, hint == null || hint.isBlank() ? null : hint));\n\t}\n\n\tpublic ArrayNode deny(String id, String reason) {\n\t\treturn submit(approvalNode(id, null, null, reason == null ? \"\" : reason, null));\n\t}"
  reason: "approve() um optionalen Hint-Parameter erweitern, analog zu deny()\""
```
/answer 5e4a-2d allow
ReasoningToken: 167
Control Request:
```yaml
id: 5e4a-2e
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  old_text: "\tprivate ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result, String rejectReason) {\n\t\tObjectNode approval = mapper.createObjectNode();\n\t\tapproval.put(\"id\", id);\n\t\tif (arguments != null)\n\t\t\tapproval.set(\"arguments\", arguments);\n\t\tif (result != null)\n\t\t\tapproval.set(\"result\", result);\n\t\tif (rejectReason != null) {\n\t\t\tapproval.put(\"rejected\", true);\n\t\t\tapproval.put(\"reason\", rejectReason);\n\t\t}\n\t\treturn approval;\n\t}"
  new_text: "\tprivate ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result, String rejectReason,\n\t\t\tString hint) {\n\t\tObjectNode approval = mapper.createObjectNode();\n\t\tapproval.put(\"id\", id);\n\t\tif (arguments != null)\n\t\t\tapproval.set(\"arguments\", arguments);\n\t\tif (result != null)\n\t\t\tapproval.set(\"result\", result);\n\t\tif (rejectReason != null) {\n\t\t\tapproval.put(\"rejected\", true);\n\t\t\tapproval.put(\"reason\", rejectReason);\n\t\t}\n\t\tif (hint != null)\n\t\t\tapproval.put(\"hint\", hint);\n\t\treturn approval;\n\t}"
  reason: "Hint-Feld in das Approval-JSON aufnehmen, das an den Server gesendet wird\""
```
/answer 5e4a-2e allow
ReasoningToken: 311
ReasoningToken: 161
Control Request:
```yaml
id: 5e4a-2f
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "controlClient.approve\|\.approve(" --include=*.java src
  reason: "Prüfen, ob approve() an weiteren Stellen mit altem Signaturaufruf verwendet wird\""
```
/answer 5e4a-2f allow
Control Request:
```yaml
id: 5e4a-30
result:
  structuredContent:
    stdout: "src/xy/ai/workbench/connector/claudecode/CCConnector.java:114:\t\t\t\tcontrolClient.approve(req.cmd.parameters[0],\n"
```
/answer 5e4a-30 allow
Text:
 Jetzt passe ich `CCConnector` an, damit der Hint auch bei `Allow` mitgegeben wird, und `CCControlClient.approve` erweitere ich um einen optionalen Hint-Parameter:
Control Request:
```yaml
id: 5e4a-31
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc/src
  script: python3 -m py_compile xy/ai/mcpc/protocol.py xy/ai/mcpc/registry.py xy/ai/mcpc/control/manager.py && echo OK
  reason: Syntax der geänderten Python-Dateien verifizieren"
```
/answer 5e4a-31 allow
Control Request:
```yaml
id: 5e4a-32
result:
  structuredContent:
    stdout: |
      OK
```
/answer 5e4a-32 allow
Control Request:
```yaml
id: 5e4a-33
toolName: python
arguments:
  script: |2

    import sys
    sys.path.insert(0, "/home/user/xyan/xy.ai.workbench/mcpc/src")
    from xy.ai.mcpc.registry import ToolRegistry, ToolResult, CONTROL_HINT_PROPERTY, REASON_PROPERTY

    reg = ToolRegistry()

    @reg.tool("demo", description="d", input_schema={"type":"object","properties":{}},
              output_schema={"type":"object","properties":{"x":{"type":"string"}}})
    def h(ctx):
        return ToolResult(structured_content={"x": "1"})

    t = reg.get("demo")
    print("input reason required:", REASON_PROPERTY in t.input_schema["required"])
    print("output has hint prop:", CONTROL_HINT_PROPERTY in t.output_schema["properties"])
    print("output hint required:", CONTROL_HINT_PROPERTY in t.output_schema.get("required", []))

    r = ToolResult(structured_content={"x":"1"}, control_hint="be careful")
    print(r.to_dict())
  reason: Verhalten der neuen inject-Mechanik und control_hint Feld end-to-end verifizieren"
```
/answer 5e4a-33 allow
Control Request:
```yaml
id: 5e4a-34
result:
  structuredContent:
    exit_code: 0
    stdout: |
      input reason required: True
      output has hint prop: True
      output hint required: False
      {'structuredContent': {'x': '1'}, 'controlHint': 'be careful'}
```
/answer 5e4a-34 allow
ReasoningToken: 30
Text:
 Der zweite Replace ist offenbar schon gelaufen (control client approve), aber der erste (parts[1]) nicht — Reihenfolge der parallelen Calls war vertauscht. Ich hole aktuellen Ausschnitt und korrigiere.
ReasoningToken: 48
ReasoningToken: 90
ReasoningToken: 128
Text:
 Jetzt der `CCControlClient` – `approve` um optionalen Hint erweitern:
ReasoningToken: 385
Text:
 Gut. Nun Python-Seite auf Syntaxfehler prüfen:
Text:
 Alle drei Dateien kompilieren fehlerfrei. Nun eine kurze funktionale Prüfung der neuen Logik mit einem kleinen Testskript:
Funktioniert wie erwartet. Zusammenfassung der Änderungen:

**Server (Python)**

1. **`registry.py`**: `_with_mandatory_reason` zu generischer `_inject_property(schema, name, description, required)` verallgemeinert. Neue `_with_optional_control_hint` nutzt dieselbe Funktion, um bei der Tool-Registrierung ein unabhängiges, optionales `controlHint`-Feld ins `output_schema` zu injizieren. `ToolResult` hat ein neues `control_hint`-Attribut, das in `to_dict()` als eigenständiges Top-Level-Feld (`controlHint`) angehängt wird – ohne `content`/`structuredContent`/`isError` zu berühren.

2. **`control/manager.py`**: `ControlDecision` hat ein neues Feld `approval_hint`. In `process_approvals` wird bei nicht-abgelehnten Entscheidungen `approval.get("hint")` übernommen. Anders als `rejection_reason` bricht dies den Call nie ab.

3. **`protocol.py`**: In `_handle_tools_call` werden Hints aus Request- und Result-Phase gesammelt und (falls beide vorhanden) im selben Feld kombiniert (`\n`-getrennt), bevor sie ins `controlHint`-Feld des Ergebnisses geschrieben werden. Ausnahme `ask-user`: dort wird ein Hint direkt als `answer` zurückgegeben (gleiche Behandlung wie `reason`/Ablehnungsgrund), da das die eigentliche Antwortfunktion dieses Tools ist.

**Client (Java)**

4. **`CCControlClient.java`**: `approve(id)` delegiert an neues `approve(id, hint)`, das ein `hint`-Feld im Approval-JSON mitsendet, sofern nicht leer. `approvalNode(...)` um den `hint`-Parameter erweitert.

5. **`CCConnector.java`**: Der `/answer <id> allow <hint>`-Befehl übergibt den optionalen Text jetzt als zusätzlichen Command-Parameter, der beim Aufruf von `controlClient.approve(id, hint)` mitgeschickt wird (vorher wurde er bei `allow` verworfen, nur bei `deny` verwendet).
