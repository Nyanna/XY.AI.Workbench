# Step 09 – Cleanup & Gesamtvalidierung (Phase 8)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
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
Via grep MCP-Tool
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

SystemInit:  id=ee2004fd-d872-47dd-91a4-fd36d6aa4491, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 8c1a-9d
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
    pattern: '"/(exit|resume|answer|tool|call)"'
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: "Control Request:"
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: createRequest\(
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: sessionProcessor\.setEnabled
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: ClaudeSessionParameters\.fromConfig
    include:
    - '*.java'
  reason: "Grep-Checkliste aus Step 09 abarbeiten, um verbleibende String-Literale und Alt-Signaturen zu finden."
```
/answer 8c1a-9d allow
Control Request:
```yaml
id: 8c1a-9e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      matches: []
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: EditorInterface.java
        matches:
        - lineno: 53
          match: public static final String CONTROL_REQUEST = "Control Request:";
      - path: connector/ControlRequestCommand.java
        matches:
        - lineno: 5
          match: "/** {@code Control Request:} markiert den folgenden yaml-Block als nicht-modellgeneriert. */"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: connector/mcp/MCPConnector.java
        matches:
        - lineno: 38
          match: "public MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: connector/claudecode/CCConnector.java
        matches:
        - lineno: 53
          match: "public CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: connector/google/GeminiConnector.java
        matches:
        - lineno: 70
          match: "public GeminiRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: connector/IAIConnector.java
        matches:
        - lineno: 14
          match: "REQ createRequest(Prompt prompt, IProgressMonitor mon);"
      - path: connector/claude/ClaudeConnector.java
        matches:
        - lineno: 62
          match: "public ClaudeRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: connector/openai/OpenAIConnector.java
        matches:
        - lineno: 69
          match: "public OpenAIRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: connector/AdaptingConnector.java
        matches:
        - lineno: 167
          match: "public IModelRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
        - lineno: 168
          match: "return getConnector(prompt.config.model).createRequest(prompt, mon);"
      - path: connector/deepseek/DeepSeekConnector.java
        matches:
        - lineno: 112
          match: "public DeepSeekRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
      - path: connector/harness/PromptHandler.java
        matches:
        - lineno: 260
          match: "IModelRequest req = connector.createRequest(prompt, sub);"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches: []
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: view/ClaudeCodeSessionView.java
        matches:
        - lineno: 269
          match: "return ClaudeSessionParameters.fromConfig(cfg, currentProjectPath, currentRelativeFilePath, \"\","
```
/answer 8c1a-9e allow
Control Request:
```yaml
id: 8c1a-9f
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
  reason: Übersicht des Plans lesen für Kontext.
```
/answer 8c1a-9f allow
Control Request:
```yaml
id: 8c1a-a0
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
      nodes:
      - id: 479ae8
        children:
        - id: 479ae8.44d126
          children:
          - id: 479ae8.44d126.gvkfnh|9vMhJA
        - id: 479ae8.f6be5a
        - id: 479ae8.272b68
        - id: 479ae8.e567e7
          children:
          - id: 479ae8.e567e7.lZhqCX|ytf8sA
        - id: 479ae8.67697a
          children:
          - id: 479ae8.67697a.5vcWCD|KpI5RD
          - id: 479ae8.67697a.904b7d
          - id: 479ae8.67697a.5b3be9
          - id: 479ae8.67697a.d2dc72
          - id: 479ae8.67697a.623799
          - id: 479ae8.67697a.4c2f5f
          - id: 479ae8.67697a.62db94
```
/answer 8c1a-a0 allow
ReasoningToken: 27
Control Request:
```yaml
id: 8c1a-a1
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
    ids:
    - 479ae8
  reason: "Vollständigen Überblick über den Plan lesen, um Step 09 im Kontext zu verstehen."
```
/answer 8c1a-a1 allow
Control Request:
```yaml
id: 8c1a-a2
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
          - id: 479ae8.44d126.gvkfnh|9vMhJA
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
          - id: 479ae8.e567e7.lZhqCX|ytf8sA
            type: paragraph
            code: |
              Jede Phase ist einzeln compilierbar. Nach jeder Phase: `ast_validate` der berührten Dateien.
        - id: 479ae8.67697a
          type: section
          children:
          - id: 479ae8.67697a.5vcWCD|KpI5RD
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
/answer 8c1a-a2 allow
Control Request:
```yaml
id: 8c1a-a3
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: class ClaudeCodeSessionView
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: ClaudeCodeSessionView
    include:
    - '*.java'
  reason: "Prüfen, ob ClaudeCodeSessionView noch aktiv verwendet wird (Legacy vs. aktueller Pfad)."
```
/answer 8c1a-a3 allow
Control Request:
```yaml
id: 8c1a-a4
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: view/ClaudeCodeSessionView.java
        matches:
        - lineno: 85
          match: "public class ClaudeCodeSessionView extends ViewPart {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: view/ClaudeCodeSessionView.java
        matches:
        - lineno: 85
          match: "public class ClaudeCodeSessionView extends ViewPart {"
        - lineno: 88
          match: public static final String ID = "xy.ai.workbench.views.ClaudeCodeSessionView";
```
/answer 8c1a-a4 allow
ReasoningToken: 193
Control Request:
```yaml
id: 8c1a-a5
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: ClaudeCodeSessionView
    include:
    - '*.xml'
    - '*.java'
    exclude:
    - view/ClaudeCodeSessionView.java
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: class CommandType|class CCRequest|class Command\b
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp
    pattern: class MCPCommand|class MCPRequest|enum
    include:
    - '*.java'
  reason: "Prüfen, ob ClaudeCodeSessionView noch registriert/aktiv ist und Struktur von CommandType/MCPCommand."
```
/answer 8c1a-a5 allow
Control Request:
```yaml
id: 8c1a-a6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: plugin.xml
        matches:
        - lineno: 56
          match: class="xy.ai.workbench.view.ClaudeCodeSessionView"
        - lineno: 58
          match: id="xy.ai.workbench.view.ClaudeCodeSessionView"
      - path: src/xy/ai/workbench/view/ClaudeCodeSessionView.java
        matches:
        - lineno: 85
          match: "public class ClaudeCodeSessionView extends ViewPart {"
        - lineno: 88
          match: public static final String ID = "xy.ai.workbench.views.ClaudeCodeSessionView";
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      matches:
      - path: CCRequestBuilder.java
        matches:
        - lineno: 6
          match: "public class CCRequestBuilder {"
      - path: CCRequest.java
        matches:
        - lineno: 8
          match: "public class CCRequest implements IModelRequest {"
        - lineno: 29
          match: "public static class Command {"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp
      matches:
      - path: MCPRequest.java
        matches:
        - lineno: 9
          match: "public class MCPRequest implements IModelRequest {"
      - path: MCPCommandType.java
        matches:
        - lineno: 3
          match: "public enum MCPCommandType {"
      - path: MCPCommand.java
        matches:
        - lineno: 5
          match: "public class MCPCommand {"
```
/answer 8c1a-a6 allow
Control Request:
```yaml
id: 8c1a-a7
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
  reason: Aktuelle Struktur der Connector-Verzeichnisse ermitteln für Step 09 Prüfungen.
```
/answer 8c1a-a7 allow
Control Request:
```yaml
id: 8c1a-a8
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      error: Not a regular file.
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
      error: Not a regular file.
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp
      error: Not a regular file.
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
      error: Not a regular file.
```
/answer 8c1a-a8 allow
ReasoningToken: 207
Control Request:
```yaml
id: 8c1a-a9
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
    pattern: ^public (final )?(class|interface|enum)
    include:
    - '*.java'
    limit: 50
  reason: "Liste aller Connector-Dateien/-Klassen ermitteln, um Schritt 09 Prüfungen durchzuführen."
```
/answer 8c1a-a9 allow
Control Request:
```yaml
id: 8c1a-aa
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector
      matches:
      - path: mcp/MCPClient.java
        matches:
        - lineno: 27
          match: "public class MCPClient {"
      - path: mcp/MCPResponse.java
        matches:
        - lineno: 6
          match: "public class MCPResponse implements IModelResponse {"
      - path: mcp/MCPControlClient.java
        matches:
        - lineno: 21
          match: "public class MCPControlClient {"
      - path: mcp/MCPRequest.java
        matches:
        - lineno: 9
          match: "public class MCPRequest implements IModelRequest {"
      - path: mcp/MCPCommandType.java
        matches:
        - lineno: 3
          match: "public enum MCPCommandType {"
      - path: mcp/MCPCommand.java
        matches:
        - lineno: 5
          match: "public class MCPCommand {"
      - path: mcp/MCPConnector.java
        matches:
        - lineno: 20
          match: "public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {"
      - path: claudecode/TimedLineReader.java
        matches:
        - lineno: 10
          match: "public class TimedLineReader {"
      - path: claudecode/HookParser.java
        matches:
        - lineno: 22
          match: "public class HookParser {"
      - path: claudecode/CCSession.java
        matches:
        - lineno: 18
          match: "public class CCSession {"
      - path: claudecode/YamlRenderer.java
        matches:
        - lineno: 15
          match: "public class YamlRenderer {"
      - path: claudecode/SessionState.java
        matches:
        - lineno: 3
          match: "public enum SessionState {"
      - path: claudecode/CCResponse.java
        matches:
        - lineno: 8
          match: "public class CCResponse implements IModelResponse {"
      - path: claudecode/ProtocolParser.java
        matches:
        - lineno: 20
          match: "public class ProtocolParser {"
      - path: claudecode/CCControlClient.java
        matches:
        - lineno: 32
          match: "public class CCControlClient {"
      - path: claudecode/CommandType.java
        matches:
        - lineno: 3
          match: "public enum CommandType {"
      - path: claudecode/CCConnector.java
        matches:
        - lineno: 36
          match: "public class CCConnector implements IAIConnector<CCRequest, CCResponse> {"
      - path: claudecode/CCRequestBuilder.java
        matches:
        - lineno: 6
          match: "public class CCRequestBuilder {"
      - path: claudecode/CCSessionManager.java
        matches:
        - lineno: 10
          match: "public class CCSessionManager {"
      - path: claudecode/ClaudeSessionParameters.java
        matches:
        - lineno: 14
          match: "public class ClaudeSessionParameters extends SessionParameters {"
      - path: claudecode/JsonUtil.java
        matches:
        - lineno: 49
          match: "public final class JsonUtil {"
      - path: claudecode/EditorLocation.java
        matches:
        - lineno: 6
          match: "public class EditorLocation {"
      - path: claudecode/CCRequest.java
        matches:
        - lineno: 8
          match: "public class CCRequest implements IModelRequest {"
      - path: ToolCommand.java
        matches:
        - lineno: 6
          match: "public class ToolCommand extends Command {"
      - path: openapi/deepseek/operators/ElementEnum.java
        matches:
        - lineno: 4
          match: "public enum ElementEnum {"
      - path: openapi/deepseek/operators/AnyOfParameters.java
        matches:
        - lineno: 6
          match: "public final class AnyOfParameters {"
      - path: openapi/deepseek/operators/AnyOfContext.java
        matches:
        - lineno: 6
          match: "public final class AnyOfContext {"
      - path: openapi/deepseek/operators/DirectToolCallCallerParam.java
        matches:
        - lineno: 7
          match: "public final class DirectToolCallCallerParam {"
      - path: openapi/deepseek/operators/PhaseEnum.java
        matches:
        - lineno: 4
          match: "public enum PhaseEnum {"
      - path: openapi/deepseek/operators/ContextEnum.java
        matches:
        - lineno: 4
          match: "public enum ContextEnum {"
      - path: openapi/deepseek/operators/AnyOfCaller.java
        matches:
        - lineno: 6
          match: "public final class AnyOfCaller {"
      - path: openapi/deepseek/operators/OutputList.java
        matches:
        - lineno: 7
          match: "public final class OutputList {"
      - path: openapi/deepseek/operators/AnyOfAllowedCallers.java
        matches:
        - lineno: 6
          match: "public final class AnyOfAllowedCallers {"
      - path: openapi/deepseek/operators/EffortEnum.java
        matches:
        - lineno: 4
          match: "public enum EffortEnum {"
      - path: openapi/deepseek/operators/AnyOfStream.java
        matches:
        - lineno: 6
          match: "public final class AnyOfStream {"
      - path: openapi/deepseek/operators/AnyOfMaxOutputTokens.java
        matches:
        - lineno: 6
          match: "public final class AnyOfMaxOutputTokens {"
      - path: openapi/deepseek/operators/Reasoning.java
        matches:
        - lineno: 7
          match: "public final class Reasoning {"
      - path: openapi/deepseek/operators/AnyOfInstructions.java
        matches:
        - lineno: 6
          match: "public final class AnyOfInstructions {"
      - path: openapi/deepseek/operators/OneOfElement.java
        matches:
        - lineno: 6
          match: "public final class OneOfElement {"
      - path: openapi/deepseek/operators/AnyOfMode.java
        matches:
        - lineno: 6
          match: "public final class AnyOfMode {"
      - path: openapi/deepseek/operators/AllowedCallersList.java
        matches:
        - lineno: 7
          match: "public final class AllowedCallersList {"
      - path: openapi/deepseek/operators/AnyOfPhase.java
        matches:
        - lineno: 6
          match: "public final class AnyOfPhase {"
      - path: openapi/deepseek/operators/AnyOfTemperature.java
        matches:
        - lineno: 6
          match: "public final class AnyOfTemperature {"
      - path: openapi/deepseek/operators/AnyOfEffort.java
        matches:
        - lineno: 6
          match: "public final class AnyOfEffort {"
      - path: openapi/deepseek/operators/AnyOfReasoning.java
        matches:
        - lineno: 6
          match: "public final class AnyOfReasoning {"
      - path: openapi/deepseek/operators/DirectToolCallCaller.java
        matches:
        - lineno: 7
          match: "public final class DirectToolCallCaller {"
      - path: openapi/deepseek/operators/AnyOfElementCaller.java
        matches:
        - lineno: 6
          match: "public final class AnyOfElementCaller {"
      - path: openapi/deepseek/operators/OneOfElementCaller.java
        matches:
        - lineno: 6
          match: "public final class OneOfElementCaller {"
      - path: openapi/deepseek/operators/OneOfOutput.java
        matches:
        - lineno: 6
          match: "public final class OneOfOutput {"
      - path: openapi/deepseek/operators/OneOfCaller.java
        matches:
        - lineno: 6
          match: "public final class OneOfCaller {"
      warning: "Limit of 50 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```
/answer 8c1a-aa allow
Control Request:
```yaml
id: 8c1a-ab
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  reason: "Prüfen, ob ClaudeCodeSessionView Legacy-Code ist, der die Command-Konsolidierung betrifft."
```
/answer 8c1a-ab allow
Control Request:
```yaml
id: 8c1a-ac
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
      nodes:
      - id: RhD3I4|WuzmRz
      - id: mpqFeD|vYNOAP
      - id: TdLTOA|qVUPCt
      - id: gdGpeM|LvU5eq
      - id: e2N8yZ|zzhYon
      - id: ClaudeCodeSessionView
        signature: "public class ClaudeCodeSessionView extends ViewPart {"
        children:
        - id: ClaudeCodeSessionView.OPYzLN|gegS3r
        - id: ClaudeCodeSessionView.q0zV8G|7IaXVh
        - id: ClaudeCodeSessionView.e0oAsC|4lJi1W
        - id: ClaudeCodeSessionView.GEDR3G|dOiQZl
        - id: ClaudeCodeSessionView.createPartControl
          signature: "public void createPartControl(Composite parent) {"
        - id: ClaudeCodeSessionView.dispose
          signature: "public void dispose() {"
        - id: ClaudeCodeSessionView.setFocus
          signature: "public void setFocus() {"
        - id: ClaudeCodeSessionView.makeActions
          signature: "private void makeActions() {"
        - id: ClaudeCodeSessionView.updateCurrentEditor
          signature: "private void updateCurrentEditor() {"
        - id: ClaudeCodeSessionView.currentParameters
          signature: "private ClaudeSessionParameters currentParameters() {"
        - id: ClaudeCodeSessionView.findAssociatedSession
          signature: "private CCSession findAssociatedSession(List<CCSession> sessions) {"
        - id: ClaudeCodeSessionView.selectSession
          signature: "private void selectSession(CCSession session) {"
        - id: ClaudeCodeSessionView.syncSelectionToCurrentFile
          signature: "private void syncSelectionToCurrentFile() {"
        - id: ClaudeCodeSessionView.idLabel
          signature: "private String idLabel(CCSession s) {"
        - id: ClaudeCodeSessionView.stateLabel
          signature: "private String stateLabel(CCSession s) {"
        - id: ClaudeCodeSessionView.formatMMSS
          signature: "private static String formatMMSS(long totalSeconds) {"
        - id: ClaudeCodeSessionView.detailLabel
          signature: "private String detailLabel(CCSession s) {"
        - id: ClaudeCodeSessionView.abbreviateForDisplay
          signature: "private static String abbreviateForDisplay(String s) {"
        - id: ClaudeCodeSessionView.fileNameOf
          signature: "private static String fileNameOf(String path) {"
        - id: ClaudeCodeSessionView.refreshAsync
          signature: "private void refreshAsync() {"
        - id: ClaudeCodeSessionView.refreshTable
          signature: "private void refreshTable() {"
        - id: ClaudeCodeSessionView.createColumn
          signature: "private TableViewerColumn createColumn(String label, int weight) {"
```
/answer 8c1a-ac allow
ReasoningToken: 341
Control Request:
```yaml
id: 8c1a-ad
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    ids: []
  reason: "Struktur von CommandType, CCRequest.Command, MCPCommand/MCPCommandType/MCPRequest lesen für Konsolidierungsprüfung."
```
/answer 8c1a-ad allow
Control Request:
```yaml
id: 8c1a-ae
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
      nodes:
      - id: Q6eIx0|8zE170
      - id: CommandType
        signature: "public enum CommandType {"
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
      nodes:
      - id: VAgrYc|ijdZfc
      - id: CCRequest
        signature: "public class CCRequest implements IModelRequest {"
        children:
        - id: CCRequest.Tvcbr3|MUSzxw
        - id: CCRequest.CCRequest
          signature: "public CCRequest(String id, String title, FrozenConfig config, Command cmd) {"
        - id: CCRequest.getID
          signature: "public String getID() {"
        - id: CCRequest.Command
          signature: "public static class Command {"
          children:
          - id: CCRequest.Command.yQ1tid|e89M8i
          - id: CCRequest.Command.Command
            signature: "public Command(CommandType type, String... parameters) {"
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
      nodes:
      - id: 3zBTmJ|TGrMqF
      - id: MCPCommand
        signature: "public class MCPCommand {"
        children:
        - id: MCPCommand.PJNTO9|QpkmbA
        - id: MCPCommand.MCPCommand
          signature: "public MCPCommand(MCPCommandType type, String... parameters) {"
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
      nodes:
      - id: H2WnQb|QmRDpJ
      - id: MCPCommandType
        signature: "public enum MCPCommandType {"
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
      nodes:
      - id: VAgrYc|svzMxJ
      - id: MCPRequest
        signature: "public class MCPRequest implements IModelRequest {"
        children:
        - id: MCPRequest.LLeok1|41cj72
        - id: MCPRequest.MCPRequest
          signature: "public MCPRequest(String id, String systemPrompt, List<String> tools, MCPComman…"
        - id: MCPRequest.getID
          signature: "public String getID() {"
      errors:
      - "ids [] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 8c1a-ae allow
Control Request:
```yaml
id: 8c1a-af
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/Command.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CommandHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ControlRequestCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ToolCommand.java
  reason: Globale Command-Hierarchie und CommandHandler-API prüfen.
```
/answer 8c1a-af allow
Control Request:
```yaml
id: 8c1a-b0
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/Command.java
      nodes:
      - id: wVPDNP|ljCXHu
      - id: iG2FSg|lJQjQg
      - id: Command
        signature: "public abstract class Command {"
        children:
        - id: Command.ProcessorAction
          signature: "public enum ProcessorAction {"
        - id: Command.iKqF62|lJcluI
        - id: Command.Command
          signature: "protected Command(String... parameters) {"
        - id: Command.prefix
          signature: public abstract String prefix();
        - id: Command.matches
          signature: "public boolean matches(String line) {"
        - id: Command.parse
          signature: public abstract Command parse(String line);
        - id: Command.processorAction
          signature: public abstract ProcessorAction processorAction();
        - id: Command.parameter
          signature: "public String parameter(int i) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CommandHandler.java
      nodes:
      - id: 2nSWaI|ZKpIW3
      - id: sABGWe|Khp7WQ
      - id: CommandHandler
        signature: "public final class CommandHandler {"
        children:
        - id: CommandHandler.mk17mE|1ct30o
        - id: CommandHandler.CommandHandler
          signature: "private CommandHandler() {"
        - id: CommandHandler.detect
          signature: "public static Optional<Command> detect(String line) {"
        - id: CommandHandler.list
          signature: "public static List<Command> list() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ControlRequestCommand.java
      nodes:
      - id: FAE3JL|G624In
      - id: dbiDKq|twDmal
      - id: ControlRequestCommand
        signature: "public class ControlRequestCommand extends Command {"
        children:
        - id: ControlRequestCommand.ControlRequestCommand
          signature: "public ControlRequestCommand() {"
        - id: ControlRequestCommand.prefix
          signature: "public String prefix() {"
        - id: ControlRequestCommand.parse
          signature: "public Command parse(String line) {"
        - id: ControlRequestCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ToolCommand.java
      nodes:
      - id: FAE3JL|G624In
      - id: HB5tpM|AUx9SJ
      - id: ToolCommand
        signature: "public class ToolCommand extends Command {"
        children:
        - id: ToolCommand.ToolCommand
          signature: "public ToolCommand() {"
        - id: ToolCommand.ToolCommand_1
          signature: "public ToolCommand(String toolId) {"
        - id: ToolCommand.prefix
          signature: "public String prefix() {"
        - id: ToolCommand.matches
          signature: "public boolean matches(String line) {"
        - id: ToolCommand.parse
          signature: "public Command parse(String line) {"
        - id: ToolCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
```
/answer 8c1a-b0 allow
ReasoningToken: 394
Control Request:
```yaml
id: 8c1a-b1
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequestBuilder.java
  node_type: MethodDeclaration|method_declaration"
  reason: "Preprocess/CreateRequest-Methoden lesen, um Command-Mapping zu verstehen.\""
```
/answer 8c1a-b1 allow
Control Request:
```yaml
id: 8c1a-b2
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes: []
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequestBuilder.java
      nodes: []
```
/answer 8c1a-b2 allow
ReasoningToken: 368
Control Request:
```yaml
id: 8c1a-b3
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequestBuilder.java
  reason: IDs der Methoden ermitteln für Command-Mapping-Analyse.
```
/answer 8c1a-b3 allow
Control Request:
```yaml
id: 8c1a-b4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: OsZtmS|YZNdWp
      - id: I8gKbt|hARN1l
      - id: uzcHjn|QCw2iw
      - id: CCConnector
        signature: "public class CCConnector implements IAIConnector<CCRequest, CCResponse> {"
        children:
        - id: CCConnector.51iiPA|yBdfkV
        - id: CCConnector.CCConnector
          signature: "public CCConnector(CCSessionManager sessionManager) {"
        - id: CCConnector.getSupportedKeyPattern
          signature: "public KeyPattern getSupportedKeyPattern() {"
        - id: CCConnector.createRequest
          signature: "public CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
        - id: CCConnector.executeRequest
          signature: "public CCResponse executeRequest(CCRequest req, IProgressMonitor mon, Job job) {"
        - id: CCConnector.readUntilResult
          signature: "private CCResponse readUntilResult(CCRequest req, CCSession session, IProgressM…"
        - id: CCConnector.abbreviateForStatus
          signature: "private static String abbreviateForStatus(String s) {"
        - id: CCConnector.preprocessInput
          signature: "private List<Command> preprocessInput(List<String> inputs) {"
        - id: CCConnector.convertResponse
          signature: "public AIAnswer convertResponse(CCResponse resp, IProgressMonitor mon) {"
        - id: CCConnector.getEditorLocation
          signature: "private EditorLocation getEditorLocation() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: 7MAfda|VWDx0H
      - id: uzcHjn|BNLYpl
      - id: MCPConnector
        signature: "public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {"
        children:
        - id: MCPConnector.hOPGQb|xV4uSy
        - id: MCPConnector.MCPConnector
          signature: "public MCPConnector(ConfigManager cfg, MCPClient client) {"
        - id: MCPConnector.getSupportedKeyPattern
          signature: "public KeyPattern getSupportedKeyPattern() {"
        - id: MCPConnector.createRequest
          signature: "public MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {"
        - id: MCPConnector.executeRequest
          signature: "public MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job…"
        - id: MCPConnector.invokeCall
          signature: "private MCPResponse invokeCall(String id, JsonNode call) {"
        - id: MCPConnector.convertResponse
          signature: "public AIAnswer convertResponse(MCPResponse resp, IProgressMonitor mon) {"
        - id: MCPConnector.preprocess
          signature: "private MCPCommand preprocess(List<String> inputs) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequestBuilder.java
      nodes:
      - id: xQlkgT|KjjfaQ
      - id: CCRequestBuilder
        signature: "public class CCRequestBuilder {"
        children:
        - id: CCRequestBuilder.buildPromptJson
          signature: "public String buildPromptJson(String text) {"
```
/answer 8c1a-b4 allow
ReasoningToken: 46
Control Request:
```yaml
id: 8c1a-b5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
    ids:
    - OsZtmS|YZNdWp
    - I8gKbt|hARN1l
    - CCConnector.createRequest
    - CCConnector.preprocessInput
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - 7MAfda|VWDx0H
    - MCPConnector.createRequest
    - MCPConnector.preprocess
  reason: Command-Mapping in CCConnector/MCPConnector konkret lesen für Konsolidierungsentscheidung.
```
/answer 8c1a-b5 allow
Control Request:
```yaml
id: 8c1a-b6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
      nodes:
      - id: OsZtmS|YZNdWp
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
      - id: I8gKbt|hARN1l
        type: imports
        code: |-
          import org.eclipse.ui.IEditorPart;
          import org.eclipse.ui.IFileEditorInput;
          import org.eclipse.ui.IWorkbenchPage;
          import org.eclipse.ui.IWorkbenchWindow;
          import org.eclipse.ui.PlatformUI;

          import xy.ai.workbench.AgentProfile;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model.KeyPattern;
          import xy.ai.workbench.connector.AnswerCommand;
          import xy.ai.workbench.connector.CommandHandler;
          import xy.ai.workbench.connector.ExitCommand;
          import xy.ai.workbench.connector.IAIConnector;
      - id: CCConnector.createRequest
        type: method_declaration
        code: "@Override\n\tpublic CCRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Create request\", 2);\n\t\tString id = UUID.randomUUID().toString();\n\n\t\t// Preprocessing: extract Allow/Deny/exit/resume lines\n\t\tString title = null;\n\t\tCommand command = null;\n\t\tStringBuilder merged = null;\n\n\t\t{ // Preprocess\n\t\t\tsub.subTask(\"Preprocess input\");\n\t\t\tfor (Command cmd : preprocessInput(prompt.inputs))\n\t\t\t\tif (CommandType.Prompt.equals(cmd.type)) {\n\t\t\t\t\tif (title == null)\n\t\t\t\t\t\ttitle = cmd.parameter.substring(0, Math.min(100, cmd.parameter.length())).replace('\\n', ' ');\n\n\t\t\t\t\tif (merged != null)\n\t\t\t\t\t\tmerged.append(\"\\n\");\n\t\t\t\t\telse\n\t\t\t\t\t\tmerged = new StringBuilder();\n\t\t\t\t\tmerged.append(cmd.parameter);\n\t\t\t\t} else {\n\t\t\t\t\tcommand = cmd;\n\t\t\t\t\tbreak;\n\t\t\t\t}\n\t\t\tsub.worked(1);\n\t\t}\n\n\t\tif (command == null && merged != null) {\n\t\t\tsub.subTask(\"Build prompt\");\n\t\t\tcommand = new Command(CommandType.Prompt, requestBuilder.buildPromptJson(merged.toString().trim()));\n\t\t\tsub.worked(1);\n\t\t}\n\n\t\treturn new CCRequest(id, title, prompt.config, command);\n\t}"
      - id: CCConnector.preprocessInput
        type: method_declaration
        code: "private List<Command> preprocessInput(List<String> inputs) {\n\t\tList<Command> commands = new ArrayList<Command>();\n\t\tString clean;\n\t\tfor (String input : inputs)\n\t\t\tif (!(clean = input != null ? input.strip() : \"\").isBlank()) {\n\t\t\t\tvar detected = CommandHandler.detect(clean);\n\t\t\t\tif (detected.isPresent() && detected.get() instanceof ExitCommand)\n\t\t\t\t\tcommands.add(new Command(CommandType.Exit, \"\"));\n\t\t\t\telse if (detected.isPresent() && detected.get() instanceof ResumeCommand rc)\n\t\t\t\t\tcommands.add(new Command(CommandType.Resume, rc.parameter(0)));\n\t\t\t\telse if (detected.isPresent() && detected.get() instanceof AnswerCommand ac) {\n\t\t\t\t\tString id = ac.parameter(0);\n\t\t\t\t\tString action = ac.parameter(1);\n\t\t\t\t\tString reason = ac.parameter(2);\n\t\t\t\t\tif (\"allow\".equals(action))\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Allow, id, reason));\n\t\t\t\t\telse\n\t\t\t\t\t\tcommands.add(new Command(CommandType.Deny, id, reason));\n\t\t\t\t} else if (controlClient.submitEdit(clean))\n\t\t\t\t\tcommands.add(new Command(CommandType.Modification, \"\"));\n\t\t\t\telse\n\t\t\t\t\tcommands.add(new Command(CommandType.Prompt, clean));\n\t\t\t}\n\t\tif (commands.isEmpty())\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn commands;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: 7MAfda|VWDx0H
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
          import xy.ai.workbench.connector.CommandHandler;
          import xy.ai.workbench.connector.ExitCommand;
          import xy.ai.workbench.connector.IAIConnector;
          import xy.ai.workbench.connector.ToolCommand;
      - id: MCPConnector.createRequest
        type: method_declaration
        code: "@Override\n\tpublic MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tMCPCommand cmd = prompt.command == Prompt.Command.CALL ? new MCPCommand(MCPCommandType.Call, prompt.yamlBlock)\n\t\t\t\t: preprocess(prompt.inputs);\n\t\treturn new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools, cmd);\n\t}"
      - id: MCPConnector.preprocess
        type: method_declaration
        code: "private MCPCommand preprocess(List<String> inputs) {\n\t\tStringBuilder merged = null;\n\t\tfor (String input : inputs) {\n\t\t\tString clean = input != null ? input.strip() : \"\";\n\t\t\tif (clean.isBlank())\n\t\t\t\tcontinue;\n\t\t\tvar detected = CommandHandler.detect(clean);\n\t\t\tif (detected.isPresent() && detected.get() instanceof ExitCommand)\n\t\t\t\treturn new MCPCommand(MCPCommandType.Exit, \"\");\n\t\t\tif (detected.isPresent() && detected.get() instanceof ToolCommand tc)\n\t\t\t\treturn new MCPCommand(MCPCommandType.Tool, tc.parameter(0));\n\t\t\tif (merged == null)\n\t\t\t\tmerged = new StringBuilder();\n\t\t\telse\n\t\t\t\tmerged.append(\"\\n\");\n\t\t\tmerged.append(clean);\n\t\t}\n\t\tif (merged == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn new MCPCommand(MCPCommandType.Prompt, merged.toString());\n\t}"
```
/answer 8c1a-b6 allow
ReasoningToken: 115
Control Request:
```yaml
id: 8c1a-b7
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
  reason: AISessionManager auf tote Prompt-Logik nach Auslagerung in PromptHandler prüfen.
```
/answer 8c1a-b7 allow
Control Request:
```yaml
id: 8c1a-b8
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
      nodes:
      - id: nl6M7j|D4KtJb
      - id: AISessionManager
        signature: "public class AISessionManager {"
        children:
        - id: AISessionManager.ZSo9oa|BAifGp
        - id: AISessionManager.AISessionManager
          signature: "public AISessionManager(ConfigManager cfg, AdaptingConnector connector, MCPClie…"
        - id: AISessionManager.discoverTools
          signature: "private void discoverTools(Model model) {"
        - id: AISessionManager.sortByToolOrder
          signature: "private String[] sortByToolOrder(Collection<String> names) {"
        - id: AISessionManager.clearObserver
          signature: "public void clearObserver() {"
        - id: AISessionManager.addInputStatObs
          signature: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {"
        - id: AISessionManager.addAnswerObs
          signature: "public void addAnswerObs(Consumer<AIAnswer> obs) {"
        - id: AISessionManager.updateInputStat
          signature: "public void updateInputStat(InputMode mode) {"
        - id: AISessionManager.initializeInputs
          signature: "public void initializeInputs() {"
        - id: AISessionManager.removeCommentLines
          signature: "public String removeCommentLines(String input) {"
        - id: AISessionManager.execute
          signature: "public void execute(Display display) {"
        - id: AISessionManager.queueAsync
          signature: "public void queueAsync(Display display, AIBatchManager batch) {"
        - id: AISessionManager.queueAndSubmit
          signature: "public void queueAndSubmit(Display display, AIBatchManager batch) {"
```
/answer 8c1a-b8 allow
Control Request:
```yaml
id: 8c1a-b9
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: setEnabled
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: processorEnabled
    include:
    - '*.java'
  reason: Alle setEnabled-Aufrufe und processorEnabled-Nutzung prüfen.
```
/answer 8c1a-b9 allow
Control Request:
```yaml
id: 8c1a-ba
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: view/AISessionView.java
        matches:
        - lineno: 243
          match: budget.setEnabled(enabled);
        - lineno: 396
          match: SelectionListener.widgetSelectedAdapter(e -> cfg.setEnabledTools(toolsList.getSelection())));
        - lineno: 444
          match: "cfg.setEnabledTools(new String[0]);"
        - lineno: 461
          match: cfg.setEnabledTools(preset.tools);
        - lineno: 562
          match: bbtn.setEnabled(m.cap.isSupportBatch());
        - lineno: 563
          match: bsbtn.setEnabled(m.cap.isSupportBatch());
        - lineno: 626
          match: label.setEnabled(enabled);
        - lineno: 628
          match: ctrl.setEnabled(enabled);
      - path: ConfigManager.java
        matches:
        - lineno: 94
          match: setEnabledProfiles(getCapabilities().getAgentProfiles());
        - lineno: 143
          match: "public void setEnabledTools(String[] enabledTools) {"
        - lineno: 338
          match: "public void setEnabledProfiles(AgentProfile[] enabledProfiles) {"
      - path: connector/harness/SessionProcessor.java
        matches:
        - lineno: 74
          match: "public void setEnabled(boolean enabled) {"
        - lineno: 97
          match: "/** Legacy overload, uses the shared {@link #setEnabled(boolean)} state; prefer the explicit-flag overload. */"
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: connector/google/GeminiConnector.java
        matches:
        - lineno: 114
          match: "sessionProcessor.process(prompt.inputs, prompt.processorEnabled, cb);"
      - path: connector/claude/ClaudeConnector.java
        matches:
        - lineno: 90
          match: "sessionProcessor.process(prompt.inputs, prompt.processorEnabled, cb);"
      - path: connector/openai/OpenAIConnector.java
        matches:
        - lineno: 104
          match: "sessionProcessor.process(prompt.inputs, prompt.processorEnabled, cb);"
      - path: connector/deepseek/DeepSeekConnector.java
        matches:
        - lineno: 153
          match: "for (ObjectNode item : sessionProcessor.process(prompt.inputs, prompt.processorEnabled,"
      - path: connector/harness/PromptHandler.java
        matches:
        - lineno: 296
          match: .processorEnabled(cfg.isInputEnabled(InputMode.Converter))//
      - path: connector/harness/Prompt.java
        matches:
        - lineno: 39
          match: public final boolean processorEnabled;
        - lineno: 48
          match: "private Prompt(List<String> inputs, InputKind kind, boolean batch, FrozenConfig config, boolean processorEnabled,"
        - lineno: 58
          match: this.processorEnabled = processorEnabled;
        - lineno: 114
          match: private boolean processorEnabled;
        - lineno: 142
          match: "public Builder processorEnabled(boolean processorEnabled) {"
        - lineno: 143
          match: this.processorEnabled = processorEnabled;
        - lineno: 163
          match: "return new Prompt(inputs, kind, batch, config, processorEnabled, absoluteFilePath, command, yamlBlock);"
```
/answer 8c1a-ba allow
ReasoningToken: 547
Control Request:
```yaml
id: 8c1a-bb
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
    ids:
    - AISessionManager.execute
    - AISessionManager.queueAsync
    - AISessionManager.queueAndSubmit
    - AISessionManager.removeCommentLines
    - AISessionManager.ZSo9oa|BAifGp
  reason: "Prüfen, ob AISessionManager noch tote Prompt-Logik nach Auslagerung in PromptHandler enthält."
```
/answer 8c1a-bb allow
Control Request:
```yaml
id: 8c1a-bc
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
      nodes:
      - id: AISessionManager.execute
        type: method_declaration
        code: "public void execute(Display display) {\n\t\tpromptHandler.execute(display);\n\t}"
      - id: AISessionManager.queueAsync
        type: method_declaration
        code: "public void queueAsync(Display display, AIBatchManager batch) {\n\t\tpromptHandler.queueAsync(display, batch);\n\t}"
      - id: AISessionManager.queueAndSubmit
        type: method_declaration
        code: "public void queueAndSubmit(Display display, AIBatchManager batch) {\n\t\tpromptHandler.queueAndSubmit(display, batch);\n\t}"
      - id: AISessionManager.removeCommentLines
        type: method_declaration
        code: "public String removeCommentLines(String input) {\n\t\treturn promptHandler.removeCommentLines(input);\n\t}"
      - id: AISessionManager.ZSo9oa|BAifGp
        type: statements
        code: "public static final String CONTEXT_PROMPT_TXT = \"context.prompt.txt\";\n\n\tprivate ActiveEditorListener editorListener = new ActiveEditorListener(this);\n\tprivate IncludeAdapter includeAdapter;\n\n\tprivate final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\tprivate final MCPClient mcpClient;\n\tprivate final SessionProcessor sessionProcessor;\n\tpublic final EditorInterface editIfc;\n\tprivate final PromptHandler promptHandler;"
```
/answer 8c1a-bc allow
Control Request:
```yaml
id: 8c1a-bd
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
    ids:
    - CommandHandler.mk17mE|1ct30o
  reason: Kontext von setEnabled Legacy-Overload lesen.
```
/answer 8c1a-bd allow
Control Request:
```yaml
id: 8c1a-be
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
      nodes: []
      errors:
      - No node matched 'CommandHandler.mk17mE|1ct30o'.
```
/answer 8c1a-be allow
ReasoningToken: 347
Text:
 AISessionManager delegiert bereits vollständig an `PromptHandler`; keine tote Prompt-Logik. Prüfe noch SessionProcessor-Legacy-Overload direkt per lineno.
Control Request:
```yaml
id: 8c1a-bf
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
    pattern: public void setEnabled|Legacy overload|public.*process\(
    include:
    - SessionProcessor.java
  reason: Kontext des Legacy-Overloads von setEnabled/process in SessionProcessor lesen.
```
/answer 8c1a-bf allow
Control Request:
```yaml
id: 8c1a-c0
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness
      matches:
      - path: SessionProcessor.java
        matches:
        - lineno: 74
          match: "public void setEnabled(boolean enabled) {"
        - lineno: 80
          match: "public <M> List<M> process(List<String> inputs, boolean enabled, SessionCallbacks<M> callbacks) {"
        - lineno: 97
          match: "/** Legacy overload, uses the shared {@link #setEnabled(boolean)} state; prefer the explicit-flag overload. */"
        - lineno: 98
          match: "public <M> List<M> process(List<String> inputs, SessionCallbacks<M> callbacks) {"
        - lineno: 103
          match: "public <M> List<M> process(String input, Path rootPath, SessionCallbacks<M> callbacks) {"
        - lineno: 115
          match: "public <M> List<M> process(String input, SessionCallbacks<M> callbacks) {"
```
/answer 8c1a-c0 allow
Control Request:
```yaml
id: 8c1a-c1
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: \.setEnabled\(
    include:
    - '*.java'
  reason: Alle Aufrufer von setEnabled im gesamten Baum finden (nicht nur sessionProcessor.-Präfix).
```
/answer 8c1a-c1 allow
Control Request:
```yaml
id: 8c1a-c2
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
      matches:
      - path: view/AISessionView.java
        matches:
        - lineno: 243
          match: budget.setEnabled(enabled);
        - lineno: 562
          match: bbtn.setEnabled(m.cap.isSupportBatch());
        - lineno: 563
          match: bsbtn.setEnabled(m.cap.isSupportBatch());
        - lineno: 626
          match: label.setEnabled(enabled);
        - lineno: 628
          match: ctrl.setEnabled(enabled);
```
/answer 8c1a-c2 allow
ReasoningToken: 209
Control Request:
```yaml
id: 8c1a-c3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
    ids:
    - Q6eIx0|8zE170
    - CommandType
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    ids:
    - H2WnQb|QmRDpJ
    - MCPCommandType
  reason: "Vollständigen Inhalt von CommandType/MCPCommandType lesen, um vorhandene Dokumentation zu prüfen."
```
/answer 8c1a-c3 allow
Control Request:
```yaml
id: 8c1a-c4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
      nodes:
      - id: Q6eIx0|8zE170
        type: imports
        code: package xy.ai.workbench.connector.claudecode;
      - id: CommandType
        type: enum_declaration
        code: "public enum CommandType {\n\tPrompt, Exit, Resume, Allow, Deny, Modification;\n}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
      nodes:
      - id: H2WnQb|QmRDpJ
        type: imports
        code: package xy.ai.workbench.connector.mcp;
      - id: MCPCommandType
        type: enum_declaration
        code: "public enum MCPCommandType {\n\tPrompt, Tool, Exit, Call;\n}"
```
/answer 8c1a-c4 allow
ReasoningToken: 228
Control Request:
```yaml
id: 8c1a-c5
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
    start_marker: package xy.ai.workbench.connector.claudecode;
    end_marker: package xy.ai.workbench.connector.claudecode;
    content: |-
      package xy.ai.workbench.connector.claudecode;

      /**
       * Protokoll-interner Command-Typ für den Claude-Code-Kontrollkanal (Wire-Payload in
       * {@link CCRequest.Command}). Bewusst getrennt von der globalen {@code connector.Command}-
       * Hierarchie: {@link xy.ai.workbench.connector.CommandHandler#detect(String)} liefert generische
       * Zeilen-Kommandos, die in {@code CCConnector.preprocessInput} auf diese protokollspezifischen
       * Typen abgebildet werden (u.a. Allow/Deny-Unterscheidung, gemergter Prompt-Text).
  reason: "Mapping-Schicht zwischen globaler Command-Hierarchie und CommandType dokumentieren (Task 2, Nicht-Konsolidierung begründen)."
```
/answer 8c1a-c5 allow
Control Request:
```yaml
id: 8c1a-c6
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
      error: End marker must start after start marker ends.
      reason: marker_order
```
/answer 8c1a-c6 allow
Control Request:
```yaml
id: 8c1a-c7
toolName: ast_edit_marks
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    start_marker: package xy.ai.workbench.connector.mcp;
    end_marker: package xy.ai.workbench.connector.mcp;
    content: |-
      package xy.ai.workbench.connector.mcp;

      /**
       * Protokoll-interner Command-Typ für den MCP-Kontrollkanal (Wire-Payload in {@link MCPCommand}).
       * Bewusst getrennt von der globalen {@code connector.Command}-Hierarchie:
       * {@link xy.ai.workbench.connector.CommandHandler#detect(String)} liefert generische
       * Zeilen-Kommandos, die in {@code MCPConnector.preprocess} auf diese protokollspezifischen
       * Typen abgebildet werden (u.a. Call mit Yaml-Block, gemergter Prompt-Text).
  reason: Mapping-Schicht zwischen globaler Command-Hierarchie und MCPCommandType dokumentieren.
```
/answer 8c1a-c7 allow
Control Request:
```yaml
id: 8c1a-c8
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
      error: End marker must start after start marker ends.
      reason: marker_order
```
/answer 8c1a-c8 allow
Control Request:
```yaml
id: 8c1a-c9
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
    id: CommandType
    position: before
    source: |-
      /**
       * Protokoll-interner Command-Typ fuer den Claude-Code-Kontrollkanal (Wire-Payload in
       * {@link CCRequest.Command}). Bewusst getrennt von der globalen {@code connector.Command}-
       * Hierarchie: {@link xy.ai.workbench.connector.CommandHandler#detect(String)} liefert generische
       * Zeilen-Kommandos, die in {@code CCConnector.preprocessInput} auf diese protokollspezifischen
       * Typen abgebildet werden (u.a. Allow/Deny-Unterscheidung, gemergter Prompt-Text).
       */
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    id: MCPCommandType
    position: before
    source: |-
      /**
       * Protokoll-interner Command-Typ fuer den MCP-Kontrollkanal (Wire-Payload in {@link MCPCommand}).
       * Bewusst getrennt von der globalen {@code connector.Command}-Hierarchie:
       * {@link xy.ai.workbench.connector.CommandHandler#detect(String)} liefert generische
       * Zeilen-Kommandos, die in {@code MCPConnector.preprocess} auf diese protokollspezifischen
       * Typen abgebildet werden (u.a. Call mit Yaml-Block, gemergter Prompt-Text).
       */
  reason: "Mapping-Schicht (Task 2) als Javadoc dokumentieren, da Konsolidierung auf globale Command-Hierarchie wegen protokollspezifischer Semantik nicht sinnvoll ist."
```
/answer 8c1a-c9 allow
ReasoningToken: 231
Text:
 Marker-Overlap; nutze stattdessen `ast_insert` vor dem Enum-Knoten.
Control Request:
```yaml
id: 8c1a-ca
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/IAIConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AdaptingConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/Command.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CommandHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/SessionParameters.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/FrozenConfig.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCRequest.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CommandType.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ClaudeSessionParameters.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claude/ClaudeConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/deepseek/DeepSeekConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/google/GeminiConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/openai/OpenAIConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java
  reason: Finale Gesamtvalidierung aller in Step 09 gelisteten Dateien nach Cleanup.
```
/answer 8c1a-ca allow
Text:
 Jetzt die abschließende Gesamtvalidierung.
Result Stats:  id=ee2004fd-d872-47dd-91a4-fd36d6aa4491, total: 0, in: 165, out: 12338, reason: 0, read: 423237, write: 42361
Alle Dateien validieren erfolgreich.

**Ergebnis Step 09:**
1. Grep-Checkliste: `/exit|/resume|/answer|/tool|/call` und `Control Request:` als Literale im `connector`-Baum – **sauber** (bereits über `EditorInterface.CONTROL_REQUEST` bzw. `CommandHandler`-Klassen abstrahiert). `createRequest(Prompt, IProgressMonitor)` ist die aktuelle, einheitliche Signatur (keine Alt-Form mehr).
2. `CommandType`/`MCPCommandType` und `CCRequest.Command`/`MCPCommand` bleiben bestehen (keine Konsolidierung), da sie protokollspezifische Semantik tragen (Allow/Deny, Call+Yaml, gemergter Prompt-Text), die die generische `Command`-Hierarchie nicht abbildet. Mapping-Schicht (`CommandHandler.detect` → `CCConnector.preprocessInput`/`MCPConnector.preprocess`) jetzt per Javadoc dokumentiert.
3. `AISessionManager` delegiert `execute`/`queueAsync`/`queueAndSubmit`/`removeCommentLines` vollständig an `PromptHandler` – keine tote Prompt-Logik.
4. `sessionProcessor.setEnabled` hat außer der eigenen (dokumentiert legacy) Definition keine Aufrufer; alle Connectoren nutzen `process(inputs, prompt.processorEnabled, cb)`.
5. Invarianten (Symmetrie via generischer `Command`-Hierarchie + Mapping, Determinismus, Prefix-Caching) unverändert erhalten, keine Code-Änderungen an Blockreihenfolge/Whitespace nötig.

Finale `ast_validate` über alle in Step 09 gelisteten Dateien: **all_ok: true**.
