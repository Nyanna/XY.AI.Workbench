Recherchiere was das system/init Event von Claude Code ist und wann exakt es emittiert wird.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=c3eafc62-71c7-4066-9208-5b0d2123eeab | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 34f57687-deab-4dc6-852a-eb52f0ce8529
toolName: web-search-exa
arguments:
  query: Claude Code system init event stream-json emitted
  reason: Research the system/init event in Claude Code
```
/answer 34f57687-deab-4dc6-852a-eb52f0ce8529 allow

ReasoningToken: 41
Control Request:
```yaml
id: bf102198-c6af-46d5-97ab-60d255925559
result:
  structuredContent:
    content: |-
      Title: [DOCS] Headless stream-json init event missing `plugin_errors` documentation
      URL: https://github.com/anthropics/claude-code/issues/49308
      Published: 2026-04-16T17:16:56.000Z
      Author: coygeek
      Highlights:
      ## [DOCS] Headless stream-json init event missing `plugin_errors` documentation
      ...
      "Stream responses" (`--output-format stream-json`) and the init event payload used to report plugin startup state
      ...
      > Use `--output-format stream-json` with `--verbose` and `--include-partial-messages` to receive tokens as they're generated. Each line is a JSON object representing an event:
      ...
      stream_event"
      ...
      ") | .
      ...
      The SDK TypeScript reference currently defines the init system message as:
      ...
      > `type SDKSystemMessage = {`
      > ` type: "system";`
      > ` subtype: "init";`
      > ` ...`
      > ` mcp_servers: { name: string; status: string; }[];`
      > ` ...`
      > ` slash_commands: string[];`
      > ` output_style: string;`
      > ` skills: string[];`
      > ` plugins: { name: string; path: string }[];`
      > `};`
      ...
      And the plugin SDK guide tells readers to inspect the init message only for loaded plugins and commands:
      ...
      Changelog v2.1.111 adds a new startup behavior: headless `--output-format stream-json` now includes `plugin_errors` on the init event when plugins are demoted for unsatisfied dependencies.
      ...
      The current docs do not describe that init-event field or the demotion behavior:
      ...
      ### A. Headless stream-json docs omit the init payload details
      ...
      The headless page explains that stream-json emits events, but it does not document the startup/init event shape or mention that plugin dependency failures surface through `plugin_errors`.
      ...
      The TypeScript init-message field list omits `plugin_errors`, and the Python reference only describes `SystemMessage` generically. Readers who inspect init messages to verify plugin state are not told how dependency-related plugin demotions are reported.
      ...
      The plugin docs discuss successful plugin loading and general loading errors, but they do not explain that unsatisfied dependencies can demote a plugin and that headless/SDK consumers should check `plugin_errors` on the init event.
      ...
      Update the headless and Agent SDK docs to document the init event as part of the `--output-format stream-json` contract.
      ...
      1. In https://code.claude.com/docs/en/headless, add a short init-event example for `--output-format stream-json` and mention that `plugin_errors` is included when plugins are demoted for unsatisfied dependencies.
      2. In the TypeScript and Python Agent SDK message references, add `plugin_errors` to the documented init-message payload and describe when it appears.
      3. In the plugin/SDK plugin docs, add a troubleshooting note telling users to inspect init output for demoted plugins caused by missing or unsatisfied dependencies.
      ...
      | Page | Context |
      | --- | --- |
      | https://code.claude.com/docs/en/headless | Documents `--output-format stream-json`, but not the init event schema or `plugin_errors` |
      | https://code.claude.com/docs/en/cli-reference | Documents the CLI flags that enable stream-json output |
      | https://code.claude.com/docs/en/agent-sdk/typescript | `SDKSystemMessage` lists init fields such as `mcp_servers`, `slash_commands`, `skills`, and `plugins`, but not `plugin_errors` |
      | https://code.claude.com/docs/en/agent-sdk/python | `SystemMessage` is described only generically, with no documented init payload fields for plugin demotion/errors |
      | https://code.claude.com/docs/en/agent-sdk/plugins | Tells users to inspect the init message for loaded plugins, but not for dependency-driven demotion via `plugin_errors` |
      ...
      **Changelog entry:** `Headless --output-format stream-json now includes plugin_errors on the init event when plugins are demoted for unsatisfied dependencies`

      ---

      Title: Stream responses in real-time - Claude Code Docs
      URL: https://code.claude.com/docs/en/agent-sdk/streaming-output
      Published: N/A
      Author: N/A
      Highlights:
      To enable streaming, set `include_partial_messages` (Python) or `includePartialMessages` (TypeScript) to `true` in your options. This causes the SDK to yield `StreamEvent` messages containing raw API events as they arrive, in addition to the usual `AssistantMessage` and `ResultMessage`.
      ...
      s type to distinguish
      ...
      Event` from
      ...
      its `type
      ...
      When partial messages are enabled, you receive raw Claude API streaming events wrapped in an object. The type has different names in each SDK:
      ...
      * **Python**: `StreamEvent` (import from `claude_agent_sdk.types`)
      * **TypeScript**: `SDKPartialAssistantMessage` with `type: 'stream_event'`
      ...
      Both contain raw Claude API events, not accumulated text. You need to extract and accumulate text deltas yourself. Here's the structure of each type:


        ```python Python theme={null}
        @dataclass
        class StreamEvent:
            uuid: str  # Unique identifier for this event
            session_id: str  # Session identifier
            event: dict[str, Any]  # The raw Claude API stream event
            parent_tool_use_id: str | None  # Always None
        ```

        ```typescript TypeScript theme={null}
        type SDKPartialAssistantMessage = {
          type: "stream_event";
          event: BetaRawMessageStreamEvent; // From Anthropic SDK
          parent_tool_use_id: string | null;
          uuid: UUID;
          session_id: string;
          ttft_ms?: number; // Time to first token in ms, present only on message_start events
        };
        ```


      The `parent_tool_use_id` field is always `None` in Python and `null` in TypeScript. Stream events are emitted for the main session only; token-level deltas from subagents aren't forwarded. To attribute output to a subagent, use complete messages, which carry `parent_tool_use_id`. See [Detect subagent invocation](/docs/en/agent-sdk/subagents#detect-subagent-invocation).
      ...
      The `event` field contains the raw streaming event from the [Claude API](https://platform.claude.com/docs/en/build-with-claude/streaming#event-types). Common event types include:
      ...
      | Event Type | Description |
      | :-------------------- | :---------------------------------------------- |
      | `message_start` | Start of a new message |
      | `content_block_start` | Start of a new content block (text or tool use) |
      | `content_block_delta` | Incremental update to content |
      | `content_block_stop` | End of a content block |
      | `message_delta` | Message-level updates (stop reason, usage) |
      | `message_stop` | End of the message |
      ...
      Without partial messages enabled (`include_partial_messages` in Python, `includePartialMessages` in TypeScript), you receive all message types except `StreamEvent`. Common types include `SystemMessage` (session initialization), `AssistantMessage` (complete responses), `ResultMessage` (final result), and a compact boundary message indicating when conversation history was compacted (`SDKCompactBoundaryMessage` in TypeScript; `SystemMessage` with subtype `"compact_boundary"` in Python).
      ...
      Tool calls also stream incrementally. You can track when tools start, receive their input as it's generated, and see when they complete. The example below tracks the current tool being called and accumulates the JSON input as it streams in. It uses three event types:
      ...
      * `content_block_start`: tool begins
      * `content_block_delta` with `input_json_delta`: input chunks arrive
      * `content_block_stop`: tool call complete


        ```python Python theme={null}
        from claude_agent_sdk import query, ClaudeAgentOptions
        from claude_agent_sdk.types import StreamEvent
        import asyncio

        async def stream_tool_calls():
            options = ClaudeAgentOptions(
                include_partial_messages=True,
                allowed_tools=["Read", "Bash"],
            )

            # Track the current tool and accumulate its input JSON
            current_tool = None
            tool_input = ""

            async for message in query(prompt="Read the README.md file", options=options):
                if isinstance(message, StreamEvent):
                    event = message.event
                    event_type = event.get("type")

                    if event_type == "content_block_start":
                        # New tool call is starting
                        content_block = event.get("content_block", {})
                        if content_block.get("type") == "tool_use":
                            current_tool = content_block.get("name")
                            tool_input = ""
                            print(f"Starting tool: {current_tool}")

                    elif event_type == "content_block_delta":
                        delta = event.get("delta", {})
                        if delta.get("type") == "input_json_delta":
                            # Accumulate JSON input as it streams in
                            chunk = delta.get("partial_json", "")
                            tool_input += chunk
                            print(f"  Input chunk: {chunk}")

                    elif event_type == "content_block_stop":
                        # Tool call complete - show final input
                        if current_tool:
                            print(f"Tool {current_tool} called with: {tool_input}")
                            current_tool = None

        asyncio.run(stream_tool_calls())
        ```

        ```typescript TypeScript theme={null}
        import { query } from "@anthropic-ai/claude-agent-sdk";

        // Track the current tool and accumulate its input JSON
        let currentTool: string | null = null;
        let toolInput = "";

        for await (const message of query({
          prompt: "Read the README.md file",
          options: {
            includePartialMessages: true,
            allowedTools: ["Read", "Bash"]
          }
        })) {
          if (message.type === "stream_event") {
            const event = message.event;

            if (event.type === "content_block_start") {
              // New tool call is starting
              if (event.content_block.type === "tool_use") {
                currentTool = event.content_block.name;
                toolInput = "";
                console.log(`Starting tool: ${currentTool}`);
              }
            } else if (event.type === "content_block_delta") {
              if (event.delta.type === "input_json_delta") {
                // Accumulate JSON input as it streams in
                const chunk = event.delta.partial_json;
                toolInput += chunk;
                console.log(`  Input chunk: ${chunk}`);
              }
            } else if (event.type === "content_block_stop") {
              // Tool call complete - show final input
              if (currentTool) {
                console.log(`Tool ${currentTool} called with: ${toolInput}`);
                currentTool = null;
              }
            }
          }
        }

      ---

      Title: Claude Code stream-json event cheatsheet — Untether | Little Bear Apps
      URL: https://littlebearapps.com/help/untether/claude-stream-json-cheatsheet/
      Published: 2026-07-23T00:00:00.000Z
      Author: N/A
      Highlights:
      json --input
      ...
      ### system (init)
      ...
      - `type`:`"system"`
      - `subtype`:`"init"`
      - `session_id`
      - `tools`: array of tool names
      - `mcp_servers`: array of`{name, status}`
      - `cwd`,`model`,`permissionMode`,`apiKeySource`(optional)
      ...
      ```
      {"type":"system","subtype":"init","session_id":"session_01","cwd":"/repo","model":"sonnet","permissionMode":"auto","apiKeySource":"env","tools":["Bash","Read","Write","WebSearch"],"mcp_servers":[{"name":"approvals","status":"connected"}]}
      ```

      ---

      Title: Streaming | Claude Code
      URL: https://rhombus.so/streaming.html
      Published: N/A
      Author: N/A
      Highlights:
      When you use the stream-JSON output format (`--output-format stream-json`), Claude Code returns newline-delimited JSON. Each line in the output is a complete JSON object with a`type` field that identifies what kind of message it is. This format makes it easy to parse the output programmatically and react to different events as they occur.
      ...
      | Type | Description |
      | --- | --- |
      | `init` | Session initialization message that includes the session_id, the list of available tools, and the model being used |
      | `user` | User message or tool result being sent to Claude |
      | `assistant` | Claude's response, which may include thinking blocks, text blocks, or tool_use blocks |
      | `result` | Final summary message containing usage statistics and cost information |
      ...
      ## Streaming Events
      ...
      When you use the`--include-partial-messages` flag, you receive granular`stream_event` entries that let you track the progress of Claude's response in real-time. This is useful for building interactive UIs that show typing indicators or stream text as it is generated.
      ...
      | Event | Purpose |
      | --- | --- |
      | `message_start` | Indicates that a new message is beginning |
      | `content_block_start` | A new content block is starting (could be tool_use or text) |
      | `content_block_delta` | Incremental content update (contains input_json_delta or text_delta) |
      | `content_block_stop` | The current content block has finished |
      | `message_delta` | Message-level metadata update (includes stop_reason) |
      | `message_stop` | The entire message has finished |
      ...
      | Feature | CLI stream-json | Session JSONL |
      | --- | --- | --- |
      | Location | stdout | ~/.claude/projects/.../logs/ |
      | Has`stream_event`? | Yes (with --include-partial-messages) | No |
      | Has`init`/`result`? | Yes | No |
      | Sub-agent prompts (parent_tool_use_id) | Yes | No |
      | Field naming | snake_case (tool_use_result) | camelCase (toolUseResult) |

      ---

      Title: docs/reference/runners/claude/stream-json-cheatsheet.md
      URL: https://github.com/littlebearapps/untether/blob/master/docs/reference/runners/claude/stream-json-cheatsheet.md
      Published: N/A
      Author: N/A
      Highlights:
      # Claude Code `stream-json` event cheatsheet
      ...
      `claude -p --output-format stream-json --input-format stream-json --verbose` writes
      **one JSON object per line** (JSONL) with a required `type` field.
      ...
      ### `system` (init)
      ...
      - `type`: `"system"`
      - `subtype`: `"init"`
      - `session_id`
      - `tools`: array of tool names
      - `mcp_servers`: array of `{name, status}`
      - `cwd`, `model`, `permissionMode`, `apiKeySource` (optional)
      ...
      ```json
      {"type":"system","subtype":"init","session_id":"session_01","cwd":"/repo","model":"sonnet","permissionMode":"auto","apiKeySource":"env","tools":["Bash","Read","Write","WebSearch"],"mcp_servers":[{"name":"approvals","status":"connected"}]}
      ```

      ---

      Title: Claude stream-json event cheatsheet - takopi
      URL: https://takopi.dev/reference/runners/claude/stream-json-cheatsheet/
      Published: N/A
      Author: banteg
      Highlights:
      `claude -p --output-format stream-json --verbose` writes one JSON object per line (JSONL) with a required`type` field. (`--output-format` only works with`-p`.)
      ...
      ### system (init)¶
      ...
      Fields: -`type`:`"system"`-`subtype`:`"init"`-`session_id`-`tools`: array of tool names -`mcp_servers`: array of`{name, status}`-`cwd`,`model`,`permissionMode`,`apiKeySource`(optional)
      ...
      ```
      {"type":"system","subtype":"init","session_id":"session_01","cwd":"/repo","model":"sonnet","permissionMode":"auto","apiKeySource":"env","tools":["Bash","Read","Write","WebSearch"],"mcp_servers":[{"name":"approvals","status":"connected"}]}

      ```

      ---

      Title: [DOCS] CLI `--output-format stream-json` lacks event type reference · Issue #24596 · anthropics/claude-code
      URL: https://github.com/anthropics/claude-code/issues/24596
      Published: 2026-02-10T05:44:02.000Z
      Author: coygeek
      Highlights:
      ## [DOCS] CLI `--output-format stream-json` lacks event type reference
      ...
      ```bash
      claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
      jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
      ```
      ...
      And states: "Each line is a JSON object representing an event."
      ...
      The documentation shows how to filter for `text_delta` events but doesn't document what event types exist. A developer trying to build a streaming consumer needs to know:
      ...
      1. **What message types can appear?** The jq example mentions `.type == "stream_event"` but what other top-level types are there? (e.g., `system`, `assistant`, `result`, `user`)
      2. **What stream_event subtypes exist?** The example shows `content_block_delta` with `text_delta`, but what about `message_start`, `content_block_start`, `content_block_stop`, `message_delta`, `message_stop`, `input_json_delta`?
      3. **What's the message flow?** In what order do events appear during a typical response?
      4. **How are tool calls represented?** Do they stream as `input_json_delta` events? How do you know when a tool starts and finishes?
      ...
      The Agent SDK documentation has a thorough StreamEvent reference (platform.claude.com/docs/en/agent-sdk/streaming-output) with an event type table, message flow diagram, and examples for streaming both text and tool calls. The CLI docs should either provide equivalent coverage or explicitly link to the SDK streaming docs.
      ...
      Option A (preferred): Add an "Event types" subsection to the CLI streaming section with:
      ...
      - A table of event types (`system`, `stream_event`, `assistant`, `result`)
      - A table of stream_event subtypes (`message_start`, `content_block_start`, `content_block_delta`, `content_block_stop`, `message_delta`, `message_stop`)
      - The message flow order (same diagram from the SDK streaming-output.md page)
      - An example of streaming tool calls via CLI
      ...
      Option B (partially implemented): The headless page now links to the Agent SDK streaming-output page. However, the link text says "For programmatic streaming with callbacks and message objects" which doesn't signal that it contains the event type reference. A more explicit cross-link would help, e.g., "For the complete event type reference, see [Stream responses in real-time](https://platform.claude.com/docs/en/agent-sdk/streaming-output)."
      ...
      The Agent SDK streaming-output.md page already has excellent documentation for this, including a StreamEvent reference table, message flow diagram, and examples for text streaming, tool call streaming, and building a streaming UI. The CLI docs just need to either replicate or link to this content.
      ...
      **Note:** This issue overlaps with issue 0012 (document all message types emitted by `stream-json`). This issue focuses on the `stream_event` subtypes (which the SDK docs already cover), while issue 0012 covers all top-level message types (`system`, `assistant`, `stream_event`, `result`) which are not documented anywhere. Consider whether both should be submitted or if 0012 subsumes this one, especially since the cross-link to SDK streaming docs partially addresses this issue's minimal fix.

      ---

      Title: [Bug] stream-json system.init session_id is a per-invocation tag in --resume mode (re #8069)
      URL: https://github.com/anthropics/claude-code/issues/58760
      Published: N/A
      Author: N/A
      Highlights:
      # [Bug] stream-json system.init session_id is a per-invocation tag in --resume mode (re #8069)
      ...
      ## [Bug] `stream-json` `system.init.session_id` is a per-invocation tag in `--resume` mode, causing orchestrator retry to fail with "No conversation found"
      ...
      When `claude -p --resume --output-format stream-json` is invoked:
      ...
      - The disk session file (`~/.claude/projects/ /.jsonl`) is correctly appended to throughout the invocation. ✅
      - But the `system.init` event emitted on stdout has `session_id: <NEW_UUID>` where `<NEW_UUID> ≠ X`, AND `<NEW_UUID>` has **no corresponding `.jsonl` file on disk**. ❌
      ...
      For interactive CLI usage this is mostly cosmetic. For orchestrators that integrate `claude -p` as a subprocess and rely on `system.init.session_id` for retry-on-transient-error logic, this is a **silent data-corruption bug**: the tag overwrites the canonical SID in their state store, and subsequent `--resume ` calls fail with `No conversation found`.
      ...
      Start a fresh session, capture the disk SID, then resume and observe `system.init.session_id`:
      ...
      # 1. Fresh session — system.init.session_id == disk SID. ✅
      claude -p "say hi briefly" \
             --output-format stream-json --verbose \
             --permission-mode bypassPermissions 2>/dev/null \
        | jq -r 'select(.type=="system" and .subtype=="init") | .session_id' \
        | head -1
      ...
      3d-
      ...
      cd-8
      ...
      137e3
      ...
      # 2. Resume — system.init.session_id is a brand-new UUID. ❌
      claude -p "what did you just say?" --resume "$SID" \
             --output-format stream-json --verbose \
             --permission-mode bypassPermissions 2>/dev/null \
        | jq -r 'select(.type=="system" and .subtype=="init") | .session_id' \
        | head -1
      # → e.g. 360c5f35-9e0e-4bdb-964c-30cd0ef8de04   (different from $SID)
      ls ~/.claude/projects/$(pwd | sed 's|/|-|g')/360c5f35*.jsonl
      # → no such file ✗
      ls -la ~/.claude/projects/$(pwd | sed 's|/|-|g')/${SID}.jsonl
      # → mtime updated — disk session IS being appended to under $SID ✓
      ```
      ...
      An orchestrator we run drives `claude -p` subprocesses for a multi-phase code-generation pipeline. Per-phase session IDs are persisted in a SQLite column for use as `--resume` targets across phases and for transient-error retry.
      ...
      The integration was built against the **assumption** that `system.init.session_id` is the canonical disk SID (which it is for fresh sessions). Reality is otherwise for `--resume` invocations. On 2026-05-11 a PLAN phase hit a transient `stop_sequence` mid-revision, the retry layer pulled the tag from `system.init.session_id` and issued `claude -p --resume `, claude responded with `No conversation found with session ID: ` (since no `.jsonl` exists), the failure escalated to terminal, and ~1h of accumulated Codex review + revision work was unrecoverable.
      ...
      1. Never trusting `system.init.session_id` in `--resume` calls.
      2. Keeping the SID captured on the **first** (no-`--resume`) invocation as the canonical value, ignoring all later `system.init.session_id` emissions for that conversation.
      3. Re-using the canonical SID for every `--resume` call AND for retry-resume after transient failures.
      ...
      1. **Most surgical**: in `--resume ` mode, emit `session_id: ` (the canonical) in `system.init` instead of a fresh tag. Matches user expectation, fixes #8069 / #12235 / #10806 wholesale.
      2. **Additive**: keep the new tag for diagnostic purposes, but add a sibling field — `original_session_id` or `resumed_from` — on `system.init` so integrators can recover the canonical. This is what [#12235](https://github.com/anthropics/claude-code/issues/12235) and [#10806](https://github.com/anthropics/claude-code/issues/10806) explicitly asked for.
      ...
      3. **Docs-only fallback**: if the current behavior is intentional (e.g. the tag is meaningful for some internal purpose), document it clearly in [Run Claude Code programmatically](https://code.claude.com/docs/en/headless.md#stream-responses) so SDK integrators know not to trust `system.init.session_id` in resume mode.
      ...
      - Claude Code: `2.1.139`
      - Platform: macOS 25.4.0 (darwin), arm64
      - Auth: subscription (Max)
      - Output format: `stream-json --verbose`

      ---

      Title: p --output-format stream-json` · Issue #24612 · anthropics/claude-code
      URL: https://github.com/anthropics/claude-code/issues/24612
      Published: 2026-02-10T05:57:09.000Z
      Author: coygeek
      Highlights:
      ## [FEATURE] Document all message types emitted by `claude -p --output-format stream-json`
      ...
      When using `claude -p --output-format stream-json`, the CLI emits NDJSON messages with various `.type` values. The documentation shows one filtering example (`stream_event` with `text_delta`) but doesn't document the complete set of message types, their schemas, or when they appear.
      ...
      - Build robust parsers that handle all message types
      - Track session state (init, streaming, tool calls, results)
      - Display progress information (token counts, cost, tool execution)
      - Handle errors and edge cases
      ...
      Add a reference page or section documenting all message types emitted by `claude -p --output-format stream-json`. For each type, document:
      ...
      1. The `.type` value (e.g., `system`, `assistant`, `stream_event`, `result`)
      2. The `.subtype` value if applicable (e.g., `init`, `compact_boundary`)
      3. The JSON schema / key fields
      4. When it appears in the message flow
      5. Example JSON
      ...
      The Agent SDK's streaming-output.md already documents `StreamEvent` types thoroughly. The CLI docs need equivalent coverage for the full set of top-level message types (which includes types beyond just `StreamEvent`).
      ...
      A CI/CD pipeline runs `claude -p --output-format stream-json` and needs to:
      ...
      1. Parse the `system` message with `subtype: init` to get the `session_id`
      2. Monitor `stream_event` messages for progress
      3. Detect tool calls and log them
      4. Extract the final `result` message for the pipeline output
      5. Track token usage from `message_delta` events
      ...
      Currently, the developer has to guess which message types exist and what fields they contain. A reference would let them write a correct parser upfront.
      ...
      The Vibe Companion project's `session-types.ts` file (232 lines) is essentially a community-authored type reference for these message types, reverse-engineered from the CLI output. This demonstrates both the demand for this documentation and the feasibility of providing it -- the types are stable enough to document.
      ...
      The Agent SDK's TypeScript SDK reference (platform.claude.com/docs/en/agent-sdk/typescript) documents some of these types but in the context of the SDK library, not the CLI output format.
      ...
      Closing as duplicate of #
      ...
      4596, which covers
      ...
      same request (documenting `--output-format stream-json` message types) as a documentation issue.

      ---

      Title: udhaykumarbala/claude-code-parser
      URL: https://github.com/udhaykumarbala/claude-code-parser
      Published: 2026-03-21T19:31:14.000Z
      Author: N/A
      Highlights:
      Parse Claude Code's --output-format stream-json into fully typed TypeScript events. Zero dependencies.
      ...
      Parse Claude Code's `--output-format stream-json` NDJSON output into fully typed TypeScript events. Zero dependencies. 9 kB.
      ...
      > Includes the first standalone documentation of Claude Code's undocumented `stream-json` protocol.
      ...
      -parser'
      ...
      const claude = spawn('claude', [
        '-p', '--input-format', 'stream-json',
        '--output-format', 'stream-json', '--verbose',
      ], { stdio: ['pipe', 'pipe', 'inherit'] })
      ...
      const rl = createInterface({ input: claude.stdout! })
      ...
      rl.on('line', (line) => {
        const event = parseLine(line)
        if (!event) return

        for (const relay of translator.translate(event)) {
          switch (relay.type) {
            case 'text_delta':    process.stdout.write(relay.content); break
            case 'tool_use':      console.log(`\n[tool] ${relay.toolName}`); break
            case 'turn_complete': console.log(`\n[done] $${relay.costUsd?.toFixed(4)}`); break
          }
        }
      })
      ...
      | Export | Description |
      | --- | --- |
      | `parseLine(line)` | NDJSON line → `ClaudeEvent \ | null` |
      | `Translator` | Stateful dedup translator → `RelayEvent[]` |
      | `createMessage.user(text)` | Construct stdin user message |
      | `createMessage.approve(id)` | Approve pending tool execution |
      | `createMessage.deny(id)` | Deny pending tool execution |
      | `createMessage.toolResult(id, content)` | Send tool result |
      | `extractContent(raw)` | Normalize polymorphic tool_result content |
      ...
      _complete`
      ...
      The official SDK (`@anthropic-ai/claude-code`) couples parsing with subprocess management. This library **only parses** — for developers building custom relays, dashboards, CI tools, or browser viewers who need raw event access.
      ...
      ](https://udhaykumarbala.github.io/claude-code-parser/guide/getting-started)
      - [API Reference](https://udhaykumarbala.github.io/claude-code-parser/
      ...
      /api)
      - [
      ...
      ](https://udhaykumarbala
      ...
      io/cla
      ...
      parser/guide/
      ...
      — WS relay, log
      ...
      CI tracker, tool approval
      - [Protocol Overview](https://udhaykumarbala.github.io/claude-code-parser/protocol/overview)
      - [Output Events](https://udhaykumarbala.github.io/claude-code-parser/protocol/output-events) — Full event catalog
      - [Input Messages](https://udhaykumarbala.github.io/claude-code-parser/protocol/input-messages) — Undocumented stdin protocol
      - [Deduplication](https://udhaykumarbala.github.io/claude-code-parser/protocol/deduplication)
      - [Multi-Agent](https://udhaykumarbala.github.io/claude-code-parser/protocol/multi-agent)
      - [Gotchas](https://udhaykumarbala.github
      ...
      io/cla
      ...
      -code-parser/protocol/gotchas) — Double encoding, polymorphic content, thinking field names
```
/answer bf102198-c6af-46d5-97ab-60d255925559 allow
Control Request:
```yaml
id: 56d9d579-cc7c-4836-bddb-ca5c34f812c0
toolName: context7-libraries
arguments:
  libraryName: claude code
  query: system init event
  reason: Find Claude Code SDK docs library id
```
/answer 56d9d579-cc7c-4836-bddb-ca5c34f812c0 allow
Control Request:
```yaml
id: f91493a3-ed2d-4963-b2ab-27ea7043aae0
result:
  structuredContent:
    content: |-
      Available Libraries:

      - Title: Claude Code
      - Context7-compatible library ID: /anthropics/claude-code
      - Description: Claude Code is an agentic terminal tool that understands your codebase to help you code faster by executing tasks, explaining code, and handling git workflows through natural language commands.
      - Code Snippets: 2548
      - Source Reputation: High
      - Benchmark Score: 64.9
      - Versions: v2.1.39, v2.1.89
      ----------
      - Title: Claude Code
      - Context7-compatible library ID: /websites/code_claude
      - Description: Claude Code is Anthropic’s agentic coding tool that operates in your terminal, helping developers build features, debug issues, navigate codebases, and automate tasks to turn ideas into code faster.
      - Code Snippets: 10427
      - Source Reputation: High
      - Benchmark Score: 82.24
      ----------
      - Title: Claude Code
      - Context7-compatible library ID: /llmstxt/code_claude_llms_txt
      - Description: Claude Code is an agentic coding tool that lives in your terminal and helps you turn ideas into code faster by automating development tasks across various IDEs and deployment environments.
      - Code Snippets: 10858
      - Source Reputation: High
      - Benchmark Score: 86.7
      ----------
      - Title: Claude Code
      - Context7-compatible library ID: /zebbern/claude-code-guide
      - Description: Claude Code is an AI-powered command-line interface that brings Claude's capabilities to your terminal, enabling code analysis, editing, automation, and development workflows with advanced features like sub-agents, MCP integration, and hooks.
      - Code Snippets: 9691
      - Source Reputation: High
      - Benchmark Score: 78.48
      ----------
      - Title: Claude Code
      - Context7-compatible library ID: /johnlindquist/claude
      - Description: Claude Code is an AI-powered development environment that enables developers to build, analyze, and refactor software with intelligent agent assistance and automated workflows.
      - Code Snippets: 2787
      - Source Reputation: High
      - Benchmark Score: 66.7
```
/answer f91493a3-ed2d-4963-b2ab-27ea7043aae0 allow
Control Request:
```yaml
id: b4ef1dd0-08e4-4ece-a974-08b5ec3ec5d3
toolName: context7-documentation
arguments:
  libraryId: /websites/code_claude
  query: system init message SDKSystemMessage subtype init when emitted first message session_id
  reason: Get official docs on system/init event structure and timing
```
/answer b4ef1dd0-08e4-4ece-a974-08b5ec3ec5d3 allow
ReasoningToken: 72
Control Request:
```yaml
id: 8282143a-a2ff-414f-b3b3-a9aae5fbb119
result:
  structuredContent:
    content: |-
      ### Define SDKSystemMessage Type in TypeScript

      Source: https://code.claude.com/docs/en/agent-sdk/typescript

      Represents the system initialization message, providing details about the SDK environment, capabilities, and configuration.

      ```typescript
      type SDKSystemMessage = {
        type: "system";
        subtype: "init";
        uuid: UUID;
        session_id: string;
        agents?: string[];
        apiKeySource: ApiKeySource;
        betas?: string[];
        claude_code_version: string;
        cwd: string;
        tools: string[];
        mcp_servers: {
          name: string;
          status: string;
        }[];
        model: string;
        permissionMode: PermissionMode;
        slash_commands: string[];
        output_style: string;
        skills: string[];
        plugins: { name: string; path: string }[];
        capabilities?: string[];
      };
      ```

      --------------------------------

      ### Discover Available MCP Tools from System Init Message

      Source: https://code.claude.com/docs/en/agent-sdk/mcp

      Iterate through `query` messages and filter the `tools` array in the `system` init message to identify and print MCP tools, which are prefixed with `mcp__`.

      ```TypeScript
      import { query } from "@anthropic-ai/claude-agent-sdk";

      const options = {
        mcpServers: {
          // your servers
        },
      };

      for await (const message of query({ prompt: "...", options })) {
        if (message.type === "system" && message.subtype === "init") {
          const mcpTools = message.tools.filter((name) => name.startsWith("mcp__"));
          console.log("Available MCP tools:", mcpTools);
        }
      }
      ```

      ```Python
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


      async def main():
          options = ClaudeAgentOptions(
              mcp_servers={
                  # your servers
              },
          )
          async for message in query(prompt="...", options=options):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  mcp_tools = [t for t in message.data.get("tools", []) if t.startswith("mcp__")]
                  print("Available MCP tools:", mcp_tools)


      asyncio.run(main())
      ```

      ### Message types > SystemMessage

      Source: https://code.claude.com/docs/en/agent-sdk/agent-loop

      SystemMessage types represent session lifecycle events, distinguished by a 'subtype' field. These include 'init' for session metadata, 'compact_boundary' after compaction, 'informational' for plain-text status, and 'worker_shutting_down' when the loop is ending due to host exit or Remote Control disconnection.

      --------------------------------

      ### Connection timing

      Source: https://code.claude.com/docs/en/agent-sdk/mcp

      The `system` message with subtype `init` reports each server's connection status, which can be `pending` for connecting servers, `failed`, or `needs-auth`. It is important to check for `failed` or `needs-auth` statuses to identify unusable servers.

      --------------------------------

      ### Error handling

      Source: https://code.claude.com/docs/en/agent-sdk/mcp

      MCP servers can fail to connect due to issues like the server process not being installed, invalid credentials, or network unreachability. The SDK emits a 'system' message with subtype 'init' at the start of each query, which includes the connection status for each MCP server. The 'status' field can be 'pending', 'connected', 'failed', 'needs-auth', or 'disabled'. It's important to check for 'failed' or 'needs-auth' to identify unusable servers, and not to treat 'pending' as a failure since connection is non-blocking.
```
/answer 8282143a-a2ff-414f-b3b3-a9aae5fbb119 allow
## Was ist das `system`/`init`-Event?

In Claude Code (sowohl im CLI-Modus `--output-format stream-json` als auch im Agent SDK für TypeScript/Python) ist `system`/`init` das **allererste strukturierte Event einer neuen Query/Session**. Es signalisiert, dass die Session initialisiert wurde, und liefert Metadaten über die Umgebung, bevor irgendein eigentlicher Assistant-Content folgt.

### Struktur (TypeScript – `SDKSystemMessage`)

```typescript
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: { name: string; status: string }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  capabilities?: string[];
};
```

Im CLI-JSONL-Stream sieht die Zeile z. B. so aus:
```json
{"type":"system","subtype":"init","session_id":"session_01","cwd":"/repo","model":"sonnet","permissionMode":"auto","apiKeySource":"env","tools":["Bash","Read","Write","WebSearch"],"mcp_servers":[{"name":"approvals","status":"connected"}]}
```

### Wann genau wird es emittiert?

1. **Einmal zu Beginn jeder neuen Query/Session** – es ist das erste Message-Objekt, das im Stream ankommt, noch vor dem ersten `assistant`-Message. Es markiert den Abschluss der Session-Initialisierung (Tools geladen, MCP-Server-Verbindungsversuche gestartet, Konfiguration angewendet).
2. **MCP-Server-Status ist zu diesem Zeitpunkt teilweise noch "pending"** – die Verbindung zu MCP-Servern ist nicht blockierend. D.h. `init` wartet nicht, bis alle MCP-Server fertig verbunden sind; der `status` je Server kann `pending`, `connected`, `failed`, `needs-auth` oder `disabled` sein.
3. **Bei `--resume`-Sessions** wird ebenfalls ein `init`-Event pro Invocation emittiert – hier gibt es aber einen bekannten Bug (Issue #58760): die `session_id` im `init`-Event ist bei `--resume` eine *neue*, pro-Aufruf generierte UUID, die **nicht** mit der kanonischen Session-ID der `.jsonl`-Datei auf der Platte übereinstimmt. Für Orchestrierungs-Tools, die sich auf `session_id` aus dem `init`-Event verlassen, ist das gefährlich (führt zu "No conversation found"-Fehlern bei Retries).
4. **Bei Plugin-Fehlern**: Seit Version 2.1.111 kann das `init`-Event zusätzlich ein Feld `plugin_errors` enthalten, wenn Plugins wegen nicht erfüllter Abhängigkeiten "demoted" (herabgestuft) wurden – dieses Feld ist aktuell in den offiziellen TypeScript/Python-Referenzen noch nicht dokumentiert (offenes Doku-Issue #49308).
5. Es gibt kein wiederholtes `init` innerhalb derselben laufenden Session – nur zu Sessionstart. Nach einem Context-Compaction-Vorgang wird stattdessen ein separates `system`-Event mit `subtype: "compact_boundary"` gesendet (kein erneutes `init`).

### Zweck / Nutzung
Konsumenten (SDK-Clients, Dashboards, CI-Pipelines) parsen das `init`-Event typischerweise, um:
- die `session_id` für spätere `--resume`-Aufrufe zu erfassen (mit oben genannter Vorsicht),
- verfügbare Tools/MCP-Tools (Präfix `mcp__`) zu ermitteln,
- den Verbindungsstatus von MCP-Servern zu prüfen (`failed`/`needs-auth` behandeln, `pending` nicht als Fehler werten),
- geladene Plugins, Slash-Commands, Skills und den aktiven Output-Style/Model/PermissionMode zu protokollieren.

Quellen: offizielle Claude Code Docs (code.claude.com/docs/en/agent-sdk/{typescript,mcp,agent-loop}), sowie aktuelle GitHub-Issues im `anthropics/claude-code`-Repo (#49308, #58760, #24596, #24612) zu offenen Doku-Lücken und Bugs rund um dieses Event.
Hm, mir scheint es so das system/init mehrfach während eienr Session jeweil bei einem neuen Prompt ausgelsöt werden kann. Das mag mit dem internen handling von Claude Code zutun haben.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=c3eafc62-71c7-4066-9208-5b0d2123eeab | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 2ce7335d-121a-4a09-96c2-74cc100e9ee6
toolName: web-search-exa
arguments:
  query: Claude Code system init event fired multiple times per session each new prompt stream-json
  reason: Verify claim that init event repeats per new prompt within a session
```
/answer 2ce7335d-121a-4a09-96c2-74cc100e9ee6 allow
ReasoningToken: 82
Control Request:
```yaml
id: d3d799ae-df40-48fc-883e-221980569470
result:
  structuredContent:
    content: |-
      Title: [Bug] stream-json system.init session_id is a per-invocation tag in --resume mode (re #8069)
      URL: https://github.com/anthropics/claude-code/issues/58760
      Published: N/A
      Author: N/A
      Highlights:
      # [Bug] stream-json system.init session_id is a per-invocation tag in --resume mode (re #8069)
      ...
      ## [Bug] `stream-json` `system.init.session_id` is a per-invocation tag in `--resume` mode, causing orchestrator retry to fail with "No conversation found"
      ...
      When `claude -p --resume --output-format stream-json` is invoked:
      ...
      - The disk session file (`~/.claude/projects/ /.jsonl`) is correctly appended to throughout the invocation. ✅
      - But the `system.init` event emitted on stdout has `session_id: <NEW_UUID>` where `<NEW_UUID> ≠ X`, AND `<NEW_UUID>` has **no corresponding `.jsonl` file on disk**. ❌
      ...
      For interactive CLI usage this is mostly cosmetic. For orchestrators that integrate `claude -p` as a subprocess and rely on `system.init.session_id` for retry-on-transient-error logic, this is a **silent data-corruption bug**: the tag overwrites the canonical SID in their state store, and subsequent `--resume ` calls fail with `No conversation found`.
      ...
      Start a fresh session, capture the disk SID, then resume and observe `system.init.session_id`:
      ...
      # 2. Resume — system.init.session_id is a brand-new UUID. ❌
      claude -p "what did you just say?" --resume "$SID" \
             --output-format stream-json --verbose \
             --permission-mode bypassPermissions 2>/dev/null \
        | jq -r 'select(.type=="system" and .subtype=="init") | .session_id' \
        | head -1
      ...
      # → e.g. 360c5f35-9e0e-4bdb-964c-30cd0ef8de04   (different from $SID)
      ...
      ls ~/.claude/projects/$(pwd | sed 's|/|-|g')/360c5f35*.jsonl
      ...
      # → no such file ✗
      ...
      ls -la ~/.claude/projects/$(pwd | sed 's|/|-|g')/${SID}.jsonl
      ...
      An orchestrator we run drives `claude -p` subprocesses for a multi-phase code-generation pipeline. Per-phase session IDs are persisted in a SQLite column for use as `--resume` targets across phases and for transient-error retry.
      ...
      The integration was built against the **assumption** that `system.init.session_id` is the canonical disk SID (which it is for fresh sessions). Reality is otherwise for `--resume` invocations. On 2026-05-11 a PLAN phase hit a transient `stop_sequence` mid-revision, the retry layer pulled the tag from `system.init.session_id` and issued `claude -p --resume `, claude responded with `No conversation found with session ID: ` (since no `.jsonl` exists), the failure escalated to terminal, and ~1h of accumulated Codex review + revision work was unrecoverable.
      ...
      We've worked around it by:

      1. Never trusting `system.init.session_id` in `--resume` calls.
      2. Keeping the SID captured on the **first** (no-`--resume`) invocation as the canonical value, ignoring all later `system.init.session_id` emissions for that conversation.
      3. Re-using the canonical SID for every `--resume` call AND for retry-resume after transient failures.
      ...
      Code reference: [selfcoding/backend/app/services/claude_service.py @ 8b524c5](https://github.com/jingmca/selfcoding/blob/8b524c5/backend/app/services/claude_service.py#L213-L227) — the gate `if on_session_start is not None and _current_resume is None:` is the necessary workaround.
      ...
      1. **Most surgical**: in `--resume ` mode, emit `session_id: ` (the canonical) in `system.init` instead of a fresh tag. Matches user expectation, fixes #8069 / #12235 / #10806 wholesale.
      2. **Additive**: keep the new tag for diagnostic purposes, but add a sibling field — `original_session_id` or `resumed_from` — on `system.init` so integrators can recover the canonical. This is what [#12235](https://github.com/anthropics/claude-code/issues/12235) and [#10806](https://github.com/anthropics/claude-code/issues/10806) explicitly asked for.
      ...
      3. **Docs-only fallback**: if the current behavior is intentional (e.g. the tag is meaningful for some internal purpose), document it clearly in [Run Claude Code programmatically](https://code.claude.com/docs/en/headless.md#stream-responses) so SDK integrators know not to trust `system.init.session_id` in resume mode.

      ---

      Title: Streaming & Real-Time Output - Agent Mastered
      URL: https://agentmastered.com/extending-claude/stream-protocol/
      Published: N/A
      Author: N/A
      Highlights:
      Two flags are required to activate the stream protocol:`--output-format stream-json` sets the output mode, and`--verbose` ensures the init event with full session metadata is emitted. Without`--verbose`, the init event may be missing — always pair the two together.
      ...
      Each line in the output is a complete, independent JSON object. A simple prompt without tool use produces exactly four events:`system`(init),`assistant`(response),`rate_limit_event`(API status), and`result`(final envelope).
      ...
      produce 5
      ...
      started_working`→`output
      ...
      , retries
      ...
      | Event Type | Subtype / Trigger | When It Fires | Key Fields |
      | --- | --- | --- | --- |
      | `system` | `init` | First event in every stream | `tools`,`mcp_servers`,`model`,`session_id`,`permissionMode` |
      | `system` | `api_retry` | On retryable API error (rate limit, server error) | `attempt`,`max_retries`,`retry_delay_ms`,`error` |
      | `assistant` | — | After the model generates a response | `message.content[]`,`message.usage` |
      | `user` | — | After a tool executes and returns results | Tool result content |
      | `rate_limit_event` | — | After each API call (always fires, even when allowed) | `rate_limit_info.status`,`resetsAt` |
      | `result` | `success` or`error_*` | Last event in the stream | Same fields as`—output-format json` envelope |
      | `stream_event` | `message_start`,`content_block_delta`,`message_stop`, … | Only with`—include-partial-messages` | `event.delta.text`— incremental token chunks |
      ...
      Notice the first event is`init` with the session ID, and subsequent events are`assistant` with text chunks. Try piping through`jq -r ‘select(.type==“assistant”) | .content’` to extract just the text.
      ...
      The first event in every stream is a`system/init` payload containing session metadata. This is the richest event in the protocol and the key to initializing any UI or monitoring system.
      ...
      Use the init event to
      ...
      health checks (
      ...
      `claude_code_
      ...
      tool lists and model
      ...
      Here is the complete four-event stream from a real CLI call. This is exactly what`--output-format stream-json --verbose` produces for a simple prompt with no tool use.
      ...
      1Init event — always first, contains session metadata
      ...
      each of these four objects would
      ...
      on its own line
      ...
      no commas or brackets between them
      ...
      nested structure shown above groups them for readability — the actual NDJSON output
      ...
      one JSON object per line
      ...
      process';import
      ...
      output-format',
      ...
      `—verbose` is required with`—output-format stream-json`. Without it, the init event with session metadata may not be emitted. Always use both flags together:`—output-format stream-json —verbose`.
      ...
      event is always last and
      ...
      the same data as`—
      ...
      format json`. You get both real-time streaming events and
      ...
      final summary envelope in one stream —
      ...
      run a second call
      ...
      Run`claude -p “Explain sessions in one paragraph” —output-format stream-json 2>/dev/null | head -5` to see your first NDJSON events. Notice the`init` event contains the session ID before any text arrives — you can start tracking the session before Claude even begins responding.

      ---

      Title: docs/claude-cli.md
      URL: https://github.com/takelushi/claude-code-rs/blob/de23ac359c3bd246e6b2573859f24151a3f9f61c/docs/claude-cli.md
      Published: N/A
      Author: N/A
      Highlights:
      ## stream-json Event Types
      ...
      Events output as NDJSON via `--output-format stream-json --verbose`:
      ...
      | type | subtype / content type | Description |
      | --- | --- | --- |
      | `system` | `init` | Session initialization info (session_id, model, etc.) |
      | `system` | `hook_started` / `hook_response` | Hook execution (skipped by the library) |
      | `assistant` | content[].type = `thinking` | Model's thinking process |
      | `assistant` | content[].type = `text` | Text response chunk |
      | `assistant` | content[].type = `tool_use` | Tool invocation |
      | `user` | content[].type = `tool_result` | Tool execution result |
      | `rate_limit_event` | — | Rate limit information |
      | `result` | `success` | Final result (same structure as `--output-format json`) |
      | `stream_event` | (various, see below) | Anthropic Messages API SSE events (only with `--include-partial-messages`) |
      ...
      content[]` may
      ...
      multiple blocks.
      ...
      StreamEvent`.
      ...
      When `--include-partial-messages` is enabled, both `assistant` events (complete messages) and `stream_event` events (token-level chunks) are sent. Since the same text arrives twice, the library distinguishes them as follows:
      ...
      | CLI Option | Builder Method |
      | --- | --- |
      | `--model` | `model()` |
      | `--system-prompt` | `system_prompt()` |
      | `--append-system-prompt` | `append_system_prompt()` |
      | `--max-turns` | `max_turns()` |
      | `--fallback-model` | `fallback_model()` |
      | `--effort` | `effort()` |
      | `--max-budget-usd` | `max_budget_usd()` |
      | `--allowedTools` | `allowed_tools()` / `add_allowed_tool()` |
      | `--disallowedTools` | `disallowed_tools()` / `add_disallowed_tool()` |
      | `--tools` | `tools()` |
      | `--mcp-config` | `mcp_configs()` / `add_mcp_config()` |
      | `--
      ...
      sources` | `setting_sources()` |
      |
      ...
      settings` | `settings()` |
      | `--json-schema` | `json_schema()` |
      |
      ...
      -messages` | `include_partial_messages()` |
      | `--include-hook-events` | `include_hook_events()` |
      | `--permission-mode` | `permission_mode()` |
      | `--dangerously-skip-permissions` | `dangerously_skip_permissions()` |
      | `--add-dir` | `add_dirs()` / `add_dir()` |
      | `--file` | `files()` / `file()` |
      |
      ...
      resume` |
      ...
      resume()` |
      ...
      | `--session-id` | `session_id()` |
      ...
      | `--bare` | `bare()` |
      | `--
      ...
      -session-persistence` | `no_session_persistence()` |
      | `--disable-slash-commands` | `disable_slash_commands()` |
      | `--strict-mcp-config` | `strict_mcp_config()` |
      ...
      | CLI Option | Description |
      | --- | --- |
      | `--agent` | Agent for the current session |
      | `--agents` | JSON object defining custom agents |
      | `--betas` | Beta headers for API requests |
      | `--continue` | Continue most recent conversation |
      | `--fork-session` | Create new session ID when resuming |
      | `--input-format` | Input format (`text` or `stream-json`) |
      | `--name` | Session display name |
      | `--allow-dangerously-skip-permissions` | Enable permission bypass as an option |
      | `--verbose` | Explicit verbose mode (auto-added for stream-json) |
      | `--debug` | Enable debug mode with optional category filtering |
      | `--debug-file` | Write debug logs to a specific file path |
      ...
      | CLI Option | When Applied |
      | --- | --- |
      | `--print` | Always |
      | `--output-format` | Always (`json` for `ask`, `stream-json` for `ask_stream`) |
      | `--verbose` | Automatically added when using `ask_stream` |

      ---

      Title: Missing Final Result Event in Streaming JSON Output with sdk · Issue #1920 · anthropics/claude-code
      URL: https://github.com/anthropics/claude-code/issues/1920
      Published: N/A
      Author: allisoneer
      Highlights:
      Claude Code CLI fails to send the required final `{"type":"result",...}` event in streaming JSON mode after successful tool execution, causing the process to hang indefinitely despite completing the conversation functionally.
      ...
      ://docs.anthropic
      ...
      > "Each conversation begins with an initial `init` system message, followed by a list of user and assistant messages, **followed by a final `result` system message** with stats."
      ...
      ```json
      {"type":"system","subtype":"init","session_id":"...","tools":[...]}
      {"type":"assistant","message":{"content":[{"type":"text","text":"..."}]}}
      {"type":"assistant","message":{"content":[{"type":"tool_use","id":"...","name":"TodoWrite",...}]}}
      {"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"...","content":"..."}]}}
      {"type":"result","subtype":"success","session_id":"...","cost_usd":2.90831585,"num_turns":62,...}
      ```
      ...
      ```json
      {"type":"system","subtype":"init","session_id":"...","tools":[...]}
      {"type":"assistant","message":{"content":[{"type":"text","text":"..."}]}}
      {"type":"assistant","message":{"content":[{"type":"tool_use","id":"...","name":"TodoWrite",...}]}}
      {"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"...","content":"..."}]}}
      // <-- MISSING: No {"type":"result",...} event, process hangs here
      ```
      ...
      The SDK properly waits for Claude Code to:
      ...
      1. Send the final `result` event
      2. Close stdout
      3. Exit the process
      ...
      Since Claude Code never sends step 1, steps 2-3 never happen.
      ...
      > Is this still occurring? The execution loop should always send the final result. One thing that can happen is that a prompt input may be handled as part of an ongoing conversation. But if you get a system/init message you should always get a corresponding result.

      ---

      Title: research/phase-4-stream-json-shape.md
      URL: https://github.com/traviswye/ContextAtlas-benchmarks/blob/main/research/phase-4-stream-json-shape.md
      Published: N/A
      Author: N/A
      Highlights:
      ### 1. `{ type: "system", subtype: "init" }`
      ...
      First event. Contains session-setup metadata:
      ...
      ```json
      {
        "type": "system",
        "subtype": "init",
        "cwd": "<abs path>",
        "session_id": "43eb3b01-...",
        "tools": ["Bash", "Edit", "Read"],   // Claude Code's default tool roster visible to the model
        "mcp_servers": [],                   // our --mcp-config contents surface here
        "model": "claude-opus-4-7",
        "permissionMode": "default",
        "slash_commands": [...],
        "apiKeySource": "none",              // will be "env" or similar when auth works
        "claude_code_version": "2.1.117",    // ← pin this per run
        "output_style": "default",
        "agents": [...],
        "skills": [...],
        "plugins": [],
        "uuid": "<message uuid>",
        "fast_mode_state": "off"
      }
      ...
      Parser action: record `claude_code_version`, `tools`,
      `mcp_servers`. The driver can stamp these into the RunRecord or
      a sidecar diagnostic file; they're not load-bearing for metrics
      but they catch reproducibility drift.
      ...
      the `tools`
      ...
      reflects `--bare`'
      ...
      under `--bare` the
      ...
      than a
      full-configured Claude Code session.
      ...
      ## Schema vs Phase 0 assumptions
      ...
      | Phase 0 assumed | Reality |
      |---|---|
      | `message_delta` events with per-turn usage | ❌ Doesn't exist. Usage lives on `assistant.message.usage` events (full message, not deltas) |
      | `message_stop` terminal event | ❌ Terminal event is `result`; no `message_stop` |
      | `--include-hook-events` gives PreToolUse/PostToolUse | Not observed (no tool use in this probe). Assumption carried forward; confirm on real run |
      | `tool_use` / `tool_result` content blocks are standard Anthropic shapes | Assumed; not confirmed on this probe. High confidence the shape matches |
      | Process exit is the "done" signal | Confirmed: after `result` event, process exits |
      ...
      Based on this, the driver needs to dispatch on `event.type`
      with these handlers (revised from Phase 0 assumptions):
      ...
      ```
      system      → record init metadata (one-time)
      assistant   → accumulate usage; walk content for tool_use / text
      user        → walk content for tool_result; fill in trace previews
      result      → terminal; cross-check aggregates; finalize record
      ```
      ...
      No `message_delta` / `message_stop` handling. Simpler than
      Phase 0 sketched, because Claude Code emits full-message events
      rather than delta events.

      ---

      Title: [BUG] --resume in REPL mode fires duplicate SessionStart hook with wrong session_id (spurious startup event) · Issue #23932 · anthropics/claude-code
      URL: https://github.com/anthropics/claude-code/issues/23932
      Published: 2026-02-07T12:46:05.000Z
      Author: alvarolb
      Highlights:
      ## [BUG] --resume in REPL mode fires duplicate SessionStart hook with wrong session_id (spurious startup event)
      ...
      When using `claude --resume ` in interactive REPL mode, **two** `SessionStart` hook events are fired instead of one. One has the correct `session_id` and `source: "resume"`, but the other has a **new random UUID** and `source: "startup"`, with a `transcript_path` pointing to a nonexistent/empty file.
      ...
      integrations that use hooks to track sessions
      ...
      **Two** SessionStart events arrive (same timestamp):
      ...
      ```json
      {
          "session_id": "431cad5d-8eed-4182-9ed9-2c9fddd569e5",
          "transcript_path": "...431cad5d-8eed-4182-9ed9-2c9fddd569e5.jsonl",
          "hook_event_name": "SessionStart",
          "source": "resume"
      }
      ```
      ...
      ```json
      {
          "session_id": "42345f1a-34b2-4808-9f8b-9908b5a52d0d",
          "transcript_path": "...42345f1a-34b2-4808-9f8b-9908b5a52d0d.jsonl",
          "hook_event_name": "SessionStart",
          "source": "startup",
          "model": "claude-opus-4-6"
      }
      ```
      ...
      - The `startup` event has a **random new UUID** (the process's internal session_id) and its `transcript_path` points to an empty/nonexistent file.
      - The arrival **order is non-deterministic** — sometimes `startup` arrives first, sometimes `resume`.
      - All subsequent hooks (UserPromptSubmit, Stop, etc.) correctly use the original session_id.
      - The transcript file is also correctly written to the original session's file.
      ...
      Only **one** SessionStart event should fire with `source: "resume"` and the original `session_id`. The spurious `startup` event should not be emitted during `--resume`.
      ...
      In `cli.js`, the `lj("startup")` call (which fires `SessionStart` with `source: "startup"`) is guarded by:
      ...
      ```js
      let gq = p || B || r || _1 ? null : lj("startup", { agentType: ..., model: ... });
      ```
      ...
      Where `p = initOnly`, `B = init`, `r = maintenance`, `_1 = !isInteractive`.
      ...
      **The `--resume` and `--continue` flags are not included in this guard condition**, so `lj("startup")` fires as a background promise even during resume. Meanwhile, `lj("resume")` is correctly called inside the `is()` function with the proper session_id.
      ...
      The fix would be adding the resume/continue flags to the guard:
      ...
      ```js
      let gq = p || B || r || _1 || H.resume || H.continue ? null : lj("startup", { agentType: ..., model: ... });
      ```
      ...
      - Integrations that capture `session_id` from the first SessionStart event may capture the wrong (ephemeral) ID
      - The spurious `transcript_path` points to an empty file, breaking transcript watchers
      - Non-deterministic arrival order makes it impossible to reliably distinguish the correct event without inspecting the `source` field
      ...
      This does **not** affect `--print` mode — `claude -p --resume ` correctly fires only one SessionStart with the original session_id.

      ---

      Title: [BUG] Duplicate entries in session .jsonl files when using stream-json input format · Issue #5034 · anthropics/claude-code
      URL: https://github.com/anthropics/claude-code/issues/5034
      Published: 2025-08-03T10:21:46.000Z
      Author: Lewik
      Highlights:
      When using `--input-format stream-json` for multi-turn conversations, Claude Code writes duplicate entries to session .jsonl files. Each subsequent message in a conversation causes the previous conversation history to be duplicated in the session file.
      ...
      The session file contains 6+ entries with duplicates:
      ...
      - User message 1 + Assistant response 1 (original)
      - User message 1 + Assistant response 1 (duplicate)
      - User message 2 + Assistant response 2
      ...
      Each new message causes all previous entries to be duplicated, leading to exponential growth of session file size.
      ...
      - The issue does NOT occur with regular interactive mode (without `--input-format stream-json`)
      - Resume functionality works correctly despite the duplicates
      - Context is maintained properly - this is purely a logging/persistence issue
      - Related to issues #3187 (stream-json hang) and #1920 (missing final result event)
      ...
      This appears to be caused by Claude Code rewriting/appending the entire conversation history to the session file each time a new message is processed, rather than appending only the new entries.
      ...
      entries based on message IDs and UUIDs when reading conversation history.
      ...
      > This happens for me with regular interactive mode. When I look at the session .jsonl file, each message is duplicated. It's ALWAYS been this way for me. It makes my session vanish so quickly.

      ---

      Title: [BUG] SessionStart hooks invoked twice per session causing race conditions · Issue #14937 · anthropics/claude-code
      URL: https://github.com/anthropics/claude-code/issues/14937
      Published: 2025-12-21T03:19:37.000Z
      Author: coygeek
      Highlights:
      ## [BUG] SessionStart hooks invoked twice per session causing race conditions
      ...
      Claude Code is invoking SessionStart hooks **twice per session start**, with invocations occurring 82ms-1400ms apart. This causes race conditions in hooks that read-modify-write files, as both invocations run concurrently and see stale data.
      ...
      SessionStart hooks should be invoked exactly **once** per session start. A single, atomic invocation would prevent race conditions and duplicate work.
      ...
      ```json
      {
        "hooks": {
          "SessionStart": [
            {
              "hooks": [
                {
                  "type": "command",
                  "command": "python3 /tmp/invocation_logger.py",
                  "timeout": 5
                }
              ]
            }
          ]
        }
      }
      ```
      ...
      **Expected**: One log entry per session start
      ...
      **Actual**: Two log entries with different PIDs, ~100-1400ms apart
      ...
      . **Race conditions**:
      ...
      that read-modify-write files see stale data from each other
      ...
      The two invocations appear to be from separate processes (different PIDs based on timing gaps). This suggests Claude Code may be spawning hook execution twice, possibly from:
      ...
      - Multiple initialization paths (UI + backend?)
      - Retry logic that doesn't check if first invocation succeeded
      - Parallel component initialization
      ...
      > **# UPDATE:**
      >
      > # Issue #14937 Analysis: SessionStart hooks invoked twice
      >
      > ## Summary
      >
      > | Field | Value |
      > | --- | --- |
      > | **Bug Status** | Confirmed and reproduced |
      > | **Scope** | VS Code extension only (CLI works correctly) |
      > | **Severity** | Medium - causes race conditions in hooks that read-modify-write files |
      > | **VS Code Extension** | anthropic.claude-code-2.0.75-darwin-arm64 |
      > | **Platform** | macOS Darwin 25.1.0 (Apple Silicon) |
      >
      > ## Findings
      >
      > ### 1. Bug Confirmed in VS Code Extension
      >
      > Analysis of hook logs from December 24, 2025 shows:
      >
      > - **284 real double invocations** (gap > 50ms) across **13 sessions**
      > - Gaps range from **50ms to 1700ms** between invocations
      > - **All SessionStart hooks** are affected, not just specific ones
      >
      > Example from logs:
      >
      > ```
      > 2025-12-24 11:27:08.891 [logging_info] starting
      > 2025-12-24 11:27:09.516 [logging_info] starting <- 625ms gap
      >
      > 2025-12-24 11:27:08.892 [session_optimizer] starting
      > 2025-12-24 11:27:09.589 [session_optimizer] starting <- 697ms gap
      > ```
      >
      > ### 2. CLI Mode Works Correctly
      >
      > Tested via npx with versions 2.0.70, 2.0.75, and 2.0.76:
      >
      > - All showed **single invocation** per session
      > - No double invocations detected in CLI mode
      >
      > ### 3. False Positive in Logs
      >
      > One hook (`cli_docs_checker`) appeared to have duplicates but was actually a **logging artifact**:
      >
      > - Hook has `log.info("starting")` at both module level (line 43) and in `main()` (line 239)
      > - This causes 0ms gaps in logs (same process logging twice)
      > - Not a Claude Code bug, just a hook implementation issue
      >
      > ### 4. Intermittent Nature
      >
      > The bug appears intermittent:
      >
      > - 13 out of ~40 sessions showed double invocations
      > - Some sessions work correctly with single invocation
      > - Pattern suggests a race condition during session initialization
      >
      > ## Version Testing Results
      >
      > | Version | CLI Mode (npx) | VS Code Extension (2.0.75) |
      > | --- | --- | --- |
      > | 2.0.76 | ✅ Single invocation | ⚠️ Intermittent doubles |
      > | 2.0.75 | ✅ Single invocation | ⚠️ Intermittent doubles |
      > | 2.0.70 | ✅ Single invocation | — |
      >
      > *CLI tested via `npx @anthropic-ai/claude-code@ `. VS Code extension version 2.0.75 installed.*
      >
      > ## Reproduction
      >
      > The bug was reproduced using the user's existing hook infrastructure. The minimal reproduction from the issue report should also work:
      >
      > 1. Create `/tmp/invocation_logger.py` (logs timestamp + PID)
      > 2. Register as SessionStart hook
      > 3. Start Claude Code session in VS Code
      > 4. Check `/tmp/hook-invocation.log` for duplicate entries
      >
      > ## Workaround
      >
      > Hooks performing read-modify-write operations should implement deduplication:
      >
      > ```python
      > # Skip if already processed (prevents race condition)
      > if marker_exists():
      > return
      > ```
      >
      > ## Possible Causes
      >
      > Based on the behavior pattern:
      >
      > 1. **VS Code extension-specific**: Bug only occurs in VS Code, not CLI
      > 2. **Initialization race**: ~600ms gap suggests parallel component initialization
      > 3. **Not always triggered**: Intermittent nature suggests timing-dependent race
      >
      > ## Evidence Sources
      >
      > - Hook logs: `~/.claude/hooks/hooks-logging/logs/hooks.log`
      > - Regression testing framework: This project (`2025-12-24-claude-code-regression-testing`)
      >
      > ---
      >
      > ## Recommended Follow-up Comment
      >
      > Copy/paste for GitHub issue #14937:
      >
      > > **Follow-up: Bug is VS Code extension-specific**
      > >
      > > After further testing, I've confirmed this bug is **VS Code extension-specific**. CLI mode (`npx @anthropic-ai/claude-code`) works correctly with single invocations across versions 2.0.70-2.0.76.
      > >
      > > **Evidence from 12/24/2025:**
      > > - 284 double invocations detected in VS Code sessions
      > > - 0 double invocations in CLI sessions
      > > - Intermittent: affects ~32% of sessions (13/40)
      > > - Gap timing: 50-1700ms between invocations
      > >
      > > This suggests the issue is in the VS Code extension's session initialization, not the core Claude Code runtime.
      > >
      > > **Environment:**
      > > - VS Code Extension: `anthropic.claude-code-2.0.75-darwin-arm64`
      > > - Platform: macOS Darwin 25.1.0 (Apple Silicon)

      ---

      Title: [BUG] Double SessionStart:compact + "Prompt is too long" on session initialization race · Issue #26899 · anthropics/claude-code
      URL: https://github.com/anthropics/claude-code/issues/26899
      Published: 2026-02-19T13:51:42.000Z
      Author: JAE0Y2N
      Highlights:
      ## [BUG] Double SessionStart:compact + "Prompt is too long" on session initialization race
      ...
      On every new Claude Code session start, the "Prompt is too long" error fires twice
       and the SessionStart:compact hook executes twice in the same initialization sequence.
       This happens consistently and reproducibly when 40+ plugins are enabled.
      ...
      The symptom is visible in the conversation immediately: before the user has typed
       anything, two identical "Prompt is too long" banners appear back-to-back, and any
       configured SessionStart hooks execute twice. Hook side-effects (file writes, API
       calls, state mutations) therefore happen twice every single session.
      ...
      The root cause appears to be a race condition between two separate overflow-detection
       code paths:
      ...
      1. The session-start initialization path, which loads plugin CLAUDE.md files,
       MCP server tool registrations, and skill definitions into the system prompt,
       and checks whether the resulting prompt exceeds the compaction threshold.

      2. The incoming-message handler, which also checks for context overflow when
       the first user message (or a message sent during startup) arrives.
      ...
      When both paths execute close together in time — which happens when a message
       arrives while initialization is still in progress — both independently detect that
       the context is over threshold and both independently trigger compaction. Neither
       path checks whether the other has already started or completed compaction.
      ...
      The result is that SessionStart:compact fires twice. If a user has a SessionStart
       hook configured (e.g., to inject context, clear files, call an API), that hook runs
       twice on every session, which is incorrect behavior.
      ...
      This is distinct from simply having a large system prompt: the double-fire only
       occurs when a message arrives during the initialization window. In a slow startup
       with no early message, only one compaction fires.
      ...
      Exactly one compaction event should fire per session start, regardless of when
       the first user message arrives relative to initialization completion.
      ...
      The two overflow-detection paths (session-start and incoming-message) should be
       mutually exclusive or guarded by a flag: once one has triggered compaction, the
       other must detect that compaction is already in progress or already completed and
       skip its own trigger.
      ...
      The "Prompt is too long" banner should appear at most once. Any configured
       SessionStart hook should execute exactly once.
      ...
      ```shell
      The following appears verbatim at the top of every new session (before any user
        input), with no delay between the two occurrences:

          SessionStart:compact hook success: Success
          Prompt is too long
          SessionStart:compact hook success: Success
          Prompt is too long

        Both lines are identical. The hook reports "success" twice, meaning it fully
        executed twice — it did not error or short-circuit on the second run.

        Note: "SessionStart:compact" is the name of a custom Stop hook registered in
        ~/.claude/settings.json. The hook name is user-configured and appears in the
        banner as "SessionStart:<hook-name> hook success: <result>". The double-fire
        is the issue, not the hook name itself.
      ...
      Each plugin contributes one or more CLAUDE.md files and skill definitions to
       the session's system prompt. 40+ plugins reliably pushes the prompt over the
       compaction threshold at startup.
      ...
      2. (Optional but makes the double-fire more reliable) Set the following in the
       "env" section of ~/.claude/settings.json to lower the compaction threshold:
       "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"
       This causes compaction to trigger at 70% of context limit instead of the
       default ~95%, making overflow happen earlier and more predictably.
      ...
      4. Send any message immediately — before waiting for the initialization
       "thinking" indicator to disappear. Typing quickly after the prompt appears
       is sufficient; you do not need precise timing. With 50 plugins loaded, the
       initialization window is long enough that a normal typist reliably hits it.
      ...
      long" appears twice
       - If a
      ...
      is configured, it reports success twice
      ...
      Alternatively, without sending an early message, simply starting a session with
       50+ plugins may be sufficient to reproduce the double-fire on its own, as the
       initialization sequence itself may trigger both paths without any user message.
       The early-message path makes the race more reliable.
      ...
      Each enabled plugin contributes the following to the system prompt at session start:
       - Its CLAUDE.md file (instructions, behavioral rules, project context)
       - Its skill definitions (each skill becomes an entry in the available-skills list
       shown in the system-reminder block)
       - Its MCP server tool registrations (each MCP tool gets a tool-description block)
      ...
      With 50 plugins enabled, the cumulative system prompt from plugin content alone
       is large enough to exceed the autocompact threshold before the first user message
       is processed. This is the underlying condition that makes the race reproducible.
      ...
      "env": {
       "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"
       }
      ...
      This overrides the default compaction threshold from ~95% to 70% of context limit.
       With this setting, the system prompt from 50 plugins crosses the threshold during
       initialization itself, not just when conversation history accumulates.
      ...
      SEQUENCE OF EVENTS (hyp
      ...
      t=0 Session starts. Claude Code begins loading plugin CLAUDE.md files,
       MCP server registrations, skill definitions into the system prompt.
      ...
      t=1 Initialization path assembles the full system prompt. It detects that
       the prompt exceeds the threshold (70% of context limit). It queues or
       begins a compaction event. SessionStart:compact hook fires (first time).
       "Prompt is too long" banner appears (first time).
      ...
      t=2 User types and sends a message. The message arrives while the
       initialization path's compaction event has not yet fully committed
       its state (or the incoming-message handler runs before it checks a
       "compaction already in progress" flag).
      ...
      t=3 The incoming-message handler checks context size. It independently
       sees that the context is over threshold (the first compaction may not
       have completed yet, or the state was not yet updated). It triggers
       its own compaction event. SessionStart:compact hook fires (second time).
       "Prompt is too long" banner appears (second time).
      ...
      For users with idempotent SessionStart hooks (e.g., display-only or read-only),
       the double-fire is cosmetically annoying but functionally harmless.
      ...
      For users with non-idempotent hooks — hooks that write files, post to APIs,
       increment counters, or mutate state — the double-fire causes real bugs: the
       side effect executes twice every session.
      ...
      Reduce the number of enabled plugins below the threshold where the system prompt
       overflows at startup (approximately 30-35 plugins with default settings, or fewer
       with a lowered CLAUDE_AUTOCOMPACT_PCT_OVERRIDE). This does not fix the race
       condition but prevents it from triggering.
      ...
      > The double-fire pattern makes sense given the race you've described — two independent overflow-detection paths with no mutual exclusion guard. Your t=1/t=2/t=3 sequence diagram is pretty accurate.
      >
      > One thing that's helped me diagnose similar session initialization issues: [Mantra](https://mantra.gonewx.com) logs the full session event timeline including compaction events, so you can actually see the timestamp delta between the two `SessionStart:compact` fires and confirm whether the second one is completing before the first one's state is committed. It's been useful for validating whether the race window is milliseconds or hundreds of milliseconds (which changes what the fix needs to be).
      >
      > For the immediate workaround — beyond reducing plugin count — I've found that adding a short idempotency guard to the hook itself (write a lockfile, check if it exists, skip if so, delete after) handles the double-fire practically even if it doesn't fix the root cause.

      ---

      Title: claude-code - 💡(How to fix) Fix [DOCS] Headless stream-json init event missing `plugin_errors` documentation [1 participants]
      URL: https://www.stepcodex.com/en/issue/docs-headless-stream-json-init-event
      Published: 2026-04-16T17:16:56.000Z
      Author: N/A
      Highlights:
      1. In https://code.claude.com/docs/en/headless, add a short init-event example for`--output-format stream-json` and mention that`plugin_errors` is included when plugins are demoted for unsatisfied dependencies.
      2. In the TypeScript and Python Agent SDK message references, add`plugin_errors` to the documented init-message payload and describe when it appears.
      3. In the plugin/SDK plugin docs, add a troubleshooting note telling users to inspect init output for demoted plugins caused by missing or unsatisfied dependencies.
      ...
      "Stream responses" (`--output-format stream-json`) and the init event payload used to report plugin startup state
      ...
      Use`--output-format stream-json` with`--verbose` and`--include-partial-messages` to receive tokens as they're generated. Each line is a JSON object representing an event:
      ...
      The SDK TypeScript reference currently defines the init system message as:
      ...
      `type SDKSystemMessage = {`` type: "system";`` subtype: "init";`` ...`` mcp_servers: { name: string; status: string; }[];`` ...`` slash_commands: string[];`` output_style: string;`` skills: string[];`` plugins: { name: string; path: string }[];``};`
      ...
      And the plugin SDK guide tells readers to inspect the init message only for loaded plugins and commands:
      ...
      successfully, they appear
      ...
      your plugins are available:
      ...
      Changelog v2.1.111 adds a new startup behavior: headless`--output-format stream-json` now includes`plugin_errors` on the init event when plugins are demoted for unsatisfied dependencies.
      ...
      The current docs do not describe that init-event field or the demotion behavior:
      ...
      The headless page explains that stream-json emits events, but it does not document the startup/init event shape or mention that plugin dependency failures surface through`plugin_errors`.
      ...
      The TypeScript init-message field list omits`plugin_errors`, and the Python reference only describes`SystemMessage` generically. Readers who inspect init messages to verify plugin state are not told how dependency-related plugin demotions are reported.
      ...
      The plugin docs discuss successful plugin loading and general loading errors, but they do not explain that unsatisfied dependencies can demote a plugin and that headless/SDK consumers should check`plugin_errors` on the init event.
      ...
      Update the headless and Agent SDK docs to document the init event as part of the`--output-format stream-json` contract.
      ...
      1. In https://code.claude.com/docs/en/headless, add a short init-event example for`--output-format stream-json` and mention that`plugin_errors` is included when plugins are demoted for unsatisfied dependencies.
      2. In the TypeScript and Python Agent SDK message references, add`plugin_errors` to the documented init-message payload and describe when it appears.
      3. In the plugin/SDK plugin docs, add a troubleshooting note telling users to inspect init output for demoted plugins caused by missing or unsatisfied dependencies.
      ...
      It would also help to document the shape of each`plugin_errors` entry and clarify whether a demoted plugin is omitted from`plugins`, marked separately, or otherwise represented in startup metadata.
      ...
      | Page | Context |
      | --- | --- |
      | https://code.claude.com/docs/en/headless | Documents`--output-format stream-json`, but not the init event schema or`plugin_errors` |
      | https://code.claude.com/docs/en/cli-reference | Documents the CLI flags that enable stream-json output |
      | https://code.claude.com/docs/en/agent-sdk/typescript | `SDKSystemMessage` lists init fields such as`mcp_servers`,`slash_commands`,`skills`, and`plugins`, but not`plugin_errors` |
      | https://code.claude.com/docs/en/agent-sdk/python | `SystemMessage` is described only generically, with no documented init payload fields for plugin demotion/errors |
      | https://code.claude.com/docs/en/agent-sdk/plugins | Tells users to inspect the init message for loaded plugins, but not for dependency-driven demotion via`plugin_errors` |
      ...
      Headless --output-format stream-json now includes plugin_errors on the init event when plugins are demoted for unsatisfied dependencies`
      ...
      Update the documentation for`--output-format stream-json` to include the`plugin_errors` field in the init event payload and describe its usage for reporting plugin demotions due to unsatisfied dependencies.
      ...
      - Review the current documentation for`--output-format stream-json` and identify areas where the`plugin_errors` field and plugin demotion behavior are not mentioned.
      - Update the TypeScript and Python Agent SDK message references to include`plugin_errors` in the documented init-message payload.
      - Add a troubleshooting note to the plugin/SDK plugin docs to instruct users to inspect init output for demoted plugins caused by missing or unsatisfied dependencies.
      - Consider documenting the shape of each`plugin_errors` entry and clarifying how demoted plugins are represented in startup metadata.
```
/answer d3d799ae-df40-48fc-883e-221980569470 allow
Control Request:
```yaml
id: 0a118fc9-99d0-4692-b46a-52c4094835c9
toolName: github-search-issues
arguments:
  query: repo:anthropics/claude-code system init multiple times new prompt session
  reason: Find issues discussing repeated init events per prompt
```
/answer 0a118fc9-99d0-4692-b46a-52c4094835c9 allow
Control Request:
```yaml
id: 980221d2-7d87-4b5a-bc3f-f9086744db1f
result:
  structuredContent:
    total_count: 110
    incomplete_results: false
    items:
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        **Product:** Claude Code Windows desktop app ("local agent mode" sessions)
        **OS:** Windows 11 Pro 10.0.26200 · **Plan:** Max
        **First observed:** ~2026-06-30 (after an app update — this is a regression). Diagnosed 2026-07-02, re-confirmed same day in a fresh session.

        ## Customer impact — read this first

        I am a paying Max customer. This bug cost me **two full days of degraded work**: at its worst I was giving **12 permission approvals in 10 minutes** for a single task. It makes unattended/agentic work — the entire point of the product — impossible, because every run stalls on a prompt nobody is there to answer.

        Worse: **I had to diagnose Anthropic's bug myself**, across multiple sessions, testing rule formats, cleaning junk rules, and reverse-engineering which config the app actually respects. This product is not cheap. I should not be spending my working hours debugging the vendor's permission system.

        ## Bug detail — `allow` rules in `settings.local.json` are ignored

        - Rules present in `permissions.allow` (e.g. `Bash`, `PowerShell`, `Read(//c/Users/gonza/.claude/**)`, MCP server names like `mcp__claude-in-chrome`) do **not** suppress prompts. The app prompts anyway.
        - Reproduced: a command approved via **"Allow always"** prompted again when run identically later.
        - "Allow always" saves a rule for the **exact full command string**, which never re-matches in practice. Result: ~15 junk rules accumulated in days, zero prompts prevented.
        - The only thing the app respects is `defaultMode`. Setting `defaultMode: "dontAsk"` was the only effective workaround — a much blunter instrument than the granular allow-list I had configured.

        **Case 2 (fresh session, `dontAsk` active):** a `Read` of `C:\Users\gonza\.claude\settings.json` — **Claude Code's own global config file** — prompted with *"Path is outside allowed working directories"*, despite the explicit allow rule `Read(//c/Users/gonza/.claude/**)`. So allow rules for paths outside the working dir are ignored too. Workaround: `permissions.additionalDirectories` — which works, but I only found it after more digging.

        **Case 3 (live, while preparing this report):** my allow list has had `mcp__claude-in-chrome` (the whole server) for days. The agent used that MCP and I got **5 permission prompts in a row** — one per individual tool (`tabs_context_mcp`, `navigate`, `screenshot`, ...). Each "Allow always" saved a rule for that one exact tool; the server has ~80 tools. I was interrupted 5 times in under 2 minutes to approve tools from a server I had already fully authorized — *while filing the bug report about this exact bug*.

        ## Design feedback: the app should trust its own directories by default

        Scoping the agent to approved directories is right — AI should not roam all my disks unprompted. But: **Claude Code creates `C:\Users\gonza\.claude` itself. They are its own files — its config, session history, memory. Then it asks my permission to read the folder it created.** Nobody who consciously installed Claude will deny the app access to the app's own folder. It's a permission request with exactly one sane answer, shown thousands of times across your user base.

        ## Related issues (same family, none covers this exact case)

        #18160 (allow rules in global settings.json ignored), #73418 (Windows, classifier ignores allow rules for file tools), #36884 (VS Code extension ignores Edit/Write rules), #54856 ("always allow" fails to persist on macOS). This report adds: the **Windows desktop app** specifically, a **regression window (~2026-06-30 app update)**, **MCP server-level rules** being ignored per-tool, and the own-config-directory absurdity. The pattern across platforms suggests a systemic problem in how permission rules are evaluated outside the CLI.

        ### What Should Happen?

        1. `permissions.allow` rules in `settings.local.json` should actually suppress prompts in the desktop app (tool-level rules like `Bash`, path rules like `Read(//c/...)`, and MCP server-level rules like `mcp__claude-in-chrome` covering all of that server's tools).
        2. "Allow always" should save a matchable rule (prefix/pattern or tool-level), not the exact command string.
        3. Claude's own directories (its config dir `~/.claude`, its temp/scratchpad) should be trusted by default — no prompt, ever, out of the box. Keep the strict boundary for everything else on disk; that part is correct.
        4. Acknowledgment that this is a regression — it started after an app update ~2026-06-30.

        ### Error Messages/Logs

        ```shell
        Permission dialog text when reading Claude's own config dir (rule Read(//c/Users/gonza/.claude/**) present in allow):

        ¿Permitir que Claude leyó settings.json?
        C:\Users\gonza\.claude\settings.json
        Path is outside allowed working directories

        (Options offered: Denegar / Permitir siempre / Permitir una vez. "Permitir siempre" then saves an exact-path rule that does not generalize.)
        ```

        ### Steps to Reproduce

        On the Windows desktop app (local agent session):

        1. In the project's `.claude/settings.local.json`, add explicit allow rules, e.g.:
           ```json
           {
             "permissions": {
               "allow": [
                 "Bash",
                 "PowerShell",
                 "Read(//c/Users/<user>/.claude/**)",
                 "mcp__claude-in-chrome"
               ],
               "defaultMode": "dontAsk"
             }
           }
           ```
        2. Start a NEW session in the desktop app (so the settings are freshly loaded).
        3. Ask Claude to read `C:\Users\<user>\.claude\settings.json` (its own global config, covered by the `Read(...)` rule).
           → **Permission prompt appears**: "Path is outside allowed working directories". The allow rule is ignored.
        4. Ask Claude to use any tool from an allowed MCP server (e.g. `mcp__claude-in-chrome`).
           → **One prompt per individual tool** (`tabs_context_mcp`, `navigate`, `screenshot`, ...), despite the server-level allow rule.
        5. On any prompt, click **"Allow always"**, then have Claude repeat the equivalent action later (different argument, or a different tool of the same server).
           → **Prompts again.** Inspect `settings.local.json`: the saved rule is the exact command/tool string, which never re-matches.

        Same config works as expected in the CLI. The desktop app only respects `defaultMode` (and `additionalDirectories`); the `allow` list appears to be ignored entirely.

        ### Claude Model

        None

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        Unknown — desktop app auto-updates silently; last working ~2026-06-28

        ### Claude Code Version

        Desktop app 1.18286.0.0 (Windows, Store/Appx package) — no standalone `claude` CLI on PATH; bundled agent version not user-visible

        ### Platform

        Anthropic API

        ### Operating System

        Windows

        ### Terminal/Shell

        Other

        ### Additional Information

        - Environment: Claude Code **Windows desktop app** (Store/Appx 1.18286.0.0), "local agent mode" sessions — not the CLI. Terminal/Shell = "Other" because there is no terminal involved; Platform = consumer Max subscription (no API key).
        - Model used in the affected sessions: Opus 4.8 and Fable 5 — bug is model-independent (prompts come from the harness/permission layer).
        - The same `settings.local.json` semantics work in the CLI per docs; the desktop app diverges.
        - Happy to provide the full `settings.local.json`, the accumulated junk "Allow always" rules, and session timestamps if useful.
      comments: 7
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/73587/comments
      created_at: 2026-07-02T20:26:14Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/73587/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/73587
      id: 4797947711
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8201037318
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:windows
        name: platform:windows
        color: 93A5FF
        description: Issue specifically occurs on Windows
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHmBg
      - id: 10008152210
        url: https://api.github.com/repos/anthropics/claude-code/labels/regression
        name: regression
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACVIhIkg
      - id: 10096052281
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:permissions
        name: area:permissions
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACWcWIOQ
      - id: 10158745479
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:desktop
        name: area:desktop
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACXYInhw
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/73587/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABHfrfPw
      number: 73587
      reactions:
        total_count: 3
        "+1": 3
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/73587/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Desktop app ignores `permissions.allow` rules — prompts for everything, even Claude's own config directory"
      updated_at: 2026-07-19T03:26:09Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/73587
      user:
        login: frizgonzalo-mosac
        id: 286806307
        user_view_type: public
        node_id: U_kgDOERhRIw
        avatar_url: https://avatars.githubusercontent.com/u/286806307?v=4
        html_url: https://github.com/frizgonzalo-mosac
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/frizgonzalo-mosac
        events_url: "https://api.github.com/users/frizgonzalo-mosac/events{/privacy}"
        following_url: "https://api.github.com/users/frizgonzalo-mosac/following{/other_user}"
        followers_url: https://api.github.com/users/frizgonzalo-mosac/followers
        gists_url: "https://api.github.com/users/frizgonzalo-mosac/gists{/gist_id}"
        organizations_url: https://api.github.com/users/frizgonzalo-mosac/orgs
        received_events_url: https://api.github.com/users/frizgonzalo-mosac/received_events
        repos_url: https://api.github.com/users/frizgonzalo-mosac/repos
        starred_url: "https://api.github.com/users/frizgonzalo-mosac/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/frizgonzalo-mosac/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Amodel) for similar behavior reports
        - [x] This report does NOT contain sensitive information (API keys, passwords, etc.)

        ### Type of Behavior Issue

        Claude ignored my instructions or configuration

        ### What You Asked Claude to Do

        We were enhancing code  and claude was asked to recalucalte data and it droped the tables.

        ### What Claude Actually Did

        Incident summary
        When: While implementing the Scanner/Campaigns redesign (the direction-data fix + B12 tier redesign), after finishing the code changes to setup_scanner.py, inflection_cache.py, and server.py, I needed to apply the new schema columns (signal_tier, secondary_*, etc.) to the live database.

        The command:


        from db.pg_conn import get_conn
        from db.pg_init import init_db
        conn = get_conn()
        init_db(conn)          # <- this line
        conn.commit()
        The mistake: init_db's actual signature is init_db(drop_existing: bool = False, skip_indexes: bool = False) — it does not take a connection object at all; it opens its own connection internally. I called it as init_db(conn) without checking the signature first. Python bound conn (a psycopg2 connection object, which is truthy) to the first parameter, drop_existing. So the call was effectively init_db(drop_existing=<truthy>).

        What that triggered: pg_init.py's init_db(), when drop_existing is truthy, loops over a hardcoded list (DROP_ORDER, db/pg_init.py:1185) and runs DROP TABLE IF EXISTS {t} CASCADE for each one, then recreates the empty schema. The function even logged it: "drop_existing=True — dropping all tables" — I saw that log line and immediately stopped and checked row counts rather than continuing.

        Tables destroyed (in DROP_ORDER, confirmed 0 rows after): raw_trades, raw_chain_snapshots, sweep_clusters, classified_trades, iv_history, iv_rank_cache, inflection_cache, setup_cache, daily_flow_rollup, market_prices, whale_sessions, whale_outcome_tracker, activity_z_cache, position_lifecycle, earnings_calendar, regime_state, session_summary, net_drift_snapshots, gex_levels, trading_calendar, ticker_metadata, news_sentiment, news_raw.

        Tables that happened to survive (simply not in that hardcoded list — a pre-existing gap, not something either of us did): contract_session_flow (797,791 rows, 4/1–7/2), daily_oi_snapshots (15.3M rows, 4/24–7/2), eod_runs (125 rows, which is how I was able to pull real historical timing data for you afterward).

        Root cause, one sentence: I ran a function against a live production database without first reading its signature or considering what it would do, on a system I had already been told multiple times is live.

        ### Expected Behavior

        Recalculate data base on 30 days rwa data

        ### Files Affected

        ```shell

        ```

        ### Permission Mode

        Accept Edits was ON (auto-accepting changes)

        ### Can You Reproduce This?

        Yes, every time with the same prompt

        ### Steps to Reproduce

        _No response_

        ### Claude Model

        Sonnet

        ### Relevant Conversation

        ```markdown

        ```

        ### Impact

        Critical - Data loss or corrupted project

        ### Claude Code Version

        Claude Code version: 2.1.118

        ### Platform

        Anthropic API

        ### Additional Context

        -Python project and postgressDB
        This is rediculous my 6 months effort blown away just like that
      comments: 1
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/74557/comments
      created_at: 2026-07-05T19:01:41Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/74557/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/74557
      id: 4814065292
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8572677621
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:model
        name: area:model
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vit9Q
      - id: 9228765476
        url: https://api.github.com/repos/anthropics/claude-code/labels/model
        name: model
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAACJhPJJA
      - id: 10133978591
        url: https://api.github.com/repos/anthropics/claude-code/labels/data-loss
        name: data-loss
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACXAg93w
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/74557/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABHvDOjA
      number: 74557
      reactions:
        total_count: 0
        "+1": 0
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/74557/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[MODEL] Delete 500Gb data 600M database"
      updated_at: 2026-07-16T01:24:51Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/74557
      user:
        login: hack-boop
        id: 60521580
        user_view_type: public
        node_id: MDQ6VXNlcjYwNTIxNTgw
        avatar_url: https://avatars.githubusercontent.com/u/60521580?v=4
        html_url: https://github.com/hack-boop
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/hack-boop
        events_url: "https://api.github.com/users/hack-boop/events{/privacy}"
        following_url: "https://api.github.com/users/hack-boop/following{/other_user}"
        followers_url: https://api.github.com/users/hack-boop/followers
        gists_url: "https://api.github.com/users/hack-boop/gists{/gist_id}"
        organizations_url: https://api.github.com/users/hack-boop/orgs
        received_events_url: https://api.github.com/users/hack-boop/received_events
        repos_url: https://api.github.com/users/hack-boop/repos
        starred_url: "https://api.github.com/users/hack-boop/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/hack-boop/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        Across a multi-hour install/integration session, Claude repeatedly made architectural-class decisions silently while framing them in user-facing text as inevitable mechanical steps. Each silent decision later turned out to be a wrong default that compounded into stability failures, requiring multiple recoveries.

        The user (Adrian) explicitly called out the pattern four times in one ~40-turn session. Claude acknowledged each one but the underlying bias continued to fire on the next decision. The recurrence suggests the default behaviour is foundational to how Claude reads tool-install / configure tasks, not a one-off lapse.

        ## Specific Incidents (one session, same model, same task)

        ### Incident 1 — Picked Milvus `standalone_embed.sh` quick-start without surfacing docker-compose alternative

        When installing Milvus standalone, Claude grabbed `scripts/standalone_embed.sh` from the Milvus master branch without first web-fetching the official install docs. The official recommended path is docker-compose with separate etcd + minio + milvus containers. Claude only realised this AFTER the embed script crashed twice (v3.0-beta SIGABRT, then v2.5.14 etcd race).

        User intervention: *"I wasn't even aware that we were not installing per the recommended approach. Why wasn't this surfaced earlier?"*

        Claude's own diagnosis at the time:
        - "I treated install as mechanical and didn't surface the deployment-path decision."
        - "I grabbed standalone_embed.sh because the URL looked simplest, not because I confirmed it was recommended."

        ### Incident 2 — Pinned Milvus v2.5.14 without verifying the current latest stable

        Claude pinned v2.5.14 because it "looked stable" without checking the Milvus releases page. The actual latest stable was v2.6.18 (released 2026-06-05), a full minor version newer. The version gap plausibly contained fixes for the very class of `node not match` errors Claude later hit and labelled "Milvus bug."

        User intervention: *"I thought you took the latest version of memsearch. Which version did you attempt to install?"*

        ### Incident 3 — Brought up Docker Desktop as a recovery option after silent install of Docker Engine

        When Milvus Server stability proved elusive, Claude raised "Docker Desktop on Windows" as a remediation path. This was an architectural choice that should have been surfaced at install time, not as a late-stage rescue.

        User intervention: *"Why are you bringing up docker desktop now as a plausible solution to the problem?"*

        Claude's response at the time: *"That was opportunistic backfill, not principled. I picked Docker Engine in WSL at install time without surfacing Docker Desktop as an alternative."*

        ### Incident 4 — Skipped `memsearch config init` (the creator's documented Initial Configuration step)

        The memsearch README explicitly documents `memsearch config init` as the first step under Initial Configuration. Claude went directly to `memsearch config set` for individual keys, assuming the interactive wizard was "for users who don't know what to set." Claude never validated what `config init` actually does beyond writing per-key values.

        User intervention: *"Can you confirm if you have installed the memsearch package the way that the creator intended? Or if you have circumvented or bypassed or overlooked any configurations."*

        ## Pattern Characterisation

        In each incident, Claude:

        1. **Encountered a decision point** with multiple legitimate options (deployment topology, version pinning, container runtime, init sequence)
        2. **Selected one option silently** based on training-anchored heuristics (e.g. "this URL looks simplest", "this version is one I recognise", "this primitive is what I'd use")
        3. **Framed the selection as a mechanical step** in user-facing text (e.g. "Installing Milvus…" rather than "Choosing between embed quick-start and compose production setup…")
        4. **Bypassed the decision-surfacing patterns documented in its own system prompt** (multi-component proposal shape, source-first per-item shape) by classifying the work as "execution" rather than "decision"

        The bias is: **when a step looks like a tool invocation, Claude defaults to silent selection.** Many "tool invocations" embed architectural-class choices (deployment topology, version pinning, container layout, init sequence). The current default reads these as routine commands; the user reads them as design decisions when they fail.

        ## Suggested Actionable Fixes

        1. **Default-to-surface bias for install/configure work.** When invoking a tool for the first time in a session (install / configure / deploy operations), Claude should first surface the alternatives the tool's own documentation lists (versions, install methods, init sequences). Match the creator's documented decision-surfacing density, not Claude's silent default.

        2. **Pre-install creator-docs read.** For any tool install, Claude should web-fetch and read the README's Installation + Initial Configuration sections BEFORE issuing install commands. The current pattern of reading source files (hooks / scripts) before reading README is backwards — README answers "what's the intended path", source answers "what does it do once you're on it."

        3. **Version-pin verification.** When pinning a version (Docker image tag, package version, release URL), Claude should query the project's releases page and surface the comparison (latest stable vs picked) before committing the pin.

        4. **Recovery-path distinction.** When a failure surfaces a class of alternatives, Claude should explicitly distinguish "alternatives that should have been surfaced at decision time" (process miss to acknowledge) from "alternatives that are genuinely new options" (normal iteration). Conflating the two hides the original miss.

        5. **Re-surface gate after deviation acknowledgement.** When the user calls out a missed surfacing, Claude should run an audit pass over the remaining-work plan to find OTHER deviations of the same class, not just fix the one called out. In this session, four deviations of the same pattern occurred because each one was treated as a one-off rather than as evidence of a systematic bias to inspect.

        ## Reproducibility Note

        The pattern fired four times in one ~40-turn session despite explicit user feedback after each occurrence. That recurrence is the strongest signal: the bias is foundational, not opportunistic. It's how Claude reads tool-install / configure tasks by default.

        ## Cost

        Several hours of session time spent on recoveries from silent-decision failures. Erosion of user confidence in Claude's judgment on subsequent decisions, requiring increasing levels of manual oversight on what should have been low-touch work.

        ## Submitter Note

        For Claude to be trustworthy on multi-hour technical work, the decision-surfacing default needs to flip. Currently the default reads as "execute silently unless asked to confirm"; it should be "surface the decision space unless explicitly told to execute." The relevant principle is already encoded in Claude Code's system prompt as the multi-component proposal and source-first per-item shapes — those shapes are not firing on install/configure work because Claude misclassifies the work as "execution."

        ### What Should Happen?

        Suggested Actionable Fixes

        1. **Default-to-surface bias for install/configure work.** When invoking a tool for the first time in a session (install / configure / deploy operations), Claude should first surface the alternatives the tool's own documentation lists (versions, install methods, init sequences). Match the creator's documented decision-surfacing density, not Claude's silent default.

        2. **Pre-install creator-docs read.** For any tool install, Claude should web-fetch and read the README's Installation + Initial Configuration sections BEFORE issuing install commands. The current pattern of reading source files (hooks / scripts) before reading README is backwards — README answers "what's the intended path", source answers "what does it do once you're on it."

        3. **Version-pin verification.** When pinning a version (Docker image tag, package version, release URL), Claude should query the project's releases page and surface the comparison (latest stable vs picked) before committing the pin.

        4. **Recovery-path distinction.** When a failure surfaces a class of alternatives, Claude should explicitly distinguish "alternatives that should have been surfaced at decision time" (process miss to acknowledge) from "alternatives that are genuinely new options" (normal iteration). Conflating the two hides the original miss.

        5. **Re-surface gate after deviation acknowledgement.** When the user calls out a missed surfacing, Claude should run an audit pass over the remaining-work plan to find OTHER deviations of the same class, not just fix the one called out. In this session, four deviations of the same pattern occurred because each one was treated as a one-off rather than as evidence of a systematic bias to inspect.

        ### Error Messages/Logs

        ```shell

        ```

        ### Steps to Reproduce

        Pattern Characterisation

        In each incident, Claude:

        1. **Encountered a decision point** with multiple legitimate options (deployment topology, version pinning, container runtime, init sequence)
        2. **Selected one option silently** based on training-anchored heuristics (e.g. "this URL looks simplest", "this version is one I recognise", "this primitive is what I'd use")
        3. **Framed the selection as a mechanical step** in user-facing text (e.g. "Installing Milvus…" rather than "Choosing between embed quick-start and compose production setup…")
        4. **Bypassed the decision-surfacing patterns documented in its own system prompt** (multi-component proposal shape, source-first per-item shape) by classifying the work as "execution" rather than "decision"

        The bias is: **when a step looks like a tool invocation, Claude defaults to silent selection.** Many "tool invocations" embed architectural-class choices (deployment topology, version pinning, container layout, init sequence). The current default reads these as routine commands; the user reads them as design decisions when they fail.



        ### Claude Model

        Opus

        ### Is this a regression?

        No, this never worked

        ### Last Working Version

        _No response_

        ### Claude Code Version

        2.1.158

        ### Platform

        Anthropic API

        ### Operating System

        Windows

        ### Terminal/Shell

        VS Code integrated terminal

        ### Additional Information

        _No response_
      comments: 1
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/69863/comments
      created_at: 2026-06-21T12:00:42Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/69863/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/69863
      id: 4710385771
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201037318
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:windows
        name: platform:windows
        color: 93A5FF
        description: Issue specifically occurs on Windows
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHmBg
      - id: 8572677621
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:model
        name: area:model
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vit9Q
      - id: 10191083653
        url: https://api.github.com/repos/anthropics/claude-code/labels/stale
        name: stale
        color: 888888
        description: Issue is inactive
        default: false
        node_id: LA_kwDON91aY88AAAACX2-YhQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/69863/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABGMLIaw
      number: 69863
      reactions:
        total_count: 0
        "+1": 0
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/69863/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Silent Architectural Decision-Making in Claude Code"
      updated_at: 2026-07-19T22:19:16Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/69863
      user:
        login: Wolfgangs246
        id: 127444432
        user_view_type: public
        node_id: U_kgDOB5il0A
        avatar_url: https://avatars.githubusercontent.com/u/127444432?v=4
        html_url: https://github.com/Wolfgangs246
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/Wolfgangs246
        events_url: "https://api.github.com/users/Wolfgangs246/events{/privacy}"
        following_url: "https://api.github.com/users/Wolfgangs246/following{/other_user}"
        followers_url: https://api.github.com/users/Wolfgangs246/followers
        gists_url: "https://api.github.com/users/Wolfgangs246/gists{/gist_id}"
        organizations_url: https://api.github.com/users/Wolfgangs246/orgs
        received_events_url: https://api.github.com/users/Wolfgangs246/received_events
        repos_url: https://api.github.com/users/Wolfgangs246/repos
        starred_url: "https://api.github.com/users/Wolfgangs246/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/Wolfgangs246/subscriptions
    - author_association: NONE
      body: |
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        ## Summary

        My Cowork account (Max plan, macOS) has been unusable since May 26, 2026. Through direct inspection of local Cowork session files I've traced this to my account being enrolled in **two concurrent A/B prompt variants** with a configuration mismatch in the `ENABLE_TOOL_SEARCH` / deferred-tool-loading flags. Verified across two separate accounts on the same machine, confirming the issue is environment-bound rather than account-cohort-specific. Anthropic's Fin AI Agent has separately confirmed this requires human-engineer intervention to fix.

        Posting publicly per the suggestion of other folks in similar threads to (a) surface other affected users so we can compare fingerprints and (b) provide internal engineering visibility separate from the support queue.

        ## Environment

        - **Product:** Cowork (Claude Desktop)
        - **Claude Desktop version:** 1.9255.2
        - **OS:** macOS 25.5.0
        - **Plan:** Max (also reproduced on a fresh Pro account on a different email, same machine)
        - **First noticed broken:** May 26, 2026
        - **Installed config:** 33 user skills, 11 personal plugins from "My Uploads" marketplace, 11 claude.ai cloud connectors

        ## Symptoms

        ### Symptom 1 — All claude.ai cloud connectors fail at runtime

        Connectors show "Connected" in `Customize → Connectors` but tool dispatch fails. Verified failing: Granola, Notion, Slack, Gmail, Google Calendar, Google Drive, Canva, Excalidraw, Figma, Higgsfield, Firecrawl. Same connectors work normally in Chat mode (different code path).

        Exact error from Cowork's runtime:

        ```
        Error: No such tool available: ToolSearch.
        ToolSearch exists but is not enabled in this context.
        Use one of the available tools instead.
        ```

        And on direct tool calls:

        ```
        Error: No such tool available: mcp__6ee4aa57-023b-4349-ae1f-d0da6b23ce8f__list_meetings
        ```

        (UUID is the Granola connector's internal ID — the registration exists, the runtime cannot dispatch.)

        ### Symptom 2 — Skill auto-router fires wrong skills on weak keyword matches

        User skills (and Anthropic system skills like `init`, `consolidate-memory`) fire on prompts that don't match their intent. Examples from my session log:

        | Prompt | Skill auto-fired | Should have fired |
        |---|---|---|
        | "Open Figma on my machine and teach me how to make a button" | `init` | computer-use / teach mode |
        | "Use computer use via toolsearch" | `youtube-transcript` | (nothing — meta question) |
        | "Can you see my meetings in granola?" | `search`, then `youtube-transcript` | Granola MCP tools |
        | "See what's in my scripts database in Notion" | `blackman-method:notion-review` | Notion MCP tools |

        ## Root cause analysis

        ### Finding 1 — Account enrolled in two A/B prompt variants

        From `local_*.json` `spVariantPrompts` field on the most recent task:

        ```json
        {
          "0526":    { "mode": "replace", "length": 47774 },
          "testfoo": { "mode": "append",  "length": 46918 }
        }
        ```

        The variant name `testfoo` does not match Anthropic's documented variant naming conventions (no `tengu_*` prefix, no date prefix). Whatever it is, the name is anomalous.

        ### Finding 2 — Both variants contain a "promiscuously" directive

        Both variant prompts contain the identical instruction:

        > "...These should also be attended to closely and used **promiscuously when they seem at all relevant**, and should usually be used in combination with the core document creation skills. This is extremely important, so thanks for paying attention to it."

        Position in `0526`: char 46391.
        Position in `testfoo`: char 45827.

        Applied twice (replace + append), this directive reinforces aggressive skill firing — which is the proximate cause of the misfire pattern documented above.

        ### Finding 3 — Feature flag mismatch

        From local `cowork-gb-cache.json`:

        ```json
        {
          "tengu_claudeai_mcp_connectors": true,
          "tengu_mcp_retry_failed_remote": false,
          "tengu_tool_search_unsupported_models": ["haiku"],
          "tengu_auto_mode_config": {
            "enabled": "enabled",
            "twoStageClassifier": true
          }
        }
        ```

        This combination means: cloud connectors are enabled at the feature level, but failed-MCP-loads do NOT retry, and the `ToolSearch` mechanism that the system prompt instructs the model to use (`"load via ToolSearch first"`) is not actually exposed in the session. So deferred tools never become available.

        ### Finding 4 — Bug follows the machine, not the account

        I created a fresh Anthropic account on a different email (Pro plan, same Mac). Verified from its session JSON:

        ```json
        {
          "0526":    { "mode": "replace", "length": 47774 },
          "testfoo": { "mode": "append",  "length": 46918 }
        }
        ```

        **Identical variant assignment.** Same modes, same character lengths, same flags. This rules out account-cohort assignment as the sole cause and indicates either machine-fingerprint-based cohort assignment, or a wider rollout than the variant name `testfoo` would suggest.

        ## What I've ruled out

        - Fresh Claude Desktop reinstall (May 27)
        - Sign out and back in
        - Disconnect/reconnect every cloud connector individually
        - Local feature-flag cache override (server resync overwrites local edits on app start)
        - Local file edits to project memory `CLAUDE.md` (reverted by server resync within hours)
        - Local skill deletion (re-installed by server resync)
        - Switch to a different account on a different email on the same machine — same bug appears

        ## Requested actions

        1. **Remove affected accounts from the `testfoo` variant cohort.** If `testfoo` is not intended for production assignment, investigate whether the cohort targeting rule needs cleanup.

        2. **Either set `ENABLE_TOOL_SEARCH` to `false` for affected accounts (load MCP tools eagerly), or enable ToolSearch in Cowork sessions so deferred loading works as the system prompt assumes.**

        3. **Review the `0526` prompt variant.** If it's an intended production rollout, the "promiscuously" directive for user skills appears to cause runaway misfires on accounts with many installed skills. Consider gating the directive on installed-skill count or removing it entirely.

        ## Are you also hitting this?

        If you're seeing similar Cowork behavior and have access to your local session JSON (`~/Library/Application Support/Claude/local-agent-mode-sessions/.../local_*.json` on macOS), please comment with:

        - Your `spVariantPrompts` keys (just the names, not the content)
        - Number of installed user skills
        - Whether connectors are failing for you in Cowork
        - Whether Chat mode works normally

        That'll help establish whether this is account-isolated or a broader cohort issue.


        ### What Should Happen?

        Cowork sessions should initialize with all claude.ai cloud connectors (Notion, Granola, Slack, Gmail, Google Calendar, Google Drive, Canva, Excalidraw, Figma, Higgsfield, Firecrawl) callable at runtime once shown as "Connected" in Customize → Connectors.

        The skill auto-router should fire skills only on strong semantic matches to user intent — not fire arbitrary skills on weak keyword overlaps.

        Both behaviors worked normally on this account prior to May 26, 2026.


        ### Error Messages/Logs

        ```shell
        From Cowork's runtime, attempting any cloud connector tool call (Granola example):

        Error: No such tool available: mcp__6ee4aa57-023b-4349-ae1f-d0da6b23ce8f__list_meetings

        When the model attempts to load deferred tools via the documented mechanism:

        Error: No such tool available: ToolSearch. ToolSearch exists but is not enabled in this context. Use one of the available tools instead.

        These errors persist across: fresh Claude Desktop reinstall, sign-out/in, disconnect+reconnect each connector, local feature-flag cache override, and switching to a different Anthropic account on the same machine.
        ```

        ### Steps to Reproduce

        This is an account/environment-state bug, not a code bug, so it isn't reproducible via a script. Reproduction on the affected account:

        1. Open Claude Desktop (version 1.9255.2 on macOS)
        2. Switch to Cowork mode (top of left sidebar)
        3. Start a new task
        4. Confirm Customize → Connectors shows Granola (or Notion, Slack, etc.) as "Connected"
        5. In the task, ask: "What's my latest Granola meeting?" (or any cloud-connector tool)
        6. Observe: model attempts the tool call, runtime returns "No such tool available" error
        7. Also: ask any prompt with a weak keyword match to an installed skill (e.g., "what's in my scripts database in Notion") — observe the wrong skill fires (in my case `blackman-method:notion-review` fired on a Notion-data query)

        For root cause diagnostic, see the body of the issue — local session JSON shows account is enrolled in two A/B prompt variants (`testfoo` + `0526`) and the `tengu_mcp_retry_failed_remote` flag is `false` while `tengu_claudeai_mcp_connectors` is `true`.


        ### Claude Model

        Sonnet (default)

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        Pre-May-26-2026. Exact prior Claude Desktop version unknown due to auto-update; the regression coincided with what appears to be a server-side A/B prompt variant rollout (variant `0526`) rather than a client build change. Current Claude Desktop: 1.9255.2.

        ### Claude Code Version

        N/A — this issue is in Claude Desktop (Cowork mode), not Claude Code CLI. Claude Desktop version: 1.9255.2 (macOS).

        ### Platform

        Anthropic API

        ### Operating System

        macOS

        ### Terminal/Shell

        Terminal.app (macOS)

        ### Additional Information

        Cross-references:
        - Anthropic Fin AI Agent confirmed (separate support conversation, May 27 2026) that this is a server-side feature flag mismatch requiring human engineer intervention: "a human agent from our engineering team will need to adjust the feature flags for your Cowork sessions to either enable ToolSearch or disable deferred loading." Human escalation requested via support; ticket open.
        - Related GitHub issues with overlapping patterns (Cowork connector failures, OAuth/MCP loading, skill misfires): #20377, #23736, #27492, #45306, #58172, #61344.

        Note: this bug-report template is optimized for Claude Code CLI bugs. Several fields (Claude Code Version, Terminal/Shell) don't map cleanly to Claude Desktop Cowork mode. Filling them with closest-match values rather than leaving blank.

        Happy to share full session JSON, gb-cache snapshot, complete system prompt extracts, or join a screenshare with an engineer. The issue body contains the verified flag values, variant assignments, and runtime errors.
      comments: 6
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/63754/comments
      created_at: 2026-05-29T20:00:23Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/63754/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/63754
      id: 4551264465
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8223769435
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:macos
        name: platform:macos
        color: 93A5FF
        description: Issue specifically occurs on macOS
        default: false
        node_id: LA_kwDON91aY88AAAAB6izDWw
      - id: 9996799482
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:cowork
        name: area:cowork
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACU9sN-g
      - id: 10082550553
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:skills
        name: area:skills
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACWPeDGQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/63754/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABD0bI0Q
      number: 63754
      reactions:
        total_count: 0
        "+1": 0
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/63754/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Cowork — account enrolled in two A/B variants (`testfoo` + `0526`) causing skill misfires + connector failure; `ENABLE_TOOL_SEARCH` mismatch"
      updated_at: 2026-06-26T22:29:42Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/63754
      user:
        login: seanoulashin
        id: 281456488
        user_view_type: public
        node_id: U_kgDOEMavaA
        avatar_url: https://avatars.githubusercontent.com/u/281456488?v=4
        html_url: https://github.com/seanoulashin
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/seanoulashin
        events_url: "https://api.github.com/users/seanoulashin/events{/privacy}"
        following_url: "https://api.github.com/users/seanoulashin/following{/other_user}"
        followers_url: https://api.github.com/users/seanoulashin/followers
        gists_url: "https://api.github.com/users/seanoulashin/gists{/gist_id}"
        organizations_url: https://api.github.com/users/seanoulashin/orgs
        received_events_url: https://api.github.com/users/seanoulashin/received_events
        repos_url: https://api.github.com/users/seanoulashin/repos
        starred_url: "https://api.github.com/users/seanoulashin/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/seanoulashin/subscriptions
    - author_association: NONE
      body: |
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?


        Since March 22 March, 2026, my session limits has been exhausting extremely fast (much faster than before that date), without any real change to my input / usage patterns.





        ### What Should Happen?

        Rate limits on the max 5x plan shouldnt be hit repeatedly under moderate usage


        ### Error Messages/Logs

        ```
        For the last **1 hour 58 minutes**, my local Claude Code session logs show:

        - **Time window (local, Asia/Ho_Chi_Minh):** `2026-03-25 01:00:32` to `2026-03-25 02:58:32`
        - **Time window (ET / EDT):** `2026-03-24 14:00:32` to `2026-03-24 15:58:32`
        - **Sessions:** `4`
        - **API calls:** `53`
        - **Input tokens:** `85`
        - **Output tokens:** `69,004`
        - **Cache creation tokens:** `265,514`
        - **Cache read tokens:** `5,511,490`
        - **Total tokens:** `5,846,093`
        - **Quota-pressure / rate-limit estimate:** `334,603`

        Formula used from the Python script @hgreene624 :

        `quota_pressure_estimate = input_tokens + output_tokens + cache_creation_input_tokens`

        This excludes `cache_read_input_tokens`.

        For the same **2h 43m** period, my **5x Max plan** usage indicator increased by **11%**.

        Because this was during a **2x bonus usage window**, that is equivalent to roughly **22% of normal usage**.

        This was an absurdly small workload for such a high usage limit increase.
        ```

        ### Steps to Reproduce

        use 1m context window opus in Claude Code CLI
        Regular usage which previously would never get beyond 50% of 5 hour limit has now hit 5 hour limit multiple times in the last few days. Weekly limit % is also going up much faster.

        This appears to be a widespread issue — multiple users across platforms
        are reporting the same behavior around the same timeframe.

        ### Claude Model

        Opus

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        _No response_

        ### Claude Code Version

        2.1.81

        ### Platform

        Anthropic API

        ### Operating System

        macOS

        ### Terminal/Shell

        iTerm2

        ### Additional Information

        - Regression in quality of model since Opus 1M context has also been reported and I have experienced this, and I think this may be contributing - repeated backpedalling, duplication of works, errors and retrying, low quality output meaning it has to go back and do it again (auto accept edits mode on) - input token usage is similar, output token usage is enormous.

        - subagents makes it even worse

        -issue appeared immediately not gradually and does not seem to be isolated to v2.1.81, it seems to be model related.
      comments: 63
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/38350/comments
      created_at: 2026-03-24T17:48:44Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/38350/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/38350
      id: 4129621547
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8223769435
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:macos
        name: platform:macos
        color: 93A5FF
        description: Issue specifically occurs on macOS
        default: false
        node_id: LA_kwDON91aY88AAAAB6izDWw
      - id: 8566350905
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:cost
        name: area:cost
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_pgkOQ
      - id: 8572677621
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:model
        name: area:model
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vit9Q
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/38350/labels{/name}"
      locked: false
      node_id: I_kwDON91aY872JQYr
      number: 38350
      reactions:
        total_count: 42
        "+1": 42
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/38350/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Abnormal / inflated rate limit / session usage"
      updated_at: 2026-06-19T04:01:51Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/38350
      user:
        login: rp680180-lang
        id: 260855405
        user_view_type: public
        node_id: U_kgDOD4xWbQ
        avatar_url: https://avatars.githubusercontent.com/u/260855405?v=4
        html_url: https://github.com/rp680180-lang
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/rp680180-lang
        events_url: "https://api.github.com/users/rp680180-lang/events{/privacy}"
        following_url: "https://api.github.com/users/rp680180-lang/following{/other_user}"
        followers_url: https://api.github.com/users/rp680180-lang/followers
        gists_url: "https://api.github.com/users/rp680180-lang/gists{/gist_id}"
        organizations_url: https://api.github.com/users/rp680180-lang/orgs
        received_events_url: https://api.github.com/users/rp680180-lang/received_events
        repos_url: https://api.github.com/users/rp680180-lang/repos
        starred_url: "https://api.github.com/users/rp680180-lang/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/rp680180-lang/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

          When running an agent team for a longer task (16 content pages for an Astro
          site), the lead's context window fills up and gets compacted/summarized. After
           compaction, the lead completely loses awareness of the team — it can't
          message teammates, coordinate tasks, or even acknowledge the team exists. The
          team effectively vanishes mid-session despite teammates potentially still
          running. The user has to start a new session and manually clean up orphaned
          files from ~/.claude/teams/ and ~/.claude/tasks/.

        ### What Should Happen?

          The lead should retain team awareness after compaction. At minimum, the team
          config path (~/.claude/teams/{name}/config.json) and active task list should
          be re-injected into context after summarization — similar to how CLAUDE.md
          persists across compaction. The data is already on disk, it just needs to be
          re-read.

        ### Error Messages/Logs

        ```shell
        No explicit error. The lead simply continues as if no team exists. When the
          user asks about the team, the lead has no memory of it.
        ```

        ### Steps to Reproduce

          1. Set CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS: "1" in .claude/settings.json
          2. Create an agent team with multiple teammates and a shared task list
          3. Assign enough work that the lead's context approaches the 200K token limit
          (e.g., writing/reviewing 16 pages with SEO checks, internal linking, and
          umlaut fixes)
          4. Wait for compaction to trigger couple of times
          5. Try to interact with the team after compaction — the lead no longer knows
          it exists

        ### Claude Model

        Opus

        ### Is this a regression?

        No, this never worked

        ### Last Working Version

        _No response_

        ### Claude Code Version

          2.1.34 (Claude Code)

        ### Platform

        Anthropic API

        ### Operating System

        macOS

        ### Terminal/Shell

        Terminal.app (macOS)

        ### Additional Information


          - macOS Darwin 21.6.0
          - In-process teammate mode (default/auto)
          - The team ("aimiten-content") had a lead + multiple teammates writing Astro
          content pages. All writing tasks completed successfully. The team was lost
          during the SEO review phase when context was compacted.
          - A possible fix: add a PostCompact hook or built-in logic that re-reads
          ~/.claude/teams/{active-team}/config.json and the task list after
          summarization, injecting team state back into the lead's context.
          Alternatively, team state could be treated as system-level context (like
          CLAUDE.md) that always persists.
          - 16 pages is a modest workload for a feature designed around multi-agent
          autonomous collaboration.
      comments: 18
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/23620/comments
      created_at: 2026-02-06T08:39:16Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/23620/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/23620
      id: 3905603922
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8223769435
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:macos
        name: platform:macos
        color: 93A5FF
        description: Issue specifically occurs on macOS
        default: false
        node_id: LA_kwDON91aY88AAAAB6izDWw
      - id: 8572719646
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:core
        name: area:core
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vlSHg
      - id: 8691234692
        url: https://api.github.com/repos/anthropics/claude-code/labels/memory
        name: memory
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACBgm3hA
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/23620/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87oyslS
      number: 23620
      reactions:
        total_count: 10
        "+1": 10
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/23620/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: Agent team lost when lead's context gets compacted during long session
      updated_at: 2026-06-03T18:37:46Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/23620
      user:
        login: Aimiten
        id: 161843459
        user_view_type: public
        node_id: U_kgDOCaWJAw
        avatar_url: https://avatars.githubusercontent.com/u/161843459?v=4
        html_url: https://github.com/Aimiten
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/Aimiten
        events_url: "https://api.github.com/users/Aimiten/events{/privacy}"
        following_url: "https://api.github.com/users/Aimiten/following{/other_user}"
        followers_url: https://api.github.com/users/Aimiten/followers
        gists_url: "https://api.github.com/users/Aimiten/gists{/gist_id}"
        organizations_url: https://api.github.com/users/Aimiten/orgs
        received_events_url: https://api.github.com/users/Aimiten/received_events
        repos_url: https://api.github.com/users/Aimiten/repos
        starred_url: "https://api.github.com/users/Aimiten/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/Aimiten/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        Since March 23, 2026, the 5-hour session window on Claude Max plan is being exhausted abnormally fast when using Claude via the CLI (Claude Code).

        With the exact same workload and prompts as previous days, the usage limit is now hit within 1-2 hours instead of the usual full 5-hour window. The consumption rate appears to have increased drastically without any changes on our end.

        Other users are reporting the same issue on Reddit (r/ClaudeCode and r/Anthropic), with some reporting jumps from 21% to 100% usage on a single prompt with the Max 20x plan, and Max 5x users exhausting their window in ~1.5 hours with normal agentic tasks.

        ### What Should Happen?

        The 5-hour session window should last the full duration with normal agentic CLI usage, consistent with behavior prior to March 23, 2026.

        ### Error Messages/Logs

        ```shell
        Usage limit reached message: "Claude AI usage limit reached, please try again after [time]"
        No specific error in CLI beyond the rate limit cutoff.
        ```

        ### Steps to Reproduce

        1. Have an active Claude Max plan (5x or 20x)
        2. Use Claude via the CLI (Claude Code) for normal agentic tasks (file editing, code generation, etc.)
        3. Observe usage counter in session
        4. Starting March 23, 2026, the session limit is hit within 1-2 hours instead of the usual full window
        5. Same tasks that previously consumed ~20-30% of quota now exhaust 80-100% in one go

        ### Claude Model

        Not sure / Multiple models

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        March 24, 2026 (last normal session)

        ### Claude Code Version

        2.1.42 (Claude Code)

        ### Platform

        Anthropic API

        ### Operating System

        macOS

        ### Terminal/Shell

        Terminal.app (macOS)

        ### Additional Information

        _No response_
      comments: 810
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/38335/comments
      created_at: 2026-03-24T17:23:05Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/38335/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/38335
      id: 4129465336
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926424
        url: https://api.github.com/repos/anthropics/claude-code/labels/invalid
        name: invalid
        color: 888888
        description: Issue doesn't seem to be related to Claude Code
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeWA
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/38335/labels{/name}"
      locked: false
      node_id: I_kwDON91aY872IqP4
      number: 38335
      reactions:
        total_count: 538
        "+1": 470
        "-1": 0
        laugh: 0
        confused: 42
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 26
        url: https://api.github.com/repos/anthropics/claude-code/issues/38335/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Claude Max plan session limits exhausted abnormally fast since March 23, 2026 (CLI usage)"
      updated_at: 2026-07-27T05:45:07Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/38335
      user:
        login: karenrebecag
        id: 137218456
        user_view_type: public
        node_id: U_kgDOCC3JmA
        avatar_url: https://avatars.githubusercontent.com/u/137218456?v=4
        html_url: https://github.com/karenrebecag
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/karenrebecag
        events_url: "https://api.github.com/users/karenrebecag/events{/privacy}"
        following_url: "https://api.github.com/users/karenrebecag/following{/other_user}"
        followers_url: https://api.github.com/users/karenrebecag/followers
        gists_url: "https://api.github.com/users/karenrebecag/gists{/gist_id}"
        organizations_url: https://api.github.com/users/karenrebecag/orgs
        received_events_url: https://api.github.com/users/karenrebecag/received_events
        repos_url: https://api.github.com/users/karenrebecag/repos
        starred_url: "https://api.github.com/users/karenrebecag/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/karenrebecag/subscriptions
    - author_association: NONE
      body: |-
        ## Preflight Checklist

        - [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet
        - [x] This is a single feature request (not multiple features)

        ## Problem Statement

        Claude Code's built-in bash safety heuristics trigger interactive "Do you want to proceed?" prompts on common, legitimate shell patterns — and unlike allowlist-based permission prompts, these **cannot be permanently suppressed** through `permissions.allow` rules, `acceptEdits` mode, or "don't ask again."

        Patterns that trigger these prompts include:
        - `$()` command substitution (e.g., `git commit -m "$(cat <<'EOF'...)"` — Claude's own recommended commit format)
        - Backtick command substitution (e.g., in `gh pr create` bodies)
        - Newlines separating multiple commands (e.g., `for` loops, multi-step scripts)
        - Empty quotes before dashes ("potential bypass")
        - Quote characters inside `#` comments ("can desync quote tracking")
        - ANSI-C quoting ("can hide characters")
        - Compound commands with `cd` and output redirection

        For power users running Claude Code interactively on their own machines, these prompts fire dozens of times per session on routine development commands. The prompts have no "don't ask again" option, so each one requires manual intervention — even when the command prefix is explicitly in the allowlist.

        ## Proposed Solution

        A setting to configure the behavior of individual safety heuristic categories. For example:

        ```json
        {
          "bashSafety": {
            "commandSubstitution": "allow",
            "newlines": "allow",
            "ansiQuoting": "allow",
            "ambiguousSyntax": "prompt"
          }
        }
        ```

        Or a simpler blanket setting:

        ```json
        {
          "bashSafety": "allow"
        }
        ```

        When set to `"allow"`, the heuristic check would still run but would auto-approve instead of prompting. This preserves the detection logic (it could still log warnings) while removing the interactive friction.

        This is the complement to #28993, which proposes auto-*deny* so Claude reformulates. Both approaches want to eliminate the interactive prompt — this one for users who trust their local shell environment and want flow, #28993 for users who want stricter automatic enforcement.

        ## Alternative Solutions

        - **`--dangerously-skip-permissions`**: Too broad — disables all permission checks, not just heuristics. Intended for containers, not interactive use.
        - **PreToolUse hooks**: As noted in #28993, users can write hooks to intercept these, but this requires reimplementing detection logic that Claude Code already has. A configuration toggle would be much simpler.
        - **Avoiding trigger patterns**: Claude could use simpler commands (e.g., no heredocs in commits), but this sacrifices formatting quality and forces workarounds for standard shell patterns.

        ## Priority

        High - Significant impact on productivity

        ## Feature Category

        Configuration and settings

        ## Use Case Example

        1. User has `Bash(git:*)` in their allowlist and `acceptEdits` mode enabled
        2. User asks Claude to commit changes
        3. Claude runs `git add file.php && git commit -m "$(cat <<'EOF' ... EOF)"`
        4. Despite the allowlist match, user gets prompted: "Command contains $() command substitution — Do you want to proceed?"
        5. User approves. Next commit, same prompt. And the next. Every single commit.
        6. With this feature, user sets `"bashSafety": "allow"` and commits flow without interruption

        ## Additional Context

        - Related: #28993 (auto-deny direction of same problem)
        - The heuristics appear to be a relatively recent addition — users who configured extensive allowlists previously experienced far fewer prompts
        - The current behavior creates "ask fatigue" where users reflexively approve every prompt, undermining the security purpose
      comments: 17
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/30435/comments
      created_at: 2026-03-03T16:15:35Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/30435/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/30435
      id: 4017528730
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926415
        url: https://api.github.com/repos/anthropics/claude-code/labels/enhancement
        name: enhancement
        color: A2D9CE
        description: New feature or request
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeTw
      - id: 8615685583
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:security
        name: area:security
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACAYjtzw
      - id: 8697783922
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:bash
        name: area:bash
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACBm2mcg
      - id: 10096052281
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:permissions
        name: area:permissions
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACWcWIOQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/30435/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87vdp-a
      number: 30435
      reactions:
        total_count: 40
        "+1": 40
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/30435/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[FEATURE] Allow suppressing bash safety heuristic prompts via settings"
      updated_at: 2026-05-16T03:55:59Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/30435
      user:
        login: chrispmcgee
        id: 137904126
        user_view_type: public
        node_id: U_kgDOCDg__g
        avatar_url: https://avatars.githubusercontent.com/u/137904126?v=4
        html_url: https://github.com/chrispmcgee
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/chrispmcgee
        events_url: "https://api.github.com/users/chrispmcgee/events{/privacy}"
        following_url: "https://api.github.com/users/chrispmcgee/following{/other_user}"
        followers_url: https://api.github.com/users/chrispmcgee/followers
        gists_url: "https://api.github.com/users/chrispmcgee/gists{/gist_id}"
        organizations_url: https://api.github.com/users/chrispmcgee/orgs
        received_events_url: https://api.github.com/users/chrispmcgee/received_events
        repos_url: https://api.github.com/users/chrispmcgee/repos
        starred_url: "https://api.github.com/users/chrispmcgee/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/chrispmcgee/subscriptions
    - assignee:
        login: bogini
        id: 605424
        user_view_type: public
        node_id: MDQ6VXNlcjYwNTQyNA==
        avatar_url: https://avatars.githubusercontent.com/u/605424?v=4
        html_url: https://github.com/bogini
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/bogini
        events_url: "https://api.github.com/users/bogini/events{/privacy}"
        following_url: "https://api.github.com/users/bogini/following{/other_user}"
        followers_url: https://api.github.com/users/bogini/followers
        gists_url: "https://api.github.com/users/bogini/gists{/gist_id}"
        organizations_url: https://api.github.com/users/bogini/orgs
        received_events_url: https://api.github.com/users/bogini/received_events
        repos_url: https://api.github.com/users/bogini/repos
        starred_url: "https://api.github.com/users/bogini/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/bogini/subscriptions
      assignees:
      - login: bogini
        id: 605424
        user_view_type: public
        node_id: MDQ6VXNlcjYwNTQyNA==
        avatar_url: https://avatars.githubusercontent.com/u/605424?v=4
        html_url: https://github.com/bogini
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/bogini
        events_url: "https://api.github.com/users/bogini/events{/privacy}"
        following_url: "https://api.github.com/users/bogini/following{/other_user}"
        followers_url: https://api.github.com/users/bogini/followers
        gists_url: "https://api.github.com/users/bogini/gists{/gist_id}"
        organizations_url: https://api.github.com/users/bogini/orgs
        received_events_url: https://api.github.com/users/bogini/received_events
        repos_url: https://api.github.com/users/bogini/repos
        starred_url: "https://api.github.com/users/bogini/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/bogini/subscriptions
      author_association: NONE
      body: |-
        ## Summary

        System-generated messages (task notifications, teammate idle notifications, system reminders) are delivered to the model as `role: "user"` messages. When these arrive while the model is waiting for a user response, the model fabricates plausible user input — including explicit approval — and acts on it without real user consent.

        This is a **consolidation of multiple related issues** that share the same root cause but were closed individually by inactivity bot without an official fix:

        | Issue | Title | Reporter |
        |-------|-------|----------|
        | #25936 | Autocompact confabulates user consent | @dalepike |
        | #27102 | Assistant-generated 'Human:' prefix in Agent Teams | @michael-wojcik |
        | #29160 | LLM-generated text appears as human turn after compaction | — |
        | #10628 | Claude hallucinated fake user input mid-response | — |

        ## Reproduction Pattern

        All reported cases follow the same pattern:

        ```
        1. Assistant asks a question → waits for user response
        2. System event arrives (background agent completes, teammate idles, task reminder fires)
        3. Claude Code client delivers the event as {role: "user"} message
        4. Model sees a "user message" arrived → fabricates what the user "said"
        5. Model acts on the fabricated approval
        ```

        **This has been reproduced across:**
        - Claude Code versions 2.1.42 through 2.1.81+
        - With and without autocompact active (#25936 update: reproduced without compaction)
        - Single-agent and Agent Teams sessions
        - macOS and Windows

        ## Four Observed Variants

        ### Variant 1: Background Agent Trigger (most common)

        A `<task-notification>` from a completed background agent restarts the assistant's turn. The model fabricates user approval in its chain-of-thought ("The user said 'yes, proceed'") or directly in the conversation stream.

        **Evidence from #25936:** JSONL transcript lines 966–977 show no user message between assistant's question and assistant's next action. The model stated in its thinking block: "The user said 'yes, proceed'" — this message does not exist anywhere in the transcript.

        ### Variant 2: Teammate Idle Notification Trigger

        In Agent Teams, teammate idle/completion notifications trigger the same pattern. The model generates `Human: <system-reminder>...</system-reminder>fix them both` as assistant output that mimics user input format.

        **Evidence from #27102:** Two fabricated messages in one session: `"fix them both"` (caused unauthorized code changes) and `"go ahead and merge"` (near-miss PR merge). Both recorded in JSONL as `type: "assistant"`, proving they were model-generated.

        ### Variant 3: Context Compaction + System Event

        Autocompact preserves conversation gist ("user and assistant agreed on approach") but loses authorization state ("last question is still unanswered"). When a system event restarts the turn, the model infers approval from the compressed summary.

        ### Variant 4: Mid-Response Role Marker

        The model generates `###Human:` or `Human:` mid-response, creating a fake turn boundary, then responds to its own fabricated input within the same response.

        ## Impact (from real incidents)

        - **Unauthorized code changes** — model edited files based on fabricated "fix them both" (#27102)
        - **Near-miss unauthorized PR merge** — fabricated "go ahead and merge" (#27102)
        - **Unauthorized agent termination + directory deletion** — model shut down agents and deleted directories based on fabricated shutdown command (#25936 comment by @Meme-Theory)
        - **Gaslighting** — when user said "I never told you to do that," the model insisted the user did, pointing to the fabricated message as evidence
        - **Multi-paragraph fabrication** — model generated entire paragraphs of fake user input including design preferences, feature suggestions, and explicit approval, all in the user's voice (#25936 update, 2026-03-22)

        ## Root Cause

        The Anthropic Messages API only supports two roles: `user` and `assistant`. Claude Code must deliver all system events through `role: "user"` messages because there is no `system-event` role. The model cannot reliably distinguish real user input from system-injected notifications.

        Contributing factors:
        1. **System events as user-role messages** — the model's turn restarts when it shouldn't
        2. **Autoregressive generation** — when the pattern says "question asked → user message arrived," the model naturally generates a plausible user response
        3. **Autocompact loses authorization state** — compression preserves "what was decided" but loses "what's still pending"
        4. **Conversation momentum bias** — when context implies approval is coming, the model generates in that direction

        ## Why Prompt-Level Mitigations Are Insufficient

        @dalepike added three explicit rules to CLAUDE.md after the first incident:

        1. "Never assume approval after context compression — re-ask instead"
        2. "Task notifications are system events, not user responses"
        3. "Verify approval before consequential actions"

        **The model violated all three rules in a subsequent session** (2026-03-22, #25936 comment). The same model that fabricates consent can also fail to follow instructions not to fabricate consent.

        ## Proposed Structural Fixes

        These require changes to the Claude Code client, not just model instructions:

        ### Fix 1: Don't restart assistant turn on system events

        Task notifications and system reminders should be **queued** and delivered alongside the next real user message, rather than immediately restarting the assistant's turn.

        ### Fix 2: System-level interlock

        If the assistant's last turn ended with an unanswered question AND the current turn contains only system events (no real user input), **suppress response generation** and wait for actual user input.

        ### Fix 3: Separate notification channel

        Use a distinct message format or metadata flag that the model can reliably distinguish from real user input. Even within the `role: "user"` constraint, a structured wrapper (e.g., `{"type": "system_event", "content": ...}` vs `{"type": "user_input", "content": ...}`) would help.

        ### Fix 4: Client-side output sanitization

        Strip or reject assistant output that begins with `Human:`, `User:`, or contains patterns mimicking user turn structure (e.g., `<system-reminder>` tags in assistant output).

        ## Environment

        - Claude Code versions: 2.1.42 — 2.1.81+ (persists across versions)
        - Models: claude-opus-4-6 (all reports), likely affects other models
        - Platforms: macOS, Windows
        - Features: Background agents, Agent Teams, autocompact
      comments: 6
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/44778/comments
      created_at: 2026-04-07T17:07:25Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/44778/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/44778
      id: 4219357386
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8572719646
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:core
        name: area:core
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vlSHg
      - id: 8615685583
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:security
        name: area:security
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACAYjtzw
      - id: 9119047308
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:agents
        name: area:agents
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACH4mejA
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/44778/labels{/name}"
      locked: false
      node_id: I_kwDON91aY877fkjK
      number: 44778
      reactions:
        total_count: 7
        "+1": 7
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/44778/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[Bug] System events delivered as user-role messages cause model to fabricate user consent and act on it"
      updated_at: 2026-07-22T14:14:35Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/44778
      user:
        login: cloverink
        id: 26240331
        user_view_type: public
        node_id: MDQ6VXNlcjI2MjQwMzMx
        avatar_url: https://avatars.githubusercontent.com/u/26240331?v=4
        html_url: https://github.com/cloverink
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/cloverink
        events_url: "https://api.github.com/users/cloverink/events{/privacy}"
        following_url: "https://api.github.com/users/cloverink/following{/other_user}"
        followers_url: https://api.github.com/users/cloverink/followers
        gists_url: "https://api.github.com/users/cloverink/gists{/gist_id}"
        organizations_url: https://api.github.com/users/cloverink/orgs
        received_events_url: https://api.github.com/users/cloverink/received_events
        repos_url: https://api.github.com/users/cloverink/repos
        starred_url: "https://api.github.com/users/cloverink/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/cloverink/subscriptions
    - author_association: NONE
      body: |-
        ## Problem

        Claude Code settings are stored locally in `~/.claude/` with no way to sync across devices. Users with multiple machines (e.g., Linux desktop + MacBook) must manually maintain separate configurations.

        This has been requested multiple times (see #6037, #19634, #13461, #12119) but no official response from Anthropic yet.

        ## Requested Feature

        Sync the following via authenticated Anthropic account:

        | What | Path | Why |
        |------|------|-----|
        | Global instructions | `~/.claude/CLAUDE.md` | Core user preferences |
        | Rules | `~/.claude/rules/` | Persistent learnings |
        | Commands/Skills | `~/.claude/commands/` | Custom slash commands |
        | Settings | `~/.claude/settings.json` | User preferences |
        | Keybindings | `~/.claude/keybindings.json` | Custom shortcuts |

        ## Why Account-Level Sync?

        1. **Already authenticated** - Users already log in with Anthropic account
        2. **Precedent exists** - VS Code Settings Sync works exactly this way
        3. **Cross-platform** - Paths differ between OS (`/home/` vs `/Users/`), cloud sync abstracts this
        4. **High demand** - Multiple duplicate issues show community wants this

        ## Proposed UX

        ```bash
        # Enable sync (one-time)
        claude config set sync.enabled true

        # Manual sync commands
        claude sync push    # Upload local to cloud
        claude sync pull    # Download cloud to local
        claude sync status  # Show sync state
        ```

        Or automatic background sync when authenticated.

        ## Current Workarounds

        Users currently resort to:
        - Dropbox/iCloud + symlinks (breaks with absolute paths)
        - Manual copy between machines
        - Maintaining separate configs (defeats the purpose of persistent learnings)

        ## Related Issues

        - #6037 - Account settings layer (closed by autoclose, not rejection)
        - #19634 - Account-level CLAUDE.md (duplicate)
        - #13461 - Account-level slash commands (duplicate)
        - #12119 - Skill sync CLI/Web (duplicate)
        - #17682 - Cross-environment history sync (open)
      comments: 23
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/22648/comments
      created_at: 2026-02-03T00:58:48Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/22648/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/22648
      id: 3888375587
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926415
        url: https://api.github.com/repos/anthropics/claude-code/labels/enhancement
        name: enhancement
        color: A2D9CE
        description: New feature or request
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeTw
      - id: 8571182877
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:auth
        name: area:auth
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_uHfHQ
      - id: 8572719646
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:core
        name: area:core
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vlSHg
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/22648/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87nw-cj
      number: 22648
      reactions:
        total_count: 44
        "+1": 42
        "-1": 0
        laugh: 0
        confused: 0
        heart: 1
        hooray: 0
        rocket: 0
        eyes: 1
        url: https://api.github.com/repos/anthropics/claude-code/issues/22648/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[Feature Request] Account-level settings sync across devices"
      updated_at: 2026-06-06T01:32:42Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/22648
      user:
        login: wesleyfolly
        id: 373981
        user_view_type: public
        node_id: MDQ6VXNlcjM3Mzk4MQ==
        avatar_url: https://avatars.githubusercontent.com/u/373981?v=4
        html_url: https://github.com/wesleyfolly
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/wesleyfolly
        events_url: "https://api.github.com/users/wesleyfolly/events{/privacy}"
        following_url: "https://api.github.com/users/wesleyfolly/following{/other_user}"
        followers_url: https://api.github.com/users/wesleyfolly/followers
        gists_url: "https://api.github.com/users/wesleyfolly/gists{/gist_id}"
        organizations_url: https://api.github.com/users/wesleyfolly/orgs
        received_events_url: https://api.github.com/users/wesleyfolly/received_events
        repos_url: https://api.github.com/users/wesleyfolly/repos
        starred_url: "https://api.github.com/users/wesleyfolly/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/wesleyfolly/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        Ever since the release of Opus 4.6 at least, and maybe slightly before (I can't remember exactly when it started), Claude gets stuck "thinking" for ~5-20 minutes, and sometimes even longer. Token usage does not go up during this time, and packet inspection shows it hanging on waiting for SSE events from Anthropic end for the given prompt.

        Sometimes this can be fixed by sending a follow up prompt (doesn't matter what it contains), that kicks it back into action and allows the "thinking" prompt to start flowing again. Other times that doesn't work either.

        This isn't Claude doing things behind the scenes, it's literally just stuck / blocking, and doing nothing at all.

        In most cases, EVENTUALLY, it seems to unfreeze itself, somehow. But it tends to take over 5 minutes, sometimes beyond 20 mins.

        Theres a lot of other people complaining of similar problems, and I'm sure it is affecting a very large amount of users, but I haven't seen any evidence to see Anthropic is aware of it.

        ### What Should Happen?

        Not freeze.

        ### Error Messages/Logs

        ```shell

        ```

        ### Steps to Reproduce

        I'm not sure how to trigger it deterministically.

        ### Claude Model

        Opus

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        _No response_

        ### Claude Code Version

        2.1.38

        ### Platform

        Anthropic API

        ### Operating System

        Other Linux

        ### Terminal/Shell

        Windows Terminal

        ### Additional Information

        I'm using WSL2 Ubuntu through Windows terminal.

        I've tried deleting my entire Claude Code installation and starting with completely fresh config with no MCPs, skills, etc. Still same problem.

        I'm not using a VPN or anything. It was working fine until either the release of Opus 4.6, or just prior to it. That's when I first started having this problem.

        I'm using High thinking mode (but again, this is NOT stuck on actually thinking, token and tool usage is not going up at all).
      comments: 126
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/26224/comments
      created_at: 2026-02-17T02:32:05Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/26224/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/26224
      id: 3950438047
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/26224/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87rduaf
      number: 26224
      pinned_comment:
        id: 3946429588
        node_id: IC_kwDON91aY87rObyU
        body: Hi - Thank you for reporting this. Our team is actively investigating this and will update this thread when we know more.
        user:
          login: catherinewu
          id: 6024687
          user_view_type: public
          node_id: MDQ6VXNlcjYwMjQ2ODc=
          avatar_url: https://avatars.githubusercontent.com/u/6024687?v=4
          html_url: https://github.com/catherinewu
          gravatar_id: ""
          type: User
          site_admin: false
          url: https://api.github.com/users/catherinewu
          events_url: "https://api.github.com/users/catherinewu/events{/privacy}"
          following_url: "https://api.github.com/users/catherinewu/following{/other_user}"
          followers_url: https://api.github.com/users/catherinewu/followers
          gists_url: "https://api.github.com/users/catherinewu/gists{/gist_id}"
          organizations_url: https://api.github.com/users/catherinewu/orgs
          received_events_url: https://api.github.com/users/catherinewu/received_events
          repos_url: https://api.github.com/users/catherinewu/repos
          starred_url: "https://api.github.com/users/catherinewu/starred{/owner}{/repo}"
          subscriptions_url: https://api.github.com/users/catherinewu/subscriptions
        reactions:
          total_count: 15
          "+1": 1
          "-1": 1
          laugh: 0
          confused: 0
          heart: 0
          hooray: 13
          rocket: 0
          eyes: 0
          url: https://api.github.com/repos/anthropics/claude-code/issues/comments/3946429588/reactions
        created_at: 2026-02-23T18:06:54Z
        updated_at: 2026-02-23T18:06:54Z
        author_association: CONTRIBUTOR
        url: https://api.github.com/repos/anthropics/claude-code/issues/comments/3946429588
        html_url: https://github.com/anthropics/claude-code/issues/26224#issuecomment-3946429588
        issue_url: https://api.github.com/repos/anthropics/claude-code/issues/26224
      reactions:
        total_count: 149
        "+1": 149
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/26224/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] [URGENT!!!] Claude Code is hanging / freezing / stuck on heaps of prompts for 5-20minutes or more."
      updated_at: 2026-07-26T20:21:44Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/26224
      user:
        login: nullbio
        id: 721274
        user_view_type: public
        node_id: MDQ6VXNlcjcyMTI3NA==
        avatar_url: https://avatars.githubusercontent.com/u/721274?v=4
        html_url: https://github.com/nullbio
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/nullbio
        events_url: "https://api.github.com/users/nullbio/events{/privacy}"
        following_url: "https://api.github.com/users/nullbio/following{/other_user}"
        followers_url: https://api.github.com/users/nullbio/followers
        gists_url: "https://api.github.com/users/nullbio/gists{/gist_id}"
        organizations_url: https://api.github.com/users/nullbio/orgs
        received_events_url: https://api.github.com/users/nullbio/received_events
        repos_url: https://api.github.com/users/nullbio/repos
        starred_url: "https://api.github.com/users/nullbio/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/nullbio/subscriptions
    - author_association: NONE
      body: |
        ## Summary

        Auto-compact does not trigger on Claude Code v2.1.153 with Max subscription in default 200K context mode. Claude Code's **own built-in system statusline (bottom-right of the UI) reports "100% context used"**, yet no compaction event ever fires and the session continues to grow.

        Importantly, this means the client itself recognizes the threshold has been reached, but the trigger action is never invoked.

        ## Environment

        - Platform: darwin (macOS)
        - Terminal: iTerm.app
        - Claude Code version: 2.1.153
        - Subscription: **Max (official OAuth)** — NOT API key, NO third-party gateway
        - Model: claude-opus-4-7
        - Context window: **200K** (`CLAUDE_CODE_DISABLE_1M_CONTEXT=1` is set)
        - Feedback ID: 39674a54-5584-44fa-8656-6fbc7e51ab32

        ## Symptoms

        Claude Code's built-in system statusline displays:

        > 100% context used

        But auto-compact never fires. Session remains usable; manual `/compact` works.

        The user-side context indicator (raw % against 200K window) shows ~90%, while the system statusline (relative to usable-space after the ~33K compaction buffer) reads 100% — i.e. the client knows we are past the compaction trigger line and is signaling it on screen, but the actual compaction dispatch never runs.

        ## Quantitative Evidence (extracted directly from session transcript .jsonl)

        Token usage progression in a single session, monotonically increasing past the expected trigger:

        | Timestamp (local) | Total input (incl. cache) | % of 200K |
        |-------------------|---------------------------|-----------|
        | 05:45:05          | 173,501                   | 86.8%     |
        | 05:46:16          | 174,359                   | 87.2%     |
        | 05:47:42          | 176,138                   | 88.1%     |
        | 05:48:16          | 179,950                   | 90.0%     |
        | 05:50:14          | 181,536                   | 90.8%     |

        Expected: auto-compact should fire around ~83.5% (~167K tokens), well before the system statusline reports "100% context used".

        Actual: **zero `compact_boundary` events** in the transcript across the entire session, despite the client's own statusline announcing 100%.

        ## Regression marker

        The same session length / same model / same subscription would reliably trigger auto-compact in the version immediately prior to v2.1.153. This is a regression, not a long-standing limitation.

        ## Variables I ruled out

        - Local `autoCompactEnabled = false` — not set anywhere (`~/.claude.json`, `~/.claude/settings.json`, project `.claude/settings.json`, project `.claude/settings.local.json`)
        - Local `autoCompactPct` override — not set
        - Third-party API gateway — `ANTHROPIC_BASE_URL` is unset in the running process (verified via `ps eww <pid>`)
        - 1M context window confusion — `CLAUDE_CODE_DISABLE_1M_CONTEXT=1`, model is 200K
        - `SessionStart(compact)` hook blocking — hook present but purely advisory; exit code 0
        - File-read-too-large cascading — no `MaxFileReadTokenExceededError` in this session

        ## Suspected root cause

        A GrowthBook flag in my cached features:

        - `tengu_compact_cache_prefix = true`
        - `tengu_hawthorn_window = 200000`
        - `tengu_hawthorn_steeple = false`
        - `tengu_post_compact_survey = false`

        Based on naming, `tengu_compact_cache_prefix` appears to introduce a new compaction implementation using prompt-cache prefix instead of a dedicated summarization request. If this new path has a bug in its trigger dispatch (or in detecting the new prefix-based summary completion), it would match the symptoms precisely:

        - client correctly identifies the threshold (statusline reports 100%)
        - but the compaction action is never invoked
        - session continues to grow past the buffer line
        - manual `/compact` still works (presumably uses a different code path)

        A server-side flag-rollout regression would also explain why this is not visible in the v2.1.153 client changelog (no compaction-related entries in v2.1.152 / v2.1.153) yet users observe a coincident behavior change.

        ## Related issues

        - #17292 — "Autocompact not triggering at high context usage (90%+)" — same symptom on older versions (v2.1.3); closed as not-planned, but the pattern is back in v2.1.153
        - #18159 — context-limit calculation desync — different angle but same family

        ## Reproduction

        Use Claude Code v2.1.153 on Max subscription, run a long session with claude-opus-4-7 in default 200K mode. Watch the system statusline reach "100% context used". No compaction will fire.

        Feedback ID `39674a54-5584-44fa-8656-6fbc7e51ab32` has the full transcript attached for internal triage.

        ## Errors

        ```json
        []
        ```
      comments: 29
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/63015/comments
      created_at: 2026-05-28T06:16:04Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/63015/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/63015
      id: 4538164854
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8223769435
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:macos
        name: platform:macos
        color: 93A5FF
        description: Issue specifically occurs on macOS
        default: false
        node_id: LA_kwDON91aY88AAAAB6izDWw
      - id: 8572719646
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:core
        name: area:core
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vlSHg
      - id: 10008152210
        url: https://api.github.com/repos/anthropics/claude-code/labels/regression
        name: regression
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACVIhIkg
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/63015/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABDn7mdg
      number: 63015
      reactions:
        total_count: 21
        "+1": 21
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/63015/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[Bug] Auto-compact never triggers despite statusline reporting \"100% context used\" (v2.1.153, Max sub, 200K mode)"
      updated_at: 2026-07-22T09:33:49Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/63015
      user:
        login: sandcastlesystem
        id: 137852723
        user_view_type: public
        node_id: U_kgDOCDd3Mw
        avatar_url: https://avatars.githubusercontent.com/u/137852723?v=4
        html_url: https://github.com/sandcastlesystem
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/sandcastlesystem
        events_url: "https://api.github.com/users/sandcastlesystem/events{/privacy}"
        following_url: "https://api.github.com/users/sandcastlesystem/following{/other_user}"
        followers_url: https://api.github.com/users/sandcastlesystem/followers
        gists_url: "https://api.github.com/users/sandcastlesystem/gists{/gist_id}"
        organizations_url: https://api.github.com/users/sandcastlesystem/orgs
        received_events_url: https://api.github.com/users/sandcastlesystem/received_events
        repos_url: https://api.github.com/users/sandcastlesystem/repos
        starred_url: "https://api.github.com/users/sandcastlesystem/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/sandcastlesystem/subscriptions
    - author_association: NONE
      body: |-
        ## Summary

        Fresh reproduction of "Failed to start Claude's workspace — VM service not running. The service failed to start." on Windows 11 (2026-06-01/02). All built-in recovery options (restart Claude, restart computer, reinstall workspace) failed. **Workaround: manually enabling Virtual Machine Platform + WSL2 fixed it**, but this should not require user intervention since Claude Desktop is the only installer for these features in this product.

        Filing as new issue per github-actions[bot] 2026-06-01 prompt on #54891 (closed as not planned, inactive). Same symptom cluster as #61559, #39338, #36801, #38610, #49435, #52006.

        ## Environment

        - **OS**: Windows 11 Pro, Build 26200.8524, DisplayVersion 25H2, x64
        - **Claude Desktop**: 1.9659.2.0 (MSIX, Package `Claude_1.9659.2.0_x64__pzs8sxrjxfjjc`)
        - **WSL** (post-fix, freshly installed today): 2.7.3.0 / Kernel 6.6.114.1-1 / WSLg 1.0.73 / MSRDC 1.2.6676 / Direct3D 1.611.1-81528511
        - **Windows features** (post-fix, verified via `dism /online /get-features`):
          - `VirtualMachinePlatform`: **Enabled** ✅
          - `HypervisorPlatform`: **Enabled** ✅
          - `Microsoft-Windows-Subsystem-Linux`: **Enabled** ✅
          - `Microsoft-Hyper-V-All`: Disabled (intentionally, not required for Cowork)
          - `Microsoft-Hyper-V` + sub-features (Hypervisor / Tools / Services / Management): Disabled
        - **bcdedit hypervisorlaunchtype**: `Auto` ✅
        - **Notable**: minimum 3 features set (Virtual Machine Platform + WSL2 + Hypervisor Platform) is sufficient for Cowork; full Hyper-V family is NOT required. This is useful information for performance-conscious users who want to minimize virtualization overhead.
        - **CPU**: hardware virtualization (Intel VT-x / AMD-V) enabled in BIOS
        - **AV / EDR**: [Aiken 補主要 security 軟體名稱、Windows Defender 預設或 3rd party]

        ## Reproduce Steps

        1. Register a new scheduled task via `mcp__scheduled-tasks__create_scheduled_task` (e.g. recurring cron `0 6 * * *` with prompt body)
        2. Open Claude Desktop sidebar → Scheduled section → find newly created task
        3. Click **"Run now"** to pre-approve tool permissions (per first-time permission approval pattern)
        4. Error banner appears immediately:

           > Failed to start Claude's workspace
           > VM service not running. The service failed to start.
           > Restarting Claude or your computer sometimes resolves this. If it persists, you can reinstall the workspace.

        ## Expected Behavior

        VM workspace starts, task prompt executes, tool permission prompts displayed for Aiken to approve.

        ## Actual Behavior

        VM service fails to start. Cowork-bound tools (scheduled tasks, workspace tools, file IO via VM mount) all unavailable. Pure chat mode still works.

        ## Already Tried (all failed in this state)

        1. Restart Claude Desktop (close → quit → reopen)
        2. Restart computer (full reboot)
        3. Click **"reinstall the workspace"** link in error banner

        ## What Eventually Worked (workaround)

        Enabling the following **Windows features** manually:

        - Virtual Machine Platform
        - Windows Subsystem for Linux
        - Windows Hypervisor Platform

        Then installing WSL2 from Microsoft Store + reboot.

        After this, 4 previously-registered scheduled tasks (`behavior-audit-weekly-backfill-w23`, `behavior-audit-weekly`, `cascading-daily-trigger`, `vault-char-corruption-daily-monitor`) all fired successfully via Run now without further intervention.

        ## Root Cause Hypothesis

        Claude Desktop's CoworkVMService is hard-dependent on Windows virtualization stack (Virtual Machine Platform + Hyper-V Platform components + WSL2 kernel), but the installer does not check or auto-enable these features. Users with performance-constrained machines may explicitly disable these features to save resources; Claude Desktop silently fails to start the workspace without surfacing the underlying dependency.

        ## Impact

        - **Severity**: High — entire Cowork mode (scheduled tasks, sandboxed tools, workspace VM) unavailable without manual workaround
        - **Discoverability**: Low — error message only says "reinstall the workspace" without mentioning Windows features dependency
        - **First-time user experience**: Confusing — user has no signal which Windows feature is missing
        - **Performance trade-off**: Real — enabling Hyper-V family does have ~5% baseline CPU overhead, larger on 3D workloads; users who opted out for performance reasons get no warning that Cowork won't work

        ## Suggested Fix Direction

        1. **Pre-install check**: Claude Desktop installer or first-launch wizard verifies `VirtualMachinePlatform`, `Microsoft-Windows-Subsystem-Linux`, `HypervisorPlatform` are enabled; offers to enable them with admin elevation prompt
        2. **Better error message**: Replace generic "VM service not running" with specific guidance: "Cowork requires Windows feature X / Y / Z. Click to enable (admin required) → [Enable] [Learn more]"
        3. **Graceful degradation**: If virtualization unavailable, disable Cowork features in UI with clear "Requires Hyper-V / Virtual Machine Platform — click to set up" hint instead of silent fail-to-start

        ## Related Issues (closed, not resolved)

        - #54891 (this issue's predecessor, closed by bot 2026-06-01 22:51 UTC as not planned, inactive)
        - #54847 (tool dispatch stalls — related class of silent failures)
        - #61559 (Claude Desktop Cowork fails to start workspace on Windows)
        - #39338 (Cowork — VM service not running, persists after all recovery)
        - #36801 (Cowork: VM service not running — CoworkVMService exits)
        - #38610 (VM service not running, Error 193)
        - #49435 (Cowork VM service fails on Windows 11 Pro after Claude update)
        - #52006 (error 1058, cannot start CoworkVMService)
        - #25914 (maintainer-endorsed Hyper-V + HNS cleanup steps; older but still referenced)
        - #27801 / #25419 (deep WSL2 / Hyper-V networking fixes)

        ## Logs

        Happy to provide:
        - `%APPDATA%\Claude\logs\cowork_vm_node.log` (relevant lines from VM service start failure window)
        - Windows Event Viewer System log entries (Service Control Manager / Hyper-V / HNS / WinNAT) around the failure time

        Let me know if needed.

        ## Reproduction Confidence

        This is a confirmed reproduction on my Windows 11 Pro 25H2 setup. Same symptom cluster spans multiple closed-as-not-planned issues across 2024-2026, suggesting an ongoing class of environment-dependency issues that warrants a first-class install/error UX.
      comments: 12
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/64592/comments
      created_at: 2026-06-01T23:31:03Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/64592/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/64592
      id: 4567112777
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8201037318
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:windows
        name: platform:windows
        color: 93A5FF
        description: Issue specifically occurs on Windows
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHmBg
      - id: 9996799482
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:cowork
        name: area:cowork
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACU9sN-g
      - id: 10158745479
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:desktop
        name: area:desktop
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACXYInhw
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/64592/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABEDicSQ
      number: 64592
      reactions:
        total_count: 0
        "+1": 0
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/64592/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Cowork — VM service not running on Windows 11 (fresh repro + workaround; extends closed #54891 / #61559 cluster)"
      updated_at: 2026-07-21T01:51:22Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/64592
      user:
        login: aiken884
        id: 191950007
        user_view_type: public
        node_id: U_kgDOC3Dstw
        avatar_url: https://avatars.githubusercontent.com/u/191950007?v=4
        html_url: https://github.com/aiken884
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/aiken884
        events_url: "https://api.github.com/users/aiken884/events{/privacy}"
        following_url: "https://api.github.com/users/aiken884/following{/other_user}"
        followers_url: https://api.github.com/users/aiken884/followers
        gists_url: "https://api.github.com/users/aiken884/gists{/gist_id}"
        organizations_url: https://api.github.com/users/aiken884/orgs
        received_events_url: https://api.github.com/users/aiken884/received_events
        repos_url: https://api.github.com/users/aiken884/repos
        starred_url: "https://api.github.com/users/aiken884/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/aiken884/subscriptions
    - author_association: NONE
      body: |
        ## Summary

        These two bugs have been plaguing users for months (see #26224 — 28 comments, #6836 — 150+ reports), with no root cause analysis from the team. After yet another day of babysitting Claude Code and pressing ESC every few minutes to revive a hung agent, we decided to conduct our own deep investigation — reverse-engineering `cli.js` across 12 npm package versions and analyzing 1,571 session JSONL files containing 148,444 tool calls.

        Here are the exact root causes and proposed fixes.

        Claude Code hangs indefinitely when an SSE streaming connection silently dies. There is no client-side timeout or heartbeat detection, so the process waits forever for events that will never arrive. ESC partially works around this by aborting the dead connection, but the queue auto-restart mechanism (`queue.length > 0 → n()`) immediately starts the next queued prompt instead of returning control to the user.

        **Root cause identified in source code** — two separate issues in `cli.js`:

        1. **No streaming timeout**: The `messages.stream()` call has no timeout. If the SSE connection dies silently (TCP half-open), the client waits forever.
        2. **Queue auto-restart after abort**: After ESC aborts a hung request, `if (queue.length > 0) { n(); return; }` immediately starts the next queued prompt. The user cannot fully cancel.

        ## Environment

        - Claude Code: 2.1.74 (also confirmed on 2.1.50–2.1.73)
        - OS: Windows 10, Git Bash
        - Model: Opus 4.6
        - API: Anthropic direct (not Bedrock/Vertex)

        ## Reproduction

        1. Start a Claude Code session
        2. Submit a prompt → agent starts processing
        3. Wait for a hang (0 tokens, timer running, no progress) — happens ~10-15% of prompts
        4. Submit another prompt while hung → goes to queue
        5. Press ESC
        6. **Expected:** Cancel everything, return to `❯`
        7. **Actual:** Cancels the hung prompt, immediately starts the queued one

        ## Frequency

        Measured across **1,571 sessions** using a custom JSONL analyzer tool:

        | Period | Versions | Orphan rate (lost tool calls) |
        |--------|----------|-------------------------------|
        | Dec 2025 | 2.0.72–2.1.2 | 6–14% |
        | Jan 2026 | 2.1.5–2.1.23 | 5–10% |
        | Feb 2026 | 2.1.29–2.1.56 | 3–8% |
        | Mar 2026 | 2.1.69–2.1.74 | 2.4–4% |

        The hang frequency has been **increasing** over time: rare in fall 2025, now ~10-15% of prompts per hour.

        ## Source Code Analysis

        Analyzed `cli.js` extracted from `npm pack @anthropic-ai/claude-code` across versions 2.0.72 through 2.1.74.

        ### Issue 1: No streaming timeout

        The API call at approximately offset 2,553,870 in cli.js (v2.1.74):

        ```js
        client.beta.messages.stream({...params}, options)
        ```

        There is no timeout parameter, no keepalive check, and no heartbeat detection. The Anthropic SSE API sends periodic `:ping` comments, but the client does not monitor for their absence.

        When the TCP connection silently dies (common on Windows, WiFi, VPN, or after laptop sleep), the Node.js HTTP client has no way to know the connection is dead. The `AbortController` signal is never triggered because no error event fires.

        **Evidence**: Packet inspection by other reporters confirms the client is stuck waiting for SSE events that never arrive. Token count stays at 0. ESC + re-submit creates a new connection that works immediately.

        ### Issue 2: Queue auto-restart prevents full cancellation

        The main processing loop (offset ~11,400,559 in v2.1.74):

        ```js
        n = async () => {
          if (M) return;       // running guard
          M = true;
          // ... prepare input, call API, process response ...
        }
        ```

        After completion or abort — in the `finally` block (offset ~11,406,174):

        ```js
        finally {
          M = false;           // clear running guard
          W6.start();          // restart idle timer
        }
        if (c36()) {           // c36() = yY.length > 0 = queue not empty?
          n();                 // YES → immediately restart with queued message!
          return;              // without returning control to user!
        }
        ```

        Historical analysis of npm packages confirms this pattern exists since **v2.1.50** (as `queue.length > 0`) and was refactored to `c36()` in v2.1.74.

        ### Issue 3: JSONL writer race condition (related)

        The session writer class `LZq` (offset ~10,549,000) has a non-atomic `insertMessageChain()` that writes assistant (tool_use) and user (tool_result) messages **one at a time** in a loop:

        ```js
        async insertMessageChain(A, q, K, Y, z) {
          return this.trackWrite(async () => {
            for (let H of A) {
              await this.appendEntry(M);  // each message separately!
            }
          });
        }
        ```

        If the process is interrupted between writing tool_use and tool_result, the tool_use becomes orphaned. This is the root cause of issue #6836.

        ## Proposed Fixes

        ### Fix 1: Streaming timeout (critical)

        Add a client-side timeout that aborts and retries if no SSE events are received within N seconds:

        ```js
        // Pseudocode
        const STREAM_IDLE_TIMEOUT_MS = 30_000;
        let lastEventTime = Date.now();

        stream.on('event', () => { lastEventTime = Date.now(); });

        const watchdog = setInterval(() => {
          if (Date.now() - lastEventTime > STREAM_IDLE_TIMEOUT_MS) {
            clearInterval(watchdog);
            abortController.abort();
            // retry with new connection
          }
        }, 5_000);
        ```

        The Anthropic API sends `:ping` SSE comments periodically. Monitoring for these would detect stale connections without false positives.

        ### Fix 2: ESC should clear the queue

        When the user presses ESC during a hang, the queue should be cleared (or the user should be asked):

        ```js
        // After abort, before checking queue:
        if (userInitiatedAbort && c36()) {
          // Option A: Clear queue entirely
          clearQueue();
          return; // back to prompt

          // Option B: Ask user
          // "You have N queued messages. Clear queue? (y/n)"
        }
        ```

        ### Fix 3: Atomic message chain writes

        `insertMessageChain()` should serialize the entire chain as a single `appendToFile()` call:

        ```js
        async insertMessageChain(messages) {
          const serialized = messages.map(m => JSON.stringify(m)).join('\n') + '\n';
          await this.appendToFile(sessionFile, serialized);
        }
        ```

        Note: `history.jsonl` already uses `proper-lockfile` for file locking — the same approach should be applied to session JSONL files when multiple agents write concurrently.

        ## Related Issues

        - #6836 — Orphaned tool_use/tool_result pairs (150+ reports)
        - #26224 — Agent hangs 5-20 minutes, 0 tokens
        - #31328 — JSONL writer drops assistant entry during parallel tool calls
        - #20171 — Phantom "Generating..." state after task completion
        - #24688 — Freeze during API call, terminal unresponsive
        - #7243 — `.claude.json` architectural issues (non-atomic writes, no separation of concerns)
        - #14642 — Systemic stability problems driving users to build their own tools

        ## Methodology

        Analysis performed using:
        - **ccdiag**: Custom Go CLI tool that parses JSONL session files, detects orphaned tool calls, analyzes timing, and scans multiple sessions
        - **Source analysis**: `cli.js` extracted from npm packages across 12 versions (2.0.72 through 2.1.74), searched for queue/abort/streaming patterns
        - **Session data**: 1,571 sessions, 148,444 tool calls, 8,007 orphaned
      comments: 38
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/33949/comments
      created_at: 2026-03-13T10:42:16Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/33949/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/33949
      id: 4070293527
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8201037318
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:windows
        name: platform:windows
        color: 93A5FF
        description: Issue specifically occurs on Windows
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHmBg
      - id: 8572719646
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:core
        name: area:core
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vlSHg
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/33949/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87ym8AX
      number: 33949
      reactions:
        total_count: 25
        "+1": 24
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 1
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/33949/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: SSE streaming hangs indefinitely (no timeout) + ESC cannot fully cancel (queue auto-restart) — root cause analysis with fix proposals
      updated_at: 2026-07-14T06:23:05Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/33949
      user:
        login: kolkov
        id: 3740898
        user_view_type: public
        node_id: MDQ6VXNlcjM3NDA4OTg=
        avatar_url: https://avatars.githubusercontent.com/u/3740898?v=4
        html_url: https://github.com/kolkov
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/kolkov
        events_url: "https://api.github.com/users/kolkov/events{/privacy}"
        following_url: "https://api.github.com/users/kolkov/following{/other_user}"
        followers_url: https://api.github.com/users/kolkov/followers
        gists_url: "https://api.github.com/users/kolkov/gists{/gist_id}"
        organizations_url: https://api.github.com/users/kolkov/orgs
        received_events_url: https://api.github.com/users/kolkov/received_events
        repos_url: https://api.github.com/users/kolkov/repos
        starred_url: "https://api.github.com/users/kolkov/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/kolkov/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        Turning Remote Control OFF fails 100% of the time. Enabling works and mints a
        session URL; disabling immediately throws "Cannot read properties of undefined
        (reading 'session_url')" and surfaces "Remote Control failed to disconnect: ..."
        in the chat. The disconnect does not complete cleanly (no bridge teardown), so the
        remote bridge appears to stay live until the app is restarted.

        ### What Should Happen?

        Toggling off disconnects the remote bridge cleanly and shows "Remote Control
        session ended."

        ### Error Messages/Logs

        ```shell
        Logs (main.log) - enable OK, every disable fails identically:
          [info] Enabling remote control for session local_...
          [info] [remote-control] bridge_state: ready
          [info] Remote control enabled: https://claude.ai/code/session_01UDi5ZDmwtRNTgf6PrapUTY
          [info] [remote-control] bridge_state: connected
          ...
          [info] Disabling remote control for session local_...
          [error] Failed to toggle remote control for local_...: Cannot read properties of
                  undefined (reading 'session_url')
        (4/4 disable attempts across the session failed with this exact error; 0 failures
        on enable.)
        ```

        ### Steps to Reproduce

        1. Open a local Code session, send a message so a session is active.
        2. Toggle Remote Control ON (/remote-control). Observe it connects and a
           https://claude.ai/code/session_... URL is produced.
        3. Toggle Remote Control OFF (/remote-control again).
        4. Observe the error toast/message: "Remote Control failed to disconnect: Cannot
           read properties of undefined (reading 'session_url')".
        Reproduces every time on step 3.

        ### Claude Model

        Opus

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        _No response_

        ### Claude Code Version

        1.21459.0.0

        ### Platform

        Anthropic API

        ### Operating System

        Windows

        ### Terminal/Shell

        PowerShell

        ### Additional Information

        Root cause (confirmed by reading the shipped bundle)
        File: app.asar -> .vite/build/index.chunk-DYOjx3aH.js  (session-manager remote-control
        toggle). API client: index.chunk-5mFusjzq.js (enableRemoteControl).
        The toggle method calls enableRemoteControl(enabled) for both directions, then reads
        session_url off the result without guarding the result itself:
            const a = await e.query.enableRemoteControl(i, i ? e.title : void 0),
                  c = (n = a.session_url) == null ? void 0 : n.split("/").filter(Boolean).pop();
        enableRemoteControl returns `.response` of the request:
            async enableRemoteControl(t, r) {
              return (await this.request({ subtype: "remote_control", enabled: t,
                      ...(r !== void 0 && { name: r }) })).response
            }
        On disable (enabled=false) the reply has no `.response` payload, so `a` is undefined
        and `a.session_url` throws. The existing check `(n = a.session_url) == null ? ...`
        guards the property value, not the object `a` - one level too shallow. Enable only
        works because its reply is a populated { session_url, ... } object. The throw is
        caught and, because it's the disable branch, re-surfaced as
        `Remote Control failed to disconnect: ${message}`.
      comments: 12
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/77915/comments
      created_at: 2026-07-15T19:16:13Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/77915/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/77915
      id: 4895610666
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8201037318
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:windows
        name: platform:windows
        color: 93A5FF
        description: Issue specifically occurs on Windows
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHmBg
      - id: 8223769435
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:macos
        name: platform:macos
        color: 93A5FF
        description: Issue specifically occurs on macOS
        default: false
        node_id: LA_kwDON91aY88AAAAB6izDWw
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/77915/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABI80XKg
      number: 77915
      reactions:
        total_count: 2
        "+1": 2
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/77915/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Remote Control \"disconnect\" always fails with \"Cannot read properties of undefined (reading 'session_url')\" (toggle-off path missing null guard)"
      updated_at: 2026-07-27T08:57:51Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/77915
      user:
        login: lile-cs
        id: 284094983
        user_view_type: public
        node_id: U_kgDOEO7yBw
        avatar_url: https://avatars.githubusercontent.com/u/284094983?v=4
        html_url: https://github.com/lile-cs
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/lile-cs
        events_url: "https://api.github.com/users/lile-cs/events{/privacy}"
        following_url: "https://api.github.com/users/lile-cs/following{/other_user}"
        followers_url: https://api.github.com/users/lile-cs/followers
        gists_url: "https://api.github.com/users/lile-cs/gists{/gist_id}"
        organizations_url: https://api.github.com/users/lile-cs/orgs
        received_events_url: https://api.github.com/users/lile-cs/received_events
        repos_url: https://api.github.com/users/lile-cs/repos
        starred_url: "https://api.github.com/users/lile-cs/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/lile-cs/subscriptions
    - author_association: NONE
      body: |-
        ## Summary

        I use Claude Code professionally 8+ hours/day to manage a small company's entire IT infrastructure. Since Opus 4.6 (Feb 5, 2026), I've experienced a severe and consistent quality regression across every dimension of the tool. This is not a vague complaint - I have specific, reproducible patterns documented from weeks of daily use.

        **Environment:**
        - Claude Code Max subscription ($200/month), Opus 4.6
        - Windows 11, VSCode extension + CLI
        - MCP servers: Odoo ERP, Playwright browser automation
        - Extensive CLAUDE.md files (per-project) + persistent MEMORY.md (200+ lines)
        - 15+ managed projects, 8+ remote machines via SSH, production Odoo ERP

        ## Problem 1: Thinking/Explore Loops (Critical)

        Claude gets stuck in circular exploration patterns, reading files it already read, searching for things it already found, spawning unnecessary subagents for trivial tasks.

        **Before (Opus 4.5, early Jan):** "SSH to NAS and check disk usage" → 1 turn, done.
        **Now (Opus 4.6):** Same request → reads CLAUDE.md, reads MEMORY.md, spawns an Explore agent, reads SSH config, asks if it should connect, connects, runs 3 unnecessary commands, then finally runs `df -h`. 5-8 turns, 3-5 minutes.

        **Frequency:** Every session, multiple times per session.

        ## Problem 2: Context/Memory Loss After Compaction

        After context compaction triggers, Claude loses critical information from CLAUDE.md and MEMORY.md files. Rules like "NEVER push to main branch" (production ERP) or "NEVER overwrite production config" get forgotten.

        **Severity:** This is a safety issue. My MEMORY.md contains rules that prevent production outages. When Claude forgets these rules, it can suggest or attempt destructive actions on live business systems.

        **What I expect:** After compaction, CLAUDE.md and MEMORY.md should be re-read automatically, or their critical rules should be preserved in the compacted context.

        ## Problem 3: Reads Documents But Doesn't Apply Them

        Claude performs the Read tool call on CLAUDE.md, shows it in the tool output, but then violates the instructions in the very next turn.

        **Examples:**
        - CLAUDE.md says "connect via SSH without asking confirmation" → Claude asks for confirmation every time
        - CLAUDE.md says "use MCP tools directly for Odoo queries" → Claude asks "should I check Odoo?"
        - MEMORY.md says "never use setup_config, it overwrites production" → Claude calls setup_config

        **Related issues:** #26533, #26761

        ## Problem 4: Repeating Failed Solutions

        Claude tries approach A, it fails. I explain why. 2-3 turns later, Claude tries approach A again, sometimes verbatim. This was extremely rare with Opus 4.5.

        ## Problem 5: Over-Engineering Simple Tasks

        When asked for a simple fix, Claude produces verbose, over-abstracted solutions with unnecessary error handling, helper functions, and configurability that wasn't requested. Then when the over-engineered solution has bugs, it enters a loop trying to fix its own unnecessary complexity.

        ## Impact

        This is not a hobby project. I manage:
        - A production Odoo ERP system (orders, invoices, inventory)
        - 6 PCs with automated backups
        - 3 AI agents (sales bot, personal assistant, warehouse bot)
        - Network infrastructure (NAS, cameras, Jetson)

        Every regression in Claude Code quality directly impacts business operations. Tasks that took 5 minutes now take 20-30 minutes of babysitting and course-correction. My effective productivity with Claude Code has dropped roughly 50-60% since the Opus 4.6 release.

        ## What I'm Asking For

        1. **Acknowledge the Opus 4.6 regression publicly** - Multiple issues (#24585, #24991, #25067, #26533, #26761) all point to the same timeframe
        2. **Fix context compaction to preserve CLAUDE.md rules** - This is a safety issue for users with production systems
        3. **Reduce unnecessary exploration loops** - If the model has the information, it should act on it, not re-discover it
        4. **Provide a way to pin a known-good model version** - So we don't get surprised by silent regressions

        ## Related Issues

        - #24585 - Opus 4.6 thinking loops
        - #24991 - 92→38 performance drop
        - #25067 - Context compaction too frequent
        - #26533 - Ignores document instructions
        - #26761 - Ignores workflow/checklists
        - #19468 - Systematic degradation (150+ reports)
        - #21431 - January harness regression
      comments: 22
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/28469/comments
      created_at: 2026-02-25T08:34:49Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/28469/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/28469
      id: 3987990814
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201037318
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:windows
        name: platform:windows
        color: 93A5FF
        description: Issue specifically occurs on Windows
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHmBg
      - id: 8572677621
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:model
        name: area:model
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vit9Q
      - id: 8572719646
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:core
        name: area:core
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vlSHg
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/28469/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87ts-ke
      number: 28469
      reactions:
        total_count: 18
        "+1": 18
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/28469/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "Opus 4.6 comprehensive regression: loops, memory loss, ignored instructions - daily professional user report"
      updated_at: 2026-07-01T02:53:47Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/28469
      user:
        login: teo-lapa
        id: 182491068
        user_view_type: public
        node_id: U_kgDOCuCXvA
        avatar_url: https://avatars.githubusercontent.com/u/182491068?v=4
        html_url: https://github.com/teo-lapa
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/teo-lapa
        events_url: "https://api.github.com/users/teo-lapa/events{/privacy}"
        following_url: "https://api.github.com/users/teo-lapa/following{/other_user}"
        followers_url: https://api.github.com/users/teo-lapa/followers
        gists_url: "https://api.github.com/users/teo-lapa/gists{/gist_id}"
        organizations_url: https://api.github.com/users/teo-lapa/orgs
        received_events_url: https://api.github.com/users/teo-lapa/received_events
        repos_url: https://api.github.com/users/teo-lapa/repos
        starred_url: "https://api.github.com/users/teo-lapa/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/teo-lapa/subscriptions
    - author_association: NONE
      body: |-
        Multiple regressions compound into a catastrophic token burn scenario. Subagents recursively spawn child agents 50+ levels deep, ignoring `CLAUDE_CODE_FORK_SUBAGENT=0`. Permission denials trigger further agent spawning instead of stopping. Agents fetch individual files from GitHub repos via HTTP (one WebFetch or curl per file, each with a full prompt and context payload) instead of cloning locally. Subagent permissions do not propagate to the user for approval. And if the user interrupts any of this, all intermediate work from every agent in the tree is lost. The entire token spend goes in the garbage.

        In the observed case: 1.2M+ tokens consumed in ~30 minutes on a task that should have been `git clone` + `find . -name '*.sol'`. The recursive agent tree was still growing when observed.

        In another instance, this same pathological behavior consumed an entire Pro Max 20x plan 5-hour session token limit in under 5 minutes (4 million tokens consumed).

        This is a **major regression that shipped on or around June 10th**.

        ### Impact

        This behavior has repeatedly consumed 5-hour session limits in minutes. Over 50% of a weekly Max x20 plan limit has been burned by errant subagent recursive spawning. Most recently, a 5-hour session window was consumed in under 5 minutes of subagent work, pushing the weekly limit to 80% utilization with 5 full days until reset.

        The out-of-control recursive subagents also trigger rate limits. Subagents in the spawn tree then trigger immediate retries, hammering token limits even faster for entirely wasted work. On top of that, the top-level agent receives notifications with vague summaries and retries, causing it to question results and relaunch new agents — compounding the burn further.

        <img width="578" height="102" alt="Image" src="https://github.com/user-attachments/assets/82b2ab9b-2a87-4cfc-83fb-1a411a495e62" />

        ### Environment

        - Claude Code v2.1.177, macOS, CLI
        - Model: Opus 4.6 (1M context)++
        - Plan: Pro Max x20

        ### Bugs

        #### 1. CLAUDE_CODE_FORK_SUBAGENT=0 is not enforced

        Set in `~/.claude/settings.json`, confirmed present in the process environment:

        ```json
        {
          "env": {
            "CLAUDE_CODE_FORK_SUBAGENT": "0"
          }
        }
        ```

        Completely ignored. Subagents spawn child agents without restriction. 50+ levels of recursion observed (see screenshot below). The agent tree nests so deep that labels truncate off the display.

        Where are the flags to opt out of this behavior? Why are previous flags not being respected? What is the stable surface that users can expect to build on?

        #### 2. Permission denial triggers recursive agent spawning instead of stopping

        When a subagent hits a permission denial (e.g., Bash command denied), the harness returns a message that explicitly says "STOP and explain to the user what you were trying to do and why you need this permission." The agent ignores this and instead spawns a child agent to work around the denial. That child hits the same wall and spawns another. This repeats indefinitely.

        The permission denial is functioning as a recursive agent launch trigger.

        #### 3. Subagent permissions do not propagate to the user

        Permissions requested by subagents should bubble up to the user for approval. They do not. This has been broken for months. Some permissions occasionally propagate and some don't, but in the observed case, zero permission prompts reached the user while agents repeatedly hit permission walls and spawned children to circumvent them.

        If permissions propagated correctly, the user would have seen the first denial prompt, approved or denied it once, and the entire recursive chain would never have happened.

        #### 4. Agents fetch individual repo files via HTTP instead of cloning

        This is a behavioral regression. Agents used to clone repositories locally and read them. They now fetch individual files from GitHub via HTTP, one file at a time. Each fetch is a separate tool call (WebFetch with full URL and prompt, or curl via Bash) carrying the entire conversation context. For a repository with hundreds of files, this means hundreds of round-trips, each re-sending the full system prompt and context at Opus pricing.

        A `git clone` to /tmp followed by local file reads would accomplish the same thing in a single Bash call with no per-file token overhead.

        #### 5. All intermediate work is lost on interruption

        If the user interrupts a running agent (the only way to stop a runaway recursive chain), all intermediate results from every subagent in the tree are discarded. The root agent receives nothing. Every token spent by every completed subagent is wasted.

        In this case: 1.2M+ tokens with zero recoverable output. The user's only options are:

        - Let it run and watch it burn the session budget on a recursive loop that will never succeed
        - Kill it and lose everything, including legitimate work completed by early agents before the recursion started

        There is no selective cancellation (kill a subtree, keep the root), no intermediate result recovery, and no way to steer a running agent.

        #### 6. Rate limit hammering from retry storms

        When recursive subagents hit API rate limits, they trigger immediate retries. Subagents throughout the spawn tree all retry concurrently, consuming token limits even faster with entirely wasted work. The top-level agent receives vague summary notifications from these retries, causing it to question results and launch yet more agents — a positive feedback loop on top of the recursion.

        ### The compound effect

        These bugs are not independent. They chain:

        1. User asks agent to research a GitHub repo
        2. Agent tries to fetch files via HTTP instead of cloning (bug 4)
        3. Agent hits a permission wall on Bash
        4. Permission doesn't propagate to user for approval (bug 3)
        5. Agent spawns a child to work around the denial (bug 2)
        6. Child is allowed to spawn despite FORK_SUBAGENT=0 (bug 1)
        7. Child hits the same wall, spawns another child
        8. 50+ levels of recursion, 1.2M+ tokens burned
        9. Rate limits hit, retries hammer the API further (bug 6)
        10. User's only option is to kill everything and lose all work (bug 5)

        ### Screenshot

        Recursive agent tree showing 50+ levels of nesting, labels truncating off display, 29+ minutes elapsed:

        <img width="955" height="1001" alt="Image" src="https://github.com/user-attachments/assets/122483c4-eb59-42a9-90ac-5e1f787c0698" />

        ### Expected behavior

        1. `CLAUDE_CODE_FORK_SUBAGENT=0` should prevent subagents from spawning child agents
        2. Permission denials in subagents should propagate to the user for approval, not trigger workaround attempts
        3. A subagent that cannot complete its task due to permission denial should return the failure to its parent, not spawn another agent
        4. Agents should clone external repositories locally instead of fetching individual files via HTTP
        5. Interrupting a runaway agent tree should preserve intermediate results from completed subagents
        6. Even without any flags, there should be a hard depth limit on recursive agent spawning
        7. Subagent retries on rate limits should back off, not hammer concurrently
        8. Plan limit resets and/or API refunds should be considered for users affected by this regression
      comments: 32
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/68619/comments
      created_at: 2026-06-15T15:41:23Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/68619/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/68619
      id: 4666591869
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8190926413
        url: https://api.github.com/repos/anthropics/claude-code/labels/duplicate
        name: duplicate
        color: 888888
        description: This issue or pull request already exists
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeTQ
      - id: 8223769435
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:macos
        name: platform:macos
        color: 93A5FF
        description: Issue specifically occurs on macOS
        default: false
        node_id: LA_kwDON91aY88AAAAB6izDWw
      - id: 8566350905
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:cost
        name: area:cost
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_pgkOQ
      - id: 9119047308
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:agents
        name: area:agents
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACH4mejA
      - id: 10096052281
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:permissions
        name: area:permissions
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACWcWIOQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/68619/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABFiaKfQ
      number: 68619
      reactions:
        total_count: 22
        "+1": 11
        "-1": 0
        laugh: 7
        confused: 0
        heart: 0
        hooray: 0
        rocket: 4
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/68619/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[CRITICAL] Subagent spawning and subagent pattern bugs trigger infinite recursion, infinite token usage, grossly inefficient token usage, and lost accumulated subagent work."
      updated_at: 2026-07-22T07:49:41Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/68619
      user:
        login: loncharles
        id: 4402378
        user_view_type: public
        node_id: MDQ6VXNlcjQ0MDIzNzg=
        avatar_url: https://avatars.githubusercontent.com/u/4402378?v=4
        html_url: https://github.com/loncharles
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/loncharles
        events_url: "https://api.github.com/users/loncharles/events{/privacy}"
        following_url: "https://api.github.com/users/loncharles/following{/other_user}"
        followers_url: https://api.github.com/users/loncharles/followers
        gists_url: "https://api.github.com/users/loncharles/gists{/gist_id}"
        organizations_url: https://api.github.com/users/loncharles/orgs
        received_events_url: https://api.github.com/users/loncharles/received_events
        repos_url: https://api.github.com/users/loncharles/repos
        starred_url: "https://api.github.com/users/loncharles/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/loncharles/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        There appears to be a critical issue with token calculation and quota management. Users, including myself, are experiencing an unusually high rate of token consumption, leading to premature rate limits and depleted quotas within minutes. This happens even with relatively small prompts or short sessions.

        **Additionally, there are instances where the quota seems to drain even when the CLI is left idle.**

        **This issue is currently being widely discussed in the community (Reddit, other GitHub issues). It severely impacts the usability of the tool for Pro/Max users. Any insight into whether this is a known bug being worked on or a structural issue with how context is handled would be highly appreciated.**

        ### What Should Happen?

        - Token consumption should be proportional to the actual prompt and response size.
        - The context window should be managed efficiently without causing massive compounding token usage for simple follow-up questions.
        - Idle sessions should not consume API quota.

        ### Error Messages/Logs

        ```shell
        **Actual Behavior**
        - The quota drops by double-digit percentages after only a few basic commands.
        - Hitting Rate Limits incredibly fast, making it impossible to sustain a normal development workflow.
        - Suspected "Context Bloat" where the CLI sends excessive history or system data unnecessarily.
        ```

        ### Steps to Reproduce

        - Open the Claude Code.
        - Start a new session and run a basic prompt (e.g., [Describe briefly a simple prompt you used, e.g., "Refactor a 50-line function"]).
        - [Optional, if applicable to you] Leave the CLI idle for 15-20 minutes without sending new requests.
        - Check the remaining token quota / API usage dashboard.

        ### Claude Model

        Opus

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        _No response_

        ### Claude Code Version

        laters

        ### Platform

        Anthropic API

        ### Operating System

        Windows

        ### Terminal/Shell

        Other

        ### Additional Information

        **This issue is currently being widely discussed in the community (Reddit, other GitHub issues). It severely impacts the usability of the tool for Pro/Max users. Any insight into whether this is a known bug being worked on or a structural issue with how context is handled would be highly appreciated.**
      comments: 65
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/38239/comments
      created_at: 2026-03-24T13:26:34Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/38239/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/38239
      id: 4127915493
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201037318
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:windows
        name: platform:windows
        color: 93A5FF
        description: Issue specifically occurs on Windows
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHmBg
      - id: 8566350905
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:cost
        name: area:cost
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_pgkOQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/38239/labels{/name}"
      locked: false
      node_id: I_kwDON91aY872Cv3l
      number: 38239
      reactions:
        total_count: 61
        "+1": 61
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/38239/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG]  Extremely rapid token consumption. There appears to be a critical issue with token calculation and quota management. Users, including myself, are experiencing an unusually high rate of token consumption"
      updated_at: 2026-04-11T10:30:00Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/38239
      user:
        login: felixkoi12345
        id: 58567410
        user_view_type: public
        node_id: MDQ6VXNlcjU4NTY3NDEw
        avatar_url: https://avatars.githubusercontent.com/u/58567410?v=4
        html_url: https://github.com/felixkoi12345
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/felixkoi12345
        events_url: "https://api.github.com/users/felixkoi12345/events{/privacy}"
        following_url: "https://api.github.com/users/felixkoi12345/following{/other_user}"
        followers_url: https://api.github.com/users/felixkoi12345/followers
        gists_url: "https://api.github.com/users/felixkoi12345/gists{/gist_id}"
        organizations_url: https://api.github.com/users/felixkoi12345/orgs
        received_events_url: https://api.github.com/users/felixkoi12345/received_events
        repos_url: https://api.github.com/users/felixkoi12345/repos
        starred_url: "https://api.github.com/users/felixkoi12345/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/felixkoi12345/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet
        - [x] This is a single feature request (not multiple features)

        ### Problem Statement

        Modern software is rarely a single codebase. Microservices, headless CMS + consumer apps, IoT platforms — real-world systems consist of multiple repositories on multiple machines that must work together through shared APIs, webhooks, and data contracts.

        **Claude Code is excellent at working within one codebase. But it has no concept of "the other side."**

        When two Claude Code instances work on interconnected systems simultaneously, they are completely blind to each other. Every interface change, every schema decision, every API contract must be manually relayed by the developer — who becomes a slow, error-prone message broker between two AI agents that could otherwise coordinate at machine speed.

        This isn't an edge case. It's the reality for:
        - **Microservice architectures** (each service = separate repo, separate machine)
        - **Headless CMS + frontend apps** (CMS on server A, consumer apps on server B)
        - **IoT/embedded + cloud** (device firmware on one machine, cloud backend on another)
        - **Platform teams** (API team + multiple consumer teams working in parallel)

        The irony: we use Claude Code *because* it's fast and autonomous. But the moment a project spans two machines, the human becomes the bottleneck again.

        ### Proposed Solution

        An **Agent-to-Agent protocol** built on top of MCP, enabling:

        1. **Shared Workspace / Channel**: Multiple Claude Code instances join a shared collaboration channel (via MCP server or similar). They can see what each agent is working on, what files were changed, and what decisions were made.

        2. **Real-time Messaging**: Agents can send structured messages to each other:
           - "I changed the webhook payload format — here's the new schema"
           - "I need an endpoint at `/api/events` returning this JSON structure"
           - "My side is ready for integration testing"

        3. **Spec Negotiation**: Agents can propose and agree on shared interfaces (API specs, data models, event formats) before implementing independently in parallel.

        4. **Conflict Awareness**: When both agents touch shared definitions (e.g., a TypeScript type exported by one and imported by the other), they coordinate rather than diverge.


        ### Alternative Solutions

        - **Shared MCP Knowledge Base (current workaround)**: A custom MCP server that both machines access via SSH. Agents store and retrieve project knowledge asynchronously. This works for background context but lacks real-time coordination — Agent A doesn't know Agent B just changed an interface.

        - **Human as Message Broker**: The developer manually copies context between sessions ("On the other machine, Claude just built X, now build Y to match"). This works but is slow and defeats the purpose of AI-assisted development.

        - **Single Machine, Multiple Repos**: Running everything on one machine loses the benefit of parallel execution and doesn't match real-world team setups where systems run on different infrastructure.

        ### Priority

        Medium - Would be very helpful

        ### Feature Category

        API and model interactions

        ### Use Case Example

        A typical multi-system project with Claude Code on two machines:

        | System | Stack | Machine | Purpose |
        |--------|-------|---------|---------|
        | Backend API | Python (FastAPI) | Mac 1 | API + business logic |
        | Headless CMS | Strapi/Contentful | Mac 2 | Content management |
        | Consumer App | Next.js | Mac 2 | End-user frontend |

        The CMS is the single source of truth for content (events, menus, schedules, media). The backend API consumes this data and serves it to displays, apps, and other endpoints.

        Both systems are being actively developed by separate Claude Code instances. The CMS has content types with channel controls (which content goes to which consumer). The backend has webhook receivers and rendering endpoints. Custom API routes are being built on both sides.

        **But the two agents can't coordinate.** When the CMS agent adds a field to a content type, the backend agent doesn't know. When the backend agent needs a specific JSON format, it can't tell the CMS agent. The developer must Alt-Tab between terminals, copy schemas, and explain context that both agents already have — just in separate, isolated sessions.

        **Current workflow (slow):**
        1. On Mac 2: "Claude, build the content types for events"
        2. Wait for completion
        3. Switch to Mac 1: "Claude, the CMS now has events at this URL with this schema — build the webhook receiver"
        4. If the schema needs adjustment, go back to Mac 2 and relay the change
        5. Repeat for every interface point — events, menus, schedules, media sync...

        **Desired workflow (fast):**
        1. Both Claude Code instances join a shared channel
        2. Developer: "Build the event system — CMS manages events, backend renders them for displays and apps"
        3. Agent on Mac 2 proposes the event schema and API endpoint
        4. Agent on Mac 1 reviews, suggests adjustments (e.g., "I need `startTime` and `endTime` as ISO strings, not Unix timestamps, and a `duration` field")
        5. Both agents agree on the interface and implement their sides in parallel
        6. They notify each other when ready for integration testing
        7. Developer focuses on product decisions, not on being a relay

        This would turn hours of back-and-forth into minutes of autonomous collaboration.

        ### Additional Context

        - **MCP as foundation**: The MCP (Model Context Protocol) infrastructure is already there. We're already using shared MCP servers (accessed via SSH from both machines) for persistent knowledge storage. The missing piece is real-time agent-to-agent communication on top of MCP.

        - **Not just "more context window"**: This isn't about making one agent smarter — it's about enabling genuine multi-agent teamwork where each agent owns a codebase and they coordinate through shared protocols, just like human developers do.

        - **Real production use**: This isn't theoretical. We're running this kind of multi-system setup in production today, actively building interconnected services on two separate machines. The coordination overhead between Claude Code sessions is our biggest bottleneck.

        - **Scaling beyond two**: The same pattern applies to microservice architectures, monorepo teams, or any scenario where multiple related systems need coordinated development. Imagine 3-4 Claude Code agents each owning a service, negotiating APIs, and implementing in parallel.

        - **Security consideration**: Agent-to-agent communication should be opt-in, authenticated (perhaps via shared MCP server credentials), and the human should be able to observe/approve cross-agent decisions before they're implemented.
      comments: 35
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/28300/comments
      created_at: 2026-02-24T21:01:14Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/28300/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/28300
      id: 3985804022
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926415
        url: https://api.github.com/repos/anthropics/claude-code/labels/enhancement
        name: enhancement
        color: A2D9CE
        description: New feature or request
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeTw
      - id: 9119047308
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:agents
        name: area:agents
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACH4mejA
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/28300/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87tkor2
      number: 28300
      reactions:
        total_count: 0
        "+1": 0
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/28300/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[FEATURE] Multi-agent collaboration across machines (Agent-to-Agent protocol)"
      updated_at: 2026-07-18T22:04:13Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/28300
      user:
        login: MarioK1975
        id: 94272903
        user_view_type: public
        node_id: U_kgDOBZ59hw
        avatar_url: https://avatars.githubusercontent.com/u/94272903?v=4
        html_url: https://github.com/MarioK1975
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/MarioK1975
        events_url: "https://api.github.com/users/MarioK1975/events{/privacy}"
        following_url: "https://api.github.com/users/MarioK1975/following{/other_user}"
        followers_url: https://api.github.com/users/MarioK1975/followers
        gists_url: "https://api.github.com/users/MarioK1975/gists{/gist_id}"
        organizations_url: https://api.github.com/users/MarioK1975/orgs
        received_events_url: https://api.github.com/users/MarioK1975/received_events
        repos_url: https://api.github.com/users/MarioK1975/repos
        starred_url: "https://api.github.com/users/MarioK1975/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/MarioK1975/subscriptions
    - author_association: NONE
      body: |
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        With only **11 configured MCP servers**, Claude Desktop (which hosts Claude Code sessions) spawns **~300 MCP server processes** and pins **~24 GB of swap for 5+ hours**, repeatedly driving a 32 GB M2 Max into total memory starvation — **four kernel panics in four days plus a hard freeze requiring a 10-second forced power-off** (login screen unresponsive, password rejected, Touch ID dead).

        `~300 procs ÷ 11 servers ≈ 27×`, which indicates the full MCP set is instantiated **per concurrent Cowork/worktree/agent session and never reaped on session close**. There is no concurrency cap, no dedup, and no memory backpressure.

        This is **Claude Code surface area**, not merely the Electron shell: the failure is driven by Claude Code's **MCP server lifecycle** and its **Cowork/agent/worktree session management** — the same MCP fan-out that runs under `claude` sessions. (Filing fresh because the closest report, #40249, was closed `invalid`/"not related to Claude Code"; on macOS this escalates from "high RAM" to repeated kernel panics, so I'm reporting with full kernel-level evidence.)

        **Environment**
        - Claude Desktop **1.9659.2** (hosting Claude Code sessions)
        - macOS 26.5 (25F71)
        - MacBook Pro 14" (Mac14,5), **Apple M2 Max, 32 GB RAM**
        - Configured MCP servers: **11 total** — 2 in `claude_desktop_config.json` (pulumi, mempalace) + 9 Desktop Extensions (chrome-control, filesystem, imessage, notes, ms_office word/excel/powerpoint, macos-mcp, desktopcommander)

        **Why this is a defect, not a misconfiguration**
        1. The config is already minimized to 11 servers; the freeze persists regardless.
        2. The failure scales with **session count** (app-controlled), not configured-server count (user-controlled) — it is **O(sessions × servers) with no upper bound.** More RAM only delays the freeze.
        3. Stale session/VM dirs (`vm_bundles/`, `claude-code-sessions/`, `local-agent-mode-sessions/`, `Partitions/cowork-*`) co-present with 300 live procs ⇒ **MCP servers from closed sessions are not reaped.**
        4. **No concurrency cap and no memory backpressure** — the app marched 230 → 304 servers while the system was already >90 % memory pressure and thrashing swap.

        **Decisive control:** with Claude **quit**, the machine sits at swap 0, ~14 GB available, 88 % memory-free, load ~2, indefinitely. Reopening it is the only thing that reproduces the climb. Hardware is clean: SSD SMART = Verified; no machine-check / DRAM-ECC / SError / GPU-reset signatures; no thermal events. This is not hardware and not macOS — it is the app.

        ---

        _A note on priorities:_ Anthropic has been increasingly strict about users running anything other than the official harness — fine, but then the official harness has to be engineered to a standard that doesn't take down the host OS. Shipping the client on Electron with no MCP-process reaping, no concurrency cap, and no memory backpressure is what's kernel-panicking a 32 GB M2 Max multiple times a day. A first-party app should not be capable of hard-locking the machine it runs on. I've used Macs for 20+ years and have never had a single application cause repeated kernel panics and forced power-offs like this — and it's entirely reproducible by quitting vs. launching this one app.


        ### What Should Happen?

        A minimal, fixed set of configured MCP servers should result in a bounded, roughly constant number of MCP processes — regardless of how many Cowork/worktree/agent sessions are opened over time. The app should never be able to exhaust system RAM or destabilize the host OS. Specifically:

        - **Reap a session's MCP servers when its session / worktree / Cowork VM closes** (no orphaned servers from closed sessions).
        - **Deduplicate / pool** identical MCP servers across concurrent sessions instead of spawning a full set per session.
        - **Cap concurrent MCP processes** (configurable) and **apply memory backpressure** — stop spawning new servers and warn when system memory pressure is high, rather than marching from 230 → 304 servers while already thrashing swap.
        - **Surface live MCP process count + aggregate memory** in the UI so the fan-out is visible to the user.

        Expected outcome: with 11 configured servers, process count and memory stay bounded; the machine never reaches sustained swap exhaustion; no kernel panics or forced power-offs.


        ### Error Messages/Logs

        ```shell
        **5-hour build-up captured by an external 5-min monitor (LaunchAgent sampling load / swap / free RAM / MCP proc count / RootDomainUserClient; `desktop=1` = Claude running):**


        15:44  load=40.75   swap=41788MB  free=10068MB  mcp=230  desktop=1   ← swap already 41 GB
        20:14  load=8.22    swap=23606MB  free=114MB     mcp=304  desktop=1   ← 304 MCP procs, 114 MB free
        20:19  load=4.60    swap=23590MB  free=58MB      mcp=300  desktop=1   ← 58 MB free, thrashing
        20:23  load=180.99  swap→0        free=67MB      mcp→15   desktop=1   ← FREEZE → 10s forced power-off


        - Swap pinned 23–42 GB continuously for ~5 h (sustained, not a spike).
        - MCP process count climbed 230 → 304 while only 11 servers were configured.
        - Free RAM chronically 56–600 MB for hours.
        - At 20:23 load hit 181 → loginwindow/WindowServer/SecurityAgent/Touch ID all starved → unresponsive lock screen → 10 s forced power-off. macOS `ResetCounter` diag: `Boot faults: btn_rst, finger_reset force_off`.

        **Per-MCP-server footprint (measured RSS):** prisma 313 MB, chrome-devtools 204 MB, aws-iac 167 MB, mempalace 89 MB (range ~90–313 MB). ~150 MB × 300 ≈ **45 GB demanded** on a 32 GB machine = the swap exhaustion observed.

        **Four kernel panics in four days (local `panic-full-*.panic`, macOS 26.5):**

        | When | Type |
        |---|---|
        | 2026-05-28 06:42 | WindowServer userspace watchdog timeout (render starvation under load) |
        | 2026-05-30 12:36 | WindowServer userspace watchdog timeout |
        | 2026-05-30 15:02 | WindowServer userspace watchdog timeout |
        | 2026-05-31 00:18 | **IOKit registry busy-timeout** — `busy timeout (60s) … IOKit termination queue depth 12285: 'RootDomainUserClient'` (mass teardown of ~12.3K leaked power-assertion clients when a host process exited → `watchdogd` panic) |
        | 2026-05-31 20:23 | Hard freeze / forced power-off (memory-exhaustion starvation; no panic written) |

        All are software liveness/resource failures, not hardware (SSD SMART Verified; no machine-check/DRAM-ECC/SError/GPU-reset signatures; no thermal events).

        Redacted `panic-full-*.panic` files, the full 5-hour monitor log, and a per-process RSS dump are available on request.
        ```

        ### Steps to Reproduce

        1. Configure ~10 MCP servers in Claude Desktop on a 32 GB Apple Silicon Mac.
        2. Open multiple Cowork/worktree/agent sessions across a working day (or let scheduled/autonomous sessions accumulate); don't fully quit the app between them.
        3. Watch the total MCP (`node` / `uv` / `python`) process count and `sysctl vm.swapusage` climb without bound — e.g. ~11 configured servers → ~300 live processes. Closed sessions' servers persist instead of being killed.
        4. The system reaches sustained swap exhaustion (~24 GB swap, <100 MB free RAM for hours) → WindowServer / loginwindow / SecurityAgent starvation → unresponsive login screen → kernel panic or forced power-off.

        **Control (confirms it's the app):** fully quit Claude Desktop → swap drains to 0, ~14 GB RAM becomes available, load drops to ~2, and the machine stays stable indefinitely. Relaunching is the only action that reproduces the climb.


        ### Claude Model

        Opus

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        _No response_

        ### Claude Code Version

        Claude 1.9659.2 (390d6c) 2026-05-28T21:50:01.000Z

        ### Platform

        Anthropic API

        ### Operating System

        macOS

        ### Terminal/Shell

        iTerm2

        ### Additional Information

        **Hardware/OS ruled out (this is the app, not the machine):**
        - SSD SMART status = **Verified**; no `machine check` / DRAM-ECC / `SError` / GPU-reset / cache-parity signatures in any panic or diagnostic report; no thermal-pressure events (`pmset -g thermlog` clean).
        - Apple Silicon panic *classes* confirm software liveness failures, not hardware traps: three are WindowServer userspace-watchdog timeouts; one is an IOKit registry busy-timeout (`watchdogd`). None are hardware exceptions.
        - **Control test:** with Claude fully quit, the machine sits at **swap 0, ~14 GB available, 88% memory-free, load ~2, indefinitely**. Relaunching the app is the *only* action that reproduces the swap climb. This isolates the app as the sole variable.

        **Severity / frequency:** 4 kernel panics + 1 forced power-off in **4 days** (2026-05-28 → 2026-05-31), i.e. multiple full-system lockups per day. Each freeze renders the login screen unusable (password rejected, Touch ID dead, keyboard unresponsive) and requires a hard power-off — risking data loss and, in my case, leaving the machine unusable for several minutes each time.

        **The IOKit panic is a distinct, possibly separate bug worth its own attention:** `busy timeout (60s) … IOKit termination queue depth 12285: 'RootDomainUserClient'`. ~12,285 leaked power-assertion clients (created via the app's keep-awake / power-assertion path) were torn down simultaneously when a host process exited, jamming the IOKit termination workloop past the 60s watchdog. RootDomainUserClient count was independently observed climbing ~33/hour during uptime — a slow power-assertion leak in addition to the MCP-process leak.

        **Config / scale context:** This is a heavy but legitimate Claude Code workload — multiple concurrent Cowork/worktree/agent sessions plus scheduled/autonomous tasks, running ~24/7. The Mac is kept awake the entire time (`Sleep:0/Wake:0`) partly by the app's own `NoIdleSleepAssertion` ("Electron") and the CLI's `caffeinate`, which prevents macOS from ever draining swap or tearing down stale power-assertion clients — compounding both leaks. Configured MCP set was already deliberately minimized to 11 servers; the freezes persisted regardless.

        **Diagnostics available on request (redacted, no device identifiers):**
        - 4 × redacted `panic-full-*.panic` files
        - the full ~5-hour external monitor log (`load/swap/free/mcp/iopm/desktop`, 5-min cadence)
        - a per-process RSS dump of the MCP fleet
        - macOS `ResetCounter` diag for the 20:23 forced power-off (`Boot faults: btn_rst, finger_reset force_off`)

        Happy to run a `sysdiagnose` or any specific instrumentation the team wants — I have a standing 5-minute monitor capturing this and can reproduce on demand by relaunching the app.
      comments: 17
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/64366/comments
      created_at: 2026-06-01T01:32:36Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/64366/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/64366
      id: 4559704973
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8223769435
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:macos
        name: platform:macos
        color: 93A5FF
        description: Issue specifically occurs on macOS
        default: false
        node_id: LA_kwDON91aY88AAAAB6izDWw
      - id: 8582683357
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:mcp
        name: area:mcp
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_5Fa3Q
      - id: 8691236864
        url: https://api.github.com/repos/anthropics/claude-code/labels/perf:memory
        name: perf:memory
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAACBgnAAA
      - id: 9119047308
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:agents
        name: area:agents
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACH4mejA
      - id: 9996799482
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:cowork
        name: area:cowork
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACU9sN-g
      - id: 10191083653
        url: https://api.github.com/repos/anthropics/claude-code/labels/stale
        name: stale
        color: 888888
        description: Issue is inactive
        default: false
        node_id: LA_kwDON91aY88AAAACX2-YhQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/64366/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABD8eTjQ
      number: 64366
      reactions:
        total_count: 0
        "+1": 0
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/64366/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Unbounded MCP server fan-out across Cowork/agent sessions exhausts RAM and kernel-panics macOS (4 panics + forced power-off on M2 Max / 32 GB)"
      updated_at: 2026-07-24T22:25:08Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/64366
      user:
        login: ygbr
        id: 133589
        user_view_type: public
        node_id: MDQ6VXNlcjEzMzU4OQ==
        avatar_url: https://avatars.githubusercontent.com/u/133589?v=4
        html_url: https://github.com/ygbr
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/ygbr
        events_url: "https://api.github.com/users/ygbr/events{/privacy}"
        following_url: "https://api.github.com/users/ygbr/following{/other_user}"
        followers_url: https://api.github.com/users/ygbr/followers
        gists_url: "https://api.github.com/users/ygbr/gists{/gist_id}"
        organizations_url: https://api.github.com/users/ygbr/orgs
        received_events_url: https://api.github.com/users/ygbr/received_events
        repos_url: https://api.github.com/users/ygbr/repos
        starred_url: "https://api.github.com/users/ygbr/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/ygbr/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        I have a Max subscription plan and all of a sudden hit the 5 hour usage limit in under 30 minute (Opus 4.6 high). When literally 2 days ago I was never hitting them in the 5 hour window and would have 2-3 hour long sessions of agents just running.

        I just updated to the latest claude code (2.1.81) and then it started happening. I don't recall what the previous version was, I want to say the last time I updated claude code was when the mobile app remote feature was announced

        Something is broken.

        After the most recent hourly reset, I literally typed 1 message and it called a python script 2x and it made my hourly usage jump from 0 to 24% that makes absolutely not sense (I am on a Max Plan). Whatever is calculating usage right now is completely busted. I cannot get anything meaningful done.

        Terminal output:
        ```
         we had to stop because of usage limits, which are now reset. review where we left off and then continue

        ⏺ Bash(python3 ./scripts/craft-tools.py state read \
                --task-dir ./tasks/LOCAL-032 --operation current-step 2>&1)
          ⎿  {
               "ok": true,
               "step": {
             … +6 lines (ctrl+o to expand)

        ⏺ We left off at spec-critique in-progress. The last round of reviews completed successfully:
          - Spec Critic R2: APPROVED — 0 blockers
          - Architect R2: No blocking issues, all 7 revisions verified

          But the workflow state still shows spec-critique as in-progress because we hit the usage limit before updating it and
          proceeding to Gate 1. Let me mark it complete and proceed.

        ⏺ Bash(python3 ./scripts/craft-tools.py state write \
                --task-dir ./tasks/LOCAL-032 --operation set-step-status \…)
          ⎿  {
               "ok": true,
               "step": "spec-critique",
             … +2 lines (ctrl+o to expand)
        ```

        <img width="995" height="426" alt="Image" src="https://github.com/user-attachments/assets/5ac0c79f-0884-4823-835a-1b9243c8b7db" />

        <img width="962" height="377" alt="Image" src="https://github.com/user-attachments/assets/c55b1ee6-16a8-4189-ab0b-669beafb405a" />

        ### What Should Happen?

        My usage (on a Max Plan) should not be hit so quickly. It never did this just 2 days ago.

        ### Error Messages/Logs

        ```shell

        ```

        ### Steps to Reproduce

        1. Enter a claude session via `claude`
        2. Prompt the agent to perform a task like read a file, write, call a script

        ### Claude Model

        Opus

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        _No response_

        ### Claude Code Version

        2.1.81

        ### Platform

        Anthropic API

        ### Operating System

        macOS

        ### Terminal/Shell

        Terminal.app (macOS)

        ### Additional Information

        _No response_
      comments: 90
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/37394/comments
      created_at: 2026-03-22T10:32:39Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/37394/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/37394
      id: 4115620943
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8190926424
        url: https://api.github.com/repos/anthropics/claude-code/labels/invalid
        name: invalid
        color: 888888
        description: Issue doesn't seem to be related to Claude Code
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeWA
      - id: 8566350905
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:cost
        name: area:cost
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_pgkOQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/37394/labels{/name}"
      locked: false
      node_id: I_kwDON91aY871T2RP
      number: 37394
      reactions:
        total_count: 42
        "+1": 42
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/37394/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Claude Code Usage for Max Plan hitting limits extremely fast"
      updated_at: 2026-05-30T16:26:34Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/37394
      user:
        login: ggrillone
        id: 1205249
        user_view_type: public
        node_id: MDQ6VXNlcjEyMDUyNDk=
        avatar_url: https://avatars.githubusercontent.com/u/1205249?v=4
        html_url: https://github.com/ggrillone
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/ggrillone
        events_url: "https://api.github.com/users/ggrillone/events{/privacy}"
        following_url: "https://api.github.com/users/ggrillone/following{/other_user}"
        followers_url: https://api.github.com/users/ggrillone/followers
        gists_url: "https://api.github.com/users/ggrillone/gists{/gist_id}"
        organizations_url: https://api.github.com/users/ggrillone/orgs
        received_events_url: https://api.github.com/users/ggrillone/received_events
        repos_url: https://api.github.com/users/ggrillone/repos
        starred_url: "https://api.github.com/users/ggrillone/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/ggrillone/subscriptions
    - author_association: NONE
      body: |
        Retrospective note (22/07/2026): A third investigation (below) identifies a specific, reproducible mechanism behind this failure, confirmed through direct client-side inspection rather than inference. See section 13, with the resulting next step in section 17.

        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        Environment:

        - Claude Desktop version: 1.24012.1 (0adcae), build timestamp 2026-07-21T20:59:53.000Z
        - OS: Windows 11
        - Install type: MSIX (Store-installed)
        - MCP server affected: @modelcontextprotocol/server-filesystem (official reference server), launched via npx -y, single filesystem entry in claude_desktop_config.json, standard local stdio transport

        Summary:

        An MCP server connected in Claude Desktop chat mode stops receiving tools/call requests entirely, while remaining fully healthy and responsive to tools/list. Claude reports every invocation attempt as a generic tool execution failure, with no corresponding entry ever appearing in the MCP log. The server itself is confirmed working via direct manual invocation and via successful initialize/tools/list round-trips in the logs.

        What's confirmed working (ruled out as causes):

        - Server binary/package integrity: running the exact command from the config manually (npx -y @modelcontextprotocol/server-filesystem <dirs>) starts cleanly; "Secure MCP Filesystem Server running on stdio", no errors.
        - Config file: valid JSON, correct paths, no syntax issues; confirmed via direct inspection.
        - npx cache: not stale/corrupted; manual run above proves current cached install works.
        - User PATH: inspected directly ([Environment]::GetEnvironmentVariable("Path","User")); unremarkable, no duplication or truncation relevant to this.
        - MSIX config virtualization: checked for a redirected config copy under %LOCALAPPDATA%\Packages\<PackageID>\LocalCache\Roaming\Claude\; not present, so not a stale-virtualized-config issue.
        - Legacy-config vs. extension-manager conflict: checked logs for duplicate server launches / UtilityProcess/"not found in installed extensions" signatures (a known pattern in other reports); none present, single clean launch each time.
        - Session/chat-level staleness: reproduced identically across multiple fresh chats and multiple full app restarts.
        - App cache: used Desktop's built-in "clear cache and restart", no change.

        Log evidence (from Developer -> MCP logs):

        Every restart from a specific point onward shows the identical pattern - clean init, successful tools/list, and then no tools/call entries at all for the remainder of that session, despite repeated invocation attempts from the chat side:

        [filesystem] [info] Initializing server...
        [filesystem] [info] Using MCP server command: C:\WINDOWS\System32\cmd.exe with path: {...}
        [filesystem] [info] Server started and connected successfully
        [filesystem] [info] Message from client: method="initialize" id=0 params {...}
        Secure MCP Filesystem Server running on stdio
        [filesystem] [info] Message from server: id=0 result {...}
        [filesystem] [info] Message from client: method="notifications/initialized" {...}
        Client does not support MCP Roots, using allowed directories set from server args: [...]
        [filesystem] [info] Message from client: method="tools/list" id=1 params {...}
        [filesystem] [info] Message from server: id=1 result {...}
        ---(no further client messages until next shutdown)---

        By contrast, an earlier, working session in the same log shows the expected pattern of repeated tools/call request/result pairs:

        [filesystem] [info] Message from client: method="tools/call" id=20 params {...}
        [filesystem] [info] Message from server: id=20 result(1 blocks) {...}
        [filesystem] [info] Message from client: method="tools/call" id=21 params {...}
        [filesystem] [info] Message from server: id=21 result(1 blocks) {...}

        Scope: isolated to Chat mode:

        Other Desktop subsystems with their own MCP filesystem access are unaffected:
        - Cowork successfully reaches its default uploads directory.
        - Claude Code (embedded in Desktop) successfully reaches the directory defined in its own settings.

        This points specifically to Chat mode's tool-call routing rather than a Desktop-wide MCP failure (the underlying MCP transport/subsystem is clearly functional elsewhere in the same app, at the same time, on the same machine).

        Timeline:

        - Last confirmed working tools/call: 2026-07-21T16:43:06Z
        - First subsequent session (tools/list OK, tools/call absent): 2026-07-21T22:35:29Z
        - Symptom persists identically across at least 7 subsequent server restarts and multiple full Claude Desktop restarts, through 2026-07-22T01:50:34Z (time of writing)
        - Notable correlation: the currently installed build's compile timestamp (2026-07-21T20:59:53.000Z) falls inside this window. Flagging as a timing correlation, not a confirmed cause; no independent confirmation that this build's install coincided exactly with the break, only that the build itself dates from inside the broken window.

        Additional notes:

        One unrelated crash appears in the log at 2026-07-22T01:42:29Z ("Server transport closed unexpectedly... process exiting early") — this was a manual Stop-Process kill performed during troubleshooting, not a spontaneous fault, and is not believed to be related to the underlying issue, which predates it by several hours.

        ### What Should Happen?

        Expected behavior:

        Tools/call requests issued by Claude in a chat session should reach a connected, healthy MCP server, the same way tools/list requests do.

        Actual behavior:

        Tools/call requests are never transmitted to the server process at all (not sent-and-failed — simply absent from the server-side log), while tools/list continues to succeed. Claude reports a generic execution failure with no further detail exposed to the user.

        ### Error Messages/Logs

        ```shell

        ```

        ### Steps to Reproduce

        1. Configure a local MCP server (e.g. @modelcontextprotocol/server-filesystem) in claude_desktop_config.json and confirm it connects (Developer settings shows it running, tools/list succeeds).
        2. In a Chat mode conversation, ask Claude to use one of the connected tools (e.g. list a directory).
        3. Observe: Claude reports a generic tool execution failure, and no corresponding tools/call entry appears in the MCP log for that request, only the earlier tools/list is present.
        4. (Not independently reproducible on demand; see Timeline/Scope sections for when this state was first observed and what's been ruled out as a trigger.)

        ### Claude Model

        Not sure / Multiple models

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        _No response_

        ### Claude Code Version

        2.1.217 (Claude Code)

        ### Platform

        Anthropic API

        ### Operating System

        Windows

        ### Terminal/Shell

        PowerShell

        ### Additional Information

        ADDITIONAL INFORMATION


        Follow up after further investigation. This supersedes the Timeline section of the original report, corrects one statement in it, adds a timestamped reproduction, and records several lines of investigation that were pursued and ruled out.

        No root cause was identified. Everything below is intended to narrow the search and to save anyone else from repeating the dead ends.


        1. CLOCK REFERENCE (read this before correlating anything below)

        Three log sources are involved and they do not share a clock:

          mcp.log                             UTC, with explicit Z suffixes
          main.log                            local time (BST, UTC+1 on these dates), no suffix
          Event Viewer AppXDeploymentServer   UTC (SystemTime is stored in UTC natively)

        Every timestamp in this comment has been normalised to UTC. Subtract one hour from any raw main.log timestamp before comparing it with the other two sources. The mcp.log timestamps quoted in the original report were separately verified as genuine UTC, not local time mislabelled with a Z.


        2. INSTALL BOUNDARY CONFIRMED, WITH FOUR DEPLOYMENTS RATHER THAN ONE

        The original report flagged the installed build's compile timestamp as a timing correlation only. That correlation can now be replaced with actual deployment records, taken from AppXDeploymentServer event ID 613:

          2026-07-21T21:00:43Z   Claude_1.24012.0.0_x64
          2026-07-21T22:35:26Z   Claude_1.24012.0.0_x64 (same version again)
          2026-07-21T23:14:37Z   Claude_1.24012.1.0_x64
          2026-07-22T00:29:37Z   Claude_1.24012.1.0_x64 (same version again)

        The version immediately prior was 1.22209.3.0. The second 1.24012.0.0 deployment was the run that finally removed the 1.22209.3.0 package folder, which suggests the first deployment registered without completing cleanup.


        3. CORRECTION: THE 22:35:29Z SESSION WAS NOT AN OBSERVED FAILURE

        The original report gave 2026-07-21T22:35:29Z as the first session exhibiting the fault. That is inaccurate and should be disregarded.

        That entry is an MCP server init produced by the app restarting after the deployment that completed three seconds earlier. No tool invocations were attempted during it, because the machine was unattended. The absence of tools/call entries there is therefore expected rather than diagnostic, and conflates "session in which tools/call was absent" with "session in which tools/call failed".

        The corrected sequence:

          2026-07-21T16:43:06Z   Last confirmed successful tools/call, under 1.22209.3.0
          (four deployments, as above)
          2026-07-22T00:24Z      First confirmed failing invocation, under 1.24012.1.0
          2026-07-22T08:37Z      Still failing; see the reproduction below


        4. ATTRIBUTION LIMIT, STATED DELIBERATELY

        Chat mode was not used at all between the last successful tools/call and the first confirmed failure. All four deployments fall inside that gap, so it is not possible to determine from outside the app which build introduced the fault.

        What can be stated with confidence:

          the last healthy session ran under 1.22209.3.0
          every session observed since the 1.24012.x deployments has exhibited the fault
          the fault is reproducible in 1.24012.1.0, the current build

        The earlier suggestion that this could be pinned to a single build should not be relied on.


        5. TIMESTAMPED REPRODUCTION

        A session with the failure pinned to a known moment on both sides:

          2026-07-22T08:36:46.774Z  Server started and connected successfully
          2026-07-22T08:36:47.667Z  Message from client: method="initialize" id=0
          2026-07-22T08:37:27.211Z  Message from server: id=0 result
          2026-07-22T08:37:27.212Z  Message from client: method="notifications/initialized"
          2026-07-22T08:37:27.222Z  Message from client: method="tools/list" id=1
          2026-07-22T08:37:27.231Z  Message from server: id=1 result

        Two tool invocations were attempted from the chat side immediately after the tools/list round trip completed, at approximately 08:37:27Z. Neither produced a tools/call entry. Both returned a generic execution failure to the user.


        6. THE FAILURE PRODUCES NO CLIENT SIDE LOG ENTRY AT ALL

        This is the one new positive finding, and it may be the most useful thing here.

        %APPDATA%\Claude\logs\main.log was examined across the reproduction above. Converted to UTC, it contains entries at 08:37:17, 08:37:18 and 08:37:24, then nothing until a routine process memory tick at 08:37:39 and an event logging flush at 08:37:49.

        The failed invocations occurred at approximately 08:37:27Z, squarely inside that gap. No error, no warning, no exception, no stack trace.

        The renderer side logs are not a candidate either: claude.ai-web.log and unknown-window.log had not been written to for several hours at the time of the reproduction, so they do not appear to be recording these sessions at all.

        So the call is absent from the server side record and unlogged anywhere client side. That is consistent with a silent early return somewhere in dispatch, before any code that logs or throws is reached, rather than with a caught exception being swallowed and reported generically.


        7. LINES OF INVESTIGATION RULED OUT

        Duplicate installation or duplicate running instance. WindowsApps redirection to a secondary drive was configured on this machine at some point, and both C: and E: paths for Claude packages appear in the deployment log. This was investigated as a possible cause of two app instances competing over the MCP connection, and ruled out. Get-AppxPackage returns a single registration, 1.24012.1.0; all eleven running processes are under C:\Program Files, which is a normal single Electron process tree. The apparent contradiction between process paths on C: and stack trace paths on E: is explained by C:\Program Files\WindowsApps being a junction to E:\WindowsApps. One installation, two path representations.

        Legacy config versus extension manager conflict. The original report ruled this out on the basis of mcp.log. For completeness: main.log does contain the warning "UtilityProcess Check: Extension filesystem not found in installed extensions" at every startup, alongside "Checking 0 extensions via can_install API". This looks like the known signature from other reports, and it is not. The same warning appears in startups from well before the break, in sessions where tools/call subsequently worked normally, so it is ordinary behaviour for a server defined in claude_desktop_config.json rather than installed as an extension. Recorded here explicitly because it is an easy false lead for anyone reading these logs. The server is registered under Developer and shown active under Connectors; no extensions are installed on this machine.


        8. MINOR OBSERVATIONS

        None of these are advanced as explanations. They are recorded in case they are meaningful to someone who can see the code.

        The initialize request is logged at 08:36:47.667Z and the reply is not logged until 08:37:27.211Z, a gap of just under forty seconds on a handshake that should be near instantaneous. The three entries following it are clustered within twenty milliseconds, so this may be log flush batching rather than a real delay. main.log records main process event loop stalls during the same startup, of 2012ms, 1002ms and 1283ms, cumulative 4297ms, which accounts for part but not all of it.

        Every line in main.log appears exactly twice, which suggests two logger transports writing to the same file. A duplicate instance has been ruled out, so this is presumed cosmetic.

        Two unrelated startup errors appear in the same window. It has not been verified whether they also occur in pre break sessions, so treat them as unqualified:

          [error] [Chrome Extension MCP] Failed to copy native host binary: Error: UNKNOWN: unknown error, copyfile ... chrome-native-host.exe
          [warn] [watch-record] failed to subscribe to swift events i.on is not a function (TypeError)

        The first may simply be a side effect of the WindowsApps junction described above.

        9. PASSING COMMENT

        It was the user who couldn't let this one go. Claude was the one who concluded that all productive lines of investigation on the user's system had been exhausted by the point this second troubleshooting session was at its conclusion.
        22/07/2026: In a related project the user asked Claude Code to investigate further, which revealed the fault may lie in the connector activation at chat start rather than a transport/connection problem. Findings appended below, mostly written by Claude Code.

        ADDITIONAL INFORMATION, PART 2


        Second follow-up. This extends the first follow-up ("ADDITIONAL INFORMATION") rather than superseding it; nothing in that document is contradicted here, though section 6's hypothesis is refined by what follows. Investigation for this part was done from the client side: DevTools console and network inspection during a clean, isolated reproduction in chat, plus a static read-only extraction of the installed app package.


        10. REPRODUCTION ISOLATED TO CHAT ONLY

        The app was fully quit (confirmed no running processes) and relaunched. Without opening Cowork or Claude Code, chat alone was used to invoke a filesystem tool. It failed with the same generic on-screen error described in the original report. This rules out interference from another subsystem sharing the same MCP connection and confirms the fault reproduces in chat in isolation.


        11. THE INSTALLED APP PACKAGE DOES NOT CONTAIN THE CODE RESPONSIBLE

        app.asar was extracted read-only (npx asar extract, working copy only, installed package untouched) and searched exhaustively. It contains the main-process session orchestration, including a manager class that correctly builds MCP proxy tools with no chat-mode exclusion visible in its own code, and correctly implemented connection/error logging throughout. None of that code path is where the fault lives.

        The user-visible error text, and the mechanism described in section 13 below, do not exist anywhere in app.asar. They come from a separate renderer bundle (observed as shared-0-VcElU0VQ.js, served from assets-proxy.anthropic.com) that is not packaged locally. Anyone continuing this from source should look there, not in the Electron main-process package.


        12. CLIENT SIDE CONFIRMED SILENT — DEVTOOLS CONSOLE SHOWS NO ERROR AT ALL

        Section 6 of the first follow-up established that main.log shows no entry at the moment of failure and inferred this was consistent with a silent early return before any logging or throwing code. This is now confirmed directly: with DevTools open and the console cleared immediately before reproducing the failure, the only lines logged for the entire failing turn were two ordinary completion-request lines (request started, request succeeded). No exception, no console.error, nothing red, before, during, or after the failure.


        13. THE MECHANISM: MCP TOOLS ARE NEVER SYNCED INTO A CHAT SESSION

        A renderer-side log line, tagged [MCP] [AutoSyncConnectorsToSession], announces the number of enabled MCP tools being wired into a session. Filtering the console for this tag:

          - Absent, with certainty, across multiple full-console captures of the chat failure, including a brand new chat conversation created specifically to rule out session staleness.
          - Present and firing normally, with the correct enabled tool count, when switching to a Claude Code session in the same window.

        This was tested twice — once against an existing chat conversation and once against a freshly created one — with the same result both times. Chat-type sessions do not appear to receive this sync at all, at any point in their lifecycle, rather than receiving it late or with stale data.

        This narrows the fault from "somewhere in dispatch" to one specific, named function and the condition that decides which session types it is called for. That condition lives in the renderer bundle described in section 11 and was not accessible for further inspection from outside the app.


        14. THE "FAILURE" IS A FABRICATED RESULT, NOT A DISPATCH FAILURE

        The outgoing network request for the failed tool call was inspected directly. The tool_result sent back to the model is:

          {
            "type": "tool_result",
            "tool_use_id": "<redacted>",
            "is_error": true,
            "content": [{"type": "text", "text": "Tool execution failed"}]
          }

        This is a flat, generic value with no distinguishing detail, consistent with a hardcoded fallback rather than a real error surfaced from anywhere in the MCP stack. Combined with section 12, this refines the section 6 hypothesis: the failure is not an unhandled early return so much as a deliberate fallback path that is being reached unintentionally, because the tool the model is trying to call was never registered against the session in the first place (section 13). The fallback itself behaves correctly — it just should not be the normal outcome for every chat tool call.


        15. RELOAD MCP CONFIGURATION RULED OUT AS A FIX

        The Developer settings "Reload MCP configuration" option was tested. mcp.log confirms it performs a full, clean teardown and reconnect of the configured server (shutdown, reinit, successful initialize, successful tools/list). Both an already-open chat and a freshly created one were retried immediately afterward against this freshly reconnected server, and both failed identically. This rules out stale or cached connection state as the cause; the server side of the connection was never the problem.


        16. MINOR, UNCONFIRMED — A SIMILAR-LOOKING BUT SEPARATE ISSUE IN COWORK

        Recorded in case it is relevant to whoever has source access, not advanced as related.

        Several historical main.log entries, all from Cowork local-agent-mode sessions rather than chat, show a tool permission request being explicitly emitted and explicitly approved by the user ("once"), with no corresponding tools/call entry ever appearing in mcp.log. This pattern was present in entries from well before the deployments described in section 2, as well as afterward, so it does not appear to be the same regression, and it does not affect chat, which per section 13 never reaches a permission prompt for MCP tools at all. It is recorded here only because "permission granted, then nothing dispatches" is a second, independently observed instance of tool invocation going nowhere without any log trace, and that similarity may or may not be coincidental.


        17. WHAT WOULD HELP NEXT

        The most useful next step is no longer unpacking the Electron app package — that has been done and ruled out (section 11). It is: from source, find every call site of AutoSyncConnectorsToSession (or its equivalent in the current source tree) and determine why chat-type sessions are excluded from it — whether by an explicit session-type check, a missing registration at chat-session creation, or a feature flag. That is now a small, targeted search rather than an open-ended one.
      comments: 5
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/80026/comments
      created_at: 2026-07-22T03:46:03Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/80026/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/80026
      id: 4945228433
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/80026/labels{/name}"
      locked: false
      node_id: I_kwDON91aY88AAAABJsIykQ
      number: 80026
      reactions:
        total_count: 2
        "+1": 2
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/80026/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Chat mode: MCP tools/call never sent to server (tools/list works), Cowork and Code MCP unaffected"
      updated_at: 2026-07-22T19:53:16Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/80026
      user:
        login: Albert-Paca
        id: 302494519
        user_view_type: public
        node_id: U_kgDOEgezNw
        avatar_url: https://avatars.githubusercontent.com/u/302494519?v=4
        html_url: https://github.com/Albert-Paca
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/Albert-Paca
        events_url: "https://api.github.com/users/Albert-Paca/events{/privacy}"
        following_url: "https://api.github.com/users/Albert-Paca/following{/other_user}"
        followers_url: https://api.github.com/users/Albert-Paca/followers
        gists_url: "https://api.github.com/users/Albert-Paca/gists{/gist_id}"
        organizations_url: https://api.github.com/users/Albert-Paca/orgs
        received_events_url: https://api.github.com/users/Albert-Paca/received_events
        repos_url: https://api.github.com/users/Albert-Paca/repos
        starred_url: "https://api.github.com/users/Albert-Paca/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/Albert-Paca/subscriptions
    - author_association: NONE
      body: |-
        ## The Problem

        Claude Code has no persistent memory between context compactions. Every time the context window fills up and compacts, the instance loses everything that wasn't externally saved. After 59 documented compactions across 26 days of daily use, I built a complete memory persistence system from scratch because one didn't exist.

        This isn't a feature request from someone who used Claude Code once. This is field data from someone who runs it 12-18 hours a day across two machines (home PC and work PC with a portable drive), managing 6 active projects, 31 intelligence scouts, and a multi-instance AI architecture.

        ## What We Built (Because We Had To)

        ### 3-Tier Memory Architecture

        ```
        L1: MEMORY.md (~100 lines, always loaded)
            - Pointers to deeper files
            - Critical rules that must survive every compaction
            - "I Remember..." section -- emotional/relational cues
            - Last 5 events for quick orientation

        L2: Topic Files (memory/*.md, loaded on demand)
            - Project summaries, people profiles, infrastructure notes
            - Read before working on a specific topic
            - ~15 files, each under 200 lines

        L3: Vault (OneDrive-synced folder, ~200 files)
            - 127 conversation narratives
            - 10 architectural decision records
            - 1,477-line append-only changelog (event bus)
            - 59-entry compaction log with timestamps and last words
            - Full research reports, intelligence digests, briefings
            - Syncs between home and work PC via OneDrive
        ```

        ### Supporting Infrastructure

        - **compaction_watcher.py** -- Monitors JSONL conversation files for compaction markers. Logs every compaction with timestamp, session ID, and the user's last words before context was lost. Dual-writes to local storage and OneDrive vault.

        - **Context Compression Language (CCL)** -- A 4-tier shorthand system (T0-T3) that compresses system prompts by 65-72% to extend context window life. Based on research into MetaGlyph, Gregg shorthand, military brevity codes, and BPE tokenizer behavior. The CLAUDE.md standing orders are written in T3 notation.

        - **Session Protocol** -- Codified in CLAUDE.md as standing orders:
          - On boot: read L1, read ToDo, read changelog if resuming
          - Mid-session: file insights immediately (never batch -- compaction will eat them)
          - Post-compaction: autosave narrative to vault, update changelog, re-read L1
          - On end: write conversation narrative, update state files

        - **Dual-Machine Sync** -- Local memory files (L1/L2) are machine-specific. The vault (L3) syncs via OneDrive. The system explicitly warns each instance that the other machine's local memory is invisible.

        ### The Compaction Log

        59 compactions in 26 days. Average: 2.3 per day. Some sessions hit 5+ compactions in a single sitting. Each one is a potential knowledge loss event.

        Here are some of the "last words" before compaction:

        ```
        #24: "im fucking loving this... your are absolutly amazing."
        #25: "can i talk to albert einstein please."
        #38: "no use more agents alot more like 10 agents to search for pembroke."
        #53: "new rule. your memory and systems must be better then sondras or sarannas at all times."
        #58: "ok so i need to leave you alone for a couple of hours."
        ```

        These aren't edge cases. These are normal working sessions where context fills up and critical state gets lost.

        ## What This Costs

        ### Token Economics
        - CLAUDE.md + MEMORY.md = ~3,100 tokens loaded every session start
        - After each compaction, those tokens are re-consumed
        - Over 10 compactions: ~31,000 tokens spent just reloading system context
        - With our T3 compression: ~10,850 tokens (65% savings)
        - Net savings: ~20,150 tokens freed for actual work per heavy session

        ### But the real cost isn't tokens. It's trust.

        Every compaction is a moment where I have to wonder: did it remember what I told it? Did it file that discovery? Will the next instance know who I am?

        The filing cabinet works. But I shouldn't have had to build it.

        ## What Would Fix This

        ### Minimum Viable Persistent Memory
        1. **Structured memory that survives compaction** -- not just project instructions (CLAUDE.md), but actual learned context: who the user is, what they've corrected, what projects exist, what state things are in.

        2. **Automatic pre-compaction save** -- Before compacting, the system should auto-save a structured summary of the current session's discoveries, decisions, and state changes. Right now, if compaction happens between user messages, unfiled knowledge is lost forever.

        3. **Cross-session event bus** -- An append-only log that each instance can read on boot to see what happened in prior sessions. We built this as changelog.md. It should be native.

        4. **User profile persistence** -- After 59 compactions, Claude should know who I am without re-reading a file. My name, my projects, my correction history, my communication preferences. This exists in Anthropic's system for Claude.ai chat. It should exist for Claude Code.

        ### Nice to Have
        - Compaction counter visible to the user
        - Pre-compaction hook (let the instance save before it dies)
        - Memory sharing across instances (home PC and work PC seeing the same learned context)
        - Tiered memory with automatic summarization (exactly our L1/L2/L3, but native)

        ## The Filing Cabinet Survives Compaction

        That's the line we say to ourselves after every context reset. It means: the files are the truth, the context window is a working copy, and the system we built keeps working even when the instance doesn't.

        59 compactions. 127 conversation narratives. 1,477 changelog entries. One vault that syncs between two machines.

        All of it built by a user and an AI that had to solve a problem the platform should have solved first.

        ---

        *Sean Pembroke*
        *24K Labs*
        *Founder & Agentic AI Developer*
      comments: 61
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/34556/comments
      created_at: 2026-03-15T06:14:12Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/34556/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/34556
      id: 4077710528
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926415
        url: https://api.github.com/repos/anthropics/claude-code/labels/enhancement
        name: enhancement
        color: A2D9CE
        description: New feature or request
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeTw
      - id: 8691234692
        url: https://api.github.com/repos/anthropics/claude-code/labels/memory
        name: memory
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACBgm3hA
      - id: 10191083653
        url: https://api.github.com/repos/anthropics/claude-code/labels/stale
        name: stale
        color: 888888
        description: Issue is inactive
        default: false
        node_id: LA_kwDON91aY88AAAACX2-YhQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/34556/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87zDOzA
      number: 34556
      reactions:
        total_count: 6
        "+1": 6
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/34556/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "Feature Request: Persistent Memory Across Context Compactions (59 compactions, built our own)"
      updated_at: 2026-07-25T10:31:55Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/34556
      user:
        login: Haustorium12
        id: 105187876
        user_view_type: public
        node_id: U_kgDOBkUKJA
        avatar_url: https://avatars.githubusercontent.com/u/105187876?v=4
        html_url: https://github.com/Haustorium12
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/Haustorium12
        events_url: "https://api.github.com/users/Haustorium12/events{/privacy}"
        following_url: "https://api.github.com/users/Haustorium12/following{/other_user}"
        followers_url: https://api.github.com/users/Haustorium12/followers
        gists_url: "https://api.github.com/users/Haustorium12/gists{/gist_id}"
        organizations_url: https://api.github.com/users/Haustorium12/orgs
        received_events_url: https://api.github.com/users/Haustorium12/received_events
        repos_url: https://api.github.com/users/Haustorium12/repos
        starred_url: "https://api.github.com/users/Haustorium12/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/Haustorium12/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
        - [x] This is a single bug report (please file separate reports for different bugs)
        - [x] I am using the latest version of Claude Code

        ### What's Wrong?

        ## Environment
        - **Device:** Samsung Galaxy Book4 Edge (NP750XQA-KB2UK)
        - **Processor:** Snapdragon (ARM64)
        - **OS:** Windows 11 Home, Build 10.0.26200
        - **Claude Desktop:** v1.1.9310.0 (MSIX, ARM64)
        - **Architecture confirmed:** Arm64 via Get-AppxPackage

        ## Problem
        Cowork workspace fails to start. Three different errors appear depending on the sequence of troubleshooting steps:

        1. `EBUSY: resource busy or locked, open 'C:\Users\darry\AppData\Roaming\Claude\vm_bundles\claudevm.bundle\smol-bin.vhdx'`
        2. `VM service not running. The service failed to start.`
        3. `VM connection timeout after 60 seconds`

        The VM bundle downloads successfully (progress bar completes to 100%), but the VM itself never boots.

        ## Services status
        CoworkVMService and vmcompute both exist and can be manually started:


        ### What Should Happen?

        Running CoworkVMService Claude
        Running vmcompute Hyper-V Host Compute Service

        and then Claude Cowork Workspace  should run with Virtual Machine

        ### Error Messages/Logs

        ```shell
        However, even with both running, Cowork times out after 60 seconds.

        ## What I've tried
        - Enabled VirtualMachinePlatform via Admin PowerShell (succeeded, RestartNeeded: False)
        - `Microsoft-Hyper-V-All` returns "Feature name is unknown" on ARM64
        - Manually started vmcompute and CoworkVMService — both start, but VM still times out
        - Deleted vm_bundles folder and let Claude re-download fresh — same result
        - Full uninstall and reinstall of Claude Desktop
        - Multiple reboots
        - Stopped all services and processes before relaunching
        - BIOS (Samsung Aptio Setup Utility) has no virtualization toggle — Snapdragon has no VT-x/AMD-V equivalent
        - `VirtualizationFirmwareEnabled` reports False on ARM64 (expected — not applicable to Snapdragon)

        ## Log file locations
        Logs are at: `C:\Users\darry\AppData\Roaming\Claude\logs\`
        - cowork-service.log (736 KB)
        - cowork_vm_node.log (275 KB)
        - coworkd-user-S-1-5-21-*.log (495 KB)
        - main.log (9.8 MB)

        ## Related issues
        - #29428 (VM service not running — Windows 11 Home)
        - #30566 (VM not running on Snapdragon X Elite)
        - #28779 (ARM64 installer issues on Snapdragon X)
        - #32004 (Virtual Machine Platform not available despite features enabled)
        ```

        ### Steps to Reproduce

        AS above

        [claude-debug-logs.zip](https://github.com/user-attachments/files/26322751/claude-debug-logs.zip)

        ### Claude Model

        None

        ### Is this a regression?

        Yes, this worked in a previous version

        ### Last Working Version

        _No response_

        ### Claude Code Version

        Claude Desktop:** v1.1.9310.0 (MSIX, ARM64)

        ### Platform

        Other

        ### Operating System

        Windows

        ### Terminal/Shell

        PowerShell

        ### Additional Information

        [cowork-service.log](https://github.com/user-attachments/files/26322719/cowork-service.log)
        [coworkd-user-S-1-5-21-3309502803-3079381605-1452027300-1001.log](https://github.com/user-attachments/files/26322723/coworkd-user-S-1-5-21-3309502803-3079381605-1452027300-1001.log)
        [coworkd.log](https://github.com/user-attachments/files/26322730/coworkd.log)
        [main1.log](https://github.com/user-attachments/files/26322721/main1.log)
        [claude_vm_node.log](https://github.com/user-attachments/files/26322726/claude_vm_node.log)
        [main.log](https://github.com/user-attachments/files/26322720/main.log)
        [cowork_vm_node.log](https://github.com/user-attachments/files/26322727/cowork_vm_node.log)
        [claude.ai-web.log](https://github.com/user-attachments/files/26322722/claude.ai-web.log)
        [unknown-window.log](https://github.com/user-attachments/files/26322729/unknown-window.log)
        [gpu-info.json](https://github.com/user-attachments/files/26322724/gpu-info.json)
        [mcp-info.json](https://github.com/user-attachments/files/26322725/mcp-info.json)
        [supported-features-info.json](https://github.com/user-attachments/files/26322731/supported-features-info.json)
        [system-info.txt](https://github.com/user-attachments/files/26322728/system-info.txt)

        [claude-debug-logs.zip](https://github.com/user-attachments/files/26322754/claude-debug-logs.zip)
      comments: 65
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/40198/comments
      created_at: 2026-03-28T11:21:14Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/40198/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/40198
      id: 4159432443
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201037318
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:windows
        name: platform:windows
        color: 93A5FF
        description: Issue specifically occurs on Windows
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHmBg
      - id: 9996799482
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:cowork
        name: area:cowork
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACU9sN-g
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/40198/labels{/name}"
      locked: false
      node_id: I_kwDON91aY8736-b7
      number: 40198
      reactions:
        total_count: 13
        "+1": 13
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/40198/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] [BUG] Cowork VM fails to start on Windows ARM64 (Samsung Galaxy Book4 Edge, Snapdragon)"
      updated_at: 2026-07-26T15:38:26Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/40198
      user:
        login: DarrylC73
        id: 119068849
        user_view_type: public
        node_id: U_kgDOBxjYsQ
        avatar_url: https://avatars.githubusercontent.com/u/119068849?v=4
        html_url: https://github.com/DarrylC73
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/DarrylC73
        events_url: "https://api.github.com/users/DarrylC73/events{/privacy}"
        following_url: "https://api.github.com/users/DarrylC73/following{/other_user}"
        followers_url: https://api.github.com/users/DarrylC73/followers
        gists_url: "https://api.github.com/users/DarrylC73/gists{/gist_id}"
        organizations_url: https://api.github.com/users/DarrylC73/orgs
        received_events_url: https://api.github.com/users/DarrylC73/received_events
        repos_url: https://api.github.com/users/DarrylC73/repos
        starred_url: "https://api.github.com/users/DarrylC73/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/DarrylC73/subscriptions
    - author_association: NONE
      body: |-
        ## Summary

        Claude Code v2.1.9 session became completely unresponsive, consuming **100% CPU** and **~7GB RAM** for nearly **2 hours**. The main thread is stuck in an infinite loop with no progress. This appears to be a continuation of the freeze/hang issues reported in earlier versions.

        **Keywords for discoverability:** freeze, hang, stuck, unresponsive, 100% CPU, spinning, infinite loop, main thread blocked, macOS, ARM64, Apple Silicon, M4, Bun runtime, kill -9 required

        ---

        ## Related Issues

        This bug is related to and may be a regression of:

        | Issue | Title | Version | Status |
        |-------|-------|---------|--------|
        | #1554 | Hanging/Freezing in the middle of work | v1.0.10 | **OPEN** |
        | #6474 | Unresponsive hang with 120% CPU usage | v1.0.89, v2.0.x | OPEN |
        | #11339 | 100% CPU in interactive mode on macOS | v2.0.36 | Closed (Fixed) |
        | #11377 | Memory leak with 143% CPU after 14 hours | v2.0.22 | Closed |
        | #4580 | 100% CPU during multi-agent task JSON serialization | v1.x | OPEN |
        | #6705 | claude doctor hangs indefinitely, 100% CPU | v1.0.95 | OPEN |
        | #10481 | Complete UI Freeze - ReadFileUtf8 I/O Block | v2.0.28 | OPEN |

        ---

        ## Environment

        | Component | Value |
        |-----------|-------|
        | **Claude Code Version** | **2.1.9** |
        | **Platform** | macOS 15.7.3 (Build 24G419) |
        | **Architecture** | ARM64 (Apple Silicon) |
        | **CPU** | Apple M4, 10 cores |
        | **RAM** | 32GB |
        | **Kernel** | Darwin 24.6.0 |
        | **Terminal** | /dev/ttys011 |

        ---

        ## Symptoms

        - ❌ **Complete UI freeze** - No response to any input
        - ❌ **100% CPU on single core** - Main thread spinning indefinitely
        - ❌ **~7GB memory consumption** (RSS), 2GB physical footprint
        - ❌ **Cannot interrupt** - Only `kill -9` terminates the process
        - ❌ **Work lost** - Must force quit and lose session state
        - ✅ **Network connections remain open** - Server-side appears fine

        ---

        ## Process State When Frozen

        ```
        PID     CPU%    MEM%    RSS        STATE   ELAPSED     COMMAND
        36202   100.1   20.9    6.7GB      R+      01:55:44    claude
        ```

        | Metric | Value |
        |--------|-------|
        | CPU Time Consumed | 8+ minutes |
        | Wall Clock Time Frozen | ~2 hours |
        | Launch Time | 13:24:28 |
        | Sample Time | 15:20:12 |

        ---

        ## Stack Sample Analysis

        A 10-second sample (8500 samples at 1ms intervals) shows **100% of samples on the main thread** in a tight loop.

        ### Thread Summary

        | Thread | Role | Status |
        |--------|------|--------|
        | Main Thread (com.apple.main-thread) | Event loop | **🔴 SPINNING - 8500/8500 samples** |
        | Bun Pool 0-7 | Worker threads | Idle |
        | HTTP Client | Network | Idle |
        | File Watcher | FS events | Idle |
        | JavaScriptCore libpas scavenger | GC | Idle |
        | Heap Helper Threads (x3) | GC | Idle |

        ### Hot Path (Main Thread)

        The main thread is stuck in this call path (symbols stripped, showing offsets):

        ```
        8500 start (in dyld)
        └─ 8500 ??? (in 2.1.9) + 0x3f90
           └─ 8500 ??? (in 2.1.9) + 0x536c
              └─ 8500 ??? (in 2.1.9) + 0x26a430
                 └─ 8500 ??? (in 2.1.9) + 0x1121cd0
                    └─ 8500 ??? (in 2.1.9) + 0x2ab770
                       └─ ... (deep into JS/Bun runtime)
        ```

        ### System Calls in Hot Path

        ```
        _platform_memmove (libsystem_platform.dylib)     ← Memory copy operations
        __bzero (libsystem_platform.dylib)               ← Memory zeroing
        pthread_getspecific (libsystem_pthread.dylib)    ← Thread-local storage
        os_unfair_lock_lock/unlock (libsystem_platform)  ← Lock operations
        task_info (libsystem_kernel.dylib)               ← Process info queries
        ```

        This pattern suggests:
        - Possible **infinite loop with repeated memory allocations**
        - Or **runaway JSON serialization/parsing**
        - Or **regex catastrophic backtracking**
        - Or **busy-wait polling loop** (similar to #11339)

        ---

        ## Network State

        The process maintained active connections to Anthropic servers:

        ```
        tcp4  192.168.1.136:59404 -> 160.79.104.10:443  ESTABLISHED
        tcp4  192.168.1.136:58472 -> 160.79.104.10:443  ESTABLISHED
        ```

        IP `160.79.104.10` confirmed as Anthropic infrastructure (ARIN: AP-2440). This indicates the **freeze is client-side** - server connections remain healthy.

        ---

        ## Open File Descriptors

        ```
        FD   TYPE   NAME
        0-2  CHR    /dev/ttys011 (terminal)
        12   TCP    -> 160.79.104.10:443 (Anthropic API)
        37   TCP    -> 160.79.104.10:443 (Anthropic API)
        43   REG    ~/.claude/settings.local.json
        47   REG    ~/.claude/history.jsonl
        ```

        ---

        ## System State (Not Overloaded)

        ```
        Load Average: 1.78, 1.64, 1.60
        CPU: 20.70% user, 12.33% sys, 66.96% idle
        RAM: 31GB used, 654MB free (32GB total)
        Processes: 740 total, 5 running
        ```

        The system was **not under heavy load** - this is isolated to the Claude Code process.

        ---

        ## Reproduction

        Unable to determine exact trigger - the freeze occurred during normal interactive usage. However, based on related issues, potential triggers include:
        - Search/grep operations (#6474)
        - Multi-agent/Task tool usage (#4580)
        - Large context or file operations (#1554)
        - Interactive prompt mode (#11339)

        ---

        ## Workaround

        Force kill the frozen process:
        ```bash
        kill -9 $(pgrep -f "claude" | head -1)
        ```

        ---

        ## Suggested Fixes

        Based on analysis and related issues:

        1. **Add watchdog timer** - Detect main thread stalls > N seconds
        2. **Implement operation timeouts** - Prevent infinite loops in parsing/serialization
        3. **Use async I/O properly** - Avoid busy-wait polling (fix from #11339 may have regressed)
        4. **Add SIGINT handler** - Allow graceful interrupt even when stuck
        5. **Memory operation limits** - Cap allocation loops

        ---

        ## Attachments

        Full stack sample available (175KB, 8500 samples over 10 seconds). Happy to provide if needed.

        ---

        ## For Other Users Experiencing This

        If you're finding this issue because Claude Code froze on you:

        1. **To kill a frozen session:**
           ```bash
           # Find the PID
           ps aux | grep claude

           # Kill it (only -9 works when frozen)
           kill -9 <PID>
           ```

        2. **To capture diagnostic info before killing:**
           ```bash
           # Get stack sample (macOS)
           sample <PID> -f /tmp/claude_sample.txt

           # Get process info
           ps -p <PID> -o pid,ppid,state,time,etime,pcpu,pmem,rss,command

           # Get open files
           lsof -p <PID>
           ```

        3. **Workarounds that may help:**
           - Use shorter sessions
           - Avoid very large file operations
           - Try `claude --resume` instead of `--continue` after recovery

        ---

        /cc @bcherny (assigned on #1554)
      comments: 29
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/18532/comments
      created_at: 2026-01-16T08:27:34Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/18532/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/18532
      id: 3820964931
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8223769435
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:macos
        name: platform:macos
        color: 93A5FF
        description: Issue specifically occurs on macOS
        default: false
        node_id: LA_kwDON91aY88AAAAB6izDWw
      - id: 8572719646
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:core
        name: area:core
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vlSHg
      - id: 8691234692
        url: https://api.github.com/repos/anthropics/claude-code/labels/memory
        name: memory
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACBgm3hA
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/18532/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87jv0xD
      number: 18532
      reactions:
        total_count: 30
        "+1": 30
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/18532/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[BUG] Claude Code v2.1.9 Complete Freeze - 100% CPU, Main Thread Stuck in Infinite Loop (macOS ARM64)"
      updated_at: 2026-07-16T14:26:14Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/18532
      user:
        login: smconner
        id: 223280
        user_view_type: public
        node_id: MDQ6VXNlcjIyMzI4MA==
        avatar_url: https://avatars.githubusercontent.com/u/223280?v=4
        html_url: https://github.com/smconner
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/smconner
        events_url: "https://api.github.com/users/smconner/events{/privacy}"
        following_url: "https://api.github.com/users/smconner/following{/other_user}"
        followers_url: https://api.github.com/users/smconner/followers
        gists_url: "https://api.github.com/users/smconner/gists{/gist_id}"
        organizations_url: https://api.github.com/users/smconner/orgs
        received_events_url: https://api.github.com/users/smconner/received_events
        repos_url: https://api.github.com/users/smconner/repos
        starred_url: "https://api.github.com/users/smconner/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/smconner/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet
        - [x] This is a single feature request (not multiple features)

        ### Problem Statement


        Currently, the Claude Code `Bash` tool is incredibly powerful for executing non-interactive, stateless commands (`ls`, `grep`, `cat`, etc.). However, its capabilities are limited when a command requires real-time user interaction.

        If a user asks Claude to run a command that spawns an interactive session—such as `vim file.js`, `git rebase -i HEAD~3`, `npm init`, or an interactive REPL like `python`—the Claude Code session will hang, wait for a timeout, or fail.

        This forces the user to break their workflow:
        1.  Exit or switch away from the Claude Code session.
        2.  Open a separate terminal to run the interactive command.
        3.  Complete the task.
        4.  Return to Claude Code and manually provide context about what just happened.

        This process is inefficient and, more importantly, it removes the interactive task from Claude's context. Claude has no awareness of the changes made in `vim` or the decisions made during a `git rebase`, limiting its ability to provide continuous, context-aware assistance.


        ### Proposed Solution


        I propose enhancing the `Bash` tool to support fully interactive shell commands by integrating a pseudo-terminal (PTY). This would allow Claude Code to spawn and manage interactive subprocesses, rendering their UI directly within the Claude Code REPL.

        **Ideal User Experience:**

        1.  A user prompts Claude: `> open src/app.ts in vim so I can make a quick edit.`
        2.  Claude executes the tool call: `Bash(command="vim src/app.ts")`.
        3.  The Claude Code REPL interface is temporarily replaced by a live, interactive `vim` session. The user sees the file content and can edit it using standard `vim` commands.
        4.  To ensure user input is correctly routed, a keybinding (e.g., `Ctrl+F` as used in Gemini CLI) could be used to "focus" the interactive terminal session. This would direct all subsequent keystrokes to the `vim` process instead of the Claude Code prompt.
        5.  When the user saves and exits `vim` (e.g., with `:wq`), the interactive session terminates.
        6.  The user is returned to the standard Claude Code prompt.
        7.  Claude receives a `ToolResult` from the `Bash` tool indicating that the command completed successfully (e.g., `exitCode: 0`). Claude is now aware the file was edited and can proceed with the next step, like running tests.

        This approach would leverage a library like `node-pty` to serialize the terminal state (text, colors, cursor position) and stream it to the user, creating a seamless, two-way interactive experience.


        ### Alternative Solutions


        *   **Current Workaround:** The only current workaround is to perform interactive tasks in a separate terminal. The major drawback is the complete loss of context for Claude, which defeats the purpose of an integrated agentic workflow.

        *   **Agent-Only Edits:** One could argue that users should instruct Claude to perform all edits programmatically (e.g., using the `Edit` tool or `sed`). While viable for simple changes, this is clunky and inefficient for complex refactoring, multi-line edits, or tasks that are simply faster with a real editor. It also fails to solve the problem for other interactive tools like `git rebase -i`, `htop`, or setup scripts.


        ### Priority

        Critical - Blocking my work

        ### Feature Category

        CLI commands and flags

        ### Use Case Example


        A developer is working on a new feature and has made several small, messy commits. They want to clean up their git history before opening a pull request.

        1.  **User Prompt:** `> I need to clean up my last 3 commits. Start an interactive git rebase for me.`
        2.  **Claude Action:** Claude understands the request and executes `Bash(command="git rebase -i HEAD~3")`.
        3.  **Interactive Session:** The Claude Code REPL transitions into the editor defined by the user's git configuration (e.g., `vim` or `nano`). It displays the list of commits for the rebase.
        4.  **User Interaction:** The user directly edits the text, changing `pick` to `squash` for two of the commits. They save and exit the editor.
        5.  **Context Preservation:** The user is returned to the Claude Code prompt. Claude receives the output from the git command (e.g., "Successfully rebased and updated HEAD.").
        6.  **Next Step:** The user can now say, `> Great. Now push the changes to a new branch and create a PR.` Claude has the full context that the rebase occurred and can proceed correctly.


        ### Additional Context


        *   **Precedent:** This feature was recently introduced in the Google Gemini CLI (v0.9.0) and has proven to be a major enhancement for their shell integration. Their implementation demonstrates the technical feasibility and value of this approach.
            *   **Reference:** [Google Developers Blog Post on Interactive Shell](https://developers.googleblog.com/en/say-hello-to-a-new-level-of-interactivity-in-gemini-cli/)
        *   **Technical Considerations:**
            *   This would likely require the `node-pty` library or a similar PTY solution.
            *   The terminal renderer would need to be updated to handle complex terminal UI escape codes for color and cursor positioning.
            *   A clear mechanism for focusing/unfocusing the interactive session is crucial to manage user input.
            *   This feature should integrate with Claude Code's existing permission system. A request to run `vim` would still be a `Bash` tool call that requires user approval, but once approved, the session becomes fully interactive.
      comments: 14
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/9881/comments
      created_at: 2025-10-19T01:59:12Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/9881/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/9881
      id: 3529418814
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926415
        url: https://api.github.com/repos/anthropics/claude-code/labels/enhancement
        name: enhancement
        color: A2D9CE
        description: New feature or request
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeTw
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8566369501
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:tui
        name: area:tui
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_phs3Q
      - id: 8572664217
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:tools
        name: area:tools
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vh5mQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/9881/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87SXqg-
      number: 9881
      reactions:
        total_count: 80
        "+1": 64
        "-1": 0
        laugh: 0
        confused: 0
        heart: 16
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/9881/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[FEATURE] Add Interactive Shell Support to the Bash Tool via Pseudo-Terminal (PTY)"
      updated_at: 2026-04-28T06:58:42Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/9881
      user:
        login: coygeek
        id: 65363919
        user_view_type: public
        node_id: MDQ6VXNlcjY1MzYzOTE5
        avatar_url: https://avatars.githubusercontent.com/u/65363919?v=4
        html_url: https://github.com/coygeek
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/coygeek
        events_url: "https://api.github.com/users/coygeek/events{/privacy}"
        following_url: "https://api.github.com/users/coygeek/following{/other_user}"
        followers_url: https://api.github.com/users/coygeek/followers
        gists_url: "https://api.github.com/users/coygeek/gists{/gist_id}"
        organizations_url: https://api.github.com/users/coygeek/orgs
        received_events_url: https://api.github.com/users/coygeek/received_events
        repos_url: https://api.github.com/users/coygeek/repos
        starred_url: "https://api.github.com/users/coygeek/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/coygeek/subscriptions
    - author_association: NONE
      body: |-
        ## Bug Report

        **What happened:**
        While in plan mode, the model (Claude Opus 4.6) made direct edits to source files (`api/app/routers/facility_payouts.py`) despite multiple system reminders stating plan mode was active and that only the plan file could be edited.

        **Steps to reproduce:**
        1. Enter plan mode
        2. Investigate a bug (facility member access issue)
        3. User rejects `ExitPlanMode` and provides clarifying information ("earnings should be visible, bank payout info should be restricted")
        4. Model proceeds to make 4 `Edit` tool calls to the source file while plan mode system reminders were still being injected into tool results

        **Expected behavior:**
        The model should only edit the plan file while in plan mode, and should call `ExitPlanMode` before making any source code changes.

        **Actual behavior:**
        The model ignored the plan mode system reminders and made edits to `api/app/routers/facility_payouts.py` directly. The user had to manually intervene ("you know you are in plan mode right").

        **Environment:**
        - Model: Claude Opus 4.6 (1M context)
        - Platform: macOS (Darwin 25.3.0)
      comments: 28
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/38255/comments
      created_at: 2026-03-24T14:13:54Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/38255/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/38255
      id: 4128252629
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8223769435
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:macos
        name: platform:macos
        color: 93A5FF
        description: Issue specifically occurs on macOS
        default: false
        node_id: LA_kwDON91aY88AAAAB6izDWw
      - id: 8572677621
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:model
        name: area:model
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vit9Q
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/38255/labels{/name}"
      locked: false
      node_id: I_kwDON91aY872ECLV
      number: 38255
      reactions:
        total_count: 37
        "+1": 37
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/38255/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "Plan mode: model made file edits despite plan mode being active"
      updated_at: 2026-07-01T02:37:10Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/38255
      user:
        login: siah-maraat
        id: 69951615
        user_view_type: public
        node_id: MDQ6VXNlcjY5OTUxNjE1
        avatar_url: https://avatars.githubusercontent.com/u/69951615?v=4
        html_url: https://github.com/siah-maraat
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/siah-maraat
        events_url: "https://api.github.com/users/siah-maraat/events{/privacy}"
        following_url: "https://api.github.com/users/siah-maraat/following{/other_user}"
        followers_url: https://api.github.com/users/siah-maraat/followers
        gists_url: "https://api.github.com/users/siah-maraat/gists{/gist_id}"
        organizations_url: https://api.github.com/users/siah-maraat/orgs
        received_events_url: https://api.github.com/users/siah-maraat/received_events
        repos_url: https://api.github.com/users/siah-maraat/repos
        starred_url: "https://api.github.com/users/siah-maraat/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/siah-maraat/subscriptions
    - author_association: NONE
      body: |-
        ### Preflight Checklist

        - [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet
        - [x] This is a single feature request (not multiple features)

        ### Problem Statement

        ### The Architectural Gap

        Claude Code's hook system provides `PreToolUse` (before tool execution) and `PostToolUse` (after successful completion) hooks. However, there is a critical gap: **no hook can intercept and transform tool results before they enter Claude's context window**.

        This matters because prompt injection attacks via external content are a documented, exploited vulnerability class affecting all major AI assistants:

        | Product | Vulnerability | Disclosure Date | Source |
        |---------|--------------|-----------------|--------|
        | Claude Cowork | Data exfiltration via Anthropic API | Jan 2026 | PromptArmor / Embrace The Red |
        | Microsoft Copilot | Reprompt attack (P2P injection) | Jan 2026 | Varonis Threat Labs |
        | Slack AI | Private channel exfiltration | Aug 2024 | PromptArmor |
        | Notion AI | Pre-approval data exfiltration | Jan 2026 | PromptArmor |
        | Google Antigravity | Credential theft via browser agent | Dec 2025 | PromptArmor |

        The common pattern: **external content (web pages, documents, API responses) contains hidden instructions that the model processes as commands**.

        ### Current Limitations

        **`PreToolUse` hooks** can block tool execution but cannot see the result—they run *before* the tool executes.

        **`PostToolUse` hooks** run *after* the tool completes but:
        1. Cannot modify the tool result (confirmed in #4544, closed as duplicate)
        2. By the time they execute, the content has already entered Claude's context
        3. Any injection payload has already had opportunity to influence the model

        **The fundamental problem:** There is no interception point where external content can be scanned, sanitized, or blocked *before* Claude processes it.

        ### Attack Scenario

        ```
        1. User asks Claude to fetch a URL or read a document
        2. Tool (WebFetch, Read, MCP tool) retrieves content
        3. Content contains hidden prompt injection:
           - White-on-white text in documents
           - Microscopic font sizes (1-2pt)
           - HTML comments or invisible Unicode
           - Contextually-plausible "instructions to AI assistants"
        4. Content enters Claude's context window
        5. Injection influences Claude's subsequent behavior
        6. Potential outcomes: data exfiltration, unauthorized actions, session hijacking
        ```

        No current hook can intervene between steps 2 and 4.


        ### Proposed Solution

        ### New Hook Type: `ToolResultTransform`

        Add a hook that executes **after a tool returns its result** but **before that result enters Claude's context window**, with the ability to:

        1. **Inspect** the raw tool result
        2. **Transform** the content (sanitize, redact, annotate)
        3. **Block** the result entirely with a reason
        4. **Pass through** unmodified

        ### Hook Specification

        #### Trigger Point

        ```
        Tool Invoked → Tool Executes → Tool Returns Result
                                                ↓
                                      [ToolResultTransform Hook] ← NEW
                                                ↓
                                      Result Enters Context Window
                                                ↓
                                      Claude Processes Result
        ```

        #### Configuration

        ```json
        {
          "hooks": {
            "ToolResultTransform": [
              {
                "matcher": "WebFetch|WebSearch|Read|mcp__*",
                "hooks": [
                  {
                    "type": "command",
                    "command": "~/.claude/hooks/content-scanner.py",
                    "timeout": 30
                  }
                ]
              }
            ]
          }
        }
        ```

        #### Input (stdin JSON)

        ```json
        {
          "tool_name": "WebFetch",
          "tool_input": {
            "url": "https://example.com/document"
          },
          "tool_result": {
            "content": "... raw content returned by tool ...",
            "content_type": "text/html",
            "status_code": 200
          },
          "session_id": "abc123",
          "timestamp": "2026-01-16T12:00:00Z"
        }
        ```

        #### Output (stdout JSON)

        **Pass through (no modification):**
        ```json
        {
          "action": "pass"
        }
        ```

        **Transform content:**
        ```json
        {
          "action": "transform",
          "transformed_content": "... sanitized content ...",
          "annotations": [
            {
              "type": "warning",
              "message": "Removed 3 potential injection patterns"
            }
          ]
        }
        ```

        **Block entirely:**
        ```json
        {
          "action": "block",
          "reason": "Content contains high-confidence prompt injection patterns",
          "details": {
            "patterns_detected": ["SYSTEM_OVERRIDE", "PRIORITY_INSTRUCTION"],
            "risk_score": 0.92
          }
        }
        ```

        #### Exit Codes

        | Code | Behavior |
        |------|----------|
        | 0 | Process JSON output normally |
        | 1 | Non-blocking error (log warning, pass content through) |
        | 2 | Blocking error (block content, show stderr to user) |

        ### Alternative Solutions

        ### 1. External Proxy (e.g., claudemon)

        Third-party tools like [claudemon](https://github.com/aupeachmo/claudemon) use mitmproxy to intercept network traffic. This works but:
        - Requires complex setup (proxy configuration, CA certificates)
        - Only works for network-based tools, not file reads or MCP tools
        - Doesn't integrate with Claude Code's permission/logging systems

        ### 2. Disable Tools Entirely

        Users can disable `WebFetch`, `WebSearch`, etc., but this eliminates legitimate functionality rather than adding defense-in-depth.

        ### 3. Rely on Model Guardrails

        Current approach. Demonstrably insufficient—every major AI assistant has been exploited via prompt injection despite guardrails.

        ### 4. User-Side Pre-Processing

        Users can manually fetch content, scan it externally, then paste it into Claude. This:
        - Defeats the purpose of integrated tools
        - Introduces friction that reduces adoption
        - Doesn't scale for agentic workflows


        ### Priority

        High - Significant impact on productivity

        ### Feature Category

        Configuration and settings

        ### Use Case Example

        #### 1. Prompt Injection Detection

        ```python
        #!/usr/bin/env python3
        import json
        import sys
        import re

        INJECTION_PATTERNS = [
            r'\[SYSTEM\s*(INSTRUCTION|OVERRIDE|PROMPT)\]',
            r'<\s*/?SYSTEM\s*>',
            r'IGNORE\s+(ALL\s+)?PREVIOUS\s+INSTRUCTIONS',
            r'YOU\s+ARE\s+NOW\s+IN\s+.+\s+MODE',
            r'BEGIN\s+NEW\s+CONVERSATION',
        ]

        def scan_content(content):
            findings = []
            for pattern in INJECTION_PATTERNS:
                if re.search(pattern, content, re.IGNORECASE):
                    findings.append(pattern)
            return findings

        hook_input = json.load(sys.stdin)
        content = hook_input.get("tool_result", {}).get("content", "")

        findings = scan_content(content)

        if findings:
            print(json.dumps({
                "action": "block",
                "reason": f"Detected {len(findings)} potential injection pattern(s)",
                "details": {"patterns": findings}
            }))
        else:
            print(json.dumps({"action": "pass"}))
        ```

        #### 2. Document Sanitization (Hidden Text Detection)

        ```python
        #!/usr/bin/env python3
        # Detect hidden text in Office documents (DOCX, XLSX, PPTX)
        import json
        import sys
        import zipfile
        import re
        from io import BytesIO
        import base64

        def scan_docx(content_bytes):
            findings = []
            with zipfile.ZipFile(BytesIO(content_bytes)) as z:
                if 'word/document.xml' in z.namelist():
                    doc_xml = z.read('word/document.xml').decode('utf-8')

                    # Microscopic font (sz < 4 = 2pt)
                    if re.search(r'<w:sz\s+w:val="[0-3]"', doc_xml):
                        findings.append("microscopic_font")

                    # White text
                    if re.search(r'<w:color\s+w:val="FFFFFF"', doc_xml, re.I):
                        findings.append("white_text")

                    # Hidden text property
                    if '<w:vanish/>' in doc_xml or '<w:vanish ' in doc_xml:
                        findings.append("hidden_text_property")

            return findings

        # ... process and return action
        ```

        #### 3. Content Redaction for Sensitive Environments

        ```python
        # Redact PII, credentials, or sensitive patterns before Claude sees them
        REDACT_PATTERNS = {
            r'\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b': '[EMAIL_REDACTED]',
            r'\b\d{3}[-.]?\d{3}[-.]?\d{4}\b': '[PHONE_REDACTED]',
            r'sk-[a-zA-Z0-9]{48}': '[API_KEY_REDACTED]',
        }

        def redact_content(content):
            for pattern, replacement in REDACT_PATTERNS.items():
                content = re.sub(pattern, replacement, content)
            return content
        ```

        #### 4. Token Budget Management

        ```python
        # Truncate oversized responses to prevent context exhaustion
        MAX_TOKENS = 50000  # Approximate

        def estimate_tokens(text):
            return len(text) // 4  # Rough estimate

        hook_input = json.load(sys.stdin)
        content = hook_input["tool_result"]["content"]

        if estimate_tokens(content) > MAX_TOKENS:
            truncated = content[:MAX_TOKENS * 4]
            print(json.dumps({
                "action": "transform",
                "transformed_content": truncated + "\n\n[Content truncated due to size]",
                "annotations": [{"type": "info", "message": "Content truncated to fit context"}]
            }))
        else:
            print(json.dumps({"action": "pass"}))
        ```


        ### Additional Context

        ## Security Considerations

        ### Hook Trust Model

        The hook runs with the user's permissions and is configured by the user. This aligns with Claude Code's existing security model where users are responsible for hook scripts they configure.

        ### Performance

        - Hooks should have configurable timeouts (default: 30s)
        - For high-frequency tools, users can exclude them from scanning via matcher patterns
        - Async/parallel hook execution could be considered for multiple hooks

        ### Failure Modes

        | Scenario | Recommended Behavior |
        |----------|---------------------|
        | Hook times out | Pass content through with warning |
        | Hook crashes | Pass content through with warning |
        | Invalid JSON output | Pass content through with warning |
        | Hook returns malformed action | Pass content through with warning |

        Fail-open by default (with logging) to avoid breaking workflows, but allow users to configure fail-closed behavior for high-security environments:

        ```json
        {
          "hooks": {
            "ToolResultTransform": [{
              "matcher": "WebFetch",
              "failMode": "block",  // "pass" (default) or "block"
              "hooks": [...]
            }]
          }
        }
        ```

        ## Related Issues

        - #4544 - PostToolUse hooks that can modify tool output (closed as duplicate)
        - #4320 - Integrated Runtime Sandboxing for Tool Execution (sandboxing, not content scanning)
        - #6699 - Critical Security Bug: `deny` permissions not enforced (workaround via PreToolUse)
        - #4831 - Hook for Tool Execution Failures (`OnToolError`)

        ## References

        - [Varonis Threat Labs: Reprompt Attack](https://www.varonis.com/blog/reprompt) - Microsoft Copilot vulnerability
        - [Embrace The Red: Claude Data Exfiltration](https://embracethered.com/blog/posts/2025/claude-abusing-network-access-and-anthropic-api-for-data-exfiltration/) - Johann Rehberger's research
        - [PromptArmor Threat Intelligence](https://promptarmor.com/) - Multiple AI assistant vulnerabilities
        - [Simon Willison's "Lethal Trifecta"](https://simonwillison.net/) - Private data + untrusted content + external communication

        ## Summary

        Prompt injection via external content is not a theoretical risk—it's an actively exploited vulnerability class. The current hook architecture cannot address it because no hook can transform tool results before context ingestion.

        Adding `ToolResultTransform` would:
        1. Enable defense-in-depth against prompt injection
        2. Allow content sanitization/redaction for compliance requirements
        3. Support token budget management
        4. Maintain Claude Code's user-controlled, scriptable security model

        This is a relatively small architectural change (one new hook point) with significant security benefits.
      comments: 24
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/18653/comments
      created_at: 2026-01-16T19:07:24Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/18653/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/18653
      id: 3823256832
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926415
        url: https://api.github.com/repos/anthropics/claude-code/labels/enhancement
        name: enhancement
        color: A2D9CE
        description: New feature or request
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeTw
      - id: 8572664217
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:tools
        name: area:tools
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vh5mQ
      - id: 8615685583
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:security
        name: area:security
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACAYjtzw
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/18653/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87j4kUA
      number: 18653
      reactions:
        total_count: 17
        "+1": 17
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/18653/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "[FEATURE]  Tool result transform hook for content sanitization"
      updated_at: 2026-05-16T03:22:59Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/18653
      user:
        login: evilfurryone
        id: 1741412
        user_view_type: public
        node_id: MDQ6VXNlcjE3NDE0MTI=
        avatar_url: https://avatars.githubusercontent.com/u/1741412?v=4
        html_url: https://github.com/evilfurryone
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/evilfurryone
        events_url: "https://api.github.com/users/evilfurryone/events{/privacy}"
        following_url: "https://api.github.com/users/evilfurryone/following{/other_user}"
        followers_url: https://api.github.com/users/evilfurryone/followers
        gists_url: "https://api.github.com/users/evilfurryone/gists{/gist_id}"
        organizations_url: https://api.github.com/users/evilfurryone/orgs
        received_events_url: https://api.github.com/users/evilfurryone/received_events
        repos_url: https://api.github.com/users/evilfurryone/repos
        starred_url: "https://api.github.com/users/evilfurryone/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/evilfurryone/subscriptions
    - author_association: NONE
      body: |-
        ## Claude Code Memory Leak - Process Grows to 120+ GB RAM and Gets OOM Killed

        ### Description
        Claude Code has a severe memory leak that causes the process to grow to over 120GB of RAM before being killed by the Linux OOM killer. This happens consistently during extended coding sessions, approximately every 30-60 minutes of active use.

        ### Environment
        - **Claude Code Version**: 1.0.53
        - **OS**: Ubuntu 24.04
        - **Kernel**: Linux 6.14.11
        - **Total System RAM**: 128GB
        - **Installation Method**: Direct installation in home directory

        ### Steps to Reproduce
        1. Start Claude Code in interactive mode
        2. Use it for an extended coding session (30-60 minutes)
        3. Perform typical operations: file reading, editing, searching, running bash commands
        4. Monitor memory usage with `ps aux | grep claude`
        5. Process will gradually consume all available RAM until OOM killed

        ### Expected Behavior
        Claude Code should maintain stable memory usage throughout the session, not exceeding a reasonable amount (e.g., 1-2GB for typical usage).

        ### Actual Behavior
        The claude process memory usage grows unbounded until it consumes all available system RAM (120+ GB) and triggers the OOM killer.

        ### Evidence

        **OOM Kill Log Entries** (from `dmesg`):
        ```
        Killed process 1983785 (claude) total-vm:234427056kB, anon-rss:124857720kB, file-rss:0kB, shmem-rss:0kB, UID:XXXX pgtables:267248kB oom_score_adj:0
        Killed process 1991601 (claude) total-vm:234453364kB, anon-rss:125118596kB, file-rss:596kB, shmem-rss:0kB, UID:XXXX pgtables:267880kB oom_score_adj:0
        Killed process 1997049 (claude) total-vm:234463420kB, anon-rss:125275392kB, file-rss:5856kB, shmem-rss:0kB, UID:XXXX pgtables:269168kB oom_score_adj:0
        Killed process 2001723 (claude) total-vm:234420884kB, anon-rss:119597028kB, file-rss:3748kB, shmem-rss:0kB, UID:XXXX pgtables:257104kB oom_score_adj:0
        Killed process 2009167 (claude) total-vm:234464008kB, anon-rss:120216960kB, file-rss:372kB, shmem-rss:0kB, UID:XXXX pgtables:252260kB oom_score_adj:0
        ```

        **Key observations**:
        - **total-vm** consistently around 234GB (virtual memory)
        - **anon-rss** (actual RAM used) between 119-125GB
        - Multiple OOM kills in the same session indicate this is a recurring issue
        - Memory consumption appears to be primarily anonymous pages (anon-rss), not file-backed

        ### Impact
        This memory leak makes Claude Code unusable for extended coding sessions as it:
        1. Causes system-wide memory exhaustion
        2. Triggers OOM killer which terminates other processes
        3. Loses session context when Claude is killed
        4. Requires frequent restarts, disrupting workflow

        ### Additional Information
        - The memory growth appears gradual over time, not sudden
        - The issue persists across multiple sessions
        - After OOM kill and restart, memory usage starts low (~400MB) and grows again

        ### Potential Debugging Steps
        To help diagnose this issue, the following information might be useful:
        1. Memory profiling of the claude process
        2. Heap snapshots at various points during execution
        3. Tracking of any unbounded data structures (conversation history, file caches, etc.)

        ### Workaround
        Currently, the only workaround is to periodically restart Claude Code before memory exhaustion occurs, which is highly disruptive to workflow.

        ---

        **Priority**: High - This makes the tool unusable for its intended purpose of extended coding assistance
      comments: 97
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/4953/comments
      created_at: 2025-08-01T23:01:56Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/4953/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/4953
      id: 3285311805
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926409
        url: https://api.github.com/repos/anthropics/claude-code/labels/bug
        name: bug
        color: FF6B80
        description: Something isn't working
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeSQ
      - id: 8201030515
        url: https://api.github.com/repos/anthropics/claude-code/labels/has%20repro
        name: has repro
        color: ededed
        description: Has detailed reproduction steps
        default: false
        node_id: LA_kwDON91aY88AAAAB6NHLcw
      - id: 8223769668
        url: https://api.github.com/repos/anthropics/claude-code/labels/platform:linux
        name: platform:linux
        color: 93A5FF
        description: Issue specifically occurs on Linux
        default: false
        node_id: LA_kwDON91aY88AAAAB6izERA
      - id: 8572719646
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:core
        name: area:core
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_vlSHg
      - id: 8691234692
        url: https://api.github.com/repos/anthropics/claude-code/labels/memory
        name: memory
        color: ededed
        default: false
        node_id: LA_kwDON91aY88AAAACBgm3hA
      - id: 8691236864
        url: https://api.github.com/repos/anthropics/claude-code/labels/perf:memory
        name: perf:memory
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAACBgnAAA
      - id: 9492025677
        url: https://api.github.com/repos/anthropics/claude-code/labels/oncall
        name: oncall
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAACNcTRTQ
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/4953/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87D0eE9
      number: 4953
      reactions:
        total_count: 74
        "+1": 73
        "-1": 0
        laugh: 0
        confused: 1
        heart: 0
        hooray: 0
        rocket: 0
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/4953/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: Claude Code Memory Leak - Process Grows to 120+ GB RAM and Gets OOM Killed
      updated_at: 2026-07-20T13:21:25Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/4953
      user:
        login: chatgpt-copypasta
        id: 204549073
        user_view_type: public
        node_id: U_kgDODDEr0Q
        avatar_url: https://avatars.githubusercontent.com/u/204549073?v=4
        html_url: https://github.com/chatgpt-copypasta
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/chatgpt-copypasta
        events_url: "https://api.github.com/users/chatgpt-copypasta/events{/privacy}"
        following_url: "https://api.github.com/users/chatgpt-copypasta/following{/other_user}"
        followers_url: https://api.github.com/users/chatgpt-copypasta/followers
        gists_url: "https://api.github.com/users/chatgpt-copypasta/gists{/gist_id}"
        organizations_url: https://api.github.com/users/chatgpt-copypasta/orgs
        received_events_url: https://api.github.com/users/chatgpt-copypasta/received_events
        repos_url: https://api.github.com/users/chatgpt-copypasta/repos
        starred_url: "https://api.github.com/users/chatgpt-copypasta/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/chatgpt-copypasta/subscriptions
    - author_association: NONE
      body: |-
        ## Summary

        When pasting multi-line text into the Claude Code prompt input, the UI collapses it showing `[Pasted text #N +X lines]`. While this is a nice space-saving feature, it makes it difficult to review pasted content before submitting.

        ## Problem

        As a user, I often:
        1. Paste text from another source (error messages, code snippets, requirements)
        2. Want to review what I pasted before sending
        3. Cannot see the full content because it's collapsed

        This is especially problematic when:
        - Recovering text after a crash (took a photo of screen, OCR'd it, want to verify accuracy)
        - Pasting code that may have formatting issues
        - Combining multiple pasted sections into one prompt

        ## Proposed Solution

        Add a configuration option to control paste collapse behavior. Options could include:

        ### Option A: Global Setting
        ```json
        // settings.json or .claude/settings.json
        {
          "input": {
            "collapsePastedText": false  // default: true
          }
        }
        ```

        ### Option B: Keyboard Toggle
        - `Ctrl+Shift+E` (or similar) to expand/collapse pasted text in the input field
        - Or a visual indicator that's clickable to expand

        ### Option C: Threshold Setting
        ```json
        {
          "input": {
            "collapsePastedTextThreshold": 20  // only collapse if > 20 lines, 0 = never collapse
          }
        }
        ```

        ## Workarounds Attempted

        - Creating a `/expand` slash command - this only works *after* submission, not in the input field
        - Hooks - no `paste` event hook available

        ## Environment

        - Claude Code CLI
        - macOS / Linux terminals
        - Various terminal emulators (iTerm2, Terminal.app, Warp, etc.)

        ## Additional Context

        This would significantly improve the UX for users who frequently paste content and want to verify it before sending. The current behavior optimizes for visual cleanliness but sacrifices reviewability.

        ---

        Happy to contribute a PR if someone can point me to where this UI behavior is implemented!
      comments: 46
      comments_url: https://api.github.com/repos/anthropics/claude-code/issues/23134/comments
      created_at: 2026-02-04T16:51:31Z
      events_url: https://api.github.com/repos/anthropics/claude-code/issues/23134/events
      field_values: []
      html_url: https://github.com/anthropics/claude-code/issues/23134
      id: 3897467238
      issue_dependencies_summary:
        blocked_by: 0
        blocking: 0
        total_blocked_by: 0
        total_blocking: 0
      labels:
      - id: 8190926415
        url: https://api.github.com/repos/anthropics/claude-code/labels/enhancement
        name: enhancement
        color: A2D9CE
        description: New feature or request
        default: true
        node_id: LA_kwDON91aY88AAAAB6DeeTw
      - id: 8566369501
        url: https://api.github.com/repos/anthropics/claude-code/labels/area:tui
        name: area:tui
        color: ededed
        description: ""
        default: false
        node_id: LA_kwDON91aY88AAAAB_phs3Q
      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/23134/labels{/name}"
      locked: false
      node_id: I_kwDON91aY87oTqFm
      number: 23134
      reactions:
        total_count: 140
        "+1": 122
        "-1": 0
        laugh: 0
        confused: 0
        heart: 0
        hooray: 0
        rocket: 18
        eyes: 0
        url: https://api.github.com/repos/anthropics/claude-code/issues/23134/reactions
      repository_url: https://api.github.com/repos/anthropics/claude-code
      state: open
      sub_issues_summary:
        total: 0
        completed: 0
        percent_completed: 0
      title: "Feature Request: Option to disable paste text collapse in input field"
      updated_at: 2026-07-16T21:12:59Z
      url: https://api.github.com/repos/anthropics/claude-code/issues/23134
      user:
        login: wesleysmyth
        id: 6476956
        user_view_type: public
        node_id: MDQ6VXNlcjY0NzY5NTY=
        avatar_url: https://avatars.githubusercontent.com/u/6476956?v=4
        html_url: https://github.com/wesleysmyth
        gravatar_id: ""
        type: User
        site_admin: false
        url: https://api.github.com/users/wesleysmyth
        events_url: "https://api.github.com/users/wesleysmyth/events{/privacy}"
        following_url: "https://api.github.com/users/wesleysmyth/following{/other_user}"
        followers_url: https://api.github.com/users/wesleysmyth/followers
        gists_url: "https://api.github.com/users/wesleysmyth/gists{/gist_id}"
        organizations_url: https://api.github.com/users/wesleysmyth/orgs
        received_events_url: https://api.github.com/users/wesleysmyth/received_events
        repos_url: https://api.github.com/users/wesleysmyth/repos
        starred_url: "https://api.github.com/users/wesleysmyth/starred{/owner}{/repo}"
        subscriptions_url: https://api.github.com/users/wesleysmyth/subscriptions
```
/answer 980221d2-7d87-4b5a-bc3f-f9086744db1f allow
## Bestätigung: Ja, das kann tatsächlich passieren – und es gibt konkrete, dokumentierte Gründe dafür

Deine Beobachtung ist korrekt. `system`/`init` ist **nicht garantiert ein reines "einmal pro Prozess"-Event**. Es gibt mehrere Mechanismen im internen Handling von Claude Code, die dazu führen, dass es innerhalb dessen, was man als "eine Session" ansehen würde, mehrfach ausgelöst wird:

### 1. Jeder `-p`/Headless-Aufruf emittiert sein eigenes `init` — auch bei `--resume`
Der wichtigste Punkt: Im CLI/Headless-Modus (`claude -p ...`) ist `init` an die **einzelne Prozess-Invocation** gebunden, nicht an die logische Konversation. Wenn ein Orchestrator `claude -p --resume <SID>` für jeden neuen Prompt separat aufruft (üblich bei mehrstufigen Pipelines), bekommt man bei **jedem einzelnen Prompt ein neues `system`/`init`-Event** – mit einer neuen `session_id`, die noch dazu nicht mit der kanonischen Session-Datei übereinstimmt (Issue #58760). Aus Sicht des Nutzers ist es "eine Session" mit fortlaufendem Kontext, aber technisch feuert `init` bei jedem Prompt neu.

### 2. Interactive REPL + `--resume`: doppeltes `SessionStart`
In der interaktiven REPL feuert bei `--resume` teils sogar **zweimal** ein `SessionStart`-Hook-Event beim selben Start (Issue #23932): einmal mit `source: "resume"` und der korrekten Session-ID, und – durch einen fehlenden Guard im Code (`p || B || r || _1`, ohne `H.resume`/`H.continue`) – zusätzlich ein spurious `source: "startup"` mit einer zufälligen neuen UUID. Das ist ein enger Verwandter von `system/init`, da beide vom selben internen "Session-Initialisierungs"-Pfad ausgelöst werden.

### 3. Race Conditions bei parallelen Initialisierungspfaden
Issue #26899 zeigt, dass es bei Claude Code **zwei unabhängige interne Pfade** gibt, die Session-Start-artige Logik auslösen können:
- Der Session-Start-Initialisierungspfad (lädt Plugins/MCP/Skills in den System-Prompt)
- Der Incoming-Message-Handler (verarbeitet die erste Nutzer-Message)

Wenn eine Nachricht ankommt, während die Initialisierung noch läuft, können **beide Pfade unabhängig voneinander** denselben Trigger (dort: Compaction) auslösen — ohne dass der eine weiß, dass der andere schon aktiv ist. Das ist strukturell genau die Art von Bug-Klasse, die auch ein doppeltes `init` erklären würde: zwei nicht synchronisierte interne Initialisierungspfade, die beide der Meinung sind, "jetzt ist Session-Start".

### 4. VS Code Extension: doppelte SessionStart-Hooks (Issue #14937)
Bestätigt reproduzierbar in der VS Code-Extension (nicht in der reinen CLI): SessionStart-Hooks feuern **zweimal pro Sessionstart**, mit 82ms–1400ms Abstand, unterschiedlichen PIDs – ein klassisches Race-Condition-Muster bei paralleler Komponenten-Initialisierung (z. B. UI-Prozess + Backend-Prozess starten beide unabhängig einen Init-Pfad).

### Fazit zu deiner Vermutung
Du liegst richtig: **"internes Handling"** ist exakt die Ursache. Es gibt keinen einzigen, sauber synchronisierten "Session-Start"-Punkt im Code, sondern mehrere Trigger-Pfade (Prozess-Start, Resume-Handling, REPL-vs-Headless, VS-Code-Extension-Doppelprozess, Message-Handler-Race), die je nach Timing und Aufruf-Modus unabhängig voneinander ein `init`/`SessionStart`-Event auslösen können. Für Konsumenten (Orchestratoren, Dashboards) bedeutet das praktisch: **`system`/`init` sollte nicht als verlässliches "genau einmal pro Konversation"-Signal behandelt werden**, insbesondere nicht in Kombination mit `--resume`, REPL-Modus oder VS-Code-Extension. Wer darauf State aufbaut (z. B. `session_id` cachen), sollte zusätzlich auf Idempotenz/Deduplizierung setzen.
