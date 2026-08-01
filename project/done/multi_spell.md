Verbessere die Spellcheck Implementierung in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java`.
Der Spellcheck soll zwei Änderungen erhalten.
Zum einen soll er nicht nur einen Background Thread haben in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java` sondern einen Pool und parallel arbeiten können. 1 Thread bereithalten und maximal 16 (Anzahl CPU Cores). Nicht mehr als einen Thread Idle.

Zum zweiten sollen die Methoden `find()` und `update()` in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java` eine ModificationRange. Diese liefert mindestens einen Knoten aber auch zusätzlich eine Subrange die den geänderten Bereich auf ganze Zeilen expandiert.
Der LineIndex kann für die Zeilenermittelung verwendet werden.
Der Spellcheck kann damit die weiterhin die representation für den ganzen Block invalidieren muss aber nur die Annotationen aus den geänderten Zeilen prüfen.
Der Spellcheck kann bestehene Annotations im Block außerhalb der Zeilen ignorieren und muss die tatsächlich betroffen Teilen neu korrigieren. Der Spellcheck muss bei Offsetänderungen die folgenden Annotations korrigieren.
Ziel ist die Optimierung der Performance vom Spellcheck.
SystemInit:  id=52f19019-790b-4b6c-a2a7-6f59ff965ab3, model=/home/user/xyan/xy.ai.workbench, cwd=claude-sonnet-5
Control Request:
```yaml
id: 1735d205-f904-4cd1-ac46-663e30e229c0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  reason: Review current spellcheck implementation
```
/answer 1735d205-f904-4cd1-ac46-663e30e229c0 allow
Control Request:
```yaml
id: 4ae67009-2253-4a6c-8b0e-f15c284a410c
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
  reason: Review current thread usage
```
/answer 4ae67009-2253-4a6c-8b0e-f15c284a410c allow
Control Request:
```yaml
id: 34025577-ecc7-45c8-b79d-1d2e5f2b2f82
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  reason: Review find/update methods
```
/answer 34025577-ecc7-45c8-b79d-1d2e5f2b2f82 allow
ReasoningToken: 14
Control Request:
```yaml
id: 1d8caa1c-23e6-4018-80a0-e30417d2a77c
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
  reason: Explore related files
```
/answer 1d8caa1c-23e6-4018-80a0-e30417d2a77c allow
Control Request:
```yaml
id: 216f425a-e4ee-4f86-a036-db31bff143b4
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
    - EditorManager.java
    - IManagerListener.java
    - ISpellChecker.java
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
    - spellcheck/SpellCheckReconciler.java
    - spellcheck/SpellingAnnotation.java
    - spellcheck/SpellingProblem.java
    - spellcheck/SpellingQuickAssistProcessor.java
    - spellcheck/SpellingStrategy.java
```
/answer 216f425a-e4ee-4f86-a036-db31bff143b4 allow
Control Request:
```yaml
id: 56f92c77-599c-4f3c-8b4a-208ad1ff8e5d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java
  reason: Understand LineIndex API
```
/answer 56f92c77-599c-4f3c-8b4a-208ad1ff8e5d allow
Control Request:
```yaml
id: 0c02a689-9ab8-4f7c-8302-1db33f50aa20
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/ISpellChecker.java
  reason: Interface
```
/answer 0c02a689-9ab8-4f7c-8302-1db33f50aa20 allow
ReasoningToken: 50
Control Request:
```yaml
id: 6a68bf1d-9d7d-4a13-82e6-c400c1a0556a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellingStrategy.java
  reason: Understand reconcile/clear implementation
```
/answer 6a68bf1d-9d7d-4a13-82e6-c400c1a0556a allow
Control Request:
```yaml
id: 841435f2-b0f1-462e-ad50-5badc6e8841d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Node.java
  reason: Understand Node structure
```
/answer 841435f2-b0f1-462e-ad50-5badc6e8841d allow
Control Request:
```yaml
id: 60a13973-ef94-4448-9bd8-ca74f24e4e53
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellingAnnotation.java
  reason: Understand annotation model
```
/answer 60a13973-ef94-4448-9bd8-ca74f24e4e53 allow
Control Request:
```yaml
id: 880de6f2-e50b-44db-bc11-64635e2f2ffa
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellingProblem.java
  reason: Understand problem model
```
/answer 880de6f2-e50b-44db-bc11-64635e2f2ffa allow
Control Request:
```yaml
id: f2f31f91-033e-4cdb-82f8-fac14fe12a9f
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
  pattern: .*Annotation.*|.*SourceViewer.*|.*Configuration.*
  reason: Find annotation model wiring
```
/answer f2f31f91-033e-4cdb-82f8-fac14fe12a9f allow
Control Request:
```yaml
id: adde6584-07b2-4702-837a-80afbfdf0af2
result:
  structuredContent:
    entries:
    - editor/AISourceViewerConfiguration.java
    - editor/spellcheck/SpellingAnnotation.java
```
/answer adde6584-07b2-4702-837a-80afbfdf0af2 allow
Control Request:
```yaml
id: 135f43a7-1902-4c65-8ef1-23cc42d04e52
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  reason: Check annotation model setup
```
/answer 135f43a7-1902-4c65-8ef1-23cc42d04e52 allow
ReasoningToken: 1990
Control Request:
```yaml
id: b5efab90-f6ea-4a18-a901-be5fb89ab33e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java
  reason: Check annotation model setup for source viewer
```
/answer b5efab90-f6ea-4a18-a901-be5fb89ab33e allow
ReasoningToken: 847
Control Request:
```yaml
id: 107c902b-8c6c-468a-ad9c-76df73191245
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/IManagerListener.java
  reason: Check listener interface
```
/answer 107c902b-8c6c-468a-ad9c-76df73191245 allow
Control Request:
```yaml
id: e4689c18-6ac7-4ee9-88e6-0229f9a75825
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/IDocumentBuffer.java
  reason: Check buffer interface
```
/answer e4689c18-6ac7-4ee9-88e6-0229f9a75825 allow
Control Request:
```yaml
id: e90097e8-e9ec-4d5e-8c94-4340d5fec735
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src
  pattern: .*\.java
  reason: find all usages potentially referencing ast.find/update/Node reconcile
```
/answer e90097e8-e9ec-4d5e-8c94-4340d5fec735 allow
Control Request:
```yaml
id: f69d160a-ba28-4cb3-b6fc-64d2dc9fa64a
result:
  content:
  - type: text
    text: Too many entries (125) exceed the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
  isError: true
