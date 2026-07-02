# Claude-Code-CLI-Tool Permission Support

Implementiere die Unterstützung für Toolnutzung und Approval in den Connector.

## Referenzen

* Java Project Source Root: `/home/user/xyan/xy.ai.workbench/src/`
* Claude-Code-CLI-Connector: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java`

## Anforderungen

Bei der bestehenden Observierung des STDIN auf einen Tool Request achten.

* Ein eingehender Tool-Request beendet die Beobachtung des Streams.
* Das Request wird interpretiert und in einen formatierten Markdown-Text konvertiert.
* Der Kontrollfluss wird mit dieser Antwort zurückgegeben.

## Tool Request Markdown Formatierung

```markdown
Tool: Bash
Input: `ls -ahl`
ID: 234234
```

## Eingehende Prompt Interpretation

Der eingehende Prompttext wird verarbeitet und erhält ein Preprocessing.

* Ein einzeln in einer Zeile stehendes "Deny 234234" wird aus dem Input entfernt.
* Anschließend in ein Tool-Request-Deny-JSON in den STDIN geschrieben
* Ist nach Entfernung der Deny-Zeile "Deny 234234" und Trimming noch weiterer Text übrig, wird dieser als normaler Prompt weiterverarbeitet und in den STDIN gegeben.
* Ebenso wird ein einzeln in einer Zeile vorkommendes "Allow <ID>" aus dem Input entfernt und als Approval behandelt.

## `tool_use` — Tool Request

```json
{
  "type": "tool_use",
  "id": "toolu_abc123",
  "name": "Read",
  "input": { "file_path": "/src/main.ts" }
}
```

The `id` field uniquely identifies this tool call — used for approval/denial and matching with `tool_result`.

## Tool Approval

Approve a pending tool execution. Send this after receiving a `tool_use` event.

```json
{"type":"approve","tool_use_id":"toolu_abc123"}
```

| Field | Type | Description |
|---|---|---|
| `type` | `"approve"` | Approval type |
| `tool_use_id` | `string` | The `id` from the `tool_use` content block |

## Tool Denial

Deny a pending tool execution.

```json
{"type":"deny","tool_use_id":"toolu_abc123"}
```

## Tool Approval Flow Example

Interactive tool approval via stdin messages example.

```typescript
const pendingApprovals = new Map<string, ToolUseEvent>()

rl.on('line', (line) => {
  const event = parseLine(line)
  if (!event) return

  for (const ev of translator.translate(event)) {
    if (ev.type === 'tool_use') {
      // Auto-approve Read/Grep, require manual approval for others
      if (['Read', 'Grep', 'Glob'].includes(ev.toolName)) {
        claude.stdin!.write(createMessage.approve(ev.toolUseId))
      } else {
        pendingApprovals.set(ev.toolUseId, ev)
        console.log(`Approve ${ev.toolName}? (tool_use_id: ${ev.toolUseId})`)
      }
    }
  }
})

// Later, when user approves/denies:
function handleUserDecision(toolUseId: string, approved: boolean) {
  if (approved) {
    claude.stdin!.write(createMessage.approve(toolUseId))
  } else {
    claude.stdin!.write(createMessage.deny(toolUseId))
  }
  pendingApprovals.delete(toolUseId)
}
```