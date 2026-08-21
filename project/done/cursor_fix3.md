Die tryMoveCursorToLastLineStart() Methode in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java`. Das Problem ist das asynchrone editieren der Datei. Der Cursor ist zwar am Ende des Dokuments aber ausstehende Änderungen fügen dann noch Content  an und er ist nicht mehr am Ende. Leider ist nicht ganz klar, ob das mit Zeit überbrückbar ist oder ob man ein anderes Prinzip braucht. Hast du eine Idee?
SystemInit:  id=835eb828-1b92-478e-beaf-958bb86db69d, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 4db6-ae
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  reason: Prüfen der tryMoveCursorToLastLineStart() Methode und des Kontexts zum asynchronen Editieren
```
/answer 4db6-ae allow
ReasoningToken: 18
ReasoningToken: 1775
Guter Fund – das Kernproblem ist ein **klassisches Race-Condition-Muster**: Du versuchst mit `tryMoveCursorToLastLineStart()` + Timer-Retry eine bewegte Zielposition ("Dokumentende") zu treffen, aber das Ziel bewegt sich weiter, *nachdem* du erfolgreich verifiziert hast. Das ist mit reiner Zeitverzögerung (`timerExec`, doubling delay, Timeout) grundsätzlich **nicht zuverlässig lösbar** – du kannst beliebig oft "gewinnen" und trotzdem kommt danach noch ein weiteres `doc.replace()` von einem anderen, parallel laufenden `Job`/`AIAnswer`-Callback.

Warum das passiert:
- Jede Antwort (`AIAnswer`) wird in einem eigenen `Job` verarbeitet (`Job.create("Scanning file for markers", ...)` bzw. der Aufrufer von `findAndReplaceMarkers`).
- Jeder Job macht `Display.getDefault().syncExec(...)` mit `doc.replace(...)` gefolgt von einem **entkoppelten** `timerExec`-Retry-Mechanismus für den Cursor.
- Es gibt keine Garantie, dass zwischen "Replace X" und "Cursor-Move für X verifiziert" nicht schon "Replace Y" (nächste Antwort/Chunk) dazwischenfunkt – dein Retry-Loop prüft nur den Moment der Prüfung, nicht was *danach* noch reinkommt.

**Die richtige Lösung ist ein Wechsel von Polling/Retry auf einen reaktiven Ansatz:** Statt zu raten, wann "endlich Ruhe" ist, hängst du einen `IDocumentListener` an das Dokument, der bei *jeder* Änderung reagiert, solange Auto-Follow aktiv ist. Damit verschwindet die Race Condition komplett, weil du nicht mehr "einmalig nach der Änderung" nachziehst, sondern strukturell auf jede künftige Änderung reagierst:

```java
private final Map<IDocument, IDocumentListener> autoFollowListeners = new java.util.HashMap<>();

private void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {
    // sofortiger Versuch
    tryMoveCursorToLastLineStart(editor, doc);

    // und zusätzlich dauerhaft "mitziehen", solange der Nutzer nicht selbst scrollt/klickt
    ensureAutoFollowListener(editor, doc);
}

private void ensureAutoFollowListener(ITextEditor editor, IDocument doc) {
    if (autoFollowListeners.containsKey(doc))
        return; // schon aktiv

    IDocumentListener listener = new IDocumentListener() {
        @Override
        public void documentAboutToBeChanged(DocumentEvent event) {
            // nichts
        }

        @Override
        public void documentChanged(DocumentEvent event) {
            // Nur nachziehen, wenn der Cursor unmittelbar vor der Änderung
            // schon "am Ende" stand (sonst hat der Nutzer selbst navigiert)
            if (!isAutoFollowModeEnabled() || !shouldAutoFollow(editor, doc)) {
                removeAutoFollowListener(doc);
                return;
            }
            tryMoveCursorToLastLineStart(editor, doc);
        }
    };
    doc.addDocumentListener(listener);
    autoFollowListeners.put(doc, listener);
}

private void removeAutoFollowListener(IDocument doc) {
    IDocumentListener l = autoFollowListeners.remove(doc);
    if (l != null)
        doc.removeDocumentListener(l);
}
```

