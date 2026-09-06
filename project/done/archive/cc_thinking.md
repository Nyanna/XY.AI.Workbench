# Claude-Code-CLI-Tool Thinking Support

Implementiere die Ausgabe von thinking im Claude-Code-CLI-Connector.

## Referenzen

* Java Project Source Root: `/home/user/xyan/xy.ai.workbench/src/`
* Claude-Code-CLI-Connector: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java`

## Anforderungen

Assistant-Events vom Typ "text" und "thinking" sollen sichtbar gemacht werden.

* Während der Connector auf das Result wartet, soll er zusätzliche Nachrichtentypen beobachten.
* Im Fall von "text" und "thinking" sollen diese gespeichert werden. 
* Die Speicherung soll in einer geordneten Hash-Struktur geschehen, um sowohl die Reihenfolge zu erhalten als auch Inhaltsduplikate zu vermeiden.
* Die gespeicherten Events sollen der finalen Antwort als Zeilen im Markdown-Format vorrangestellt werden.

```
_Thinking: <Text>
_Text: <Text>
```

## Prompt Kommando Unterstützung

Der eingehende Prompttext wird verarbeitet und erhält ein Preprocessing.

* Ein einzeln in einer Zeile stehendes "/exit" wird aus dem Input entfernt und ein Exit-Flag gesetzt.
* Ist nach Entfernung der Kommando-Zeile "/exit" und Trimming noch weiterer Text übrig, wird dieser als normaler Prompt weiterverarbeitet und in den STDIN gegeben.
* Wurde das Exit-Flag gesetzt, wird Folgendes getan:
	1. Es gab weiteren Text. Dieser wird normal als Prompt an den STDIN gegeben und auf das Result wird gewartet. Anschließend wird der offene Subprozess beendet.
	2. Es gab nur das "Exit"-Kommando dann muss nicht auf ein Result gewartet werden und der Subprozess kann gleich beendet werden.
* In beiden Fällen wird eine Antwort mit dem Kontrollfluss zurückgegeben.

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
Some Claude Code versions use `"text"` instead of `"thinking"` for the content field. Always check both:

