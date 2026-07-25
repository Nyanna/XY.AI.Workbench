Verbessere den Editor und den Spellcheck.

Auf Basis des neu implementierten AST in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java` soll die Region Optimierung wieder aktiviert werden.
Der Damager in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java`, soll nun nur noch die jeweils betroffene Region invalidieren. Das gilt für den "AIRuleScanner" und den `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckInstaller.java`.
Es scheint auch so als würd die `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellingStrategy.java` mit dem Aufruf von `fViewer.invalidateTextPresentation();`, mehr invalidieren als notwendig.
Der Spellcheck soll nur die geänderte Bereiche validieren und auch nur die Annotations aus diesem Bereich prüfen und ersetzen.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=99381881-b107-4547-a867-af106fb021eb | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 80245363-3308-4b9e-a7d4-ef56f970ac65
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  reason: Understand AST implementation
```
/answer 80245363-3308-4b9e-a7d4-ef56f970ac65 allow
Control Request:
```yaml
id: a9304289-d778-4bed-81d8-5f4e894276a9
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
  reason: Understand damager/scanner config
```
/answer a9304289-d778-4bed-81d8-5f4e894276a9 allow
Control Request:
```yaml
id: 248f3d13-5058-45fd-9031-5814a19f560b
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckInstaller.java
  reason: Understand spell check installer
```
/answer 248f3d13-5058-45fd-9031-5814a19f560b allow
Control Request:
```yaml
id: 9bb41bd8-06d6-4d60-9f37-5c8622764fbb
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellingStrategy.java
  reason: Understand spelling strategy
```
/answer 9bb41bd8-06d6-4d60-9f37-5c8622764fbb allow
ReasoningToken: 21
Control Request:
```yaml
id: da77a414-b1b4-4043-b54e-f028214aeb78
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java
  reason: Check rule scanner for context
```
/answer da77a414-b1b4-4043-b54e-f028214aeb78 allow
ReasoningToken: 220
Control Request:
```yaml
id: 0a3aaf14-60a8-4451-a571-9f8ddb776986
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast
  reason: Explore AST package structure
```
/answer 0a3aaf14-60a8-4451-a571-9f8ddb776986 allow
Control Request:
```yaml
id: 4c94d1ec-73b0-45c1-955c-2ae7d8fc9642
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
/answer 4c94d1ec-73b0-45c1-955c-2ae7d8fc9642 allow
Control Request:
```yaml
id: 456aa984-b13c-40ed-ad21-a761fcefefb9
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/DocumentBuffer.java
  reason: Understand DocumentBuffer
```
/answer 456aa984-b13c-40ed-ad21-a761fcefefb9 allow
ReasoningToken: 63
Control Request:
```yaml
id: bf1b7968-e767-409a-990f-f7d211dadc58
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  reason: Understand AST update method and region tracking
```
/answer bf1b7968-e767-409a-990f-f7d211dadc58 allow
Control Request:
```yaml
id: fefd6dd1-e5e6-40a5-98a0-5d9fd75dab63
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Node.java
  reason: Understand Node interface
```
/answer fefd6dd1-e5e6-40a5-98a0-5d9fd75dab63 allow
Control Request:
```yaml
id: 5358d98a-7adf-439b-877a-5b976385fb2a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/IDocumentBuffer.java
  reason: Check package dependencies purity
```
/answer 5358d98a-7adf-439b-877a-5b976385fb2a allow
Control Request:
```yaml
id: c2652438-bd27-4f15-8892-7d2ab585ae9f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  reason: Check reconciler implementation for region handling
```
/answer c2652438-bd27-4f15-8892-7d2ab585ae9f allow
ReasoningToken: 3517
Control Request:
```yaml
id: fc446b1f-bc7d-4e03-8a48-aaf9fb158277
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find / -iname "*org.eclipse.jface.text*.jar" 2>/dev/null | head -20
  reason: Locate jface.text jar to inspect API
```
/answer fc446b1f-bc7d-4e03-8a48-aaf9fb158277 deny Wenn du API specs spezifischer Klassen benötigst, Suche gezielt nach diesen.
ReasoningToken: 196
Control Request:
```yaml
id: 83f93051-afb8-4de7-b7f1-5c1e4ed7d073
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find ~ -iname "org.eclipse.jface.text_*.jar" 2>/dev/null
  reason: Locate jface.text jar for ITextViewer API inspection
```
/answer 83f93051-afb8-4de7-b7f1-5c1e4ed7d073 deny ITextviewer liegt hier `/home/user/xyan/xy.ai.workbench/project/ITextViewer.java`
ReasoningToken: 17
Control Request:
```yaml
id: da62d5e3-d68e-4669-9542-408ffdc26da8
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -n "invalidateTextPresentation" /home/user/xyan/xy.ai.workbench/project/ITextViewer.java /home/user/xyan/xy.ai.workbench/project/*.java 2>/dev/null
  reason: Check invalidateTextPresentation signatures available
```
/answer da62d5e3-d68e-4669-9542-408ffdc26da8 allow
Control Request:
```yaml
id: d08bd8b6-6673-41c9-af52-cd6e7d3d4d26
result:
  structuredContent:
    stdout: "/home/user/xyan/xy.ai.workbench/project/ITextViewer.java:387:\t * See {@link ITextViewerExtension2#invalidateTextPresentation(int, int)}\n/home/user/xyan/xy.ai.workbench/project/ITextViewer.java:393:\tvoid invalidateTextPresentation();\n/home/user/xyan/xy.ai.workbench/project/ITextViewer.java:387:\t * See {@link ITextViewerExtension2#invalidateTextPresentation(int, int)}\n/home/user/xyan/xy.ai.workbench/project/ITextViewer.java:393:\tvoid invalidateTextPresentation();\n"
```
/answer d08bd8b6-6673-41c9-af52-cd6e7d3d4d26 allow
Text:
 ITextViewerExtension2 has a region-based `invalidateTextPresentation(int, int)`. Let's confirm.
