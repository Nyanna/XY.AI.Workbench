# Claude-Code CLI Connector

Implementiere auf Basis des vorhandenen Interfaces einen Connector, der das Claude-Code-CLI als Backend verwendet. 

## Kommandozeile und Prozess

Bei erstmaligem Aufruf soll ein Subprozess gestartet werden, der permanent im Hintergrund bleibt.
STDIN und STDOUT werden dauerhaft gehalten.
Ein erneuter Aufruf verwendet den bereits laufenden Prozess.

* Skriptdatei: `${HOME}/xyan/xy.ai.workbench/claude-code/claude-session.sh'
* Der Prozess stirbt mit der Beendigung von Eclipse
* Das Arbeitsverzeichnis ist das Root des Eclipse Projektes der Editordatei, die den Prozess erstmalig gestartet hat.
* Outputspiegelung findet nur statt, solange der Kontrollfluss im Connector liegt

### Parameter

* `--profile personal`
* `--agent default`
* `--verbose`
* `--include-hook-events`
* `--include-partial-messages`
* `--input-format stream-json`
* `--output-format stream-json`
* `--replay-user-messages`
* `--effort` wird aus den übergebenen Parametern übernommen

## Kontrollfluss

1. Der Connector erhält einen Prompt
2. Startet, falls notwendig, den Subprozess und übergibt den Prompt
3. Hört STDOUT ab und spiegelt die Ausgabe inklusive des Result-Events
4. Übergibt das Ergebnis, die Metriken und den Kontrollfluss zurück.

* Weitere Ausgaben auf ATDOUT werden erst in der nächsten Schleife gelesen

## Referenzen

* Java Project Source Root: `/home/user/xyan/xy.ai.workbench/src/`
* Andere Connector Implementierung für die Claude-API: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claude/ClaudeConnector.java`

## Spiegelung der Ausgabe

Der STDOUT wird permanent parallel in eine JSON Datei geschrieben die neben der vom Prompt verursachten geöffneten Datei im Projekt liegt.
Die Ausgabedatei hat den gleichen Namen und Ort, endet aber auf `.json`.

## Input Support

Die Implementierung muss nur den Standard-Prompt Input unterstützen.

```json
{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Deine Frage"}]}}

## Output Support

Die Implementierung gibt nur das Result Event zurück und die Token-Metriken.

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