Wichtige Design-Punkte dabei:

1. **Kein Timer, kein Timeout mehr nötig** – jede Änderung am Dokument löst sofort (synchron, im UI-Thread, da `doc.replace` ohnehin in `syncExec` läuft) ein Nachziehen aus. Es gibt also keinen Moment, in dem "Cursor war am Ende, aber neue Änderung kommt noch" unbemerkt bleiben kann.
2. **Abbruchbedingung für Auto-Follow**: Sobald der Nutzer selbst scrollt/klickt (`shouldAutoFollow` liefert `false`, weil die Selektion nicht mehr in der letzten Zeile ist), entfernst du den Listener wieder – sonst reißt du den Nutzer ständig zurück ans Ende, obwohl er gerade woanders liest/editiert.
3. **Lifecycle**: Der Listener sollte auch entfernt werden, wenn der Editor geschlossen wird (`IPartListener2`) oder das `ITextFileBuffer` disconnected wird, um Leaks zu vermeiden. Eine `Map<IDocument, IDocumentListener>` ist ok, aber am besten mit einem `IPartListener2`/`disposeListener` kombinieren, der bei Editor-Close aufräumt.
4. Der bisherige `tryMoveCursorToLastLineStart()` bleibt als Baustein bestehen (er ist ja schon idempotent und prüft nur den *aktuellen* `doc.getLength()`), du entfernst nur `scheduleMoveCursorAttempt`/den Timer-Retry-Mechanismus komplett, weil er durch den Listener obsolet wird.

Kurz gesagt: **Nicht "mit Zeit überbrücken", sondern das Problem umdrehen** – nicht *nachdem* editiert wurde raten, ob noch mehr kommt, sondern *bei jeder* Edit-Operation automatisch reagieren, solange die Auto-Follow-Bedingung erfüllt ist. Das ist robust unabhängig davon, wie viele parallele Jobs/Antworten gleichzeitig ins Dokument schreiben.

Soll ich das direkt so in `MarkerRessourceScanner.java` umsetzen (inkl. Aufräumen der Timer-Konstanten/-Methoden und Hinzufügen eines sauberen Listener-Lifecycles)?
Aber muss der Listener entfernt werden? Reicht es nicht, wenn er die Bedingung prüft?
SystemInit:  id=835eb828-1b92-478e-beaf-958bb86db69d, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 756
Guter Einwand – du hast recht, und das vereinfacht die Sache:

**Für den Fall "Nutzer ist nicht mehr am Ende" brauchst du kein Entfernen.** Wenn `documentChanged` bei *jedem* Aufruf erneut `isAutoFollowModeEnabled()` und `shouldAutoFollow(editor, doc)` prüft, ist der Listener selbst-gating: Er hängt zwar permanent am Dokument, tut aber einfach nichts, sobald die Bedingung nicht mehr erfüllt ist – und zieht sofort wieder nach, sobald sie (z. B. weil der Nutzer wieder ans Ende scrollt) erneut erfüllt ist. Das ist sogar ein Vorteil gegenüber dem Entfernen: Kehrt der Nutzer ans Ende zurück, funktioniert Auto-Follow ohne dass du den Listener neu registrieren musst.

Damit reduziert sich die Methode auf:

```java
private void documentChanged(DocumentEvent event, ITextEditor editor, IDocument doc) {
    if (isAutoFollowModeEnabled() && shouldAutoFollow(editor, doc))
        tryMoveCursorToLastLineStart(editor, doc);
}
```

**Trotzdem gibt es einen anderen, echten Grund, den Listener irgendwann zu entfernen: Ressourcen-/Lifecycle-Hygiene, nicht Korrektheit der Bedingung.**

