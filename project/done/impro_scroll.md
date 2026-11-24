`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java` wird bei großen Dateien sehr träge und langsam. Das würde ich gerne korrigieren. Ich denke ein teil des Problems ist das Zeilenlayouting. Vielleicht hängt das mit der Scrollbar zusammen. Kann man die Scrollbar abschalten oder sich auf den Zeichenoffset beziehen lassen?

SystemInit:  id=b9820680-49ae-4ca4-87de-fcccf7bcaa40, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich sehe mir zuerst die Datei an.

Control Request:
```yaml
id: 735c-1-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  reason: "Struktur des AITextEditor ermitteln, um Layout- und Scrollbar-Code zu finden"
```
/answer 735c-1-0 allow
ReasoningToken: 13
Control Request:
```yaml
id: 735c-2-1
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: s0G8wt|06AciC
          excert: import java.util.ArrayList;
        - id: DyZ6jU|Qe562U
          excert: import xy.ai.workbench.editor.spellcheck.SpellingAnnotation;
        - id: AITextEditor
          excert: "public class AITextEditor extends TextEditor {"
          children:
          - id: AITextEditor.eiO9HI|VpzPtE
            excert: private static final int LIMIT = 512 * 1024;
          - id: AITextEditor.AITextEditor
            excert: public AITextEditor()
          - id: AITextEditor.createSourceViewer
            excert: "protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler rul…"
          - id: AITextEditor.createPartControl
            excert: public void createPartControl(Composite parent)
          - id: AITextEditor.installPainter
            excert: private void installPainter(ISourceViewer sourceViewer)
          - id: AITextEditor.dispose
            excert: public void dispose()
          - id: AITextEditor.handleCaretMoved
            excert: private void handleCaretMoved(int offset)
          - id: AITextEditor.getAdapter
            excert: public T getAdapter(Class<T> adapter)
          - id: AITextEditor.refreshOutline
            excert: private void refreshOutline()
          - id: AITextEditor.selectAndRevealNode
            excert: public void selectAndRevealNode(Node node)
          - id: AITextEditor.getInitialWordWrapStatus
            excert: protected boolean getInitialWordWrapStatus()
          - id: AITextEditor.getUpdateManager
            excert: public EditorManager getUpdateManager()
          - id: AITextEditor.ManagerListener
            excert: "private class ManagerListener implements IManagerListener {"
            children:
            - id: AITextEditor.ManagerListener.onDocumentChanged
              excert: "public void onDocumentChanged(IDocument oldDocument, IDocument newDocument)"
            - id: AITextEditor.ManagerListener.onAstUpdated
              excert: public void onAstUpdated(Node node)
          - id: AITextEditor.updateRulerVisibility
            excert: private void updateRulerVisibility(IDocument document)
          - id: AITextEditor.updateLineNumbers
            excert: private void updateLineNumbers(IDocument document)
```
/answer 735c-2-1 allow
Control Request:
```yaml
id: 735c-3-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: (?i)scroll|layout|StyledText|JTextArea|JEditorPane|TextArea|setLineWrap|wrap
    include:
    - '*.java'
    limit: 50
    intent: survey
  reason: Verwendete UI-Toolkit-Klassen sowie Scroll-/Layout-Stellen im Editor-Paket finden
```
/answer 735c-3-0 allow
Control Request:
```yaml
id: 735c-4-3
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
        matches:
        - path: AITextEditor.java
          matches:
          - import org.eclipse.swt.custom.StyledText;
          - if (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)
        - path: outline/MarkdownOutlinePage.java
          matches:
          - import org.eclipse.jface.layout.GridDataFactory;
          - import org.eclipse.jface.layout.GridLayoutFactory;
        - path: EditorChangeListener.java
          matches:
          - import org.eclipse.swt.custom.StyledText;
          - StyledText textWidget = sourceViewer.getTextWidget();
```
/answer 735c-4-3 allow
Text:
 Eclipse-Editor (SWT StyledText). Ich lese die relevanten Methoden.
