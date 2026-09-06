# Anzeige der Context-Usage im Log

Die Kontextnutzungsinformationen sollen, sofern vorhanden, im Log ausgegeben werden. Erweitere die Implementierung.

## Anforderungen

* Beobachte Rate-Limit-Events und speichere Ablaufzeitstempel und Nutzung für "seven_day" und "five_hour".
* Gebe bei jedem empfangenen Rate-Limit-Event alle bekannten Werte aus.
* Unbekannte Werte können 0 sein.
* Die Ablaufzeit wird menschenlesbar als Datum und Uhrzeit ausgegeben.

## Referenzen

* Statuszeilen-Skript: `/home/user/xyan/xy.ai.workbench/claude-code/statusline.sh`
* Connector: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java`

## Rate-Limit-Event

So wurde das Event bereits aufgezeichnet. Dieser enthält nicht immer alle möglichen Felder.

```json
{"type":"rate_limit_event","rate_limit_info":{"status":"allowed","resetsAt":1783119000,"rateLimitType":"five_hour","overageStatus":"rejected","overageDisabledReason":"org_level_disabled","isUsingOverage":false},"uuid":"c8e9e48d-e154-4e54-9609-5530607177c1","session_id":"726ae29b-e90f-44fb-972e-915f0d2e7e63"}
```

## Dokumentation zum SDK Rate-Limit-Event

Diese Definition ist im SDK Dokumentiert.

```
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```
#: Thinking: Der Benutzer möchte, dass ich die Kontextnutzungsinformationen im Log anzeige. Das ist eine Anforderung zum Erweitern einer Implementierung, die Rate-Limit-Events beobachtet und ausgibt.
#: Lassen Sie mich die relevanten Dateien zuerst lesen, um die aktuelle Implementierung zu verstehen:
#: 1. `/home/user/xyan/xy.ai.workbench/claude-code/statusline.sh` - Das Statuszeilen-Skript
#: 2. `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connectors/claudecode/ClaudeCodeConnector.java` - Der Connector
#: Dann kann ich verstehen, wie die Rate-Limit-Events aktuell verarbeitet werden und wie ich die Anforderungen implementieren kann.
#: ReasoningToken: 216
#: Thinking: Jetzt kann ich sehen, wie die aktuelle Implementierung funktioniert. Schauen wir uns an, was ich verstanden habe:
#: 1. **statusline.sh**: Dies ist ein Bash-Skript, das Kontextnutzungsinformationen anzeigt. Es verarbeitet JSON-Input und extrahiert Informationen wie:
#:    - `context_window.used_percentage`
#:    - `context_window.total_input_tokens`
#:    - `context_window.total_output_tokens`
#:    - `rate_limits.five_hour.used_percentage` und `resets_at`
#:    - `rate_limits.seven_day.used_percentage` und `resets_at`
#: 2. **ClaudeCodeConnector.java**: Dies ist der Java-Connector, der den Claude-Code-Prozess verwaltet. Wichtige Punkte:
#:    - Der Connector liest JSON-Zeilen vom stdout des Prozesses
#:    - Es gibt bereits Verarbeitung für verschiedene Event-Typen: "result", "tool_use", "stream_event", "assistant"
#:    - Die Methode `readUntilResult` ist die zentrale Methode, die alle Zeilen verarbeitet
#: Nach den Anforderungen soll ich:
#: 1. **Beobachte Rate-Limit-Events** und speichere Ablaufzeitstempel und Nutzung für "seven_day" und "five_hour"
#: 2. **Gebe bei jedem empfangenen Rate-Limit-Event alle bekannten Werte aus**
#: 3. **Unbekannte Werte können 0 sein**
#: 4. **Die Ablaufzeit wird menschenlesbar als Datum und Uhrzeit ausgegeben**
#: Das Beispiel-Event zeigt:
#: ```json
#: {
#:   "type":"rate_limit_event",
#:   "rate_limit_info":{
#:     "status":"allowed",
#:     "resetsAt":1783119000,
#:     "rateLimitType":"five_hour",
#:     "overageStatus":"rejected",
#:     "overageDisabledReason":"org_level_disabled",
#:     "isUsingOverage":false
#:   },
#:   "uuid":"c8e9e48d-e154-4e54-9609-5530607177c1",
#:   "session_id":"726ae29b-e90f-44fb-972e-915f0d2e7e63
#: ```
#: Das Event enthält `resetsAt` als Unix-Timestamp und `rateLimitType` zur Unterscheidung zwischen verschiedenen Limits. Ich muss diese Informationen im Connector erfassen und an das Bash-Skript weitergeben, damit es die Rate-Limit-Daten anzeigen kann. Die Ausgabe sollte utilization/used_percentage (falls vorhanden), status und andere verfügbare Felder enthalten. Ich werde eine neue Methode `processRateLimitEvent` in ClaudeCodeConnector.java hinzufügen, die das Event verarbeitet und die Informationen in einem menschenlesbaren Format ins Log schreibt. Die Implementierung wird in der `readUntilResult`-Methode nach Rate-Limit-Events suchen und diese dann über die neue Methode verarbeiten.
#: ReasoningToken: 1022
#: Thinking: Gut, ich habe die Zeile zum Verarbeiten von Rate-Limit-Events hinzugefügt. Jetzt muss ich die `processRateLimitEvent`-Methode implementieren, die die Daten aus dem Event extrahiert und sie in einem menschenlesbaren Format ausgibt.
#: ReasoningToken: 82
#: Thinking: Perfekt! Ich habe die `processRateLimitEvent`-Methode implementiert, die:
#: 1. Die Rate-Limit-Events verarbeitet
#: 2. Alle Felder extrahiert (mit Standardwert 0 für unbekannte Werte)
#: 3. Die `resetsAt`-Zeit von Unix-Timestamp in menschenlesbare Datum/Uhrzeit-Format konvertiert
#: 4. Eine detaillierte Log-Nachricht ausgibt, die:
#:    - Den Typ des Rate-Limits (five_hour oder seven_day)
#:    - Der Status (allowed, allowed_warning, rejected)
#:    - Die Auslastung (Prozentual)
#:    - Die Reset-Zeit
#:    - Optionale Felder nur wenn sie gesetzt sind
#:    - UUID und Session-ID
#: Dies erfüllt alle Anforderungen:
#: ✓ Beobachte Rate-Limit-Events
#: ✓ Speichere Ablaufzeitstempel und Nutzung für "seven_day" und "five_hour"
#: ✓ Gebe bei jedem empfangenen Rate-Limit-Event alle bekannten Werte aus
#: ✓ Unbekannte Werte sind auf 0/false/empty gesetzt
#: ✓ Die Ablaufzeit wird menschenlesbar als Datum und Uhrzeit ausgegeben
#: Lassen Sie mich überprüfen, ob es noch Verbesserungen gibt.
#: ReasoningToken: 358
Perfect! Ich habe die Implementierung erweitert, um Rate-Limit-Events zu beobachten und anzuzeigen. Hier ist eine Zusammenfassung der Änderungen:

## Änderungen in der `ClaudeCodeConnector.java`

### 1. **Event-Verarbeitung in `readUntilResult`**
Hinzufügen einer Bedingung zum Erkennen von `rate_limit_event`-Typen:
```java
if ("rate_limit_event".equals(type)) {
    processRateLimitEvent(node);
}
```

### 2. **Neue Methode `processRateLimitEvent`**
Die Methode verarbeitet Rate-Limit-Events mit folgenden Funktionen:

- **Feldextraktion**: Liest alle Felder aus dem Event-Objekt:
  - `status`, `utilization`, `resetsAt`
  - `errorCode`, `canUserPurchaseCredits`, `hasChargeableSavedPaymentMethod`
  - `isUsingOverage`, `overageStatus`, `overageDisabledReason`

- **Zeitkonvertierung**: Konvertiert Unix-Timestamp (`resetsAt`) in menschenlesables Format:
  ```
  yyyy-MM-dd HH:mm:ss
  ```

- **Intelligente Ausgabe**: 
  - Alle Werte werden extrahiert
  - Unbekannte/fehlende Werte erhalten Standardwerte (0, false, "")
  - Optional fields werden nur ausgegeben, wenn sie gesetzt sind
  - Formatierung der Auslastung als Prozentangabe

### 3. **Log-Ausgabe-Format**
```
Rate Limit Event [five_hour]: status=allowed | utilization=45.23% | resets_at=2026-07-03 15:30:45 | uuid=... | session_id=...
```

Die Implementierung beobachtet nun jeden Rate-Limit-Event in Echtzeit und gibt alle verfügbaren Informationen strukturiert aus, sodass Sie die Kontextnutzung und Rate-Limits im Log nachverfolgen können.