- Der Listener hält eine Closure auf `editor` und `doc`. Schließt der Nutzer den Editor (oder wird der `ITextFileBuffer` disconnected), bleibt der Listener am `IDocument` hängen, solange das Dokument-Objekt selbst noch existiert (z. B. weil ein anderer Editor/Buffer es noch referenziert, oder es GC-technisch noch nicht weg ist).
- `editor.selectAndReveal(...)` bzw. `editor.getSelectionProvider()` auf einem **disposed/geschlossenen Editor** kann eine Exception werfen (SWT `Widget is disposed`). Die fängst du zwar in `tryMoveCursorToLastLineStart` ab, aber du bekommst dann bei jeder künftigen Änderung an dem Dokument (z. B. wenn andere Marker es noch bearbeiten) unnötigen Log-Spam (`LOG.error`) und unnötige Arbeit.
- Die `Map<IDocument, IDocumentListener>` würde sonst über die Zeit unbegrenzt wachsen (Memory-Leak), weil du nie wieder `remove` aufrufst.

**Praktischer Kompromiss:** Kein `remove` bei "Bedingung nicht erfüllt", aber ein `remove` bei echten Lifecycle-Events:
- Editor wird geschlossen (`IPartListener2.partClosed`), oder
- `ITextFileBuffer` wird disconnected (du disconnectest ja im `finally` von `replaceMarker` bereits den Buffer via `bm.disconnect(...)`).

Am einfachsten: Du registrierst zusätzlich einen leichten `IPartListener2` (einmalig, global auf allen `IWorkbenchWindow`en), der bei `partClosed` prüft, ob es sich um einen `ITextEditor` mit einem Eintrag in `autoFollowListeners` handelt, und dann `removeAutoFollowListener(doc)` aufruft. Das ist der einzige "Removal"-Grund – nicht die fachliche Bedingung, sondern Speicher-/Widget-Hygiene.

Soll ich das so umsetzen (Listener bleibt dauerhaft aktiv/self-gating, plus schlanker Cleanup bei Editor-Close)?
Setze es um.
SystemInit:  id=835eb828-1b92-478e-beaf-958bb86db69d, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 4db6-af
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  old_text: |-
    import org.eclipse.jface.text.BadLocationException;
    import org.eclipse.jface.text.IDocument;
    import org.eclipse.jface.text.ITextSelection;
    import org.eclipse.jface.viewers.ISelection;
    import org.eclipse.swt.widgets.Display;
    import org.eclipse.ui.IEditorInput;
    import org.eclipse.ui.IEditorPart;
    import org.eclipse.ui.IEditorReference;
    import org.eclipse.ui.IFileEditorInput;
    import org.eclipse.ui.IWorkbenchPage;
    import org.eclipse.ui.IWorkbenchWindow;
    import org.eclipse.ui.PlatformUI;
    import org.eclipse.ui.texteditor.ITextEditor;
    import org.osgi.framework.BundleContext;
  new_text: |-
    import org.eclipse.jface.text.BadLocationException;
    import org.eclipse.jface.text.DocumentEvent;
    import org.eclipse.jface.text.IDocument;
    import org.eclipse.jface.text.IDocumentListener;
    import org.eclipse.jface.text.ITextSelection;
    import org.eclipse.jface.viewers.ISelection;
    import org.eclipse.swt.widgets.Display;
    import org.eclipse.ui.IEditorInput;
    import org.eclipse.ui.IEditorPart;
    import org.eclipse.ui.IEditorReference;
    import org.eclipse.ui.IFileEditorInput;
    import org.eclipse.ui.IPartListener2;
    import org.eclipse.ui.IWindowListener;
    import org.eclipse.ui.IWorkbenchPage;
    import org.eclipse.ui.IWorkbenchPart;
    import org.eclipse.ui.IWorkbenchPartReference;
    import org.eclipse.ui.IWorkbenchWindow;
    import org.eclipse.ui.PlatformUI;
    import org.eclipse.ui.texteditor.ITextEditor;
    import org.osgi.framework.BundleContext;
  reason: Neue Imports für IDocumentListener-basiertes Auto-Follow und Editor-Close-Cleanup ergänzen
