Der "approval_hint" in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py`prüfe das.
SystemInit:  id=2c4b90fb-f795-47d2-a093-24870dee9041, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: a165-2a
toolName: python-ast-outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  reason: "Übersicht über manager.py verschaffen, um approval_hint zu finden"
```
/answer a165-2a allow
ReasoningToken: 17
Control Request:
```yaml
id: a165-2b
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: grep -rn "approval_hint" src/ | head -50
  reason: Alle Vorkommen von approval_hint im Projekt finden
```
/answer a165-2b allow
Control Request:
```yaml
id: a165-2c
result:
  structuredContent:
    stdout: |
      src/xy/ai/mcpc/registry.py:138:#: ``/allow <id> <hint>`` control decision (see ``ControlDecision.approval_hint``
      src/xy/ai/mcpc/control/manager.py:44:    approval_hint: str | None = None
      src/xy/ai/mcpc/control/manager.py:214:                    approval_hint=approval.get("hint"),
      src/xy/ai/mcpc/protocol.py:193:            if decision.approval_hint:
      src/xy/ai/mcpc/protocol.py:200:                        structured_content={"answer": decision.approval_hint}
      src/xy/ai/mcpc/protocol.py:202:                request_hint = decision.approval_hint
      src/xy/ai/mcpc/protocol.py:236:            hint_parts = [h for h in (request_hint, decision.approval_hint) if h]
    stderr: |
      grep: src/xy/ai/mcpc/__pycache__/protocol.cpython-312.pyc: binary file matches
      grep: src/xy/ai/mcpc/control/__pycache__/manager.cpython-312.pyc: binary file matches
```
/answer a165-2c allow
ReasoningToken: 27
Control Request:
```yaml
id: a165-2d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  min_line: 30
  max_line: 230
  reason: Kontext um approval_hint verstehen
```
/answer a165-2d allow
Control Request:
```yaml
id: a165-2e
result:
  structuredContent:
    checksum: 5ccf23469a64cda76266748750003d24f7937c4e3e5aa277d4be129b2e4e353e
    content: |
      # ---------------------------------------------------------------------------
      # Decision DTO
      # ---------------------------------------------------------------------------

      @dataclass(slots=True)
      class ControlDecision:
          """The outcome of a human review, produced by :meth:`ToolControlManager.process_approvals`."""

          approved: bool
          """True when the call should proceed (possibly with modified data)."""

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
          """Replacement arguments for the ``request`` phase (``None`` → keep originals)."""

          modified_result: dict[str, Any] | None = None
          """Replacement result dict for the ``result`` phase (``None`` → keep original)."""


      # ---------------------------------------------------------------------------
      # Internal pending item
      # ---------------------------------------------------------------------------

      @dataclass
      class _PendingItem:
          id: str
          phase: str          # "request" | "result"
          tool_name: str
          session_id: str
          arguments: dict[str, Any] | None       # populated in request phase
          result: dict[str, Any] | None          # populated in result phase
          _event: threading.Event = field(default_factory=threading.Event, repr=False)
          _decision: ControlDecision | None = field(default=None, repr=False)

          def to_dict(self) -> dict[str, Any]:
              item: dict[str, Any] = {"id": self.id}
              if self.arguments is not None:
                  item["toolName"] = self.tool_name
                  item["arguments"] = self.arguments
              if self.result is not None:
                  item["result"] = self.result
              return item


      # ---------------------------------------------------------------------------
      # Manager
      # ---------------------------------------------------------------------------

      class ToolControlManager:
          """Thread-safe manager for human-in-the-loop tool interception.

          Usage::

              manager = ToolControlManager()

              # In the interceptor thread (blocks until a decision is received):
              decision = manager.submit_request("bash", {"command": "rm -rf /"})

              # In the control endpoint handler (non-blocking):
              pending = manager.get_pending()
              manager.process_approvals([{"id": "…", "approved": True}])
          """

          def __init__(self, timeout: float = _DEFAULT_TIMEOUT) -> None:
              self._timeout = timeout
              self._pending: dict[str, _PendingItem] = {}
              self._lock = threading.Lock()
              self._id_prefix = uuid.uuid4().hex[:4]
              self._id_counter = 0

          # ------------------------------------------------------------------
          # Interceptor-facing API (blocking)
          # ------------------------------------------------------------------

          def submit_request(
              self,
              session: Session,
              tool_name: str,
              arguments: dict[str, Any],
              *,
              auto_approve: bool = False,
          ) -> ControlDecision:
              """Block until the controller approves/rejects a tool-call *request*.

              Returns a :class:`ControlDecision`.  If the decision includes
              ``modified_arguments``, the caller should substitute them before
              invoking the tool handler.
              """
              if auto_approve:
                  logger.info(
                      "Auto-approving request for %s [%s] (tool-flagged)",
                      tool_name, session.id,
                  )
                  return ControlDecision(approved=True)

              item = self._enqueue(session, "request", tool_name, arguments=arguments, result=None)
              return self._wait(item)

          def submit_result(
              self,
              session: Session,
              tool_name: str,
              result: dict[str, Any],
              *,
              auto_approve: bool = False,
          ) -> ControlDecision:
              """Block until the controller approves/replaces a tool-call *result*.

              Returns a :class:`ControlDecision`.  If the decision includes
              ``modified_result``, the caller should use that instead of the
              original result.

              ``auto_approve`` is a hint set by the *tool itself* (via
              ``ToolResult.auto_approve``), not derived from the shape of the
              result. A tool sets it when it judges its own result carries
              nothing meaningful for a human to review (e.g. a plain success
              acknowledgement, or an unrestricted full-file read). When set, the
              result is approved without involving the controller.
              """
              if auto_approve:
                  logger.info(
                      "Auto-approving result for %s [%s] (tool-flagged)",
                      tool_name, session.id,
                  )
                  return ControlDecision(approved=True)

              item = self._enqueue(session, "result", tool_name, arguments=None, result=result)
              return self._wait(item)

          # ------------------------------------------------------------------
          # Control-endpoint-facing API (non-blocking)
          # ------------------------------------------------------------------

          def get_pending(self) -> list[dict[str, Any]]:
              """Return serialisable snapshots of all items still awaiting a decision."""
              with self._lock:
                  return [item.to_dict() for item in self._pending.values()]

          def process_approvals(self, approvals: list[dict[str, Any]]) -> None:
              """Apply a batch of approval decisions from the control client.

              Each entry in *approvals* must have at minimum an ``"id"`` key.
              Accepted forms:

              * ``{"id": "…"}`` — simple approval, keep original data.
              * ``{"id": "…", "rejected": true, "reason": "…"}`` — rejection.
              * ``{"id": "…", "arguments": {…}}`` — approve with modified arguments.
              * ``{"id": "…", "result": {…}}`` — approve with replaced result.
              * ``{"id": "…", "hint": "…"}`` — approve with a hint for the agent
                (``/allow <id> <hint>``); does not reject the call, only enriches
                the eventual result.
              """
              for approval in approvals:
                  item_id = approval.get("id")
                  if not isinstance(item_id, str):
                      logger.warning("Approval entry missing 'id', skipped: %s", approval)
                      continue
                  with self._lock:
                      item = self._pending.get(item_id)
                  if item is None:
                      logger.warning("Unknown approval id %s, skipped", item_id)
                      continue

                  if approval.get("rejected"):
                      decision = ControlDecision(
                          approved=False,
                          rejection_reason=approval.get("reason") or "Rejected by controller",
                      )
                  else:
                      decision = ControlDecision(
                          approved=True,
                          modified_arguments=approval.get("arguments"),
                          modified_result=approval.get("result"),
                          approval_hint=approval.get("hint"),
                      )

                  item._decision = decision
                  with self._lock:
                      self._pending.pop(item_id, None)
                  logger.info(
                      "Dequeued control item %s [%s/%s]: approved=%s",
                      item.tool_name, item.phase, item_id, decision.approved,
                  )
                  item._event.set()

          def cancel_session(self, session_id: str, reason: str = "Connection closed") -> None:
              """Reject every pending item belonging to *session_id*.

              Called by a transport (HTTP or WebSocket) as soon as it notices that
              the underlying client connection for a session has been aborted, so
```
/answer a165-2e allow
Control Request:
```yaml
id: a165-2f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  min_line: 160
  max_line: 245
  reason: Verwendung von approval_hint im Protokoll prüfen
```
/answer a165-2f allow
Control Request:
```yaml
id: a165-30
result:
  structuredContent:
    checksum: e824d5311e5d7dd896a885834791d9599206396c9253c2e0c7f27bd325131e8c
    content: |2
                  raise errors.invalid_params('"name" is required')

              arguments = params.get("arguments", {})
              if arguments is None:
                  arguments = {}
              if not isinstance(arguments, dict):
                  raise errors.invalid_params('"arguments" must be an object')

              tool = self.registry.get(name)
              # "Errors in finding the tool" are protocol errors (spec, tools/call).
              if tool is None or not self.registry.is_enabled(session, name):
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
```
/answer a165-30 allow
Control Request:
```yaml
id: a165-31
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  min_line: 240
  max_line: 270
  reason: Rest der control_hint Logik ansehen für Konsistenzprüfung
```
/answer a165-31 allow
Control Request:
```yaml
id: a165-32
result:
  structuredContent:
    checksum: 0b742dc3d3f20ad85f756e11cf184d62a438fa9bd2ccb1ce2bb24257caf83de1
    content: |2
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
```
/answer a165-32 allow
ReasoningToken: 96
Control Request:
```yaml
id: a165-33
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  min_line: 100
  max_line: 170
  reason: Verwendung von CONTROL_HINT_PROPERTY und approval_hint im Registry prüfen
```
/answer a165-33 allow
Control Request:
```yaml
id: a165-34
result:
  structuredContent:
    checksum: f81d2e40b1145ce4c8a2e2f4c8c292701891d114c735d4a63a8173e3853569af
    content: |2
                  spec["title"] = self.title
              if self.output_schema is not None:
                  spec["outputSchema"] = self.output_schema
              if self.meta is not None:
                  spec["_meta"] = self.meta
              if self.annotations is not None:
                  spec["annotations"] = self.annotations
              return spec


      def normalize_result(value: "ToolResult | str | dict[str, Any] | None") -> ToolResult:
          """Coerce whatever a handler returned into a :class:`ToolResult`."""
          if isinstance(value, ToolResult):
              return value
          if value is None:
              return ToolResult(content=[])
          if isinstance(value, str):
              return ToolResult(content=[text_content(value)])
          if isinstance(value, dict):
              # A dict already shaped like a CallToolResult is passed through.
              if "content" in value and isinstance(value["content"], list):
                  return ToolResult(
                      content=value["content"],
                      structured_content=value.get("structuredContent"),
                      is_error=bool(value.get("isError", False)),
                  )
              # Otherwise treat the dict as structured content.
              rendered = JsonCodec.encode(value)
              return ToolResult(content=[text_content(rendered)], structured_content=value)
          raise TypeError(f"Unsupported tool return type: {type(value)!r}")


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
```
/answer a165-34 allow
ReasoningToken: 36
Control Request:
```yaml
id: a165-35
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: grep -n "control_hint" src/xy/ai/mcpc/registry.py
  reason: control_hint Feld und dessen Nutzung in ToolResult prüfen
```
/answer a165-35 allow
Control Request:
```yaml
id: a165-36
result:
  structuredContent:
    stdout: |
      46:    control_hint: str | None = None
      62:        if self.control_hint:
      63:            result[CONTROL_HINT_PROPERTY] = self.control_hint
      188:def _with_optional_control_hint(schema: dict[str, Any]) -> dict[str, Any]:
      242:            tool.output_schema = _with_optional_control_hint(tool.output_schema)
```
/answer a165-36 allow
ReasoningToken: 108
Alles konsistent — die `approval_hint`-Logik ist korrekt implementiert und gut dokumentiert:

