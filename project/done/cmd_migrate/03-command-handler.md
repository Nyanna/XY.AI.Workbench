# Step 03 – Globaler CommandHandler + Command-Hierarchie (Phase 2)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
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

SystemInit:  id=081ef136-43e8-419f-af0c-4c39cf0c4b95, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7fa5-4d
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
  reason: "Übersicht des Plans lesen für Kontext (ProcessorAction-Tabelle, Konventionen)"
```
/answer 7fa5-4d allow
Control Request:
```yaml
id: 7fa5-4e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
      nodes:
      - id: 479ae8
        type: section
        children:
        - id: 479ae8.44d126
          type: section
          children:
          - id: 479ae8.44d126.9vMhJA
            type: paragraph
        - id: 479ae8.f6be5a
          type: section
        - id: 479ae8.272b68
          type: section
        - id: 479ae8.e567e7
          type: section
          children:
          - id: 479ae8.e567e7.ytf8sA
            type: paragraph
        - id: 479ae8.67697a
          type: section
          children:
          - id: 479ae8.67697a.KpI5RD
            type: paragraph
          - id: 479ae8.67697a.904b7d
            type: section
          - id: 479ae8.67697a.5b3be9
            type: section
          - id: 479ae8.67697a.d2dc72
            type: section
          - id: 479ae8.67697a.623799
            type: section
          - id: 479ae8.67697a.4c2f5f
            type: section
          - id: 479ae8.67697a.62db94
            type: section
```
/answer 7fa5-4e allow
Control Request:
```yaml
id: 7fa5-4f
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
  reason: Struktur des connector-Packages einsehen
```
/answer 7fa5-4f allow
Control Request:
```yaml
id: 7fa5-50
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      error: Not a regular file.
```
/answer 7fa5-50 allow
ReasoningToken: 29
Control Request:
```yaml
id: 7fa5-51
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
    ids:
    - 479ae8
  reason: Volltext der Übersicht lesen (ProcessorAction-Tabelle etc.)
```
/answer 7fa5-51 allow
Control Request:
```yaml
id: 7fa5-52
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
      nodes:
      - id: 479ae8
        type: section
        children:
        - id: 479ae8.44d126
          type: section
          children:
          - id: 479ae8.44d126.9vMhJA
            type: paragraph
            code: |
              Zentrale Vereinheitlichung von Nutzung, Behandlung und Implementierung der Kommandos, die im
              Editor angegeben/ausgeführt werden. Der Session Processor erhält eine zentrale Anlaufstelle,
              um Kommandos von normalem Text zu unterscheiden. Ein globaler Command Handler stellt die
              Infrastruktur bereit; die prompt-spezifische Logik wird in einen Prompt-Handler ausgelagert;
              die Session-Parameter werden in eine generische Basisklasse extrahiert; alle Connectoren
              werden auf ein einheitliches Prompt-Objekt umgestellt.
        - id: 479ae8.f6be5a
          type: section
          code: |+
            ## Invarianten (müssen erhalten bleiben)
            - **Symmetrie / Austauschbarkeit:** Ein-/Ausgaben der Komponenten müssen zwischen Connectoren
              austauschbar bleiben. Tool-Call-/Result-Blöcke, die von MCP oder Claude erzeugt wurden,
              müssen auch durch einen anderen Connector geleitet werden können.
            - **Determinismus:** Gleiche Eingabe ⇒ gleiche Ausgabe.
            - **Prefix-Caching:** Umformungen dürfen das Prefix-Caching nicht unterbrechen.
            - **Kein permanenter Testsuite-Lauf.** Validierung erfolgt via `ast_validate` je berührter Datei.

        - id: 479ae8.272b68
          type: section
          code: |
            ## Kommando-Semantik im Session Processor
            | Kommando | Präfix | Behandlung im SessionProcessor |
            |---|---|---|
            | ANSWER | `/answer` | TRANSFORM: Kommando inkl. Option (`allow`/`deny`) entfernen, nur optionalen Resttext der Zeile behalten; beendet dennoch einen ggf. offenen Block (meist vorausgehender Tool Call → erzeugt Message-Block). |
            | CONTROL_REQUEST | `Control Request:` | IGNORE_BLOCK: Zeile **und** folgenden `\`\`\`yaml`-Block ignorieren (nicht-modellgenerierte Tool-Call-Anforderung). |
            | RESUME | `/resume` | REMOVE: Zeile aus Eingabe entfernen (nur Session-Kontext). |
            | EXIT | `/exit` | REMOVE. |
            | CALL | `/call` | REMOVE der Kommandozeile; der vorausgehende `Tool:`-Block wird als Tool Call in den korrekten Message-Typ gewandelt. |
            | TOOL | `/tool` | REMOVE (Template-Anforderung). |
        - id: 479ae8.e567e7
          type: section
          children:
          - id: 479ae8.e567e7.ytf8sA
            type: paragraph
            code: |
              Jede Phase ist einzeln compilierbar. Nach jeder Phase: `ast_validate` der berührten Dateien.
        - id: 479ae8.67697a
          type: section
          children:
          - id: 479ae8.67697a.KpI5RD
            type: paragraph
            code: |
              Alle Pfade absolut unter `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/`.
          - id: 479ae8.67697a.904b7d
            type: section
            code: |+
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

          - id: 479ae8.67697a.5b3be9
            type: section
            code: |+
              ### Connector-Infrastruktur (`connector/`)
              - `IAIConnector.java` – **Node `IAIConnector`**; Methoden `.createRequest`, `.executeRequest`,
                `.convertResponse`, `.getSupportedKeyPattern`. Aktuelle Signatur:
                `REQ createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix, IProgressMonitor mon);`
              - `AdaptingConnector.java` – `.createRequest` delegiert an `getConnector(cfg.getModel())`;
                `.executeRequest` delegiert an `getConnector(request)`.

          - id: 479ae8.67697a.d2dc72
            type: section
            code: |+
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

          - id: 479ae8.67697a.623799
            type: section
            code: |+
              ### API-Connectoren
              - `claude/ClaudeConnector.java` – `.ClaudeConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`, `.appendTools`, `.executeRequest`, `.convertResponse`.
              - `deepseek/DeepSeekConnector.java` – `.DeepSeekConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`, innere `SessionRequestCallbacks`.
              - `google/GeminiConnector.java` – `.GeminiConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`, `.getThinkingBudget`.
              - `openai/OpenAIConnector.java` – `.OpenAIConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`.

          - id: 479ae8.67697a.4c2f5f
            type: section
            code: |+
              ### Harness (`connector/harness/`)
              - `SessionProcessor.java` – `.process(List<String>,cb)`, `.process(String,cb)`, `.setEnabled`, innere `Run`:
                `.run`, `.flush`, `.isMarkerLine`, `.consumeReasoning`, `.consumeText`, `.consumeToolCall`,
                `.consumeToolResult`, `.fenceRange`, `.readYaml`, `.include*`.
              - `SessionCallbacks.java`, `SessionRenderer.java`, `SessionAnswerBuilder.java` (Tool-Call-Rendering).

          - id: 479ae8.67697a.62db94
            type: section
            code: |
              ### MCP (`connector/mcp/`)
              - `MCPConnector.java` – `.preprocess` (`/exit`,`/tool`), `.createRequest`, `.executeRequest`.
              - `MCPControlClient.java` – `.renderSchema` (liefert `\`\`\`yaml … \`\`\``), `.extractYamlBlock`,
                `.parseYaml`, `.fillReason`, `.prettyResult`.
              - `MCPClient.java` – `.findTool`, `.callTool`, `.renderToolCall`.
              - `MCPRequest.java` – Konstruktor `MCPRequest(String id, String systemPrompt, List<String> tools, MCPCommand cmd)`.
              - `MCPCommand.java`, `MCPCommandType.java` – `enum { Exit, Tool, Prompt }` (+ ggf. weitere).