Control Request:
```yaml
id: 7d35e7d0-2ee6-4fac-8e1f-e87cd3d8704c
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: "find /home/user/xyan/xy.ai.workbench/project -iname \"*ITextViewerExtension2*\"; echo \"---\"; cat /home/user/xyan/xy.ai.workbench/project/ITextViewerExtension2.java 2>/dev/null | sed -n '1,80p'"
  reason: Check ITextViewerExtension2 API
```
/answer 7d35e7d0-2ee6-4fac-8e1f-e87cd3d8704c allow
Control Request:
```yaml
id: 8f83edc5-edbd-466b-98d0-f3d50db2505c
result:
  structuredContent:
    stdout: |
      ---
```
/answer 8f83edc5-edbd-466b-98d0-f3d50db2505c allow
Control Request:
```yaml
id: 4f3ec67a-e833-43eb-877e-ace99c042016
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/project
  reason: See what API stub files exist
```
/answer 4f3ec67a-e833-43eb-877e-ace99c042016 deny es existieren keine API Stub Files
Control Request:
```yaml
id: 8309fb01-4488-40d7-be28-d5515bfd78a3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/project/ITextViewer.java
  start: invalidateTextPresentation
  max_char: 4000
  reason: Read context around invalidateTextPresentation doc
```
/answer 8309fb01-4488-40d7-be28-d5515bfd78a3 allow
Control Request:
```yaml
id: 294a9ca4-66f7-4135-8935-2d4f2bb7d5d6
result:
  content:
  - type: text
    text: Start marker is ambiguous – found 2 occurrences in file.
  isError: true
```
/answer 294a9ca4-66f7-4135-8935-2d4f2bb7d5d6 allow
ReasoningToken: 40
Control Request:
```yaml
id: 8dfe69d3-9e54-49e4-9ba5-ff4f328cfed5
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/project/ITextViewer.java
  min_line: 370
  max_line: 395
  reason: Read invalidateTextPresentation javadoc
```
/answer 8dfe69d3-9e54-49e4-9ba5-ff4f328cfed5 allow
Control Request:
```yaml
id: 44d0bc20-d9dc-4357-adbd-5e27bc8fccb1
result:
  structuredContent:
    checksum: f881462eab9b1c5cb201533dbfe34d2ff35a98518870faf462221890ce6af73b
    content: "\t/**\n\t * Applies the color information encoded in the given text presentation.\n\t * <code>controlRedraw</code> tells this viewer whether it should take care of\n\t * redraw management or not. If, e.g., this call is one in a sequence of multiple\n\t * presentation calls, it is more appropriate to explicitly control redrawing at the\n\t * beginning and the end of the sequence.\n\t *\n\t * @param presentation the presentation to be applied to this viewer\n\t * @param controlRedraw indicates whether this viewer should manage redraws\n\t */\n\tvoid changeTextPresentation(TextPresentation presentation, boolean controlRedraw);\n\n\t/**\n\t * Marks the currently applied text presentation as invalid. It is the\n\t * viewer's responsibility to take any action it can to repair the text\n\t * presentation.\n\t * <p>\n\t * See {@link ITextViewerExtension2#invalidateTextPresentation(int, int)}\n\t * for a way to invalidate specific regions rather than the presentation as\n\t * a whole.\n\t *\n\t * @since 2.0\n\t */\n\tvoid invalidateTextPresentation();\n\n\t/**\n"
```
/answer 44d0bc20-d9dc-4357-adbd-5e27bc8fccb1 allow
Text:
 Confirmed: `ITextViewerExtension2.invalidateTextPresentation(int offset, int length)` exists. Now let's implement the changes.