```
/answer 4db6-af allow
Control Request:
```yaml
id: 4db6-b0
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  old_text: "\t/** Initial retry delay in ms, doubled after every failed attempt. */\n\tprivate static final int CURSOR_MOVE_INITIAL_DELAY_MS = 800;\n\t/** Upper bound for the (doubling) retry delay. */\n\tprivate static final int CURSOR_MOVE_MAX_DELAY_MS = 800;\n\t/** Overall time budget after which retries are given up. */\n\tprivate static final long CURSOR_MOVE_TIMEOUT_MS = 4000;\n\n\t/**\n\t * Moves the cursor to the end of the doc (start of the last line, i.e. an\n\t * empty selection at the doc's end). {@link ITextEditor#selectAndReveal}\n\t * is unreliable right after a doc replace (e.g. because the widget/editor\n\t * is not yet fully laid out), so the move is retried with an increasing\n\t * delay between attempts until it can be verified to have succeeded or a\n\t * timeout is hit.\n\t */\n\tprivate void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\tscheduleMoveCursorAttempt(editor, doc, System.currentTimeMillis(), CURSOR_MOVE_INITIAL_DELAY_MS);\n\t}\n\n\tprivate void scheduleMoveCursorAttempt(ITextEditor editor, IDocument doc, long startTime, int delayMs) {\n\t\tDisplay.getDefault().timerExec(delayMs, () -> {\n\t\t\tif (tryMoveCursorToLastLineStart(editor, doc))\n\t\t\t\treturn;\n\n\t\t\tlong elapsed = System.currentTimeMillis() - startTime;\n\t\t\tif (elapsed >= CURSOR_MOVE_TIMEOUT_MS) {\n\t\t\t\tLOG.error(\"moveCursorToLastLineStart: giving up after \" + elapsed + \"ms\", null);\n\t\t\t\treturn;\n\t\t\t}\n\n\t\t\tint nextDelay = Math.min(delayMs * 2, CURSOR_MOVE_MAX_DELAY_MS);\n\t\t\tscheduleMoveCursorAttempt(editor, doc, startTime, nextDelay);\n\t\t});\n\t}\n\n\t/**\n\t * Attempts to place the cursor at the end of the doc and verifies that the\n\t * editor's selection actually reflects the requested position afterwards.\n\t *\n\t * @return {@code true} when the move could be verified to have succeeded.\n\t */\n\tprivate boolean tryMoveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\ttry {\n\t\t\tint targetOffset = doc.getLength();\n\t\t\teditor.selectAndReveal(targetOffset, 0);\n\n\t\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\t\t\tif (!(selection instanceof ITextSelection))\n\t\t\t\treturn false;\n\n\t\t\tITextSelection tsel = (ITextSelection) selection;\n\t\t\treturn tsel.getOffset() == targetOffset && tsel.getLength() == 0;\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t\treturn false;\n\t\t}\n\t}"
  new_text: "\t/**\n\t * Holds, per doc, the auto-follow {@link IDocumentListener} together with\n\t * the editor it was registered for, so it can be located/removed again\n\t * when the editor closes.\n\t */\n\tprivate static final class AutoFollowState {\n\t\tfinal ITextEditor editor;\n\t\tfinal IDocumentListener listener;\n\n\t\tAutoFollowState(ITextEditor editor, IDocumentListener listener) {\n\t\t\tthis.editor = editor;\n\t\t\tthis.listener = listener;\n\t\t}\n\t}\n\n\t/** Auto-follow listeners currently registered, keyed by doc. Only ever accessed on the UI thread. */\n\tprivate final Map<IDocument, AutoFollowState> autoFollowListeners = new HashMap<>();\n\tprivate boolean partCloseCleanupRegistered = false;\n\n\t/**\n\t * Moves the cursor to the end of the doc (start of the last line, i.e. an\n\t * empty selection at the doc's end) and, as long as auto-follow stays\n\t * applicable, keeps it there for every future change of the doc. A\n\t * one-shot retry with a fixed delay cannot work reliably here: further,\n\t * independent edits (e.g. from other pending AI answers) can arrive at\n\t * any time after a move has already been verified as successful, moving\n\t * the \"end of doc\" target again. Reacting to every {@link DocumentEvent}\n\t * instead removes the race entirely, since each change immediately\n\t * re-evaluates and re-applies the cursor position.\n\t */\n\tprivate void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\ttryMoveCursorToLastLineStart(editor, doc);\n\t\tensureAutoFollowListener(editor, doc);\n\t}\n\n\tprivate void ensureAutoFollowListener(ITextEditor editor, IDocument doc) {\n\t\tif (autoFollowListeners.containsKey(doc))\n\t\t\treturn;\n\n\t\tIDocumentListener listener = new IDocumentListener() {\n\t\t\t@Override\n\t\t\tpublic void documentAboutToBeChanged(DocumentEvent event) {\n\t\t\t\t// nothing to do\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void documentChanged(DocumentEvent event) {\n\t\t\t\t// Self-gating: only follow while the condition still holds.\n\t\t\t\t// If the user navigated away, this simply becomes a no-op\n\t\t\t\t// until (if ever) the selection is at the end again.\n\t\t\t\tif (isAutoFollowModeEnabled() && shouldAutoFollow(editor, doc))\n\t\t\t\t\ttryMoveCursorToLastLineStart(editor, doc);\n\t\t\t}\n\t\t};\n\t\tdoc.addDocumentListener(listener);\n\t\tautoFollowListeners.put(doc, new AutoFollowState(editor, listener));\n\n\t\tensurePartCloseCleanupRegistered();\n\t}\n\n\t/**\n\t * Registers, once, a listener that removes an editor's auto-follow\n\t * {@link IDocumentListener} when the editor is closed. This is purely\n\t * about avoiding a resource/memory leak (stale editor references, dead\n\t * doc listeners) - it is unrelated to and independent of the\n\t * \"user scrolled away\" case, which the listener itself already handles.\n\t */\n\tprivate void ensurePartCloseCleanupRegistered() {\n\t\tif (partCloseCleanupRegistered)\n\t\t\treturn;\n\t\tpartCloseCleanupRegistered = true;\n\n\t\tIPartListener2 cleanupListener = new IPartListener2() {\n\t\t\t@Override\n\t\t\tpublic void partClosed(IWorkbenchPartReference partRef) {\n\t\t\t\tITextEditor editor = unwrapTextEditor(partRef.getPart(false));\n\t\t\t\tif (editor != null)\n\t\t\t\t\tremoveAutoFollowListenerFor(editor);\n\t\t\t}\n\t\t};\n\n\t\tfor (IWorkbenchWindow window : PlatformUI.getWorkbench().getWorkbenchWindows())\n\t\t\twindow.getPartService().addPartListener(cleanupListener);\n\n\t\tPlatformUI.getWorkbench().addWindowListener(new IWindowListener() {\n\t\t\t@Override\n\t\t\tpublic void windowOpened(IWorkbenchWindow window) {\n\t\t\t\twindow.getPartService().addPartListener(cleanupListener);\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void windowClosed(IWorkbenchWindow window) {\n\t\t\t\t// nothing to do\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void windowActivated(IWorkbenchWindow window) {\n\t\t\t\t// nothing to do\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void windowDeactivated(IWorkbenchWindow window) {\n\t\t\t\t// nothing to do\n\t\t\t}\n\t\t});\n\t}\n\n\tprivate ITextEditor unwrapTextEditor(IWorkbenchPart part) {\n\t\tif (part instanceof AISessionEditor)\n\t\t\tpart = ((AISessionEditor) part).getEditor();\n\t\treturn part instanceof ITextEditor ? (ITextEditor) part : null;\n\t}\n\n\tprivate void removeAutoFollowListenerFor(ITextEditor editor) {\n\t\tautoFollowListeners.entrySet().removeIf(e -> {\n\t\t\tif (e.getValue().editor != editor)\n\t\t\t\treturn false;\n\t\t\te.getKey().removeDocumentListener(e.getValue().listener);\n\t\t\treturn true;\n\t\t});\n\t}\n\n\t/**\n\t * Attempts to place the cursor at the end of the doc and verifies that the\n\t * editor's selection actually reflects the requested position afterwards.\n\t *\n\t * @return {@code true} when the move could be verified to have succeeded.\n\t */\n\tprivate boolean tryMoveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\ttry {\n\t\t\tint targetOffset = doc.getLength();\n\t\t\teditor.selectAndReveal(targetOffset, 0);\n\n\t\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\t\t\tif (!(selection instanceof ITextSelection))\n\t\t\t\treturn false;\n\n\t\t\tITextSelection tsel = (ITextSelection) selection;\n\t\t\treturn tsel.getOffset() == targetOffset && tsel.getLength() == 0;\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t\treturn false;\n\t\t}\n\t}"
  reason: "Timer-basiertes Retry durch reaktiven, self-gating IDocumentListener ersetzen; Cleanup bei Editor-Close ergänzen"