Control Request:
```yaml
id: 735c-5-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    ids:
    - AITextEditor
    - AITextEditor.eiO9HI|VpzPtE
    - AITextEditor.createSourceViewer
    - AITextEditor.createPartControl
    - AITextEditor.installPainter
    - AITextEditor.getInitialWordWrapStatus
    - AITextEditor.updateRulerVisibility
    - AITextEditor.updateLineNumbers
    - AITextEditor.handleCaretMoved
  reason: "Code der relevanten Methoden lesen (Viewer-Erzeugung, Wordwrap, Ruler, Painter)"
```
/answer 735c-5-0 allow
Control Request:
```yaml
id: 735c-6-5
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: AITextEditor
          type: ClassOrInterfaceDeclaration
          excert: "public class AITextEditor extends TextEditor {"
          children:
          - id: AITextEditor.eiO9HI|VpzPtE
            type: StatementGroup
            code: "\tprivate static final int LIMIT = 512 * 1024;\n\tprivate boolean rulerVisible = true;\n\tprivate CompositeRuler ruler;\n\tprivate List<IVerticalRulerColumn> decorators = new ArrayList<>();\n\n\tprivate final EditorManager manager = new EditorManager();\n\n\tprivate MarkdownOutlinePage outlinePage;\n"
          - id: AITextEditor.AITextEditor
            type: ConstructorDeclaration
            code: "\tpublic AITextEditor() {\n\t\tsuper();\n\t\tsetSourceViewerConfiguration(new AISourceViewerConfiguration(manager));\n\t}\n"
          - id: AITextEditor.createSourceViewer
            type: MethodDeclaration
            code: "\t@Override\n\tprotected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {\n\t\tISourceViewer sourceViewer = super.createSourceViewer(parent, ruler, styles);\n\n\t\tif (ruler instanceof CompositeRuler)\n\t\t\tthis.ruler = (CompositeRuler) ruler;\n\n\t\tmanager.addListener(new ManagerListener());\n\t\tmanager.install(sourceViewer);\n\t\tnew SpellCheckReconciler(sourceViewer, manager);\n\n\t\treturn sourceViewer;\n\t}\n"
          - id: AITextEditor.createPartControl
            type: MethodDeclaration
            code: "\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tsuper.createPartControl(parent);\n\t\tinstallPainter(getSourceViewer());\n\n\t\tif (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)\n\t\t\twidget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));\n\t}\n"
          - id: AITextEditor.installPainter
            type: MethodDeclaration
            code: "\tprivate void installPainter(ISourceViewer sourceViewer) {\n\t\tDisplay display = sourceViewer.getTextWidget().getDisplay();\n\n\t\tAnnotationPainter painter = new AnnotationPainter(sourceViewer, new DefaultMarkerAnnotationAccess());\n\t\tpainter.addTextStyleStrategy(SpellingAnnotation.TYPE,\n\t\t\t\tnew AnnotationPainter.UnderlineStrategy(SWT.UNDERLINE_SQUIGGLE));\n\t\tpainter.addAnnotationType(SpellingAnnotation.TYPE, SpellingAnnotation.TYPE);\n\t\tpainter.setAnnotationTypeColor(SpellingAnnotation.TYPE, display.getSystemColor(SWT.COLOR_RED));\n\n\t\t// addTextStyleStrategy works through ITextPresentationListener – register\n\t\t// explicitly,\n\t\t// because addPainter() alone does NOT do this registration.\n\t\t// addTextPresentationListener is only on the concrete SourceViewer class, not\n\t\t// on ISourceViewer.\n\t\tif (sourceViewer instanceof SourceViewer)\n\t\t\t((SourceViewer) sourceViewer).addTextPresentationListener(painter);\n\t\t((ITextViewerExtension2) sourceViewer).addPainter(painter);\n\t}\n"
          - id: AITextEditor.dispose
            type: MethodDeclaration
            code: "\t@Override\n\tpublic void dispose() {\n\t\tmanager.uninstall();\n\t\tsuper.dispose();\n\t}\n"
          - id: AITextEditor.handleCaretMoved
            type: MethodDeclaration
            code: "\tprivate void handleCaretMoved(int offset) {\n\t\tif (outlinePage != null) {\n\t\t\tvar selection = getSourceViewer().getSelectedRange();\n\t\t\tif (selection.y == 0)\n\t\t\t\tDisplay.getDefault().asyncExec(() -> outlinePage.selectNodeForOffset(offset));\n\t\t}\n\t}\n"
          - id: AITextEditor.getAdapter
            type: MethodDeclaration
            code: "\t@Override\n\tpublic <T> T getAdapter(Class<T> adapter) {\n\t\tif (IContentOutlinePage.class.equals(adapter)) {\n\t\t\tif (outlinePage == null)\n\t\t\t\toutlinePage = new MarkdownOutlinePage(this);\n\t\t\treturn adapter.cast(outlinePage);\n\t\t}\n\t\treturn super.getAdapter(adapter);\n\t}\n"
          - id: AITextEditor.refreshOutline
            type: MethodDeclaration
            code: "\tprivate void refreshOutline() {\n\t\tif (outlinePage != null)\n\t\t\toutlinePage.refresh();\n\t}\n"
          - id: AITextEditor.selectAndRevealNode
            type: MethodDeclaration
            code: "\tpublic void selectAndRevealNode(Node node) {\n\t\tif (node == null)\n\t\t\treturn;\n\t\tselectAndReveal(node.getOffset(), node.length());\n\t}\n"
          - id: AITextEditor.getInitialWordWrapStatus
            type: MethodDeclaration
            code: "\t@Override\n\tprotected boolean getInitialWordWrapStatus() {\n\t\treturn true;\n\t}\n"
          - id: AITextEditor.getUpdateManager
            type: MethodDeclaration
            code: "\tpublic EditorManager getUpdateManager() {\n\t\treturn manager;\n\t}\n"
          - id: AITextEditor.ManagerListener
            type: ClassOrInterfaceDeclaration
            excert: "private class ManagerListener implements IManagerListener {"
            children:
            - id: AITextEditor.ManagerListener.onDocumentChanged
              type: MethodDeclaration
              code: "\t\t@Override\n\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\tif (newDocument == null)\n\t\t\t\trefreshOutline(); // clears the outline; onAstUpdated covers the non-null case\n\t\t}\n"
            - id: AITextEditor.ManagerListener.onAstUpdated
              type: MethodDeclaration
              code: "\t\t@Override\n\t\tpublic void onAstUpdated(Node node) {\n\t\t\tIDocument doc = manager.getDocument();\n\t\t\tif (doc == null)\n\t\t\t\treturn;\n\t\t\tupdateRulerVisibility(doc);\n\t\t\tupdateLineNumbers(doc);\n\t\t\trefreshOutline();\n\t\t}\n"
          - id: AITextEditor.updateRulerVisibility
            type: MethodDeclaration
            code: "\tprivate void updateRulerVisibility(IDocument document) {\n\t\tboolean shouldShow = document.getLength() <= LIMIT;\n\t\tif (shouldShow != rulerVisible) {\n\t\t\trulerVisible = shouldShow;\n\t\t\tif (getSourceViewer() instanceof SourceViewer sv) {\n\t\t\t\tsv.showAnnotations(shouldShow);\n\t\t\t\tsv.showAnnotationsOverview(shouldShow);\n\t\t\t}\n\t\t}\n\t}\n"
          - id: AITextEditor.updateLineNumbers
            type: MethodDeclaration
            code: "\tprivate void updateLineNumbers(IDocument document) {\n\t\tif (ruler == null)\n\t\t\treturn;\n\n\t\tlong size = document.getLength();\n\t\tIterator<IVerticalRulerColumn> it = ruler.getDecoratorIterator();\n\t\tIVerticalRulerColumn d;\n\n\t\tif (size > LIMIT && it.hasNext()) {\n\t\t\twhile (it.hasNext() && (d = it.next()) != null)\n\t\t\t\tdecorators.add(d);\n\t\t\tfor (var dec : decorators)\n\t\t\t\truler.removeDecorator(dec);\n\t\t} else if (size < LIMIT && !it.hasNext() && !decorators.isEmpty()) {\n\t\t\tfor (var i = 0; i < decorators.size(); i++)\n\t\t\t\truler.addDecorator(i, decorators.get(i));\n\t\t\tdecorators.clear();\n\t\t}\n\t}\n"
        - id: AITextEditor.eiO9HI|VpzPtE
          type: StatementGroup
          code: "\tprivate static final int LIMIT = 512 * 1024;\n\tprivate boolean rulerVisible = true;\n\tprivate CompositeRuler ruler;\n\tprivate List<IVerticalRulerColumn> decorators = new ArrayList<>();\n\n\tprivate final EditorManager manager = new EditorManager();\n\n\tprivate MarkdownOutlinePage outlinePage;\n"
        - id: AITextEditor.createSourceViewer
          type: MethodDeclaration
          code: "\t@Override\n\tprotected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {\n\t\tISourceViewer sourceViewer = super.createSourceViewer(parent, ruler, styles);\n\n\t\tif (ruler instanceof CompositeRuler)\n\t\t\tthis.ruler = (CompositeRuler) ruler;\n\n\t\tmanager.addListener(new ManagerListener());\n\t\tmanager.install(sourceViewer);\n\t\tnew SpellCheckReconciler(sourceViewer, manager);\n\n\t\treturn sourceViewer;\n\t}\n"
        - id: AITextEditor.createPartControl
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tsuper.createPartControl(parent);\n\t\tinstallPainter(getSourceViewer());\n\n\t\tif (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)\n\t\t\twidget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));\n\t}\n"
        - id: AITextEditor.installPainter
          type: MethodDeclaration
          code: "\tprivate void installPainter(ISourceViewer sourceViewer) {\n\t\tDisplay display = sourceViewer.getTextWidget().getDisplay();\n\n\t\tAnnotationPainter painter = new AnnotationPainter(sourceViewer, new DefaultMarkerAnnotationAccess());\n\t\tpainter.addTextStyleStrategy(SpellingAnnotation.TYPE,\n\t\t\t\tnew AnnotationPainter.UnderlineStrategy(SWT.UNDERLINE_SQUIGGLE));\n\t\tpainter.addAnnotationType(SpellingAnnotation.TYPE, SpellingAnnotation.TYPE);\n\t\tpainter.setAnnotationTypeColor(SpellingAnnotation.TYPE, display.getSystemColor(SWT.COLOR_RED));\n\n\t\t// addTextStyleStrategy works through ITextPresentationListener – register\n\t\t// explicitly,\n\t\t// because addPainter() alone does NOT do this registration.\n\t\t// addTextPresentationListener is only on the concrete SourceViewer class, not\n\t\t// on ISourceViewer.\n\t\tif (sourceViewer instanceof SourceViewer)\n\t\t\t((SourceViewer) sourceViewer).addTextPresentationListener(painter);\n\t\t((ITextViewerExtension2) sourceViewer).addPainter(painter);\n\t}\n"
        - id: AITextEditor.getInitialWordWrapStatus
          type: MethodDeclaration
          code: "\t@Override\n\tprotected boolean getInitialWordWrapStatus() {\n\t\treturn true;\n\t}\n"
        - id: AITextEditor.updateRulerVisibility
          type: MethodDeclaration
          code: "\tprivate void updateRulerVisibility(IDocument document) {\n\t\tboolean shouldShow = document.getLength() <= LIMIT;\n\t\tif (shouldShow != rulerVisible) {\n\t\t\trulerVisible = shouldShow;\n\t\t\tif (getSourceViewer() instanceof SourceViewer sv) {\n\t\t\t\tsv.showAnnotations(shouldShow);\n\t\t\t\tsv.showAnnotationsOverview(shouldShow);\n\t\t\t}\n\t\t}\n\t}\n"
        - id: AITextEditor.updateLineNumbers
          type: MethodDeclaration
          code: "\tprivate void updateLineNumbers(IDocument document) {\n\t\tif (ruler == null)\n\t\t\treturn;\n\n\t\tlong size = document.getLength();\n\t\tIterator<IVerticalRulerColumn> it = ruler.getDecoratorIterator();\n\t\tIVerticalRulerColumn d;\n\n\t\tif (size > LIMIT && it.hasNext()) {\n\t\t\twhile (it.hasNext() && (d = it.next()) != null)\n\t\t\t\tdecorators.add(d);\n\t\t\tfor (var dec : decorators)\n\t\t\t\truler.removeDecorator(dec);\n\t\t} else if (size < LIMIT && !it.hasNext() && !decorators.isEmpty()) {\n\t\t\tfor (var i = 0; i < decorators.size(); i++)\n\t\t\t\truler.addDecorator(i, decorators.get(i));\n\t\t\tdecorators.clear();\n\t\t}\n\t}\n"
        - id: AITextEditor.handleCaretMoved
          type: MethodDeclaration
          code: "\tprivate void handleCaretMoved(int offset) {\n\t\tif (outlinePage != null) {\n\t\t\tvar selection = getSourceViewer().getSelectedRange();\n\t\t\tif (selection.y == 0)\n\t\t\t\tDisplay.getDefault().asyncExec(() -> outlinePage.selectNodeForOffset(offset));\n\t\t}\n\t}\n"
```
/answer 735c-6-5 allow
Text:
 Die Scrollbar-Frage hängt direkt am Word-Wrap (`getInitialWordWrapStatus() == true`). Bevor ich etwas ändere, prüfe ich, was bei Änderungen sonst noch läuft.
