In `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py` werden Tool-Anfragen und Genehmigungen verarbietet.
Die Ausstehenden Anfragen einer Session sollen beim Abbruch der HTTP oder Websocketverbindung automatisch antfernt werden.

* Websocket: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/ws_transport.py`
* HTTP: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/transport.py`
* Server: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server.py`
SystemInit:  id=27160463-637e-4966-81a6-fbb759283bc0, model=/home/user/xyan/xy.ai.workbench, cwd=claude-sonnet-5
Control Request:
```yaml
id: a6ff0cff-8bd4-44ad-9285-ee2c14e61624
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  reason: Understand pending request handling
```
/answer a6ff0cff-8bd4-44ad-9285-ee2c14e61624 allow
Control Request:
```yaml
id: d5a3820e-5922-463e-958a-28e84c3f64dc
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/ws_transport.py
  reason: Understand websocket transport
```
/answer d5a3820e-5922-463e-958a-28e84c3f64dc allow
Control Request:
```yaml
id: 8d270cd0-d91e-4078-a562-06370d803ab2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/transport.py
  reason: Understand http transport
```
/answer 8d270cd0-d91e-4078-a562-06370d803ab2 allow
ReasoningToken: 16
Control Request:
```yaml
id: 3be58353-43df-48f5-a0a8-9d4babeb83da
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server.py
  reason: Understand server wiring of manager and transports
```
/answer 3be58353-43df-48f5-a0a8-9d4babeb83da allow
Control Request:
```yaml
id: eb0ac8c6-02ca-4c0a-9b0c-7b8c47eec06a
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
  reason: List module files
```
/answer eb0ac8c6-02ca-4c0a-9b0c-7b8c47eec06a allow
Control Request:
```yaml
id: f5fa06e6-9424-4b4d-b00a-83d208f7153e
result:
  content:
  - type: text
    text: Too many entries (54) exceed the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
  isError: true
```
/answer f5fa06e6-9424-4b4d-b00a-83d208f7153e allow
ReasoningToken: 958
Control Request:
```yaml
id: 6ba2bc90-a10b-4602-ba14-e5760f58a84a
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc
  pattern: \.py$
  reason: list module files filtered
```
/answer 6ba2bc90-a10b-4602-ba14-e5760f58a84a allow
Control Request:
```yaml
id: 622a5794-5eb7-4d13-a835-760cc33ed092
result:
  content:
  - type: text
    text: Too many entries (53) exceed the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
  isError: true
```
/answer 622a5794-5eb7-4d13-a835-760cc33ed092 allow
Control Request:
```yaml
id: 4d06ef86-eebc-4948-9186-b871fa2fdc42
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/session.py
  reason: Understand Session and SessionStore
```
/answer 4d06ef86-eebc-4948-9186-b871fa2fdc42 allow
ReasoningToken: 790
ReasoningToken: 3754
Control Request:
```yaml
id: d34ba9ea-4e1a-4c12-a263-b01db95663ea
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |
    python3 -c "
    import websockets
    print(websockets.__version__)
    from websockets.asyncio.server import ServerConnection
    import inspect
    print([m for m in dir(ServerConnection) if 'close' in m.lower() or 'wait' in m.lower()])
    print(inspect.signature(ServerConnection.wait_closed))
    "
  reason: Check websockets ServerConnection API for wait_closed
```
/answer d34ba9ea-4e1a-4c12-a263-b01db95663ea allow
Control Request:
```yaml
id: 0a1a6ef6-af1c-4848-8aac-c0828be5b9ba
result:
  structuredContent:
    stdout: |
      15.0.1
      ['close', 'close_code', 'close_reason', 'wait_closed']
      (self) -> 'None'
```
/answer 0a1a6ef6-af1c-4848-8aac-c0828be5b9ba allow
ReasoningToken: 1547
Control Request:
```yaml
id: 519279c8-80dc-498a-80f0-1500267cd476
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/transport.py
  start: from __future__ import annotations
  end: logger = logging.getLogger("xy.ai.mcpc.transport")
  reason: Check imports section
