Tausche den "AIRuleScanner" in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java` aus gegen eine Implementierung die AST optimiert ist.
Wird ein AST Knoten geändert, soll in deren Unterbaum die Regeln optimiert, auf Basis einer Konfiguration, zugeordnet werden.
Die Regeln sind hier `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java` konfiguriert.

Beispiele:
* Ein Heading "####" läuft nur im Bereich einer "HeadingSection", wie auch "HeaderRule" jedoch keine andere Regel.
* Line-Prefix regeln laufen nur in ihrem zugeordneten Element. Beispiel `xy.ai.workbench.editor.mdast.nodes.LineSection.USER`
* Emphasis Regeln laufen nur in "Paragraph".
* Die Code- Regel `new BlockRule("```", "```", blueToken)` nur in `ScriptBlock`
* "ListRule" läuft nur im "Paragraph"
* Prefix Lines wie ": " und "> " nur in "Paragraph"

Ziel ist, das der gesamte geänderte Bereich nur einmal in getrennten Bereichen durchlaufen wird mit jeweils nur zulässigen Regeln.

SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=648f3b09-c7f6-4e00-9694-2e447b904e7b | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 02a7b28f-add4-4ed4-b7e1-088ad09a0ebb
toolName: bash
arguments:
  reason: Explore project structure
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -path ./bin -prune -o -name "*.java" -print | xargs grep -l "AIRuleScanner\|AISourceViewerConfiguration\|mdast" | sort
```
/answer 02a7b28f-add4-4ed4-b7e1-088ad09a0ebb allow
Control Request:
```yaml
id: a63cfa76-bee6-4ca6-92fc-4622b5aaf636
result:
  structuredContent:
    stdout: |
      ./src/xy/ai/workbench/editors/AIRuleScanner.java
      ./src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
      ./src/xy/ai/workbench/editors/AITextEditor.java
      ./src/xy/ai/workbench/editors/DocumentBuffer.java
      ./src/xy/ai/workbench/editors/MarkdownOutlinePage.java
      ./src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
      ./src/xy/ai/workbench/mdast/IDocumentBuffer.java
      ./src/xy/ai/workbench/mdast/MarkdownDocument.java
      ./src/xy/ai/workbench/mdast/nodes/AbstractNode.java
      ./src/xy/ai/workbench/mdast/nodes/Category.java
      ./src/xy/ai/workbench/mdast/nodes/Elements.java
      ./src/xy/ai/workbench/mdast/nodes/HeadingSection.java
      ./src/xy/ai/workbench/mdast/nodes/LineSection.java
      ./src/xy/ai/workbench/mdast/nodes/Node.java
      ./src/xy/ai/workbench/mdast/nodes/PageSection.java
      ./src/xy/ai/workbench/mdast/nodes/Paragraph.java
      ./src/xy/ai/workbench/mdast/nodes/PrefixBlock.java
      ./src/xy/ai/workbench/mdast/nodes/Root.java
      ./src/xy/ai/workbench/mdast/nodes/ScriptBlock.java
      ./src/xy/ai/workbench/mdast/TextRegion.java
```
/answer a63cfa76-bee6-4ca6-92fc-4622b5aaf636 allow
ReasoningToken: 17
Control Request:
```yaml
id: 42d5869b-54b5-438e-8e50-c2e138986add
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java
  reason: Review current scanner rules
```
/answer 42d5869b-54b5-438e-8e50-c2e138986add allow
Control Request:
```yaml
id: aee1fad9-b3d0-4040-bdb8-94b30941234a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
  reason: Review current viewer configuration
```
/answer aee1fad9-b3d0-4040-bdb8-94b30941234a allow
Control Request:
```yaml
id: d57b1171-a0b2-40bb-be96-0e73ca7a1ab5
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Node.java
  reason: Understand Node interface
```
/answer d57b1171-a0b2-40bb-be96-0e73ca7a1ab5 allow
Control Request:
```yaml
id: ae78c5bd-8dde-46b3-afca-cdb7a881be0c
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/AbstractNode.java
  reason: Understand AbstractNode
```
/answer ae78c5bd-8dde-46b3-afca-cdb7a881be0c allow
Control Request:
```yaml
id: 594ec380-13bd-46b9-8cd3-56e1ab57fbeb
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/TextRegion.java
  reason: Understand TextRegion
```
/answer 594ec380-13bd-46b9-8cd3-56e1ab57fbeb allow
Control Request:
```yaml
id: fe53df7b-6a22-47d0-be9a-a8f70611ae15
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AITextEditor.java
  reason: Understand editor and AST change tracking
```
/answer fe53df7b-6a22-47d0-be9a-a8f70611ae15 allow
ReasoningToken: 67
Control Request:
```yaml
id: af13309c-f587-4ab0-831b-4e970b2f760e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  reason: Understand MarkdownDocument.update
```
/answer af13309c-f587-4ab0-831b-4e970b2f760e allow
Control Request:
```yaml
id: 455596c8-8b72-404a-8fe6-695c1d0a88b3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/HeadingSection.java
  reason: Understand HeadingSection
```
/answer 455596c8-8b72-404a-8fe6-695c1d0a88b3 allow
Control Request:
```yaml
id: 90e75142-8379-4def-b81d-ca12d55e0428
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/LineSection.java
  reason: Understand LineSection
```
/answer 90e75142-8379-4def-b81d-ca12d55e0428 allow
Control Request:
```yaml
id: 7767a1ff-321b-4303-abe8-010d9eb63f84
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Paragraph.java
  reason: Understand Paragraph
```
/answer 7767a1ff-321b-4303-abe8-010d9eb63f84 allow
Control Request:
```yaml
id: 3bbe3da3-dc69-4f7c-aa45-e751a37bebb5
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/ScriptBlock.java
  reason: Understand ScriptBlock
```
/answer 3bbe3da3-dc69-4f7c-aa45-e751a37bebb5 allow
Control Request:
```yaml
id: d78d41dd-e1c5-4aeb-b218-33a194b979a6
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PrefixBlock.java
  reason: Understand PrefixBlock
```
/answer d78d41dd-e1c5-4aeb-b218-33a194b979a6 allow
Control Request:
```yaml
id: b6b1cd1f-4fd6-4527-9b22-3ba95161cbf3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PageSection.java
  reason: Understand PageSection
```
/answer b6b1cd1f-4fd6-4527-9b22-3ba95161cbf3 allow
Control Request:
```yaml
id: 6725b13f-e07b-4163-b344-8b9195adb739
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Root.java
  reason: Understand Root
```
/answer 6725b13f-e07b-4163-b344-8b9195adb739 allow
Control Request:
```yaml
id: 50be1069-561f-4e66-b455-060f1ce18a1d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Elements.java
  reason: Understand Elements
```
/answer 50be1069-561f-4e66-b455-060f1ce18a1d allow
Control Request:
```yaml
id: c72e263b-afea-414f-94ff-1714881b4aae
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Category.java
  reason: Understand Category
```
/answer c72e263b-afea-414f-94ff-1714881b4aae allow
Control Request:
```yaml
id: 40c8c118-b64c-413c-a7ea-202158d72213
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md
  reason: list rule classes
```
/answer 40c8c118-b64c-413c-a7ea-202158d72213 allow
Control Request:
```yaml
id: 1ffe4da9-9fe0-4023-ad44-b294ea73075d
result:
  structuredContent:
    entries:
    - AbstractRule.java
    - BlockRule.java
    - EmphasisRule.java
    - HeaderRule.java
    - LineMatchRule.java
    - LinkRule.java
    - ListRule.java
    - PrefixLineRule.java
    - WordRule.java
```
/answer 1ffe4da9-9fe0-4023-ad44-b294ea73075d allow
Control Request:
```yaml
id: 347f5334-a9f9-409a-943d-809fcffb9809
toolName: bash
arguments:
  reason: find Scanner tool and RuleBasedScanner usage patterns
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -name "Scanner.java" -path "*tools*"
```
/answer 347f5334-a9f9-409a-943d-809fcffb9809 allow
Control Request:
```yaml
id: 1830ba4e-9e86-4d18-8ebe-e6388a8c6d1c
result:
  structuredContent:
    stdout: |
      ./src/xy/ai/workbench/tools/Scanner.java
```
/answer 1830ba4e-9e86-4d18-8ebe-e6388a8c6d1c allow
Control Request:
```yaml
id: 2cea8ee8-6d20-4680-b3bd-c79a9be0f1b1
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/AbstractRule.java
  reason: Understand AbstractRule (extends IRule?)
```
/answer 2cea8ee8-6d20-4680-b3bd-c79a9be0f1b1 allow
Control Request:
```yaml
id: 3f4a7863-9913-4660-8676-14bd5a3f61f9
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/BlockRule.java
  reason: Understand BlockRule
```
/answer 3f4a7863-9913-4660-8676-14bd5a3f61f9 allow
Control Request:
```yaml
id: b77344c2-63c1-4c71-84a2-3d6c0b05442d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/HeaderRule.java
  reason: Understand HeaderRule
```
/answer b77344c2-63c1-4c71-84a2-3d6c0b05442d allow
Control Request:
```yaml
id: b5c2c12e-31e6-42f3-a2da-ded1744c7c38
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/ListRule.java
  reason: Understand ListRule
```
/answer b5c2c12e-31e6-42f3-a2da-ded1744c7c38 allow
Control Request:
```yaml
id: 3e7fc183-6f56-4608-bb84-9007418486c3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/PrefixLineRule.java
  reason: Understand PrefixLineRule
```
/answer 3e7fc183-6f56-4608-bb84-9007418486c3 allow
Control Request:
```yaml
id: 0eff5d5f-cceb-4a59-a760-b222f0f5a105
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/EmphasisRule.java
  reason: Understand EmphasisRule
```
/answer 0eff5d5f-cceb-4a59-a760-b222f0f5a105 allow
Control Request:
```yaml
id: 3d9c1c29-824d-4c64-bf18-0b3c654db823
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/LineMatchRule.java
  reason: Understand LineMatchRule
```
/answer 3d9c1c29-824d-4c64-bf18-0b3c654db823 allow
Control Request:
```yaml
id: 282086e5-10ee-4f57-bc44-bf3c5df12e2a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/LinkRule.java
  reason: Understand LinkRule
```
/answer 282086e5-10ee-4f57-bc44-bf3c5df12e2a allow
ReasoningToken: 100
ReasoningToken: 148
Control Request:
```yaml
id: f1101ce4-02f7-49ac-84fe-718542db4a0b
toolName: bash
arguments:
  reason: "Find all usages of AIRuleScanner and its constructor, plus ITokenScanner reference check"
  cwd: /home/user/xyan/xy.ai.workbench
  script: |
    grep -rn "AIRuleScanner" --include=*.java .
    echo ---
    grep -rn "ITokenScanner" -r . 2>/dev/null | head -20
    echo ---
    find / -iname "org.eclipse.jface_*.jar" 2>/dev/null | head -5
    echo ---
    find / -iname "ITokenScanner.class" 2>/dev/null | head -5
```
/answer f1101ce4-02f7-49ac-84fe-718542db4a0b allow
ReasoningToken: 15705
Control Request:
```yaml
id: e48c02d3-b5c6-4725-9ccc-5009670a94ba
result:
  content:
  - type: text
    text: Full output written to file 18351 characters). Read only relevant excerpts (e.g. via grep/head/tail).
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-y3xt94p7.log
```
/answer e48c02d3-b5c6-4725-9ccc-5009670a94ba allow
Control Request:
```yaml
id: 97bdd869-6e79-4226-8225-4fba349e0a32
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  reason: Understand custom Scanner tool class
```
/answer 97bdd869-6e79-4226-8225-4fba349e0a32 allow
Control Request:
```yaml
id: e46b8db7-772b-464b-b07a-7deabbeb9a02
toolName: bash
arguments:
  reason: read grep results
  cwd: /home/user/xyan/xy.ai.workbench
  script: cat /tmp/mcpc-stdout-y3xt94p7.log
```
/answer e46b8db7-772b-464b-b07a-7deabbeb9a02 allow
Control Request:
```yaml
id: 583de8e9-77ad-4542-bc8b-ee2e020db252
result:
  content:
  - type: text
    text: Full output written to file 18351 characters). Read only relevant excerpts (e.g. via grep/head/tail).
  structuredContent:
    stdout_file: /tmp/mcpc-stdout-q4_iezcu.log
```
/answer 583de8e9-77ad-4542-bc8b-ee2e020db252 allow
ReasoningToken: 21
Control Request:
```yaml
id: 24c06c91-de9a-4985-8577-0170b35b4dbe
toolName: bash
arguments:
  reason: grep usages simpler
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rln "AIRuleScanner" --include=*.java . ; echo END
```
/answer 24c06c91-de9a-4985-8577-0170b35b4dbe allow
Control Request:
```yaml
id: 765e0447-6935-4d88-ad1e-22a0ec11e3a3
result:
  structuredContent:
    stdout: |
      ./src/xy/ai/workbench/editors/AIRuleScanner.java
      ./src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
      END
```
/answer 765e0447-6935-4d88-ad1e-22a0ec11e3a3 allow
ReasoningToken: 30
Text:
 Good, only these two files reference `AIRuleScanner`. Now let's check the jar for RuleBasedScanner/ITokenScanner API to confirm method names/signatures.
