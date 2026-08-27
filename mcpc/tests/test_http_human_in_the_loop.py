"""End-to-end smoke test for the human-in-the-loop tool control flow.

With ``enable_control=True`` (the default used in production), every
``tools/call`` is intercepted twice — once before execution ("request"
phase) and once after ("result" phase) — and blocks until a decision is
posted to the ``/control/tool`` endpoint. This drives that whole loop over
real HTTP: fire a ``bash`` tool call from a background thread, poll the
control endpoint until the pending request-phase item shows up, approve it,
then do the same for the result-phase item (attaching a human hint), and
finally check that the original call unblocks with the expected result and
that the hint landed in ``structuredContent.controlHint``.
"""
from __future__ import annotations

import http.client
import json
import threading
import time
import uuid

import pytest

from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.server.server import build_server


@pytest.fixture
def hitl_server():
    """Spin up a real server with the control manager enabled."""
    config = ServerConfig(host="127.0.0.1", port=0)
    server = build_server(config=config, enable_control=True)
    host, port = server.server_address[0], server.server_address[1]

    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()

    try:
        yield server, config, host, port
    finally:
        server.shutdown()
        thread.join(timeout=5)
        server.server_close()


def _post(host: str, port: int, path: str, body: dict | None, headers: dict, timeout: float = 15.0):
    conn = http.client.HTTPConnection(host, port, timeout=timeout)
    try:
        payload = json.dumps(body).encode("utf-8") if body is not None else b""
        conn.request("POST", path, body=payload, headers=headers)
        resp = conn.getresponse()
        raw = resp.read()
    finally:
        conn.close()
    return resp.status, raw


def test_http_tool_call_awaits_human_approval(hitl_server, tmp_path):
    server, config, host, port = hitl_server
    session_id = str(uuid.uuid4())

    def rpc(method: str, params: dict | None = None, *, notification: bool = False, timeout: float = 15.0):
        body: dict = {"jsonrpc": "2.0", "method": method, "params": params or {}}
        if not notification:
            body["id"] = 1
        headers = {
            "Content-Type": "application/json",
            config.session_header: session_id,
            config.tools_header: "bash",
        }
        status, raw = _post(host, port, config.path, body, headers, timeout=timeout)
        if notification:
            assert status == 202, raw
            return None
        assert status == 200, raw
        message = json.loads(raw)
        assert "error" not in message, message.get("error")
        return message["result"]

    def control_poll(approvals: list[dict] | None = None) -> list[dict]:
        status, raw = _post(
            host, port, config.control_path,
            {"approvals": approvals or []},
            {"Content-Type": "application/json"},
        )
        assert status == 200, raw
        return json.loads(raw)["pending"]

    def wait_for_pending(tool_name: str, timeout: float = 5.0) -> dict:
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            for item in control_poll():
                if item.get("toolName") == tool_name or "result" in item:
                    return item
            time.sleep(0.02)
        raise AssertionError(f"no pending control item for {tool_name!r} within {timeout}s")

    # -- handshake --------------------------------------------------------
    rpc("initialize", {"protocolVersion": "2025-06-18", "clientInfo": {"name": "test", "version": "0"}})
    rpc("notifications/initialized", notification=True)

    # -- fire the tool call in the background; it blocks on control -------
    outcome: dict = {}

    def call_bash():
        outcome["result"] = rpc(
            "tools/call",
            {
                "name": "bash",
                "arguments": {
                    "cwd": str(tmp_path),
                    "script": "echo hitl-ok",
                    "reason": "human-in-the-loop smoke test",
                },
            },
            timeout=30,
        )

    caller = threading.Thread(target=call_bash, daemon=True)
    caller.start()

    # -- phase 1: approve the request before execution ---------------------
    request_item = wait_for_pending("bash")
    assert request_item["toolName"] == "bash"
    assert request_item["arguments"]["script"] == "echo hitl-ok"
    control_poll([{"id": request_item["id"]}])

    # -- phase 2: approve the result, attaching a human hint ---------------
    result_item = wait_for_pending("bash")
    assert "stdout" in result_item.get("result", {}).get("structuredContent", {})
    control_poll([{"id": result_item["id"], "hint": "looks fine"}])

    caller.join(timeout=15)
    assert not caller.is_alive(), "tool call did not unblock after approval"

    result = outcome["result"]
    assert result.get("isError") is not True
    structured = result["structuredContent"]
    assert "hitl-ok" in structured.get("stdout", "")
    assert structured.get("controlHint") == "looks fine"


def test_http_tool_call_rejected_by_human(hitl_server, tmp_path):
    server, config, host, port = hitl_server
    session_id = str(uuid.uuid4())

    def rpc(method: str, params: dict | None = None, *, notification: bool = False, timeout: float = 15.0):
        body: dict = {"jsonrpc": "2.0", "method": method, "params": params or {}}
        if not notification:
            body["id"] = 1
        headers = {
            "Content-Type": "application/json",
            config.session_header: session_id,
            config.tools_header: "bash",
        }
        status, raw = _post(host, port, config.path, body, headers, timeout=timeout)
        if notification:
            assert status == 202, raw
            return None
        assert status == 200, raw
        message = json.loads(raw)
        assert "error" not in message, message.get("error")
        return message["result"]

    def control_poll(approvals: list[dict] | None = None) -> list[dict]:
        status, raw = _post(
            host, port, config.control_path,
            {"approvals": approvals or []},
            {"Content-Type": "application/json"},
        )
        assert status == 200, raw
        return json.loads(raw)["pending"]

    def wait_for_pending(tool_name: str, timeout: float = 5.0) -> dict:
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            for item in control_poll():
                if item.get("toolName") == tool_name:
                    return item
            time.sleep(0.02)
        raise AssertionError(f"no pending control item for {tool_name!r} within {timeout}s")

    rpc("initialize", {"protocolVersion": "2025-06-18", "clientInfo": {"name": "test", "version": "0"}})
    rpc("notifications/initialized", notification=True)

    outcome: dict = {}

    def call_bash():
        outcome["result"] = rpc(
            "tools/call",
            {
                "name": "bash",
                "arguments": {
                    "cwd": str(tmp_path),
                    "script": "echo should-not-run",
                    "reason": "human-in-the-loop rejection test",
                },
            },
            timeout=30,
        )

    caller = threading.Thread(target=call_bash, daemon=True)
    caller.start()

    request_item = wait_for_pending("bash")
    control_poll([{"id": request_item["id"], "rejected": True, "reason": "not now"}])

    caller.join(timeout=15)
    assert not caller.is_alive(), "tool call did not unblock after rejection"

    result = outcome["result"]
    assert result.get("isError") is True
    text = "".join(block.get("text", "") for block in result.get("content", []))
    assert "not now" in text
