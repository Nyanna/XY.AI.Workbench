Das Nummernschema der Control Requests in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py` soll sich ändern.

Aktuell enthält es eine aufsteigen Nummer hinter der Session Hash: "id: 3186-5".
Dies soll um eine weitere Stelle getrennt mit Dash erweiter werden "id: 3186-5-0".
Die neue zweite Stelle Soll eine Rückreferenz auf den Tool-Calls sein. Ein Tool Call trägt hier immer eine Null.

Beispiel:
- Tool-Call: "id: 3186-27-0"
- Tool-Response: "id: 3186-28-27"

Dies soll dazu dienen Responses den Tool-Call zuordnen zu können, ohne ein neues Feld einzuführen.
SystemInit:  id=b74076fe-c51a-4568-a6f8-7c6934609630, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Control Request:
```yaml
id: 3186-1b
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  reason: "Struktur von manager.py verstehen, um das Nummernschema der Control Requests zu finden"
```
/answer 3186-1b allow
Control Request:
```yaml
id: 3186-1c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
      nodes:
      - id: NXKGJd|ABmM9X
        signature: '''Human-in-the-loop tool control manager.\n\nImplements the Manager Pattern for …'
      - id: XNKbmy|OT0fAY
        signature: import logging
      - id: mGVkgb|UCEZKE
        signature: logger = logging.getLogger('xy.ai.mcpc.control')
      - id: ControlDecision
        signature: "@dataclass(slots=True) class ControlDecision:"
        docstring: "The outcome of a human review, produced by :meth:`ToolControlManager.process_ap…"
      - id: reason_last
        signature: "def _reason_last(arguments: dict[str, Any]) -> dict[str, Any]:"
        docstring: "Return *arguments* with ``reason``, if present, moved to the end so it is alway…"
      - id: PendingItem
        signature: "@dataclass class _PendingItem:"
        children:
        - id: PendingItem.QOjskF|HDKZo4
          signature: "id: str"
        - id: PendingItem.to_dict
          signature: "def to_dict(self) -> dict[str, Any]:"
      - id: ToolControlManager
        signature: "class ToolControlManager:"
        docstring: "Thread-safe manager for human-in-the-loop tool interception. Usage:: manager = …"
        children:
        - id: ToolControlManager.Jy5BQH|BflXLc
          signature: '''Thread-safe manager for human-in-the-loop tool interception.\n\n    Usage::\n\…'
        - id: ToolControlManager.init
          signature: "def __init__(self, timeout: float=_DEFAULT_TIMEOUT) -> None:"
        - id: ToolControlManager.submit_request
          signature: "def submit_request(self, session: Session, tool_name: str, arguments: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:"
          docstring: Block until the controller approves/rejects a tool-call *request*. Returns a :c…
        - id: ToolControlManager.submit_result
          signature: "def submit_result(self, session: Session, tool_name: str, result: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:"
          docstring: Block until the controller approves/replaces a tool-call *result*. Returns a :c…
        - id: ToolControlManager.get_pending
          signature: "def get_pending(self, session_id: str | None=None) -> list[dict[str, Any]]:"
          docstring: Return serialisable snapshots of items still awaiting a decision. When *session…
        - id: ToolControlManager.process_approvals
          signature: "def process_approvals(self, approvals: list[dict[str, Any]]) -> None:"
          docstring: Apply a batch of approval decisions from the control client. Each entry in *app…
        - id: ToolControlManager.cancel_session
          signature: "def cancel_session(self, session_id: str, reason: str='Connection closed') -> None:"
          docstring: Reject every pending item belonging to *session_id*. Called by a transport (HTT…
        - id: ToolControlManager.enqueue
          signature: "def _enqueue(self, session: Session, phase: str, tool_name: str, arguments: dict[str, Any] | None, result: dict[str, Any] | None) -> _PendingItem:"
        - id: ToolControlManager.wait
          signature: "def _wait(self, item: _PendingItem) -> ControlDecision:"
```
/answer 3186-1c allow
Control Request:
```yaml
id: 3186-1d
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: request_id|_counter|next_id|session_hash|\bid\b.*-
    include:
    - '*.py'
    exclude:
    - node_modules/**
    limit: 50
  reason: "Stellen finden, an denen die IDs der Control Requests erzeugt und geparst werden"
```
/answer 3186-1d allow
Control Request:
```yaml
id: 3186-1e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/control/manager.py
        matches:
        - self._id_counter = 0
        - self._id_counter += 1
        - "item_id = f'{self._id_prefix}-{self._id_counter:x}'"
      - path: xy/ai/mcpc/server/http_transport.py
        matches:
        - "The session id is taken from the configured ``X-MCPC-SESSION-ID`` header, which"
        - "def _send_jsonrpc_error(self, status: HTTPStatus, request_id, error: errors.JsonRpcError, session_id: str | None) -> None:"
        - "response = jsonrpc.error_response(request_id, error)"
      - path: xy/ai/mcpc/server/jsonrpc.py
        matches:
        - request_id = message.get('id')
        - "if not is_notification and (not isinstance(request_id, (str, int, float))) and (request_id is not None):"
        - "return JsonRpcRequest(method=method, params=params, id=request_id, is_notification=is_notification)"
        - "def success_response(request_id: RequestId, result: Any) -> dict[str, Any]:"
        - "return {'jsonrpc': JSONRPC_VERSION, 'id': request_id, 'result': result}"
        - "def error_response(request_id: RequestId, error: JsonRpcError | dict[str, Any]) -> dict[str, Any]:"
        - '``request_id`` is ``None`` for protocol-level errors (e.g. a parse error)'
        - "return {'jsonrpc': JSONRPC_VERSION, 'id': request_id, 'error': obj}"
      - path: xy/ai/mcpc/server/ws_transport.py
        matches:
        - '* The session id is taken from the configured ``X-MCPC-SESSION-ID`` header'
      - path: xy/ai/mcpc/embedded/agent.py
        matches:
        - "def id(self) -> str:"
      - path: xy/ai/mcpc/cli/session.py
        matches:
        - passing ``--session-id`` on first start and ``--resume`` on restart.
      - path: xy/ai/mcpc/tools/mcp/client.py
        matches:
        - "'id': self._next_id(),"
        - "message = self._send({'jsonrpc': '2.0', 'id': self._next_id(), 'method': 'tools/call',"
        - "def _next_id(self) -> int:"
      - path: xy/ai/mcpc/tools/mcp/exa/core.py
        matches:
        - "Drops ``_DROPPED_FIELDS``, backfills a missing ``id`` with a random 6-char"
        - "id, and renames ``highlights`` to ``excerpt`` - synthesizing the first and"
      - path: xy/ai/mcpc/tools/ast/base.py
        matches:
        - "node_id: The node's unique ``id`` — its fully-qualified path from the"
        - "``id`` is the node's unique, primarily name-based path used by every tool to"
        - "\"\"\"Return a unique-within-siblings id segment, name-based when feasible."
        - "def matches(loc: Located, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> bool:"
      - path: xy/ai/mcpc/tools/ast/find.py
        matches:
        - "def ast_find(paths: list[str], *, id: str | None=None, name: str | None=None, node_type: str | None=None, lineno: int | None=None, end_lineno: int | None=None, parent_type: str | None=None, text: str | None=None, regexp: str | None=None, with_lines: bool=True) -> FindNodesResult:"
        - "id: Engine-independent unique node id (primarily name-based path)."
      - path: xy/ai/mcpc/tools/ast/script.py
        matches:
        - "def find(self, *, id: str | None=None, node_type: str | None=None, name: str | None=None, parent_type: str | None=None) -> list[core.Located]:"
        - "'description': \"Python source executed with 'tree' and 'ast' as only globals; assign 'result' to return a value; no import statements; builtins limite…ne header / short doc, only set for definition-like nodes when code is omitted) have no direct Located field; derive via node_code(loc) if needed.\"}},"
      - path: xy/ai/mcpc/tools/ast/generic/_css.py
        matches:
        - Doubles as both ``name`` (id/outline label) and the base for ``signature`` -
      - path: xy/ai/mcpc/tools/ast/common.py
        matches:
        - "def select_by_path(tree, *, id: str | None=None) -> core.Located:"
        - "def select_by_text(tree, texts: list[str], *, id: str | None=None) -> core.Located:"
      - path: xy/ai/mcpc/tools/ast/read.py
        matches:
        - "ReadBatchResult: One result per file (with its own id-level errors), and"
      - path: xy/ai/mcpc/tools/ast/python/_comments.py
        matches:
        - "next_idx = _next_code_line_index(lines, lineno)"
        - "if next_idx is not None and _is_continuation_header(lines[next_idx]):"
        - header_lineno = next_idx + 1
      - path: xy/ai/mcpc/tools/agent/__init__.py
        matches:
        - '"# sub-agent''s toolset) and the CLI session (--session-id).  The sub-agent''s"'
        - '''# CLI connects back with this id and never sends X-MCPC-TOOLS itself.'''
      - path: xy/ai/mcpc/tools/tool_call/__init__.py
        matches:
        - "counter = namespace.get('_tool_call_spill_counter', 0) + 1"
        - "namespace['_tool_call_spill_counter'] = counter"
      - path: xy/ai/mcpc/tools/openalex/work.py
        matches:
        - "def _openalex_work_raw(id: str, fields: str | None=None) -> dict[str, Any]:"
        - "def openalex_work(id: str, fields: str | None=None) -> WorkResult:"
      error: 1 match line(s) exceeded 300 characters and were shortened in the middle with an ellipsis (150 leading and trailing characters kept).
```
/answer 3186-1e allow
ReasoningToken: 51
Control Request:
```yaml
id: 3186-1f
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    ids:
    - PendingItem
    - ToolControlManager.init
    - ToolControlManager.submit_request
    - ToolControlManager.submit_result
    - ToolControlManager.enqueue
    - ToolControlManager.process_approvals
    - NXKGJd|ABmM9X
  reason: Code der ID-Erzeugung und der Aufrufer lesen
```
/answer 3186-1f allow
Control Request:
```yaml
id: 3186-20
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
      nodes:
      - id: PendingItem
        type: ClassDef
        signature: "@dataclass class _PendingItem:"
        children:
        - id: PendingItem.QOjskF|HDKZo4
          type: statements
          code: |-
            id: str
            '# "request" | "result"'
            phase: str
            tool_name: str
            session_id: str
            '# populated in request phase'
            arguments: dict[str, Any] | None
            '# populated in result phase'
            result: dict[str, Any] | None
            _event: threading.Event = field(default_factory=threading.Event, repr=False)
            _decision: ControlDecision | None = field(default=None, repr=False)
        - id: PendingItem.to_dict
          type: FunctionDef
          code: |-
            def to_dict(self) -> dict[str, Any]:
                item: dict[str, Any] = {'id': self.id}
                if self.arguments is not None:
                    item['toolName'] = self.tool_name
                    item['arguments'] = _reason_last(self.arguments)
                if self.result is not None:
                    item['result'] = self.result
                return item
      - id: ToolControlManager.init
        type: FunctionDef
        code: |-
          def __init__(self, timeout: float=_DEFAULT_TIMEOUT) -> None:
              self._timeout = timeout
              self._pending: dict[str, _PendingItem] = {}
              self._lock = threading.Lock()
              self._id_prefix = uuid.uuid4().hex[:4]
              self._id_counter = 0
      - id: ToolControlManager.submit_request
        type: FunctionDef
        code: |-
          def submit_request(self, session: Session, tool_name: str, arguments: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:
              """Block until the controller approves/rejects a tool-call *request*.

                  Returns a :class:`ControlDecision`.  If the decision includes
                  ``modified_arguments``, the caller should substitute them before
                  invoking the tool handler.
                  """
              if auto_approve or session.control_disabled:
                  logger.info('Auto-approving request for %s [%s] (tool-flagged or control disabled)', tool_name, session.id)
                  return ControlDecision(approved=True)
              item = self._enqueue(session, 'request', tool_name, arguments=arguments, result=None)
              return self._wait(item)
      - id: ToolControlManager.submit_result
        type: FunctionDef
        code: |-
          def submit_result(self, session: Session, tool_name: str, result: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:
              """Block until the controller approves/replaces a tool-call *result*.

                  Returns a :class:`ControlDecision`.  If the decision includes
                  ``modified_result``, the caller should use that instead of the
                  original result.

                  ``auto_approve`` is a hint set by the *tool itself* (via
                  ``ToolResult.auto_approve``), not derived from the shape of the
                  result.
                  """
              if auto_approve or session.control_disabled:
                  logger.info('Auto-approving result for %s [%s] (tool-flagged or control disabled)', tool_name, session.id)
                  return ControlDecision(approved=True)
              item = self._enqueue(session, 'result', tool_name, arguments=None, result=result)
              return self._wait(item)
      - id: ToolControlManager.enqueue
        type: FunctionDef
        code: |-
          def _enqueue(self, session: Session, phase: str, tool_name: str, arguments: dict[str, Any] | None, result: dict[str, Any] | None) -> _PendingItem:
              with self._lock:
                  self._id_counter += 1
                  item_id = f'{self._id_prefix}-{self._id_counter:x}'
              item = _PendingItem(
                  id=item_id,
                  phase=phase,
                  tool_name=tool_name,
                  session_id=session.id,
                  arguments=arguments,
                  result=result)
              with self._lock:
                  self._pending[item_id] = item
              logger.info('Enqueued control item %s [%s/%s/%s]', tool_name, phase, session.id, item_id)
              return item
      - id: ToolControlManager.process_approvals
        type: FunctionDef
        code: |-
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
                  * ``{"id": "…", "exec": true}`` — approve like ``allow`` but disable
                    a possible tool-provided ``auto_approve`` for the result phase
                    (``/answer <id> exec``), forcing the result to be reviewed too.
                  """
              for approval in approvals:
                  item_id = approval.get('id')
                  if not isinstance(item_id, str):
                      logger.warning("Approval entry missing 'id', skipped: %s", approval)
                      continue
                  with self._lock:
                      item = self._pending.get(item_id)
                  if item is None:
                      logger.warning('Unknown approval id %s, skipped', item_id)
                      continue
                  if approval.get('rejected'):
                      decision = ControlDecision(approved=False, rejection_reason=approval.get(
                          'reason') or 'Reconsider your approach based on the requirements.')
                  else:
                      decision = ControlDecision(
                          approved=True,
                          modified_arguments=approval.get('arguments'),
                          modified_result=approval.get('result'),
                          approval_hint=approval.get('hint'),
                          force_result_review=bool(
                              approval.get('exec')))
                  item._decision = decision
                  with self._lock:
                      self._pending.pop(item_id, None)
                  logger.info('Dequeued control item %s [%s/%s]: approved=%s',
                              item.tool_name, item.phase, item_id, decision.approved)
                  item._event.set()
      - id: NXKGJd|ABmM9X
        type: statements
        code: "'Human-in-the-loop tool control manager.\\n\\nImplements the Manager Pattern for intercepting tool calls at two points:\\n\\n1. *Before* execution (``phase=\"request\"``): the interceptor may approve,\\n   modify arguments, or reject the call entirely.\\n2. *After* execution (``phase=\"result\"``): the interceptor may approve,\\n   replace the tool output, or inject instructions as the result.\\n\\nThe control endpoint at ``/control/tool`` lets an external client poll for\\npending items and post approval decisions.  The intercepting threads block on\\nper-item ``threading.Event`` objects until a decision arrives.\\n'"
```
/answer 3186-20 allow
ReasoningToken: 80
Control Request:
```yaml
id: 3186-21
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy
    pattern: submit_request|submit_result|ControlDecision\(|_id_counter|_id_prefix
    include:
    - '*.py'
    - '*.md'
    exclude:
    - node_modules/**
    - .git/**
    - .venv/**
    limit: 50
    intent: editing
  reason: Aufrufer von submit_request/submit_result und ControlDecision finden
```
/answer 3186-21 allow
Control Request:
```yaml
id: 3186-22
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy
      matches:
      - path: ai/mcpc/control/manager.py
        nodes:
        - id: ToolControlManager.Jy5BQH|BflXLc
          matches:
          - "decision = manager.submit_request(\"bash\", {\"command\": \"rm -rf /\"})"
        - id: ToolControlManager.init.bMxYyn|wmZCRw
          matches:
          - "self._id_prefix = uuid.uuid4().hex[:4]"
          - self._id_counter = 0
        - id: ToolControlManager.submit_request
          matches:
          - "def submit_request(self, session: Session, tool_name: str, arguments: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:"
        - id: ToolControlManager.submit_request.FVslBU|wqaIqV
          matches:
          - return ControlDecision(approved=True)
        - id: ToolControlManager.submit_result
          matches:
          - "def submit_result(self, session: Session, tool_name: str, result: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:"
        - id: ToolControlManager.submit_result.DWelQS|VhbO3N
          matches:
          - return ControlDecision(approved=True)
        - id: ToolControlManager.process_approvals.v0IhH7|7RG2Nh
          matches:
          - decision = ControlDecision(
          - decision = ControlDecision(
        - id: ToolControlManager.cancel_session.rG2XLw|bFz1jG
          matches:
          - the interceptor thread blocked in :meth:`submit_request` /
          - :meth:`submit_result` doesn't hang around until the (up to 24h)
        - id: ToolControlManager.cancel_session.MCD35q|XQAWtj
          matches:
          - "item._decision = ControlDecision(approved=False, rejection_reason=reason)"
        - id: ToolControlManager.enqueue.khH8X5|XcDYom
          matches:
          - self._id_counter += 1
          - "item_id = f'{self._id_prefix}-{self._id_counter:x}'"
        - id: ToolControlManager.wait.BoAlj6|4u3jGV
          matches:
          - return ControlDecision(approved=True)
      - path: ai/mcpc/server/mcp_protocol.py
        nodes:
        - id: McpProtocol.handle_tools_call.2w8tpw|5tuSRq
          matches:
          - "decision = control.submit_request(session, name, arguments)"
        - id: McpProtocol.handle_tools_call.dDYvUB|KOA8ZR
          matches:
          - decision = control.submit_result(
      - path: ai/mcpc/tools/ast/base.py
        nodes:
        - id: id_prefix
          matches:
          - "def _id_prefix(segment: str) -> str | None:"
        - id: resolve_by_prefix.ZRgJo9|a3FPdw
          matches:
          - prefix = _id_prefix(last)
        - id: resolve_by_prefix.OWZHhg|nqsZUH
          matches:
          - "'.')[0] == parent and _id_prefix(loc.node_id.rpartition('.')[2]) == prefix]"
```
/answer 3186-22 allow
Control Request:
```yaml
id: 3186-23
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
    pattern: def handle_tools_call
    include:
    - mcp_protocol.py
    exclude: []
    intent: editing
  reason: handle_tools_call-Knoten finden
```
/answer 3186-23 allow
Control Request:
```yaml
id: 3186-24
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
```
/answer 3186-24 allow
Control Request:
```yaml
id: 3186-25
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    ids:
    - ControlDecision
    - ToolControlManager.wait
    - ToolControlManager.cancel_session
  reason: ControlDecision und _wait lesen
```
/answer 3186-25 allow
Control Request:
```yaml
id: 3186-26
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
      nodes:
      - id: ControlDecision
        type: ClassDef
        code: |-
          @dataclass(slots=True)
          class ControlDecision:
              """The outcome of a human review, produced by :meth:`ToolControlManager.process_approvals`."""
              approved: bool
              'True when the call should proceed (possibly with modified data).'
              rejection_reason: str | None = None
              'Human-readable hint for the agent when ``approved`` is False.'
              approval_hint: str | None = None
              'Optional human-written hint attached to an *approval* (``/allow <id> <hint>``).\n\n    Unlike :attr:`rejection_reason`, this never aborts the call — it enriches\n    the eventual result instead. The same field is used for both the\n    ``request`` and ``result`` phase; when a hint is supplied at both phases\n    for the same call, the two are combined (see ``protocol.py``).\n    '
              modified_arguments: dict[str, Any] | None = None
              'Replacement arguments for the ``request`` phase (``None`` → keep originals).'
              modified_result: dict[str, Any] | None = None
              'Replacement result dict for the ``result`` phase (``None`` → keep original).'
              force_result_review: bool = False
              "Set by an ``exec`` approval: behaves like ``allow`` for the request phase\n    but disables any tool-provided ``auto_approve`` for the subsequent result\n    phase, so a questionable command's output is always reviewed before use.\n    "
      - id: ToolControlManager.wait
        type: FunctionDef
        code: |-
          def _wait(self, item: _PendingItem) -> ControlDecision:
              signalled = item._event.wait(timeout=self._timeout)
              with self._lock:
                  self._pending.pop(item.id, None)
              if not signalled or item._decision is None:
                  '# Timeout — auto-approve to avoid hanging the agent forever.'
                  logger.warning('Control item %s [%s/%s] timed out, auto-approving', item.tool_name, item.phase, item.id)
                  return ControlDecision(approved=True)
              return item._decision
      - id: ToolControlManager.cancel_session
        type: FunctionDef
        code: |-
          def cancel_session(self, session_id: str, reason: str='Connection closed') -> None:
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
                      '# already resolved by a concurrent approval/timeout'
                      continue
                  item._decision = ControlDecision(approved=False, rejection_reason=reason)
                  logger.info('Cancelling control item %s [%s/%s] for session %s: %s',
                              item.tool_name, item.phase, item.id, session_id, reason)
                  item._event.set()
```
/answer 3186-26 allow
ReasoningToken: 132
Control Request:
```yaml
id: 3186-27
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
    pattern: control\.submit_|decision\b
    include:
    - mcp_protocol.py
    exclude: []
    intent: editing
    limit: 40
  reason: Aufrufstellen in mcp_protocol.py mit Kontext finden
```
/answer 3186-27 allow
Control Request:
```yaml
id: 3186-28
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server
      matches:
      - path: mcp_protocol.py
        nodes:
        - id: McpProtocol.handle_tools_call.2w8tpw|5tuSRq
          matches:
          - "decision = control.submit_request(session, name, arguments)"
          - "if not decision.approved:"
          - reason = decision.rejection_reason
          - "if decision.modified_arguments is not None:"
          - arguments = decision.modified_arguments
          - force_result_review = decision.force_result_review
          - "if decision.approval_hint:"
          - "return ToolResult(structured_content={'answer': decision.approval_hint}).to_dict()"
          - request_hint = decision.approval_hint
        - id: McpProtocol.handle_tools_call.dDYvUB|KOA8ZR
          matches:
          - decision = control.submit_result(
          - "if not decision.approved:"
          - reason = decision.rejection_reason
          - "hint_parts = [h for h in (request_hint, decision.approval_hint) if h]"
          - "if decision.modified_result is not None:"
          - result_dict = dict(decision.modified_result)
          - "if combined_hint and decision.modified_result is not None:"
```
/answer 3186-28 allow
Control Request:
```yaml
id: 3186-29
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
    ids:
    - McpProtocol.handle_tools_call
  reason: "Ablauf von request/result-Phase im Aufrufer verstehen, um die Rückreferenz durchzureichen"
```
/answer 3186-29 allow
Control Request:
```yaml
id: 3186-2a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
      nodes:
      - id: McpProtocol.handle_tools_call
        type: FunctionDef
        code: |-
          def _handle_tools_call(self, session: Session, params: dict[str, Any], *, skip_control: bool=False) -> dict[str, Any]:
              name = params.get('name')
              if not isinstance(name, str) or not name:
                  raise errors.invalid_params('"name" is required')
              arguments = params.get('arguments', {})
              if arguments is None:
                  arguments = {}
              if not isinstance(arguments, dict):
                  raise errors.invalid_params('"arguments" must be an object')
              tool = self.registry.get(name)
              '# "Errors in finding the tool" are protocol errors (spec, tools/call).'
              if tool is None or not self.registry.is_enabled(session, name):
                  raise errors.invalid_params(f'Unknown or unavailable tool: {name}', {'name': name})
              _validate_arguments(tool.input_schema, arguments)
              control = self.environment.control_manager if self.environment else None
              request_hint: str | None = None
              force_result_review = False
              if control is not None and (not skip_control):
                  decision = control.submit_request(session, name, arguments)
                  if not decision.approved:
                      reason = decision.rejection_reason
                      if name == TOOLNAME_ASK_USER:
                          return ToolResult(
                              structured_content={
                                  'answer': reason or 'No answer given, anticipate and proceed'}).to_dict()
                      return ToolResult(
                          content=[
                              text_content(
                                  f'Rejected by User: {reason}' if reason else 'Rejected by User. No reason was given; Reconsider your approach based on the requirements.')],
                          is_error=True).to_dict()
                  if decision.modified_arguments is not None:
                      arguments = decision.modified_arguments
                  force_result_review = decision.force_result_review
                  if decision.approval_hint:
                      "# For ask_user, an approval hint *is* the human's answer"
                      if name == TOOLNAME_ASK_USER:
                          return ToolResult(structured_content={'answer': decision.approval_hint}).to_dict()
                      request_hint = decision.approval_hint
              context = ToolContext(session=session, arguments=arguments)
              '# Tool execution errors are reported *inside* the result (isError=true)'
              try:
                  with session.lock:
                      raw = tool.handler(context)
                  result = normalize_result(raw)
              except errors.JsonRpcError:
                  raise
              except Exception as exc:
                  '# noqa: BLE001 - surface as tool error result'
                  result = ToolResult(content=[text_content(f"Tool '{name}' failed: {exc}")], is_error=True)
              if control is not None and (not skip_control):
                  decision = control.submit_result(
                      session,
                      name,
                      result.to_dict(),
                      auto_approve=result.auto_approve and (
                          not force_result_review))
                  if not decision.approved:
                      reason = decision.rejection_reason
                      if name == TOOLNAME_ASK_USER:
                          return ToolResult(
                              structured_content={
                                  'answer': reason or 'No answer given, anticipate and proceed'}).to_dict()
                      return ToolResult(
                          content=[
                              text_content(
                                  f'Rejected by User: {reason}' if reason else 'Rejected by User. No reason was given; Reconsider your approach based on the requirements.')],
                          is_error=True).to_dict()
                  hint_parts = [h for h in (request_hint, decision.approval_hint) if h]
                  combined_hint = '\n'.join(hint_parts) if hint_parts else None
                  if decision.modified_result is not None:
                      result_dict = dict(decision.modified_result)
                  else:
                      result.message = combined_hint
                      result_dict = result.to_dict()
                  if combined_hint and name == TOOLNAME_ASK_USER:
                      '# Same exception as in the request phase: for ask_user the'
                      '# hint *is* the answer, not an independent side-channel field.'
                      return ToolResult(structured_content={'answer': combined_hint}).to_dict()
                  if combined_hint and decision.modified_result is not None:
                      attach_message(result_dict, combined_hint)
                  return result_dict
              return result.to_dict()
```
/answer 3186-2a allow
Control Request:
```yaml
id: 3186-2b
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    id: ControlDecision
    old_text: "    force_result_review: bool = False"
    new_text: |2-
          request_number: int = 0
          "Sequence number of the ``request`` phase item; set by :meth:`ToolControlManager.submit_request`."
          force_result_review: bool = False
    exact: true
  reason: "ControlDecision um die Nummer des Tool-Calls erweitern, damit sie an submit_result durchgereicht werden kann"
```
/answer 3186-2b allow
ReasoningToken: 1193
Control Request:
```yaml
id: 3186-2c
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    id: ToolControlManager.submit_request
    source: |-
      def submit_request(self, session: Session, tool_name: str, arguments: dict[str, Any], *, auto_approve: bool=False) -> ControlDecision:
          """Block until the controller approves/rejects a tool-call *request*.

              Returns a :class:`ControlDecision`.  If the decision includes
              ``modified_arguments``, the caller should substitute them before
              invoking the tool handler.  ``ControlDecision.request_number`` must be
              passed to :meth:`submit_result` to reference this call.
              """
          number = self._next_number()
          if auto_approve or session.control_disabled:
              logger.info('Auto-approving request for %s [%s] (tool-flagged or control disabled)', tool_name, session.id)
              return ControlDecision(approved=True, request_number=number)
          item = self._enqueue(session, 'request', tool_name, arguments=arguments, result=None, number=number, ref=0)
          decision = self._wait(item)
          decision.request_number = number
          return decision
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    id: ToolControlManager.submit_result
    source: |-
      def submit_result(self, session: Session, tool_name: str, result: dict[str, Any], *, request_number: int, auto_approve: bool=False) -> ControlDecision:
          """Block until the controller approves/replaces a tool-call *result*.

              Returns a :class:`ControlDecision`.  If the decision includes
              ``modified_result``, the caller should use that instead of the
              original result.

              ``request_number`` is the number of the originating tool call
              (:attr:`ControlDecision.request_number`); it forms the back reference
              in the item id.

              ``auto_approve`` is a hint set by the *tool itself* (via
              ``ToolResult.auto_approve``), not derived from the shape of the
              result.
              """
          if auto_approve or session.control_disabled:
              logger.info('Auto-approving result for %s [%s] (tool-flagged or control disabled)', tool_name, session.id)
              return ControlDecision(approved=True)
          item = self._enqueue(session, 'result', tool_name, arguments=None, result=result, number=self._next_number(), ref=request_number)
          return self._wait(item)
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
    id: ToolControlManager.enqueue
    source: |-
      def _next_number(self) -> int:
          with self._lock:
              self._id_counter += 1
              return self._id_counter

      def _enqueue(self, session: Session, phase: str, tool_name: str, arguments: dict[str, Any] | None, result: dict[str, Any] | None, *, number: int, ref: int) -> _PendingItem:
          item_id = f'{self._id_prefix}-{number:x}-{ref:x}'
          item = _PendingItem(
              id=item_id,
              phase=phase,
              tool_name=tool_name,
              session_id=session.id,
              arguments=arguments,
              result=result)
          with self._lock:
              self._pending[item_id] = item
          logger.info('Enqueued control item %s [%s/%s/%s]', tool_name, phase, session.id, item_id)
          return item
  reason: "ID-Schema <prefix>-<nummer>-<ref> in manager.py umsetzen; Tool-Calls tragen ref 0, Results die Nummer ihres Calls"
```
/answer 3186-2c allow
ReasoningToken: 59
Control Request:
```yaml
id: 3186-2d
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
    id: McpProtocol.handle_tools_call
    old_text: |2
          force_result_review = False
    new_text: |2
          force_result_review = False
          request_number = 0
    exact: true
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
    id: McpProtocol.handle_tools_call
    old_text: |2
              decision = control.submit_request(session, name, arguments)
    new_text: |2
              decision = control.submit_request(session, name, arguments)
              request_number = decision.request_number
    exact: true
  - path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
    id: McpProtocol.handle_tools_call
    old_text: |2-
                  result.to_dict(),
                  auto_approve=
    new_text: |2-
                  result.to_dict(),
                  request_number=request_number,
                  auto_approve=
    exact: true
  reason: Nummer des Tool-Calls aus der Request-Phase an die Result-Phase durchreichen
```
/answer 3186-2d allow
Text:
 Jetzt prüfen, ob etwas die alte ID-Form parst, und die Dateien validieren.
Control Request:
```yaml
id: 3186-2e
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/mcp_protocol.py
  reason: Syntax der geänderten Dateien prüfen
```
/answer 3186-2e allow Die alte ID Form wird nicht geparsed
Control Request:
```yaml
id: 3186-2f
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/mcpc/src
    pattern: "submit_result\\(|_enqueue\\(|[0-9a-f]{4}-[0-9a-f]+\\b.*(allow|answer)|split\\('-'"
    include:
    - '*.py'
    - '*.md'
    - '*.ts'
    - '*.js'
    exclude:
    - node_modules/**
    - .git/**
    - .venv/**
    limit: 30
  reason: Weitere Aufrufer oder Parser des ID-Formats (inkl. Tests und Clients) finden
```
/answer 3186-2f allow
Control Request:
```yaml
id: 3186-30
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/mcpc/src
      matches:
      - path: xy/ai/mcpc/control/manager.py
        matches:
        - "item = self._enqueue(session, 'request', tool_name, arguments=arguments, result=None, number=number, ref=0)"
        - "def submit_result(self, session: Session, tool_name: str, result: dict[str, Any], *, request_number: int, auto_approve: bool=False) -> ControlDecision:"
        - item = self._enqueue(
        - "def _enqueue(self, session: Session, phase: str, tool_name: str, arguments: dict[str, Any] | None, result: dict[str, Any] | None, *, number: int, ref: int) -> _PendingItem:"
      - path: xy/ai/mcpc/server/mcp_protocol.py
        matches:
        - decision = control.submit_result(
```
/answer 3186-30 allow
Result Stats: id=b74076fe-c51a-4568-a6f8-7c6934609630, total: 30691, in: 20, out: 6584, reason: 0, read: 139695, write: 24087, start: 1794072769761, end: 1794072775705, time: 5882, total: 83381, io: 0, control: 62, sent: 0, recv: 155
Das neue ID-Schema ist umgesetzt. Getestet habe ich es nicht: Beide geänderten Dateien kompilieren, und kein weiterer Code ruft `submit_result` oder `_enqueue` auf. Ein Parser für die alte ID-Form existiert laut deiner Rückmeldung nicht, daher habe ich dort nichts angepasst.

Eine ID hat jetzt die Form `<hash>-<nummer>-<ref>`, beide Zahlen hexadezimal wie bisher. Ein Tool-Call bekommt `ref = 0`, z. B. `3186-27-0`. Die zugehörige Response bekommt die Nummer ihres Calls als `ref`, z. B. `3186-28-27`.

Dafür habe ich in `manager.py` Folgendes geändert:
- **Nummernvergabe:** Die neue Methode `_next_number()` vergibt die laufende Nummer. `_enqueue(...)` bekommt `number` und `ref` als Parameter und baut daraus die ID.
- **Durchreichen der Nummer:** `ControlDecision` hat das neue Feld `request_number`, das `submit_request` setzt. `submit_result` erwartet jetzt `request_number` als Pflichtargument.
- **Aufrufer:** In `server/mcp_protocol.py` (`_handle_tools_call`) merkt sich der Aufrufer die Nummer nach `submit_request` und gibt sie an `submit_result` weiter.

Auch ein automatisch genehmigter Tool-Call verbraucht eine Nummer, obwohl er nie in der Warteschlange erscheint. Sonst hätte eine Response, die zu einem automatisch genehmigten Call gehört, keine eindeutige Rückreferenz. Dadurch kann die sichtbare Nummerierung Lücken haben. Wenn du lieber lückenlos nummerieren willst, wäre die Alternative, bei automatisch genehmigten Calls keine Nummer zu vergeben. Dann müsste die Response aber `ref = 0` tragen, und das wäre mit einem Tool-Call verwechselbar.
