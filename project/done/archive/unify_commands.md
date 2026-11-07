Wir vereinheitlichen das Slash-Command-System. Ziel ist die zentrale Vereinheitlichung der Nutzung, Behandlung und Implementierung von Kommandos, die im Editor angegeben oder ausgeführt werden. Dies gibt dem Session Processor eine zentrale Anlaufstelle und ermöglicht es, Kommandos von anderem Text zu unterscheiden.

## Kommandos für den Control Client
Implementiert in: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java`
- `ANSWER`: Das ANSWER-Kommando soll im Session Processor umgewandelt werden. Das Kommando wird einschließlich der Option (deny/allow) entfernt; es verbleibt nur der optionale Text in der Zeile. Das ANSWER-Kommando beendet dennoch einen gegebenenfalls vorhandenen Block. In den meisten Fällen ist dies ohnehin ein vorausgehender Tool Call. In der aktuellen Implementierung wird damit bereits ein Message-Block erzeugt.
- `CONTROL_REQUEST`: Der Session Processor ignoriert die Zeile und den folgenden YAML-Skript-Block. Dabei handelt es sich um eine Tool-Call-Anforderung, die nicht von einem Modell erzeugt wurde.

## Claude Code Connector

Über den Control Client hinaus implementiert der Claude Code Connector noch weitere Kommandos zur Session-Verwaltung.

- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java` für Claude Code
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java`
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java`, innere Klasse `Command`

## Ziel
Ein globaler Command Handler soll eine Infrastruktur zur Implementierung von Kommandos bereitstellen.

- Globaler Command Handler in: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector`
- Kommandos erhalten eine abstrakte Oberklasse, von der sie abgeleitet werden.
- Verwaltet eine statische Liste möglicher (registrierter) Kommandos mit hart kodierten Referenzen.
    - Diese Liste wird vom Session Processor für die Verarbeitung genutzt. Er kann dadurch erkennen, wie semantisch zusammenhängende, durch Kommandos definierte Bereiche behandelt werden müssen. Beispielsweise wird eine `/resume`-Zeile aus der Eingabe entfernt, ein `/answer*`-Kommando hingegen umgewandelt. Ziel ist die Erhaltung der Semantik und des symmetrischen Modus des Session Processor. Eingaben und Ausgaben der Komponenten sollen untereinander und zwischen den Connectoren austauschbar sein, deterministisch sein und Prefix-Caching nicht unterbrechen. Vom MCP-Connector oder Claude erzeugte Tool-Call- und Result-Blöcke sollen auch durch einen anderen Connector geleitet werden können. Eine globale Kommandosyntax wird etabliert und geteilt.

## Prompt-Behandlung

Lagere den prompt-spezifischen Code, der sich in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java` befindet, in einen Prompt-Handler im Package `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness` aus.

Der Prompt-Handler empfängt den auslösenden Trigger und friert alle Eingaben in ein Prompt-Objekt ein (ausgewählte Tools, Textbereich, Batch, Modellkonfiguration, Systemprompt).

- Konfigurationszustand: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java`
- Der Aktivierungszustand des Session Processor muss zukünftig über das Prompt-Objekt steuerbar sein und soll nicht mehr direkt gesetzt werden.
- Ziel ist die Kompatibilität mit einer Multi-Session-Struktur und möglichem Parallelbetrieb.
- Für das Fixieren der Konfiguration soll ein eigenes Objekt verwendet werden. Die semantische Trennung von Eingaben, Batch und Konfiguration soll auch im Prompt-Objekt erhalten bleiben.

Der Prompt-Handler implementiert künftig die Kommandoerkennung und Eingabeauswahl. Beispielsweise parst er bei einem `/call`-Kommando den davor liegenden YAML-Block für das Kommando, ausgehend von der Cursor-Position und unabhängig davon, ob dieser ausgewählt wurde. Anwendungsfall ist das komfortable Aktivieren eines Kommandos ohne vorherige Auswahl.

### Prompt-Objekt

Das Prompt-Objekt generiert eine Session-ID auf Basis eines Hashwerts der folgenden Parameter:
- Absoluter Dateipfad der Datei, in der der Prompt initiiert wurde (auf einem analogen Mechanismus basiert der Session-Parameter-Hash in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java`)
- Batch Trigger ja oder nein
- Ein Hash der eingefrorenen Konfiguration mit den Bestandteilen:
    - Modell, Profil, Reasoning, Tools, Systemprompt, TopP, Temperature, Max Tokens

Der Implementierungsteil soll aus `ClaudeSessionParameters` herausgelöst und in ein neues Objekt `SessionParameters` extrahiert werden. `ClaudeSessionParameters` soll von `SessionParameters` abgeleitet werden.

Alle direkten Verwendungen der Konfiguration in den Connectoren sollen auf das Prompt-Objekt umgestellt werden. Statt einzelner Parameter soll das Prompt-Objekt an die Connectoren übergeben werden.
Connectoren: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claude/ClaudeConnector.java`, `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/deepseek/DeepSeekConnector.java`, `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/google/GeminiConnector.java`, `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/openai/OpenAIConnector.java`

## Umgang mit Full Editor Mode und Selection Input Mode

Der Prompt-Handler erkennt ein Kommando in der Eingabe wie folgt:
1. Prüfe, ob die Auswahl eine aktuelle Zeileneingabe, eine Blockauswahl oder eine Gesamtdatei-Eingabe ist.
    - Bei aktueller Zeileneingabe: Prüfe, ob die aktuelle Zeile mit einem Kommando beginnt.
    - Bei Blockauswahl: Prüfe, ob der Block mit einem Kommando beginnt (mehrzeilige Kommandos) oder ein Kommando enthält, das in der letzten Zeile beginnt (z. B. wenn es einem Tool Call nachgestellt ist).
    - Bei Gesamtdatei-Eingabe („Processor“-Modus): Prüfe zuerst die aktuelle Cursorzeile auf ein Kommando.
        - Danach prüfe, ob die letzte Zeile der Datei mit einem Kommando beginnt.
        - Kommandos an anderer Stelle in einer Gesamtdatei-Eingabe werden lediglich als Session-Kontext betrachtet.

## MCP-Call-Kommando

Der MCP-Client soll künftig einem zurückgegebenen Tool-Call-Template ein `/call`-Kommando anfügen. Beim Auslösen des Call-Kommandos soll der Prompt-Handler das Kommando erkennen, parsen und typisieren sowie es mittels des Prompt-Objekts an den MCP-Client übergeben. Dabei erfasst der Prompt-Handler von der Zeile des Kommandos aus rückwärts den davor liegenden YAML-Block (`` `^```yaml$` ``).

Zudem wird dem YAML Template das Präfix `Tool:` vorangestellt.

- Präfix-Konstanten: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java`
- Der Session Processor wird den Tool Call erkennen, in den richtigen Message-Typ umwandeln und das `/call`-Kommando ignorieren.


SystemInit:  id=ea8143ba-d655-4aff-a0b8-83dffb2c226a, cwd=/home/user/xyan/xy.ai.workbench, model=claude-opus-4-8
Text:
 Ich beginne mit der Analyse der zentralen Dateien, um die aktuelle Struktur zu verstehen.
Control Request:
```yaml
id: 7fa5-1
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
  reason: Verstehen der aktuellen Kommando-Implementierung im Claude Code Connector
```
/answer 7fa5-1 allow
Control Request:
```yaml
id: 7fa5-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: WHD6Rc
        type: imports
      - id: msdu0B
        type: imports
      - id: AQWMTb
        type: statements
      - id: CCControlClient
        type: class_declaration
        signature: "public class CCControlClient {"
        children:
        - id: CCControlClient.hibHfZ
          type: statements
        - id: CCControlClient.checkControlEndpoint
          type: method_declaration
          signature: "public void checkControlEndpoint(CCResponse resp) {"
        - id: CCControlClient.toYaml
          type: method_declaration
          signature: "public String toYaml(JsonNode node) {"
        - id: CCControlClient.fromYaml
          type: method_declaration
          signature: "public JsonNode fromYaml(String text) throws JsonProcessingException {"
        - id: CCControlClient.isMCPCAvailable
          type: method_declaration
          signature: "public boolean isMCPCAvailable() {"
        - id: CCControlClient.poll
          type: method_declaration
          signature: "private ArrayNode poll() {"
        - id: CCControlClient.approve
          type: method_declaration
          signature: "public ArrayNode approve(String id) {"
        - id: CCControlClient.approve_1
          type: method_declaration
          signature: "public ArrayNode approve(String id, String hint) {"
        - id: CCControlClient.deny
          type: method_declaration
          signature: "public ArrayNode deny(String id, String reason) {"
        - id: CCControlClient.submitEdit
          type: method_declaration
          signature: "public boolean submitEdit(String rawText) {"
        - id: CCControlClient.extractYamlBlock
          type: method_declaration
          signature: "private String extractYamlBlock(String text) {"
        - id: CCControlClient.approvalNode
          type: method_declaration
          signature: "private ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result,…"
        - id: CCControlClient.submit
          type: method_declaration
          signature: "private ArrayNode submit(ObjectNode approval) {"
        - id: CCControlClient.post
          type: method_declaration
          signature: "private ArrayNode post(ObjectNode body) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
      nodes:
      - id: 8zE170
        type: imports
      - id: CommandType
        type: enum_declaration
        signature: "public enum CommandType {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: yJKOSV
        type: imports
      - id: CCRequest
        type: class_declaration
        signature: "public class CCRequest implements IModelRequest {"
        children:
        - id: CCRequest.Ki6WvE
          type: statements
        - id: CCRequest.CCRequest
          type: constructor_declaration
          signature: "public CCRequest(String id, String title, String systemPrompt, List<String> too…"
        - id: CCRequest.getID
          type: method_declaration
          signature: '@Override'
        - id: CCRequest.Command
          type: class_declaration
          signature: "public static class Command {"
          children:
          - id: CCRequest.Command.e89M8i
            type: statements
          - id: CCRequest.Command.Command
            type: constructor_declaration
            signature: "public Command(CommandType type, String... parameters) {"
```
/answer 7fa5-2 allow
Control Request:
```yaml
id: 7fa5-3
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  reason: Verstehen der Session-Parameter und Connector-Struktur
```
/answer 7fa5-3 allow
Control Request:
```yaml
id: 7fa5-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
      nodes:
      - id: UIM5xW
        type: imports
      - id: ClaudeSessionParameters
        type: class_declaration
        signature: "public class ClaudeSessionParameters {"
        children:
        - id: ClaudeSessionParameters.igRtKw
          type: statements
        - id: ClaudeSessionParameters.ZbqmG7
          type: statements
        - id: ClaudeSessionParameters.ClaudeSessionParameters
          type: constructor_declaration
          signature: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tool…"
        - id: ClaudeSessionParameters.ClaudeSessionParameters_1
          type: constructor_declaration
          signature: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tool…"
        - id: ClaudeSessionParameters.fromConfig
          type: method_declaration
          signature: "public static ClaudeSessionParameters fromConfig(ConfigManager cfg, Path cwd, S…"
        - id: ClaudeSessionParameters.getFilePath
          type: method_declaration
          signature: "public String getFilePath() {"
        - id: ClaudeSessionParameters.buildBaseCommand
          type: method_declaration
          signature: "public List<String> buildBaseCommand() {"
        - id: ClaudeSessionParameters.buildEvironment
          type: method_declaration
          signature: "public void buildEvironment(ProcessBuilder pb) {"
        - id: ClaudeSessionParameters.getHash
          type: method_declaration
          signature: "public String getHash() {"
        - id: ClaudeSessionParameters.computeHash
          type: method_declaration
          signature: "private String computeHash() {"
        - id: ClaudeSessionParameters.setTitle
          type: method_declaration
          signature: "public void setTitle(String title) {"
        - id: ClaudeSessionParameters.getTitle
          type: method_declaration
          signature: "public String getTitle() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: YZNdWp
        type: imports
      - id: 84X5CG
        type: imports
      - id: CCConnector
        type: class_declaration
        signature: "public class CCConnector implements IAIConnector<CCRequest, CCResponse> {"
        children:
        - id: CCConnector.oN6Hbi
          type: statements
        - id: CCConnector.CCConnector
          type: constructor_declaration
          signature: "public CCConnector(ConfigManager cfg, CCSessionManager sessionManager) {"
        - id: CCConnector.getSupportedKeyPattern
          type: method_declaration
          signature: '@Override'
        - id: CCConnector.createRequest
          type: method_declaration
          signature: '@Override'
        - id: CCConnector.executeRequest
          type: method_declaration
          signature: '@Override'
        - id: CCConnector.readUntilResult
          type: method_declaration
          signature: "private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressM…"
        - id: CCConnector.abbreviateForStatus
          type: method_declaration
          signature: "private static String abbreviateForStatus(String s) {"
        - id: CCConnector.preprocessInput
          type: method_declaration
          signature: "private List<Command> preprocessInput(List<String> inputs) {"
        - id: CCConnector.convertResponse
          type: method_declaration
          signature: '@Override'
        - id: CCConnector.getEditorLocation
          type: method_declaration
          signature: "private EditorLocation getEditorLocation() {"
```
/answer 7fa5-4 allow
ReasoningToken: 66
Control Request:
```yaml
id: 7fa5-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
    ids:
    - CommandType
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    ids:
    - CCRequest.Command
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - CCControlClient.checkControlEndpoint
    - CCControlClient.submitEdit
    - CCControlClient.extractYamlBlock
  reason: Detailverständnis der Kommandotypen und Control-Client-Verarbeitung
```
/answer 7fa5-5 allow
Control Request:
```yaml
id: 7fa5-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
      nodes:
      - id: CommandType
        type: enum_declaration
        code: "public enum CommandType {\n\tPrompt, Exit, Resume, Allow, Deny, Modification;\n}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: CCRequest.Command
        type: class_declaration
        signature: "public static class Command {"
        children:
        - id: CCRequest.Command.e89M8i
          type: statements
          code: "public final CommandType type;\n\t\tpublic final String parameter;\n\t\tpublic final String[] parameters;"
        - id: CCRequest.Command.Command
          type: constructor_declaration
          code: "public Command(CommandType type, String... parameters) {\n\t\t\tObjects.requireNonNull(type, \"Type can't be null\");\n\t\t\tthis.type = type;\n\t\t\tthis.parameter = parameters[0];\n\t\t\tthis.parameters = parameters;\n\t\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: CCControlClient.checkControlEndpoint
        type: method_declaration
        code: "public void checkControlEndpoint(CCResponse resp) {\n\t\tJsonNode pending = poll();\n\t\tif (pending.isEmpty())\n\t\t\treturn;\n\t\tStringBuilder res = new StringBuilder();\n\t\tProtocolParser.appendEvents(resp.events, res);\n\n\t\tJsonNode first = pending.get(0);\n\t\tres.append(String.format(\"%s\\n```yaml\\n%s\\n```\\n%s %s allow\", CONTROL_REQUEST, toYaml(first), ANSWER,\n\t\t\t\tfirst.path(\"id\").asText()));\n\t\tresp.resultText = res.toString();\n\t}"
      - id: CCControlClient.submitEdit
        type: method_declaration
        code: "public boolean submitEdit(String rawText) {\n\t\tif (rawText == null)\n\t\t\treturn false;\n\t\tString block = extractYamlBlock(rawText.strip());\n\t\tif (block == null)\n\t\t\treturn false;\n\n\t\tJsonNode edited;\n\t\ttry {\n\t\t\tedited = fromYaml(block);\n\t\t} catch (Exception e) {\n\t\t\tthrow new IllegalArgumentException(\"YAML Error\", e);\n\t\t}\n\t\tif (edited == null || !edited.isObject() || !edited.hasNonNull(\"id\"))\n\t\t\treturn false;\n\t\tsubmit((ObjectNode) edited);\n\t\treturn true;\n\t}"
      - id: CCControlClient.extractYamlBlock
        type: method_declaration
        code: "private String extractYamlBlock(String text) {\n\t\tint start = text.indexOf(\"```yaml\");\n\t\tif (start != 0)\n\t\t\treturn null;\n\t\tint contentStart = start + \"```yaml\".length();\n\t\tint end = text.indexOf(\"```\", contentStart);\n\t\tif (end == -1)\n\t\t\treturn null;\n\t\treturn text.substring(contentStart, end).strip();\n\t}"
```
/answer 7fa5-6 allow
Control Request:
```yaml
id: 7fa5-7
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  reason: Verstehen des Prompt-spezifischen Codes und der Editor-Präfix-Konstanten
```
/answer 7fa5-7 allow
Control Request:
```yaml
id: 7fa5-8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
      nodes:
      - id: KhOYVD
        type: imports
      - id: mGo4hS
        type: imports
      - id: Somu8B
        type: imports
      - id: AISessionManager
        type: class_declaration
        signature: "public class AISessionManager {"
        children:
        - id: AISessionManager.BQ3GIF
          type: statements
        - id: AISessionManager.LdjI0C
          type: statements
        - id: AISessionManager.AISessionManager
          type: constructor_declaration
          signature: "public AISessionManager(ConfigManager cfg, AdaptingConnector connector, MCPClie…"
        - id: AISessionManager.discoverTools
          type: method_declaration
          signature: "private void discoverTools(Model model) {"
        - id: AISessionManager.sortByToolOrder
          type: method_declaration
          signature: "private String[] sortByToolOrder(Collection<String> names) {"
        - id: AISessionManager.clearObserver
          type: method_declaration
          signature: "public void clearObserver() {"
        - id: AISessionManager.addInputStatObs
          type: method_declaration
          signature: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {"
        - id: AISessionManager.addAnswerObs
          type: method_declaration
          signature: "public void addAnswerObs(Consumer<AIAnswer> obs) {"
        - id: AISessionManager.updateInputStat
          type: method_declaration
          signature: "public void updateInputStat(InputMode mode) {"
        - id: AISessionManager.initializeInputs
          type: method_declaration
          signature: "public void initializeInputs() {"
        - id: AISessionManager.getInput
          type: method_declaration
          signature: "private String getInput(InputMode mode) {"
        - id: AISessionManager.removeCommentLines
          type: method_declaration
          signature: "public String removeCommentLines(String input) {"
        - id: AISessionManager.execute
          type: method_declaration
          signature: "public void execute(Display display) {"
        - id: AISessionManager.PromptJob
          type: class_declaration
          signature: "private class PromptJob extends Job {"
          children:
          - id: AISessionManager.PromptJob.KFkQPW
            type: statements
          - id: AISessionManager.PromptJob.PromptJob
            type: constructor_declaration
            signature: "private PromptJob(String name, Display display) {"
          - id: AISessionManager.PromptJob.run
            type: method_declaration
            signature: '@Override'
        - id: AISessionManager.queueAsync
          type: method_declaration
          signature: "public void queueAsync(Display display, AIBatchManager batch) {"
        - id: AISessionManager.queueSync
          type: method_declaration
          signature: "private void queueSync(Display display, AIBatchManager batch, IProgressMonitor …"
        - id: AISessionManager.queueAndSubmit
          type: method_declaration
          signature: "public void queueAndSubmit(Display display, AIBatchManager batch) {"
        - id: AISessionManager.prepareInner
          type: method_declaration
          signature: "private IModelRequest prepareInner(Display display, boolean batchFix, IProgress…"
        - id: AISessionManager.executeInner
          type: method_declaration
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: crr53s
        type: imports
      - id: dN3kdP
        type: imports
      - id: nccVUE
        type: imports
      - id: EditorInterface
        type: class_declaration
        signature: "public class EditorInterface {"
        children:
        - id: EditorInterface.nZrzm0
          type: statements
        - id: EditorInterface.EditorInterface
          type: constructor_declaration
          signature: "public EditorInterface(ActiveEditorListener editorListener, AdaptingConnector c…"
        - id: EditorInterface.insertTag
          type: method_declaration
          signature: "public void insertTag(Display display, IModelRequest req, IProgressMonitor mon)…"
        - id: EditorInterface.generateTag
          type: method_declaration
          signature: "private String generateTag(IModelRequest req) {"
        - id: EditorInterface.replaceTag
          type: method_declaration
          signature: "public void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {"
```
/answer 7fa5-8 allow
ReasoningToken: 43
Text:
 Dies ist ein sehr umfangreicher, mehrteiliger Umbau (globaler Command Handler, abstrakte Command-Klassen, PromptHandler, `SessionParameters`-Extraktion, Umstellung von 5 Connectoren, `/call`-Kommando). Bevor ich beginne und um die Session effizient zu halten:
Control Request:
```yaml
id: 7fa5-9
toolName: ask_user
arguments:
  question: |-
    Der Umbau umfasst viele Teile mit starken Abhängigkeiten. Wie soll ich vorgehen?

    A) Komplett in einem Zug (hohes Risiko für Build-Brüche zwischendurch, viele Dateien)
    B) Schrittweise in dieser Reihenfolge, mit Validierung nach jedem Schritt:
       1. `SessionParameters` aus `ClaudeSessionParameters` extrahieren
       2. Globaler CommandHandler + abstrakte Command-Oberklasse + Registry (inkl. ANSWER/CONTROL_REQUEST/Resume/Call...)
       3. PromptHandler + Prompt-Objekt (Config einfrieren, Session-ID-Hash) im Package `connector/harness`
       4. Connectoren auf Prompt-Objekt umstellen
       5. `/call`-Kommando (MCP-Client) + Session-Processor-Anbindung

    Soll ich mit B) und Schritt 1 starten? Oder gibt es einen Teil, den du zuerst willst?
  reason: Der Task ist sehr groß und mehrdeutig in der Reihenfolge; die Vorgehensweise bestimmt alle folgenden Änderungen und vermeidet unnötige Build-Brüche.