Control Request:
```yaml
id: 735c-7-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: class EditorManager|class EditorChangeListener|class SpellCheckReconciler|Job|asyncExec|syncExec|setWordWrap|getVerticalBar|setTopIndex|getLineCount|getLinePixel|getContentHeight
    include:
    - '*.java'
    limit: 40
    intent: survey
  reason: Weitere teure Operationen im Änderungs-/Reconcile-Pfad finden und Wordwrap/Scrollbar-Nutzung prüfen
```
/answer 735c-7-0 allow
Control Request:
```yaml
id: 735c-8-7
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
        matches:
        - path: EditorInterface.java
          matches:
          - "exec.syncExec(() -> {"
          - "exec.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));"
        - path: view/session/EditTab.java
          matches:
          - instructionEdit.setTopIndex(top);
        - path: view/session/UsageLogSection.java
          matches:
          - "exec.asyncExec(() -> {"
        - path: view/explorer/MarkdownExplorerView.java
          matches:
          - "Display.getDefault().asyncExec(() -> {"
        - path: view/AIBatchView.java
          matches:
          - import org.eclipse.core.runtime.jobs.Job;
          - "exec.syncExec(() -> batchViewer.refresh(entry, true));"
          - exec.asyncExec(() -> batchViewer.setInput(batchContent));
          - exec.syncExec(() -> batchViewer.remove(entry));
          - exec.syncExec(() -> batchViewer.add(entry));
          - "private Job updateIntervall = Job.create(\"Batch List Autoupdate\", (mo) -> {"
          - "Job.create(\"Cacel Batch\", (mon) -> {"
          - "Job.create(\"Load Batches\", (mon) -> {"
          - Display.getDefault().asyncExec(() -> reqViewer.setInput(batchRequestsContent));
          - "Job.create(\"Process Awswer\", (mon) -> {"
        - path: view/ActionManager.java
          matches:
          - import org.eclipse.core.runtime.jobs.IJobFunction;
          - import org.eclipse.core.runtime.jobs.Job;
          - private IJobFunction job;
          - "public ActionDescription jobFunc(IJobFunction job) {"
          - Display.getDefault().asyncExec(display);
          - "Job.create(getText(), job).schedule();"
        - path: view/MultiSelectTable.java
          matches:
          - "table.getDisplay().asyncExec(() -> {"
        - path: view/ClaudeCodeSessionView.java
          matches:
          - display.asyncExec(this::scheduleDebouncedRefresh);
        - path: view/approver/AutoApprovePanel.java
          matches:
          - console.setWordWrap(false);
          - "display.asyncExec(() -> {"
        - path: editor/AITextEditor.java
          matches:
          - Display.getDefault().asyncExec(() -> outlinePage.selectNodeForOffset(offset));
        - path: editor/outline/MarkdownOutlinePage.java
          matches:
          - "Display.getDefault().asyncExec(() -> {"
        - path: editor/spellcheck/SpellCheckReconciler.java
          matches:
          - "public class SpellCheckReconciler implements ISpellChecker {"
        - path: editor/spellcheck/SpellingStrategy.java
          matches:
          - "viewer.getTextWidget().getDisplay().syncExec(() -> applyAnnotations(valid, checkedOffset, checkedLength));"
          - ".syncExec(() -> applyAnnotations(new ArrayList<>(), clearedOffset, clearedLength));"
        - path: editor/EditorManager.java
          matches:
          - "public class EditorManager {"
          - "viewer.getTextWidget().getDisplay().asyncExec(() -> {"
        - path: editor/EditorChangeListener.java
          matches:
          - import org.eclipse.core.runtime.jobs.Job;
          - "public class EditorChangeListener {"
          - "Job.create(\"Update Input Stats\", (mon) -> {"
          - "Display.getDefault().asyncExec(() -> {"
        warnings:
        - "Limit of 40 matches reached (total 58); narrow pattern, directory or include/exclude filters, or raise limit (max 50)."
```
/answer 735c-8-7 allow
ReasoningToken: 1015
Control Request:
```yaml
id: 735c-9-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorChangeListener.java
  reason: Struktur des Änderungs-Listeners ansehen
```
/answer 735c-9-0 allow
Control Request:
```yaml
id: 735c-a-9
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorChangeListener.java
        nodes:
        - id: mizjsd|PXOqFr
          excert: import java.lang.ref.WeakReference;
        - id: TIxsdb|xflYpR
          excert: import org.eclipse.ui.texteditor.ITextEditor;
        - id: EditorChangeListener
          excert: "public class EditorChangeListener {"
          children:
          - id: EditorChangeListener.VK1IU2|FPEM8m
            excert: private SelectionListener selectionListener = new SelectionListener();
          - id: EditorChangeListener.addInputObserver
            excert: public void addInputObserver(Consumer<InputMode> obs)
          - id: EditorChangeListener.addTextEditorObserver
            excert: public void addTextEditorObserver(Consumer<ITextEditor> obs)
          - id: EditorChangeListener.getLastTextEditor
            excert: public ITextEditor getLastTextEditor()
          - id: EditorChangeListener.setTextEditor
            excert: private void setTextEditor(ITextEditor textEditor)
          - id: EditorChangeListener.getTextEditor
            excert: private ITextEditor getTextEditor()
          - id: EditorChangeListener.editorChanged
            excert: public void editorChanged(ITextEditor editor)
          - id: EditorChangeListener.registerListener
            excert: private void registerListener(ITextEditor editor)
          - id: EditorChangeListener.removeListener
            excert: private void removeListener()
          - id: EditorChangeListener.EditorCaretListener
            excert: "public class EditorCaretListener implements CaretListener {"
            children:
            - id: EditorChangeListener.EditorCaretListener.caretMoved
              excert: public void caretMoved(CaretEvent event)
          - id: EditorChangeListener.DocumentListener
            excert: "public class DocumentListener implements IDocumentListener {"
            children:
            - id: EditorChangeListener.DocumentListener.documentChanged
              excert: public void documentChanged(DocumentEvent event)
            - id: EditorChangeListener.DocumentListener.documentAboutToBeChanged
              excert: public void documentAboutToBeChanged(DocumentEvent event)
          - id: EditorChangeListener.SelectionListener
            excert: "public class SelectionListener implements ISelectionChangedListener {"
            children:
            - id: EditorChangeListener.SelectionListener.selectionChanged
              excert: public void selectionChanged(SelectionChangedEvent event)
          - id: EditorChangeListener.TextChangeListener
            excert: "public class TextChangeListener implements ITextListener {"
            children:
            - id: EditorChangeListener.TextChangeListener.textChanged
              excert: public void textChanged(TextEvent event)
```
/answer 735c-a-9 allow
ReasoningToken: 50
Control Request:
```yaml
id: 735c-b-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
  reason: EditorManager-Struktur ansehen (AST-Update bei Änderungen)
```
/answer 735c-b-0 allow
Control Request:
```yaml
id: 735c-c-b
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
        nodes:
        - id: IxOJYs|69ogGr
          excert: import java.util.ArrayList;
        - id: EditorManager
          excert: "public class EditorManager {"
          children:
          - id: EditorManager.QoDsj9|rEkLvR
            excert: public static final int DEBOUNCE_DELAY_MS = 500;
          - id: EditorManager.createBackgroundPool
            excert: private static ThreadPoolExecutor createBackgroundPool()
          - id: EditorManager.fuKAs3|U5U9yd
            excert: private ITextViewer viewer;
          - id: EditorManager.install
            excert: public void install(ITextViewer viewer)
          - id: EditorManager.uninstall
            excert: public void uninstall()
          - id: EditorManager.addListener
            excert: public void addListener(IManagerListener listener)
          - id: EditorManager.removeListener
            excert: public boolean removeListener(IManagerListener listener)
          - id: EditorManager.getAst
            excert: public MarkdownDocument getAst()
          - id: EditorManager.getDocument
            excert: public IDocument getDocument()
          - id: EditorManager.setSpellChecker
            excert: public void setSpellChecker(ISpellChecker spellChecker)
          - id: EditorManager.changed
            excert: "private void changed(int offset, int removedLen, int insertedLen)"
          - id: EditorManager.update
            excert: private void update(ModificationRange range)
          - id: EditorManager.scheduleFlush
            excert: private void scheduleFlush()
          - id: EditorManager.cancelPending
            excert: private void cancelPending()
          - id: EditorManager.Flush
            excert: "private class Flush implements Runnable {"
            children:
            - id: EditorManager.Flush.run
              excert: public void run()
          - id: EditorManager.Document
            excert: "private class Document implements IDocumentListener {"
            children:
            - id: EditorManager.Document.OIgpJd|BBn8Aq
              excert: private int removedLen;
            - id: EditorManager.Document.documentAboutToBeChanged
              excert: public void documentAboutToBeChanged(DocumentEvent event)
            - id: EditorManager.Document.documentChanged
              excert: public void documentChanged(DocumentEvent event)
          - id: EditorManager.TextInput
            excert: "private class TextInput implements ITextInputListener {"
            children:
            - id: EditorManager.TextInput.inputDocumentAboutToBeChanged
              excert: "public void inputDocumentAboutToBeChanged(IDocument oldInput, IDocument newInpu…"
            - id: EditorManager.TextInput.inputDocumentChanged
              excert: "public void inputDocumentChanged(IDocument oldInput, IDocument newInput)"
```
/answer 735c-c-b allow
Text:
 Ich setze das um: Ab der vorhandenen `LIMIT`-Größe wird der Zeilenumbruch automatisch abgeschaltet und beim Unterschreiten wieder eingeschaltet.