### 1. MarkdownDocument.update: return changed region
Control Request:
```yaml
id: 5f0caaa7-4aef-4ec0-b171-1e227f7f7938
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  old_text: "\tpublic void update(int offset, int removed, int inserted) {\n\t\tint lo = offset;\n\t\tint hi = offset + removed;\n\t\tint delta = inserted - removed;\n\t\tlines.update(buffer, offset, removed, inserted);\n\n\t\tNode sec = find(lo, hi);\n\t\twhile (true) {\n\t\t\tNode parent = sec.parent;\n\t\t\tint absStart = sec.getOffset();\n\t\t\tint newLen = sec.length() + delta;\n\t\t\tNode rn = parse(absStart, absStart + newLen);\n\n\t\t\tif (parent == null || isCompatible(rn.children, sec, parent)) {\n\t\t\t\treplace(sec, rn.children, delta);\n\t\t\t\treturn;\n\t\t\t}\n\t\t\tsec = parent;\n\t\t}\n\t}"
  new_text: "\t/**\n\t * Updates the AST for the given document edit and returns the region that\n\t * was actually reparsed/replaced, i.e. the minimal region that needs to be\n\t * re-highlighted/re-checked by consumers (damager, spell checker, ...).\n\t */\n\tpublic TextRegion update(int offset, int removed, int inserted) {\n\t\tint lo = offset;\n\t\tint hi = offset + removed;\n\t\tint delta = inserted - removed;\n\t\tlines.update(buffer, offset, removed, inserted);\n\n\t\tNode sec = find(lo, hi);\n\t\twhile (true) {\n\t\t\tNode parent = sec.parent;\n\t\t\tint absStart = sec.getOffset();\n\t\t\tint newLen = sec.length() + delta;\n\t\t\tNode rn = parse(absStart, absStart + newLen);\n\n\t\t\tif (parent == null || isCompatible(rn.children, sec, parent)) {\n\t\t\t\treplace(sec, rn.children, delta);\n\t\t\t\treturn new TextRegion(absStart, newLen);\n\t\t\t}\n\t\t\tsec = parent;\n\t\t}\n\t}\n\n\t/**\n\t * Simple, framework-agnostic offset/length pair describing a region of the\n\t * underlying document.\n\t */\n\tpublic static final class TextRegion {\n\t\tprivate final int offset;\n\t\tprivate final int length;\n\n\t\tpublic TextRegion(int offset, int length) {\n\t\t\tthis.offset = offset;\n\t\t\tthis.length = length;\n\t\t}\n\n\t\tpublic int getOffset() {\n\t\t\treturn offset;\n\t\t}\n\n\t\tpublic int getLength() {\n\t\t\treturn length;\n\t\t}\n\t}"
  reason: Return changed AST region from update() for damage-region optimization
```
/answer 5f0caaa7-4aef-4ec0-b171-1e227f7f7938 allow
Control Request:
```yaml
id: 4c979755-d9f2-4c07-8068-d9726ea243e0
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  old_text: "\tprivate MarkdownDocument ast;\n\tprivate DocumentBuffer astBuffer;\n\tprivate int pendingRemoved;"
  new_text: "\tprivate MarkdownDocument ast;\n\tprivate DocumentBuffer astBuffer;\n\tprivate int pendingRemoved;\n\tprivate MarkdownDocument.TextRegion lastAstChangeRegion;"
  reason: Track last changed AST region for damager/spellcheck optimization
```
/answer 4c979755-d9f2-4c07-8068-d9726ea243e0 allow
Control Request:
```yaml
id: ef185f3f-280d-446f-9e11-a828a03d5527
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  old_text: "\tpublic AITextEditor() {\n\t\tsuper();\n\t\tsetSourceViewerConfiguration(new AISourceViewerConfiguration());\n\t}"
  new_text: "\tpublic AITextEditor() {\n\t\tsuper();\n\t\tsetSourceViewerConfiguration(new AISourceViewerConfiguration(this));\n\t}"
  reason: Pass editor reference so the damager can access AST change region
```
/answer ef185f3f-280d-446f-9e11-a828a03d5527 allow
Control Request:
```yaml
id: 1d441e8e-d012-47d8-9b88-910cb763c487
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  old_text: "\tprivate void buildAst(IDocument document) {\n\t\tastBuffer = new DocumentBuffer(document);\n\t\tast = new MarkdownDocument(astBuffer);\n\t\tast.update(0, 0, astBuffer.length());\n\t}\n\n\tprivate void updateAst(DocumentEvent evt) {\n\t\tif (ast == null || astBuffer == null || astBuffer.document() != evt.getDocument())\n\t\t\treturn;\n\t\tString text = evt.getText();\n\t\tint inserted = text == null ? 0 : text.length();\n\t\tast.update(evt.getOffset(), pendingRemoved, inserted);\n\t}\n\n\tpublic MarkdownDocument getMarkdownAst() {\n\t\treturn ast;\n\t}"
  new_text: "\tprivate void buildAst(IDocument document) {\n\t\tastBuffer = new DocumentBuffer(document);\n\t\tast = new MarkdownDocument(astBuffer);\n\t\tlastAstChangeRegion = ast.update(0, 0, astBuffer.length());\n\t}\n\n\tprivate void updateAst(DocumentEvent evt) {\n\t\tif (ast == null || astBuffer == null || astBuffer.document() != evt.getDocument())\n\t\t\treturn;\n\t\tString text = evt.getText();\n\t\tint inserted = text == null ? 0 : text.length();\n\t\tlastAstChangeRegion = ast.update(evt.getOffset(), pendingRemoved, inserted);\n\t}\n\n\tpublic MarkdownDocument getMarkdownAst() {\n\t\treturn ast;\n\t}\n\n\t/**\n\t * Returns the region of the document that was affected by the most recent\n\t * AST update, i.e. the minimal region that changed structurally and needs\n\t * to be re-highlighted/re-checked. May be {@code null} if no update has\n\t * happened yet.\n\t */\n\tpublic MarkdownDocument.TextRegion getLastAstChangeRegion() {\n\t\treturn lastAstChangeRegion;\n\t}"
  reason: Expose last AST change region and store it on every update
```
/answer 1d441e8e-d012-47d8-9b88-910cb763c487 allow
Text:
 Now update `AITextEditor` to track and expose the last changed AST region, and pass itself into `AISourceViewerConfiguration`.
