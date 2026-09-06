# Result Nachverarbeitung

Implementiere in der Java-Klasse `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java` eine Nachverarbeitung des Result-Events.

* Die Funktionalität soll in einer separaten Klasse untergebracht werden.
* Die Nachverarbeitung findet auf dem Textinhalt des Result-Events statt.
* Die Textausgabe soll von redundanten Informationen befreit werden.

## Aktuell Format

Erkenne das aktuelle Format und extrahiere die Hook-Ausgabe.

```text
UserPromptSubmit operation blocked by hook:
[${CLAUDE_PLUGIN_ROOT}/scripts/spell-check.sh]: 
<hook ausgabe>


Original prompt: <original prompt>
```

## Ziel Format

Die Ausgabe soll auf den reinen Hook-Output begrenzt sein.


```text 
<hook ausgabe>
```
Die Änderungen sind vollständig. Hier eine Zusammenfassung:

---

## Was wurde implementiert

### Neue Klasse: `ResultPostProcessor`

Die gesamte Logik wurde in einer eigenständigen Klasse `ResultPostProcessor` im selben Package untergebracht.

**Methode `process(String text)`** — öffentlicher Einstiegspunkt, ruft alle Nachverarbeitungsschritte auf (aktuell `extractHookOutput`). So können künftig weitere Schritte einfach ergänzt werden.

**Methode `extractHookOutput(String text)`** — erkennt das Block-Muster in drei Schritten:

| Schritt | Erkennung |
|---|---|
| 1 | Zeile, die mit `UserPromptSubmit operation blocked by hook:` beginnt |
| 2 | Direkt folgende Nicht-Leer-Zeile, die mit `[` beginnt und auf `]:` endet (Hook-Script-Bezeichner) |
| 3 | Zeile, die mit `Original prompt:` beginnt (Endmarkierung) |

Der Text zwischen Script-Bezeichner und `Original prompt:` wird als Hook-Output extrahiert und — mit gestrippten führenden/nachfolgenden Leerzeilen — zurückgegeben. Passt das Muster nicht, bleibt der Text unverändert.

### Integration in `ClaudeCodeConnector`

- Eine Instanz `resultPostProcessor` wird als `final`-Feld gehalten (zustandslos, threadsicher).
- In `parseResult(...)` wird `node.path("result").asText("")` direkt durch `resultPostProcessor.process(...)` gewrappt — minimaler Eingriff an genau einer Stelle.
/exit
Session closed!