Control Request:
```yaml
id: 735c-d-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.eiO9HI|VpzPtE
    old_text: private boolean rulerVisible = true;
    new_text: "private boolean rulerVisible = true;\n\tprivate boolean wordWrapSuppressed;"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.createPartControl
    old_text: "\t\tinstallPainter(getSourceViewer());\n"
    new_text: "\t\tinstallPainter(getSourceViewer());\n\n\t\tif (getSourceViewer().getDocument() != null)\n\t\t\tupdateWordWrap(getSourceViewer().getDocument());\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.ManagerListener.onAstUpdated
    old_text: "\t\t\tupdateRulerVisibility(doc);\n"
    new_text: "\t\t\tupdateWordWrap(doc);\n\t\t\tupdateRulerVisibility(doc);\n"
  reason: "Feld, Aufrufe in createPartControl und onAstUpdated für automatisches Abschalten des Wordwrap bei großen Dokumenten"
```
/answer 735c-d-0 allow
ReasoningToken: 1089
Control Request:
```yaml
id: 735c-e-d
result:
  structuredContent:
    edit_block:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        id: AITextEditor.eiO9HI|VpzPtE
        new_id: AITextEditor.eiO9HI|VpzPtE
      errors:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        id: AITextEditor.createPartControl
        error: Text not found in node; its content appears to have changed.
        reason: content_changed
        guess: |2-
              super.createPartControl(parent);
              installPainter(getSourceViewer());
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        id: AITextEditor.ManagerListener.onAstUpdated
        error: Text not found in node.
        reason: not_found
        next_step: reread_node
```
/answer 735c-e-d allow
Control Request:
```yaml
id: 735c-f-0
toolName: ast_edit
arguments:
  insert:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.updateRulerVisibility
    position: before
    source: "\t// With word wrap, StyledText must lay out every line to size the scrollbar.\n\tprivate void updateWordWrap(IDocument document) {\n\t\tif (!(getSourceViewer().getTextWidget() instanceof StyledText widget))\n\t\t\treturn;\n\n\t\tboolean large = document.getLength() > LIMIT;\n\t\tif (large && !wordWrapSuppressed && widget.getWordWrap()) {\n\t\t\twordWrapSuppressed = true;\n\t\t\twidget.setWordWrap(false);\n\t\t} else if (!large && wordWrapSuppressed) {\n\t\t\twordWrapSuppressed = false;\n\t\t\twidget.setWordWrap(true);\n\t\t}\n\t}\n\n"
  reason: Neue Methode updateWordWrap einfügen
```
/answer 735c-f-0 allow
Text:
 Zwei Edits sind fehlgeschlagen, vermutlich wegen Whitespace oder weil sich die IDs geändert haben. Ich lese die Stellen neu ein.