Control Request:
```yaml
id: b263f991-2bb9-4480-89fb-b9dad20ea9d3
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "AISourceViewerConfiguration(" src/ 2>/dev/null
  reason: Find all usages of AISourceViewerConfiguration constructor
```
/answer b263f991-2bb9-4480-89fb-b9dad20ea9d3 allow
Control Request:
```yaml
id: 1b39c531-e69d-4db8-bfc7-73b6c890d9de
result:
  structuredContent:
    stdout: "src/xy/ai/workbench/editors/AITextEditor.java:56:\t\tsetSourceViewerConfiguration(new AISourceViewerConfiguration(this));\n"
```
/answer 1b39c531-e69d-4db8-bfc7-73b6c890d9de allow
Control Request:
```yaml
id: 40d195a0-0efd-41c8-bfed-14898e16bfb2
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
  old_text: "import xy.ai.workbench.editors.spellcheck.SpellCheckInstaller;\nimport xy.ai.workbench.editors.spellcheck.SpellingQuickAssistProcessor;\n\npublic class AISourceViewerConfiguration extends SourceViewerConfiguration {\n\tprivate static final int LIMIT = 512 * 1024;\n\n\t@Override"
  new_text: "import xy.ai.workbench.editors.spellcheck.SpellCheckInstaller;\nimport xy.ai.workbench.editors.spellcheck.SpellingQuickAssistProcessor;\nimport xy.ai.workbench.mdast.MarkdownDocument;\n\npublic class AISourceViewerConfiguration extends SourceViewerConfiguration {\n\tprivate static final int LIMIT = 512 * 1024;\n\n\tprivate final AITextEditor editor;\n\n\tpublic AISourceViewerConfiguration(AITextEditor editor) {\n\t\tthis.editor = editor;\n\t}\n\n\t@Override"
  reason: Add editor reference to access AST change region
```
/answer 40d195a0-0efd-41c8-bfed-14898e16bfb2 allow
ReasoningToken: 36
Control Request:
```yaml
id: 31aa73cb-fbce-4434-9dfa-580df584eac0
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
  old_text: "\t\t\t@Override\n\t\t\tpublic IRegion getDamageRegion(ITypedRegion partition, DocumentEvent e,\n\t\t\t\t\tboolean documentPartitioningChanged) {\n\t\t\t\tIDocument document = sourceViewer.getDocument();\n\t\t\t\tif (document != null && document.getLength() > LIMIT)\n\t\t\t\t\treturn new Region(0, 1);\n\t\t\t\treturn new Region(0, document.getLength());\n//\t\t\t\treturn super.getDamageRegion(partition, e, documentPartitioningChanged);\n\t\t\t}"
  new_text: "\t\t\t@Override\n\t\t\tpublic IRegion getDamageRegion(ITypedRegion partition, DocumentEvent e,\n\t\t\t\t\tboolean documentPartitioningChanged) {\n\t\t\t\tIDocument document = sourceViewer.getDocument();\n\t\t\t\tif (document == null)\n\t\t\t\t\treturn partition;\n\t\t\t\tif (document.getLength() > LIMIT)\n\t\t\t\t\treturn new Region(0, 1);\n\n\t\t\t\tMarkdownDocument.TextRegion astRegion = editor != null ? editor.getLastAstChangeRegion() : null;\n\t\t\t\tif (astRegion != null) {\n\t\t\t\t\tint offset = Math.max(0, Math.min(astRegion.getOffset(), document.getLength()));\n\t\t\t\t\tint end = Math.max(offset, Math.min(astRegion.getOffset() + astRegion.getLength(),\n\t\t\t\t\t\t\tdocument.getLength()));\n\t\t\t\t\treturn new Region(offset, end - offset);\n\t\t\t\t}\n\n\t\t\t\treturn super.getDamageRegion(partition, e, documentPartitioningChanged);\n\t\t\t}"
  reason: Use AST-derived region instead of whole document for damage region
```
/answer 31aa73cb-fbce-4434-9dfa-580df584eac0 allow
ReasoningToken: 92
Text:
 Good, only one usage. Now let's rewrite `AISourceViewerConfiguration`.
Control Request:
```yaml
id: 2bae26de-3b6b-4047-8929-68df2395980e
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
  old_text: "\t@Override\n\tpublic IReconciler getReconciler(ISourceViewer sourceViewer) {\n\t\treturn SpellCheckInstaller.createReconciler(sourceViewer);\n\t}"
  new_text: "\t@Override\n\tpublic IReconciler getReconciler(ISourceViewer sourceViewer) {\n\t\treturn SpellCheckInstaller.createReconciler(sourceViewer, editor);\n\t}"
  reason: Pass editor to spellcheck reconciler for AST-based dirty regions
```
/answer 2bae26de-3b6b-4047-8929-68df2395980e allow
Text:
 Now update `getDamageRegion` to use the AST region instead of the whole document:
Control Request:
```yaml
id: 7acd0c8e-c64f-49ef-b985-641b358aa093
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckInstaller.java
  old_text: |-
    import org.eclipse.ui.texteditor.DefaultMarkerAnnotationAccess;

    public class SpellCheckInstaller {

        private static final int RECONCILE_DELAY_MS = 500;

        public static IReconciler createReconciler(ISourceViewer sourceViewer) {
            SpellingStrategy strategy = new SpellingStrategy(sourceViewer);
            return new SpellCheckReconciler(strategy, RECONCILE_DELAY_MS);
        }
  new_text: |-
    import org.eclipse.ui.texteditor.DefaultMarkerAnnotationAccess;

    import xy.ai.workbench.editors.AITextEditor;

    public class SpellCheckInstaller {

        private static final int RECONCILE_DELAY_MS = 500;

        public static IReconciler createReconciler(ISourceViewer sourceViewer, AITextEditor editor) {
            SpellingStrategy strategy = new SpellingStrategy(sourceViewer);
            return new SpellCheckReconciler(strategy, RECONCILE_DELAY_MS, editor);
        }
  reason: Pass editor into reconciler for AST-based dirty region tracking
```
/answer 7acd0c8e-c64f-49ef-b985-641b358aa093 allow
Control Request:
```yaml
id: d0d4e4a4-c99f-43bc-888e-e91b911e0179
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  old_text: |-
    import org.eclipse.jface.text.reconciler.IReconciler;
    import org.eclipse.jface.text.reconciler.IReconcilingStrategy;

    /**
  new_text: |-
    import org.eclipse.jface.text.reconciler.IReconciler;
    import org.eclipse.jface.text.reconciler.IReconcilingStrategy;

    import xy.ai.workbench.editors.AITextEditor;
    import xy.ai.workbench.mdast.MarkdownDocument;

    /**
  reason: Import AITextEditor and MarkdownDocument for AST-based dirty region
```
/answer d0d4e4a4-c99f-43bc-888e-e91b911e0179 allow
Control Request:
```yaml
id: eef1f754-fa5d-4300-981b-46a01e34e6fc
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  old_text: |2-
        private final SpellingStrategy fStrategy;
        private final int fDelayMs;

        private ITextViewer fViewer;
  new_text: |2-
        private final SpellingStrategy fStrategy;
        private final int fDelayMs;
        private final AITextEditor fEditor;

        private ITextViewer fViewer;
  reason: Add editor field for AST-based dirty regions
```
/answer eef1f754-fa5d-4300-981b-46a01e34e6fc allow
Control Request:
```yaml
id: c1cb0689-b849-4ee4-b076-2a85dbb31405
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  old_text: |2-
            @Override
            public void documentChanged(DocumentEvent event) {
                int start = event.getOffset();
                int end   = start + Math.max(
                        event.getLength(),
                        event.getText() != null ? event.getText().length() : 0);
                mergeDirty(start, Math.max(end, start + 1));
                scheduleReconcile();
            }
  new_text: |2-
            @Override
            public void documentChanged(DocumentEvent event) {
                MarkdownDocument.TextRegion astRegion = fEditor != null ? fEditor.getLastAstChangeRegion() : null;
                int start;
                int end;
                if (astRegion != null) {
                    start = astRegion.getOffset();
                    end   = astRegion.getOffset() + astRegion.getLength();
                } else {
                    start = event.getOffset();
                    end   = start + Math.max(
                            event.getLength(),
                            event.getText() != null ? event.getText().length() : 0);
                }
                mergeDirty(start, Math.max(end, start + 1));
                scheduleReconcile();
            }
  reason: Use AST change region instead of raw event offsets for dirty tracking
```
/answer c1cb0689-b849-4ee4-b076-2a85dbb31405 allow
Control Request:
```yaml
id: 1d3cde08-7b8b-4766-a600-8b7000a86576
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  old_text: |2-
        public SpellCheckReconciler(SpellingStrategy strategy, int delayMs) {
            fStrategy = strategy;
            fDelayMs  = delayMs;
        }
  new_text: |2-
        public SpellCheckReconciler(SpellingStrategy strategy, int delayMs, AITextEditor editor) {
            fStrategy = strategy;
            fDelayMs  = delayMs;
            fEditor   = editor;
        }
  reason: Accept editor reference in constructor
```
/answer 1d3cde08-7b8b-4766-a600-8b7000a86576 allow
ReasoningToken: 1470
Text:
 Now update `SpellCheckInstaller.createReconciler` and `SpellCheckReconciler` to accept/use the editor's AST region:
Control Request:
```yaml
id: a9697659-b0ae-403d-a8bd-5d608f593d46
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellingStrategy.java
  old_text: |-
    import org.eclipse.jface.text.source.Annotation;
    import org.eclipse.jface.text.source.IAnnotationModel;
    import org.eclipse.jface.text.source.IAnnotationModelExtension;
    import org.eclipse.jface.text.source.ISourceViewer;
  new_text: |-
    import org.eclipse.jface.text.ITextViewerExtension2;
    import org.eclipse.jface.text.source.Annotation;
    import org.eclipse.jface.text.source.IAnnotationModel;
    import org.eclipse.jface.text.source.IAnnotationModelExtension;
    import org.eclipse.jface.text.source.ISourceViewer;
  reason: Import ITextViewerExtension2 for region-limited invalidation
```
/answer a9697659-b0ae-403d-a8bd-5d608f593d46 allow
Control Request:
```yaml
id: d428ad75-ce67-4ccc-804b-077d0bdf89eb
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellingStrategy.java
  old_text: "\t\t// Explicitly invalidate the region so the AnnotationPainter redraws it.\n\t\tfViewer.invalidateTextPresentation();\n\t}"
  new_text: "\t\t// Explicitly invalidate only the checked region so the AnnotationPainter\n\t\t// redraws it, instead of the whole viewer's text presentation.\n\t\tif (fViewer instanceof ITextViewerExtension2) {\n\t\t\t((ITextViewerExtension2) fViewer).invalidateTextPresentation(region.getOffset(), region.getLength());\n\t\t} else {\n\t\t\tfViewer.invalidateTextPresentation();\n\t\t}\n\t}"
  reason: Limit invalidation to the checked region rather than the whole presentation
```
/answer d428ad75-ce67-4ccc-804b-077d0bdf89eb allow
Control Request:
```yaml
id: f85fd537-55f8-41d4-943b-6ba88d5b6b5f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
  reason: Verify final file content
```
/answer f85fd537-55f8-41d4-943b-6ba88d5b6b5f allow
Control Request:
```yaml
id: 6fc48b2d-2247-4c58-8989-abf76cf524ef
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: ls; find . -iname "pom.xml" -o -iname "build.gradle" | head
  reason: Check build setup
```
/answer 6fc48b2d-2247-4c58-8989-abf76cf524ef allow
Control Request:
```yaml
id: ffeb4441-3ff1-4a5b-ad5c-60cec4da7c71
result:
  structuredContent:
    stdout: |
      bin
      build.properties
      claude-code
      contexts.xml
      docs
      icons
      language-tool
      libs
      mcpc
      META-INF
      plugin.xml
      project
      proxy
      README.md
      src
      TODO.md
```
/answer ffeb4441-3ff1-4a5b-ad5c-60cec4da7c71 deny es gibt kein build setup
Control Request:
```yaml
id: cd7fcb25-7862-4236-b0dd-1e3ba592115a
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "\.update(" src/xy/ai/workbench/mdast src/xy/ai/workbench/editors 2>/dev/null; echo ---; grep -rn "getMarkdownAst\|SpellCheckReconciler(\|createReconciler(" src 2>/dev/null
  reason: Check for other call sites depending on old signatures
```
/answer cd7fcb25-7862-4236-b0dd-1e3ba592115a allow
Control Request:
```yaml
id: 6af91499-8f85-4f8f-b763-72547327d8b4
result:
  structuredContent:
    stdout: "src/xy/ai/workbench/mdast/MarkdownDocument.java:29:\t\tlines.update(buffer, offset, removed, inserted);\nsrc/xy/ai/workbench/editors/AITextEditor.java:142:\t\tlastAstChangeRegion = ast.update(0, 0, astBuffer.length());\nsrc/xy/ai/workbench/editors/AITextEditor.java:150:\t\tlastAstChangeRegion = ast.update(evt.getOffset(), pendingRemoved, inserted);\n---\nsrc/xy/ai/workbench/editors/MarkdownOutlinePage.java:45:\t\tviewer.setInput(editor.getMarkdownAst());\nsrc/xy/ai/workbench/editors/MarkdownOutlinePage.java:61:\t\tif (viewer.getInput() != editor.getMarkdownAst())\nsrc/xy/ai/workbench/editors/MarkdownOutlinePage.java:62:\t\t\tviewer.setInput(editor.getMarkdownAst());\nsrc/xy/ai/workbench/editors/MarkdownOutlinePage.java:73:\t\tMarkdownDocument ast = editor.getMarkdownAst();\nsrc/xy/ai/workbench/editors/AITextEditor.java:153:\tpublic MarkdownDocument getMarkdownAst() {\nsrc/xy/ai/workbench/editors/AISourceViewerConfiguration.java:34:\t\treturn SpellCheckInstaller.createReconciler(sourceViewer, editor);\nsrc/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java:100:    public SpellCheckReconciler(SpellingStrategy strategy, int delayMs, AITextEditor editor) {\nsrc/xy/ai/workbench/editors/spellcheck/SpellCheckInstaller.java:18:    public static IReconciler createReconciler(ISourceViewer sourceViewer, AITextEditor editor) {\nsrc/xy/ai/workbench/editors/spellcheck/SpellCheckInstaller.java:20:        return new SpellCheckReconciler(strategy, RECONCILE_DELAY_MS, editor);\n"
```
/answer 6af91499-8f85-4f8f-b763-72547327d8b4 allow
Control Request:
```yaml
id: 79ad1e93-7942-4199-b277-9ecef824f32d
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -path ./project -prune -o -iname "*Test*.java" -print 2>/dev/null | xargs grep -ln "MarkdownDocument\|SpellCheckReconciler\|SpellCheckInstaller\|AISourceViewerConfiguration" 2>/dev/null
  reason: Find tests possibly affected
```
/answer 79ad1e93-7942-4199-b277-9ecef824f32d deny es gibt keine Tests
Control Request:
```yaml
id: 83fd5202-edb1-4a21-ae37-88a3e68ce4bb
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  reason: Final review of reconciler changes
```
/answer 83fd5202-edb1-4a21-ae37-88a3e68ce4bb allow
Control Request:
```yaml
id: f430ffcd-511c-430c-a422-18e3cc72a075
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  reason: Final review of AST update changes
```
/answer f430ffcd-511c-430c-a422-18e3cc72a075 allow
ReasoningToken: 33
ReasoningToken: 15
Text:
 Now fix `SpellingStrategy.applyAnnotations` to invalidate only the checked region:
ReasoningToken: 82
ReasoningToken: 52
Text:
 Now let's compile the project to check for errors.
ReasoningToken: 74
Text:
 Good, all call sites consistent. Let's check for other tests referencing `MarkdownDocument.update` return type or `SpellCheckReconciler`/`SpellCheckInstaller` in test dirs.
Text:
 Fine, no tests exist. Let's do a final review of all changed files for correctness and consistency.
Alles konsistent. Zusammenfassung der Änderungen:

**1. `MarkdownDocument.java`** – `update()` gibt jetzt eine `TextRegion` (offset/length) zurück: genau der AST-Knotenbereich, der beim Re-Parsing tatsächlich ersetzt wurde. Damit kennt der Aufrufer die minimal betroffene Region statt nur die reine Edit-Range.

**2. `AITextEditor.java`** – speichert das Ergebnis jedes `ast.update(...)`-Aufrufs in `lastAstChangeRegion` und stellt es über `getLastAstChangeRegion()` bereit. Übergibt sich selbst an `AISourceViewerConfiguration`.

**3. `AISourceViewerConfiguration.java`** – der `DefaultDamagerRepairer` für den `AIRuleScanner` nutzt in `getDamageRegion(...)` jetzt `editor.getLastAstChangeRegion()`, um nur die tatsächlich geänderte AST-Region zu invalidieren, statt immer das komplette Dokument (`new Region(0, document.getLength())`). Für sehr große Dokumente (> LIMIT) bleibt die Performance-Notbremse (`Region(0,1)`) erhalten. `getReconciler(...)` reicht den Editor nun an `SpellCheckInstaller.createReconciler(...)` weiter.