```
/answer 7fa5-9 allow Analysiere den Prompt, Analysiere den Code. Restrukturiere dann zu einem neuen sequentiellen Plan. Gebe dann den Plan aus. Dieser muss keinen finalen Code beinhalten.
ReasoningToken: 715
Text:
 Verstanden. Ich analysiere den Code tiefer, um einen fundierten Plan zu erstellen.
Control Request:
```yaml
id: 7fa5-a
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
    ids:
    - AISessionManager.getInput
    - AISessionManager.execute
    - AISessionManager.prepareInner
    - AISessionManager.queueSync
    - AISessionManager.PromptJob.run
    - AISessionManager.updateInputStat
    - AISessionManager.initializeInputs
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - CCConnector.preprocessInput
    - CCConnector.createRequest
    - CCConnector.executeRequest
  reason: Prompt-spezifischer Code und Kommandoverarbeitung im Detail verstehen
```
/answer 7fa5-a allow
Control Request:
```yaml
id: 7fa5-b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
      nodes:
      - id: AISessionManager.getInput
        type: method_declaration
        code: "private String getInput(InputMode mode) {\n\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\n\t\tswitch (mode) {\n\t\tcase SystemPrompt:\n\t\t\tStringBuffer systemPrompt = new StringBuffer();\n\t\t\tArrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith(\"#\"))\n\t\t\t\t\t.forEach(e -> systemPrompt.append(\"* \").append(e).append(\".\\n\"));\n\t\t\tString freeText = cfg.getFreeText();\n\t\t\tif (freeText != null && !freeText.isBlank())\n\t\t\t\tsystemPrompt.append(\".\\n\").append(cfg.getFreeText()).append(\".\\n\");\n\t\t\tString prompttext = systemPrompt.toString();\n\t\t\treturn prompttext.length() > 0 && !prompttext.isBlank() ? prompttext : null;\n\t\tcase Selection:\n\t\t\tif (textEditor != null) {\n\t\t\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\t\t\tif (selectionProvider != null) {\n\t\t\t\t\tISelection selection = selectionProvider.getSelection();\n\t\t\t\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\t\t\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1)\n\t\t\t\t\t\treturn removeCommentLines(tsel.getText());\n\n\t\t\t\t\tif (tsel != null) {\n\t\t\t\t\t\tint line = tsel.getEndLine();\n\t\t\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\tIRegion lineInfo = doc.getLineInformation(line);\n\t\t\t\t\t\t\treturn doc.get(lineInfo.getOffset(), lineInfo.getLength());\n\t\t\t\t\t\t} catch (BadLocationException e1) {\n\t\t\t\t\t\t\tLOG.error(\"Exception\", e1);\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t\tbreak;\n\t\tcase Converter:\n\t\t\tif (textEditor != null) {\n\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\tif (doc != null)\n\t\t\t\t\treturn doc.get();\n\t\t\t}\n\t\t\tbreak;\n\t\tcase Tools:\n\t\t\tthrow new UnsupportedOperationException();\n\t\t}\n\t\treturn null;\n\t}"
      - id: AISessionManager.execute
        type: method_declaration
        code: "public void execute(Display display) {\n\t\tnew PromptJob(\"Starting Prompt\", display).schedule();\n\t}"
      - id: AISessionManager.prepareInner
        type: method_declaration
        code: "private IModelRequest prepareInner(Display display, boolean batchFix, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Preparing Call\", 1);\n\t\tsub.subTask(\"Preparing Call\");\n\n\t\tList<String> inputs = new ArrayList<String>();\n\t\tdisplay.syncExec(() -> {\n\t\t\tString input = null;\n\t\t\tif (cfg.isInputEnabled(InputMode.Selection))\n\t\t\t\tinput = getInput(InputMode.Selection);\n\t\t\telse if (cfg.isInputEnabled(InputMode.Converter))\n\t\t\t\tinput = getInput(InputMode.Converter);\n\t\t\tif (input != null)\n\t\t\t\tinputs.add(input);\n\t\t});\n\n\t\tStringBuffer systemPrompt = new StringBuffer();\n\t\tif (cfg.isInputEnabled(InputMode.SystemPrompt)) {\n\t\t\tString input = getInput(InputMode.SystemPrompt);\n\t\t\tif (input == null)\n\t\t\t\tthrow new IllegalArgumentException(\"Systemprompt is selected but null\");\n\t\t\tsystemPrompt.append(input);\n\t\t}\n\n\t\tif ((inputs == null || inputs.isEmpty()) && systemPrompt.length() == 0)\n\t\t\tthrow new IllegalArgumentException(\"Input and System Prompt Empty\");\n\n\t\tif (editorListener.getLastTextEditor() == null && !batchFix)\n\t\t\tthrow new IllegalArgumentException(\"Result editor unset\");\n\n\t\tList<String> tools = cfg.isInputEnabled(InputMode.Tools) ? List.of(cfg.getTools()) : List.of();\n\n\t\tsub.subTask(\"Input prepared\");\n\n\t\t// InputMode.Converter also toggles the shared SessionProcessor itself: while off, all\n\t\t// connectors turn the whole input into a single plain message via their message callback.\n\t\tsessionProcessor.setEnabled(cfg.isInputEnabled(InputMode.Converter));\n\n\t\tIModelRequest req = connector.createRequest(//\n\t\t\t\tinputs, //\n\t\t\t\tsystemPrompt.toString(), //\n\t\t\t\ttools, //\n\t\t\t\tbatchFix, //\n\t\t\t\tsub//\n\t\t);\n\t\tsub.worked(1);\n\t\treturn req;\n\t}"
      - id: AISessionManager.queueSync
        type: method_declaration
        code: "private void queueSync(Display display, AIBatchManager batch, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Enqueue batch prompt\", 3);\n\t\tsub.subTask(\"Prepare inputs\");\n\t\tvar req = prepareInner(display, true, sub.split(1));\n\t\tsub.subTask(\"Insert Tag\");\n\t\teditIfc.insertTag(display, req, sub.split(1));\n\t\tsub.subTask(\"Enqueue prompt\");\n\t\tbatch.enqueue(req, sub.split(1));\n\t}"
      - id: AISessionManager.PromptJob.run
        type: method_declaration
        code: "@Override\n\t\tprotected IStatus run(IProgressMonitor mon) {\n\t\t\tSubMonitor sub = SubMonitor.convert(mon, \"Executing prompt\", 4);\n\t\t\tString reqId = null;\n\t\t\ttry {\n\t\t\t\tsub.subTask(\"Prepare inputs\");\n\t\t\t\tvar req = prepareInner(display, false, sub);\n\t\t\t\tsub.worked(1);\n\t\t\t\tsub.subTask(\"Insert Tag\");\n\t\t\t\teditIfc.insertTag(display, req, sub);\n\t\t\t\treqId = req.getID();\n\t\t\t\tmon.worked(1);\n\n\t\t\t\tsub.subTask(\"Execute prompt\");\n\t\t\t\tvar ans = executeInner(display, req, sub, this);\n\t\t\t\tmon.worked(1);\n\t\t\t\tsub.subTask(\"Process Answer\");\n\t\t\t\teditIfc.replaceTag(display, ans, sub);\n\t\t\t\tmon.worked(1);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t// uncatched error case\n\t\t\t\tif (reqId != null) {\n\t\t\t\t\tAIAnswer error = new AIAnswer(reqId);\n\t\t\t\t\terror.answer = e.getMessage();\n\t\t\t\t\teditIfc.replaceTag(display, error, sub);\n\t\t\t\t}\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}"
      - id: AISessionManager.updateInputStat
        type: method_declaration
        code: "public void updateInputStat(InputMode mode) {\n\t\tif (mode == InputMode.Tools) {\n\t\t\tString[] tools = cfg.getTools();\n\t\t\tinputStats[mode.ordinal()] = tools != null ? tools.length : 0;\n\t\t} else {\n\t\t\tString input = getInput(mode);\n\t\t\tinputStats[mode.ordinal()] = input != null ? input.length() : -1;\n\t\t}\n\t\tinputStatObs.forEach(c -> c.accept(inputStats));\n\t}"
      - id: AISessionManager.initializeInputs
        type: method_declaration
        code: "public void initializeInputs() {\n\t\tfor (var mode : InputMode.values())\n\t\t\tupdateInputStat(mode);\n\t\tincludeAdapter.initializeInputs();\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: CCConnector.preprocessInput
        type: method_declaration
        code: "private List<Command> preprocessInput(List<String> inputs) {\n\t\tList<Command> commands = new ArrayList<Command>();\n\t\tString clean;\n\t\tfor (String input : inputs)\n\t\t\tif (!(clean = input != null ? input.strip() : \"\").isBlank())\n\t\t\t\tif (\"/exit\".equalsIgnoreCase(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Exit, \"\"));\n\t\t\t\telse if (clean.matches(\"(?i)/resume\\\\s+\\\\S+\"))\n\t\t\t\t\tcommands.add(new Command(CommandType.Resume, clean.split(\"\\\\s+\", 2)[1].strip()));\n\t\t\t\telse if (clean.matches(\"(?i)\" + CCControlClient.ANSWER + \"\\\\s+\\\\S+\\\\s+(allow|deny)(\\\\s+.*)?\")) {\n\t\t\t\t\tString[] parts = clean.split(\"\\\\s+\", 4);\n\t\t\t\t\tString id = parts[1];\n\t\t\t\t\tString action = parts[2].toLowerCase();\n\t\t\t\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id, reason));\n\t\t\t\t\telse\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));\n\t\t\t\t} else if (controlClient.submitEdit(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Modification, \"\"));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Prompt, clean));\n\t\tif (commands.isEmpty())\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn commands;\n\t}"
      - id: CCConnector.createRequest
        type: method_declaration
        code: "@Override\n\tpublic CCRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,\n\t\t\tIProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Create request\", 2);\n\t\tString id = UUID.randomUUID().toString();\n\n\t\t// Preprocessing: extract Allow/Deny/exit/resume lines\n\t\tString title = null;\n\t\tCommand command = null;\n\t\tStringBuilder merged = null;\n\n\t\t{ // Preprocess\n\t\t\tsub.subTask(\"Preprocess input\");\n\t\t\tfor (Command cmd : preprocessInput(inputs))\n\t\t\t\tif (CommandType.Prompt.equals(cmd.type)) {\n\t\t\t\t\tif (title == null)\n\t\t\t\t\t\ttitle = cmd.parameter.substring(0, Math.min(100, cmd.parameter.length())).replace('\\n', ' ');\n\n\t\t\t\t\tif (merged != null)\n\t\t\t\t\t\tmerged.append(\"\\n\");\n\t\t\t\t\telse\n\t\t\t\t\t\tmerged = new StringBuilder();\n\t\t\t\t\tmerged.append(cmd.parameter);\n\t\t\t\t} else {\n\t\t\t\t\tcommand = cmd;\n\t\t\t\t\tbreak;\n\t\t\t\t}\n\t\t\tsub.worked(1);\n\t\t}\n\n\t\tif (command == null && merged != null) {\n\t\t\tsub.subTask(\"Build prompt\");\n\t\t\tcommand = new Command(CommandType.Prompt, requestBuilder.buildPromptJson(merged.toString().trim()));\n\t\t\tsub.worked(1);\n\t\t}\n\n\t\treturn new CCRequest(id, title, systemPrompt, tools, command);\n\t}"
      - id: CCConnector.executeRequest
        type: method_declaration
        code: "@Override\n\tpublic CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Executing prompt\", 2);\n\t\tCCSession session = null;\n\n\t\tEditorLocation loc = getEditorLocation();\n\t\tClaudeSessionParameters params = ClaudeSessionParameters.fromConfig(cfg, loc.projectPath, loc.relativeFilePath,\n\t\t\t\treq.systemPrompt, req.tools);\n\t\tparams.setTitle(req.title);\n\n\t\tswitch (req.cmd.type) {\n\t\tcase Resume:\n\t\t\tsub.subTask(\"Importing session\");\n\t\t\tsessionManager.importSession(req.cmd.parameter, params);\n\t\t\treturn new CCResponse(req.id, \"Session created\");\n\t\tcase Exit:\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t\tsub.subTask(\"Terminating CLI process\");\n\t\t\tsession.terminate();\n\t\t\treturn new CCResponse(req.id, \"Session closed!\");\n\t\tcase Allow:\n\t\tcase Deny:\n\t\tcase Modification:\n\t\t\tswitch (req.cmd.type) {\n\t\t\tcase Allow:\n\t\t\t\tcontrolClient.approve(req.cmd.parameters[0],\n\t\t\t\t\t\treq.cmd.parameters.length > 1 ? req.cmd.parameters[1] : null);\n\t\t\t\tbreak;\n\t\t\tcase Deny:\n\t\t\t\tcontrolClient.deny(req.cmd.parameters[0], req.cmd.parameters[1]);\n\t\t\t\tbreak;\n\t\t\tcase Modification:\n\t\t\t\tbreak; // allready sent\n\t\t\tdefault:\n\t\t\t\tthrow new UnsupportedOperationException();\n\t\t\t}\n\t\t\tsession = sessionManager.getSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t\tbreak;\n\t\tcase Prompt:\n\t\t\tsub.subTask(\"Acquiring session\");\n\t\t\tsession = sessionManager.requestSession(sessionManager.getSelectedSessionUuid(), params);\n\t\t\tbreak;\n\t\t}\n\n\t\ttry {\n\t\t\tsession.setInPrompt(true);\n\t\t\tif (CommandType.Prompt.equals(req.cmd.type)) {\n\t\t\t\tif (AgentProfile.MCPC.equals(session.getParameters().agentProfile) && !controlClient.isMCPCAvailable())\n\t\t\t\t\tthrow new IllegalStateException(\"MCPC not reachable for AgentProfile\");\n\n\t\t\t\tsub.subTask(\"Sending prompt\");\n\t\t\t\tsession.writeLine(req.cmd.parameter);\n\t\t\t}\n\n\t\t\tsub.subTask(\"Waiting for answer\");\n\t\t\treturn readUntilResult(req, session, sub.split(1), job);\n\n\t\t} catch (IOException e) {\n\t\t\tthrow new IllegalStateException(\"Claude Code CLI error\", e);\n\t\t} finally {\n\t\t\tsession.setInPrompt(false);\n\t\t}\n\t}"
```
/answer 7fa5-b allow
Control Request:
```yaml
id: 7fa5-c
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
    ids:
    - ClaudeSessionParameters
  reason: Vollständige ClaudeSessionParameters für Extraktion von SessionParameters
