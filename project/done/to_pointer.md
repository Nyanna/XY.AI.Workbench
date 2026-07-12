In `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java` in der Methode "execute" in Zeile 285, wird `editIfc.replaceTag(display, ans, sub);` aufgerufen, um einen Tag mit Output zu ersetzen. Implementiere eine Änderung.

Wenn die Zieldatei:

* gerade geöffnet ist
* unmodified gegenüber der gespeicherten Datei, also der Benutzer nichts editiert
* der OutputMode auf "Append" oder "Chat" steht
* die Cursorposition bereits in der letzten Zeile ist

Dann soll nach dem Ersetzen des Tags der Cursor erneut an den Anfang der letzten Zeile im Dokument gesetzt werden.
Das Ziel ist eine Autofollow, als Komfortfunktion, ohne den Nutzer zu  beeinträchtigen.
#: Control Request:
#: {
#:   "id" : "03f735db-25c5-42e0-8eaf-6a8e5e7b9732",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "script" : "sed -n '1,60p' /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java | head -60",
#:     "cwd" : "/home/user/xyan"
#:   }
#: }
/allow 03f735db-25c5-42e0-8eaf-6a8e5e7b9732
#: Control Request:
#: {
#:   "id" : "6f9f76ac-c67d-4b25-b529-fccbf52324ce",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "package xy.ai.workbench;\n\nimport java.io.IOException;\nimport java.io.InputStream;\nimport java.nio.charset.StandardCharsets;\nimport java.util.ArrayList;\nimport java.util.Arrays;\nimport java.util.List;\nimport java.util.function.Consumer;\nimport java.util.stream.Collectors;\n\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IResource;\nimport org.eclipse.core.runtime.CoreException;\nimport org.eclipse.core.runtime.IProgressMonitor;\nimport org.eclipse.core.runtime.Status;\nimport org.eclipse.core.runtime.SubMonitor;\nimport org.eclipse.core.runtime.jobs.Job;\nimport org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.Document;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IRegion;\nimport org.eclipse.jface.text.ITextSelection;\nimport org.eclipse.jface.viewers.ISelection;\nimport org.eclipse.jface.viewers.ISelectionProvider;\nimport org.eclipse.jface.viewers.TreeSelection;\nimport org.eclipse.search.ui.ISearchQuery;\nimport org.eclipse.search.ui.ISearchResult;\nimport org.eclipse.search.ui.ISearchResultListener;\nimport org.eclipse.search.ui.NewSearchUI;\nimport org.eclipse.search.ui.SearchResultEvent;\nimport org.eclipse.search.ui.text.AbstractTextSearchResult;\nimport org.eclipse.search.ui.text.Match;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.IEditorInput;\nimport org.eclipse.ui.IFileEditorInput;\nimport org.eclipse.ui.IWorkbenchPage;\nimport org.eclipse.ui.IWorkbenchWindow;\nimport org.eclipse.ui.PlatformUI;\nimport org.eclipse.ui.texteditor.IDocumentProvider;\nimport org.eclipse.ui.texteditor.ITextEditor;\n\nimport xy.ai.workbench.batch.AIBatchManager;\nimport xy.ai.workbench.connectors.AdaptingConnector;\nimport xy.ai.workbench.editors.AIRuleScanner;\nimport xy.ai.workbench.models.AIAnswer;\nimport xy.ai.workbench.models.IModelRequest;\nimport xy.ai.workbench.models.IModelResponse;\nimport xy.ai.workbench.tools.AbstractQueryListener;\n\npublic class AISessionManager {\n\tpublic static final String CONTEXT_PROMPT_TXT = \"context.prompt.txt\";\n\n\tprivate ActiveEditorListener editorListener = new ActiveEditorListener(this);\n\n\tprivate final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\tpublic final EditorInterface editIfc;\n\tprivate int[] inputStats = new int[InputMode.values().length];\n\tprivate List<Consumer<AIAnswer>> answerObs = new ArrayList<>();\n"
#:     }
#:   }
#: }
/allow 6f9f76ac-c67d-4b25-b529-fccbf52324ce
#: Control Request:
#: {
#:   "id" : "b5c7cd0a-31da-4139-b1d1-c32f3a7fd9c0",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "script" : "grep -n \"execute\\|replaceTag\\|OutputMode\\|editIfc\\.\" /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java | head -80",
#:     "cwd" : "/home/user/xyan"
#:   }
#: }
/allow b5c7cd0a-31da-4139-b1d1-c32f3a7fd9c0
#: Control Request:
#: {
#:   "id" : "d6a483aa-afe2-4862-9c4d-96bb6edde203",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "271:\tpublic void execute(Display display) {\n279:\t\t\t\teditIfc.insertTag(display, req, sub);\n282:\t\t\t\tvar ans = executeInner(display, req, sub);\n285:\t\t\t\teditIfc.replaceTag(display, ans, sub);\n316:\t\teditIfc.insertTag(display, req, sub.split(1));\n396:\tprivate AIAnswer executeInner(Display display, IModelRequest req, IProgressMonitor mon) {\n398:\t\tIModelResponse resp = connector.executeRequest(req, mon);\n"
#:     }
#:   }
#: }
/allow d6a483aa-afe2-4862-9c4d-96bb6edde203
#: Control Request:
#: {
#:   "id" : "9ed8e792-4868-4503-9f4f-7e39b158ec9d",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java",
#:     "min_line" : 260,
#:     "max_line" : 330
#:   }
#: }
/allow 9ed8e792-4868-4503-9f4f-7e39b158ec9d
#: Control Request:
#: {
#:   "id" : "d473e994-c85c-4668-a0d3-17306de3752e",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\t\tfor (IFile file : files) {\n\t\t\ttry {\n\t\t\t\tString content = file.readString();\n\t\t\t\tfullContent.append(content).append(\"\\n\");\n\t\t\t} catch (CoreException e) {\n\t\t\t\tLOG.error(\"Error on reading \" + file.getName(), e);\n\t\t\t}\n\t\t}\n\t\treturn fullContent.toString();\n\t}\n\n\tpublic void execute(Display display) {\n\t\tJob.create(\"Starting Prompt\", (mon) -> {\n\t\t\tSubMonitor sub = SubMonitor.convert(mon, \"Executing prompt\", 4);\n\t\t\ttry {\n\t\t\t\tsub.subTask(\"Prepare inputs\");\n\t\t\t\tvar req = prepareInner(display, false, sub);\n\t\t\t\tsub.worked(1);\n\t\t\t\tsub.subTask(\"Insert Tag\");\n\t\t\t\teditIfc.insertTag(display, req, sub);\n\t\t\t\tmon.worked(1);\n\t\t\t\tsub.subTask(\"Execute prompt\");\n\t\t\t\tvar ans = executeInner(display, req, sub);\n\t\t\t\tmon.worked(1);\n\t\t\t\tsub.subTask(\"Process Answer\");\n\t\t\t\teditIfc.replaceTag(display, ans, sub);\n\t\t\t\tmon.worked(1);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}\n\n\tpublic void queueAsync(Display display, AIBatchManager batch) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, batch, mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n\t\t\t}\n\t\t\treturn Status.OK_STATUS;\n\t\t}).schedule();\n\t}\n\n\tprivate void queueSync(Display display, AIBatchManager batch, IProgressMonitor mon) {\n\t\tSubMonitor sub = SubMonitor.convert(mon, \"Enqueue batch prompt\", 3);\n\t\tsub.subTask(\"Prepare inputs\");\n\t\tvar req = prepareInner(display, true, sub.split(1));\n\t\tsub.subTask(\"Insert Tag\");\n\t\teditIfc.insertTag(display, req, sub.split(1));\n\t\tsub.subTask(\"Enqueue prompt\");\n\t\tbatch.enqueue(req, sub.split(1));\n\t}\n\n\tpublic void queueAndSubmit(Display display, AIBatchManager batch) {\n\t\tJob.create(\"Enqueue Prompt\", (mon) -> {\n\t\t\ttry {\n\t\t\t\tqueueSync(display, batch, mon);\n\t\t\t\tbatch.submitBatches(mon);\n\t\t\t} catch (Exception e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\treturn Status.CANCEL_STATUS;\n\t\t\t} finally {\n\t\t\t\tmon.done();\n",
#:       "min_line" : 260,
#:       "max_line" : 330
#:     }
#:   }
#: }
/allow d473e994-c85c-4668-a0d3-17306de3752e
#: Control Request:
#: {
#:   "id" : "14b3ade2-b6b7-4ae4-91f9-0ce49ba593c8",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "script" : "find /home/user/xyan -iname \"EditorInterface.java\" -o -iname \"OutputMode.java\"",
#:     "cwd" : "/home/user/xyan"
#:   }
#: }
/allow 14b3ade2-b6b7-4ae4-91f9-0ce49ba593c8
#: Control Request:
#: {
#:   "id" : "b93dff29-fba6-4a0c-a686-9e8ca7ee6be5",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java\n/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/OutputMode.java\n"
#:     }
#:   }
#: }
/allow b93dff29-fba6-4a0c-a686-9e8ca7ee6be5
#: Control Request:
#: {
#:   "id" : "530699b9-f75b-4e0d-bf04-b4942ec96291",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java"
#:   }
#: }
/allow 530699b9-f75b-4e0d-bf04-b4942ec96291
#: Control Request:
#: {
#:   "id" : "9560dccd-d38c-433c-aab9-145afc81d315",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench;\n\nimport java.io.ByteArrayInputStream;\nimport java.io.InputStream;\nimport java.io.UnsupportedEncodingException;\nimport java.net.URI;\nimport java.time.LocalDateTime;\nimport java.time.format.DateTimeFormatter;\n\nimport org.eclipse.core.resources.IContainer;\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IResource;\nimport org.eclipse.core.resources.ResourcesPlugin;\nimport org.eclipse.core.runtime.CoreException;\nimport org.eclipse.core.runtime.IProgressMonitor;\nimport org.eclipse.core.runtime.Path;\nimport org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.ITextSelection;\nimport org.eclipse.jface.viewers.ISelection;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.IEditorInput;\nimport org.eclipse.ui.IFileEditorInput;\nimport org.eclipse.ui.IURIEditorInput;\nimport org.eclipse.ui.IWorkbenchPage;\nimport org.eclipse.ui.PartInitException;\nimport org.eclipse.ui.PlatformUI;\nimport org.eclipse.ui.ide.IDE;\nimport org.eclipse.ui.texteditor.ITextEditor;\n\nimport xy.ai.workbench.Model.KeyPattern;\nimport xy.ai.workbench.connectors.AdaptingConnector;\nimport xy.ai.workbench.marker.MarkerRessourceScanner;\nimport xy.ai.workbench.models.AIAnswer;\nimport xy.ai.workbench.models.IModelRequest;\n\npublic class EditorInterface {\n\tpublic static final String USER = \"User:\";\n\tpublic static final String AGENT = \"Agent:\";\n\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final AdaptingConnector connector;\n\tprivate final ConfigManager cfg;\n\n\tpublic EditorInterface(ActiveEditorListener editorListener, AdaptingConnector connector, ConfigManager cfg) {\n\t\tthis.editorListener = editorListener;\n\t\tthis.connector = connector;\n\t\tthis.cfg = cfg;\n\t}\n\n\tpublic void insertTag(Display display, IModelRequest req, IProgressMonitor mon) {\n\t\tdisplay.syncExec(() -> {\n\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\tif (OutputMode.New_File.equals(cfg.getOuputMode())) {\n\n\t\t\t\tIEditorInput editorInput = textEditor.getEditorInput();\n\t\t\t\tIFile currentFile;\n\t\t\t\tif (editorInput instanceof IFileEditorInput)\n\t\t\t\t\tcurrentFile = ((IFileEditorInput) editorInput).getFile();\n\t\t\t\telse if (editorInput instanceof IURIEditorInput) {\n\t\t\t\t\tURI uri = ((IURIEditorInput) editorInput).getURI();\n\t\t\t\t\tString fileName = new Path(uri.getPath()).lastSegment();\n\t\t\t\t\tcurrentFile = ResourcesPlugin.getWorkspace().getRoot().getProject(\"ExternalFiles\")\n\t\t\t\t\t\t\t.getFile(fileName);\n\n\t\t\t\t\tif (!currentFile.exists())\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\tcurrentFile.createLink(uri, IResource.ALLOW_MISSING_LOCAL, mon);\n\t\t\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\t\t\tthrow new IllegalStateException(\"Could not link external file\", e);\n\t\t\t\t\t\t}\n\t\t\t\t} else\n\t\t\t\t\tthrow new IllegalArgumentException(\"Editor type not supported for new file output mode\");\n\n\t\t\t\tIContainer parent = currentFile.getParent();\n\n\t\t\t\tString timestamp = LocalDateTime.now().format(DateTimeFormatter.ofPattern(\"yyyyMMdd.HHmmss\"));\n\t\t\t\tIFile newFile = parent.getFile(new Path(timestamp + \".md\"));\n\t\t\t\tString tag = generateTag(req);\n\t\t\t\ttry {\n\t\t\t\t\tInputStream source = new ByteArrayInputStream(tag.getBytes(\"UTF-8\"));\n\n\t\t\t\t\tif (!newFile.exists()) {\n\t\t\t\t\t\tnewFile.create(source, true, null);\n\t\t\t\t\t} else {\n\t\t\t\t\t\tnewFile.setContents(source, true, true, null);\n\t\t\t\t\t}\n\t\t\t\t\tnewFile.touch(null);\n\n\t\t\t\t\tIWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();\n\t\t\t\t\tIDE.openEditor(page, newFile);\n\t\t\t\t} catch (PartInitException e) {\n\t\t\t\t\tLOG.info(\"Error opening new editor file\");\n\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\tLOG.info(\"Error writting file\");\n\t\t\t\t} catch (UnsupportedEncodingException e) {\n\t\t\t\t\tLOG.info(\"Error unsupported encoding\");\n\t\t\t\t}\n\n\t\t\t} else {\n\n\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\tISelection selection = textEditor.getSelectionProvider().getSelection();\n\t\t\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\n\t\t\t\ttry {\n\t\t\t\t\tString tag = generateTag(req);\n\t\t\t\t\tswitch (cfg.getOuputMode()) {\n\t\t\t\t\tcase Chat:\n\t\t\t\t\t\tString replace = String.format(\"\\n%s\\n%s\\n%s\\n\", AGENT, tag, USER);\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, replace);\n\t\t\t\t\t\ttextEditor.selectAndReveal(doc.getLength(), 0);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Append:\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, \"\\n\" + tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Replace:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), tsel.getLength(), tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Cursor:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), 0, tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase New_File:\n\t\t\t\t\t\tthrow new UnsupportedOperationException();\n\t\t\t\t\t}\n\t\t\t\t\ttextEditor.doSave(mon);\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.info(\"Error adding text\");\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t}\n\n\tprivate String generateTag(IModelRequest req) {\n\t\tKeyPattern pattern = connector.getConnector(req).getSupportedKeyPattern();\n\t\treturn MarkerRessourceScanner.getPromptTag(pattern.name(), req.getID());\n\t}\n\n\tpublic void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {\n\t\tif (!Activator.getDefault().markerScanner.findAndReplaceMarkers(ans))\n\t\t\tLOG.info(\"Error: wasn't able to replace prompt marker with answer:\\n\" + ans.answer);\n\t}\n}\n"
#:     }
#:   }
#: }
/allow 9560dccd-d38c-433c-aab9-145afc81d315
#: Control Request:
#: {
#:   "id" : "035875d3-c682-45ae-b888-fb827c1748af",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/OutputMode.java"
#:   }
#: }
/allow 035875d3-c682-45ae-b888-fb827c1748af
#: Control Request:
#: {
#:   "id" : "6891c574-f562-4e8e-8593-e3f54cb1fce3",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench;\n\npublic enum OutputMode {\n\tAppend, Replace, Cursor, Chat, New_File\n}\n"
#:     }
#:   }
#: }
/allow 6891c574-f562-4e8e-8593-e3f54cb1fce3
#: Control Request:
#: {
#:   "id" : "cacff1af-0762-4003-97b3-ac6ad7d1ef39",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "script" : "find /home/user/xyan -iname \"MarkerRessourceScanner.java\"",
#:     "cwd" : "/home/user/xyan"
#:   }
#: }
/allow cacff1af-0762-4003-97b3-ac6ad7d1ef39
#: Control Request:
#: {
#:   "id" : "c21aad10-f791-4894-9f98-475c3e792e02",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java\n"
#:     }
#:   }
#: }
/allow c21aad10-f791-4894-9f98-475c3e792e02
#: Control Request:
#: {
#:   "id" : "f7aaf2db-75f5-42d4-a938-6441f7cb6e6a",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java"
#:   }
#: }
/allow f7aaf2db-75f5-42d4-a938-6441f7cb6e6a
#: Control Request:
#: {
#:   "id" : "235284a4-102f-419f-83b7-3f8541249556",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench.marker;\n\nimport java.io.BufferedReader;\nimport java.io.IOException;\nimport java.io.InputStreamReader;\nimport java.util.regex.Matcher;\nimport java.util.regex.Pattern;\n\nimport org.eclipse.core.filebuffers.FileBuffers;\nimport org.eclipse.core.filebuffers.ITextFileBuffer;\nimport org.eclipse.core.filebuffers.ITextFileBufferManager;\nimport org.eclipse.core.filebuffers.LocationKind;\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IMarker;\nimport org.eclipse.core.resources.IResource;\nimport org.eclipse.core.resources.IResourceChangeEvent;\nimport org.eclipse.core.resources.IResourceChangeListener;\nimport org.eclipse.core.resources.IResourceDelta;\nimport org.eclipse.core.resources.IResourceDeltaVisitor;\nimport org.eclipse.core.resources.IWorkspaceRoot;\nimport org.eclipse.core.resources.ResourcesPlugin;\nimport org.eclipse.core.runtime.CoreException;\nimport org.eclipse.core.runtime.Status;\nimport org.eclipse.core.runtime.jobs.Job;\nimport org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.IEditorInput;\nimport org.eclipse.ui.IEditorPart;\nimport org.eclipse.ui.IEditorReference;\nimport org.eclipse.ui.IFileEditorInput;\nimport org.eclipse.ui.IWorkbenchPage;\nimport org.eclipse.ui.IWorkbenchWindow;\nimport org.eclipse.ui.PlatformUI;\nimport org.eclipse.ui.texteditor.ITextEditor;\nimport org.osgi.framework.BundleContext;\n\nimport xy.ai.workbench.LOG;\nimport xy.ai.workbench.models.AIAnswer;\n\npublic class MarkerRessourceScanner implements IResourceChangeListener, IResourceDeltaVisitor {\n\tprivate static final String AIREQ_PREFIX = \"xy.ai.req\";\n\tprivate static final String MARKER_ID = \"xy.ai.workbench.promptmarker\";\n\tprivate static final String MARKER_REQ_ID_ATTR = \"requestId\";\n\tprivate static final String MARKER_OFF_ID_ATTR = \"offset\";\n\tprivate static final String MARKER_LEN_ID_ATTR = \"length\";\n\tprivate final Pattern pattern;\n\n\tpublic MarkerRessourceScanner(BundleContext context) {\n\t\tResourcesPlugin.getWorkspace().addResourceChangeListener(this, IResourceChangeEvent.POST_CHANGE);\n\t\tpattern = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\", Pattern.CASE_INSENSITIVE);\n\t}\n\n\tpublic void dispose(BundleContext context) {\n\t\tResourcesPlugin.getWorkspace().removeResourceChangeListener(this);\n\t}\n\n\t@Override\n\tpublic boolean visit(IResourceDelta delta) throws CoreException {\n\t\tIResource rsc = delta.getResource();\n\t\tif (rsc.getType() == IResource.FILE //\n\t\t\t\t&& rsc.getName().endsWith(\".md\")//\n\t\t\t\t&& delta.getKind() == IResourceDelta.CHANGED //\n\t\t\t\t&& rsc instanceof IFile //\n\t\t\t\t&& (delta.getFlags() & IResourceDelta.CONTENT) != 0)\n\t\t\tJob.create(\"Scanning file for markers\", mon -> {\n\t\t\t\trescannFile((IFile) rsc);\n\t\t\t\treturn Status.OK_STATUS;\n\t\t\t}).schedule();\n\t\treturn true;\n\t}\n\n\tprivate void rescannFile(IFile file) {\n\t\tint lineNumber = 1;\n\t\tint totaloff = 0;\n\n\t\ttry (BufferedReader reader = new BufferedReader(new InputStreamReader(file.getContents()))) {\n\t\t\tfile.deleteMarkers(MARKER_ID, false, IResource.DEPTH_ZERO);\n\n\t\t\tString line;\n\t\t\twhile ((line = reader.readLine()) != null) {\n\t\t\t\tMatcher m = pattern.matcher(line);\n\t\t\t\twhile (m.find()) {\n\t\t\t\t\tIMarker marker = file.createMarker(MARKER_ID);\n\t\t\t\t\tmarker.setAttribute(IMarker.MESSAGE, m.group(1) + \": \" + m.group(2));\n\t\t\t\t\tmarker.setAttribute(IMarker.LINE_NUMBER, lineNumber);\n\t\t\t\t\tmarker.setAttribute(MARKER_OFF_ID_ATTR, totaloff + m.start());\n\t\t\t\t\tmarker.setAttribute(MARKER_LEN_ID_ATTR, m.end() - m.start());\n\t\t\t\t\tmarker.setAttribute(IMarker.PRIORITY, IMarker.PRIORITY_NORMAL);\n\t\t\t\t\tmarker.setAttribute(MARKER_REQ_ID_ATTR, m.group(2));\n\t\t\t\t}\n\t\t\t\tlineNumber++;\n\t\t\t\ttotaloff += line.length() + 1;\n\t\t\t}\n\t\t} catch (IOException | CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}\n\n\t@Override\n\tpublic void resourceChanged(IResourceChangeEvent event) {\n\t\tif (event.getType() == IResourceChangeEvent.POST_CHANGE) {\n\t\t\ttry {\n\t\t\t\tevent.getDelta().accept(this);\n\t\t\t} catch (CoreException e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t}\n\t\t}\n\t}\n\n\t/**\n\t * @param ans\n\t * @return true when at least one marker was displayed and the AI answer is\n\t *         persisted\n\t */\n\tpublic boolean findAndReplaceMarkers(AIAnswer ans) {\n\t\tIWorkspaceRoot root = ResourcesPlugin.getWorkspace().getRoot();\n\t\tboolean res = false;\n\t\ttry {\n\t\t\tIMarker[] markers = root.findMarkers(MARKER_ID, true, IResource.DEPTH_INFINITE);\n\t\t\tfor (IMarker marker : markers)\n\t\t\t\tif (ans.id.equals(marker.getAttribute(MARKER_REQ_ID_ATTR)))\n\t\t\t\t\tres |= replaceMarker(ans, marker);\n\t\t} catch (CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\n\t\t// Fallback: the marker's stored position is outdated\n\t\tif (!res)\n\t\t\tres = findAndReplaceInOpenEditors(ans);\n\n\t\treturn res;\n\t}\n\n\tprivate boolean replaceMarker(AIAnswer ans, IMarker marker) {\n\t\tIResource resource = marker.getResource();\n\t\tif (!(resource instanceof IFile))\n\t\t\treturn false;\n\n\t\tIFile file = (IFile) resource;\n\t\tint line = marker.getAttribute(IMarker.LINE_NUMBER, -1);\n\t\tif (line <= 0)\n\t\t\treturn false;\n\n\t\tITextFileBufferManager bm = FileBuffers.getTextFileBufferManager();\n\t\tboolean[] replaced = { false };\n\t\ttry {\n\t\t\tbm.connect(file.getFullPath(), LocationKind.IFILE, null);\n\t\t\tITextFileBuffer tb = bm.getTextFileBuffer(file.getFullPath(), LocationKind.IFILE);\n\n\t\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\t\ttry {\n\t\t\t\t\tIDocument doc = tb.getDocument();\n\t\t\t\t\tint off = marker.getAttribute(MARKER_OFF_ID_ATTR, -1);\n\t\t\t\t\tint len = marker.getAttribute(MARKER_LEN_ID_ATTR, -1);\n\n\t\t\t\t\tint[] range = resolveTagRange(doc, ans.id, off, len);\n\t\t\t\t\tif (range == null)\n\t\t\t\t\t\treturn; // stored position no longer matches the live document\n\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\treplaced[0] = true;\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t});\n\n\t\t\tif (replaced[0]) {\n\t\t\t\ttb.commit(null, false);\n\t\t\t\tmarker.delete();\n\t\t\t\treturn true;\n\t\t\t}\n\n\t\t} catch (CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t} finally {\n\t\t\ttry {\n\t\t\t\tbm.disconnect(file.getFullPath(), LocationKind.IFILE, null);\n\t\t\t} catch (CoreException e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t}\n\t\t}\n\t\treturn false;\n\t}\n\n\t/**\n\t * Verifies that the given offset/length still points at the tag belonging to\n\t * the given request id. If it does not (e.g. because the document was edited in\n\t * the meantime and the marker position is stale) the whole document is searched\n\t * for the tag instead.\n\t *\n\t * @return an {offset, length} pair pointing at the current location of the tag\n\t *         in the document, or {@code null} if the tag can no longer be found.\n\t */\n\tprivate int[] resolveTagRange(IDocument doc, String requestId, int off, int len) {\n\t\tif (off >= 0 && len >= 0 && off + len <= doc.getLength()) {\n\t\t\ttry {\n\t\t\t\tString candidate = doc.get(off, len);\n\t\t\t\tMatcher m = pattern.matcher(candidate);\n\t\t\t\tif (m.matches() && requestId.equals(m.group(2)))\n\t\t\t\t\treturn new int[] { off, len };\n\t\t\t} catch (BadLocationException e) {\n\t\t\t\t// fall through to full-document search\n\t\t\t}\n\t\t}\n\t\treturn findTagInDocument(doc, requestId);\n\t}\n\n\t/**\n\t * Scans the full document content for the tag belonging to the given request\n\t * id.\n\t *\n\t * @return an {offset, length} pair, or {@code null} if not found.\n\t */\n\tprivate int[] findTagInDocument(IDocument doc, String requestId) {\n\t\tString content = doc.get();\n\t\tMatcher m = pattern.matcher(content);\n\t\twhile (m.find())\n\t\t\tif (requestId.equals(m.group(2)))\n\t\t\t\treturn new int[] { m.start(), m.end() - m.start() };\n\t\treturn null;\n\t}\n\n\t/**\n\t * Fallback used when the marker based replacement failed, e.g. because the\n\t * marker's stored offset is no longer in sync with the (still dirty) editor\n\t * content, or no marker exists at all yet. Searches all currently open text\n\t * editors for the tag belonging to the given request id and replaces it\n\t * directly in the editor's document. The editor is intentionally not saved so\n\t * that a parallel edit by the user is not disturbed.\n\t */\n\tprivate boolean findAndReplaceInOpenEditors(AIAnswer ans) {\n\t\tboolean[] res = { false };\n\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\tfor (ITextEditor editor : getOpenTextEditors()) {\n\t\t\t\tIDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());\n\t\t\t\tif (doc == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\tint[] range = findTagInDocument(doc, ans.id);\n\t\t\t\tif (range == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\ttry {\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\tres[0] = true;\n\t\t\t\t\tbreak;\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t\treturn res[0];\n\t}\n\n\tprivate java.util.List<ITextEditor> getOpenTextEditors() {\n\t\tjava.util.List<ITextEditor> editors = new java.util.ArrayList<>();\n\t\tfor (IWorkbenchWindow window : PlatformUI.getWorkbench().getWorkbenchWindows())\n\t\t\tfor (IWorkbenchPage page : window.getPages())\n\t\t\t\tfor (IEditorReference ref : page.getEditorReferences()) {\n\t\t\t\t\tIEditorPart part = ref.getEditor(false);\n\t\t\t\t\tif (part instanceof ITextEditor) {\n\t\t\t\t\t\tIEditorInput input = part.getEditorInput();\n\t\t\t\t\t\t// prefer file based / URI based editors, but any ITextEditor works\n\t\t\t\t\t\tif (input instanceof IFileEditorInput || input != null)\n\t\t\t\t\t\t\teditors.add((ITextEditor) part);\n\t\t\t\t\t}\n\t\t\t\t}\n\t\treturn editors;\n\t}\n\n\tpublic static String getPromptTag(String meta, String id) {\n\t\treturn String.format(\"[%s:%s:%s]\", AIREQ_PREFIX, meta, id);\n\t}\n}\n"
#:     }
#:   }
#: }
/allow 235284a4-102f-419f-83b7-3f8541249556
#: Control Request:
#: {
#:   "id" : "19195f1e-cb16-4bac-9e5d-1754b120770a",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "script" : "grep -n \"markerScanner\\|ConfigManager\\|getOuputMode\" -R /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java 2>/dev/null | head -60",
#:     "cwd" : "/home/user/xyan"
#:   }
#: }
/allow 19195f1e-cb16-4bac-9e5d-1754b120770a
#: Control Request:
#: {
#:   "id" : "d4ea96b2-3e14-45f4-9be4-418b67720ca8",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java:24:\tpublic ConfigManager cfg = new ConfigManager();\n/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java:34:\tpublic MarkerRessourceScanner markerScanner;\n/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java:40:\t\tmarkerScanner = new MarkerRessourceScanner(context);\n/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java:47:\t\tif (markerScanner != null)\n/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java:48:\t\t\tmarkerScanner.dispose(context);\n/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java:49:\t\tmarkerScanner = null;\n/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java:13:public class ConfigManager {\n/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/ConfigManager.java:246:\tpublic OutputMode getOuputMode() {\n"
#:     }
#:   }
#: }
/allow d4ea96b2-3e14-45f4-9be4-418b67720ca8
#: Control Request:
#: {
#:   "id" : "369a086e-aaa6-4897-bd32-bf59f7a03b0d",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "import org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;",
#:     "end" : "import org.eclipse.swt.widgets.Display;",
#:     "content" : "import org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.ITextSelection;\nimport org.eclipse.jface.viewers.ISelection;\nimport org.eclipse.swt.widgets.Display;"
#:   }
#: }
/allow 369a086e-aaa6-4897-bd32-bf59f7a03b0d
#: Control Request:
#: {
#:   "id" : "c5418529-c4dc-4032-8491-e1221f69ced1",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "import xy.ai.workbench.LOG;\nimport xy.ai.workbench.models.AIAnswer;",
#:     "end" : "import xy.ai.workbench.models.AIAnswer;",
#:     "content" : "import xy.ai.workbench.Activator;\nimport xy.ai.workbench.LOG;\nimport xy.ai.workbench.OutputMode;\nimport xy.ai.workbench.models.AIAnswer;"
#:   }
#: }
/allow c5418529-c4dc-4032-8491-e1221f69ced1
#: Control Request:
#: {
#:   "id" : "a5eca6c9-8044-4ed5-9ce8-869bf9ab8c14",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\t\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\t\ttry {\n\t\t\t\t\tIDocument doc = tb.getDocument();\n\t\t\t\t\tint off = marker.getAttribute(MARKER_OFF_ID_ATTR, -1);\n\t\t\t\t\tint len = marker.getAttribute(MARKER_LEN_ID_ATTR, -1);\n\n\t\t\t\t\tint[] range = resolveTagRange(doc, ans.id, off, len);\n\t\t\t\t\tif (range == null)\n\t\t\t\t\t\treturn; // stored position no longer matches the live document\n\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\treplaced[0] = true;\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t});",
#:     "end" : "\t\t\tif (replaced[0]) {",
#:     "content" : "\t\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\t\ttry {\n\t\t\t\t\tIDocument doc = tb.getDocument();\n\t\t\t\t\tint off = marker.getAttribute(MARKER_OFF_ID_ATTR, -1);\n\t\t\t\t\tint len = marker.getAttribute(MARKER_LEN_ID_ATTR, -1);\n\n\t\t\t\t\tint[] range = resolveTagRange(doc, ans.id, off, len);\n\t\t\t\t\tif (range == null)\n\t\t\t\t\t\treturn; // stored position no longer matches the live document\n\n\t\t\t\t\tITextEditor editor = findOpenEditorFor(file);\n\t\t\t\t\tboolean autoFollow = shouldAutoFollow(editor, doc);\n\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\treplaced[0] = true;\n\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t});\n\n\t\t\tif (replaced[0]) {"
#:   }
#: }
/allow a5eca6c9-8044-4ed5-9ce8-869bf9ab8c14
#: Control Request:
#: {
#:   "id" : "de2f453a-0121-4f1f-9e81-6b4a94515106",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\t\t\t\tint[] range = findTagInDocument(doc, ans.id);\n\t\t\t\tif (range == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\ttry {\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\tres[0] = true;\n\t\t\t\t\tbreak;\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}",
#:     "end" : "\t\t\t}\n\t\t});\n\t\treturn res[0];\n\t}",
#:     "content" : "\t\t\t\tint[] range = findTagInDocument(doc, ans.id);\n\t\t\t\tif (range == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\ttry {\n\t\t\t\t\tboolean autoFollow = shouldAutoFollow(editor, doc);\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\tres[0] = true;\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t\tbreak;\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t\treturn res[0];\n\t}\n\n\t/**\n\t * Finds the currently open text editor showing the given file, if any.\n\t */\n\tprivate ITextEditor findOpenEditorFor(IFile file) {\n\t\tfor (ITextEditor editor : getOpenTextEditors()) {\n\t\t\tIEditorInput input = editor.getEditorInput();\n\t\t\tif (input instanceof IFileEditorInput && file.equals(((IFileEditorInput) input).getFile()))\n\t\t\t\treturn editor;\n\t\t}\n\t\treturn null;\n\t}\n\n\t/**\n\t * Determines whether the \"autofollow\" comfort feature should be applied for\n\t * the given editor/document, i.e. whether the cursor should be moved to the\n\t * start of the last line after the tag has been replaced with the AI answer.\n\t *\n\t * This is only the case when:\n\t * <ul>\n\t * <li>the target file is currently open in an editor,</li>\n\t * <li>the editor is unmodified compared to the saved file (not dirty),</li>\n\t * <li>the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat}, and</li>\n\t * <li>the cursor is already positioned on the last line of the document.</li>\n\t * </ul>\n\t *\n\t * The check is done <b>before</b> the tag is replaced, so that inserting the\n\t * (possibly multi-line) answer does not affect the outcome.\n\t */\n\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {\n\t\tif (editor == null || editor.isDirty())\n\t\t\treturn false;\n\n\t\tOutputMode mode = Activator.getDefault().cfg.getOuputMode();\n\t\tif (mode != OutputMode.Append && mode != OutputMode.Chat)\n\t\t\treturn false;\n\n\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\t\tif (!(selection instanceof ITextSelection))\n\t\t\treturn false;\n\n\t\tITextSelection tsel = (ITextSelection) selection;\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\treturn tsel.getStartLine() >= lastLine;\n\t}\n\n\t/**\n\t * Places the cursor at the beginning of the (new) last line of the document,\n\t * used to implement the \"autofollow\" comfort feature.\n\t */\n\tprivate void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\ttry {\n\t\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\t\tint offset = doc.getLineOffset(lastLine);\n\t\t\teditor.selectAndReveal(offset, 0);\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}"
#:   }
#: }
/allow de2f453a-0121-4f1f-9e81-6b4a94515106
#: Control Request:
#: {
#:   "id" : "6f5930c9-092a-427a-927a-e9d9806c014f",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java"
#:   }
#: }
/allow 6f5930c9-092a-427a-927a-e9d9806c014f
#: Control Request:
#: {
#:   "id" : "9a8a5d14-4897-4738-a100-48272e53105d",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench.marker;\n\nimport java.io.BufferedReader;\nimport java.io.IOException;\nimport java.io.InputStreamReader;\nimport java.util.regex.Matcher;\nimport java.util.regex.Pattern;\n\nimport org.eclipse.core.filebuffers.FileBuffers;\nimport org.eclipse.core.filebuffers.ITextFileBuffer;\nimport org.eclipse.core.filebuffers.ITextFileBufferManager;\nimport org.eclipse.core.filebuffers.LocationKind;\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IMarker;\nimport org.eclipse.core.resources.IResource;\nimport org.eclipse.core.resources.IResourceChangeEvent;\nimport org.eclipse.core.resources.IResourceChangeListener;\nimport org.eclipse.core.resources.IResourceDelta;\nimport org.eclipse.core.resources.IResourceDeltaVisitor;\nimport org.eclipse.core.resources.IWorkspaceRoot;\nimport org.eclipse.core.resources.ResourcesPlugin;\nimport org.eclipse.core.runtime.CoreException;\nimport org.eclipse.core.runtime.Status;\nimport org.eclipse.core.runtime.jobs.Job;\nimport org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.ITextSelection;\nimport org.eclipse.jface.viewers.ISelection;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.IEditorInput;\nimport org.eclipse.ui.IEditorPart;\nimport org.eclipse.ui.IEditorReference;\nimport org.eclipse.ui.IFileEditorInput;\nimport org.eclipse.ui.IWorkbenchPage;\nimport org.eclipse.ui.IWorkbenchWindow;\nimport org.eclipse.ui.PlatformUI;\nimport org.eclipse.ui.texteditor.ITextEditor;\nimport org.osgi.framework.BundleContext;\n\nimport xy.ai.workbench.Activator;\nimport xy.ai.workbench.LOG;\nimport xy.ai.workbench.OutputMode;\nimport xy.ai.workbench.models.AIAnswer;\n\npublic class MarkerRessourceScanner implements IResourceChangeListener, IResourceDeltaVisitor {\n\tprivate static final String AIREQ_PREFIX = \"xy.ai.req\";\n\tprivate static final String MARKER_ID = \"xy.ai.workbench.promptmarker\";\n\tprivate static final String MARKER_REQ_ID_ATTR = \"requestId\";\n\tprivate static final String MARKER_OFF_ID_ATTR = \"offset\";\n\tprivate static final String MARKER_LEN_ID_ATTR = \"length\";\n\tprivate final Pattern pattern;\n\n\tpublic MarkerRessourceScanner(BundleContext context) {\n\t\tResourcesPlugin.getWorkspace().addResourceChangeListener(this, IResourceChangeEvent.POST_CHANGE);\n\t\tpattern = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\", Pattern.CASE_INSENSITIVE);\n\t}\n\n\tpublic void dispose(BundleContext context) {\n\t\tResourcesPlugin.getWorkspace().removeResourceChangeListener(this);\n\t}\n\n\t@Override\n\tpublic boolean visit(IResourceDelta delta) throws CoreException {\n\t\tIResource rsc = delta.getResource();\n\t\tif (rsc.getType() == IResource.FILE //\n\t\t\t\t&& rsc.getName().endsWith(\".md\")//\n\t\t\t\t&& delta.getKind() == IResourceDelta.CHANGED //\n\t\t\t\t&& rsc instanceof IFile //\n\t\t\t\t&& (delta.getFlags() & IResourceDelta.CONTENT) != 0)\n\t\t\tJob.create(\"Scanning file for markers\", mon -> {\n\t\t\t\trescannFile((IFile) rsc);\n\t\t\t\treturn Status.OK_STATUS;\n\t\t\t}).schedule();\n\t\treturn true;\n\t}\n\n\tprivate void rescannFile(IFile file) {\n\t\tint lineNumber = 1;\n\t\tint totaloff = 0;\n\n\t\ttry (BufferedReader reader = new BufferedReader(new InputStreamReader(file.getContents()))) {\n\t\t\tfile.deleteMarkers(MARKER_ID, false, IResource.DEPTH_ZERO);\n\n\t\t\tString line;\n\t\t\twhile ((line = reader.readLine()) != null) {\n\t\t\t\tMatcher m = pattern.matcher(line);\n\t\t\t\twhile (m.find()) {\n\t\t\t\t\tIMarker marker = file.createMarker(MARKER_ID);\n\t\t\t\t\tmarker.setAttribute(IMarker.MESSAGE, m.group(1) + \": \" + m.group(2));\n\t\t\t\t\tmarker.setAttribute(IMarker.LINE_NUMBER, lineNumber);\n\t\t\t\t\tmarker.setAttribute(MARKER_OFF_ID_ATTR, totaloff + m.start());\n\t\t\t\t\tmarker.setAttribute(MARKER_LEN_ID_ATTR, m.end() - m.start());\n\t\t\t\t\tmarker.setAttribute(IMarker.PRIORITY, IMarker.PRIORITY_NORMAL);\n\t\t\t\t\tmarker.setAttribute(MARKER_REQ_ID_ATTR, m.group(2));\n\t\t\t\t}\n\t\t\t\tlineNumber++;\n\t\t\t\ttotaloff += line.length() + 1;\n\t\t\t}\n\t\t} catch (IOException | CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}\n\n\t@Override\n\tpublic void resourceChanged(IResourceChangeEvent event) {\n\t\tif (event.getType() == IResourceChangeEvent.POST_CHANGE) {\n\t\t\ttry {\n\t\t\t\tevent.getDelta().accept(this);\n\t\t\t} catch (CoreException e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t}\n\t\t}\n\t}\n\n\t/**\n\t * @param ans\n\t * @return true when at least one marker was displayed and the AI answer is\n\t *         persisted\n\t */\n\tpublic boolean findAndReplaceMarkers(AIAnswer ans) {\n\t\tIWorkspaceRoot root = ResourcesPlugin.getWorkspace().getRoot();\n\t\tboolean res = false;\n\t\ttry {\n\t\t\tIMarker[] markers = root.findMarkers(MARKER_ID, true, IResource.DEPTH_INFINITE);\n\t\t\tfor (IMarker marker : markers)\n\t\t\t\tif (ans.id.equals(marker.getAttribute(MARKER_REQ_ID_ATTR)))\n\t\t\t\t\tres |= replaceMarker(ans, marker);\n\t\t} catch (CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\n\t\t// Fallback: the marker's stored position is outdated\n\t\tif (!res)\n\t\t\tres = findAndReplaceInOpenEditors(ans);\n\n\t\treturn res;\n\t}\n\n\tprivate boolean replaceMarker(AIAnswer ans, IMarker marker) {\n\t\tIResource resource = marker.getResource();\n\t\tif (!(resource instanceof IFile))\n\t\t\treturn false;\n\n\t\tIFile file = (IFile) resource;\n\t\tint line = marker.getAttribute(IMarker.LINE_NUMBER, -1);\n\t\tif (line <= 0)\n\t\t\treturn false;\n\n\t\tITextFileBufferManager bm = FileBuffers.getTextFileBufferManager();\n\t\tboolean[] replaced = { false };\n\t\ttry {\n\t\t\tbm.connect(file.getFullPath(), LocationKind.IFILE, null);\n\t\t\tITextFileBuffer tb = bm.getTextFileBuffer(file.getFullPath(), LocationKind.IFILE);\n\n\t\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\t\ttry {\n\t\t\t\t\tIDocument doc = tb.getDocument();\n\t\t\t\t\tint off = marker.getAttribute(MARKER_OFF_ID_ATTR, -1);\n\t\t\t\t\tint len = marker.getAttribute(MARKER_LEN_ID_ATTR, -1);\n\n\t\t\t\t\tint[] range = resolveTagRange(doc, ans.id, off, len);\n\t\t\t\t\tif (range == null)\n\t\t\t\t\t\treturn; // stored position no longer matches the live document\n\n\t\t\t\t\tITextEditor editor = findOpenEditorFor(file);\n\t\t\t\t\tboolean autoFollow = shouldAutoFollow(editor, doc);\n\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\treplaced[0] = true;\n\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t});\n\n\t\t\tif (replaced[0]) {\n\t\t\t\ttb.commit(null, false);\n\t\t\t\tmarker.delete();\n\t\t\t\treturn true;\n\t\t\t}\n\n\t\t} catch (CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t} finally {\n\t\t\ttry {\n\t\t\t\tbm.disconnect(file.getFullPath(), LocationKind.IFILE, null);\n\t\t\t} catch (CoreException e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t}\n\t\t}\n\t\treturn false;\n\t}\n\n\t/**\n\t * Verifies that the given offset/length still points at the tag belonging to\n\t * the given request id. If it does not (e.g. because the document was edited in\n\t * the meantime and the marker position is stale) the whole document is searched\n\t * for the tag instead.\n\t *\n\t * @return an {offset, length} pair pointing at the current location of the tag\n\t *         in the document, or {@code null} if the tag can no longer be found.\n\t */\n\tprivate int[] resolveTagRange(IDocument doc, String requestId, int off, int len) {\n\t\tif (off >= 0 && len >= 0 && off + len <= doc.getLength()) {\n\t\t\ttry {\n\t\t\t\tString candidate = doc.get(off, len);\n\t\t\t\tMatcher m = pattern.matcher(candidate);\n\t\t\t\tif (m.matches() && requestId.equals(m.group(2)))\n\t\t\t\t\treturn new int[] { off, len };\n\t\t\t} catch (BadLocationException e) {\n\t\t\t\t// fall through to full-document search\n\t\t\t}\n\t\t}\n\t\treturn findTagInDocument(doc, requestId);\n\t}\n\n\t/**\n\t * Scans the full document content for the tag belonging to the given request\n\t * id.\n\t *\n\t * @return an {offset, length} pair, or {@code null} if not found.\n\t */\n\tprivate int[] findTagInDocument(IDocument doc, String requestId) {\n\t\tString content = doc.get();\n\t\tMatcher m = pattern.matcher(content);\n\t\twhile (m.find())\n\t\t\tif (requestId.equals(m.group(2)))\n\t\t\t\treturn new int[] { m.start(), m.end() - m.start() };\n\t\treturn null;\n\t}\n\n\t/**\n\t * Fallback used when the marker based replacement failed, e.g. because the\n\t * marker's stored offset is no longer in sync with the (still dirty) editor\n\t * content, or no marker exists at all yet. Searches all currently open text\n\t * editors for the tag belonging to the given request id and replaces it\n\t * directly in the editor's document. The editor is intentionally not saved so\n\t * that a parallel edit by the user is not disturbed.\n\t */\n\tprivate boolean findAndReplaceInOpenEditors(AIAnswer ans) {\n\t\tboolean[] res = { false };\n\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\tfor (ITextEditor editor : getOpenTextEditors()) {\n\t\t\t\tIDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());\n\t\t\t\tif (doc == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\tint[] range = findTagInDocument(doc, ans.id);\n\t\t\t\tif (range == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\ttry {\n\t\t\t\t\tboolean autoFollow = shouldAutoFollow(editor, doc);\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\tres[0] = true;\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t\tbreak;\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t\treturn res[0];\n\t}\n\n\t/**\n\t * Finds the currently open text editor showing the given file, if any.\n\t */\n\tprivate ITextEditor findOpenEditorFor(IFile file) {\n\t\tfor (ITextEditor editor : getOpenTextEditors()) {\n\t\t\tIEditorInput input = editor.getEditorInput();\n\t\t\tif (input instanceof IFileEditorInput && file.equals(((IFileEditorInput) input).getFile()))\n\t\t\t\treturn editor;\n\t\t}\n\t\treturn null;\n\t}\n\n\t/**\n\t * Determines whether the \"autofollow\" comfort feature should be applied for\n\t * the given editor/document, i.e. whether the cursor should be moved to the\n\t * start of the last line after the tag has been replaced with the AI answer.\n\t *\n\t * This is only the case when:\n\t * <ul>\n\t * <li>the target file is currently open in an editor,</li>\n\t * <li>the editor is unmodified compared to the saved file (not dirty),</li>\n\t * <li>the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat}, and</li>\n\t * <li>the cursor is already positioned on the last line of the document.</li>\n\t * </ul>\n\t *\n\t * The check is done <b>before</b> the tag is replaced, so that inserting the\n\t * (possibly multi-line) answer does not affect the outcome.\n\t */\n\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {\n\t\tif (editor == null || editor.isDirty())\n\t\t\treturn false;\n\n\t\tOutputMode mode = Activator.getDefault().cfg.getOuputMode();\n\t\tif (mode != OutputMode.Append && mode != OutputMode.Chat)\n\t\t\treturn false;\n\n\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\t\tif (!(selection instanceof ITextSelection))\n\t\t\treturn false;\n\n\t\tITextSelection tsel = (ITextSelection) selection;\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\treturn tsel.getStartLine() >= lastLine;\n\t}\n\n\t/**\n\t * Places the cursor at the beginning of the (new) last line of the document,\n\t * used to implement the \"autofollow\" comfort feature.\n\t */\n\tprivate void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\ttry {\n\t\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\t\tint offset = doc.getLineOffset(lastLine);\n\t\t\teditor.selectAndReveal(offset, 0);\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}\n\n\tprivate java.util.List<ITextEditor> getOpenTextEditors() {\n\t\tjava.util.List<ITextEditor> editors = new java.util.ArrayList<>();\n\t\tfor (IWorkbenchWindow window : PlatformUI.getWorkbench().getWorkbenchWindows())\n\t\t\tfor (IWorkbenchPage page : window.getPages())\n\t\t\t\tfor (IEditorReference ref : page.getEditorReferences()) {\n\t\t\t\t\tIEditorPart part = ref.getEditor(false);\n\t\t\t\t\tif (part instanceof ITextEditor) {\n\t\t\t\t\t\tIEditorInput input = part.getEditorInput();\n\t\t\t\t\t\t// prefer file based / URI based editors, but any ITextEditor works\n\t\t\t\t\t\tif (input instanceof IFileEditorInput || input != null)\n\t\t\t\t\t\t\teditors.add((ITextEditor) part);\n\t\t\t\t\t}\n\t\t\t\t}\n\t\treturn editors;\n\t}\n\n\tpublic static String getPromptTag(String meta, String id) {\n\t\treturn String.format(\"[%s:%s:%s]\", AIREQ_PREFIX, meta, id);\n\t}\n}\n"
#:     }
#:   }
#: }
/allow 9a8a5d14-4897-4738-a100-48272e53105d
#: Control Request:
#: {
#:   "id" : "7f86b5b8-e850-420d-a9c7-63091a63b853",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "script" : "find /home/user/xyan -iname \"*.classpath\" -o -iname \"pom.xml\" | head; find / -iname \"org.eclipse.jface*.jar\" 2>/dev/null | head -3",
#:     "cwd" : "/home/user/xyan"
#:   }
#: }
/allow 7f86b5b8-e850-420d-a9c7-63091a63b853
#: Control Request:
#: {
#:   "id" : "7156eaa3-cbae-47f3-91f6-ca6ee5702885",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "/home/user/xyan/ccv2-project-template/core-customize/hybris/bin/custom/extras/modelt/.classpath\n/home/user/xyan/ccv2-project-template/core-customize/hybris/bin/custom/cloudhotfolders_2005/azurecloudhotfolder/.classpath\n/home/user/xyan/ccv2-project-template/core-customize/hybris/bin/custom/cloudhotfolders_2005/cloudhotfolder/.classpath\n/home/user/xyan/ccv2-project-template/core-customize/hybris/bin/custom/cloudhotfolders_2005/cloudcommons/.classpath\n/home/user/xyan/xy.ai.workbench/.classpath\n/usr/share/dbeaver-ce/plugins/org.eclipse.jface_3.34.0.v20240425-1100.jar\n/usr/share/dbeaver-ce/plugins/org.eclipse.jface.databinding_1.15.300.v20240424-0444.jar\n/usr/share/dbeaver-ce/plugins/org.eclipse.jface.notifications.nls_0.7.200.v20240321-1245.jar\n"
#:     }
#:   }
#: }
/allow 7156eaa3-cbae-47f3-91f6-ca6ee5702885
#: ReasoningToken: 198
Ich habe geprüft, dass sich das Projekt als Eclipse-PDE-Plugin nicht trivial per `javac`/Maven bauen lässt (keine Maven/Tycho-Konfiguration mit klassifizierten Abhängigkeiten vorhanden). Die Änderung wurde daher sorgfältig manuell durchgesehen; Syntax und Imports sind konsistent.