```
/answer 7fa5-52 allow
Control Request:
```yaml
id: 7fa5-53
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
    pattern: .
    include:
    - '*.java'
    limit: 50
  reason: Liste der vorhandenen Java-Dateien im connector-Package
```
/answer 7fa5-53 allow
Control Request:
```yaml
id: 7fa5-54
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      pattern: .
      matches:
      - path: mcp/MCPClient.java
        lineno: 1
        match: package xy.ai.workbench.connector.mcp;
      - path: mcp/MCPClient.java
        lineno: 3
        match: import java.io.IOException;
      - path: mcp/MCPClient.java
        lineno: 4
        match: import java.net.URI;
      - path: mcp/MCPClient.java
        lineno: 5
        match: import java.net.http.HttpClient;
      - path: mcp/MCPClient.java
        lineno: 6
        match: import java.net.http.HttpRequest;
      - path: mcp/MCPClient.java
        lineno: 7
        match: import java.net.http.HttpRequest.BodyPublishers;
      - path: mcp/MCPClient.java
        lineno: 8
        match: import java.net.http.HttpResponse;
      - path: mcp/MCPClient.java
        lineno: 9
        match: import java.net.http.HttpResponse.BodyHandlers;
      - path: mcp/MCPClient.java
        lineno: 10
        match: import java.time.Duration;
      - path: mcp/MCPClient.java
        lineno: 11
        match: import java.util.UUID;
      - path: mcp/MCPClient.java
        lineno: 13
        match: import com.fasterxml.jackson.core.JsonProcessingException;
      - path: mcp/MCPClient.java
        lineno: 14
        match: import com.fasterxml.jackson.databind.JsonNode;
      - path: mcp/MCPClient.java
        lineno: 15
        match: import com.fasterxml.jackson.databind.ObjectMapper;
      - path: mcp/MCPClient.java
        lineno: 16
        match: import com.fasterxml.jackson.databind.node.ArrayNode;
      - path: mcp/MCPClient.java
        lineno: 17
        match: import com.fasterxml.jackson.databind.node.ObjectNode;
      - path: mcp/MCPClient.java
        lineno: 19
        match: import xy.ai.workbench.LOG;
      - path: mcp/MCPClient.java
        lineno: 20
        match: import xy.ai.workbench.connector.claudecode.JsonUtil;
      - path: mcp/MCPClient.java
        lineno: 21
        match: import xy.ai.workbench.connector.claudecode.YamlRenderer;
      - path: mcp/MCPClient.java
        lineno: 23
        match: /**
      - path: mcp/MCPClient.java
        lineno: 24
        match: " * Minimal MCP client over the Streamable-HTTP transport. Connection, session id"
      - path: mcp/MCPClient.java
        lineno: 25
        match: ' * and tool cache live 1:1 in this instance; no shared manager.'
      - path: mcp/MCPClient.java
        lineno: 26
        match: ' */'
      - path: mcp/MCPClient.java
        lineno: 27
        match: "public class MCPClient {"
      - path: mcp/MCPClient.java
        lineno: 28
        match: "\tprivate static final String SERVER_URL = \"http://localhost:9093/mcp\";"
      - path: mcp/MCPClient.java
        lineno: 29
        match: "\tprivate static final String PROTOCOL_VERSION = \"2025-06-18\";"
      - path: mcp/MCPClient.java
        lineno: 30
        match: "\tprivate static final Duration TIMEOUT = Duration.ofSeconds(300);"
      - path: mcp/MCPClient.java
        lineno: 32
        match: "\tprivate final ObjectMapper mapper = JsonUtil.mapper();"
      - path: mcp/MCPClient.java
        lineno: 33
        match: "\tprivate final YamlRenderer yaml = new YamlRenderer();"
      - path: mcp/MCPClient.java
        lineno: 34
        match: "\tprivate final HttpClient http = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(5)).build();"
      - path: mcp/MCPClient.java
        lineno: 36
        match: "\t/** Instance-stable session id sent as X-MCPC-SESSION-ID. */"
      - path: mcp/MCPClient.java
        lineno: 37
        match: "\tprivate final String sessionId = UUID.randomUUID().toString();"
      - path: mcp/MCPClient.java
        lineno: 39
        match: "\tprivate boolean initialized;"
      - path: mcp/MCPClient.java
        lineno: 40
        match: "\tprivate String mcpSessionId;"
      - path: mcp/MCPClient.java
        lineno: 41
        match: "\tprivate ArrayNode toolCache;"
      - path: mcp/MCPClient.java
        lineno: 42
        match: "\tprivate long nextId = 1;"
      - path: mcp/MCPClient.java
        lineno: 44
        match: "\tpublic JsonNode findTool(String name) {"
      - path: mcp/MCPClient.java
        lineno: 45
        match: "\t\tfor (JsonNode tool : listTools())"
      - path: mcp/MCPClient.java
        lineno: 46
        match: "\t\t\tif (name.equals(tool.path(\"name\").asText()))"
      - path: mcp/MCPClient.java
        lineno: 47
        match: "\t\t\t\treturn tool;"
      - path: mcp/MCPClient.java
        lineno: 48
        match: "\t\treturn null;"
      - path: mcp/MCPClient.java
        lineno: 49
        match: "\t}"
      - path: mcp/MCPClient.java
        lineno: 51
        match: "\tpublic synchronized ArrayNode listTools() {"
      - path: mcp/MCPClient.java
        lineno: 52
        match: "\t\tif (toolCache != null)"
      - path: mcp/MCPClient.java
        lineno: 53
        match: "\t\t\treturn toolCache;"
      - path: mcp/MCPClient.java
        lineno: 54
        match: "\t\tensureConnected();"
      - path: mcp/MCPClient.java
        lineno: 55
        match: "\t\tJsonNode result = send(request(\"tools/list\", mapper.createObjectNode()), true);"
      - path: mcp/MCPClient.java
        lineno: 56
        match: "\t\tJsonNode tools = result.path(\"tools\");"
      - path: mcp/MCPClient.java
        lineno: 57
        match: "\t\ttoolCache = tools.isArray() ? (ArrayNode) tools : mapper.createArrayNode();"
      - path: mcp/MCPClient.java
        lineno: 58
        match: "\t\treturn toolCache;"
      - path: mcp/MCPClient.java
        lineno: 59
        match: "\t}"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 7fa5-54 allow
