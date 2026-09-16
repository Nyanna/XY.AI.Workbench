# Step 08 – `/call`-Kommando (Phase 7)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
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

SystemInit:  id=9a02c09e-6eff-4620-9b53-430d9b29fbc7, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 8c1a-85
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
  reason: Overview lesen für Kontext des Gesamtplans
  regexp: .*
```
/answer 8c1a-85 allow
Control Request:
```yaml
id: 8c1a-86
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md
      nodes:
      - id: 479ae8
        children:
        - id: 479ae8.44d126
          children:
          - id: 479ae8.44d126.gvkfnh|9vMhJA
            code: |
              Zentrale Vereinheitlichung von Nutzung, Behandlung und Implementierung der Kommandos, die im
              Editor angegeben/ausgeführt werden. Der Session Processor erhält eine zentrale Anlaufstelle,
              um Kommandos von normalem Text zu unterscheiden. Ein globaler Command Handler stellt die
              Infrastruktur bereit; die prompt-spezifische Logik wird in einen Prompt-Handler ausgelagert;
              die Session-Parameter werden in eine generische Basisklasse extrahiert; alle Connectoren
              werden auf ein einheitliches Prompt-Objekt umgestellt.
        - id: 479ae8.f6be5a
          code: |+
            ## Invarianten (müssen erhalten bleiben)
            - **Symmetrie / Austauschbarkeit:** Ein-/Ausgaben der Komponenten müssen zwischen Connectoren
              austauschbar bleiben. Tool-Call-/Result-Blöcke, die von MCP oder Claude erzeugt wurden,
              müssen auch durch einen anderen Connector geleitet werden können.
            - **Determinismus:** Gleiche Eingabe ⇒ gleiche Ausgabe.
            - **Prefix-Caching:** Umformungen dürfen das Prefix-Caching nicht unterbrechen.
            - **Kein permanenter Testsuite-Lauf.** Validierung erfolgt via `ast_validate` je berührter Datei.

        - id: 479ae8.272b68
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
          children:
          - id: 479ae8.e567e7.lZhqCX|ytf8sA
            code: |
              Jede Phase ist einzeln compilierbar. Nach jeder Phase: `ast_validate` der berührten Dateien.
        - id: 479ae8.67697a
          children:
          - id: 479ae8.67697a.5vcWCD|KpI5RD
            code: |
              Alle Pfade absolut unter `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/`.
          - id: 479ae8.67697a.904b7d
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
            code: |+
              ### Connector-Infrastruktur (`connector/`)
              - `IAIConnector.java` – **Node `IAIConnector`**; Methoden `.createRequest`, `.executeRequest`,
                `.convertResponse`, `.getSupportedKeyPattern`. Aktuelle Signatur:
                `REQ createRequest(List<String> inputs, String systemPrompt, List<String> tools, boolean batchFix, IProgressMonitor mon);`
              - `AdaptingConnector.java` – `.createRequest` delegiert an `getConnector(cfg.getModel())`;
                `.executeRequest` delegiert an `getConnector(request)`.

          - id: 479ae8.67697a.d2dc72
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
            code: |+
              ### API-Connectoren
              - `claude/ClaudeConnector.java` – `.ClaudeConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`, `.appendTools`, `.executeRequest`, `.convertResponse`.
              - `deepseek/DeepSeekConnector.java` – `.DeepSeekConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`, innere `SessionRequestCallbacks`.
              - `google/GeminiConnector.java` – `.GeminiConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`, `.getThinkingBudget`.
              - `openai/OpenAIConnector.java` – `.OpenAIConnector(cfg, mcpClient, sessionProcessor)`, `.createRequest`.

          - id: 479ae8.67697a.4c2f5f
            code: |+
              ### Harness (`connector/harness/`)
              - `SessionProcessor.java` – `.process(List<String>,cb)`, `.process(String,cb)`, `.setEnabled`, innere `Run`:
                `.run`, `.flush`, `.isMarkerLine`, `.consumeReasoning`, `.consumeText`, `.consumeToolCall`,
                `.consumeToolResult`, `.fenceRange`, `.readYaml`, `.include*`.
              - `SessionCallbacks.java`, `SessionRenderer.java`, `SessionAnswerBuilder.java` (Tool-Call-Rendering).

          - id: 479ae8.67697a.62db94
            code: |
              ### MCP (`connector/mcp/`)
              - `MCPConnector.java` – `.preprocess` (`/exit`,`/tool`), `.createRequest`, `.executeRequest`.
              - `MCPControlClient.java` – `.renderSchema` (liefert `\`\`\`yaml … \`\`\``), `.extractYamlBlock`,
                `.parseYaml`, `.fillReason`, `.prettyResult`.
              - `MCPClient.java` – `.findTool`, `.callTool`, `.renderToolCall`.
              - `MCPRequest.java` – Konstruktor `MCPRequest(String id, String systemPrompt, List<String> tools, MCPCommand cmd)`.
              - `MCPCommand.java`, `MCPCommandType.java` – `enum { Exit, Tool, Prompt }` (+ ggf. weitere).
```
/answer 8c1a-86 allow
Control Request:
```yaml
id: 8c1a-87
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class MCPControlClient
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class MCPConnector
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class PromptHandler
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class SessionProcessor
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class EditorInterface
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class CallCommand
    include:
    - '*.java'
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: class Prompt\b
    include:
    - '*.java'
  reason: Relevante Dateipfade finden
```
/answer 8c1a-87 allow
Control Request:
```yaml
id: 8c1a-88
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/connector/mcp/MCPControlClient.java
        matches:
        - lineno: 20
          match: "public class MCPControlClient {"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/connector/mcp/MCPConnector.java
        matches:
        - lineno: 20
          match: "public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/connector/harness/PromptHandler.java
        matches:
        - lineno: 50
          match: "public final class PromptHandler {"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/connector/harness/SessionProcessor.java
        matches:
        - lineno: 47
          match: "public final class SessionProcessor {"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/EditorInterface.java
        matches:
        - lineno: 37
          match: "public class EditorInterface {"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/connector/CallCommand.java
        matches:
        - lineno: 6
          match: "public class CallCommand extends Command {"
    - directory:
      - /home/user/xyan/xy.ai.workbench
      matches:
      - path: src/xy/ai/workbench/connector/harness/Prompt.java
        matches:
        - lineno: 17
          match: "public final class Prompt {"
```
/answer 8c1a-88 allow
ReasoningToken: 44
Control Request:
```yaml
id: 8c1a-89
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CallCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
  reason: Volles Bild der relevanten Klassen für Step 08
