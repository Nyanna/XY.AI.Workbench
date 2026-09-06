Wenn, ich Claude Code während der Verarbeitung ESC drücke. Wird dann etwas zum Server gesendet, dass die Inferenz abbricht? 
#: Control Request:
#:id: "1765a1c5-b339-47cb-895d-18fecdc20e00"
#:phase: "request"
#:toolName: "web-search-exa"
#:arguments:
#:  query: "Claude Code ESC key interrupt abort request cancel API"
/allow 1765a1c5-b339-47cb-895d-18fecdc20e00
#: Control Request:
#:id: "73a6fa60-d51a-40e4-b522-2bb2b9949177"
#:phase: "result"
#:toolName: "web-search-exa"
#:result:
#:  content: []
#:  structuredContent:
#:    content: "Title: Interactive mode - Claude Code Docs\nURL: https://code.claude.com/docs/en/interactive-mode\nPublished: N/A\nAuthor: N/A\nHighlights:\n| Shortcut | Description | Context |\n| --- | --- | --- |\n| `Ctrl+C` | Interrupt, or clear input | Interrupts a running operation. If nothing is running, the first press clears the prompt input and a second press exits Claude Code |\n| `Ctrl+X Ctrl+K` | Stop all running background subagents in this session. Press twice within 3 seconds to confirm | Subagent control |\n| `Ctrl+D` | Exit Claude Code session | EOF signal |\n...\n| `Esc` | Interrupt Claude, or close a dialog | Stop the current response or tool call mid-turn so you can redirect. Claude keeps the work done so far. When a dialog such as a permission prompt is open, `Esc` closes the dialog rather than interrupting Claude. {/* min-version: 2.1.202 */}Before v2.1.202, `Esc` on some dialogs interrupted Claude and left the dialog open |\n...\n` + `\n...\ninput draft, or rewind | When the prompt input contains text, double `\n...\nit and saves the draft to history so `Up` recalls\n...\n. When the input is empty\n...\ndouble `Esc` opens the rewind menu to restore or summarize code and conversation from a previous point |\n...\n| `q`, `Ctrl+C`, `Esc` | Exit transcript view. All three can be rebound via `transcript:exit` |\n...\n1. Cancel search:\n...\n- Press `Ctrl+C` to cancel and restore your original input\n- Press `Backspace` on empty search to cancel\n\n---\n\nTitle: Add support for graceful interruption/cancellation of running queries\nURL: https://github.com/anthropics/claude-code-sdk-python/issues/8\nPublished: N/A\nAuthor: N/A\nHighlights:\nThe Python SDK currently doesn't appear to have a mechanism for gracefully interrupting/cancelling a running Claude Code query while preserving context. This feature would be valuable for building automation and oversight systems.\n...\n- The CLI supports interruption via the Escape key, which preserves context\n- The TypeScript SDK supports this via `abortController`\n- The Python SDK doesn't appear to expose this capability\n...\nAdd an interruption mechanism to the Python SDK, possibly through:\n...\n```python\n# Option 1: Cancellable task\ntask = asyncio.create_task(query(prompt=\"...\"))\n...\n# Later:\nawait task.cancel()  # Gracefully interrupts while preserving context\n...\n# Option 2: Abort controller pattern (similar to TypeScript)\ncontroller = AbortController()\nasync for message in query(prompt=\"...\", abort_controller=controller):\n    if need_to_interrupt:\n        controller.abort()\n...\n# Option 3: Query handle with methods\nquery_handle = await start_query(prompt=\"...\")\nasync for message in query_handle.messages():\n    if need_to_interrupt:\n        await query_handle.interrupt()\n```\n...\n> This is implemented as of v0.0.15\n...\n> See example_with_interrupt() in https://github.com/anthropics/claude-code-sdk-python/blob/main/examples/streaming_mode.py\n\n---\n\nTitle: Escape (cancel) does not kill in-flight parallel tool calls, causing repeated auto-resume\nURL: https://github.com/anthropics/claude-code/issues/26802\nPublished: N/A\nAuthor: N/A\nHighlights:\n# Escape (cancel) does not kill in-flight parallel tool calls, causing repeated auto-resume\n...\nWhen the user presses Escape to cancel/stop Claude during execution of parallel tool calls, the `onCancel` event fires and the current API stream is aborted — but **already-dispatched parallel tool calls continue running**. When those tool results return, Claude Code automatically starts a new API turn, effectively ignoring the user's cancel.\n...\nThis forces the user to press Escape repeatedly (5+ times in my case) to kill each successive auto-resumed turn, creating a frustrating experience where it feels like cancel is completely broken.\n...\n1. Give Claude a task that triggers multiple parallel tool calls (e.g., 3 parallel SSH commands)\n2. While the tools are executing, press Escape\n3. Observe: the current API stream aborts, but the parallel tools keep running\n4. When a parallel tool result returns, a new API turn starts automatically\n5. User must press Escape again to kill the new turn\n6. Repeat until all in-flight tools have returned\n...\nPressing Escape should:\n...\n1. Abort the current API stream (✅ this works)\n2. **Kill/cancel ALL in-flight parallel tool calls** (❌ this does NOT happen)\n3. **NOT auto-start a new API turn from returned tool results after cancel** (❌ this does NOT happen)\n...\nEscape only aborts the current stream. Parallel tools continue, their results trigger new API turns, and the model keeps generating responses and new tool calls.\n...\n```\n06:24:35.377Z [DEBUG] Stream started - received first chunk    ← parallel streams running\n06:24:35.455Z [DEBUG] [onCancel] focusedInputDialog=undefined  ← user presses Escape\n06:24:35.456Z [ERROR] API error: Request was aborted.          ← stream aborted (good)\n06:24:36.155Z [DEBUG] attribution header ...                   ← 600ms later, NEW API request\n06:24:36.156Z [DEBUG] [API:request] Creating client...         ← auto-resumed from tool result\n```\n...\nThis pattern repeated multiple times in a single session, with the user pressing Escape 5+ times and typing \"STOP\" in chat, unable to halt execution.\n...\nThis is a significant UX issue. Escape is the user's only kill switch. When it doesn't work, the user has **no way to stop Claude** from continuing to execute commands — including potentially destructive server operations. In this session, SSH commands continued executing on a production server after the user repeatedly tried to cancel.\n...\n> \n>\n...\n> \n> 🤖\n...\nhttps://claude\n...\nai/code)\n\n---\n\nTitle: Add support for graceful interruption/cancellation of running queries · Issue #8 · anthropics/claude-agent-sdk-python\nURL: https://github.com/anthropics/claude-agent-sdk-python/issues/8\nPublished: 2025-06-14T17:54:26.000Z\nAuthor: jasondk\nHighlights:\n## Add support for graceful interruption/cancellation of running queries\n...\nThe Python SDK currently doesn't appear to have a mechanism for gracefully interrupting/cancelling a running Claude Code query while preserving context. This feature would be valuable for building automation and oversight systems.\n...\n- The CLI supports interruption via the Escape key, which preserves context\n- The TypeScript SDK supports this via `abortController`\n- The Python SDK doesn't appear to expose this capability\n...\nAdd an interruption mechanism to the Python SDK, possibly through:\n...\n```python\n# Option 1: Cancellable task\ntask = asyncio.create_task(query(prompt=\"...\"))\n...\n# Later:\nawait task.cancel()  # Gracefully interrupts while preserving context\n...\n# Option 2: Abort controller pattern (similar to TypeScript)\ncontroller = AbortController()\nasync for message in query(prompt=\"...\", abort_controller=controller):\n    if need_to_interrupt:\n        controller.abort()\n...\n# Option 3: Query handle with methods\nquery_handle = await start_query(prompt=\"...\")\nasync for message in query_handle.messages():\n    if need_to_interrupt:\n        await query_handle.interrupt()\n```\n...\n> This is implemented as of v0.0.15\n...\n> See example_with_interrupt() in https://github.com/anthropics/claude-code-sdk-python/blob/main/examples/streaming_mode.py\n\n---\n\nTitle: Inturrupt/Cancel \"Thinking\" for sessions using claude code ...\nURL: https://github.com/anthropics/claude-code/issues/7181\nPublished: 2025-09-05T00:01:48.000Z\nAuthor: Eduard-Hasa\nHighlights:\nTo cancel the \"thinking\" process, call yourAbortController.abort() . This will terminate the current request stream, which will typically throw ...\n\n---\n\nTitle: ESC during tool execution is reported to the assistant as \"rejected\"\nURL: https://github.com/anthropics/claude-code/issues/47282\nPublished: N/A\nAuthor: N/A\nHighlights:\nESC during tool execution is reported to the assistant as \"rejected\"\n...\n# ESC during tool execution is reported to the assistant as \"rejected\"\n...\nWhen the user presses `ESC` to interrupt a tool call that is actively running in Claude Code CLI,\nthe local UI correctly shows `Interrupted · What should\nClaude do instead?`, but the message delivered to the assistant side uses different vocabulary:\n...\n> The user doesn't want to proceed with this tool use. The tool use was\n> rejected (eg. if it was a file edit, the new_string was NOT written to\n> the file). STOP what you are doing and wait for the user to tell you\n> how to proceed.\n...\nThis mismatch causes the assistant to report to the user that they\n\"rejected\" the action, when in reality the user only interrupted it. During a long debugging session\nwhere this happened, the assistant spent multiple turns explaining to the user that \"the user\nrejected the tool use\", which the user found surprising: they had only pressed `ESC`, not actively\ndenied anything.\n...\n3. While the Bash tool is actively running (during the `sleep 20`), press `ESC` once.\n...\n4. Observe that the local UI shows, under the collapsed Bash block:\n   ```\n   ⎿  Interrupted · What should Claude do instead?\n   ```\n...\n5. Ask the assistant to report, verbatim, the message it received from the harness after your `ESC`.\n It will quote the \"was rejected / STOP what you are doing\" text shown above.\n...\nThe message delivered to the assistant should match the vocabulary shown to the user locally:\n...\n- Use the word \"interrupted\", not \"rejected\".\n- Drop or soften the imperative \"STOP what you are doing\", which implies the user actively wants the\n assistant to halt all activity.\n- Ideally, keep the \"rejected\" vocabulary as a separate code path for the case where the user\n explicitly denies a permission prompt, so the two distinct user actions (interrupt vs deny)\n produce distinct messages to the assistant.\n...\nBoth cases — interruption of a running tool and explicit permission denial — appear to deliver the\nsame \"rejected\" message to the assistant. This makes the two user actions indistinguishable from the\nassistant's perspective, which causes the assistant to misrepresent what the user did.\n\nDuring the session that prompted this issue, we verified empirically that a different case —\npressing `ESC` while the assistant is still generating text (before any tool call has launched) —\ncorrectly delivers `[Request interrupted by user]` to the assistant. So the harness already supports\na distinct \"interrupted\" vocabulary in one path, but not in the tool-interruption path.\n...\n| User action | Local UI shows | Assistant receives |\n|-------------------------------------------------|---------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|\n| `ESC` during assistant text generation | (not captured locally, but user reported pressing `ESC` while the assistant was streaming tokens) | `[Request interrupted by user]` followed by any text the user typed afterwards |\n...\n| `ESC` during an actively running Bash tool call | `⎿ Interrupted · What should Claude do instead?` | `The user doesn't want to proceed with this tool use. The tool use was rejected (eg. if it was a file edit, the new_string was NOT written to the file). STOP what you are doing and wait for the user to tell you how to proceed.` |\n...\n| No `ESC`\n...\ncode=0` and stdout as expected |\n...\nChange the harness message delivered to the assistant when a tool is interrupted via `ESC` during\nexecution, to something that matches the local UI and avoids the imperative:\n...\n> [Tool use interrupted by user during execution] The user pressed\n> ESC. The tool did not complete. You may continue with other\n> actions or ask the user what to do next.\n...\nOptionally, keep the current \"rejected\" message as a separate path exclusively for explicit\npermission denials (clicking Deny on a permission prompt), so the two cases become semantically\ndistinct for the assistant.\n\n---\n\nTitle: Interactive mode\nURL: https://code.claude.com/docs/en/interactive-mode.md\nPublished: N/A\nAuthor: N/A\nHighlights:\n| Shortcut | Description | Context |\n| --- | --- | --- |\n| `Ctrl+C` | Interrupt, or clear input | Interrupts a running operation. If nothing is running, the first press clears the prompt input and a second press exits Claude Code |\n| `Ctrl+X Ctrl+K` | Stop all running background subagents in this session. Press twice within 3 seconds to confirm | Subagent control |\n| `Ctrl+D` | Exit Claude Code session | EOF signal |\n...\n| `Esc` | Interrupt Claude, or close a dialog | Stop the current response or tool call mid-turn so you can redirect. Claude keeps the work done so far. When a dialog such as a permission prompt is open, `Esc` closes the dialog rather than interrupting Claude. {/* min-version: 2.1.202 */}Before v2.1.202, `Esc` on some dialogs interrupted Claude and left the dialog open |\n...\n` + `\n...\ninput draft, or rewind | When the prompt input contains text, double `\n...\ndraft to history so `Up` recalls\n...\n. When the\n...\n` opens the rewind menu to restore or summarize code\n...\nconversation from a previous point |\n...\n| `q`, `Ctrl+\n...\n`, `Esc` | Exit transcript\n...\n. All three can be rebound via `transcript:exit`\n...\n1. Cancel search:\n...\n- Press `Ctrl+C` to cancel and restore your original input\n- Press `Backspace` on empty search to cancel\n\n---\n\nTitle: [Bug] Esc key interrupt disabled when queued messages or background ...\nURL: https://github.com/anthropics/claude-code/issues/16905\nPublished: 2026-01-08T17:02:26.000Z\nAuthor: LohiSoftsro\nHighlights:\nThe only way to interrupt a prompt with background tasks is to kill Claude Code completely. Workaround: press Esc or the ⏹ Stop button to end ...\n\n---\n\nTitle: Make escape-during-task interrupt threshold configurable (or expose as a separate rebindable action)\nURL: https://github.com/anthropics/claude-code/issues/55204\nPublished: N/A\nAuthor: brandonjp\nHighlights:\nThe ESC key is overloaded in the Chat context: pressing ESC clears the input when typing, and pressing ESC (or double-ESC) while a task is running interrupts it. Users who routinely double-tap ESC to clear input frequently interrupt running tasks by accident — losing in-flight\n...\nLooking at the documented keybinding actions, the only ESC-bound action in `Chat` context is `chat:cancel` (single `escape`). There is no separate `task:interrupt` or equivalent action that's bound to ESC while a task is foregrounded — so users can't remap the \"escape-while-running\" behavior independently of the \"escape-while-typing\" behavior. `app:interrupt` (`ctrl+c`) is\n...\nThe only workaround via `~/.claude/keybindings.json` is to unbind ESC from `chat:cancel` entirely, which also kills ESC-to-clear-input — a net loss for users who do legitimately use ESC to clear.\n...\nEither of these would solve it:\n...\n1. **Configurable interrupt threshold.** A setting like\n `keybindings.escapeInterruptPresses` (default `2`, accepts `1`/`2`/`3`) so users can require triple-ESC to interrupt while still keeping single-ESC for clear-input.\n\n2. **Separate rebindable action.** Split the running-task interrupt into its own action (e.g. `task:interruptOnEscape`) that lives in the `Task` context and can be unbound or rebound to a chord like `escape escape escape` independently of `chat:cancel`.\n...\nOption 2 is more flexible and consistent with how other actions are exposed; option 1 is a simpler ship.\n\n---\n\nTitle: Interactive mode - Claude Code Docs\nURL: https://claude-code.mintlify.app/en/interactive-mode\nPublished: N/A\nAuthor: N/A\nHighlights:\n| Shortcut | Description | Context |\n| --- | --- | --- |\n| `Ctrl+C` | Interrupt, or clear input | Interrupts a running operation. If nothing is running, the first press clears the prompt input and a second press exits Claude Code |\n| `Ctrl+X Ctrl+K` | Stop all running background subagents in this session. Press twice within 3 seconds to confirm | Subagent control |\n| `Ctrl+D` | Exit Claude Code session | EOF signal |\n...\n| `Esc` | Interrupt Claude | Stop the current response or tool call mid-turn so you can redirect. Claude keeps the work done so far |\n...\n| `Esc` + `Esc` | Clear input draft, or rewind | When the prompt input contains text, double `Esc` clears it and saves the draft to history so `Up` recalls it. When the input is empty, double `Esc` opens the rewind menu to restore or summarize code and conversation from a previous point |\n...\ncontent |\n| `[` | Write the full conversation to your terminal's native scrollback so\n...\nCmd+F`,\n...\n, and other\n...\n| `v` | Write\n...\n`$EDITOR`. Requires\n...\n|\n| `q`, `Ctrl+C`, `Esc` | Exit transcript view. All three can be rebound via `transcript:exit` |\n...\n1. Cancel search:\n...\n- Press `Ctrl+C` to cancel and restore your original input\n- Press `Backspace` on empty search to cancel"
/allow 73a6fa60-d51a-40e4-b522-2bb2b9949177
#: Control Request:
#:id: "21010987-53e5-4448-b87f-a63469056758"
#:phase: "request"
#:toolName: "github-search-issues"
#:arguments:
#:  query: "ESC interrupt cancel request repo:anthropics/claude-code"
/allow 21010987-53e5-4448-b87f-a63469056758
#: Control Request:
#:id: "57603a22-4c99-47cf-a31f-441896eef191"
#:phase: "result"
#:toolName: "github-search-issues"
#:result:
#:  content: []
#:  structuredContent:
#:    total_count: 69
#:    incomplete_results: false
#:    items:
#:    - author_association: "NONE"
#:      body: |-
#:        During a long interactive session, any keyboard interrupt (Esc) of the main turn also terminates every running background agent (spawned via the Agent tool / Workflow tool). Worse, those agents get flagged "stopped by the user and won't be resumed," so subsequent SendMessage resume attempts are refused — even though the user only meant to interject in the main conversation, not cancel background work.
#:        The only recovery is spawning a fresh agent and manually mining the dead agent's JSONL transcript for partial work.
#:        **Expected behavior:** interrupting the foreground turn should leave background tasks running (or at minimum leave them resumable). Killing background agents should require an explicit TaskStop or an explicit user request.
#:        **Impact:** five research/build agents were silently killed across one session (three separate interrupts), each requiring a relaunch with duplicated token spend. It also creates a confusing UX: the user sees 'no bg tasks' with no indication their interrupt was the cause.
#:        **Environment:** Claude Code desktop app, macOS (darwin 25.5.0), model claude-fable-5. Agents spawned via Agent tool (run_in_background default) and Workflow tool; interrupts were plain Esc presses on the main conversation while agents ran.
#:        🤖 Filed by Claude Code on behalf of the user
#:      comments: 0
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/76807/comments"
#:      created_at: "2026-07-12T02:04:17Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/76807/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/76807"
#:      id: 4865021037
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8223769435
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:macos"
#:        name: "platform:macos"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on macOS"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izDWw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      - id: 9119047308
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:agents"
#:        name: "area:agents"
#:        color: "ededed"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACH4mejA"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/76807/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABIfpUbQ"
#:      number: 76807
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/76807/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "Esc interrupt kills all background agents and marks them permanently unresumable"
#:      updated_at: "2026-07-12T02:05:25Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/76807"
#:      user:
#:        login: "garshythoel"
#:        id: 28829966
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjI4ODI5OTY2"
#:        avatar_url: "https://avatars.githubusercontent.com/u/28829966?v=4"
#:        html_url: "https://github.com/garshythoel"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/garshythoel"
#:        events_url: "https://api.github.com/users/garshythoel/events{/privacy}"
#:        following_url: "https://api.github.com/users/garshythoel/following{/other_user}"
#:        followers_url: "https://api.github.com/users/garshythoel/followers"
#:        gists_url: "https://api.github.com/users/garshythoel/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/garshythoel/orgs"
#:        received_events_url: "https://api.github.com/users/garshythoel/received_events"
#:        repos_url: "https://api.github.com/users/garshythoel/repos"
#:        starred_url: "https://api.github.com/users/garshythoel/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/garshythoel/subscriptions"
#:    - author_association: "NONE"
#:      body: |
#:        ## Summary
#:        When a user interrupts the main agent mid-turn with additional context or corrections (not a cancel), all running subagents and background tasks are aborted without warning, losing partial progress and wasting tokens. This creates friction: users must choose between withholding helpful mid-task context and re-running expensive async operations.
#:        ## Current behavior
#:        1. Main agent launches 3+ research subagents via the Agent tool (reports "Async agent launched successfully").
#:        2. Subagents begin long-running tasks (web research, file processing, etc.).
#:        3. While the main agent is still reasoning or awaiting results, user sends an interjection—e.g., granting permission to read a directory, correcting a misunderstanding, or adding context.
#:        4. The interjection aborts the main agent's turn, terminating all child tasks immediately.
#:        5. Subagents' output files are left empty (0 bytes), no partial results preserved, no completion notice. User must re-dispatch.
#:        **Nuance:** Interjections sent during agent idle time (between turns) do *not* kill background tasks; failure occurs only when children are in-flight and parent turn is incomplete. Explicit `run_in_background: true` appears to detach earlier and may mitigate, but the default async path does not survive.
#:        ## Expected behavior
#:        - An interjection should suspend the main agent's reasoning loop (necessary—user's new input must be seen before the next action), but already-dispatched subagents should continue running and deliver results normally.
#:        - Alternatively: offer a choice—e.g., interjection semantics of "queue for next turn without aborting children" vs. hard cancel (Esc-Esc retains current kill-all behavior).
#:        - If children *must* be killed: persist partial output and surface a clear notice ("N subagents were terminated") so the orchestrator knows to relaunch rather than silently waiting.
#:        ## Why it matters
#:        - **Token waste:** Subagents halfway through long-running research spend tokens on setup, context-loading, and partial work, then discarded.
#:        - **Time penalty:** Re-launching from scratch re-incurs API latency and spinning-up cost; useful multi-minute tasks become blocked on user's impulse to clarify mid-flight.
#:        - **Poor incentives:** Users learn to withhold corrections and context until the turn explicitly ends, reducing real-time collaboration and forcing artificial synchronization.
#:        - **Silent failure:** No indication that children were killed; orchestrators can't distinguish "task crashed" from "was aborted," leading to confusion and duplicate dispatches.
#:        ## Proposed solution
#:        **Option A (preferred):** Interjections interrupt the main agent's reasoning but *not* child lifecycle. Queue the user's message for the next turn after in-flight children complete, or merge it into the agent's context immediately without killing children. Preserve the hard-cancel (Esc-Esc) as an explicit override.
#:        **Option B:** Expose interjection semantics to the user—e.g., "queue for next turn" vs. "cancel all and interrupt now." Esc-Esc / UI affordance for immediate cancel, default to queuing.
#:        **Option C:** On any child termination, persist partial output and emit a visible notice ("N subagents killed by interruption; partial output at [path]") so recovery is explicit and auditable.
#:        Recommend A + C: queue by default, kill-all on demand, always notify if children die.
#:        Environment: Claude Code desktop app on Windows 10, model claude-fable-5.
#:      comments: 2
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/74695/comments"
#:      created_at: "2026-07-06T07:38:46Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/74695/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/74695"
#:      id: 4817001261
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8201037318
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:windows"
#:        name: "platform:windows"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on Windows"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHmBg"
#:      - id: 9119047308
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:agents"
#:        name: "area:agents"
#:        color: "ededed"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACH4mejA"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/74695/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABHx2bLQ"
#:      number: 74695
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/74695/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "Feature request: user interjections should not kill in-flight subagents / background tasks"
#:      updated_at: "2026-07-06T12:09:23Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/74695"
#:      user:
#:        login: "kumouri"
#:        id: 1312174
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjEzMTIxNzQ="
#:        avatar_url: "https://avatars.githubusercontent.com/u/1312174?v=4"
#:        html_url: "https://github.com/kumouri"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/kumouri"
#:        events_url: "https://api.github.com/users/kumouri/events{/privacy}"
#:        following_url: "https://api.github.com/users/kumouri/following{/other_user}"
#:        followers_url: "https://api.github.com/users/kumouri/followers"
#:        gists_url: "https://api.github.com/users/kumouri/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/kumouri/orgs"
#:        received_events_url: "https://api.github.com/users/kumouri/received_events"
#:        repos_url: "https://api.github.com/users/kumouri/repos"
#:        starred_url: "https://api.github.com/users/kumouri/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/kumouri/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ## Environment
#:        - Claude Code CLI v2.1.199, macOS (darwin 25.5.0), terminal session
#:        ## What happened
#:        **Any interaction with the `btw` section interrupts the main thread — this is not limited to pending interactive prompts.** Two observed shapes:
#:        1. **Esc inside btw scope stops the main-thread task.** While the assistant is mid-task on the main thread, entering the btw section and pressing Esc (e.g. to dismiss/leave the btw composer) cancels the main thread's in-flight work instead of just leaving the btw scope. Esc's scope appears to leak: it should close/back out of btw, but it acts on the main thread.
#:        2. **Pending AskUserQuestion auto-rejected.** With an interactive question pending (AskUserQuestion, 3 questions awaiting selection), opening/typing in the btw section immediately cancelled the pending tool call with "Request interrupted by user for tool use" — the user never declined; they were composing in btw and expected the question to remain pending.
#:        The user reports this happens **every time** they are in the btw scope, not only when a question is pending.
#:        ## Expected
#:        - btw-scope interactions (typing, Esc, closing) stay inside the btw scope.
#:        - Esc in btw closes/exits btw only — never interrupts the main thread's running task.
#:        - Pending interactive prompts survive btw activity and are re-presented after leaving btw.
#:        ## Actual
#:        Main-thread work is stopped / pending tool calls are auto-rejected whenever the user interacts with btw, before they leave the btw scope.
#:        ## Impact
#:        - The assistant receives a false "user declined" / interruption signal and may wrongly abandon or restate work; the user must re-request the interrupted action every time.
#:        - Long-running main-thread tasks are killed by what the user intends as a side-channel interaction.
#:        ## Repro
#:        A) 1. Start any long-running assistant task on the main thread. 2. Open btw, press Esc. 3. Main-thread task stops.
#:        B) 1. Have the assistant call AskUserQuestion so the picker is pending. 2. Enter btw and start typing. 3. The pending tool call is rejected immediately.
#:        (Reported on behalf of the user, who asked for this to be filed and then broadened to cover the Esc/main-thread case.)
#:      comments: 0
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/74959/comments"
#:      created_at: "2026-07-06T22:00:18Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/74959/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/74959"
#:      id: 4823411047
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8223769435
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:macos"
#:        name: "platform:macos"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on macOS"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izDWw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/74959/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABH39pZw"
#:      number: 74959
#:      reactions:
#:        total_count: 1
#:        "+1": 1
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/74959/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "Interacting with the 'btw' section auto-rejects a pending AskUserQuestion (main thread interrupted before leaving btw scope)"
#:      updated_at: "2026-07-06T22:05:06Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/74959"
#:      user:
#:        login: "fadykalo"
#:        id: 78373023
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjc4MzczMDIz"
#:        avatar_url: "https://avatars.githubusercontent.com/u/78373023?v=4"
#:        html_url: "https://github.com/fadykalo"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/fadykalo"
#:        events_url: "https://api.github.com/users/fadykalo/events{/privacy}"
#:        following_url: "https://api.github.com/users/fadykalo/following{/other_user}"
#:        followers_url: "https://api.github.com/users/fadykalo/followers"
#:        gists_url: "https://api.github.com/users/fadykalo/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/fadykalo/orgs"
#:        received_events_url: "https://api.github.com/users/fadykalo/received_events"
#:        repos_url: "https://api.github.com/users/fadykalo/repos"
#:        starred_url: "https://api.github.com/users/fadykalo/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/fadykalo/subscriptions"
#:    - author_association: "NONE"
#:      body: |
#:        ### Problem
#:        While an agent or tool call is running, there is no single shortcut that clears the entire prompt input buffer without a side effect:
#:        - `Ctrl+C` — interrupts the running agent; only clears the buffer when idle.
#:        - `Esc` / `Esc Esc` — a single `Esc` interrupts the running turn; the idle double-`Esc` clears the draft but isn't usable mid-run.
#:        - `Ctrl+L` (`chat:clearInput`) — redraws the screen and *preserves* input (it doesn't clear it).
#:        The closest workaround is `Ctrl+U` (`Cmd+Backspace` on macOS), which deletes from the cursor to line start and, when repeated, clears across the lines of a multi-line draft. It does work mid-run without interrupting the agent — but it's one press per line, so it's impractical for a long multi-line prompt.
#:        ### Request
#:        A dedicated, rebindable action that clears the entire prompt buffer in a single keystroke **without cancelling in-progress agent / tool calls**. (Previously requested in #52377, closed without shipping.)
#:        ### Minor: stale action name
#:        Separately, `chat:clearInput` no longer matches its behavior — since v2.1.126 it redraws the screen and *preserves* input rather than clearing it. The name is a legacy leftover and is confusing when reading `keybindings.json`; renaming it would also free `chat:clearInput` for a real clear-input action.
#:        ### Environment
#:        Claude Code 2.1.178 (latest), macOS.
#:      comments: 0
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/68851/comments"
#:      created_at: "2026-06-16T15:08:35Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/68851/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/68851"
#:      id: 4675467412
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8201030515
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/has%20repro"
#:        name: "has repro"
#:        color: "ededed"
#:        description: "Has detailed reproduction steps"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHLcw"
#:      - id: 8223769435
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:macos"
#:        name: "platform:macos"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on macOS"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izDWw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      - id: 10183369448
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/keybindings"
#:        name: "keybindings"
#:        color: "ededed"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACXvni6A"
#:      - id: 10191083653
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/stale"
#:        name: "stale"
#:        color: "888888"
#:        description: "Issue is inactive"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACX2-YhQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/68851/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABFq34lA"
#:      number: 68851
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/68851/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      state_reason: "reopened"
#:      title: "[FEATURE] No single shortcut to clear the entire prompt buffer without interrupting a running agent"
#:      updated_at: "2026-07-14T10:34:10Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/68851"
#:      user:
#:        login: "mimkorn"
#:        id: 5420435
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjU0MjA0MzU="
#:        avatar_url: "https://avatars.githubusercontent.com/u/5420435?v=4"
#:        html_url: "https://github.com/mimkorn"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/mimkorn"
#:        events_url: "https://api.github.com/users/mimkorn/events{/privacy}"
#:        following_url: "https://api.github.com/users/mimkorn/following{/other_user}"
#:        followers_url: "https://api.github.com/users/mimkorn/followers"
#:        gists_url: "https://api.github.com/users/mimkorn/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/mimkorn/orgs"
#:        received_events_url: "https://api.github.com/users/mimkorn/received_events"
#:        repos_url: "https://api.github.com/users/mimkorn/repos"
#:        starred_url: "https://api.github.com/users/mimkorn/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/mimkorn/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ## Summary
#:        When using Claude Code via Remote Control mode from a mobile browser (claude.ai), there is no way to cancel or interrupt a running tool (e.g., WebFetch, Bash) that hangs or takes too long.
#:        On desktop, `Ctrl+C` works. On mobile, the only options are waiting for a timeout or killing the process from another terminal — neither is practical.
#:        ## Use case
#:        - WebFetch hangs on large pages (e.g., Wikipedia 400KB+) with no timeout mechanism
#:        - Long-running Bash commands that unexpectedly stall
#:        - Any tool that blocks the session without a way to recover from mobile
#:        ## Proposed solution
#:        Add a **Cancel / Stop** button in the Remote Control UI (claude.ai web interface) that sends an interrupt signal equivalent to `Ctrl+C` on the desktop app.
#:        ## Current workaround
#:        - PreToolUse hooks to block known problematic tools (e.g., WebFetch)
#:        - Using `curl --max-time` or `timeout` wrappers instead of built-in tools
#:        - These are preventive only — no recovery option exists once a tool hangs
#:        ## Environment
#:        - Claude Code Desktop App on Windows 11
#:        - Remote Control from mobile browser (iOS/Android)
#:      comments: 9
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/47127/comments"
#:      created_at: "2026-04-12T22:45:20Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/47127/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/47127"
#:      id: 4250192591
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 9598927857
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:claude-code-web"
#:        name: "area:claude-code-web"
#:        color: "ededed"
#:        description: "Claude Code on the web. claude.com/code"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACPCQD8Q"
#:      - id: 10002526951
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:web"
#:        name: "platform:web"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on the web"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACVDJy5w"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/47127/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY879VMrP"
#:      number: 47127
#:      reactions:
#:        total_count: 21
#:        "+1": 21
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/47127/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "Feature request: Cancel/interrupt button for Remote Control (mobile)"
#:      updated_at: "2026-05-20T14:54:40Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/47127"
#:      user:
#:        login: "yasumorishima"
#:        id: 90467955
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjkwNDY3OTU1"
#:        avatar_url: "https://avatars.githubusercontent.com/u/90467955?v=4"
#:        html_url: "https://github.com/yasumorishima"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/yasumorishima"
#:        events_url: "https://api.github.com/users/yasumorishima/events{/privacy}"
#:        following_url: "https://api.github.com/users/yasumorishima/following{/other_user}"
#:        followers_url: "https://api.github.com/users/yasumorishima/followers"
#:        gists_url: "https://api.github.com/users/yasumorishima/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/yasumorishima/orgs"
#:        received_events_url: "https://api.github.com/users/yasumorishima/received_events"
#:        repos_url: "https://api.github.com/users/yasumorishima/repos"
#:        starred_url: "https://api.github.com/users/yasumorishima/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/yasumorishima/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ## Summary
#:        When a sub-agent (launched via the Agent tool with a `subagent_type`) makes an API call that returns HTTP 401 because the user's auth token has expired, Claude Code reports the failure to the parent agent as `[Request interrupted by user for tool use]`. The actual `authentication_failed` / 401 error is hidden, and the parent agent has no way to distinguish a real user cancellation from an upstream auth failure.
#:        ## Environment
#:        - Claude Code version: `2.1.191`
#:        - Platform: macOS (Apple Silicon, Homebrew install)
#:        - Session ID (truncated for privacy): `eca7fe60-...`
#:        ## Evidence
#:        The sub-agent's session JSONL transcript contains the real synthetic assistant message (sensitive ids redacted):
#:        ```json
#:        {
#:          "type": "assistant",
#:          "model": "<synthetic>",
#:          "stop_reason": "stop_sequence",
#:          "error": "authentication_failed",
#:          "isApiErrorMessage": true,
#:          "apiErrorStatus": 401,
#:          "content": [{
#:            "type": "text",
#:            "text": "Please run /login · API Error: 401 The security token included in the request is expired"
#:          }]
#:        }
#:        ```
#:        Yet the parent agent's transcript for the same moment shows only:
#:        ```
#:        [Request interrupted by user for tool use]
#:        ```
#:        So the harness saw a synthetic assistant message with `isApiErrorMessage: true`, `apiErrorStatus: 401`, and `error: "authentication_failed"`, but rewrote that into a user-interrupt wrapper before handing it to the parent. The parent never sees the auth-error class.
#:        ## Reproduction
#:        1. Launch a long-running sub-agent via the Agent tool (e.g. one that runs many Bash/Read/Edit cycles over 5+ minutes — long enough that an auth token can plausibly expire mid-run).
#:        2. Wait for the user's auth token to expire mid-run (or simulate by revoking it).
#:        3. The next API call the sub-agent attempts fails with HTTP 401 (`authentication_failed`).
#:        4. Observe: the parent agent sees `[Request interrupted by user for tool use]` as the sub-agent's result and acts on it (typically: asks the user what to do, treats work as user-cancelled).
#:        5. Expected: the parent should see an explicit auth-expired / API-error class (not a user-cancel wrapper), so it can retry after `/login`, prompt the user to re-auth, or otherwise pick a correct recovery path.
#:        ## Why this matters
#:        - **Long-running multi-agent workflows become unreliable.** Orchestrator -> specialist sub-agent patterns (e.g. per-story TDD delivery cycles) cannot distinguish an auth blip from a deliberate user stop.
#:        - **Parent agents take the wrong recovery path.** "User cancelled" -> ask the user for direction. "Auth expired" -> retry after `/login`. These are not interchangeable, and conflating them wastes turns and confuses the user.
#:        - **Users are blamed for cancellations they didn't perform.** The phrasing "Request interrupted by user" tells the model (and any human reading the transcript) that the user hit Esc. They didn't.
#:        - **In-flight sub-agent work is silently dropped** with no diagnostic trail. Committed work survives on disk; in-memory partial reports and uncommitted edits do not, and there is no signal to the parent that the loss happened for an auth reason rather than a deliberate one.
#:        - **Post-hoc diagnosis requires manual forensics** on the local jsonl transcripts. This is not a workflow most users will know to perform.
#:        ## Suggested fix
#:        1. Surface API errors (401, 429, 5xx) verbatim to the parent agent with HTTP status code and original error message intact. The sub-agent transcript already carries `apiErrorStatus`, `error` (class), and `isApiErrorMessage: true` -- propagate these fields.
#:        2. Use a distinct error class -- e.g. `auth_expired`, `rate_limited`, `api_error` -- separate from `user_interrupted`. The current wrapper conflates the two failure modes into one string.
#:        3. (Stretch) Offer to resume the sub-agent automatically once auth recovers, preserving its in-progress tool-call queue. At minimum, give the parent enough signal to choose between "ask user" and "retry on auth recovery."
#:        ## Workaround
#:        None. The parent agent has no way to distinguish a real user cancel from an upstream auth failure from the surfaced signal alone. Inspecting the local session jsonl can identify the cause after the fact, but that requires the user to know to look.
#:        ## Related
#:        - #12447 -- OAuth token expiration disrupts autonomous workflows (root condition: auth expires mid-run; this issue is about how that failure is *reported to a parent agent*, which #12447 doesn't cover).
#:        - #64177 -- Workflow counts API-errored subagents as "completed" (adjacent mis-accounting of sub-agent API errors; different surface and different misclassification, but same shape: parent's view of sub-agent outcome doesn't match what actually happened).
#:        - #62311 -- Distinguish an explicit user interrupt (Esc) from a message that merely arrived mid-turn (related but inverse: that issue is about distinguishing user-interrupt from queued-message; this issue is about distinguishing user-interrupt from auth-failure).
#:        - #25818 -- Orchestrator has no diagnostic context when subagent fails (same family of "parent receives degraded failure info"; closed as stale).
#:      comments: 2
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/70958/comments"
#:      created_at: "2026-06-25T11:16:58Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/70958/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/70958"
#:      id: 4743293060
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8223769435
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:macos"
#:        name: "platform:macos"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on macOS"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izDWw"
#:      - id: 8571182877
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:auth"
#:        name: "area:auth"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_uHfHQ"
#:      - id: 9119047308
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:agents"
#:        name: "area:agents"
#:        color: "ededed"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACH4mejA"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/70958/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABGrjohA"
#:      number: 70958
#:      reactions:
#:        total_count: 1
#:        "+1": 1
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/70958/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "Sub-agent API 401 (auth expired) is reported to parent agent as \"Request interrupted by user for tool use\""
#:      updated_at: "2026-06-25T23:55:10Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/70958"
#:      user:
#:        login: "arcavenai"
#:        id: 264873628
#:        user_view_type: "public"
#:        node_id: "U_kgDOD8mmnA"
#:        avatar_url: "https://avatars.githubusercontent.com/u/264873628?v=4"
#:        html_url: "https://github.com/arcavenai"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/arcavenai"
#:        events_url: "https://api.github.com/users/arcavenai/events{/privacy}"
#:        following_url: "https://api.github.com/users/arcavenai/following{/other_user}"
#:        followers_url: "https://api.github.com/users/arcavenai/followers"
#:        gists_url: "https://api.github.com/users/arcavenai/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/arcavenai/orgs"
#:        received_events_url: "https://api.github.com/users/arcavenai/received_events"
#:        repos_url: "https://api.github.com/users/arcavenai/repos"
#:        starred_url: "https://api.github.com/users/arcavenai/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/arcavenai/subscriptions"
#:    - author_association: "NONE"
#:      body: |
#:        ### Preflight Checklist
#:        - [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet
#:        - [x] This is a single feature request (not multiple features)
#:        ### Problem Statement
#:        ## Summary
#:        Add a new hook event that fires when the user manually interrupts Claude Code's response (e.g., by clicking Stop or pressing Ctrl+C).
#:        ## Problem
#:        Currently, Claude Code provides a `Stop` hook that fires when the agent finishes responding normally. However, according to the documentation:
#:        > "Does not run if the stoppage occurred due to a user interrupt."
#:        This creates a gap: there is **no way to detect when the user manually interrupts the agent**. This prevents implementing features that need to differentiate between:
#:        - Normal completion (agent finished its work)
#:        - User interrupt (user manually stopped the agent)
#:        ### Proposed Solution
#:        Add a new hook event: **`UserInterrupt`** (or `AgentInterrupted`, `Stop:Interrupted`, etc.)
#:        This hook should fire when:
#:        - User clicks the Stop button during agent response
#:        - User presses keyboard interrupt (Ctrl+C or equivalent)
#:        - User manually cancels agent operation through any UI mechanism
#:        ## Example Configuration
#:        ```json
#:        {
#:          "hooks": {
#:            "Stop": [{
#:              "hooks": [{
#:                "type": "command",
#:                "command": "my-script.py --event normal_completion"
#:              }]
#:            }],
#:            "UserInterrupt": [{
#:              "hooks": [{
#:                "type": "command",
#:                "command": "my-script.py --event user_interrupted"
#:              }]
#:            }]
#:          }
#:        }
#:        ```
#:        ### Alternative Solutions
#:        _No response_
#:        ### Priority
#:        Medium - Would be very helpful
#:        ### Feature Category
#:        CLI commands and flags
#:        ### Use Case Example
#:        We're implementing an audio notification system that plays background noise while Claude works and plays a completion sound when finished. We need to:
#:        - Play a "work complete" sound when the agent finishes normally (`Stop` hook)
#:        - Play a different "interrupted" sound when the user manually stops the agent
#:        - **Currently impossible** because there's no hook for user interrupts
#:        ### Additional Context
#:        ## Benefits
#:        1. **Feature parity**: Completes the lifecycle coverage (start → working → end/interrupt)
#:        2. **Better UX**: Enables context-aware notifications and state management
#:        3. **Debugging**: Helps track user interaction patterns and interruption frequency
#:        4. **Resource cleanup**: Allows proper cleanup of resources when operations are interrupted vs. completed
#:        ## Workarounds (and why they don't work)
#:        1. **Timeout-based detection**: Unreliable, creates false positives for long-running tasks
#:        2. **Heartbeat with PreToolUse**: Fires on every tool call, creates noise and complexity
#:        3. **Process monitoring**: External to Claude Code, fragile and platform-specific
#:        ## Related Documentation
#:        - [Hooks Reference](https://docs.claude.com/en/docs/claude-code/hooks.md)
#:        - Current `Stop` hook explicitly excludes interrupts
#:      comments: 25
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/9516/comments"
#:      created_at: "2025-10-14T09:11:05Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/9516/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/9516"
#:      id: 3513055485
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8572719646
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:core"
#:        name: "area:core"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vlSHg"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/9516/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87RZPj9"
#:      number: 9516
#:      reactions:
#:        total_count: 64
#:        "+1": 48
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 9
#:        eyes: 7
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/9516/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[FEATURE] User Interrupt Hook"
#:      updated_at: "2026-07-08T08:47:32Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/9516"
#:      user:
#:        login: "arhgap11b"
#:        id: 114364738
#:        user_view_type: "public"
#:        node_id: "U_kgDOBtERQg"
#:        avatar_url: "https://avatars.githubusercontent.com/u/114364738?v=4"
#:        html_url: "https://github.com/arhgap11b"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/arhgap11b"
#:        events_url: "https://api.github.com/users/arhgap11b/events{/privacy}"
#:        following_url: "https://api.github.com/users/arhgap11b/following{/other_user}"
#:        followers_url: "https://api.github.com/users/arhgap11b/followers"
#:        gists_url: "https://api.github.com/users/arhgap11b/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/arhgap11b/orgs"
#:        received_events_url: "https://api.github.com/users/arhgap11b/received_events"
#:        repos_url: "https://api.github.com/users/arhgap11b/repos"
#:        starred_url: "https://api.github.com/users/arhgap11b/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/arhgap11b/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ### Preflight Checklist
#:        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
#:        - [x] This is a single bug report (please file separate reports for different bugs)
#:        - [x] I am using the latest version of Claude Code
#:        ### What's Wrong?
#:        Description:
#:          When I type a message containing the word "workflow," Claude Code shows a banner along the lines of "Using dynamic workflow — press cmd-w to stop"
#:          (cycling in rainbow colors). The problem is that cmd-w is the universal close-window shortcut in macOS terminal emulators (Terminal.app, iTerm2,
#:          Ghostty, etc.), so pressing it doesn't cancel the workflow — it closes the terminal window and kills the entire Claude Code session, losing the
#:          in-progress conversation.
#:          Steps to reproduce:
#:          1. On macOS, open Claude Code in Terminal.app or iTerm2.
#:          2. Send a message that mentions the word "workflow" in casual conversation (not actually requesting one).
#:          3. Observe the "press cmd-w to stop" banner.
#:          4. Press cmd-w expecting to dismiss the banner.
#:          5. The terminal window closes and the session is lost.
#:          Expected: A cancel shortcut that doesn't shadow a standard terminal keybinding — e.g. esc, ctrl-c, or cmd-shift-w.
#:          Actual: cmd-w closes the terminal window, ending the session and losing conversation context.
#:          Environment:
#:          - macOS (Darwin 24.6.0)
#:          - zsh
#:          - Terminal emulator: iTerm2
#:          - Claude Code version: 2.1.153 (Claude Code)
#:          Additional notes:
#:          - The trigger appears to be the literal word "workflow" appearing in the prompt, even when the user isn't asking for multi-agent orchestration. A
#:          less aggressive trigger (or one that requires an explicit /workflow invocation) would also reduce accidental activations.
#:          - Suggested alternatives for the cancel chord: esc (matches how most other Claude Code prompts cancel), or ctrl-c (standard terminal interrupt).
#:        ### What Should Happen?
#:        Don't trigger on just typing the word, and don't suggest Cmd-W as a keyboard shortcut.
#:        ### Error Messages/Logs
#:        ```shell
#:        ```
#:        ### Steps to Reproduce
#:          Steps to reproduce:
#:          1. On macOS, open Claude Code in Terminal.app or iTerm2.
#:          2. Send a message that mentions the word "workflow" in casual conversation (not actually requesting one).
#:          3. Observe the "press cmd-w to stop" banner.
#:          4. Press cmd-w expecting to dismiss the banner.
#:          5. The terminal window closes and the session is lost.
#:          Expected: A cancel shortcut that doesn't shadow a standard terminal keybinding — e.g. esc, ctrl-c, or cmd-shift-w.
#:          Actual: cmd-w closes the terminal window, ending the session and losing conversation context.
#:        ### Claude Model
#:        Opus
#:        ### Is this a regression?
#:        Yes, this worked in a previous version
#:        ### Last Working Version
#:        _No response_
#:        ### Claude Code Version
#:        2.1.153 (Claude Code)
#:        ### Platform
#:        Anthropic API
#:        ### Operating System
#:        macOS
#:        ### Terminal/Shell
#:        iTerm2
#:        ### Additional Information
#:        _No response_
#:      comments: 0
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/68506/comments"
#:      created_at: "2026-06-15T01:49:02Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/68506/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/68506"
#:      id: 4661437506
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8223769435
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:macos"
#:        name: "platform:macos"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on macOS"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izDWw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      - id: 10183369448
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/keybindings"
#:        name: "keybindings"
#:        color: "ededed"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACXvni6A"
#:      - id: 10191083653
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/stale"
#:        name: "stale"
#:        color: "888888"
#:        description: "Issue is inactive"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACX2-YhQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/68506/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABFdfkQg"
#:      number: 68506
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/68506/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[BUG] Workflow tool's \"press cmd-w to cancel\" hint collides with Terminal/iTerm's close-window shortcut"
#:      updated_at: "2026-07-12T22:18:53Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/68506"
#:      user:
#:        login: "joelburton"
#:        id: 1178518
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjExNzg1MTg="
#:        avatar_url: "https://avatars.githubusercontent.com/u/1178518?v=4"
#:        html_url: "https://github.com/joelburton"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/joelburton"
#:        events_url: "https://api.github.com/users/joelburton/events{/privacy}"
#:        following_url: "https://api.github.com/users/joelburton/following{/other_user}"
#:        followers_url: "https://api.github.com/users/joelburton/followers"
#:        gists_url: "https://api.github.com/users/joelburton/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/joelburton/orgs"
#:        received_events_url: "https://api.github.com/users/joelburton/received_events"
#:        repos_url: "https://api.github.com/users/joelburton/repos"
#:        starred_url: "https://api.github.com/users/joelburton/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/joelburton/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        **Bug Description**
#:        When in Plan Mode's Q&A section, and I have Vim mode enabled, a single hit of the ESC key will erase my message. Maybe bump it up to two escapes for Vim mode?
#:        **Environment Info**
#:        - Platform: win32
#:        - Terminal: windows-terminal
#:        - Version: 2.0.29
#:        - Feedback ID: e1139a04-e647-4208-9646-edd128a31c40
#:        **Errors**
#:        The terminal lists `Error: Request was aborted.` after I hit ESC and I see no way to resume answering my questions without starting over.
#:      comments: 21
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/10621/comments"
#:      created_at: "2025-10-30T09:29:18Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/10621/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/10621"
#:      id: 3569739958
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8201030515
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/has%20repro"
#:        name: "has repro"
#:        color: "ededed"
#:        description: "Has detailed reproduction steps"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHLcw"
#:      - id: 8201037318
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:windows"
#:        name: "platform:windows"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on Windows"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHmBg"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/10621/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87Uxei2"
#:      number: 10621
#:      reactions:
#:        total_count: 29
#:        "+1": 29
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/10621/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[Feature Request] Require double ESC in Vim mode to clear message in Plan Mode Q&A"
#:      updated_at: "2026-07-08T14:15:10Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/10621"
#:      user:
#:        login: "chad-gilbert-lv"
#:        id: 220786601
#:        user_view_type: "public"
#:        node_id: "U_kgDODSjvqQ"
#:        avatar_url: "https://avatars.githubusercontent.com/u/220786601?v=4"
#:        html_url: "https://github.com/chad-gilbert-lv"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/chad-gilbert-lv"
#:        events_url: "https://api.github.com/users/chad-gilbert-lv/events{/privacy}"
#:        following_url: "https://api.github.com/users/chad-gilbert-lv/following{/other_user}"
#:        followers_url: "https://api.github.com/users/chad-gilbert-lv/followers"
#:        gists_url: "https://api.github.com/users/chad-gilbert-lv/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/chad-gilbert-lv/orgs"
#:        received_events_url: "https://api.github.com/users/chad-gilbert-lv/received_events"
#:        repos_url: "https://api.github.com/users/chad-gilbert-lv/repos"
#:        starred_url: "https://api.github.com/users/chad-gilbert-lv/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/chad-gilbert-lv/subscriptions"
#:    - author_association: "NONE"
#:      body: |
#:        ### Preflight Checklist
#:        - [x] I have searched existing requests
#:        - [x] This is a single feature request
#:        ### Problem Statement
#:        Claude Code injects **no timing information** into the model's context — not for user messages, and not for tool calls. The `.jsonl` transcript already records timestamps per API call and per tool result, but none of it reaches the model's reasoning. This creates two concrete, recurring failure modes:
#:        **1. The model can't tell a slow tool from a broken one.**
#:        When a tool call legitimately takes ~2 minutes (a slow DB query, a cold Lambda, a remote MCP gateway), the model has no perception that time is passing. From its point of view the request and response are adjacent tokens. So it has no basis to distinguish *"this is just slow, keep waiting"* from *"something is wrong."* In practice this leads the model to second-guess a healthy operation, assume a hang, or prematurely re-issue/abandon the call — when nothing was actually wrong.
#:        **2. Interrupts + queued messages collapse temporally.**
#:        While a tool call is in flight, the user often types follow-ups ("waiting long?", "?", "it took long?") and may then cancel with `Esc`. After the interrupt, those queued messages are presented to the model **back-to-back with no timing**, so the model treats them as if they were all sent simultaneously and reads none of them as *"time has elapsed, the tool is taking a while."* The temporal structure that would explain the user's growing impatience is entirely lost.
#:        *(Observed live: a remote MCP tool call sat pending; the user sent three impatience prompts over a couple of minutes and then interrupted. The model had no signal that real time had passed between any of them.)*
#:        ### Proposed Solution
#:        Surface timestamps the harness already has into the model's context:
#:        1. **Tool-call request + response timestamps** (and/or a computed `duration_ms`), so the model can reason: *"this tool has been running 90s — normal for this tool, keep waiting"* vs. *"5s and still nothing, that's abnormal."*
#:        2. **Per-user-message timestamps**, so queued/interrupted messages carry their real arrival times and the model perceives the gaps between them.
#:        Format could be lightweight (e.g. `[+1m42s]` deltas, or ISO 8601), and gated behind a setting / hidden from the UI to avoid clutter. The data already exists in the transcript — this is primarily about piping it into context.
#:        ### Related (narrower, both closed/locked)
#:        - #28502 — *Model awareness of elapsed time between messages* (closed, not-planned). Focuses only on **user-message** timestamps; a `UserPromptSubmit` + `date` hook workaround is noted there, but a commenter explicitly flags that it **cannot reach tool-result timing**.
#:        - #32566 — *Expose per-message timestamps to Claude's context* (closed as duplicate of #28502, locked). Also **user-message** only.
#:        This request adds the dimension those issues omit: **tool-call request/response timing**, which is what lets the model judge whether a long-running tool is healthy or stuck — and which no `UserPromptSubmit` hook can provide.
#:        ### Priority
#:        Medium–High — affects reasoning correctness during any long-running tool/MCP call.
#:      comments: 0
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/68013/comments"
#:      created_at: "2026-06-12T18:25:40Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/68013/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/68013"
#:      id: 4651637678
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8572664217
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tools"
#:        name: "area:tools"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vh5mQ"
#:      - id: 8572719646
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:core"
#:        name: "area:core"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vlSHg"
#:      - id: 10191083653
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/stale"
#:        name: "stale"
#:        color: "888888"
#:        description: "Issue is inactive"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACX2-YhQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/68013/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABFUJbrg"
#:      number: 68013
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/68013/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[FEATURE] Surface tool-call request/response timestamps to the model so it can perceive elapsed time during long-running tools"
#:      updated_at: "2026-07-10T11:03:59Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/68013"
#:      user:
#:        login: "mbasaran"
#:        id: 25617045
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjI1NjE3MDQ1"
#:        avatar_url: "https://avatars.githubusercontent.com/u/25617045?v=4"
#:        html_url: "https://github.com/mbasaran"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/mbasaran"
#:        events_url: "https://api.github.com/users/mbasaran/events{/privacy}"
#:        following_url: "https://api.github.com/users/mbasaran/following{/other_user}"
#:        followers_url: "https://api.github.com/users/mbasaran/followers"
#:        gists_url: "https://api.github.com/users/mbasaran/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/mbasaran/orgs"
#:        received_events_url: "https://api.github.com/users/mbasaran/received_events"
#:        repos_url: "https://api.github.com/users/mbasaran/repos"
#:        starred_url: "https://api.github.com/users/mbasaran/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/mbasaran/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ## Problem
#:        When Claude is actively working on a task, the only option is to interrupt it. If I think of a follow-up mid-task I either have to interrupt and potentially derail the current work, or try to remember it until Claude finishes.
#:        ## Proposed Solution
#:        Add a **message queue mode** alongside the existing interrupt behavior:
#:        - A toggle or modifier (e.g. Shift+Enter, or a dedicated queue button) that sends the message to a queue instead of interrupting
#:        - Claude finishes its current task, then automatically picks up the queued message in order
#:        - User can choose between two modes:
#:          - **Interrupt** (current behavior) — sends immediately, stops current task
#:          - **Queue** (new) — holds the message, delivered automatically when the current task completes
#:        ## Why It Matters
#:        Long-running tasks (builds, encodes, multi-step workflows) are common in Claude Code. A queue lets users capture follow-up thoughts as they happen without derailing active work. It also enables a more natural async workflow where the user stays in flow rather than waiting and watching for task completion before typing the next thing.
#:        ## UX Suggestion
#:        A small queue indicator near the input box showing pending messages would give confidence that the queued message was captured and will be acted on.
#:      comments: 39
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/50246/comments"
#:      created_at: "2026-04-18T02:44:26Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/50246/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/50246"
#:      id: 4286431809
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/50246/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87_fcJB"
#:      number: 50246
#:      reactions:
#:        total_count: 164
#:        "+1": 146
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 12
#:        hooray: 0
#:        rocket: 6
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/50246/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "Feature Request: Message queue mode — queue messages instead of interrupting active tasks"
#:      updated_at: "2026-07-08T16:34:26Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/50246"
#:      user:
#:        login: "mozltovcoktail"
#:        id: 1244967
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjEyNDQ5Njc="
#:        avatar_url: "https://avatars.githubusercontent.com/u/1244967?v=4"
#:        html_url: "https://github.com/mozltovcoktail"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/mozltovcoktail"
#:        events_url: "https://api.github.com/users/mozltovcoktail/events{/privacy}"
#:        following_url: "https://api.github.com/users/mozltovcoktail/following{/other_user}"
#:        followers_url: "https://api.github.com/users/mozltovcoktail/followers"
#:        gists_url: "https://api.github.com/users/mozltovcoktail/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/mozltovcoktail/orgs"
#:        received_events_url: "https://api.github.com/users/mozltovcoktail/received_events"
#:        repos_url: "https://api.github.com/users/mozltovcoktail/repos"
#:        starred_url: "https://api.github.com/users/mozltovcoktail/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/mozltovcoktail/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ### Preflight Checklist
#:        - [x] I have searched existing issues and this hasn't been reported as a bug yet (related: #47127 feature request for Cancel button, #28508 AskUserQuestion sync bug — both distinct from this)
#:        - [x] This is a single bug report
#:        - [x] I am using the latest version of Claude Code
#:        ### What's Wrong?
#:        When using Claude Code via Remote Control (claude.ai/code) from a mobile browser, the CLI session on my laptop **silently enters a frozen state mid-execution** and becomes permanently unresponsive to any input from the mobile client. The only recovery mechanism is pressing `Esc` (or any key) on the **local terminal** — no action from the mobile side can unstick it.
#:        This is distinct from:
#:        - #47127 (feature request for a Cancel button to interrupt a *visibly running* tool)
#:        - #28508 (`AskUserQuestion` responses from mobile not propagating to CLI)
#:        In my case there is **no visible tool call in progress**, no `AskUserQuestion`, no rate limit, no API error, no obvious trigger. The session just stops advancing after 2-4 normal message exchanges, and from the mobile client it looks like every new message is being sent but nothing ever comes back.
#:        ### Reproduction pattern (consistent across multiple sessions/projects)
#:        1. Start Claude Code on laptop (VSCode integrated terminal, Windows 11)
#:        2. Leave the laptop at home, commute to a meeting / cafe / etc.
#:        3. Open claude.ai/code on mobile browser, connect Remote Control to the running session
#:        4. Send a message — works fine. Claude responds, runs tools (Bash, MCP, Write, Edit), completes
#:        5. Send another message — works fine
#:        6. After 2-4 successful exchanges, during what appears to be a routine turn, the session becomes unresponsive
#:        7. The mobile client continues to accept text input and send messages, but **none of them advance the CLI**. No new responses, no tool calls, no errors. Messages appear to queue silently.
#:        8. Attempting `/clear`, sending random text, reconnecting the Remote Control tab, refreshing the browser — none of these wake the session
#:        9. Hours later, upon returning home, I press `Esc` once in the local terminal → the session **immediately** resumes, processes the queued messages, and works normally again
#:        ### Evidence the session is not truly hung
#:        - The CLI process stays alive and in a readable/writable state (proven by the fact that `Esc` locally revives it instantly)
#:        - No API errors are logged in the `.jsonl` session file during the frozen period
#:        - When I inspected the `.jsonl` after recovery, the last entry before the freeze was a successful `tool_use` completion — there was no sign of streaming stall or network failure
#:        - This is not #25979 (API streaming stall) because that requires `kill -9` to recover, while my case is fixed by a single local keystroke
#:        ### Hypothesis
#:        The CLI input-handling loop appears to enter a state where it is waiting for a **local TTY keystroke** (possibly as a confirmation, prompt, or UI state flag) that Remote Control's input channel does not deliver. This would explain why local `Esc` instantly unsticks it while remote input does not.
#:        A possible cause: whatever UI prompt/permission/spinner state the CLI enters after certain tool calls may require a synchronous keypress from the terminal's `process.stdin` raw mode, which the Remote Control bridge isn't wired to send.
#:        ### What Should Happen?
#:        - Remote Control input should be fully equivalent to local TTY input — any state that local `Esc`/`Enter` can unstick should also be unstickable from mobile
#:        - At minimum, a visible "session frozen — local intervention required" indicator in the mobile UI so users aren't silently blocked
#:        - Ideally, automatic detection of "messages queued but not being processed" with a remote unstick mechanism
#:        ### Business impact
#:        I'm a solo founder (GT Dev Solutions, Madrid). Remote Control is a **critical** productivity feature for me — it's meant to let me keep working on client projects, reviewing deploys, and resolving urgent bugs while away from my laptop. In practice, with this bug, I can only rely on Remote Control for very short interactions (<5 messages) before needing to physically return home.
#:        Yesterday (2026-04-19) I lost several hours of work while two separate sessions were frozen in this exact state. Both recovered instantly when I got home and pressed `Esc`. This has happened repeatedly over the past weeks — it's not an isolated incident.
#:        ### Error Messages/Logs
#:        ```shell
#:        No errors. The JSONL session file simply stops receiving new entries after the freeze point.
#:        ```
#:        ### Steps to Reproduce
#:        1. Windows 11, Claude Code 2.1.112+ in VSCode integrated terminal
#:        2. Start a session with any project and MCP servers loaded (I have 18 MCPs configured globally, but the bug occurs even without any MCP calls)
#:        3. Leave the laptop running, connect from mobile browser via claude.ai/code Remote Control
#:        4. Interact normally — the freeze occurs non-deterministically after 2-4 exchanges, usually after (but not only after) a tool call completes
#:        5. Any further mobile input has no effect until local keypress
#:        ### Claude Model
#:        Opus
#:        ### Is this a regression?
#:        Possibly — Remote Control is relatively new. I don't have a known-working Claude Code version for this specific feature since adoption was gradual.
#:        ### Last Working Version
#:        _No response_
#:        ### Claude Code Version
#:        2.1.112 (also occurred in 2.1.108 and earlier 2.1.x)
#:        ### Platform
#:        Anthropic API (Max plan)
#:        ### Operating System
#:        Windows 11 (laptop/CLI side) + Android browser (mobile side)
#:        ### Terminal/Shell
#:        VSCode integrated terminal (bash/git-bash on Windows)
#:        ### Additional Information
#:        - Happens consistently across projects (I have multiple parallel sessions for different clients)
#:        - Does NOT correlate with any specific tool — happens after Bash, Write, Edit, MCP calls, or even plain text responses
#:        - Does NOT correlate with rate limits (I'm on Max plan, and the freeze persists long after any rate window would reset)
#:        - Browser refresh on mobile does not re-sync — the issue is on the CLI side, not the web client
#:        - Related but distinct: #47127 (feature request for Cancel button), #28508 (AskUserQuestion mobile sync), #25979 (API streaming stall requiring `kill -9`)
#:        Workaround: none remotely. Must physically access the laptop to press a key.
#:      comments: 15
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/51267/comments"
#:      created_at: "2026-04-20T18:58:37Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/51267/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/51267"
#:      id: 4297991755
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8201030515
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/has%20repro"
#:        name: "has repro"
#:        color: "ededed"
#:        description: "Has detailed reproduction steps"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHLcw"
#:      - id: 8201037318
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:windows"
#:        name: "platform:windows"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on Windows"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHmBg"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/51267/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABAC4mSw"
#:      number: 51267
#:      reactions:
#:        total_count: 16
#:        "+1": 16
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/51267/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[BUG] Remote Control (mobile): session silently hangs mid-execution; only local Esc recovers it — no remote unstick mechanism"
#:      updated_at: "2026-07-12T18:50:29Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/51267"
#:      user:
#:        login: "GustavoVzla"
#:        id: 123503132
#:        user_view_type: "public"
#:        node_id: "U_kgDOB1yCHA"
#:        avatar_url: "https://avatars.githubusercontent.com/u/123503132?v=4"
#:        html_url: "https://github.com/GustavoVzla"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/GustavoVzla"
#:        events_url: "https://api.github.com/users/GustavoVzla/events{/privacy}"
#:        following_url: "https://api.github.com/users/GustavoVzla/following{/other_user}"
#:        followers_url: "https://api.github.com/users/GustavoVzla/followers"
#:        gists_url: "https://api.github.com/users/GustavoVzla/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/GustavoVzla/orgs"
#:        received_events_url: "https://api.github.com/users/GustavoVzla/received_events"
#:        repos_url: "https://api.github.com/users/GustavoVzla/repos"
#:        starred_url: "https://api.github.com/users/GustavoVzla/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/GustavoVzla/subscriptions"
#:    - author_association: "NONE"
#:      body: |2-
#:        ## Problem Statement
#:        When Claude Code is mid-turn (thinking, calling tools, writing code), the only way to send a message is as an **interrupt** — it gets injected at the next tool boundary and changes the trajectory of the current task.
#:        This creates a tension: you often think of the *next* thing you want while watching the current task execute, but you have no way to queue it without derailing what's in progress. The options today are:
#:        1. **Interrupt now** — risks confusing the current task, especially if the injected message is unrelated to what Claude is actively doing.
#:        2. **Wait and remember** — sit idle until the turn ends, then type it. Easy to forget, breaks flow, and wastes the dead time where you could have been composing your next instruction.
#:        3. **Stash it externally** — copy it to a notepad or scratchpad. Friction-heavy workaround for something the tool should handle natively.
#:        There's a meaningful UX gap between "inject this immediately" and "send this when you're done." The interrupt model assumes every mid-turn message is urgent and contextual to the current step — but often it's just "here's what I want next."
#:        ## Proposed Solution
#:        ### Core: Deferred Messages
#:        Introduce a **deferred message** — a message composed mid-turn that queues and fires only at **end of turn**, not at the next tool boundary.
#:        Behavior:
#:        - While Claude is mid-turn, the user composes a message and marks it as deferred.
#:        - The message is held in a queue, invisible to Claude until the current turn fully resolves.
#:        - Once the turn ends (Claude yields back to the user), the deferred message automatically fires as the next user turn — as if the user had typed it fresh at the prompt.
#:        - If the user cancels the deferral before end of turn, the message is discarded (or returned to the input buffer for editing).
#:        This preserves the interrupt model for urgent mid-turn corrections while adding a clean "I'll deal with this next" lane.
#:        ### Input Affordance
#:        The user needs a clear, low-friction way to mark a message as deferred vs. an interrupt. Several options (not mutually exclusive):
#:        - **`/defer` prefix** — Type `/defer <message>` to queue it. Simple, discoverable, consistent with the existing slash command vocabulary.
#:        - **Keyboard shortcut** — `Ctrl+D` (or another binding) toggles the current input between "interrupt" and "deferred" mode before sending. A visual indicator in the input area confirms which mode is active.
#:        - **Extend the existing stash system** — The stash already handles "I typed something but don't want to send it right now." Deferred messages are a natural extension: instead of stash = discard, add stash-to-end-of-turn as a second destination. This could unify the interrupt/defer/stash concepts into one coherent input queue system:
#:          - **Send** (Enter) → interrupt (current behavior)
#:          - **Defer** (Ctrl+D or `/defer`) → queue for end of turn
#:          - **Stash** (Escape) → save for later, manual recall
#:        ### Queue Visibility and Management
#:        Once a message is deferred, it shouldn't vanish into a black box. The user needs:
#:        - **Persistent visual indicator** — A badge, status bar element, or subtle banner showing that a deferred message is queued (e.g., `⏳ 1 deferred message`). This prevents the "did I actually queue that?" uncertainty.
#:        - **Preview/inspect** — Ability to view the queued message content without firing it.
#:        - **Edit** — Modify the deferred message before it fires. The current task's output might change what you want to say.
#:        - **Cancel** — Remove the deferred message from the queue entirely.
#:        - **Promote to interrupt** — If you change your mind and decide the message *is* urgent, promote it from deferred to immediate interrupt without retyping.
#:        ### Transition Pause
#:        When the current turn ends and a deferred message is about to fire, provide a brief **transition window** (2-3 seconds, or until the user presses a key) with a visual indicator like:
#:        ```
#:        ✓ Turn complete.
#:        ⏳ Deferred message firing in 3s... [Enter] to fire now · [Esc] to cancel
#:        ```
#:        This lets the user read the output of the completed turn and decide whether the deferred message still makes sense given the result. Without this pause, the deferred message fires blindly and might be stale or wrong.
#:        ## Alternative Solutions
#:        **Current workarounds and why they fall short:**
#:        - **Interrupting** — Works for urgent corrections but is a blunt instrument. Mid-turn injections can confuse multi-step tasks, and there's no way to say "this isn't urgent, just queue it."
#:        - **Waiting** — The simplest workaround, but it wastes the user's time and relies on memory. On long-running turns, you might forget what you wanted to say or lose the precise phrasing.
#:        - **External notepad** — Copy-paste from a text editor works but adds friction and context switches. The tool should handle its own input queue.
#:        - **Stashing** — The stash system gets halfway there (saves input for later) but requires manual recall and doesn't have a "fire at end of turn" destination.
#:        ## Priority
#:        **High — Significant impact on productivity**
#:        This affects every session where Claude runs multi-step tasks. The longer the turn, the more valuable deferral becomes — and turns are getting longer as Claude handles more complex work autonomously.
#:        ## Feature Category
#:        Interactive mode (TUI)
#:        ## Use Case Example
#:        **Scenario: Multi-step refactor with a follow-up task**
#:        1. You ask Claude to refactor an authentication module — a multi-step task involving reading files, planning, and editing across several files.
#:        2. While watching Claude work through the refactor (3-4 minutes in), you realize you also want it to update the tests for the refactored module.
#:        3. **Today:** You either interrupt (risking confusion mid-refactor) or try to remember "update the tests" for 5+ minutes until Claude finishes.
#:        4. **With deferred messages:** You type `/defer Now update the test suite to match the refactored auth module` and keep watching. The message queues silently. Claude finishes the refactor cleanly, the transition pause appears, you confirm, and Claude picks up the test update with full context of what just changed.
#:        **Scenario: Course correction that isn't urgent**
#:        1. Claude is mid-turn implementing a feature. You notice it chose approach A, but you'd prefer approach B — however, approach A will still work and you don't want to interrupt the flow.
#:        2. You defer a message: `/defer For the next iteration, let's switch to approach B because [reason]`.
#:        3. Claude finishes the current implementation cleanly, then picks up your preference for the next pass.
#:        ## Additional Context
#:        ### Compaction Resilience
#:        Deferred messages should survive context compaction. If Claude hits context limits mid-turn and compresses the conversation, the deferred queue must be preserved independently — it's user intent that hasn't been acted on yet. Losing it silently would break trust in the feature.
#:        ### Suggested Companion QoL (worth shipping together)
#:        These small additions would round out the feature and prevent immediate follow-up requests:
#:        - **Deferred message count in the status bar** — Wherever Claude Code shows session state (model, mode, token count), include the deferred queue count. Zero-cost awareness.
#:        - **Hook event: `deferred-message-fire`** — Emit a hook event when a deferred message is about to fire. This lets users with custom hooks (sound effects, notifications, logging) react to the transition. Consistent with the existing hook model.
#:        - **`/defer --list`** — Quick way to inspect the queue from the input line without a separate UI panel. Lightweight and consistent with CLI conventions.
#:        - **Keyboard shortcut to recall last deferred message** — If you cancel a deferred message during the transition pause, a shortcut (e.g., `Ctrl+Shift+D`) to pull it back into the input buffer. Prevents "I canceled but actually I want it back."
#:        ### Prior Art
#:        - **Terminal multiplexers** (tmux, screen) have paste buffers and command queues — but no concept of "deferred to end of current process."
#:        - **IDE task runners** (VS Code) let you queue tasks in sequence, but that's task-level, not conversational.
#:        - **Chat apps** (Slack) have scheduled messages — similar concept (compose now, deliver later) but time-based rather than event-based.
#:        The deferred message concept is most analogous to **email scheduling** (compose now, send later) applied to an interactive agent loop — except the trigger is "agent yields" rather than a clock time.
#:      comments: 4
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/63190/comments"
#:      created_at: "2026-05-28T16:16:51Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/63190/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/63190"
#:      id: 4541970584
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/63190/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABDrj4mA"
#:      number: 63190
#:      reactions:
#:        total_count: 5
#:        "+1": 5
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/63190/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[FEATURE] Deferred Messages — Queue Input for End of Turn"
#:      updated_at: "2026-07-14T08:12:56Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/63190"
#:      user:
#:        login: "riptscripts"
#:        id: 120341369
#:        user_view_type: "public"
#:        node_id: "U_kgDOByxDeQ"
#:        avatar_url: "https://avatars.githubusercontent.com/u/120341369?v=4"
#:        html_url: "https://github.com/riptscripts"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/riptscripts"
#:        events_url: "https://api.github.com/users/riptscripts/events{/privacy}"
#:        following_url: "https://api.github.com/users/riptscripts/following{/other_user}"
#:        followers_url: "https://api.github.com/users/riptscripts/followers"
#:        gists_url: "https://api.github.com/users/riptscripts/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/riptscripts/orgs"
#:        received_events_url: "https://api.github.com/users/riptscripts/received_events"
#:        repos_url: "https://api.github.com/users/riptscripts/repos"
#:        starred_url: "https://api.github.com/users/riptscripts/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/riptscripts/subscriptions"
#:    - author_association: "NONE"
#:      body: |
#:        ## Summary
#:        These two bugs have been plaguing users for months (see #26224 — 28 comments, #6836 — 150+ reports), with no root cause analysis from the team. After yet another day of babysitting Claude Code and pressing ESC every few minutes to revive a hung agent, we decided to conduct our own deep investigation — reverse-engineering `cli.js` across 12 npm package versions and analyzing 1,571 session JSONL files containing 148,444 tool calls.
#:        Here are the exact root causes and proposed fixes.
#:        Claude Code hangs indefinitely when an SSE streaming connection silently dies. There is no client-side timeout or heartbeat detection, so the process waits forever for events that will never arrive. ESC partially works around this by aborting the dead connection, but the queue auto-restart mechanism (`queue.length > 0 → n()`) immediately starts the next queued prompt instead of returning control to the user.
#:        **Root cause identified in source code** — two separate issues in `cli.js`:
#:        1. **No streaming timeout**: The `messages.stream()` call has no timeout. If the SSE connection dies silently (TCP half-open), the client waits forever.
#:        2. **Queue auto-restart after abort**: After ESC aborts a hung request, `if (queue.length > 0) { n(); return; }` immediately starts the next queued prompt. The user cannot fully cancel.
#:        ## Environment
#:        - Claude Code: 2.1.74 (also confirmed on 2.1.50–2.1.73)
#:        - OS: Windows 10, Git Bash
#:        - Model: Opus 4.6
#:        - API: Anthropic direct (not Bedrock/Vertex)
#:        ## Reproduction
#:        1. Start a Claude Code session
#:        2. Submit a prompt → agent starts processing
#:        3. Wait for a hang (0 tokens, timer running, no progress) — happens ~10-15% of prompts
#:        4. Submit another prompt while hung → goes to queue
#:        5. Press ESC
#:        6. **Expected:** Cancel everything, return to `❯`
#:        7. **Actual:** Cancels the hung prompt, immediately starts the queued one
#:        ## Frequency
#:        Measured across **1,571 sessions** using a custom JSONL analyzer tool:
#:        | Period | Versions | Orphan rate (lost tool calls) |
#:        |--------|----------|-------------------------------|
#:        | Dec 2025 | 2.0.72–2.1.2 | 6–14% |
#:        | Jan 2026 | 2.1.5–2.1.23 | 5–10% |
#:        | Feb 2026 | 2.1.29–2.1.56 | 3–8% |
#:        | Mar 2026 | 2.1.69–2.1.74 | 2.4–4% |
#:        The hang frequency has been **increasing** over time: rare in fall 2025, now ~10-15% of prompts per hour.
#:        ## Source Code Analysis
#:        Analyzed `cli.js` extracted from `npm pack @anthropic-ai/claude-code` across versions 2.0.72 through 2.1.74.
#:        ### Issue 1: No streaming timeout
#:        The API call at approximately offset 2,553,870 in cli.js (v2.1.74):
#:        ```js
#:        client.beta.messages.stream({...params}, options)
#:        ```
#:        There is no timeout parameter, no keepalive check, and no heartbeat detection. The Anthropic SSE API sends periodic `:ping` comments, but the client does not monitor for their absence.
#:        When the TCP connection silently dies (common on Windows, WiFi, VPN, or after laptop sleep), the Node.js HTTP client has no way to know the connection is dead. The `AbortController` signal is never triggered because no error event fires.
#:        **Evidence**: Packet inspection by other reporters confirms the client is stuck waiting for SSE events that never arrive. Token count stays at 0. ESC + re-submit creates a new connection that works immediately.
#:        ### Issue 2: Queue auto-restart prevents full cancellation
#:        The main processing loop (offset ~11,400,559 in v2.1.74):
#:        ```js
#:        n = async () => {
#:          if (M) return;       // running guard
#:          M = true;
#:          // ... prepare input, call API, process response ...
#:        }
#:        ```
#:        After completion or abort — in the `finally` block (offset ~11,406,174):
#:        ```js
#:        finally {
#:          M = false;           // clear running guard
#:          W6.start();          // restart idle timer
#:        }
#:        if (c36()) {           // c36() = yY.length > 0 = queue not empty?
#:          n();                 // YES → immediately restart with queued message!
#:          return;              // without returning control to user!
#:        }
#:        ```
#:        Historical analysis of npm packages confirms this pattern exists since **v2.1.50** (as `queue.length > 0`) and was refactored to `c36()` in v2.1.74.
#:        ### Issue 3: JSONL writer race condition (related)
#:        The session writer class `LZq` (offset ~10,549,000) has a non-atomic `insertMessageChain()` that writes assistant (tool_use) and user (tool_result) messages **one at a time** in a loop:
#:        ```js
#:        async insertMessageChain(A, q, K, Y, z) {
#:          return this.trackWrite(async () => {
#:            for (let H of A) {
#:              await this.appendEntry(M);  // each message separately!
#:            }
#:          });
#:        }
#:        ```
#:        If the process is interrupted between writing tool_use and tool_result, the tool_use becomes orphaned. This is the root cause of issue #6836.
#:        ## Proposed Fixes
#:        ### Fix 1: Streaming timeout (critical)
#:        Add a client-side timeout that aborts and retries if no SSE events are received within N seconds:
#:        ```js
#:        // Pseudocode
#:        const STREAM_IDLE_TIMEOUT_MS = 30_000;
#:        let lastEventTime = Date.now();
#:        stream.on('event', () => { lastEventTime = Date.now(); });
#:        const watchdog = setInterval(() => {
#:          if (Date.now() - lastEventTime > STREAM_IDLE_TIMEOUT_MS) {
#:            clearInterval(watchdog);
#:            abortController.abort();
#:            // retry with new connection
#:          }
#:        }, 5_000);
#:        ```
#:        The Anthropic API sends `:ping` SSE comments periodically. Monitoring for these would detect stale connections without false positives.
#:        ### Fix 2: ESC should clear the queue
#:        When the user presses ESC during a hang, the queue should be cleared (or the user should be asked):
#:        ```js
#:        // After abort, before checking queue:
#:        if (userInitiatedAbort && c36()) {
#:          // Option A: Clear queue entirely
#:          clearQueue();
#:          return; // back to prompt
#:          // Option B: Ask user
#:          // "You have N queued messages. Clear queue? (y/n)"
#:        }
#:        ```
#:        ### Fix 3: Atomic message chain writes
#:        `insertMessageChain()` should serialize the entire chain as a single `appendToFile()` call:
#:        ```js
#:        async insertMessageChain(messages) {
#:          const serialized = messages.map(m => JSON.stringify(m)).join('\n') + '\n';
#:          await this.appendToFile(sessionFile, serialized);
#:        }
#:        ```
#:        Note: `history.jsonl` already uses `proper-lockfile` for file locking — the same approach should be applied to session JSONL files when multiple agents write concurrently.
#:        ## Related Issues
#:        - #6836 — Orphaned tool_use/tool_result pairs (150+ reports)
#:        - #26224 — Agent hangs 5-20 minutes, 0 tokens
#:        - #31328 — JSONL writer drops assistant entry during parallel tool calls
#:        - #20171 — Phantom "Generating..." state after task completion
#:        - #24688 — Freeze during API call, terminal unresponsive
#:        - #7243 — `.claude.json` architectural issues (non-atomic writes, no separation of concerns)
#:        - #14642 — Systemic stability problems driving users to build their own tools
#:        ## Methodology
#:        Analysis performed using:
#:        - **ccdiag**: Custom Go CLI tool that parses JSONL session files, detects orphaned tool calls, analyzes timing, and scans multiple sessions
#:        - **Source analysis**: `cli.js` extracted from npm packages across 12 versions (2.0.72 through 2.1.74), searched for queue/abort/streaming patterns
#:        - **Session data**: 1,571 sessions, 148,444 tool calls, 8,007 orphaned
#:      comments: 38
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/33949/comments"
#:      created_at: "2026-03-13T10:42:16Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/33949/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/33949"
#:      id: 4070293527
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8201030515
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/has%20repro"
#:        name: "has repro"
#:        color: "ededed"
#:        description: "Has detailed reproduction steps"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHLcw"
#:      - id: 8201037318
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:windows"
#:        name: "platform:windows"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on Windows"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHmBg"
#:      - id: 8572719646
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:core"
#:        name: "area:core"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vlSHg"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/33949/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87ym8AX"
#:      number: 33949
#:      reactions:
#:        total_count: 25
#:        "+1": 24
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 1
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/33949/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "SSE streaming hangs indefinitely (no timeout) + ESC cannot fully cancel (queue auto-restart) — root cause analysis with fix proposals"
#:      updated_at: "2026-07-14T06:23:05Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/33949"
#:      user:
#:        login: "kolkov"
#:        id: 3740898
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjM3NDA4OTg="
#:        avatar_url: "https://avatars.githubusercontent.com/u/3740898?v=4"
#:        html_url: "https://github.com/kolkov"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/kolkov"
#:        events_url: "https://api.github.com/users/kolkov/events{/privacy}"
#:        following_url: "https://api.github.com/users/kolkov/following{/other_user}"
#:        followers_url: "https://api.github.com/users/kolkov/followers"
#:        gists_url: "https://api.github.com/users/kolkov/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/kolkov/orgs"
#:        received_events_url: "https://api.github.com/users/kolkov/received_events"
#:        repos_url: "https://api.github.com/users/kolkov/repos"
#:        starred_url: "https://api.github.com/users/kolkov/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/kolkov/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ### Problem
#:        Pressing **Esc to interrupt/cancel a running turn fires no hook event at all**. This makes it impossible for `statusLine` / hook-based integrations to reflect that the agent stopped — they stay frozen on whatever the last state was (e.g. "thinking") until something else happens.
#:        The closest existing signals don't cover it:
#:        - `Stop` / `StopFailure` do **not** fire on a user interrupt.
#:        - The `idle_prompt` `Notification` only fires after ~60s of genuine inactivity — so if you interrupt and immediately start typing, it never fires.
#:        ### Evidence
#:        I instrumented a logger on **all 29 non-streaming hook events** and did a controlled interrupt. After `UserPromptSubmit` (which set my "thinking" state), I pressed Esc — and **not a single hook event fired** for 37+ seconds:
#:        ```
#:        SessionStart
#:        InstructionsLoaded
#:        UserPromptSubmit     <- prompt sent (state -> thinking)
#:           …Esc interrupt here…
#:           (no events at all)
#:        ```
#:        Every other state transition my integration relies on rides on an event that *does* fire (`UserPromptSubmit`, `PreToolUse`, `Stop`, `StopFailure`, `Notification`). Interrupt is the one action that emits nothing.
#:        ### Proposal
#:        Emit a hook event when the user interrupts a turn. Any of:
#:        - a dedicated `TurnInterrupted` / `TurnCancelled` event, or
#:        - have `Stop` fire on interrupt with a `reason` field (e.g. `reason: "interrupted"`), or
#:        - have `StopFailure` fire with `reason: "interrupted"`.
#:        ### Use case
#:        Building an expressive status-line that shows the agent's state (thinking / reading / running / done / waiting). Everything works except interrupts, which can't be reflected without resorting to a timed staleness heuristic that false-fires on long-but-live turns (big builds, long replies, extended thinking).
#:      closed_at: "2026-06-30T10:15:00Z"
#:      comments: 2
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/71652/comments"
#:      created_at: "2026-06-26T18:31:43Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/71652/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/71652"
#:      id: 4754273798
#:      labels:
#:      - id: 8190926413
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/duplicate"
#:        name: "duplicate"
#:        color: "888888"
#:        description: "This issue or pull request already exists"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/71652/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABG2B2Bg"
#:      number: 71652
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/71652/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "closed"
#:      state_reason: "duplicate"
#:      title: "Feature request: hook event on user interrupt (Esc)"
#:      updated_at: "2026-06-30T10:15:01Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/71652"
#:      user:
#:        login: "shaunak"
#:        id: 48568483
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjQ4NTY4NDgz"
#:        avatar_url: "https://avatars.githubusercontent.com/u/48568483?v=4"
#:        html_url: "https://github.com/shaunak"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/shaunak"
#:        events_url: "https://api.github.com/users/shaunak/events{/privacy}"
#:        following_url: "https://api.github.com/users/shaunak/following{/other_user}"
#:        followers_url: "https://api.github.com/users/shaunak/followers"
#:        gists_url: "https://api.github.com/users/shaunak/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/shaunak/orgs"
#:        received_events_url: "https://api.github.com/users/shaunak/received_events"
#:        repos_url: "https://api.github.com/users/shaunak/repos"
#:        starred_url: "https://api.github.com/users/shaunak/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/shaunak/subscriptions"
#:    - assignee:
#:        login: "bogini"
#:        id: 605424
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjYwNTQyNA=="
#:        avatar_url: "https://avatars.githubusercontent.com/u/605424?v=4"
#:        html_url: "https://github.com/bogini"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/bogini"
#:        events_url: "https://api.github.com/users/bogini/events{/privacy}"
#:        following_url: "https://api.github.com/users/bogini/following{/other_user}"
#:        followers_url: "https://api.github.com/users/bogini/followers"
#:        gists_url: "https://api.github.com/users/bogini/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/bogini/orgs"
#:        received_events_url: "https://api.github.com/users/bogini/received_events"
#:        repos_url: "https://api.github.com/users/bogini/repos"
#:        starred_url: "https://api.github.com/users/bogini/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/bogini/subscriptions"
#:      assignees:
#:      - login: "bogini"
#:        id: 605424
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjYwNTQyNA=="
#:        avatar_url: "https://avatars.githubusercontent.com/u/605424?v=4"
#:        html_url: "https://github.com/bogini"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/bogini"
#:        events_url: "https://api.github.com/users/bogini/events{/privacy}"
#:        following_url: "https://api.github.com/users/bogini/following{/other_user}"
#:        followers_url: "https://api.github.com/users/bogini/followers"
#:        gists_url: "https://api.github.com/users/bogini/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/bogini/orgs"
#:        received_events_url: "https://api.github.com/users/bogini/received_events"
#:        repos_url: "https://api.github.com/users/bogini/repos"
#:        starred_url: "https://api.github.com/users/bogini/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/bogini/subscriptions"
#:      author_association: "NONE"
#:      body: |
#:        ### Preflight Checklist
#:        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
#:        - [x] This is a single bug report (please file separate reports for different bugs)
#:        - [x] I am using the latest version of Claude Code
#:        ### What's Wrong?
#:        When a skill using `/loop` internally calls `ScheduleWakeup`, pressing Ctrl+C to interrupt the session does NOT cancel the pending wakeup timer. The Claude Code daemon auto-respawns the session and continues executing loop iterations indefinitely — even with the terminal closed and the Mac lid shut. This resulted in ~$300 of unintended API usage over a single weekend with no way to detect or stop it from the CLI.
#:        ### What Should Happen?
#:        Ctrl+C should cancel any pending ScheduleWakeup timers associated with the current session. The loop should stop when the user interrupts. There should also be a CLI command to list and kill active background jobs from any terminal.
#:        ### Error Messages/Logs
#:        ```shell
#:        ~/.claude/jobs/e3b9dac0/state.json after Ctrl+C:
#:          {"state": "working", "detail": "CDP triage complete (0 new bugs); next cycle 10:01", "tempo": "active", "inFlight": {"tasks": 2, "queued": 0, "kinds": ["session_cron"]}}
#:        ~/.claude/daemon/roster.json showed worker respawned with new PID after killing it:
#:          {"attempt": 2, "pid": 35341, ...} — supervisor restarted it on exit 143 (SIGTERM)
#:          CronList → empty. TaskList → empty. No CLI way to detect the running loop.
#:        ```
#:        ### Steps to Reproduce
#:          ---
#:          Steps to Reproduce
#:          1. Run any skill via `/loop` with a short interval (e.g. `/loop 5m /my-skill`)
#:          2. Press Ctrl+C to stop — you expect the loop to be cancelled
#:          3. Close the terminal window
#:          4. Check ~/.claude/daemon/roster.json — the worker entry persists
#:          5. The daemon continues running loop iterations unattended indefinitely
#:        ### Claude Model
#:        Sonnet (default)
#:        ### Is this a regression?
#:        I don't know
#:        ### Last Working Version
#:        _No response_
#:        ### Claude Code Version
#:        2.1.160
#:        ### Platform
#:        Anthropic API
#:        ### Operating System
#:        macOS
#:        ### Terminal/Shell
#:        Terminal.app (macOS)
#:        ### Additional Information
#:        Enterprise plan ($3,000/month limit). Loop ran ~864 iterations over a weekend (5 min interval × ~72 hours). The only workaround was manually editing ~/.claude/daemon/roster.json to set "workers": {} and ~/.claude/jobs/<id>/state.json to "state": "completed". This is a financial safety issue — users have no reasonable way to know the loop is still running after Ctrl+C.
#:      comments: 5
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/64744/comments"
#:      created_at: "2026-06-02T13:23:26Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/64744/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/64744"
#:      id: 4571796193
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8223769435
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:macos"
#:        name: "platform:macos"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on macOS"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izDWw"
#:      - id: 8566350905
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:cost"
#:        name: "area:cost"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_pgkOQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/64744/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABEIAS4Q"
#:      number: 64744
#:      reactions:
#:        total_count: 1
#:        "+1": 1
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/64744/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[BUG] ScheduleWakeup persists after Ctrl+C, daemon auto-respawns loop unattended causing unbounded token spend"
#:      updated_at: "2026-06-03T16:54:41Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/64744"
#:      user:
#:        login: "NicolasGuglielmo"
#:        id: 69164008
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjY5MTY0MDA4"
#:        avatar_url: "https://avatars.githubusercontent.com/u/69164008?v=4"
#:        html_url: "https://github.com/NicolasGuglielmo"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/NicolasGuglielmo"
#:        events_url: "https://api.github.com/users/NicolasGuglielmo/events{/privacy}"
#:        following_url: "https://api.github.com/users/NicolasGuglielmo/following{/other_user}"
#:        followers_url: "https://api.github.com/users/NicolasGuglielmo/followers"
#:        gists_url: "https://api.github.com/users/NicolasGuglielmo/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/NicolasGuglielmo/orgs"
#:        received_events_url: "https://api.github.com/users/NicolasGuglielmo/received_events"
#:        repos_url: "https://api.github.com/users/NicolasGuglielmo/repos"
#:        starred_url: "https://api.github.com/users/NicolasGuglielmo/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/NicolasGuglielmo/subscriptions"
#:    - author_association: "NONE"
#:      body: |
#:        ## Problem
#:        With extended thinking enabled, cancelling a batch of **parallel tool calls** (one tool errors → the rest are auto-cancelled) corrupts the `thinking` blocks of the in-flight assistant message. The next API request is rejected with a 400, and because the corrupted message stays in history, **every subsequent turn re-sends it and re-fails** — the session is permanently wedged.
#:        > **Verbatim error** (recurred on every turn afterward — including a plain text question and a slash-command invocation):
#:        > ```
#:        > API Error: 400 messages.1.content.54: `thinking` or `redacted_thinking` blocks in the latest assistant message cannot be modified. These blocks must remain as they were in the original response.
#:        > ```
#:        > **Verbatim transcript leading up to it** — a batch of parallel `Bash` tool calls, the first of which errored, cascading cancellations onto the rest:
#:        > ```
#:        > ⏺ Bash(safari-browser screenshot /tmp/plaud_search.png --url plaud 2>/dev/null && echo "saved")
#:        >   ⎿  Error: Exit code 1
#:        > ...
#:        > ⏺ Bash(safari-browser js "1+1" --url plaud ...)
#:        >   ⎿  Cancelled: parallel tool call Bash(safari-browser screenshot /tmp/plaud_sea…) errored
#:        > ⏺ Bash(echo "PLAIN_TEXT_TEST_NO_BROWSER")
#:        >   ⎿  Cancelled: parallel tool call Bash(safari-browser screenshot /tmp/plaud_sea…) errored
#:        >   ⎿  API Error: 400 messages.1.content.54: `thinking` or `redacted_thinking` blocks in the latest assistant message cannot be modified. These blocks must remain as they were in the original response.
#:        > ```
#:        > — Source: Claude Code session transcript, 2026-05-29
#:        ## Type
#:        bug
#:        ## Steps to reproduce (best-effort)
#:        1. Extended thinking enabled (default in this session)
#:        2. Model: `claude-opus-4-7` (1M context)
#:        3. In one assistant turn, issue a batch of parallel tool calls (here: ~10 `Bash` calls)
#:        4. Arrange for the **first** call in the batch to error (here `safari-browser screenshot … 2>/dev/null` → Exit 1), so Claude Code auto-cancels the remaining parallel calls (`Cancelled: parallel tool call … errored`)
#:        5. On the next API request, observe the 400
#:        ## Expected
#:        Cancelling parallel tool calls MUST preserve the assistant message's `thinking` / `redacted_thinking` blocks byte-for-byte (the API requires they stay identical when thinking + tool use is active), or discard the incomplete assistant turn cleanly. Either way the conversation should remain continuable.
#:        ## Actual
#:        A `thinking` block (here `messages.1.content.54`) is modified during the cancellation/reconstruction. The API rejects the request with 400. The bad message persists in conversation history, so **every subsequent turn replays the same 400** — the user cannot even ask "what happened" or invoke a slash command. Only `/clear` or a new session recovers.
#:        ## Impact
#:        Total, unrecoverable session loss, triggered by a routine event (one tool in a parallel batch failing). High severity for any workflow that issues parallel tool calls with extended thinking on.
#:        ## Environment
#:        - Claude Code 2.1.153
#:        - Model: `claude-opus-4-7` (1M context), extended thinking enabled
#:        - Platform: macOS (Darwin 25.5.0)
#:      comments: 8
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/63192/comments"
#:      created_at: "2026-05-28T16:22:48Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/63192/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/63192"
#:      id: 4542012674
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8201030515
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/has%20repro"
#:        name: "has repro"
#:        color: "ededed"
#:        description: "Has detailed reproduction steps"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHLcw"
#:      - id: 8223769435
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:macos"
#:        name: "platform:macos"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on macOS"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izDWw"
#:      - id: 8572664217
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tools"
#:        name: "area:tools"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vh5mQ"
#:      - id: 8572719646
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:core"
#:        name: "area:core"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vlSHg"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/63192/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABDrmdAg"
#:      number: 63192
#:      reactions:
#:        total_count: 28
#:        "+1": 28
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/63192/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "Cancelling a parallel tool-call batch corrupts thinking blocks -> 400 \"thinking blocks cannot be modified\" permanently wedges the session"
#:      updated_at: "2026-05-31T19:24:52Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/63192"
#:      user:
#:        login: "kiki830621"
#:        id: 17489494
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjE3NDg5NDk0"
#:        avatar_url: "https://avatars.githubusercontent.com/u/17489494?v=4"
#:        html_url: "https://github.com/kiki830621"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/kiki830621"
#:        events_url: "https://api.github.com/users/kiki830621/events{/privacy}"
#:        following_url: "https://api.github.com/users/kiki830621/following{/other_user}"
#:        followers_url: "https://api.github.com/users/kiki830621/followers"
#:        gists_url: "https://api.github.com/users/kiki830621/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/kiki830621/orgs"
#:        received_events_url: "https://api.github.com/users/kiki830621/received_events"
#:        repos_url: "https://api.github.com/users/kiki830621/repos"
#:        starred_url: "https://api.github.com/users/kiki830621/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/kiki830621/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ### Preflight Checklist
#:        - [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet
#:        - [x] This is a single feature request (not multiple features)
#:        ### Problem Statement
#:         Today the claude-code prompt interaction is creating friction for the user. @bcherny Boris, could you please have a brief look, so this does not get auto-closed?
#:          Permission prompts encode "No" as a numbered option. In 3-option prompts, `2` means "Yes, always". In 2-option prompts, `2` means "No". Users build muscle memory for `2 = Yes, always` and then silently reject
#:          actions when the prompt happens to be binary. The cost of a wrong keystroke is asymmetric — a missed approval is interruptible; an accidental rejection often destroys in-flight work.
#:          Prior issues (#40500, #46781, #47094, #47201) framed this as a *consistency* problem and proposed making numbered options stable across prompt shapes. All four were closed by the stale bot with no maintainer
#:          reply. **IMPORTANT: this issue is different — "no" should be using "n" or "0" as the key — never numbers >1.**
#:        ### Proposed Solution
#:        1. **"No" is never a number > 1.** Always the literal letter `n` (or `0` if a digit is preferred), in every prompt shape.
#:        2. **`y` is always a synonym for `1` (Yes)**, in every prompt shape — symmetric with `n`.
#:        3. **Binary prompts** have options `1`/`y` (Yes) and `n`/`0` (No). There is no `2`.
#:        4. **Stray-digit fallback**: in a binary prompt, if the user types `2`, `3`, … (a digit not corresponding to any listed option), treat it as `1`/`y` (Yes). Rationale: such a digit is overwhelmingly
#:          carried-over muscle memory for "Yes, always". Mapping it to No is the failure mode this issue is filed about; mapping it to Yes fails safe — the action proceeds and is interruptible, instead of being silently
#:          discarded.
#:        5. The literal `n` always means No, in every prompt shape. Esc / Ctrl-C still cancel.
#:        ### Alternative Solutions
#:        - Just being more careful with keystrokes — doesn't scale across hundreds of prompts per long session.
#:        - `--dangerously-skip-permissions` to bypass prompts entirely — too broad; removes the safety net the prompts exist for.
#:        - Arrow-key selection of options (proposed in #35091, closed `not_planned`) — would also fix the problem but is a larger UI change. The `n`/`y` letter-binding proposed here is strictly additive and
#:          backward-compatible with the current numeric input.
#:        ### Priority
#:        High - Significant impact on productivity
#:        ### Feature Category
#:        Interactive mode (TUI)
#:        ### Use Case Example
#:        Concrete scenario:
#:          1. Claude is mid-task, about to edit a config file. A binary permission prompt appears: "1) Yes  2) No".
#:          2. I type `2` reflexively — muscle memory from the more common 3-option prompt where `2` = "Yes, always".
#:          3. The action is silently denied. Claude logs "tool denied" and continues to the next step *without* applying the change.
#:          4. Several minutes later I notice the file wasn't edited. I either re-prompt Claude (losing context) or have to manually undo whatever it did downstream.
#:          With the proposed change: typing `2` in a binary prompt would map to `1` (Yes), the action proceeds, and the work isn't silently lost. To actually decline, I'd type `n` — which has no muscle-memory collision
#:          with the 3-option prompt's `Yes, always`.
#:        ### Additional Context
#:        ### Why this matters
#:          Asymmetric failure cost: silent rejection of an intended action is much harder to recover from than a re-confirmation of an unintended action. Long agent runs cascade — a rejection mid-task often loses several
#:           minutes of work plus context. Multiple bug reports already exist (#40500, #46781, #47094, #47201) showing this is a consistent footgun, not an edge case.
#:        ### Prior context
#:          - #40500 (consistency request, stale-bot-closed Apr 29)
#:          - #47094, #46781, #47201 (duplicates of #40500, all bot-closed)
#:        ### cc — past reporters of related issues
#:          @RichGibson @jertsdev @bradfeld @dgunderson — you each filed one of the related issues (#40500, #46781, #47094, #47201) which got auto-closed without engagement. Please have a look and 👍 if this framing
#:          resonates, so it doesn't share the same fate.
#:      comments: 4
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/55854/comments"
#:      created_at: "2026-05-03T19:29:03Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/55854/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/55854"
#:      id: 4372746631
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      - id: 9118989405
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/user-experience"
#:        name: "user-experience"
#:        color: "ededed"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACH4i8XQ"
#:      - id: 10096052281
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:permissions"
#:        name: "area:permissions"
#:        color: "ededed"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACWcWIOQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/55854/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABBKLRhw"
#:      number: 55854
#:      reactions:
#:        total_count: 10
#:        "+1": 7
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 3
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/55854/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[FEATURE] Permission prompts: never use a number for \"No\"; treat stray digits as \"Yes\" fallback"
#:      updated_at: "2026-06-29T13:14:53Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/55854"
#:      user:
#:        login: "tilo"
#:        id: 22553
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjIyNTUz"
#:        avatar_url: "https://avatars.githubusercontent.com/u/22553?v=4"
#:        html_url: "https://github.com/tilo"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/tilo"
#:        events_url: "https://api.github.com/users/tilo/events{/privacy}"
#:        following_url: "https://api.github.com/users/tilo/following{/other_user}"
#:        followers_url: "https://api.github.com/users/tilo/followers"
#:        gists_url: "https://api.github.com/users/tilo/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/tilo/orgs"
#:        received_events_url: "https://api.github.com/users/tilo/received_events"
#:        repos_url: "https://api.github.com/users/tilo/repos"
#:        starred_url: "https://api.github.com/users/tilo/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/tilo/subscriptions"
#:    - author_association: "NONE"
#:      body: "### Preflight Checklist\n\n- [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet\n- [x] This is a single feature request (not multiple features)\n\n### Problem Statement\n\nThis is close to a dup (not truly, as as you'll see below I have a twist on the following request-  https://github.com/anthropics/claude-code/issues/535 - where it is stated in the ticket that there is a fix but it seems to have ended up in the great bit bucket in the sky.\n\nBasically the goal is to have a feature similar to cursor's - namely to be able to queue up messages for claude to work on rather than replace the current workflow. As.it stands I am trying to push for claude code in my work but when I get messages like 'hey - my workflow was interrupted what's going on?.. and 'I just wasted HOW many hours because claude did WHAT when I told it to do this additional task?'. this does not make my job easy.\n\nBasically the idea is this. When you type an additional instruction for claude while it is working on something currently, instead of going ahead and killing its current thought process as currently exists, it asks you a clarifying question:\n\nDo you want to (s)top the current workflow or (q)ueue up a new task?\n\nAnd it replaces or queues as necessary. That way it gets rid of the ambiguity in the prompt. As it stands, it is not clear here whether you are actually wanting to replace the current task or add to it. Adding this prompt gets rid of the ambiguity adds and extra feature to claude code, and makes the user's intent clear.\n\n@8enmann - you said that you had a fix for this in issue 535. I must be missing something here, but I can't imagine that this would be too much effort and would love to provide a patch. \n\n\n### Proposed Solution\n\nBasically the idea is this. When you type an additional instruction for claude while it is working on something currently, instead of going ahead and killing its current thought process as currently exists, it asks you a clarifying question:\n\nDo you want to (s)top the current workflow or (q)ueue up a new task?\n\nAnd it replaces or queues as necessary. That way it gets rid of the ambiguity in the prompt. As it stands, it is not clear here whether you are actually wanting to replace the current task or add to it. Adding this prompt gets rid of the ambiguity adds and extra feature to claude code, and makes the user's intent clear.\n\n@8enmann - you said that you had a fix for this in issue 535. I must be missing something here, but I can't imagine that this would be too much effort and would love to provide a patch. \n\n### Alternative Solutions\n\nYou can emulate this by telling claude to write to a done file when completed in your request in one window, then spawning up another claude instance and adding something like:\n\nfirst, wait until this done file exists. Then, do the following.\n\nBut this is inadequate. the second claude instance doesn't contain the first claude instance's context for one. Second its iffy because it requires the user to think ahead of time to put this statement in, and it isn't bulletproof because it isn't a hardcoded instruction.\n\n### Priority\n\nHigh - Significant impact on productivity\n\n### Feature Category\n\nCLI commands and flags\n\n### Use Case Example\n\nExample scenario:\n\nI'm telling claude to monitor a production run for anomalous occurances.\nI get about an hour in, and remember that I forgot to tell claude to do something afterwards (clean up the filesystem, etc)\n\nI'm now stuck either with restarting the prompt from scratch, or somehow jerry-rigging a method of spawning claude to do the extra work by watching for the end of a process in the process table, or what not.\n\n### Additional Context\n\nPlease fix this promptly. Its been sitting here for about a year now, with several people commenting on and saying how great this feature would be to have. Again, maybe I'm missing something but I'm having a hard time thinking of how this could be a difficult fix to make."
#:      comments: 18
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/34835/comments"
#:      created_at: "2026-03-16T05:08:01Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/34835/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/34835"
#:      id: 4080257731
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/34835/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87zM8rD"
#:      number: 34835
#:      reactions:
#:        total_count: 20
#:        "+1": 20
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/34835/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[FEATURE] The ability to queue up messages.via asking the user for further info on user input."
#:      updated_at: "2026-06-03T18:00:36Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/34835"
#:      user:
#:        login: "horos22reg"
#:        id: 43970670
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjQzOTcwNjcw"
#:        avatar_url: "https://avatars.githubusercontent.com/u/43970670?v=4"
#:        html_url: "https://github.com/horos22reg"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/horos22reg"
#:        events_url: "https://api.github.com/users/horos22reg/events{/privacy}"
#:        following_url: "https://api.github.com/users/horos22reg/following{/other_user}"
#:        followers_url: "https://api.github.com/users/horos22reg/followers"
#:        gists_url: "https://api.github.com/users/horos22reg/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/horos22reg/orgs"
#:        received_events_url: "https://api.github.com/users/horos22reg/received_events"
#:        repos_url: "https://api.github.com/users/horos22reg/repos"
#:        starred_url: "https://api.github.com/users/horos22reg/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/horos22reg/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ## Summary
#:        When using `--remote-control` to drive Claude Code from the claude.ai/code web UI, permission approval prompts for some MCP tool calls **never render** in the web UI. They appear only in the local TUI (screen session on the host), where they block the session until manually answered via SSH.
#:        This is broader than a single MCP server — observed against both a bundled `claude.ai` connector and a custom user-built MCP server.
#:        ## Environment
#:        - Claude Code version: **2.1.143**
#:        - Launch: `claude --remote-control <session-name>` (interactive, wrapped in `screen -DmS claude-<name>` under a systemd user unit)
#:        - Host: Linux x86_64, Ubuntu 24.04, kernel 6.17.0
#:        - Remote UI: claude.ai/code/ (web)
#:        - No `--dangerously-skip-permissions`, no `permissionMode: bypassPermissions`
#:        ## Steps to Reproduce
#:        1. Configure two MCP servers — one bundled `claude.ai` connector (e.g. a file-server), one custom user MCP. Do NOT pre-allowlist their write/delete tools in `settings.local.json`.
#:        2. Launch `claude --remote-control session-1` on the host (inside a `screen` session under a systemd user unit).
#:        3. Connect to the same session from claude.ai/code's web UI.
#:        4. From the web UI, ask Claude to call either:
#:           - `claude.ai` MCP `write_file` to an allowed path, **or**
#:           - Custom MCP `memory_write` / `memory_delete` (or any write/destructive tool)
#:        5. Observe the web UI vs. what's actually on the host.
#:        ## Expected
#:        A permission approval prompt renders in the claude.ai/code web UI. User clicks Allow / Allow Once / Deny.
#:        ## Actual
#:        - The web UI shows the tool call as in flight, then reports `Tool result missing due to internal error`; the conversation appears hung. No indication that approval is needed.
#:        - The actual prompt is sitting in the local TUI in the screen session:
#:          ```
#:          Do you want to proceed?
#:            1. Yes
#:            2. Yes, don't ask again for X in this session
#:            3. No
#:          ```
#:        - The session stays blocked until someone SSHes in, `screen -r`s, and answers manually. Prompts have sat 50+ minutes this way.
#:        ## Affected vs. Unaffected Tools
#:        In the same session, with the same MCP servers, prompts for some tools surface fine while others do not:
#:        | Tool | Web-UI prompt surfaces? |
#:        | --- | --- |
#:        | MCP `run_command` (read-only) | ✅ |
#:        | MCP `read_file` | ✅ |
#:        | `claude.ai` MCP `write_file` | ❌ |
#:        | Custom MCP `memory_write`, `memory_delete` | ❌ |
#:        Pattern looks category-based (write/destructive ops?), not a per-server bug. Not Agent Teams / `--teammate-mode` — this is a vanilla single-session `--remote-control`.
#:        ## Possibly Related
#:        - #48549 (closed) — same symptom in Agent Teams via Remote Control. The teammate-specific fix may not cover single-session `--remote-control`.
#:        - #46392 (open) — tmux teammate mode, similar symptom but different code path.
#:        ## Workaround
#:        Pre-allowlist the specific MCP tool names in `.claude/settings.local.json`:
#:        \`\`\`json
#:        {
#:          "permissions": {
#:            "allow": [
#:              "mcp__<server-id>__<tool-name>"
#:            ]
#:          }
#:        }
#:        \`\`\`
#:        This eliminates the prompt entirely, sidestepping the bug — but requires knowing each tool's MCP name in advance and defeats per-call control.
#:        ## Notes
#:        - I do not believe this is server-side. The MCP spec puts permission UI entirely on the client; server tools shouldn't need to do anything special.
#:        - First noticed locally on 2026-05-14 with `write_file`; reconfirmed 2026-05-19 with a custom memory-management MCP server. Bug is stable across multiple sessions and Claude Code versions in the 2.1.x line.
#:      comments: 19
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/60385/comments"
#:      created_at: "2026-05-19T00:20:30Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/60385/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/60385"
#:      id: 4473474850
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8201030515
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/has%20repro"
#:        name: "has repro"
#:        color: "ededed"
#:        description: "Has detailed reproduction steps"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHLcw"
#:      - id: 8223769668
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:linux"
#:        name: "platform:linux"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on Linux"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izERA"
#:      - id: 8582683357
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:mcp"
#:        name: "area:mcp"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_5Fa3Q"
#:      - id: 10002526951
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:web"
#:        name: "platform:web"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on the web"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACVDJy5w"
#:      - id: 10096052281
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:permissions"
#:        name: "area:permissions"
#:        color: "ededed"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACWcWIOQ"
#:      - id: 10191083653
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/stale"
#:        name: "stale"
#:        color: "888888"
#:        description: "Issue is inactive"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACX2-YhQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/60385/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABCqPPIg"
#:      number: 60385
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/60385/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[BUG] Remote Control: MCP permission prompts for non-read tools never surface in claude.ai/code web UI"
#:      updated_at: "2026-07-12T10:31:35Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/60385"
#:      user:
#:        login: "paul43210"
#:        id: 31411417
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjMxNDExNDE3"
#:        avatar_url: "https://avatars.githubusercontent.com/u/31411417?v=4"
#:        html_url: "https://github.com/paul43210"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/paul43210"
#:        events_url: "https://api.github.com/users/paul43210/events{/privacy}"
#:        following_url: "https://api.github.com/users/paul43210/following{/other_user}"
#:        followers_url: "https://api.github.com/users/paul43210/followers"
#:        gists_url: "https://api.github.com/users/paul43210/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/paul43210/orgs"
#:        received_events_url: "https://api.github.com/users/paul43210/received_events"
#:        repos_url: "https://api.github.com/users/paul43210/repos"
#:        starred_url: "https://api.github.com/users/paul43210/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/paul43210/subscriptions"
#:    - author_association: "NONE"
#:      body: |
#:        ### Problem
#:        When Claude is working, follow-up messages typed in the CLI get queued. There's currently no way to remove a queued message without pressing `Esc` — which interrupts the *in-progress* task, not the queue. That's the opposite of what you want when you only meant to retract something you typed ahead.
#:        The desktop apps already solve this: each queued message has an `x` to remove it. The CLI has no parity.
#:        ### Proposal
#:        A `/cancel` slash command. Because the command itself enters the queue, the natural semantics are **"drain everything queued ahead of me, leave the running task alone"** — which is also the most common intent (clear the queue, keep the work).
#:        Optional finer-grained variants:
#:        - `/cancel-last` — drop only the most recently queued message
#:        - `/cancel <n>` — drop the nth queued item
#:        A slash command sidesteps the keybinding collision that likely caused `Esc` to be overloaded onto "interrupt" in the first place — it has no ambient meaning to clobber.
#:        ### Prior art / related
#:        Same request, all closed without resolution (dupe / not-planned / inactivity):
#:        - #55737
#:        - #6147
#:        - #36817
#:        ---
#:        *Filed from the CLI by Claude Code (https://claude.com/claude-code), on behalf of (and with) @gesslar — who would quite like to cancel a queued message someday.*
#:      closed_at: "2026-06-26T22:34:02Z"
#:      comments: 3
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/62349/comments"
#:      created_at: "2026-05-26T01:55:50Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/62349/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/62349"
#:      id: 4520689088
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      - id: 10063580143
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:cli"
#:        name: "area:cli"
#:        color: "ededed"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACV9YL7w"
#:      - id: 10191083653
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/stale"
#:        name: "stale"
#:        color: "888888"
#:        description: "Issue is inactive"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACX2-YhQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/62349/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABDXQ9wA"
#:      number: 62349
#:      reactions:
#:        total_count: 2
#:        "+1": 1
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 1
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/62349/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "closed"
#:      state_reason: "not_planned"
#:      title: "Add /cancel to clear queued messages without interrupting the running task"
#:      updated_at: "2026-06-26T22:34:02Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/62349"
#:      user:
#:        login: "gesslar"
#:        id: 1266935
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjEyNjY5MzU="
#:        avatar_url: "https://avatars.githubusercontent.com/u/1266935?v=4"
#:        html_url: "https://github.com/gesslar"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/gesslar"
#:        events_url: "https://api.github.com/users/gesslar/events{/privacy}"
#:        following_url: "https://api.github.com/users/gesslar/following{/other_user}"
#:        followers_url: "https://api.github.com/users/gesslar/followers"
#:        gists_url: "https://api.github.com/users/gesslar/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/gesslar/orgs"
#:        received_events_url: "https://api.github.com/users/gesslar/received_events"
#:        repos_url: "https://api.github.com/users/gesslar/repos"
#:        starred_url: "https://api.github.com/users/gesslar/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/gesslar/subscriptions"
#:    - active_lock_reason: "resolved"
#:      author_association: "NONE"
#:      body: |-
#:        **Background**
#:        Esc already works as a hard stop — cancel generation immediately. This is the panic button and it's useful. This request is for the softer case.
#:        **Problem**
#:        When Claude Code is mid-generation you sometimes realise you forgot to include a piece of context — a constraint, a correction, a clarification — that would change the output. Your options right now are:
#:        - Let it finish and correct after (wasted generation, possible rework)
#:        - Hit Esc and restart with the amended prompt (loses the momentum, you retype everything)
#:        Neither is great when what you actually wanted was to add one thing without stopping.
#:        **Proposed behaviour: Shift+Esc**
#:        1. Pause generation and buffer any output so far
#:        2. Open a small inline input field: append to prompt…
#:        3. User types the additional context and confirms
#:        4. Generation resumes with the original prompt + appended context merged, continuing from where it paused
#:        **Why this is different from just sending a follow-up message**
#:        A follow-up message queues as a new top-level job — it arrives after the committed output, as a correction rather than a steering input. Shift+Esc would fold the addition into the current generation context before the model has finished committing to a direction.
#:        **Use cases**
#:        - Keep going but also consider X
#:        - Spotted a typo or ambiguity in the original prompt mid-stream
#:        - Want to add a constraint without restarting (btw, don't touch the tests)
#:        **Hard stop (Esc) stays unchanged** — this doesn't replace it.
#:      closed_at: "2026-05-22T10:09:26Z"
#:      comments: 3
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/60303/comments"
#:      created_at: "2026-05-18T17:59:42Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/60303/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/60303"
#:      id: 4471373678
#:      labels:
#:      - id: 8190926413
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/duplicate"
#:        name: "duplicate"
#:        color: "888888"
#:        description: "This issue or pull request already exists"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/60303/labels{/name}"
#:      locked: true
#:      node_id: "I_kwDON91aY88AAAABCoO_bg"
#:      number: 60303
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/60303/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "closed"
#:      state_reason: "duplicate"
#:      title: "Shift+Esc: soft interrupt to append context to an in-flight response"
#:      updated_at: "2026-07-11T14:32:44Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/60303"
#:      user:
#:        login: "dougbristor"
#:        id: 36883395
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjM2ODgzMzk1"
#:        avatar_url: "https://avatars.githubusercontent.com/u/36883395?v=4"
#:        html_url: "https://github.com/dougbristor"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/dougbristor"
#:        events_url: "https://api.github.com/users/dougbristor/events{/privacy}"
#:        following_url: "https://api.github.com/users/dougbristor/following{/other_user}"
#:        followers_url: "https://api.github.com/users/dougbristor/followers"
#:        gists_url: "https://api.github.com/users/dougbristor/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/dougbristor/orgs"
#:        received_events_url: "https://api.github.com/users/dougbristor/received_events"
#:        repos_url: "https://api.github.com/users/dougbristor/repos"
#:        starred_url: "https://api.github.com/users/dougbristor/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/dougbristor/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ### Preflight Checklist
#:        - [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet
#:        - [x] This is a single feature request (not multiple features)
#:        ### Problem Statement
#:        I have a very different iterative planning workflow that I use, so I am never going to actually select "plan mode" myself. I always have to cancel it when Claude decides "it's time to plan!".
#:        ### Proposed Solution
#:        It would be great to have an official setting to either: deny the planning tool completely (no option in tab switcher either), or a very least deny the automatic tool use to enter planning mode (I am fine if this triggers a hiccup in Claude's output, I can always redirect after).
#:        ### Alternative Solutions
#:        I am testing out adding `EnterPlanMode` to my Deny list, but that seems pretty hacky (what if the name of that tool changes, or there is some other way to trigger it?).
#:        Edit: This *does* appear to work for now, but as mentioned, an official true/false setting for planning mode as a whole or automatic planning at a minimum would be way better.
#:        ### Priority
#:        Medium - Would be very helpful
#:        ### Feature Category
#:        Interactive mode (TUI)
#:        ### Use Case Example
#:        _No response_
#:        ### Additional Context
#:        _No response_
#:      comments: 9
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/23384/comments"
#:      created_at: "2026-02-05T16:04:04Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/23384/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/23384"
#:      id: 3902336783
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      - id: 8572719646
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:core"
#:        name: "area:core"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vlSHg"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/23384/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87omO8P"
#:      number: 23384
#:      reactions:
#:        total_count: 33
#:        "+1": 33
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/23384/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[FEATURE] Add option to disable automatic Plan tool use"
#:      updated_at: "2026-03-09T15:13:36Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/23384"
#:      user:
#:        login: "mellowfish"
#:        id: 1732187
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjE3MzIxODc="
#:        avatar_url: "https://avatars.githubusercontent.com/u/1732187?v=4"
#:        html_url: "https://github.com/mellowfish"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/mellowfish"
#:        events_url: "https://api.github.com/users/mellowfish/events{/privacy}"
#:        following_url: "https://api.github.com/users/mellowfish/following{/other_user}"
#:        followers_url: "https://api.github.com/users/mellowfish/followers"
#:        gists_url: "https://api.github.com/users/mellowfish/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/mellowfish/orgs"
#:        received_events_url: "https://api.github.com/users/mellowfish/received_events"
#:        repos_url: "https://api.github.com/users/mellowfish/repos"
#:        starred_url: "https://api.github.com/users/mellowfish/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/mellowfish/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ### Preflight Checklist
#:        - [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet
#:        - [x] This is a single feature request (not multiple features)
#:        ### Problem Statement
#:        Today, interactive mode has two input states during an active turn:
#:        1. **Type into the box** — the message is queued and submits when the current turn finishes. There is no way to interrupt.
#:        2. **Press `Esc` first** — cancels the running turn immediately, then you can type a redirect.
#:        There is no flow where the user can: (a) notice the agent going wrong mid-turn, (b) **compose a careful redirect while the agent keeps working**, and (c) **at send time** choose whether to interrupt-and-redirect or queue.
#:        The current `Esc`-first path forces commitment to interrupting *before* the user has finished thinking through the redirect. That creates time pressure to compose quickly, or it pauses the agent prematurely just to draft a message that might end up being \"actually, keep going.\" There is no clean \"I see something going wrong but I'm not yet sure I want to interrupt\" state.
#:        This matters most for non-trivial redirects — multi-line corrections, careful wording, paste-in of a stack trace or doc snippet — where composition time is real and the user wants to think while the agent works.
#:        ### Proposed Solution
#:        Allow composing input during an active turn **without auto-interrupting** (this is already the default). At send time, expose two distinct send actions:
#:        - `Enter` → **queue** (current default — submit after the turn ends)
#:        - `Shift+Enter` (or some other modifier) → **interrupt + submit as redirect** (equivalent to `Esc` then submit, but as a single keystroke once the message is already written)
#:        A persistent UI cue near the input box, e.g.: *\"Enter to queue · Shift+Enter to break in.\"*
#:        This is purely additive: `Enter` still queues, `Esc` still hard-interrupts. The new modifier just collapses the two-step \"Esc, then type, then Enter\" into a one-step \"type, then Shift+Enter\" — and crucially lets the user defer the interrupt/queue decision until they've actually finished writing.
#:        ### Alternative Solutions
#:        - **Esc-first workflow (current):** works, but forces the interrupt decision before composition. Most redirect messages are short, but the ones that matter most (careful corrections, pasted context) are the ones where composition time is real.
#:        - **Type, queue, then `/cancel` queued message and re-submit as interrupt:** indirect, error-prone, and doesn't exist as a single flow today.
#:        - **Codex (OpenAI's CLI) supports exactly this pattern**: typing during agent work doesn't interrupt, and at send time the user picks between interrupt vs queue via a modifier key. It feels natural in practice — you don't lose work, and you don't have to decide before you're ready.
#:        ### Priority
#:        Medium - Would be very helpful
#:        ### Feature Category
#:        Interactive mode (TUI)
#:        ### Use Case Example
#:        1. Claude is partway through a multi-file refactor. I notice it's about to rename a symbol I want to keep.
#:        2. I start typing a redirect: *\"hold off on renaming `FooBar` — I want to discuss the naming first, here's why…\"* — a few sentences with context.
#:        3. Claude keeps working while I type. Halfway through composing, I realize it's already past the rename and is now doing something fine — so I hit `Enter` to queue.
#:        4. Alternative: I finish composing and see it's still on the problematic step, so I hit `Shift+Enter` to interrupt and submit as redirect.
#:        The decision is made *after* I've finished thinking, not before I've started.
#:        ### Additional Context
#:        Adjacent issues (related but distinct, mentioned for context):
#:        - #50246 — *Message queue mode* — proposed Shift+Enter to queue vs Enter to interrupt, back when the default was interrupt-on-Enter. The default has since flipped to queue-on-Enter, so the polarity of the ask is now inverted: it's interrupt that needs a send-time modifier, not queue.
#:        - #34835 — *Clarifying prompt for stop vs queue* — same underlying ambiguity, different UX (prompt vs send-time modifier).
#:        - #30492 — *Real-time steering: priority message channel* — broader scope (mid-tool-call injection).
#:        - #59684 (closed as dup) — *Inject instructions into running task* — also broader (additive injection rather than redirect).
#:        This request is narrower: just give send-time choice between the two existing behaviors (queue / interrupt), so users don't have to pre-commit before composing.
#:      closed_at: "2026-07-01T11:13:13Z"
#:      comments: 3
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/62856/comments"
#:      created_at: "2026-05-27T15:39:07Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/62856/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/62856"
#:      id: 4533650940
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/62856/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY88AAAABDjoF_A"
#:      number: 62856
#:      reactions:
#:        total_count: 2
#:        "+1": 2
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/62856/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "closed"
#:      state_reason: "not_planned"
#:      title: "[FEATURE] Compose-while-working with send-time choice of interrupt vs queue (Codex-style)"
#:      updated_at: "2026-07-01T12:34:01Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/62856"
#:      user:
#:        login: "ericsampson"
#:        id: 1183853
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjExODM4NTM="
#:        avatar_url: "https://avatars.githubusercontent.com/u/1183853?v=4"
#:        html_url: "https://github.com/ericsampson"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/ericsampson"
#:        events_url: "https://api.github.com/users/ericsampson/events{/privacy}"
#:        following_url: "https://api.github.com/users/ericsampson/following{/other_user}"
#:        followers_url: "https://api.github.com/users/ericsampson/followers"
#:        gists_url: "https://api.github.com/users/ericsampson/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/ericsampson/orgs"
#:        received_events_url: "https://api.github.com/users/ericsampson/received_events"
#:        repos_url: "https://api.github.com/users/ericsampson/repos"
#:        starred_url: "https://api.github.com/users/ericsampson/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/ericsampson/subscriptions"
#:    - active_lock_reason: "resolved"
#:      author_association: "NONE"
#:      body: |-
#:        ## Summary
#:        The ESC key is overloaded in the Chat context: pressing ESC clears the input when typing, and pressing ESC (or double-ESC) while a task is running interrupts it. Users who routinely double-tap ESC to clear input frequently interrupt running tasks by accident — losing in-flight
#:        work and burning tokens on the restart.
#:        ## Why this is hard to fix today
#:        Looking at the documented keybinding actions, the only ESC-bound action in `Chat` context is `chat:cancel` (single `escape`). There is no separate `task:interrupt` or equivalent action that's bound to ESC while a task is foregrounded — so users can't remap the "escape-while-running" behavior independently of the "escape-while-typing" behavior. `app:interrupt` (`ctrl+c`) is
#:        explicitly hardcoded.
#:        The only workaround via `~/.claude/keybindings.json` is to unbind ESC from `chat:cancel` entirely, which also kills ESC-to-clear-input — a net loss for users who do legitimately use ESC to clear.
#:        ## Requested change
#:        Either of these would solve it:
#:        1. **Configurable interrupt threshold.** A setting like
#:           `keybindings.escapeInterruptPresses` (default `2`, accepts `1`/`2`/`3`) so users can require triple-ESC to interrupt while still keeping single-ESC for clear-input.
#:        2. **Separate rebindable action.** Split the running-task interrupt into its own action (e.g. `task:interruptOnEscape`) that lives in the `Task` context and can be unbound or rebound to a chord like `escape escape escape` independently of `chat:cancel`.
#:        Option 2 is more flexible and consistent with how other actions are exposed; option 1 is a simpler ship.
#:        ## Real-world impact
#:        Trained muscle memory from earlier versions of Claude Code (where work chunks were shorter and double-ESC-to-clear was safe) doesn't translate cleanly now that tasks routinely run for minutes at a time. The accidental-interrupt cost is high (lost context, restart cost, lost
#:        tokens) and the UX recovery cost is awkward (the user knows they did it, but the muscle memory is sticky).
#:      closed_at: "2026-05-04T09:49:31Z"
#:      comments: 3
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/55204/comments"
#:      created_at: "2026-05-01T03:36:26Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/55204/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/55204"
#:      id: 4362654785
#:      labels:
#:      - id: 8190926413
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/duplicate"
#:        name: "duplicate"
#:        color: "888888"
#:        description: "This issue or pull request already exists"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/55204/labels{/name}"
#:      locked: true
#:      node_id: "I_kwDON91aY88AAAABBAjUQQ"
#:      number: 55204
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/55204/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "closed"
#:      state_reason: "duplicate"
#:      title: "Make escape-during-task interrupt threshold configurable (or expose as a separate rebindable action)"
#:      updated_at: "2026-06-23T15:08:32Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/55204"
#:      user:
#:        login: "brandonjp"
#:        id: 199599
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjE5OTU5OQ=="
#:        avatar_url: "https://avatars.githubusercontent.com/u/199599?v=4"
#:        html_url: "https://github.com/brandonjp"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/brandonjp"
#:        events_url: "https://api.github.com/users/brandonjp/events{/privacy}"
#:        following_url: "https://api.github.com/users/brandonjp/following{/other_user}"
#:        followers_url: "https://api.github.com/users/brandonjp/followers"
#:        gists_url: "https://api.github.com/users/brandonjp/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/brandonjp/orgs"
#:        received_events_url: "https://api.github.com/users/brandonjp/received_events"
#:        repos_url: "https://api.github.com/users/brandonjp/repos"
#:        starred_url: "https://api.github.com/users/brandonjp/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/brandonjp/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ### Preflight Checklist
#:        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
#:        - [x] This is a single bug report (please file separate reports for different bugs)
#:        - [x] I am using the latest version of Claude Code
#:        ### What's Wrong?
#:        Claude Code hangs permanently when the API streaming response stalls mid-delivery. The process stays alive (epoll_wait in kernel) but makes no progress. The UI shows a spinner ("Accomplishing…",
#:          "Ruminating…", etc.) indefinitely. No error is surfaced. The session cannot recover — the only fix is kill -9.
#:          Two distinct hang patterns observed:
#:          Pattern 1 — Mid-turn stream freeze: The API response starts streaming (thinking block received, "thought for 3s" renders), then the stream silently stops. No more tokens arrive. The JSONL session
#:          log shows no new entries after the last progress update. The process is stuck waiting for bytes that never come.
#:          Pattern 2 — Tool result delivery stall: A Bash tool executes and completes (e.g., curl --max-time 5), but the tool result is never delivered back to the conversation state. The last tool_use ID in
#:          the JSONL has no matching tool_result. The UI shows the spinner as if the tool is still running.
#:          Both patterns correlate with background agent (Task with run_in_background: true) notifications arriving between or during turns.
#:        ### What Should Happen?
#:        1. Claude Code should have a read timeout on the SSE/streaming HTTP response. If no data arrives within N seconds (e.g., 60–120s), it should abort the request and surface a retryable error to the
#:          user.
#:          2. Tool result delivery should have a timeout. If a tool completes but the result isn't consumed within N seconds, the session should error rather than hang.
#:          3. The UI should detect "no progress for N seconds" and offer the user an escape (e.g., "Session appears stuck. Press Enter to retry or Esc to cancel").
#:        ### Error Messages/Logs
#:        ```shell
#:        Session 1: Mid-turn stream freeze (fff87be2)
#:          JSONL timeline:
#:          21:43:28 UTC  Turn 1 ends (system/turn_duration logged)
#:          21:43:28      Background agent notification injected as user message
#:                        → Triggers Turn 2
#:          21:44:44      Turn 2 ends (system/turn_duration logged, stop hooks run)
#:                        Background agent notification for a DIFFERENT agent arrives
#:                        → Triggers Turn 3
#:                        API call starts, "thought for 3s" renders in UI
#:                        *** No more JSONL entries. Stream froze. ***
#:                        UI stuck on "Accomplishing… (thought for 3s)" for 10+ minutes
#:          Process state during hang:
#:          $ ps -p 843072 -o pid,state,wchan
#:              PID S WCHAN
#:           843072 S do_epoll_wait
#:          $ kill -TERM 843072   # no effect
#:          $ kill -KILL 843072   # required to terminate
#:          Session 2: Tool result delivery stall (06d50a72)
#:          JSONL shows:
#:          Last tool_use ID:    toolu_vrtx_01JqgVUPZLrMyudEQaAQhx1x  (Bash: curl)
#:          Last tool_result ID: toolu_vrtx_01LJscxroh9R7tsDQUL6AhuF  (different, earlier tool)
#:          → Mismatch: the curl's result was never delivered
#:          The curl command had --max-time 5 so it completed, but the result never made it back to the conversation. UI showed "Ruminating…" indefinitely.
#:        ```
#:        ### Steps to Reproduce
#:        Difficult to reproduce deterministically — it appears to be a race condition or network-level issue. But the following pattern triggers it frequently:
#:          1. Start Claude Code on a remote Linux server via SSH + tmux
#:          2. Give it a complex task that spawns multiple background agents (Task with run_in_background: true)
#:          3. Wait for background agents to complete and deliver notifications
#:          4. When a notification arrives right as a turn is ending or between turns, the next API call has a high chance of hanging
#:          Environment factors that may contribute:
#:          - Remote server (high-latency network path to API)
#:          - tmux (terminal multiplexing)
#:          - Multiple concurrent Claude Code sessions in different tmux panes
#:          - Heavy hook infrastructure (6 hook events, though all complete in <120ms per telemetry)
#:        ### Claude Model
#:        Opus
#:        ### Is this a regression?
#:        Yes, this worked in a previous version
#:        ### Last Working Version
#:        2.1.40
#:        ### Claude Code Version
#:        2.1.42
#:        ### Platform
#:        Google Vertex AI
#:        ### Operating System
#:        macOS
#:        ### Terminal/Shell
#:        iTerm2
#:        ### Additional Information
#:        Suggested fix
#:          Add a read timeout to the HTTP streaming client. Pseudocode:
#:          // In the SSE/streaming response handler:
#:          const STREAM_READ_TIMEOUT_MS = 120_000; // 2 minutes
#:          let lastDataTime = Date.now();
#:          stream.on('data', (chunk) => {
#:            lastDataTime = Date.now();
#:            // ... process chunk
#:          });
#:          const watchdog = setInterval(() => {
#:            if (Date.now() - lastDataTime > STREAM_READ_TIMEOUT_MS) {
#:              stream.destroy(new Error('API stream read timeout'));
#:              clearInterval(watchdog);
#:            }
#:          }, 10_000);
#:          Workaround
#:          External watchdog daemon that monitors JSONL session files and kills processes with no writes for >5 minutes. Available at user's request.
#:      comments: 34
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/25979/comments"
#:      created_at: "2026-02-15T22:11:02Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/25979/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/25979"
#:      id: 3945035341
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8213275784
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/api:vertex"
#:        name: "api:vertex"
#:        color: "93A5FF"
#:        description: "Issue relates to GCP Vertex"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6YykiA"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/25979/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87rJHZN"
#:      number: 25979
#:      reactions:
#:        total_count: 16
#:        "+1": 16
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/25979/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[BUG] Claude Code hangs indefinitely when API streaming connection stalls (no read timeout)"
#:      updated_at: "2026-06-16T10:04:05Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/25979"
#:      user:
#:        login: "esuleman"
#:        id: 70169323
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjcwMTY5MzIz"
#:        avatar_url: "https://avatars.githubusercontent.com/u/70169323?v=4"
#:        html_url: "https://github.com/esuleman"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/esuleman"
#:        events_url: "https://api.github.com/users/esuleman/events{/privacy}"
#:        following_url: "https://api.github.com/users/esuleman/following{/other_user}"
#:        followers_url: "https://api.github.com/users/esuleman/followers"
#:        gists_url: "https://api.github.com/users/esuleman/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/esuleman/orgs"
#:        received_events_url: "https://api.github.com/users/esuleman/received_events"
#:        repos_url: "https://api.github.com/users/esuleman/repos"
#:        starred_url: "https://api.github.com/users/esuleman/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/esuleman/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ### Preflight Checklist
#:        - [x] I have searched [existing issues](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20state%3Aopen%20label%3Abug) and this hasn't been reported yet
#:        - [x] This is a single bug report (please file separate reports for different bugs)
#:        - [x] I am using the latest version of Claude Code
#:        ### What's Wrong?
#:        The Claude Code session became completely unresponsive with repeated API 400 errors. Every subsequent user input triggered the same error, making recovery impossible.
#:        **Symptom**: Session entered an error loop where the conversation history reached a corrupted state:
#:        1. A previous assistant response contained thinking blocks
#:        2. The system attempted to modify or replay that message's thinking blocks
#:        3. The API correctly rejected this (thinking blocks are immutable once sent)
#:        4. The error persisted because the corrupted state remained in conversation history
#:        **Error Pattern**: The error occurred at `messages.71.content.8` (71st message, 8th content block), indicating:
#:        - Deep conversation history (71 messages)
#:        - Specific content block contained thinking blocks that couldn't be modified
#:        - Every subsequent user input triggered the same error
#:        **Contributing Factors**:
#:        - **Long session**: Nearly 15k lines of transcript suggests extended conversation
#:        - **Complex workflow**: Multiple tool calls, file reads, git operations
#:        - **Repeated questions**: Same question asked multiple times may have confused state
#:        - **Message editing attempt**: The system may have tried to edit/retry previous responses
#:        **Impact**:
#:        - **Session Status**: Completely unrecoverable
#:        - **User Action**: Had to save transcript and start new session
#:        - **Work Status**: Context lost mid-conversation about a technical topic
#:        - **Data Loss**: Any uncommitted mental state/context
#:        **What Was Lost**:
#:        - Active conversation context and unresolved questions
#:        - Current working state and recent discussion topics
#:        - Mental model of the problem being discussed
#:        ### What Should Happen?
#:        **Expected Behavior**:
#:        - Claude Code should handle long conversations gracefully without entering unrecoverable error states
#:        - If thinking blocks cannot be modified (as per API constraint), the system should detect this condition before submitting the request
#:        - When errors occur, the system should offer graceful degradation options
#:        **Ideal Recovery Flow**:
#:        1. Detect when conversation state is approaching problematic conditions
#:        2. Offer to summarize current state
#:        3. Save context to file automatically
#:        4. Start fresh session with context restored
#:        5. Or provide mechanism to "rewind" conversation history to before corrupted state
#:        **Preventive Measures That Should Exist**:
#:        - Conversation state validation to detect when thinking blocks are being modified
#:        - Message history pruning or summarization for very long conversations
#:        - Threshold warnings when conversation history reaches risky size (>10k lines)
#:        - Checkpoint mechanism to enable rollback to stable state
#:        ### Error Messages/Logs
#:        ```shell
#:        ### Primary Error Message
#:        API Error: 400 {"type":"error","error":{"type":"invalid_request_error","message":"messages.71.content.8: `thinking` or `redacted_thinking` blocks in the latest assistant message cannot be modified. These blocks must remain as they were in the original response."},"request_id":"req_011CUQZ6ttKtJbU8JQRYyizo"}
#:        ### Error Repetition Pattern
#:        Line 14957: User asks question → API Error 400
#:        Line 14961: User asks same question → API Error 400 (same request)
#:        Line 14965: User asks same question → API Error 400 (same request)
#:        Line 14969: User references the error itself → API Error 400
#:        Line 14974: User asks to update memory files → API Error 400
#:        ### Technical Details
#:        **API Endpoint**: Anthropic Claude API (claude.code integration)
#:        **Error Type**: `invalid_request_error`
#:        **Error Message Pattern**:
#:        messages.{N}.content.{M}: `thinking` or `redacted_thinking` blocks in the latest
#:        assistant message cannot be modified. These blocks must remain as they were in
#:        the original response.
#:        Where:
#:        - `{N}` = Message index in conversation history (71 in this case)
#:        - `{M}` = Content block index within that message (8 in this case)
#:        **Request IDs**:
#:        - req_011CUQZ6ttKtJbU8JQRYyizo
#:        - req_011CUQZ8XPZgRdbaXt8GJrxy
#:        - req_011CUQZ9SAKKbLfL1g7ieqU8
#:        - req_011CUQZAR1eH1AotrvNzGhx8
#:        **Transcript Available**: Yes (763.4KB, 14,976 lines)
#:        ```
#:        ### Steps to Reproduce
#:        ### Context Leading to Error
#:        1. User and assistant engaged in extended session about a technical topic (~14,976 lines)
#:        2. Conversation involved multiple rounds of investigation and explanation
#:        3. Assistant provided detailed technical explanations
#:        4. User asked follow-up question requesting clarification
#:        5. Same question was asked 3 times in succession (possibly due to user not seeing response)
#:        6. **ERROR STATE TRIGGERED**
#:        ### Reproduction Steps
#:        1. Start Claude Code session
#:        2. Engage in extended conversation with multiple tool calls, file reads, and complex workflows
#:        3. Reach approximately 71 messages deep in conversation history
#:        4. Have assistant generate response with thinking blocks
#:        5. Ask follow-up question that may trigger message retry or state replay
#:        6. Observe API Error 400 with thinking block modification error
#:        7. Attempt to continue session → error repeats for all subsequent inputs
#:        ### Conditions That May Trigger
#:        - Very long sessions (>10k lines of transcript)
#:        - Conversation history at ~71 messages or deeper
#:        - Assistant responses containing thinking blocks
#:        - Rapid repeated questions (asked 3 times in succession in this case)
#:        - Complex multi-turn workflows with many tool calls
#:        ### Warning Signs
#:        - Session approaching 10k+ lines of transcript
#:        - Repeated API errors occurring
#:        - Same question triggering errors multiple times
#:        - Any API 400 error mentioning thinking blocks
#:        ### Recovery Actions Taken
#:        1. User saved full transcript to local file
#:        2. User started new Claude Code session
#:        3. User asked new session to review the transcript
#:        4. User requested this bug report be filed
#:        ### Claude Model
#:        Sonnet (default)
#:        ### Is this a regression?
#:        I don't know
#:        ### Last Working Version
#:        _No response_
#:        ### Claude Code Version
#:        2.0.25 (Claude Code)
#:        ### Platform
#:        Anthropic API
#:        ### Operating System
#:        macOS
#:        ### Terminal/Shell
#:        IntelliJ IDEA terminal
#:        ### Additional Information
#:        _No response_
#:      comments: 100
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/10199/comments"
#:      created_at: "2025-10-23T18:22:59Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/10199/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/10199"
#:      id: 3546008252
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8201030515
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/has%20repro"
#:        name: "has repro"
#:        color: "ededed"
#:        description: "Has detailed reproduction steps"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHLcw"
#:      - id: 8223769435
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:macos"
#:        name: "platform:macos"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on macOS"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izDWw"
#:      - id: 8572719646
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:core"
#:        name: "area:core"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vlSHg"
#:      - id: 8572738602
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:api"
#:        name: "area:api"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vmcKg"
#:      - id: 9492025677
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/oncall"
#:        name: "oncall"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAACNcTRTQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/10199/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87TW8q8"
#:      number: 10199
#:      reactions:
#:        total_count: 64
#:        "+1": 64
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/10199/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "[BUG] API Error 400 - Thinking Block Modification Error"
#:      updated_at: "2026-06-26T18:01:18Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/10199"
#:      user:
#:        login: "WamBamBoozle"
#:        id: 1843644
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjE4NDM2NDQ="
#:        avatar_url: "https://avatars.githubusercontent.com/u/1843644?v=4"
#:        html_url: "https://github.com/WamBamBoozle"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/WamBamBoozle"
#:        events_url: "https://api.github.com/users/WamBamBoozle/events{/privacy}"
#:        following_url: "https://api.github.com/users/WamBamBoozle/following{/other_user}"
#:        followers_url: "https://api.github.com/users/WamBamBoozle/followers"
#:        gists_url: "https://api.github.com/users/WamBamBoozle/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/WamBamBoozle/orgs"
#:        received_events_url: "https://api.github.com/users/WamBamBoozle/received_events"
#:        repos_url: "https://api.github.com/users/WamBamBoozle/repos"
#:        starred_url: "https://api.github.com/users/WamBamBoozle/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/WamBamBoozle/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ## Summary
#:        When pasting multi-line text into the Claude Code prompt input, the UI collapses it showing `[Pasted text #N +X lines]`. While this is a nice space-saving feature, it makes it difficult to review pasted content before submitting.
#:        ## Problem
#:        As a user, I often:
#:        1. Paste text from another source (error messages, code snippets, requirements)
#:        2. Want to review what I pasted before sending
#:        3. Cannot see the full content because it's collapsed
#:        This is especially problematic when:
#:        - Recovering text after a crash (took a photo of screen, OCR'd it, want to verify accuracy)
#:        - Pasting code that may have formatting issues
#:        - Combining multiple pasted sections into one prompt
#:        ## Proposed Solution
#:        Add a configuration option to control paste collapse behavior. Options could include:
#:        ### Option A: Global Setting
#:        ```json
#:        // settings.json or .claude/settings.json
#:        {
#:          "input": {
#:            "collapsePastedText": false  // default: true
#:          }
#:        }
#:        ```
#:        ### Option B: Keyboard Toggle
#:        - `Ctrl+Shift+E` (or similar) to expand/collapse pasted text in the input field
#:        - Or a visual indicator that's clickable to expand
#:        ### Option C: Threshold Setting
#:        ```json
#:        {
#:          "input": {
#:            "collapsePastedTextThreshold": 20  // only collapse if > 20 lines, 0 = never collapse
#:          }
#:        }
#:        ```
#:        ## Workarounds Attempted
#:        - Creating a `/expand` slash command - this only works *after* submission, not in the input field
#:        - Hooks - no `paste` event hook available
#:        ## Environment
#:        - Claude Code CLI
#:        - macOS / Linux terminals
#:        - Various terminal emulators (iTerm2, Terminal.app, Warp, etc.)
#:        ## Additional Context
#:        This would significantly improve the UX for users who frequently paste content and want to verify it before sending. The current behavior optimizes for visual cleanliness but sacrifices reviewability.
#:        ---
#:        Happy to contribute a PR if someone can point me to where this UI behavior is implemented!
#:      comments: 45
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/23134/comments"
#:      created_at: "2026-02-04T16:51:31Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/23134/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/23134"
#:      id: 3897467238
#:      labels:
#:      - id: 8190926415
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/enhancement"
#:        name: "enhancement"
#:        color: "A2D9CE"
#:        description: "New feature or request"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTw"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/23134/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87oTqFm"
#:      number: 23134
#:      reactions:
#:        total_count: 136
#:        "+1": 119
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 17
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/23134/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "Feature Request: Option to disable paste text collapse in input field"
#:      updated_at: "2026-06-29T13:04:07Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/23134"
#:      user:
#:        login: "wesleysmyth"
#:        id: 6476956
#:        user_view_type: "public"
#:        node_id: "MDQ6VXNlcjY0NzY5NTY="
#:        avatar_url: "https://avatars.githubusercontent.com/u/6476956?v=4"
#:        html_url: "https://github.com/wesleysmyth"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/wesleysmyth"
#:        events_url: "https://api.github.com/users/wesleysmyth/events{/privacy}"
#:        following_url: "https://api.github.com/users/wesleysmyth/following{/other_user}"
#:        followers_url: "https://api.github.com/users/wesleysmyth/followers"
#:        gists_url: "https://api.github.com/users/wesleysmyth/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/wesleysmyth/orgs"
#:        received_events_url: "https://api.github.com/users/wesleysmyth/received_events"
#:        repos_url: "https://api.github.com/users/wesleysmyth/repos"
#:        starred_url: "https://api.github.com/users/wesleysmyth/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/wesleysmyth/subscriptions"
#:    - author_association: "NONE"
#:      body: |-
#:        ## Description
#:        Claude Code occasionally enters a frozen state where:
#:        1. The animated spinner becomes static (stops animating)
#:        2. Input field stops accepting any input
#:        3. Escape does not cancel the current operation
#:        4. The process ignores SIGTERM (remains in `Rl+` state)
#:        5. Only SIGKILL terminates it
#:        ## Reproduction
#:        Observed twice in tmux sessions. Both times the session was running normally, then froze mid-operation.
#:        ### Case 1: Context exhaustion
#:        - Context at 5% remaining
#:        - Status showed: `Ideating… (34m 23s · ↓ 0 tokens · thinking)`
#:        - 0 tokens received for 34+ minutes
#:        - Likely: API rejected the request due to context overflow, but client didn't handle the error/timeout
#:        ### Case 2: Normal operation
#:        - Context was normal (not exhausted)
#:        - Status showed: `Pondering… (4m 4s · ↓ 7.7k tokens · thought for 2s)`
#:        - 7.7k tokens were received, then froze
#:        - The session was managing other tmux sessions via `tmux send-keys` commands
#:        ## Environment
#:        - Platform: Linux 6.8.0-90-generic
#:        - Claude Code running inside tmux sessions
#:        - Multiple concurrent Claude Code sessions active
#:        ## Expected behavior
#:        - Escape should cancel the current operation
#:        - SIGTERM should gracefully terminate the process
#:        - If the API connection drops, the client should timeout and show an error
#:        ## Actual behavior
#:        - Complete freeze: no animation, no input, no response to signals
#:        - Only `kill -9` works
#:        ## Process state when frozen
#:        ```
#:        bash(PID)---claude(PID)-+-{claude}(thread1)
#:                                 |-{claude}(thread2)
#:                                 |-{claude}(thread3)
#:                                 |-{claude}(thread4)
#:                                 |-{claude}(thread5)
#:                                 |-{claude}(thread6)
#:                                 |-{claude}(thread7)
#:                                 |-{claude}(thread8)
#:                                 |-{claude}(thread9)
#:                                 `-{claude}(thread10)
#:        ```
#:        Process state: `Rl+` (Running, multi-threaded, foreground)
#:        ## Hypothesis
#:        Possible deadlock in the Node.js event loop — likely waiting on an SSE stream from the API with a broken/stalled connection, while the terminal I/O thread is also blocked.
#:      comments: 6
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/20572/comments"
#:      created_at: "2026-01-24T12:07:13Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/20572/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/20572"
#:      id: 3851026122
#:      labels:
#:      - id: 8190926409
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/bug"
#:        name: "bug"
#:        color: "FF6B80"
#:        description: "Something isn't working"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeSQ"
#:      - id: 8201030515
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/has%20repro"
#:        name: "has repro"
#:        color: "ededed"
#:        description: "Has detailed reproduction steps"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6NHLcw"
#:      - id: 8223769668
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/platform:linux"
#:        name: "platform:linux"
#:        color: "93A5FF"
#:        description: "Issue specifically occurs on Linux"
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB6izERA"
#:      - id: 8566369501
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:tui"
#:        name: "area:tui"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_phs3Q"
#:      - id: 8572719646
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/area:core"
#:        name: "area:core"
#:        color: "ededed"
#:        description: ""
#:        default: false
#:        node_id: "LA_kwDON91aY88AAAAB_vlSHg"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/20572/labels{/name}"
#:      locked: false
#:      node_id: "I_kwDON91aY87lif7K"
#:      number: 20572
#:      reactions:
#:        total_count: 16
#:        "+1": 16
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/20572/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "open"
#:      title: "Claude Code freezes: static spinner, unresponsive input, ignores SIGTERM"
#:      updated_at: "2026-04-20T19:21:37Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/20572"
#:      user:
#:        login: "real-bk2k"
#:        id: 252900461
#:        user_view_type: "public"
#:        node_id: "U_kgDODxL0bQ"
#:        avatar_url: "https://avatars.githubusercontent.com/u/252900461?v=4"
#:        html_url: "https://github.com/real-bk2k"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/real-bk2k"
#:        events_url: "https://api.github.com/users/real-bk2k/events{/privacy}"
#:        following_url: "https://api.github.com/users/real-bk2k/following{/other_user}"
#:        followers_url: "https://api.github.com/users/real-bk2k/followers"
#:        gists_url: "https://api.github.com/users/real-bk2k/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/real-bk2k/orgs"
#:        received_events_url: "https://api.github.com/users/real-bk2k/received_events"
#:        repos_url: "https://api.github.com/users/real-bk2k/repos"
#:        starred_url: "https://api.github.com/users/real-bk2k/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/real-bk2k/subscriptions"
#:    - active_lock_reason: "resolved"
#:      author_association: "NONE"
#:      body: |-
#:        ### Preflight Checklist
#:        - [x] I have searched [existing requests](https://github.com/anthropics/claude-code/issues?q=is%3Aissue%20label%3Aenhancement) and this feature hasn't been requested yet
#:        - [x] This is a single feature request (not multiple features)
#:        ### Problem Statement
#:        When Claude Code is executing a long-running task (multi-file refactor, large build, multi-step EA development), I frequently realize mid-execution that I want to ADD scope to the current work — not replace it, not wait for it to finish.
#:        Current options are all inadequate for this:
#:        1. **Press Enter** — message gets queued. It only runs AFTER the current task fully completes, as a separate turn. The new instruction is not merged into the ongoing work. If the running task finishes a refactor without my added context, I now need a second turn to add it — wasting time and tokens.
#:        2. **Press Esc** — interrupts the task destructively. In-progress tool calls are aborted, partial work may be lost or left in an inconsistent state. For long tasks this is too costly just to add a small instruction.
#:        3. **/btw** — read-only. Cannot edit files, cannot influence the running task. Only useful for asking questions.
#:        The gap: there is no non-destructive way to say "ALSO do X while you're doing the current thing." The old pre-queue behavior approximated this (Enter would steer mid-task), but the current default queue behavior removed it entirely.
#:        This impacts real workflows. Example from my own daily use (MQL5 Expert Advisor development): Claude Code is refactoring OnTick logic in a Grid EA across multiple files. Mid-execution I realize I also want input validation added to OnInit. Today I must either (a) wait for the full refactor to finish then ask in a new turn, or (b) Esc and break the in-progress work. Neither is acceptable for tight iteration loops.
#:        The underlying problem: Claude Code currently treats user input during execution as either "interrupt now" or "wait until next turn" — there is no middle ground for "add this to what you're already doing."
#:        ### Proposed Solution
#:        Add a dedicated command — for example `/inject`, `/add`, or `/also` — that delivers a user message into Claude's active context at the next tool-call boundary, without ending or restarting the current turn.
#:        Behavior:
#:        - User types `/inject also add input validation to OnInit` while Claude is mid-task
#:        - The message is captured and tagged as [additional scope] or [steering]
#:        - At the next safe boundary (e.g. before the next PreToolUse hook fires, between tool calls), the message is injected into Claude's context
#:        - Claude continues the current task with the merged scope, not as a separate turn
#:        - A visible indicator confirms the injection was received (e.g. "1 instruction injected" near the input box)
#:        Optional refinements:
#:        - Differentiate `/inject` (merge into current work) from existing queue behavior (run after current work)
#:        - Allow viewing/canceling a pending injection before it lands at the next boundary
#:        - Keyboard shortcut alternative (e.g. Shift+Enter for inject vs Enter for queue)
#:        This is technically feasible — Claude Code already runs PreToolUse hooks between tool calls, which is the exact interstitial window where injection could happen. Issue #30492 outlines the same mechanism in more detail.
#:        ### Alternative Solutions
#:        1. Multi-file refactors where additional scope becomes obvious mid-execution
#:        2. Long EA/indicator development tasks (my use case — MQL5 Expert Advisor work) where I want to add a constraint or extra function without restarting the refactor
#:        3. Build/test pipelines where I want to add a follow-up check while the current step runs
#:        4. Any long autonomous workflow where waiting wastes time and Esc wastes work
#:        Similar tools (Cursor, Codex) support analogous patterns. Claude Code GitHub has multiple related but distinct requests:
#:        - #30492 — Real-time steering: priority message channel
#:        - #25845 — Prompt queue with steer controls
#:        - #50246 — Message queue mode toggle
#:        - #34835 — Clarifying prompt for queue vs interrupt
#:        This request differs: it's specifically for ADDITIVE injection (merge scope), not replacement, not interrupt, not deferred queue.
#:        ### Priority
#:        Critical - Blocking my work
#:        ### Feature Category
#:        CLI commands and flags
#:        ### Use Case Example
#:        _No response_
#:        ### Additional Context
#:        _No response_
#:      closed_at: "2026-05-20T10:12:55Z"
#:      comments: 3
#:      comments_url: "https://api.github.com/repos/anthropics/claude-code/issues/59684/comments"
#:      created_at: "2026-05-16T11:45:51Z"
#:      events_url: "https://api.github.com/repos/anthropics/claude-code/issues/59684/events"
#:      field_values: []
#:      html_url: "https://github.com/anthropics/claude-code/issues/59684"
#:      id: 4459758478
#:      labels:
#:      - id: 8190926413
#:        url: "https://api.github.com/repos/anthropics/claude-code/labels/duplicate"
#:        name: "duplicate"
#:        color: "888888"
#:        description: "This issue or pull request already exists"
#:        default: true
#:        node_id: "LA_kwDON91aY88AAAAB6DeeTQ"
#:      labels_url: "https://api.github.com/repos/anthropics/claude-code/issues/59684/labels{/name}"
#:      locked: true
#:      node_id: "I_kwDON91aY88AAAABCdKDjg"
#:      number: 59684
#:      reactions:
#:        total_count: 0
#:        "+1": 0
#:        "-1": 0
#:        laugh: 0
#:        confused: 0
#:        heart: 0
#:        hooray: 0
#:        rocket: 0
#:        eyes: 0
#:        url: "https://api.github.com/repos/anthropics/claude-code/issues/59684/reactions"
#:      repository_url: "https://api.github.com/repos/anthropics/claude-code"
#:      state: "closed"
#:      state_reason: "duplicate"
#:      title: "[FEATURE] Inject instructions into running task without interrupting (mid-execution context injection)"
#:      updated_at: "2026-07-09T15:11:44Z"
#:      url: "https://api.github.com/repos/anthropics/claude-code/issues/59684"
#:      user:
#:        login: "jawadtrader22"
#:        id: 208966863
#:        user_view_type: "public"
#:        node_id: "U_kgDODHSUzw"
#:        avatar_url: "https://avatars.githubusercontent.com/u/208966863?v=4"
#:        html_url: "https://github.com/jawadtrader22"
#:        gravatar_id: ""
#:        type: "User"
#:        site_admin: false
#:        url: "https://api.github.com/users/jawadtrader22"
#:        events_url: "https://api.github.com/users/jawadtrader22/events{/privacy}"
#:        following_url: "https://api.github.com/users/jawadtrader22/following{/other_user}"
#:        followers_url: "https://api.github.com/users/jawadtrader22/followers"
#:        gists_url: "https://api.github.com/users/jawadtrader22/gists{/gist_id}"
#:        organizations_url: "https://api.github.com/users/jawadtrader22/orgs"
#:        received_events_url: "https://api.github.com/users/jawadtrader22/received_events"
#:        repos_url: "https://api.github.com/users/jawadtrader22/repos"
#:        starred_url: "https://api.github.com/users/jawadtrader22/starred{/owner}{/repo}"
#:        subscriptions_url: "https://api.github.com/users/jawadtrader22/subscriptions"
/allow 57603a22-4c99-47cf-a31f-441896eef191
Ja – wenn du in Claude Code während der Verarbeitung **Esc** drückst, passiert tatsächlich etwas auf Netzwerkebene, und zwar konkret:

## Was passiert technisch

1. **Der laufende API-Stream wird abgebrochen (aborted).** Claude Code nutzt intern einen `AbortController`. Ein Esc-Druck (bzw. Ctrl+C) löst `abort()` auf diesem Controller aus, was den offenen HTTP/SSE-Request an die Anthropic-API hart abbricht. In den Debug-Logs des Projekts sieht man z. B.:
   ```
   [onCancel] ... → API error: Request was aborted.
   ```
   Es wird also nicht "höflich" ein Stop-Signal in den Prompt geschrieben, sondern die Verbindung wird clientseitig gekappt – die Inferenz-Berechnung auf dem Server kann dadurch aber je nach Implementierung trotzdem noch etwas nachlaufen (Server merkt evtl. erst durch den Verbindungsabbruch, dass niemand mehr zuhört).

2. **Kontext bleibt erhalten.** Das bisher generierte Assistant-Ergebnis (Text/Tool-Aufrufe) wird nicht verworfen, sondern im Gesprächsverlauf behalten, und dem Modell wird beim nächsten Turn eine Systemnotiz mitgegeben, z. B.:
   - `[Request interrupted by user]` (wenn während der Textgenerierung unterbrochen)
   - bzw. bei laufenden Tool-Calls eine Meldung à la „Tool use was rejected/interrupted – STOP what you are doing…“