```
/answer f69d160a-ba28-4cb3-b6fc-64d2dc9fa64a allow
ReasoningToken: 7277
ReasoningToken: 48
Control Request:
```yaml
id: 46373876-0a77-4e29-9cc1-0a5defd19f61
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "\.find(\|getAst()\|reconcile(\|ISpellChecker\|onAstUpdated\|ast\.update\|MarkdownDocument" src --include=*.java
  reason: Find all usages of relevant APIs
```
/answer 46373876-0a77-4e29-9cc1-0a5defd19f61 allow
Control Request:
```yaml
id: fa2296c9-e561-4746-b061-68455a15a3fe
result:
  content:
  - type: text
    text: Full output written to file 3878 characters). Reduce to relevant excerpts (e.g. via grep/head/tail).
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-367izk7n.log
```
/answer fa2296c9-e561-4746-b061-68455a15a3fe allow
Control Request:
```yaml
id: d0d4b5e2-64bb-42ef-8dc4-97ba607b7870
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: cat /tmp/mcpc-stdout-367izk7n.log
  reason: view grep output
```
/answer d0d4b5e2-64bb-42ef-8dc4-97ba607b7870 deny nutze read-file
Control Request:
```yaml
id: 8da72613-79e7-47dc-bd13-fecf50425907
toolName: read-file
arguments:
  path: /tmp/mcpc-stdout-367izk7n.log
  reason: view grep output
```
/answer 8da72613-79e7-47dc-bd13-fecf50425907 allow
Control Request:
```yaml
id: 60621721-7194-4069-9e4a-a85a89e13621
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/MarkdownOutlinePage.java
  min_line: 60
  max_line: 90
  reason: check find() usage context
```
/answer 60621721-7194-4069-9e4a-a85a89e13621 allow
Control Request:
```yaml
id: b5ffd42f-f436-426b-9885-da203f35c5d8
result:
  structuredContent:
    checksum: 88cd8995ee63c565a9cc19d206c2b231273b20323a1a6189d36e04f5ec533d50
    content: "\t\telse\n\t\t\tviewer.refresh();\n\t}\n\n\t/** Highlights the deepest node that contains the given document offset. */\n\tpublic void selectNodeForOffset(int offset) {\n\t\tTreeViewer viewer = getTreeViewer();\n\t\tif (!isAlive(viewer))\n\t\t\treturn;\n\t\tMarkdownDocument ast = editor.getUpdateManager().getAst();\n\t\tif (ast == null || ast.getRoot() == null)\n\t\t\treturn;\n\n\t\tNode node = ast.find(offset, offset);\n\t\tif (node == null || node.instance == Elements.ROOT)\n\t\t\treturn;\n\n\t\tvar sel = viewer.getSelection();\n\t\tif (sel instanceof StructuredSelection ssel && ssel.getFirstElement() == node)\n\t\t\treturn;\n\t\tDisplay.getDefault().asyncExec(() -> {\n\t\t\tsyncingFromEditor = true;\n\t\t\ttry {\n\t\t\t\tviewer.setSelection(new StructuredSelection(node), true);\n\t\t\t} finally {\n\t\t\t\tsyncingFromEditor = false;\n\t\t\t}\n\t\t});\n\t}\n\n\tprivate boolean isAlive(TreeViewer viewer) {\n"
```
/answer b5ffd42f-f436-426b-9885-da203f35c5d8 allow
Control Request:
```yaml
id: af460600-f47a-487b-80ca-4ab2ac3a787c
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  min_line: 185
  max_line: 215
  reason: check find() usage context
```
/answer af460600-f47a-487b-80ca-4ab2ac3a787c allow
Control Request:
```yaml
id: e29d2301-6062-45a5-bd40-272ca58754e0
result:
  structuredContent:
    checksum: 2b79959c7a08d61855bc42f0d48ec785e54cc7cc7254544cb11694cb81529970
    content: "\t}\n\n\t@Override\n\tpublic void setRange(IDocument document, int offset, int length) {\n\t\tpieces.clear();\n\t\ttokenOffset = offset;\n\t\ttokenLength = 0;\n\n\t\tif (length <= 0)\n\t\t\treturn;\n\n\t\tMarkdownDocument ast = updateManager.getAst();\n\t\tif (ast == null) {\n\t\t\tscanFlat(fallbackScanner, document, offset, offset + length);\n\t\t\treturn;\n\t\t}\n\n\t\tNode governing = ast.find(offset, offset + length);\n\t\tcollect(document, governing, offset, offset + length);\n\t}\n\n\t/**\n\t * Walks the subtree of {@code node} that overlaps [lo, hi), emitting rule\n\t * matches for the node's own text (the \"gaps\" between its children) and\n\t * recursing into every overlapping child using the child's own rules. Every\n\t * character of [lo, hi) is visited exactly once.\n\t */\n\tprivate void collect(IDocument document, Node node, int lo, int hi) {\n\t\tint cursor = Math.max(node.getOffset(), lo);\n\n\t\tfor (Node child : node.children) {\n"
```
/answer e29d2301-6062-45a5-bd40-272ca58754e0 allow
Control Request:
```yaml
id: ba34b574-b4c5-4d06-98f0-7b4a1a0411d0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AITextEditor.java
  min_line: 100
  max_line: 145
  reason: check onAstUpdated usage
```
/answer ba34b574-b4c5-4d06-98f0-7b4a1a0411d0 allow
Control Request:
```yaml
id: 70ccc165-add8-4b22-8d99-75849f3ae76d
result:
  structuredContent:
    checksum: 0831a4617fe6322cfe83245c684fe0c16dcfaca6b6706b2dba982383721544e8
    content: "\t\tif (IContentOutlinePage.class.equals(adapter)) {\n\t\t\tif (outlinePage == null)\n\t\t\t\toutlinePage = new MarkdownOutlinePage(this);\n\t\t\treturn adapter.cast(outlinePage);\n\t\t}\n\t\treturn super.getAdapter(adapter);\n\t}\n\n\tprivate void refreshOutline() {\n\t\tif (outlinePage != null)\n\t\t\toutlinePage.refresh();\n\t}\n\n\tpublic void selectAndRevealNode(Node node) {\n\t\tif (node == null)\n\t\t\treturn;\n\t\tselectAndReveal(node.getOffset(), node.length());\n\t}\n\n\t@Override\n\tprotected boolean getInitialWordWrapStatus() {\n\t\treturn true;\n\t}\n\n\tpublic EditorManager getUpdateManager() {\n\t\treturn manager;\n\t}\n\n\tprivate class ManagerListener implements IManagerListener {\n\t\t@Override\n\t\tpublic void onDocumentChanged(IDocument oldDocument, IDocument newDocument) {\n\t\t\tif (newDocument == null)\n\t\t\t\trefreshOutline(); // clears the outline; onAstUpdated covers the non-null case\n\t\t}\n\n\t\t@Override\n\t\tpublic void onAstUpdated(Node node) {\n\t\t\tIDocument doc = manager.getDocument();\n\t\t\tif (doc == null)\n\t\t\t\treturn;\n\t\t\tupdateRulerVisibility(doc);\n\t\t\tupdateLineNumbers(doc);\n\t\t\trefreshOutline();\n\t\t}\n\t}\n\n"
