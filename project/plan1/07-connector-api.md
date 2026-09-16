# Step 07 – Connectoren auf Prompt-Objekt umstellen (Phase 6)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
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
