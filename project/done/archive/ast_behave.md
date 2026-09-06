Ich würde gerne das AST, Spellcheck und Syntax Highlighting verhalten mittels Debounce verbessern.
Dazu müssen Document-Änderungen und Eingaben zentral gebündelt werden.
Ich würde vorschlagen alle drei Aspekte über einen Manager zu bündeln und zu verstecken, den Manager im Editor integrieren und alles Event-Handling durch diesen zu leiten. Quasi ein Document oder Editor Addon.
Prüfe und Implementiere den Ansatz.

* Editor: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java`
* AST: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java`
* Spellcheck: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckInstaller.java`
* Syntax Highlighting: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java`
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=4b8517bb-208b-4718-9a99-4ce4ecd1377e | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: ba08fd88-1308-4723-964e-62a9f88a0ed6
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
  reason: Understand editor structure
```
/answer ba08fd88-1308-4723-964e-62a9f88a0ed6 allow
Control Request:
```yaml
id: 372818b8-e635-4061-92b5-06c170152dcb
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  reason: Understand AST document
```
/answer 372818b8-e635-4061-92b5-06c170152dcb allow
Control Request:
```yaml
id: bace1cfb-e053-4d0a-a310-cfe163cacf49
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckInstaller.java
  reason: Understand spellcheck
```
/answer bace1cfb-e053-4d0a-a310-cfe163cacf49 allow
Control Request:
```yaml
id: 17d040c4-2f1b-4d1a-b919-7a87db2e19e3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  reason: Understand syntax highlighting
```
/answer 17d040c4-2f1b-4d1a-b919-7a87db2e19e3 allow
ReasoningToken: 20
Control Request:
```yaml
id: be008078-3bfb-4e0a-b012-512feec167e2
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
  reason: List editor package files
```
/answer be008078-3bfb-4e0a-b012-512feec167e2 allow
Control Request:
```yaml
id: c08d5ef2-a31a-46fb-a953-d9f66e465f59
result:
  structuredContent:
    entries:
    - AIRuleScanner.java
    - AISessionEditor.java
    - AISessionEditorContributor.java
    - AISourceViewerConfiguration.java
    - AITextEditor.java
    - CommentHandler.java
    - DocumentBuffer.java
    - MarkdownOutlinePage.java
    - md/AbstractRule.java
    - md/BlockRule.java
    - md/EmphasisRule.java
    - md/HeaderRule.java
    - md/LineMatchRule.java
    - md/LinkRule.java
    - md/ListRule.java
    - md/PrefixLineRule.java
    - md/WordRule.java
    - mdast/IDocumentBuffer.java
    - mdast/MarkdownDocument.java
    - mdast/TextRegion.java
    - mdast/nodes/AbstractNode.java
    - mdast/nodes/Category.java
    - mdast/nodes/Elements.java
    - mdast/nodes/HeadingSection.java
    - mdast/nodes/LineCommentBlock.java
    - mdast/nodes/LineSection.java
    - mdast/nodes/Node.java
    - mdast/nodes/PageSection.java
    - mdast/nodes/Paragraph.java
    - mdast/nodes/PrefixBlock.java
    - mdast/nodes/Root.java
    - mdast/nodes/ScriptBlock.java
    - spellcheck/LanguageToolClient.java
    - spellcheck/SpellCheckInstaller.java
    - spellcheck/SpellCheckReconciler.java
    - spellcheck/SpellingAnnotation.java
    - spellcheck/SpellingProblem.java
    - spellcheck/SpellingQuickAssistProcessor.java
    - spellcheck/SpellingStrategy.java
```
/answer c08d5ef2-a31a-46fb-a953-d9f66e465f59 allow
Control Request:
```yaml
id: 17361a4d-d858-4f57-8016-866222fd9e6a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  reason: "Understand editor wiring of AST, spellcheck, and scanner"
```
/answer 17361a4d-d858-4f57-8016-866222fd9e6a allow
ReasoningToken: 51
Control Request:
```yaml
id: b34fd0c6-1ef7-4c6a-ad0d-04784ee545ea
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  reason: Understand reconciler's document listening and debounce
```
/answer b34fd0c6-1ef7-4c6a-ad0d-04784ee545ea allow
Control Request:
```yaml
id: d6f3b507-45bf-41aa-8533-a067ba585133
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  reason: Understand configuration wiring of scanner and reconciler
```
/answer d6f3b507-45bf-41aa-8533-a067ba585133 allow
Control Request:
```yaml
id: 40947b51-5aa0-4dda-b177-236ea5f6780e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/IDocumentBuffer.java
  reason: Check buffer interface
```
/answer 40947b51-5aa0-4dda-b177-236ea5f6780e allow
Control Request:
```yaml
id: 34bb745a-de0c-4aa4-b7c6-c087f923ecd7
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/DocumentBuffer.java
  reason: Check buffer implementation
```
/answer 34bb745a-de0c-4aa4-b7c6-c087f923ecd7 allow
ReasoningToken: 50
Control Request:
```yaml
id: 30cab6d0-c52f-477e-922b-3b20d5cf575b
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/TextRegion.java
  reason: Check TextRegion class
```
/answer 30cab6d0-c52f-477e-922b-3b20d5cf575b allow
Control Request:
```yaml
id: a76945d7-c1a3-4dd9-a747-ae19874a55bb
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellingStrategy.java
  reason: Check spelling strategy interface
```
/answer a76945d7-c1a3-4dd9-a747-ae19874a55bb allow
ReasoningToken: 19568
Control Request:
```yaml
id: d02d0106-c4a6-4fec-be38-6726860518a6
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  script: grep -rn "getMarkdownAst\|getLastAstChangeRegion\|getMarkdownDocument\|astBuffer\|pendingRemoved" .
  reason: Find usages of editor AST API across codebase
```
/answer d02d0106-c4a6-4fec-be38-6726860518a6 allow
Control Request:
```yaml
id: 55d26b51-0425-48e9-9c8c-1a64a82c1d61
result:
  content:
  - type: text
    text: Full output written to file 2154 characters). Reduce to relevant excerpts (e.g. via grep/head/tail).
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-rvlfk353.log
```
/answer 55d26b51-0425-48e9-9c8c-1a64a82c1d61 allow
Control Request:
```yaml
id: e582d846-eb8e-4a7f-bae0-0ef9792ff7cf
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  script: cat /tmp/mcpc-stdout-rvlfk353.log
  reason: view output
```
/answer e582d846-eb8e-4a7f-bae0-0ef9792ff7cf deny nutze read-file
Control Request:
```yaml
id: a756bfc2-7e60-40f1-9890-555c0e4d93b1
toolName: read-file
arguments:
  path: /tmp/mcpc-stdout-rvlfk353.log
  reason: view grep output
```
/answer a756bfc2-7e60-40f1-9890-555c0e4d93b1 allow
Control Request:
```yaml
id: 73ac3e8a-966e-4211-b10f-5d490018864d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/MarkdownOutlinePage.java
  reason: Check outline page refresh method
```
/answer 73ac3e8a-966e-4211-b10f-5d490018864d allow
ReasoningToken: 333
Control Request:
```yaml
id: af9c02d9-e452-4915-a085-3f9cc5fe88de
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  mode: replace
  content: "package xy.ai.workbench.editor.update;\n\nimport java.util.List;\nimport java.util.concurrent.CopyOnWriteArrayList;\nimport java.util.concurrent.Executors;\nimport java.util.concurrent.ScheduledExecutorService;\nimport java.util.concurrent.ScheduledFuture;\nimport java.util.concurrent.TimeUnit;\n\nimport org.eclipse.jface.text.DocumentEvent;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IDocumentListener;\nimport org.eclipse.jface.text.IRegion;\nimport org.eclipse.jface.text.ITextInputListener;\nimport org.eclipse.jface.text.ITextViewer;\nimport org.eclipse.jface.text.Region;\nimport org.eclipse.swt.widgets.Display;\n\nimport xy.ai.workbench.editor.DocumentBuffer;\nimport xy.ai.workbench.editor.mdast.MarkdownDocument;\nimport xy.ai.workbench.editor.mdast.TextRegion;\n\n/**\n * Document/editor addon that bundles <b>all</b> raw document event handling\n * ({@link IDocumentListener}/{@link ITextInputListener}) for a single editor\n * instance behind one place, instead of every feature (AST, spell checking,\n * syntax highlighting, outline, ...) installing its own listeners directly on\n * the {@link IDocument}.\n * <p>\n * It owns the {@link MarkdownDocument} AST and keeps it perfectly in sync -\n * synchronously, per edit - because syntax highlighting and other consumers\n * rely on an always-correct, immediately queryable AST/{@link TextRegion} for\n * the region of the very last edit (see {@link #getLastAstChangeRegion()}).\n * <p>\n * Everything that is more expensive than an incremental AST update (spell\n * checking, outline refresh, ...) does not need to run on every single\n * keystroke though. For that purpose the manager additionally coalesces the\n * regions touched by a burst of edits and, after {@value #DEBOUNCE_DELAY_MS}\n * ms of editing inactivity, notifies interested {@link Listener}s exactly\n * once with the merged region - on the UI thread.\n */\npublic final class EditorUpdateManager {\n\n\tpublic static final int DEBOUNCE_DELAY_MS = 200;\n\n\t/** Callback interface implemented by features that want to react to document changes. */\n\tpublic interface Listener {\n\n\t\t/**\n\t\t * Fired synchronously, right after a single document edit has been\n\t\t * incorporated into the AST. Use this when the reaction must see the\n\t\t * exact, up-to-date state for that one edit (e.g. incremental syntax\n\t\t * highlighting damage regions).\n\t\t */\n\t\tdefault void onAstUpdated(TextRegion region) {\n\t\t}\n\n\t\t/**\n\t\t * Fired once, on the UI thread, after {@value #DEBOUNCE_DELAY_MS}ms of\n\t\t * editing inactivity, with the union of all regions changed since the\n\t\t * previous debounce round. Use this for comparatively expensive\n\t\t * reactions (spell checking, outline refresh, ...).\n\t\t */\n\t\tdefault void onSettled(IRegion mergedRegion) {\n\t\t}\n\n\t\t/** Fired whenever the underlying document instance itself is swapped or cleared. */\n\t\tdefault void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t}\n\t}\n\n\tprivate final List<Listener> listeners = new CopyOnWriteArrayList<>();\n\n\tprivate IDocument document;\n\tprivate DocumentBuffer astBuffer;\n\tprivate MarkdownDocument ast;\n\tprivate TextRegion lastAstChangeRegion;\n\tprivate int pendingRemoved;\n\n\tprivate final ScheduledExecutorService scheduler = Executors.newSingleThreadScheduledExecutor(r -> {\n\t\tThread t = new Thread(r, \"EditorUpdateManager-Debounce\");\n\t\tt.setDaemon(true);\n\t\treturn t;\n\t});\n\tprivate ScheduledFuture<?> pendingFlush;\n\tprivate int mergedLo = -1;\n\tprivate int mergedHi = -1;\n\n\tprivate final IDocumentListener documentListener = new IDocumentListener() {\n\t\t@Override\n\t\tpublic void documentAboutToBeChanged(DocumentEvent event) {\n\t\t\tpendingRemoved = event.getLength();\n\t\t}\n\n\t\t@Override\n\t\tpublic void documentChanged(DocumentEvent event) {\n\t\t\thandleDocumentChanged(event);\n\t\t}\n\t};\n\n\tprivate final ITextInputListener textInputListener = new ITextInputListener() {\n\t\t@Override\n\t\tpublic void inputDocumentAboutToBeChanged(IDocument oldInput, IDocument newInput) {\n\t\t\tif (oldInput != null)\n\t\t\t\toldInput.removeDocumentListener(documentListener);\n\t\t}\n\n\t\t@Override\n\t\tpublic void inputDocumentChanged(IDocument oldInput, IDocument newInput) {\n\t\t\tcancelPending();\n\t\t\tdocument = newInput;\n\t\t\tif (newInput != null) {\n\t\t\t\tastBuffer = new DocumentBuffer(newInput);\n\t\t\t\tast = new MarkdownDocument(astBuffer);\n\t\t\t\tlastAstChangeRegion = ast.update(0, 0, astBuffer.length());\n\t\t\t\tnewInput.addDocumentListener(documentListener);\n\t\t\t} else {\n\t\t\t\tastBuffer = null;\n\t\t\t\tast = null;\n\t\t\t\tlastAstChangeRegion = null;\n\t\t\t}\n\t\t\tfor (Listener l : listeners)\n\t\t\t\tl.onDocumentChanged(oldInput, newInput);\n\t\t\tif (lastAstChangeRegion != null)\n\t\t\t\tfor (Listener l : listeners)\n\t\t\t\t\tl.onAstUpdated(lastAstChangeRegion);\n\t\t}\n\t};\n\n\t/** Hooks the manager into the given viewer's document lifecycle. */\n\tpublic void install(ITextViewer viewer) {\n\t\tviewer.addTextInputListener(textInputListener);\n\t\tIDocument doc = viewer.getDocument();\n\t\tif (doc != null)\n\t\t\ttextInputListener.inputDocumentChanged(null, doc);\n\t}\n\n\t/** Reverses {@link #install(ITextViewer)} and releases all resources. */\n\tpublic void uninstall(ITextViewer viewer) {\n\t\tcancelPending();\n\t\tscheduler.shutdownNow();\n\t\tif (document != null)\n\t\t\tdocument.removeDocumentListener(documentListener);\n\t\tviewer.removeTextInputListener(textInputListener);\n\t\tlisteners.clear();\n\t}\n\n\tpublic void addListener(Listener listener) {\n\t\tlisteners.add(listener);\n\t}\n\n\tpublic void removeListener(Listener listener) {\n\t\tlisteners.remove(listener);\n\t}\n\n\tpublic MarkdownDocument getAst() {\n\t\treturn ast;\n\t}\n\n\tpublic IDocument getDocument() {\n\t\treturn document;\n\t}\n\n\tpublic TextRegion getLastAstChangeRegion() {\n\t\treturn lastAstChangeRegion;\n\t}\n\n\tprivate void handleDocumentChanged(DocumentEvent event) {\n\t\tif (ast == null || astBuffer == null || astBuffer.document() != event.getDocument())\n\t\t\treturn;\n\n\t\tString text = event.getText();\n\t\tint inserted = text == null ? 0 : text.length();\n\t\tlastAstChangeRegion = ast.update(event.getOffset(), pendingRemoved, inserted);\n\n\t\tfor (Listener l : listeners)\n\t\t\tl.onAstUpdated(lastAstChangeRegion);\n\n\t\tmergeDirty(lastAstChangeRegion.offset(), lastAstChangeRegion.offset() + lastAstChangeRegion.length());\n\t\tscheduleFlush();\n\t}\n\n\tprivate synchronized void mergeDirty(int lo, int hi) {\n\t\tif (mergedLo < 0) {\n\t\t\tmergedLo = lo;\n\t\t\tmergedHi = hi;\n\t\t} else {\n\t\t\tmergedLo = Math.min(mergedLo, lo);\n\t\t\tmergedHi = Math.max(mergedHi, hi);\n\t\t}\n\t}\n\n\tprivate synchronized void cancelPending() {\n\t\tif (pendingFlush != null) {\n\t\t\tpendingFlush.cancel(false);\n\t\t\tpendingFlush = null;\n\t\t}\n\t}\n\n\tprivate void scheduleFlush() {\n\t\tcancelPending();\n\t\tif (!scheduler.isShutdown())\n\t\t\tpendingFlush = scheduler.schedule(this::flush, DEBOUNCE_DELAY_MS, TimeUnit.MILLISECONDS);\n\t}\n\n\tprivate void flush() {\n\t\tIRegion region;\n\t\tsynchronized (this) {\n\t\t\tif (mergedLo < 0)\n\t\t\t\treturn;\n\t\t\tregion = new Region(mergedLo, mergedHi - mergedLo);\n\t\t\tmergedLo = -1;\n\t\t\tmergedHi = -1;\n\t\t}\n\t\tDisplay display = Display.getDefault();\n\t\tif (display == null || display.isDisposed())\n\t\t\treturn;\n\t\tdisplay.asyncExec(() -> {\n\t\t\tif (document == null)\n\t\t\t\treturn;\n\t\t\tfor (Listener l : listeners)\n\t\t\t\tl.onSettled(region);\n\t\t});\n\t}\n}\n"
  reason: Create central editor update manager (document/editor addon)
