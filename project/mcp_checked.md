Ich möchte den Prefill eines LLM Modells nutzen, um effizient Kontext anzureichern. Dazu möchte ich selbst manuell MCP Tools aufrufen und deren Ergebnisse dem Kontext im Editor hinzufügen. 

Implementiere daher einen MCP-Client im Package `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp`.

- Zielserver ist "http://localhost:9094/mcp", definiert als Konstante im Client. Zielprotokoll ist HTTP nicht WebSocket.

Der Client wird als `IAIConnector` Connector implementiert mit dem `Misc` Key Pattern und ist damit automatisch immer selektierbar.
- Der Connector wird in den Default Modells in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Model.java` registriert
	- Das Model erhält nur eine rudimentäre Config, Beispielweise keine Profile/Cache, nur ein Default Reasoning usw.

Die Verarbeitung wird mittels eines Kommandos im Editor aktiviert, über einen analogen Mechanismus über den auch der Control Client seine Kommandos verarbeitet.
Der Vorhandene Prompting Mechanismus wählt bereits Eingabe und Ausgabemodalitäten.
Analog dem `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java` regiert der Client auf Kommandos.
- Beim ersten Aufruf etabliert oder wiederverwendet er eine Verbindung zum MCP-Server und initialisiert wenn notwendig die Session.
- Der Abruf der Toolliste wird gecached und nur bei reconnects erneuert
- Beim Kommando "/tool <id>" wird ein Toolschema aus dem Cache als YAML Markdown Scriptblock zurückgegeben analog dem CCControlClient
	- Das Zurückgegebene Schema dient dem Nutzer als Template für die Argumente des Tools
	- Beschreibungen und Validierung wird als Kommentar zurückgegeben, genauso wie Min/Max oder Informationen wie Optional. Alles für den Aufruf Notwendige ist kein Kommentar. Das Ziel ist dem User eine Vorlage zu geben.
	- Default-Werte Parameter werden mit dem Default befüllt.
	- Das Ziel ist analog dem CCControlClient, der Nutzer kann das Schema Ausfüllen, Selektieren und als Kommando abschicken
	- Der Ouput soll keinen Control Header oder ein "/answer" erhalten
	
Der MCP-Client nimmt diesen erstellen Toolaufruf, wandelt ihn in valides JSON für einen MCP-Toolaufruf um.
- Validierungfehler werden analog anderer Exceptions als Answer zurückgegeben.
- Der MCP Client muss bestimmte Header verwenden
	- "X-MCPC-TOOLS" mit dem value "all", aktiviert die Verfügbarkeit aller Tools
	- "X-MCPC-CONTROL"mit value "off", deaktiviert die Kontrollschnittstelle
	- "X-MCPC-SESSION-ID" enthält eine erzeuge Session ID, diese sollte stabil pro Instanz des Clients sein.
	
Ein Tool Result wird anders als im CCControlClient jedoch nicht im YAML umgewandelt, sondern als Pretty JSON in einem Markdown Script Block zurückgegeben.
Dieses Vorgehensmodell erlaubt es den Prefill zu maximieren und absehbare Toolaufrufe bereits im Vorfeld in die Eingabe zu füllen unter Nutzung derselben Tools die Später dem Agenten zur Verfügung stehen.

## Betroffene Dateien

**Neu – Package `connector/mcp`:**
- `MCPConnector.java` – `IAIConnector<MCPRequest, MCPResponse>`, analog `CCConnector`
- `MCPRequest.java` / `MCPResponse.java` – `IModelRequest`/`IModelResponse`, analog `CCRequest`/`CCResponse`
- `MCPCommand.java` / `MCPCommandType.java` (`Prompt`, `Tool`, ggf. `Exit`) – analog `CCRequest.Command`/`CommandType`
- `MCPClient.java` – HTTP-Session zu `http://localhost:9094/mcp` (Konstante), Header `X-MCPC-TOOLS: all`, `X-MCPC-CONTROL: off`, `X-MCPC-SESSION-ID: <UUID, instanzstabil>`; JSON-RPC-`initialize`/`tools/list`/`tools/call`; Tool-Cache (invalidiert bei Reconnect)
- `MCPControlClient.java` – `/tool <id>`-Erkennung (analog `submitEdit`/`extractYamlBlock`), Schema→YAML-Template-Rendering (Kommentare für Description/Constraints, Defaults vorbefüllt), Ergebnis→Pretty-JSON-Markdown-Block (kein YAML, kein Control-Header, kein `/answer`)