```
/answer 8c1a-89 allow
Control Request:
```yaml
id: 8c1a-8a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
      nodes:
      - id: fXSZh1|rhl4eY
      - id: txfzo4|udCxJD
      - id: MCPControlClient
        signature: "public class MCPControlClient {"
        children:
        - id: MCPControlClient.JloL0D|yKZBpv
        - id: MCPControlClient.renderSchema
          signature: "public String renderSchema(JsonNode tool) {"
        - id: MCPControlClient.dKZb74|tHZasV
        - id: MCPControlClient.extractYamlBlock
          signature: "public String extractYamlBlock(String text) {"
        - id: MCPControlClient.parseYaml
          signature: "public JsonNode parseYaml(String block) {"
        - id: MCPControlClient.prettyResult
          signature: "public String prettyResult(JsonNode result) {"
        - id: MCPControlClient.khCRJ6|sVRL8A
        - id: MCPControlClient.extractText
          signature: "public String extractText(JsonNode content) {"
        - id: MCPControlClient.82vyIy|L43zbT
        - id: MCPControlClient.fillReason
          signature: "public JsonNode fillReason(JsonNode tool, JsonNode arguments) {"
        - id: MCPControlClient.appendProperty
          signature: "private void appendProperty(StringBuilder sb, String key, JsonNode prop, boolea…"
        - id: MCPControlClient.renderValue
          signature: "private String renderValue(JsonNode prop, String type, String indent) {"
        - id: MCPControlClient.Mwqdao|cn5YCc
        - id: MCPControlClient.renderArrayValue
          signature: "private String renderArrayValue(JsonNode prop, String indent) {"
        - id: MCPControlClient.requiredSet
          signature: "private Set<String> requiredSet(JsonNode schema) {"
        - id: MCPControlClient.appendComment
          signature: "private void appendComment(StringBuilder sb, String indent, String text) {"
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
        - id: MCPConnector.convertResponse
          signature: "public AIAnswer convertResponse(MCPResponse resp, IProgressMonitor mon) {"
        - id: MCPConnector.preprocess
          signature: "private MCPCommand preprocess(List<String> inputs) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: jyVQdv|ey0ixs
      - id: bXTkJR|w6qdUr
      - id: sEQQ8c|1Hkjcz
      - id: KEc50E|BnEeaS
      - id: fPHG2Y|QliQBw
      - id: PromptHandler
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.1gKcmV|Beh0X2
        - id: PromptHandler.LSyBuh|FgyeYv
        - id: PromptHandler.PromptHandler
          signature: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, SessionPro…"
        - id: PromptHandler.clearObserver
          signature: "public void clearObserver() {"
        - id: PromptHandler.addInputStatObs
          signature: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {"
        - id: PromptHandler.addAnswerObs
          signature: "public void addAnswerObs(Consumer<AIAnswer> obs) {"
        - id: PromptHandler.updateInputStat
          signature: "public void updateInputStat(InputMode mode) {"
        - id: PromptHandler.initializeInputs
          signature: "public void initializeInputs() {"
        - id: PromptHandler.getInput
          signature: "private String getInput(InputMode mode) {"
        - id: PromptHandler.removeCommentLines
          signature: "public String removeCommentLines(String input) {"
        - id: PromptHandler.execute
          signature: "public void execute(Display display) {"
        - id: PromptHandler.PromptJob
          signature: "private class PromptJob extends Job {"
          children:
          - id: PromptHandler.PromptJob.zOXTQk|KFkQPW
          - id: PromptHandler.PromptJob.PromptJob
            signature: "private PromptJob(String name, Display display) {"
          - id: PromptHandler.PromptJob.run
            signature: "protected IStatus run(IProgressMonitor mon) {"
        - id: PromptHandler.queueAsync
          signature: "public void queueAsync(Display display, AIBatchManager batch) {"
        - id: PromptHandler.queueSync
          signature: "private void queueSync(Display display, AIBatchManager batch, IProgressMonitor …"
        - id: PromptHandler.queueAndSubmit
          signature: "public void queueAndSubmit(Display display, AIBatchManager batch) {"
        - id: PromptHandler.prepareRequest
          signature: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgre…"
        - id: PromptHandler.maceLN|sqs7wQ
        - id: PromptHandler.buildPrompt
          signature: "Prompt buildPrompt(Display display, boolean batch) {"
        - id: PromptHandler.ohnvjH|nhSFvk
        - id: PromptHandler.detectSelection
          signature: "private Prompt.InputKind detectSelection(ITextEditor textEditor, List<String> i…"
        - id: PromptHandler.Txg3oX|3Xw56U
        - id: PromptHandler.detectFullFile
          signature: "private Prompt.InputKind detectFullFile(ITextEditor textEditor, List<String> in…"
        - id: PromptHandler.applyDetected
          signature: "private boolean applyDetected(Optional<Command> cmd, IDocument doc, int lineInd…"
        - id: PromptHandler.lineText
          signature: "private String lineText(IDocument doc, int lineIndex) {"
        - id: PromptHandler.Q5D2ne|VNsv72
        - id: PromptHandler.captureYamlBlock
          signature: "private String captureYamlBlock(IDocument doc, int lineIndex) {"
        - id: PromptHandler.resolveAbsoluteFilePath
          signature: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {"
        - id: PromptHandler.executeInner
          signature: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMoni…"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
      nodes:
      - id: hNG9ma|BxBk17
      - id: w6eseZ|oXrUSh
      - id: fJudos|IVSBB9
      - id: SessionProcessor
        signature: "public final class SessionProcessor {"
        children:
        - id: SessionProcessor.9qssKV|49o8fN
        - id: SessionProcessor.SessionProcessor
          signature: "public SessionProcessor() {"
        - id: SessionProcessor.SessionProcessor_1
          signature: "public SessionProcessor(IIncludeAdapter adapter) {"
        - id: SessionProcessor.mwuWxm|F0JmSu
        - id: SessionProcessor.setAdapter
          signature: "public void setAdapter(IIncludeAdapter adapter) {"
        - id: SessionProcessor.OYD2gH|H8L24o
        - id: SessionProcessor.setEnabled
          signature: "public void setEnabled(boolean enabled) {"
        - id: SessionProcessor.GkXvQx|QWFiaE
        - id: SessionProcessor.process
          signature: "public <M> List<M> process(List<String> inputs, boolean enabled, SessionCallbac…"
        - id: SessionProcessor.yjVg0u|op356o
        - id: SessionProcessor.process_1
          signature: "public <M> List<M> process(List<String> inputs, SessionCallbacks<M> callbacks) {"
        - id: SessionProcessor.GH0Ski|onBBkU
        - id: SessionProcessor.process_2
          signature: "public <M> List<M> process(String input, Path rootPath, SessionCallbacks<M> cal…"
        - id: SessionProcessor.process_3
          signature: "public <M> List<M> process(String input, SessionCallbacks<M> callbacks) {"
        - id: SessionProcessor.Zo5MT0|u9akBQ
        - id: SessionProcessor.Run
          signature: "private final class Run<M> {"
          children:
          - id: SessionProcessor.Run.dCNsrf|wxJNjN
          - id: SessionProcessor.Run.Run
            signature: "Run(SessionCallbacks<M> callbacks, List<M> out) {"
          - id: SessionProcessor.Run.run
            signature: "void run(String text, List<Path> chain) {"
          - id: SessionProcessor.Run.flush
            signature: "private void flush() {"
          - id: SessionProcessor.Run.isMarkerLine
            signature: "private boolean isMarkerLine(String line) {"
          - id: SessionProcessor.Run.consumeReasoning
            signature: "private int consumeReasoning(String[] lines, int start) {"
          - id: SessionProcessor.Run.consumeText
            signature: "private int consumeText(String[] lines, int start) {"
          - id: SessionProcessor.Run.consumeToolCall
            signature: "private int consumeToolCall(String[] lines, int start) {"
          - id: SessionProcessor.Run.consumeToolResult
            signature: "private int consumeToolResult(String[] lines, int start) {"
          - id: SessionProcessor.Run.5LYnz5|7Jclc5
          - id: SessionProcessor.Run.fenceRange
            signature: "private int[] fenceRange(String[] lines, int from) {"
          - id: SessionProcessor.Run.readYaml
            signature: "private JsonNode readYaml(String[] lines, int[] range) {"
          - id: SessionProcessor.Run.include
            signature: "private void include(String kind, String arg, List<Path> chain) {"
          - id: SessionProcessor.Run.0CpLmU|lM1IBJ
          - id: SessionProcessor.Run.includePath
            signature: "private void includePath(String pathText, List<Path> chain) {"
          - id: SessionProcessor.Run.includeContextPrompt
            signature: "private void includeContextPrompt(String dir) {"
          - id: SessionProcessor.Run.includeFiles
            signature: "private void includeFiles(String arg) {"
          - id: SessionProcessor.Run.includeFile
            signature: "private void includeFile(String path) {"
          - id: SessionProcessor.Run.includeSearch
            signature: "private void includeSearch(String arg) {"
          - id: SessionProcessor.Run.renderMatches
            signature: "private String renderMatches(List<IIncludeAdapter.Match> matches) {"
          - id: SessionProcessor.Run.requireAdapter
            signature: "private IIncludeAdapter requireAdapter() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: eIZpM6|crr53s
      - id: YvzJYV|dN3kdP
      - id: Tu3Bgf|nccVUE
      - id: EditorInterface
        signature: "public class EditorInterface {"
        children:
        - id: EditorInterface.LN3qky|o7g4i2
        - id: EditorInterface.fuV05T|lPHpxv
        - id: EditorInterface.EditorInterface
          signature: "public EditorInterface(ActiveEditorListener editorListener, AdaptingConnector c…"
        - id: EditorInterface.insertTag
          signature: "public void insertTag(Display display, IModelRequest req, IProgressMonitor mon)…"
        - id: EditorInterface.generateTag
          signature: "private String generateTag(IModelRequest req) {"
        - id: EditorInterface.replaceTag
          signature: "public void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CallCommand.java
      nodes:
      - id: FAE3JL|G624In
      - id: dZq3Us|iibdak
      - id: CallCommand
        signature: "public class CallCommand extends Command {"
        children:
        - id: CallCommand.CallCommand
          signature: "public CallCommand() {"
        - id: CallCommand.prefix
          signature: "public String prefix() {"
        - id: CallCommand.matches
          signature: "public boolean matches(String line) {"
        - id: CallCommand.parse
          signature: "public Command parse(String line) {"
        - id: CallCommand.processorAction
          signature: "public ProcessorAction processorAction() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
      nodes:
      - id: 7n1FzW|AZ2luy
      - id: nrEKDc|TRc1ez
      - id: Prompt
        signature: "public final class Prompt {"
        children:
        - id: Prompt.g8D0gf|nXhdxi
        - id: Prompt.InputKind
          signature: "public enum InputKind {"
        - id: Prompt.0DK5HF|f1yN30
        - id: Prompt.Command
          signature: "public enum Command {"
        - id: Prompt.Kait7i|locSTk
        - id: Prompt.Prompt
          signature: "private Prompt(List<String> inputs, InputKind kind, boolean batch, FrozenConfig…"
        - id: Prompt.Xua3u0|1iYvhx
        - id: Prompt.sessionId
          signature: "public String sessionId() {"
        - id: Prompt.computeSessionId
          signature: "private String computeSessionId() {"
        - id: Prompt.G0QOxV|j22yeX
        - id: Prompt.toSessionParameters
          signature: "public static SessionParameters toSessionParameters(Prompt prompt, Path cwd) {"
        - id: Prompt.luEJel|M0HBGM
        - id: Prompt.toClaudeSessionParameters
          signature: "public static ClaudeSessionParameters toClaudeSessionParameters(Prompt prompt, …"
        - id: Prompt.builder
          signature: "public static Builder builder() {"
        - id: Prompt.4K9Ahw|LWbHnr
        - id: Prompt.Builder
          signature: "public static final class Builder {"
          children:
          - id: Prompt.Builder.vzZSXJ|xcGm6d
          - id: Prompt.Builder.Builder
            signature: "private Builder() {"
          - id: Prompt.Builder.inputs
            signature: "public Builder inputs(List<String> inputs) {"
          - id: Prompt.Builder.kind
            signature: "public Builder kind(InputKind kind) {"
          - id: Prompt.Builder.batch
            signature: "public Builder batch(boolean batch) {"
          - id: Prompt.Builder.config
            signature: "public Builder config(FrozenConfig config) {"
          - id: Prompt.Builder.processorEnabled
            signature: "public Builder processorEnabled(boolean processorEnabled) {"
          - id: Prompt.Builder.absoluteFilePath
            signature: "public Builder absoluteFilePath(String absoluteFilePath) {"
          - id: Prompt.Builder.command
            signature: "public Builder command(Command command) {"
          - id: Prompt.Builder.yamlBlock
            signature: "public Builder yamlBlock(String yamlBlock) {"
          - id: Prompt.Builder.build
            signature: "public Prompt build() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java
      nodes:
      - id: EdXLLM|DLA8pY
      - id: rBrJSJ|uC4fPi
      - id: xa1GeE|d9koxH
      - id: MCPClient
        signature: "public class MCPClient {"
        children:
        - id: MCPClient.62SeLa|J8XhWN
        - id: MCPClient.c5fgij|UujMEH
        - id: MCPClient.findTool
          signature: "public JsonNode findTool(String name) {"
        - id: MCPClient.listTools
          signature: "public synchronized ArrayNode listTools() {"
        - id: MCPClient.dyZrFr|tFfTSc
        - id: MCPClient.renderToolCall
          signature: "public String renderToolCall(String name, JsonNode arguments) {"
        - id: MCPClient.callTool
          signature: "public synchronized JsonNode callTool(String name, JsonNode arguments) {"
        - id: MCPClient.QWIJyc|pLeYmp
        - id: MCPClient.extractErrorText
          signature: "private String extractErrorText(JsonNode result) {"
        - id: MCPClient.close
          signature: "public synchronized void close() {"
        - id: MCPClient.ensureConnected
          signature: "private void ensureConnected() {"
        - id: MCPClient.send
          signature: "private JsonNode send(ObjectNode rpc, boolean expectResult) {"
        - id: MCPClient.parseMessage
          signature: private JsonNode parseMessage(HttpResponse<String> response) throws JsonProcess…
        - id: MCPClient.pickResponse
          signature: "private JsonNode pickResponse(String json, JsonNode current) {"
        - id: MCPClient.request
          signature: "private ObjectNode request(String method, JsonNode params) {"
        - id: MCPClient.notification
          signature: "private ObjectNode notification(String method) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
      nodes:
      - id: 3zBTmJ|TGrMqF
      - id: MCPCommand
        signature: "public class MCPCommand {"
        children:
        - id: MCPCommand.PJNTO9|QpkmbA
        - id: MCPCommand.MCPCommand
          signature: "public MCPCommand(MCPCommandType type, String... parameters) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
      nodes:
      - id: H2WnQb|QmRDpJ
      - id: MCPCommandType
        signature: "public enum MCPCommandType {"
```
/answer 8c1a-8a allow
ReasoningToken: 14
Control Request:
```yaml
id: 8c1a-8b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
    ids:
    - MCPControlClient
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - MCPConnector
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
    ids:
    - PromptHandler
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    ids:
    - EditorInterface
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CallCommand.java
    ids:
    - CallCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
    ids:
    - Prompt
  reason: Vollständigen Quellcode dieser Klassen lesen
```
/answer 8c1a-8b allow
Control Request:
```yaml
id: 8c1a-8c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
      nodes:
      - id: MCPControlClient
        type: class_declaration
        signature: "public class MCPControlClient {"
        children:
        - id: MCPControlClient.JloL0D|yKZBpv
          type: statements
          code: "private final YamlRenderer yaml = new YamlRenderer();\n\n\t/**\n\t * Renders a tool's input schema as a YAML template (fenced block). Everything\n\t * required for the call is bare YAML; descriptions and constraints are\n\t * comments; defaults are pre-filled.\n\t */"
        - id: MCPControlClient.renderSchema
          type: method_declaration
          code: "@SuppressWarnings(\"deprecation\")\n\tpublic String renderSchema(JsonNode tool) {\n\t\tStringBuilder sb = new StringBuilder();\n\t\tsb.append(\"```yaml\\n\");\n\t\tsb.append(\"tool: \").append(tool.path(\"name\").asText()).append(\"\\n\");\n\n\t\tJsonNode schema = tool.path(\"inputSchema\");\n\t\tJsonNode props = schema.path(\"properties\");\n\t\tSet<String> required = requiredSet(schema);\n\t\tStringBuilder argsBody = new StringBuilder();\n\t\tif (props.isObject()) {\n\t\t\tIterator<Map.Entry<String, JsonNode>> it = props.fields();\n\t\t\twhile (it.hasNext()) {\n\t\t\t\tMap.Entry<String, JsonNode> e = it.next();\n\t\t\t\tif (\"reason\".equals(e.getKey()))\n\t\t\t\t\tcontinue;\n\t\t\t\tappendProperty(argsBody, e.getKey(), e.getValue(), required.contains(e.getKey()));\n\t\t\t}\n\t\t}\n\t\tif (argsBody.length() > 0) {\n\t\t\tsb.append(\"arguments:\\n\").append(argsBody);\n\t\t} else {\n\t\t\tsb.append(\"arguments: {}\\n\");\n\t\t}\n\t\tsb.append(\"```\");\n\t\treturn sb.toString();\n\t}"
        - id: MCPControlClient.dKZb74|tHZasV
          type: statements
          code: "/** Extracts the content of a leading/embedded ```yaml block, or null. */"
        - id: MCPControlClient.extractYamlBlock
          type: method_declaration
          code: "public String extractYamlBlock(String text) {\n\t\tif (text == null)\n\t\t\treturn null;\n\t\tString t = text.strip();\n\t\tint start = t.indexOf(\"```yaml\");\n\t\tif (start < 0)\n\t\t\treturn null;\n\t\tint contentStart = start + \"```yaml\".length();\n\t\tint end = t.indexOf(\"```\", contentStart);\n\t\tif (end == -1)\n\t\t\treturn null;\n\t\treturn t.substring(contentStart, end).strip();\n\t}"
        - id: MCPControlClient.parseYaml
          type: method_declaration
          code: "public JsonNode parseYaml(String block) {\n\t\ttry {\n\t\t\tJsonNode node = yaml.readTree(block);\n\t\t\tif (node == null || !node.isObject())\n\t\t\t\tthrow new IllegalArgumentException(\"Tool call YAML must be a mapping\");\n\t\t\treturn node;\n\t\t} catch (JsonProcessingException e) {\n\t\t\tthrow new IllegalArgumentException(\"Invalid YAML tool call: \" + e.getMessage(), e);\n\t\t}\n\t}"
        - id: MCPControlClient.prettyResult
          type: method_declaration
          code: "public String prettyResult(JsonNode result) {\n\t\tJsonNode structured = result.path(\"structuredContent\");\n\t\tif (!structured.isMissingNode() && !structured.isNull())\n\t\t\treturn \"```json\\n\" + JsonUtil.pretty(structured) + \"\\n```\";\n\n\t\tJsonNode content = result.path(\"content\");\n\t\tif (content.isArray())\n\t\t\treturn extractText(content);\n\n\t\treturn \"```json\\n\" + JsonUtil.pretty(result) + \"\\n```\";\n\t}"
        - id: MCPControlClient.khCRJ6|sVRL8A
          type: statements
          code: "/** Concatenates the \"text\" entries of an MCP \"content\" array (plain, unfenced). */"
        - id: MCPControlClient.extractText
          type: method_declaration
          code: "public String extractText(JsonNode content) {\n\t\tStringBuilder sb = new StringBuilder();\n\t\tfor (JsonNode c : content) {\n\t\t\tif (sb.length() > 0)\n\t\t\t\tsb.append(\"\\n\");\n\t\t\tsb.append(c.path(\"text\").asText(\"\"));\n\t\t}\n\t\treturn sb.toString();\n\t}"
        - id: MCPControlClient.82vyIy|L43zbT
          type: statements
          code: "/**\n\t * Ensures a \"reason\" argument is present when the tool's schema declares one,\n\t * since it is filtered out of the rendered template and never filled in by\n\t * the user.\n\t */"
        - id: MCPControlClient.fillReason
          type: method_declaration
          code: "public JsonNode fillReason(JsonNode tool, JsonNode arguments) {\n\t\tObjectNode args = arguments != null && arguments.isObject() ? (ObjectNode) arguments\n\t\t\t\t: JsonUtil.mapper().createObjectNode();\n\t\tJsonNode props = tool == null ? null : tool.path(\"inputSchema\").path(\"properties\");\n\t\tif (props != null && props.has(\"reason\") && !args.has(\"reason\"))\n\t\t\targs.put(\"reason\", \"\");\n\t\treturn args;\n\t}"
        - id: MCPControlClient.appendProperty
          type: method_declaration
          code: "private void appendProperty(StringBuilder sb, String key, JsonNode prop, boolean required) {\n\t\tString indent = \"  \";\n\t\tappendComment(sb, indent, prop.path(\"description\").asText(\"\"));\n\t\tString type = prop.path(\"type\").asText(null);\n\t\tString value = renderValue(prop, type, indent);\n\t\tsb.append(indent).append(key).append(\":\");\n\t\tif (value.startsWith(\"\\n\"))\n\t\t\tsb.append(value).append(\"\\n\");\n\t\telse\n\t\t\tsb.append(\" \").append(value).append(\"\\n\");\n\t}"
        - id: MCPControlClient.renderValue
          type: method_declaration
          code: "private String renderValue(JsonNode prop, String type, String indent) {\n\t\tJsonNode def = prop.get(\"default\");\n\t\tif (def != null && !def.isNull())\n\t\t\treturn JsonUtil.compact(def);\n\t\tif (type == null)\n\t\t\treturn \"null\";\n\t\tswitch (type) {\n\t\tcase \"string\":\n\t\t\treturn \"\\\"\\\"\";\n\t\tcase \"integer\":\n\t\tcase \"number\":\n\t\t\treturn \"0\";\n\t\tcase \"boolean\":\n\t\t\treturn \"false\";\n\t\tcase \"array\":\n\t\t\treturn renderArrayValue(prop, indent);\n\t\tcase \"object\":\n\t\t\treturn \"{}\";\n\t\tdefault:\n\t\t\treturn \"null\";\n\t\t}\n\t}"
        - id: MCPControlClient.Mwqdao|cn5YCc
          type: statements
          code: "/**\n\t * Renders an array value. Generically pre-fills a single example entry when\n\t * the array's item schema is an object with a \"path\" property, since that is\n\t * by far the most common list-of-paths shape (e.g. \"- path: /path\").\n\t */\n\t/**\n\t * Renders an array value. Generically pre-fills a single example entry from\n\t * the array's item schema when it is an object with properties, instead of\n\t * just an empty \"[]\" — so the user sees the expected item shape directly.\n\t */"
        - id: MCPControlClient.renderArrayValue
          type: method_declaration
          code: "@SuppressWarnings(\"deprecation\")\n\tprivate String renderArrayValue(JsonNode prop, String indent) {\n\t\tJsonNode itemProps = prop.path(\"items\").path(\"properties\");\n\t\tif (!itemProps.isObject() || itemProps.size() == 0)\n\t\t\treturn \"[]\";\n\n\t\tString itemIndent = indent + \"  \";\n\t\tStringBuilder sb = new StringBuilder();\n\t\tIterator<Map.Entry<String, JsonNode>> it = itemProps.fields();\n\t\tboolean first = true;\n\t\twhile (it.hasNext()) {\n\t\t\tMap.Entry<String, JsonNode> e = it.next();\n\t\t\tString childType = e.getValue().path(\"type\").asText(null);\n\t\t\tString value = renderValue(e.getValue(), childType, itemIndent + \"  \");\n\t\t\tsb.append(\"\\n\").append(itemIndent).append(first ? \"- \" : \"  \").append(e.getKey()).append(\": \")\n\t\t\t\t\t.append(value);\n\t\t\tfirst = false;\n\t\t}\n\t\treturn sb.toString();\n\t}"
        - id: MCPControlClient.requiredSet
          type: method_declaration
          code: "private Set<String> requiredSet(JsonNode schema) {\n\t\tSet<String> req = new HashSet<>();\n\t\tJsonNode r = schema.path(\"required\");\n\t\tif (r.isArray())\n\t\t\tfor (JsonNode n : r)\n\t\t\t\treq.add(n.asText());\n\t\treturn req;\n\t}"
        - id: MCPControlClient.appendComment
          type: method_declaration
          code: "private void appendComment(StringBuilder sb, String indent, String text) {\n\t\tif (text == null || text.isBlank())\n\t\t\treturn;\n\t\tfor (String line : text.strip().split(\"\\n\"))\n\t\t\tsb.append(indent).append(\"# \").append(line.stripTrailing()).append(\"\\n\");\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
      nodes:
      - id: MCPConnector
        type: class_declaration
        signature: "public class MCPConnector implements IAIConnector<MCPRequest, MCPResponse> {"
        children:
        - id: MCPConnector.hOPGQb|xV4uSy
          type: statements
          code: "private final MCPClient client;\n\tprivate final MCPControlClient control = new MCPControlClient();\n\n\t@SuppressWarnings(\"unused\")\n\tprivate final ConfigManager cfg;"
        - id: MCPConnector.MCPConnector
          type: constructor_declaration
          code: "public MCPConnector(ConfigManager cfg, MCPClient client) {\n\t\tthis.cfg = cfg;\n\t\tthis.client = client;\n\t}"
        - id: MCPConnector.getSupportedKeyPattern
          type: method_declaration
          code: "@Override\n\tpublic KeyPattern getSupportedKeyPattern() {\n\t\treturn KeyPattern.Misc;\n\t}"
        - id: MCPConnector.createRequest
          type: method_declaration
          code: "@Override\n\tpublic MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\treturn new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools,\n\t\t\t\tpreprocess(prompt.inputs));\n\t}"
        - id: MCPConnector.executeRequest
          type: method_declaration
          code: "@Override\n\tpublic MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job) {\n\t\tswitch (req.cmd.type) {\n\t\tcase Exit:\n\t\t\tclient.close();\n\t\t\treturn new MCPResponse(req.id, \"MCP connection closed\");\n\t\tcase Tool: {\n\t\t\tJsonNode tool = client.findTool(req.cmd.parameter);\n\t\t\tif (tool == null)\n\t\t\t\tthrow new IllegalArgumentException(\"Unknown tool: \" + req.cmd.parameter);\n\t\t\treturn new MCPResponse(req.id, control.renderSchema(tool));\n\t\t}\n\t\tcase Prompt: {\n\t\t\tString block = control.extractYamlBlock(req.cmd.parameter);\n\t\t\tif (block == null)\n\t\t\t\tthrow new IllegalArgumentException(\n\t\t\t\t\t\t\"No ```yaml tool call block found. Use \\\"/tool <id>\\\" to get a template.\");\n\t\t\tJsonNode call = control.parseYaml(block);\n\t\t\tString name = call.path(\"tool\").asText(null);\n\t\t\tif (name == null || name.isBlank())\n\t\t\t\tthrow new IllegalArgumentException(\"Tool call is missing the 'tool' field\");\n\t\t\tJsonNode tool = client.findTool(name);\n\t\t\tJsonNode arguments = control.fillReason(tool, call.path(\"arguments\"));\n\t\t\tJsonNode result = client.callTool(name, arguments);\n\t\t\treturn new MCPResponse(req.id, control.prettyResult(result));\n\t\t}\n\t\tdefault:\n\t\t\tthrow new UnsupportedOperationException(\"Unsupported command: \" + req.cmd.type);\n\t\t}\n\t}"
        - id: MCPConnector.convertResponse
          type: method_declaration
          code: "@Override\n\tpublic AIAnswer convertResponse(MCPResponse resp, IProgressMonitor mon) {\n\t\tAIAnswer answer = new AIAnswer(resp.id);\n\t\tanswer.answer = resp.resultText;\n\t\treturn answer;\n\t}"
        - id: MCPConnector.preprocess
          type: method_declaration
          code: "private MCPCommand preprocess(List<String> inputs) {\n\t\tStringBuilder merged = null;\n\t\tfor (String input : inputs) {\n\t\t\tString clean = input != null ? input.strip() : \"\";\n\t\t\tif (clean.isBlank())\n\t\t\t\tcontinue;\n\t\t\tvar detected = CommandHandler.detect(clean);\n\t\t\tif (detected.isPresent() && detected.get() instanceof ExitCommand)\n\t\t\t\treturn new MCPCommand(MCPCommandType.Exit, \"\");\n\t\t\tif (detected.isPresent() && detected.get() instanceof ToolCommand tc)\n\t\t\t\treturn new MCPCommand(MCPCommandType.Tool, tc.parameter(0));\n\t\t\tif (merged == null)\n\t\t\t\tmerged = new StringBuilder();\n\t\t\telse\n\t\t\t\tmerged.append(\"\\n\");\n\t\t\tmerged.append(clean);\n\t\t}\n\t\tif (merged == null)\n\t\t\tthrow new IllegalStateException(\"No commands in inputs\");\n\t\treturn new MCPCommand(MCPCommandType.Prompt, merged.toString());\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
      nodes:
      - id: PromptHandler
        type: class_declaration
        signature: "public final class PromptHandler {"
        children:
        - id: PromptHandler.1gKcmV|Beh0X2
          type: statements
          code: "// Captures the last (i.e. closest preceding) ```yaml ... ``` fenced block before a command line.\n\tprivate static final Pattern YAML_BLOCK = Pattern.compile(\"^```yaml\\\\R(.*?)^```$\", Pattern.MULTILINE | Pattern.DOTALL);\n\n\tprivate final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\tprivate final SessionProcessor sessionProcessor;\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final EditorInterface editIfc;\n\tprivate final IncludeAdapter includeAdapter;"
        - id: PromptHandler.LSyBuh|FgyeYv
          type: statements
          code: "private int[] inputStats = new int[InputMode.values().length];\n\tprivate List<Consumer<AIAnswer>> answerObs = new ArrayList<>();\n\tprivate List<Consumer<int[]>> inputStatObs = new ArrayList<>();"
        - id: PromptHandler.PromptHandler
          type: constructor_declaration
          code: "public PromptHandler(ConfigManager cfg, AdaptingConnector connector, SessionProcessor sessionProcessor,\n\t\t\tActiveEditorListener editorListener, EditorInterface editIfc, IncludeAdapter includeAdapter) {\n\t\tthis.cfg = cfg;\n\t\tthis.connector = connector;\n\t\tthis.sessionProcessor = sessionProcessor;\n\t\tthis.editorListener = editorListener;\n\t\tthis.editIfc = editIfc;\n\t\tthis.includeAdapter = includeAdapter;\n\t}"
        - id: PromptHandler.clearObserver
          type: method_declaration
          code: "public void clearObserver() {\n\t\tanswerObs.clear();\n\t\tinputStatObs.clear();\n\t}"
        - id: PromptHandler.addInputStatObs
          type: method_declaration
          code: "public void addInputStatObs(Consumer<int[]> obs, boolean initialize) {\n\t\tinputStatObs.add(obs);\n\t\tif (initialize)\n\t\t\tobs.accept(inputStats);\n\t}"
        - id: PromptHandler.addAnswerObs
          type: method_declaration
          code: "public void addAnswerObs(Consumer<AIAnswer> obs) {\n\t\tanswerObs.add(obs);\n\t}"
        - id: PromptHandler.updateInputStat
          type: method_declaration
          code: "public void updateInputStat(InputMode mode) {\n\t\tif (mode == InputMode.Tools) {\n\t\t\tString[] tools = cfg.getTools();\n\t\t\tinputStats[mode.ordinal()] = tools != null ? tools.length : 0;\n\t\t} else {\n\t\t\tString input = getInput(mode);\n\t\t\tinputStats[mode.ordinal()] = input != null ? input.length() : -1;\n\t\t}\n\t\tinputStatObs.forEach(c -> c.accept(inputStats));\n\t}"
        - id: PromptHandler.initializeInputs
          type: method_declaration
          code: "public void initializeInputs() {\n\t\tfor (var mode : InputMode.values())\n\t\t\tupdateInputStat(mode);\n\t\tincludeAdapter.initializeInputs();\n\t}"
        - id: PromptHandler.getInput
          type: method_declaration
          code: "private String getInput(InputMode mode) {\n\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\n\t\tswitch (mode) {\n\t\tcase SystemPrompt:\n\t\t\tStringBuffer systemPrompt = new StringBuffer();\n\t\t\tArrays.stream(cfg.getSystemPrompt()).filter(e -> !e.startsWith(\"#\"))\n\t\t\t\t\t.forEach(e -> systemPrompt.append(\"* \").append(e).append(\".\\n\"));\n\t\t\tString freeText = cfg.getFreeText();\n\t\t\tif (freeText != null && !freeText.isBlank())\n\t\t\t\tsystemPrompt.append(\".\\n\").append(cfg.getFreeText()).append(\".\\n\");\n\t\t\tString prompttext = systemPrompt.toString();\n\t\t\treturn prompttext.length() > 0 && !prompttext.isBlank() ? prompttext : null;\n\t\tcase Selection:\n\t\t\tif (textEditor != null) {\n\t\t\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\t\t\tif (selectionProvider != null) {\n\t\t\t\t\tISelection selection = selectionProvider.getSelection();\n\t\t\t\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\t\t\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1)\n\t\t\t\t\t\treturn removeCommentLines(tsel.getText());\n\n\t\t\t\t\tif (tsel != null) {\n\t\t\t\t\t\tint line = tsel.getEndLine();\n\t\t\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\tIRegion lineInfo = doc.getLineInformation(line);\n\t\t\t\t\t\t\treturn doc.get(lineInfo.getOffset(), lineInfo.getLength());\n\t\t\t\t\t\t} catch (BadLocationException e1) {\n\t\t\t\t\t\t\tLOG.error(\"Exception\", e1);\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t\tbreak;\n\t\tcase Converter:\n\t\t\tif (textEditor != null) {\n\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\tif (doc != null)\n\t\t\t\t\treturn doc.get();\n\t\t\t}\n\t\t\tbreak;\n\t\tcase Tools:\n\t\t\tthrow new UnsupportedOperationException();\n\t\t}\n\t\treturn null;\n\t}"
        - id: PromptHandler.removeCommentLines
          type: method_declaration
          code: "public String removeCommentLines(String input) {\n\t\tif (input == null || input.isEmpty())\n\t\t\treturn input;\n\n\t\tStringBuffer result = new StringBuffer();\n\t\tString[] lines = input.split(\"\\\\R\");\n\n\t\tfor (String line : lines)\n\t\t\tif (!line.trim().startsWith(AbstractRule.LINE_COMMENT))\n\t\t\t\tresult.append(line).append(System.lineSeparator());\n\n\t\treturn result.toString();\n\t}"
        - id: PromptHandler.execute
          type: method_declaration
          code: "public void execute(Display display) {\n\t\tnew PromptJob(\"Starting Prompt\", display).schedule();\n\t}"
        - id: PromptHandler.PromptJob
          type: class_declaration
          signature: "private class PromptJob extends Job {"
          children:
          - id: PromptHandler.PromptJob.zOXTQk|KFkQPW
            type: statements
            code: private final Display display;
          - id: PromptHandler.PromptJob.PromptJob
            type: constructor_declaration
            code: "private PromptJob(String name, Display display) {\n\t\t\tsuper(name);\n\t\t\tthis.display = display;\n\t\t}"
          - id: PromptHandler.PromptJob.run
            type: method_declaration
            code: "@Override\n\t\tprotected IStatus run(IProgressMonitor mon) {\n\t\t\tSubMonitor sub = SubMonitor.convert(mon, \"Executing prompt\", 4);\n\t\t\tString reqId = null;\n\t\t\ttry {\n\t\t\t\tsub.subTask(\"Prepare inputs\");\n\t\t\t\tvar req = prepareRequest(display, false, sub);\n\t\t\t\tsub.worked(1);\n\t\t\t\tsub.subTask(\"Insert Tag\");\n\t\t\t\teditIfc.insertTag(display, req, sub);\n\t\t\t\treqId = req.getID();\n\t\t\t\tmon.worked(1);\n\n\t\t\t\tsub.subTask(\"Execute prompt\");\n\t\t\t\tvar ans = executeInner(display, req, sub, this);\n\t\t\t\tmon.worked(1);\n\t\t\t\tsub.subTask(\"Process Answer\");\n\t\t\t\teditIfc.replaceTag(display, ans, sub);\n\t\t\t\tmon.worked(1);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t// uncatched error case\n\t\t\t\tif (reqId != null) {\n\t\t\t\t\tAIAnswer error = new AIAnswer(reqId);\n\t\t\t\t\terror.answer = e.getMessage();\n\t\t\t\t\teditIfc.replaceTag(display, error, sub);\n\t\t\t\t}\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}"
        - id: PromptHandler.queueAsync
          type: method_declaration
          code: "public void queueAsync(Display display, AIBatchManager batch) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, batch, mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.queueSync
          type: method_declaration
          code: "private void queueSync(Display display, AIBatchManager batch, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Enqueue batch prompt\", 3);\n\t\tsub.subTask(\"Prepare inputs\");\n\t\tvar req = prepareRequest(display, true, sub.split(1));\n\t\tsub.subTask(\"Insert Tag\");\n\t\teditIfc.insertTag(display, req, sub.split(1));\n\t\tsub.subTask(\"Enqueue prompt\");\n\t\tbatch.enqueue(req, sub.split(1));\n\t}"
        - id: PromptHandler.queueAndSubmit
          type: method_declaration
          code: "public void queueAndSubmit(Display display, AIBatchManager batch) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, batch, mon);\n\t\t\t\tbatch.submitBatches(mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}"
        - id: PromptHandler.prepareRequest
          type: method_declaration
          code: "private IModelRequest prepareRequest(Display display, boolean batchFix, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Preparing Call\", 1);\n\t\tsub.subTask(\"Preparing Call\");\n\t\tPrompt prompt = buildPrompt(display, batchFix);\n\t\tIModelRequest req = connector.createRequest(prompt, sub);\n\t\tsub.worked(1);\n\t\treturn req;\n\t}"
        - id: PromptHandler.maceLN|sqs7wQ
          type: statements
          code: "/** Freezes the current editor/config state into a {@link Prompt}; runs command detection (see class doc). */"
        - id: PromptHandler.buildPrompt
          type: method_declaration
          code: "Prompt buildPrompt(Display display, boolean batch) {\n\t\tList<String> inputs = new ArrayList<>();\n\t\tString[] absoluteFilePath = new String[1];\n\t\tPrompt.InputKind[] kind = new Prompt.InputKind[1];\n\t\tCommand[] detected = new Command[1];\n\t\tString[] yamlBlock = new String[1];\n\n\t\tdisplay.syncExec(() -> {\n\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\tabsoluteFilePath[0] = resolveAbsoluteFilePath(textEditor);\n\n\t\t\tif (cfg.isInputEnabled(InputMode.Selection))\n\t\t\t\tkind[0] = detectSelection(textEditor, inputs, detected, yamlBlock);\n\t\t\telse if (cfg.isInputEnabled(InputMode.Converter))\n\t\t\t\tkind[0] = detectFullFile(textEditor, inputs, detected, yamlBlock);\n\t\t});\n\n\t\tFrozenConfig frozen = FrozenConfig.from(cfg);\n\n\t\tif (inputs.isEmpty() && (frozen.systemPrompt == null || frozen.systemPrompt.isBlank()))\n\t\t\tthrow new IllegalArgumentException(\"Input and System Prompt Empty\");\n\n\t\tif (editorListener.getLastTextEditor() == null && !batch)\n\t\t\tthrow new IllegalArgumentException(\"Result editor unset\");\n\n\t\tPrompt.Builder builder = Prompt.builder()//\n\t\t\t\t.inputs(inputs)//\n\t\t\t\t.kind(kind[0])//\n\t\t\t\t.batch(batch)//\n\t\t\t\t.config(frozen)//\n\t\t\t\t.processorEnabled(cfg.isInputEnabled(InputMode.Converter))//\n\t\t\t\t.absoluteFilePath(absoluteFilePath[0]);\n\n\t\tif (detected[0] instanceof CallCommand)\n\t\t\tbuilder.command(Prompt.Command.CALL).yamlBlock(yamlBlock[0]);\n\t\telse\n\t\t\tbuilder.command(Prompt.Command.NONE);\n\n\t\treturn builder.build();\n\t}"
        - id: PromptHandler.ohnvjH|nhSFvk
          type: statements
          code: "// Selection mode: a real (multi-char) selection is a BlockSelection, an empty/caret selection\n\t// falls back to the current cursor line (LineInput)."
        - id: PromptHandler.detectSelection
          type: method_declaration
          code: "private Prompt.InputKind detectSelection(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn Prompt.InputKind.LineInput;\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn Prompt.InputKind.LineInput;\n\n\t\tif (tsel != null && !tsel.isEmpty() && tsel.getLength() > 1) {\n\t\t\tinputs.add(removeCommentLines(tsel.getText()));\n\t\t\t// Block start (multi-line command) takes precedence over a trailing command line.\n\t\t\tif (!applyDetected(CommandHandler.detect(lineText(doc, tsel.getStartLine())), doc, tsel.getStartLine(),\n\t\t\t\t\tdetected, yamlBlock))\n\t\t\t\tapplyDetected(CommandHandler.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(), detected,\n\t\t\t\t\t\tyamlBlock);\n\t\t\treturn Prompt.InputKind.BlockSelection;\n\t\t}\n\n\t\tif (tsel != null) {\n\t\t\ttry {\n\t\t\t\tIRegion lineInfo = doc.getLineInformation(tsel.getEndLine());\n\t\t\t\tinputs.add(doc.get(lineInfo.getOffset(), lineInfo.getLength()));\n\t\t\t} catch (BadLocationException e) {\n\t\t\t\tLOG.error(\"Exception\", e);\n\t\t\t}\n\t\t\tapplyDetected(CommandHandler.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(), detected,\n\t\t\t\t\tyamlBlock);\n\t\t}\n\t\treturn Prompt.InputKind.LineInput;\n\t}"
        - id: PromptHandler.Txg3oX|3Xw56U
          type: statements
          code: "// Processor (full file) mode: the caret line is checked first, then the last line of the file,\n\t// matching a command appended after the generated content."
        - id: PromptHandler.detectFullFile
          type: method_declaration
          code: "private Prompt.InputKind detectFullFile(ITextEditor textEditor, List<String> inputs, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (textEditor == null)\n\t\t\treturn Prompt.InputKind.FullFile;\n\n\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\tif (doc == null)\n\t\t\treturn Prompt.InputKind.FullFile;\n\t\tinputs.add(doc.get());\n\n\t\tISelectionProvider selectionProvider = textEditor.getSelectionProvider();\n\t\tISelection selection = selectionProvider != null ? selectionProvider.getSelection() : null;\n\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\t\tif (tsel != null\n\t\t\t\t&& applyDetected(CommandHandler.detect(lineText(doc, tsel.getEndLine())), doc, tsel.getEndLine(),\n\t\t\t\t\t\tdetected, yamlBlock))\n\t\t\treturn Prompt.InputKind.FullFile;\n\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\tapplyDetected(CommandHandler.detect(lineText(doc, lastLine)), doc, lastLine, detected, yamlBlock);\n\t\treturn Prompt.InputKind.FullFile;\n\t}"
        - id: PromptHandler.applyDetected
          type: method_declaration
          code: "private boolean applyDetected(Optional<Command> cmd, IDocument doc, int lineIndex, Command[] detected,\n\t\t\tString[] yamlBlock) {\n\t\tif (cmd.isEmpty())\n\t\t\treturn false;\n\t\tdetected[0] = cmd.get();\n\t\tif (detected[0] instanceof CallCommand)\n\t\t\tyamlBlock[0] = captureYamlBlock(doc, lineIndex);\n\t\treturn true;\n\t}"
        - id: PromptHandler.lineText
          type: method_declaration
          code: "private String lineText(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tIRegion info = doc.getLineInformation(lineIndex);\n\t\t\treturn doc.get(info.getOffset(), info.getLength());\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn \"\";\n\t\t}\n\t}"
        - id: PromptHandler.Q5D2ne|VNsv72
          type: statements
          code: // Walks backwards from the command line and keeps the closest preceding ```yaml block.
        - id: PromptHandler.captureYamlBlock
          type: method_declaration
          code: "private String captureYamlBlock(IDocument doc, int lineIndex) {\n\t\ttry {\n\t\t\tString prefix = doc.get(0, doc.getLineOffset(lineIndex));\n\t\t\tMatcher m = YAML_BLOCK.matcher(prefix);\n\t\t\tString last = null;\n\t\t\twhile (m.find())\n\t\t\t\tlast = m.group(1);\n\t\t\treturn last;\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(\"Exception\", e);\n\t\t\treturn null;\n\t\t}\n\t}"
        - id: PromptHandler.resolveAbsoluteFilePath
          type: method_declaration
          code: "private String resolveAbsoluteFilePath(ITextEditor textEditor) {\n\t\tif (textEditor == null)\n\t\t\treturn null;\n\t\tIEditorInput input = textEditor.getEditorInput();\n\t\tif (input instanceof IFileEditorInput)\n\t\t\treturn ((IFileEditorInput) input).getFile().getLocation().toFile().getAbsolutePath();\n\t\tif (input instanceof IURIEditorInput)\n\t\t\treturn new File(((IURIEditorInput) input).getURI()).getAbsolutePath();\n\t\treturn null;\n\t}"
        - id: PromptHandler.executeInner
          type: method_declaration
          code: "private AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon, PromptJob job) {\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(null)));\n\t\tIModelResponse resp = connector.executeRequest(req, mon, job);\n\t\tAIAnswer res = connector.convertResponse(resp, mon);\n\t\tdisplay.asyncExec(() -> answerObs.forEach(c -> c.accept(res)));\n\t\treturn res;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: EditorInterface
        type: class_declaration
        signature: "public class EditorInterface {"
        children:
        - id: EditorInterface.LN3qky|o7g4i2
          type: statements
          code: "public static final String USER = \"User:\";\n\tpublic static final String AGENT = \"Agent:\";\n\n\t/** Session block markers recognized by {@code connector.harness.SessionProcessor}. */\n\tpublic static final String TEXT = \"Text:\";\n\tpublic static final String THINKING = \"Thinking:\";\n\tpublic static final String TOOLUSE = \"Tool:\";\n\tpublic static final String TOOLRESULT = \"ToolResult:\";\n\n\t/** Command prefixes recognized by connector.CommandHandler / SessionProcessor. */\n\tpublic static final String CMD_ANSWER = \"/answer\";"
        - id: EditorInterface.fuV05T|lPHpxv
          type: statements
          code: "public static final String CMD_RESUME = \"/resume\";\n\tpublic static final String CMD_EXIT   = \"/exit\";\n\tpublic static final String CMD_TOOL   = \"/tool\";\n\tpublic static final String CMD_CALL   = \"/call\";\n\tpublic static final String CONTROL_REQUEST = \"Control Request:\";\n\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final AdaptingConnector connector;\n\tprivate final ConfigManager cfg;"
        - id: EditorInterface.EditorInterface
          type: constructor_declaration
          code: "public EditorInterface(ActiveEditorListener editorListener, AdaptingConnector connector, ConfigManager cfg) {\n\t\tthis.editorListener = editorListener;\n\t\tthis.connector = connector;\n\t\tthis.cfg = cfg;\n\t}"
        - id: EditorInterface.insertTag
          type: method_declaration
          code: "public void insertTag(Display display, IModelRequest req, IProgressMonitor mon) {\n\t\tdisplay.syncExec(() -> {\n\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\tif (OutputMode.New_File.equals(cfg.getOuputMode())) {\n\n\t\t\t\tIEditorInput editorInput = textEditor.getEditorInput();\n\t\t\t\tIFile currentFile;\n\t\t\t\tif (editorInput instanceof IFileEditorInput)\n\t\t\t\t\tcurrentFile = ((IFileEditorInput) editorInput).getFile();\n\t\t\t\telse if (editorInput instanceof IURIEditorInput) {\n\t\t\t\t\tURI uri = ((IURIEditorInput) editorInput).getURI();\n\t\t\t\t\tString fileName = new Path(uri.getPath()).lastSegment();\n\t\t\t\t\tcurrentFile = ResourcesPlugin.getWorkspace().getRoot().getProject(\"ExternalFiles\")\n\t\t\t\t\t\t\t.getFile(fileName);\n\n\t\t\t\t\tif (!currentFile.exists())\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\tcurrentFile.createLink(uri, IResource.ALLOW_MISSING_LOCAL, mon);\n\t\t\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\t\t\tthrow new IllegalStateException(\"Could not link external file\", e);\n\t\t\t\t\t\t}\n\t\t\t\t} else\n\t\t\t\t\tthrow new IllegalArgumentException(\"Editor type not supported for new file output mode\");\n\n\t\t\t\tIContainer parent = currentFile.getParent();\n\n\t\t\t\tString timestamp = LocalDateTime.now().format(DateTimeFormatter.ofPattern(\"yyyyMMdd.HHmmss\"));\n\t\t\t\tIFile newFile = parent.getFile(new Path(timestamp + \".md\"));\n\t\t\t\tString tag = generateTag(req);\n\t\t\t\ttry {\n\t\t\t\t\tInputStream source = new ByteArrayInputStream(tag.getBytes(\"UTF-8\"));\n\n\t\t\t\t\tif (!newFile.exists()) {\n\t\t\t\t\t\tnewFile.create(source, true, null);\n\t\t\t\t\t} else {\n\t\t\t\t\t\tnewFile.setContents(source, true, true, null);\n\t\t\t\t\t}\n\t\t\t\t\tnewFile.touch(null);\n\n\t\t\t\t\tIWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();\n\t\t\t\t\tIDE.openEditor(page, newFile);\n\t\t\t\t} catch (PartInitException e) {\n\t\t\t\t\tLOG.info(\"Error opening new editor file\");\n\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\tLOG.info(\"Error writting file\");\n\t\t\t\t} catch (UnsupportedEncodingException e) {\n\t\t\t\t\tLOG.info(\"Error unsupported encoding\");\n\t\t\t\t}\n\n\t\t\t} else {\n\n\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\tISelection selection = textEditor.getSelectionProvider().getSelection();\n\t\t\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\n\t\t\t\ttry {\n\t\t\t\t\tString tag = generateTag(req);\n\t\t\t\t\tswitch (cfg.getOuputMode()) {\n\t\t\t\t\tcase Chat:\n\t\t\t\t\t\tString replace = String.format(\"\\n%s\\n%s\\n%s\\n\", AGENT, tag, USER);\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, replace);\n\t\t\t\t\t\tdisplay.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Append:\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, \"\\n\" + tag);\n\t\t\t\t\t\tdisplay.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Replace:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), tsel.getLength(), tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Cursor:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), 0, tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase New_File:\n\t\t\t\t\t\tthrow new UnsupportedOperationException();\n\t\t\t\t\t}\n\t\t\t\t\ttextEditor.doSave(mon);\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.info(\"Error adding text\");\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t}"
        - id: EditorInterface.generateTag
          type: method_declaration
          code: "private String generateTag(IModelRequest req) {\n\t\tKeyPattern pattern = connector.getConnector(req).getSupportedKeyPattern();\n\t\treturn MarkerRessourceScanner.getPromptTag(pattern.name(), req.getID());\n\t}"
        - id: EditorInterface.replaceTag
          type: method_declaration
          code: "public void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {\n\t\tif (!Activator.getDefault().markerScanner.findAndReplaceMarkers(ans))\n\t\t\tLOG.info(\"Error: wasn't able to replace prompt marker with answer:\\n\" + ans.answer);\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CallCommand.java
      nodes:
      - id: CallCommand
        type: class_declaration
        signature: "public class CallCommand extends Command {"
        children:
        - id: CallCommand.CallCommand
          type: constructor_declaration
          code: "public CallCommand() {\n\t\tsuper();\n\t}"
        - id: CallCommand.prefix
          type: method_declaration
          code: "@Override\n\tpublic String prefix() {\n\t\treturn EditorInterface.CMD_CALL;\n\t}"
        - id: CallCommand.matches
          type: method_declaration
          code: "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && prefix().equalsIgnoreCase(line.strip());\n\t}"
        - id: CallCommand.parse
          type: method_declaration
          code: "@Override\n\tpublic Command parse(String line) {\n\t\treturn new CallCommand();\n\t}"
        - id: CallCommand.processorAction
          type: method_declaration
          code: "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java
      nodes:
      - id: Prompt
        type: class_declaration
        signature: "public final class Prompt {"
        children:
        - id: Prompt.g8D0gf|nXhdxi
          type: statements
          code: /** Origin of the collected input text (see Step 06). */
        - id: Prompt.InputKind
          type: enum_declaration
          code: "public enum InputKind {\n\t\tLineInput, BlockSelection, FullFile\n\t}"
        - id: Prompt.0DK5HF|f1yN30
          type: statements
          code: "/** Typed command payload, captured alongside an optional YAML block (Step 06/08). */"
        - id: Prompt.Command
          type: enum_declaration
          code: "public enum Command {\n\t\tNONE, CALL\n\t}"
        - id: Prompt.Kait7i|locSTk
          type: statements
          code: "// Inputs\n\tpublic final List<String> inputs;\n\tpublic final InputKind kind;\n\n\t// Batch\n\tpublic final boolean batch;\n\n\t// Config\n\tpublic final FrozenConfig config;\n\n\tpublic final boolean processorEnabled;\n\tpublic final String absoluteFilePath;\n\n\t// Optional command payload\n\tpublic final Command command;\n\tpublic final String yamlBlock;\n\n\tprivate String sessionId;"
        - id: Prompt.Prompt
          type: constructor_declaration
          code: "private Prompt(List<String> inputs, InputKind kind, boolean batch, FrozenConfig config, boolean processorEnabled,\n\t\t\tString absoluteFilePath, Command command, String yamlBlock) {\n\t\tif (config == null)\n\t\t\tthrow new IllegalArgumentException(\"config must not be null\");\n\t\tif (absoluteFilePath == null || absoluteFilePath.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"absoluteFilePath must not be blank\");\n\t\tthis.inputs = inputs != null ? List.copyOf(inputs) : List.of();\n\t\tthis.kind = kind;\n\t\tthis.batch = batch;\n\t\tthis.config = config;\n\t\tthis.processorEnabled = processorEnabled;\n\t\tthis.absoluteFilePath = absoluteFilePath;\n\t\tthis.command = command != null ? command : Command.NONE;\n\t\tthis.yamlBlock = yamlBlock;\n\t}"
        - id: Prompt.Xua3u0|1iYvhx
          type: statements
          code: "/** Deterministic session id: hash(absoluteFilePath, batch, config.getHash()). */"
        - id: Prompt.sessionId
          type: method_declaration
          code: "public String sessionId() {\n\t\tif (sessionId == null)\n\t\t\tsessionId = computeSessionId();\n\t\treturn sessionId;\n\t}"
        - id: Prompt.computeSessionId
          type: method_declaration
          code: "private String computeSessionId() {\n\t\tString input = absoluteFilePath + \"|\" + batch + \"|\" + config.getHash();\n\t\ttry {\n\t\t\tMessageDigest md = MessageDigest.getInstance(\"MD5\");\n\t\t\tbyte[] bytes = md.digest(input.getBytes(StandardCharsets.UTF_8));\n\t\t\tStringBuilder sb = new StringBuilder();\n\t\t\tfor (byte b : bytes)\n\t\t\t\tsb.append(String.format(\"%02x\", b));\n\t\t\treturn sb.substring(0, 8);\n\t\t} catch (NoSuchAlgorithmException e) {\n\t\t\t// Stable fallback (no external dependency)\n\t\t\tlong h = 0;\n\t\t\tfor (char c : input.toCharArray())\n\t\t\t\th = h * 31L + c;\n\t\t\treturn String.format(\"%08x\", h & 0xFFFFFFFFL);\n\t\t}\n\t}"
        - id: Prompt.G0QOxV|j22yeX
          type: statements
          code: "/** Derives generic SessionParameters from {@code prompt.config} (replaces Step 07's fromConfig(cfg, ...)). */"
        - id: Prompt.toSessionParameters
          type: method_declaration
          code: "public static SessionParameters toSessionParameters(Prompt prompt, Path cwd) {\n\t\tFrozenConfig c = prompt.config;\n\t\treturn new SessionParameters(cwd, c.systemPrompt, c.tools, c.model, c.reasoning, c.profile, c.cliProfile,\n\t\t\t\tc.cacheMode, prompt.absoluteFilePath, c.topP, c.temperature,\n\t\t\t\tc.maxOutputTokens != null ? c.maxOutputTokens.intValue() : null);\n\t}"
        - id: Prompt.luEJel|M0HBGM
          type: statements
          code: "/** Derives Claude-specific SessionParameters from {@code prompt.config}. */"
        - id: Prompt.toClaudeSessionParameters
          type: method_declaration
          code: "public static ClaudeSessionParameters toClaudeSessionParameters(Prompt prompt, Path cwd) {\n\t\tFrozenConfig c = prompt.config;\n\t\treturn new ClaudeSessionParameters(cwd, c.systemPrompt, c.tools, c.model, c.reasoning, c.profile, c.cliProfile,\n\t\t\t\tc.cacheMode, prompt.absoluteFilePath);\n\t}"
        - id: Prompt.builder
          type: method_declaration
          code: "public static Builder builder() {\n\t\treturn new Builder();\n\t}"
        - id: Prompt.4K9Ahw|LWbHnr
          type: statements
          code: "/** Builder for {@link Prompt}; intended for use by the PromptHandler (Step 06). */"
        - id: Prompt.Builder
          type: class_declaration
          signature: "public static final class Builder {"
          children:
          - id: Prompt.Builder.vzZSXJ|xcGm6d
            type: statements
            code: "private List<String> inputs;\n\t\tprivate InputKind kind;\n\t\tprivate boolean batch;\n\t\tprivate FrozenConfig config;\n\t\tprivate boolean processorEnabled;\n\t\tprivate String absoluteFilePath;\n\t\tprivate Command command;\n\t\tprivate String yamlBlock;"
          - id: Prompt.Builder.Builder
            type: constructor_declaration
            code: "private Builder() {\n\t\t}"
          - id: Prompt.Builder.inputs
            type: method_declaration
            code: "public Builder inputs(List<String> inputs) {\n\t\t\tthis.inputs = inputs;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.kind
            type: method_declaration
            code: "public Builder kind(InputKind kind) {\n\t\t\tthis.kind = kind;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.batch
            type: method_declaration
            code: "public Builder batch(boolean batch) {\n\t\t\tthis.batch = batch;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.config
            type: method_declaration
            code: "public Builder config(FrozenConfig config) {\n\t\t\tthis.config = config;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.processorEnabled
            type: method_declaration
            code: "public Builder processorEnabled(boolean processorEnabled) {\n\t\t\tthis.processorEnabled = processorEnabled;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.absoluteFilePath
            type: method_declaration
            code: "public Builder absoluteFilePath(String absoluteFilePath) {\n\t\t\tthis.absoluteFilePath = absoluteFilePath;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.command
            type: method_declaration
            code: "public Builder command(Command command) {\n\t\t\tthis.command = command;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.yamlBlock
            type: method_declaration
            code: "public Builder yamlBlock(String yamlBlock) {\n\t\t\tthis.yamlBlock = yamlBlock;\n\t\t\treturn this;\n\t\t}"
          - id: Prompt.Builder.build
            type: method_declaration
            code: "public Prompt build() {\n\t\t\treturn new Prompt(inputs, kind, batch, config, processorEnabled, absoluteFilePath, command, yamlBlock);\n\t\t}"
```
/answer 8c1a-8c allow
ReasoningToken: 94
Control Request:
```yaml
id: 8c1a-8d
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
    ids:
    - MCPCommand
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    ids:
    - MCPCommandType
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
    ids:
    - SessionProcessor.Run
  reason: Details zu MCPCommand/-Type und consumeToolCall lesen
```
/answer 8c1a-8d allow
Control Request:
```yaml
id: 8c1a-8e
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommand.java
      nodes:
      - id: MCPCommand
        type: class_declaration
        signature: "public class MCPCommand {"
        children:
        - id: MCPCommand.PJNTO9|QpkmbA
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
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
      nodes:
      - id: SessionProcessor.Run
        type: class_declaration
        signature: "private final class Run<M> {"
        children:
        - id: SessionProcessor.Run.dCNsrf|wxJNjN
          type: statements
          code: "private final SessionCallbacks<M> callbacks;\n\t\tprivate final List<M> out;\n\t\tprivate Role role = Role.User;\n\t\tprivate StringBuilder buffer = new StringBuilder();"
        - id: SessionProcessor.Run.Run
          type: constructor_declaration
          code: "Run(SessionCallbacks<M> callbacks, List<M> out) {\n\t\t\tthis.callbacks = callbacks;\n\t\t\tthis.out = out;\n\t\t}"
        - id: SessionProcessor.Run.run
          type: method_declaration
          code: "void run(String text, List<Path> chain) {\n\t\t\tString[] lines = text == null ? new String[0] : text.split(\"\\n\", -1);\n\t\t\tint i = 0;\n\t\t\twhile (i < lines.length) {\n\t\t\t\tString line = lines[i];\n\t\t\t\tString stripped = line.strip();\n\n\t\t\t\tOptional<Command> command = CommandHandler.detect(line);\n\t\t\t\tif (command.isPresent()) {\n\t\t\t\t\tswitch (command.get().processorAction()) {\n\t\t\t\t\tcase IGNORE_BLOCK: {\n\t\t\t\t\t\tflush();\n\t\t\t\t\t\tint[] range = fenceRange(lines, i + 1);\n\t\t\t\t\t\ti = range != null ? range[1] + 1 : i + 1;\n\t\t\t\t\t\tcontinue;\n\t\t\t\t\t}\n\t\t\t\t\tcase TRANSFORM: {\n\t\t\t\t\t\tflush();\n\t\t\t\t\t\tString rest = command.get().parameter(2);\n\t\t\t\t\t\tif (rest != null && !rest.isBlank()) {\n\t\t\t\t\t\t\tif (buffer.length() > 0)\n\t\t\t\t\t\t\t\tbuffer.append(\"\\n\");\n\t\t\t\t\t\t\tbuffer.append(rest);\n\t\t\t\t\t\t}\n\t\t\t\t\t\ti++;\n\t\t\t\t\t\tcontinue;\n\t\t\t\t\t}\n\t\t\t\t\tcase REMOVE: {\n\t\t\t\t\t\tflush();\n\t\t\t\t\t\ti++;\n\t\t\t\t\t\tcontinue;\n\t\t\t\t\t}\n\t\t\t\t\tdefault:\n\t\t\t\t\t\tbreak;\n\t\t\t\t\t}\n\t\t\t\t}\n\n\t\t\t\tif (INCLUDE_LINE.matcher(line).matches()) {\n\t\t\t\t\tflush();\n\t\t\t\t\tMatcher m = INCLUDE_TAG.matcher(line);\n\t\t\t\t\twhile (m.find())\n\t\t\t\t\t\tinclude(m.group(1), m.group(2).strip(), chain);\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.USER)) {\n\t\t\t\t\tflush();\n\t\t\t\t\trole = Role.User;\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.AGENT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\trole = Role.Agent;\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.THINKING)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeReasoning(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.TEXT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeText(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.TOOLUSE)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeToolCall(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.TOOLRESULT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeToolResult(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\n\t\t\t\tif (buffer.length() > 0)\n\t\t\t\t\tbuffer.append(\"\\n\");\n\t\t\t\tbuffer.append(line);\n\t\t\t\ti++;\n\t\t\t}\n\t\t\tflush();\n\t\t}"
        - id: SessionProcessor.Run.flush
          type: method_declaration
          code: "private void flush() {\n\t\t\tString text = buffer.toString().strip();\n\t\t\tbuffer = new StringBuilder();\n\t\t\tif (!text.isEmpty())\n\t\t\t\tout.add(callbacks.message(role, text));\n\t\t}"
        - id: SessionProcessor.Run.isMarkerLine
          type: method_declaration
          code: "private boolean isMarkerLine(String line) {\n\t\t\tString s = line.strip();\n\t\t\treturn INCLUDE_LINE.matcher(line).matches() //\n\t\t\t\t\t|| s.equals(EditorInterface.USER) || s.equals(EditorInterface.AGENT) //\n\t\t\t\t\t|| s.equals(EditorInterface.THINKING) || s.equals(EditorInterface.TEXT) //\n\t\t\t\t\t|| s.equals(EditorInterface.TOOLUSE) || s.equals(EditorInterface.TOOLRESULT) //\n\t\t\t\t\t|| CommandHandler.detect(s).isPresent();\n\t\t}"
        - id: SessionProcessor.Run.consumeReasoning
          type: method_declaration
          code: "private int consumeReasoning(String[] lines, int start) {\n\t\t\tStringBuilder body = new StringBuilder();\n\t\t\tint i = start;\n\t\t\twhile (i < lines.length && !isMarkerLine(lines[i])) {\n\t\t\t\tif (body.length() > 0)\n\t\t\t\t\tbody.append(\"\\n\");\n\t\t\t\tbody.append(lines[i]);\n\t\t\t\ti++;\n\t\t\t}\n\t\t\tString text = body.toString().strip();\n\t\t\tif (!text.isEmpty())\n\t\t\t\tout.add(callbacks.reasoning(role, text));\n\t\t\treturn i;\n\t\t}"
        - id: SessionProcessor.Run.consumeText
          type: method_declaration
          code: "private int consumeText(String[] lines, int start) {\n\t\t\tStringBuilder body = new StringBuilder();\n\t\t\tint i = start;\n\t\t\twhile (i < lines.length && !isMarkerLine(lines[i])) {\n\t\t\t\tif (body.length() > 0)\n\t\t\t\t\tbody.append(\"\\n\");\n\t\t\t\tbody.append(lines[i]);\n\t\t\t\ti++;\n\t\t\t}\n\t\t\tString text = body.toString().strip();\n\t\t\tif (!text.isEmpty())\n\t\t\t\tout.add(callbacks.message(role, text));\n\t\t\treturn i;\n\t\t}"
        - id: SessionProcessor.Run.consumeToolCall
          type: method_declaration
          code: "private int consumeToolCall(String[] lines, int start) {\n\t\t\tint[] range = fenceRange(lines, start);\n\t\t\tif (range == null)\n\t\t\t\treturn start;\n\t\t\tJsonNode node = readYaml(lines, range);\n\t\t\tout.add(callbacks.toolCall(\n\t\t\t\t\tnew ToolCall(node.path(\"id\").asText(\"\"), node.path(\"tool\").asText(\"\"), node.path(\"arguments\"))));\n\t\t\treturn range[1] + 1;\n\t\t}"
        - id: SessionProcessor.Run.consumeToolResult
          type: method_declaration
          code: "private int consumeToolResult(String[] lines, int start) {\n\t\t\tint[] range = fenceRange(lines, start);\n\t\t\tif (range == null)\n\t\t\t\treturn start;\n\t\t\tJsonNode node = readYaml(lines, range);\n\t\t\tout.add(callbacks.toolResult(new ToolResult(node.path(\"id\").asText(\"\"), node.path(\"result\").asText(\"\"))));\n\t\t\treturn range[1] + 1;\n\t\t}"
        - id: SessionProcessor.Run.5LYnz5|7Jclc5
          type: statements
          code: "/** Finds the fenced ``` block immediately following a marker line; returns [startLine, endLine] (both fence delimiters), or null. */"
        - id: SessionProcessor.Run.fenceRange
          type: method_declaration
          code: "private int[] fenceRange(String[] lines, int from) {\n\t\t\tint i = from;\n\t\t\twhile (i < lines.length && lines[i].isBlank())\n\t\t\t\ti++;\n\t\t\tif (i >= lines.length || !lines[i].strip().startsWith(\"```\"))\n\t\t\t\treturn null;\n\t\t\tint start = i;\n\t\t\ti++;\n\t\t\twhile (i < lines.length && !lines[i].strip().equals(\"```\"))\n\t\t\t\ti++;\n\t\t\tif (i >= lines.length)\n\t\t\t\treturn null;\n\t\t\treturn new int[] { start, i };\n\t\t}"
        - id: SessionProcessor.Run.readYaml
          type: method_declaration
          code: "private JsonNode readYaml(String[] lines, int[] range) {\n\t\t\tStringBuilder body = new StringBuilder();\n\t\t\tfor (int j = range[0] + 1; j < range[1]; j++)\n\t\t\t\tbody.append(lines[j]).append(\"\\n\");\n\t\t\ttry {\n\t\t\t\tJsonNode node = yaml.readTree(body.toString());\n\t\t\t\treturn node == null ? NullNode.getInstance() : node;\n\t\t\t} catch (Exception e) {\n\t\t\t\tthrow new IllegalStateException(\"Invalid YAML block: \" + e.getMessage(), e);\n\t\t\t}\n\t\t}"
        - id: SessionProcessor.Run.include
          type: method_declaration
          code: "private void include(String kind, String arg, List<Path> chain) {\n\t\t\tif (kind == null) {\n\t\t\t\tincludePath(arg, chain);\n\t\t\t\treturn;\n\t\t\t}\n\t\t\tswitch (kind) {\n\t\t\tcase \"contextprompt\":\n\t\t\t\tincludeContextPrompt(arg);\n\t\t\t\treturn;\n\t\t\tcase \"files\":\n\t\t\t\tincludeFiles(arg);\n\t\t\t\treturn;\n\t\t\tcase \"file\":\n\t\t\t\tincludeFile(arg);\n\t\t\t\treturn;\n\t\t\tcase \"search\":\n\t\t\t\tincludeSearch(arg);\n\t\t\t\treturn;\n\t\t\tdefault:\n\t\t\t\tthrow new IllegalStateException(\"Unknown include kind: \" + kind);\n\t\t\t}\n\t\t}"
        - id: SessionProcessor.Run.0CpLmU|lM1IBJ
          type: statements
          code: "/** Generic {@code [include](path)}: parses the target file as a nested, independent document. */"
        - id: SessionProcessor.Run.includePath
          type: method_declaration
          code: "private void includePath(String pathText, List<Path> chain) {\n\t\t\tPath path = Paths.get(pathText).toAbsolutePath().normalize();\n\t\t\tif (chain.contains(path))\n\t\t\t\tthrow new IncludeCycleException(path.toString());\n\t\t\tString content;\n\t\t\ttry {\n\t\t\t\tcontent = Files.readString(path);\n\t\t\t} catch (IOException e) {\n\t\t\t\tthrow new IllegalStateException(\"Could not read include target: \" + path, e);\n\t\t\t}\n\t\t\tList<Path> nested = new ArrayList<>(chain);\n\t\t\tnested.add(path);\n\t\t\tnew Run<>(callbacks, out).run(content, nested);\n\t\t}"
        - id: SessionProcessor.Run.includeContextPrompt
          type: method_declaration
          code: "private void includeContextPrompt(String dir) {\n\t\t\tString text = requireAdapter().contextPrompt(dir);\n\t\t\tif (text != null && !text.isBlank())\n\t\t\t\tout.add(callbacks.message(Role.User, text.strip()));\n\t\t}"
        - id: SessionProcessor.Run.includeFiles
          type: method_declaration
          code: "private void includeFiles(String arg) {\n\t\t\tfor (IIncludeAdapter.Entry entry : requireAdapter().files(arg))\n\t\t\t\tout.add(callbacks.toolResult(new ToolResult(entry.id, entry.content)));\n\t\t}"
        - id: SessionProcessor.Run.includeFile
          type: method_declaration
          code: "private void includeFile(String path) {\n\t\t\tString content = requireAdapter().file(path);\n\t\t\tif (content != null)\n\t\t\t\tout.add(callbacks.toolResult(new ToolResult(path, content)));\n\t\t}"
        - id: SessionProcessor.Run.includeSearch
          type: method_declaration
          code: "private void includeSearch(String arg) {\n\t\t\tIIncludeAdapter a = requireAdapter();\n\t\t\tif (\"files\".equals(arg)) {\n\t\t\t\tfor (IIncludeAdapter.Entry entry : a.searchFiles())\n\t\t\t\t\tout.add(callbacks.toolResult(new ToolResult(entry.id, entry.content)));\n\t\t\t} else if (\"matches\".equals(arg)) {\n\t\t\t\tList<IIncludeAdapter.Match> matches = a.searchMatches();\n\t\t\t\tout.add(callbacks.toolResult(new ToolResult(\"search-matches\", renderMatches(matches))));\n\t\t\t} else\n\t\t\t\tthrow new IllegalStateException(\"Unknown search include argument: \" + arg);\n\t\t}"
        - id: SessionProcessor.Run.renderMatches
          type: method_declaration
          code: "private String renderMatches(List<IIncludeAdapter.Match> matches) {\n\t\t\tcom.fasterxml.jackson.databind.node.ArrayNode arr = com.fasterxml.jackson.databind.node.JsonNodeFactory.instance\n\t\t\t\t\t.arrayNode();\n\t\t\tfor (IIncludeAdapter.Match match : matches) {\n\t\t\t\tcom.fasterxml.jackson.databind.node.ObjectNode node = arr.addObject();\n\t\t\t\tnode.put(\"file\", match.file);\n\t\t\t\tnode.put(\"line\", match.line);\n\t\t\t\tnode.put(\"text\", match.text);\n\t\t\t}\n\t\t\treturn yaml.toYaml(arr);\n\t\t}"
        - id: SessionProcessor.Run.requireAdapter
          type: method_declaration
          code: "private IIncludeAdapter requireAdapter() {\n\t\t\tif (adapter == null)\n\t\t\t\tthrow new IllegalStateException(\"No IIncludeAdapter configured for this SessionProcessor\");\n\t\t\treturn adapter;\n\t\t}"
```
/answer 8c1a-8e allow
ReasoningToken: 448
Control Request:
```yaml
id: 8c1a-8f
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: CallCommand
    include:
    - '*.java'
  reason: "Prüfen, ob CallCommand im CommandHandler registriert ist"
```
/answer 8c1a-8f allow
Control Request:
```yaml
id: 8c1a-90
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/connector/CallCommand.java
        matches:
        - lineno: 6
          match: "public class CallCommand extends Command {"
        - lineno: 8
          match: "public CallCommand() {"
        - lineno: 24
          match: return new CallCommand();
      - path: xy/ai/workbench/connector/harness/PromptHandler.java
        matches:
        - lineno: 37
          match: import xy.ai.workbench.connector.CallCommand;
        - lineno: 299
          match: "if (detected[0] instanceof CallCommand)"
        - lineno: 374
          match: "if (detected[0] instanceof CallCommand)"
      - path: xy/ai/workbench/connector/CommandHandler.java
        matches:
        - lineno: 12
          match: "new ResumeCommand(), new ExitCommand(), new ToolCommand(), new CallCommand());"
```
/answer 8c1a-90 allow
ReasoningToken: 191
Text:
 Gut, CallCommand ist bereits registriert und SessionProcessor handhabt es via generischen REMOVE-Pfad und `isMarkerLine`. Step 04 ist also bereits erledigt. Jetzt fehlen: (1) `Tool:` + `/call` Wrapping in `renderSchema`, (2) MCPConnector nutzt den getypten `Prompt.command`/`yamlBlock` statt reinem String-Parsing im Prompt-Pfad.
Control Request:
```yaml
id: 8c1a-91
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
    ids: []
  reason: MCPRequest-Konstruktor prüfen
```
/answer 8c1a-91 allow
Control Request:
```yaml
id: 8c1a-92
result:
  structuredContent:
    results:
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
/answer 8c1a-92 allow
Control Request:
```yaml
id: 8c1a-93
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPRequest.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/CommandHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/ToolCommand.java
  reason: Zusätzliche Referenzdateien inspizieren
```
/answer 8c1a-93 allow
Control Request:
```yaml
id: 8c1a-94
result:
  structuredContent:
    results:
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
/answer 8c1a-94 allow
Control Request:
```yaml
id: 8c1a-95
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
    ids:
    - fXSZh1|rhl4eY
    - txfzo4|udCxJD
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    ids:
    - 7MAfda|VWDx0H
    - uzcHjn|BNLYpl
  reason: "Imports der beiden Dateien einsehen, um konsistent zu ergänzen"
```
/answer 8c1a-95 allow
Control Request:
```yaml
id: 8c1a-96
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
      nodes:
      - id: fXSZh1|rhl4eY
        type: imports
        code: |-
          package xy.ai.workbench.connector.mcp;

          import java.util.HashSet;
          import java.util.Iterator;
          import java.util.Map;
          import java.util.Set;

          import com.fasterxml.jackson.core.JsonProcessingException;
          import com.fasterxml.jackson.databind.JsonNode;
          import com.fasterxml.jackson.databind.node.ObjectNode;

          import xy.ai.workbench.connector.claudecode.JsonUtil;
          import xy.ai.workbench.connector.claudecode.YamlRenderer;
      - id: txfzo4|udCxJD
        type: statements
        code: |-
          /**
           * Renders tool schemas as fillable YAML templates and turns a submitted
           * template back into a tool call. Results are returned as pretty JSON, giving
           * the user a maximal prefill using the same tools the agent later sees.
           */
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
      - id: uzcHjn|BNLYpl
        type: imports
        code: |-
          import xy.ai.workbench.connector.harness.Prompt;
          import xy.ai.workbench.models.AIAnswer;
```
/answer 8c1a-96 allow
ReasoningToken: 7177
Text:
 Jetzt die Änderungen umsetzen.
Control Request:
```yaml
id: 8c1a-97
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
    old_text: |-
      import xy.ai.workbench.connector.claudecode.JsonUtil;
      import xy.ai.workbench.connector.claudecode.YamlRenderer;
    new_text: |-
      import xy.ai.workbench.EditorInterface;
      import xy.ai.workbench.connector.claudecode.JsonUtil;
      import xy.ai.workbench.connector.claudecode.YamlRenderer;
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
    old_text: "StringBuilder sb = new StringBuilder();\n\t\tsb.append(\"```yaml\\n\");\n\t\tsb.append(\"tool: \")"
    new_text: "StringBuilder sb = new StringBuilder();\n\t\tsb.append(EditorInterface.TOOLUSE).append(\"\\n\");\n\t\tsb.append(\"```yaml\\n\");\n\t\tsb.append(\"tool: \")"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
    old_text: "\t\tsb.append(\"```\");\n\t\treturn sb.toString();\n\t}"
    new_text: "\t\tsb.append(\"```\\n\");\n\t\tsb.append(EditorInterface.CMD_CALL);\n\t\treturn sb.toString();\n\t}"
  reason: renderSchema um Tool:-Präfix und /call-Kommandozeile ergänzen
```
/answer 8c1a-97 allow
Control Request:
```yaml
id: 8c1a-98
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
    old_text: "Prompt, Tool, Exit;"
    new_text: "Prompt, Tool, Exit, Call;"
  reason: Neuen Command-Type Call für getypte /call-Verarbeitung ergänzen
```
/answer 8c1a-98 allow
ReasoningToken: 17
Control Request:
```yaml
id: 8c1a-99
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    id: MCPConnector.createRequest
    source: "@Override\n\tpublic MCPRequest createRequest(Prompt prompt, IProgressMonitor mon) {\n\t\tMCPCommand cmd = prompt.command == Prompt.Command.CALL ? new MCPCommand(MCPCommandType.Call, prompt.yamlBlock)\n\t\t\t\t: preprocess(prompt.inputs);\n\t\treturn new MCPRequest(UUID.randomUUID().toString(), prompt.config.systemPrompt, prompt.config.tools, cmd);\n\t}"
  reason: createRequest nutzt den bereits von PromptHandler getypten /call-Block statt erneutem String-Scan
```
/answer 8c1a-99 allow
Text:
 Jetzt `MCPConnector.createRequest` und `executeRequest` anpassen.
Control Request:
```yaml
id: 8c1a-9a
toolName: ast_replace
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    id: MCPConnector.executeRequest
    source: "@Override\n\tpublic MCPResponse executeRequest(MCPRequest req, IProgressMonitor mon, Job job) {\n\t\tswitch (req.cmd.type) {\n\t\tcase Exit:\n\t\t\tclient.close();\n\t\t\treturn new MCPResponse(req.id, \"MCP connection closed\");\n\t\tcase Tool: {\n\t\t\tJsonNode tool = client.findTool(req.cmd.parameter);\n\t\t\tif (tool == null)\n\t\t\t\tthrow new IllegalArgumentException(\"Unknown tool: \" + req.cmd.parameter);\n\t\t\treturn new MCPResponse(req.id, control.renderSchema(tool));\n\t\t}\n\t\tcase Call: {\n\t\t\t// Already typed/extracted by PromptHandler (Step 08): just parse and invoke.\n\t\t\tif (req.cmd.parameter == null || req.cmd.parameter.isBlank())\n\t\t\t\tthrow new IllegalArgumentException(\"No preceding ```yaml tool call block found before /call\");\n\t\t\treturn invokeCall(req.id, control.parseYaml(req.cmd.parameter));\n\t\t}\n\t\tcase Prompt: {\n\t\t\tString block = control.extractYamlBlock(req.cmd.parameter);\n\t\t\tif (block == null)\n\t\t\t\tthrow new IllegalArgumentException(\n\t\t\t\t\t\t\"No ```yaml tool call block found. Use \\\"/tool <id>\\\" to get a template.\");\n\t\t\treturn invokeCall(req.id, control.parseYaml(block));\n\t\t}\n\t\tdefault:\n\t\t\tthrow new UnsupportedOperationException(\"Unsupported command: \" + req.cmd.type);\n\t\t}\n\t}"
  reason: executeRequest um typisierten Call-Pfad (Step 08) erweitern und mit Prompt-Pfad gemeinsamen Invoke teilen
```
/answer 8c1a-9a allow
Control Request:
```yaml
id: 8c1a-9b
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
    id: MCPConnector.executeRequest
    position: after
    source: "\n\tprivate MCPResponse invokeCall(String id, JsonNode call) {\n\t\tString name = call.path(\"tool\").asText(null);\n\t\tif (name == null || name.isBlank())\n\t\t\tthrow new IllegalArgumentException(\"Tool call is missing the 'tool' field\");\n\t\tJsonNode tool = client.findTool(name);\n\t\tJsonNode arguments = control.fillReason(tool, call.path(\"arguments\"));\n\t\tJsonNode result = client.callTool(name, arguments);\n\t\treturn new MCPResponse(id, control.prettyResult(result));\n\t}"
  reason: Gemeinsame Call-Logik für Prompt- und Call-Pfad extrahieren
```
/answer 8c1a-9b allow
Control Request:
```yaml
id: 8c1a-9c
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPControlClient.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPConnector.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPCommandType.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
  reason: Alle in Step 08 berührten/relevanten Dateien kompilieren lassen
```
/answer 8c1a-9c allow
Text:
 Jetzt `invokeCall` als Hilfsmethode ergänzen.
Text:
 Jetzt validieren.
Result Stats:  id=9a02c09e-6eff-4620-9b53-430d9b29fbc7, total: 0, in: 159, out: 14348, reason: 0, read: 605647, write: 64374
Alle Dateien kompilieren erfolgreich. Step 08 ist damit umgesetzt:

- `MCPControlClient.renderSchema` liefert nun `Tool:` + yaml-Block + `/call`.
- `MCPCommandType` um `Call` erweitert.
- `MCPConnector.createRequest` nutzt bei `Prompt.Command.CALL` direkt den von `PromptHandler` bereits erfassten `yamlBlock` (kein erneutes String-Scannen).
- `MCPConnector.executeRequest` hat einen eigenen `Call`-Zweig (nur `parseYaml`, keine Fence-Suche) und teilt sich die Ausführung mit dem bestehenden `Prompt`-Zweig über `invokeCall`.
- `PromptHandler`, `SessionProcessor`, `CallCommand`, `EditorInterface` waren bereits aus früheren Steps korrekt vorbereitet (Erkennung, Rückwärtssuche des yaml-Blocks, `REMOVE`-Verhalten inkl. `isMarkerLine`).
