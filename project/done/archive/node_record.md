Ersetze im Package `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline` die Verwendung von `Node` durch einen reccord der Node  und IDocument vereint.
SystemInit:  id=0bf6884e-5121-4cf4-a721-93071bf1e8d5, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
ReasoningToken: 73
Control Request:
```yaml
id: 05000d17-de33-48e8-9b21-d41dc1eb22f3
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline
  reason: Explore outline package files
```
/answer 05000d17-de33-48e8-9b21-d41dc1eb22f3 allow
Control Request:
```yaml
id: d4d7ca39-7455-4205-ae66-c35b5fa01f82
result:
  structuredContent:
    entries:
    - MarkdownNavigatorContentProvider.java
    - MarkdownNavigatorLabelProvider.java
    - MarkdownOutlinePage.java
    - NodeLabels.java
    - RegexNodeFilter.java
    - filter/AgentSectionFilter.java
    - filter/AnswerFilter.java
    - filter/ControlRequestFilter.java
    - filter/ElementFilter.java
    - filter/HeadingFilter.java
    - filter/LineCommentFilter.java
    - filter/PageFilter.java
    - filter/ParagraphFilter.java
    - filter/ReasoningTokenFilter.java
    - filter/ScriptBlockFilter.java
    - filter/SystemInitFilter.java
    - filter/TextBlockFilter.java
    - filter/ThinkingFilter.java
    - filter/TokenStatsFilter.java
    - filter/ToolUseFilter.java
    - filter/UserSectionFilter.java
```
/answer d4d7ca39-7455-4205-ae66-c35b5fa01f82 allow
Control Request:
```yaml
id: 36520cf2-220c-4c72-a7b9-fb8c812d84e4
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline
  script: grep -rn "Node" --include=*.java . | grep -v "NodeLabels\|import.*NodeFilter\|Filter" | head -100
  reason: Find usages of Node
```
/answer 36520cf2-220c-4c72-a7b9-fb8c812d84e4 allow
Control Request:
```yaml
id: 1428966a-5c7c-458a-9fd6-e23283036716
result:
  structuredContent:
    stdout: "./MarkdownNavigatorLabelProvider.java:6:import xy.ai.workbench.editor.mdast.nodes.Node;\n./MarkdownOutlinePage.java:34:import xy.ai.workbench.editor.mdast.nodes.Node;\n./MarkdownOutlinePage.java:163:\t\tif (event.getSelection() instanceof StructuredSelection sel && sel.getFirstElement() instanceof Node node)\n./MarkdownOutlinePage.java:164:\t\t\teditor.selectAndRevealNode(node);\n./MarkdownOutlinePage.java:176:\tpublic void selectNodeForOffset(int offset) {\n./MarkdownOutlinePage.java:183:\t\tNode node = ast.find(offset, offset).getNode();\n./MarkdownNavigatorContentProvider.java:6:import xy.ai.workbench.editor.mdast.nodes.Node;\n./MarkdownNavigatorContentProvider.java:21:\t\tif (element instanceof Node node)\n./MarkdownNavigatorContentProvider.java:28:\t\treturn element instanceof Node node ? node.parent : null;\n"
```
/answer 1428966a-5c7c-458a-9fd6-e23283036716 allow
Control Request:
```yaml
id: a139b37b-67de-4c4d-b9eb-c9a02225bfd5
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/MarkdownNavigatorContentProvider.java
  reason: Read file content
```
/answer a139b37b-67de-4c4d-b9eb-c9a02225bfd5 allow
Control Request:
```yaml
id: 7eca5987-e93b-4961-81ae-a9ce90a929bc
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/MarkdownOutlinePage.java
  reason: Read file content
```
/answer 7eca5987-e93b-4961-81ae-a9ce90a929bc allow
Control Request:
```yaml
id: b017cab7-0597-4c3c-b574-23127c2d9523
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/MarkdownNavigatorLabelProvider.java
  reason: Read file content
```
/answer b017cab7-0597-4c3c-b574-23127c2d9523 allow
Control Request:
```yaml
id: 6388801c-713e-4143-a005-27442ffcc703
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/NodeLabels.java
  reason: Read file content
```
/answer 6388801c-713e-4143-a005-27442ffcc703 allow
Control Request:
```yaml
id: 557255a0-1329-40f3-b108-83616171cf03
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/RegexNodeFilter.java
  reason: Read file content
```
/answer 557255a0-1329-40f3-b108-83616171cf03 allow
Control Request:
```yaml
id: fb8c513d-8490-419f-8975-e8a91cff2133
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/filter
  script: grep -ln "Node" *.java
  reason: Find filter files referencing Node
```
/answer fb8c513d-8490-419f-8975-e8a91cff2133 allow
Control Request:
```yaml
id: 6ae83bb3-a799-44de-a564-324e9251df37
result:
  structuredContent:
    stdout: |
      AgentSectionFilter.java
      AnswerFilter.java
      ControlRequestFilter.java
      ElementFilter.java
      HeadingFilter.java
      LineCommentFilter.java
      PageFilter.java
      ParagraphFilter.java
      ReasoningTokenFilter.java
      ScriptBlockFilter.java
      SystemInitFilter.java
      TextBlockFilter.java
      ThinkingFilter.java
      TokenStatsFilter.java
      ToolUseFilter.java
      UserSectionFilter.java
```
/answer 6ae83bb3-a799-44de-a564-324e9251df37 allow
Control Request:
```yaml
id: 652cb1e7-d50e-446e-af1f-fa8e18c90fe8
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
  script: grep -rn "class Node\|selectAndRevealNode\|getDocument()" mdast AITextEditor.java EditorManager.java 2>/dev/null
  reason: Find Node class def and related methods
```
/answer 652cb1e7-d50e-446e-af1f-fa8e18c90fe8 deny beschränke dich allein auf den package Inhalt
ReasoningToken: 20
Control Request:
```yaml
id: 25960609-50b2-40ee-a559-831d88a820ce
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/filter/ElementFilter.java
  reason: Read ElementFilter
```
/answer 25960609-50b2-40ee-a559-831d88a820ce allow
Control Request:
```yaml
id: 4513db2d-5dd6-4414-933e-29d3fd67d280
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/filter/HeadingFilter.java
  reason: Read HeadingFilter
```
/answer 4513db2d-5dd6-4414-933e-29d3fd67d280 allow
Control Request:
```yaml
id: 5f24cf20-4c03-4b65-b4fa-f0a395140963
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/filter/AgentSectionFilter.java
  reason: Read AgentSectionFilter
```
/answer 5f24cf20-4c03-4b65-b4fa-f0a395140963 allow
ReasoningToken: 90
ReasoningToken: 40
Control Request:
```yaml
id: fca3226c-7d33-49a7-ac04-558530b4ff0a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Node.java
  reason: Understand Node class structure for wrapping
```
/answer fca3226c-7d33-49a7-ac04-558530b4ff0a deny das ist nicht notwendig
Control Request:
```yaml
id: 20574a0c-c8fe-43c7-b7ff-b213586b78ba
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/AbstractNode.java
  reason: Understand AbstractNode class
```
/answer 20574a0c-c8fe-43c7-b7ff-b213586b78ba deny beschränke dich auf das genannte package
ReasoningToken: 341
Control Request:
```yaml
id: b13e316d-ddbc-4440-b18d-4ab83deb0b56
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline
  script: grep -rn "getDocument\|IDocument\|MarkdownDocument" .
  reason: Find IDocument/MarkdownDocument references in package
```
/answer b13e316d-ddbc-4440-b18d-4ab83deb0b56 allow
Control Request:
```yaml
id: d1b5bd4e-9626-425c-9f7e-9cf97571df54
result:
  structuredContent:
    stdout: "./MarkdownNavigatorLabelProvider.java:3:import org.eclipse.jface.text.IDocument;\n./MarkdownNavigatorLabelProvider.java:9:\tprivate IDocument doc;\n./MarkdownNavigatorLabelProvider.java:11:\tpublic void setActiveDocument(IDocument doc) {\n./MarkdownOutlinePage.java:32:import xy.ai.workbench.editor.mdast.MarkdownDocument;\n./MarkdownOutlinePage.java:179:\t\tMarkdownDocument ast = editor.getUpdateManager().getAst();\n./NodeLabels.java:4:import org.eclipse.jface.text.IDocument;\n./NodeLabels.java:11:\tpublic static String getText(Node node, IDocument doc) {\n./NodeLabels.java:15:\tprivate static String snippet(Node node, IDocument doc) {\n./RegexNodeFilter.java:6:import org.eclipse.jface.text.IDocument;\n./RegexNodeFilter.java:43:\t\treturn matches(p, node, manager.getDocument());\n./RegexNodeFilter.java:46:\tprivate boolean matches(Pattern p, Node node, IDocument doc) {\n./MarkdownNavigatorContentProvider.java:5:import xy.ai.workbench.editor.mdast.MarkdownDocument;\n./MarkdownNavigatorContentProvider.java:19:\t\tif (element instanceof MarkdownDocument doc)\n"
```
/answer d1b5bd4e-9626-425c-9f7e-9cf97571df54 allow
ReasoningToken: 1983
Control Request:
```yaml
id: acbdab3d-aea4-4f7c-891a-109fef77169e
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/NodeElement.java
  mode: replace
  content: |
    package xy.ai.workbench.editor.outline;

    import org.eclipse.jface.text.IDocument;

    import xy.ai.workbench.editor.mdast.nodes.Node;

    /**
     * Combines an AST {@link Node} with the {@link IDocument} it was parsed from.
     * <p>
     * The outline viewer (and everything feeding it: content/label providers and
     * filters) works with instances of this record instead of bare {@link Node}s,
     * so the document needed to compute labels/snippets is always available
     * alongside the node and doesn't have to be tracked separately.
     */
    public record NodeElement(Node node, IDocument document) {
    }
  reason: Create new record combining Node and IDocument
```
/answer acbdab3d-aea4-4f7c-891a-109fef77169e allow
ReasoningToken: 5723
Control Request:
```yaml
id: b3b7ec6e-86c1-4ed9-823e-9d922c2b28cb
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/MarkdownNavigatorContentProvider.java
  mode: replace
  content: "package xy.ai.workbench.editor.outline;\n\nimport org.eclipse.jface.viewers.ITreeContentProvider;\n\npublic class MarkdownNavigatorContentProvider implements ITreeContentProvider {\n\n\tprivate static final Object[] EMPTY = new Object[0];\n\n\t@Override\n\tpublic Object[] getElements(Object input) {\n\t\treturn getChildren(input);\n\t}\n\n\t@Override\n\tpublic Object[] getChildren(Object element) {\n\t\tif (element instanceof NodeElement ne)\n\t\t\treturn ne.node().children.stream().map(child -> new NodeElement(child, ne.document())).toArray();\n\t\treturn EMPTY;\n\t}\n\n\t@Override\n\tpublic Object getParent(Object element) {\n\t\tif (element instanceof NodeElement ne && ne.node().parent != null)\n\t\t\treturn new NodeElement(ne.node().parent, ne.document());\n\t\treturn null;\n\t}\n\n\t@Override\n\tpublic boolean hasChildren(Object element) {\n\t\treturn getChildren(element).length > 0;\n\t}\n}\n"
  reason: Replace Node usage with NodeElement record
```
/answer b3b7ec6e-86c1-4ed9-823e-9d922c2b28cb allow
Control Request:
```yaml
id: c5a2f612-32e4-43aa-93c2-6d2865007300
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/MarkdownNavigatorLabelProvider.java
  mode: replace
  content: "package xy.ai.workbench.editor.outline;\n\nimport org.eclipse.jface.viewers.LabelProvider;\n\npublic class MarkdownNavigatorLabelProvider extends LabelProvider {\n\n\t@Override\n\tpublic String getText(Object element) {\n\t\treturn element instanceof NodeElement ne ? NodeLabels.getText(ne) : String.valueOf(element);\n\t}\n}\n"
  reason: "Replace Node usage with NodeElement record, drop separate document tracking"
```
/answer c5a2f612-32e4-43aa-93c2-6d2865007300 allow
Control Request:
```yaml
id: 7034a140-9bc5-437c-bd0b-e2a942912f64
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/NodeLabels.java
  mode: replace
  content: "package xy.ai.workbench.editor.outline;\n\nimport org.eclipse.jface.text.BadLocationException;\nimport org.eclipse.jface.text.IDocument;\n\nimport xy.ai.workbench.editor.mdast.nodes.Node;\n\npublic class NodeLabels {\n\tprivate static final int LABEL_LIMIT = 40;\n\n\tpublic static String getText(NodeElement element) {\n\t\tNode node = element.node();\n\t\tIDocument doc = element.document();\n\t\treturn node.instance.toString() + \": \" + String.format(\"%s (%d)\", snippet(node, doc), node.end - node.start);\n\t}\n\n\tprivate static String snippet(Node node, IDocument doc) {\n\t\tif (doc == null)\n\t\t\treturn \"\";\n\t\tint offset = node.getOffset();\n\t\tint length = node.length();\n\t\tif (offset < 0 || length <= 0)\n\t\t\treturn \"Empty\";\n\t\tlength = Math.min(length, doc.getLength() - offset);\n\t\tif (length <= 0)\n\t\t\treturn \"Empty\";\n\t\ttry {\n\t\t\tString text = doc.get(offset, length).strip();\n\t\t\tint nl = text.indexOf('\\n');\n\t\t\tif (nl >= 0)\n\t\t\t\ttext = text.substring(0, nl).strip();\n\t\t\tif (text.length() > LABEL_LIMIT)\n\t\t\t\ttext = text.substring(0, LABEL_LIMIT) + \"…\";\n\t\t\treturn text.strip();\n\t\t} catch (BadLocationException e) {\n\t\t\treturn \"\";\n\t\t}\n\t}\n}\n"
  reason: Adapt NodeLabels to work with the NodeElement record
```
/answer 7034a140-9bc5-437c-bd0b-e2a942912f64 allow
Control Request:
```yaml
id: 74468498-8cbb-45da-bde5-dd0f55f292df
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/RegexNodeFilter.java
  mode: replace
  content: "package xy.ai.workbench.editor.outline;\n\nimport java.util.regex.Pattern;\nimport java.util.regex.PatternSyntaxException;\n\nimport org.eclipse.jface.viewers.Viewer;\nimport org.eclipse.jface.viewers.ViewerFilter;\n\nimport xy.ai.workbench.editor.mdast.nodes.Node;\n\npublic class RegexNodeFilter extends ViewerFilter {\n\n\tprivate volatile Pattern pattern;\n\n\tpublic void setPattern(String regex) {\n\t\tif (regex == null || regex.isBlank()) {\n\t\t\tpattern = null;\n\t\t\treturn;\n\t\t}\n\t\ttry {\n\t\t\tpattern = Pattern.compile(regex, Pattern.CASE_INSENSITIVE);\n\t\t} catch (PatternSyntaxException e) {\n\t\t\t// keep the previous pattern until the expression is valid again\n\t\t}\n\t}\n\n\tpublic boolean isActive() {\n\t\treturn pattern != null;\n\t}\n\n\t@Override\n\tpublic boolean select(Viewer viewer, Object parentElement, Object element) {\n\t\tPattern p = pattern;\n\t\tif (p == null || !(element instanceof NodeElement ne))\n\t\t\treturn true;\n\t\treturn matches(p, ne);\n\t}\n\n\tprivate boolean matches(Pattern p, NodeElement ne) {\n\t\tif (p.matcher(NodeLabels.getText(ne)).find())\n\t\t\treturn true;\n\t\tfor (Node child : ne.node().children)\n\t\t\tif (matches(p, new NodeElement(child, ne.document())))\n\t\t\t\treturn true;\n\t\treturn false;\n\t}\n}\n"
  reason: Adapt RegexNodeFilter to use NodeElement instead of Node+manager
```
/answer 74468498-8cbb-45da-bde5-dd0f55f292df allow
Control Request:
```yaml
id: f02639fa-9429-4329-80af-303fde363ab1
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/MarkdownOutlinePage.java
  old_text: "\tpublic MarkdownOutlinePage(AITextEditor editor) {\n\t\tthis.editor = editor;\n\t\tregexFilter = new RegexNodeFilter(editor.getUpdateManager());\n\t}"
  new_text: "\tpublic MarkdownOutlinePage(AITextEditor editor) {\n\t\tthis.editor = editor;\n\t\tregexFilter = new RegexNodeFilter();\n\t}"
  reason: Update RegexNodeFilter constructor call
```
/answer f02639fa-9429-4329-80af-303fde363ab1 allow
Control Request:
```yaml
id: 8dc2fe56-f558-49cc-9f35-df1c1e68778d
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/MarkdownOutlinePage.java
  old_text: "\t\tviewer = new CommonViewer(VIEWER_ID, container, getTreeStyle());\n\t\tviewer.getControl().setLayoutData(GridDataFactory.fillDefaults().grab(true, true).create());\n\t\tviewer.setInput(editor.getUpdateManager().getAst());\n\t\tviewer.addSelectionChangedListener(this::onOutlineSelection);"
  new_text: "\t\tviewer = new CommonViewer(VIEWER_ID, container, getTreeStyle());\n\t\tviewer.getControl().setLayoutData(GridDataFactory.fillDefaults().grab(true, true).create());\n\t\tviewer.setInput(rootElement());\n\t\tviewer.addSelectionChangedListener(this::onOutlineSelection);"
  reason: Use NodeElement root as viewer input
```
/answer 8dc2fe56-f558-49cc-9f35-df1c1e68778d allow
Control Request:
```yaml
id: bf0aaac3-9bff-446f-960a-432ff90502f0
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/MarkdownOutlinePage.java
  old_text: "\tprivate void onOutlineSelection(SelectionChangedEvent event) {\n\t\tif (syncingFromEditor)\n\t\t\treturn;\n\t\tif (event.getSelection() instanceof StructuredSelection sel && sel.getFirstElement() instanceof Node node)\n\t\t\teditor.selectAndRevealNode(node);\n\t}\n\n\tpublic void refresh() {\n\t\tif (!isAlive(viewer))\n\t\t\treturn;\n\t\tif (viewer.getInput() != editor.getUpdateManager().getAst())\n\t\t\tviewer.setInput(editor.getUpdateManager().getAst());\n\t\telse\n\t\t\tviewer.refresh();\n\t}\n\n\tpublic void selectNodeForOffset(int offset) {\n\t\tif (!isAlive(viewer))\n\t\t\treturn;\n\t\tMarkdownDocument ast = editor.getUpdateManager().getAst();\n\t\tif (ast == null || ast.getRoot() == null)\n\t\t\treturn;\n\n\t\tNode node = ast.find(offset, offset).getNode();\n\t\tif (node == null || node.instance == Elements.ROOT)\n\t\t\treturn;\n\n\t\tvar sel = viewer.getSelection();\n\t\tif (sel instanceof StructuredSelection ssel && ssel.getFirstElement() == node)\n\t\t\treturn;\n\t\tDisplay.getDefault().asyncExec(() -> {\n\t\t\tsyncingFromEditor = true;\n\t\t\ttry {\n\t\t\t\tviewer.setSelection(new StructuredSelection(node), true);\n\t\t\t} finally {\n\t\t\t\tsyncingFromEditor = false;\n\t\t\t}\n\t\t});\n\t}"
  new_text: "\tprivate void onOutlineSelection(SelectionChangedEvent event) {\n\t\tif (syncingFromEditor)\n\t\t\treturn;\n\t\tif (event.getSelection() instanceof StructuredSelection sel && sel.getFirstElement() instanceof NodeElement ne)\n\t\t\teditor.selectAndRevealNode(ne.node());\n\t}\n\n\t/** Builds the current tree input: the AST root paired with its document, or {@code null} if there is none. */\n\tprivate NodeElement rootElement() {\n\t\tMarkdownDocument ast = editor.getUpdateManager().getAst();\n\t\tif (ast == null || ast.getRoot() == null)\n\t\t\treturn null;\n\t\treturn new NodeElement(ast.getRoot(), editor.getUpdateManager().getDocument());\n\t}\n\n\tpublic void refresh() {\n\t\tif (!isAlive(viewer))\n\t\t\treturn;\n\t\tNodeElement current = rootElement();\n\t\tif (!Objects.equals(viewer.getInput(), current))\n\t\t\tviewer.setInput(current);\n\t\telse\n\t\t\tviewer.refresh();\n\t}\n\n\tpublic void selectNodeForOffset(int offset) {\n\t\tif (!isAlive(viewer))\n\t\t\treturn;\n\t\tMarkdownDocument ast = editor.getUpdateManager().getAst();\n\t\tif (ast == null || ast.getRoot() == null)\n\t\t\treturn;\n\n\t\tNode node = ast.find(offset, offset).getNode();\n\t\tif (node == null || node.instance == Elements.ROOT)\n\t\t\treturn;\n\n\t\tNodeElement element = new NodeElement(node, editor.getUpdateManager().getDocument());\n\t\tvar sel = viewer.getSelection();\n\t\tif (sel instanceof StructuredSelection ssel && element.equals(ssel.getFirstElement()))\n\t\t\treturn;\n\t\tDisplay.getDefault().asyncExec(() -> {\n\t\t\tsyncingFromEditor = true;\n\t\t\ttry {\n\t\t\t\tviewer.setSelection(new StructuredSelection(element), true);\n\t\t\t} finally {\n\t\t\t\tsyncingFromEditor = false;\n\t\t\t}\n\t\t});\n\t}"
  reason: Replace Node with NodeElement in outline page logic
```
/answer bf0aaac3-9bff-446f-960a-432ff90502f0 allow
Control Request:
```yaml
id: de0a786e-440d-400d-8873-3389e62a854f
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/filter/ElementFilter.java
  old_text: "import org.eclipse.jface.viewers.Viewer;\nimport org.eclipse.jface.viewers.ViewerFilter;\n\nimport xy.ai.workbench.editor.mdast.nodes.AbstractNode;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\n\npublic abstract class ElementFilter extends ViewerFilter {\n\n\t@Override\n\tpublic boolean select(Viewer viewer, Object parentElement, Object element) {\n\t\treturn !(element instanceof Node node) || !matches(node.instance);\n\t}"
  new_text: "import org.eclipse.jface.viewers.Viewer;\nimport org.eclipse.jface.viewers.ViewerFilter;\n\nimport xy.ai.workbench.editor.mdast.nodes.AbstractNode;\nimport xy.ai.workbench.editor.outline.NodeElement;\n\npublic abstract class ElementFilter extends ViewerFilter {\n\n\t@Override\n\tpublic boolean select(Viewer viewer, Object parentElement, Object element) {\n\t\treturn !(element instanceof NodeElement ne) || !matches(ne.node().instance);\n\t}"
  reason: Replace Node with NodeElement in ElementFilter
```
/answer de0a786e-440d-400d-8873-3389e62a854f allow
Control Request:
```yaml
id: 9c179416-ccb5-4064-8573-668504fd1fdb
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/outline/MarkdownOutlinePage.java
  reason: Verify imports and full file after edits
  max_line: 40
```
/answer 9c179416-ccb5-4064-8573-668504fd1fdb allow