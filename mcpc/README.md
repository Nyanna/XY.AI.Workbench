# MCP Controller (`xy.ai.mcpc`)

A **stateful** [Model Context Protocol](https://modelcontextprotocol.io) server
implemented in Python. It speaks JSON-RPC 2.0 over the
MCP **Streamable HTTP** transport and targets the current protocol revision
(`2025-11-25`, with `2025-06-18` / `2025-03-26` negotiated as fallbacks).

## Highlights

- No runtime dependencies — built on the Python standard library only.
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
`--log-level`. Equivalent `MCPC_*` environment variables are also honoured
(`MCPC_HOST`, `MCPC_PORT`, `MCPC_PATH`, `MCPC_LOG_DIR`, `MCPC_SESSION_HEADER`).

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
├── server.py         McpHTTPServer + build_server() / run()
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