ReasoningToken: 91
Control Request:
```yaml
id: 7fa5-55
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
    pattern: ^package
    include:
    - '*.java'
    limit: 50
  reason: Alle Java-Dateien im connector-Package auflisten
```
/answer 7fa5-55 allow
Control Request:
```yaml
id: 7fa5-56
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      pattern: ^package
      matches:
      - path: mcp/MCPClient.java
        lineno: 1
        match: package xy.ai.workbench.connector.mcp;
      - path: mcp/MCPResponse.java
        lineno: 1
        match: package xy.ai.workbench.connector.mcp;
      - path: mcp/MCPControlClient.java
        lineno: 1
        match: package xy.ai.workbench.connector.mcp;
      - path: mcp/MCPRequest.java
        lineno: 1
        match: package xy.ai.workbench.connector.mcp;
      - path: mcp/MCPCommandType.java
        lineno: 1
        match: package xy.ai.workbench.connector.mcp;
      - path: mcp/MCPCommand.java
        lineno: 1
        match: package xy.ai.workbench.connector.mcp;
      - path: mcp/MCPConnector.java
        lineno: 1
        match: package xy.ai.workbench.connector.mcp;
      - path: claudecode/TimedLineReader.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/HookParser.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/CCSession.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/YamlRenderer.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/SessionState.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/CCResponse.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/ProtocolParser.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/CCControlClient.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/CommandType.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/CCConnector.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/CCRequestBuilder.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/CCSessionManager.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/ClaudeSessionParameters.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/JsonUtil.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/EditorLocation.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: claudecode/CCRequest.java
        lineno: 1
        match: package xy.ai.workbench.connector.claudecode;
      - path: openapi/deepseek/operators/ElementEnum.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfParameters.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfContext.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/DirectToolCallCallerParam.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/PhaseEnum.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/ContextEnum.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfCaller.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/OutputList.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfAllowedCallers.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/EffortEnum.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfStream.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfMaxOutputTokens.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/Reasoning.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfInstructions.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/OneOfElement.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfMode.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AllowedCallersList.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfPhase.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfTemperature.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfEffort.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfReasoning.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/DirectToolCallCaller.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/AnyOfElementCaller.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/OneOfElementCaller.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/OneOfOutput.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/OneOfCaller.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      - path: openapi/deepseek/operators/ModeEnum.java
        lineno: 1
        match: package xy.ai.workbench.connector.openapi.deepseek.operators;
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 7fa5-56 allow
ReasoningToken: 91
Text:
 Jetzt die relevanten Dateien lesen: CCConnector, CommandType, CCRequest, MCPConnector, MCPCommand(Type), CCControlClient (für Konstanten), EditorInterface.
Control Request:
```yaml
id: 7fa5-57
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    ids:
    - '*'
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - '*'
  reason: "Vollständigen Code der Ist-Logik lesen, um Command-Hierarchie und CommandHandler korrekt zu implementieren"