```
/answer 7fa5-c allow
Control Request:
```yaml
id: 7fa5-d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
      nodes:
      - id: ClaudeSessionParameters
        type: class_declaration
        signature: "public class ClaudeSessionParameters {"
        children:
        - id: ClaudeSessionParameters.igRtKw
          type: statements
          code: "private static final String SCRIPT = System.getProperty(\"user.home\")\n\t\t\t+ \"/xyan/xy.ai.workbench/claude-code/claude-session.sh\";\n\tprivate static final String COMMAND = \"claude\";\n\n\t/** Deterministic hash of the session parameters. Immutable. */\n\tpublic final Path cwd;\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final Model model;\n\tpublic final Reasoning reasoning;\n\tpublic final AgentProfile agentProfile;\n\tpublic final String cliProfile;\n\tpublic final String filePath;"
        - id: ClaudeSessionParameters.ZbqmG7
          type: statements
          code: "public final CacheMode cacheMode;\n\tprivate String hash;\n\tprivate String title;"
        - id: ClaudeSessionParameters.ClaudeSessionParameters
          type: constructor_declaration
          code: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode) {\n\t\tthis(cwd, systemPrompt, tools, model, reasoning, agentProfile, cliProfile, cacheMode, null);\n\t}"
        - id: ClaudeSessionParameters.ClaudeSessionParameters_1
          type: constructor_declaration
          code: "public ClaudeSessionParameters(Path cwd, String systemPrompt, List<String> tools, Model model, Reasoning reasoning,\n\t\t\tAgentProfile agentProfile, String cliProfile, CacheMode cacheMode, String filePath) {\n\t\tif (cwd == null)\n\t\t\tthrow new IllegalStateException(\"Work directory (cwd) not set\");\n\t\tif (model == null)\n\t\t\tthrow new IllegalArgumentException(\"Model must not be null\");\n\t\tif (model.apiName == null || model.apiName.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"Model apiName must not be null or blank\");\n\t\tif (reasoning == null)\n\t\t\tthrow new IllegalArgumentException(\"Reasoning must not be null\");\n\n\t\tthis.cwd = cwd;\n\t\tthis.systemPrompt = systemPrompt != null ? systemPrompt : \"\";\n\t\tthis.tools = tools != null ? tools : Collections.emptyList();\n\t\tthis.model = model;\n\t\tthis.reasoning = reasoning;\n\t\tthis.agentProfile = agentProfile;\n\t\tthis.cliProfile = cliProfile;\n\t\tthis.filePath = filePath;\n\t\tthis.cacheMode = cacheMode;\n\t}"
        - id: ClaudeSessionParameters.fromConfig
          type: method_declaration
          code: "public static ClaudeSessionParameters fromConfig(ConfigManager cfg, Path cwd, String filePath, String systemPrompt,\n\t\t\tList<String> tools) {\n\t\treturn new ClaudeSessionParameters(cwd, systemPrompt, tools, cfg.getModel(), cfg.getReasoning(), cfg.getProfile(),\n\t\t\t\tcfg.getKeys(), cfg.getCacheMode(), filePath);\n\t}"
        - id: ClaudeSessionParameters.getFilePath
          type: method_declaration
          code: "public String getFilePath() {\n\t\treturn filePath;\n\t}"
        - id: ClaudeSessionParameters.buildBaseCommand
          type: method_declaration
          code: "public List<String> buildBaseCommand() {\n\t\tList<String> cmd = new ArrayList<>();\n\t\tif (AgentProfile.MCPC.equals(agentProfile)) {\n\t\t\tcmd.add(COMMAND);\n\t\t\tcmd.add(\"--system-prompt\");\n\t\t\tcmd.add(systemPrompt);\n\t\t\tcmd.add(\"--tools\");\n\t\t\tcmd.add(\"\"); // restrict builtin tools\n\t\t\t// evil: breaks STDIN handling\n//\t\t\tcmd.add(\"--debug\");\n//\t\t\tcmd.add(\"mcp\");\n\t\t\tcmd.add(\"--settings\");\n\t\t\tcmd.add(\"\"\"\n\t\t\t\t\t{\n\t\t\t\t\t\t\"hooks\": {\n\t\t\t\t\t\t\t\"PreToolUse\": [\n\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\"hooks\": [\n\t\t\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\t\t\"type\": \"http\",\n\t\t\t\t\t\t\t\t\t\t\t\"url\":\"http://localhost:9093/hooks/tool\",\n\t\t\t\t\t\t\t\t\t\t\t\"headers\":{\n\t\t\t\t\t\t\t\t\t\t\t   \"X-MCPC-SESSION-ID\":\"$MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t},\n\t\t\t\t\t\t\t\t\t\t\t\"allowedEnvVars\":[\n\t\t\t\t\t\t\t\t\t\t\t   \"MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t],\n\t\t\t\t\t\t\t\t\t\t\t\"timeout\": 86400\n\t\t\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t]\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t],\n\t\t\t\t\t\t\t\"PermissionRequest\": [\n\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\"hooks\": [\n\t\t\t\t\t\t\t\t\t\t{\n\t\t\t\t\t\t\t\t\t\t\t\"type\": \"http\",\n\t\t\t\t\t\t\t\t\t\t\t\"url\":\"http://localhost:9093/hooks/permission\",\n\t\t\t\t\t\t\t\t\t\t\t\"headers\":{\n\t\t\t\t\t\t\t\t\t\t\t   \"X-MCPC-SESSION-ID\":\"$MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t},\n\t\t\t\t\t\t\t\t\t\t\t\"allowedEnvVars\":[\n\t\t\t\t\t\t\t\t\t\t\t   \"MCPC_SESSION_ID\"\n\t\t\t\t\t\t\t\t\t\t\t],\n\t\t\t\t\t\t\t\t\t\t\t\"timeout\": 86400\n\t\t\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t]\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t]\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t\t\"\"\".replace(\"\\t\", \"\").replace(\"\\n\", \"\").strip());\n\t\t\tcmd.add(\"--mcp-config\");\n\t\t\tcmd.add(\"\"\"\n\t\t\t\t\t{\n\t\t\t\t\t\t\"mcpServers\": {\n\t\t\t\t\t\t\t\"mcpc\": {\n\t\t\t\t\t\t\t\t\"type\": \"ws\",\n\t\t\t\t\t\t\t\t\"url\": \"http://localhost:9094/mcp\",\n\t\t\t\t\t\t\t\t\"timeout\": 86400000,\n\t\t\t\t\t\t\t\t\"alwaysLoad\": true,\n\t\t\t\t\t\t\t\t\"headers\": {\n\t\t\t\t\t\t\t\t\t\"X-MCPC-SESSION-ID\": \"${MCPC_SESSION_ID}\",\n\t\t\t\t\t\t\t\t\t\"X-MCPC-TOOLS\": \"${MCPC_TOOLS}\",\n\t\t\t\t\t\t\t\t\t\"X-MCPC-CC-PROFILE\": \"${MCPC_CC_PROFILE}\"\n\t\t\t\t\t\t\t\t}\n\t\t\t\t\t\t\t}\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\t\t\t\t\t\t\"\"\".replace(\"\\t\", \"\").replace(\"\\n\", \"\").strip());\n\t\t} else {\n\t\t\tcmd.add(SCRIPT);\n\t\t\tcmd.add(agentProfile != null ? agentProfile.name : \"\"); // Agent definition\n\t\t\tcmd.add(\"--profile\");\n\t\t\tcmd.add(cliProfile);\n\t\t}\n\n\t\tcmd.add(\"--verbose\");\n\t\t// replaced by MCPC\n\t\t// cmd.add(\"--include-hook-events\");\n\t\tcmd.add(\"--include-partial-messages\");\n\t\tcmd.add(\"--input-format\");\n\t\tcmd.add(\"stream-json\");\n\t\tcmd.add(\"--output-format\");\n\t\tcmd.add(\"stream-json\");\n\t\t// replaced by self mirror input\n\t\t// cmd.add(\"--replay-user-messages\");\n\t\tcmd.add(\"--model\");\n\t\tcmd.add(model.apiName);\n\t\tif (Reasoning.Disabled != reasoning) {\n\t\t\tcmd.add(\"--effort\");\n\t\t\tcmd.add(reasoning.name().toLowerCase());\n\t\t}\n\t\tcmd.add(\"--dangerously-skip-permissions\"); // as long there is no permission prompt handling implemented\n\t\treturn cmd;\n\t}"
        - id: ClaudeSessionParameters.buildEvironment
          type: method_declaration
          code: "public void buildEvironment(ProcessBuilder pb) {\n\t\tpb.directory(cwd.toFile());\n\t\tif (AgentProfile.MCPC.equals(agentProfile)) {\n\t\t\tif (!tools.isEmpty()) {\n\t\t\t\tif (CacheMode.Disabled.equals(cacheMode))\n\t\t\t\t\tthrow new IllegalArgumentException(\"Cache is required for tool loops\");\n\t\t\t\tpb.environment().put(\"MCPC_TOOLS\", String.join(\",\", tools));\n\t\t\t} else\n\t\t\t\tpb.environment().put(\"MCPC_TOOLS\", \"None\");\n\t\t\tpb.environment().put(\"MCPC_CC_PROFILE\", cliProfile);\n\t\t\tpb.environment().put(\"CLAUDE_CONFIG_DIR\", System.getProperty(\"user.home\") + \"/.claude-\" + cliProfile);\n\n\t\t\tpb.environment().put(\"CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_AGENT_VIEW\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_BACKGROUND_TASKS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_BUNDLED_SKILLS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_CLAUDE_MDS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_CRON\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_POLICY_SKILLS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_WORKFLOWS\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_ENABLE_AWAY_SUMMARY\", \"0\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_ENABLE_BACKGROUND_PLUGIN_REFRESH\", \"1\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_FORK_SUBAGENT\", \"0\");\n\t\t\tpb.environment().put(\"CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY\", \"1\"); // number of parralel read tools\n\t\t\tpb.environment().put(\"ENABLE_TOOL_SEARCH\", \"false\");\n\t\t\tpb.environment().put(\"MAX_MCP_OUTPUT_TOKENS\", \"\" + (25000 * 10));\n\t\t}\n\n\t\tif (cacheMode != null)\n\t\t\tswitch (cacheMode) {\n\t\t\tcase Disabled:\n\t\t\t\tpb.environment().put(\"DISABLE_PROMPT_CACHING\", \"1\");\n\t\t\t\tbreak;\n\t\t\tcase Minutes_5:\n\t\t\t\tpb.environment().put(\"FORCE_PROMPT_CACHING_5M\", \"1\");\n\t\t\t\tbreak;\n\t\t\tcase Hours_1:\n\t\t\t\tpb.environment().put(\"ENABLE_PROMPT_CACHING_1H\", \"1\");\n\t\t\tcase Default:\n\t\t\tdefault:\n\t\t\t}\n\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_SPELLCHECK\", \"true\");\n\t\tpb.environment().put(\"CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT\", \"0\");\n\t\tpb.environment().put(\"MCP_TOOL_TIMEOUT\", \"86400000\");\n\t\tpb.environment().put(\"CLAUDE_ENABLE_STREAM_WATCHDOG\", \"0\");\n\t\tpb.environment().put(\"CLAUDE_ENABLE_BYTE_WATCHDOG\", \"0\");\n\t\tpb.environment().put(\"CLAUDE_STREAM_IDLE_TIMEOUT_MS\", \"86400000\");\n\t\tpb.environment().put(\"API_FORCE_IDLE_TIMEOUT\", \"0\");\n\t\tpb.environment().put(\"MCP_TIMEOUT\", \"86400000\");\n\n\t\tif (Reasoning.Disabled == reasoning) {\n\t\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING\", \"1\");\n\t\t\tpb.environment().put(\"MAX_THINKING_TOKENS\", \"0\");\n\t\t}\n\t\tpb.environment().put(\"CLAUDE_CODE_DISABLE_ADVISOR_TOOL\", \"1\");\n\t}"
        - id: ClaudeSessionParameters.getHash
          type: method_declaration
          code: "public String getHash() {\n\t\tif (hash == null)\n\t\t\thash = computeHash();\n\t\treturn hash;\n\t}"
        - id: ClaudeSessionParameters.computeHash
          type: method_declaration
          code: "private String computeHash() {\n\t\tString input = String.join(\",\", tools) + \"|\" + cwd.toString() + \"|\"\n\t\t\t\t+ model.apiName + \"|\" + reasoning.name() + \"|\" + (agentProfile != null ? agentProfile.name : \"\") + \"|\"\n\t\t\t\t+ cliProfile + \"|\" + (filePath != null ? filePath : \"\");\n\t\ttry {\n\t\t\tMessageDigest md = MessageDigest.getInstance(\"MD5\");\n\t\t\tbyte[] bytes = md.digest(input.getBytes(StandardCharsets.UTF_8));\n\t\t\tStringBuilder sb = new StringBuilder();\n\t\t\tfor (byte b : bytes)\n\t\t\t\tsb.append(String.format(\"%02x\", b));\n\t\t\treturn sb.substring(0, 8);\n\t\t} catch (NoSuchAlgorithmException e) {\n\t\t\t// Stable fallback (no external dependency)\n\t\t\tlong h = 0;\n\t\t\tfor (char c : input.toCharArray())\n\t\t\t\th = h * 31L + c;\n\t\t\treturn String.format(\"%08x\", h & 0xFFFFFFFFL);\n\t\t}\n\t}"
        - id: ClaudeSessionParameters.setTitle
          type: method_declaration
          code: "public void setTitle(String title) {\n\t\tif (this.title == null)\n\t\t\tthis.title = title;\n\t}"
        - id: ClaudeSessionParameters.getTitle
          type: method_declaration
          code: "public String getTitle() {\n\t\treturn title;\n\t}"
```
/answer 7fa5-d allow
ReasoningToken: 146
Text:
 Nun die Connector-Schnittstelle, den MCP-Client, Session-Processor und relevante Konstanten.
Control Request:
```yaml
id: 7fa5-e
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: createRequest|IAIConnector|AdaptingConnector
    include:
    - '*.java'
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: CONTROL_REQUEST|ANSWER|Tool:|public static final String
    include:
    - EditorInterface.java
    - CCControlClient.java
    limit: 40
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: setEnabled|sessionProcessor|SessionProcessor
    include:
    - '*.java'
    limit: 30
  reason: "Connector-Schnittstelle, Prozessor-Aktivierung und Präfix-Konstanten lokalisieren"
```
/answer 7fa5-e allow
Control Request:
```yaml
id: 7fa5-f
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      pattern: createRequest|IAIConnector|AdaptingConnector
      matches:
      - path: AISessionManager.java
        lineno: 27
        match: import xy.ai.workbench.connector.AdaptingConnector;
      - path: AISessionManager.java
        lineno: 42
        match: "\tprivate final AdaptingConnector connector;"
      - path: AISessionManager.java
        lineno: 50
        match: "\tpublic AISessionManager(ConfigManager cfg, AdaptingConnector connector, MCPClient mcpClient,"
      - path: AISessionManager.java
        lineno: 301
        match: "\t\tIModelRequest req = connector.createRequest(//"
      - path: EditorInterface.java
        lineno: 32
        match: import xy.ai.workbench.connector.AdaptingConnector;
      - path: EditorInterface.java
        lineno: 48
        match: "\tprivate final AdaptingConnector connector;"
      - path: EditorInterface.java
        lineno: 51
        match: "\tpublic EditorInterface(ActiveEditorListener editorListener, AdaptingConnector connector, ConfigManager cfg) {"
      - path: Activator.java
        lineno: 9
        match: import xy.ai.workbench.connector.AdaptingConnector;
      - path: Activator.java
        lineno: 30
        match: "\tprivate AdaptingConnector connector = new AdaptingConnector(cfg, cliSessionManager, mcpClient, sessionProcessor);"
      - path: connector/mcp/MCPConnector.java
        lineno: 13
        match: import xy.ai.workbench.connector.IAIConnector;
      - path: connector/mcp/MCPConnector.java
        lineno: 16
        match: "public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {"
      - path: connector/mcp/MCPConnector.java
        lineno: 34
        match: "\tpublic MCPRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
      - path: connector/claudecode/CCConnector.java
        lineno: 27
        match: import xy.ai.workbench.connector.IAIConnector;
      - path: connector/claudecode/CCConnector.java
        lineno: 31
        match: "public class CCConnector implements IAIConnector<CCRequest, CCResponse> {"
      - path: connector/claudecode/CCConnector.java
        lineno: 51
        match: "\tpublic CCRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
      - path: connector/google/GeminiConnector.java
        lineno: 35
        match: import xy.ai.workbench.connector.IAIConnector;
      - path: connector/google/GeminiConnector.java
        lineno: 43
        match: "public class GeminiConnector implements IAIConnector<GeminiRequest, GeminiResponse> {"
      - path: connector/google/GeminiConnector.java
        lineno: 68
        match: "\tpublic GeminiRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
      - path: connector/IAIConnector.java
        lineno: 13
        match: "public interface IAIConnector<REQ extends IModelRequest, RESP extends IModelResponse> {"
      - path: connector/IAIConnector.java
        lineno: 15
        match: "\tREQ createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
      - path: connector/claude/ClaudeConnector.java
        lineno: 28
        match: import xy.ai.workbench.connector.IAIConnector;
      - path: connector/claude/ClaudeConnector.java
        lineno: 36
        match: "public class ClaudeConnector implements IAIConnector<ClaudeRequest, ClaudeResponse> {"
      - path: connector/claude/ClaudeConnector.java
        lineno: 60
        match: "\tpublic ClaudeRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
      - path: connector/openai/OpenAIConnector.java
        lineno: 36
        match: import xy.ai.workbench.connector.IAIConnector;
      - path: connector/openai/OpenAIConnector.java
        lineno: 43
        match: "public class OpenAIConnector implements IAIConnector<OpenAIRequest, OpenAIResponse> {"
      - path: connector/openai/OpenAIConnector.java
        lineno: 67
        match: "\tpublic OpenAIRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
      - path: connector/AdaptingConnector.java
        lineno: 44
        match: "public class AdaptingConnector implements IAIConnector<IModelRequest, IModelResponse>, IAIBatchConnector {"
      - path: connector/AdaptingConnector.java
        lineno: 58
        match: "public AdaptingConnector(ConfigManager cfg, CCSessionManager sessionManager, MCPClient mcpClient,"
      - path: connector/AdaptingConnector.java
        lineno: 75
        match: "\t\tprivate IAIConnector<? extends IModelRequest, ? extends IModelResponse> getConnector(Model model) {"
      - path: connector/AdaptingConnector.java
        lineno: 108
        match: "\tpublic IAIConnector getConnector(IModelRequest request) {"
      warning: "Limit of 30 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      pattern: CONTROL_REQUEST|ANSWER|Tool:|public static final String
      matches:
      - path: EditorInterface.java
        lineno: 38
        match: "\tpublic static final String USER = \"User:\";"
      - path: EditorInterface.java
        lineno: 39
        match: "\tpublic static final String AGENT = \"Agent:\";"
      - path: EditorInterface.java
        lineno: 42
        match: "\tpublic static final String TEXT = \"Text:\";"
      - path: EditorInterface.java
        lineno: 43
        match: "\tpublic static final String THINKING = \"Thinking:\";"
      - path: EditorInterface.java
        lineno: 44
        match: "\tpublic static final String TOOLUSE = \"Tool:\";"
      - path: EditorInterface.java
        lineno: 45
        match: "\tpublic static final String TOOLRESULT = \"ToolResult:\";"
      - path: connector/claudecode/CCControlClient.java
        lineno: 32
        match: "\tpublic static final String ANSWER = \"/answer\";"
      - path: connector/claudecode/CCControlClient.java
        lineno: 33
        match: "\tpublic static final String CONTROL_REQUEST = \"Control Request:\";"
      - path: connector/claudecode/CCControlClient.java
        lineno: 49
        match: "\t\tres.append(String.format(\"%s\\n```yaml\\n%s\\n```\\n%s %s allow\", CONTROL_REQUEST, toYaml(first), ANSWER,"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      pattern: setEnabled|sessionProcessor|SessionProcessor
      matches:
      - path: AISessionManager.java
        lineno: 28
        match: import xy.ai.workbench.connector.harness.SessionProcessor;
      - path: AISessionManager.java
        lineno: 44
        match: "\tprivate final SessionProcessor sessionProcessor;"
      - path: AISessionManager.java
        lineno: 51
        match: "\t\t\tSessionProcessor sessionProcessor) {"
      - path: AISessionManager.java
        lineno: 55
        match: "\t\tthis.sessionProcessor = sessionProcessor;"
      - path: AISessionManager.java
        lineno: 61
        match: "\t\tsessionProcessor.setAdapter(includeAdapter);"
      - path: AISessionManager.java
        lineno: 297
        match: "\t\t// InputMode.Converter also toggles the shared SessionProcessor itself: while off, all"
      - path: AISessionManager.java
        lineno: 299
        match: "\t\tsessionProcessor.setEnabled(cfg.isInputEnabled(InputMode.Converter));"
      - path: IncludeAdapter.java
        lineno: 36
        match: import xy.ai.workbench.connector.harness.SessionProcessor;
      - path: IncludeAdapter.java
        lineno: 40
        match: " * Resolves the {@link SessionProcessor}'s typed include kinds against Eclipse resources,"
      - path: EditorInterface.java
        lineno: 41
        match: "\t/** Session block markers recognized by {@code connector.harness.SessionProcessor}. */"
      - path: view/AISessionView.java
        lineno: 243
        match: "\t\t\tbudget.setEnabled(enabled);"
      - path: view/AISessionView.java
        lineno: 396
        match: "\t\t\t\tSelectionListener.widgetSelectedAdapter(e -> cfg.setEnabledTools(toolsList.getSelection())));"
      - path: view/AISessionView.java
        lineno: 444
        match: "\t\t\t\tcfg.setEnabledTools(new String[0]);"
      - path: view/AISessionView.java
        lineno: 461
        match: "\t\t\t\tcfg.setEnabledTools(preset.tools);"
      - path: view/AISessionView.java
        lineno: 562
        match: "\t\t\tbbtn.setEnabled(m.cap.isSupportBatch());"
      - path: view/AISessionView.java
        lineno: 563
        match: "\t\t\tbsbtn.setEnabled(m.cap.isSupportBatch());"
      - path: view/AISessionView.java
        lineno: 626
        match: "\t\tlabel.setEnabled(enabled);"
      - path: view/AISessionView.java
        lineno: 628
        match: "\t\tctrl.setEnabled(enabled);"
      - path: Activator.java
        lineno: 11
        match: import xy.ai.workbench.connector.harness.SessionProcessor;
      - path: Activator.java
        lineno: 29
        match: "\tpublic SessionProcessor sessionProcessor = new SessionProcessor();"
      - path: Activator.java
        lineno: 30
        match: "\tprivate AdaptingConnector connector = new AdaptingConnector(cfg, cliSessionManager, mcpClient, sessionProcessor);"
      - path: Activator.java
        lineno: 32
        match: "\tpublic AISessionManager session = new AISessionManager(cfg, connector, mcpClient, sessionProcessor);"
      - path: ConfigManager.java
        lineno: 94
        match: "\t\tsetEnabledProfiles(getCapabilities().getAgentProfiles());"
      - path: ConfigManager.java
        lineno: 143
        match: "\tpublic void setEnabledTools(String[] enabledTools) {"
      - path: ConfigManager.java
        lineno: 338
        match: "\tpublic void setEnabledProfiles(AgentProfile[] enabledProfiles) {"
      - path: connector/google/GeminiConnector.java
        lineno: 40
        match: import xy.ai.workbench.connector.harness.SessionProcessor;
      - path: connector/google/GeminiConnector.java
        lineno: 48
        match: "\tprivate final SessionProcessor sessionProcessor;"
      - path: connector/google/GeminiConnector.java
        lineno: 50
        match: "\tpublic GeminiConnector(ConfigManager cfg, MCPClient mcpClient, SessionProcessor sessionProcessor) {"
      - path: connector/google/GeminiConnector.java
        lineno: 53
        match: "\t\tthis.sessionProcessor = sessionProcessor;"
      - path: connector/google/GeminiConnector.java
        lineno: 111
        match: "\t\t\tsessionProcessor.process(inputs, cb);"
      warning: "Limit of 30 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 7fa5-f allow