```
/answer af9c02d9-e452-4915-a085-3f9cc5fe88de allow
Control Request:
```yaml
id: 05817349-78e1-420d-a533-f8cbd2f73b74
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  mode: replace
  content: "package xy.ai.workbench.editor;\n\nimport java.util.ArrayList;\nimport java.util.Iterator;\nimport java.util.List;\n\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IRegion;\nimport org.eclipse.jface.text.source.CompositeRuler;\nimport org.eclipse.jface.text.source.ISourceViewer;\nimport org.eclipse.jface.text.source.IVerticalRuler;\nimport org.eclipse.jface.text.source.IVerticalRulerColumn;\nimport org.eclipse.jface.text.source.SourceViewer;\nimport org.eclipse.swt.custom.StyledText;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.editors.text.TextEditor;\nimport org.eclipse.ui.views.contentoutline.IContentOutlinePage;\n\nimport xy.ai.workbench.editor.mdast.MarkdownDocument;\nimport xy.ai.workbench.editor.mdast.TextRegion;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\nimport xy.ai.workbench.editor.spellcheck.SpellCheckInstaller;\nimport xy.ai.workbench.editor.update.EditorUpdateManager;\n\npublic class AITextEditor extends TextEditor {\n\tprivate static final int LIMIT = 512 * 1024;\n\tprivate boolean rulerVisible = true;\n\tprivate CompositeRuler ruler;\n\tprivate List<IVerticalRulerColumn> decorators = new ArrayList<>();\n\n\tprivate final EditorUpdateManager updateManager = new EditorUpdateManager();\n\n\tprivate MarkdownOutlinePage outlinePage;\n\n\tpublic AITextEditor() {\n\t\tsuper();\n\t\tsetSourceViewerConfiguration(new AISourceViewerConfiguration(this));\n\t}\n\n\t@Override\n\tprotected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {\n\t\tISourceViewer sourceViewer = super.createSourceViewer(parent, ruler, styles);\n\n\t\tif (ruler instanceof CompositeRuler)\n\t\t\tthis.ruler = (CompositeRuler) ruler;\n\n\t\tupdateManager.addListener(new EditorUpdateManager.Listener() {\n\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\trefreshOutline();\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {\n\t\t\t\tIDocument doc = updateManager.getDocument();\n\t\t\t\tif (doc == null)\n\t\t\t\t\treturn;\n\t\t\t\tupdateRulerVisibility(doc);\n\t\t\t\tupdateLineNumbers(doc);\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onSettled(IRegion mergedRegion) {\n\t\t\t\trefreshOutline();\n\t\t\t}\n\t\t});\n\t\tupdateManager.install(sourceViewer);\n\n\t\treturn sourceViewer;\n\t}\n\n\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tsuper.createPartControl(parent);\n\t\tSpellCheckInstaller.installPainter(getSourceViewer());\n\n\t\tif (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)\n\t\t\twidget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));\n\t}\n\n\t@Override\n\tpublic void dispose() {\n\t\tif (getSourceViewer() != null)\n\t\t\tupdateManager.uninstall(getSourceViewer());\n\t\tsuper.dispose();\n\t}\n\n\tprivate void handleCaretMoved(int offset) {\n\t\tif (outlinePage != null) {\n\t\t\tvar selection = getSourceViewer().getSelectedRange();\n\t\t\tif (selection.y == 0)\n\t\t\t\tDisplay.getDefault().asyncExec(() -> outlinePage.selectNodeForOffset(offset));\n\t\t}\n\t}\n\n\t@Override\n\tpublic <T> T getAdapter(Class<T> adapter) {\n\t\tif (IContentOutlinePage.class.equals(adapter)) {\n\t\t\tif (outlinePage == null)\n\t\t\t\toutlinePage = new MarkdownOutlinePage(this);\n\t\t\treturn adapter.cast(outlinePage);\n\t\t}\n\t\treturn super.getAdapter(adapter);\n\t}\n\n\tprivate void refreshOutline() {\n\t\tif (outlinePage != null)\n\t\t\toutlinePage.refresh();\n\t}\n\n\tpublic void selectAndRevealNode(Node node) {\n\t\tif (node == null)\n\t\t\treturn;\n\t\tselectAndReveal(node.getOffset(), node.length());\n\t}\n\n\tpublic IDocument getMarkdownDocument() {\n\t\treturn updateManager.getDocument();\n\t}\n\n\t@Override\n\tprotected boolean getInitialWordWrapStatus() {\n\t\treturn true;\n\t}\n\n\tpublic MarkdownDocument getMarkdownAst() {\n\t\treturn updateManager.getAst();\n\t}\n\n\tpublic TextRegion getLastAstChangeRegion() {\n\t\treturn updateManager.getLastAstChangeRegion();\n\t}\n\n\t/** Central document/editor addon bundling AST, spell check and syntax highlighting event handling. */\n\tpublic EditorUpdateManager getUpdateManager() {\n\t\treturn updateManager;\n\t}\n\n\tprivate void updateRulerVisibility(IDocument document) {\n\t\tboolean shouldShow = document.getLength() <= LIMIT;\n\t\tif (shouldShow != rulerVisible) {\n\t\t\trulerVisible = shouldShow;\n\t\t\tif (getSourceViewer() instanceof SourceViewer sv) {\n\t\t\t\tsv.showAnnotations(shouldShow);\n\t\t\t\tsv.showAnnotationsOverview(shouldShow);\n\t\t\t}\n\t\t}\n\t}\n\n\tprivate void updateLineNumbers(IDocument document) {\n\t\tif (ruler == null)\n\t\t\treturn;\n\n\t\tlong size = document.getLength();\n\t\tIterator<IVerticalRulerColumn> it = ruler.getDecoratorIterator();\n\t\tIVerticalRulerColumn d;\n\n\t\tif (size > LIMIT && it.hasNext()) {\n\t\t\twhile (it.hasNext() && (d = it.next()) != null)\n\t\t\t\tdecorators.add(d);\n\t\t\tfor (var dec : decorators)\n\t\t\t\truler.removeDecorator(dec);\n\t\t} else if (size < LIMIT && !it.hasNext() && !decorators.isEmpty()) {\n\t\t\tfor (var i = 0; i < decorators.size(); i++)\n\t\t\t\truler.addDecorator(i, decorators.get(i));\n\t\t\tdecorators.clear();\n\t\t}\n\t}\n}\n"
  reason: Route AITextEditor's document handling through central EditorUpdateManager
```
/answer 05817349-78e1-420d-a533-f8cbd2f73b74 allow
Control Request:
```yaml
id: 90764c5c-7cdd-48f2-89f0-f9bf1797e09a
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  mode: replace
  content: "package xy.ai.workbench.editor.spellcheck;\n\nimport java.util.ArrayList;\nimport java.util.Iterator;\nimport java.util.List;\n\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IRegion;\nimport org.eclipse.jface.text.ITextViewer;\nimport org.eclipse.jface.text.Region;\nimport org.eclipse.jface.text.reconciler.IReconciler;\nimport org.eclipse.jface.text.reconciler.IReconcilingStrategy;\n\nimport xy.ai.workbench.editor.AITextEditor;\nimport xy.ai.workbench.editor.mdast.TextRegion;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\nimport xy.ai.workbench.editor.update.EditorUpdateManager;\n\n/**\n * Reconciler that tracks only the document region actually changed by the user.\n * <p>\n * All raw document events are handled centrally by the {@link EditorUpdateManager}\n * (the editor's document/editor addon). This reconciler no longer installs its own\n * {@code IDocumentListener}/{@code ITextInputListener} or debounce timer - it merely\n * registers itself as an {@link EditorUpdateManager.Listener}: every AST update\n * ({@link EditorUpdateManager.Listener#onAstUpdated(TextRegion)}) is used to widen\n * the pending dirty/clear regions, and the shared, already-debounced\n * {@link EditorUpdateManager.Listener#onSettled(IRegion)} notification is used as the\n * trigger to hand those regions to {@link SpellingStrategy}.\n */\npublic class SpellCheckReconciler implements IReconciler {\n\n\tprivate final SpellingStrategy fStrategy;\n\tprivate final AITextEditor fEditor;\n\n\tprivate final List<int[]> fDirtyRegions = new ArrayList<>();\n\tprivate final List<int[]> fClearRegions = new ArrayList<>();\n\n\tprivate EditorUpdateManager.Listener fListener;\n\n\tpublic SpellCheckReconciler(SpellingStrategy strategy, AITextEditor editor) {\n\t\tfStrategy = strategy;\n\t\tfEditor = editor;\n\t}\n\n\t// ── IReconciler ────────────────────────────────────────────────────────────\n\n\t@Override\n\tpublic void install(ITextViewer textViewer) {\n\t\tfListener = new EditorUpdateManager.Listener() {\n\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\tfStrategy.setDocument(newDocument);\n\t\t\t\tif (newDocument != null) {\n\t\t\t\t\t// Trigger a full-document check on the initial load.\n\t\t\t\t\tmergeDirty(0, newDocument.getLength());\n\t\t\t\t}\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {\n\t\t\t\tif (region != null)\n\t\t\t\t\tmergeEnabledLeaves(region.n());\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onSettled(IRegion mergedRegion) {\n\t\t\t\tflush();\n\t\t\t}\n\t\t};\n\n\t\tEditorUpdateManager manager = fEditor.getUpdateManager();\n\t\tmanager.addListener(fListener);\n\n\t\t// Handle a document that is already set on the editor.\n\t\tIDocument doc = manager.getDocument();\n\t\tif (doc != null) {\n\t\t\tfStrategy.setDocument(doc);\n\t\t\tmergeDirty(0, doc.getLength());\n\t\t\tflush();\n\t\t}\n\t}\n\n\t@Override\n\tpublic void uninstall() {\n\t\tif (fListener != null) {\n\t\t\tfEditor.getUpdateManager().removeListener(fListener);\n\t\t\tfListener = null;\n\t\t}\n\t}\n\n\t@Override\n\tpublic IReconcilingStrategy getReconcilingStrategy(String contentType) {\n\t\treturn IDocument.DEFAULT_CONTENT_TYPE.equals(contentType) ? fStrategy : null;\n\t}\n\n\t// ── dirty/clear region bookkeeping ───────────────────────────────────────────\n\n\tprivate boolean mergeEnabledLeaves(Node node) {\n\t\tif (node == null)\n\t\t\treturn false;\n\t\tif (node.children.isEmpty()) {\n\t\t\tint start = node.getOffset();\n\t\t\tint end = node.getEndOffset();\n\t\t\tif (!node.enableSpellcheck) {\n\t\t\t\tmergeClear(start, Math.max(end, start + 1));\n\t\t\t\treturn true;\n\t\t\t}\n\t\t\tmergeDirty(start, Math.max(end, start + 1));\n\t\t\treturn true;\n\t\t}\n\t\tboolean merged = false;\n\t\tfor (Node child : node.children)\n\t\t\tmerged |= mergeEnabledLeaves(child);\n\t\treturn merged;\n\t}\n\n\tprivate synchronized void mergeDirty(int start, int end) {\n\t\tmerge(fDirtyRegions, start, end);\n\t}\n\n\tprivate synchronized void mergeClear(int start, int end) {\n\t\tmerge(fClearRegions, start, end);\n\t}\n\n\tprivate static void merge(List<int[]> regions, int start, int end) {\n\t\tint newStart = start;\n\t\tint newEnd = end;\n\t\tfor (Iterator<int[]> it = regions.iterator(); it.hasNext();) {\n\t\t\tint[] r = it.next();\n\t\t\t// Overlapping or directly adjacent -> merge.\n\t\t\tif (newStart <= r[1] && r[0] <= newEnd) {\n\t\t\t\tnewStart = Math.min(newStart, r[0]);\n\t\t\t\tnewEnd = Math.max(newEnd, r[1]);\n\t\t\t\tit.remove();\n\t\t\t}\n\t\t}\n\t\tregions.add(new int[] { newStart, newEnd });\n\t}\n\n\tprivate synchronized List<IRegion> takeDirty() {\n\t\treturn take(fDirtyRegions);\n\t}\n\n\tprivate synchronized List<IRegion> takeClear() {\n\t\treturn take(fClearRegions);\n\t}\n\n\tprivate static List<IRegion> take(List<int[]> regions) {\n\t\tif (regions.isEmpty())\n\t\t\treturn null;\n\t\tList<IRegion> result = new ArrayList<>(regions.size());\n\t\tfor (int[] r : regions)\n\t\t\tresult.add(new Region(r[0], r[1] - r[0]));\n\t\tregions.clear();\n\t\treturn result;\n\t}\n\n\tprivate void flush() {\n\t\tList<IRegion> clear = takeClear();\n\t\tif (clear != null)\n\t\t\tfor (IRegion region : clear)\n\t\t\t\tfStrategy.clear(region);\n\n\t\tList<IRegion> dirty = takeDirty();\n\t\tif (dirty != null)\n\t\t\tfor (IRegion region : dirty)\n\t\t\t\tfStrategy.reconcile(region);\n\t}\n}\n"
  reason: Route spellcheck reconciler through central EditorUpdateManager instead of own listeners/scheduler