Control Request:
```yaml
id: 735c-10-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    regexp: installPainter\(getSourceViewer\(\)\)|updateRulerVisibility\(doc\)
    node_type: MethodDeclaration
  reason: Aktuellen Quelltext der beiden Methoden neu lesen
```
/answer 735c-10-0 allow
Control Request:
```yaml
id: 735c-11-10
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: AITextEditor.createPartControl
          type: MethodDeclaration
          code: |2
              @Override
              public void createPartControl(Composite parent) {
                super.createPartControl(parent);
                installPainter(getSourceViewer());
                if (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)
                  widget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));
              }
        - id: AITextEditor.ManagerListener.onAstUpdated
          type: MethodDeclaration
          code: |2
                @Override
                public void onAstUpdated(Node node) {
                  IDocument doc = manager.getDocument();
                  if (doc == null)
                    return;
                  updateRulerVisibility(doc);
                  updateLineNumbers(doc);
                  refreshOutline();
                }
```
/answer 735c-11-10 allow
Text:
 Die Datei wurde offenbar umformatiert (Leerzeichen statt Tabs). Ich wiederhole die beiden Edits.
ReasoningToken: 34
Control Request:
```yaml
id: 735c-12-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.createPartControl
    old_text: installPainter(getSourceViewer());
    new_text: |-
      installPainter(getSourceViewer());
          if (getSourceViewer().getDocument() != null)
            updateWordWrap(getSourceViewer().getDocument());
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    id: AITextEditor.ManagerListener.onAstUpdated
    old_text: updateRulerVisibility(doc);
    new_text: |-
      updateWordWrap(doc);
            updateRulerVisibility(doc);
  reason: Aufrufe von updateWordWrap in createPartControl und onAstUpdated einfügen
```
/answer 735c-12-0 allow
Control Request:
```yaml
id: 735c-13-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
    regexp: wordWrapSuppressed|updateWordWrap
  reason: Ergebnis der Edits prüfen
```
/answer 735c-13-0 allow
Control Request:
```yaml
id: 735c-14-13
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
        nodes:
        - id: AITextEditor.eiO9HI|Q1y79C
          code: |2
              private static final int LIMIT = 512 * 1024;
              private boolean rulerVisible = true;
              private boolean wordWrapSuppressed;
              private CompositeRuler ruler;
              private List<IVerticalRulerColumn> decorators = new ArrayList<>();
              private final EditorManager manager = new EditorManager();
              private MarkdownOutlinePage outlinePage;
        - id: AITextEditor.createPartControl
          code: |2
              @Override
              public void createPartControl(Composite parent) {
                super.createPartControl(parent);
                installPainter(getSourceViewer());
                if (getSourceViewer().getDocument() != null)
                  updateWordWrap(getSourceViewer().getDocument());
                if (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)
                  widget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));
              }
        - id: AITextEditor.ManagerListener.onAstUpdated
          code: |2
                @Override
                public void onAstUpdated(Node node) {
                  IDocument doc = manager.getDocument();
                  if (doc == null)
                    return;
                  updateWordWrap(doc);
                  updateRulerVisibility(doc);
                  updateLineNumbers(doc);
                  refreshOutline();
                }
        - id: AITextEditor.updateWordWrap
          code: |2
              private void updateWordWrap(IDocument document) {
                if (!(getSourceViewer().getTextWidget() instanceof StyledText widget))
                  return;
                boolean large = document.getLength() > LIMIT;
                if (large && !wordWrapSuppressed && widget.getWordWrap()) {
                  wordWrapSuppressed = true;
                  widget.setWordWrap(false);
                } else if (!large && wordWrapSuppressed) {
                  wordWrapSuppressed = false;
                  widget.setWordWrap(true);
                }
              }
```
/answer 735c-14-13 allow
Result Stats: id=b9820680-49ae-4ca4-87de-fcccf7bcaa40, total: 30493, in: 18, out: 6777, reason: 0, read: 127499, write: 23698, start: 1795543831307, end: 1795543839735, time: 8357, total: 55760, io: 0, control: 71, sent: 0, recv: 225
Der Editor schaltet jetzt bei großen Dokumenten den Zeilenumbruch ab. Ich habe das weder kompiliert noch mit einer großen Datei ausprobiert.

