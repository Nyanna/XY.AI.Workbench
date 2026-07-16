Agenten habe Probleme mit meinem MCP Server im Fall von API-Fehlern. Obwohl, ich der Spec entsprechend reagiere ignorieren die Agenten die Anweisung im Fehlertext.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=5899f2dd-d5f2-407c-9073-56140a755b2f | model=claude-sonnet-5 | plugins=
Kurze Einordnung, bevor ich dir konkrete Lösungsvorschläge gebe: Ich habe gerade keinen direkten Einblick in deinen MCP-Server, dessen Code oder die genauen Fehlerantworten, die du sendest – ich kann also nicht "von außen" sehen, was da schiefläuft. Aber das Problem, das du beschreibst, ist bekannt und hat meistens einen von wenigen typischen Gründen. Lass uns das systematisch durchgehen.

## Was du mir noch zeigen solltest

Damit ich konkret helfen kann, wäre es hilfreich, wenn du mir zeigst:

1. Ein Beispiel eines Fehler-Response-Payloads (JSON), das du bei einem API-Fehler zurückgibst
2. Ob du den Fehler als MCP `isError: true` Content zurückgibst, als JSON-RPC-Error-Objekt, oder als Text im normalen Result
3. Welchen Agenten/Client du testest (Claude Desktop, Claude Code, eigener Agent-Loop, etc.)

## Häufigste Ursachen, wenn Agenten Fehleranweisungen ignorieren

**1. Fehler wird als JSON-RPC Protocol-Error statt als Tool-Result gesendet**
Wenn du einen echten JSON-RPC-Error (`error` Feld auf Top-Level) zurückgibst, sieht das Modell den Text oft gar nicht als "Content" – der Client behandelt es als technischen Fehler und zeigt dem Modell nur eine generische Fehlermeldung, nicht deinen Instruktionstext. Besser: Tool-Ergebnis mit `isError: true` und dem Text im `content`-Array zurückgeben – dann sieht das Modell den vollen Text und kann reagieren.

```json
{
  "content": [
    { "type": "text", "text": "API-Fehler: Rate limit erreicht. Bitte warte 30 Sekunden und versuche es dann mit reduziertem Batch-Size erneut." }
  ],
  "isError": true
}
```

**2. Anweisung ist nicht imperativ/eindeutig genug formuliert**
Modelle reagieren besser auf klare, handlungsorientierte Sprache statt auf beschreibende Fehlermeldungen. Vergleich:

- Schlecht: `"Error: rate limited"`
- Besser: `"RATE_LIMITED: Do not retry immediately. Wait at least 30s before calling this tool again. If this is the second failure, ask the user before retrying."`

**3. Kein strukturiertes Fehler-Schema, das man verlässlich parsen/befolgen kann**
Wenn jeder Fehler anders aussieht, "lernt" das Modell im Kontext nicht zuverlässig, wie es reagieren soll. Ein konsistentes Format (z. B. immer `ERROR_CODE: message + instruction`) hilft, weil das Modell darauf trainiert reagiert, Muster zu erkennen.

**4. Anweisung steht in Metadata statt im sichtbaren Content**
Manche Server packen Handlungsanweisungen in `_meta` oder strukturierte Felder, die das Modell nie zu Gesicht bekommt, weil der Client sie nicht in den Prompt einspeist. Alles, was das Modell befolgen soll, muss im `text`-Content stehen.

**5. System-/Tool-Prompt des Agenten übertrumpft deine Instruktion**
Wenn der Agent bereits eine harte Regel hat wie "bei Fehlern immer sofort neu versuchen" oder "gib bei Fehlern auf", kann das deine im Fehlertext gegebene Anweisung überschreiben – Systemebene schlägt oft Tool-Response-Ebene.

## Konkreter Vorschlag

Zeig mir bitte:
- Ein reales Beispiel deiner Fehlerantwort (Rohformat)
- Falls vorhanden, deine Server-Spezifikation/Doku dazu, wie Fehler strukturiert sein sollen

Dann kann ich dir sagen, ob das Format selbst das Problem ist, oder ob es eher an der Formulierung/Platzierung der Instruktion liegt.
Token Usage: total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