**4. `SpellCheckInstaller.java`** – `createReconciler(...)` nimmt zusätzlich den `AITextEditor` entgegen und gibt ihn an den `SpellCheckReconciler` weiter.

**5. `SpellCheckReconciler.java`** – nutzt bei `documentChanged` bevorzugt die AST-Region des Editors (`fEditor.getLastAstChangeRegion()`) statt der rohen `DocumentEvent`-Offsets, um den Dirty-Bereich zu bestimmen (Fallback auf Event-Offsets falls kein Editor/AST vorhanden).

**6. `SpellingStrategy.java`** – `applyAnnotations(...)` ruft nun `((ITextViewerExtension2) fViewer).invalidateTextPresentation(region.getOffset(), region.getLength())` auf, statt das komplette Text-Presentation zu invalidieren. Damit wird nach dem Rechtschreib-Check tatsächlich nur der geprüfte Bereich neu gezeichnet (Annotations wurden dort bereits vorher schon region-genau ersetzt/entfernt).

Damit ist die Region-Optimierung end-to-end wiederhergestellt: AST liefert die minimale geänderte Region → Syntax-Highlighting-Damager invalidiert nur diese Region → Spellcheck-Reconciler dirty-trackt nur diese Region → Annotations werden nur dort ersetzt → Presentation-Invalidation beschränkt sich ebenfalls auf diese Region.