```
/answer 70ccc165-add8-4b22-8d99-75849f3ae76d allow
ReasoningToken: 102
Control Request:
```yaml
id: 5976f00b-59fc-421e-8d49-5aa75d96c59d
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/ModificationRange.java
  mode: replace
  content: "package xy.ai.workbench.editor.mdast;\n\nimport xy.ai.workbench.editor.mdast.nodes.Node;\n\n/**\n * Result of a {@link MarkdownDocument#update(int, int, int)} or\n * {@link MarkdownDocument#find(int, int)} call.\n * <p>\n * Carries the (possibly coarse grained) AST {@link Node} that has to be\n * treated as changed/looked up - used e.g. for structural purposes such as\n * representation invalidation or damage regions - together with the actually\n * touched sub-range of the document, expanded to complete lines.\n * <p>\n * Consumers that only care about the exact text that changed (e.g. the spell\n * checker) can restrict expensive per-character work to\n * [{@link #getStart()}, {@link #getEnd()}) instead of re-processing the whole\n * node, while everything outside that range - but still inside the node -\n * can be left untouched.\n */\npublic class ModificationRange {\n\n\tprivate final Node node;\n\tprivate final int start;\n\tprivate final int end;\n\n\tpublic ModificationRange(Node node, int start, int end) {\n\t\tthis.node = node;\n\t\tthis.start = start;\n\t\tthis.end = end;\n\t}\n\n\t/** The (at least) affected AST node. */\n\tpublic Node getNode() {\n\t\treturn node;\n\t}\n\n\t/** Start offset of the modified range, expanded to a full line start. */\n\tpublic int getStart() {\n\t\treturn start;\n\t}\n\n\t/** End offset of the modified range, expanded to a full line end. */\n\tpublic int getEnd() {\n\t\treturn end;\n\t}\n\n\tpublic int length() {\n\t\treturn end - start;\n\t}\n}\n"
  reason: Create new ModificationRange class
```
/answer 5976f00b-59fc-421e-8d49-5aa75d96c59d allow
ReasoningToken: 3450
Control Request:
```yaml
id: 24f217ea-b7ef-4c1d-9c79-d7eab7a3fa46
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  old_text: "\tpublic Node update(int offset, int removed, int inserted) {\n\t\tint lo = offset;\n\t\tint hi = offset + removed;\n\t\tint delta = inserted - removed;\n\t\tlines.update(buffer, offset, removed, inserted);\n\n\t\tNode sec = findForUpdate(lo, hi);\n\t\twhile (true) {\n\t\t\tNode parent = sec.parent;\n\t\t\tint absStart = sec.getOffset();\n\t\t\tint newLen = Math.max(hi - absStart, sec.length()) + delta;\n\t\t\tNode rn = parse(absStart, absStart + newLen);\n\n\t\t\tif (parent == null || isCompatible(rn.children, sec, parent))\n\t\t\t\treturn replace(sec, rn.children, delta);\n\t\t\tsec = parent;\n\t\t}\n\t}"
  new_text: "\tpublic ModificationRange update(int offset, int removed, int inserted) {\n\t\tint lo = offset;\n\t\tint hi = offset + removed;\n\t\tint delta = inserted - removed;\n\t\tlines.update(buffer, offset, removed, inserted);\n\n\t\tNode sec = findForUpdate(lo, hi);\n\t\twhile (true) {\n\t\t\tNode parent = sec.parent;\n\t\t\tint absStart = sec.getOffset();\n\t\t\tint newLen = Math.max(hi - absStart, sec.length()) + delta;\n\t\t\tNode rn = parse(absStart, absStart + newLen);\n\n\t\t\tif (parent == null || isCompatible(rn.children, sec, parent)) {\n\t\t\t\tNode changed = replace(sec, rn.children, delta);\n\t\t\t\t// the line index has already been updated above, so it reflects\n\t\t\t\t// the new document - expand the actually inserted text\n\t\t\t\t// (offset .. offset + inserted) to full line boundaries.\n\t\t\t\treturn expand(changed, offset, offset + inserted);\n\t\t\t}\n\t\t\tsec = parent;\n\t\t}\n\t}"
  reason: Return ModificationRange from update()
```
/answer 24f217ea-b7ef-4c1d-9c79-d7eab7a3fa46 allow
Text:
 Jetzt `MarkdownDocument.java` anpassen:
Control Request:
```yaml
id: aceae317-e2bc-4bfd-8015-2ae2da3353b1
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  old_text: "\tpublic Node find(int lo, int hi) {\n\t\treturn find(root, lo, hi);\n\t}"
  new_text: "\tpublic ModificationRange find(int lo, int hi) {\n\t\treturn expand(find(root, lo, hi), lo, hi);\n\t}\n\n\t/**\n\t * Expands [lo, hi) to full line boundaries (using the up to date\n\t * {@link #lines} index) and pairs the result with {@code node}.\n\t */\n\tprivate ModificationRange expand(Node node, int lo, int hi) {\n\t\tint docLen = lines.bufferLength();\n\t\tlo = clamp(lo, docLen);\n\t\thi = clamp(hi, docLen);\n\t\tif (hi < lo)\n\t\t\thi = lo;\n\n\t\tint startLine = lines.lineOfOffset(lo);\n\t\tint endLine = hi > lo ? lines.lineOfOffset(hi - 1) : startLine;\n\t\tint start = lines.lineStartOffset(startLine);\n\t\tint end = lines.lineEndOffset(endLine);\n\t\treturn new ModificationRange(node, start, end);\n\t}\n\n\tprivate static int clamp(int value, int max) {\n\t\treturn Math.max(0, Math.min(value, max));\n\t}"
  reason: Return ModificationRange from find() and add expand helper