```
/answer 7fa5-57 allow
Control Request:
```yaml
id: 7fa5-58
result:
  structuredContent:
    results:
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
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
      nodes:
      - id: 8zE170
        type: imports
      - id: CommandType
        type: enum_declaration
        signature: "public enum CommandType {"
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
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
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
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
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
      nodes:
      - id: TGrMqF
        type: imports
      - id: MCPCommand
        type: class_declaration
        signature: "public class MCPCommand {"
        children:
        - id: MCPCommand.QpkmbA
          type: statements
        - id: MCPCommand.MCPCommand
          type: constructor_declaration
          signature: "public MCPCommand(MCPCommandType type, String... parameters) {"
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
      nodes:
      - id: QmRDpJ
        type: imports
      - id: MCPCommandType
        type: enum_declaration
        signature: "public enum MCPCommandType {"
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: WHD6Rc
        type: imports
      - id: NegutY
        type: imports
      - id: AQWMTb
        type: statements
      - id: CCControlClient
        type: class_declaration
        signature: "public class CCControlClient {"
        children:
        - id: CCControlClient.F5qBaf
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
      errors:
      - "ids ['*'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 7fa5-58 allow
Text:
 Jetzt die konkreten Node-Inhalte lesen (Code statt nur Outline):
Control Request:
```yaml
id: 7fa5-59
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - YZNdWp
    - 84X5CG
    - CCConnector.oN6Hbi
    - CCConnector.preprocessInput
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
    ids:
    - CommandType
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    ids:
    - yJKOSV
    - CCRequest.Ki6WvE
    - CCRequest.Command
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - sGAUbf
    - MCPConnector.xV4uSy
    - MCPConnector.preprocess
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
    ids:
    - TGrMqF
    - MCPCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    ids:
    - MCPCommandType
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - WHD6Rc
    - CCControlClient.F5qBaf
    - CCControlClient.submitEdit
    - CCControlClient.extractYamlBlock
  reason: "Vollständigen Code der Ist-Logik lesen, um Command-Hierarchie und CommandHandler korrekt zu implementieren"
```
/answer 7fa5-59 allow
Control Request:
```yaml
id: 7fa5-5a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: YZNdWp
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import java.io.IOException;
          import java.nio.file.Path;
          import java.nio.file.Paths;
          import java.util.ArrayList;
          import java.util.List;
          import java.util.UUID;
          import java.util.zip.CRC32;

          import org.eclipse.core.resources.IProject;
          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.SubMonitor;
          import org.eclipse.core.runtime.jobs.Job;
          import org.eclipse.swt.widgets.Display;
          import org.eclipse.ui.IEditorInput;
      - id: 84X5CG
        type: imports
        code: |-
          import org.eclipse.ui.IEditorPart;
          import org.eclipse.ui.IFileEditorInput;
          import org.eclipse.ui.IWorkbenchPage;
          import org.eclipse.ui.IWorkbenchWindow;
          import org.eclipse.ui.PlatformUI;

          import xy.ai.workbench.AgentProfile;
          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model.KeyPattern;
          import xy.ai.workbench.connector.IAIConnector;
          import xy.ai.workbench.connector.claudecode.CCRequest.Command;
          import xy.ai.workbench.models.AIAnswer;
      - id: CCConnector.oN6Hbi
        type: statements
        code: "private final CCRequestBuilder requestBuilder = new CCRequestBuilder();\n\tprivate final ProtocolParser jsonParser = new ProtocolParser();\n\tprivate final CCControlClient controlClient = new CCControlClient();\n\tprivate final CCSessionManager sessionManager;\n\n\tprivate ConfigManager cfg;"
      - id: CCConnector.preprocessInput
        type: method_declaration
        code: "private List<Command> preprocessInput(List<String> inputs) {\n\t\tList<Command> commands = new ArrayList<Command>();\n\t\tString clean;\n\t\tfor (String input : inputs)\n\t\t\tif (!(clean = input != null ? input.strip() : \"\").isBlank())\n\t\t\t\tif (\"/exit\".equalsIgnoreCase(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Exit, \"\"));\n\t\t\t\telse if (clean.matches(\"(?i)/resume\\\\s+\\\\S+\"))\n\t\t\t\t\tcommands.add(new Command(CommandType.Resume, clean.split(\"\\\\s+\", 2)[1].strip()));\n\t\t\t\telse if (clean.matches(\"(?i)\" + CCControlClient.ANSWER + \"\\\\s+\\\\S+\\\\s+(allow|deny)(\\\\s+.*)?\")) {\n\t\t\t\t\tString[] parts = clean.split(\"\\\\s+\", 4);\n\t\t\t\t\tString id = parts[1];\n\t\t\t\t\tString action = parts[2].toLowerCase();\n\t\t\t\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id, reason));\n\t\t\t\t\telse\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));\n\t\t\t\t} else if (controlClient.submitEdit(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Modification, \"\"));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Prompt, clean));\n\t\tif (commands.isEmpty())\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn commands;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
      nodes:
      - id: CommandType
        type: enum_declaration
        code: "public enum CommandType {\n\tPrompt, Exit, Resume, Allow, Deny, Modification;\n}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: yJKOSV
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import java.util.Collections;
          import java.util.List;
          import java.util.Objects;

          import xy.ai.workbench.models.IModelRequest;
      - id: CCRequest.Ki6WvE
        type: statements
        code: "public final String id;\n\tpublic final String title;\n\n\tpublic final String systemPrompt;\n\tpublic final List<String> tools;\n\tpublic final Command cmd;"
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: sGAUbf
        type: imports
        code: |-
          package xy.ai.workbench.connector.mcp;

          import java.util.List;
          import java.util.UUID;

          import org.eclipse.core.runtime.IProgressMonitor;
          import org.eclipse.core.runtime.jobs.Job;

          import com.fasterxml.jackson.databind.JsonNode;

          import xy.ai.workbench.ConfigManager;
          import xy.ai.workbench.Model.KeyPattern;
          import xy.ai.workbench.connector.IAIConnector;
          import xy.ai.workbench.models.AIAnswer;
      - id: MCPConnector.xV4uSy
        type: statements
        code: "private final MCPClient client;\n\tprivate final MCPControlClient control = new MCPControlClient();\n\n\t@SuppressWarnings(\"unused\")\n\tprivate final ConfigManager cfg;"
      - id: MCPConnector.preprocess
        type: method_declaration
        code: "private MCPCommand preprocess(List<String> inputs) {\n\t\tStringBuilder merged = null;\n\t\tfor (String input : inputs) {\n\t\t\tString clean = input != null ? input.strip() : \"\";\n\t\t\tif (clean.isBlank())\n\t\t\t\tcontinue;\n\t\t\tif (\"/exit\".equalsIgnoreCase(clean))\n\t\t\t\treturn new MCPCommand(MCPCommandType.Exit, \"\");\n\t\t\tif (clean.matches(\"(?i)/tool\\\\s+\\\\S+\"))\n\t\t\t\treturn new MCPCommand(MCPCommandType.Tool, clean.split(\"\\\\s+\", 2)[1].strip());\n\t\t\tif (merged == null)\n\t\t\t\tmerged = new StringBuilder();\n\t\t\telse\n\t\t\t\tmerged.append(\"\\n\");\n\t\t\tmerged.append(clean);\n\t\t}\n\t\tif (merged == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn new MCPCommand(MCPCommandType.Prompt, merged.toString());\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
      nodes:
      - id: TGrMqF
        type: imports
        code: |-
          package xy.ai.workbench.connector.mcp;

          import java.util.Objects;
      - id: MCPCommand
        type: class_declaration
        signature: "public class MCPCommand {"
        children:
        - id: MCPCommand.QpkmbA
          type: statements
          code: "public final MCPCommandType type;\n\tpublic final String parameter;\n\tpublic final String[] parameters;"
        - id: MCPCommand.MCPCommand
          type: constructor_declaration
          code: "public MCPCommand(MCPCommandType type, String... parameters) {\n\t\tObjects.requireNonNull(type, \"Type can't be null\");\n\t\tthis.type = type;\n\t\tthis.parameter = parameters[0];\n\t\tthis.parameters = parameters;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
      nodes:
      - id: MCPCommandType
        type: enum_declaration
        code: "public enum MCPCommandType {\n\tPrompt, Tool, Exit;\n}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: WHD6Rc
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import java.io.IOException;
          import java.net.URI;
          import java.net.http.HttpClient;
          import java.net.http.HttpRequest;
          import java.net.http.HttpRequest.BodyPublishers;
          import java.net.http.HttpResponse;
          import java.net.http.HttpResponse.BodyHandlers;
          import java.time.Duration;

          import com.fasterxml.jackson.core.JsonProcessingException;
          import com.fasterxml.jackson.databind.JsonNode;
          import com.fasterxml.jackson.databind.ObjectMapper;
      - id: CCControlClient.F5qBaf
        type: statements
        code: "public static final String ANSWER = EditorInterface.CMD_ANSWER;\n\tpublic static final String CONTROL_REQUEST = EditorInterface.CONTROL_REQUEST;\n\tprivate static final String CONTROL_URL = \"http://localhost:9093/control/tool\";\n\tprivate static final Duration TIMEOUT = Duration.ofSeconds(5);\n\n\tprivate final ObjectMapper mapper = JsonUtil.mapper();\n\tprivate YamlRenderer yaml = new YamlRenderer();\n\tprivate final HttpClient http = HttpClient.newBuilder().connectTimeout(TIMEOUT).build();"
      - id: CCControlClient.submitEdit
        type: method_declaration
        code: "public boolean submitEdit(String rawText) {\n\t\tif (rawText == null)\n\t\t\treturn false;\n\t\tString block = extractYamlBlock(rawText.strip());\n\t\tif (block == null)\n\t\t\treturn false;\n\n\t\tJsonNode edited;\n\t\ttry {\n\t\t\tedited = fromYaml(block);\n\t\t} catch (Exception e) {\n\t\t\tthrow new IllegalArgumentException(\"YAML Error\", e);\n\t\t}\n\t\tif (edited == null || !edited.isObject() || !edited.hasNonNull(\"id\"))\n\t\t\treturn false;\n\t\tsubmit((ObjectNode) edited);\n\t\treturn true;\n\t}"
      - id: CCControlClient.extractYamlBlock
        type: method_declaration
        code: "private String extractYamlBlock(String text) {\n\t\tint start = text.indexOf(\"```yaml\");\n\t\tif (start != 0)\n\t\t\treturn null;\n\t\tint contentStart = start + \"```yaml\".length();\n\t\tint end = text.indexOf(\"```\", contentStart);\n\t\tif (end == -1)\n\t\t\treturn null;\n\t\treturn text.substring(contentStart, end).strip();\n\t}"
