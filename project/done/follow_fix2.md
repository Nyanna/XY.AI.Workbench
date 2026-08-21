Die `moveCursorToLastLineStart()` in xy.ai.workbench.marker.MarkerRessourceScanner.moveCursorToLastLineStart(ITextEditor, IDocument) ist unzuverlässig. Vielleicht gibt es einen retry Ansatz und der Erfolg lässt sich überprüfen, vielleicht kombiniert in einer schleife mit einem zunehmenden Fenster bis Erfolgreich oder die Zeit überschritten.

- Einstieg: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java`
SystemInit:  id=3a52cade-4f12-40a5-aa27-ddf70f2f9a01, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 5e4a-39
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  reason: Analyse der aktuellen Implementierung von moveCursorToLastLineStart und des Kontexts
```
/answer 5e4a-39 allow
ReasoningToken: 9
Control Request:
```yaml
id: 5e4a-3a
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  old_text: "\tprivate void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\tDisplay.getDefault().timerExec(200, () -> {\n\t\t\teditor.selectAndReveal(0, 0);\n\t\t\teditor.selectAndReveal(doc.getLength(), 0);\n\t\t});\n\t\tDisplay.getDefault().timerExec(500, () -> {\n\t\t\teditor.selectAndReveal(doc.getLength(), 0);\n\t\t});\n\t}"
  new_text: "\t/** Initial retry delay in ms, doubled after every failed attempt. */\n\tprivate static final int CURSOR_MOVE_INITIAL_DELAY_MS = 100;\n\t/** Upper bound for the (doubling) retry delay. */\n\tprivate static final int CURSOR_MOVE_MAX_DELAY_MS = 800;\n\t/** Overall time budget after which retries are given up. */\n\tprivate static final long CURSOR_MOVE_TIMEOUT_MS = 4000;\n\n\t/**\n\t * Moves the cursor to the end of the doc (start of the last line, i.e. an\n\t * empty selection at the doc's end). {@link ITextEditor#selectAndReveal}\n\t * is unreliable right after a doc replace (e.g. because the widget/editor\n\t * is not yet fully laid out), so the move is retried with an increasing\n\t * delay between attempts until it can be verified to have succeeded or a\n\t * timeout is hit.\n\t */\n\tprivate void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\tscheduleMoveCursorAttempt(editor, doc, System.currentTimeMillis(), CURSOR_MOVE_INITIAL_DELAY_MS);\n\t}\n\n\tprivate void scheduleMoveCursorAttempt(ITextEditor editor, IDocument doc, long startTime, int delayMs) {\n\t\tDisplay.getDefault().timerExec(delayMs, () -> {\n\t\t\tif (tryMoveCursorToLastLineStart(editor, doc))\n\t\t\t\treturn;\n\n\t\t\tlong elapsed = System.currentTimeMillis() - startTime;\n\t\t\tif (elapsed >= CURSOR_MOVE_TIMEOUT_MS) {\n\t\t\t\tLOG.error(\"moveCursorToLastLineStart: giving up after \" + elapsed + \"ms\", null);\n\t\t\t\treturn;\n\t\t\t}\n\n\t\t\tint nextDelay = Math.min(delayMs * 2, CURSOR_MOVE_MAX_DELAY_MS);\n\t\t\tscheduleMoveCursorAttempt(editor, doc, startTime, nextDelay);\n\t\t});\n\t}\n\n\t/**\n\t * Attempts to place the cursor at the end of the doc and verifies that the\n\t * editor's selection actually reflects the requested position afterwards.\n\t *\n\t * @return {@code true} when the move could be verified to have succeeded.\n\t */\n\tprivate boolean tryMoveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\ttry {\n\t\t\tint targetOffset = doc.getLength();\n\t\t\teditor.selectAndReveal(0, 0);\n\t\t\teditor.selectAndReveal(targetOffset, 0);\n\n\t\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\t\t\tif (!(selection instanceof ITextSelection))\n\t\t\t\treturn false;\n\n\t\t\tITextSelection tsel = (ITextSelection) selection;\n\t\t\treturn tsel.getOffset() == targetOffset && tsel.getLength() == 0;\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t\treturn false;\n\t\t}\n\t}"
  reason: Retry-Mechanismus mit zunehmendem Delay und Erfolgsverifikation für moveCursorToLastLineStart implementieren
```
/answer 5e4a-3a allow
Control Request:
```yaml
id: 5e4a-3b
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -n "public static void error" -r src/xy/ai/workbench/LOG.java
  reason: "Signatur von LOG.error prüfen, um Kompatibilität des neuen Codes sicherzustellen"
```
/answer 5e4a-3b allow
Control Request:
```yaml
id: 5e4a-3c
result:
  structuredContent:
    stdout: "18:\tpublic static void error(String message) {\n22:\tpublic static void error(String message, Throwable throwable) {\n"
```
/answer 5e4a-3c allow
ReasoningToken: 940
ReasoningToken: 43
Passt, `LOG.error(String, Throwable)` mit `null` als Throwable ist gültig. Damit ist die Änderung konsistent.

Kurze Zusammenfassung der Änderung in `MarkerRessourceScanner.java`:

- **`moveCursorToLastLineStart(editor, doc)`**: startet nun einen Retry-Zyklus statt zweier fest verdrahteter `timerExec`-Aufrufe.
- **`scheduleMoveCursorAttempt(...)`**: plant den nächsten Versuch via `Display.timerExec`, verdoppelt die Wartezeit bei jedem Fehlschlag (`100ms → 800ms` Obergrenze) und bricht nach einem Gesamt-Timeout von `4000ms` ab (mit Log-Meldung).
- **`tryMoveCursorToLastLineStart(editor, doc)`**: führt den eigentlichen `selectAndReveal`-Versuch aus und verifiziert den Erfolg, indem die aktuelle `ITextSelection` des Editors ausgelesen und mit dem Zieloffset (`doc.getLength()`, leere Selektion) verglichen wird. Nur bei Übereinstimmung gilt der Versuch als erfolgreich und die Retry-Schleife endet.

Damit wird die Unzuverlässigkeit durch aktive Erfolgskontrolle plus exponentiell wachsendes Retry-Fenster bis zum Timeout adressiert, ohne die restliche Klasse zu verändern.
DU kannst `editor.selectAndReveal(0, 0)` gerne entfernen, das war der schlechte Versuch eine fixes
SystemInit:  id=3a52cade-4f12-40a5-aa27-ddf70f2f9a01, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 5e4a-3d
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  old_text: "\t\t\tint targetOffset = doc.getLength();\n\t\t\teditor.selectAndReveal(0, 0);\n\t\t\teditor.selectAndReveal(targetOffset, 0);"
  new_text: "\t\t\tint targetOffset = doc.getLength();\n\t\t\teditor.selectAndReveal(targetOffset, 0);"
  reason: "Entfernen des überflüssigen selectAndReveal(0,0)-Workarounds auf Wunsch des Users"
```
/answer 5e4a-3d allow
ReasoningToken: 42
Entfernt. `tryMoveCursorToLastLineStart` ruft jetzt nur noch direkt `editor.selectAndReveal(targetOffset, 0)` auf, ohne den vorherigen `selectAndReveal(0, 0)`-Umweg.