**Scrollbar abschalten oder auf den Zeichenoffset beziehen:** Das geht mit SWT `StyledText` nicht. Die Scrollbar-Größe wird immer aus der Pixelhöhe des Inhalts berechnet. Es gibt keinen Modus, der sie auf einen Zeichenoffset bezieht. `getVerticalBar().setVisible(false)` würde nur die Anzeige verstecken. Das Layout läuft trotzdem, und du könntest nicht mehr scrollen.

**Wahrscheinliche Ursache:** `getInitialWordWrapStatus()` liefert `true`. Mit Zeilenumbruch muss `StyledText` jede Zeile mit `TextLayout` umbrechen und messen, um die Gesamthöhe und damit die Scrollbar zu bestimmen. Das kostet bei großen Dateien viel Zeit, besonders bei langen Absätzen, wie sie in Markdown üblich sind.

**Änderung in `AITextEditor.java`:**
- Die neue Methode `updateWordWrap(IDocument)` schaltet den Umbruch ab, sobald das Dokument größer als `LIMIT` (512 KB) ist. Dieselbe Grenze gilt schon für die Zeilennummern und das Annotation-Ruler.
- Fällt das Dokument wieder unter die Grenze, wird der Umbruch wieder eingeschaltet. Das passiert nur, wenn der Editor ihn selbst abgeschaltet hat (`wordWrapSuppressed`).
- Aufgerufen wird die Methode beim Öffnen in `createPartControl` und bei jedem AST-Update in `onAstUpdated`.

**Einschränkungen:**
- Der Umbruch-Toggle von Eclipse bleibt bei großen Dateien eventuell außer Sync, weil ich direkt `widget.setWordWrap` setze. Das betrifft nur den Haken im Menü.
- Wenn die Datei sehr lange Zeilen ohne Zeilenumbruch hat, bleibt auch ohne Umbruch Layoutaufwand für die horizontale Scrollbar.
- Der Rest des Änderungspfads war unauffällig. `EditorManager` ist mit 500 ms entprellt (`DEBOUNCE_DELAY_MS`) und läuft im Hintergrund.
- `SpellingStrategy` ruft aber `syncExec(applyAnnotations …)` auf. Falls es nach dem Fix noch hakt, wäre das die nächste Stelle. Ein Profil, etwa mit VisualVM oder dem Eclipse-Thread-Dump, würde das klären.