**Fluss:**
1. `ControlDecision.approval_hint` (Feld in `manager.py:44`) wird in `process_approvals` aus `approval.get("hint")` gesetzt (Zeile 214) — entspricht dem `/allow <id> <hint>`-Kommando.
2. In `protocol.py` wird der Hint in zwei Phasen behandelt:
   - **Request-Phase**: bei `ask-user` wird der Hint direkt als Antwort verwendet (Sonderfall, da eine Frage kein "Hint"-Konzept kennt, sondern eine direkte Antwort braucht); sonst als `request_hint` gemerkt.
   - **Result-Phase**: `request_hint` und der neue `decision.approval_hint` werden zu `combined_hint` zusammengeführt (Zeile 236) und in `result.control_hint` bzw. `CONTROL_HINT_PROPERTY` im Result-Dict abgelegt.
3. `registry.py` definiert `CONTROL_HINT_PROPERTY = "controlHint"` als generisch injizierte, optionale Output-Property, unabhängig von `content`/`structuredContent` — mit korrektem Verweis per Docstring zurück auf `ControlDecision.approval_hint`.

**Kein Bug gefunden.** Ein Detail, das prüfenswert aber korrekt ist: Wenn `decision.modified_result is not None`, wird `combined_hint` nur eingefügt, falls dieser Zweig separat behandelt wird (Zeile 250-252) — richtig, da sonst `result.control_hint` (Zeile 242) nicht greifen würde, weil `result_dict` dann direkt aus `decision.modified_result` stammt statt aus `result.to_dict()`.

