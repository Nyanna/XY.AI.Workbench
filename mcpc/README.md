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