```
/answer 4db6-b0 allow
Control Request:
```yaml
id: 4db6-b1
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  old_text: |-
    import java.io.BufferedReader;
    import java.io.IOException;
    import java.io.InputStreamReader;
    import java.util.regex.Matcher;
    import java.util.regex.Pattern;
  new_text: |-
    import java.io.BufferedReader;
    import java.io.IOException;
    import java.io.InputStreamReader;
    import java.util.HashMap;
    import java.util.Map;
    import java.util.regex.Matcher;
    import java.util.regex.Pattern;
  reason: Map/HashMap für die neue autoFollowListeners-Verwaltung importieren
```
/answer 4db6-b1 allow
Control Request:
```yaml
id: 4db6-b2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  reason: "Konsistenzprüfung nach den Änderungen (verwendete Imports, keine Doppeldefinitionen)"
```
/answer 4db6-b2 allow
Control Request:
```yaml
id: 4db6-b3
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find / -name "org.eclipse.ui_*.jar" 2>/dev/null | head -5
  reason: "Prüfen, ob IPartListener2 im verwendeten Eclipse-Target Default-Methoden hat (sonst kompiliert die anonyme Klasse nicht)"
```
/answer 4db6-b3 deny hier sind eclipse sourcen: `/home/user/xyan/eclipse.platform.ui`
Control Request:
```yaml
id: 4db6-b4
toolName: bash
arguments:
  cwd: /home/user/xyan/eclipse.platform.ui
  script: find . -name "IPartListener2.java" 2>/dev/null
  reason: "Sourcen von IPartListener2 finden, um zu prüfen ob es default-Methoden hat"