```
/answer 7fa5-5a allow
Control Request:
```yaml
id: 7fa5-5b
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: CMD_|CONTROL_REQUEST
    include:
    - EditorInterface.java
    limit: 50
  reason: Konstanten CMD_* und CONTROL_REQUEST in EditorInterface finden (aus Step 01)
```
/answer 7fa5-5b allow
Control Request:
```yaml
id: 7fa5-5c
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      pattern: CMD_|CONTROL_REQUEST
      matches:
      - path: EditorInterface.java
        lineno: 48
        match: "\tpublic static final String CMD_ANSWER = \"/answer\";"
      - path: EditorInterface.java
        lineno: 49
        match: "\tpublic static final String CMD_RESUME = \"/resume\";"
      - path: EditorInterface.java
        lineno: 50
        match: "\tpublic static final String CMD_EXIT   = \"/exit\";"
      - path: EditorInterface.java
        lineno: 51
        match: "\tpublic static final String CMD_TOOL   = \"/tool\";"
      - path: EditorInterface.java
        lineno: 52
        match: "\tpublic static final String CMD_CALL   = \"/call\";"
      - path: EditorInterface.java
        lineno: 53
        match: "\tpublic static final String CONTROL_REQUEST = \"Control Request:\";"
```
/answer 7fa5-5c allow
ReasoningToken: 115
Control Request:
```yaml
id: 7fa5-5d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: "\\bvar [a-zA-Z]+ ="
    include:
    - '*.java'
    limit: 10
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: source|target
    include:
    - .classpath
    limit: 10
  reason: Prüfen ob 'var' (Java 10+) im Projekt verwendet wird bzw. Java-Version bestimmen