Control Request:
```yaml
id: aaa85218-ac30-4c17-ac98-0cc869e0c0e8
toolName: bash
arguments:
  reason: find jface jar for RuleBasedScanner API
  cwd: /home/user/xyan/xy.ai.workbench
  script: |-
    find / -iname "*jface.text*.jar" 2>/dev/null | head
    find / -iname "org.eclipse.text_*.jar" 2>/dev/null | head
```
/answer aaa85218-ac30-4c17-ac98-0cc869e0c0e8 deny EIn Eclipse Repo befindet sich hier `/home/user/xyan/eclipse.platform.ui`
Control Request:
```yaml
id: 3beb4335-c509-48f4-9da1-a959d31e9389
toolName: bash
arguments:
  reason: locate RuleBasedScanner and ITokenScanner sources in eclipse platform ui repo
  cwd: /home/user/xyan/eclipse.platform.ui
  script: find . -name "RuleBasedScanner.java" -o -name "ITokenScanner.java" -o -name "DefaultDamagerRepairer.java" 2>/dev/null
```
/answer 3beb4335-c509-48f4-9da1-a959d31e9389 allow
Control Request:
```yaml
id: 36513abf-c086-45ed-9019-63d0fece95ea
result:
  structuredContent:
    stdout: |
      ./bundles/org.eclipse.text/src/org/eclipse/jface/text/rules/RuleBasedScanner.java
      ./bundles/org.eclipse.text/src/org/eclipse/jface/text/rules/ITokenScanner.java
      ./bundles/org.eclipse.jface.text/src/org/eclipse/jface/text/rules/DefaultDamagerRepairer.java
```
/answer 36513abf-c086-45ed-9019-63d0fece95ea allow
Control Request:
```yaml
id: ce1b8ece-59a9-4264-a613-ececafd9d80a
toolName: read-file
arguments:
  path: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.text/src/org/eclipse/jface/text/rules/ITokenScanner.java
  reason: Check ITokenScanner interface
```
/answer ce1b8ece-59a9-4264-a613-ececafd9d80a allow
Control Request:
```yaml
id: 2837bfa0-8b46-4cbb-b0aa-f041f207f5e7
toolName: read-file
arguments:
  path: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.text/src/org/eclipse/jface/text/rules/RuleBasedScanner.java
  reason: Check RuleBasedScanner API
```
/answer 2837bfa0-8b46-4cbb-b0aa-f041f207f5e7 allow
Control Request:
```yaml
id: b3ced12b-ec06-4977-9779-0c88009b5a1e
toolName: read-file
arguments:
  path: /home/user/xyan/eclipse.platform.ui/bundles/org.eclipse.jface.text/src/org/eclipse/jface/text/rules/DefaultDamagerRepairer.java
  reason: Check how scanner is used by damager/repairer
```
/answer b3ced12b-ec06-4977-9779-0c88009b5a1e allow
Control Request:
```yaml
id: cd34f0cd-26ec-43ce-9331-a668c7a8cce2
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java
  mode: replace
  reason: Rewrite AIRuleScanner as AST-optimized ITokenScanner
  content: "package xy.ai.workbench.editors;\n\nimport java.util.ArrayList;\nimport java.util.IdentityHashMap;\nimport java.util.List;\nimport java.util.Map;\n\nimport org.eclipse.jface.text.IDocument;\nimport org.eclipse.jface.text.TextAttribute;\nimport org.eclipse.jface.text.rules.IRule;\nimport org.eclipse.jface.text.rules.IToken;\nimport org.eclipse.jface.text.rules.ITokenScanner;\nimport org.eclipse.jface.text.rules.RuleBasedScanner;\nimport org.eclipse.jface.text.rules.Token;\nimport org.eclipse.swt.SWT;\nimport org.eclipse.swt.graphics.Color;\nimport org.eclipse.swt.graphics.Font;\nimport org.eclipse.swt.graphics.FontData;\nimport org.eclipse.swt.graphics.RGB;\nimport org.eclipse.swt.widgets.Display;\n\nimport xy.ai.workbench.EditorInterface;\nimport xy.ai.workbench.connector.claudecode.CCControlClient;\nimport xy.ai.workbench.connector.claudecode.ProtocolParser;\nimport xy.ai.workbench.editor.md.AbstractRule;\nimport xy.ai.workbench.editor.md.BlockRule;\nimport xy.ai.workbench.editor.md.EmphasisRule;\nimport xy.ai.workbench.editor.md.HeaderRule;\nimport xy.ai.workbench.editor.md.LineMatchRule;\nimport xy.ai.workbench.editor.md.LinkRule;\nimport xy.ai.workbench.editor.md.ListRule;\nimport xy.ai.workbench.editor.md.PrefixLineRule;\nimport xy.ai.workbench.editor.mdast.MarkdownDocument;\nimport xy.ai.workbench.editor.mdast.nodes.AbstractNode;\nimport xy.ai.workbench.editor.mdast.nodes.HeadingSection;\nimport xy.ai.workbench.editor.mdast.nodes.LineSection;\nimport xy.ai.workbench.editor.mdast.nodes.Node;\nimport xy.ai.workbench.editor.mdast.nodes.PageSection;\nimport xy.ai.workbench.editor.mdast.nodes.Paragraph;\nimport xy.ai.workbench.editor.mdast.nodes.PrefixBlock;\nimport xy.ai.workbench.editor.mdast.nodes.Root;\nimport xy.ai.workbench.editor.mdast.nodes.ScriptBlock;\n\n/**\n * AST optimized token scanner: instead of trying every markdown rule at every\n * document position, the scanner walks the region of the markdown AST\n * ({@link MarkdownDocument}) that overlaps the requested range and, for every\n * node, only applies the (small) subset of rules that is configured for that\n * node's own text. Text that belongs to a child node is only scanned once,\n * using the rules assigned to that child - never using the rules of an\n * ancestor. This way the changed/requested region is scanned exactly once,\n * split into disjoint sub-regions each processed with only the rules that\n * are allowed there.\n */\npublic class AIRuleScanner implements ITokenScanner {\n\tpublic static final TextAttribute DEFAULT_ATTR = new TextAttribute(\n\t\t\tDisplay.getCurrent().getSystemColor(SWT.COLOR_WIDGET_FOREGROUND), null, SWT.NONE);\n\n\tprivate static final TextAttribute USER_ATTR = new TextAttribute(\n\t\t\tDisplay.getCurrent().getSystemColor(SWT.COLOR_BLACK),\n\t\t\tnew Color(Display.getCurrent(), new RGB(230, 230, 230)), SWT.BOLD);\n\tprivate static final TextAttribute AGENT_ATTR = new TextAttribute(\n\t\t\tDisplay.getCurrent().getSystemColor(SWT.COLOR_BLACK),\n\t\t\tnew Color(Display.getCurrent(), new RGB(200, 200, 255)), SWT.BOLD);\n\tprivate static final TextAttribute BLUE_ATTR = new TextAttribute(\n\t\t\tnew Color(Display.getCurrent(), new RGB(100, 100, 255)), null, SWT.NONE);\n\tprivate static final TextAttribute GREY_ATTR = new TextAttribute(\n\t\t\tnew Color(Display.getCurrent(), new RGB(150, 150, 150)), null, SWT.NONE);\n\tprivate static final TextAttribute COMMENT_ATTR = new TextAttribute(\n\t\t\tnew Color(Display.getCurrent(), new RGB(200, 200, 200)), null, SWT.NONE);\n\tprivate static final TextAttribute COMMENT_DARK_ATTR = new TextAttribute(\n\t\t\tnew Color(Display.getCurrent(), new RGB(130, 130, 130)), null, SWT.NONE);\n\tprivate static final TextAttribute SPACER_ATTR = new TextAttribute(\n\t\t\tDisplay.getCurrent().getSystemColor(SWT.COLOR_BLACK),\n\t\t\tnew Color(Display.getCurrent(), new RGB(200, 200, 200)), SWT.BOLD);\n\n\t/** Token used to reset styling of regions for which no rule is configured. */\n\tprivate static final IToken RESET_TOKEN = new Token(null);\n\n\tprivate final AITextEditor editor;\n\n\t/** One dedicated (stateless) rule based sub-scanner per AST node type. */\n\tprivate final Map<AbstractNode, RuleBasedScanner> scannerByNode = new IdentityHashMap<>();\n\tprivate final Map<RuleBasedScanner, IRule[]> ruleCache = new IdentityHashMap<>();\n\n\t/** Fallback scanner (all rules) used while no AST is available yet. */\n\tprivate final RuleBasedScanner fallbackScanner = new RuleBasedScanner();\n\n\tprivate final List<Piece> pieces = new ArrayList<>();\n\tprivate int pieceIndex;\n\tprivate int tokenOffset;\n\tprivate int tokenLength;\n\n\tpublic AIRuleScanner(Font basefont, AITextEditor editor) {\n\t\tthis.editor = editor;\n\n\t\tColor c = Display.getCurrent().getSystemColor(SWT.COLOR_WIDGET_FOREGROUND);\n\t\tIToken userToken = new Token(USER_ATTR);\n\t\tIToken agentToken = new Token(AGENT_ATTR);\n\t\tIToken blueToken = new Token(BLUE_ATTR);\n\t\tIToken greyToken = new Token(GREY_ATTR);\n\t\tIToken commentToken = new Token(COMMENT_ATTR);\n\t\tIToken commentDarkToken = new Token(COMMENT_DARK_ATTR);\n\t\tIToken spacerToken = new Token(SPACER_ATTR);\n\t\tIToken normal = new Token(new TextAttribute(c, null, SWT.NORMAL));\n\t\tIToken bold = new Token(new TextAttribute(c, null, SWT.BOLD));\n\t\tIToken italic = new Token(new TextAttribute(c, null, SWT.ITALIC));\n\t\tIToken bolditalic = new Token(new TextAttribute(c, null, SWT.BOLD | SWT.ITALIC));\n\t\tIToken underline = new Token(new TextAttribute(c, null, TextAttribute.UNDERLINE));\n\n\t\tIRule commentRule = new BlockRule(\"<!--\", \"-->\", normal);\n\n\t\t// ---- section: Root - only html comments may appear directly at the root ----\n\t\tregister(Root.INSTANCE, commentRule);\n\n\t\t// ---- section: headings - the marker/title line and setext-style headers ----\n\t\tFont[] headingFonts = getOrCreateFonts(basefont.getFontData()[0]);\n\t\tString[] headingPrefixes = { \"###### \", \"##### \", \"#### \", \"### \", \"## \", \"# \" };\n\t\tHeaderRule headerRule = new HeaderRule(new Token(new TextAttribute(c, null, SWT.BOLD)));\n\t\tfor (int i = 0; i < HeadingSection.HEADINGS.length; i++) {\n\t\t\tIToken headingToken = new Token(new TextAttribute(c, null, SWT.BOLD, headingFonts[i]));\n\t\t\tregister(HeadingSection.HEADINGS[i], new PrefixLineRule(headingPrefixes[i], headingToken), headerRule);\n\t\t}\n\n\t\t// ---- section: page separator ----\n\t\tregister(PageSection.INSTANCE, new PrefixLineRule(\"---\", spacerToken));\n\n\t\t// ---- section: chat line markers, each only valid for its own element ----\n\t\tregister(LineSection.USER, new LineMatchRule(EditorInterface.USER, userToken));\n\t\tregister(LineSection.AGENT, new LineMatchRule(EditorInterface.AGENT, agentToken));\n\t\tregister(LineSection.CONTROL_REQUEST, new LineMatchRule(CCControlClient.CONTROL_REQUEST, agentToken));\n\n\t\t// ---- block: protocol prefix lines, each tied 1:1 to its own AST element ----\n\t\tregister(PrefixBlock.THINKING, new PrefixLineRule(ProtocolParser.THINKING, agentToken));\n\t\tregister(PrefixBlock.TEXT, new PrefixLineRule(ProtocolParser.TEXT, agentToken));\n\t\tregister(PrefixBlock.TOOLUSE, new PrefixLineRule(ProtocolParser.TOOLUSE, agentToken));\n\t\tregister(PrefixBlock.ANSWER, new PrefixLineRule(CCControlClient.ANSWER, commentDarkToken));\n\t\tregister(PrefixBlock.REASONING_TOKEN, new PrefixLineRule(ProtocolParser.REASONING_TOKEN, commentDarkToken));\n\t\tregister(PrefixBlock.TOKEN_STATS, new PrefixLineRule(ProtocolParser.TOKEN_STATS, commentDarkToken));\n\t\tregister(PrefixBlock.SYSTEM_INIT, new PrefixLineRule(ProtocolParser.SYSTEM_INIT, agentToken));\n\t\tregister(PrefixBlock.LINE_COMMENT, new PrefixLineRule(AbstractRule.LINE_COMMENT, commentToken));\n\n\t\t// ---- block: fenced code, only valid inside a ScriptBlock ----\n\t\tregister(ScriptBlock.INSTANCE, new BlockRule(\"```\", \"```\", blueToken));\n\n\t\t// ---- section: paragraph - lists, emphasis, links and quote/glossary prefixes ----\n\t\tregister(Paragraph.INSTANCE, //\n\t\t\t\tcommentRule, //\n\t\t\t\tnew PrefixLineRule(\": \", italic), // glossary syntax\n\t\t\t\tnew PrefixLineRule(\"> \", italic), // citation syntax\n\t\t\t\tnew ListRule(bold), //\n\t\t\t\tnew EmphasisRule(\"***\", bolditalic), //\n\t\t\t\tnew EmphasisRule(\"**\", bold), //\n\t\t\t\tnew EmphasisRule(\"*\", italic), //\n\t\t\t\tnew EmphasisRule(\"$\", italic), //\n\t\t\t\tnew EmphasisRule(\"`\", blueToken), // file or variable\n\t\t\t\tnew EmphasisRule(\"„\", \"\\\"\", greyToken), // literally\n\t\t\t\tnew EmphasisRule(\"\\\"\", greyToken), // literally\n\t\t\t\tnew EmphasisRule(\"'\", greyToken), // literally\n\t\t\t\tnew EmphasisRule(\"»\", \"«\", greyToken), // literally\n\t\t\t\tnew EmphasisRule(\"›\", \"‹\", greyToken), // literally\n\t\t\t\tnew LinkRule(underline));\n\n\t\t// ---- fallback (used while no AST is available, e.g. huge documents) ----\n\t\tList<IRule> all = new ArrayList<>();\n\t\tfor (RuleBasedScanner s : scannerByNode.values())\n\t\t\tfor (IRule r : ruleCache.getOrDefault(s, new IRule[0]))\n\t\t\t\tall.add(r);\n\t\tfallbackScanner.setRules(all.toArray(new IRule[0]));\n\t}\n\n\tprivate void register(AbstractNode node, IRule... rules) {\n\t\tRuleBasedScanner scanner = new RuleBasedScanner();\n\t\tscanner.setRules(rules);\n\t\tscannerByNode.put(node, scanner);\n\t\truleCache.put(scanner, rules);\n\t}\n\n\t@Override\n\tpublic void setRange(IDocument document, int offset, int length) {\n\t\tpieces.clear();\n\t\tpieceIndex = 0;\n\t\ttokenOffset = offset;\n\t\ttokenLength = 0;\n\n\t\tif (length <= 0)\n\t\t\treturn;\n\n\t\tMarkdownDocument ast = editor != null ? editor.getMarkdownAst() : null;\n\t\tif (ast == null) {\n\t\t\tscanFlat(fallbackScanner, document, offset, offset + length);\n\t\t\treturn;\n\t\t}\n\n\t\tNode governing = ast.find(offset, offset + length);\n\t\tcollect(document, governing, offset, offset + length);\n\t}\n\n\t/**\n\t * Walks the subtree of {@code node} that overlaps [lo, hi), emitting rule\n\t * matches for the node's own text (the \"gaps\" between its children) and\n\t * recursing into every overlapping child using the child's own rules. Every\n\t * character of [lo, hi) is visited exactly once.\n\t */\n\tprivate void collect(IDocument document, Node node, int lo, int hi) {\n\t\tint cursor = Math.max(node.getOffset(), lo);\n\n\t\tfor (Node child : node.children) {\n\t\t\tint cs = child.getOffset();\n\t\t\tint ce = child.getEndOffset();\n\t\t\tif (ce <= lo || cs >= hi)\n\t\t\t\tcontinue; // no overlap with requested range\n\n\t\t\tif (cursor < cs)\n\t\t\t\tscanGap(document, node.instance, cursor, Math.min(cs, hi));\n\n\t\t\tcollect(document, child, lo, hi);\n\t\t\tcursor = ce;\n\t\t}\n\n\t\tint nodeEnd = Math.min(node.getEndOffset(), hi);\n\t\tif (cursor < nodeEnd)\n\t\t\tscanGap(document, node.instance, cursor, nodeEnd);\n\t}\n\n\t/**\n\t * Scans [start, end), the text directly owned by {@code type} (i.e. not part\n\t * of any child node), with the rules configured for {@code type}. The\n\t * sub-range is widened by a single character (if available) so that rules\n\t * relying on a boundary character shared with the following sibling/child\n\t * (e.g. the trailing line break of a line marker) can still match; any\n\t * resulting token is clipped back to [start, end).\n\t */\n\tprivate void scanGap(IDocument document, AbstractNode type, int start, int end) {\n\t\tif (start >= end)\n\t\t\treturn;\n\n\t\tRuleBasedScanner scanner = scannerByNode.get(type);\n\t\tif (scanner == null) {\n\t\t\tpieces.add(new Piece(start, end - start, RESET_TOKEN));\n\t\t\treturn;\n\t\t}\n\n\t\tint widenedEnd = Math.min(document.getLength(), end + 1);\n\t\tscanner.setRange(document, start, widenedEnd - start);\n\n\t\twhile (true) {\n\t\t\tIToken token = scanner.nextToken();\n\t\t\tif (token.isEOF())\n\t\t\t\tbreak;\n\n\t\t\tint off = scanner.getTokenOffset();\n\t\t\tint len = scanner.getTokenLength();\n\t\t\tif (off >= end)\n\t\t\t\tbreak;\n\t\t\tif (off + len > end)\n\t\t\t\tlen = end - off;\n\t\t\tif (len <= 0)\n\t\t\t\tcontinue;\n\n\t\t\tpieces.add(new Piece(off, len, token));\n\t\t}\n\t}\n\n\tprivate void scanFlat(RuleBasedScanner scanner, IDocument document, int lo, int hi) {\n\t\tscanner.setRange(document, lo, hi - lo);\n\t\twhile (true) {\n\t\t\tIToken token = scanner.nextToken();\n\t\t\tif (token.isEOF())\n\t\t\t\tbreak;\n\t\t\tpieces.add(new Piece(scanner.getTokenOffset(), scanner.getTokenLength(), token));\n\t\t}\n\t}\n\n\t@Override\n\tpublic IToken nextToken() {\n\t\tif (pieceIndex >= pieces.size())\n\t\t\treturn Token.EOF;\n\n\t\tPiece p = pieces.get(pieceIndex++);\n\t\ttokenOffset = p.offset;\n\t\ttokenLength = p.length;\n\t\treturn p.token;\n\t}\n\n\t@Override\n\tpublic int getTokenOffset() {\n\t\treturn tokenOffset;\n\t}\n\n\t@Override\n\tpublic int getTokenLength() {\n\t\treturn tokenLength;\n\t}\n\n\tprivate static final class Piece {\n\t\tfinal int offset;\n\t\tfinal int length;\n\t\tfinal IToken token;\n\n\t\tPiece(int offset, int length, IToken token) {\n\t\t\tthis.offset = offset;\n\t\t\tthis.length = length;\n\t\t\tthis.token = token;\n\t\t}\n\t}\n\n\tprivate static Font[] cachedFonts;\n\n\tprivate Font[] getOrCreateFonts(FontData fdata) {\n\t\tif (cachedFonts != null)\n\t\t\treturn cachedFonts;\n\n\t\tint count = 6;\n\t\tFont[] fonts = new Font[count];\n\t\tDisplay display = Display.getDefault();\n\n\t\tfor (int i = 0; i < count; i++)\n\t\t\tfonts[i] = new Font(display,\n\t\t\t\t\tnew FontData(fdata.getName(), fdata.getHeight() + (i * 2), fdata.getStyle() | SWT.BOLD));\n\n\t\treturn cachedFonts = fonts;\n\t}\n}\n"