```
/answer 519279c8-80dc-498a-80f0-1500267cd476 allow
Control Request:
```yaml
id: e0ad1092-082d-4048-846d-2bc82d219669
result:
  structuredContent:
    checksum: aeda545ad6967d7ea0d9f64aed8995923479d7e82e8958d919034536392ff89f
    content: |-
      from __future__ import annotations

      import logging

      from http import HTTPStatus
      from http.server import BaseHTTPRequestHandler
      from typing import Any
      from urllib.parse import urlparse

      from . import errors, jsonrpc
      from .codec import JsonCodec
      from .control.handler import ControlHandler
      from .hooks import HookHandler, PermissionHookHandler
      from .jsonrpc import MessageKind
      from .logging_utils import EVENT, IN, OUT
      from .session import is_valid_uuid
      from aptdaemon import logger

      logger = logging.getLogger("xy.ai.mcpc.transport")
```
/answer e0ad1092-082d-4048-846d-2bc82d219669 allow
Control Request:
```yaml
id: 04d3d70a-904d-418f-a2d1-f68ca17b9219
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/context.py
  reason: Check AppServices fields for control_manager access
```
/answer 04d3d70a-904d-418f-a2d1-f68ca17b9219 allow
Control Request:
```yaml
id: 1e78c367-2e81-4d8a-bece-a25fc167e120
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  old_text: |-
    @dataclass
    class _PendingItem:
        id: str
        phase: str          # "request" | "result"
        tool_name: str
        arguments: dict[str, Any] | None       # populated in request phase
        result: dict[str, Any] | None          # populated in result phase
        _event: threading.Event = field(default_factory=threading.Event, repr=False)
        _decision: ControlDecision | None = field(default=None, repr=False)
  new_text: |-
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
  reason: Add session_id field to pending item so it can be cancelled on disconnect
```
/answer 1e78c367-2e81-4d8a-bece-a25fc167e120 allow
Control Request:
```yaml
id: da2411ff-c401-4aee-863f-b535fbe25e31
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/control/manager.py
  old_text: |2-
                item._decision = decision
                with self._lock:
                    self._pending.pop(item_id, None)
                logger.info(
                    "Dequeued control item %s [%s/%s]: approved=%s",
                    item.tool_name, item.phase, item_id, decision.approved,
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
                arguments=arguments,
                result=result,
            )
            with self._lock:
                self._pending[item_id] = item
            logger.info("Enqueued control item %s [%s/%s/%s]", tool_name, phase, session.id, item_id)
            return item
  new_text: |2-
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
  reason: Add cancel_session method and wire session_id into _enqueue
```
/answer da2411ff-c401-4aee-863f-b535fbe25e31 allow
Control Request:
```yaml
id: 689dba7a-84d7-41fc-85f0-64fbac94eeb1
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/ws_transport.py
  old_text: |2-
            try:
                async for raw in connection:
                    await self._handle_message(session_id, session, connection, raw, skip_control)
            except ConnectionClosed:
                pass
            except Exception:  # noqa: BLE001 - never let one connection kill the loop
                logger.exception("WS: unhandled error on connection for session %s", session_id)
            finally:
                self.comm_log.log(session_id, EVENT, {"event": "session.ws_disconnected"})
  new_text: |2-
            try:
                async for raw in connection:
                    await self._handle_message(session_id, session, connection, raw, skip_control)
            except ConnectionClosed:
                pass
            except Exception:  # noqa: BLE001 - never let one connection kill the loop
                logger.exception("WS: unhandled error on connection for session %s", session_id)
            finally:
                self.comm_log.log(session_id, EVENT, {"event": "session.ws_disconnected"})
                control_manager = getattr(self.services, "control_manager", None) if self.services else None
                if control_manager is not None:
                    control_manager.cancel_session(session_id, reason="WebSocket connection closed")
  reason: Cancel pending control items for session on WS disconnect