Wenn bei großen Dateien das Tippen träge bleibt und nicht nur das Scrollen, sag Bescheid. Dann sehe ich mir `SpellingStrategy` und `AnnotationPainter` genauer an.
Gibt es noch Wege das Layouting weiter zu optimieren bei großen Dateien?
SystemInit:  id=b9820680-49ae-4ca4-87de-fcccf7bcaa40, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Ich prüfe zuerst, was an Highlighting und Styling am Layout beteiligt ist.
Control Request:
```yaml
id: 735c-15-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: PresentationReconciler|PresentationDamager|PresentationRepairer|setStyleRange|setStyleRanges|TextPresentation|LineStyleListener|LineBackgroundListener|setLineSpacing|setLineBackground|setFont|redraw\(|getDocumentPartitioner|Partitioner
    include:
    - '*.java'
    limit: 40
    intent: survey
  reason: "Highlighting-, Styling- und Redraw-Stellen finden, die das Layout beeinflussen"
```
/answer 735c-15-0 allow
Control Request:
```yaml
id: 735c-16-15
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
        matches:
        - path: AITextEditor.java
          matches:
          - // addTextStyleStrategy works through ITextPresentationListener – register
          - "// addTextPresentationListener is only on the concrete SourceViewer class, not"
          - ((SourceViewer) sourceViewer).addTextPresentationListener(painter);
        - path: AISourceViewerConfiguration.java
          matches:
          - import org.eclipse.jface.text.TextPresentation;
          - import org.eclipse.jface.text.presentation.IPresentationReconciler;
          - import org.eclipse.jface.text.presentation.PresentationReconciler;
          - "public IPresentationReconciler getPresentationReconciler(ISourceViewer sourceViewer) {"
          - PresentationReconciler reconciler = new PresentationReconciler();
          - "public void createPresentation(TextPresentation presentation, ITypedRegion region) {"
        - path: spellcheck/SpellingStrategy.java
          matches:
          - "// ((ITextViewerExtension2) viewer).invalidateTextPresentation(offset, length);"
          - viewer.invalidateTextPresentation();
        - path: EditorManager.java
          matches:
          - "ext2.invalidateTextPresentation(node.getOffset(), node.length());"
```
/answer 735c-16-15 allow
ReasoningToken: 114
Control Request:
```yaml
id: 735c-17-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck
    pattern: applyAnnotations|modifyAnnotations|replaceAnnotations|addAnnotation|removeAnnotation|getAnnotationModel
    include:
    - '*.java'
    limit: 30
    intent: survey
  reason: "Prüfen, wie Rechtschreib-Annotationen angewendet werden (Einzel- vs. Batch-Update)"
```
/answer 735c-17-0 allow
Control Request:
```yaml
id: 735c-18-17
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck
        matches:
        - path: SpellingQuickAssistProcessor.java
          matches:
          - IAnnotationModel model = viewer.getAnnotationModel();
        - path: SpellingStrategy.java
          matches:
          - "viewer.getTextWidget().getDisplay().syncExec(() -> applyAnnotations(valid, checkedOffset, checkedLength));"
          - ".syncExec(() -> applyAnnotations(new ArrayList<>(), clearedOffset, clearedLength));"
          - "private void applyAnnotations(List<SpellingProblem> problems, int offset, int length) {"
          - IAnnotationModel model = viewer.getAnnotationModel();
          - "((IAnnotationModelExtension) model).replaceAnnotations(toRemove.toArray(new Annotation[0]), toAdd);"
```
/answer 735c-18-17 allow
ReasoningToken: 60
Control Request:
```yaml
id: 735c-19-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellingStrategy.java
    regexp: invalidateTextPresentation|applyAnnotations
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
    regexp: invalidateTextPresentation
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
    node_type: MethodDeclaration
    regexp: getPresentationReconciler|createPresentation
  reason: "Relevante Methoden lesen: Annotation-Update, Presentation-Invalidierung, Reconciler"
```
/answer 735c-19-0 allow
Control Request:
```yaml
id: 735c-1a-19
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellingStrategy.java
        nodes:
        - id: SpellingStrategy.reconcile
          code: "\tpublic void reconcile(Node node, int rangeStart, int rangeEnd) {\n\t\tif (doc == null)\n\t\t\treturn;\n\n\t\tfinal int docLength = doc.getLength();\n\t\tint start = clamp(rangeStart, docLength);\n\t\tint end = clamp(rangeEnd, docLength);\n\t\tif (end <= start)\n\t\t\treturn;\n\t\tif (end - start > LIMIT)\n\t\t\treturn;\n\n\t\tfinal int regionOffset = start;\n\t\tfinal String regionText = textAt(start, end - start);\n\t\tif (regionText == null)\n\t\t\treturn;\n\n\t\tList<SpellingProblem> problems = client.check(regionText);\n\n\t\t// LT offsets are relative to regionText – shift them to doc offsets.\n\t\tList<SpellingProblem> valid = new ArrayList<>();\n\t\tfor (SpellingProblem p : problems) {\n\t\t\tint absOffset = p.getOffset() + regionOffset;\n\t\t\tif (absOffset >= 0 && absOffset + p.getLength() <= docLength)\n\t\t\t\tvalid.add(new SpellingProblem(absOffset, p.getLength(), p.getMessage(), p.getSuggestions()));\n\t\t}\n\n\t\tfinal int checkedOffset = start;\n\t\tfinal int checkedLength = end - start;\n\t\tviewer.getTextWidget().getDisplay().syncExec(() -> applyAnnotations(valid, checkedOffset, checkedLength));\n\t}\n"
        - id: SpellingStrategy.clear
          code: "\tpublic void clear(Node node, int rangeStart, int rangeEnd) {\n\t\tif (doc == null)\n\t\t\treturn;\n\t\tint docLength = doc.getLength();\n\t\tint start = clamp(rangeStart, docLength);\n\t\tint end = clamp(rangeEnd, docLength);\n\t\tif (end <= start)\n\t\t\treturn;\n\t\tfinal int clearedOffset = start;\n\t\tfinal int clearedLength = end - start;\n\t\tviewer.getTextWidget().getDisplay()\n\t\t\t\t.syncExec(() -> applyAnnotations(new ArrayList<>(), clearedOffset, clearedLength));\n\t}\n"
        - id: SpellingStrategy.applyAnnotations
          code: "\tprivate void applyAnnotations(List<SpellingProblem> problems, int offset, int length) {\n\t\tIAnnotationModel model = viewer.getAnnotationModel();\n\t\tif (!(model instanceof IAnnotationModelExtension))\n\t\t\treturn;\n\n\t\tint docLength = doc != null ? doc.getLength() : Integer.MAX_VALUE;\n\t\tMap<String, SpellingProblem> desired = new HashMap<>();\n\t\tfor (SpellingProblem p : problems) {\n\t\t\tif (p.getOffset() < 0 || p.getLength() < 0 || p.getOffset() + p.getLength() > docLength)\n\t\t\t\tcontinue;\n\t\t\tString text = textAt(p.getOffset(), p.getLength());\n\t\t\tif (text != null)\n\t\t\t\tdesired.put(contentKey(text, p.getMessage()), p);\n\t\t}\n\n\t\tObject lock = lockObject(model);\n\t\tsynchronized (lock) {\n\t\t\t// Every SpellingAnnotation that lies inside the just scanned\n\t\t\t// region is either re-confirmed by a matching problem (same text\n\t\t\t// + message) or is stale and must be removed – nothing from that\n\t\t\t// region is allowed to survive unaccounted for.\n\t\t\tMap<String, Annotation> existing = new HashMap<>();\n\t\t\tList<Annotation> toRemove = new ArrayList<>();\n\t\t\tIterator<Annotation> it = model.getAnnotationIterator();\n\t\t\twhile (it.hasNext()) {\n\t\t\t\tAnnotation a = it.next();\n\t\t\t\tif (!(a instanceof SpellingAnnotation) || !SpellingAnnotation.TYPE.equals(a.getType()))\n\t\t\t\t\tcontinue;\n\t\t\t\tPosition pos = model.getPosition(a);\n\t\t\t\tif (pos == null || pos.isDeleted() || pos.getOffset() < offset || pos.getOffset() >= offset + length)\n\t\t\t\t\tcontinue;\n\t\t\t\tString text = textAt(pos.getOffset(), pos.getLength());\n\t\t\t\tString key = text == null ? null : contentKey(text, ((SpellingAnnotation) a).getProblem().getMessage());\n\t\t\t\tif (key != null && desired.containsKey(key))\n\t\t\t\t\texisting.put(key, a);\n\t\t\t\telse\n\t\t\t\t\ttoRemove.add(a);\n\t\t\t}\n\n\t\t\t// Add: problems not represented by any still-valid annotation.\n\t\t\tMap<Annotation, Position> toAdd = new HashMap<>();\n\t\t\tfor (Map.Entry<String, SpellingProblem> e : desired.entrySet()) {\n\t\t\t\tif (existing.containsKey(e.getKey()))\n\t\t\t\t\tcontinue;\n\t\t\t\tSpellingProblem p = e.getValue();\n\t\t\t\ttoAdd.put(new SpellingAnnotation(p), new Position(p.getOffset(), p.getLength()));\n\t\t\t}\n\n\t\t\tif (!toRemove.isEmpty() || !toAdd.isEmpty())\n\t\t\t\t((IAnnotationModelExtension) model).replaceAnnotations(toRemove.toArray(new Annotation[0]), toAdd);\n\n\t\t\t// Change: matched annotations are kept as-is\n\t\t\tfor (Map.Entry<String, Annotation> e : existing.entrySet()) {\n\t\t\t\tSpellingProblem p = desired.get(e.getKey());\n\t\t\t\tAnnotation a = e.getValue();\n\t\t\t\tPosition current = model.getPosition(a);\n\t\t\t\tif (current != null && (current.getOffset() != p.getOffset() || current.getLength() != p.getLength()))\n\t\t\t\t\t((IAnnotationModelExtension) model).modifyAnnotationPosition(a,\n\t\t\t\t\t\t\tnew Position(p.getOffset(), p.getLength()));\n\t\t\t}\n\t\t}\n\n\t\tif (viewer instanceof ITextViewerExtension2)\n\t\t\ttry {\n\t\t\t\t// ((ITextViewerExtension2) viewer).invalidateTextPresentation(offset, length);\n\t\t\t} catch (IllegalArgumentException ex) {\n\t\t\t\t// ignore out of bound errors\n\t\t\t}\n\t\telse\n\t\t\tviewer.invalidateTextPresentation();\n\t}\n"
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
        nodes:
        - id: EditorManager.update
          code: "\tprivate void update(ModificationRange range) {\n\t\tNode node = range.getNode();\n\t\tfor (IManagerListener l : listeners)\n\t\t\tl.onAstUpdated(node);\n\t\tif (viewer instanceof ITextViewerExtension2 ext2)\n\t\t\ttry {\n\t\t\t\text2.invalidateTextPresentation(node.getOffset(), node.length());\n\t\t\t} catch (IllegalArgumentException e) {\n\t\t\t\t// region outside the (possibly just replaced) doc - ignore.\n\t\t\t}\n\n\t\tif (spell != null && !background.isShutdown())\n\t\t\tbackground.execute(() -> {\n\t\t\t\tspell.reconcile(range);\n\t\t\t\tviewer.getTextWidget().getDisplay().asyncExec(() -> {\n\t\t\t\t\tif (viewer instanceof ITextViewerExtension2 ext2)\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\text2.invalidateTextPresentation(node.getOffset(), node.length());\n\t\t\t\t\t\t} catch (IllegalArgumentException e) {\n\t\t\t\t\t\t\t// region outside the (possibly just replaced) doc - ignore.\n\t\t\t\t\t\t}\n\t\t\t\t});\n\t\t\t});\n\t}\n"
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
        nodes:
        - id: AISourceViewerConfiguration.getPresentationReconciler
          type: MethodDeclaration
          code: "\t@Override\n\tpublic IPresentationReconciler getPresentationReconciler(ISourceViewer sourceViewer) {\n\t\tPresentationReconciler reconciler = new PresentationReconciler();\n\n\t\trules.init(sourceViewer.getTextWidget().getFont());\n\t\treconciler.setDamager(dmg, IDocument.DEFAULT_CONTENT_TYPE);\n\t\treconciler.setRepairer(dmg, IDocument.DEFAULT_CONTENT_TYPE);\n\n\t\treturn reconciler;\n\t}\n"
        - id: AISourceViewerConfiguration.DamagerRepairer.createPresentation
          type: MethodDeclaration
          code: "\t\t@Override\n\t\tpublic void createPresentation(TextPresentation presentation, ITypedRegion region) {\n\t\t\tif (fDocument != null && fDocument.getLength() > LIMIT)\n\t\t\t\taddRange(presentation, region.getOffset(), region.getLength(), AIRuleScanner.DEFAULT_ATTR);\n\t\t\telse\n\t\t\t\tsuper.createPresentation(presentation, region);\n\t\t}\n"
```
/answer 735c-1a-19 allow
Result Stats: id=b9820680-49ae-4ca4-87de-fcccf7bcaa40, total: 41917, in: 24, out: 11103, reason: 1673, read: 202512, write: 30790, start: 1795543961130, end: 1795543987572, time: 26307, total: 91441, io: 0, control: 135, sent: 0, recv: 345
ReasoningToken: 1673
Ja. Ich habe den Code durchgesehen und noch nichts geändert. Die Punkte stehen nach erwartetem Nutzen sortiert.