**Zusammenfassung der Änderung** (`MarkerRessourceScanner.java`):

1. Neue Imports: `ITextSelection`, `ISelection`, `Activator`, `OutputMode`.
2. Neue Hilfsmethode `findOpenEditorFor(IFile)` – findet den ggf. offenen `ITextEditor` für die Zieldatei.
3. Neue Hilfsmethode `shouldAutoFollow(ITextEditor, IDocument)` – prüft **vor** dem Ersetzen des Tags:
   - Editor ist geöffnet (`editor != null`)
   - Editor ist nicht dirty (`!editor.isDirty()`, entspricht "unmodified gegenüber gespeicherter Datei")
   - `OutputMode` ist `Append` oder `Chat`
   - Cursor (Selektionsstart) befindet sich bereits in der letzten Zeile des Dokuments
4. Neue Hilfsmethode `moveCursorToLastLineStart(ITextEditor, IDocument)` – setzt nach dem Replace den Cursor per `selectAndReveal` an den Anfang der (neuen) letzten Zeile.
5. In `replaceMarker(...)` (dem regulären Pfad über die Marker/`ITextFileBuffer`) und in `findAndReplaceInOpenEditors(...)` (Fallback-Pfad für offene Editoren) wird jeweils:
   - vor dem `doc.replace(...)` per `shouldAutoFollow(...)` geprüft, ob Autofollow greifen soll,
   - nach erfolgreichem `doc.replace(...)` bei positivem Ergebnis `moveCursorToLastLineStart(...)` aufgerufen.