```
/answer aceae317-e2bc-4bfd-8015-2ae2da3353b1 allow
Control Request:
```yaml
id: 1eb9b73c-ce64-4870-8026-0d77458bc2ae
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/ISpellChecker.java
  mode: replace
  content: "package xy.ai.workbench.editor;\n\nimport org.eclipse.jface.text.IDocument;\n\nimport xy.ai.workbench.editor.mdast.ModificationRange;\n\npublic interface ISpellChecker {\n\n\tpublic void onDocumentChanged(IDocument document);\n\n\tpublic void reconcile(ModificationRange range);\n}\n"
  reason: Change reconcile() to take ModificationRange
```
/answer 1eb9b73c-ce64-4870-8026-0d77458bc2ae allow
Control Request:
```yaml
id: aa03f03b-1f72-42ed-8c4b-163f710c9d91
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  mode: replace
  content: "package xy.ai.workbench.editor.spellcheck;\n\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.source.ISourceViewer;\n\nimport xy.ai.workbench.editor.EditorManager;\nimport xy.ai.workbench.editor.ISpellChecker;\nimport xy.ai.workbench.editor.mdast.ModificationRange;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\n\npublic class SpellCheckReconciler implements ISpellChecker {\n\n\tprivate final SpellingStrategy strategy;\n\n\tpublic SpellCheckReconciler(ISourceViewer sourceViewer, EditorManager manager) {\n\t\tthis.strategy = new SpellingStrategy(sourceViewer);\n\t\tmanager.setSpellChecker(this);\n\t}\n\n\t@Override\n\tpublic void onDocumentChanged(IDocument document) {\n\t\tstrategy.setDocument(document);\n\t}\n\n\t@Override\n\tpublic void reconcile(ModificationRange range) {\n\t\t// The node still needs to be treated as invalidated as a whole (e.g. for\n\t\t// presentation invalidation, done by the caller), but the actual, costly\n\t\t// spell checking work is restricted to the modified lines\n\t\t// [range.getStart(), range.getEnd()). Everything else inside the node is\n\t\t// left untouched - existing annotations there stay valid.\n\t\treconcileLeaves(range.getNode(), range.getStart(), range.getEnd());\n\t}\n\n\tprivate void reconcileLeaves(Node node, int rangeStart, int rangeEnd) {\n\t\tint nodeStart = node.getOffset();\n\t\tint nodeEnd = node.getEndOffset();\n\t\tif (nodeEnd <= rangeStart || nodeStart >= rangeEnd)\n\t\t\treturn; // outside the modified lines - ignore, don't touch existing annotations\n\n\t\tint start = Math.max(nodeStart, rangeStart);\n\t\tint end = Math.min(nodeEnd, rangeEnd);\n\n\t\tif (!node.enableSpellcheck)\n\t\t\tstrategy.clear(node, start, end);\n\t\telse if (node.children.isEmpty())\n\t\t\tstrategy.reconcile(node, start, end);\n\t\telse\n\t\t\tfor (Node child : node.children)\n\t\t\t\treconcileLeaves(child, rangeStart, rangeEnd);\n\t}\n}\n"
  reason: Rewrite reconciler to restrict work to modified line range
```
/answer aa03f03b-1f72-42ed-8c4b-163f710c9d91 allow
Text:
 Jetzt der Import hinzufügen (ModificationRange liegt im selben Package, kein Import nötig). Nun `ISpellChecker.java`:
Control Request:
```yaml
id: 65b063f6-c1d0-46ae-89db-fb51eed668a9
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellingStrategy.java
  old_text: "\tpublic void reconcile(Node node) {\n\t\tif (doc == null)\n\t\t\treturn;\n\n\t\tfinal String text = doc.get();\n\t\tfinal int docLength = text.length();\n\n\t\tint offset = node.getOffset();\n\t\tint length = Math.max(node.length(), 1);\n\n\t\t// Expand the dirty range to full line boundaries.\n\t\tint start = Math.min(offset, docLength);\n\t\tint end = Math.min(start + length, docLength);\n\t\tif (end - start > LIMIT)\n\t\t\treturn;\n\n\t\twhile (start > 0 && text.charAt(start - 1) != '\\n')\n\t\t\tstart--;\n\t\twhile (end < docLength && text.charAt(end) != '\\n')\n\t\t\tend++;\n\n\t\tfinal int regionOffset = start;\n\t\tfinal String regionText = text.substring(start, end);\n\n\t\tList<SpellingProblem> problems = client.check(regionText);\n\n\t\t// LT offsets are relative to regionText – shift them to document offsets.\n\t\tList<SpellingProblem> valid = new ArrayList<>();\n\t\tfor (SpellingProblem p : problems) {\n\t\t\tint absOffset = p.getOffset() + regionOffset;\n\t\t\tif (absOffset >= 0 && absOffset + p.getLength() <= docLength)\n\t\t\t\tvalid.add(new SpellingProblem(absOffset, p.getLength(), p.getMessage(), p.getSuggestions()));\n\t\t}\n\n\t\tfinal int checkedOffset = regionOffset;\n\t\tfinal int checkedLength = end - start;\n\t\tviewer.getTextWidget().getDisplay().syncExec(() -> applyAnnotations(valid, checkedOffset, checkedLength));\n\t}\n\n\tpublic void clear(Node node) {\n\t\tif (doc == null)\n\t\t\treturn;\n\t\tint docLength = doc.getLength();\n\t\tint offset = node.getOffset();\n\t\tint length = Math.max(node.length(), 1);\n\t\tint start = Math.max(0, Math.min(offset, docLength));\n\t\tint end = Math.max(start, Math.min(start + length, docLength));\n\t\tfinal int clearedOffset = start;\n\t\tfinal int clearedLength = end - start;\n\t\tviewer.getTextWidget().getDisplay()\n\t\t\t\t.syncExec(() -> applyAnnotations(new ArrayList<>(), clearedOffset, clearedLength));\n\t}"
  new_text: "\t/**\n\t * (Re-)checks the given node's contribution to the already line-expanded\n\t * range [rangeStart, rangeEnd) - the caller is expected to have restricted\n\t * this to the modified lines, so only that much smaller region is sent to\n\t * the (potentially slow) spelling backend, instead of the node's full text.\n\t */\n\tpublic void reconcile(Node node, int rangeStart, int rangeEnd) {\n\t\tif (doc == null)\n\t\t\treturn;\n\n\t\tfinal int docLength = doc.getLength();\n\t\tint start = clamp(rangeStart, docLength);\n\t\tint end = clamp(rangeEnd, docLength);\n\t\tif (end <= start)\n\t\t\treturn;\n\t\tif (end - start > LIMIT)\n\t\t\treturn;\n\n\t\tfinal int regionOffset = start;\n\t\tfinal String regionText = textAt(start, end - start);\n\t\tif (regionText == null)\n\t\t\treturn;\n\n\t\tList<SpellingProblem> problems = client.check(regionText);\n\n\t\t// LT offsets are relative to regionText – shift them to document offsets.\n\t\tList<SpellingProblem> valid = new ArrayList<>();\n\t\tfor (SpellingProblem p : problems) {\n\t\t\tint absOffset = p.getOffset() + regionOffset;\n\t\t\tif (absOffset >= 0 && absOffset + p.getLength() <= docLength)\n\t\t\t\tvalid.add(new SpellingProblem(absOffset, p.getLength(), p.getMessage(), p.getSuggestions()));\n\t\t}\n\n\t\tfinal int checkedOffset = start;\n\t\tfinal int checkedLength = end - start;\n\t\tviewer.getTextWidget().getDisplay().syncExec(() -> applyAnnotations(valid, checkedOffset, checkedLength));\n\t}\n\n\t/** Clears annotations in [rangeStart, rangeEnd), restricted to the document bounds. */\n\tpublic void clear(Node node, int rangeStart, int rangeEnd) {\n\t\tif (doc == null)\n\t\t\treturn;\n\t\tint docLength = doc.getLength();\n\t\tint start = clamp(rangeStart, docLength);\n\t\tint end = clamp(rangeEnd, docLength);\n\t\tif (end <= start)\n\t\t\treturn;\n\t\tfinal int clearedOffset = start;\n\t\tfinal int clearedLength = end - start;\n\t\tviewer.getTextWidget().getDisplay()\n\t\t\t\t.syncExec(() -> applyAnnotations(new ArrayList<>(), clearedOffset, clearedLength));\n\t}\n\n\tprivate static int clamp(int value, int max) {\n\t\treturn Math.max(0, Math.min(value, max));\n\t}"
  reason: Restrict reconcile/clear to explicit line-expanded ranges
