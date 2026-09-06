Die ID's der Kontrollnachrichten in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py` ist zu lang. Ändere das gegen ein bei Instanziierung zufällig erstelltem Prefix und einem Counter im Hexformat. 4-6 Byte sollten reichen.
SystemInit:  id=542e522f-69f6-49a1-8e49-64c1e6edba72, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 704a0c5e-81b8-4a96-8756-ad920213a3d9
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control
  script: grep -n "id" manager.py | grep -i -E "uuid|id ="
  reason: Find current ID generation logic in manager.py
```
/answer 704a0c5e-81b8-4a96-8756-ad920213a3d9 allow
Control Request:
```yaml
id: 845a1b5c-bb6e-44dd-bcf2-482bb9255549
result:
  structuredContent:
    stdout: |
      19:import uuid
      180:            item_id = approval.get("id")
      221:            items = [item for item in self._pending.values() if item.session_id == session_id]
      248:        item_id = str(uuid.uuid4())
```
/answer 845a1b5c-bb6e-44dd-bcf2-482bb9255549 allow
ReasoningToken: 19
Control Request:
```yaml
id: 0d2d1486-0075-4def-a333-15fa9952bd24
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  reason: View context around class definition and ID usage for prefix+counter change
  max_line: 280
```
/answer 0d2d1486-0075-4def-a333-15fa9952bd24 allow
Control Request:
```yaml
id: 0aee62fb-93c0-47c0-9f1e-34337e22245f
result:
  structuredContent:
    checksum: 69a39245cd586d8fa350fb12c500ce3f73c245a8f366766c587f2bb1a6d31b80
    content: |
      """Human-in-the-loop tool control manager.

      Implements the Manager Pattern for intercepting tool calls at two points:

      1. *Before* execution (``phase="request"``): the interceptor may approve,
         modify arguments, or reject the call entirely.
      2. *After* execution (``phase="result"``): the interceptor may approve,
         replace the tool output, or inject instructions as the result.

      The control endpoint at ``/control/tool`` lets an external client poll for
      pending items and post approval decisions.  The intercepting threads block on
      per-item ``threading.Event`` objects until a decision arrives.
      """

      from __future__ import annotations

      import logging
      import threading
      import uuid
      from dataclasses import dataclass, field
      from typing import Any
      from xy.ai.mcpc.session import Session

      logger = logging.getLogger("xy.ai.mcpc.control")

      # How long (seconds) an intercepted call waits before timing out and auto-approving.
      _DEFAULT_TIMEOUT = 24 * 60 * 60.0  # 24 h — matches agent MCP timeout


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
              the interceptor thread blocked in :meth:`submit_request` /
              :meth:`submit_result` doesn't hang around until the (up to 24h)
              timeout elapses for a decision that can never be delivered anymore.
              """
              with self._lock:
                  items = [item for item in self._pending.values() if item.session_id == session_id]

              for item in items:
                  with self._lock:
                      still_pending = self._pending.pop(item.id, None) is not None
                  if not still_pending:
                      continue  # already resolved by a concurrent approval/timeout

                  item._decision = ControlDecision(approved=False, rejection_reason=reason)
                  logger.info(
                      "Cancelling control item %s [%s/%s] for session %s: %s",
                      item.tool_name, item.phase, item.id, session_id, reason,
                  )
                  item._event.set()

          # ------------------------------------------------------------------
          # Internal helpers
          # ------------------------------------------------------------------

          def _enqueue(
              self,
              session: Session,
              phase: str,
              tool_name: str,
              arguments: dict[str, Any] | None,
              result: dict[str, Any] | None,
          ) -> _PendingItem:
              item_id = str(uuid.uuid4())
              item = _PendingItem(
                  id=item_id,
                  phase=phase,
                  tool_name=tool_name,
                  session_id=session.id,
                  arguments=arguments,
                  result=result,
              )
              with self._lock:
                  self._pending[item_id] = item
              logger.info("Enqueued control item %s [%s/%s/%s]", tool_name, phase, session.id, item_id)
              return item

          def _wait(self, item: _PendingItem) -> ControlDecision:
              signalled = item._event.wait(timeout=self._timeout)
              with self._lock:
                  self._pending.pop(item.id, None)

              if not signalled or item._decision is None:
                  # Timeout — auto-approve to avoid hanging the agent forever.
                  logger.warning(
                      "Control item %s [%s/%s] timed out, auto-approving",
                      item.tool_name, item.phase, item.id,
                  )
                  return ControlDecision(approved=True)

              return item._decision
```
/answer 0aee62fb-93c0-47c0-9f1e-34337e22245f allow
Text:
 Jetzt passe ich `__init__` und `_enqueue` an.
Control Request:
```yaml
id: e5905b24-bf21-4447-ad37-a7a45644f517
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  old_text: |2-
        def __init__(self, timeout: float = _DEFAULT_TIMEOUT) -> None:
            self._timeout = timeout
            self._pending: dict[str, _PendingItem] = {}
            self._lock = threading.Lock()
  new_text: |2-
        def __init__(self, timeout: float = _DEFAULT_TIMEOUT) -> None:
            self._timeout = timeout
            self._pending: dict[str, _PendingItem] = {}
            self._lock = threading.Lock()
            self._id_prefix = uuid.uuid4().hex[:8]
            self._id_counter = 0
  reason: Add random prefix and counter fields for short control item IDs
```
/answer e5905b24-bf21-4447-ad37-a7a45644f517 allow
Control Request:
```yaml
id: df06c865-6335-417f-a69c-ce3aef75a513
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  old_text: |2-
            item_id = str(uuid.uuid4())
            item = _PendingItem(
  new_text: |2-
            with self._lock:
                self._id_counter += 1
                item_id = f"{self._id_prefix}-{self._id_counter:x}"
            item = _PendingItem(
  reason: Replace UUID-based item ID with short instance prefix + hex counter
```
/answer df06c865-6335-417f-a69c-ce3aef75a513 allow
ReasoningToken: 67
Fertig. Die Kontroll-IDs werden jetzt beim Instanziieren von `ToolControlManager` mit einem zufälligen 4-Byte-Hex-Prefix (`uuid.uuid4().hex[:8]`) initialisiert und pro `_enqueue`-Aufruf um einen Hex-Counter ergänzt (z. B. `a1b2c3d4-1`, `a1b2c3d4-2`, …), statt jedes Mal eine vollständige UUID zu erzeugen.
Ist die ganze ID in HEX ?
SystemInit:  id=542e522f-69f6-49a1-8e49-64c1e6edba72, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 75
Ja – beide Teile sind reines Hex:

- **Prefix**: `uuid.uuid4().hex[:8]` → 8 Hex-Zeichen (4 Byte Zufall)
- **Counter**: `f"{self._id_counter:x}"` → fortlaufende Zahl in Hex-Darstellung

Verbunden werden sie nur durch einen Bindestrich `-` als Trenner (kein Hex-Zeichen), z. B. `a1b2c3d4-1`, `a1b2c3d4-2a`, … Falls du auch den Trenner weglassen möchtest, kann ich das ändern (dann z. B. `a1b2c3d4001`).
