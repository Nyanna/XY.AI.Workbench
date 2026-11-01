# xy.ai.mcpc.embedded

Run an MCPC server (MCP server giving an agent focused tools) inside your own Python process and drive Claude CLI agents against it. Use it for batch scripts: iterate over inputs, one agent per item, parallel.

```python
from xy.ai.mcpc.embedded import EmbeddedMcpc, launch_agent, FunctionTool, ToolProvider, Model, Effort
```

## Pick a topology

| Need | Use |
|---|---|
| Many agents, **one shared** MCPC with preconfigured tools | `EmbeddedMcpc` + `run_batch` / `start_agent` |
| Each agent its **own specially configured** MCPC (tools, profile) | `launch_agent(...)` |

## Shared MCPC (default pattern)

```python
with EmbeddedMcpc(builtin_tools=["read_file", "search"], auto_approve=True) as mcpc:

    @mcpc.tool("lookup", "Look up a record by id",
               {"type": "object", "properties": {"id": {"type": "string"}}, "required": ["id"]})
    def lookup(ctx):                       # ctx.arguments: dict, ctx.session
        return db[ctx.arguments["id"]]     # str | dict | ToolResult

    results = mcpc.run_batch(
        prompts,                           # one fresh agent per prompt, results in prompt order
        "You classify records. Use `lookup`.",   # system prompt
        tools={"lookup", "read_file"},     # per-agent toolset (None = all registered)
        model=Model.HAIKU, effort=Effort.LOW,
        max_workers=8)

    for r in results:                      # CliResult(text, is_error, subtype)
        print("ERR" if r.is_error else "ok", r.text)
```

Register tools **before or after** `start()`; `@mcpc.tool` works in both cases.

## Multi-turn / manual control

```python
with mcpc.start_agent("You are a reviewer.", tools={"read_file"}) as agent:
    a = agent.ask("Review foo.py")         # str; raises CliSessionError on error result
    r = agent.prompt("Now bar.py")         # CliResult, never raises on agent-side errors
```

The conversation persists across calls on the same `Agent`. Closing (context manager or `agent.close()`) kills the CLI process and the MCPC session.

## Dedicated MCPC per agent

```python
with launch_agent("You review SQL.", builtin_tools=["read_file"], tools=[MyProvider()],
                  cc_profile="review", model=Model.OPUS) as agent:
    print(agent.ask("..."))                # closing also stops its server
```

For a pool of specialised agents run several `launch_agent` calls in a `ThreadPoolExecutor`.

## API

**`EmbeddedMcpc(config=None, *, builtin_tools=True, tools=(), auto_approve=None, ignore_tools_header=None, enable_control=True)`**
- `builtin_tools`: `True` all, `False` none, or names to keep. Run `mcpc.tool_names` after `start()` to see what is registered.
- `tools`: `ToolDefinition`s or `ToolProvider`s. `FunctionTool(name, description, input_schema, handler)` wraps a callable `(ctx) -> result`.
- `auto_approve=True`: required in scripts, otherwise tool calls wait for a controller that does not exist.
- Default config: `127.0.0.1`, ephemeral HTTP and WS ports.
- `start()`/`stop()`/context manager, `running`, `url`, `ws_url`, `server`.
- `add_tool(tool)`, `@tool(name, description, input_schema=None)`.
- `open_session(tools=None, session_id=None, cc_profile="none") -> sid`, `close_session(sid)`, `headers(sid)`: for an external MCP client instead of the built-in agent.
- `start_agent(system_prompt, *, model=Model.SONNET, effort=Effort.MEDIUM, tools=None, cc_profile="none", cache_mode=CacheMode.DEFAULT, executable="claude", extra_env=None, agent_id=None) -> Agent`
- `run_batch(prompts, system_prompt, *, max_workers=4, **start_agent_options) -> list[CliResult]`

**`Agent`**: `id`, `prompt(text) -> CliResult`, `ask(text) -> str`, `close()`.

**Enums** (plain strings also accepted): `Model` = haiku|sonnet|opus; `Effort` = disabled|low|medium|high|xhigh|max; `CacheMode` = default|disabled|5m|1h.

## Gotchas

- `tools=` on `start_agent`/`run_batch` selects from the **registered** tool names; unknown names give the agent nothing.
- `cc_profile` selects the Claude config dir `~/.claude-<profile>` (login, settings). Use a profile that is authenticated.
- `run_batch` converts `CliSessionError` (timeout, crash) into `CliResult(is_error=True)`; check `is_error` per result.
- Sessions expire after `agent_session_ttl_seconds` of inactivity; the response timeout defaults to 24 h. Override via `ServerConfig`.
- Keep `max_workers` modest: each agent is one `claude` process.
- CLI traffic is logged per agent under `config.cli_log_dir` (`<agent id>.json.log`); look there when a result is an error.