```
/answer 65b063f6-c1d0-46ae-89db-fb51eed668a9 allow
Text:
 Jetzt `SpellCheckReconciler.java`:
Control Request:
```yaml
id: 8e3d6610-eecd-43b7-975b-37aa0309ca67
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellingStrategy.java
  reason: verify final file consistency
```
/answer 8e3d6610-eecd-43b7-975b-37aa0309ca67 allow
Text:
 Nun `SpellingStrategy.java` anpassen (reconcile/clear mit expliziten Range-Parametern):
Control Request:
```yaml
id: 31eb9e81-90cb-464a-abdf-a8d575a51b6a
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
  old_text: "import java.util.ArrayList;\nimport java.util.Comparator;\nimport java.util.Iterator;\nimport java.util.List;\nimport java.util.concurrent.CopyOnWriteArrayList;\nimport java.util.concurrent.ExecutorService;\nimport java.util.concurrent.Executors;\n\nimport org.eclipse.jface.text.DocumentEvent;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IDocumentListener;\nimport org.eclipse.jface.text.ITextInputListener;\nimport org.eclipse.jface.text.ITextViewer;\nimport org.eclipse.jface.text.ITextViewerExtension2;\nimport org.eclipse.swt.widgets.Display;\n\nimport xy.ai.workbench.editor.mdast.MarkdownDocument;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\nimport xy.ai.workbench.tools.RegionList;\n\npublic class EditorManager {\n\n\tpublic static final int DEBOUNCE_DELAY_MS = 500;\n\n\tprivate final List<IManagerListener> listeners = new CopyOnWriteArrayList<>();\n\n\tprivate final ExecutorService background = Executors.newSingleThreadExecutor(r -> {\n\t\tThread t = new Thread(r, \"EditorManager-Background\");\n\t\tt.setDaemon(true);\n\t\treturn t;\n\t});"
  new_text: "import java.util.ArrayList;\nimport java.util.Comparator;\nimport java.util.Iterator;\nimport java.util.List;\nimport java.util.concurrent.CopyOnWriteArrayList;\nimport java.util.concurrent.SynchronousQueue;\nimport java.util.concurrent.ThreadFactory;\nimport java.util.concurrent.ThreadPoolExecutor;\nimport java.util.concurrent.TimeUnit;\nimport java.util.concurrent.atomic.AtomicInteger;\n\nimport org.eclipse.jface.text.DocumentEvent;\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.IDocumentListener;\nimport org.eclipse.jface.text.ITextInputListener;\nimport org.eclipse.jface.text.ITextViewer;\nimport org.eclipse.jface.text.ITextViewerExtension2;\nimport org.eclipse.swt.widgets.Display;\n\nimport xy.ai.workbench.editor.mdast.MarkdownDocument;\nimport xy.ai.workbench.editor.mdast.ModificationRange;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\nimport xy.ai.workbench.tools.RegionList;\n\npublic class EditorManager {\n\n\tpublic static final int DEBOUNCE_DELAY_MS = 500;\n\n\t/** Idle non-core threads are kept around this long before terminating. */\n\tprivate static final long IDLE_KEEP_ALIVE_MS = 30_000L;\n\n\tprivate final List<IManagerListener> listeners = new CopyOnWriteArrayList<>();\n\n\tprivate final ThreadPoolExecutor background = createBackgroundPool();\n\n\tprivate static ThreadPoolExecutor createBackgroundPool() {\n\t\tint maxThreads = Math.max(1, Math.min(16, Runtime.getRuntime().availableProcessors()));\n\t\tThreadFactory factory = new ThreadFactory() {\n\t\t\tprivate final AtomicInteger counter = new AtomicInteger();\n\n\t\t\t@Override\n\t\t\tpublic Thread newThread(Runnable r) {\n\t\t\t\tThread t = new Thread(r, \"EditorManager-Background-\" + counter.incrementAndGet());\n\t\t\t\tt.setDaemon(true);\n\t\t\t\treturn t;\n\t\t\t}\n\t\t};\n\t\t// corePoolSize=1: exactly one thread is always kept ready (never times out).\n\t\t// maximumPoolSize=maxThreads: scales up to the number of CPU cores (capped\n\t\t// at 16) under load, so reconcile jobs for independent regions can run in\n\t\t// parallel. A SynchronousQueue hands tasks directly to a (possibly newly\n\t\t// started) thread instead of queuing them, so the pool actually grows\n\t\t// under concurrent load; idle non-core threads terminate after the\n\t\t// keep-alive, so at most one thread (the permanent core thread) stays\n\t\t// idle at any given time.\n\t\tThreadPoolExecutor executor = new ThreadPoolExecutor(1, maxThreads, IDLE_KEEP_ALIVE_MS, TimeUnit.MILLISECONDS,\n\t\t\t\tnew SynchronousQueue<>(), factory, new ThreadPoolExecutor.CallerRunsPolicy());\n\t\texecutor.allowCoreThreadTimeOut(false);\n\t\treturn executor;\n\t}"
  reason: "Replace single background thread with a bounded, parallel thread pool"