**Zu ändern:**
- `Model.java` – neuer Enum-Wert `KeyPattern.Misc("^.*$")` (matcht immer); neues Model (z.B. `MCP_TOOLS`) mit `.key(KeyPattern.Misc)`, minimaler `Capabilities` (kein `cacheMode`, keine `agentProfiles`, ein `Reasoning`-Default, `supportTemperature/TopP/MaxToken/Batch(false)`), Eintrag in `DEFAULTS[]`
- `connector/AdaptingConnector.java` – **zwingend**, im Prompt nicht erwähnt: neues Feld+Konstruktor-Init für `MCPConnector`; `case Misc:` in `getConnector(Model)`; `instanceof MCPRequest`/`MCPResponse`-Zweige in `getConnector(IModelRequest)`/`getConnector(IModelResponse)`
- `model/ModelResolverRegistry.java` – kein Codeeingriff nötig (Enum-Iteration/`EnumMap` erfasst `Misc` automatisch), aber Kommentarzeile „ClaudeCode/None…“ sollte um `Misc` ergänzt werden
- `EditorInterface.java` – `generateTag` nutzt `getSupportedKeyPattern().name()` fürs Prompt-Tagging; mit neuem Pattern kompatibel, keine Änderung nötig, aber zu verifizieren

Zusatzerkenntnis: Der Default-Wert von `cfg.keys` ist `"dummy"` (`SessionConfig.keys`). Da `Misc`s Regex alles matcht, wird das Modell bereits gegen diesen Default-Key aktiviert – das erfüllt die geforderte „automatisch immer selektierbar“-Eigenschaft tatsächlich, im Gegensatz zum bisherigen `None`-Pattern (`^none$`), das ohne expliziten Nutzer-Key `"none"` nie aktiv wäre.

`Reasoning.Disabled` wird verwendet (existiert bereits), nicht `Reasoning.None`.

## Aktualisierter Stand

1. **Modellwahl:** `MCP_TOOLS`-Model mit `KeyPattern.Misc` muss vom Nutzer aktiv als Model selektiert werden; das steuert Routing in `AdaptingConnector`.
2. **Session-Lifecycle:** Kein zentraler Session-Manager wie `CCSessionManager` nötig – Connection/Session-State (HTTP-Client, Session-ID, Tool-Cache) lebt als Instanzfeld direkt in `MCPConnector`/`MCPClient` (1:1, nicht geteilt).
3. **Unterscheidung Kommando/Toolaufruf:** Nur relevant, wenn `MCP_TOOLS` selektiert ist; Texterkennung analog `CCConnector.preprocessInput`: `/tool <id>` → Schema-Ausgabe; sonst Versuch, Eingabe als YAML-Toolaufruf-Block zu parsen (analog `CCControlClient.extractYamlBlock`/`fromYaml`); alles andere ggf. Fehler oder Passthrough.
4. **Fehler:** `IllegalArgumentException`/vorhandene Exception-Konventionen genügen, werden vom Framework als Answer-Text im Editor angezeigt.
5. **MCP-Protokoll:** Minimal-Subset (kein strikter Versions-Check), Reconnect nur bei tatsächlichem Verbindungsverlust (lazy, kein Timeout/Extra-Kommando).
6. **Reasoning:** `Reasoning.Disabled`.
7. **Keine Batch-Fähigkeit:** `MCPConnector` implementiert nur `IAIConnector`, kein `IAIBatchConnector`; `AdaptingConnector.getBatchConnector` bleibt ohne `case Misc`.

## Finale Liste betroffener Dateien

**Neu (`connector/mcp`):**
- `MCPConnector.java`
- `MCPRequest.java`, `MCPResponse.java`
- `MCPCommand.java`, `MCPCommandType.java`
- `MCPClient.java` (Verbindung, Header, Session-ID, Tool-Cache, JSON-RPC `initialize`/`tools/list`/`tools/call`, Lazy-Reconnect bei Verbindungsverlust)
- `MCPControlClient.java` (`/tool <id>`-Handling, Schema→YAML-Template, Toolaufruf-Erkennung/-Ausführung, Ergebnis→Pretty-JSON-Codeblock)

**Ändern:**
- `Model.java`: `KeyPattern.Misc("^.*$")`, Model `MCP_TOOLS` (`.key(KeyPattern.Misc)`, `.reasonings(Reasoning.Disabled)`, `supportTemperature/TopP/MaxToken/Batch(false)`, kein `cacheMode`/`agentProfiles`), Eintrag in `DEFAULTS[]`
- `connector/AdaptingConnector.java`: Feld + Konstruktor-Init `MCPConnector`, `case Misc:` in `getConnector(Model)`, `instanceof MCPRequest/MCPResponse` in beiden `getConnector(...)`-Overloads
- `model/ModelResolverRegistry.java`: Kommentar ergänzen (kein Codeeingriff nötig)
SystemInit:  id=496ecbd3-30b6-4989-aa37-8e21af4ddd0c, cwd=/home/user/xyan/xy.ai.workbench, model=claude-opus-4-8
Text:
 I'll explore the existing code structure to understand the patterns before implementing.
Control Request:
```yaml
id: 7dc9-34
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
  reason: Understand connector package structure and existing files
```
/answer 7dc9-34 allow
Control Request:
```yaml
id: 7dc9-35
result:
  structuredContent:
    results: []
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      error: Too many entries (207) exceed the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
  isError: true
```
/answer 7dc9-35 allow
Control Request:
```yaml
id: 7dc9-36
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Model.java
  reason: Understand Model.java structure for adding Misc KeyPattern and MCP_TOOLS model
```
/answer 7dc9-36 allow