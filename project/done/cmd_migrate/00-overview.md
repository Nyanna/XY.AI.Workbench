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