```
/answer 7fa5-5d allow
Control Request:
```yaml
id: 7fa5-5e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      pattern: "\\bvar [a-zA-Z]+ ="
      matches:
      - path: xy/ai/workbench/AISessionManager.java
        lineno: 196
        match: "\t\t\t\tvar req = prepareInner(display, false, sub);"
      - path: xy/ai/workbench/AISessionManager.java
        lineno: 204
        match: "\t\t\t\tvar ans = executeInner(display, req, sub, this);"
      - path: xy/ai/workbench/AISessionManager.java
        lineno: 242
        match: "\t\tvar req = prepareInner(display, true, sub.split(1));"
      - path: xy/ai/workbench/view/explorer/GitIgnoreFilter.java
        lineno: 93
        match: "\t\ttry (var in = gitignore.getContents()) {"
      - path: xy/ai/workbench/view/AISessionView.java
        lineno: 519
        match: "\t\t\t\tvar checked = item.getChecked();"
      - path: xy/ai/workbench/view/AIBatchView.java
        lineno: 190
        match: "\t\tvar res = new TableColumn(parent, SWT.NONE);"
      - path: xy/ai/workbench/editor/mdast/nodes/AbstractNode.java
        lineno: 47
        match: "\t\t\t\tvar nn = new Node(n, child);"
      - path: xy/ai/workbench/editor/AITextEditor.java
        lineno: 93
        match: "\t\t\tvar selection = getSourceViewer().getSelectedRange();"
      - path: xy/ai/workbench/editor/AITextEditor.java
        lineno: 172
        match: "\t\t\tfor (var i = 0; i < decorators.size(); i++)"
      - path: xy/ai/workbench/editor/outline/MarkdownOutlinePage.java
        lineno: 316
        match: "\t\tvar sel = viewer.getSelection();"
      warning: "Limit of 10 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    - directory:
      - /home/user/xyan/xy.ai.workbench
      pattern: source|target
      matches: []