Control Request:
```yaml
id: 7fa5-10
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: class MCPClient|/call|toolCall|Tool Call|buildTemplate|template
    include:
    - '*.java'
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
    pattern: class .*Connector|createRequest
    include:
    - ClaudeConnector.java
    - DeepSeekConnector.java
    - GeminiConnector.java
    - OpenAIConnector.java
    limit: 40
  reason: MCP-Client und Connector-Klassen für /call und Prompt-Objekt-Umstellung lokalisieren
```
/answer 7fa5-10 allow Beispielweise kann die Umwandlung der Argumente für den Prompt unabhängig geschehen
Control Request:
```yaml
id: 7fa5-11
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      pattern: class MCPClient|/call|toolCall|Tool Call|buildTemplate|template
      matches:
      - path: connector/mcp/MCPClient.java
        lineno: 27
        match: "public class MCPClient {"
      - path: connector/mcp/MCPClient.java
        lineno: 77
        match: "\t\tJsonNode result = send(request(\"tools/call\", params), true);"
      - path: connector/mcp/MCPControlClient.java
        lineno: 16
        match: ' * Renders tool schemas as fillable YAML templates and turns a submitted'
      - path: connector/mcp/MCPControlClient.java
        lineno: 17
        match: " * template back into a tool call. Results are returned as pretty JSON, giving"
      - path: connector/mcp/MCPControlClient.java
        lineno: 24
        match: "\t * Renders a tool's input schema as a YAML template (fenced block). Everything"
      - path: connector/mcp/MCPControlClient.java
        lineno: 107
        match: "\t * since it is filtered out of the rendered template and never filled in by"
      - path: connector/mcp/MCPConnector.java
        lineno: 55
        match: "\t\t\t\t\t\t\"No ```yaml tool call block found. Use \\\"/tool <id>\\\" to get a template.\");"
      - path: connector/google/GeminiConnector.java
        lineno: 175
        match: "\t\t\t\tanswer.toolCall(\"\", call.name().orElse(\"unknown\"), args);"
      - path: connector/claude/ClaudeConnector.java
        lineno: 153
        match: "\t\t\t\tanswer.toolCall(toolUse.id(), toolUse.name(), args);"
      - path: connector/openai/OpenAIConnector.java
        lineno: 189
        match: "\t\t\t\t\tanswer.toolCall(fc.callId(), fc.name(), args);"
      - path: connector/deepseek/DeepSeekConnector.java
        lineno: 209
        match: "\t\tpublic ObjectNode toolCall(ToolCall call) {"
      - path: connector/deepseek/DeepSeekConnector.java
        lineno: 325
        match: "\t\t\t\t\t\tanswer.toolCall(call.getCallId(), call.getName(), args);"
      - path: connector/harness/SessionCallbacks.java
        lineno: 23
        match: "\tdefault M toolCall(ToolCall call) {"
      - path: connector/harness/SessionCallbacks.java
        lineno: 24
        match: "\t\treturn message(Role.Agent, SessionRenderer.toolCall(call));"
      - path: connector/harness/SessionRenderer.java
        lineno: 31
        match: "\tpublic static String toolCall(ToolCall call) {"
      - path: connector/harness/SessionProcessor.java
        lineno: 228
        match: "\t\t\tout.add(callbacks.toolCall("
      - path: connector/harness/SessionAnswerBuilder.java
        lineno: 33
        match: "\tpublic SessionAnswerBuilder toolCall(String id, String name, JsonNode arguments) {"
      - path: connector/harness/SessionAnswerBuilder.java
        lineno: 34
        match: "\t\treturn separate().appendRaw(SessionRenderer.toolCall(new ToolCall(id, name, arguments)));"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      pattern: class .*Connector|createRequest
      matches:
      - path: google/GeminiConnector.java
        lineno: 43
        match: "public class GeminiConnector implements IAIConnector<GeminiRequest, GeminiResponse> {"
      - path: google/GeminiConnector.java
        lineno: 68
        match: "\tpublic GeminiRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
      - path: claude/ClaudeConnector.java
        lineno: 36
        match: "public class ClaudeConnector implements IAIConnector<ClaudeRequest, ClaudeResponse> {"
      - path: claude/ClaudeConnector.java
        lineno: 60
        match: "\tpublic ClaudeRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
      - path: openai/OpenAIConnector.java
        lineno: 43
        match: "public class OpenAIConnector implements IAIConnector<OpenAIRequest, OpenAIResponse> {"
      - path: openai/OpenAIConnector.java
        lineno: 67
        match: "\tpublic OpenAIRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
      - path: deepseek/DeepSeekConnector.java
        lineno: 80
        match: "public class DeepSeekConnector implements IAIConnector<DeepSeekRequest, DeepSeekResponse> {"
      - path: deepseek/DeepSeekConnector.java
        lineno: 110
        match: "\tpublic DeepSeekRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,"
```
/answer 7fa5-11 allow
Control Request:
```yaml
id: 7fa5-12
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AdaptingConnector.java
  reason: "Struktur von SessionProcessor, MCP-Client-Seite und AdaptingConnector für Plan"
```
/answer 7fa5-12 allow
Control Request:
```yaml
id: 7fa5-13
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
      nodes:
      - id: CpA22r
        type: imports
      - id: IVSBB9
        type: statements
      - id: SessionProcessor
        type: class_declaration
        signature: "public final class SessionProcessor {"
        children:
        - id: SessionProcessor.49o8fN
          type: statements
        - id: SessionProcessor.SessionProcessor
          type: constructor_declaration
          signature: "public SessionProcessor() {"
        - id: SessionProcessor.SessionProcessor_1
          type: constructor_declaration
          signature: "public SessionProcessor(IIncludeAdapter adapter) {"
        - id: SessionProcessor.F0JmSu
          type: statements
        - id: SessionProcessor.setAdapter
          type: method_declaration
          signature: "public void setAdapter(IIncludeAdapter adapter) {"
        - id: SessionProcessor.H8L24o
          type: statements
        - id: SessionProcessor.setEnabled
          type: method_declaration
          signature: "public void setEnabled(boolean enabled) {"
        - id: SessionProcessor.G1iqGW
          type: statements
        - id: SessionProcessor.process
          type: method_declaration
          signature: "public <M> List<M> process(List<String> inputs, SessionCallbacks<M> callbacks) {"
        - id: SessionProcessor.onBBkU
          type: statements
        - id: SessionProcessor.process_1
          type: method_declaration
          signature: "public <M> List<M> process(String input, Path rootPath, SessionCallbacks<M> cal…"
        - id: SessionProcessor.process_2
          type: method_declaration
          signature: "public <M> List<M> process(String input, SessionCallbacks<M> callbacks) {"
        - id: SessionProcessor.u9akBQ
          type: statements
        - id: SessionProcessor.Run
          type: class_declaration
          signature: "private final class Run<M> {"
          children:
          - id: SessionProcessor.Run.wxJNjN
            type: statements
          - id: SessionProcessor.Run.Run
            type: constructor_declaration
            signature: "Run(SessionCallbacks<M> callbacks, List<M> out) {"
          - id: SessionProcessor.Run.run
            type: method_declaration
            signature: "void run(String text, List<Path> chain) {"
          - id: SessionProcessor.Run.flush
            type: method_declaration
            signature: "private void flush() {"
          - id: SessionProcessor.Run.isMarkerLine
            type: method_declaration
            signature: "private boolean isMarkerLine(String line) {"
          - id: SessionProcessor.Run.consumeReasoning
            type: method_declaration
            signature: "private int consumeReasoning(String[] lines, int start) {"
          - id: SessionProcessor.Run.consumeText
            type: method_declaration
            signature: "private int consumeText(String[] lines, int start) {"
          - id: SessionProcessor.Run.consumeToolCall
            type: method_declaration
            signature: "private int consumeToolCall(String[] lines, int start) {"
          - id: SessionProcessor.Run.consumeToolResult
            type: method_declaration
            signature: "private int consumeToolResult(String[] lines, int start) {"
          - id: SessionProcessor.Run.7Jclc5
            type: statements
          - id: SessionProcessor.Run.fenceRange
            type: method_declaration
            signature: "private int[] fenceRange(String[] lines, int from) {"
          - id: SessionProcessor.Run.readYaml
            type: method_declaration
            signature: "private JsonNode readYaml(String[] lines, int[] range) {"
          - id: SessionProcessor.Run.include
            type: method_declaration
            signature: "private void include(String kind, String arg, List<Path> chain) {"
          - id: SessionProcessor.Run.lM1IBJ
            type: statements
          - id: SessionProcessor.Run.includePath
            type: method_declaration
            signature: "private void includePath(String pathText, List<Path> chain) {"
          - id: SessionProcessor.Run.includeContextPrompt
            type: method_declaration
            signature: "private void includeContextPrompt(String dir) {"
          - id: SessionProcessor.Run.includeFiles
            type: method_declaration
            signature: "private void includeFiles(String arg) {"
          - id: SessionProcessor.Run.includeFile
            type: method_declaration
            signature: "private void includeFile(String path) {"
          - id: SessionProcessor.Run.includeSearch
            type: method_declaration
            signature: "private void includeSearch(String arg) {"
          - id: SessionProcessor.Run.renderMatches
            type: method_declaration
            signature: "private String renderMatches(List<IIncludeAdapter.Match> matches) {"
          - id: SessionProcessor.Run.requireAdapter
            type: method_declaration
            signature: "private IIncludeAdapter requireAdapter() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: sGAUbf
        type: imports
      - id: MCPConnector
        type: class_declaration
        signature: "public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {"
        children:
        - id: MCPConnector.xV4uSy
          type: statements
        - id: MCPConnector.MCPConnector
          type: constructor_declaration
          signature: "public MCPConnector(ConfigManager cfg, MCPClient client) {"
        - id: MCPConnector.getSupportedKeyPattern
          type: method_declaration
          signature: '@Override'
        - id: MCPConnector.createRequest
          type: method_declaration
          signature: '@Override'
        - id: MCPConnector.executeRequest
          type: method_declaration
          signature: '@Override'
        - id: MCPConnector.convertResponse
          type: method_declaration
          signature: '@Override'
        - id: MCPConnector.preprocess
          type: method_declaration
          signature: "private MCPCommand preprocess(List<String> inputs) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
      nodes:
      - id: rhl4eY
        type: imports
      - id: udCxJD
        type: statements
      - id: MCPControlClient
        type: class_declaration
        signature: "public class MCPControlClient {"
        children:
        - id: MCPControlClient.yKZBpv
          type: statements
        - id: MCPControlClient.renderSchema
          type: method_declaration
          signature: '@SuppressWarnings("deprecation")'
        - id: MCPControlClient.tHZasV
          type: statements
        - id: MCPControlClient.extractYamlBlock
          type: method_declaration
          signature: "public String extractYamlBlock(String text) {"
        - id: MCPControlClient.parseYaml
          type: method_declaration
          signature: "public JsonNode parseYaml(String block) {"
        - id: MCPControlClient.prettyResult
          type: method_declaration
          signature: "public String prettyResult(JsonNode result) {"
        - id: MCPControlClient.sVRL8A
          type: statements
        - id: MCPControlClient.extractText
          type: method_declaration
          signature: "public String extractText(JsonNode content) {"
        - id: MCPControlClient.L43zbT
          type: statements
        - id: MCPControlClient.fillReason
          type: method_declaration
          signature: "public JsonNode fillReason(JsonNode tool, JsonNode arguments) {"
        - id: MCPControlClient.appendProperty
          type: method_declaration
          signature: "private void appendProperty(StringBuilder sb, String key, JsonNode prop, boolea…"
        - id: MCPControlClient.renderValue
          type: method_declaration
          signature: "private String renderValue(JsonNode prop, String type, String indent) {"
        - id: MCPControlClient.cn5YCc
          type: statements
        - id: MCPControlClient.renderArrayValue
          type: method_declaration
          signature: '@SuppressWarnings("deprecation")'
        - id: MCPControlClient.requiredSet
          type: method_declaration
          signature: "private Set<String> requiredSet(JsonNode schema) {"
        - id: MCPControlClient.appendComment
          type: method_declaration
          signature: "private void appendComment(StringBuilder sb, String indent, String text) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AdaptingConnector.java
      nodes:
      - id: DHrxIx
        type: imports
      - id: Z6B0EE
        type: imports
      - id: vjpmWe
        type: imports
      - id: 3Kg6lR
        type: imports
      - id: AdaptingConnector
        type: class_declaration
        signature: "public class AdaptingConnector implements IAIConnector<IModelRequest, IModelRes…"
        children:
        - id: AdaptingConnector.xrm1T7
          type: statements
        - id: AdaptingConnector.AdaptingConnector
          type: constructor_declaration
          signature: "public AdaptingConnector(ConfigManager cfg, CCSessionManager sessionManager, MC…"
        - id: AdaptingConnector.getSupportedKeyPattern
          type: method_declaration
          signature: '@Override'
        - id: AdaptingConnector.getConnector
          type: method_declaration
          signature: "private IAIConnector<? extends IModelRequest, ? extends IModelResponse> getConn…"
        - id: AdaptingConnector.getBatchConnector
          type: method_declaration
          signature: "public IAIBatchConnector getBatchConnector(Model model) {"
        - id: AdaptingConnector.getConnector_1
          type: method_declaration
          signature: '@SuppressWarnings("rawtypes")'
        - id: AdaptingConnector.getConnector_2
          type: method_declaration
          signature: '@SuppressWarnings("rawtypes")'
        - id: AdaptingConnector.getBatchConnector_1
          type: method_declaration
          signature: "private IAIBatchConnector getBatchConnector(IModelRequest request) {"
        - id: AdaptingConnector.getBatchConnector_2
          type: method_declaration
          signature: "private IAIBatchConnector getBatchConnector(IAIBatch entry) {"
        - id: AdaptingConnector.createRequest
          type: method_declaration
          signature: '@Override'
        - id: AdaptingConnector.updateBatches
          type: method_declaration
          signature: '@Override'
        - id: AdaptingConnector.submitBatch
          type: method_declaration
          signature: '@Override'
        - id: AdaptingConnector.executeRequest
          type: method_declaration
          signature: '@SuppressWarnings("unchecked")'
        - id: AdaptingConnector.requestsToJson
          type: method_declaration
          signature: '@Override'
        - id: AdaptingConnector.convertResponse
          type: method_declaration
          signature: '@SuppressWarnings("unchecked")'
        - id: AdaptingConnector.cancelBatch
          type: method_declaration
          signature: '@Override'
        - id: AdaptingConnector.loadBatch
          type: method_declaration
          signature: '@Override'
        - id: AdaptingConnector.convertAnswers
          type: method_declaration
          signature: '@Override'
```
/answer 7fa5-13 allow
ReasoningToken: 77
Text:
 Nun der Session-Processor, MCP-Connector/-ControlClient und AdaptingConnector.
ReasoningToken: 88
Control Request:
```yaml
id: 7fa5-14
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
    ids:
    - SessionProcessor.process
    - SessionProcessor.setEnabled
    - SessionProcessor.Run.run
    - SessionProcessor.Run.isMarkerLine
    - SessionProcessor.Run.consumeToolCall
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - MCPConnector.preprocess
    - MCPConnector.createRequest
    - MCPConnector.executeRequest
  reason: Marker-Erkennung im SessionProcessor und MCP-Command-Verarbeitung verstehen
```
/answer 7fa5-14 allow
Control Request:
```yaml
id: 7fa5-15
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
      nodes:
      - id: SessionProcessor.process
        type: method_declaration
        code: "public <M> List<M> process(List<String> inputs, SessionCallbacks<M> callbacks) {\n\t\tList<M> out = new ArrayList<>();\n\t\tif (inputs == null)\n\t\t\treturn out;\n\t\tif (!enabled) {\n\t\t\tString joined = inputs.stream().filter(i -> i != null && !i.isBlank())\n\t\t\t\t\t.collect(java.util.stream.Collectors.joining(\"\\n\"));\n\t\t\tif (!joined.isBlank())\n\t\t\t\tout.add(callbacks.message(Role.User, joined));\n\t\t\treturn out;\n\t\t}\n\t\tfor (String input : inputs)\n\t\t\tif (input != null && !input.isBlank())\n\t\t\t\tnew Run<>(callbacks, out).run(input, List.of());\n\t\treturn out;\n\t}"
      - id: SessionProcessor.setEnabled
        type: method_declaration
        code: "public void setEnabled(boolean enabled) {\n\t\tthis.enabled = enabled;\n\t}"
      - id: SessionProcessor.Run.run
        type: method_declaration
        code: "void run(String text, List<Path> chain) {\n\t\t\tString[] lines = text == null ? new String[0] : text.split(\"\\n\", -1);\n\t\t\tint i = 0;\n\t\t\twhile (i < lines.length) {\n\t\t\t\tString line = lines[i];\n\t\t\t\tString stripped = line.strip();\n\n\t\t\t\tif (INCLUDE_LINE.matcher(line).matches()) {\n\t\t\t\t\tflush();\n\t\t\t\t\tMatcher m = INCLUDE_TAG.matcher(line);\n\t\t\t\t\twhile (m.find())\n\t\t\t\t\t\tinclude(m.group(1), m.group(2).strip(), chain);\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.USER)) {\n\t\t\t\t\tflush();\n\t\t\t\t\trole = Role.User;\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.AGENT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\trole = Role.Agent;\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.THINKING)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeReasoning(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.TEXT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeText(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.TOOLUSE)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeToolCall(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.TOOLRESULT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeToolResult(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\n\t\t\t\tif (buffer.length() > 0)\n\t\t\t\t\tbuffer.append(\"\\n\");\n\t\t\t\tbuffer.append(line);\n\t\t\t\ti++;\n\t\t\t}\n\t\t\tflush();\n\t\t}"
      - id: SessionProcessor.Run.isMarkerLine
        type: method_declaration
        code: "private boolean isMarkerLine(String line) {\n\t\t\tString s = line.strip();\n\t\t\treturn INCLUDE_LINE.matcher(line).matches() //\n\t\t\t\t\t|| s.equals(EditorInterface.USER) || s.equals(EditorInterface.AGENT) //\n\t\t\t\t\t|| s.equals(EditorInterface.THINKING) || s.equals(EditorInterface.TEXT) //\n\t\t\t\t\t|| s.equals(EditorInterface.TOOLUSE) || s.equals(EditorInterface.TOOLRESULT);\n\t\t}"
      - id: SessionProcessor.Run.consumeToolCall
        type: method_declaration
        code: "private int consumeToolCall(String[] lines, int start) {\n\t\t\tint[] range = fenceRange(lines, start);\n\t\t\tif (range == null)\n\t\t\t\treturn start;\n\t\t\tJsonNode node = readYaml(lines, range);\n\t\t\tout.add(callbacks.toolCall(\n\t\t\t\t\tnew ToolCall(node.path(\"id\").asText(\"\"), node.path(\"tool\").asText(\"\"), node.path(\"arguments\"))));\n\t\t\treturn range[1] + 1;\n\t\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: MCPConnector.preprocess
        type: method_declaration
        code: "private MCPCommand preprocess(List<String> inputs) {\n\t\tStringBuilder merged = null;\n\t\tfor (String input : inputs) {\n\t\t\tString clean = input != null ? input.strip() : \"\";\n\t\t\tif (clean.isBlank())\n\t\t\t\tcontinue;\n\t\t\tif (\"/exit\".equalsIgnoreCase(clean))\n\t\t\t\treturn new MCPCommand(MCPCommandType.Exit, \"\");\n\t\t\tif (clean.matches(\"(?i)/tool\\\\s+\\\\S+\"))\n\t\t\t\treturn new MCPCommand(MCPCommandType.Tool, clean.split(\"\\\\s+\", 2)[1].strip());\n\t\t\tif (merged == null)\n\t\t\t\tmerged = new StringBuilder();\n\t\t\telse\n\t\t\t\tmerged.append(\"\\n\");\n\t\t\tmerged.append(clean);\n\t\t}\n\t\tif (merged == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn new MCPCommand(MCPCommandType.Prompt, merged.toString());\n\t}"
      - id: MCPConnector.createRequest
        type: method_declaration
        code: "@Override\n\tpublic MCPRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,\n\t\t\tIProgressMonitor mon) {\n\t\treturn new MCPRequest(UUID.randomUUID().toString(), systemPrompt, tools, preprocess(inputs));\n\t}"
      - id: MCPConnector.executeRequest
        type: method_declaration
        code: "@Override\n\tpublic MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job) {\n\t\tswitch (req.cmd.type) {\n\t\tcase Exit:\n\t\t\tclient.close();\n\t\t\treturn new MCPResponse(req.id, \"MCP connection closed\");\n\t\tcase Tool: {\n\t\t\tJsonNode tool = client.findTool(req.cmd.parameter);\n\t\t\tif (tool == null)\n\t\t\t\tthrow new IllegalArgumentException(\"Unknown tool: \" + req.cmd.parameter);\n\t\t\treturn new MCPResponse(req.id, control.renderSchema(tool));\n\t\t}\n\t\tcase Prompt: {\n\t\t\tString block = control.extractYamlBlock(req.cmd.parameter);\n\t\t\tif (block == null)\n\t\t\t\tthrow new IllegalArgumentException(\n\t\t\t\t\t\t\"No ```yaml tool call block found. Use \\\"/tool <id>\\\" to get a template.\");\n\t\t\tJsonNode call = control.parseYaml(block);\n\t\t\tString name = call.path(\"tool\").asText(null);\n\t\t\tif (name == null || name.isBlank())\n\t\t\t\tthrow new IllegalArgumentException(\"Tool call is missing the 'tool' field\");\n\t\t\tJsonNode tool = client.findTool(name);\n\t\t\tJsonNode arguments = control.fillReason(tool, call.path(\"arguments\"));\n\t\t\tJsonNode result = client.callTool(name, arguments);\n\t\t\treturn new MCPResponse(req.id, control.prettyResult(result));\n\t\t}\n\t\tdefault:\n\t\t\tthrow new UnsupportedOperationException(\"Unsupported command: \" + req.cmd.type);\n\t\t}\n\t}"
```
/answer 7fa5-15 allow
Control Request:
```yaml
id: 7fa5-16
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
    ids:
    - MCPControlClient.renderSchema
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/IAIConnector.java
    ids:
    - IAIConnector
  reason: Template-Rendering für /call und Connector-Interface