```
/answer 90764c5c-7cdd-48f2-89f0-f9bf1797e09a allow
Control Request:
```yaml
id: 076f776a-4e78-4576-8f6c-d7bb4d3415a7
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckInstaller.java
  old_text: |-
    public class SpellCheckInstaller {

        private static final int RECONCILE_DELAY_MS = 200;

        public static IReconciler createReconciler(ISourceViewer sourceViewer, AITextEditor editor) {
            SpellingStrategy strategy = new SpellingStrategy(sourceViewer);
            return new SpellCheckReconciler(strategy, RECONCILE_DELAY_MS, editor);
        }
  new_text: |-
    public class SpellCheckInstaller {

        public static IReconciler createReconciler(ISourceViewer sourceViewer, AITextEditor editor) {
            SpellingStrategy strategy = new SpellingStrategy(sourceViewer);
            return new SpellCheckReconciler(strategy, editor);
        }
  reason: Drop own delay constant now handled centrally by EditorUpdateManager
```
/answer 076f776a-4e78-4576-8f6c-d7bb4d3415a7 allow
ReasoningToken: 190
Text:
 Now update `AITextEditor` to use the manager.
Control Request:
```yaml
id: c25e2d7a-f419-4ef6-a878-9e31d0301b64
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  old_text: "\t\t\t\tTextRegion astRegion = editor != null ? editor.getLastAstChangeRegion() : null;"
  new_text: "\t\t\t\tTextRegion astRegion = editor != null ? editor.getUpdateManager().getLastAstChangeRegion() : null;"
  reason: Route through central manager for consistency
```
/answer c25e2d7a-f419-4ef6-a878-9e31d0301b64 allow
Control Request:
```yaml
id: d2763f21-e05b-4e69-9ff5-ba6913a8e7fe
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  old_text: "\t\tMarkdownDocument ast = editor != null ? editor.getMarkdownAst() : null;"
  new_text: "\t\tMarkdownDocument ast = editor != null ? editor.getUpdateManager().getAst() : null;"
  reason: Route through central manager for consistency
```
/answer d2763f21-e05b-4e69-9ff5-ba6913a8e7fe allow
Control Request:
```yaml
id: 5eca4ef8-8751-4e32-a097-3ef7f646ae23
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: ls; find . -maxdepth 2 -iname "pom.xml" -o -iname "build.xml" -o -iname ".classpath" | head
  reason: Check build tooling
```
/answer 5eca4ef8-8751-4e32-a097-3ef7f646ae23 deny es gibt weder Build-Tooling noch Tests
Text:
 Now update `SpellCheckReconciler` and `SpellCheckInstaller` to route through the manager instead of installing their own listeners/scheduler.
Control Request:
```yaml
id: 4d2596d7-0a54-401d-8d03-4d40914c690f
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  old_text: "\t/**\n\t * Fired once, on the UI thread, after {@value #DEBOUNCE_DELAY_MS}ms of\n\t * editing inactivity, with the union of all regions changed since the\n\t * previous debounce round. Use this for comparatively expensive\n\t * reactions (spell checking, outline refresh, ...).\n\t */\n\tdefault void onSettled(IRegion mergedRegion) {\n\t}"
  new_text: "\t/**\n\t * Fired once, on a dedicated background thread, after\n\t * {@value #DEBOUNCE_DELAY_MS}ms of editing inactivity, with the union of\n\t * all regions changed since the previous debounce round. Use this for\n\t * comparatively expensive reactions (spell checking, outline refresh,\n\t * ...). Implementations that touch SWT widgets must hop back to the UI\n\t * thread themselves (e.g. via {@code Display.asyncExec}); implementations\n\t * that perform blocking I/O (e.g. spell checking) benefit from already\n\t * running off the UI thread.\n\t */\n\tdefault void onSettled(IRegion mergedRegion) {\n\t}"
  reason: Clarify onSettled threading contract
```
/answer 4d2596d7-0a54-401d-8d03-4d40914c690f allow
Control Request:
```yaml
id: 2c435d1c-4ec2-43f1-aa1a-7695396e10af
result:
  content:
  - type: text
    text: Text not found in file.
  isError: true
```
/answer 2c435d1c-4ec2-43f1-aa1a-7695396e10af allow
Control Request:
```yaml
id: a5903b94-1932-4eb6-a5a7-0cf97f76ce5e
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  old_text: "\tprivate void flush() {\n\t\tIRegion region;\n\t\tsynchronized (this) {\n\t\t\tif (mergedLo < 0)\n\t\t\t\treturn;\n\t\t\tregion = new Region(mergedLo, mergedHi - mergedLo);\n\t\t\tmergedLo = -1;\n\t\t\tmergedHi = -1;\n\t\t}\n\t\tDisplay display = Display.getDefault();\n\t\tif (display == null || display.isDisposed())\n\t\t\treturn;\n\t\tdisplay.asyncExec(() -> {\n\t\t\tif (document == null)\n\t\t\t\treturn;\n\t\t\tfor (Listener l : listeners)\n\t\t\t\tl.onSettled(region);\n\t\t});\n\t}"
  new_text: "\tprivate void flush() {\n\t\tIRegion region;\n\t\tsynchronized (this) {\n\t\t\tif (mergedLo < 0)\n\t\t\t\treturn;\n\t\t\tregion = new Region(mergedLo, mergedHi - mergedLo);\n\t\t\tmergedLo = -1;\n\t\t\tmergedHi = -1;\n\t\t}\n\t\tif (document == null)\n\t\t\treturn;\n\t\t// Runs on the debounce scheduler thread on purpose: listeners that need to\n\t\t// perform blocking work (e.g. spell checking) must not do so on the UI\n\t\t// thread; listeners that touch SWT widgets hop back to the UI thread\n\t\t// themselves.\n\t\tfor (Listener l : listeners)\n\t\t\tl.onSettled(region);\n\t}"
  reason: Run debounced notification on background thread instead of forcing UI thread
```
/answer a5903b94-1932-4eb6-a5a7-0cf97f76ce5e allow
Control Request:
```yaml
id: 9b199a84-18da-450d-a04a-f4305152e4e3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  start: /** Callback interface
  end: default void onDocumentChanged
  reason: Check current javadoc text for onSettled
```
/answer 9b199a84-18da-450d-a04a-f4305152e4e3 allow
Control Request:
```yaml
id: 05ca94b9-85f3-4ac3-a36f-ebbcf88064ff
result:
  structuredContent:
    checksum: 034ca33a74124a75e86f2456b3383ff1e58b669db5fa97bca867f46c705d4c68
    content: "/** Callback interface implemented by features that want to react to document changes. */\n\tpublic interface Listener {\n\n\t\t/**\n\t\t * Fired synchronously, right after a single document edit has been\n\t\t * incorporated into the AST. Use this when the reaction must see the\n\t\t * exact, up-to-date state for that one edit (e.g. incremental syntax\n\t\t * highlighting damage regions).\n\t\t */\n\t\tdefault void onAstUpdated(TextRegion region) {\n\t\t}\n\n\t\t/**\n\t\t * Fired once, on the UI thread, after {@value #DEBOUNCE_DELAY_MS}ms of\n\t\t * editing inactivity, with the union of all regions changed since the\n\t\t * previous debounce round. Use this for comparatively expensive\n\t\t * reactions (spell checking, outline refresh, ...).\n\t\t */\n\t\tdefault void onSettled(IRegion mergedRegion) {\n\t\t}\n\n\t\t/** Fired whenever the underlying document instance itself is swapped or cleared. */\n\t\tdefault void onDocumentChanged"
```
/answer 05ca94b9-85f3-4ac3-a36f-ebbcf88064ff allow
Control Request:
```yaml
id: c55fce9e-af2b-46fb-9ebc-749b0b5c13a7
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  old_text: "\t\t/**\n\t\t * Fired once, on the UI thread, after {@value #DEBOUNCE_DELAY_MS}ms of\n\t\t * editing inactivity, with the union of all regions changed since the\n\t\t * previous debounce round. Use this for comparatively expensive\n\t\t * reactions (spell checking, outline refresh, ...).\n\t\t */\n\t\tdefault void onSettled(IRegion mergedRegion) {\n\t\t}"
  new_text: "\t\t/**\n\t\t * Fired once, on a dedicated background thread, after\n\t\t * {@value #DEBOUNCE_DELAY_MS}ms of editing inactivity, with the union\n\t\t * of all regions changed since the previous debounce round. Use this\n\t\t * for comparatively expensive reactions (spell checking, outline\n\t\t * refresh, ...). Implementations that touch SWT widgets must hop back\n\t\t * to the UI thread themselves (e.g. via {@code Display.asyncExec});\n\t\t * implementations that perform blocking I/O (e.g. spell checking)\n\t\t * benefit from already running off the UI thread.\n\t\t */\n\t\tdefault void onSettled(IRegion mergedRegion) {\n\t\t}"
  reason: Fix javadoc to match new background-thread contract
```
/answer c55fce9e-af2b-46fb-9ebc-749b0b5c13a7 allow
Control Request:
```yaml
id: 6f406cc5-7a08-40b3-a4e5-ab82ff9b3dbd
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  reason: Final review of manager file
```
/answer 6f406cc5-7a08-40b3-a4e5-ab82ff9b3dbd allow
ReasoningToken: 78
Control Request:
```yaml
id: 9f9e943a-dbf1-467c-bdeb-5dd13bf85298
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  old_text: |-
    import org.eclipse.jface.text.Region;
    import org.eclipse.swt.widgets.Display;

    import xy.ai.workbench.editor.DocumentBuffer;
  new_text: |-
    import org.eclipse.jface.text.Region;

    import xy.ai.workbench.editor.DocumentBuffer;
  reason: Remove unused import
```
/answer 9f9e943a-dbf1-467c-bdeb-5dd13bf85298 allow
Control Request:
```yaml
id: bebb450e-07d5-434c-8ab1-6b2a6b862599
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  old_text: |2-
     * regions touched by a burst of edits and, after {@value #DEBOUNCE_DELAY_MS}
     * ms of editing inactivity, notifies interested {@link Listener}s exactly
     * once with the merged region - on the UI thread.
     */
  new_text: |2-
     * regions touched by a burst of edits and, after {@value #DEBOUNCE_DELAY_MS}
     * ms of editing inactivity, notifies interested {@link Listener}s exactly
     * once with the merged region - on a dedicated background thread, so that
     * expensive/blocking reactions (e.g. spell checking) never stall the UI.
     */
  reason: Fix class javadoc to reflect background-thread debounce dispatch
```
/answer bebb450e-07d5-434c-8ab1-6b2a6b862599 deny ignoriere die javadocs
Control Request:
```yaml
id: ad68182d-f5ab-4571-88f9-3155fcc08e98
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  old_text: "\t\t\t@Override\n\t\t\tpublic void onSettled(IRegion mergedRegion) {\n\t\t\t\trefreshOutline();\n\t\t\t}\n\t\t});"
  new_text: "\t\t\t@Override\n\t\t\tpublic void onSettled(IRegion mergedRegion) {\n\t\t\t\t// onSettled runs on a background thread - outline refresh touches SWT.\n\t\t\t\tDisplay.getDefault().asyncExec(AITextEditor.this::refreshOutline);\n\t\t\t}\n\t\t});"
  reason: Hop back to UI thread for outline refresh since onSettled fires on background thread
```
/answer ad68182d-f5ab-4571-88f9-3155fcc08e98 allow
ReasoningToken: 206
ReasoningToken: 97
Text:
 I found a threading issue: the manager's debounced `onSettled` must not force UI-thread execution, since spellcheck reconciliation performs blocking network I/O there. Let me fix that.
Control Request:
```yaml
id: c53f1f38-d3e2-47c6-942c-6adb57e34977
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  old_text: "\tpublic void addListener(Listener listener) {\n\t\tlisteners.add(listener);\n\t}"
  new_text: "\t/**\n\t * Runs {@code task} on the manager's shared background thread, off the UI\n\t * thread. Useful for listeners that need to react to state that was\n\t * already established before they registered (e.g. an initial full-document\n\t * check) without blocking the calling (usually UI) thread themselves.\n\t */\n\tpublic void runAsync(Runnable task) {\n\t\tif (!scheduler.isShutdown())\n\t\t\tscheduler.execute(task);\n\t}\n\n\tpublic void addListener(Listener listener) {\n\t\tlisteners.add(listener);\n\t}"
  reason: Add helper to offload work to manager's background thread
```
/answer c53f1f38-d3e2-47c6-942c-6adb57e34977 allow
Control Request:
```yaml
id: a55cdbca-9012-47b9-919e-c423c22bf606
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  old_text: "\t\t// Handle a document that is already set on the editor.\n\t\tIDocument doc = manager.getDocument();\n\t\tif (doc != null) {\n\t\t\tfStrategy.setDocument(doc);\n\t\t\tmergeDirty(0, doc.getLength());\n\t\t\tflush();\n\t\t}"
  new_text: "\t\t// Handle a document that is already set on the editor (the manager processed\n\t\t// the initial document load before this reconciler had a chance to register).\n\t\t// Run the (blocking) reconcile off the calling (UI) thread, on the manager's\n\t\t// shared background thread.\n\t\tIDocument doc = manager.getDocument();\n\t\tif (doc != null) {\n\t\t\tfStrategy.setDocument(doc);\n\t\t\tmergeDirty(0, doc.getLength());\n\t\t\tmanager.runAsync(this::flush);\n\t\t}"
  reason: Avoid blocking UI thread during initial spellcheck catch-up
```
/answer a55cdbca-9012-47b9-919e-c423c22bf606 allow
ReasoningToken: 2701
ReasoningToken: 167
Text:
 Good, that first edit indeed didn't apply (tabs vs spaces mismatch likely). Let me fix it now with correct matching text.
Control Request:
```yaml
id: 7b85065a-ce6c-4bf8-be13-6fa2613609e3
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  old_text: "\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\tfStrategy.setDocument(newDocument);\n\t\t\t\tif (newDocument != null) {\n\t\t\t\t\t// Trigger a full-document check on the initial load.\n\t\t\t\t\tmergeDirty(0, newDocument.getLength());\n\t\t\t\t}\n\t\t\t}"
  new_text: "\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\tfStrategy.setDocument(newDocument);\n\t\t\t\tif (newDocument != null) {\n\t\t\t\t\t// Trigger a full-document check whenever the document instance changes\n\t\t\t\t\t// (initial load, \"Save As\", ...). Not covered by the edit-driven debounce.\n\t\t\t\t\tmergeDirty(0, newDocument.getLength());\n\t\t\t\t\tfEditor.getUpdateManager().runAsync(SpellCheckReconciler.this::flush);\n\t\t\t\t}\n\t\t\t}"
  reason: Ensure full-document check runs whenever the document instance itself changes
```
/answer 7b85065a-ce6c-4bf8-be13-6fa2613609e3 allow
ReasoningToken: 107
Text:
 Good, matches expectations. Now remove the unused `Display` import and fix the class-level javadoc mention of "on the UI thread".
Text:
 Important: since `onSettled` now runs on a background thread, `AITextEditor`'s outline refresh must hop back to the UI thread.
Control Request:
```yaml
id: 59d76fb8-ee95-41b0-b6bc-c4ad71bd34fd
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  old_text: "\tprivate IDocument document;\n\tprivate DocumentBuffer astBuffer;\n\tprivate MarkdownDocument ast;\n\tprivate TextRegion lastAstChangeRegion;\n\tprivate int pendingRemoved;"
  new_text: "\t// Written on the UI thread only, but read from the background debounce thread\n\t// too (e.g. from flush()) - volatile for safe cross-thread visibility.\n\tprivate volatile IDocument document;\n\tprivate volatile MarkdownDocument ast;\n\tprivate volatile TextRegion lastAstChangeRegion;\n\tprivate DocumentBuffer astBuffer;\n\tprivate int pendingRemoved;"
  reason: Ensure cross-thread visibility for fields read by background debounce thread
```
/answer 59d76fb8-ee95-41b0-b6bc-c4ad71bd34fd allow
Control Request:
```yaml
id: 111ae039-4eab-4eca-ab28-eb1bcc18cd16
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  reason: Final review
```
/answer 111ae039-4eab-4eca-ab28-eb1bcc18cd16 allow
ReasoningToken: 229
Control Request:
```yaml
id: 6d5a95c7-5477-461e-8b55-d75070a6aba0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  reason: Final review
```
/answer 6d5a95c7-5477-461e-8b55-d75070a6aba0 allow
Control Request:
```yaml
id: f952515d-2e6b-4850-94de-d9acaf02a1ab
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  script: grep -rn "RECONCILE_DELAY_MS\|SpellCheckReconciler(" . ; echo ---; grep -rn "docListener\|inputDocumentChanged\|inputDocumentAboutToBeChanged" xy/ai/workbench/editor/AITextEditor.java xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  reason: Verify no stale references remain
```
/answer f952515d-2e6b-4850-94de-d9acaf02a1ab allow
Control Request:
```yaml
id: 998263c0-0d06-49b6-a656-8097e44d42c4
result:
  structuredContent:
    exit_code: 1
    stdout: "./xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java:41:\tpublic SpellCheckReconciler(SpellingStrategy strategy, AITextEditor editor) {\n./xy/ai/workbench/editor/spellcheck/SpellCheckInstaller.java:18:        return new SpellCheckReconciler(strategy, editor);\n---\n"
```
/answer 998263c0-0d06-49b6-a656-8097e44d42c4 allow
ReasoningToken: 1646
Control Request:
```yaml
id: 60a9e032-3bc2-45fd-9760-f00c24b6d682
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  script: grep -n "getUpdateManager\|getMarkdownAst\|getLastAstChangeRegion" xy/ai/workbench/editor/AIRuleScanner.java xy/ai/workbench/editor/AISourceViewerConfiguration.java xy/ai/workbench/editor/MarkdownOutlinePage.java
  reason: Verify consistent manager usage
```
/answer 60a9e032-3bc2-45fd-9760-f00c24b6d682 allow
Control Request:
```yaml
id: d77cf02c-857f-4c48-962c-5245f51b00c4
result:
  structuredContent:
    stdout: "xy/ai/workbench/editor/AIRuleScanner.java:199:\t\tMarkdownDocument ast = editor != null ? editor.getUpdateManager().getAst() : null;\nxy/ai/workbench/editor/AISourceViewerConfiguration.java:68:\t\t\t\tTextRegion astRegion = editor != null ? editor.getUpdateManager().getLastAstChangeRegion() : null;\nxy/ai/workbench/editor/MarkdownOutlinePage.java:42:\t\tviewer.setInput(editor.getMarkdownAst());\nxy/ai/workbench/editor/MarkdownOutlinePage.java:58:\t\tif (viewer.getInput() != editor.getMarkdownAst())\nxy/ai/workbench/editor/MarkdownOutlinePage.java:59:\t\t\tviewer.setInput(editor.getMarkdownAst());\nxy/ai/workbench/editor/MarkdownOutlinePage.java:69:\t\tMarkdownDocument ast = editor.getMarkdownAst();\n"
```
/answer d77cf02c-857f-4c48-962c-5245f51b00c4 allow
Text:
 I found another gap: when the document is swapped later (e.g. "Save As"), no flush is scheduled. Let me fix that too.
ReasoningToken: 466
Text:
 Let me add proper visibility guarantees for fields read across threads (UI thread writes, background thread reads).
ReasoningToken: 1224
Text:
 Now let's do a final review of all changed files to check for consistency and correctness.
Text:
 This all looks correct. Let me do a final sanity grep across the codebase for any remaining stale references or compile-breaking gaps.
Text:
 Good, everything is consistent. Let me also double check `AIRuleScanner.java` and `AISourceViewerConfiguration.java` once more for full correctness.
Everything is consistent (`MarkdownOutlinePage` still uses `editor.getMarkdownAst()`/`getMarkdownDocument()`, which now simply delegate to the manager — no change needed there).

## Zusammenfassung

Ich habe den vorgeschlagenen Ansatz geprüft, für sinnvoll befunden und umgesetzt:

**Neu:** `xy.ai.workbench.editor.update.EditorUpdateManager` – das zentrale Document/Editor-Addon.
- Einziger `IDocumentListener`/`ITextInputListener` pro Editor (statt bisher zwei separate: einer in `AITextEditor`, einer in `SpellCheckReconciler`).
- Pflegt das `MarkdownDocument`-AST **synchron** pro Edit (`onAstUpdated`) – notwendig, da Syntax-Highlighting (`AIRuleScanner`/`AISourceViewerConfiguration`) exakte, aktuelle Regionen für jedes einzelne Edit braucht.
- Bündelt/debounced die daraus abgeleiteten, teureren Reaktionen (Spellcheck, Outline-Refresh) über eine gemeinsame `onSettled`-Benachrichtigung nach 200ms Inaktivität, ausgeführt auf einem einzigen Hintergrundthread pro Editor (statt vorher einem eigenen `ScheduledExecutorService` nur im Spellcheck).

**Geändert:**
- `AITextEditor`: AST/Buffer-Felder entfernt, delegiert an `EditorUpdateManager`; registriert Listener für Ruler/Zeilennummern (sofort) und Outline-Refresh (debounced, per `Display.asyncExec` zurück auf UI-Thread, da `onSettled` auf dem Hintergrundthread feuert); `dispose()` deinstalliert den Manager sauber.
- `SpellCheckReconciler`: installiert keine eigenen Listener/Timer mehr, sondern registriert sich als `EditorUpdateManager.Listener`; nutzt weiterhin die Leaf-genaue Dirty/Clear-Merge-Logik, getriggert aber zentral über `onAstUpdated`/`onSettled`. Initiale/Dokumentwechsel-Prüfung läuft bewusst über `manager.runAsync(...)`, um den UI-Thread nicht mit dem blockierenden LanguageTool-Aufruf zu belasten.
- `SpellCheckInstaller`: eigener Delay-Parameter entfernt (Debounce jetzt zentral im Manager).
- `AIRuleScanner`/`AISourceViewerConfiguration`: greifen konsistent über `editor.getUpdateManager()` auf AST/letzte Änderungsregion zu.

**Wichtiger Punkt bei der Umsetzung:** Das AST-Update selbst bleibt bewusst synchron pro Tastenanschlag (nicht debounced), da Syntax-Highlighting sonst mit veralteten/inkonsistenten Baumzuständen arbeiten würde und eine korrekte inkrementelle Nachbearbeitung von gebündelten Edits gegen den bereits vollständig veränderten Dokumenttext technisch nicht sauber möglich ist (keine Zwischenzustände mehr verfügbar). Debounced wird stattdessen die **Weiterverarbeitung** (Spellcheck-Netzwerkaufrufe, Outline-Rebuild), was den eigentlichen Performance-Gewinn bringt und Thread-Sicherheit (kein Blockieren des UI-Threads) sauber berücksichtigt.
Ich dachte das anders. Das AST ist das zentrale Element das enkoppelt werden muss. Aus den AST Änderungen ergeben sich direkt die Regionen für das Syntaxhighlighting. Wenn das Highlighting etwas hinterherhängt ist das kein Problem. Der Manager kann das Syntaxhiglighting direkt mit den Ergebnissen des AST reparse aufrufen. Das macht auch die last changes Felder unnötig.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=4b8517bb-208b-4718-9a99-4ce4ecd1377e | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 5f0f8adc-2cd9-45c4-bffa-ee6fc2bb2c8d
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -name LineIndex.java
  reason: Locate LineIndex class
```
/answer 5f0f8adc-2cd9-45c4-bffa-ee6fc2bb2c8d allow
Control Request:
```yaml
id: 25e07213-c2cd-4b81-aa35-2eb694c47328
result:
  structuredContent:
    stdout: |
      ./src/xy/ai/workbench/tools/LineIndex.java
```
/answer 25e07213-c2cd-4b81-aa35-2eb694c47328 allow
ReasoningToken: 8956
Control Request:
```yaml
id: 4cdee300-01bd-490e-b8a6-f70da20b472f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java
  reason: Verify LineIndex.update contract for batching feasibility
```
/answer 4cdee300-01bd-490e-b8a6-f70da20b472f allow
Text:
 Verstanden — guter Einwand. Ich baue das AST-Update selbst als das debounced, zentrale Element um (Edits werden zu einem einzigen Diff komponiert und erst nach der Ruhephase geparst), und der Manager stößt das Syntax-Highlighting direkt aktiv an (statt dass es sich die Region pull-basiert holt). Das macht `lastAstChangeRegion` als eigenes Feld überflüssig.
Control Request:
```yaml
id: 06bfdde3-9d1e-462a-a5dc-963aa2ca3674
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  mode: replace
  content: "package xy.ai.workbench.editor.update;\n\nimport java.util.List;\nimport java.util.concurrent.CopyOnWriteArrayList;\nimport java.util.concurrent.ExecutorService;\nimport java.util.concurrent.Executors;\n\nimport org.eclipse.jface.text.DocumentEvent;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IDocumentListener;\nimport org.eclipse.jface.text.ITextInputListener;\nimport org.eclipse.jface.text.ITextViewer;\nimport org.eclipse.jface.text.ITextViewerExtension2;\nimport org.eclipse.swt.widgets.Display;\n\nimport xy.ai.workbench.editor.DocumentBuffer;\nimport xy.ai.workbench.editor.mdast.MarkdownDocument;\nimport xy.ai.workbench.editor.mdast.TextRegion;\n\n/**\n * Document/editor addon that bundles <b>all</b> raw document event handling\n * ({@link IDocumentListener}/{@link ITextInputListener}) for a single editor\n * instance behind one place, instead of every feature (AST, spell checking,\n * syntax highlighting, outline, ...) installing its own listeners directly on\n * the {@link IDocument}.\n * <p>\n * The AST ({@link MarkdownDocument}) is the central, decoupled element: raw\n * edits are not reparsed one by one. Instead every edit is losslessly composed\n * into a single pending (offset, removedLength, insertedLength) delta, and the\n * actual AST reparse only happens once, {@value #DEBOUNCE_DELAY_MS}ms after\n * the last edit (\"debounce\"). This works because {@link MarkdownDocument}\n * reparses lazily against the live buffer at the time {@code update()} is\n * called - composing edits before calling it is equivalent to calling it once\n * per edit, just delayed and cheaper.\n * <p>\n * Everything else derives from that single reparse result: syntax\n * highlighting is driven <b>directly</b> by the manager (it pushes the\n * changed region into the viewer via {@code invalidateTextPresentation}\n * instead of consumers pulling a \"last change region\" on every repaint), and\n * all other {@link Listener}s (spell checking, outline, ...) are notified once\n * per reparse, on the UI thread. A brief lag between typing and highlighting/\n * spell check/outline update is an accepted, deliberate trade-off.\n */\npublic final class EditorUpdateManager {\n\n\tpublic static final int DEBOUNCE_DELAY_MS = 200;\n\n\t/** Callback interface implemented by features that want to react to AST changes. */\n\tpublic interface Listener {\n\n\t\t/** Fired whenever the underlying document instance itself is swapped or cleared. */\n\t\tdefault void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t}\n\n\t\t/**\n\t\t * Fired once, on the UI thread, after the debounced AST reparse for a\n\t\t * batch of edits has completed, with the resulting changed region.\n\t\t * Listeners that need to perform expensive/blocking work (e.g. spell\n\t\t * checking) should hand it off via {@link EditorUpdateManager#runAsync(Runnable)}\n\t\t * instead of blocking this call.\n\t\t */\n\t\tdefault void onAstUpdated(TextRegion region) {\n\t\t}\n\t}\n\n\tprivate final List<Listener> listeners = new CopyOnWriteArrayList<>();\n\n\tprivate final ExecutorService background = Executors.newSingleThreadExecutor(r -> {\n\t\tThread t = new Thread(r, \"EditorUpdateManager-Background\");\n\t\tt.setDaemon(true);\n\t\treturn t;\n\t});\n\n\tprivate ITextViewer viewer;\n\tprivate Display display;\n\tprivate IDocument document;\n\tprivate DocumentBuffer astBuffer;\n\tprivate MarkdownDocument ast;\n\n\t// ── pending, composed (not yet reparsed) edit ────────────────────────────────\n\tprivate boolean pendingActive;\n\tprivate int pendingStart;\n\tprivate int pendingOldLen;\n\tprivate int pendingNewLen;\n\n\tprivate final Runnable flushRunnable = this::flush;\n\n\tprivate final IDocumentListener documentListener = new IDocumentListener() {\n\t\tprivate int removedLen;\n\n\t\t@Override\n\t\tpublic void documentAboutToBeChanged(DocumentEvent event) {\n\t\t\tremovedLen = event.getLength();\n\t\t}\n\n\t\t@Override\n\t\tpublic void documentChanged(DocumentEvent event) {\n\t\t\tString text = event.getText();\n\t\t\tint insertedLen = text == null ? 0 : text.length();\n\t\t\tcomposeEdit(event.getOffset(), removedLen, insertedLen);\n\t\t\tscheduleFlush();\n\t\t}\n\t};\n\n\tprivate final ITextInputListener textInputListener = new ITextInputListener() {\n\t\t@Override\n\t\tpublic void inputDocumentAboutToBeChanged(IDocument oldInput, IDocument newInput) {\n\t\t\tcancelPending();\n\t\t\tif (oldInput != null)\n\t\t\t\toldInput.removeDocumentListener(documentListener);\n\t\t}\n\n\t\t@Override\n\t\tpublic void inputDocumentChanged(IDocument oldInput, IDocument newInput) {\n\t\t\tdocument = newInput;\n\t\t\tTextRegion initial = null;\n\t\t\tif (newInput != null) {\n\t\t\t\tastBuffer = new DocumentBuffer(newInput);\n\t\t\t\tast = new MarkdownDocument(astBuffer);\n\t\t\t\tinitial = ast.update(0, 0, astBuffer.length());\n\t\t\t\tnewInput.addDocumentListener(documentListener);\n\t\t\t} else {\n\t\t\t\tastBuffer = null;\n\t\t\t\tast = null;\n\t\t\t}\n\t\t\tfor (Listener l : listeners)\n\t\t\t\tl.onDocumentChanged(oldInput, newInput);\n\t\t\tif (initial != null)\n\t\t\t\tpushAstUpdated(initial);\n\t\t}\n\t};\n\n\t/** Hooks the manager into the given viewer's document lifecycle. */\n\tpublic void install(ITextViewer viewer) {\n\t\tthis.viewer = viewer;\n\t\tif (viewer.getTextWidget() != null)\n\t\t\tdisplay = viewer.getTextWidget().getDisplay();\n\t\tviewer.addTextInputListener(textInputListener);\n\t\tIDocument doc = viewer.getDocument();\n\t\tif (doc != null)\n\t\t\ttextInputListener.inputDocumentChanged(null, doc);\n\t}\n\n\t/** Reverses {@link #install(ITextViewer)} and releases all resources. */\n\tpublic void uninstall() {\n\t\tcancelPending();\n\t\tbackground.shutdownNow();\n\t\tif (document != null)\n\t\t\tdocument.removeDocumentListener(documentListener);\n\t\tif (viewer != null)\n\t\t\tviewer.removeTextInputListener(textInputListener);\n\t\tlisteners.clear();\n\t}\n\n\tpublic void addListener(Listener listener) {\n\t\tlisteners.add(listener);\n\t}\n\n\tpublic void removeListener(Listener listener) {\n\t\tlisteners.remove(listener);\n\t}\n\n\tpublic MarkdownDocument getAst() {\n\t\treturn ast;\n\t}\n\n\tpublic IDocument getDocument() {\n\t\treturn document;\n\t}\n\n\t/**\n\t * Runs {@code task} on the manager's shared background thread, off the UI\n\t * thread. For listeners of {@link Listener#onAstUpdated(TextRegion)} that\n\t * need to perform blocking work (e.g. spell checking) without stalling the\n\t * UI thread the reparse itself runs on.\n\t */\n\tpublic void runAsync(Runnable task) {\n\t\tif (!background.isShutdown())\n\t\t\tbackground.execute(task);\n\t}\n\n\t// ── edit composition ─────────────────────────────────────────────────────────\n\n\t/**\n\t * Losslessly folds a new raw edit (given in current/live document\n\t * coordinates) into the still-pending, not yet reparsed edit, so that a\n\t * whole burst of edits can be represented - and later applied to the AST -\n\t * as if it was a single edit.\n\t */\n\tprivate void composeEdit(int offset, int removedLen, int insertedLen) {\n\t\tif (!pendingActive) {\n\t\t\tpendingActive = true;\n\t\t\tpendingStart = offset;\n\t\t\tpendingOldLen = removedLen;\n\t\t\tpendingNewLen = insertedLen;\n\t\t\treturn;\n\t\t}\n\n\t\tif (offset < pendingStart) {\n\t\t\tint leftExtra = pendingStart - offset;\n\t\t\tpendingStart = offset;\n\t\t\tpendingOldLen += leftExtra;\n\t\t\tpendingNewLen += leftExtra;\n\t\t}\n\n\t\tint pendingNewEnd = pendingStart + pendingNewLen;\n\t\tint editEnd = offset + removedLen;\n\t\tif (editEnd > pendingNewEnd) {\n\t\t\tint rightExtra = editEnd - pendingNewEnd;\n\t\t\tpendingOldLen += rightExtra;\n\t\t\tpendingNewLen += rightExtra;\n\t\t}\n\n\t\tpendingNewLen += insertedLen - removedLen;\n\t}\n\n\t// ── debounce ──────────────────────────────────────────────────────────────────\n\n\tprivate void scheduleFlush() {\n\t\tif (display == null || display.isDisposed())\n\t\t\treturn;\n\t\tdisplay.timerExec(DEBOUNCE_DELAY_MS, flushRunnable);\n\t}\n\n\tprivate void cancelPending() {\n\t\tpendingActive = false;\n\t\tif (display != null && !display.isDisposed())\n\t\t\tdisplay.timerExec(-1, flushRunnable);\n\t}\n\n\t/** Runs on the UI thread (via {@link Display#timerExec}) once editing has settled. */\n\tprivate void flush() {\n\t\tif (!pendingActive || ast == null || astBuffer == null)\n\t\t\treturn;\n\t\tint offset = pendingStart;\n\t\tint removed = pendingOldLen;\n\t\tint inserted = pendingNewLen;\n\t\tpendingActive = false;\n\n\t\tTextRegion region = ast.update(offset, removed, inserted);\n\t\tpushAstUpdated(region);\n\t}\n\n\t/** Directly drives syntax highlighting from the reparse result, then notifies listeners. */\n\tprivate void pushAstUpdated(TextRegion region) {\n\t\tif (viewer instanceof ITextViewerExtension2 ext2) {\n\t\t\ttry {\n\t\t\t\text2.invalidateTextPresentation(region.offset(), Math.max(1, region.length()));\n\t\t\t} catch (IllegalArgumentException e) {\n\t\t\t\t// region outside the (possibly just replaced) document - ignore.\n\t\t\t}\n\t\t}\n\t\tfor (Listener l : listeners)\n\t\t\tl.onAstUpdated(region);\n\t}\n}\n"
  reason: "Redesign manager: debounce the AST reparse itself and push syntax highlighting updates directly, eliminating pull-based last-change-region field"
```
/answer 06bfdde3-9d1e-462a-a5dc-963aa2ca3674 allow
Control Request:
```yaml
id: 5bb6c872-77fc-4860-bdc5-fd4a4c901409
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  old_text: "\t\tupdateManager.addListener(new EditorUpdateManager.Listener() {\n\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\trefreshOutline();\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {\n\t\t\t\tIDocument doc = updateManager.getDocument();\n\t\t\t\tif (doc == null)\n\t\t\t\t\treturn;\n\t\t\t\tupdateRulerVisibility(doc);\n\t\t\t\tupdateLineNumbers(doc);\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onSettled(IRegion mergedRegion) {\n\t\t\t\t// onSettled runs on a background thread - outline refresh touches SWT.\n\t\t\t\tDisplay.getDefault().asyncExec(AITextEditor.this::refreshOutline);\n\t\t\t}\n\t\t});\n\t\tupdateManager.install(sourceViewer);"
  new_text: "\t\tupdateManager.addListener(new EditorUpdateManager.Listener() {\n\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\trefreshOutline();\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {\n\t\t\t\t// Runs on the UI thread (the debounced AST reparse itself runs there).\n\t\t\t\tIDocument doc = updateManager.getDocument();\n\t\t\t\tif (doc == null)\n\t\t\t\t\treturn;\n\t\t\t\tupdateRulerVisibility(doc);\n\t\t\t\tupdateLineNumbers(doc);\n\t\t\t\trefreshOutline();\n\t\t\t}\n\t\t});\n\t\tupdateManager.install(sourceViewer);"
  reason: Simplify listener now that AST update itself is debounced and runs on UI thread
```
/answer 5bb6c872-77fc-4860-bdc5-fd4a4c901409 allow
Control Request:
```yaml
id: 3e2bf8f6-92a8-46da-aec9-7939b97a12cb
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  old_text: "\t@Override\n\tpublic void dispose() {\n\t\tif (getSourceViewer() != null)\n\t\t\tupdateManager.uninstall(getSourceViewer());\n\t\tsuper.dispose();\n\t}"
  new_text: "\t@Override\n\tpublic void dispose() {\n\t\tupdateManager.uninstall();\n\t\tsuper.dispose();\n\t}"
  reason: "Manager now stores its own viewer reference, no arg needed"
```
/answer 3e2bf8f6-92a8-46da-aec9-7939b97a12cb allow
Control Request:
```yaml
id: b22082bb-94e6-48aa-8388-894f81065ef2
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  old_text: "\tpublic MarkdownDocument getMarkdownAst() {\n\t\treturn updateManager.getAst();\n\t}\n\n\tpublic TextRegion getLastAstChangeRegion() {\n\t\treturn updateManager.getLastAstChangeRegion();\n\t}\n\n\t/** Central document/editor addon bundling AST, spell check and syntax highlighting event handling. */"
  new_text: "\tpublic MarkdownDocument getMarkdownAst() {\n\t\treturn updateManager.getAst();\n\t}\n\n\t/** Central document/editor addon bundling AST, spell check and syntax highlighting event handling. */"
  reason: "Remove last-change-region pull API, no longer needed"
```
/answer b22082bb-94e6-48aa-8388-894f81065ef2 allow
Control Request:
```yaml
id: 00cfb35a-fded-4979-b8ef-d1bdb7564a94
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  reason: Check for unused IRegion import and overall consistency
```
/answer 00cfb35a-fded-4979-b8ef-d1bdb7564a94 allow
Control Request:
```yaml
id: 2b51fb61-6a68-4282-8f98-34cd8daa7cd7
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  old_text: |-
    import org.eclipse.jface.text.IDocument;
    import org.eclipse.jface.text.IRegion;
    import org.eclipse.jface.text.source.CompositeRuler;
  new_text: |-
    import org.eclipse.jface.text.IDocument;
    import org.eclipse.jface.text.source.CompositeRuler;
  reason: Remove unused import
```
/answer 2b51fb61-6a68-4282-8f98-34cd8daa7cd7 allow
Control Request:
```yaml
id: 5ed9d4fa-1270-4c74-839d-296def6bb6f7
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  old_text: "\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\trefreshOutline();\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {"
  new_text: "\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\tif (newDocument == null)\n\t\t\t\t\trefreshOutline(); // clears the outline; onAstUpdated covers the non-null case\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {"
  reason: Avoid duplicate outline refresh on document load; keep clearing behaviour for document==null
```
/answer 5ed9d4fa-1270-4c74-839d-296def6bb6f7 allow
Control Request:
```yaml
id: 03c2a67b-17f2-4100-8324-c30c6df20f00
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  reason: Review before editing damage region logic
```
/answer 03c2a67b-17f2-4100-8324-c30c6df20f00 allow
Control Request:
```yaml
id: 79901e4c-bf29-4aec-b313-e952ff801415
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  old_text: "\t\t\t@Override\n\t\t\tpublic IRegion getDamageRegion(ITypedRegion partition, DocumentEvent e,\n\t\t\t\t\tboolean documentPartitioningChanged) {\n\t\t\t\tIDocument document = sourceViewer.getDocument();\n\t\t\t\tif (document == null)\n\t\t\t\t\treturn partition;\n\t\t\t\tif (document.getLength() > LIMIT)\n\t\t\t\t\treturn new Region(0, 1);\n\n\t\t\t\tTextRegion astRegion = editor != null ? editor.getUpdateManager().getLastAstChangeRegion() : null;\n\t\t\t\tif (astRegion != null) {\n\t\t\t\t\tint offset = Math.max(0, Math.min(astRegion.offset(), document.getLength()));\n\t\t\t\t\tint end = Math.max(offset, Math.min(astRegion.offset() + astRegion.length(),\n\t\t\t\t\t\t\tdocument.getLength()));\n\t\t\t\t\treturn new Region(offset, end - offset);\n\t\t\t\t}\n\n\t\t\t\treturn super.getDamageRegion(partition, e, documentPartitioningChanged);\n\t\t\t}"
  new_text: "\t\t\t@Override\n\t\t\tpublic IRegion getDamageRegion(ITypedRegion partition, DocumentEvent e,\n\t\t\t\t\tboolean documentPartitioningChanged) {\n\t\t\t\tIDocument document = sourceViewer.getDocument();\n\t\t\t\tif (document == null)\n\t\t\t\t\treturn partition;\n\t\t\t\tif (document.getLength() > LIMIT)\n\t\t\t\t\treturn new Region(0, 1);\n\n\t\t\t\t// No AST-based damage region here anymore: the AST reparse (and thus the\n\t\t\t\t// precise, authoritative changed region) is debounced centrally in\n\t\t\t\t// EditorUpdateManager, which actively pushes a repaint for that region via\n\t\t\t\t// invalidateTextPresentation() once it is available. This default,\n\t\t\t\t// per-edit damage computation only provides transient, best-effort\n\t\t\t\t// highlighting in between - falling behind briefly is acceptable.\n\t\t\t\treturn super.getDamageRegion(partition, e, documentPartitioningChanged);\n\t\t\t}"
  reason: Remove pull-based AST damage region logic; highlighting is now pushed directly by the manager
```
/answer 79901e4c-bf29-4aec-b313-e952ff801415 allow
Control Request:
```yaml
id: 0ac74e1d-8463-47e1-9933-dea2da53803c
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  old_text: |-
    import xy.ai.workbench.editor.mdast.TextRegion;
    import xy.ai.workbench.editor.spellcheck.SpellCheckInstaller;
  new_text: import xy.ai.workbench.editor.spellcheck.SpellCheckInstaller;
  reason: Remove unused import
```
/answer 0ac74e1d-8463-47e1-9933-dea2da53803c allow
ReasoningToken: 10664
Text:
 Now update `AITextEditor`:
Control Request:
```yaml
id: b6b6da0c-beff-4662-b4ef-8f1283b74e01
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  mode: replace
  content: "package xy.ai.workbench.editor.spellcheck;\n\nimport java.util.ArrayList;\nimport java.util.Iterator;\nimport java.util.List;\n\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IRegion;\nimport org.eclipse.jface.text.ITextViewer;\nimport org.eclipse.jface.text.Region;\nimport org.eclipse.jface.text.reconciler.IReconciler;\nimport org.eclipse.jface.text.reconciler.IReconcilingStrategy;\n\nimport xy.ai.workbench.editor.AITextEditor;\nimport xy.ai.workbench.editor.mdast.TextRegion;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\nimport xy.ai.workbench.editor.update.EditorUpdateManager;\n\n/**\n * Reconciler that tracks only the document region actually changed since the\n * previous check.\n * <p>\n * All raw document events, as well as the (debounced) AST reparse itself, are\n * handled centrally by the {@link EditorUpdateManager} (the editor's\n * document/editor addon). This reconciler no longer installs its own\n * {@code IDocumentListener}/{@code ITextInputListener} or debounce timer - it\n * merely registers itself as an {@link EditorUpdateManager.Listener}: every\n * (already debounced) AST update ({@link EditorUpdateManager.Listener#onAstUpdated(TextRegion)})\n * is used to widen the pending dirty/clear regions and, since it fires on the\n * UI thread, immediately handed off to {@link EditorUpdateManager#runAsync(Runnable)}\n * so the actual (blocking) spell check never runs on the UI thread.\n */\npublic class SpellCheckReconciler implements IReconciler {\n\n\tprivate final SpellingStrategy fStrategy;\n\tprivate final AITextEditor fEditor;\n\n\tprivate final List<int[]> fDirtyRegions = new ArrayList<>();\n\tprivate final List<int[]> fClearRegions = new ArrayList<>();\n\n\tprivate EditorUpdateManager.Listener fListener;\n\n\tpublic SpellCheckReconciler(SpellingStrategy strategy, AITextEditor editor) {\n\t\tfStrategy = strategy;\n\t\tfEditor = editor;\n\t}\n\n\t// ── IReconciler ────────────────────────────────────────────────────────────\n\n\t@Override\n\tpublic void install(ITextViewer textViewer) {\n\t\tfListener = new EditorUpdateManager.Listener() {\n\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\tfStrategy.setDocument(newDocument);\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {\n\t\t\t\tif (region == null)\n\t\t\t\t\treturn;\n\t\t\t\tmergeEnabledLeaves(region.n());\n\t\t\t\tfEditor.getUpdateManager().runAsync(SpellCheckReconciler.this::flush);\n\t\t\t}\n\t\t};\n\n\t\tEditorUpdateManager manager = fEditor.getUpdateManager();\n\t\tmanager.addListener(fListener);\n\n\t\t// Defensive fallback for the (unlikely) case the document was already fully\n\t\t// parsed before this reconciler had a chance to register: trigger a full\n\t\t// check off the calling (UI) thread.\n\t\tIDocument doc = manager.getDocument();\n\t\tif (doc != null && manager.getAst() != null) {\n\t\t\tfStrategy.setDocument(doc);\n\t\t\tmergeDirty(0, doc.getLength());\n\t\t\tmanager.runAsync(this::flush);\n\t\t}\n\t}\n\n\t@Override\n\tpublic void uninstall() {\n\t\tif (fListener != null) {\n\t\t\tfEditor.getUpdateManager().removeListener(fListener);\n\t\t\tfListener = null;\n\t\t}\n\t}\n\n\t@Override\n\tpublic IReconcilingStrategy getReconcilingStrategy(String contentType) {\n\t\treturn IDocument.DEFAULT_CONTENT_TYPE.equals(contentType) ? fStrategy : null;\n\t}\n\n\t// ── dirty/clear region bookkeeping ───────────────────────────────────────────\n\n\tprivate boolean mergeEnabledLeaves(Node node) {\n\t\tif (node == null)\n\t\t\treturn false;\n\t\tif (node.children.isEmpty()) {\n\t\t\tint start = node.getOffset();\n\t\t\tint end = node.getEndOffset();\n\t\t\tif (!node.enableSpellcheck) {\n\t\t\t\tmergeClear(start, Math.max(end, start + 1));\n\t\t\t\treturn true;\n\t\t\t}\n\t\t\tmergeDirty(start, Math.max(end, start + 1));\n\t\t\treturn true;\n\t\t}\n\t\tboolean merged = false;\n\t\tfor (Node child : node.children)\n\t\t\tmerged |= mergeEnabledLeaves(child);\n\t\treturn merged;\n\t}\n\n\tprivate synchronized void mergeDirty(int start, int end) {\n\t\tmerge(fDirtyRegions, start, end);\n\t}\n\n\tprivate synchronized void mergeClear(int start, int end) {\n\t\tmerge(fClearRegions, start, end);\n\t}\n\n\tprivate static void merge(List<int[]> regions, int start, int end) {\n\t\tint newStart = start;\n\t\tint newEnd = end;\n\t\tfor (Iterator<int[]> it = regions.iterator(); it.hasNext();) {\n\t\t\tint[] r = it.next();\n\t\t\t// Overlapping or directly adjacent -> merge.\n\t\t\tif (newStart <= r[1] && r[0] <= newEnd) {\n\t\t\t\tnewStart = Math.min(newStart, r[0]);\n\t\t\t\tnewEnd = Math.max(newEnd, r[1]);\n\t\t\t\tit.remove();\n\t\t\t}\n\t\t}\n\t\tregions.add(new int[] { newStart, newEnd });\n\t}\n\n\tprivate synchronized List<IRegion> takeDirty() {\n\t\treturn take(fDirtyRegions);\n\t}\n\n\tprivate synchronized List<IRegion> takeClear() {\n\t\treturn take(fClearRegions);\n\t}\n\n\tprivate static List<IRegion> take(List<int[]> regions) {\n\t\tif (regions.isEmpty())\n\t\t\treturn null;\n\t\tList<IRegion> result = new ArrayList<>(regions.size());\n\t\tfor (int[] r : regions)\n\t\t\tresult.add(new Region(r[0], r[1] - r[0]));\n\t\tregions.clear();\n\t\treturn result;\n\t}\n\n\tprivate void flush() {\n\t\tList<IRegion> clear = takeClear();\n\t\tif (clear != null)\n\t\t\tfor (IRegion region : clear)\n\t\t\t\tfStrategy.clear(region);\n\n\t\tList<IRegion> dirty = takeDirty();\n\t\tif (dirty != null)\n\t\t\tfor (IRegion region : dirty)\n\t\t\t\tfStrategy.reconcile(region);\n\t}\n}\n"
  reason: Route spellcheck through unified onAstUpdated callback of redesigned manager
```
/answer b6b6da0c-beff-4662-b4ef-8f1283b74e01 allow
Control Request:
```yaml
id: f2c35213-8bf4-4e0b-9a54-a99f3b0e30eb
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  script: grep -rn "getLastAstChangeRegion\|onSettled\|lastAstChangeRegion" . ; echo "---"; grep -rln "EditorUpdateManager" .
  reason: Check for stale references to removed API
```
/answer f2c35213-8bf4-4e0b-9a54-a99f3b0e30eb allow
Control Request:
```yaml
id: eb691862-3272-47cb-93cc-d1b5215e9556
result:
  structuredContent:
    stdout: |
      ---
      ./xy/ai/workbench/editor/update/EditorUpdateManager.java
      ./xy/ai/workbench/editor/AITextEditor.java
      ./xy/ai/workbench/editor/AISourceViewerConfiguration.java
      ./xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
```
/answer eb691862-3272-47cb-93cc-d1b5215e9556 allow
Control Request:
```yaml
id: 0e9a9592-5f66-46bb-b3ee-5c79ca0d8638
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  reason: Final review
```
/answer 0e9a9592-5f66-46bb-b3ee-5c79ca0d8638 allow
Control Request:
```yaml
id: edc5f28e-f772-42ac-9eab-934b78a79ba3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  reason: Final review
```
/answer edc5f28e-f772-42ac-9eab-934b78a79ba3 allow
Control Request:
```yaml
id: 58b46595-1d78-4ebb-9e0f-fac94a0e7c00
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/update/EditorUpdateManager.java
  reason: Final review
```
/answer 58b46595-1d78-4ebb-9e0f-fac94a0e7c00 allow
ReasoningToken: 56
Text:
 Now remove `getLastAstChangeRegion()` and the now-unused `IRegion` import, and clean up.
Text:
 IRegion import now unused; remove it. Also `onDocumentChanged` and `onAstUpdated` both call `refreshOutline()` (harmless double call on load, but let's simplify by removing it from `onDocumentChanged` since `onAstUpdated` always fires right after for a non-null document).
Text:
 Now update `AISourceViewerConfiguration` — remove the AST-region pull logic from `getDamageRegion`, since highlighting is now driven directly by the manager's push.
ReasoningToken: 201
Text:
 Now clean up unused import `TextRegion` in this file.
Text:
 Now update `SpellCheckReconciler` to use the unified `onAstUpdated` callback instead of the previous `onSettled`/dual-callback design.
Text:
 Let's verify there are no remaining stale references across the codebase.
ReasoningToken: 28
Text:
 Good, no stale references. Let's do a final full review of all four touched/created files together for consistency.
ReasoningToken: 1478
Danke für die Klarstellung – das ist tatsächlich der bessere Ansatz. Ich habe den Manager entsprechend umgebaut.

## Was sich geändert hat

**Kernidee vorher (falsch):** AST wurde weiterhin synchron pro Tastendruck aktualisiert; nur Spellcheck/Outline wurden debounced benachrichtigt. Syntax-Highlighting blieb pull-basiert (`getLastAstChangeRegion()`).

**Kernidee jetzt (wie gewünscht):** Das AST-Reparsing selbst ist das debounced Element:

1. **Edit-Komposition statt Sofort-Reparse:** Jedes rohe `DocumentEvent` wird nicht mehr sofort an `MarkdownDocument.update()` weitergereicht, sondern verlustfrei in ein einziges pending `(offset, removedLength, insertedLength)`-Delta einkomponiert (klassische OT-artige Delta-Komposition). Das funktioniert korrekt, weil `MarkdownDocument.update()` beim tatsächlichen Aufruf ohnehin immer live gegen den aktuellen Puffer parst – mehrere Edits vorher zusammenzufassen ist äquivalent zu N Einzelaufrufen, nur einmal statt N-mal ausgeführt.
2. **Debounce via `Display.timerExec`:** Nach 200ms Tipp-Pause wird `MarkdownDocument.update(...)` genau einmal mit dem komponierten Delta aufgerufen – auf dem UI-Thread (nötig für sicheren `IDocument`-Zugriff).
3. **Push statt Pull fürs Highlighting:** Das Ergebnis des Reparse (die `TextRegion`) wird vom Manager **aktiv** über `sourceViewer.invalidateTextPresentation(offset, length)` an den Viewer geschickt. Damit entfällt `getLastAstChangeRegion()` als Feld/API vollständig – kein Consumer muss mehr "die letzte Änderungsregion" abfragen. `AISourceViewerConfiguration.getDamageRegion()` nutzt jetzt nur noch JFace-Standardverhalten als Zwischenlösung, bis der Manager den präzisen Repaint nachliefert (bewusst akzeptiertes kurzes Hinterherhinken).
4. **Einheitliches `onAstUpdated`-Event:** Spellcheck und Outline reagieren jetzt auf denselben einzigen, bereits debounced Event – kein separates `onSettled` mehr nötig. Spellcheck hängt seinen blockierenden LanguageTool-Call über `manager.runAsync(...)` aus, damit der UI-Thread (auf dem der Reparse läuft) nicht blockiert.

**Entfernt:** `lastAstChangeRegion`-Feld/Getter in `AITextEditor`/`AISourceViewerConfiguration`/`SpellCheckReconciler` – wie gewünscht komplett unnötig geworden.

Die Aggregation der Piece in AIRuleScanner kann nun extrahiert und wiederverwendbar gemacht werden. Der Manager ist dann die einzige Komponente die Änderungen noch aggregieren muss und dann nacheinander Bereiche durch den AST vorverarbeitet und expandiert und anschließend SpellChecl und Syntax Highlighting darauf anwenden kann.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=4b8517bb-208b-4718-9a99-4ce4ecd1377e | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: e5b6535f-f2ba-425f-925b-4a71d0517a78
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Node.java
  reason: Understand Node structure for extracting reusable walker
```
/answer e5b6535f-f2ba-425f-925b-4a71d0517a78 deny die Node Struktur dafür ist nicht Relevant. Du kannst die `List<Piece> pieces` Implementierung als Grundlage nehmen. Es geht nur um das Zussamenschließen von Überlappenden Regionen von Änderungen.
Control Request:
```yaml
id: 360ba092-3d9b-46da-afd0-e04141c69748
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/AbstractNode.java
  reason: Understand AbstractNode structure
```
/answer 360ba092-3d9b-46da-afd0-e04141c69748 deny ebenfalls nicht relevant
ReasoningToken: 2554
Control Request:
```yaml
id: 17a20dcd-a2f3-49d0-9032-81caad1161a3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  reason: Get current AIRuleScanner content for extraction of Piece list logic
```
/answer 17a20dcd-a2f3-49d0-9032-81caad1161a3 allow
ReasoningToken: 13551
Text:
 Now let's create the reusable `PieceList` utility, then refactor `AIRuleScanner` and `SpellCheckReconciler` to use it.
Control Request:
```yaml
id: e57c6361-5dcb-4218-9c37-edc78f6a6030
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/PieceList.java
  mode: replace
  content: "package xy.ai.workbench.tools;\n\nimport java.util.ArrayList;\nimport java.util.Iterator;\nimport java.util.List;\nimport java.util.Objects;\n\n/**\n * Reusable, ordered collection of (offset, length, value) \"pieces\" describing\n * disjoint regions of a document. Originally the private {@code Piece}\n * record/list bookkeeping of {@code AIRuleScanner}'s token aggregation,\n * extracted so it can be reused wherever regions of change need to be\n * collected and/or merged - e.g. spell-check dirty/clear region tracking.\n * <p>\n * {@link #add(int, int, Object)} merges the new piece with any existing\n * piece(s) it overlaps or directly touches <b>and</b> that carry an equal\n * value, keeping the list free of redundant/overlapping entries. Pieces with\n * different values are kept separate even if they overlap.\n * <p>\n * Besides plain list access ({@link #asList()}, {@link #iterator()}), a\n * simple sequential read cursor ({@link #hasNext()}/{@link #next()}) is\n * provided for consumers that want to step through the pieces once, e.g. to\n * implement {@link org.eclipse.jface.text.rules.ITokenScanner#nextToken()}.\n */\npublic final class PieceList<T> implements Iterable<PieceList.Piece<T>> {\n\n\tpublic record Piece<T>(int offset, int length, T value) {\n\t\tpublic int end() {\n\t\t\treturn offset + length;\n\t\t}\n\t}\n\n\tprivate final List<Piece<T>> pieces = new ArrayList<>();\n\tprivate int cursor;\n\n\t/** Appends a piece, merging it with any overlapping/adjacent piece(s) of equal value. */\n\tpublic void add(int offset, int length, T value) {\n\t\tint start = offset;\n\t\tint end = offset + length;\n\t\tfor (Iterator<Piece<T>> it = pieces.iterator(); it.hasNext();) {\n\t\t\tPiece<T> p = it.next();\n\t\t\tif (start <= p.end() && p.offset() <= end && Objects.equals(p.value(), value)) {\n\t\t\t\tstart = Math.min(start, p.offset());\n\t\t\t\tend = Math.max(end, p.end());\n\t\t\t\tit.remove();\n\t\t\t}\n\t\t}\n\t\tpieces.add(new Piece<>(start, end - start, value));\n\t}\n\n\tpublic boolean isEmpty() {\n\t\treturn pieces.isEmpty();\n\t}\n\n\tpublic int size() {\n\t\treturn pieces.size();\n\t}\n\n\tpublic List<Piece<T>> asList() {\n\t\treturn pieces;\n\t}\n\n\t/** Clears all pieces and resets the read cursor. */\n\tpublic void clear() {\n\t\tpieces.clear();\n\t\tcursor = 0;\n\t}\n\n\t/** Resets the sequential read cursor to the beginning without discarding the pieces. */\n\tpublic void resetCursor() {\n\t\tcursor = 0;\n\t}\n\n\tpublic boolean hasNext() {\n\t\treturn cursor < pieces.size();\n\t}\n\n\tpublic Piece<T> next() {\n\t\treturn pieces.get(cursor++);\n\t}\n\n\t@Override\n\tpublic Iterator<Piece<T>> iterator() {\n\t\treturn pieces.iterator();\n\t}\n}\n"
  reason: Extract reusable piece/region aggregation utility from AIRuleScanner
```
/answer e57c6361-5dcb-4218-9c37-edc78f6a6030 allow
Control Request:
```yaml
id: f6c3497c-d8b7-4c18-bd6e-cf5f9377fe3d
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  old_text: |-
    import xy.ai.workbench.editor.mdast.MarkdownDocument;
    import xy.ai.workbench.editor.mdast.nodes.AbstractNode;
    import xy.ai.workbench.editor.mdast.nodes.Elements;
    import xy.ai.workbench.editor.mdast.nodes.Node;
  new_text: |-
    import xy.ai.workbench.editor.mdast.MarkdownDocument;
    import xy.ai.workbench.editor.mdast.nodes.AbstractNode;
    import xy.ai.workbench.editor.mdast.nodes.Elements;
    import xy.ai.workbench.editor.mdast.nodes.Node;
    import xy.ai.workbench.tools.PieceList;
  reason: Import extracted PieceList
```
/answer f6c3497c-d8b7-4c18-bd6e-cf5f9377fe3d allow
Control Request:
```yaml
id: affbba0e-81f8-49b0-98eb-e32727db5f48
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  old_text: "\tprivate final List<Piece> pieces = new ArrayList<>();\n\tprivate int pieceIndex;\n\tprivate int tokenOffset;\n\tprivate int tokenLength;"
  new_text: "\tprivate final PieceList<IToken> pieces = new PieceList<>();\n\tprivate int tokenOffset;\n\tprivate int tokenLength;"
  reason: Use extracted reusable PieceList instead of private bookkeeping
```
/answer affbba0e-81f8-49b0-98eb-e32727db5f48 allow
Control Request:
```yaml
id: f9335b93-4350-4b38-a67f-c04e498f810f
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  old_text: "\tpublic void setRange(IDocument document, int offset, int length) {\n\t\tpieces.clear();\n\t\tpieceIndex = 0;\n\t\ttokenOffset = offset;"
  new_text: "\tpublic void setRange(IDocument document, int offset, int length) {\n\t\tpieces.clear();\n\t\ttokenOffset = offset;"
  reason: PieceList.clear() already resets the cursor
```
/answer f9335b93-4350-4b38-a67f-c04e498f810f allow
Control Request:
```yaml
id: de77f0dd-c56e-459a-b082-a0aff189a482
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  old_text: "\t\tRuleBasedScanner scanner = scannerByNode.get(type);\n\t\tif (scanner == null) {\n\t\t\tpieces.add(new Piece(start, end - start, RESET_TOKEN));\n\t\t\treturn;\n\t\t}"
  new_text: "\t\tRuleBasedScanner scanner = scannerByNode.get(type);\n\t\tif (scanner == null) {\n\t\t\tpieces.add(start, end - start, RESET_TOKEN);\n\t\t\treturn;\n\t\t}"
  reason: Use PieceList.add
```
/answer de77f0dd-c56e-459a-b082-a0aff189a482 allow
Text:
 Now refactor `AIRuleScanner` to use `PieceList<IToken>`:
Control Request:
```yaml
id: bdfa5fe8-352e-4721-aa09-18a47de612ce
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  old_text: "\t\t\tif (len <= 0)\n\t\t\t\tcontinue;\n\n\t\t\tpieces.add(new Piece(off, len, token));\n\t\t}\n\t}\n\n\tprivate void scanFlat(RuleBasedScanner scanner, IDocument document, int lo, int hi) {\n\t\tapplyDocumentBounds(scanner, document, lo, hi);\n\t\tscanner.setRange(document, lo, hi - lo);\n\t\twhile (true) {\n\t\t\tIToken token = scanner.nextToken();\n\t\t\tif (token.isEOF())\n\t\t\t\tbreak;\n\t\t\tpieces.add(new Piece(scanner.getTokenOffset(), scanner.getTokenLength(), token));\n\t\t}\n\t}\n\n\t@Override\n\tpublic IToken nextToken() {\n\t\tif (pieceIndex >= pieces.size())\n\t\t\treturn Token.EOF;\n\n\t\tPiece p = pieces.get(pieceIndex++);\n\t\ttokenOffset = p.offset;\n\t\ttokenLength = p.length;\n\t\treturn p.token;\n\t}"
  new_text: "\t\t\tif (len <= 0)\n\t\t\t\tcontinue;\n\n\t\t\tpieces.add(off, len, token);\n\t\t}\n\t}\n\n\tprivate void scanFlat(RuleBasedScanner scanner, IDocument document, int lo, int hi) {\n\t\tapplyDocumentBounds(scanner, document, lo, hi);\n\t\tscanner.setRange(document, lo, hi - lo);\n\t\twhile (true) {\n\t\t\tIToken token = scanner.nextToken();\n\t\t\tif (token.isEOF())\n\t\t\t\tbreak;\n\t\t\tpieces.add(scanner.getTokenOffset(), scanner.getTokenLength(), token);\n\t\t}\n\t}\n\n\t@Override\n\tpublic IToken nextToken() {\n\t\tif (!pieces.hasNext())\n\t\t\treturn Token.EOF;\n\n\t\tPieceList.Piece<IToken> p = pieces.next();\n\t\ttokenOffset = p.offset();\n\t\ttokenLength = p.length();\n\t\treturn p.value();\n\t}"
  reason: Use PieceList sequential cursor for nextToken()
```
/answer bdfa5fe8-352e-4721-aa09-18a47de612ce allow
Control Request:
```yaml
id: 0404d525-4080-4146-9ab2-342239033147
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  old_text: "\trecord Piece(int offset, int length, IToken token) {\n\t}\n\n\tprivate static Font[] cachedFonts;"
  new_text: "\tprivate static Font[] cachedFonts;"
  reason: "Remove now-unused private Piece record, replaced by extracted PieceList"
```
/answer 0404d525-4080-4146-9ab2-342239033147 allow
Control Request:
```yaml
id: daa4e37b-bf3b-4187-b3bf-1dc1637757e0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  reason: Verify full file after refactor
```
/answer daa4e37b-bf3b-4187-b3bf-1dc1637757e0 allow
Text:
 Now remove the now-unused local `Piece` record and check the `ArrayList`/`List` imports are still needed elsewhere in the file.
ReasoningToken: 134
Text:
 Now refactor `SpellCheckReconciler` to reuse `PieceList` instead of its own merge logic.
Control Request:
```yaml
id: 188ddbe6-87ef-49b8-8851-0375be096ffe
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  mode: replace
  content: "package xy.ai.workbench.editor.spellcheck;\n\nimport java.util.ArrayList;\nimport java.util.List;\n\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IRegion;\nimport org.eclipse.jface.text.ITextViewer;\nimport org.eclipse.jface.text.Region;\nimport org.eclipse.jface.text.reconciler.IReconciler;\nimport org.eclipse.jface.text.reconciler.IReconcilingStrategy;\n\nimport xy.ai.workbench.editor.AITextEditor;\nimport xy.ai.workbench.editor.mdast.TextRegion;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\nimport xy.ai.workbench.editor.update.EditorUpdateManager;\nimport xy.ai.workbench.tools.PieceList;\n\n/**\n * Reconciler that tracks only the document region actually changed since the\n * previous check.\n * <p>\n * All raw document events, as well as the (debounced) AST reparse itself, are\n * handled centrally by the {@link EditorUpdateManager} (the editor's\n * document/editor addon). This reconciler no longer installs its own\n * {@code IDocumentListener}/{@code ITextInputListener} or debounce timer - it\n * merely registers itself as an {@link EditorUpdateManager.Listener}: every\n * (already debounced) AST update ({@link EditorUpdateManager.Listener#onAstUpdated(TextRegion)})\n * is used to widen the pending dirty/clear regions - via the same, reusable\n * {@link PieceList} that {@code AIRuleScanner} uses to aggregate its rule\n * matches - and, since it fires on the UI thread, immediately handed off to\n * {@link EditorUpdateManager#runAsync(Runnable)} so the actual (blocking)\n * spell check never runs on the UI thread.\n */\npublic class SpellCheckReconciler implements IReconciler {\n\n\t/** Shared, identity-equal marker value so same-kind pieces always get merged by {@link PieceList}. */\n\tprivate static final Object MARK = new Object();\n\n\tprivate final SpellingStrategy fStrategy;\n\tprivate final AITextEditor fEditor;\n\n\tprivate final PieceList<Object> fDirtyRegions = new PieceList<>();\n\tprivate final PieceList<Object> fClearRegions = new PieceList<>();\n\n\tprivate EditorUpdateManager.Listener fListener;\n\n\tpublic SpellCheckReconciler(SpellingStrategy strategy, AITextEditor editor) {\n\t\tfStrategy = strategy;\n\t\tfEditor = editor;\n\t}\n\n\t// ── IReconciler ────────────────────────────────────────────────────────────\n\n\t@Override\n\tpublic void install(ITextViewer textViewer) {\n\t\tfListener = new EditorUpdateManager.Listener() {\n\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\tfStrategy.setDocument(newDocument);\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {\n\t\t\t\tif (region == null)\n\t\t\t\t\treturn;\n\t\t\t\tmergeEnabledLeaves(region.n());\n\t\t\t\tfEditor.getUpdateManager().runAsync(SpellCheckReconciler.this::flush);\n\t\t\t}\n\t\t};\n\n\t\tEditorUpdateManager manager = fEditor.getUpdateManager();\n\t\tmanager.addListener(fListener);\n\n\t\t// Defensive fallback for the (unlikely) case the document was already fully\n\t\t// parsed before this reconciler had a chance to register: trigger a full\n\t\t// check off the calling (UI) thread.\n\t\tIDocument doc = manager.getDocument();\n\t\tif (doc != null && manager.getAst() != null) {\n\t\t\tfStrategy.setDocument(doc);\n\t\t\tmergeDirty(0, doc.getLength());\n\t\t\tmanager.runAsync(this::flush);\n\t\t}\n\t}\n\n\t@Override\n\tpublic void uninstall() {\n\t\tif (fListener != null) {\n\t\t\tfEditor.getUpdateManager().removeListener(fListener);\n\t\t\tfListener = null;\n\t\t}\n\t}\n\n\t@Override\n\tpublic IReconcilingStrategy getReconcilingStrategy(String contentType) {\n\t\treturn IDocument.DEFAULT_CONTENT_TYPE.equals(contentType) ? fStrategy : null;\n\t}\n\n\t// ── dirty/clear region bookkeeping ───────────────────────────────────────────\n\n\tprivate boolean mergeEnabledLeaves(Node node) {\n\t\tif (node == null)\n\t\t\treturn false;\n\t\tif (node.children.isEmpty()) {\n\t\t\tint start = node.getOffset();\n\t\t\tint end = node.getEndOffset();\n\t\t\tif (!node.enableSpellcheck) {\n\t\t\t\tmergeClear(start, Math.max(end, start + 1));\n\t\t\t\treturn true;\n\t\t\t}\n\t\t\tmergeDirty(start, Math.max(end, start + 1));\n\t\t\treturn true;\n\t\t}\n\t\tboolean merged = false;\n\t\tfor (Node child : node.children)\n\t\t\tmerged |= mergeEnabledLeaves(child);\n\t\treturn merged;\n\t}\n\n\tprivate synchronized void mergeDirty(int start, int end) {\n\t\tfDirtyRegions.add(start, end - start, MARK);\n\t}\n\n\tprivate synchronized void mergeClear(int start, int end) {\n\t\tfClearRegions.add(start, end - start, MARK);\n\t}\n\n\tprivate synchronized List<PieceList.Piece<Object>> takeDirty() {\n\t\treturn take(fDirtyRegions);\n\t}\n\n\tprivate synchronized List<PieceList.Piece<Object>> takeClear() {\n\t\treturn take(fClearRegions);\n\t}\n\n\tprivate static List<PieceList.Piece<Object>> take(PieceList<Object> regions) {\n\t\tif (regions.isEmpty())\n\t\t\treturn null;\n\t\tList<PieceList.Piece<Object>> result = new ArrayList<>(regions.asList());\n\t\tregions.clear();\n\t\treturn result;\n\t}\n\n\tprivate void flush() {\n\t\tList<PieceList.Piece<Object>> clear = takeClear();\n\t\tif (clear != null)\n\t\t\tfor (PieceList.Piece<Object> p : clear)\n\t\t\t\tfStrategy.clear(toRegion(p));\n\n\t\tList<PieceList.Piece<Object>> dirty = takeDirty();\n\t\tif (dirty != null)\n\t\t\tfor (PieceList.Piece<Object> p : dirty)\n\t\t\t\tfStrategy.reconcile(toRegion(p));\n\t}\n\n\tprivate static IRegion toRegion(PieceList.Piece<Object> p) {\n\t\treturn new Region(p.offset(), p.length());\n\t}\n}\n"
  reason: "Reuse extracted PieceList instead of bespoke int[]-based region merging"
```
/answer 188ddbe6-87ef-49b8-8851-0375be096ffe allow
Control Request:
```yaml
id: d20a935c-3bd0-4bf1-a0ce-e6161e4c3207
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  script: grep -rln "PieceList" .; echo ---; grep -rn "record Piece\|new Piece(" xy/ai/workbench/editor/AIRuleScanner.java xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  reason: Verify PieceList usage and no leftover stale Piece references
```
/answer d20a935c-3bd0-4bf1-a0ce-e6161e4c3207 allow
Control Request:
```yaml
id: e31f7bf7-a9a4-462f-8338-a154011f2837
result:
  structuredContent:
    exit_code: 1
    stdout: |
      ./xy/ai/workbench/tools/PieceList.java
      ./xy/ai/workbench/editor/AIRuleScanner.java
      ./xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
      ---
```
/answer e31f7bf7-a9a4-462f-8338-a154011f2837 allow
Control Request:
```yaml
id: 9fe12a1d-fa60-41da-9536-2a89d48fdcc3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  reason: Final review
```
/answer 9fe12a1d-fa60-41da-9536-2a89d48fdcc3 allow
ReasoningToken: 209
Text:
 Let's do a final consistency check across the codebase.
Text:
 Sauber – keine Altlasten mehr, beide Stellen nutzen jetzt die extrahierte, gemeinsame Komponente.
Das passt.

## Zusammenfassung

- **Neu:** `xy.ai.workbench.tools.PieceList<T>` – generische, wiederverwendbare Komponente, extrahiert aus `AIRuleScanner`s bisheriger privater `Piece`-Record/`List<Piece>`/Cursor-Logik. Sie fasst überlappende bzw. direkt angrenzende Regionen mit gleichem Wert (`Objects.equals`) automatisch zusammen und bietet zusätzlich einen sequentiellen Lese-Cursor (`hasNext()`/`next()`).
- **`AIRuleScanner`:** nutzt jetzt `PieceList<IToken>` statt eigener Bookkeeping-Implementierung (`record Piece`, `pieceIndex`) – `nextToken()`/`getTokenOffset()`/`getTokenLength()` greifen auf den gemeinsamen Cursor zu.
- **`SpellCheckReconciler`:** die bisher separate, händisch nachgebaute Region-Merge-Logik (`int[]`-Listen + eigene statische `merge()`-Methode für Dirty-/Clear-Regionen) wurde komplett entfernt und durch zwei `PieceList<Object>`-Instanzen (mit einem gemeinsamen Identity-Marker, damit gleichartige Stücke immer verschmelzen) ersetzt – dieselbe Komponente, die auch das Syntax-Highlighting benutzt.

Damit ist die Aggregation von Änderungsregionen an genau einer Stelle (`PieceList`) implementiert und wiederverwendbar; der `EditorUpdateManager` bleibt die einzige Komponente, die rohe Dokumentänderungen noch aggregiert (via Edit-Komposition), parst danach einmalig über den AST (`ast.update(...)` → `TextRegion`) und wendet darauf sequenziell Syntax-Highlighting (`invalidateTextPresentation`, direkt) und – über den entkoppelten `onAstUpdated`-Listener – Spellcheck an, das intern dieselbe `PieceList`-Aggregation nutzt.
Im SpellCheckReconciler müsste das Region-Tracking für dirty und clear jetzt überflüssig sein. Eine vom Manager übergebene Region wird entweder gecleared oder verarbeitet.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=4b8517bb-208b-4718-9a99-4ce4ecd1377e | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: b551cd19-4414-415e-9f82-c1885b71be6a
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  mode: replace
  content: "package xy.ai.workbench.editor.spellcheck;\n\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IRegion;\nimport org.eclipse.jface.text.ITextViewer;\nimport org.eclipse.jface.text.Region;\nimport org.eclipse.jface.text.reconciler.IReconciler;\nimport org.eclipse.jface.text.reconciler.IReconcilingStrategy;\n\nimport xy.ai.workbench.editor.AITextEditor;\nimport xy.ai.workbench.editor.mdast.TextRegion;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\nimport xy.ai.workbench.editor.update.EditorUpdateManager;\n\n/**\n * Reconciler that checks (or clears) exactly the region the last, already\n * debounced AST reparse touched.\n * <p>\n * All raw document events, as well as the AST reparse itself, are handled\n * centrally by the {@link EditorUpdateManager} (the editor's document/editor\n * addon), which fires {@link EditorUpdateManager.Listener#onAstUpdated(TextRegion)}\n * exactly once per settled edit batch. There is nothing left to aggregate\n * here: the region the manager hands over is either cleared (if it belongs to\n * a node with spell checking disabled) or checked - once, off the UI thread\n * via {@link EditorUpdateManager#runAsync(Runnable)} so the actual (blocking)\n * spell check never runs on the UI thread the reparse itself uses.\n */\npublic class SpellCheckReconciler implements IReconciler {\n\n\tprivate final SpellingStrategy fStrategy;\n\tprivate final AITextEditor fEditor;\n\n\tprivate EditorUpdateManager.Listener fListener;\n\n\tpublic SpellCheckReconciler(SpellingStrategy strategy, AITextEditor editor) {\n\t\tfStrategy = strategy;\n\t\tfEditor = editor;\n\t}\n\n\t// ── IReconciler ────────────────────────────────────────────────────────────\n\n\t@Override\n\tpublic void install(ITextViewer textViewer) {\n\t\tfListener = new EditorUpdateManager.Listener() {\n\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\tfStrategy.setDocument(newDocument);\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {\n\t\t\t\tif (region == null)\n\t\t\t\t\treturn;\n\t\t\t\tEditorUpdateManager manager = fEditor.getUpdateManager();\n\t\t\t\tmanager.runAsync(() -> apply(region.n(), new Region(region.offset(), region.length())));\n\t\t\t}\n\t\t};\n\n\t\tEditorUpdateManager manager = fEditor.getUpdateManager();\n\t\tmanager.addListener(fListener);\n\n\t\t// Defensive fallback for the (unlikely) case the document was already fully\n\t\t// parsed before this reconciler had a chance to register: trigger a full\n\t\t// check off the calling (UI) thread.\n\t\tIDocument doc = manager.getDocument();\n\t\tif (doc != null && manager.getAst() != null) {\n\t\t\tfStrategy.setDocument(doc);\n\t\t\tmanager.runAsync(() -> fStrategy.reconcile(new Region(0, doc.getLength())));\n\t\t}\n\t}\n\n\t@Override\n\tpublic void uninstall() {\n\t\tif (fListener != null) {\n\t\t\tfEditor.getUpdateManager().removeListener(fListener);\n\t\t\tfListener = null;\n\t\t}\n\t}\n\n\t@Override\n\tpublic IReconcilingStrategy getReconcilingStrategy(String contentType) {\n\t\treturn IDocument.DEFAULT_CONTENT_TYPE.equals(contentType) ? fStrategy : null;\n\t}\n\n\t/** Runs off the UI thread: clears or checks {@code region} depending on {@code node}. */\n\tprivate void apply(Node node, IRegion region) {\n\t\tif (node != null && !node.enableSpellcheck)\n\t\t\tfStrategy.clear(region);\n\t\telse\n\t\t\tfStrategy.reconcile(region);\n\t}\n}\n"
  reason: Remove now-superfluous dirty/clear region tracking; a manager-provided region is either cleared or checked as a whole
```
/answer b551cd19-4414-415e-9f82-c1885b71be6a allow
Control Request:
```yaml
id: 199bc21c-d482-4636-92b4-bce37eb8d4f0
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  script: grep -rn "PieceList\|mergeEnabledLeaves\|fDirtyRegions\|fClearRegions" xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  reason: Confirm cleanup complete
```
/answer 199bc21c-d482-4636-92b4-bce37eb8d4f0 allow
Control Request:
```yaml
id: edfc8f00-c44c-4051-899b-47e8e5c85298
result:
  structuredContent:
    exit_code: 1
    stdout: ""
```
/answer edfc8f00-c44c-4051-899b-47e8e5c85298 allow
ReasoningToken: 2696
Sauber entfernt, `PieceList` bleibt weiterhin ausschließlich in `AIRuleScanner` im Einsatz (dort weiterhin sinnvoll für die Token-Aggregation innerhalb eines Scans).

## Zusammenfassung

`SpellCheckReconciler` wurde vereinfacht:
- Kein `fDirtyRegions`/`fClearRegions`, keine `PieceList`-Nutzung, kein `mergeEnabledLeaves`-Rekursions-/Merge-Mechanismus mehr.
- `onAstUpdated(TextRegion region)` trifft jetzt genau eine Entscheidung für die vom Manager gelieferte Region: `node.enableSpellcheck == false` → `clear(region)`, sonst → `reconcile(region)` – ausgeführt off-UI-Thread via `manager.runAsync(...)`.

Das ist konsistent mit dem neuen Manager-Design: Da `onAstUpdated` bereits genau einmal pro (debounced) Reparse-Zyklus mit exakt einer, bereits durch den AST aufgelösten/expandierten Region feuert, gibt es nichts mehr zu aggregieren – die frühere Mehrfach-Region-Verwaltung war ein Relikt aus der Zeit, in der Reconcile-Aufrufe unabhängig vom AST-Update eigenständig gebündelt werden mussten.
Die fEditor Referenz in SpellCheckReconciler wird nicht benötigt wenn nur auf den EditorUpdateManager zugegriffen werden muss.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=4b8517bb-208b-4718-9a99-4ce4ecd1377e | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 166ecb7d-ee92-4547-bd61-d95e41fdf482
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckInstaller.java
  reason: Check current installer for constructor call
```
/answer 166ecb7d-ee92-4547-bd61-d95e41fdf482 allow
ReasoningToken: 389
Control Request:
```yaml
id: 60fe7cd0-14df-4ecd-ada8-364ab6d4d740
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckInstaller.java
  old_text: |2-
        public static IReconciler createReconciler(ISourceViewer sourceViewer, AITextEditor editor) {
            SpellingStrategy strategy = new SpellingStrategy(sourceViewer);
            return new SpellCheckReconciler(strategy, editor);
        }
  new_text: |2-
        public static IReconciler createReconciler(ISourceViewer sourceViewer, AITextEditor editor) {
            SpellingStrategy strategy = new SpellingStrategy(sourceViewer);
            return new SpellCheckReconciler(strategy, editor.getUpdateManager());
        }
  reason: Pass EditorUpdateManager directly instead of the whole editor
```
/answer 60fe7cd0-14df-4ecd-ada8-364ab6d4d740 allow
Text:
 Now update `SpellCheckReconciler` to hold the `EditorUpdateManager` directly instead of `AITextEditor`.
Control Request:
```yaml
id: df6e1ea1-54de-425d-9f37-e004635c8f73
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  mode: replace
  content: "package xy.ai.workbench.editor.spellcheck;\n\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IRegion;\nimport org.eclipse.jface.text.ITextViewer;\nimport org.eclipse.jface.text.Region;\nimport org.eclipse.jface.text.reconciler.IReconciler;\nimport org.eclipse.jface.text.reconciler.IReconcilingStrategy;\n\nimport xy.ai.workbench.editor.mdast.TextRegion;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\nimport xy.ai.workbench.editor.update.EditorUpdateManager;\n\n/**\n * Reconciler that checks (or clears) exactly the region the last, already\n * debounced AST reparse touched.\n * <p>\n * All raw document events, as well as the AST reparse itself, are handled\n * centrally by the {@link EditorUpdateManager} (the editor's document/editor\n * addon), which fires {@link EditorUpdateManager.Listener#onAstUpdated(TextRegion)}\n * exactly once per settled edit batch. There is nothing left to aggregate\n * here: the region the manager hands over is either cleared (if it belongs to\n * a node with spell checking disabled) or checked - once, off the UI thread\n * via {@link EditorUpdateManager#runAsync(Runnable)} so the actual (blocking)\n * spell check never runs on the UI thread the reparse itself uses.\n */\npublic class SpellCheckReconciler implements IReconciler {\n\n\tprivate final SpellingStrategy fStrategy;\n\tprivate final EditorUpdateManager fManager;\n\n\tprivate EditorUpdateManager.Listener fListener;\n\n\tpublic SpellCheckReconciler(SpellingStrategy strategy, EditorUpdateManager manager) {\n\t\tfStrategy = strategy;\n\t\tfManager = manager;\n\t}\n\n\t// ── IReconciler ────────────────────────────────────────────────────────────\n\n\t@Override\n\tpublic void install(ITextViewer textViewer) {\n\t\tfListener = new EditorUpdateManager.Listener() {\n\t\t\t@Override\n\t\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\t\tfStrategy.setDocument(newDocument);\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic void onAstUpdated(TextRegion region) {\n\t\t\t\tif (region == null)\n\t\t\t\t\treturn;\n\t\t\t\tfManager.runAsync(() -> apply(region.n(), new Region(region.offset(), region.length())));\n\t\t\t}\n\t\t};\n\n\t\tfManager.addListener(fListener);\n\n\t\t// Defensive fallback for the (unlikely) case the document was already fully\n\t\t// parsed before this reconciler had a chance to register: trigger a full\n\t\t// check off the calling (UI) thread.\n\t\tIDocument doc = fManager.getDocument();\n\t\tif (doc != null && fManager.getAst() != null) {\n\t\t\tfStrategy.setDocument(doc);\n\t\t\tfManager.runAsync(() -> fStrategy.reconcile(new Region(0, doc.getLength())));\n\t\t}\n\t}\n\n\t@Override\n\tpublic void uninstall() {\n\t\tif (fListener != null) {\n\t\t\tfManager.removeListener(fListener);\n\t\t\tfListener = null;\n\t\t}\n\t}\n\n\t@Override\n\tpublic IReconcilingStrategy getReconcilingStrategy(String contentType) {\n\t\treturn IDocument.DEFAULT_CONTENT_TYPE.equals(contentType) ? fStrategy : null;\n\t}\n\n\t/** Runs off the UI thread: clears or checks {@code region} depending on {@code node}. */\n\tprivate void apply(Node node, IRegion region) {\n\t\tif (node != null && !node.enableSpellcheck)\n\t\t\tfStrategy.clear(region);\n\t\telse\n\t\t\tfStrategy.reconcile(region);\n\t}\n}\n"
  reason: Store EditorUpdateManager directly instead of unused AITextEditor reference
```
/answer df6e1ea1-54de-425d-9f37-e004635c8f73 allow
Control Request:
```yaml
id: e5b664cb-da29-49bc-9dab-da4e53076ff2
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  script: grep -rn "AITextEditor\|fEditor" xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java; echo ---; grep -n "SpellCheckReconciler(" xy/ai/workbench/editor/spellcheck/SpellCheckInstaller.java
  reason: Verify no leftover AITextEditor reference and installer call matches new constructor
```
/answer e5b664cb-da29-49bc-9dab-da4e53076ff2 allow
Control Request:
```yaml
id: c536152f-edc3-4a2e-8d4a-55468b7b3a90
result:
  structuredContent:
    stdout: |
      ---
      18:        return new SpellCheckReconciler(strategy, editor.getUpdateManager());
```
/answer c536152f-edc3-4a2e-8d4a-55468b7b3a90 allow
Text:
 Kurzer Konsistenz-Check:
Passt – `SpellCheckReconciler` kennt jetzt nur noch den `EditorUpdateManager`, nicht mehr `AITextEditor`. `SpellCheckInstaller` reicht entsprechend `editor.getUpdateManager()` durch.
Der AIRuleScanner braucht dann aber auch keine eigene Piece-verwaltung mehr.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=4b8517bb-208b-4718-9a99-4ce4ecd1377e | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 2e2b1855-f152-42cd-9036-65621fae66c1
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  script: grep -rln "PieceList" .
  reason: Check remaining PieceList consumers before removal
```
/answer 2e2b1855-f152-42cd-9036-65621fae66c1 allow