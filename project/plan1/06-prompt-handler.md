# Step 06 – PromptHandler (Phase 5)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
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