Damit wird die gewünschte Autofollow-Komfortfunktion umgesetzt, ohne den Nutzer zu stören, wenn er die Datei gerade selbst bearbeitet (dirty), einen anderen Output-Modus nutzt oder der Cursor nicht in der letzten Zeile steht.
Kehre die Reihenfolge um. Der Editor wird erst benötigt, wenn "autoFollow" ermittelt wurde und zutrifft.
#: Control Request:
#: {
#:   "id" : "ab9a3194-2dba-46d3-83e1-fe530fd7c233",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "private boolean replaceMarker(AIAnswer ans, IMarker marker) {",
#:     "end" : "\t}\n\n\t/**\n\t * Verifies that the given offset/length still points at the tag belonging to"
#:   }
#: }
/allow ab9a3194-2dba-46d3-83e1-fe530fd7c233
#: Control Request:
#: {
#:   "id" : "6803c6f5-4d60-43ef-b02c-95b6771421ef",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "private boolean replaceMarker(AIAnswer ans, IMarker marker) {\n\t\tIResource resource = marker.getResource();\n\t\tif (!(resource instanceof IFile))\n\t\t\treturn false;\n\n\t\tIFile file = (IFile) resource;\n\t\tint line = marker.getAttribute(IMarker.LINE_NUMBER, -1);\n\t\tif (line <= 0)\n\t\t\treturn false;\n\n\t\tITextFileBufferManager bm = FileBuffers.getTextFileBufferManager();\n\t\tboolean[] replaced = { false };\n\t\ttry {\n\t\t\tbm.connect(file.getFullPath(), LocationKind.IFILE, null);\n\t\t\tITextFileBuffer tb = bm.getTextFileBuffer(file.getFullPath(), LocationKind.IFILE);\n\n\t\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\t\ttry {\n\t\t\t\t\tIDocument doc = tb.getDocument();\n\t\t\t\t\tint off = marker.getAttribute(MARKER_OFF_ID_ATTR, -1);\n\t\t\t\t\tint len = marker.getAttribute(MARKER_LEN_ID_ATTR, -1);\n\n\t\t\t\t\tint[] range = resolveTagRange(doc, ans.id, off, len);\n\t\t\t\t\tif (range == null)\n\t\t\t\t\t\treturn; // stored position no longer matches the live document\n\n\t\t\t\t\tITextEditor editor = findOpenEditorFor(file);\n\t\t\t\t\tboolean autoFollow = shouldAutoFollow(editor, doc);\n\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\treplaced[0] = true;\n\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t});\n\n\t\t\tif (replaced[0]) {\n\t\t\t\ttb.commit(null, false);\n\t\t\t\tmarker.delete();\n\t\t\t\treturn true;\n\t\t\t}\n\n\t\t} catch (CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t} finally {\n\t\t\ttry {\n\t\t\t\tbm.disconnect(file.getFullPath(), LocationKind.IFILE, null);\n\t\t\t} catch (CoreException e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t}\n\t\t}\n\t\treturn false;\n\t}\n\n\t/**\n\t * Verifies that the given offset/length still points at the tag belonging to",
#:       "start_line" : 139,
#:       "end_line" : 197
#:     }
#:   }
#: }
/allow 6803c6f5-4d60-43ef-b02c-95b6771421ef
#: Control Request:
#: {
#:   "id" : "32013d75-2159-4b1a-915b-9e5712dda058",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\t\t\t\t\tITextEditor editor = findOpenEditorFor(file);\n\t\t\t\t\tboolean autoFollow = shouldAutoFollow(editor, doc);\n\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\treplaced[0] = true;\n\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);",
#:     "end" : "\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t});\n\n\t\t\tif (replaced[0]) {",
#:     "content" : "\t\t\t\t\tITextEditor editor = null;\n\t\t\t\t\tif (isAutoFollowModeEnabled())\n\t\t\t\t\t\teditor = findOpenEditorFor(file);\n\t\t\t\t\tboolean autoFollow = shouldAutoFollow(editor, doc);\n\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\treplaced[0] = true;\n\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t});\n\n\t\t\tif (replaced[0]) {"
#:   }
#: }
/allow 32013d75-2159-4b1a-915b-9e5712dda058
#: Control Request:
#: {
#:   "id" : "af084b88-95a8-49c9-a4ea-68a04e67e316",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\t\t\t\ttry {\n\t\t\t\t\tboolean autoFollow = shouldAutoFollow(editor, doc);\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);",
#:     "end" : "\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t\tbreak;",
#:     "content" : "\t\t\t\ttry {\n\t\t\t\t\tboolean autoFollow = isAutoFollowModeEnabled() && shouldAutoFollow(editor, doc);\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\tres[0] = true;\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t\tbreak;"
#:   }
#: }
/allow af084b88-95a8-49c9-a4ea-68a04e67e316
#: Control Request:
#: {
#:   "id" : "80ef287d-73f5-4453-a086-74358e382967",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "private boolean findAndReplaceInOpenEditors",
#:     "end" : "\t/**\n\t * Finds the currently open text editor showing the given file, if any.\n\t */"
#:   }
#: }
/allow 80ef287d-73f5-4453-a086-74358e382967
#: Control Request:
#: {
#:   "id" : "a5d13f84-b9ae-4dbd-8f13-93e08d126aa1",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "private boolean findAndReplaceInOpenEditors(AIAnswer ans) {\n\t\tboolean[] res = { false };\n\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\tfor (ITextEditor editor : getOpenTextEditors()) {\n\t\t\t\tIDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());\n\t\t\t\tif (doc == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\tint[] range = findTagInDocument(doc, ans.id);\n\t\t\t\tif (range == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\ttry {\n\t\t\t\t\tboolean autoFollow = isAutoFollowModeEnabled() && shouldAutoFollow(editor, doc);\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\tres[0] = true;\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t\tbreak;\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t\treturn res[0];\n\t}\n\n\t/**\n\t * Finds the currently open text editor showing the given file, if any.\n\t */",
#:       "start_line" : 244,
#:       "end_line" : 273
#:     }
#:   }
#: }
/allow a5d13f84-b9ae-4dbd-8f13-93e08d126aa1
#: Control Request:
#: {
#:   "id" : "2e877ac2-affc-433c-8e21-00ab5fc68801",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {",
#:     "end" : "\t\treturn tsel.getStartLine() >= lastLine;\n\t}"
#:   }
#: }
/allow 2e877ac2-affc-433c-8e21-00ab5fc68801
#: Control Request:
#: {
#:   "id" : "ea157043-061c-4de9-af1c-5e379c2be48c",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {\n\t\tif (editor == null || editor.isDirty())\n\t\t\treturn false;\n\n\t\tOutputMode mode = Activator.getDefault().cfg.getOuputMode();\n\t\tif (mode != OutputMode.Append && mode != OutputMode.Chat)\n\t\t\treturn false;\n\n\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\t\tif (!(selection instanceof ITextSelection))\n\t\t\treturn false;\n\n\t\tITextSelection tsel = (ITextSelection) selection;\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\treturn tsel.getStartLine() >= lastLine;\n\t}",
#:       "start_line" : 300,
#:       "end_line" : 315
#:     }
#:   }
#: }
/allow ea157043-061c-4de9-af1c-5e379c2be48c
#: Control Request:
#: {
#:   "id" : "b53f8f4c-4d96-4b30-9767-ddb3aba13802",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {\n\t\tif (editor == null || editor.isDirty())\n\t\t\treturn false;\n\n\t\tOutputMode mode = Activator.getDefault().cfg.getOuputMode();\n\t\tif (mode != OutputMode.Append && mode != OutputMode.Chat)\n\t\t\treturn false;\n\n\t\tISelection selection = editor.getSelectionProvider().getSelection();",
#:     "end" : "\t\treturn tsel.getStartLine() >= lastLine;\n\t}",
#:     "content" : "\t/**\n\t * Cheap, editor-independent pre-check for the \"autofollow\" comfort feature:\n\t * only when the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat} is it worth looking up the (potentially costly to\n\t * find) open editor for the target file at all.\n\t */\n\tprivate boolean isAutoFollowModeEnabled() {\n\t\tOutputMode mode = Activator.getDefault().cfg.getOuputMode();\n\t\treturn mode == OutputMode.Append || mode == OutputMode.Chat;\n\t}\n\n\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {\n\t\tif (editor == null || editor.isDirty())\n\t\t\treturn false;\n\n\t\tISelection selection = editor.getSelectionProvider().getSelection();"
#:   }
#: }
/allow b53f8f4c-4d96-4b30-9767-ddb3aba13802
#: Control Request:
#: {
#:   "id" : "62fbe093-eee2-43cf-b661-0f4f6489ade9",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\t/**\n\t * Determines whether the \"autofollow\" comfort feature should be applied for",
#:     "end" : "\tprivate boolean isAutoFollowModeEnabled() {"
#:   }
#: }
/allow 62fbe093-eee2-43cf-b661-0f4f6489ade9
#: Control Request:
#: {
#:   "id" : "7cbf78d7-953d-423f-817d-e407733b21ca",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\t/**\n\t * Determines whether the \"autofollow\" comfort feature should be applied for\n\t * the given editor/document, i.e. whether the cursor should be moved to the\n\t * start of the last line after the tag has been replaced with the AI answer.\n\t *\n\t * This is only the case when:\n\t * <ul>\n\t * <li>the target file is currently open in an editor,</li>\n\t * <li>the editor is unmodified compared to the saved file (not dirty),</li>\n\t * <li>the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat}, and</li>\n\t * <li>the cursor is already positioned on the last line of the document.</li>\n\t * </ul>\n\t *\n\t * The check is done <b>before</b> the tag is replaced, so that inserting the\n\t * (possibly multi-line) answer does not affect the outcome.\n\t */\n\t/**\n\t * Cheap, editor-independent pre-check for the \"autofollow\" comfort feature:\n\t * only when the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat} is it worth looking up the (potentially costly to\n\t * find) open editor for the target file at all.\n\t */\n\tprivate boolean isAutoFollowModeEnabled() {",
#:       "start_line" : 283,
#:       "end_line" : 306
#:     }
#:   }
#: }
/allow 7cbf78d7-953d-423f-817d-e407733b21ca
#: Control Request:
#: {
#:   "id" : "2f41a603-db0a-4751-8cf2-1c706187a0ec",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\t/**\n\t * Determines whether the \"autofollow\" comfort feature should be applied for\n\t * the given editor/document, i.e. whether the cursor should be moved to the\n\t * start of the last line after the tag has been replaced with the AI answer.\n\t *\n\t * This is only the case when:\n\t * <ul>\n\t * <li>the target file is currently open in an editor,</li>\n\t * <li>the editor is unmodified compared to the saved file (not dirty),</li>\n\t * <li>the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat}, and</li>\n\t * <li>the cursor is already positioned on the last line of the document.</li>\n\t * </ul>\n\t *\n\t * The check is done <b>before</b> the tag is replaced, so that inserting the\n\t * (possibly multi-line) answer does not affect the outcome.\n\t */\n\t/**\n\t * Cheap, editor-independent pre-check for the \"autofollow\" comfort feature:\n\t * only when the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat} is it worth looking up the (potentially costly to\n\t * find) open editor for the target file at all.\n\t */\n\tprivate boolean isAutoFollowModeEnabled() {",
#:     "end" : "\tprivate boolean isAutoFollowModeEnabled() {",
#:     "content" : "\t/**\n\t * Cheap, editor-independent pre-check for the \"autofollow\" comfort feature:\n\t * only when the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat} is it worth looking up the (potentially costly to\n\t * find) open editor for the target file at all. Callers are expected to only\n\t * look up the {@link ITextEditor} (via {@link #findOpenEditorFor(IFile)}) and\n\t * call {@link #shouldAutoFollow(ITextEditor, IDocument)} once this method\n\t * returned {@code true}.\n\t */\n\tprivate boolean isAutoFollowModeEnabled() {"
#:   }
#: }
/allow 2f41a603-db0a-4751-8cf2-1c706187a0ec
#: Control Request:
#: {
#:   "id" : "24699959-6d2b-4525-a894-faced7e4c1de",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\tprivate boolean isAutoFollowModeEnabled() {",
#:     "end" : "\t\treturn tsel.getStartLine() >= lastLine;\n\t}"
#:   }
#: }
/allow 24699959-6d2b-4525-a894-faced7e4c1de
#: Control Request:
#: {
#:   "id" : "a967cde6-47c7-4930-93ad-9d83528b03ef",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "error" : "End marker not found in file: '\\t\\treturn tsel.getStartLine() >= lastLine;\\n\\t}'"
#:     },
#:     "isError" : true
#:   }
#: }
/allow a967cde6-47c7-4930-93ad-9d83528b03ef
#: Control Request:
#: {
#:   "id" : "32602e59-792a-44a1-a136-bec77e14ef7d",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "min_line" : 280,
#:     "max_line" : 330
#:   }
#: }
/allow 32602e59-792a-44a1-a136-bec77e14ef7d
#: Control Request:
#: {
#:   "id" : "26baf508-7a11-4921-bac3-9d240b40d771",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\t\treturn null;\n\t}\n\n\t/**\n\t * Cheap, editor-independent pre-check for the \"autofollow\" comfort feature:\n\t * only when the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat} is it worth looking up the (potentially costly to\n\t * find) open editor for the target file at all. Callers are expected to only\n\t * look up the {@link ITextEditor} (via {@link #findOpenEditorFor(IFile)}) and\n\t * call {@link #shouldAutoFollow(ITextEditor, IDocument)} once this method\n\t * returned {@code true}.\n\t */\n\tprivate boolean isAutoFollowModeEnabled() {\n\t\tOutputMode mode = Activator.getDefault().cfg.getOuputMode();\n\t\treturn mode == OutputMode.Append || mode == OutputMode.Chat;\n\t}\n\n\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {\n\t\tif (editor == null || editor.isDirty())\n\t\t\treturn false;\n\n\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\n\t/**\n\t * Places the cursor at the beginning of the (new) last line of the document,\n\t * used to implement the \"autofollow\" comfort feature.\n\t */\n\tprivate void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\ttry {\n\t\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\t\tint offset = doc.getLineOffset(lastLine);\n\t\t\teditor.selectAndReveal(offset, 0);\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}\n\n\tprivate java.util.List<ITextEditor> getOpenTextEditors() {\n\t\tjava.util.List<ITextEditor> editors = new java.util.ArrayList<>();\n\t\tfor (IWorkbenchWindow window : PlatformUI.getWorkbench().getWorkbenchWindows())\n\t\t\tfor (IWorkbenchPage page : window.getPages())\n\t\t\t\tfor (IEditorReference ref : page.getEditorReferences()) {\n\t\t\t\t\tIEditorPart part = ref.getEditor(false);\n\t\t\t\t\tif (part instanceof ITextEditor) {\n\t\t\t\t\t\tIEditorInput input = part.getEditorInput();\n\t\t\t\t\t\t// prefer file based / URI based editors, but any ITextEditor works\n\t\t\t\t\t\tif (input instanceof IFileEditorInput || input != null)\n\t\t\t\t\t\t\teditors.add((ITextEditor) part);\n\t\t\t\t\t}\n\t\t\t\t}\n\t\treturn editors;\n",
#:       "min_line" : 280,
#:       "max_line" : 330
#:     }
#:   }
#: }
/allow 26baf508-7a11-4921-bac3-9d240b40d771
#: Control Request:
#: {
#:   "id" : "241b6202-8399-4f95-9f65-0876d259e286",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
#:     "start" : "\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {\n\t\tif (editor == null || editor.isDirty())\n\t\t\treturn false;\n\n\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\n\t/**\n\t * Places the cursor at the beginning of the (new) last line of the document,",
#:     "end" : "\t * used to implement the \"autofollow\" comfort feature.\n\t */",
#:     "content" : "\t/**\n\t * Determines whether the \"autofollow\" comfort feature should be applied for\n\t * the given editor/document, i.e. whether the cursor should be moved to the\n\t * start of the last line after the tag has been replaced with the AI answer.\n\t *\n\t * This assumes {@link #isAutoFollowModeEnabled()} has already been checked\n\t * (and returned {@code true}) and additionally requires that:\n\t * <ul>\n\t * <li>the target file is currently open in an editor (i.e. {@code editor} is\n\t * not {@code null}),</li>\n\t * <li>the editor is unmodified compared to the saved file (not dirty), and</li>\n\t * <li>the cursor is already positioned on the last line of the document.</li>\n\t * </ul>\n\t *\n\t * The check is done <b>before</b> the tag is replaced, so that inserting the\n\t * (possibly multi-line) answer does not affect the outcome.\n\t */\n\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {\n\t\tif (editor == null || editor.isDirty())\n\t\t\treturn false;\n\n\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\t\tif (!(selection instanceof ITextSelection))\n\t\t\treturn false;\n\n\t\tITextSelection tsel = (ITextSelection) selection;\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\treturn tsel.getStartLine() >= lastLine;\n\t}\n\n\t/**\n\t * Places the cursor at the beginning of the (new) last line of the document,\n\t * used to implement the \"autofollow\" comfort feature.\n\t */"
#:   }
#: }
/allow 241b6202-8399-4f95-9f65-0876d259e286
#: Control Request:
#: {
#:   "id" : "45a1d3dc-76a0-41c0-9627-526ee7d140e3",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java"
#:   }
#: }
/allow 45a1d3dc-76a0-41c0-9627-526ee7d140e3
#: Control Request:
#: {
#:   "id" : "895a3bd3-2276-4f1b-aa19-6975fb4cb92f",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench.marker;\n\nimport java.io.BufferedReader;\nimport java.io.IOException;\nimport java.io.InputStreamReader;\nimport java.util.regex.Matcher;\nimport java.util.regex.Pattern;\n\nimport org.eclipse.core.filebuffers.FileBuffers;\nimport org.eclipse.core.filebuffers.ITextFileBuffer;\nimport org.eclipse.core.filebuffers.ITextFileBufferManager;\nimport org.eclipse.core.filebuffers.LocationKind;\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IMarker;\nimport org.eclipse.core.resources.IResource;\nimport org.eclipse.core.resources.IResourceChangeEvent;\nimport org.eclipse.core.resources.IResourceChangeListener;\nimport org.eclipse.core.resources.IResourceDelta;\nimport org.eclipse.core.resources.IResourceDeltaVisitor;\nimport org.eclipse.core.resources.IWorkspaceRoot;\nimport org.eclipse.core.resources.ResourcesPlugin;\nimport org.eclipse.core.runtime.CoreException;\nimport org.eclipse.core.runtime.Status;\nimport org.eclipse.core.runtime.jobs.Job;\nimport org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.ITextSelection;\nimport org.eclipse.jface.viewers.ISelection;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.IEditorInput;\nimport org.eclipse.ui.IEditorPart;\nimport org.eclipse.ui.IEditorReference;\nimport org.eclipse.ui.IFileEditorInput;\nimport org.eclipse.ui.IWorkbenchPage;\nimport org.eclipse.ui.IWorkbenchWindow;\nimport org.eclipse.ui.PlatformUI;\nimport org.eclipse.ui.texteditor.ITextEditor;\nimport org.osgi.framework.BundleContext;\n\nimport xy.ai.workbench.Activator;\nimport xy.ai.workbench.LOG;\nimport xy.ai.workbench.OutputMode;\nimport xy.ai.workbench.models.AIAnswer;\n\npublic class MarkerRessourceScanner implements IResourceChangeListener, IResourceDeltaVisitor {\n\tprivate static final String AIREQ_PREFIX = \"xy.ai.req\";\n\tprivate static final String MARKER_ID = \"xy.ai.workbench.promptmarker\";\n\tprivate static final String MARKER_REQ_ID_ATTR = \"requestId\";\n\tprivate static final String MARKER_OFF_ID_ATTR = \"offset\";\n\tprivate static final String MARKER_LEN_ID_ATTR = \"length\";\n\tprivate final Pattern pattern;\n\n\tpublic MarkerRessourceScanner(BundleContext context) {\n\t\tResourcesPlugin.getWorkspace().addResourceChangeListener(this, IResourceChangeEvent.POST_CHANGE);\n\t\tpattern = Pattern.compile(\"\\\\[\" + AIREQ_PREFIX + \":(.*):(.*)\\\\]\", Pattern.CASE_INSENSITIVE);\n\t}\n\n\tpublic void dispose(BundleContext context) {\n\t\tResourcesPlugin.getWorkspace().removeResourceChangeListener(this);\n\t}\n\n\t@Override\n\tpublic boolean visit(IResourceDelta delta) throws CoreException {\n\t\tIResource rsc = delta.getResource();\n\t\tif (rsc.getType() == IResource.FILE //\n\t\t\t\t&& rsc.getName().endsWith(\".md\")//\n\t\t\t\t&& delta.getKind() == IResourceDelta.CHANGED //\n\t\t\t\t&& rsc instanceof IFile //\n\t\t\t\t&& (delta.getFlags() & IResourceDelta.CONTENT) != 0)\n\t\t\tJob.create(\"Scanning file for markers\", mon -> {\n\t\t\t\trescannFile((IFile) rsc);\n\t\t\t\treturn Status.OK_STATUS;\n\t\t\t}).schedule();\n\t\treturn true;\n\t}\n\n\tprivate void rescannFile(IFile file) {\n\t\tint lineNumber = 1;\n\t\tint totaloff = 0;\n\n\t\ttry (BufferedReader reader = new BufferedReader(new InputStreamReader(file.getContents()))) {\n\t\t\tfile.deleteMarkers(MARKER_ID, false, IResource.DEPTH_ZERO);\n\n\t\t\tString line;\n\t\t\twhile ((line = reader.readLine()) != null) {\n\t\t\t\tMatcher m = pattern.matcher(line);\n\t\t\t\twhile (m.find()) {\n\t\t\t\t\tIMarker marker = file.createMarker(MARKER_ID);\n\t\t\t\t\tmarker.setAttribute(IMarker.MESSAGE, m.group(1) + \": \" + m.group(2));\n\t\t\t\t\tmarker.setAttribute(IMarker.LINE_NUMBER, lineNumber);\n\t\t\t\t\tmarker.setAttribute(MARKER_OFF_ID_ATTR, totaloff + m.start());\n\t\t\t\t\tmarker.setAttribute(MARKER_LEN_ID_ATTR, m.end() - m.start());\n\t\t\t\t\tmarker.setAttribute(IMarker.PRIORITY, IMarker.PRIORITY_NORMAL);\n\t\t\t\t\tmarker.setAttribute(MARKER_REQ_ID_ATTR, m.group(2));\n\t\t\t\t}\n\t\t\t\tlineNumber++;\n\t\t\t\ttotaloff += line.length() + 1;\n\t\t\t}\n\t\t} catch (IOException | CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}\n\n\t@Override\n\tpublic void resourceChanged(IResourceChangeEvent event) {\n\t\tif (event.getType() == IResourceChangeEvent.POST_CHANGE) {\n\t\t\ttry {\n\t\t\t\tevent.getDelta().accept(this);\n\t\t\t} catch (CoreException e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t}\n\t\t}\n\t}\n\n\t/**\n\t * @param ans\n\t * @return true when at least one marker was displayed and the AI answer is\n\t *         persisted\n\t */\n\tpublic boolean findAndReplaceMarkers(AIAnswer ans) {\n\t\tIWorkspaceRoot root = ResourcesPlugin.getWorkspace().getRoot();\n\t\tboolean res = false;\n\t\ttry {\n\t\t\tIMarker[] markers = root.findMarkers(MARKER_ID, true, IResource.DEPTH_INFINITE);\n\t\t\tfor (IMarker marker : markers)\n\t\t\t\tif (ans.id.equals(marker.getAttribute(MARKER_REQ_ID_ATTR)))\n\t\t\t\t\tres |= replaceMarker(ans, marker);\n\t\t} catch (CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\n\t\t// Fallback: the marker's stored position is outdated\n\t\tif (!res)\n\t\t\tres = findAndReplaceInOpenEditors(ans);\n\n\t\treturn res;\n\t}\n\n\tprivate boolean replaceMarker(AIAnswer ans, IMarker marker) {\n\t\tIResource resource = marker.getResource();\n\t\tif (!(resource instanceof IFile))\n\t\t\treturn false;\n\n\t\tIFile file = (IFile) resource;\n\t\tint line = marker.getAttribute(IMarker.LINE_NUMBER, -1);\n\t\tif (line <= 0)\n\t\t\treturn false;\n\n\t\tITextFileBufferManager bm = FileBuffers.getTextFileBufferManager();\n\t\tboolean[] replaced = { false };\n\t\ttry {\n\t\t\tbm.connect(file.getFullPath(), LocationKind.IFILE, null);\n\t\t\tITextFileBuffer tb = bm.getTextFileBuffer(file.getFullPath(), LocationKind.IFILE);\n\n\t\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\t\ttry {\n\t\t\t\t\tIDocument doc = tb.getDocument();\n\t\t\t\t\tint off = marker.getAttribute(MARKER_OFF_ID_ATTR, -1);\n\t\t\t\t\tint len = marker.getAttribute(MARKER_LEN_ID_ATTR, -1);\n\n\t\t\t\t\tint[] range = resolveTagRange(doc, ans.id, off, len);\n\t\t\t\t\tif (range == null)\n\t\t\t\t\t\treturn; // stored position no longer matches the live document\n\n\t\t\t\t\tITextEditor editor = null;\n\t\t\t\t\tif (isAutoFollowModeEnabled())\n\t\t\t\t\t\teditor = findOpenEditorFor(file);\n\t\t\t\t\tboolean autoFollow = shouldAutoFollow(editor, doc);\n\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\treplaced[0] = true;\n\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t});\n\n\t\t\tif (replaced[0]) {\n\t\t\t\ttb.commit(null, false);\n\t\t\t\tmarker.delete();\n\t\t\t\treturn true;\n\t\t\t}\n\n\t\t} catch (CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t} finally {\n\t\t\ttry {\n\t\t\t\tbm.disconnect(file.getFullPath(), LocationKind.IFILE, null);\n\t\t\t} catch (CoreException e) {\n\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t}\n\t\t}\n\t\treturn false;\n\t}\n\n\t/**\n\t * Verifies that the given offset/length still points at the tag belonging to\n\t * the given request id. If it does not (e.g. because the document was edited in\n\t * the meantime and the marker position is stale) the whole document is searched\n\t * for the tag instead.\n\t *\n\t * @return an {offset, length} pair pointing at the current location of the tag\n\t *         in the document, or {@code null} if the tag can no longer be found.\n\t */\n\tprivate int[] resolveTagRange(IDocument doc, String requestId, int off, int len) {\n\t\tif (off >= 0 && len >= 0 && off + len <= doc.getLength()) {\n\t\t\ttry {\n\t\t\t\tString candidate = doc.get(off, len);\n\t\t\t\tMatcher m = pattern.matcher(candidate);\n\t\t\t\tif (m.matches() && requestId.equals(m.group(2)))\n\t\t\t\t\treturn new int[] { off, len };\n\t\t\t} catch (BadLocationException e) {\n\t\t\t\t// fall through to full-document search\n\t\t\t}\n\t\t}\n\t\treturn findTagInDocument(doc, requestId);\n\t}\n\n\t/**\n\t * Scans the full document content for the tag belonging to the given request\n\t * id.\n\t *\n\t * @return an {offset, length} pair, or {@code null} if not found.\n\t */\n\tprivate int[] findTagInDocument(IDocument doc, String requestId) {\n\t\tString content = doc.get();\n\t\tMatcher m = pattern.matcher(content);\n\t\twhile (m.find())\n\t\t\tif (requestId.equals(m.group(2)))\n\t\t\t\treturn new int[] { m.start(), m.end() - m.start() };\n\t\treturn null;\n\t}\n\n\t/**\n\t * Fallback used when the marker based replacement failed, e.g. because the\n\t * marker's stored offset is no longer in sync with the (still dirty) editor\n\t * content, or no marker exists at all yet. Searches all currently open text\n\t * editors for the tag belonging to the given request id and replaces it\n\t * directly in the editor's document. The editor is intentionally not saved so\n\t * that a parallel edit by the user is not disturbed.\n\t */\n\tprivate boolean findAndReplaceInOpenEditors(AIAnswer ans) {\n\t\tboolean[] res = { false };\n\t\tDisplay.getDefault().syncExec(() -> {\n\t\t\tfor (ITextEditor editor : getOpenTextEditors()) {\n\t\t\t\tIDocument doc = editor.getDocumentProvider().getDocument(editor.getEditorInput());\n\t\t\t\tif (doc == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\tint[] range = findTagInDocument(doc, ans.id);\n\t\t\t\tif (range == null)\n\t\t\t\t\tcontinue;\n\n\t\t\t\ttry {\n\t\t\t\t\tboolean autoFollow = isAutoFollowModeEnabled() && shouldAutoFollow(editor, doc);\n\t\t\t\t\tdoc.replace(range[0], range[1], ans.answer);\n\t\t\t\t\tres[0] = true;\n\t\t\t\t\tif (autoFollow)\n\t\t\t\t\t\tmoveCursorToLastLineStart(editor, doc);\n\t\t\t\t\tbreak;\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.error(e.getMessage(), e);\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t\treturn res[0];\n\t}\n\n\t/**\n\t * Finds the currently open text editor showing the given file, if any.\n\t */\n\tprivate ITextEditor findOpenEditorFor(IFile file) {\n\t\tfor (ITextEditor editor : getOpenTextEditors()) {\n\t\t\tIEditorInput input = editor.getEditorInput();\n\t\t\tif (input instanceof IFileEditorInput && file.equals(((IFileEditorInput) input).getFile()))\n\t\t\t\treturn editor;\n\t\t}\n\t\treturn null;\n\t}\n\n\t/**\n\t * Cheap, editor-independent pre-check for the \"autofollow\" comfort feature:\n\t * only when the configured output mode is {@link OutputMode#Append} or\n\t * {@link OutputMode#Chat} is it worth looking up the (potentially costly to\n\t * find) open editor for the target file at all. Callers are expected to only\n\t * look up the {@link ITextEditor} (via {@link #findOpenEditorFor(IFile)}) and\n\t * call {@link #shouldAutoFollow(ITextEditor, IDocument)} once this method\n\t * returned {@code true}.\n\t */\n\tprivate boolean isAutoFollowModeEnabled() {\n\t\tOutputMode mode = Activator.getDefault().cfg.getOuputMode();\n\t\treturn mode == OutputMode.Append || mode == OutputMode.Chat;\n\t}\n\n\t/**\n\t * Determines whether the \"autofollow\" comfort feature should be applied for\n\t * the given editor/document, i.e. whether the cursor should be moved to the\n\t * start of the last line after the tag has been replaced with the AI answer.\n\t *\n\t * This assumes {@link #isAutoFollowModeEnabled()} has already been checked\n\t * (and returned {@code true}) and additionally requires that:\n\t * <ul>\n\t * <li>the target file is currently open in an editor (i.e. {@code editor} is\n\t * not {@code null}),</li>\n\t * <li>the editor is unmodified compared to the saved file (not dirty), and</li>\n\t * <li>the cursor is already positioned on the last line of the document.</li>\n\t * </ul>\n\t *\n\t * The check is done <b>before</b> the tag is replaced, so that inserting the\n\t * (possibly multi-line) answer does not affect the outcome.\n\t */\n\tprivate boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {\n\t\tif (editor == null || editor.isDirty())\n\t\t\treturn false;\n\n\t\tISelection selection = editor.getSelectionProvider().getSelection();\n\t\tif (!(selection instanceof ITextSelection))\n\t\t\treturn false;\n\n\t\tITextSelection tsel = (ITextSelection) selection;\n\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\treturn tsel.getStartLine() >= lastLine;\n\t}\n\n\t/**\n\t * Places the cursor at the beginning of the (new) last line of the document,\n\t * used to implement the \"autofollow\" comfort feature.\n\t */\n\tprivate void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {\n\t\ttry {\n\t\t\tint lastLine = doc.getNumberOfLines() - 1;\n\t\t\tint offset = doc.getLineOffset(lastLine);\n\t\t\teditor.selectAndReveal(offset, 0);\n\t\t} catch (BadLocationException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}\n\n\tprivate java.util.List<ITextEditor> getOpenTextEditors() {\n\t\tjava.util.List<ITextEditor> editors = new java.util.ArrayList<>();\n\t\tfor (IWorkbenchWindow window : PlatformUI.getWorkbench().getWorkbenchWindows())\n\t\t\tfor (IWorkbenchPage page : window.getPages())\n\t\t\t\tfor (IEditorReference ref : page.getEditorReferences()) {\n\t\t\t\t\tIEditorPart part = ref.getEditor(false);\n\t\t\t\t\tif (part instanceof ITextEditor) {\n\t\t\t\t\t\tIEditorInput input = part.getEditorInput();\n\t\t\t\t\t\t// prefer file based / URI based editors, but any ITextEditor works\n\t\t\t\t\t\tif (input instanceof IFileEditorInput || input != null)\n\t\t\t\t\t\t\teditors.add((ITextEditor) part);\n\t\t\t\t\t}\n\t\t\t\t}\n\t\treturn editors;\n\t}\n\n\tpublic static String getPromptTag(String meta, String id) {\n\t\treturn String.format(\"[%s:%s:%s]\", AIREQ_PREFIX, meta, id);\n\t}\n}\n"
#:     }
#:   }
#: }
/allow 895a3bd3-2276-4f1b-aa19-6975fb4cb92f
#: ReasoningToken: 205
Die Datei ist jetzt konsistent und vollständig. Zusammenfassung der Reihenfolgen-Umkehr:

**`isAutoFollowModeEnabled()`** (neu) prüft zuerst nur den `OutputMode` (günstig, ohne Editor-Zugriff).
Erst **wenn dies `true` ist**, wird in `replaceMarker(...)` der (potenziell teure) Editor per `findOpenEditorFor(file)` gesucht – vorher bleibt `editor = null` und der Lookup entfällt komplett. In `findAndReplaceInOpenEditors(...)` wird durch Short-Circuit (`isAutoFollowModeEnabled() && shouldAutoFollow(editor, doc)`) ebenfalls sichergestellt, dass `shouldAutoFollow` (das den Editor-Status/Cursor prüft) nur ausgewertet wird, wenn der Modus bereits passt.

`shouldAutoFollow(editor, doc)` selbst enthält jetzt keine Mode-Prüfung mehr, sondern nur noch: `editor != null`, `!editor.isDirty()` und Cursor in letzter Zeile — der Editor wird also wirklich erst benötigt/verwendet, nachdem feststeht, dass der Modus grundsätzlich für Autofollow in Frage kommt.