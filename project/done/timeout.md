Die Anzeige des State "Open" in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java` soll sich ändern. Im State Open soll nur noch ein Counter (Minuten und Sekunden) die verbleibende Gültigkeitsdauer Anzeigen. Dies bedingt, dass das Panel nun sekündlich aktualisiert wird.
Zudem soll die Autoselektion geändert werden und nur gültige Session berücksichtigen. Ist Sync aktiviert soll die Selektion automatisch auf "CREATE" zurückfallen, wenn die Session abläuft.
SystemInit:  id=3f26989e-b1e0-443b-91d3-a579977c673f, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: ccee601a-2ed0-423b-b015-924516c08013
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  reason: Datei analysieren
```
/answer ccee601a-2ed0-423b-b015-924516c08013 allow
ReasoningToken: 11
Control Request:
```yaml
id: 6fe303f0-bc69-4012-87ec-e66d968b8b62
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
  reason: Verzeichnis der Session-Klassen anzeigen
```
/answer 6fe303f0-bc69-4012-87ec-e66d968b8b62 allow
Control Request:
```yaml
id: 9a07265e-e31b-4463-a4f9-c551c26f4655
result:
  structuredContent:
    entries:
    - CCConnector.java
    - CCControlClient.java
    - CCRequest.java
    - CCRequestBuilder.java
    - CCResponse.java
    - CCSession.java
    - CCSessionManager.java
    - CommandType.java
    - EditorLocation.java
    - HookParser.java
    - JsonUtil.java
    - ProtocolParser.java
    - SessionParameters.java
    - SessionState.java
    - TimedLineReader.java
    - YamlRenderer.java
```
/answer 9a07265e-e31b-4463-a4f9-c551c26f4655 allow
ReasoningToken: 36
Control Request:
```yaml
id: 99a806b6-de15-4ce7-be6c-ca07a3a9a380
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
  reason: Felder für TTL/Open-State prüfen
```
/answer 99a806b6-de15-4ce7-be6c-ca07a3a9a380 allow
Control Request:
```yaml
id: 326fd235-30b7-484f-b34a-f3b5abf484e0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/SessionState.java
  reason: States prüfen
```
/answer 326fd235-30b7-484f-b34a-f3b5abf484e0 allow
Control Request:
```yaml
id: 8316f7f1-302c-4930-a8b1-8bdf3702297e
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCSession.java
  old_text: "\tpublic SessionState getState() {\n\t\tif (inPrompt)\n\t\t\treturn SessionState.Prompting;\n\t\tif (isExpired())\n\t\t\treturn SessionState.Expired;\n\t\tif (isProcessAlive())\n\t\t\treturn SessionState.Open;\n\t\treturn SessionState.Created;\n\t}"
  new_text: "\tpublic SessionState getState() {\n\t\tif (inPrompt)\n\t\t\treturn SessionState.Prompting;\n\t\tif (isExpired())\n\t\t\treturn SessionState.Expired;\n\t\tif (isProcessAlive())\n\t\t\treturn SessionState.Open;\n\t\treturn SessionState.Created;\n\t}\n\n\t/**\n\t * @return the number of whole seconds until this session's TTL runs out, or\n\t *         {@code -1} if no prompt has been sent yet (TTL not started).\n\t */\n\tpublic long getRemainingTtlSeconds() {\n\t\tif (lastSentAt == null)\n\t\t\treturn -1;\n\t\tlong elapsed = ChronoUnit.SECONDS.between(lastSentAt, Instant.now());\n\n\t\tlong totalSeconds = 60 * 60;\n\t\tif (parameters.cacheMode != null)\n\t\t\tswitch (parameters.cacheMode) {\n\t\t\tcase Disabled:\n\t\t\t\treturn 0L;\n\t\t\tcase Minutes_5:\n\t\t\t\ttotalSeconds = 5 * 60;\n\t\t\t\tbreak;\n\t\t\tcase Default:\n\t\t\tcase Hours_1:\n\t\t\t}\n\t\treturn Math.max(0, totalSeconds - elapsed);\n\t}\n\n\t/**\n\t * @return {@code true} unless the session is {@link SessionState#Expired}.\n\t *         Used to filter out no-longer-usable sessions from automatic\n\t *         selection logic.\n\t */\n\tpublic boolean isValid() {\n\t\treturn getState() != SessionState.Expired;\n\t}"
  reason: TTL-Sekunden-Berechnung und isValid() ergänzen
```
/answer 8316f7f1-302c-4930-a8b1-8bdf3702297e allow
ReasoningToken: 3206
Text:
 Jetzt die View anpassen:
Control Request:
```yaml
id: 683984bd-e5d8-4316-93a2-3d731c42d67b
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  old_text: " * <p>\n * The view registers a change listener with the {@link CCSessionManager} and\n * refreshes the table on any session state change. A periodic timer refreshes\n * the TTL column every 30 seconds even when no prompt is active.\n * </p>\n */\npublic class ClaudeCodeSessionView extends ViewPart {\n\n\t/** The ID used in plugin.xml. */\n\tpublic static final String ID = \"xy.ai.workbench.views.ClaudeCodeSessionView\";\n\n\t/** Periodic TTL refresh interval in milliseconds. */\n\tprivate static final int TTL_REFRESH_INTERVAL_MS = 30_000;"
  new_text: " * <p>\n * The view registers a change listener with the {@link CCSessionManager} and\n * refreshes the table on any session state change. A periodic timer refreshes\n * the table every second so that the mm:ss countdown shown for\n * {@link SessionState#Open} sessions stays accurate, and so that the\n * selection can automatically fall back to \"Create new session\" once a\n * synced session expires.\n * </p>\n */\npublic class ClaudeCodeSessionView extends ViewPart {\n\n\t/** The ID used in plugin.xml. */\n\tpublic static final String ID = \"xy.ai.workbench.views.ClaudeCodeSessionView\";\n\n\t/** Periodic TTL refresh interval in milliseconds. */\n\tprivate static final int TTL_REFRESH_INTERVAL_MS = 1_000;"
  reason: JavaDoc und Refresh-Intervall anpassen
```
/answer 683984bd-e5d8-4316-93a2-3d731c42d67b allow
Control Request:
```yaml
id: 0773ee26-3d8b-4f7a-9aab-bfd556a6c7b4
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  old_text: "\tprivate String stateLabel(CCSession s) {\n\t\tif (SessionState.Prompting.equals(s.getState()) && s.getLastSentAt() != null)\n\t\t\treturn s.getState().name() + \" (\" + (Instant.now().toEpochMilli() - s.getLastSentAt().toEpochMilli()) + \")\";\n\t\treturn s.getState().name();\n\t}"
  new_text: "\tprivate String stateLabel(CCSession s) {\n\t\tif (SessionState.Prompting.equals(s.getState()) && s.getLastSentAt() != null)\n\t\t\treturn s.getState().name() + \" (\" + (Instant.now().toEpochMilli() - s.getLastSentAt().toEpochMilli()) + \")\";\n\t\tif (SessionState.Open.equals(s.getState()))\n\t\t\treturn formatMMSS(s.getRemainingTtlSeconds());\n\t\treturn s.getState().name();\n\t}\n\n\tprivate static String formatMMSS(long totalSeconds) {\n\t\tif (totalSeconds < 0)\n\t\t\treturn \"--:--\";\n\t\tlong minutes = totalSeconds / 60;\n\t\tlong seconds = totalSeconds % 60;\n\t\treturn String.format(\"%02d:%02d\", minutes, seconds);\n\t}"
  reason: State-Label auf mm:ss-Counter im Open-State umstellen
```
/answer 0773ee26-3d8b-4f7a-9aab-bfd556a6c7b4 allow
Control Request:
```yaml
id: a566701a-a5a5-43aa-8c7f-eabbbff9bc21
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  old_text: "\tprivate CCSession findAssociatedSession(List<CCSession> sessions) {\n\t\tif (currentProjectPath == null)\n\t\t\treturn null;\n\t\tfor (CCSession s : sessions) {\n\t\t\tSessionParameters p = s.getParameters();\n\t\t\tif (p != null && currentProjectPath.equals(p.cwd) && Objects.equals(currentRelativeFilePath, p.filePath))\n\t\t\t\treturn s;\n\t\t}\n\t\treturn null;\n\t}"
  new_text: "\tprivate CCSession findAssociatedSession(List<CCSession> sessions) {\n\t\tif (currentProjectPath == null)\n\t\t\treturn null;\n\t\tfor (CCSession s : sessions) {\n\t\t\tif (!s.isValid())\n\t\t\t\tcontinue;\n\t\t\tSessionParameters p = s.getParameters();\n\t\t\tif (p != null && currentProjectPath.equals(p.cwd) && Objects.equals(currentRelativeFilePath, p.filePath))\n\t\t\t\treturn s;\n\t\t}\n\t\treturn null;\n\t}"
  reason: Autoselektion nur für gültige Sessions
```
/answer a566701a-a5a5-43aa-8c7f-eabbbff9bc21 allow
Control Request:
```yaml
id: 0cb76f7f-0ca5-49f7-861e-541df81d5a1a
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/ClaudeCodeSessionView.java
  old_text: "\t\tknownSessionIds = newIds;\n\n\t\tif (syncAction.isChecked() && allowSyncOnNewSession && !added.isEmpty()) {\n\t\t\tCCSession match = findAssociatedSession(sessions);\n\t\t\tif (match != null && added.contains(match))\n\t\t\t\tselectSession(match);\n\t\t}\n\t}"
  new_text: "\t\tknownSessionIds = newIds;\n\n\t\tif (syncAction.isChecked()) {\n\t\t\tIStructuredSelection currentSel = viewer.getStructuredSelection();\n\t\t\tObject selected = currentSel.getFirstElement();\n\t\t\tif (selected instanceof CCSession && selected != CNEW_LAUDE_CODE_SESSION\n\t\t\t\t\t&& !((CCSession) selected).isValid()) {\n\t\t\t\t// the synced session has expired in the meantime: fall back to \"Create new session\"\n\t\t\t\tselectSession(null);\n\t\t\t} else if (allowSyncOnNewSession && !added.isEmpty()) {\n\t\t\t\tCCSession match = findAssociatedSession(sessions);\n\t\t\t\tif (match != null && added.contains(match))\n\t\t\t\t\tselectSession(match);\n\t\t\t}\n\t\t}\n\t}"
  reason: Auto-Fallback auf CREATE bei ablaufender synchronisierter Session
```
/answer 0cb76f7f-0ca5-49f7-861e-541df81d5a1a allow
Control Request:
```yaml
id: 65101162-5374-4f0a-a782-1c3058d7be5b
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench
  pattern: pom.xml|build.gradle
  reason: Build-System für Kompilierprüfung finden
```
/answer 65101162-5374-4f0a-a782-1c3058d7be5b deny verwende direkt `build.sh` ohne Analyse der Datei
Control Request:
```yaml
id: 4d38370c-31a7-4e75-a48a-787406af2f95
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: ./build.sh
  reason: Projekt kompilieren zur Verifikation der Änderungen
```
/answer 4d38370c-31a7-4e75-a48a-787406af2f95 allow
Control Request:
```yaml
id: d0feb6aa-bdbd-4cd1-afeb-5faf7f9a4f03
result:
  structuredContent:
    stdout: |
      Successfull
```
/answer d0feb6aa-bdbd-4cd1-afeb-5faf7f9a4f03 allow
ReasoningToken: 32
Text:
 Jetzt die Auto-Fallback-Logik in `refreshTable`:
Text:
 Nun kompilieren/prüfen lassen, ob es einen Build-Mechanismus gibt (z.B. Maven/Gradle), um Syntax zu validieren.
Der Build läuft erfolgreich durch. Zusammenfassung der Änderungen:

**`CCSession.java`**
- `getRemainingTtlSeconds()`: liefert die verbleibende Gültigkeitsdauer sekundengenau (analog zu `getRemainingTtlMinutes()`, berücksichtigt `CacheMode`).
- `isValid()`: `true`, solange die Session nicht `Expired` ist.

**`ClaudeCodeSessionView.java`**
- `TTL_REFRESH_INTERVAL_MS` von 30 s auf 1 s reduziert → das Panel aktualisiert sich jetzt sekündlich.
- `stateLabel(...)`: im State `Open` wird nur noch der Countdown im Format `mm:ss` angezeigt (neue Hilfsmethode `formatMMSS`), statt des Textes „Open“.
- `findAssociatedSession(...)`: berücksichtigt bei der Autoselektion nur noch gültige Sessions (`s.isValid()`), abgelaufene Sessions werden übersprungen.
- `refreshTable(...)`: prüft bei aktiviertem Sync bei jedem Refresh (also auch beim sekündlichen Timer-Tick), ob die aktuell ausgewählte Session mittlerweile abgelaufen ist; ist das der Fall, wird die Selektion automatisch auf „Create new session“ (`CREATE`) zurückgesetzt.
