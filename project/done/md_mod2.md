Implementiere für den Editor `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java` auf Basis von `MarkdownDocument ast` eine Outline. Die Eclipse Editor Outline soll den aktuellen Zustand der MDast visualisieren. Die reine Strukturanzeige ist ausreichend zusammen mit dem Highlighting des Elements, indem sich der Cursor befindet.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=33f8b021-623e-480f-bb54-fb5a217d77fc | model=claude-opus-4-8 | plugins=
Control Request:
```yaml
id: b2031460-d544-468b-8b17-59c6e853be84
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  reason: Read the editor to implement outline
```
/answer b2031460-d544-468b-8b17-59c6e853be84 allow
Control Request:
```yaml
id: 06e63f98-539e-4d60-a799-a0f0e21f0092
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
  reason: Understand project structure
```
/answer 06e63f98-539e-4d60-a799-a0f0e21f0092 allow
Control Request:
```yaml
id: bd48e706-abb4-4f92-9980-38205ea4be2d
result:
  content:
  - type: text
    text: Too many entries (115) exceed the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
  isError: true
```
/answer bd48e706-abb4-4f92-9980-38205ea4be2d deny es handelt sich um ein Eclipse Plugin Projekt.
ReasoningToken: 21
Control Request:
```yaml
id: 28f41390-9530-4813-b434-fd911715a028
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast
  reason: Explore mdast package for MarkdownDocument API
```
/answer 28f41390-9530-4813-b434-fd911715a028 allow
Control Request:
```yaml
id: c35dd2fa-78a1-4b93-bc66-f270349c126b
result:
  structuredContent:
    entries:
    - IDocumentBuffer.java
    - MarkdownDocument.java
    - nodes/AbstractNode.java
    - nodes/Category.java
    - nodes/HeadingSection.java
    - nodes/Node.java
    - nodes/Paragraph.java
    - nodes/Root.java
```
/answer c35dd2fa-78a1-4b93-bc66-f270349c126b allow
Control Request:
```yaml
id: d7e97206-9240-466c-a3cb-cf02898b9bc5
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors
  reason: Explore editors package
```
/answer d7e97206-9240-466c-a3cb-cf02898b9bc5 allow
Control Request:
```yaml
id: 0c790c9d-22cf-4c8e-a1a6-bfd30913378b
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
    - md/AbstractRule.java
    - md/BlockRule.java
    - md/EmphasisRule.java
    - md/HeaderRule.java
    - md/LineMatchRule.java
    - md/LinkRule.java
    - md/ListRule.java
    - md/PrefixLineRule.java
    - md/WordRule.java
    - spellcheck/LanguageToolClient.java
    - spellcheck/SpellCheckInstaller.java
    - spellcheck/SpellCheckReconciler.java
    - spellcheck/SpellingAnnotation.java
    - spellcheck/SpellingProblem.java
    - spellcheck/SpellingQuickAssistProcessor.java
    - spellcheck/SpellingStrategy.java
```
/answer 0c790c9d-22cf-4c8e-a1a6-bfd30913378b allow
ReasoningToken: 30
Control Request:
```yaml
id: 42ab581a-3d7d-43a1-9254-d8b8a877adfe
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  reason: Understand MarkdownDocument API
```
/answer 42ab581a-3d7d-43a1-9254-d8b8a877adfe allow
Control Request:
```yaml
id: 26ffdff8-3536-4ac2-acb0-21eb5bd48a98
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Node.java
  reason: Understand Node interface
```
/answer 26ffdff8-3536-4ac2-acb0-21eb5bd48a98 allow
Control Request:
```yaml
id: 42a663a2-9d44-44d6-8735-edab8b7cdef5
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/AbstractNode.java
  reason: Understand AbstractNode
```
/answer 42a663a2-9d44-44d6-8735-edab8b7cdef5 allow
Control Request:
```yaml
id: 8cb93414-2028-4ecf-8908-0b672dd9fc50
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Category.java
  reason: Understand Category
```
/answer 8cb93414-2028-4ecf-8908-0b672dd9fc50 allow
Control Request:
```yaml
id: ce65f724-6176-4b65-bc41-78491c4b5ecb
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/HeadingSection.java
  reason: Understand HeadingSection node
```
/answer ce65f724-6176-4b65-bc41-78491c4b5ecb allow
Control Request:
```yaml
id: 399562af-78db-4f3e-af69-a34318aba2f0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Root.java
  reason: Understand Root node
```
/answer 399562af-78db-4f3e-af69-a34318aba2f0 allow
Control Request:
```yaml
id: ebd50a5e-4096-4c3f-82d9-ff245cab1cec
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Paragraph.java
  reason: Understand Paragraph node
```
/answer ebd50a5e-4096-4c3f-82d9-ff245cab1cec allow
Control Request:
```yaml
id: 0cd08746-2c6c-4a2b-9600-56182250ba88
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/DocumentBuffer.java
  reason: Understand DocumentBuffer for text extraction
```
/answer 0cd08746-2c6c-4a2b-9600-56182250ba88 allow
Control Request:
```yaml
id: 1fa5ef11-0018-4195-b395-94ca5b08fbca
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/IDocumentBuffer.java
  reason: Understand buffer interface
```
/answer 1fa5ef11-0018-4195-b395-94ca5b08fbca allow
ReasoningToken: 103
Text:
 I now understand the structure. Let me implement the outline. First, I'll expose the root node in `MarkdownDocument`.