```
/answer 7fa5-5e allow
ReasoningToken: 8400
Text:
 Gut, `var` wird verwendet. Jetzt erstelle ich die neuen Dateien.
Control Request:
```yaml
id: 7fa5-5f
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/Command.java
    source: "package xy.ai.workbench.connector;\n\n/**\n * Abstrakte Basis fuer alle Editor-/Session-Kommandos. Konkrete Kommandos\n * kapseln Erkennung (matches/prefix), Parsing (parse) und Behandlung im\n * SessionProcessor (processorAction).\n */\npublic abstract class Command {\n\n\tpublic enum ProcessorAction {\n\t\tREMOVE, TRANSFORM, IGNORE_BLOCK, NONE\n\t}\n\n\tprotected final String[] parameters;\n\n\tprotected Command(String... parameters) {\n\t\tthis.parameters = parameters;\n\t}\n\n\tpublic abstract String prefix();\n\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().startsWith(prefix());\n\t}\n\n\tpublic abstract Command parse(String line);\n\n\tpublic abstract ProcessorAction processorAction();\n\n\tpublic String parameter(int i) {\n\t\treturn parameters[i];\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AnswerCommand.java
    source: "package xy.ai.workbench.connector;\n\nimport xy.ai.workbench.EditorInterface;\n\n/** {@code /answer <id> allow|deny [reason]} */\npublic class AnswerCommand extends Command {\n\n\tpublic AnswerCommand() {\n\t\tthis(\"\", \"\", \"\");\n\t}\n\n\tpublic AnswerCommand(String id, String action, String reason) {\n\t\tsuper(id, action, reason);\n\t}\n\n\t@Override\n\tpublic String prefix() {\n\t\treturn EditorInterface.CMD_ANSWER;\n\t}\n\n\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\\\\s+(allow|deny)(\\\\s+.*)?\");\n\t}\n\n\t@Override\n\tpublic Command parse(String line) {\n\t\tString[] parts = line.strip().split(\"\\\\s+\", 4);\n\t\tString id = parts[1];\n\t\tString action = parts[2].toLowerCase();\n\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\treturn new AnswerCommand(id, action, reason);\n\t}\n\n\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.TRANSFORM;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ControlRequestCommand.java
    source: "package xy.ai.workbench.connector;\n\nimport xy.ai.workbench.EditorInterface;\n\n/** {@code Control Request:} markiert den folgenden yaml-Block als nicht-modellgeneriert. */\npublic class ControlRequestCommand extends Command {\n\n\tpublic ControlRequestCommand() {\n\t\tsuper();\n\t}\n\n\t@Override\n\tpublic String prefix() {\n\t\treturn EditorInterface.CONTROL_REQUEST;\n\t}\n\n\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new ControlRequestCommand();\n\t}\n\n\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.IGNORE_BLOCK;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ResumeCommand.java
    source: "package xy.ai.workbench.connector;\n\nimport xy.ai.workbench.EditorInterface;\n\n/** {@code /resume <session-id>} */\npublic class ResumeCommand extends Command {\n\n\tpublic ResumeCommand() {\n\t\tthis(\"\");\n\t}\n\n\tpublic ResumeCommand(String sessionId) {\n\t\tsuper(sessionId);\n\t}\n\n\t@Override\n\tpublic String prefix() {\n\t\treturn EditorInterface.CMD_RESUME;\n\t}\n\n\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\");\n\t}\n\n\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new ResumeCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());\n\t}\n\n\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ExitCommand.java
    source: "package xy.ai.workbench.connector;\n\nimport xy.ai.workbench.EditorInterface;\n\n/** {@code /exit} */\npublic class ExitCommand extends Command {\n\n\tpublic ExitCommand() {\n\t\tsuper();\n\t}\n\n\t@Override\n\tpublic String prefix() {\n\t\treturn EditorInterface.CMD_EXIT;\n\t}\n\n\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && prefix().equalsIgnoreCase(line.strip());\n\t}\n\n\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new ExitCommand();\n\t}\n\n\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ToolCommand.java
    source: "package xy.ai.workbench.connector;\n\nimport xy.ai.workbench.EditorInterface;\n\n/** {@code /tool <tool-id>} */\npublic class ToolCommand extends Command {\n\n\tpublic ToolCommand() {\n\t\tthis(\"\");\n\t}\n\n\tpublic ToolCommand(String toolId) {\n\t\tsuper(toolId);\n\t}\n\n\t@Override\n\tpublic String prefix() {\n\t\treturn EditorInterface.CMD_TOOL;\n\t}\n\n\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\");\n\t}\n\n\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new ToolCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());\n\t}\n\n\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CallCommand.java
    source: "package xy.ai.workbench.connector;\n\nimport xy.ai.workbench.EditorInterface;\n\n/** {@code /call} erfasst den vorausgehenden {@code Tool:}-Block als Tool Call (siehe Step 08). */\npublic class CallCommand extends Command {\n\n\tpublic CallCommand() {\n\t\tsuper();\n\t}\n\n\t@Override\n\tpublic String prefix() {\n\t\treturn EditorInterface.CMD_CALL;\n\t}\n\n\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && prefix().equalsIgnoreCase(line.strip());\n\t}\n\n\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new CallCommand();\n\t}\n\n\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ModificationCommand.java
    source: "package xy.ai.workbench.connector;\n\n/**\n * yaml-Edit ohne Praefix; nur Claude Code erkennt dies (via CCControlClient.submitEdit),\n * daher nicht Teil der statischen {@link CommandHandler}-Registry.\n */\npublic class ModificationCommand extends Command {\n\n\tpublic ModificationCommand() {\n\t\tsuper();\n\t}\n\n\t@Override\n\tpublic String prefix() {\n\t\treturn \"\";\n\t}\n\n\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn false;\n\t}\n\n\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new ModificationCommand();\n\t}\n\n\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.NONE;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/PromptCommand.java
    source: "package xy.ai.workbench.connector;\n\n/** Default-Kommando ohne Praefix: gewoehnlicher Prompt-Text. */\npublic class PromptCommand extends Command {\n\n\tpublic PromptCommand() {\n\t\tthis(\"\");\n\t}\n\n\tpublic PromptCommand(String text) {\n\t\tsuper(text);\n\t}\n\n\t@Override\n\tpublic String prefix() {\n\t\treturn \"\";\n\t}\n\n\t@Override\n\tpublic boolean matches(String line) {\n\t\treturn true;\n\t}\n\n\t@Override\n\tpublic Command parse(String line) {\n\t\treturn new PromptCommand(line.strip());\n\t}\n\n\t@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.NONE;\n\t}\n}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CommandHandler.java
    source: "package xy.ai.workbench.connector;\n\nimport java.util.List;\nimport java.util.Optional;\n\n/**\n * Statische, hart kodierte Registry aller bekannten Editor-/Session-Kommandos.\n */\npublic final class CommandHandler {\n\n\tprivate static final List<Command> REGISTRY = List.of(new AnswerCommand(), new ControlRequestCommand(),\n\t\t\tnew ResumeCommand(), new ExitCommand(), new ToolCommand(), new CallCommand());\n\n\tprivate CommandHandler() {\n\t}\n\n\tpublic static Optional<Command> detect(String line) {\n\t\tfor (Command prototype : REGISTRY)\n\t\t\tif (prototype.matches(line))\n\t\t\t\treturn Optional.of(prototype.parse(line));\n\t\treturn Optional.empty();\n\t}\n\n\tpublic static List<Command> list() {\n\t\treturn REGISTRY;\n\t}\n}\n"
  reason: Command-Hierarchie und CommandHandler-Registry gemäß Step 03 anlegen