```
/answer 31eb9e81-90cb-464a-abdf-a8d575a51b6a allow
Control Request:
```yaml
id: 9df52f9f-50cf-43a1-b857-d8422ff698c0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
  reason: Review current full state after edits
```
/answer 9df52f9f-50cf-43a1-b857-d8422ff698c0 allow
ReasoningToken: 76
Text:
 Gut, das sieht konsistent aus. Jetzt EditorManager.java aktualisieren (Thread-Pool + ModificationRange).
Control Request:
```yaml
id: a55436b7-2a72-4f1d-b0e2-1dc0b93bd16f
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
  old_text: "\tpublic void setSpellChecker(ISpellChecker spellChecker) {\n\t\tthis.spell = spellChecker;\n\t\tif (spellChecker != null && doc != null && ast != null) {\n\t\t\tspellChecker.onDocumentChanged(doc);\n\t\t\tupdate(ast.getRoot());\n\t\t}\n\t}"
  new_text: "\tpublic void setSpellChecker(ISpellChecker spellChecker) {\n\t\tthis.spell = spellChecker;\n\t\tif (spellChecker != null && doc != null && ast != null) {\n\t\t\tspellChecker.onDocumentChanged(doc);\n\t\t\t// a freshly attached spell checker has to check the whole document.\n\t\t\tupdate(new ModificationRange(ast.getRoot(), 0, doc.getLength()));\n\t\t}\n\t}"
  reason: Wrap root node in a full-document ModificationRange for the initial spell check
```
/answer a55436b7-2a72-4f1d-b0e2-1dc0b93bd16f allow
Control Request:
```yaml
id: bfe12c5e-d374-46b4-872b-585ccb9fcb20
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
  old_text: "\tprivate void update(Node node) {\n\t\tfor (IManagerListener l : listeners)\n\t\t\tl.onAstUpdated(node);\n\t\tif (viewer instanceof ITextViewerExtension2 ext2)\n\t\t\ttry {\n\t\t\t\text2.invalidateTextPresentation(node.getOffset(), node.length());\n\t\t\t} catch (IllegalArgumentException e) {\n\t\t\t\t// region outside the (possibly just replaced) document - ignore.\n\t\t\t}\n\n\t\tif (spell != null && !background.isShutdown())\n\t\t\tbackground.execute(() -> {\n\t\t\t\tspell.reconcile(node);\n\t\t\t\tviewer.getTextWidget().getDisplay().asyncExec(() -> {\n\t\t\t\t\tif (viewer instanceof ITextViewerExtension2 ext2)\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\text2.invalidateTextPresentation(node.getOffset(), node.length());\n\t\t\t\t\t\t} catch (IllegalArgumentException e) {\n\t\t\t\t\t\t\t// region outside the (possibly just replaced) document - ignore.\n\t\t\t\t\t\t}\n\t\t\t\t});\n\t\t\t});\n\t}"
  new_text: "\tprivate void update(ModificationRange range) {\n\t\tNode node = range.getNode();\n\t\tfor (IManagerListener l : listeners)\n\t\t\tl.onAstUpdated(node);\n\t\tif (viewer instanceof ITextViewerExtension2 ext2)\n\t\t\ttry {\n\t\t\t\text2.invalidateTextPresentation(node.getOffset(), node.length());\n\t\t\t} catch (IllegalArgumentException e) {\n\t\t\t\t// region outside the (possibly just replaced) document - ignore.\n\t\t\t}\n\n\t\t// The whole node's representation was invalidated above; the (costly)\n\t\t// actual spell check is restricted to range's modified line range and\n\t\t// may run in parallel with other, independent reconcile jobs on the\n\t\t// background pool.\n\t\tif (spell != null && !background.isShutdown())\n\t\t\tbackground.execute(() -> {\n\t\t\t\tspell.reconcile(range);\n\t\t\t\tviewer.getTextWidget().getDisplay().asyncExec(() -> {\n\t\t\t\t\tif (viewer instanceof ITextViewerExtension2 ext2)\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\text2.invalidateTextPresentation(node.getOffset(), node.length());\n\t\t\t\t\t\t} catch (IllegalArgumentException e) {\n\t\t\t\t\t\t\t// region outside the (possibly just replaced) document - ignore.\n\t\t\t\t\t\t}\n\t\t\t\t});\n\t\t\t});\n\t}"
  reason: update() now consumes a ModificationRange and passes it through to the spell checker
```
/answer bfe12c5e-d374-46b4-872b-585ccb9fcb20 allow
Control Request:
```yaml
id: fcec6447-4cc4-43f3-a036-bc9bf3fb35e0
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
  old_text: "\t\tpublic void inputDocumentChanged(IDocument oldInput, IDocument newInput) {\n\t\t\tdoc = newInput;\n\t\t\tNode initial = null;\n\t\t\tif (newInput != null) {"
  new_text: "\t\tpublic void inputDocumentChanged(IDocument oldInput, IDocument newInput) {\n\t\t\tdoc = newInput;\n\t\t\tModificationRange initial = null;\n\t\t\tif (newInput != null) {"
  reason: Fix type of initial var to ModificationRange
```
/answer fcec6447-4cc4-43f3-a036-bc9bf3fb35e0 allow
Control Request:
```yaml
id: 011498e3-4050-45aa-b9ee-838379a4a2fe
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/MarkdownOutlinePage.java
  old_text: "\t\tNode node = ast.find(offset, offset);\n\t\tif (node == null || node.instance == Elements.ROOT)"
  new_text: "\t\tNode node = ast.find(offset, offset).getNode();\n\t\tif (node == null || node.instance == Elements.ROOT)"
  reason: Adapt to ModificationRange return type of find()