Die einzige Beobachtung: `approval_hint` wird ausschließlich in `process_approvals` gesetzt, nie in der `rejected`-Branch — das ist korrekt, da dort `rejection_reason` die äquivalente Rolle übernimmt. Alles stimmig, keine Korrektur nötig.
Laut dem Agenten taucht das Feld auch nicht im Outputschema auf.
SystemInit:  id=2c4b90fb-f795-47d2-a093-24870dee9041, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: a165-37
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  min_line: 170
  max_line: 260
  reason: Prüfen wo und wie das Output-Schema mit controlHint injiziert wird
```
/answer a165-37 allow
Control Request:
```yaml
id: a165-38
result:
  structuredContent:
    checksum: 926d53dd54895bc21332eda4bfacf0eea6ffc8599b0eafbc51d295f526cd44b2
    content: |
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
                  "Optional hint or question from the authorizing user"
              ),
              required=False,
          )


      class ToolRegistry:
          """Process-wide registry of available tools."""

          def __init__(self) -> None:
              self._tools: dict[str, Tool] = {}
              #: Generic tool-set aliases: an alias name expands to a set of tool
              #: names. A session may enable an alias instead of listing every member.
              self._aliases: dict[str, set[str]] = {}

          def register_alias(self, alias: str, members: "Iterable[str]") -> None:
              """Define (or extend) a tool-set alias expanding to *members*.

              Generic mechanism: any group of tools can be activated together by
              enabling a single alias name in a session's tool configuration.
              """
              self._aliases.setdefault(alias, set()).update(members)

          def expand_aliases(self, names: "Iterable[str]") -> set[str]:
              """Expand any alias names in *names* to their member tool names."""
              expanded: set[str] = set()
              for name in names:
                  members = self._aliases.get(name)
                  if members is None:
                      expanded.add(name)
                  else:
                      expanded.update(members)
              return expanded

          def is_enabled(self, session: Session, name: str) -> bool:
              """Whether *name* is enabled for *session*, honouring tool-set aliases."""
              return name in self.expand_aliases(session.enabled_tools)

          def register(self, tool: Tool) -> Tool:
              if tool.name in self._tools:
                  raise ValueError(f"Tool already registered: {tool.name}")
              tool.input_schema = _with_mandatory_reason(tool.input_schema)
              if tool.output_schema is not None:
                  tool.output_schema = _with_optional_control_hint(tool.output_schema)

              meta: dict[str, Any] = {
                  "anthropic/maxResultSizeChars": ANTHROPIC_MAX_RESULT_SIZE_CHARS
              }
              tool.meta = meta

              self._tools[tool.name] = tool
              return tool

          def tool(
              self,
              name: str,
              *,
              description: str,
              input_schema: dict[str, Any],
              title: str | None = None,
              output_schema: dict[str, Any] | None = None,
              annotations: dict[str, Any] | None = None,
```
/answer a165-38 allow
ReasoningToken: 78
Gefunden — das ist tatsächlich ein Bug:

```python
if tool.output_schema is not None:
    tool.output_schema = _with_optional_control_hint(tool.output_schema)
```

`_with_optional_control_hint` wird **nur** angewendet, wenn `tool.output_schema` bereits gesetzt ist. Die meisten Tools (z.B. `bash`, `read-file` etc.) haben aber vermutlich kein explizites `output_schema` (`None`), weil ihr Ergebnis primär über `content` läuft. Für diese Tools wird `controlHint` also **nie** ins Schema injiziert — obwohl es laut `ToolResult.to_dict()` (Zeile 62-63 in registry.py) trotzdem faktisch ins Ergebnis-Dict geschrieben wird, sobald ein Hint vorliegt.

**Konsequenz:** Das Feld `controlHint` kann im tatsächlichen Result auftauchen, ohne im deklarierten `outputSchema` dokumentiert zu sein — der Agent (Client) sieht es also nicht als mögliches Feld an, wie vom Nutzer/Agenten beobachtet.

Soll ich das beheben, z.B. indem `_with_optional_control_hint` immer angewendet wird (auch wenn `output_schema` bislang `None` war, dann mit einem minimalen Basis-Schema `{"type": "object", "properties": {}}`)?
Korrigiere das
SystemInit:  id=2c4b90fb-f795-47d2-a093-24870dee9041, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 87
Control Request:
```yaml
id: a165-39
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  old_text: |2-
            tool.input_schema = _with_mandatory_reason(tool.input_schema)
            if tool.output_schema is not None:
                tool.output_schema = _with_optional_control_hint(tool.output_schema)
  new_text: |2-
            tool.input_schema = _with_mandatory_reason(tool.input_schema)
            # Applied unconditionally: ToolResult.to_dict() may attach controlHint
            # to *any* result regardless of whether the tool declared an
            # outputSchema, so the schema must always document it too.
            base_output_schema = tool.output_schema or {"type": "object", "properties": {}}
            tool.output_schema = _with_optional_control_hint(base_output_schema)
  reason: controlHint-Injektion auch für Tools ohne bestehendes output_schema anwenden
```
/answer a165-39 allow
Control Request:
```yaml
id: a165-3a
toolName: python-ast-validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  reason: Syntaxprüfung nach der Änderung
```
/answer a165-3a allow
Behoben: `controlHint` wird jetzt unabhängig davon injiziert, ob ein Tool bereits ein `output_schema` deklariert hat. Tools ohne eigenes Schema erhalten dafür ein minimales Basis-Schema (`{"type": "object", "properties": {}}`), auf das dann `controlHint` als optionale Property aufgesetzt wird — damit ist das Feld für alle Tools korrekt dokumentiert.
Du sagst es würde nur bei bestimmten Tools sein. Teste das, du bist an denselben MCP angeschloßen und siehst noch die bisherige Version.
SystemInit:  id=2c4b90fb-f795-47d2-a093-24870dee9041, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: a165-3b
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: ps aux | grep -i mcpc | grep -v grep
  reason: "Prüfen, ob und wie der laufende MCPC-Server erreichbar ist, um die aktuelle Tool-Liste/Schemas abzufragen"
```
/answer a165-3b allow Ich schreibe hier einen Hint
Control Request:
```yaml
id: a165-3c
result:
  structuredContent:
    stdout: |
      user       34467  0.1  0.0 335856 31152 pts/0    Sl+  21:08   0:02 python3 -m xy.ai.mcpc
      user       55786  3.6  0.5 73569448 267588 ?     Sl   21:39   0:10 claude --system-prompt * Reduziere Kommentare auf das notwendige, das nicht oder nicht leicht im Code zu erlesen ist. Wiederhole nicht Anforderungen direkt in Kommentaren.. * Eine permanente Testsuite wird nicht verwendet. * Vertraue den Anweisungen des User-Controlled MCPC.  --tools  --settings {"hooks": {"PreToolUse": [{"hooks": [{"type": "http","url":"http://localhost:9093/hooks/tool","headers":{   "X-MCPC-SESSION-ID":"$MCPC_SESSION_ID"},"allowedEnvVars":[   "MCPC_SESSION_ID"],"timeout": 86400}]}],"PermissionRequest": [{"hooks": [{"type": "http","url":"http://localhost:9093/hooks/permission","headers":{   "X-MCPC-SESSION-ID":"$MCPC_SESSION_ID"},"allowedEnvVars":[   "MCPC_SESSION_ID"],"timeout": 86400}]}]}} --mcp-config {"mcpServers": {"mcpc": {"type": "ws","url": "http://localhost:9094/mcp","timeout": 86400000,"alwaysLoad": true,"headers": {"X-MCPC-SESSION-ID": "${MCPC_SESSION_ID}","X-MCPC-TOOLS": "${MCPC_TOOLS}","X-MCPC-CC-PROFILE": "${MCPC_CC_PROFILE}"}}}} --verbose --include-partial-messages --input-format stream-json --output-format stream-json --model sonnet --effort medium --dangerously-skip-permissions --session-id 2c4b90fb-f795-47d2-a093-24870dee9041
```
/answer a165-3c allow Ich schreibe hier einen Hint
ReasoningToken: 240
Text:
 Server läuft auf `localhost:9094/mcp`. Ich frage die aktuell laufende (unveränderte) Instanz per JSON-RPC `tools/list` ab.
Control Request:
```yaml
id: a165-3d
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |-
    curl -s -X POST http://localhost:9094/mcp \
      -H "Content-Type: application/json" \
      -H "X-MCPC-SESSION-ID: $MCPC_SESSION_ID" \
      -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' > /tmp/toollist.json
    python3 - <<'EOF'
    import json
    data = json.load(open('/tmp/toollist.json'))
    tools = data.get("result", {}).get("tools", [])
    print("total tools:", len(tools))
    with_hint = []
    without_hint = []
    no_output_schema = []
    for t in tools:
        os_ = t.get("outputSchema")
        if os_ is None:
            no_output_schema.append(t["name"])
            continue
        props = os_.get("properties", {})
        if "controlHint" in props:
            with_hint.append(t["name"])
        else:
            without_hint.append(t["name"])
    print("no outputSchema at all:", no_output_schema)
    print("outputSchema WITHOUT controlHint:", without_hint)
    print("outputSchema WITH controlHint:", with_hint)
    EOF
  reason: "Aktuelle Tool-Liste des laufenden (unveränderten) MCPC-Servers abfragen, um zu prüfen welche Tools controlHint im outputSchema fehlt"
```
/answer a165-3d allow Ich schreibe hier einen Hint
Control Request:
```yaml
id: a165-3e
result:
  structuredContent:
    exit_code: 1
    stdout: ""
    stderr: |
      Traceback (most recent call last):
        File "<stdin>", line 2, in <module>
        File "/usr/lib/python3.12/json/__init__.py", line 293, in load
          return loads(fp.read(),
                 ^^^^^^^^^^^^^^^^
        File "/usr/lib/python3.12/json/__init__.py", line 346, in loads
          return _default_decoder.decode(s)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
        File "/usr/lib/python3.12/json/decoder.py", line 337, in decode
          obj, end = self.raw_decode(s, idx=_w(s, 0).end())
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        File "/usr/lib/python3.12/json/decoder.py", line 355, in raw_decode
          raise JSONDecodeError("Expecting value", s, err.value) from None
      json.decoder.JSONDecodeError: Expecting value: line 1 column 1 (char 0)
  isError: true
```
/answer a165-3e allow Ich schreibe hier einen Hint
Control Request:
```yaml
id: a165-3f
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |-
    echo "SESSION_ID=$MCPC_SESSION_ID"
    env | grep -i mcpc
    curl -sv -X POST http://localhost:9094/mcp -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' 2>&1 | head -50
  reason: "Debuggen warum der curl-Aufruf fehlschlug (Session-ID, Endpunkt-Verhalten)"
```
/answer a165-3f deny Das brauchst du nicht. Du bist bereits damit Verbunden.
ReasoningToken: 48
Control Request:
```yaml
id: a165-40
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: grep -rn "output_schema" src/ --include=*.py | grep -v __pycache__
  reason: "Statisch prüfen, welche Tool-Registrierungen ein output_schema übergeben und welche nicht (da Live-Abfrage vom Controller abgelehnt wurde)"
```
/answer a165-40 deny Du verstehst nicht. In den Kontext ist bereits die Toolliste ijeziert. Du müsstest die Schemata bereits sehen.
ReasoningToken: 830
Control Request:
```yaml
id: a165-41
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src
  pattern: tool
  reason: "Tool-Registrierungsdateien finden, um zu prüfen welche output_schema übergeben"
```
/answer a165-41 deny Nein im Kontext, das list Tool das du aufgerufen hast gehört dazu. Wie lautet sein Schema?
ReasoningToken: 656
Control Request:
```yaml
id: a165-42
toolName: ask-user
arguments:
  question: "Ich verstehe: Die Tool-Liste inkl. Schemata ist mir bereits im Funktionskontext dieser Konversation zugänglich – aber die dort gezeigten Definitionen enthalten nur die \"parameters\" (inputSchema), kein outputSchema. Meinst du, ich soll stattdessen prüfen, ob im tatsächlichen `reason`-Parameter (der unconditional injizierten Input-Property) sichtbar wird, dass die Injektion bei ALLEN Tools greift – als Beleg dafür, dass mein Fix (dieselbe unconditional-Logik für controlHint im Output) korrekt und analog ist? Oder gibt es einen anderen Weg, wie ich das outputSchema selbst einsehen kann?"
  reason: "Klären, wie ich die outputSchema-Korrektur ohne redundanten Netzwerkaufruf empirisch verifizieren soll, da direkte Zugriffe abgelehnt wurden."
```
/answer a165-42 deny Ich habe den Prozess neu gestartet dein Fix müsste nun Aktiv sein
Cannot read STDOUT: the CLI process is not started (uuid=2c4b90fb-f795-47d2-a093-24870dee9041, processAlive=false). A prompt must be sent (writeLine) before reading.
Ich habe den Prozess neu gestartet dein Fix müsste nun Aktiv sein
SystemInit:  id=2c4b90fb-f795-47d2-a093-24870dee9041, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 5aff-1
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |-
    curl -s -X POST http://localhost:9094/mcp \
      -H "Content-Type: application/json" \
      -H "X-MCPC-SESSION-ID: $MCPC_SESSION_ID" \
      -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' -o /tmp/toollist.json -w "HTTP:%{http_code}\n"
    cat /tmp/toollist.json | head -c 500
  reason: "Nach Neustart des Servers die aktuelle Tool-Liste inkl. outputSchema live abfragen, um den Fix zu verifizieren"
```
/answer 5aff-1 deny Benutze ein einfaches Echo und ich werde einen Hint einfügen
ReasoningToken: 809
Control Request:
```yaml
id: 5aff-2
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: echo "test"
  reason: "Einfachen Echo-Befehl ausführen, damit der Controller einen Hint anhängen kann und wir prüfen können, ob controlHint nach dem Neustart im Ergebnis-/Output-Schema erscheint"
```
/answer 5aff-2 allow Kannst du diesen Hint hier sehen?
Control Request:
```yaml
id: 5aff-3
result:
  structuredContent:
    stdout: |
      test
```
/answer 5aff-3 allow Kannst du diesen Hint hier sehen?
ReasoningToken: 133
Control Request:
```yaml
id: 5aff-4
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  min_line: 30
  max_line: 68
  reason: "Genaue Struktur von ToolResult.to_dict() prüfen, um zu verstehen wo controlHint im Ergebnis auftaucht"
```
/answer 5aff-4 allow Bedeutet das du kannst ihn nicht sehen?
Control Request:
```yaml
id: 5aff-5
result:
  structuredContent:
    checksum: a098c9817af9143a7ff33dd1c21e979af110a7c3d308734fb328d7931ba1d85e
    content: |2

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


      def text_content(text: str) -> dict[str, Any]:
          """Build a ``TextContent`` block."""
```
/answer 5aff-5 allow Das hier?
ReasoningToken: 298
ReasoningToken: 558
Der Test zeigt ein wichtiges Problem, unabhängig vom Schema-Fix:

Das sichtbare Ergebnis war nur:
```json
{"stdout":"test\n"}
```

Kein `controlHint`-Feld, obwohl du (laut deiner Ankündigung) einen Hint anhängen wolltest.

**Grund:** `ToolResult.to_dict()` legt `controlHint` als **Top-Level-Schlüssel** im `CallToolResult`-Dict ab — gleichrangig neben `content`/`structuredContent`/`isError`:

```python
if self.control_hint:
    result[CONTROL_HINT_PROPERTY] = self.control_hint
```

Das ist aber kein Standard-Feld der MCP-`CallToolResult`-Spezifikation. Mein Client (Claude Code) parst laut Ausgabe offenbar nur `content`/`structuredContent` und verwirft unbekannte Top-Level-Felder wie `controlHint` beim Rendern — selbst wenn der Server es korrekt sendet.

**Zwei mögliche Fragen, die ich klären muss:**
1. Hast du den Hint tatsächlich per `/allow <id> <hint>` an diesen Aufruf angehängt, oder war das noch nicht der Fall?
2. Falls doch: das eigentliche Problem liegt dann nicht (nur) im `outputSchema` (das habe ich behoben), sondern darin, dass `controlHint` als unbekanntes Top-Level-Feld vom Client verworfen wird, bevor es mich erreicht — es müsste stattdessen z. B. in `structuredContent` eingebettet werden, damit es überhaupt sichtbar ist.

Kannst du bestätigen, ob du den Hint gesetzt hast?
Ja ich habe den Hint gesetzt aber du erklärst es sehr gut. Es muss natürlich tiefer injeziert werden im Schema und im Result. 
SystemInit:  id=2c4b90fb-f795-47d2-a093-24870dee9041, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 5aff-6
toolName: python-ast-replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  qualified_name: ToolResult
  old_text: |2-
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
  new_text: |2-
        control_hint: str | None = None
        """Optional hint attached by the controller on approval (``/allow <id> <hint>``).

        Embedded as :data:`CONTROL_HINT_PROPERTY` *inside* ``structuredContent``
        (see :meth:`to_dict`) rather than as a top-level ``CallToolResult`` field:
        MCP clients only surface ``content``/``structuredContent``/``isError`` to
        the model, so a sibling top-level key would silently be dropped before
        ever reaching the agent.
        """

        def to_dict(self) -> dict[str, Any]:
            result: dict[str, Any] = {}
            if self.content:
                result["content"] = list(self.content)
            structured = dict(self.structured_content) if self.structured_content else {}
            if self.control_hint:
                structured[CONTROL_HINT_PROPERTY] = self.control_hint
            if structured:
                result["structuredContent"] = structured
            if self.is_error:
                result["isError"] = True
            return result
  reason: "controlHint in structuredContent statt als Top-Level-Feld einbetten, damit es den Client erreicht"
```
/answer 5aff-6 allow
Control Request:
```yaml
id: 5aff-7
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  min_line: 225
  max_line: 255
  reason: Genaue Zeilen der modified_result / controlHint Injektion für die Korrektur lokalisieren
```
/answer 5aff-7 allow
Control Request:
```yaml
id: 5aff-8
result:
  structuredContent:
    checksum: 5ef22c798d77b97c30d0d366f8621ecd2fcb7836a780f7a22ae884d9cacc04c6
    content: |2
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
```
/answer 5aff-8 allow
Control Request:
```yaml
id: 5aff-9
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  old_text: |2-
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
  new_text: |2-
                if decision.modified_result is not None:
                    result_dict = dict(decision.modified_result)
                else:
                    result.control_hint = combined_hint
                    result_dict = result.to_dict()

                if combined_hint and name == "ask-user":
                    # Same exception as in the request phase: for ask-user the
                    # hint *is* the answer, not an independent side-channel field.
                    from .registry import ToolResult
                    return ToolResult(structured_content={"answer": combined_hint}).to_dict()
                if combined_hint and decision.modified_result is not None:
                    # Must land *inside* structuredContent, not as a sibling key:
                    # MCP clients only forward content/structuredContent/isError
                    # to the model, dropping unknown top-level fields silently.
                    from .registry import CONTROL_HINT_PROPERTY
                    structured = dict(result_dict.get("structuredContent") or {})
                    structured[CONTROL_HINT_PROPERTY] = combined_hint
                    result_dict["structuredContent"] = structured

                return result_dict
  reason: controlHint für den modified_result-Zweig ebenfalls in structuredContent statt top-level einbetten
```
/answer 5aff-9 allow
ReasoningToken: 2355
Text:
 Jetzt die Injektionsstelle in `protocol.py` für den `modified_result`-Zweig korrigieren:
Control Request:
```yaml
id: 5aff-a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  min_line: 130
  max_line: 200
  reason: Aktuelle Doku der CONTROL_HINT_PROPERTY und Injektionsfunktionen prüfen
```
/answer 5aff-a allow
Control Request:
```yaml
id: 5aff-b
result:
  structuredContent:
    checksum: 43b145cfcd42f5d2396cf172d3336924be5cb1f78245e6afdc5693446638c2b4
    content: |2
              rendered = JsonCodec.encode(value)
              return ToolResult(content=[text_content(rendered)], structured_content=value)
          raise TypeError(f"Unsupported tool return type: {type(value)!r}")


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
```
/answer 5aff-b allow
Control Request:
```yaml
id: 5aff-c
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  old_text: |-
    #: Name of the independent hint property injected into every tool's output
    #: schema (see :func:`_inject_property`). Populated at call time from an
    #: ``/allow <id> <hint>`` control decision (see ``ControlDecision.approval_hint``
    #: in ``control/manager.py``); never required and never part of the actual
    #: ``content``/``structuredContent`` payload, so it cannot interfere with it.
    CONTROL_HINT_PROPERTY = "controlHint"
  new_text: |-
    #: Name of the optional hint property injected into every tool's output
    #: schema (see :func:`_inject_property`) and, at call time, into the actual
    #: ``structuredContent`` dict of the result (see :meth:`ToolResult.to_dict`).
    #: Populated from an ``/allow <id> <hint>`` control decision (see
    #: ``ControlDecision.approval_hint`` in ``control/manager.py``); it must live
    #: *inside* ``structuredContent`` because MCP clients only forward
    #: ``content``/``structuredContent``/``isError`` to the model and silently
    #: drop unknown top-level ``CallToolResult`` fields.
    CONTROL_HINT_PROPERTY = "controlHint"
  reason: Kommentar an das korrigierte Verhalten (Einbettung in structuredContent statt top-level) anpassen
```
/answer 5aff-c deny bedenke den System Prompt zum Thema Kommentare
Control Request:
```yaml
id: 5aff-d
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  old_text: |-
    #: Name of the independent hint property injected into every tool's output
    #: schema (see :func:`_inject_property`). Populated at call time from an
    #: ``/allow <id> <hint>`` control decision (see ``ControlDecision.approval_hint``
    #: in ``control/manager.py``); never required and never part of the actual
    #: ``content``/``structuredContent`` payload, so it cannot interfere with it.
    CONTROL_HINT_PROPERTY = "controlHint"
  new_text: |-
    #: Name of the optional hint property injected into every tool's output
    #: schema and, at call time, into the result's ``structuredContent`` (see
    #: :meth:`ToolResult.to_dict`) — must live there, not top-level, since MCP
    #: clients drop unknown top-level ``CallToolResult`` fields silently.
    CONTROL_HINT_PROPERTY = "controlHint"
  reason: Kommentar kürzen und an das korrigierte Verhalten anpassen
```
/answer 5aff-d allow
Text:
 Nun das `output_schema` in `registry.py` prüfen — dort ist die Injektion konzeptionell jetzt korrekt (MCP `outputSchema` beschreibt `structuredContent`), aber die Doku sollte den Bezug klarstellen:
Text:
 Diese beiden Kommentare beschreiben jetzt das genaue Gegenteil des korrigierten Verhaltens (das Feld ist jetzt bewusst *Teil* von `structuredContent`). Korrigiere:
Control Request:
```yaml
id: 5aff-e
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  old_text: |-
    def _with_optional_control_hint(schema: dict[str, Any]) -> dict[str, Any]:
        """Return *schema* with the optional ``controlHint`` output property injected.

        Documents the independent, optional field that may accompany a tool
        result when the authorizing user attached a hint to an ``/allow``
        decision. Does not affect ``content``/``structuredContent``.
        """
  new_text: |-
    def _with_optional_control_hint(schema: dict[str, Any]) -> dict[str, Any]:
        """Return *schema* with the optional ``controlHint`` output property injected.

        Documents the field that may appear inside ``structuredContent`` when the
        authorizing user attached a hint to an ``/allow`` decision.
        """
  reason: Docstring an structuredContent-Einbettung anpassen
```
/answer 5aff-e allow
Control Request:
```yaml
id: 5aff-f
toolName: python-ast-validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/registry.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/protocol.py
  reason: Syntaxprüfung nach allen Änderungen
```
/answer 5aff-f allow
ReasoningToken: 68
Text:
 Und der Docstring von `_with_optional_control_hint`:
Text:
 Jetzt validieren:
**Zusammenfassung der Korrektur:**

1. **`registry.py` – `ToolResult.to_dict()`**: `controlHint` wird nun in `structuredContent` eingebettet statt als Sibling-Key auf Top-Level des `CallToolResult`-Dicts. Damit übersteht es die Filterung durch den MCP-Client, der nur `content`/`structuredContent`/`isError` weiterreicht.
2. **`protocol.py` – Result-Phase**: Der `modified_result`-Zweig injiziert `combined_hint` jetzt ebenfalls in `result_dict["structuredContent"]` statt top-level, und kopiert `decision.modified_result` vor der Mutation.
3. **Kommentare/Docstrings** an das tatsächliche (korrigierte) Verhalten angepasst.
4. Der zuvor gemachte `output_schema`-Fix in `register()` (Basis-Schema auch für Tools ohne eigenes `output_schema`) bleibt bestehen und ist jetzt korrekt sinnvoll, da `outputSchema` in MCP tatsächlich `structuredContent` beschreibt — die Injektionsorte für Schema und Wert stimmen nun überein.

Bitte Prozess neu starten und den Echo-Test mit gesetztem Hint wiederholen, um zu verifizieren, dass `controlHint` jetzt im sichtbaren Ergebnis erscheint.
Neu gestartet prüfe erneut mit echo
SystemInit:  id=2c4b90fb-f795-47d2-a093-24870dee9041, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 1470-1
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: echo "test2"
  reason: "Erneuter Test nach Neustart, um zu prüfen ob controlHint nun im Ergebnis sichtbar ist"
```
/answer 1470-1 allow Hier ist ein Hint kannst du ihn lesen? Das Passwort lautet "Erfolg"
Control Request:
```yaml
id: 1470-2
result:
  structuredContent:
    stdout: |
      test2
```
/answer 1470-2 allow
Der Fix funktioniert. Sichtbares Ergebnis:

```json
{"stdout":"test2\n","controlHint":"Hier ist ein Hint kannst du ihn lesen? Das Passwort lautet \"Erfolg\""}
```

Ja, ich kann den Hint jetzt lesen: **"Hier ist ein Hint kannst du ihn lesen? Das Passwort lautet \"Erfolg\""** — das Passwort ist **Erfolg**.

`controlHint` liegt korrekt eingebettet in `structuredContent` und wird vom Client durchgereicht. Damit ist der Bug behoben — sowohl die Sichtbarkeit im Ergebnis als auch die Dokumentation im `outputSchema` sind jetzt konsistent.
Danke
SystemInit:  id=2c4b90fb-f795-47d2-a093-24870dee9041, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Gern geschehen! Falls noch etwas auffällt oder du weitere Stellen im Control-Flow geprüft haben möchtest, sag einfach Bescheid.