```
/answer 7fa5-16 allow
Control Request:
```yaml
id: 7fa5-17
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
      nodes:
      - id: MCPControlClient.renderSchema
        type: method_declaration
        code: "@SuppressWarnings(\"deprecation\")\n\tpublic String renderSchema(JsonNode tool) {\n\t\tStringBuilder sb = new StringBuilder();\n\t\tsb.append(\"```yaml\\n\");\n\t\tsb.append(\"tool: \").append(tool.path(\"name\").asText()).append(\"\\n\");\n\n\t\tJsonNode schema = tool.path(\"inputSchema\");\n\t\tJsonNode props = schema.path(\"properties\");\n\t\tSet<String> required = requiredSet(schema);\n\t\tStringBuilder argsBody = new StringBuilder();\n\t\tif (props.isObject()) {\n\t\t\tIterator<Map.Entry<String, JsonNode>> it = props.fields();\n\t\t\twhile (it.hasNext()) {\n\t\t\t\tMap.Entry<String, JsonNode> e = it.next();\n\t\t\t\tif (\"reason\".equals(e.getKey()))\n\t\t\t\t\tcontinue;\n\t\t\t\tappendProperty(argsBody, e.getKey(), e.getValue(), required.contains(e.getKey()));\n\t\t\t}\n\t\t}\n\t\tif (argsBody.length() > 0) {\n\t\t\tsb.append(\"arguments:\\n\").append(argsBody);\n\t\t} else {\n\t\t\tsb.append(\"arguments: {}\\n\");\n\t\t}\n\t\tsb.append(\"```\");\n\t\treturn sb.toString();\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/IAIConnector.java
      nodes:
      - id: IAIConnector
        type: interface_declaration
        signature: "public interface IAIConnector<REQ extends IModelRequest, RESP extends IModelRes…"
        children:
        - id: IAIConnector.createRequest
          type: method_declaration
          code: "REQ createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,\n\t\t\tIProgressMonitor mon);"
        - id: IAIConnector.executeRequest
          type: method_declaration
          code: "RESP executeRequest(REQ req, IProgressMonitor mon, Job job);"
        - id: IAIConnector.convertResponse
          type: method_declaration
          code: "AIAnswer convertResponse(RESP resp, IProgressMonitor mon);"
        - id: IAIConnector.getSupportedKeyPattern
          type: method_declaration
          code: KeyPattern getSupportedKeyPattern();