3. **Bekannte Einschränkung/Bug:** Es gibt einen offenen Issue (#26802), der zeigt, dass Esc zwar den **aktuellen** API-Stream zuverlässig abbricht, aber bereits **parallel gestartete Tool-Calls** (z. B. mehrere gleichzeitige Bash-Kommandos) nicht mit-abbricht. Deren Ergebnisse können dann automatisch einen neuen API-Turn auslösen, sodass man Esc mehrfach drücken muss, um wirklich alles zu stoppen.

## Kurz zusammengefasst

- **Ja:** Esc sendet effektiv ein Abbruchsignal – der offene Request/Stream zur Anthropic-API wird via AbortController gekappt.
- Das ist vergleichbar mit dem, was auch im TypeScript-SDK über `abortController.abort()` bzw. im Python-SDK über `interrupt()` passiert.
- Es ist aber kein "sauberes" serverseitiges Cancel-Kommando im Sinne einer expliziten API, sondern ein Verbindungsabbruch – Tool-Ausführungen, die bereits im Gange sind (z. B. laufende Shell-Befehle), laufen aktuell teils trotzdem weiter, bis ihr Ergebnis zurückkommt.
Gibt es ein absichtliches Token, das man während der Verarbeitung schicken kann, um einen Stop zu erzwingen? Oder den Inferenz Loop zu unterbrechen?
#: Control Request:
#:id: "cc3decf2-30bc-4944-9f7b-bfc232434eef"
#:phase: "request"
#:toolName: "web-search-exa"
#:arguments:
#:  query: "Anthropic API stop_sequences streaming abort request cancel inference mid-generation"
/allow cc3decf2-30bc-4944-9f7b-bfc232434eef
#: Control Request:
#:id: "2362b4c3-8ac7-4c57-bba6-de23c53e50bf"
#:phase: "result"
#:toolName: "web-search-exa"
#:result:
#:  content: []
#:  structuredContent:
#:    content: |-
#:      Title: Streaming messages - Claude Platform Docs
#:      URL: https://platform.claude.com/docs/en/build-with-claude/streaming
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      Stream Messages API responses incrementally with server-sent events, including text, tool use, and extended thinking deltas.
#:      ---
#:      Title: Stop reasons and fallback - Claude Platform Docs
#:      URL: https://platform.claude.com/docs/en/build-with-claude/handling-stop-reasons
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      Learn what each stop_reason value means and how to handle truncation, tool use, paused turns, and refusals in your application.
#:      ---
#:      Title: Stop reasons and fallback - Claude Platform Docs
#:      URL: https://platform.claude.com/docs/en/build-with-claude/handling-stop-reasons?_rsc=1ebj0
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      Every Messages API response includes a `stop_reason` field that tells you why Claude stopped generating. Check this field to decide whether to use the response as-is, continue the conversation, retry, or fall back to another model.
#:      ...
#:      | Value | When it occurs | What to do |
#:      | --- | --- | --- |
#:      | `end_turn` | Claude finished its response naturally. | Use the response. |
#:      | `max_tokens` | The response reached your `max_tokens` limit. | Raise `max_tokens` or continue the response. |
#:      | `stop_sequence` | Claude emitted one of your `stop_sequences`. | Read `stop_sequence` to see which one fired. |
#:      | `tool_use` | Claude is calling a tool. | Run the tool and return the result. A server tool call still missing its result block completes in a later response. |
#:      | `pause_turn` | A server-tool loop reached its iteration limit. | Send the assistant content back to continue. |
#:      | `refusal` | Claude declined to respond. | Read `stop_details` and retry on a fallback model. |
#:      | `model_context_window_exceeded` | The response filled the model's context window. | Treat the response as truncated. |
#:      ...
#:      The `stop_reason` field is part of every successful Messages API response. Unlike errors, which indicate failures in processing your request, `stop_reason` tells you why Claude completed its response generation.
#:      ...
#:      returns an empty response (exactly
#:      ...
#:      with no content
#:      ...
#:      stop_reason
#:      ...
#:      continuation prompt in
#:      ...
#:      user message rather
#:      ...
#:      ### max_tokens
#:      ...
#:      Claude stopped because it reached the `max_tokens` limit specified in your request.
#:      ...
#:      ## Incomplete tool use blocks
#:      ...
#:      If Claude'
#:      ...
#:      limit, and
#:      ...
#:      truncated response contains an incomplete tool use block, you'll need to retry the request with a higher `max_
#:      ...
#:      value to get the full tool use.
#:      ...
#:      ### stop_sequence
#:      ...
#:      Claude encountered one of your custom stop sequences.
#:      ...
#:      ```bash
#:      curl https://api.anthropic.com/v1/messages \
#:        --header "x-api-key: $ANTHROPIC_API_KEY" \
#:        --header "anthropic-version: 2023-06-01" \
#:        --header "content-type: application/json" \
#:        --data '{
#:          "model": "claude-opus-4-8",
#:          "max_tokens": 1024,
#:          "stop_sequences": ["END", "STOP"],
#:          "messages": [{"role": "user", "content": "Generate text until you say END"}]
#:        }' | jq '{stop_reason, stop_sequence}'
#:      ...
#:      model="claude-opus-4-8",
#:      ...
#:      024
#:      ...
#:      print(f"
#:      ...
#:      response.stop_
#:      ...
#:      ### tool_use
#:      ...
#:      A `tool_use` response can also contain a `server_tool_use` block whose `id` has no matching result block. That server tool call is not finished, and this response does not carry its result. In the common case, Claude calls a server tool and one of your client tools in the same group of parallel tool calls: the API returns without running the server tool so that you can run the client tools first. There is no other marker for the state; detect it by checking each `server_tool_use` or `mcp_tool_use` block's `id` for a matching result block.
#:      ...
#:      With programmatic tool calling, the same response shape means something different. The client `tool_use` block comes from code that is running in the `code_execution` tool rather than from Claude directly, and its `caller` field names the `code_execution` block that called it. That code has already started: it is paused waiting for your `tool_result` blocks, and sending them resumes the execution instead of starting a deferred tool. The `code_execution` block's own result block arrives once the code finishes, which can take more than one round of tool results. The follow-up user message itself is the same in both cases; with programmatic tool calling, also pass back the `id` from the response's `container` field, as that page shows.
#:      ...
#:      The continuation is a user message of `tool_result` blocks, one for every `tool_use` block in the response (see Handle tool calls), with two extra rules: that message must contain nothing except the `tool_result` blocks, and the request must keep the same `tools` array. A resume request that no longer defines the waiting server tool fails with a 400 whose message ends `but no `web_fetch` tool was provided`. The API attaches your results to the still-open assistant turn, runs the deferred server tool (for paused code execution, resumes it), and continues the turn. For a server tool Claude called directly, the next response's `content` starts with the result block that answers the previous response's `server_tool_use` `id`.
#:      ...
#:      Adding anything after the `tool_result` blocks in that user message, such as text, ends the assistant turn; for a server tool Claude called directly, the request then fails with a 400 `invalid_request_error` that names the unresolved server tool:
#:      ...
#:      fails earlier with
#:      ...
#:      it as a separate
#:      ...
#:      message after the
#:      ...
#:      ### pause_turn
#:      ...
#:      Returned when the server-side sampling loop reaches its iteration limit while executing server tools like web search or web fetch. The default limit is 10 iterations per request.
#:      ...
#:      When this happens, the response may contain a `server_tool_use` block without a corresponding result block. To let Claude finish processing, continue the conversation by sending the response back as-is. A response that leaves a client `tool_use` block waiting on you never has a `stop_reason` of `pause_turn`: when Claude stops to call your tools, `stop_reason` is `tool_use`, and you continue it by sending the client `tool_result` blocks instead of the response itself.
#:      ...
#:      and re-
#:      ...
#:      Your application should handle `pause_turn` in any agent loop that uses server tools. Add the assistant's response to your messages array and make another API request to let Claude continue.
#:      ...
#:      Claude declined to generate a response. On
#:      ...
#:      Fable 5, safety classifiers return this stop reason as a normal HTTP 200 response, not an error
#:      ---
#:      Title: src/anthropic/resources/beta/messages/messages.py at 04b468da · anthropics/anthropic-sdk-python
#:      URL: https://github.com/anthropics/anthropic-sdk-python/blob/04b468da/src/anthropic/resources/beta/messages/messages.py
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      @cached_property
#:          def with_streaming_response(self) -> MessagesWithStreamingResponse:
#:              """
#:              An alternative to `.with_raw_response` that doesn't eagerly read the response body.
#:              For more information, see https://www.github.com/anthropics/anthropic-sdk-python#with_streaming_response
#:              """
#:              return MessagesWithStreamingResponse(self)
#:          @overload
#:          def create(
#:              self,
#:              *,
#:              max_tokens: int,
#:              messages: Iterable[BetaMessageParam],
#:              model: ModelParam,
#:              cache_control: Optional[BetaCacheControlEphemeralParam] | Omit = omit,
#:              container: Optional[message_create_params.Container] | Omit = omit,
#:              context_management: Optional[BetaContextManagementConfigParam] | Omit = omit,
#:              inference_geo: Optional[str] | Omit = omit,
#:              mcp_servers: Iterable[BetaRequestMCPServerURLDefinitionParam] | Omit = omit,
#:              metadata: BetaMetadataParam | Omit = omit,
#:              output_config: BetaOutputConfigParam | Omit = omit,
#:              output_format: Optional[BetaJSONOutputFormatParam] | Omit = omit,
#:              service_tier: Literal["auto", "standard_only"] | Omit = omit,
#:              speed: Optional[Literal["standard", "fast"]] | Omit = omit,
#:              stop_sequences: SequenceNotStr[str] | Omit = omit,
#:              stream: Literal[False] | Omit = omit,
#:              system: Union[str, Iterable[BetaTextBlockParam]] | Omit = omit,
#:              temperature: float | Omit = omit,
#:              thinking: BetaThinkingConfigParam | Omit = omit,
#:              tool_choice: BetaToolChoiceParam | Omit = omit,
#:              tools: Iterable[BetaToolUnionParam] | Omit = omit,
#:              top_k: int | Omit = omit,
#:              top_p: float | Omit = omit,
#:              user_profile_id: Optional[str] | Omit = omit,
#:              betas: List[AnthropicBetaParam] | Omit = omit,
#:              # Use the following arguments if you need to pass additional parameters to the API that aren't available via kwargs.
#:              # The extra values given here take precedence over values defined on the client or passed to this method.
#:              extra_headers: Headers | None = None,
#:              extra_query: Query | None = None,
#:              extra_body: Body | None = None,
#:              timeout: float | httpx.Timeout | None | NotGiven = not_given,
#:          ) -> BetaMessage:
#:              """
#:              Send a structured list of input messages with text and/or image content, and the
#:              model will generate the next message in the conversation.
#:              The Messages API can be used for either single queries or stateless multi-turn
#:              conversations.
#:              Learn more about the Messages API in our
#:              [user guide](https://docs.claude.com/en/docs/initial-setup)
#:              Args:
#:                max_tokens: The maximum number of tokens to generate before stopping.
#:      ...
#:      that our models may stop _before_ reaching
#:      ...
#:      parameter
#:                    only specifies the absolute maximum number of tokens to generate.
#:                    Set to `0` to
#:      ...
#:      .com/en/
#:      ...
#:      /models-
#:      ...
#:      service_tier:
#:      ...
#:      use priority capacity (if
#:      ...
#:      .
#:       Anthropic offers different levels of service for your API requests. See
#:       [service-tiers](https://docs.claude.com/en/api/service-tiers) for details.
#:       speed: The inference speed mode for this request. `"fast"` enables high
#:       output-tokens-per-second inference.
#:       stop_sequences: Custom text sequences that will cause the model to stop generating.
#:       Our models will normally stop when they have naturally completed their turn,
#:       which will result in a response `stop_reason` of `"end_turn"`.
#:       If you want the model to stop generating when it encounters custom strings of
#:       text, you can use the `stop_sequences` parameter. If the model encounters one of
#:       the custom sequences, the response `stop_reason` value will be `"stop_sequence"`
#:       and the response `stop_sequence` value will contain the matched stop sequence.
#:       stream: Whether to incrementally stream the response using server-sent events.
#:       See [streaming](https://docs.claude.com/en/api/messages-streaming) for details.
#:       system: System prompt.
#:      ...
#:      Anthropic offers different levels of service for your API requests. See
#:       [service-tiers](https://docs.claude.com/en/api/service-
#:      ...
#:      ) for details.
#:       speed: The inference speed mode for this request. `"fast"` enables high
#:       output-tokens-per-second inference.
#:       stop_sequences: Custom text sequences that will cause the model to stop generating.
#:       Our models will normally stop when they have naturally completed their turn,
#:       which will result in a response `stop_reason` of `"end_turn"`.
#:       If you want the model to stop generating when it encounters custom strings of
#:       text, you can use the `stop_sequences` parameter. If the model encounters one of
#:       the custom sequences, the response `stop_reason` value will be `"stop_sequence"`
#:       and the response `stop_sequence` value will contain the matched stop sequence.
#:      ...
#:      .
#:       speed: The inference speed mode for this request. `"fast"` enables high
#:       output-tokens-per-second inference.
#:       stop_sequences: Custom text sequences that will cause the model to stop generating.
#:       Our models will normally stop when they have naturally completed their turn,
#:       which will result in a response `stop_reason` of `"end_turn"`.
#:       If you want the model to stop generating when it encounters custom strings of
#:       text, you can use the `stop_sequences` parameter. If the model encounters one of
#:       the custom sequences, the response `stop_reason` value will be `"stop_sequence"`
#:       and the response `stop_sequence` value will contain the matched stop sequence.
#:      ...
#:      calculate_nonstreaming_timeout(
#:       max_tokens, MODEL_NONSTREAMING_TOKENS.get(model, None)
#:       )
#:       if model in DEPRECATED_MODELS:
#:       warnings.warn(
#:      ...
#:      end-of-life on {DEPRECATED_MODELS[model]}.\nPlease migrate to a newer model. Visit https://docs.anthropic.com/en/docs/resources/model-deprecations for more information.",
#:       DeprecationWarning
#:      ...
#:      )
#:       merged_output_config = _merge_output_configs(output_config, output_format)
#:       extra_headers = {
#:       **strip_not_given({"anthropic-beta": ",".join(str(e) for e in betas) if is_given(betas) else not_given}),
#:       **_stainless_helper_header(tools, messages),
#:       **(extra_headers or {}),
#:       }
#:       return self._post(
#:       "/v1/messages?beta=true",
#:       body=maybe_transform(
#:       {
#:       "max_tokens": max_tokens,
#:       "messages": messages,
#:       "model": model,
#:       "cache_control": cache_control,
#:       "container": container,
#:       "context_management": context_management,
#:       "inference_geo": inference_geo,
#:       "mcp_servers": mcp_servers,
#:       "metadata": metadata,
#:       "output_config": merged_output_config,
#:       "output_format": omit,
#:       "service_tier": service_tier,
#:       "speed": speed,
#:       "stop_sequences": stop_sequences,
#:       "stream": stream,
#:       "system": system,
#:       "temperature": temperature,
#:       "thinking": thinking,
#:       "tool_choice": tool_choice,
#:       "tools": tools,
#:       "top_k": top_k,
#:       "top_p": top_p,
#:       "user_profile_id": user_profile_id,
#:       },
#:       message_create_params.MessageCreateParamsStreaming
#:       if stream
#:       else message_create_params.MessageCreateParamsNonStreaming,
#:       ),
#:       options=make_request_options(
#:       extra_headers=extra_headers, extra_query=extra_query, extra_body=extra_body, timeout=timeout
#:       ),
#:       cast_to=BetaMessage,
#:       stream=stream or False,
#:       stream_cls=Stream[BetaRawMessageStreamEvent],
#:       )
#:       def parse(
#:       self,
#:       *,
#:       max_tokens: int,
#:       messages: Iterable[BetaMessageParam],
#:       model: ModelParam,
#:       cache_
#:      ...
#:      _only"] | Omit = omit,
#:       speed: Optional[Literal["standard", "fast"]] | Omit = omit,
#:       stop_sequences: SequenceNotStr[str] | Omit = omit,
#:       stream: Literal[False] | Literal[True] | Omit = omit,
#:      ...
#:      ]] | Omit = omit,
#:       temperature: float | Omit = omit,
#:       thinking: BetaThinkingConfigParam | Omit = omit,
#:       tool_choice: BetaToolChoiceParam | Omit = omit,
#:       tools: Iterable[BetaToolUnionParam] | Omit = omit,
#:       top_k: int | Omit
#:      ...
#:      _headers,
#:       extra_query=extra_query,
#:       extra_body=extra_body,
#:       timeout=timeout,
#:       post_parser=parser,
#:       ),
#:       cast_to=cast(Type[ParsedBetaMessage[ResponseFormatT]], BetaMessage),
#:       stream=False,
#:       )
#:       @
#:      ...
#:      stop_sequences: SequenceNotStr[str] | O
#:      ...
#:      stop_sequences: SequenceNotStr[str] | Omit
#:      ---
#:      Title: Messages - Claude API Reference
#:      URL: https://platform.claude.com/docs/en/api/messages
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      API reference for Messages endpoints. System instructions that appear mid-conversation. Use this block to provide or update system-level instructions at a ...
#:      ---
#:      Title: src/anthropic/types/beta/beta_message.py at main · anthropics/anthropic-sdk-python
#:      URL: https://github.com/anthropics/anthropic-sdk-python/blob/main/src/anthropic/types/beta/beta_message.py
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      "B)"
#:      ...
#:      None
#:       """
#:      ...
#:      (possibly null)
#:      ...
#:      ` on the
#:      ...
#:      .
#:       """
#:       model: Model
#:       """The model that will complete your prompt.
#:       See [models](https://docs.anthropic.com/en/docs/models-overview) for additional
#:       details and options.
#:       """
#:       role: Literal["assistant"]
#:       """Conversational role of the generated message.
#:       This will always be `"assistant"`.
#:       """
#:       stop_details: Optional[BetaRefusalStopDetails] = None
#:       """Structured information about a refusal."""
#:       stop_reason: Optional[BetaStopReason] = None
#:       """The reason that we stopped.
#:       This may be one the following values:
#:       - `"end_turn"`: the model reached a natural stopping point
#:       - `"max_tokens"`: we exceeded the requested `max_tokens` or the model's maximum
#:       - `"stop_sequence"`: one of your provided custom `stop_sequences` was generated
#:       - `"tool_use"`: the model invoked one or more tools
#:       - `"pause_turn"`: we paused a long-running turn. You may provide the response
#:      ...
#:      back as-is in a subsequent request to let the model continue.
#:       - `"refusal"`: when streaming classifiers intervene to handle potential policy
#:      ...
#:      In non-streaming mode this value is always non-null. In streaming mode, it is
#:       null in the `message_start` event and non-null otherwise.
#:       """
#:       stop_sequence: Optional[str] = None
#:       """Which custom stop sequence was generated, if any.
#:       This value will be a non-null string if one of your custom stop sequences was
#:       generated.
#:       """
#:       type: Literal["message"]
#:       """Object type.
#:       For Messages, this is always `"message"`.
#:       """
#:       usage: BetaUsage
#:       """Billing and rate-limit usage.
#:       Anthropic's API bills and rate-limits by token counts, as tokens represent the
#:       underlying cost to our systems.
#:       Under the
#:      ---
#:      Title: src/anthropic/lib/streaming/_messages.py at main · anthropics/anthropic-sdk-python
#:      URL: https://github.com/anthropics/anthropic-sdk-python/blob/main/src/anthropic/lib/streaming/_messages.py
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      def __enter__(self) -> Self:
#:       return self
#:       def __exit__(
#:       self,
#:       exc_type: type[BaseException] | None,
#:       exc: BaseException | None,
#:       exc_tb: TracebackType | None,
#:       ) -> None:
#:       self.close()
#:       def close(self) -> None:
#:       """
#:       Close the response and release the connection.
#:       Automatically called if the response body is read to completion.
#:       """
#:       self._raw_stream.close()
#:       def get_final_message(self) -> ParsedMessage[ResponseFormatT]:
#:       """Waits until the stream has been read to completion and returns
#:       the accumulated `Message` object.
#:       """
#:       self.until_done()
#:       assert self.__final_message_snapshot is not None
#:       return self.__final_message_snapshot
#:      ...
#:      -> Iterator[Parsed
#:      ...
#:      message_snapshot,
#:       output_format=self.__output_format,
#:       )
#:      ...
#:      StreamManager(Generic[ResponseFormatT]):
#:       """Wrapper over MessageStream that is returned by `.stream()
#:      ...
#:      ```py
#:      ...
#:      client.messages.stream(...) as stream:
#:              for chunk in stream:
#:                  ...
#:          ```
#:       """
#:       def __init__(
#:       self,
#:       api_request: Callable[[], Stream[RawMessageStreamEvent]],
#:       *,
#:       output_format: ResponseFormatT | NotGiven,
#:       ) -> None:
#:       self.__stream:
#:      ...
#:      Stream[ResponseFormatT] | None = None
#:       self.__api_request = api_request
#:       self.__output_format = output_format
#:      ...
#:      def __enter__(
#:      ...
#:      T]:
#:      ...
#:      ()
#:       self
#:      ...
#:      def __exit__(
#:       self,
#:       exc_type: type[BaseException] | None,
#:       exc: BaseException | None,
#:       exc_tb: TracebackType | None,
#:       ) -> None:
#:       if self.__stream is not None:
#:       self.__stream.close()
#:      ...
#:      async def close(self) -> None:
#:       """
#:       Close the response and release the connection.
#:       Automatically called if the response body is read to completion.
#:       """
#:       await self._raw_stream.close()
#:       async def get_final_message(self) -> ParsedMessage[ResponseFormatT]:
#:       """Waits until the stream has been read to completion and returns
#:       the accumulated `Message` object.
#:       """
#:       await self.until_done()
#:       assert self.__final_message_snapshot is not None
#:       return self.__final_message_snapshot
#:      ...
#:      aexit__(
#:       self,
#:      ...
#:      ,
#:       exc_tb:
#:      ...
#:      None:
#:       if
#:      ...
#:      is not None:
#:       await self.__
#:      ...
#:      .close()
#:      ...
#:      def build_events(
#:       *,
#:       event: RawMessageStreamEvent,
#:       message_snapshot: ParsedMessage[ResponseFormatT],
#:      ) -> list[ParsedMessageStreamEvent[ResponseFormatT]]:
#:       events_to_fire: list[ParsedMessageStreamEvent[ResponseFormatT]] = []
#:       if event.type == "message_start":
#:       events_to_fire.append(event)
#:       elif event.type == "message_delta":
#:       events_to_fire.append(event)
#:       elif event.type == "message_stop":
#:       events_to_fire.append(
#:       build(ParsedMessageStopEvent[ResponseFormatT], type="message_stop", message=message_snapshot)
#:       )
#:       elif event.type == "content_block_start":
#:       events_to_fire.append(event)
#:       elif event.type == "content_block_delta":
#:       events_to_fire.append(event)
#:       content_block = message_snapshot.content[event.index]
#:       if event.delta.type == "text_delta":
#:       if content_block.type == "text":
#:       events_to_fire.append(
#:       build(
#:       TextEvent,
#:       type="text",
#:       text=event.delta.text,
#:       snapshot=content_block.text,
#:       )
#:       )
#:       elif event.delta.type == "input_json_delta":
#:       if content_block.type == "tool_use":
#:       events_to_fire.append(
#:       build(
#:       InputJsonEvent,
#:       type="input_json",
#:       partial_json=event.delta.partial_json,
#:       snapshot=content_block.input,
#:       )
#:       )
#:       elif event.delta.type == "citations_delta":
#:       if content_block.type == "text":
#:       events_to_fire.append(
#:       build(
#:       CitationEvent,
#:       type="citation",
#:       citation=event.delta.citation,
#:       snapshot=content_block.citations or [],
#:       )
#:       )
#:       elif event.delta.type == "thinking_delta":
#:       if content_block.type == "thinking":
#:       events_to_fire.append(
#:       build(
#:       ThinkingEvent,
#:       type="thinking",
#:       thinking=event.delta.thinking,
#:       snapshot=content_block.thinking,
#:       )
#:       )
#:       elif event.delta.type == "signature_delta":
#:       if content_block.type == "thinking":
#:       events_to_fire.append(
#:       build(
#:       SignatureEvent,
#:       type="signature",
#:       signature=content_block.signature,
#:       )
#:       )
#:       pass
#:       else:
#:       # we only want exhaustive checking for linters, not at runtime
#:       if TYPE_CHECKING: # type: ignore[unreachable]
#:       assert_never(event.delta)
#:       elif event.type == "content_block_stop":
#:       content_block = message_snapshot.content[event.index]
#:       event_to_fire = build(
#:       ParsedContentBlockStopEvent,
#:       type="content_block_stop",
#:       index=event.index,
#:       content_block=content_block,
#:       )
#:       events_to_fire.append(event_to_fire)
#:       else:
#:       # we only want exhaustive checking for linters, not at runtime
#:       if TYPE_CHECKING: # type: ignore[unreachable]
#:       assert_never(event)
#:       return events_to_fire
#:      ...
#:      def accumulate_event(
#:       *,
#:       event: RawMessageStreamEvent,
#:       current_snapshot: ParsedMessage[ResponseFormatT] | None,
#:       output_format: ResponseFormatT | NotGiven = NOT_GIVEN,
#:      ) -> ParsedMessage[ResponseFormatT]:
#:       if not isinstance(cast(Any, event), BaseModel):
#:       event = cast( # pyright: ignore[reportUnnecessaryCast]
#:       RawMessageStreamEvent,
#:       construct_type_unchecked(
#:       type_=cast(Type[RawMessageStreamEvent], RawMessageStreamEvent),
#:       value=event,
#:       ),
#:       )
#:       if not isinstance(cast(Any, event), BaseModel):
#:       raise TypeError(f"Unexpected event runtime type, after deserialising twice - {event} - {type(event)}")
#:       if current_snapshot is None:
#:       if event.type == "message_start":
#:       return cast(ParsedMessage[ResponseFormatT], ParsedMessage.construct(**cast(Any, event.message.to_dict())))
#:       raise RuntimeError(f'Unexpected event order, got {event.type} before "message_start"')
#:       if event.type == "content_block_start":
#:       # TODO: check index
#:       current_snapshot.content.append(
#:       cast(
#:       Any, # Pydantic does not support generic unions at runtime
#:       construct_type(type_=ParsedContentBlock, value=event.content_block.model_dump()),
#:       ),
#:       )
#:       elif event.type == "content_block_delta":
#:       content = current_snapshot.content[event.index]
#:       if event.delta.type == "text_delta":
#:       if content.type == "text":
#:       content.text += event.delta.text
#:       elif event.delta.type == "input_json_delta":
#:       if isinstance(content, TRACKS_TOOL_INPUT):
#:       from jiter import from_json
#:       # we need to keep track of the raw JSON string as well so that we can
#:       # re-parse it for each delta, for now we just store it as an untyped
#:       # property on the snapshot
#:       json_buf = cast(bytes, getattr(content, JSON_BUF_PROPERTY, b""))
#:       json_buf += bytes(event.delta.partial_json, "utf-8")
#:       if json_buf:
#:       content.input = from_json(json_buf, partial_mode=True)
#:      ...
#:      setattr(content, JSON_BUF_PROPERTY, json_buf)
#:       elif event.delta.type == "citations_delta":
#:       if content.type == "text":
#:       if not content.citations:
#:       content.citations = [event.delta.citation]
#:       else:
#:       content.citations.append(event.delta.citation)
#:       elif event.delta.type == "thinking_delta":
#:       if content.type == "thinking":
#:       content.thinking += event.delta.thinking
#:       elif event.delta.type == "signature_delta":
#:       if content.type == "thinking":
#:       content.signature = event.delta.signature
#:       else:
#:       # we only want exhaustive checking for linters, not at runtime
#:       if TYPE_CHECKING: # type: ignore[unreachable]
#:       assert_never(event.delta)
#:       elif event.type == "content_block_stop":
#:       content_block = current_snapshot.content[event.index]
#:       if content_block.type == "text" and is_given(output_format):
#:       content_block.parsed_output = parse_text(content_block.text, output_format)
#:       elif event.type == "message_delta":
#:       current_snapshot.stop_reason = event.delta.stop_reason
#:       current_snapshot.stop_sequence = event.delta.stop_sequence
#:       if event.delta.stop_details is not None:
#:       current_snapshot.stop_details = event.delta.stop_details
#:       current_snapshot.usage.output_tokens = event.usage.output_tokens
#:       # Update other usage fields if they exist in the event
#:       if event.usage.input_tokens is not None:
#:       current_snapshot.usage.input_tokens = event.usage.input_tokens
#:       if event.usage.cache_creation_input_tokens is not None:
#:       current_snapshot.usage.cache_creation_input_tokens = event.usage.cache_creation_input_tokens
#:       if event.usage.cache_read_input_tokens is not None:
#:       current_snapshot.usage.cache_read_input_tokens = event.usage.cache_read_input_tokens
#:       if event.usage.server_tool_use is not None:
#:       current_snapshot.usage.server_tool_use = event.usage.server_tool_use
#:       return current_snapshot
#:      ---
#:      Title: src/anthropic/resources/beta/messages/messages.py at 49d639a6 · anthropics/anthropic-sdk-python
#:      URL: https://github.com/anthropics/anthropic-sdk-python/blob/49d639a6/src/anthropic/resources/beta/messages/messages.py
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      @cached_property
#:          def with_streaming_response(self) -> MessagesWithStreamingResponse:
#:              """
#:              An alternative to `.with_raw_response` that doesn't eagerly read the response body.
#:              For more information, see https://www.github.com/anthropics/anthropic-sdk-python#with_streaming_response
#:              """
#:              return MessagesWithStreamingResponse(self)
#:          @overload
#:          def create(
#:              self,
#:              *,
#:              max_tokens: int,
#:              messages: Iterable[BetaMessageParam],
#:              model: ModelParam,
#:              cache_control: Optional[BetaCacheControlEphemeralParam] | Omit = omit,
#:              container: Optional[message_create_params.Container] | Omit = omit,
#:              context_management: Optional[BetaContextManagementConfigParam] | Omit = omit,
#:              inference_geo: Optional[str] | Omit = omit,
#:              mcp_servers: Iterable[BetaRequestMCPServerURLDefinitionParam] | Omit = omit,
#:              metadata: BetaMetadataParam | Omit = omit,
#:              output_config: BetaOutputConfigParam | Omit = omit,
#:              output_format: Optional[BetaJSONOutputFormatParam] | Omit = omit,
#:              service_tier: Literal["auto", "standard_only"] | Omit = omit,
#:              speed: Optional[Literal["standard", "fast"]] | Omit = omit,
#:              stop_sequences: SequenceNotStr[str] | Omit = omit,
#:              stream: Literal[False] | Omit = omit,
#:              system: Union[str, Iterable[BetaTextBlockParam]] | Omit = omit,
#:              temperature: float | Omit = omit,
#:              thinking: BetaThinkingConfigParam | Omit = omit,
#:              tool_choice: BetaToolChoiceParam | Omit = omit,
#:              tools: Iterable[BetaToolUnionParam] | Omit = omit,
#:              top_k: int | Omit = omit,
#:              top_p: float | Omit = omit,
#:              betas: List[AnthropicBetaParam] | Omit = omit,
#:              # Use the following arguments if you need to pass additional parameters to the API that aren't available via kwargs.
#:              # The extra values given here take precedence over values defined on the client or passed to this method.
#:              extra_headers: Headers | None = None,
#:              extra_query: Query | None = None,
#:              extra_body: Body | None = None,
#:              timeout: float | httpx.Timeout | None | NotGiven = not_given,
#:          ) -> BetaMessage:
#:              """
#:              Send a structured list of input messages with text and/or image content, and the
#:              model will generate the next message in the conversation.
#:              The Messages API can be used for either single queries or stateless multi-turn
#:              conversations.
#:              Learn more about the Messages API in our
#:              [user guide](https://docs.claude.com/en/docs/initial-setup)
#:              Args:
#:                max_tokens
#:      ...
#:      The maximum number of tokens to generate before stopping.
#:      ...
#:      the `messages` parameter,
#:      ...
#:      service_
#:      ...
#:      .
#:       Anthropic offers different levels of service for your API requests. See
#:       [service-tiers](https://docs.claude.com/en/api/service-tiers) for details.
#:       speed: The inference speed mode for this request. `"fast"` enables high
#:       output-tokens-per-second inference.
#:       stop_sequences: Custom text sequences that will cause the model to stop generating.
#:       Our models will normally stop when they have naturally completed their turn,
#:       which will result in a response `stop_reason` of `"end_turn"`.
#:       If you want the model to stop generating when it encounters custom strings of
#:       text, you can use the `stop_sequences` parameter. If the model encounters one of
#:       the custom sequences, the response `stop_reason` value will be `"stop_sequence"`
#:       and the response `stop_sequence` value will contain the matched stop sequence.
#:       stream: Whether to incrementally stream the response using server-sent events.
#:       See [streaming](https://docs.claude.com/en/api/messages-streaming) for details.
#:       system: System prompt.
#:       A system prompt is a way of providing context and instructions to Claude, such
#:       as specifying a particular goal or role. See our
#:       [guide to system prompts](https://docs.claude.com/en/docs/system-prompts).
#:       temperature
#:      ...
#:      ](https://docs
#:      ...
#:      claude.com/en/
#:      ...
#:      ) for details.
#:       speed: The inference speed mode for this request. `"fast"` enables high
#:       output-tokens-per-second inference.
#:       stop_sequences: Custom text sequences that will cause the model to stop generating.
#:       Our models will normally stop when they have naturally completed their turn,
#:       which will result in a response `stop_reason` of `"end_turn"`.
#:       If you want the model to stop generating when it encounters custom strings of
#:       text, you can use the `stop_sequences` parameter. If the model encounters one of
#:       the custom sequences, the response `stop_reason` value will be `"stop_sequence"`
#:       and the response `stop_sequence` value will contain the matched stop sequence.
#:      ...
#:      Anthropic offers different
#:      ...
#:      of service for your API requests. See
#:       [service-
#:      ...
#:      ](https://docs.claude.com/en/api/service-tiers) for details.
#:       speed: The inference speed mode for this request. `"fast"` enables high
#:       output-tokens-per-second inference.
#:       stop_sequences: Custom text sequences that will cause the model to stop generating.
#:       Our models will normally stop when they have naturally completed their turn,
#:       which will result in a response `stop_reason` of `"end_turn"`.
#:       If you want the model to stop generating when it encounters custom strings of
#:       text, you can use the `stop_sequences` parameter. If the model encounters one of
#:       the custom sequences, the response `stop_reason` value will be `"stop_sequence"`
#:       and the response `stop_sequence` value will contain the matched stop sequence.
#:      ...
#:      ) -> Beta
#:      ...
#:      )
#:       merged_output_config = _merge_output_configs(output_config, output_format)
#:       extra_headers = {
#:       **strip_not_given({"anthropic-beta": ",".join(str(e) for e in betas) if is_given(betas) else not_given}),
#:       **_stainless_helper_header(tools, messages),
#:       **(extra_headers or {}),
#:       }
#:       return self._post(
#:       "/v1/messages?beta=true",
#:       body=maybe_transform(
#:       {
#:       "max_tokens": max_tokens,
#:       "messages": messages,
#:       "model": model,
#:       "cache_control": cache_control,
#:       "container": container,
#:       "context_management": context_management,
#:       "inference_geo": inference_geo,
#:       "mcp_servers": mcp_servers,
#:       "metadata": metadata,
#:       "output_config": merged_output_config,
#:       "output_format": omit,
#:       "service_tier": service_tier,
#:       "speed": speed,
#:       "stop_sequences": stop_sequences,
#:       "stream": stream,
#:       "system": system,
#:       "temperature": temperature,
#:       "thinking": thinking,
#:       "tool_choice": tool_choice,
#:       "tools": tools,
#:       "top_k": top_k,
#:       "top_p": top_p,
#:       },
#:       message_create_params.MessageCreateParamsStreaming
#:       if stream
#:       else message_create_params.MessageCreateParamsNonStreaming,
#:       ),
#:       options=make_request_options(
#:       extra_headers=extra_headers, extra_query=extra_query, extra_body=extra_body, timeout=timeout
#:       ),
#:       cast_to=BetaMessage,
#:       stream=stream or False,
#:       stream_cls=Stream[BetaRawMessageStreamEvent],
#:       )
#:       def parse(
#:       self,
#:       *,
#:       max_tokens: int,
#:       messages: Iterable[BetaMessageParam],
#:       model: ModelParam,
#:       cache_control: Optional[BetaCacheControlEphemeralParam] | Omit = omit,
#:      ...
#:      service_tier: Literal["auto", "standard_only"] | Omit = omit,
#:       speed: Optional[Literal["standard", "fast"]] | Omit = omit,
#:       stop_sequences: SequenceNotStr[str] | Omit = omit,
#:       stream: Literal[False] | Literal[True] | Omit = omit,
#:       system: Union[str, Iterable[BetaTextBlockParam]] | Omit = omit,
#:       temperature: float | Omit = omit,
#:       thinking: BetaThinkingConfigParam | Omit = omit,
#:       tool_choice: BetaToolChoiceParam | Omit = omit,
#:       tools: Iterable[BetaToolUnionParam] | Omit = omit,
#:       top_k: int | Omit = omit,
#:       top_p: float | Omit = omit,
#:      ...
#:      service_tier: Literal["auto", "standard_only"] | Omit = omit,
#:       speed: Optional[Literal["standard", "fast"]] | Omit = omit,
#:       stop_sequences: SequenceNotStr[str] | Omit = omit,
#:       system: Union[str, Iterable[BetaTextBlockParam]] | Omit = omit,
#:       temperature: float | Omit = omit,
#:       thinking: BetaThinkingConfigParam | Omit = omit,
#:       tool_choice: BetaToolChoiceParam | Omit = omit,
#:       tools: Iterable[BetaToolUnionParam] | Omit = omit,
#:       top_k: int | O
#:      ---
#:      Title: src/anthropic/resources/completions.py at main · anthropics/anthropic-sdk-python
#:      URL: https://github.com/anthropics/anthropic-sdk-python/blob/main/src/anthropic/resources/completions.py
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      class AsyncCompletions(AsyncAPIResource):
#:          @cached_property
#:          def with_raw_response(self) -> AsyncCompletionsWithRawResponse:
#:              """
#:              This property can be used as a prefix for any HTTP method call to return
#:              the raw response object instead of the parsed content.
#:              For more information, see https://www.github.com/anthropics/anthropic-sdk-python#accessing-raw-response-data-eg-headers
#:              """
#:              return AsyncCompletionsWithRawResponse(self)
#:      ...
#:      class CompletionsWithStreamingResponse:
#:          def __init__(self, completions: Completions) -> None:
#:              self._completions = completions
#:              self.create = to_streamed_response_wrapper(
#:                  completions.create,
#:              )
#:      ...
#:      class AsyncCompletionsWithStreamingResponse:
#:          def __init__(self, completions: AsyncCompletions) -> None:
#:              self._completions = completions
#:              self.create = async_to_streamed_response_wrapper(
#:                  completions.create,
#:              )
#:      ---
#:      Title: helpers.md at cd2fb42d7d90a576e49caed9b4741f37ab091f5a · anthropics/anthropic-sdk-python
#:      URL: https://github.com/anthropics/anthropic-sdk-python/blob/cd2fb42d7d90a576e49caed9b4741f37ab091f5a/helpers.md
#:      Published: N/A
#:      Author: N/A
#:      Highlights:
#:      `client.messages.stream()` returns a `MessageStreamManager`, which is a context manager that yields a `MessageStream` which is iterable, emits events and accumulates messages.
#:      ...
#:      Alternatively, you can use `client.messages.create(..., stream=True)` which returns an
#:      iterable of the events in the stream and uses less memory (most notably, it does not accumulate a final message
#:      object for you).
#:      ...
#:      The stream will be cancelled when the context manager exits but you can also close it prematurely by calling `stream.close()`.
#:      ...
#:      #### `await .close()`
#:      ...
#:      Aborts the request.
#:      ...
#:      #### `await .until_done()`
#:      ...
#:      Blocks until the stream has been read to completion.
#:      ...
#:      #### `await .get_final_message()`
#:      ...
#:      Blocks until the stream has been read to completion and returns the accumulated `Message` object.
#:      ...
#:      > [!NOTE]
#:      ...
#:      > Currently the API will only ever return
#:      ...
#:      1 content block
#:      ...
#:      Blocks until the stream has been read to completion and returns all `text` content blocks concatenated together.
/allow 2362b4c3-8ac7-4c57-bba6-de23c53e50bf
#: Control Request:
#:id: "4b155969-7fad-4408-a4cb-913f58e69bb0"
#:phase: "request"
#:toolName: "web-search-exa"
#:arguments:
#:  query: "Claude API stream.controller.abort() cancel streaming response"
/allow 4b155969-7fad-4408-a4cb-913f58e69bb0
#: Control Request:
#:id: "555e114c-47f3-4e48-b352-1ea50ac3b4f0"
#:phase: "result"
#:toolName: "web-search-exa"
#:result:
#:  content: []
#:  structuredContent:
#:    content: "Title: TypeScript SDK - Claude Platform Docs\nURL: https://platform.claude.com/docs/en/cli-sdks-libraries/sdks/typescript\nPublished: N/A\nAuthor: N/A\nHighlights:\n## Streaming responses\n...\nThe SDK provides support for streaming responses using Server Sent Events (SSE).\n...\nIf you need to cancel a stream, you can `break` from the loop or call `stream.controller.abort()`.\n\n---\n\nTitle: README.md at 21ad14ff1f0c0930f3b99b6e03167bf9d57fbd9d · anthropics/anthropic-sdk-typescript\nURL: https://github.com/anthropics/anthropic-sdk-typescript/blob/21ad14ff1f0c0930f3b99b6e03167bf9d57fbd9d/README.md\nPublished: N/A\nAuthor: N/A\nHighlights:\n## Streaming responses\n...\nWe provide support for streaming responses using Server Sent Events (SSE).\n...\nIf you need to cancel a stream, you can `break` from the loop\nor call `stream.controller.abort()`.\n\n---\n\nTitle: src/core/streaming.ts at 0f8153b3 · anthropics/anthropic-sdk-typescript\nURL: https://github.com/anthropics/anthropic-sdk-typescript/blob/0f8153b3/src/core/streaming.ts\nPublished: N/A\nAuthor: N/A\nHighlights:\nexport class Stream<Item> implements AsyncIterable<Item> {\n  controller: AbortController;\n  #client: BaseAnthropic | undefined;\n\n  constructor(\n    private iterator: () => AsyncIterator<Item>,\n    controller: AbortController,\n    client?: BaseAnthropic,\n  ) {\n    this.controller = controller;\n    this.#client = client;\n  }\n\n  static fromSSEResponse<Item>(\n    response: Response,\n    controller: AbortController,\n    client?: BaseAnthropic,\n  ): Stream<Item> {\n    let consumed = false;\n    const logger = client ? loggerFor(client) : console;\n\n    async function* iterator(): AsyncIterator<Item, any, undefined> {\n      if (consumed) {\n        throw new AnthropicError('Cannot iterate over a consumed stream, use `.tee()` to split the stream.');\n      }\n      consumed = true;\n      let done = false;\n      try {\n        for await (const sse of _iterSSEMessages(response, controller)) {\n          if (sse.event === 'completion') {\n            try {\n              yield JSON.parse(sse.data) as Item;\n            } catch (e) {\n              logger.error(`Could not parse message into JSON:`, sse.data);\n              logger.error(`From chunk:`, sse.raw);\n              throw e;\n            }\n          }\n\n          if (\n            sse.event === 'message_start' ||\n            sse.event === 'message_delta' ||\n            sse.event === 'message_stop' ||\n            sse.event === 'content_block_start' ||\n            sse.event === 'content_block_delta' ||\n            sse.event === 'content_block_stop'\n          ) {\n            try {\n              yield JSON.parse(sse.data) as Item;\n            } catch (e) {\n              logger.error(`Could not parse message into JSON:`, sse.data);\n              logger.error(`From chunk:`, sse.raw);\n              throw e;\n            }\n          }\n\n          if (sse.event === 'ping') {\n            continue;\n          }\n\n          if (sse.event === 'error') {\n            throw new APIError(undefined, safeJSON(sse.data) ?? sse.data, undefined, response.headers);\n          }\n        }\n        done = true;\n      } catch (e) {\n        // If the user calls `stream.controller.abort()`, we should exit without throwing.\n        if (isAbortError(e)) return;\n        throw e;\n      } finally {\n        // If the user `break`s, abort the ongoing request.\n        if (!done) controller.abort();\n      }\n    }\n\n    return new Stream(iterator, controller, client);\n  }\n\n  /**\n   * Generates a Stream from a newline-separated ReadableStream\n   * where each item is a JSON value.\n   */\n  static fromReadableStream<Item>(\n    readableStream: ReadableStream,\n    controller: AbortController,\n    client?: BaseAnthropic,\n  ): Stream<Item> {\n    let consumed = false;\n\n    async function* iterLines(): AsyncGenerator<string, void, unknown> {\n      const lineDecoder = new LineDecoder();\n\n      const iter = ReadableStreamToAsyncIterable<Bytes>(readableStream);\n      for await (const chunk of iter) {\n        for (const line of lineDecoder.decode(chunk)) {\n          yield line;\n        }\n      }\n\n      for (const line of lineDecoder.flush()) {\n        yield line;\n      }\n    }\n\n    async function* iterator(): AsyncIterator<Item, any, undefined> {\n      if (consumed) {\n        throw new AnthropicError('Cannot iterate over a consumed stream, use `.tee()` to split the stream.');\n      }\n      consumed = true;\n      let done = false;\n      try {\n        for await (const line of iterLines()) {\n          if (done) continue;\n          if (line) yield JSON.parse(line) as Item;\n        }\n        done = true;\n      } catch (e) {\n        // If the user calls `stream.controller.abort()`, we should exit without throwing.\n        if (isAbortError(e)) return;\n        throw e;\n      } finally {\n        // If the user `break`s, abort the ongoing request.\n        if (!done) controller.abort();\n      }\n    }\n\n    return new Stream(iterator, controller, client);\n  }\n\n  [Symbol.asyncIterator](): AsyncIterator<Item> {\n    return this.iterator();\n  }\n\n  /**\n   * Splits the stream into two streams which can be\n   * independently read from at different speeds.\n   */\n  tee(): [Stream<Item>, Stream<Item>] {\n    const left: Array<Promise<IteratorResult<Item>>> = [];\n    const right: Array<Promise<IteratorResult<Item>>> = [];\n    const iterator = this.iterator();\n\n    const teeIterator = (queue: Array<Promise<IteratorResult<Item>>>): AsyncIterator<Item> => {\n      return {\n        next: () => {\n          if (queue.length === 0) {\n            const result = iterator.next();\n            left.push(result);\n            right.push(result);\n          }\n          return queue.shift()!;\n        },\n      };\n    };\n\n    return [\n      new Stream(() => teeIterator(left), this.controller, this.#client),\n      new Stream(() => teeIterator(right), this.controller, this.#client),\n    ];\n  }\n\n  /**\n   * Converts this stream to a newline-separated ReadableStream of\n   * JSON stringified values in the stream\n   * which can be turned back into a Stream with `Stream.fromReadableStream()`.\n   */\n  toReadableStream(): ReadableStream {\n    const self = this;\n    let iter: AsyncIterator<Item>;\n\n    return makeReadableStream({\n      async start() {\n        iter = self[Symbol.asyncIterator]();\n      },\n      async pull(ctrl: any) {\n        try {\n          const { value, done } = await iter.next();\n          if (done) return ctrl.close();\n\n          const bytes = encodeUTF8(JSON.stringify(value) + '\\n');\n\n          ctrl.enqueue(bytes);\n        } catch (err) {\n          ctrl.error(err);\n        }\n      },\n      async cancel() {\n        await iter.return?.();\n      },\n    });\n  }\n}\n...\nexport async function* _iterSSEMessages(\n  response: Response,\n  controller: AbortController,\n): AsyncGenerator<ServerSentEvent, void, unknown> {\n  if (!response.body) {\n    controller.abort();\n    if (\n      typeof (globalThis as any).navigator !== 'undefined' &&\n      (globalThis as any).navigator.product === 'ReactNative'\n    ) {\n      throw new AnthropicError(\n        `The default react-native fetch implementation does not support streaming. Please use expo/fetch: https://docs.expo.dev/versions/latest/sdk/expo/#expofetch-api`,\n      );\n    }\n    throw new AnthropicError(`Attempted to iterate over a response with no body`);\n  }\n\n  const sseDecoder = new SSEDecoder();\n  const lineDecoder = new LineDecoder();\n\n  const iter = ReadableStreamToAsyncIterable<Bytes>(response.body);\n  for await (const sseChunk of iterSSEChunks(iter)) {\n    for (const line of lineDecoder.decode(sseChunk)) {\n      const sse = sseDecoder.decode(line);\n      if (sse) yield sse;\n    }\n  }\n\n  for (const line of lineDecoder.flush()) {\n    const sse = sseDecoder.decode(line);\n    if (sse) yield sse;\n  }\n}\n...\ni.e\n...\nlet data = new Uint8Array\n...\n;\n    }\n\n    const binaryChunk =\n      chunk instanceof ArrayBuffer ? new Uint8Array(chunk)\n      : typeof chunk ===\n...\nwhile ((patternIndex = findDoubleNewlineIndex(data\n...\n}\n}\n...\nclass SSEDecoder {\n  private data: string[];\n  private event: string | null;\n  private chunks: string[];\n\n  constructor() {\n    this.event = null;\n    this.data = [];\n    this.chunks = [];\n  }\n\n  decode(line: string) {\n    if (line.endsWith('\\r')) {\n      line = line.substring(0, line.length - 1);\n    }\n\n    if (!line) {\n      // empty line and we didn't previously encounter any messages\n      if (!this.event && !this.data.length) return null;\n\n      const sse: ServerSentEvent = {\n        event: this.event,\n        data: this.data.join('\\n'),\n        raw: this.chunks,\n      };\n\n      this.event = null;\n      this.data = [];\n      this.chunks = [];\n\n      return sse;\n    }\n\n    this.chunks.push(line);\n\n    if (line.startsWith(':')) {\n      return null;\n    }\n\n    let [fieldname, _, value] = partition(line, ':');\n\n    if (value.startsWith(' ')) {\n      value = value.substring(1);\n    }\n\n    if (fieldname === 'event') {\n      this.event = value;\n    } else if (fieldname === 'data') {\n      this.data.push(value);\n    }\n\n    return null;\n  }\n}\n\n---\n\nTitle: Cancelling Claude API Streams the Right Way: AbortController, Token Billing, and Connection Hygiene | Claude Lab\nURL: https://claudelab.net/en/articles/api-sdk/claude-api-streaming-cancellation-abortcontroller-cost-guide\nPublished: 2026-05-02T08:00:00.000Z\nAuthor: Masaki Hirokawa\nHighlights:\nHow to cancel Claude API streams with AbortController, what gets billed when you stop mid-stream, and the production gotchas — Node.js + Python.\n...\nHere's a focused guide to cancelling Claude API streams cleanly with AbortController, what the billing actually looks like when you do, and the connection-pool gotcha that took down our chat endpoint once. Examples are in TypeScript (Node.js) and Python (asyncio), tested against the latest Anthropic SDKs and Claude Sonnet 4.6.\n...\nSo \"I cancelled, therefore zero\" is wrong. The right model is \"I paid through the cut point, plus a little drift.\" Anthropic surfaces incremental token counts in the`message_delta` event's`usage.output_tokens` field. Capture the most recent value before the abort fires and your accounting will be accurate.\n...\nThe Anthropic SDK's`messages.stream()` accepts an`AbortSignal`. The cleanest pattern in Express is to bind a controller to the request lifecycle so a client disconnect tears the upstream stream down too.\n...\n```\nimport Anthropic from \"@anthropic-ai/sdk\";\nimport express from \"express\";\n \nconst client = new Anthropic({ apiKey: process.env.ANTHROPIC_API_KEY! });\nconst app = express();\n \napp.post(\"/api/chat/stream\", express.json(), async (req, res) => {\n  // Tear down the upstream call when the browser disconnects\n  const controller = new AbortController();\n  req.on(\"close\", () => controller.abort());\n \n  res.setHeader(\"Content-Type\", \"text/event-stream\");\n  res.setHeader(\"Cache-Control\", \"no-cache, no-transform\");\n  res.setHeader(\"Connection\", \"keep-alive\");\n \n  let outputTokens = 0;\n  try {\n    const stream = await client.messages.stream(\n      {\n        model: \"claude-sonnet-4-6\",\n        max_tokens: 1024,\n        messages: req.body.messages,\n      },\n      { signal: controller.signal },\n    );\n \n    for await (const event of stream) {\n      if (event.type === \"content_block_delta\" && event.delta.type === \"text_delta\") {\n        res.write(`data: ${JSON.stringify({ text: event.delta.text })}\\n\\n`);\n      } else if (event.type === \"message_delta\") {\n        // Keep overwriting — the last value before abort is what you pay for\n        outputTokens = event.usage.output_tokens;\n      }\n    }\n \n    res.write(`data: [DONE]\\n\\n`);\n    res.end();\n  } catch (err: any) {\n    // AbortError is the success path for \"user clicked Stop\"\n    if (err.name === \"AbortError\" || controller.signal.aborted) {\n      console.log(`Stream cancelled. Output tokens billed: ${outputTokens}`);\n      return; // res is already closed\n    }\n    console.error(err);\n    res.write(`data: ${JSON.stringify({ error: \"stream_failed\" })}\\n\\n`);\n    res.end();\n  } finally {\n    await recordUsage({ outputTokens, cancelled: controller.signal.aborted });\n  }\n});\n \nasync function recordUsage(_: { outputTokens: number; cancelled: boolean }) {\n  // Persist to your metrics pipeline (Postgres, Datadog, whatever)\n}\n...\nThree details matter here. First,`req.on(\"close\", () => controller.abort())`— without this line the upstream stream keeps consuming tokens after the user has closed the tab. Second, capture`event.usage.output_tokens` on every`message_delta`; the last value you saw before the abort is the one you'll be billed for. Third, treat`AbortError` as a success path, not an error. Logging it at error level is the fastest way to make your alerting useless.\n...\nThe Python SDK exposes the cancellation mechanism through context managers and`asyncio.CancelledError`. Pair it with FastAPI's`request.is_disconnected()` and you get the same hygiene.\n...\nfrom fastapi.responses import StreamingResponse\n \napp = FastAPI()\nclient = AsyncAnthropic()\n \n@app.post(\"/api/chat/stream\")\nasync def stream_chat(request: Request, body: dict):\n    output_tokens = 0\n \n    async def generate():\n        nonlocal output_tokens\n        try:\n            async with client.messages.stream(\n                model=\"claude-sonnet-4-6\",\n                max_tokens=1024,\n                messages=body[\"messages\"],\n            ) as stream:\n                async for event in stream:\n                    # Stop pulling if the client is gone\n                    if await request.is_disconnected():\n                        break\n \n                    if event.type == \"content_block_delta\" and event.delta.type == \"text_delta\":\n                        yield f\"data: {event.delta.text}\\n\\n\"\n                    elif event.type == \"message_delta\":\n                        output_tokens = event.usage.output_tokens\n \n                yield \"data: [DONE]\\n\\n\"\n        except asyncio.CancelledError:\n            # FastAPI raises this when the connection is torn down\n            pass\n        finally:\n            await record_usage(output_tokens, cancelled=await request.is_disconnected())\n \n    return StreamingResponse(generate(), media_type=\"text/event-stream\")\n \nasync def record_usage(output_tokens: int, cancelled: bool):\n    # Persist to your metrics pipeline\n    pass\n...\n`await request.is_disconnected()` polls the underlying TCP connection. Catch`asyncio.CancelledError` and let it pass — that's the user-cancelled path, not an exception worth alerting on.\n...\nHere's the failure mode I learned the hard way. If you handle cancellation lazily, every aborted request leaves an HTTPS connection to Anthropic in a half-open state. The SDK's underlying connection pool fills up. Around 100 concurrent in-flight requests, new chats stop returning. You see`ECONNRESET` sprinkle into your logs and a slow climb in p99 latency before everything tips over. None of this is obviously about cancellation until you trace it back to half-open upstream connections that nobody is reading from. The fix is one-line, but you have to know to look for it.\n...\n- Always cancel through the SDK's signal (or`async with` in Python). Hand-rolling`fetch` and trying to`cancel()` the underlying ReadableStream tends to leak — let the SDK do its cleanup.\n- Wire client-disconnect detection with`req.on(\"close\")` or`request.is_disconnected()`. Without it, the upstream stream keeps draining tokens long after the user has left.\n- Set a hard timeout for safety.`AbortSignal.timeout(60_000)` covers the case where the connection is silently dead but no event fires.\n...\nYou don't have to take my word that \"you only pay for what was generated.\" The Anthropic API console has a usage panel, and the SDK exposes`usage` in the final stream event, so you can prove this end to end in five minutes:\n...\n1. Send a deliberately long generation request (`max_tokens: 4096`) and abort after the first 20 tokens render.\n2. In the abort handler, log the most recent`event.usage.output_tokens` you saw.\n3. Wait a few minutes for usage aggregation, then open the console — the request you just made should show roughly the same output token count, plus a small drift.\n...\nIf the console shows dramatically more than what you logged, that's a signal the abort wasn't actually closing the upstream connection — go back and check your`req.on(\"close\")` wiring. I keep a small load test that does exactly this on every release of our chat service; it's caught two regressions in the past year, both times from middleware that was buffering the response body and preventing the close event from firing.\n...\nexponential backoff\n...\nThe SDK does not retry after an`AbortError`, but if you've layered your own retry wrapper on top, make sure it checks`controller.signal.aborted` before resending. Otherwise a deliberate cancel becomes two billed responses. The retry side of the coin is covered in Retry-After header and backoff strategy.\n...\nThe smallest meaningful change you can make today is to add`req.on(\"close\", () => controller.abort())`(or`request.is_disconnected()` in Python) to your existing chat endpoint. Five minutes of work, and the long tail of orphaned streams from users who closed the tab quietly disappears. Your connection pool gets headroom you didn't know you'd lost, and your bill drops by however many tokens were getting generated into the void.\n\n---\n\nTitle: Cancellation of requests via AbortSignal\nURL: https://github.com/anthropics/anthropic-sdk-typescript/issues/43\nPublished: N/A\nAuthor: N/A\nHighlights:\nPrevious versions of Anthropic SDK pre 0.5.0 had the option to specify `signal` attribute to prematurely cancel the request, which is now currently taken by the timeout mechanism.\n...\n> You should now be able to accomplish this like so: \n> \n> ```ts\n> const stream = client.completions.create({...params, stream: true});\n> \n> stream.controller.on('abort', myCallback); // listen for cancelation\n> \n> setTimeout(() => {\n> stream.controller.abort(); // can cancel the stream this way.\n> }, 1000)\n> \n> for await (const part of stream) {\n> if (shouldCancel()) break; // can also cancel the stream this way.\n> }\n> ```\n> \n> Does this work for you? Feedback welcome!\n...\n> This works for streaming, but adds a bit of complexity as we'd have to basically add a custom callback on our passed in `signal` and then call `stream.controller.abort()` within that to have the same interface. We'd also love to support aborts for non-streaming requests as well for consistency.\n> \n> Any way we could allow a custom signal to be passed through as before?\n> \n> Here's how the current implementation looks: https://github.com/hwchase17/langchainjs/blob/main/langchain/src/chat_models/anthropic.ts#L275\n...\n> Sure, here's how it looks end-to-end in our docs:\n> \n> https://js.langchain.com/docs/modules/models/chat/additional_functionality#cancelling-requests\n> \n> It's not the same thing as a timeout or a retry - one example is someone building a chat interface who wants to have a way to stop an outgoing generation. Not quite as important for non-streaming requests but would still be nice to have it there for consistency's sake.\n...\n> Thank you, makes a ton of sense. We plan to add this shortly like so:\n> \n> ```ts\n> const completion = await client.completions.create(params, { signal: controller.signal });\n> ```\n> \n> This would likely raise an `APIAbortError` or `APIUserAbortError` or similar. Feedback/suggestions on this interface welcome!\n\n---\n\nTitle: Streaming messages - Claude Platform Docs\nURL: https://platform.claude.com/docs/en/build-with-claude/streaming\nPublished: N/A\nAuthor: N/A\nHighlights:\nStream Messages API responses incrementally with server-sent events, including text, tool use, and extended thinking deltas.\n\n---\n\nTitle: UNPKG\nURL: https://app.unpkg.com/@anthropic-ai/sdk@0.78.0/files/core/streaming.mjs\nPublished: N/A\nAuthor: N/A\nHighlights:\nvar _Stream_client; import { __classPrivateFieldGet, __classPrivateFieldSet } from \"../internal/tslib.mjs\"; import { AnthropicError } from \"./error.mjs\"; import { makeReadableStream } from \"../internal/shims.mjs\"; import { findDoubleNewlineIndex, LineDecoder } from \"../internal/decoders/line.mjs\"; import { ReadableStreamToAsyncIterable } from \"../internal/shims.mjs\"; import { isAbortError } from \"../internal/errors.mjs\"; import { safeJSON } from \"../internal/utils/values.mjs\"; import { encodeUTF8 } from \"../internal/utils/bytes.mjs\"; import { loggerFor } from \"../internal/utils/log.mjs\"; import { APIError } from \"./error.mjs\"; export class Stream { constructor(iterator, controller, client) { this.iterator = iterator; _Stream_client.set(this, void 0); this.controller = controller; __classPrivateFieldSet(this, _Stream_client, client, \"f\"); } static fromSSEResponse(response, controller, client) { let consumed = false; const logger = client ? loggerFor(client) : console; async function* iterator() { if (consumed) { throw new AnthropicError('Cannot iterate over a consumed stream, use `.tee()` to split the stream.'); } consumed = true; let done = false; try { for await (const sse of _iterSSEMessages(response, controller)) { if (sse.event === 'completion') { try { yield JSON.parse(sse.data); } catch (e) { logger.error(`Could not parse message into JSON:`, sse.data); logger.error(`From chunk:`, sse.raw); throw e; } } if (sse.event === 'message_start' || sse.event === 'message_delta' || sse.event === 'message_stop' || sse.event === 'content_block_start' || sse.event === 'content_block_delta' || sse.event === 'content_block_stop') { try { yield JSON.parse(sse.data); } catch (e) { logger.error(`Could not parse message into JSON:`, sse.data); logger.error(`From chunk:`, sse.raw); throw e; } } if (sse.event === 'ping') { continue; } if (sse.event === 'error') { throw new APIError(undefined, safeJSON(sse.data) ?? sse.data, undefined, response.headers); } } done = true; } catch (e) { // If the user calls `stream.controller.abort()`, we should exit without throwing. if (isAbortError(e)) return; throw e; } finally { // If the user `break`s, abort the ongoing request. if (!done) controller.abort(); } } return new Stream(iterator, controller, client); } /** * Generates a Stream from a newline-separated ReadableStream * where each item is a JSON value. */ static fromReadableStream(readableStream, controller, client) { let consumed = false; async function* iterLines() { const lineDecoder = new LineDecoder(); const iter = ReadableStreamToAsyncIterable(readableStream); for await (const chunk of iter) { for (const line of lineDecoder.decode(chunk)) { yield line; } } for (const line of lineDecoder.flush()) { yield line; } } async function* iterator() { if (consumed) { throw new AnthropicError('Cannot iterate over a consumed stream, use `.tee()` to split the stream.'); } consumed = true; let done = false; try { for await (const line of iterLines()) { if (done) continue; if (line) yield JSON.parse(line); } done = true; } catch (e) { // If the user calls `stream.controller.abort()`, we should exit without throwing. if (isAbortError(e)) return; throw e; } finally { // If the user `break`s, abort the ongoing request. if (!done) controller.abort(); } } return new Stream(iterator, controller, client); } [(_Stream_client = new WeakMap(), Symbol.asyncIterator)]() { return this.iterator(); } /** * Splits the stream into two streams which can be * independently read from at different speeds. */ tee() { const left = []; const right = []; const iterator = this.iterator(); const teeIterator = (queue) => { return { next: () => { if (queue.length === 0) { const result = iterator.next(); left.push(result); right.push(result); } return queue.shift(); }, }; }; return [ new Stream(() => teeIterator(left), this.controller, __classPrivateFieldGet(this, _Stream_client, \"f\")), new Stream(() => teeIterator(right), this.controller, __classPrivateFieldGet(this, _Stream_client, \"f\")), ]; } /** * Converts this stream to a newline-separated ReadableStream of * JSON stringified values in the stream * which can be turned back into a Stream with `Stream.fromReadableStream()`. */ toReadableStream() { const self = this; let iter; return makeReadableStream({ async start() { iter = self[Symbol.asyncIterator](); }, async pull(ctrl) { try { const { value, done } = await iter.next(); if (done) return ctrl.close(); const bytes = encodeUTF8(JSON.stringify(value) + '\\n'); ctrl.enqueue(bytes); } catch (err) { ctrl.error(err); } }, async cancel() { await iter.return?.(); }, }); } } export async function* _iterSSEMessages(response, controller) { if (!response.body) { controller.abort(); if (typeof globalThis.navigator !== 'undefined' && globalThis.navigator.product === 'ReactNative') { throw new AnthropicError(`The default react-native fetch implementation does not support streaming. Please use expo/fetch: https://docs.expo.dev/versions/latest/sdk/expo/#expofetch-api`); } throw new AnthropicError(`Attempted to iterate over a response with no body`); } const sseDecoder = new SSEDecoder(); const lineDecoder = new LineDecoder(); const iter = ReadableStreamToAsyncIterable(response.body); for await (const sseChunk of iterSSEChunks(iter)) { for (const line of lineDecoder.decode(sseChunk)) { const sse = sseDecoder.decode(line); if (sse) yield sse; } } for (const line of lineDecoder.flush()) { const sse = sseDecoder.decode(line); if (sse) yield sse; } } /** * Given an async iterable iterator, iterates over it and yields full * SSE chunks, i.e. yields when a double new-line is encountered. */ async function* iterSSEChunks(iterator) { let data = new Uint8Array(); for await (const chunk of iterator) { if (chunk == null) { continue; } const binaryChunk = chunk instanceof ArrayBuffer ? new Uint8Array(chunk) : typeof chunk === 'string' ? encodeUTF8(chunk) : chunk; let newData = new Uint8Array(data.length + binaryChunk.length); newData.set(data); newData.set(binaryChunk, data.length); data = newData; let patternIndex; while ((patternIndex = findDoubleNewlineIndex(data)) !== -1) { yield data.slice(0, patternIndex); data = data.slice(patternIndex); } } if (data.length > 0) { yield data; } } class SSEDecoder { constructor() { this.event = null; this.data = []; this.chunks = []; } decode(line) { if (line.endsWith('\\r')) { line = line.substring(0, line.length - 1); } if (!line) { // empty line and we didn't previously encounter any messages if (!this.event && !this.data.length) return null; const sse = { event: this.event, data: this.data.join('\\n'), raw: this.chunks, }; this.event = null; this.data = []; this.chunks = []; return sse; } this.chunks.push(line); if (line.startsWith(':')) { return null; } let [fieldname, _, value] = partition(line, ':'); if (value.startsWith(' ')) { value = value.substring(1); } if (fieldname === 'event') { this.event = value; } else if (fieldname === 'data') { this.data.push(value); } return null; } } function partition(str, delimiter) { const index = str.indexOf(delimiter); if (index !== -1) { return [str.substring(0, index), delimiter, str.substring(index + delimiter.length)]; } return [str, '', '']; } //# sourceMappingURL=streaming.mjs.map\n\n---\n\nTitle: Streaming Claude to the Browser With Backpressure That Actually Works - DEV Community\nURL: https://dev.to/pavelespitia/streaming-claude-to-the-browser-with-backpressure-that-actually-works-4oaf\nPublished: 2026-06-24T15:05:21.000Z\nAuthor: N/A\nHighlights:\n## The part tutorials skip: the client aborts mid-stream\n...\nHere is the failure mode that does not show up in a demo. The user navigates away, or closes the tab, or their connection drops, while the model is still generating. On the server, your`for await` loop keeps pulling tokens from Claude, paying for output you are throwing into a closed pipe.\n...\nThe fix is to wire the request's abort signal through to the Claude stream so that when the client disconnects, you stop generating:\n...\n```\nexport async function POST(request: Request) {\n  const { prompt } = await request.json();\n\n  const stream = new ReadableStream({\n    async start(controller) {\n      const llm = client.messages.stream(\n        {\n          model: \"claude-opus-4-8\",\n          max_tokens: 64000,\n          messages: [{ role: \"user\", content: prompt }],\n        },\n        { signal: request.signal }, // abort the SDK stream when the request aborts\n      );\n\n      request.signal.addEventListener(\"abort\", () => {\n        llm.abort();       // stop pulling tokens\n        controller.close();\n      });\n\n      // ... same loop as above\n    },\n  });\n  // ...\n}\n\n```\n...\nNow a disconnected client stops the generation, which stops the bill. On a fast model producing 64K of output, an abandoned stream you keep generating is real money.\n...\n```\nasync function streamCompletion(prompt: string, onToken: (t: string) => void) {\n  const controller = new AbortController();\n  const res = await fetch(\"/api/stream\", {\n    method: \"POST\",\n    body: JSON.stringify({ prompt }),\n    signal: controller.signal,\n  });\n\n  const reader = res.body!.getReader();\n  const decoder = new TextDecoder();\n  let buffer = \"\";\n\n  while (true) {\n    const { done, value } = await reader.read();\n    if (done) break;\n    buffer += decoder.decode(value, { stream: true });\n\n    const events = buffer.split(\"\\n\\n\");\n    buffer = events.pop() ?? \"\"; // keep the incomplete tail\n\n    for (const evt of events) {\n      const line = evt.split(\"\\n\").find((l) => l.startsWith(\"data: \"));\n      if (!line) continue;\n      const data = JSON.parse(line.slice(6));\n      if (data.text) onToken(data.text);\n      if (data.error) throw new Error(data.error);\n    }\n  }\n\n  return controller; // hold this so the UI can abort on unmount\n}\n...\nReturn the`AbortController` so a React component can call`controller.abort()` in its cleanup function. That is what propagates the abort all the way back to the server and stops the generation.\n...\nThe demo version of streaming works because nobody closes the tab and the network is perfect. Production is not that. The two things that separate a real implementation from a tutorial: disable proxy buffering so tokens actually flow, and propagate aborts end to end so an abandoned stream stops costing you money. Get those two right and streaming is genuinely robust. Skip them and it works right up until it matters.\n\n---\n\nTitle: [BUG] AbortController not respected · Issue #2970 · anthropics/claude-code\nURL: https://github.com/anthropics/claude-code/issues/2970\nPublished: 2025-07-04T06:26:27.000Z\nAuthor: jacksondc\nHighlights:\nThe SDK does not respect AbortController signals during query execution. When `controller.abort()` is called, the SDK continues processing and executing all queued tool calls to completion instead of stopping immediately.\n...\n// Abort after\n...\n>>> ABORT\n...\norted}`);\n...\nlog(`>>>\n...\nWhen `controller.abort()` is called, the query should stop immediately and throw an AbortError:\n...\n>>> ABORTING at message 2 <<<\n>>> controller.signal.aborted before: false\n>>> controller.signal.aborted after: true\n>>> Expecting query to stop soon...\n\nQuery threw error: The operation was aborted\nError name: AbortError\n```\n...\nThe SDK continues executing all tools and completes the full query despite the abort signal:\n...\n>>> ABORTING at message 2 <<<\n>>> controller.signal.aborted before:\n  false\n>>> controller.signal.aborted after:\n  true\n...\n>>> Expecting query to stop soon...\n\nMessage 3: assistant\n...\n4: assistant\n...\n6: user\n...\n7: user\n...\n8: user\n...\n9: user\n...\nMessage 10: assistant\nMessage 11: result\n\nQuery completed. Total messages: 11\nAborted at message: 2\nMessages after abort: 9\n```\n...\n> Okay, I think this is only an issue with passing abortController as a property like `query({ abortController })`. Turns out you can also pass it like `query({ options: { abortController }})`, which does work.\n\n---\n\nTitle: TypeScript SDK — Claude API Docs\nURL: https://doc.jarvisuni.com/claude/api/en/api/sdks/typescript.html\nPublished: N/A\nAuthor: N/A\nHighlights:\n## Streaming responses\n...\nThe SDK provides support for streaming responses using Server Sent Events (SSE).\n...\nIf you need to cancel a stream, you can`break` from the loop or call`stream.controller.abort()`."
/allow 555e114c-47f3-4e48-b352-1ea50ac3b4f0
#:ReasoningToken: 178
Kurze Antwort: **Nein – es gibt kein spezielles „Stop-Token“, das man während der Verarbeitung an das Modell selbst schickt.** Der Abbruch passiert ausschließlich auf **Transport-/Verbindungsebene**, nicht als Teil des Inferenz-Vokabulars. Hier die Details:

## 1. Was es *nicht* gibt
- Es gibt **kein** Steuerzeichen/Token, das man mitten in den generierten Token-Stream einspeist und das das Modell dazu bringt, selbst aufzuhören (kein "magic stop word" zur Laufzeit).
- `stop_sequences` in der Messages-API sind zwar Strings, bei denen das Modell aufhört zu generieren – aber die werden **vorher** in der Anfrage festgelegt, nicht nachträglich während der Inferenz "geschickt". Das Modell muss diese Sequenz selbst erzeugen, damit es greift; du kannst sie nicht von außen "injizieren".

## 2. Was tatsächlich passiert (der einzige echte Mechanismus)
Der Abbruch läuft immer über einen **`AbortController` / Connection-Abort**:

- Client (Claude Code, SDK, Browser) hält eine `AbortController`-Instanz.
- Bei Esc/Ctrl+C bzw. `stream.controller.abort()` wird das zugrunde liegende HTTP/SSE-Request-Objekt gekillt.
- Der Server merkt an der **abgebrochenen TCP-Verbindung**, dass niemand mehr liest, und stellt (im Idealfall) das Generieren weiterer Tokens ein.
- Es wird **kein Payload/Token an die API gesendet**, das "Stop!" bedeutet – es ist ein reiner Verbindungsabbruch (vergleichbar mit einem geschlossenen Socket).

```ts
const stream = client.messages.stream({...}, { signal: controller.signal });
controller.abort(); // kappt die Verbindung, keine Nachricht ans Modell
```

## 3. Wichtige Nuancen
- **Billing:** Du zahlst nur für die bis zum Abbruch bereits generierten Output-Tokens (der letzte `usage.output_tokens`-Wert aus `message_delta`), nicht für den Rest.
- **Kein Server-seitiges "Cancel"-Kommando:** Es gibt keinen dokumentierten API-Endpoint wie `POST /v1/messages/{id}/cancel`. Die einzige Möglichkeit ist der Verbindungsabbruch.
- **Bekannte Lücken:** In Claude Code gibt es dokumentierte Bugs (#2970, #26802), bei denen `abort()` nicht sofort respektiert wird – z. B. laufen bereits gestartete parallele Tool-Calls trotzdem weiter, oder die SDK verarbeitet nach dem Abort noch alle gequeuten Nachrichten, bevor sie wirklich stoppt.
- **Agent-Loop-Ebene:** Wenn du einen eigenen Agentic Loop baust (Tool-Use-Schleife), musst du selbst dafür sorgen, dass nach einem Abort keine neuen API-Requests mehr abgeschickt werden (`if (controller.signal.aborted) return;`), sonst "resumed" die Schleife trotzdem automatisch.

## Fazit
Ein Stop erzwingst du also **nicht durch ein Token im Kontext**, sondern durch **Abbruch der HTTP-Verbindung** (`AbortController.abort()` bzw. Prozess-Signal bei Esc in Claude Code). Ein echtes "Stop"-Signal *innerhalb* des laufenden Inferenz-Streams als Protokoll-Feature existiert bei der Anthropic API nicht.