```
/answer 011498e3-4050-45aa-b9ee-838379a4a2fe allow
Control Request:
```yaml
id: 711d4017-4207-4506-ac39-5732edfac318
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java
  old_text: "\t\tNode governing = ast.find(offset, offset + length);\n\t\tcollect(document, governing, offset, offset + length);"
  new_text: "\t\tNode governing = ast.find(offset, offset + length).getNode();\n\t\tcollect(document, governing, offset, offset + length);"
  reason: Adapt to ModificationRange return type of find()
```
/answer 711d4017-4207-4506-ac39-5732edfac318 allow
Control Request:
```yaml
id: 9cebea64-be48-4762-9a04-d46c7e4dbdad
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISourceViewerConfiguration.java
  old_text: "\t\t\tvar node = updateManager.getAst().find(e.getOffset(), e.getOffset() + e.getLength());"
  new_text: "\t\t\tvar node = updateManager.getAst().find(e.getOffset(), e.getOffset() + e.getLength()).getNode();"
  reason: Adapt to ModificationRange return type of find()
```
/answer 9cebea64-be48-4762-9a04-d46c7e4dbdad allow
Control Request:
```yaml
id: 6f9af021-20f6-4e3b-9d28-7063874ea388
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: ls; find . -maxdepth 1 -iname "pom.xml" -o -iname "build.xml"
  reason: check build tooling
```
/answer 6f9af021-20f6-4e3b-9d28-7063874ea388 deny es gibt build.sh
Control Request:
```yaml
id: 55fa8b98-e9b0-429e-af60-7f0be1a0802f
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench
  reason: check project root for build files
```
/answer 55fa8b98-e9b0-429e-af60-7f0be1a0802f deny es gibt build.sh
Control Request:
```yaml
id: dff6ae7b-8776-42d3-85fa-e621fcbc087c
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/build.sh
  reason: Check build script to compile project
```
/answer dff6ae7b-8776-42d3-85fa-e621fcbc087c deny führe es einfach aus
Control Request:
```yaml
id: 2981c238-2c9a-446a-95e0-14c7ff235034
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: ./build.sh 2>&1 | tail -100
  reason: Build project to check compile errors
```
/answer 2981c238-2c9a-446a-95e0-14c7ff235034 allow
Control Request:
```yaml
id: ad970d14-5214-4430-99e0-b58bccbc09b0
result:
  content:
  - type: text
    text: Full output written to file 3120 characters). Reduce to relevant excerpts (e.g. via grep/head/tail).
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-t0fe5con.log
```
/answer ad970d14-5214-4430-99e0-b58bccbc09b0 allow
Control Request:
```yaml
id: fb7cd4f6-665e-4e7a-abaa-84747f023a07
toolName: read-file
arguments:
  path: /tmp/mcpc-stdout-t0fe5con.log
  reason: view build output
```
/answer fb7cd4f6-665e-4e7a-abaa-84747f023a07 allow
ReasoningToken: 54
Control Request:
```yaml
id: 6d585f0b-85e0-4333-b4d2-6b1bc11c17f0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  min_line: 130
  max_line: 160
  reason: check findForUpdate conflict
```
/answer 6d585f0b-85e0-4333-b4d2-6b1bc11c17f0 allow
Control Request:
```yaml
id: c1839223-1802-41ba-94a9-c8f59e190bf3
result:
  structuredContent:
    checksum: f70ac9aa1d2f98a7d75f369153c248e723ac96207bac9b2aae6faaa0837c17de
    content: "\n\t\tint startLine = lines.lineOfOffset(lo);\n\t\tint endLine = hi > lo ? lines.lineOfOffset(hi - 1) : startLine;\n\t\tint start = lines.lineStartOffset(startLine);\n\t\tint end = lines.lineEndOffset(endLine);\n\t\treturn new ModificationRange(node, start, end);\n\t}\n\n\tprivate static int clamp(int value, int max) {\n\t\treturn Math.max(0, Math.min(value, max));\n\t}\n\n\tprivate Node findForUpdate(int lo, int hi) {\n\t\tNode tail = lastLeaf(root);\n\t\tif (tail != root && tail.getEndOffset() <= lo)\n\t\t\treturn tail;\n\t\treturn find(lo, hi);\n\t}\n\n\tprivate Node lastLeaf(Node node) {\n\t\twhile (!node.children.isEmpty())\n\t\t\tnode = node.children.get(node.children.size() - 1);\n\t\treturn node;\n\t}\n\n\tprivate Node find(Node node, int lo, int hi) {\n\t\tfor (Node child : node.children) {\n\t\t\tint cs = child.getOffset();\n\t\t\tif (cs <= lo && hi <= cs + child.length())\n\t\t\t\treturn find(child, lo, hi);\n\t\t}\n"
```
/answer c1839223-1802-41ba-94a9-c8f59e190bf3 allow
Text:
 Jetzt die restlichen Stellen im `EditorManager` anpassen: `uninstall()`, `setSpellChecker`, `update(Node)`, `Flush.run()`, `TextInput.inputDocumentChanged`.
Control Request:
```yaml
id: c0c505e5-bee3-4108-a23d-81da34983ac6
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  old_text: "\tprivate Node findForUpdate(int lo, int hi) {\n\t\tNode tail = lastLeaf(root);\n\t\tif (tail != root && tail.getEndOffset() <= lo)\n\t\t\treturn tail;\n\t\treturn find(lo, hi);\n\t}"
  new_text: "\tprivate Node findForUpdate(int lo, int hi) {\n\t\tNode tail = lastLeaf(root);\n\t\tif (tail != root && tail.getEndOffset() <= lo)\n\t\t\treturn tail;\n\t\treturn find(root, lo, hi);\n\t}"
  reason: Fix ambiguous call to internal Node-returning find()
```
/answer c0c505e5-bee3-4108-a23d-81da34983ac6 allow
Control Request:
```yaml
id: c99d8bc0-fbf0-4a2c-b6ff-6ab68eead611
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: ./build.sh 2>&1 | tail -100
  reason: Rebuild after fix