```
/answer 7fa5-17 allow
ReasoningToken: 165
Control Request:
```yaml
id: 7fa5-18
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: public .* (getModel|getReasoning|getProfile|getKeys|getCacheMode|getTools|getSystemPrompt|getFreeText|getTopP|getTemperature|getMaxTokens|isInputEnabled)\(
    include:
    - ConfigManager.java
    limit: 40
  reason: Config-Getter für das Einfrieren der Konfiguration im Prompt-Objekt bestätigen
```
/answer 7fa5-18 allow
Control Request:
```yaml
id: 7fa5-19
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      pattern: public .* (getModel|getReasoning|getProfile|getKeys|getCacheMode|getTools|getSystemPrompt|getFreeText|getTopP|getTemperature|getMaxTokens|isInputEnabled)\(
      matches:
      - path: ConfigManager.java
        lineno: 154
        match: "\tpublic String getKeys() {"
      - path: ConfigManager.java
        lineno: 162
        match: "\tpublic Double getTemperature() {"
      - path: ConfigManager.java
        lineno: 166
        match: "\tpublic Double getTopP() {"
      - path: ConfigManager.java
        lineno: 170
        match: "\tpublic Model getModel() {"
      - path: ConfigManager.java
        lineno: 174
        match: "\tpublic AgentProfile getProfile() {"
      - path: ConfigManager.java
        lineno: 178
        match: "\tpublic Reasoning getReasoning() {"
      - path: ConfigManager.java
        lineno: 182
        match: "\tpublic CacheMode getCacheMode() {"
      - path: ConfigManager.java
        lineno: 186
        match: "\tpublic String[] getSystemPrompt() {"
      - path: ConfigManager.java
        lineno: 190
        match: "\tpublic String getFreeText() {"
      - path: ConfigManager.java
        lineno: 297
        match: "\tpublic boolean isInputEnabled(InputMode mode) {"
      - path: ConfigManager.java
        lineno: 415
        match: "\tpublic String[] getTools() {"
```
/answer 7fa5-19 allow Das einfrieren der Konfiguration ist ebenso ein isolierbarer Breich des Promptes.
ReasoningToken: 126
ReasoningToken: 1100
Result Stats:  id=ea8143ba-d655-4aff-a0b8-83dffb2c226a, total: 0, in: 133, out: 10263, reason: 0, read: 215017, write: 49742
Analyse abgeschlossen. Hier der restrukturierte, sequentielle Umsetzungsplan.

---

# Umsetzungsplan: Vereinheitlichung Slash-Command-System

## Ausgangslage (Analyse)

**Kommando-Logik ist heute verstreut & dupliziert:**
- `CCConnector.preprocessInput`: Regex für `/exit`, `/resume`, `/answer … allow|deny`, YAML-Modification, sonst `Prompt`.
- `MCPConnector.preprocess`: `/exit`, `/tool`, sonst `Prompt`.
- Konstanten geteilt uneinheitlich: `CCControlClient.ANSWER="/answer"`, `CONTROL_REQUEST="Control Request:"`; Marker in `EditorInterface` (`USER/AGENT/TEXT/THINKING/TOOLUSE="Tool:"/TOOLRESULT`).
- `SessionProcessor.Run.run` kennt nur Marker, **keine** Kommandos → `/answer`/`Control Request:` werden derzeit nicht semantisch behandelt.

**Prompt-Logik liegt in `AISessionManager`:** `getInput` (Selection/Converter/SystemPrompt/Tools), `prepareInner` (friert Inputs ein, setzt `sessionProcessor.setEnabled(...)` direkt anhand `InputMode.Converter`), `PromptJob`/`execute`/`queueSync`.

**Config-Fixierung:** `ClaudeSessionParameters` vermischt generische Parameter (cwd, filePath, model, reasoning, profile, tools, systemPrompt, hash) mit CC-spezifischem CLI-/Env-Aufbau.

**Connector-API:** `IAIConnector.createRequest(inputs, systemPrompt, tools, batchFix, mon)` – einzelne Parameter statt Prompt-Objekt.

**MCP-Template:** `MCPControlClient.renderSchema` liefert `\`\`\`yaml … \`\`\``, ohne `Tool:`-Präfix und ohne `/call`-Kommando.

---

## Phase 0 — Globale Kommandosyntax (Grundlage, keine Verhaltensänderung)
1. Neue Konstanten-Quelle für Kommando-Präfixe (in `EditorInterface` bei den Marker-Konstanten, symmetrisch): `/answer`, `/resume`, `/exit`, `/call`, `/tool`, sowie `CONTROL_REQUEST="Control Request:"`. `CCControlClient.ANSWER/CONTROL_REQUEST` auf diese zentralen Konstanten umbiegen (Kompatibilität behalten).

## Phase 1 — `SessionParameters` extrahieren (isoliert, testbar)
2. Neue Basisklasse `connector/SessionParameters.java`: generische Felder (cwd/`filePath`, model, reasoning, profile, tools, systemPrompt, topP, temperature, maxTokens, cacheMode) + `computeHash()`/`getHash()` (analog bestehendem MD5-Mechanismus, Parameter-Superset).
3. `ClaudeSessionParameters extends SessionParameters`: nur noch CC-Spezifika (`buildBaseCommand`, `buildEvironment`, `SCRIPT/COMMAND`, `title`). Hash/Felder erben.
4. Aufrufer (`CCConnector.executeRequest`, `ClaudeSessionParameters.fromConfig`) anpassen. **Validieren.**

## Phase 2 — Globaler Command Handler + Command-Hierarchie
5. `connector/Command.java` (abstrakte Oberklasse): Feld `type`, `parameters`; abstrakte/gemeinsame API: `matches(String line)`, `parse(...)`, und **Prozessor-Semantik** `ProcessorAction` ∈ {`REMOVE` (z. B. `/resume`, `/exit`, `/call`), `TRANSFORM` (`/answer`: Kommando+Option entfernen, Resttext behalten, Block beenden), `IGNORE_BLOCK` (`Control Request:` inkl. folgendem YAML)}.
6. Konkrete Command-Klassen: `AnswerCommand`, `ControlRequestCommand`, `ResumeCommand`, `ExitCommand`, `ToolCommand`, `CallCommand`, `PromptCommand`, `ModificationCommand`.
7. `connector/CommandHandler.java`: statische, hart kodierte Registry der Commands; API `detect(line)`/`list()`. Wird von SessionProcessor und PromptHandler genutzt.
8. `CommandType` (claudecode) auf gemeinsame Nutzung heben bzw. durch die neue Hierarchie ersetzen; `CCConnector.preprocessInput` und `MCPConnector.preprocess` auf `CommandHandler` umstellen (Verhalten unverändert). **Validieren.**

## Phase 3 — SessionProcessor an Registry anbinden
9. `SessionProcessor.Run.run`: vor Marker-Prüfung Registry konsultieren →
   - `Control Request:` + folgender `\`\`\`yaml\`\`\``-Block: **ignorieren** (Zeile + Block überspringen).
   - `/answer … allow|deny [text]`: Block flushen (schließt vorausgehenden Tool-Call/Message-Block), Kommando+Option entfernen, optionalen Resttext als Message behalten.
   - `/resume`, `/exit`, `/call`: **entfernen** (nur Kontext).
10. Determinismus/Prefix-Caching-Symmetrie prüfen: Ein-/Ausgaben bleiben austauschbar. **Validieren.**

## Phase 4 — FrozenConfig + Prompt-Objekt (`connector/harness`)
11. `connector/harness/FrozenConfig.java`: eingefrorene Konfiguration (Model, Profil, Reasoning, Tools, SystemPrompt, TopP, Temperature, MaxTokens) + Hash. Erzeugung via `from(ConfigManager)`.
12. `connector/harness/Prompt.java`: friert Trigger + Eingaben ein — semantisch getrennt: **Inputs** (Textbereich/Selection/Converter), **Batch-Flag**, **FrozenConfig**. Zusätzlich: Aktivierungszustand des SessionProcessor als Prompt-Feld (statt direktem `setEnabled`). Session-ID = Hash(absoluter Dateipfad, Batch-ja/nein, `FrozenConfig.getHash()`).

## Phase 5 — PromptHandler (Auslagerung aus `AISessionManager`)
13. `connector/harness/PromptHandler.java`: übernimmt `getInput`, `prepareInner`-Eingabeauswahl, `execute`/`PromptJob`/`queueSync`. Empfängt Trigger, baut `Prompt`.
14. **Kommandoerkennung nach Eingabemodus** (aus Aufgabenteil „Full Editor / Selection Input Mode“):
    - Zeileneingabe: aktuelle Zeile auf Kommando prüfen.
    - Blockauswahl: erste Zeile (mehrzeilig) oder Kommando in letzter Zeile.
    - Gesamtdatei/Processor: zuerst Cursorzeile, dann letzte Dateizeile; sonstige Kommandos = nur Kontext.
15. `AISessionManager` delegiert an `PromptHandler`; `sessionProcessor.setEnabled` wird über das Prompt-Objekt gesteuert (kein direkter Aufruf mehr) → Multi-Session-tauglich. **Validieren.**

## Phase 6 — Connectoren auf Prompt-Objekt umstellen
16. `IAIConnector.createRequest(...)` → `createRequest(Prompt prompt, IProgressMonitor mon)`.
17. Umstellen: `AdaptingConnector`, `CCConnector`, `MCPConnector`, `ClaudeConnector`, `DeepSeekConnector`, `GeminiConnector`, `OpenAIConnector`. Direkte `cfg`-Zugriffe der Connectoren durch `prompt.frozenConfig()` ersetzen; `ClaudeSessionParameters.fromConfig` durch Ableitung aus `FrozenConfig`. **Nach jedem Connector validieren.**

## Phase 7 — `/call`-Kommando (MCP)
18. `MCPControlClient.renderSchema`: Template mit `Tool:`-Präfix (`EditorInterface.TOOLUSE`) voranstellen und `/call`-Zeile anfügen.
19. `PromptHandler`: `/call` erkennen, rückwärts ab Kommandozeile den vorangehenden `^\`\`\`yaml$`-Block erfassen, typisieren, via `Prompt` an MCP-Client übergeben.
20. `SessionProcessor`: `Tool:`-Block als Tool-Call in korrekten Message-Typ wandeln, `/call`-Zeile ignorieren (bereits durch Phase 3 Registry). **Validieren.**

## Phase 8 — Abschluss
21. Alte, nun redundante Preprocess-Regex/`CommandType`-Reste entfernen. Gesamtvalidierung aller berührten Dateien.

---

**Reihenfolge-Logik:** Konstanten (0) → datentragende Extraktion (1) → Command-Infrastruktur (2) → Prozessor-Semantik (3) → Prompt-Datenmodell (4) → Handler-Auslagerung (5) → API-Umstellung (6) → `/call` als konkrete Anwendung (7) → Cleanup (8). Jede Phase ist einzeln compilierbar/validierbar; harte Abhängigkeiten laufen strikt vorwärts.

Soll ich mit **Phase 0/1** starten?
Lass mich das Kurz prüfen.
SystemInit:  id=ea8143ba-d655-4aff-a0b8-83dffb2c226a, cwd=/home/user/xyan/xy.ai.workbench, model=claude-opus-4-8
Result Stats:  id=ea8143ba-d655-4aff-a0b8-83dffb2c226a, total: 0, in: 135, out: 10285, reason: 0, read: 264759, write: 54395
Verstanden, ich warte auf deine Prüfung.
Ich möchte das du daraus einen Mehrteiligen Plan im Zielverzeichnis: `/home/user/xyan/xy.ai.workbench/project/plan1` erstellst.
Der Plan muss unabhängig von diesem Prompt funktionieren. Daher kopiere alle notwendigen Informationen sowie komplette Datei, Verzeichnis, Methoden und Knoten-ID Informationen in die Steps und das Retrieval zu beschleunigen. Der Plan wird separate durch Agenten umgesetzt.
SystemInit:  id=ea8143ba-d655-4aff-a0b8-83dffb2c226a, cwd=/home/user/xyan/xy.ai.workbench, model=claude-opus-4-8
Text:
 Ich sammle die noch fehlenden Node-IDs und Signaturen, damit die Plan-Steps eigenständig sind.
Control Request:
```yaml
id: 7fa5-1a
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claude/ClaudeConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/deepseek/DeepSeekConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/google/GeminiConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/openai/OpenAIConnector.java
  reason: Node-IDs und Struktur der vier API-Connectoren für Prompt-Objekt-Umstellung
```
/answer 7fa5-1a allow
Control Request:
```yaml
id: 7fa5-1b
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claude/ClaudeConnector.java
      nodes:
      - id: umw4M8
        type: imports
      - id: 6veoan
        type: imports
      - id: lELuBq
        type: imports
      - id: ClaudeConnector
        type: class_declaration
        signature: "public class ClaudeConnector implements IAIConnector<ClaudeRequest, ClaudeRespo…"
        children:
        - id: ClaudeConnector.jfdJa3
          type: statements
        - id: ClaudeConnector.ClaudeConnector
          type: constructor_declaration
          signature: "public ClaudeConnector(ConfigManager cfg, MCPClient mcpClient, SessionProcessor…"
        - id: ClaudeConnector.getSupportedKeyPattern
          type: method_declaration
          signature: '@Override'
        - id: ClaudeConnector.createRequest
          type: method_declaration
          signature: '@SuppressWarnings("deprecation")'
        - id: ClaudeConnector.appendTools
          type: method_declaration
          signature: "private void appendTools(Builder builder, List<String> tools) {"
        - id: ClaudeConnector.executeRequest
          type: method_declaration
          signature: '@Override'
        - id: ClaudeConnector.convertResponse
          type: method_declaration
          signature: '@Override'
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/deepseek/DeepSeekConnector.java
      nodes:
      - id: t80dJw
        type: imports
      - id: pdkpbV
        type: imports
      - id: 0usLAl
        type: imports
      - id: TOApxM
        type: imports
      - id: g5DQTk
        type: imports
      - id: tfIw6g
        type: imports
      - id: 4qpNbW
        type: imports
      - id: DKgut1
        type: imports
      - id: NUfXn4
        type: imports
      - id: xJYUN4
        type: imports
      - id: ojQ35Q
        type: statements
      - id: DeepSeekConnector
        type: class_declaration
        signature: "public class DeepSeekConnector implements IAIConnector<DeepSeekRequest, DeepSee…"
        children:
        - id: DeepSeekConnector.9l3Pbm
          type: statements
        - id: DeepSeekConnector.DeepSeekConnector
          type: constructor_declaration
          signature: "public DeepSeekConnector(ConfigManager cfg, MCPClient mcpClient, SessionProcess…"
        - id: DeepSeekConnector.getSupportedKeyPattern
          type: method_declaration
          signature: '@Override'
        - id: DeepSeekConnector.createRequest
          type: method_declaration
          signature: '@Override'
        - id: DeepSeekConnector.appendTools
          type: method_declaration
          signature: "private void appendTools(ObjectMapper mapper, AllOfBody requestBody, List<Strin…"
        - id: DeepSeekConnector.createMessage
          type: method_declaration
          signature: "private ObjectNode createMessage(ObjectMapper mapper, RoleEnum role, String tex…"
        - id: DeepSeekConnector.YmJgBe
          type: statements
        - id: DeepSeekConnector.SessionRequestCallbacks
          type: class_declaration
          signature: private final class SessionRequestCallbacks implements SessionCallbacks<ObjectN…
          children:
          - id: DeepSeekConnector.SessionRequestCallbacks.apm5zB
            type: statements
          - id: DeepSeekConnector.SessionRequestCallbacks.SessionRequestCallbacks
            type: constructor_declaration
            signature: "private SessionRequestCallbacks(ObjectMapper mapper) {"
          - id: DeepSeekConnector.SessionRequestCallbacks.message
            type: method_declaration
            signature: '@Override'
          - id: DeepSeekConnector.SessionRequestCallbacks.toolCall
            type: method_declaration
            signature: '@Override'
          - id: DeepSeekConnector.SessionRequestCallbacks.toolResult
            type: method_declaration
            signature: '@Override'
        - id: DeepSeekConnector.toEffort
          type: method_declaration
          signature: "private EffortEnum toEffort(Reasoning reasoning) {"
        - id: DeepSeekConnector.executeRequest
          type: method_declaration
          signature: '@Override'
        - id: DeepSeekConnector.convertResponse
          type: method_declaration
          signature: '@Override'
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/google/GeminiConnector.java
      nodes:
      - id: MpzmxB
        type: imports
      - id: fw3P4U
        type: imports
      - id: RWRJBj
        type: imports
      - id: 6q9SQi
        type: imports
      - id: GeminiConnector
        type: class_declaration
        signature: "public class GeminiConnector implements IAIConnector<GeminiRequest, GeminiRespo…"
        children:
        - id: GeminiConnector.gj1iLe
          type: statements
        - id: GeminiConnector.GeminiConnector
          type: constructor_declaration
          signature: "public GeminiConnector(ConfigManager cfg, MCPClient mcpClient, SessionProcessor…"
        - id: GeminiConnector.getSupportedKeyPattern
          type: method_declaration
          signature: '@Override'
        - id: GeminiConnector.createRequest
          type: method_declaration
          signature: '@Override'
        - id: GeminiConnector.appendTools
          type: method_declaration
          signature: "private void appendTools(Builder config, List<String> tools) {"
        - id: GeminiConnector.getThinkingBudget
          type: method_declaration
          signature: "private Integer getThinkingBudget(Reasoning reasoning, ConfigManager cfg2) {"
        - id: GeminiConnector.executeRequest
          type: method_declaration
          signature: '@Override'
        - id: GeminiConnector.convertResponse
          type: method_declaration
          signature: '@Override'
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/openai/OpenAIConnector.java
      nodes:
      - id: fEJBVt
        type: imports
      - id: H3472Y
        type: imports
      - id: G5wCUb
        type: imports
      - id: XppXth
        type: imports
      - id: OpenAIConnector
        type: class_declaration
        signature: "public class OpenAIConnector implements IAIConnector<OpenAIRequest, OpenAIRespo…"
        children:
        - id: OpenAIConnector.VD2jfd
          type: statements
        - id: OpenAIConnector.OpenAIConnector
          type: constructor_declaration
          signature: "public OpenAIConnector(ConfigManager cfg, MCPClient mcpClient, SessionProcessor…"
        - id: OpenAIConnector.getSupportedKeyPattern
          type: method_declaration
          signature: '@Override'
        - id: OpenAIConnector.createRequest
          type: method_declaration
          signature: '@SuppressWarnings("deprecation")'
        - id: OpenAIConnector.executeRequest
          type: method_declaration
          signature: '@Override'
        - id: OpenAIConnector.convertResponse
          type: method_declaration
          signature: '@Override'
        - id: OpenAIConnector.appendTools
          type: method_declaration
          signature: "private Builder appendTools(Builder builder, List<String> tools) {"
```
/answer 7fa5-1b allow
Control Request:
```yaml
id: 7fa5-1c
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/IAIConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java
  reason: "Interface, Wiring und MCP-Request/Client-Struktur für Plan-Retrieval"
```
/answer 7fa5-1c allow
Control Request:
```yaml
id: 7fa5-1d
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/IAIConnector.java
      nodes:
      - id: UYllGy
        type: imports
      - id: IAIConnector
        type: interface_declaration
        signature: "public interface IAIConnector<REQ extends IModelRequest, RESP extends IModelRes…"
        children:
        - id: IAIConnector.createRequest
          type: method_declaration
          signature: "REQ createRequest(List<String> inputs, String systemPrompt, List<String> tools,…"
        - id: IAIConnector.executeRequest
          type: method_declaration
          signature: "RESP executeRequest(REQ req, IProgressMonitor mon, Job job);"
        - id: IAIConnector.convertResponse
          type: method_declaration
          signature: "AIAnswer convertResponse(RESP resp, IProgressMonitor mon);"
        - id: IAIConnector.getSupportedKeyPattern
          type: method_declaration
          signature: KeyPattern getSupportedKeyPattern();
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
      nodes:
      - id: wOqaoM
        type: imports
      - id: 1FfhNT
        type: imports
      - id: sMGxXy
        type: statements
      - id: Activator
        type: class_declaration
        signature: "public class Activator extends AbstractUIPlugin {"
        children:
        - id: Activator.H6kbFx
          type: statements
        - id: Activator.T57XAK
          type: statements
        - id: Activator.start
          type: method_declaration
          signature: '@Override'
        - id: Activator.stop
          type: method_declaration
          signature: '@Override'
        - id: Activator.QOclzD
          type: statements
        - id: Activator.getDefault
          type: method_declaration
          signature: "public static Activator getDefault() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
      nodes:
      - id: svzMxJ
        type: imports
      - id: MCPRequest
        type: class_declaration
        signature: "public class MCPRequest implements IModelRequest {"
        children:
        - id: MCPRequest.41cj72
          type: statements
        - id: MCPRequest.MCPRequest
          type: constructor_declaration
          signature: "public MCPRequest(String id, String systemPrompt, List<String> tools, MCPComman…"
        - id: MCPRequest.getID
          type: method_declaration
          signature: '@Override'
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java
      nodes:
      - id: DLA8pY
        type: imports
      - id: uC4fPi
        type: imports
      - id: d9koxH
        type: statements
      - id: MCPClient
        type: class_declaration
        signature: "public class MCPClient {"
        children:
        - id: MCPClient.J8XhWN
          type: statements
        - id: MCPClient.UujMEH
          type: statements
        - id: MCPClient.findTool
          type: method_declaration
          signature: "public JsonNode findTool(String name) {"
        - id: MCPClient.listTools
          type: method_declaration
          signature: "public synchronized ArrayNode listTools() {"
        - id: MCPClient.tFfTSc
          type: statements
        - id: MCPClient.renderToolCall
          type: method_declaration
          signature: "public String renderToolCall(String name, JsonNode arguments) {"
        - id: MCPClient.callTool
          type: method_declaration
          signature: "public synchronized JsonNode callTool(String name, JsonNode arguments) {"
        - id: MCPClient.pLeYmp
          type: statements
        - id: MCPClient.extractErrorText
          type: method_declaration
          signature: "private String extractErrorText(JsonNode result) {"
        - id: MCPClient.close
          type: method_declaration
          signature: "public synchronized void close() {"
        - id: MCPClient.ensureConnected
          type: method_declaration
          signature: "private void ensureConnected() {"
        - id: MCPClient.send
          type: method_declaration
          signature: "private JsonNode send(ObjectNode rpc, boolean expectResult) {"
        - id: MCPClient.parseMessage
          type: method_declaration
          signature: private JsonNode parseMessage(HttpResponse<String> response) throws JsonProcess…
        - id: MCPClient.pickResponse
          type: method_declaration
          signature: "private JsonNode pickResponse(String json, JsonNode current) {"
        - id: MCPClient.request
          type: method_declaration
          signature: "private ObjectNode request(String method, JsonNode params) {"
        - id: MCPClient.notification
          type: method_declaration
          signature: "private ObjectNode notification(String method) {"
```
/answer 7fa5-1d allow
ReasoningToken: 570
Control Request:
```yaml
id: 7fa5-1e
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    ids:
    - EditorInterface.nZrzm0
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AdaptingConnector.java
    ids:
    - AdaptingConnector.createRequest
    - AdaptingConnector.executeRequest
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
    ids:
    - AISessionManager.BQ3GIF
    - AISessionManager.LdjI0C
    - AISessionManager.queueAsync
    - AISessionManager.queueAndSubmit
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    ids:
    - CCRequest.CCRequest
    - CCRequest.Ki6WvE
  reason: "Konstantenblock, Adapter-Request/Execute, SessionManager-Felder und CCRequest-Konstruktor für Plan-Retrieval"
```
/answer 7fa5-1e allow
Control Request:
```yaml
id: 7fa5-1f
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: EditorInterface.nZrzm0
        type: statements
        code: "public static final String USER = \"User:\";\n\tpublic static final String AGENT = \"Agent:\";\n\n\t/** Session block markers recognized by {@code connector.harness.SessionProcessor}. */\n\tpublic static final String TEXT = \"Text:\";\n\tpublic static final String THINKING = \"Thinking:\";\n\tpublic static final String TOOLUSE = \"Tool:\";\n\tpublic static final String TOOLRESULT = \"ToolResult:\";\n\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final AdaptingConnector connector;\n\tprivate final ConfigManager cfg;"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AdaptingConnector.java
      nodes:
      - id: AdaptingConnector.createRequest
        type: method_declaration
        code: "@Override\n\tpublic IModelRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix,\n\t\t\tIProgressMonitor mon) {\n\t\treturn getConnector(cfg.getModel()).createRequest(inputs, systemPrompt, tools, batchFix, mon);\n\t}"
      - id: AdaptingConnector.executeRequest
        type: method_declaration
        code: "@SuppressWarnings(\"unchecked\")\n\t@Override\n\tpublic IModelResponse executeRequest(IModelRequest request, IProgressMonitor mon, Job job) {\n\t\treturn getConnector(request).executeRequest(request, mon, job);\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
      nodes:
      - id: AISessionManager.BQ3GIF
        type: statements
        code: "public static final String CONTEXT_PROMPT_TXT = \"context.prompt.txt\";\n\n\tprivate ActiveEditorListener editorListener = new ActiveEditorListener(this);\n\tprivate IncludeAdapter includeAdapter;\n\n\tprivate final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\tprivate final MCPClient mcpClient;\n\tprivate final SessionProcessor sessionProcessor;\n\tpublic final EditorInterface editIfc;\n\tprivate int[] inputStats = new int[InputMode.values().length];"
      - id: AISessionManager.LdjI0C
        type: statements
        code: "private List<Consumer<AIAnswer>> answerObs = new ArrayList<>();\n\tprivate List<Consumer<int[]>> inputStatObs = new ArrayList<>();"
      - id: AISessionManager.queueAsync
        type: method_declaration
        code: "public void queueAsync(Display display, AIBatchManager batch) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, batch, mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
      - id: AISessionManager.queueAndSubmit
        type: method_declaration
        code: "public void queueAndSubmit(Display display, AIBatchManager batch) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, batch, mon);\n\t\t\t\tbatch.submitBatches(mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: CCRequest.CCRequest
        type: constructor_declaration
        code: "public CCRequest(String id, String title, String systemPrompt, List<String> tools, Command cmd) {\n\t\tObjects.requireNonNull(cmd, \"Command can't be null\");\n\t\tthis.id = id;\n\t\tthis.title = title;\n\t\tthis.systemPrompt = systemPrompt;\n\t\tthis.tools = tools != null ? tools : Collections.emptyList();\n\t\tthis.cmd = cmd;\n\t}"
      - id: CCRequest.Ki6WvE
        type: statements
        code: "public final String id;\n\tpublic final String title;\n\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final Command cmd;"
```
/answer 7fa5-1f allow
ReasoningToken: 177
Control Request:
```yaml
id: 7fa5-20
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp
    pattern: enum MCPCommandType|class MCPCommand|MCPCommandType\.|new MCPCommand
    include:
    - '*.java'
    limit: 20
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: public static final String (ANSWER|CONTROL_REQUEST)
    include:
    - CCControlClient.java
    limit: 5
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: "public (String|Double|Model|AgentProfile|Reasoning|CacheMode|String\\[\\]) (getTemperature|getTopP|getModel|getProfile|getReasoning|getCacheMode|getSystemPrompt|getKeys|getTools)\\("
    include:
    - ConfigManager.java
    limit: 20
  reason: "MCP-Command-Typen, CC-Konstanten und Config-Getter-Signaturen für vollständige Plan-Steps"
```
/answer 7fa5-20 allow
Control Request:
```yaml
id: 7fa5-21
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp
      pattern: enum MCPCommandType|class MCPCommand|MCPCommandType\.|new MCPCommand
      matches:
      - path: MCPCommandType.java
        lineno: 3
        match: "public enum MCPCommandType {"
      - path: MCPCommand.java
        lineno: 5
        match: "public class MCPCommand {"
      - path: MCPConnector.java
        lineno: 84
        match: "\t\t\t\treturn new MCPCommand(MCPCommandType.Exit, \"\");"
      - path: MCPConnector.java
        lineno: 86
        match: "\t\t\t\treturn new MCPCommand(MCPCommandType.Tool, clean.split(\"\\\\s+\", 2)[1].strip());"
      - path: MCPConnector.java
        lineno: 95
        match: "\t\treturn new MCPCommand(MCPCommandType.Prompt, merged.toString());"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      pattern: public static final String (ANSWER|CONTROL_REQUEST)
      matches:
      - path: CCControlClient.java
        lineno: 32
        match: "\tpublic static final String ANSWER = \"/answer\";"
      - path: CCControlClient.java
        lineno: 33
        match: "\tpublic static final String CONTROL_REQUEST = \"Control Request:\";"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      pattern: "public (String|Double|Model|AgentProfile|Reasoning|CacheMode|String\\[\\]) (getTemperature|getTopP|getModel|getProfile|getReasoning|getCacheMode|getSystemPrompt|getKeys|getTools)\\("
      matches:
      - path: ConfigManager.java
        lineno: 154
        match: "\tpublic String getKeys() {"
      - path: ConfigManager.java
        lineno: 162
        match: "\tpublic Double getTemperature() {"
      - path: ConfigManager.java
        lineno: 166
        match: "\tpublic Double getTopP() {"
      - path: ConfigManager.java
        lineno: 170
        match: "\tpublic Model getModel() {"
      - path: ConfigManager.java
        lineno: 174
        match: "\tpublic AgentProfile getProfile() {"
      - path: ConfigManager.java
        lineno: 178
        match: "\tpublic Reasoning getReasoning() {"
      - path: ConfigManager.java
        lineno: 182
        match: "\tpublic CacheMode getCacheMode() {"
      - path: ConfigManager.java
        lineno: 186
        match: "\tpublic String[] getSystemPrompt() {"
      - path: ConfigManager.java
        lineno: 415
        match: "\tpublic String[] getTools() {"
```
/answer 7fa5-21 allow
ReasoningToken: 117
Control Request:
```yaml
id: 7fa5-22
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: "[Mm]axToken"
    include:
    - ConfigManager.java
    limit: 10
  reason: "Prüfen, ob ein MaxTokens-Getter existiert (Teil der eingefrorenen Config)"
```
/answer 7fa5-22 allow
Control Request:
```yaml
id: 7fa5-23
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      pattern: "[Mm]axToken"
      matches: []
```
/answer 7fa5-23 deny dieser heißt getMaxOutputTokens
ReasoningToken: 162
Text:
 Ich habe alle nötigen Referenzen. Ich erstelle nun den mehrteiligen, eigenständigen Plan.
Control Request:
```yaml
id: 7fa5-24
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
    source: |
      # Plan 1 – Vereinheitlichung des Slash-Command-Systems

      > Dieser Plan ist eigenständig. Alle benötigten Datei-, Verzeichnis-, Methoden- und
      > Node-ID-Informationen sind in den Step-Dateien eingebettet, damit umsetzende Agenten
      > ohne den Ursprungs-Prompt und mit minimalem Retrieval arbeiten können.

      ## Ziel
      Zentrale Vereinheitlichung von Nutzung, Behandlung und Implementierung der Kommandos, die im
      Editor angegeben/ausgeführt werden. Der Session Processor erhält eine zentrale Anlaufstelle,
      um Kommandos von normalem Text zu unterscheiden. Ein globaler Command Handler stellt die
      Infrastruktur bereit; die prompt-spezifische Logik wird in einen Prompt-Handler ausgelagert;
      die Session-Parameter werden in eine generische Basisklasse extrahiert; alle Connectoren
      werden auf ein einheitliches Prompt-Objekt umgestellt.

      ## Invarianten (müssen erhalten bleiben)
      - **Symmetrie / Austauschbarkeit:** Ein-/Ausgaben der Komponenten müssen zwischen Connectoren
        austauschbar bleiben. Tool-Call-/Result-Blöcke, die von MCP oder Claude erzeugt wurden,
        müssen auch durch einen anderen Connector geleitet werden können.
      - **Determinismus:** Gleiche Eingabe ⇒ gleiche Ausgabe.
      - **Prefix-Caching:** Umformungen dürfen das Prefix-Caching nicht unterbrechen.
      - **Kein permanenter Testsuite-Lauf.** Validierung erfolgt via `ast_validate` je berührter Datei.

      ## Kommando-Semantik im Session Processor
      | Kommando | Präfix | Behandlung im SessionProcessor |
      |---|---|---|
      | ANSWER | `/answer` | TRANSFORM: Kommando inkl. Option (`allow`/`deny`) entfernen, nur optionalen Resttext der Zeile behalten; beendet dennoch einen ggf. offenen Block (meist vorausgehender Tool Call → erzeugt Message-Block). |
      | CONTROL_REQUEST | `Control Request:` | IGNORE_BLOCK: Zeile **und** folgenden `\`\`\`yaml`-Block ignorieren (nicht-modellgenerierte Tool-Call-Anforderung). |
      | RESUME | `/resume` | REMOVE: Zeile aus Eingabe entfernen (nur Session-Kontext). |
      | EXIT | `/exit` | REMOVE. |
      | CALL | `/call` | REMOVE der Kommandozeile; der vorausgehende `Tool:`-Block wird als Tool Call in den korrekten Message-Typ gewandelt. |
      | TOOL | `/tool` | REMOVE (Template-Anforderung). |

      ## Sequenz der Phasen (strikt vorwärts)
      1. `01-command-constants.md` – Globale Kommando-Konstanten (Grundlage, keine Verhaltensänderung).
      2. `02-session-parameters.md` – `SessionParameters` aus `ClaudeSessionParameters` extrahieren.
      3. `03-command-handler.md` – Globaler CommandHandler + abstrakte Command-Hierarchie + Registry.
      4. `04-session-processor.md` – SessionProcessor an Registry anbinden (ANSWER/CONTROL_REQUEST/…).
      5. `05-prompt-object.md` – `FrozenConfig` + `Prompt` (Session-ID-Hash) in `connector/harness`.
      6. `06-prompt-handler.md` – `PromptHandler`: Auslagerung aus `AISessionManager`, Kommandoerkennung.
      7. `07-connector-api.md` – `IAIConnector.createRequest` auf Prompt-Objekt umstellen (6 Connectoren).
      8. `08-call-command.md` – `/call`-Kommando (MCP-Template + Handler + Processor).
      9. `09-cleanup.md` – Redundanzen entfernen, Gesamtvalidierung.

      Jede Phase ist einzeln compilierbar. Nach jeder Phase: `ast_validate` der berührten Dateien.

      ## Datei- & Node-Referenz (Basis: gemessener Ist-Zustand)
      Alle Pfade absolut unter `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/`.

      ### Kern-Editor/Manager
      - `EditorInterface.java` – Marker-Konstanten im Feldblock **Node `EditorInterface.nZrzm0`**:
        `USER="User:"`, `AGENT="Agent:"`, `TEXT="Text:"`, `THINKING="Thinking:"`, `TOOLUSE="Tool:"`,
        `TOOLRESULT="ToolResult:"`. Methoden: `EditorInterface.insertTag`, `.generateTag`, `.replaceTag`.
      - `AISessionManager.java` – Felder **Node `AISessionManager.BQ3GIF`** (cfg, connector, mcpClient,
        sessionProcessor, editIfc, editorListener, includeAdapter, inputStats), **`.LdjI0C`** (Observer).
        Prompt-Methoden: `.getInput`, `.removeCommentLines`, `.prepareInner`, `.execute`,
        `.PromptJob` (Kinder `.PromptJob.PromptJob`, `.PromptJob.run`), `.queueAsync`, `.queueSync`,
        `.queueAndSubmit`, `.executeInner`, `.updateInputStat`, `.initializeInputs`.
      - `Activator.java` – Wiring (Zeilen ~29-32): `sessionProcessor = new SessionProcessor();`
        `connector = new AdaptingConnector(cfg, cliSessionManager, mcpClient, sessionProcessor);`
        `session = new AISessionManager(cfg, connector, mcpClient, sessionProcessor);`.
      - `ConfigManager.java` – Getter (Zeilen): `getKeys`154, `getTemperature`162, `getTopP`166,
        `getModel`170, `getProfile`174, `getReasoning`178, `getCacheMode`182, `getSystemPrompt`186,
        `getFreeText`190, `isInputEnabled`297, `getTools`415, **`getMaxOutputTokens`** (existiert).

      ### Connector-Infrastruktur (`connector/`)
      - `IAIConnector.java` – **Node `IAIConnector`**; Methoden `.createRequest`, `.executeRequest`,
        `.convertResponse`, `.getSupportedKeyPattern`. Aktuelle Signatur:
        `REQ createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix, IProgressMonitor mon);`
      - `AdaptingConnector.java` – `.createRequest` delegiert an `getConnector(cfg.getModel())`;
        `.executeRequest` delegiert an `getConnector(request)`.

      ### Claude Code (`connector/claudecode/`)
      - `CommandType.java` – **Node `CommandType`**: `enum { Prompt, Exit, Resume, Allow, Deny, Modification }`.
      - `CCRequest.java` – Felder **`CCRequest.Ki6WvE`** (id,title,systemPrompt,tools,cmd); Konstruktor
        **`CCRequest.CCRequest`**; innere Klasse **`CCRequest.Command`** (Felder `type`,`parameter`,`parameters`).
      - `CCConnector.java` – `.preprocessInput` (Regex `/exit`,`/resume`,`/answer … allow|deny`,Modification),
        `.createRequest`, `.executeRequest`, `.getEditorLocation`.
      - `CCControlClient.java` – Konstanten **`ANSWER="/answer"`**, **`CONTROL_REQUEST="Control Request:"`**
        (Feldblock, ~Z.32/33); `.checkControlEndpoint` (rendert `CONTROL_REQUEST` + yaml + `/answer <id> allow`),
        `.submitEdit`, `.extractYamlBlock`.
      - `ClaudeSessionParameters.java` – siehe `02-session-parameters.md` (vollständig).

      ### API-Connectoren
      - `claude/ClaudeConnector.java` – `.ClaudeConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`, `.appendTools`, `.executeRequest`, `.convertResponse`.
      - `deepseek/DeepSeekConnector.java` – `.DeepSeekConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`, innere `SessionRequestCallbacks`.
      - `google/GeminiConnector.java` – `.GeminiConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`, `.getThinkingBudget`.
      - `openai/OpenAIConnector.java` – `.OpenAIConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`.

      ### Harness (`connector/harness/`)
      - `SessionProcessor.java` – `.process(List<String>,cb)`, `.process(String,cb)`, `.setEnabled`, innere `Run`:
        `.run`, `.flush`, `.isMarkerLine`, `.consumeReasoning`, `.consumeText`, `.consumeToolCall`,
        `.consumeToolResult`, `.fenceRange`, `.readYaml`, `.include*`.
      - `SessionCallbacks.java`, `SessionRenderer.java`, `SessionAnswerBuilder.java` (Tool-Call-Rendering).

      ### MCP (`connector/mcp/`)
      - `MCPConnector.java` – `.preprocess` (`/exit`,`/tool`), `.createRequest`, `.executeRequest`.
      - `MCPControlClient.java` – `.renderSchema` (liefert `\`\`\`yaml … \`\`\``), `.extractYamlBlock`,
        `.parseYaml`, `.fillReason`, `.prettyResult`.
      - `MCPClient.java` – `.findTool`, `.callTool`, `.renderToolCall`.
      - `MCPRequest.java` – Konstruktor `MCPRequest(String id, String systemPrompt, List<String> tools, MCPCommand cmd)`.
      - `MCPCommand.java`, `MCPCommandType.java` – `enum { Exit, Tool, Prompt }` (+ ggf. weitere).
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/01-command-constants.md
    source: |
      # Step 01 – Globale Kommando-Konstanten (Phase 0)

      **Ziel:** Eine einzige, geteilte Quelle für Kommando-Präfixe etablieren, ohne Verhalten zu ändern.
      **Abhängigkeiten:** keine. Grundlage für Steps 03/04/06/08.

      ## Ist-Zustand
      - `EditorInterface.java`, Feldblock **Node `EditorInterface.nZrzm0`** enthält bereits die Marker:
      ```java
      public static final String USER = "User:";
      public static final String AGENT = "Agent:";
      public static final String TEXT = "Text:";
      public static final String THINKING = "Thinking:";
      public static final String TOOLUSE = "Tool:";
      public static final String TOOLRESULT = "ToolResult:";
      ```
      - `connector/claudecode/CCControlClient.java` (Feldblock ~Z.32/33):
      ```java
      public static final String ANSWER = "/answer";
      public static final String CONTROL_REQUEST = "Control Request:";
      ```
      - Verstreute String-Literale: `"/exit"`, `"/resume"` in `CCConnector.preprocessInput`;
        `"/exit"`, `"/tool"` in `MCPConnector.preprocess`.

      ## Aktionen
      1. In `EditorInterface.java` (Node `EditorInterface.nZrzm0`), unterhalb der Marker, einen
         Kommando-Konstantenblock ergänzen (symmetrisch zu den Markern):
      ```java
      /** Command prefixes recognized by connector.CommandHandler / SessionProcessor. */
      public static final String CMD_ANSWER = "/answer";
      public static final String CMD_RESUME = "/resume";
      public static final String CMD_EXIT   = "/exit";
      public static final String CMD_TOOL   = "/tool";
      public static final String CMD_CALL   = "/call";
      public static final String CONTROL_REQUEST = "Control Request:";
      ```
      2. `CCControlClient.ANSWER` und `CCControlClient.CONTROL_REQUEST` auf die zentralen Konstanten
         umbiegen (Kompatibilität behalten – Felder bleiben bestehen, referenzieren aber `EditorInterface`):
      ```java
      public static final String ANSWER = EditorInterface.CMD_ANSWER;
      public static final String CONTROL_REQUEST = EditorInterface.CONTROL_REQUEST;
      ```
         Import für `EditorInterface` in `CCControlClient` sicherstellen.

      ## Hinweise
      - Noch **keine** Umstellung von `CCConnector`/`MCPConnector` hier (erfolgt in Step 03).
      - `checkControlEndpoint` in `CCControlClient` nutzt `ANSWER`/`CONTROL_REQUEST` bereits – bleibt funktional.

      ## Validierung
      `ast_validate` für `EditorInterface.java`, `connector/claudecode/CCControlClient.java`.
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/02-session-parameters.md
    source: |
      # Step 02 – `SessionParameters` extrahieren (Phase 1)

      **Ziel:** Generische Session-Parameter (inkl. deterministischem Hash) aus `ClaudeSessionParameters`
      in eine neue Basisklasse `connector/SessionParameters.java` herauslösen. `ClaudeSessionParameters`
      leitet davon ab und behält nur CC-Spezifika (CLI-/Env-Aufbau, Title).
      **Abhängigkeiten:** keine (kann parallel zu Step 01). Grundlage für `FrozenConfig`-Hash (Step 05).

      ## Ist-Zustand `connector/claudecode/ClaudeSessionParameters.java`
      Felder (Node `ClaudeSessionParameters.igRtKw` + `.ZbqmG7`):
      ```java
      private static final String SCRIPT = System.getProperty("user.home") + "/xyan/xy.ai.workbench/claude-code/claude-session.sh";
      private static final String COMMAND = "claude";
      public final Path cwd; public final String systemPrompt; public final List<String> tools;
      public final Model model; public final Reasoning reasoning; public final AgentProfile agentProfile;
      public final String cliProfile; public final String filePath; public final CacheMode cacheMode;
      private String hash; private String title;
      ```
      Methoden (Node-IDs): `.ClaudeSessionParameters` (2 Konstruktoren), `.fromConfig`, `.getFilePath`,
      `.buildBaseCommand`, `.buildEvironment`, `.getHash`, `.computeHash`, `.setTitle`, `.getTitle`.

      `computeHash()` (Ist):
      ```java
      String input = String.join(",", tools) + "|" + cwd.toString() + "|" + model.apiName + "|"
          + reasoning.name() + "|" + (agentProfile != null ? agentProfile.name : "") + "|"
          + cliProfile + "|" + (filePath != null ? filePath : "");
      // MD5 -> hex, substring(0,8); Fallback: 31er-Rolling-Hash
      ```
      `fromConfig(ConfigManager cfg, Path cwd, String filePath, String systemPrompt, List<String> tools)`
      nutzt `cfg.getModel/getReasoning/getProfile/getKeys/getCacheMode`.

      ## Aktionen
      1. Neue Datei `connector/SessionParameters.java` (package `xy.ai.workbench.connector`):
         - Generische `public final` Felder: `cwd`, `filePath`, `systemPrompt`, `tools`, `model`,
           `reasoning`, `agentProfile`, `cliProfile`, `cacheMode`.
         - Optionale Config-Felder für spätere Nutzung durch `FrozenConfig`: `topP`, `temperature`,
           `maxOutputTokens` (Double/Integer, nullbar).
         - Konstruktor mit Validierung (aus `ClaudeSessionParameters` übernehmen: cwd!=null, model!=null,
           model.apiName nicht blank, reasoning!=null).
         - `getHash()` + `computeHash()` (Logik 1:1 übernehmen). Hash-Parameter-Set unverändert lassen,
           um bestehende Session-Zuordnung stabil zu halten (Prefix-Caching!).
         - `getFilePath()`.
      2. `ClaudeSessionParameters extends SessionParameters`:
         - Nur noch behalten: `SCRIPT`, `COMMAND`, `title`, `setTitle`, `getTitle`, `buildBaseCommand`,
           `buildEvironment`, `fromConfig` (Factory bleibt hier).
         - Konstruktoren rufen `super(...)` auf; `hash`/generische Felder entfallen lokal.
         - `getHash`/`computeHash`/generische Felder aus dieser Klasse **entfernen** (kommen von Basis).
      3. Zugriffe prüfen: `params.model`, `params.tools`, `params.cwd`, `params.getHash()`, `params.filePath`
         etc. funktionieren weiter über Vererbung. `CCConnector.executeRequest` nutzt
         `ClaudeSessionParameters.fromConfig(...)` + `params.setTitle(req.title)` – unverändert.

      ## Hinweise / Fallstricke
      - **Hash-Stabilität:** Parameter-String für den Hash NICHT verändern.
      - CC-spezifische Env/CLI-Logik (`buildEvironment`, `buildBaseCommand`) bleibt in `ClaudeSessionParameters`.
      - Sichtbarkeit der Felder als `public final` beibehalten (viele Direktzugriffe).

      ## Validierung
      `ast_validate` für `connector/SessionParameters.java`,
      `connector/claudecode/ClaudeSessionParameters.java`, `connector/claudecode/CCConnector.java`,
      `connector/claudecode/CCSessionManager.java` (nutzt Parameter), sowie alle Referenzen von
      `ClaudeSessionParameters` (grep `ClaudeSessionParameters`).
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/03-command-handler.md
    source: |
      # Step 03 – Globaler CommandHandler + Command-Hierarchie (Phase 2)

      **Ziel:** Globale Infrastruktur zur Implementierung von Kommandos: abstrakte Oberklasse `Command`,
      konkrete Kommandos, statische Registry `CommandHandler` mit hart kodierten Referenzen. Bestehende
      Preprocess-Logik von `CCConnector`/`MCPConnector` auf die Registry umstellen (Verhalten unverändert).
      **Abhängigkeiten:** Step 01 (Konstanten).

      ## Verzeichnis
      Neu in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/` (package `xy.ai.workbench.connector`).

      ## Ist-Zustand (zu vereinheitlichende Logik)
      `connector/claudecode/CCConnector.java` → `.preprocessInput(List<String>)`:
      ```java
      if ("/exit".equalsIgnoreCase(clean)) -> Command(Exit,"")
      else if (clean.matches("(?i)/resume\\s+\\S+")) -> Command(Resume, <arg>)
      else if (clean.matches("(?i)/answer\\s+\\S+\\s+(allow|deny)(\\s+.*)?")) -> Command(Allow|Deny, id, reason)
      else if (controlClient.submitEdit(clean)) -> Command(Modification,"")
      else -> Command(Prompt, clean)
      ```
      `connector/claudecode/CommandType.java` → **Node `CommandType`**:
      `enum { Prompt, Exit, Resume, Allow, Deny, Modification }`.
      `connector/claudecode/CCRequest.java` → innere **`CCRequest.Command`** (Felder `type`,`parameter`,`parameters`).
      `connector/mcp/MCPConnector.java` → `.preprocess`:
      ```java
      if ("/exit") -> MCPCommand(Exit,"")
      if (clean.matches("(?i)/tool\\s+\\S+")) -> MCPCommand(Tool, <arg>)
      else -> MCPCommand(Prompt, merged)
      ```
      `connector/mcp/MCPCommandType.java` → `enum { Exit, Tool, Prompt }`.

      ## Aktionen
      1. `connector/Command.java` – abstrakte Oberklasse:
         - Felder: `final String[] parameters;` (bzw. typspezifisch), Zugriff `parameter(i)`.
         - Enum `ProcessorAction { REMOVE, TRANSFORM, IGNORE_BLOCK, NONE }` (Behandlung im Processor).
         - Abstrakte/Standard-API:
           - `String prefix()` – z. B. `EditorInterface.CMD_ANSWER`.
           - `boolean matches(String line)` – Standard: `line.strip()` beginnt mit `prefix()`.
           - `Command parse(String line)` – erzeugt konkrete Instanz mit extrahierten Parametern.
           - `ProcessorAction processorAction()` – siehe Tabelle in `00-overview.md`.
      2. Konkrete Command-Klassen (je eigene Datei oder als geschachtelte Klassen in `Command`):
         - `AnswerCommand` (prefix `/answer`, Regex `(?i)/answer\s+\S+\s+(allow|deny)(\s+.*)?`, Action TRANSFORM,
           Parameter: id, action, reason/text).
         - `ControlRequestCommand` (prefix `Control Request:`, Action IGNORE_BLOCK; markiert folgenden yaml-Block).
         - `ResumeCommand` (`/resume <arg>`, REMOVE, Parameter: session-id/arg).
         - `ExitCommand` (`/exit`, REMOVE).
         - `ToolCommand` (`/tool <id>`, REMOVE, Parameter: tool-id).
         - `CallCommand` (`/call`, REMOVE; siehe Step 08 – erfasst vorausgehenden yaml-Block).
         - `ModificationCommand` (yaml-Edit ohne Präfix; nur CC – via `controlClient.submitEdit`).
         - `PromptCommand` (Default/kein Präfix, Action NONE).
      3. `connector/CommandHandler.java` – statische Registry:
         - `private static final List<Command> REGISTRY = List.of(new AnswerCommand(), new ControlRequestCommand(), new ResumeCommand(), new ExitCommand(), new ToolCommand(), new CallCommand());` (hart kodiert).
         - `public static Optional<Command> detect(String line)` – erstes `matches()`.
         - `public static List<Command> list()` – für SessionProcessor/PromptHandler.
         - Konstanten-Bezug ausschließlich über `EditorInterface.CMD_*` / `CONTROL_REQUEST`.
      4. `CCConnector.preprocessInput` auf `CommandHandler.detect(...)` umstellen; Mapping der neuen
         Command-Typen auf bestehende `CommandType`/`CCRequest.Command` beibehalten (Verhalten identisch).
         `Modification` weiterhin via `controlClient.submitEdit(clean)` prüfen.
      5. `MCPConnector.preprocess` analog auf `CommandHandler.detect(...)` umstellen (`/exit`→Exit, `/tool`→Tool,
         sonst Prompt).

      ## Hinweise
      - `CCRequest.Command`/`CommandType` und `MCPCommand`/`MCPCommandType` **vorerst behalten** – nur die
        Erkennung zentralisieren. Endgültige Konsolidierung optional in Step 09.
      - Regexe exakt aus dem Ist-Zustand übernehmen, um Verhaltensgleichheit zu garantieren.

      ## Validierung
      `ast_validate` für neue Dateien + `connector/claudecode/CCConnector.java`,
      `connector/mcp/MCPConnector.java`.
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/04-session-processor.md
    source: |
      # Step 04 – SessionProcessor an Registry anbinden (Phase 3)

      **Ziel:** Der SessionProcessor erkennt Kommandos zentral über `CommandHandler` und behandelt sie
      gemäß ihrer `ProcessorAction` (siehe Tabelle in `00-overview.md`).
      **Abhängigkeiten:** Step 03 (CommandHandler/Command).

      ## Ist-Zustand `connector/harness/SessionProcessor.java`
      - `.process(List<String> inputs, cb)`: bei `!enabled` ⇒ ein einziges `callbacks.message(Role.User, joined)`.
        Bei `enabled` ⇒ pro Input `new Run<>(...).run(input, List.of())`.
      - Innere `Run.run(String text, List<Path> chain)` iteriert Zeilen und erkennt Marker
        (`INCLUDE_LINE`, `USER`, `AGENT`, `THINKING`, `TEXT`, `TOOLUSE`, `TOOLRESULT`); sonst Buffer.
        Am Ende `flush()`.
      ```java
      if (stripped.equals(EditorInterface.TOOLUSE)) { flush(); i = consumeToolCall(lines, i + 1); continue; }
      ```
      - `.consumeToolCall(lines,start)`: liest `fenceRange` + `readYaml`, emittiert
        `callbacks.toolCall(new ToolCall(node.path("id"), node.path("tool"), node.path("arguments")))`.
      - `.isMarkerLine(line)`, `.fenceRange`, `.readYaml`, `.flush` vorhanden.

      ## Aktionen (in `Run.run`, vor der bestehenden Marker-Kette)
      1. Am Schleifenkopf `CommandHandler.detect(line)` prüfen. Bei Treffer je `processorAction()`:
         - **IGNORE_BLOCK** (`Control Request:`): `flush()`, aktuelle Zeile überspringen; wenn die
           Folgezeile(n) einen `\`\`\`yaml`-Block bilden, via `fenceRange` überspringen. `i` hinter den
           Block setzen. Keine Emission.
         - **TRANSFORM** (`/answer`): `flush()` (beendet vorausgehenden Tool-Call-/Message-Block). Rest der
           Zeile nach Entfernen von Kommando + Option (`allow`/`deny`) ermitteln; falls nicht leer, als
           Buffer/Text weiterführen (wird beim nächsten `flush()` zur Message). `i++`.
         - **REMOVE** (`/resume`, `/exit`, `/tool`, `/call`): `flush()`, Zeile überspringen. `i++`.
           (Für `/call` erfolgt die Tool-Call-Wandlung über den vorausgehenden `Tool:`-Block – Step 08.)
      2. `isMarkerLine` um Kommando-Zeilen erweitern (damit Kommandos einen offenen Buffer korrekt beenden):
         `|| CommandHandler.detect(s).isPresent()`.
      3. Determinismus/Symmetrie sicherstellen: Bei `!enabled` bleibt das bisherige Verhalten (ein
         User-Message-Block) – Kommandobehandlung greift nur im aktivierten Processor-Modus.

      ## Hinweise / Fallstricke
      - Die von `CCControlClient.checkControlEndpoint` erzeugte Ausgabe hat die Form:
        `Control Request:\n\`\`\`yaml\n<...>\n\`\`\`\n/answer <id> allow` – d. h. IGNORE_BLOCK muss den
        yaml-Block direkt nach der `Control Request:`-Zeile erfassen; die nachfolgende `/answer`-Zeile wird
        separat durch TRANSFORM behandelt.
      - Prefix-Caching: Reihenfolge/Whitespace der emittierten Blöcke stabil halten.

      ## Validierung
      `ast_validate` für `connector/harness/SessionProcessor.java`. Danach manuell prüfen (Review), dass
      Tool-Call-/Result-Blöcke unverändert durchgereicht werden (Symmetrie).
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/05-prompt-object.md
    source: |
      # Step 05 – FrozenConfig + Prompt-Objekt (Phase 4)

      **Ziel:** Eingaben und Konfiguration in unveränderliche Objekte einfrieren. `FrozenConfig` kapselt die
      Konfiguration; `Prompt` bündelt Trigger + Eingaben (semantisch getrennt: Inputs, Batch, Config) und
      steuert den Aktivierungszustand des SessionProcessor. Session-ID = Hash(Dateipfad, Batch, ConfigHash).
      **Abhängigkeiten:** Step 02 (Hash-Mechanik als Vorlage).

      ## Verzeichnis
      Neu in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/` (package `...connector.harness`).

      ## Config-Quellen (`ConfigManager.java`, gemessen)
      Getter: `getModel()`170, `getProfile()`174, `getReasoning()`178, `getTools()`415,
      `getSystemPrompt()`186 (`String[]`), `getFreeText()`190, `getTopP()`166 (`Double`),
      `getTemperature()`162 (`Double`), `getMaxOutputTokens()` (existiert), `getCacheMode()`182,
      `getKeys()`154 (cliProfile), `isInputEnabled(InputMode)`297.

      Systemprompt-Aufbau (Ist, aus `AISessionManager.getInput(SystemPrompt)`):
      ```java
      Arrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith("#"))
        .forEach(e -> sb.append("* ").append(e).append(".\n"));
      String freeText = cfg.getFreeText();
      if (freeText != null && !freeText.isBlank()) sb.append(".\n").append(freeText).append(".\n");
      ```

      ## Aktionen
      1. `connector/harness/FrozenConfig.java` – unveränderliche Momentaufnahme:
         - Felder: `model`, `profile`, `reasoning`, `tools` (List/Array, kopiert), `systemPrompt` (fertig
           gerenderter String), `topP`, `temperature`, `maxOutputTokens`, ergänzend `cacheMode`, `cliProfile`.
         - Factory `static FrozenConfig from(ConfigManager cfg)`: liest alle Werte einmalig aus, rendert den
           Systemprompt (Logik oben), friert Tools ein.
         - `String getHash()` – deterministischer Hash NUR über: **Modell, Profil, Reasoning, Tools,
           Systemprompt, TopP, Temperature, MaxTokens** (exakt diese Bestandteile). Hash-Mechanik analog
           `SessionParameters.computeHash` (MD5→hex, stabiler Fallback).
      2. `connector/harness/Prompt.java` – eingefrorenes Prompt-Objekt (immutable):
         - **Semantisch getrennte** Bestandteile:
           - Inputs: `List<String> inputs` (ausgewählter Textbereich / Selection / Converter-Inhalt),
             plus Meta `InputKind kind { LineInput, BlockSelection, FullFile }` (für Step 06).
           - Batch: `boolean batch` (Batch-Trigger ja/nein).
           - Config: `FrozenConfig config`.
         - `boolean processorEnabled` – ersetzt das direkte `sessionProcessor.setEnabled(...)`.
         - `String absoluteFilePath` – absoluter Pfad der auslösenden Datei.
         - `String sessionId()` – Hash über: `absoluteFilePath` + `"|"` + `batch` + `"|"` + `config.getHash()`
           (Mechanik analog `ClaudeSessionParameters.computeHash`, siehe Step 02). Referenz-Semantik:
           `ClaudeSessionParameters` nutzt den Datei-/Parameter-Hash analog.
         - Optionale, typisierte Kommando-Payload (für Step 06/08): `Command command` + ggf. erfasster
           `String yamlBlock` (für `/call`).
         - Konstruktion nur über Builder/Factory im PromptHandler (Step 06) – hier nur die Datenklasse.
      3. Optionale Ableitung: statische Hilfsmethode, um aus `Prompt.config` eine `ClaudeSessionParameters`
         bzw. generische `SessionParameters` zu erzeugen (ersetzt in Step 07 `fromConfig(cfg, ...)`).

      ## Hinweise
      - **Kein** direktes `cfg` in Connectoren mehr (Step 07) – Werte kommen aus `Prompt.config`.
      - Der Systemprompt wird beim Einfrieren einmalig gerendert (Determinismus, Multi-Session-fähig).
      - Hash-Bestandteile für `FrozenConfig` strikt auf die 8 genannten begrenzen.

      ## Validierung
      `ast_validate` für `connector/harness/FrozenConfig.java`, `connector/harness/Prompt.java`.
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/06-prompt-handler.md
    source: |
      # Step 06 – PromptHandler (Phase 5)

      **Ziel:** Prompt-spezifischen Code aus `AISessionManager` in `connector/harness/PromptHandler`
      auslagern. Der Handler empfängt den Trigger, friert Eingaben in ein `Prompt` ein, implementiert die
      Kommandoerkennung/Eingabeauswahl und steuert den Processor-Zustand über das Prompt-Objekt.
      **Abhängigkeiten:** Steps 03 (CommandHandler), 05 (Prompt/FrozenConfig).

      ## Ist-Zustand `AISessionManager.java`
      Felder (Node `AISessionManager.BQ3GIF`): `editorListener`, `includeAdapter`, `cfg`, `connector`,
      `mcpClient`, `sessionProcessor`, `editIfc`, `inputStats`.
      Auszulagernde Methoden (Node-IDs):
      - `.getInput(InputMode)` – Selection/Converter/SystemPrompt/Tools-Extraktion aus dem aktiven Editor.
      - `.removeCommentLines(String)`.
      - `.prepareInner(Display, boolean batchFix, IProgressMonitor)` – wählt Inputs, baut Systemprompt,
        Tools, **setzt aktuell direkt** `sessionProcessor.setEnabled(cfg.isInputEnabled(InputMode.Converter))`
        und ruft `connector.createRequest(inputs, systemPrompt, tools, batchFix, sub)`.
      - `.execute(Display)` → `new PromptJob(...).schedule()`; innere `.PromptJob` mit `.run` (Prepare→insertTag
        →executeInner→replaceTag).
      - `.queueAsync`, `.queueSync`, `.queueAndSubmit` (Batch-Enqueue über `AIBatchManager`).
      - `.executeInner(Display, IModelRequest, mon, job)`.
      - `.updateInputStat(InputMode)`, `.initializeInputs()`.

      Relevanter Ist-Code `prepareInner` (Kern):
      ```java
      if (cfg.isInputEnabled(InputMode.Selection)) input = getInput(InputMode.Selection);
      else if (cfg.isInputEnabled(InputMode.Converter)) input = getInput(InputMode.Converter);
      ...
      sessionProcessor.setEnabled(cfg.isInputEnabled(InputMode.Converter)); // <-- ersetzen
      IModelRequest req = connector.createRequest(inputs, systemPrompt.toString(), tools, batchFix, sub);
      ```
      `getInput(Selection)`: bei nicht-leerer Selektion (>1) `removeCommentLines(tsel.getText())`; sonst
      aktuelle Cursorzeile via `doc.getLineInformation(tsel.getEndLine())`. `Converter`: gesamte Datei `doc.get()`.

      ## Aktionen
      1. Neue Klasse `connector/harness/PromptHandler.java`:
         - Konstruktor erhält Abhängigkeiten: `ConfigManager cfg`, `AdaptingConnector connector`,
           `SessionProcessor sessionProcessor`, `ActiveEditorListener editorListener`, `EditorInterface editIfc`,
           `IncludeAdapter includeAdapter` (aus `AISessionManager` übernehmen).
         - Methoden aus `AISessionManager` hierher verschieben: `getInput`, `removeCommentLines`,
           `updateInputStat`, `initializeInputs`, sowie die Prompt-/Job-Orchestrierung (`execute`,
           `PromptJob`, `queueSync`, `queueAsync`, `queueAndSubmit`, `executeInner`).
         - `prepareInner` ersetzen durch `Prompt buildPrompt(Display, boolean batch)`:
           - Eingabemodus bestimmen (`InputKind`): Selection (Zeile vs. Block) vs. Converter (FullFile).
           - Systemprompt/Tools/Config einfrieren via `FrozenConfig.from(cfg)`.
           - Absoluten Dateipfad des aktiven Editors ermitteln (für `sessionId`).
           - `processorEnabled = cfg.isInputEnabled(InputMode.Converter)` **in das Prompt-Objekt schreiben**
             (kein direkter `setEnabled`-Aufruf mehr).
           - Kommandoerkennung ausführen (siehe 2) und ggf. `Command`/`yamlBlock` in `Prompt` setzen.
         - `connector.createRequest(prompt, mon)` aufrufen (neue Signatur, Step 07).
      2. **Kommandoerkennung nach Eingabemodus** (`CommandHandler.detect`):
         - **LineInput** (aktuelle Zeileneingabe): prüfe, ob die aktuelle Zeile mit einem Kommando beginnt.
         - **BlockSelection**: prüfe, ob der Block mit einem Kommando beginnt (mehrzeilige Kommandos) ODER
           ob ein Kommando in der letzten Zeile beginnt (z. B. Kommando nachgestellt an Tool Call).
         - **FullFile** („Processor“-Modus): prüfe zuerst die aktuelle Cursorzeile; danach, ob die letzte
           Zeile der Datei mit einem Kommando beginnt. Kommandos an anderer Stelle = nur Session-Kontext.
         - Für `/call`: den davor liegenden yaml-Block rückwärts ab Kommandozeile erfassen (Regex
           `^\`\`\`yaml$` … `^\`\`\`$`) und typisiert im `Prompt` ablegen (Details Step 08).
      3. `AISessionManager` umbauen: delegiert an `PromptHandler` (Instanz halten). Öffentliche Methoden, die
         von `view/AISessionView` u. a. genutzt werden (`execute`, `queueAsync`, `queueAndSubmit`,
         `updateInputStat`, `initializeInputs`, `addAnswerObs`, `addInputStatObs`), als Delegation beibehalten
         ODER Aufrufer auf `PromptHandler` umstellen (grep-Prüfung nötig).
      4. `sessionProcessor.setEnabled(...)` NICHT mehr in `AISessionManager`/`PromptHandler` direkt setzen;
         Aktivierungszustand fließt über `Prompt.processorEnabled` in den Verarbeitungspfad (Connector ruft
         `sessionProcessor` mit dem Prompt-Zustand auf – siehe Step 07). Ziel: Multi-Session/Parallelbetrieb.
      5. Wiring in `Activator.java` anpassen: `PromptHandler` erzeugen und in `AISessionManager` injizieren
         (oder `AISessionManager` erzeugt ihn intern). Bestehende Reihenfolge:
         `sessionProcessor` → `connector` → `session`.

      ## Hinweise / Fallstricke
      - `getInput` greift auf `editorListener.getLastTextEditor()` und Eclipse `IDocument`/`ITextSelection` zu –
        UI-Thread-Zugriffe (`display.syncExec`) beibehalten.
      - Aufrufer von `AISessionManager` per grep ermitteln: `grep "session\.(execute|queueAsync|queueAndSubmit|updateInputStat|initializeInputs)"`.
      - Semantische Trennung im `Prompt` (Inputs/Batch/Config) strikt einhalten.

      ## Validierung
      `ast_validate` für `connector/harness/PromptHandler.java`, `AISessionManager.java`, `Activator.java`
      sowie Aufrufer (`view/AISessionView.java`).
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/07-connector-api.md
    source: |
      # Step 07 – Connectoren auf Prompt-Objekt umstellen (Phase 6)

      **Ziel:** `IAIConnector.createRequest` erhält das `Prompt`-Objekt statt Einzelparameter. Alle direkten
      `cfg`-Zugriffe der Connectoren werden auf `prompt.config` (`FrozenConfig`) umgestellt. Der
      Processor-Aktivierungszustand kommt aus `prompt.processorEnabled`.
      **Abhängigkeiten:** Steps 05 (Prompt/FrozenConfig), 06 (PromptHandler erzeugt Prompt).

      ## Ist-Zustand
      `connector/IAIConnector.java` (**Node `IAIConnector`**):
      ```java
      REQ createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix, IProgressMonitor mon);
      RESP executeRequest(REQ req, IProgressMonitor mon, Job job);
      AIAnswer convertResponse(RESP resp, IProgressMonitor mon);
      KeyPattern getSupportedKeyPattern();
      ```
      `connector/AdaptingConnector.java`:
      ```java
      public IModelRequest createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix, IProgressMonitor mon) {
        return getConnector(cfg.getModel()).createRequest(inputs, systemPrompt, tools, batchFix, mon);
      }
      ```
      Connector-`createRequest`-Implementierungen (Node-IDs):
      - `claudecode/CCConnector.createRequest` (baut `CCRequest(id,title,systemPrompt,tools,command)`; nutzt
        `preprocessInput`, `requestBuilder.buildPromptJson`). `executeRequest` nutzt
        `ClaudeSessionParameters.fromConfig(cfg, loc.projectPath, loc.relativeFilePath, req.systemPrompt, req.tools)`.
      - `mcp/MCPConnector.createRequest` → `new MCPRequest(UUID, systemPrompt, tools, preprocess(inputs))`.
      - `claude/ClaudeConnector.createRequest`, `.appendTools`, Konstruktor `(cfg, mcpClient, sessionProcessor)`.
      - `deepseek/DeepSeekConnector.createRequest` (+ innere `SessionRequestCallbacks`).
      - `google/GeminiConnector.createRequest`, `.getThinkingBudget(Reasoning, ConfigManager)`.
      - `openai/OpenAIConnector.createRequest`.
      Die API-Connectoren nutzen `sessionProcessor.process(inputs, cb)` (z. B. `GeminiConnector` Z.111).

      ## Aktionen
      1. `IAIConnector.createRequest` neue Signatur:
         `REQ createRequest(Prompt prompt, IProgressMonitor mon);`
         (Import `xy.ai.workbench.connector.harness.Prompt`).
      2. `AdaptingConnector.createRequest`:
         `return getConnector(prompt.config().model()).createRequest(prompt, mon);`
         (Model aus `FrozenConfig`, nicht mehr `cfg.getModel()`).
      3. Jede Connector-Implementierung anpassen:
         - Inputs: `prompt.inputs()`; Systemprompt: `prompt.config().systemPrompt()`; Tools:
           `prompt.config().tools()`; Batch: `prompt.batch()`.
         - `cfg`-Zugriffe für Modellparameter (Reasoning, TopP, Temperature, MaxTokens, Profil) durch
           `prompt.config()` ersetzen. `getThinkingBudget(Reasoning, ConfigManager)` → auf `FrozenConfig` umstellen.
         - `sessionProcessor.process(prompt.inputs(), cb)`: Der Processor-Aktivierungszustand muss pro Prompt
           gelten. Option A: `sessionProcessor.process(prompt.inputs(), prompt.processorEnabled(), cb)`
           (neue Overload, threadsicher, kein globaler `setEnabled`). Option B: `setEnabled` unmittelbar vor
           `process` unter Lock. **Empfehlung: Overload (A)** für Multi-Session/Parallelbetrieb.
      4. `CCConnector.executeRequest`: `ClaudeSessionParameters` aus `prompt.config()` ableiten statt
         `fromConfig(cfg, ...)`; `filePath`/`cwd` aus `getEditorLocation()`/Prompt. `req.title` weiterhin
         aus `CCRequest` (Preprocess). Session-Zuordnung nutzt weiterhin `params.getHash()`.
      5. `SessionProcessor.setEnabled` als öffentliche Methode nur noch intern/legacy; neue Overload
         `process(List<String> inputs, boolean enabled, SessionCallbacks<M> cb)` ergänzen und `process(...)`
         ohne Flag auf den gespeicherten Zustand mappen (Rückwärtskompatibilität).

      ## Reihenfolge (nach jedem Schritt `ast_validate`)
      1) `IAIConnector` + `AdaptingConnector` + `SessionProcessor`-Overload.
      2) `CCConnector` (createRequest/executeRequest).
      3) `MCPConnector`.
      4) `ClaudeConnector`.
      5) `DeepSeekConnector`.
      6) `GeminiConnector`.
      7) `OpenAIConnector`.

      ## Hinweise / Fallstricke
      - Batch-Pfad: `AISessionManager.queueSync`/`AIBatchManager` verwendet `createRequest` ebenfalls – über
        `PromptHandler.buildPrompt(display, true)` versorgen.
      - Determinismus/Prefix-Caching: gerenderten Systemprompt aus `FrozenConfig` unverändert übernehmen.

      ## Validierung
      `ast_validate` je Connector + `IAIConnector.java`, `AdaptingConnector.java`, `SessionProcessor.java`.
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/08-call-command.md
    source: |
      # Step 08 – `/call`-Kommando (Phase 7)

      **Ziel:** MCP-Templates erhalten ein angehängtes `/call`-Kommando und das `Tool:`-Präfix. Der
      PromptHandler erkennt `/call`, erfasst den davor liegenden yaml-Block, typisiert ihn und übergibt ihn
      via Prompt-Objekt an den MCP-Client. Der SessionProcessor wandelt den Tool Call in den korrekten
      Message-Typ und ignoriert die `/call`-Zeile.
      **Abhängigkeiten:** Steps 03 (CallCommand), 04 (Processor), 06 (PromptHandler), 07 (Prompt-API).

      ## Ist-Zustand
      `connector/mcp/MCPControlClient.renderSchema(JsonNode tool)` liefert:
      ```java
      sb.append("```yaml\n");
      sb.append("tool: ").append(tool.path("name").asText()).append("\n");
      // arguments: ... (oder 'arguments: {}')
      sb.append("```");
      return sb.toString();
      ```
      `connector/mcp/MCPConnector.executeRequest` (case Prompt): `control.extractYamlBlock(param)` →
      `parseYaml` → `client.findTool(name)` → `control.fillReason(tool, args)` → `client.callTool(name, args)`.
      `EditorInterface.TOOLUSE = "Tool:"`. SessionProcessor `consumeToolCall` erwartet nach `Tool:` einen
      `\`\`\`yaml`-Block mit `id`/`tool`/`arguments` und emittiert `callbacks.toolCall(new ToolCall(...))`.
      `MCPClient.renderToolCall(String name, JsonNode arguments)` existiert bereits.

      ## Aktionen
      1. `MCPControlClient.renderSchema` (bzw. der Rückgabepfad in `MCPConnector.executeRequest` case `Tool`):
         - Dem yaml-Template das Präfix `EditorInterface.TOOLUSE` (`"Tool:"`) auf eigener Zeile voranstellen.
         - Nach dem schließenden ` ``` ` eine Kommandozeile `EditorInterface.CMD_CALL` (`"/call"`) anhängen.
         Ergebnisform:
         ```
         Tool:
         ```yaml
         tool: <name>
         arguments:
           ...
         ```
         /call
         ```
      2. `PromptHandler` (Step 06, Kommandoerkennung): Bei `/call` in der Zeile
         - rückwärts ab der Kommandozeile den vorausgehenden yaml-Block erfassen: Startgrenze Regex
           `^\`\`\`yaml$`, Endgrenze `^\`\`\`$` (die Zeile unmittelbar vor `/call`).
         - Block parsen/typisieren (`CallCommand`), in `Prompt` ablegen (`command` + `yamlBlock`).
         - Bei FullFile-Modus: Cursorzeile bzw. letzte Zeile gemäß Regeln aus Step 06.
      3. Übergabe an MCP: Der MCP-Pfad (`MCPConnector.executeRequest` / `MCPClient.callTool`) erhält den
         getypten Call aus dem `Prompt` (statt erneutem String-Parsing). `fillReason` weiterhin anwenden.
      4. `SessionProcessor` (Step 04): Der `Tool:`-Block wird via `consumeToolCall` in den korrekten
         Message-Typ gewandelt; die nachfolgende `/call`-Zeile ist ein REMOVE-Kommando (CallCommand) und wird
         ignoriert. Sicherstellen, dass `/call` einen offenen Block via `isMarkerLine` beendet (Step 04).

      ## Hinweise / Fallstricke
      - Der `Tool:`-Block muss vom SessionProcessor unverändert als Tool Call erkannt werden (Symmetrie/
        Austauschbarkeit zwischen Connectoren). yaml-Schlüssel `tool`/`arguments`/optional `id` beibehalten.
      - `/call` darf das Prefix-Caching nicht unterbrechen: Kommandozeile am Blockende, deterministische Form.
      - Konsistenz mit bestehendem `MCPConnector` Prompt-Pfad (bisher `/tool` → Template, freier yaml-Block →
        Call). `/tool` bleibt für Template-Anforderung; `/call` löst den Call aus.

      ## Validierung
      `ast_validate` für `connector/mcp/MCPControlClient.java`, `connector/mcp/MCPConnector.java`,
      `connector/harness/PromptHandler.java`, `connector/harness/SessionProcessor.java`.
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/09-cleanup.md
    source: |
      # Step 09 – Cleanup & Gesamtvalidierung (Phase 8)

      **Ziel:** Redundanzen entfernen, Konsistenz herstellen, alles validieren.
      **Abhängigkeiten:** Steps 01–08 abgeschlossen.

      ## Aktionen
      1. Verstreute String-Literale entfernen: Alle verbliebenen `"/exit"`, `"/resume"`, `"/answer"`,
         `"/tool"`, `"Control Request:"` durch `EditorInterface.CMD_*` / `CONTROL_REQUEST` bzw.
         `CommandHandler` ersetzen (grep im gesamten `connector/`-Baum).
      2. Prüfen, ob `CommandType` (claudecode) / `MCPCommandType` und die inneren `Command`/`MCPCommand`
         noch nötig sind oder auf die globale `Command`-Hierarchie konsolidiert werden können. Falls
         Konsolidierung: `CCRequest.Command`, `MCPCommand`, `MCPRequest`-Payload auf globale Typen umstellen.
         Sonst: Mapping-Schicht dokumentieren.
      3. `AISessionManager` auf verbleibende tote Prompt-Logik prüfen (nach Auslagerung in `PromptHandler`).
      4. `sessionProcessor.setEnabled` – prüfen, dass es keine direkten Aufrufe außerhalb des
         Kompatibilitätspfads mehr gibt (grep). Aktivierung ausschließlich über `Prompt.processorEnabled`
         / `process(inputs, enabled, cb)`.
      5. Invarianten-Review: Symmetrie (Tool-Call-/Result-Blöcke connectorübergreifend durchleitbar),
         Determinismus, Prefix-Caching (stabile Block-Reihenfolge/Whitespace).

      ## Grep-Checkliste
      ```
      grep -rE '"/(exit|resume|answer|tool|call)"' src/xy/ai/workbench/connector
      grep -rE 'Control Request:' src/xy/ai/workbench
      grep -rE 'createRequest\(' src/xy/ai/workbench            # keine Alt-Signatur mehr
      grep -rE 'sessionProcessor\.setEnabled' src/xy/ai/workbench
      grep -rE 'ClaudeSessionParameters\.fromConfig' src/xy/ai/workbench
      ```

      ## Finale Validierung (`ast_validate`)
      - `EditorInterface.java`, `AISessionManager.java`, `Activator.java`, `ConfigManager.java`
      - `connector/`: `IAIConnector.java`, `AdaptingConnector.java`, `Command.java`, `CommandHandler.java`,
        `SessionParameters.java`
      - `connector/harness/`: `SessionProcessor.java`, `Prompt.java`, `FrozenConfig.java`, `PromptHandler.java`
      - `connector/claudecode/`: `CCControlClient.java`, `CCConnector.java`, `CCRequest.java`,
        `CommandType.java`, `ClaudeSessionParameters.java`
      - `connector/mcp/`: `MCPConnector.java`, `MCPControlClient.java`, `MCPClient.java`, `MCPRequest.java`
      - `connector/claude/ClaudeConnector.java`, `connector/deepseek/DeepSeekConnector.java`,
        `connector/google/GeminiConnector.java`, `connector/openai/OpenAIConnector.java`
      - `view/AISessionView.java` (Aufrufer)
  reason: "Erstellung des eigenständigen, mehrteiligen Umsetzungsplans im Zielverzeichnis plan1"