```
/answer 7fa5-5f allow
Control Request:
```yaml
id: 7fa5-60
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    start_marker: private List<Command> preprocessInput
    end_marker: "return commands;\n\t}"
    content: "private List<Command> preprocessInput(List<String> inputs) {\n\t\tList<Command> commands = new ArrayList<Command>();\n\t\tString clean;\n\t\tfor (String input : inputs)\n\t\t\tif (!(clean = input != null ? input.strip() : \"\").isBlank()) {\n\t\t\t\tvar detected = CommandHandler.detect(clean);\n\t\t\t\tif (detected.isPresent() && detected.get() instanceof ExitCommand)\n\t\t\t\t\tcommands.add(new Command(CommandType.Exit, \"\"));\n\t\t\t\telse if (detected.isPresent() && detected.get() instanceof ResumeCommand rc)\n\t\t\t\t\tcommands.add(new Command(CommandType.Resume, rc.parameter(0)));\n\t\t\t\telse if (detected.isPresent() && detected.get() instanceof AnswerCommand ac) {\n\t\t\t\t\tString id = ac.parameter(0);\n\t\t\t\t\tString action = ac.parameter(1);\n\t\t\t\t\tString reason = ac.parameter(2);\n\t\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id, reason));\n\t\t\t\t\telse\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));\n\t\t\t\t} else if (controlClient.submitEdit(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Modification, \"\"));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Prompt, clean));\n\t\t\t}\n\t\tif (commands.isEmpty())\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn commands;\n\t}"
  reason: "preprocessInput auf CommandHandler.detect umstellen, Verhalten identisch beibehalten"
```
/answer 7fa5-60 allow
Control Request:
```yaml
id: 7fa5-61
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    id: 84X5CG
    old_text: |-
      import xy.ai.workbench.connector.IAIConnector;
      import xy.ai.workbench.connector.claudecode.CCRequest.Command;
    new_text: |-
      import xy.ai.workbench.commands.AnswerCommand;
      import xy.ai.workbench.commands.CommandHandler;
      import xy.ai.workbench.commands.ExitCommand;
      import xy.ai.workbench.connector.IAIConnector;
      import xy.ai.workbench.commands.ResumeCommand;
      import xy.ai.workbench.connector.claudecode.CCRequest.Command;
  reason: Imports für CommandHandler und Command-Subtypen in CCConnector ergänzen
```
/answer 7fa5-61 allow
Control Request:
```yaml
id: 7fa5-62
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    start_marker: "private MCPCommand preprocess(List<String> inputs) {"
    end_marker: "return new MCPCommand(MCPCommandType.Prompt, merged.toString());\n\t}"
    content: "private MCPCommand preprocess(List<String> inputs) {\n\t\tStringBuilder merged = null;\n\t\tfor (String input : inputs) {\n\t\t\tString clean = input != null ? input.strip() : \"\";\n\t\t\tif (clean.isBlank())\n\t\t\t\tcontinue;\n\t\t\tvar detected = CommandHandler.detect(clean);\n\t\t\tif (detected.isPresent() && detected.get() instanceof ExitCommand)\n\t\t\t\treturn new MCPCommand(MCPCommandType.Exit, \"\");\n\t\t\tif (detected.isPresent() && detected.get() instanceof ToolCommand tc)\n\t\t\t\treturn new MCPCommand(MCPCommandType.Tool, tc.parameter(0));\n\t\t\tif (merged == null)\n\t\t\t\tmerged = new StringBuilder();\n\t\t\telse\n\t\t\t\tmerged.append(\"\\n\");\n\t\t\tmerged.append(clean);\n\t\t}\n\t\tif (merged == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn new MCPCommand(MCPCommandType.Prompt, merged.toString());\n\t}"
  reason: "MCPConnector.preprocess auf CommandHandler.detect umstellen, Verhalten identisch beibehalten"
```
/answer 7fa5-62 allow
Control Request:
```yaml
id: 7fa5-63
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    id: sGAUbf
    old_text: |-
      import xy.ai.workbench.connector.IAIConnector;
      import xy.ai.workbench.models.AIAnswer;
    new_text: |-
      import xy.ai.workbench.commands.CommandHandler;
      import xy.ai.workbench.commands.ExitCommand;
      import xy.ai.workbench.connector.IAIConnector;
      import xy.ai.workbench.commands.ToolCommand;
      import xy.ai.workbench.models.AIAnswer;
  reason: Imports für CommandHandler und Command-Subtypen in MCPConnector ergänzen
```
/answer 7fa5-63 allow
Control Request:
```yaml
id: 7fa5-64
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/Command.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AnswerCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ControlRequestCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ResumeCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ExitCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ToolCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CallCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ModificationCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/PromptCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CommandHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  reason: Alle neuen und geänderten Dateien aus Step 03 validieren
```
/answer 7fa5-64 allow
Text:
 Jetzt `CCConnector.preprocessInput` und `MCPConnector.preprocess` auf `CommandHandler.detect` umstellen.
Text:
 Jetzt die Imports in CCConnector ergänzen (CommandHandler, ExitCommand, ResumeCommand, AnswerCommand aus `xy.ai.workbench.connector`).
Text:
 Jetzt MCPConnector.preprocess umstellen.
Text:
 Validierung aller neuen und geänderten Dateien.
Result Stats:  id=081ef136-43e8-419f-af0c-4c39cf0c4b95, total: 0, in: 159, out: 19115, reason: 0, read: 391809, write: 41936
Alle Dateien validieren erfolgreich. Step 03 ist umgesetzt: `Command`-Hierarchie, `CommandHandler`-Registry, und `CCConnector`/`MCPConnector` nutzen `CommandHandler.detect(...)` bei unverändertem Verhalten.