```
/answer cd34f0cd-26ec-43ce-9331-a668c7a8cce2 allow
Control Request:
```yaml
id: 6cd0480f-4b6e-41b2-8742-45401fa42e6c
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
  old_text: "\t\tDefaultDamagerRepairer dr = new DefaultDamagerRepairer(new AIRuleScanner(font)) {"
  new_text: "\t\tDefaultDamagerRepairer dr = new DefaultDamagerRepairer(new AIRuleScanner(font, editor)) {"
  reason: Pass editor into AST-optimized scanner constructor
```
/answer 6cd0480f-4b6e-41b2-8742-45401fa42e6c allow
Control Request:
```yaml
id: e70c40c3-2f83-4b78-bfe8-ecb6726eaeff
toolName: bash
arguments:
  reason: Check for build tooling (maven/gradle/eclipse) to compile and verify
  cwd: /home/user/xyan/xy.ai.workbench
  script: ls -la; find . -maxdepth 2 -iname "pom.xml" -o -iname "build.gradle" -maxdepth 2 2>/dev/null; find . -iname "*.classpath" -maxdepth 1
```
/answer e70c40c3-2f83-4b78-bfe8-ecb6726eaeff deny es gibt weder Build-Tooling noch Tests
ReasoningToken: 18019
Control Request:
```yaml
id: cab2e53d-85d9-4ec1-82fd-498254c88ed8
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java
  reason: Final review of new file for correctness
