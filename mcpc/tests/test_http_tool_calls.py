"""End-to-end smoke tests driving real tool calls over the Streamable-HTTP
transport (the actual wire protocol a client speaks), not just in-process
Python calls.

Covers: the ``initialize``/``notifications/initialized`` handshake, then
``tools/call`` for ``list``, ``bash`` and ``python_ast_outline`` — exercising
JSON-RPC parsing, session handling, tool dispatch and (for the AST tool) the
real parser stack in one pass.
"""
from __future__ import annotations

import http.client
import json
import textwrap
import threading
import uuid

import pytest

from xy.ai.mcpc.config import ServerConfig
from xy.ai.mcpc.server.server import build_server


@pytest.fixture
def http_client():
    """Spin up a real ``McpHTTPServer`` on a free port and yield a small
    helper for firing JSON-RPC requests at it.

    ``enable_control=False`` disables the human-in-the-loop control manager,
    which would otherwise block ``tools/call`` waiting for an approval that
    never comes.
    """
    config = ServerConfig(host="127.0.0.1", port=0)
    server = build_server(config=config, enable_control=False)
    host, port = server.server_address[0], server.server_address[1]

    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()

    session_id = str(uuid.uuid4())
    enabled_tools = "list,bash,python_ast_outline"

    class Client:
        def rpc(self, method: str, params: dict | None = None, *, notification: bool = False):
            body: dict = {"jsonrpc": "2.0", "method": method, "params": params or {}}
            if not notification:
                body["id"] = 1
            payload = json.dumps(body).encode("utf-8")
            conn = http.client.HTTPConnection(host, port, timeout=10)
            try:
                conn.request(
                    "POST",
                    config.path,
                    body=payload,
                    headers={
                        "Content-Type": "application/json",
                        config.session_header: session_id,
                        # Every request re-evaluates the enabled tool-set; keep
                        # sending it so the session stays configured.
                        config.tools_header: enabled_tools,
                    },
                )
                resp = conn.getresponse()
                raw = resp.read()
            finally:
                conn.close()
            if notification:
                assert resp.status == 202, raw
                return None
            assert resp.status == 200, raw
            message = json.loads(raw)
            assert "error" not in message, message["error"]
            return message["result"]

        def call_tool(self, name: str, arguments: dict) -> dict:
            # The registry injects a mandatory "reason" property onto every
            # tool's input schema (shown to a human authorizer); supply it.
            full_arguments = {"reason": f"automated test of the '{name}' tool", **arguments}
            return self.rpc("tools/call", {"name": name, "arguments": full_arguments})

    client = Client()
    client.rpc("initialize", {"protocolVersion": "2025-06-18", "clientInfo": {"name": "test", "version": "0"}})
    client.rpc("notifications/initialized", notification=True)

    try:
        yield client
    finally:
        server.shutdown()
        thread.join(timeout=5)
        server.server_close()


def _first_text(result: dict) -> str:
    for block in result.get("content", []):
        if block.get("type") == "text":
            return block["text"]
    return ""


def test_http_list_tool(http_client, tmp_path):
    (tmp_path / "a.txt").write_text("a")
    (tmp_path / "b.txt").write_text("b")

    result = http_client.call_tool("list", {"path": str(tmp_path)})

    assert result.get("isError") is not True
    entries = result["structuredContent"]["entries"]
    assert sorted(entries) == ["a.txt", "b.txt"]


def test_http_bash_tool(http_client, tmp_path):
    result = http_client.call_tool(
        "bash", {"cwd": str(tmp_path), "script": "echo hello-mcpc"}
    )

    assert result.get("isError") is not True
    assert "hello-mcpc" in _first_text(result) or "hello-mcpc" in json.dumps(
        result.get("structuredContent", {})
    )


def test_http_python_ast_outline_tool(http_client, tmp_path):
    source = tmp_path / "sample.py"
    source.write_text(
        textwrap.dedent(
            '''
            """Module docstring."""
            import os


            def greet(name: str) -> str:
                """Say hello."""
                return f"hello {name}"


            class Greeter:
                """A greeter."""

                def greet(self) -> None:
                    """Greet."""
                    pass
            '''
        )
    )

    result = http_client.call_tool("python_ast_outline", {"paths": [str(source)]})

    assert result.get("isError") is not True
    structured = result["structuredContent"]
    files = structured["files"]
    assert len(files) == 1
    outline = files[0]
    assert outline["ok"] is True
    assert outline["path"] == str(source)
    function_names = {f["name"] for f in outline.get("functions", [])}
    class_names = {c["name"] for c in outline.get("classes", [])}
    assert "greet" in function_names
    assert "Greeter" in class_names
