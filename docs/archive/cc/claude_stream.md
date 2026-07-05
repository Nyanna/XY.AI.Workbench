## Stream
Sources: Official Claude Code docs (code.claude.com), GitHub issues #24594, #44911, #41230, and the community-maintained claude-code-parser documentation.

## `--input-format stream-json` in Claude Code

### Format
**Newline-delimited JSON (NDJSON)** – ein vollständiges JSON-Objekt pro Zeile:
```json
{"type":"user","message":{"role":"user","content":[{"type":"text","text":"Deine Frage"}]}}
```

### Muss STDIN offen bleiben?

**Ja.** STDIN muss während der gesamten Session offen bleiben. Die CLI:
- liest zeilenweise und verarbeitet jede JSON-Zeile sobald sie vollständig ist
- wartet auf **EOF als Abschlusssignal** – erst dann beendet sie sich sauber
- unterstützt **mehrere aufeinanderfolgende Nachrichten** (Multi-Turn), solange STDIN offen ist

### Wichtige Details

| Aspekt                  | Verhalten                                                                                |
| ----------------------- | ---------------------------------------------------------------------------------------- |
| Abschluss pro Nachricht | Kein explizites Signal – nur `\n` nach der JSON-Zeile                                    |
| Session-Ende            | STDIN schließen → CLI beendet sich nach letzter Antwort                                  |
| Pflicht-Flags           | `--input-format stream-json` erfordert `-p` + `--output-format stream-json`              |
| Pufferung               | CLI puffert stdin; Nachrichten >~146 KB können in manchen Versionen abgeschnitten werden |

### Typischer Ablauf
```
STDIN offen ──► Zeile 1 senden ──► Antwort lesen ──► Zeile 2 senden ──► ... ──► STDIN schließen (EOF) ──► CLI exit
```

**Kurzfassung:** STDIN muss offen bleiben, solange du Nachrichten senden willst. Das Schließen von STDIN ist das Signal zum geordneten Beenden – kein separater "done"-Marker existiert.
**Nein** – der Umfang ist deutlich kleiner als bei der direkten Claude-API. `stream-json` ist ein **eigenes, minimales Protokoll**, kein Wrapper um die Messages API.

### Unterstützte Input-Types

| Type                                                           | Zweck                  |
| -------------------------------------------------------------- | ---------------------- |
| `{"type":"user", "message":{...}}`                             | Textnachricht senden   |
| `{"type":"approve", "tool_use_id":"..."}`                      | Tool-Aufruf genehmigen |
| `{"type":"deny", "tool_use_id":"..."}`                         | Tool-Aufruf ablehnen   |
| `{"type":"tool_result", "tool_use_id":"...", "content":"..."}` | Tool-Ergebnis liefern  |

### Was **nicht** per stream-json geht (vs. direkte API)

- ❌ System-Prompt dynamisch setzen
- ❌ Bilder / multimodale Inhalte
- ❌ Eigene Tool-Definitionen
- ❌ `temperature`, `max_tokens` oder Modell per Request wählen
- ❌ Mid-Session Konfigurationsänderungen

### Was stattdessen geht

Diese Parameter werden **einmalig beim Start** über CLI-Flags gesetzt:
```bash
claude -p \
  --input-format stream-json \
  --output-format stream-json \
  --model claude-opus-4-5 \
  --append-system-prompt "Du bist ..." \
  --allowedTools "Bash,Read,Write"
```

### Fazit

`stream-json` ist ein schlankes Steuerungsprotokoll für **Turn-basierte Interaktion und Tool-Approval** – nicht ein vollwertiger API-Ersatz. Wer volle Kontrolle über Model-Parameter, Vision oder eigene Tools braucht, muss direkt die Anthropic Messages API ansprechen.
