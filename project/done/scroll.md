Der Aufruf "textEditor.selectAndReveal(doc.getLength(), 0);" in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java` scheint nicht zu bewirken das der Editor scrolled.
#: Control Request:
#: {
#:   "id" : "ea33e82a-1782-42f5-ba4b-3f79d0f3ecf0",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java"
#:   }
#: }
/allow ea33e82a-1782-42f5-ba4b-3f79d0f3ecf0
#: Control Request:
#: {
#:   "id" : "e794ba57-e07f-4fd6-b2d5-30d5cabe1b5d",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench;\n\nimport java.io.ByteArrayInputStream;\nimport java.io.InputStream;\nimport java.io.UnsupportedEncodingException;\nimport java.net.URI;\nimport java.time.LocalDateTime;\nimport java.time.format.DateTimeFormatter;\n\nimport org.eclipse.core.resources.IContainer;\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IResource;\nimport org.eclipse.core.resources.ResourcesPlugin;\nimport org.eclipse.core.runtime.CoreException;\nimport org.eclipse.core.runtime.IProgressMonitor;\nimport org.eclipse.core.runtime.Path;\nimport org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.ITextSelection;\nimport org.eclipse.jface.viewers.ISelection;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.IEditorInput;\nimport org.eclipse.ui.IFileEditorInput;\nimport org.eclipse.ui.IURIEditorInput;\nimport org.eclipse.ui.IWorkbenchPage;\nimport org.eclipse.ui.PartInitException;\nimport org.eclipse.ui.PlatformUI;\nimport org.eclipse.ui.ide.IDE;\nimport org.eclipse.ui.texteditor.ITextEditor;\n\nimport xy.ai.workbench.Model.KeyPattern;\nimport xy.ai.workbench.connectors.AdaptingConnector;\nimport xy.ai.workbench.marker.MarkerRessourceScanner;\nimport xy.ai.workbench.models.AIAnswer;\nimport xy.ai.workbench.models.IModelRequest;\n\npublic class EditorInterface {\n\tpublic static final String USER = \"User:\";\n\tpublic static final String AGENT = \"Agent:\";\n\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final AdaptingConnector connector;\n\tprivate final ConfigManager cfg;\n\n\tpublic EditorInterface(ActiveEditorListener editorListener, AdaptingConnector connector, ConfigManager cfg) {\n\t\tthis.editorListener = editorListener;\n\t\tthis.connector = connector;\n\t\tthis.cfg = cfg;\n\t}\n\n\tpublic void insertTag(Display display, IModelRequest req, IProgressMonitor mon) {\n\t\tdisplay.syncExec(() -> {\n\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\tif (OutputMode.New_File.equals(cfg.getOuputMode())) {\n\n\t\t\t\tIEditorInput editorInput = textEditor.getEditorInput();\n\t\t\t\tIFile currentFile;\n\t\t\t\tif (editorInput instanceof IFileEditorInput)\n\t\t\t\t\tcurrentFile = ((IFileEditorInput) editorInput).getFile();\n\t\t\t\telse if (editorInput instanceof IURIEditorInput) {\n\t\t\t\t\tURI uri = ((IURIEditorInput) editorInput).getURI();\n\t\t\t\t\tString fileName = new Path(uri.getPath()).lastSegment();\n\t\t\t\t\tcurrentFile = ResourcesPlugin.getWorkspace().getRoot().getProject(\"ExternalFiles\")\n\t\t\t\t\t\t\t.getFile(fileName);\n\n\t\t\t\t\tif (!currentFile.exists())\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\tcurrentFile.createLink(uri, IResource.ALLOW_MISSING_LOCAL, mon);\n\t\t\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\t\t\tthrow new IllegalStateException(\"Could not link external file\", e);\n\t\t\t\t\t\t}\n\t\t\t\t} else\n\t\t\t\t\tthrow new IllegalArgumentException(\"Editor type not supported for new file output mode\");\n\n\t\t\t\tIContainer parent = currentFile.getParent();\n\n\t\t\t\tString timestamp = LocalDateTime.now().format(DateTimeFormatter.ofPattern(\"yyyyMMdd.HHmmss\"));\n\t\t\t\tIFile newFile = parent.getFile(new Path(timestamp + \".md\"));\n\t\t\t\tString tag = generateTag(req);\n\t\t\t\ttry {\n\t\t\t\t\tInputStream source = new ByteArrayInputStream(tag.getBytes(\"UTF-8\"));\n\n\t\t\t\t\tif (!newFile.exists()) {\n\t\t\t\t\t\tnewFile.create(source, true, null);\n\t\t\t\t\t} else {\n\t\t\t\t\t\tnewFile.setContents(source, true, true, null);\n\t\t\t\t\t}\n\t\t\t\t\tnewFile.touch(null);\n\n\t\t\t\t\tIWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();\n\t\t\t\t\tIDE.openEditor(page, newFile);\n\t\t\t\t} catch (PartInitException e) {\n\t\t\t\t\tLOG.info(\"Error opening new editor file\");\n\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\tLOG.info(\"Error writting file\");\n\t\t\t\t} catch (UnsupportedEncodingException e) {\n\t\t\t\t\tLOG.info(\"Error unsupported encoding\");\n\t\t\t\t}\n\n\t\t\t} else {\n\n\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\tISelection selection = textEditor.getSelectionProvider().getSelection();\n\t\t\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\n\t\t\t\ttry {\n\t\t\t\t\tString tag = generateTag(req);\n\t\t\t\t\tswitch (cfg.getOuputMode()) {\n\t\t\t\t\tcase Chat:\n\t\t\t\t\t\tString replace = String.format(\"\\n%s\\n%s\\n%s\\n\", AGENT, tag, USER);\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, replace);\n\t\t\t\t\t\ttextEditor.selectAndReveal(doc.getLength(), 0);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Append:\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, \"\\n\" + tag);\n\t\t\t\t\t\ttextEditor.selectAndReveal(doc.getLength(), 0);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Replace:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), tsel.getLength(), tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Cursor:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), 0, tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase New_File:\n\t\t\t\t\t\tthrow new UnsupportedOperationException();\n\t\t\t\t\t}\n\t\t\t\t\ttextEditor.doSave(mon);\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.info(\"Error adding text\");\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t}\n\n\tprivate String generateTag(IModelRequest req) {\n\t\tKeyPattern pattern = connector.getConnector(req).getSupportedKeyPattern();\n\t\treturn MarkerRessourceScanner.getPromptTag(pattern.name(), req.getID());\n\t}\n\n\tpublic void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {\n\t\tif (!Activator.getDefault().markerScanner.findAndReplaceMarkers(ans))\n\t\t\tLOG.info(\"Error: wasn't able to replace prompt marker with answer:\\n\" + ans.answer);\n\t}\n}\n"
#:     }
#:   }
#: }
/allow e794ba57-e07f-4fd6-b2d5-30d5cabe1b5d
#: ReasoningToken: 14
#: ReasoningToken: 1518
## Diagnose

`textEditor.selectAndReveal(offset, length)` delegiert intern an den `ISourceViewer` (bzw. dessen `TextViewer.revealRange(...)`). Dass hier "nichts passiert", hat typischerweise einen der folgenden Gründe:

1. **Editor ist nicht der aktive/sichtbare Editor bzw. Widget noch nicht realisiert**
   `editorListener.getLastTextEditor()` liefert den *zuletzt aktiven* Text-Editor – das muss nicht der aktuell sichtbare/fokussierte sein. `TextViewer.revealRange()` prüft intern die Größe/Sichtbarkeit des `StyledText`-Controls (`getClientArea()`), um den Scroll-Offset zu berechnen. Ist das Control gerade unsichtbar, minimiert, in einem Hintergrund-Tab oder noch nicht gelayoutet (Höhe/Breite = 0), wird der Scroll entweder gar nicht oder fehlerhaft berechnet – und beim nächsten Layout-Pass nicht nachgeholt.

   → Fix: Editor vor dem Reveal aktivieren:
   ```java
   IWorkbenchPage page = textEditor.getSite().getPage();
   page.activate(textEditor);
   textEditor.selectAndReveal(doc.getLength(), 0);
   ```

2. **Timing: Reveal läuft, bevor der Reconciler/StyledText das `doc.replace()` vollständig verarbeitet hat**
   Auch wenn `doc.replace()` synchron ist, kann der `SourceViewer` intern noch mit Folding/Projection/Dirty-Region-Verarbeitung beschäftigt sein (Reconciler-Job), sodass `getTopIndex()`/`getBottomIndex()`-Berechnungen im `revealRange` noch auf altem Zustand basieren.

   → Fix: Reveal in einem separaten `asyncExec` nach dem `replace()` anstoßen, statt direkt danach:
   ```java
   doc.replace(doc.getLength(), 0, replace);
   int newLen = doc.getLength();
   display.asyncExec(() -> textEditor.selectAndReveal(newLen, 0));
   ```

3. **Offset exakt am Dokumentende ohne trailing Newline**
   Wenn der letzte sichtbare Text bereits genau auf der letzten (teilweise sichtbaren) Zeile liegt, hält Eclipse die Position u. U. für "schon sichtbar" und scrollt nicht, obwohl die Zeile am unteren Rand abgeschnitten ist. Das ist reines UX-Verhalten von `revealRange`, kein Bug.

   → Fix: Statt Offset am Dateiende direkt den Viewer zwingen, ein paar Zeilen zusätzlich Puffer zu zeigen, z. B. über `ITextViewerExtension5`/`StyledText.setTopIndex(...)` basierend auf Zeilenzahl + 1, oder `doc.getLength()` in Kombination mit einem kurzen `revealRange` über eine etwas größere Range (z. B. die letzte eingefügte Zeile komplett selektieren statt nur Offset 0-Länge).

4. **`textEditor.doSave(mon)` direkt danach**
   Falls der `IDocumentProvider` beim Speichern das Dokument neu lädt/validiert (z. B. bei externen Dateiänderungen, Encoding-Checks etc.), kann das die Anzeige/Scrollposition zurücksetzen. Prüfen, ob nach `doSave` die Selektion/Scrollposition erhalten bleibt – falls nicht, Reveal **nach** dem Save erneut ausführen.

## Empfohlene Änderung

```java
doc.replace(doc.getLength(), 0, replace);
int revealOffset = doc.getLength();

IWorkbenchPage page = textEditor.getSite().getPage();
page.activate(textEditor); // sicherstellen, dass der Editor aktiv/sichtbar ist

textEditor.doSave(mon);

display.asyncExec(() -> textEditor.selectAndReveal(revealOffset, 0));
```

Falls das Problem weiterhin besteht, wäre es hilfreich zu wissen, ob der betroffene Editor zum Zeitpunkt des Aufrufs sichtbar/aktiv ist (z. B. bei Hintergrund-Requests, während der Nutzer in einem anderen Tab arbeitet) – das ist die häufigste Ursache für "kein Scroll trotz erfolgreichem `selectAndReveal`-Aufruf".

---

Zur zusätzlichen Systemmeldung ("`[xy.ai.req :ClaudeCode:...]`"): Das sieht nach einem internen Tracking-/Request-ID-Marker deines Plugins aus, nicht nach einer eigenständigen Nutzerfrage – es enthält keinen weiteren Inhalt, den ich adressieren könnte. Falls das ein Fehler in der Nachrichtenübermittlung war, gib mir gern die eigentliche Frage dazu.