```
/answer 689dba7a-84d7-41fc-85f0-64fbac94eeb1 allow
ReasoningToken: 322
Control Request:
```yaml
id: 2d89a16d-b6c3-4062-a3cb-c249d5373913
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/ws_transport.py
  old_text: |2-
            loop = asyncio.get_running_loop()

            def _run() -> dict[str, Any]:
                try:
                    with session.lock:
                        result = self.protocol.handle_request(session, request, skip_control=skip_control)
                    return jsonrpc.success_response(request.id, result)
                except errors.JsonRpcError as exc:
                    return jsonrpc.error_response(request.id, exc)
                except Exception as exc:  # noqa: BLE001 - never leak a stack trace
                    logger.exception("WS: unhandled error processing request")
                    return jsonrpc.error_response(request.id, errors.internal_error(str(exc)))

            # Runs on a worker thread: `handle_request` blocks (session lock,
            # human-in-the-loop approval) and must not stall the event loop.
            response = await loop.run_in_executor(None, _run)
            await self._send(connection, session_id, response)
  new_text: |2-
            loop = asyncio.get_running_loop()

            def _run() -> dict[str, Any]:
                try:
                    with session.lock:
                        result = self.protocol.handle_request(session, request, skip_control=skip_control)
                    return jsonrpc.success_response(request.id, result)
                except errors.JsonRpcError as exc:
                    return jsonrpc.error_response(request.id, exc)
                except Exception as exc:  # noqa: BLE001 - never leak a stack trace
                    logger.exception("WS: unhandled error processing request")
                    return jsonrpc.error_response(request.id, errors.internal_error(str(exc)))

            # Runs on a worker thread: `handle_request` blocks (session lock,
            # human-in-the-loop approval) and must not stall the event loop.
            request_future = loop.run_in_executor(None, _run)

            control_manager = getattr(self.services, "control_manager", None) if self.services else None
            if control_manager is not None:
                # Race the (possibly hours-long) request against the connection
                # closing so a pending approval is cancelled the moment the
                # client disconnects, instead of only when it times out.
                closed_future = asyncio.ensure_future(connection.wait_closed())
                try:
                    done, _pending = await asyncio.wait(
                        {request_future, closed_future}, return_when=asyncio.FIRST_COMPLETED
                    )
                    if closed_future in done and request_future not in done:
                        control_manager.cancel_session(
                            session_id, reason="WebSocket connection closed"
                        )
                finally:
                    if not closed_future.done():
                        closed_future.cancel()
                # The interceptor thread unblocks promptly once cancelled above;
                # await it regardless so we always send/skip the final response.
                response = await request_future
            else:
                response = await request_future

            await self._send(connection, session_id, response)
  reason: Race blocking request against connection close to cancel promptly
```
/answer 2d89a16d-b6c3-4062-a3cb-c249d5373913 allow
Control Request:
```yaml
id: bc0bed63-97f3-41a7-836b-e27c7f9dc4f4
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/transport.py
  old_text: |-
    from __future__ import annotations

    import logging

    from http import HTTPStatus
    from http.server import BaseHTTPRequestHandler
    from typing import Any
    from urllib.parse import urlparse

    from . import errors, jsonrpc
    from .codec import JsonCodec
    from .control.handler import ControlHandler
    from .hooks import HookHandler, PermissionHookHandler
    from .jsonrpc import MessageKind
    from .logging_utils import EVENT, IN, OUT
    from .session import is_valid_uuid
    from aptdaemon import logger

    logger = logging.getLogger("xy.ai.mcpc.transport")
  new_text: |-
    from __future__ import annotations

    import logging
    import select
    import socket
    import threading

    from http import HTTPStatus
    from http.server import BaseHTTPRequestHandler
    from typing import Any
    from urllib.parse import urlparse

    from . import errors, jsonrpc
    from .codec import JsonCodec
    from .control.handler import ControlHandler
    from .hooks import HookHandler, PermissionHookHandler
    from .jsonrpc import MessageKind
    from .logging_utils import EVENT, IN, OUT
    from .session import is_valid_uuid

    logger = logging.getLogger("xy.ai.mcpc.transport")

    #: Polling interval (seconds) for the connection watchdog used while a
    #: request is blocked awaiting a human-in-the-loop control decision.
    _CONNECTION_WATCH_INTERVAL = 1.0
  reason: Add imports for connection watchdog and drop unused stray import