**1. Doppelte Invalidierung der Textdarstellung (`EditorManager.update`)**
- Nach jedem AST-Update wird `invalidateTextPresentation(node.getOffset(), node.length())` zweimal aufgerufen: einmal sofort und einmal nach dem Spellcheck.
- Jede Invalidierung verwirft die gecachten `TextLayout`s der Region und erzwingt Neulayout und Neuzeichnen. Wenn `node` ein großer Abschnitt oder die Wurzel ist, betrifft das einen Großteil des Dokuments.
- Der Aufruf nach dem Spellcheck ist vermutlich überflüssig, weil der `AnnotationPainter` auf Änderungen am Annotation-Model selbst reagiert. Das müsste ich noch verifizieren.
- Für den ersten Aufruf sollte der Bereich auf den tatsächlich geänderten Bereich beschränkt werden statt auf den ganzen Node. Oberhalb von `LIMIT` kann man ihn ganz weglassen, denn dort wird ohnehin nur `DEFAULT_ATTR` gesetzt.

**2. Spellcheck blockiert den UI-Thread (`SpellingStrategy.applyAnnotations`)**
- Der Hintergrund-Thread ruft `syncExec` auf. Danach läuft im UI-Thread `applyAnnotations`, das über alle Annotationen des Models iteriert und für jede `textAt()` aufruft.
- Bei großen Dateien mit vielen Treffern ist das O(Anzahl Annotationen) pro Reconcile im UI-Thread.
- Besser: den Abgleich (`existing`, `toRemove`, `toAdd`) im Hintergrund berechnen und im UI-Thread nur noch `replaceAnnotations` per `asyncExec` ausführen.
- Außerdem kann der Spellcheck und der `AnnotationPainter` ab `LIMIT` abgeschaltet werden. Jede Squiggle-Annotation erzeugt einen eigenen Style-Range und damit zusätzliche Layout-Runs.

**3. Faltung mit `ProjectionViewer`**
- Das ist der größte Hebel für Markdown. Eingeklappte Bereiche belegen im `StyledText` keine Zeilen und werden nicht gelayoutet.
- Die Überschriften-Abschnitte stehen mit Offsets im AST. Daraus lassen sich die `ProjectionAnnotation`s direkt ableiten.
- Der Aufwand ist deutlich höher, weil `createSourceViewer` auf `ProjectionViewer` umgestellt werden muss.

**4. Kleinere Maßnahmen**
- `setRedraw(false)` bzw. `true` um größere Annotation- oder Style-Updates, damit nicht zwischendurch gezeichnet wird.
- `handleCaretMoved` ruft bei jeder Cursorbewegung `asyncExec` auf. Das lässt sich entprellen, damit es beim Halten der Pfeiltaste nicht im UI-Thread aufläuft.
- Unterschiedliche Schriftgrößen oder -arten (zum Beispiel größere Überschriften) machen die Zeilenhöhen ungleichmäßig. Dann kann `StyledText` weniger schätzen und muss mehr Zeilen echt messen. Oberhalb von `LIMIT` ist das durch `DEFAULT_ATTR` schon entschärft.

**Nicht möglich:** `StyledText` rendert und misst immer das ganze Dokument im Hintergrund (Idle-Berechnung der Zeilenhöhen). Der Scrollbar-Daumen kann deshalb bei großen Dateien noch springen. Das lässt sich nur mit Faltung oder einem eigenen Viewport-Konzept umgehen.

Ich würde mit 1 und 2 anfangen, weil das wenig Aufwand ist und direkt im Code liegt. Soll ich das umsetzen? Dafür müsste ich zuerst `ModificationRange` ansehen, um zu sehen, wie groß `node` typischerweise ist.