```
/answer 7fa5-24 allow
ReasoningToken: 1774
Result Stats:  id=ea8143ba-d655-4aff-a0b8-83dffb2c226a, total: 0, in: 147, out: 36945, reason: 0, read: 627595, write: 90098
Plan erstellt in `/home/user/xyan/xy.ai.workbench/project/plan1`:

| Datei | Inhalt |
|---|---|
| `00-overview.md` | Ziel, Invarianten, Kommando-Semantik-Tabelle, Phasensequenz, **vollständige Datei-/Node-ID-Referenz** |
| `01-command-constants.md` | Phase 0: Globale Kommando-Konstanten in `EditorInterface` |
| `02-session-parameters.md` | Phase 1: `SessionParameters` aus `ClaudeSessionParameters` extrahieren |
| `03-command-handler.md` | Phase 2: `Command`-Hierarchie + `CommandHandler`-Registry |
| `04-session-processor.md` | Phase 3: Registry-Anbindung (ANSWER/CONTROL_REQUEST/…) |
| `05-prompt-object.md` | Phase 4: `FrozenConfig` + `Prompt` (Session-ID-Hash) |
| `06-prompt-handler.md` | Phase 5: `PromptHandler` + Eingabemodus-Kommandoerkennung |
| `07-connector-api.md` | Phase 6: `createRequest(Prompt,mon)` für 6 Connectoren |
| `08-call-command.md` | Phase 7: `/call`-Kommando (MCP-Template + Handler + Processor) |
| `09-cleanup.md` | Phase 8: Redundanzen, Grep-Checkliste, Gesamtvalidierung |

Jeder Step ist eigenständig: enthält Ist-Code-Auszüge, konkrete Node-IDs, Methodensignaturen, Aktionen, Fallstricke und Validierungsanweisungen. Die Phasen laufen strikt vorwärts, jede ist einzeln via `ast_validate` prüfbar.