```
/answer 4db6-b4 allow
Control Request:
```yaml
id: 4db6-b5
result:
  structuredContent:
    stdout: |
      ./bundles/org.eclipse.ui.workbench/eclipseui/org/eclipse/ui/IPartListener2.java
```
/answer 4db6-b5 allow
Control Request:
```yaml
id: 4db6-b6
toolName: read-file
arguments:
  path: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.ui.workbench/eclipseui/org/eclipse/ui/IPartListener2.java
  reason: "Prüfen, ob die Interface-Methoden default implementiert sind (nötig, damit die anonyme Klasse mit nur partClosed kompiliert)"
```
/answer 4db6-b6 allow
Control Request:
```yaml
id: 4db6-b7
toolName: bash
arguments:
  cwd: /home/user/xyan/eclipse.platform.ui
  script: "find . -name \"IWindowListener.java\" -exec cat {} \\; 2>/dev/null | head -60"
  reason: "Prüfen, ob IWindowListener ebenfalls Default-Methoden hat"
```
/answer 4db6-b7 allow
Control Request:
```yaml
id: 4db6-b8
result:
  structuredContent:
    stdout: "/*******************************************************************************\n * Copyright (c) 2000, 2008 IBM Corporation and others.\n *\n * This program and the accompanying materials\n * are made available under the terms of the Eclipse Public License 2.0\n * which accompanies this distribution, and is available at\n * https://www.eclipse.org/legal/epl-2.0/\n *\n * SPDX-License-Identifier: EPL-2.0\n *\n * Contributors:\n *     IBM Corporation - initial API and implementation\n *******************************************************************************/\npackage org.eclipse.ui;\n\n/**\n * Interface for listening to window lifecycle events.\n * <p>\n * This interface may be implemented by clients.\n * </p>\n */\npublic interface IWindowListener {\n\t/**\n\t * Notifies this listener that the given window has been activated.\n\t * <p>\n\t * <b>Note:</b> This event is not fired if no perspective is open (the window is\n\t * empty).\n\t * </p>\n\t *\n\t * @param window the window that was activated\n\t */\n\tvoid windowActivated(IWorkbenchWindow window);\n\n\t/**\n\t * Notifies this listener that the given window has been deactivated.\n\t * <p>\n\t * <b>Note:</b> This event is not fired if no perspective is open (the window is\n\t * empty).\n\t * </p>\n\t *\n\t * @param window the window that was activated\n\t */\n\tvoid windowDeactivated(IWorkbenchWindow window);\n\n\t/**\n\t * Notifies this listener that the given window has been closed.\n\t *\n\t * @param window the window that was closed\n\t * @see IWorkbenchWindow#close\n\t */\n\tvoid windowClosed(IWorkbenchWindow window);\n\n\t/**\n\t * Notifies this listener that the given window has been opened.\n\t *\n\t * @param window the window that was opened\n\t * @see IWorkbench#openWorkbenchWindow\n\t */\n\tvoid windowOpened(IWorkbenchWindow window);\n\n"