```
/answer c99d8bc0-fbf0-4a2c-b6ff-6ab68eead611 allow
ReasoningToken: 409
Control Request:
```yaml
id: 7945212f-c901-4f09-b902-fa62993ec158
result:
  structuredContent:
    stdout: |
      [INFO] Scanning for projects...
      [INFO]
      [INFO] -----------------------< xy.ai:xy.ai.workbench >------------------------
      [INFO] Building XY.AI.Workbench 260713.1.0
      [INFO]   from pom.xml
      [INFO] --------------------------------[ jar ]---------------------------------
      [INFO]
      [INFO] --- clean:3.2.0:clean (default-clean) @ xy.ai.workbench ---
      [INFO] Deleting /home/user/xyan/xy.ai.workbench/bin
      [INFO]
      [INFO] --- resources:3.3.1:resources (default-resources) @ xy.ai.workbench ---
      [INFO] skip non existing resourceDirectory /home/user/xyan/xy.ai.workbench/src/main/resources
      [INFO]
      [INFO] --- compiler:3.13.0:compile (default-compile) @ xy.ai.workbench ---
      [INFO] Recompiling the module because of changed source code.
      [INFO] Compiling 126 source files with javac [forked debug release 21] to bin
      [INFO]
      [INFO] --- resources:3.3.1:testResources (default-testResources) @ xy.ai.workbench ---
      [INFO] skip non existing resourceDirectory /home/user/xyan/xy.ai.workbench/src/test/resources
      [INFO]
      [INFO] --- compiler:3.13.0:testCompile (default-testCompile) @ xy.ai.workbench ---
      [INFO] No sources to compile
      [INFO]
      [INFO] --- surefire:3.5.4:test (default-test) @ xy.ai.workbench ---
      [INFO] No tests to run.
      [INFO]
      [INFO] --- jar:3.4.1:jar (default-jar) @ xy.ai.workbench ---
      [INFO] Building jar: /home/user/xyan/xy.ai.workbench/bin/xy.ai.workbench.jar
      [INFO]
      [INFO] --- resources:3.3.1:copy-resources (assemble-plugin) @ xy.ai.workbench ---
      [INFO] Copying 41 resources from  to bin/xy.ai.workbench
      [INFO] Copying 235 resources from bin to bin/xy.ai.workbench
      [INFO] ------------------------------------------------------------------------
      [INFO] BUILD SUCCESS
      [INFO] ------------------------------------------------------------------------
      [INFO] Total time:  6.380 s
      [INFO] Finished at: 2026-08-01T09:13:51+02:00
      [INFO] ------------------------------------------------------------------------
```
/answer 7945212f-c901-4f09-b902-fa62993ec158 allow
Control Request:
```yaml
id: 771de584-9f30-4eb9-9421-049574b52b64
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -path ./bin -prune -o -iname "*Test*.java" -print
  reason: Look for existing tests referencing changed APIs
```
/answer 771de584-9f30-4eb9-9421-049574b52b64 deny es gibt keine Tests
Control Request:
```yaml
id: f12129a1-713f-43f3-9b19-25f685d8eff7
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  reason: Final review
```
/answer f12129a1-713f-43f3-9b19-25f685d8eff7 allow
ReasoningToken: 39
Control Request:
```yaml
id: f0003b6f-17fa-4303-b70b-dec276966a4e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
  reason: Final review of full file
```
/answer f0003b6f-17fa-4303-b70b-dec276966a4e allow
Text:
 Jetzt die übrigen Aufrufer von `ast.find(...)` anpassen:
ReasoningToken: 102
Text:
 Nun kompilieren wir das Projekt, um Fehler zu finden.
ReasoningToken: 19
ReasoningToken: 59
ReasoningToken: 66
Text:
 `findForUpdate` calls `find(lo, hi)` (the public one now returning ModificationRange) instead of the private `find(Node, int, int)`. Fix to call `find(root, lo, hi)`.
Text:
 Build erfolgreich. Nun prüfe ich noch, ob es Tests gibt, die die alte API nutzen (z.B. Unit-Tests für MarkdownDocument/SpellCheck).
ReasoningToken: 85
Text:
 Sieht gut aus. Abschließend ein Blick auf die vollständige EditorManager.java und SpellCheckReconciler.java zur Kontrolle.
Alles konsistent und der Build läuft erfolgreich durch. Zusammenfassung der Änderungen:

## 1. Thread-Pool statt Single-Thread (`EditorManager.java`)
- `Executors.newSingleThreadExecutor()` wurde durch einen `ThreadPoolExecutor` ersetzt: `corePoolSize=1` (immer ein bereitstehender Thread, `allowCoreThreadTimeOut(false)`), `maximumPoolSize = min(16, verfügbare CPU-Kerne)`.
- `SynchronousQueue` als Übergabe-Queue sorgt dafür, dass bei paralleler Auslastung tatsächlich neue Threads bis zum Maximum gestartet werden, statt Aufgaben zu puffern.
- Überzählige (nicht-Core-)Threads terminieren nach 30s Idle-Zeit automatisch → nie mehr als ein idler Thread (der permanente Core-Thread).
- `reconcile()`-Aufrufe für unabhängige Regionen können jetzt parallel laufen.

## 2. `ModificationRange` (neu) + `find()`/`update()` in `MarkdownDocument.java`
- Neue Klasse `ModificationRange` (Node + auf ganze Zeilen expandierte [start,end)-Subrange), Nutzung von `LineIndex` (`lineOfOffset`, `lineStartOffset`, `lineEndOffset`), die bei `update()` ohnehin schon aktualisiert wird.
- `find()` und `update()` geben jetzt `ModificationRange` statt `Node` zurück.
- Alle Aufrufer (`MarkdownOutlinePage`, `AIRuleScanner`, `AISourceViewerConfiguration`, `EditorManager`) wurden auf `.getNode()` angepasst.

## 3. Spellcheck nutzt die Zeilen-Subrange
- `ISpellChecker.reconcile(ModificationRange)` statt `reconcile(Node)`.
- `SpellCheckReconciler` durchläuft den AST-Teilbaum des invalidierten Blocks, überspringt aber Zweige, die die geänderten Zeilen gar nicht berühren (bestehende Annotations dort bleiben unangetastet), und prüft/leert nur die tatsächliche Schnittmenge Knoten∩Zeilenbereich.
- `SpellingStrategy.reconcile(Node, start, end)`/`clear(Node, start, end)` bekommen jetzt die exakte, schon zeilen-expandierte Range statt selbst zu expandieren – dadurch wird nur der wirklich geänderte Text an LanguageTool geschickt, während Folge-Annotationen außerhalb dieses Bereichs unverändert (mit von JFace automatisch nachgeführten Offsets) bestehen bleiben.

Build (`./build.sh`) läuft erfolgreich durch (BUILD SUCCESS).
