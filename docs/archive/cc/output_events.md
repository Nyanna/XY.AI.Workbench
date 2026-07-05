# Claude Code Output Events

Claude Code emits one JSON object per line on stdout. Each has a top-level `type` field.

## `system` Events

### `system/init` — Session Start

Emitted once when the process starts.

```json
{
  "type": "system",
  "subtype": "init",
  "session_id": "abc-123-def-456",
  "model": "claude-sonnet-4-6-20250514",
  "tools": ["Read", "Write", "Edit", "Bash", "Glob", "Grep", "Agent"]
}
```

| Field | Type | Description |
|---|---|---|
| `session_id` | `string` | Claude Code's internal session ID |
| `model` | `string` | Model being used (e.g., `claude-sonnet-4-6-20250514`) |
| `tools` | `string[]` | Available tool names (when present) |

### `system/result` — Legacy Turn Complete

Older Claude Code versions emit this instead of a top-level `result` event.

```json
{
  "type": "system",
  "subtype": "result",
  "session_id": "abc-123",
  "result": "\"Here is the summary...\"",
  "is_error": false
}
```

::: tip
The `result` field is **double-encoded**. `JSON.parse(event.result)` gives you the actual string.
:::

## `assistant` Events

Emitted whenever the assistant's message state changes. In `--verbose` mode, this contains the **full message snapshot** — all content blocks accumulated so far.

```json
{
  "type": "assistant",
  "message": {
    "role": "assistant",
    "content": [
      { "type": "thinking", "thinking": "Let me analyze this code..." },
      { "type": "text", "text": "Here's what I found:" },
      {
        "type": "tool_use",
        "id": "toolu_abc123",
        "name": "Read",
        "input": { "file_path": "/src/main.ts" }
      }
    ],
    "stop_reason": "tool_use"
  }
}
```

### Content Block Types

#### `text` — Markdown Output

```json
{ "type": "text", "text": "Here's the analysis..." }
```

#### `thinking` — Extended Thinking

```json
{ "type": "thinking", "thinking": "Let me consider the architecture..." }
```

::: warning
Some Claude Code versions use `"text"` instead of `"thinking"` for the content field. Always check both:
```typescript
const text = block.thinking ?? block.text ?? ''
```
:::

#### `tool_use` — Tool Invocation

```json
{
  "type": "tool_use",
  "id": "toolu_abc123",
  "name": "Read",
  "input": { "file_path": "/src/main.ts" }
}
```

The `id` field uniquely identifies this tool call — used for approval/denial and matching with `tool_result`.

#### `tool_result` — Tool Result (in assistant)

Can also appear in assistant events (less common). See `user` events below for the primary path.

```json
{
  "type": "tool_result",
  "tool_use_id": "toolu_abc123",
  "content": "file contents here...",
  "is_error": false
}
```

### Common Tool Names

| Tool | Typical Input | Description |
|---|---|---|
| `Read` | `{ "file_path": "..." }` | Read file contents |
| `Write` | `{ "file_path": "...", "content": "..." }` | Create/overwrite file |
| `Edit` | `{ "file_path": "...", "old_string": "...", "new_string": "..." }` | String replacement |
| `Bash` | `{ "command": "..." }` | Execute shell command |
| `Glob` | `{ "pattern": "..." }` | Find files by pattern |
| `Grep` | `{ "pattern": "...", "path": "..." }` | Search file contents |
| `Agent` | `{ "prompt": "...", "description": "..." }` | Spawn sub-agent |
| `AskUserQuestion` | `{ "question": "..." }` | Ask user for input |
| `WebSearch` | `{ "query": "..." }` | Search the web |
| `WebFetch` | `{ "url": "..." }` | Fetch URL contents |

## `user` Events

Emitted when Claude Code processes a tool result. This is the primary path for tool results flowing through the conversation.

```json
{
  "type": "user",
  "message": {
    "role": "user",
    "content": [
      {
        "type": "tool_result",
        "tool_use_id": "toolu_abc123",
        "content": "file contents...",
        "is_error": false
      }
    ]
  }
}
```

The `content` field is **polymorphic** — see [Gotchas](/protocol/gotchas#polymorphic-tool_resultcontent).

## `result` Events

The primary turn-completion event in current Claude Code versions.

```json
{
  "type": "result",
  "subtype": "success",
  "session_id": "abc-123",
  "result": "\"Task completed successfully.\"",
  "is_error": false,
  "total_cost_usd": 0.042,
  "duration_ms": 12500,
  "duration_api_ms": 8200,
  "num_turns": 3,
  "modelUsage": {
    "claude-sonnet-4-6-20250514": {
      "inputTokens": 1200,
      "outputTokens": 450,
      "cacheReadInputTokens": 8500,
      "cacheCreationInputTokens": 0,
      "contextWindow": 200000
    }
  }
}
```

| Field | Type | Description |
|---|---|---|
| `subtype` | `"success"` \| `"error"` | Whether the turn succeeded |
| `total_cost_usd` | `number` | Cumulative session cost in USD |
| `duration_ms` | `number` | Total wall time for this turn |
| `duration_api_ms` | `number` | Time spent in API calls |
| `num_turns` | `number` | Number of agentic loops in this interaction |
| `modelUsage` | `Record<string, ModelUsage>` | Per-model token breakdown |

### `modelUsage` Structure

Keyed by model ID. Each entry:

| Field | Description |
|---|---|
| `inputTokens` | Fresh input tokens sent |
| `outputTokens` | Tokens generated |
| `cacheReadInputTokens` | Tokens served from prompt cache |
| `cacheCreationInputTokens` | Tokens written to prompt cache |
| `contextWindow` | Model's maximum context window |

**Total input tokens** = `inputTokens + cacheReadInputTokens + cacheCreationInputTokens`

::: info Both result paths
Both `system/result` (legacy) and top-level `result` can appear depending on Claude Code version. The translator handles both.
:::

## Ignored Event Types

| Type | Purpose | Action |
|---|---|---|
| `progress` | Internal progress updates | Silently skipped |
| `rate_limit_event` | Rate limit notifications | Silently skipped |