```
/answer bc0bed63-97f3-41a7-836b-e27c7f9dc4f4 allow
ReasoningToken: 53
Text:
 Jetzt die WebSocket-Transport-Anpassung für sofortige Erkennung und den finally-Block:
Text:
 Jetzt die schnellere Erkennung während einer laufenden blockierenden Anfrage (race gegen `wait_closed()`):
Control Request:
```yaml
id: 6bdc9940-d1ee-4f00-bb2f-bd5b4e899900
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/transport.py
  old_text: |2-
        # -- request processing -------------------------------------------------
        def _handle_request(self, session_id: str, session, request) -> None:
            skip_control = self.headers.get(self.config.control_header, "").lower() == "off"
            try:
                with session.lock:
                    result = self.protocol.handle_request(session, request, skip_control=skip_control)
                response = jsonrpc.success_response(request.id, result)
            except errors.JsonRpcError as exc:
                response = jsonrpc.error_response(request.id, exc)
            except Exception as exc:  # noqa: BLE001 - never leak a stack trace
                self.server.logger.exception("Unhandled error processing request")  # type: ignore[attr-defined]
                response = jsonrpc.error_response(
                    request.id, errors.internal_error(str(exc))
                )

            self.comm_log.log(session_id, OUT, response, http="POST")
            self._send_json(HTTPStatus.OK, jsonrpc.dumps(response), session_id)
  new_text: |2-
        # -- request processing -------------------------------------------------
        def _handle_request(self, session_id: str, session, request) -> None:
            skip_control = self.headers.get(self.config.control_header, "").lower() == "off"

            control_manager = getattr(self.server.services, "control_manager", None)  # type: ignore[attr-defined]
            stop_watch = threading.Event()
            watcher: threading.Thread | None = None
            if control_manager is not None:
                # `handle_request` may block for hours awaiting a human-in-the-loop
                # decision while holding no socket I/O of its own; without this
                # watchdog an aborted client connection would go unnoticed until
                # the (up to 24h) control timeout elapses.
                watcher = threading.Thread(
                    target=self._watch_connection,
                    args=(session_id, control_manager, stop_watch),
                    name="mcpc-http-connwatch",
                    daemon=True,
                )
                watcher.start()

            try:
                with session.lock:
                    result = self.protocol.handle_request(session, request, skip_control=skip_control)
                response = jsonrpc.success_response(request.id, result)
            except errors.JsonRpcError as exc:
                response = jsonrpc.error_response(request.id, exc)
            except Exception as exc:  # noqa: BLE001 - never leak a stack trace
                self.server.logger.exception("Unhandled error processing request")  # type: ignore[attr-defined]
                response = jsonrpc.error_response(
                    request.id, errors.internal_error(str(exc))
                )
            finally:
                stop_watch.set()
                if watcher is not None:
                    watcher.join(timeout=_CONNECTION_WATCH_INTERVAL + 1.0)

            self.comm_log.log(session_id, OUT, response, http="POST")
            self._send_json(HTTPStatus.OK, jsonrpc.dumps(response), session_id)

        def _watch_connection(self, session_id: str, control_manager, stop_event: threading.Event) -> None:
            """Poll the client socket while a request is in flight and cancel
            this session's pending control items the moment the connection is
            found to be closed (client disconnected without waiting for the
            approval decision).
            """
            sock = self.connection
            while not stop_event.wait(_CONNECTION_WATCH_INTERVAL):
                try:
                    ready, _, _ = select.select([sock], [], [], 0)
                except (OSError, ValueError):
                    return
                if not ready:
                    continue
                try:
                    data = sock.recv(1, socket.MSG_PEEK)
                except BlockingIOError:
                    continue
                except OSError:
                    data = b""
                if data == b"":
                    logger.info(
                        "HTTP connection for session %s closed while awaiting control decision",
                        session_id,
                    )
                    control_manager.cancel_session(session_id, reason="HTTP connection closed")
                    return
  reason: Watchdog thread detects HTTP disconnect during blocked control wait
```
/answer 6bdc9940-d1ee-4f00-bb2f-bd5b4e899900 allow
Control Request:
```yaml
id: 7a7a9f9c-b2ee-4814-893b-97677b5d92d6
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |
    python3 -c "
    import ast
    for f in ['src/xy/ai/mcpc/control/manager.py', 'src/xy/ai/mcpc/transport.py', 'src/xy/ai/mcpc/ws_transport.py']:
        ast.parse(open(f).read())
        print(f, 'OK')
    "
    cd /home/user/xyan/xy.ai.workbench/mcpc && python3 -c "
    import sys
    sys.path.insert(0, 'src')
    import xy.ai.mcpc.transport
    import xy.ai.mcpc.ws_transport
    import xy.ai.mcpc.control.manager
    print('imports OK')
    "
  reason: Validate syntax and imports of modified files
```
/answer 7a7a9f9c-b2ee-4814-893b-97677b5d92d6 allow
Control Request:
```yaml
id: 8469f1cf-250a-402f-b5d8-f0c40780f975
result:
  structuredContent:
    stdout: |
      src/xy/ai/mcpc/control/manager.py OK
      src/xy/ai/mcpc/transport.py OK
      src/xy/ai/mcpc/ws_transport.py OK
      imports OK
```
/answer 8469f1cf-250a-402f-b5d8-f0c40780f975 allow
Control Request:
```yaml
id: fdb39aab-ed29-4efb-829a-cb1de8299550
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc
  pattern: test.*control|test.*transport|test.*ws
  reason: Find relevant tests