```
/answer 4db6-b8 allow
Control Request:
```yaml
id: 4db6-b9
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -iname "*.classpath" -o -iname "pom.xml" | head; ls
  reason: "Prüfen, ob ein Build (Maven/Tycho) verfügbar ist, um die Datei probeweise zu kompilieren"
```
/answer 4db6-b9 deny du kannst blind build.sh ausführen
Control Request:
```yaml
id: 4db6-ba
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: ./build.sh 2>&1 | tail -100
  reason: "Build ausführen, um die geänderte Datei zu kompilieren und Fehler zu finden"
```
/answer 4db6-ba allow
ReasoningToken: 3586
Text:
 Jetzt fehlen noch die Imports für `Map`/`HashMap`.
Control Request:
```yaml
id: 4db6-bb
result:
  structuredContent:
    stdout: |
      Successfull
```
/answer 4db6-bb allow
Text:
 Nun kurz die Datei durchgehen, um Konsistenz zu prüfen (z. B. ob `Display` noch verwendet wird, keine doppelten Methoden etc.):
ReasoningToken: 60
ReasoningToken: 224
Text:
 Gut, `IPartListener2` hat Default-Methoden – die anonyme Klasse mit nur `partClosed` überschrieben kompiliert also problemlos. Auch `IWindowListener` prüfen wir noch kurz, sowie ob das Projekt tatsächlich mit diesen Sourcen baut:
Text:
 `IWindowListener` verlangt alle vier Methoden explizit (keine Defaults) – ich habe in meinem Code bereits alle vier implementiert, das passt also. Kurzer Kompilier-Check:
Der Build läuft durch. Zusammenfassung der Umsetzung:

**Was geändert wurde in `MarkerRessourceScanner.java`:**

1. **Timer-basiertes Retry entfernt** (`CURSOR_MOVE_*`-Konstanten, `scheduleMoveCursorAttempt`) – ersetzt durch einen **reaktiven, self-gating `IDocumentListener`**, der bei jeder Dokumentänderung neu prüft, ob Auto-Follow noch gilt, und den Cursor entsprechend nachzieht. Damit gibt es keine Race Condition mehr zwischen "Cursor verifiziert" und "weitere Änderung kam noch rein".
2. **`moveCursorToLastLineStart`** macht jetzt sofort einen Versuch (`tryMoveCursorToLastLineStart`, unverändert) und registriert danach – falls noch nicht vorhanden – den dauerhaften Listener für dieses Doc (`ensureAutoFollowListener`).
3. **Keine Entfernung bei "Bedingung nicht erfüllt"** – der Listener bleibt bestehen und tut einfach nichts, wenn `shouldAutoFollow` mal `false` liefert; er greift automatisch wieder, sobald der Nutzer erneut ans Ende scrollt.
4. **Cleanup nur aus Lifecycle-Gründen**: Ein einmalig registrierter `IPartListener2` entfernt den Doc-Listener, wenn der zugehörige Editor geschlossen wird (verhindert Leaks/Exceptions auf disposed Widgets); ein `IWindowListener` sorgt dafür, dass auch in später geöffneten Workbench-Fenstern aufgeräumt wird.
5. Neue Imports (`IDocumentListener`, `DocumentEvent`, `Map`/`HashMap`, `IPartListener2`, `IWindowListener`, `IWorkbenchPart`, `IWorkbenchPartReference`) ergänzt.

Build (`./build.sh`) läuft erfolgreich durch.
