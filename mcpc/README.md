# MCP Controller (`xy.ai.mcpc`)

A **stateful** [Model Context Protocol](https://modelcontextprotocol.io) server
implemented in Python. It speaks JSON-RPC 2.0 over the
MCP **Streamable HTTP** transport and targets the current protocol revision
(`2025-11-25`, with `2025-06-18` / `2025-03-26` negotiated as fallbacks).

## Highlights

- Streamable-HTTP transport built on the Python standard library only; the
  WebSocket transport (see below) is the one part that depends on the
  `websockets` package.
- Single MCP endpoint (default `http://127.0.0.1:9093/mpc`), `POST` + `GET` + `DELETE`.
- Central **tool registry**, enabled **per session** (registry is reconciled
  against the session context on every `tools/list` / `tools/call`).
- Session id is the primary key, supplied by the client on **every** request via
  the `X-MCPC-SESSION-ID` header (must be a UUID).
- In-memory, server-side session state persists configuration and arbitrary
  key/value state for the lifetime of the process.
- All communication is logged line-by-line (NDJSON) to
  `<log_dir>/<session-id>.log`.

## Running

```bash
# from the project root
PYTHONPATH=src python -m xy.ai.mcpc --host 127.0.0.1 --port 9093 --path /mpc

# or, after `pip install -e .`
mcpc --port 9093
```

CLI options: `--host`, `--port`, `--path`, `--log-dir`, `--session-header`,
`--log-level`, `--ws-host`, `--ws-port`, `--ws-path`, `--no-ws`. Equivalent
`MCPC_*` environment variables are also honoured (`MCPC_HOST`, `MCPC_PORT`,
`MCPC_PATH`, `MCPC_LOG_DIR`, `MCPC_SESSION_HEADER`, `MCPC_WS_ENABLED`,
`MCPC_WS_HOST`, `MCPC_WS_PORT`, `MCPC_WS_PATH`).

## WebSocket transport

A second interface to the same server-side sessions, tools and protocol
logic — no functionality is duplicated, just exposed over a different wire
format. It listens independently of the HTTP transport (default
`ws://127.0.0.1:9094/mcp`) and is started automatically unless `--no-ws` /
`MCPC_WS_ENABLED=0` is set or the `websockets` package is missing (in which
case the HTTP transport still starts normally, with a warning logged).

One WebSocket connection = one MCP session for its whole lifetime:

- The session id, `X-MCPC-TOOLS` and `X-MCPC-CC-PROFILE` are read once from
  the **opening handshake** request (headers, or — for clients that cannot
  set custom headers — same-named query parameters) and applied exactly like
  their HTTP-header counterparts. `X-MCPC-CONTROL: off` on the handshake
  disables tool-call interception for every request on that connection.
- Every text frame is one JSON-RPC message: a request gets one JSON-RPC
  response frame back; a notification produces no reply; the server never
  sends unsolicited messages.
- Handshake validation mirrors the HTTP transport (unknown path, forbidden
  `Origin`, missing/invalid session id) and rejects the connection with
  WebSocket close code `1008` (policy violation) instead of an HTTP status.
- All traffic is appended to the same per-session NDJSON log
  (`<log_dir>/<session-id>.json.log`), tagged `"transport": "ws"`.

```python
import asyncio, json, uuid, websockets

async def main():
    session_id = str(uuid.uuid4())
    async with websockets.connect(
        "ws://127.0.0.1:9094/mcp",
        additional_headers={"X-MCPC-SESSION-ID": session_id},
    ) as ws:
        await ws.send(json.dumps({
            "jsonrpc": "2.0", "id": 1, "method": "initialize",
            "params": {"protocolVersion": "2025-06-18", "capabilities": {},
                       "clientInfo": {"name": "example", "version": "0"}},
        }))
        print(await ws.recv())

asyncio.run(main())
```

## Transport semantics

| Method   | Behaviour                                                                 |
| -------- | ------------------------------------------------------------------------- |
| `POST`   | Body is one JSON-RPC message. Requests → `200 application/json`; notifications/responses → `202 Accepted`. |
| `GET`    | `405` — no server-to-client SSE stream (notifications unsupported).       |
| `DELETE` | Terminates the session → `204` (or `404` if unknown).                     |

Every request must carry `X-MCPC-SESSION-ID: <uuid>`. Missing/invalid → `400`.
The `Origin` header is validated (localhost only) to prevent DNS rebinding, and
an unsupported `MCP-Protocol-Version` header yields `400`.

## Built-in tools

`echo`, `add`, `server_time`, `session_set`, `session_get`, `session_info`.
The `session_*` tools demonstrate per-session, cross-request state.

## Architecture

```
src/xy/ai/mcpc/
├── config.py         ServerConfig (host/port/path/log dir/protocol versions)
├── errors.py         JSON-RPC / MCP error codes + JsonRpcError
├── jsonrpc.py        JSON-RPC 2.0 parsing, classification, envelopes
├── session.py        Session + thread-safe SessionStore (stateful, in-memory)
├── logging_utils.py  CommunicationLog (per-session NDJSON audit log)
├── registry.py       Tool, ToolRegistry, ToolResult, content helpers
├── protocol.py       McpProtocol: initialize / ping / tools.list / tools.call
├── transport.py      StreamableHttpHandler (http.server)
├── ws_transport.py   WebSocketMcpServer — second interface, same protocol/sessions
├── server.py         McpHTTPServer + build_server() / build_ws_server() / run()
├── tools/            built-in example tools
└── __main__.py       CLI entry point
```

### Registering your own tools

```python
from xy.ai.mcpc import ServerConfig, ToolRegistry, ToolContext, build_server

registry = ToolRegistry()

@registry.tool(
    "greet",
    description="Greet someone by name.",
    input_schema={"type": "object", "properties": {"name": {"type": "string"}},
                  "required": ["name"]},
)
def greet(ctx: ToolContext):
    return f"Hello, {ctx.arguments['name']}!"

server = build_server(ServerConfig(port=9093), registry=registry, register_builtin=False)
server.serve_forever()
```

A handler may return a `str`, a `dict` (structured content) or a `ToolResult`.
Exceptions raised inside a handler are reported as a tool error result
(`isError: true`) rather than a protocol error, so the model can self-correct.
