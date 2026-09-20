# Workbench Prompt Syntax — User Guide

## 1. Labels (prefix lines that segment content)

A **label** is a line prefix that marks the start of a segment. It separates content into roles and content types. The following labels are recognized:

| Label (exact) | Segment type | Meaning |
|---|---|---|
| `User:` | User turn | Everything following belongs to the user request. Starts a user role segment. |
| `Agent:` | Agent turn | Everything following belongs to the assistant/agent response. Starts an agent role segment. |
| `Text:` | Agent text | The actual answer text produced by the agent. |
| `Thinking:` | Reasoning | The agent's reasoning/thinking text. |
| `Thinking Meta:` | Reasoning metadata | Metadata block inside the thinking segment. |
| `Tool:` | Tool call | A tool invocation made by the agent. |
| `ToolResult:` | Tool result | The result returned by a tool. |
| `Control Request:` | Control block | Introduces a control/approval request; the following fenced code block is skipped during processing. |
| `Result Stats: ` | Result metadata | Final result statistics. Format: `Result Stats: id=<session-id>, total: <n>, in: <n>, out: <n>, reason: <n>, read: <n>, write: <n>` |
| `SystemInit: ` | Session metadata | Session startup info. Format: `SystemInit: id=<session-id>, cwd=<directory>, model=<model>` |
| `ReasoningToken: ` | Reasoning token count | Number of reasoning tokens. |
| `Token Usage: ` | Token usage | Token usage information. |
| `[xy.ai.req:<provider>:<request-id>]` | Prompt marker | Generated placeholder for a request. `<provider>` is one of: `OpenAI`, `Gemini`, `Claude`, `Deepseek`, `None`, `ClaudeCode`, `Misc`. |

> Note: The labels with a trailing space (`Result Stats: `, `SystemInit: `, `ReasoningToken: `, `Token Usage: `) include that space in their exact form. All other labels use a colon without a trailing space.

---

## 2. Slash commands

Slash commands are recognized lines that control processing. They are not sent to the model as normal text; they are either removed or replaced before sending.

| Command | Syntax | Effect |
|---|---|---|
| `/answer` | `/answer <id> allow\|deny [reason]` | Approves or denies a control request. |
| `/resume` | `/resume <session-id>` | Resumes a CC session |
| `/exit` | `/exit` | Marks session exit. |
| `/tool` | `/tool <tool-id>` | References a tool. Creates a tool call template|
| `/call` | `/call ...` | Executes a tool by MCPC |
| (YAML block) | ```` ```yaml ... ``` ```` | Can be used for control messages and MCP to alter or execute tool calls |

---

## 3. Include directives

Include directives insert external content into the prompt. They are recognized only when an entire line consists of one or more include directives (whitespace around them is allowed).

**Syntax**

```
[include](<file-path>)
[include <kind>](<argument>)
```

Multiple directives may appear on the same line; they are processed left to right.

| Directive | Argument | Behavior |
|---|---|---|
| `[include](<file-path>)` | File path | Reads the file, resolves it to an absolute path, and processes its content as if it had been entered directly. Nested includes are supported; circular includes are detected and rejected. |
| `[include contextprompt](<directory>)` | Directory | Inserts the context prompt provided for the directory as a user message. |
| `[include files](<argument>)` | File pattern/argument | Inserts in Eclipse selected files as tool results. |
| `[include file](<path>)` | File path | Inserts a single file as a tool result. |
| `[include search](files)` | `files` | Inserts the files found by the search as tool results. |
| `[include search](matches)` | `matches` | Inserts the search matches as a YAML list with `file`, `line`, and `text` fields. |

**Notes**

- A path include without a kind (`[include](...)`) reads and processes the target file recursively, including any further include directives inside it.
- Unknown include kinds or unknown search arguments are rejected with an error.