```
/answer fdb39aab-ed29-4efb-829a-cb1de8299550 allow
Control Request:
```yaml
id: e120dbe9-bba0-4f73-8057-25ff212bf434
result:
  structuredContent:
    entries: []
```
/answer e120dbe9-bba0-4f73-8057-25ff212bf434 deny Es gibt keine Tests
Control Request:
```yaml
id: 718c6aa5-5428-4494-9c39-07451c2244ac
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/mcpc
  script: |
    cd /home/user/xyan/xy.ai.workbench/mcpc && python3 -c "
    import sys, threading, time
    sys.path.insert(0, 'src')
    from xy.ai.mcpc.control.manager import ToolControlManager
    from xy.ai.mcpc.session import Session

    mgr = ToolControlManager(timeout=5)
    sess = Session(id='abc')

    result = {}
    def worker():
        decision = mgr.submit_request(sess, 'bash', {'command': 'ls'})
        result['decision'] = decision

    t = threading.Thread(target=worker)
    t.start()
    time.sleep(0.2)
    print('pending', mgr.get_pending())
    mgr.cancel_session('abc', reason='test disconnect')
    t.join(timeout=2)
    print('approved=', result['decision'].approved, 'reason=', result['decision'].rejection_reason)
    print('pending after', mgr.get_pending())
    "
  reason: Sanity test cancel_session behavior
```
/answer 718c6aa5-5428-4494-9c39-07451c2244ac allow
Control Request:
```yaml
id: 38dabe3a-a3d6-402d-9e9d-af9583302645
result:
  structuredContent:
    stdout: |
      pending [{'id': '52d0eb9a-1c6f-4c89-859b-aee01d9f34d5', 'toolName': 'bash', 'arguments': {'command': 'ls'}}]
      approved= False reason= test disconnect
      pending after []
```
/answer 38dabe3a-a3d6-402d-9e9d-af9583302645 allow
Text:
 Jetzt der HTTP-Transport: Watcher-Thread, der die Socket-Verbindung während des blockierenden Requests überwacht.
ReasoningToken: 41
ReasoningToken: 62
Text:
 Jetzt die `_handle_request`-Methode anpassen, um den Watchdog-Thread zu starten:
Text:
 Nun kurze Syntax-/Importprüfung:
Text:
 Jetzt versuche ich, vorhandene Tests laufen zu lassen, um Regressionen auszuschließen.
ReasoningToken: 59 
Die Kernlogik funktioniert wie erwartet. Zusammenfassung der Änderungen:

## Änderungen

**`control/manager.py`**
- `_PendingItem` trägt nun ein `session_id`-Feld.
- Neue Methode `ToolControlManager.cancel_session(session_id, reason=...)`: entfernt alle noch offenen Requests/Results einer Session aus `_pending`, setzt eine `ControlDecision(approved=False, rejection_reason=...)` und weckt den blockierten Interceptor-Thread sofort auf (statt bis zu 24h zu warten).

**`ws_transport.py`**
- Im `finally`-Block von `_handle_connection` wird beim Verbindungsende (egal ob sauber oder durch Exception) `control_manager.cancel_session(...)` aufgerufen.
- Zusätzlich läuft in `_handle_request` die blockierende `handle_request`-Ausführung (im Executor-Thread) parallel zu `connection.wait_closed()`. Schließt der Client die WS-Verbindung *während* eine Genehmigung noch aussteht, wird sofort abgebrochen statt erst beim regulären Verbindungsende.

**`transport.py`**
- Neuer Watchdog-Thread (`_watch_connection`), der während einer laufenden `_handle_request`-Bearbeitung den zugrunde liegenden TCP-Socket per `select` + `MSG_PEEK` überwacht. Erkennt er, dass die Gegenstelle die Verbindung geschlossen hat (EOF/Reset), ruft er ebenfalls `control_manager.cancel_session(...)` auf.
- Nebenbei einen offensichtlich fehlerhaften Altimport (`from aptdaemon import logger`) entfernt, der sofort durch die eigentliche Logger-Initialisierung überschrieben wurde.

Damit werden ausstehende Genehmigungsanfragen einer Session automatisch entfernt/abgelehnt, sobald die zugehörige HTTP- oder WebSocket-Verbindung abbricht, ohne auf den (bis zu 24h langen) Timeout warten zu müssen.
 