Control Request:
```yaml
id: 339b96f4-c0cb-41df-889a-41fe1fc67c85
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  old_text: "\tpublic Node find(int lo, int hi) {\n\t\treturn find(root, lo, hi);\n\t}"
  new_text: "\tpublic Node getRoot() {\n\t\treturn root;\n\t}\n\n\tpublic Node find(int lo, int hi) {\n\t\treturn find(root, lo, hi);\n\t}"
  reason: Expose root node for the outline
```
/answer 339b96f4-c0cb-41df-889a-41fe1fc67c85 allow
ReasoningToken: 5030
Text:
 Now the outline page:
Control Request:
```yaml
id: c31ab94e-a743-47c9-8afb-9dc5d608f92b
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/MarkdownOutlinePage.java
  mode: replace
  content: "package xy.ai.workbench.editors;\n\nimport org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.viewers.ITreeContentProvider;\nimport org.eclipse.jface.viewers.LabelProvider;\nimport org.eclipse.jface.viewers.SelectionChangedEvent;\nimport org.eclipse.jface.viewers.StructuredSelection;\nimport org.eclipse.jface.viewers.TreeViewer;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.ui.views.contentoutline.ContentOutlinePage;\n\nimport xy.ai.workbench.mdast.MarkdownDocument;\nimport xy.ai.workbench.mdast.nodes.HeadingSection;\nimport xy.ai.workbench.mdast.nodes.Node;\nimport xy.ai.workbench.mdast.nodes.Paragraph;\nimport xy.ai.workbench.mdast.nodes.Root;\n\n/**\n * Content outline page that visualizes the current state of the\n * {@link MarkdownDocument} MDast. It only reflects the tree structure and keeps\n * the node in which the editor caret is located highlighted.\n */\npublic class MarkdownOutlinePage extends ContentOutlinePage {\n\n\tprivate static final Object[] EMPTY = new Object[0];\n\tprivate static final int LABEL_LIMIT = 80;\n\n\tprivate final AITextEditor editor;\n\tprivate boolean syncingFromEditor;\n\n\tpublic MarkdownOutlinePage(AITextEditor editor) {\n\t\tthis.editor = editor;\n\t}\n\n\t@Override\n\tpublic void createControl(Composite parent) {\n\t\tsuper.createControl(parent);\n\n\t\tTreeViewer viewer = getTreeViewer();\n\t\tviewer.setContentProvider(new OutlineContentProvider());\n\t\tviewer.setLabelProvider(new OutlineLabelProvider());\n\t\tviewer.setInput(editor.getMarkdownAst());\n\t\tviewer.addSelectionChangedListener(this::onOutlineSelection);\n\t}\n\n\tprivate void onOutlineSelection(SelectionChangedEvent event) {\n\t\tif (syncingFromEditor)\n\t\t\treturn;\n\t\tif (event.getSelection() instanceof StructuredSelection sel\n\t\t\t\t&& sel.getFirstElement() instanceof Node node)\n\t\t\teditor.selectAndRevealNode(node);\n\t}\n\n\t/** Rebuilds the tree from the current AST state. */\n\tpublic void refresh() {\n\t\tTreeViewer viewer = getTreeViewer();\n\t\tif (!isAlive(viewer))\n\t\t\treturn;\n\t\tif (viewer.getInput() != editor.getMarkdownAst())\n\t\t\tviewer.setInput(editor.getMarkdownAst());\n\t\telse\n\t\t\tviewer.refresh();\n\t}\n\n\t/** Highlights the deepest node that contains the given document offset. */\n\tpublic void selectNodeForOffset(int offset) {\n\t\tTreeViewer viewer = getTreeViewer();\n\t\tif (!isAlive(viewer))\n\t\t\treturn;\n\t\tMarkdownDocument ast = editor.getMarkdownAst();\n\t\tif (ast == null || ast.getRoot() == null)\n\t\t\treturn;\n\n\t\tNode node = deepest(ast.getRoot(), offset);\n\t\tif (node == null || node.instance == Root.INSTANCE)\n\t\t\treturn;\n\n\t\tsyncingFromEditor = true;\n\t\ttry {\n\t\t\tviewer.setSelection(new StructuredSelection(node), true);\n\t\t} finally {\n\t\t\tsyncingFromEditor = false;\n\t\t}\n\t}\n\n\tprivate Node deepest(Node node, int offset) {\n\t\tfor (Node child : node.children)\n\t\t\tif (offset >= child.getOffset() && offset < child.getEndOffset())\n\t\t\t\treturn deepest(child, offset);\n\t\treturn node;\n\t}\n\n\tprivate boolean isAlive(TreeViewer viewer) {\n\t\treturn viewer != null && viewer.getControl() != null && !viewer.getControl().isDisposed();\n\t}\n\n\tprivate String label(Node node) {\n\t\tString snippet = snippet(node);\n\t\tif (!snippet.isEmpty())\n\t\t\treturn snippet;\n\t\tif (node.instance instanceof HeadingSection)\n\t\t\treturn \"Heading\";\n\t\tif (node.instance instanceof Paragraph)\n\t\t\treturn \"Paragraph\";\n\t\treturn node.instance.getCategory().name();\n\t}\n\n\tprivate String snippet(Node node) {\n\t\tIDocument doc = editor.getMarkdownDocument();\n\t\tif (doc == null)\n\t\t\treturn \"\";\n\t\tint offset = node.getOffset();\n\t\tint length = node.length();\n\t\tif (offset < 0 || length <= 0)\n\t\t\treturn \"\";\n\t\tlength = Math.min(length, doc.getLength() - offset);\n\t\tif (length <= 0)\n\t\t\treturn \"\";\n\t\ttry {\n\t\t\tString text = doc.get(offset, length).strip();\n\t\t\tint nl = text.indexOf('\\n');\n\t\t\tif (nl >= 0)\n\t\t\t\ttext = text.substring(0, nl).strip();\n\t\t\tif (text.length() > LABEL_LIMIT)\n\t\t\t\ttext = text.substring(0, LABEL_LIMIT) + \"…\";\n\t\t\treturn text;\n\t\t} catch (BadLocationException e) {\n\t\t\treturn \"\";\n\t\t}\n\t}\n\n\tprivate final class OutlineContentProvider implements ITreeContentProvider {\n\t\t@Override\n\t\tpublic Object[] getElements(Object input) {\n\t\t\tif (input instanceof MarkdownDocument doc && doc.getRoot() != null)\n\t\t\t\treturn doc.getRoot().children.toArray();\n\t\t\treturn EMPTY;\n\t\t}\n\n\t\t@Override\n\t\tpublic Object[] getChildren(Object element) {\n\t\t\treturn element instanceof Node node ? node.children.toArray() : EMPTY;\n\t\t}\n\n\t\t@Override\n\t\tpublic Object getParent(Object element) {\n\t\t\treturn element instanceof Node node ? node.parent : null;\n\t\t}\n\n\t\t@Override\n\t\tpublic boolean hasChildren(Object element) {\n\t\t\treturn element instanceof Node node && !node.children.isEmpty();\n\t\t}\n\t}\n\n\tprivate final class OutlineLabelProvider extends LabelProvider {\n\t\t@Override\n\t\tpublic String getText(Object element) {\n\t\t\treturn element instanceof Node node ? label(node) : String.valueOf(element);\n\t\t}\n\t}\n}\n"
  reason: Create the markdown content outline page
```
/answer c31ab94e-a743-47c9-8afb-9dc5d608f92b allow
Control Request:
```yaml
id: c742cec0-ecac-4a6b-b01e-cfa999054f74
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  old_text: |-
    import org.eclipse.jface.text.source.SourceViewer;
    import org.eclipse.swt.widgets.Composite;
    import org.eclipse.ui.editors.text.TextEditor;

    import xy.ai.workbench.editors.spellcheck.SpellCheckInstaller;
    import xy.ai.workbench.mdast.MarkdownDocument;
  new_text: |-
    import org.eclipse.jface.text.source.SourceViewer;
    import org.eclipse.swt.custom.StyledText;
    import org.eclipse.swt.widgets.Composite;
    import org.eclipse.ui.editors.text.TextEditor;
    import org.eclipse.ui.views.contentoutline.IContentOutlinePage;

    import xy.ai.workbench.editors.spellcheck.SpellCheckInstaller;
    import xy.ai.workbench.mdast.MarkdownDocument;
    import xy.ai.workbench.mdast.nodes.Node;
  reason: Add imports for outline support
```
/answer c742cec0-ecac-4a6b-b01e-cfa999054f74 allow
Control Request:
```yaml
id: 397d2726-942f-4df5-a696-dee86fd0bffe
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  old_text: "\tprivate MarkdownDocument ast;\n\tprivate DocumentBuffer astBuffer;\n\tprivate int pendingRemoved;\n\n\tprivate final IDocumentListener docListener = new IDocumentListener() {\n\t\t@Override\n\t\tpublic void documentChanged(DocumentEvent evt) {\n\t\t\tupdateRulerVisibility(evt.getDocument());\n\t\t\tupdateLineNumbers(evt.getDocument());\n\t\t\tupdateAst(evt);\n\t\t}"
  new_text: "\tprivate MarkdownDocument ast;\n\tprivate DocumentBuffer astBuffer;\n\tprivate int pendingRemoved;\n\n\tprivate MarkdownOutlinePage outlinePage;\n\n\tprivate final IDocumentListener docListener = new IDocumentListener() {\n\t\t@Override\n\t\tpublic void documentChanged(DocumentEvent evt) {\n\t\t\tupdateRulerVisibility(evt.getDocument());\n\t\t\tupdateLineNumbers(evt.getDocument());\n\t\t\tupdateAst(evt);\n\t\t\trefreshOutline();\n\t\t}"
  reason: Track outline page and refresh on change
```
/answer 397d2726-942f-4df5-a696-dee86fd0bffe allow
Control Request:
```yaml
id: 873577dd-fb97-4ceb-85e8-b589d9506fc7
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  old_text: "\t\t\t\t\tupdateLineNumbers(newInput);\n\t\t\t\t\tbuildAst(newInput);\n\t\t\t\t} else {\n\t\t\t\t\tast = null;\n\t\t\t\t\tastBuffer = null;\n\t\t\t\t}"
  new_text: "\t\t\t\t\tupdateLineNumbers(newInput);\n\t\t\t\t\tbuildAst(newInput);\n\t\t\t\t\trefreshOutline();\n\t\t\t\t} else {\n\t\t\t\t\tast = null;\n\t\t\t\t\tastBuffer = null;\n\t\t\t\t\trefreshOutline();\n\t\t\t\t}"
  reason: Refresh outline on input change
```
/answer 873577dd-fb97-4ceb-85e8-b589d9506fc7 allow
Control Request:
```yaml
id: 4fc9cae1-8188-4169-b772-f96b5d4fb0e0
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  old_text: "\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tsuper.createPartControl(parent);\n\t\tSpellCheckInstaller.installPainter(getSourceViewer());\n\t}"
  new_text: "\t@Override\n\tpublic void createPartControl(Composite parent) {\n\t\tsuper.createPartControl(parent);\n\t\tSpellCheckInstaller.installPainter(getSourceViewer());\n\n\t\tif (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)\n\t\t\twidget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));\n\t}\n\n\tprivate void handleCaretMoved(int offset) {\n\t\tif (outlinePage != null)\n\t\t\toutlinePage.selectNodeForOffset(offset);\n\t}\n\n\t@Override\n\tpublic <T> T getAdapter(Class<T> adapter) {\n\t\tif (IContentOutlinePage.class.equals(adapter)) {\n\t\t\tif (outlinePage == null)\n\t\t\t\toutlinePage = new MarkdownOutlinePage(this);\n\t\t\treturn adapter.cast(outlinePage);\n\t\t}\n\t\treturn super.getAdapter(adapter);\n\t}\n\n\tprivate void refreshOutline() {\n\t\tif (outlinePage != null)\n\t\t\toutlinePage.refresh();\n\t}\n\n\tpublic void selectAndRevealNode(Node node) {\n\t\tif (node == null)\n\t\t\treturn;\n\t\tselectAndReveal(node.getOffset(), node.length());\n\t}\n\n\tpublic IDocument getMarkdownDocument() {\n\t\treturn astBuffer != null ? astBuffer.document() : null;\n\t}"
  reason: "Wire outline adapter, caret sync and helpers"
```
/answer 4fc9cae1-8188-4169-b772-f96b5d4fb0e0 allow
Control Request:
```yaml
id: ef1dc6c9-46b7-42f8-955c-2d19f8a8dc87
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  reason: Verify integration
  start: "\tpublic void createPartControl"
  end: "\tpublic MarkdownDocument getMarkdownAst"
```
/answer ef1dc6c9-46b7-42f8-955c-2d19f8a8dc87 allow
Control Request:
```yaml
id: 2286bdf3-955c-4c21-83df-500138d649a7
result:
  structuredContent:
    checksum: 46f690a6ab1bdd0596bda0fe09ca87e9d449fda90a2a70f10784471afddaebe7
    content: "\tpublic void createPartControl(Composite parent) {\n\t\tsuper.createPartControl(parent);\n\t\tSpellCheckInstaller.installPainter(getSourceViewer());\n\n\t\tif (getSourceViewer() != null && getSourceViewer().getTextWidget() instanceof StyledText widget)\n\t\t\twidget.addCaretListener(evt -> handleCaretMoved(evt.caretOffset));\n\t}\n\n\tprivate void handleCaretMoved(int offset) {\n\t\tif (outlinePage != null)\n\t\t\toutlinePage.selectNodeForOffset(offset);\n\t}\n\n\t@Override\n\tpublic <T> T getAdapter(Class<T> adapter) {\n\t\tif (IContentOutlinePage.class.equals(adapter)) {\n\t\t\tif (outlinePage == null)\n\t\t\t\toutlinePage = new MarkdownOutlinePage(this);\n\t\t\treturn adapter.cast(outlinePage);\n\t\t}\n\t\treturn super.getAdapter(adapter);\n\t}\n\n\tprivate void refreshOutline() {\n\t\tif (outlinePage != null)\n\t\t\toutlinePage.refresh();\n\t}\n\n\tpublic void selectAndRevealNode(Node node) {\n\t\tif (node == null)\n\t\t\treturn;\n\t\tselectAndReveal(node.getOffset(), node.length());\n\t}\n\n\tpublic IDocument getMarkdownDocument() {\n\t\treturn astBuffer != null ? astBuffer.document() : null;\n\t}\n\n\t@Override\n\tprotected boolean getInitialWordWrapStatus() {\n\t\treturn true;\n\t}\n\n\tprivate void buildAst(IDocument document) {\n\t\tastBuffer = new DocumentBuffer(document);\n\t\tast = new MarkdownDocument(astBuffer);\n\t\tast.update(0, 0, astBuffer.length());\n\t}\n\n\tprivate void updateAst(DocumentEvent evt) {\n\t\tif (ast == null || astBuffer == null || astBuffer.document() != evt.getDocument())\n\t\t\treturn;\n\t\tString text = evt.getText();\n\t\tint inserted = text == null ? 0 : text.length();\n\t\tast.update(evt.getOffset(), pendingRemoved, inserted);\n\t}\n\n\tpublic MarkdownDocument getMarkdownAst"
```
/answer 2286bdf3-955c-4c21-83df-500138d649a7 allow
Text:
 Now wire it into the editor:
ReasoningToken: 125
Text:
 Let me verify the final editor file compiles logically:
ReasoningToken: 48
ReasoningToken: 201
Die Implementierung ist vollständig. Zusammenfassung der Änderungen:

## Neue Datei: `MarkdownOutlinePage.java`
Eine `ContentOutlinePage`, die einen `TreeViewer` mit der MDast-Struktur befüllt:

- **`OutlineContentProvider`** – navigiert den `Node`-Baum ab `MarkdownDocument.getRoot()` (`getElements` → Root-Kinder, `getChildren` → `node.children`, plus `getParent`/`hasChildren`). Damit bildet die Outline exakt die aktuelle Baumstruktur der MDast ab.
- **`OutlineLabelProvider`** – erzeugt reine Struktur-Labels: erste nicht-leere Zeile des jeweiligen Node-Bereichs (auf 80 Zeichen begrenzt), mit Fallback auf `Heading`/`Paragraph`/Kategorie.
- **`refresh()`** – aktualisiert den Baum bei AST-Änderungen (setzt Input neu bzw. `viewer.refresh()`).
- **`selectNodeForOffset(int)`** – ermittelt den tiefsten Node, der den Offset enthält (`deepest(...)`), und selektiert/highlightet ihn im Tree. Ein Flag `syncingFromEditor` verhindert Rückkopplungsschleifen.

## Änderungen an `AITextEditor.java`
- **`getAdapter(Class)`** liefert (lazy erzeugt) die `MarkdownOutlinePage` für `IContentOutlinePage` – so bindet Eclipse die Outline an den Editor.
- **Caret-Tracking**: In `createPartControl` wird ein `CaretListener` am `StyledText` registriert → `handleCaretMoved` → `outlinePage.selectNodeForOffset(...)`. So wird das Element unter dem Cursor hervorgehoben.
- **`refreshOutline()`** wird bei jedem `documentChanged` (nach `updateAst`) sowie beim Input-Wechsel (nach `buildAst`) aufgerufen, damit die Outline stets den aktuellen MDast-Zustand zeigt.
- **`selectAndRevealNode(Node)`** setzt bei Klick in der Outline den Editor-Cursor/Selektion auf den Node-Bereich.
- **`getMarkdownDocument()`** stellt das `IDocument` für die Label-Textextraktion bereit.

## Änderung an `MarkdownDocument.java`
- **`getRoot()`** hinzugefügt, um den Wurzel-Node dem Content Provider zugänglich zu machen.

Die Anzeige ist rein strukturell (keine Editier-Aktionen), mit bidirektionaler Synchronisation zwischen Cursor und Outline-Highlight.