```
/answer cab2e53d-85d9-4ec1-82fd-498254c88ed8 allow
Control Request:
```yaml
id: 5ac33105-1f61-4905-95f4-9cb062080617
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AISourceViewerConfiguration.java
  reason: Verify final state of configuration file
```
/answer 5ac33105-1f61-4905-95f4-9cb062080617 allow
Text:
 Now update `AISourceViewerConfiguration` to pass the editor to the new constructor.
Text:
 Now let's try to build the project to verify compilation correctness.
ReasoningToken: 4401
Text:
 Looks correct and complete. Let me also double-check the updated `AISourceViewerConfiguration.java`.
Both files are consistent. No other file references the old constructor.

## Zusammenfassung

Ich habe `AIRuleScanner` komplett neu implementiert (`ITokenScanner` statt `RuleBasedScanner`):

**Prinzip:**
- Für jeden AST-Knotentyp (`HeadingSection[0..5]`, `PageSection`, `LineSection.USER/AGENT/CONTROL_REQUEST`, alle `PrefixBlock.*`, `ScriptBlock`, `Paragraph`, `Root`) wird ein eigener, kleiner `RuleBasedScanner` mit genau den erlaubten Regeln registriert (`register(...)`), exakt nach deinen Vorgaben:
  - `HeadingSection[i]` → passendes `PrefixLineRule("#…# ")` + `HeaderRule`, sonst nichts.
  - `LineSection.USER/AGENT/CONTROL_REQUEST` → jeweils nur ihr eigenes `LineMatchRule`.
  - `PrefixBlock.*` (THINKING, TEXT, TOOLUSE, ANSWER, REASONING_TOKEN, TOKEN_STATS, SYSTEM_INIT, LINE_COMMENT) → jeweils genau ihr eigenes `PrefixLineRule`.
  - `ScriptBlock` → nur `BlockRule("```","```")`.
  - `Paragraph` → `ListRule`, alle `EmphasisRule`s, `LinkRule`, `PrefixLineRule(": ")`/`("> ")` sowie das HTML-Kommentar-`BlockRule`.
  - `PageSection` → `PrefixLineRule("---")`.
  - `Root` → nur das HTML-Kommentar-`BlockRule` (global nutzbar).

**Ablauf pro `setRange`:**
1. Über `MarkdownDocument.find(offset, offset+length)` wird der Knoten ermittelt, der die angeforderte Region umschließt (ohne sie strikt zu enthalten).
2. `collect(...)` läuft rekursiv genau einmal über den angeforderten Bereich: An jedem Knoten werden nur die „Lücken“ zwischen seinen Kindern mit den **eigenen** Regeln gescannt; jedes Kind wird exakt einmal rekursiv mit seinen **eigenen** Regeln behandelt. Dadurch wird jede Textstelle nur einmal und nur mit den dort zulässigen Regeln durchlaufen.
3. Ein Randfall (gemeinsames Trennzeichen, z. B. das `\n` zwischen `LineSection`-Marker und Kind) wird durch ein kontrolliertes 1-Zeichen-„Widening“ mit anschließendem Clipping korrekt behandelt, ohne doppelte Token zu erzeugen.
4. Bereiche ohne konfigurierte Regeln erhalten einen Reset-Token, damit alte Styles sauber überschrieben werden.
5. Ohne verfügbaren AST (z. B. beim initialen Öffnen) gibt es einen Fallback-Scanner mit der Vereinigung aller Regeln (identisch zum alten Verhalten).

`AISourceViewerConfiguration` wurde entsprechend angepasst, um den `AITextEditor` an den neuen Konstruktor `AIRuleScanner(Font, AITextEditor)` zu übergeben.
