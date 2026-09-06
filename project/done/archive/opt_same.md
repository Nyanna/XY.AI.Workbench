Optimiere `xy.ai.workbench.editor.mdast.MarkdownDocument.replace(Node, List<Node>, int)` in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java`. Wenn der neu geparste Node ein Blatt und kompatibel ist, kann die Node Instanz aktualisiert statt ausgetauscht werden.
SystemInit:  id=97501765-4a3e-42ac-b40c-dd37c05e5aa5, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: ac4cbda8-8b89-42a1-a196-2c8052066ac7
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  reason: Inspect file to optimize replace method
```
/answer ac4cbda8-8b89-42a1-a196-2c8052066ac7 allow
ReasoningToken: 10
Control Request:
```yaml
id: 79071fb3-a04e-4518-94c4-17a9a9b69c29
toolName: bash
arguments:
  script: "find /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes -name \"Node.java\" -o -name \"Elements.java\" | xargs -I{} echo {}"
  cwd: /home/user/xyan
  reason: find Node.java
```
/answer 79071fb3-a04e-4518-94c4-17a9a9b69c29 allow
Control Request:
```yaml
id: 0154881e-7c50-4ded-a31b-958569da6aca
result:
  structuredContent:
    stdout: |
      /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Elements.java
      /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Node.java
```
/answer 0154881e-7c50-4ded-a31b-958569da6aca allow
ReasoningToken: 40
Control Request:
```yaml
id: 0720dd4a-f375-4369-be38-b61a71b018e5
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Node.java
  reason: Inspect Node structure
```
/answer 0720dd4a-f375-4369-be38-b61a71b018e5 allow
Control Request:
```yaml
id: 484d411e-b109-465c-be69-0fe94732f8af
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes
  reason: check for Node subclasses / extra state fields
```
/answer 484d411e-b109-465c-be69-0fe94732f8af allow
Control Request:
```yaml
id: f3f7dccd-593c-4729-96ed-a8ef113e8ed7
result:
  structuredContent:
    entries:
    - AbstractNode.java
    - Category.java
    - Elements.java
    - HeadingSection.java
    - LineCommentBlock.java
    - LineSection.java
    - Node.java
    - PageSection.java
    - Paragraph.java
    - PrefixBlock.java
    - Root.java
    - ScriptBlock.java
```
/answer f3f7dccd-593c-4729-96ed-a8ef113e8ed7 deny Node hat keine Ableitungen
ReasoningToken: 2071
ReasoningToken: 171
Control Request:
```yaml
id: 76d1a7b5-5dc5-4221-b3be-e10d00dea744
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  old_text: "\t\tList<Node> siblings = parent.children;\n\t\tint idx = siblings.indexOf(sec);\n\t\tfor (int i = idx + 1; i < siblings.size(); i++)\n\t\t\tshift(siblings.get(i), delta);\n\n\t\t// TODO optimized when changing the exact same Node\n\t\tsiblings.remove(idx);\n\t\tint at = idx;\n\t\tfor (Node c : nchilds) {\n\t\t\tc.start += sec.start;\n\t\t\tc.end += sec.start;\n\t\t\tsiblings.add(at++, reparent(c, parent));\n\t\t}\n\n\t\tfor (Node anc = parent; anc != null; anc = anc.parent) {"
  new_text: "\t\tList<Node> siblings = parent.children;\n\t\tint idx = siblings.indexOf(sec);\n\t\tfor (int i = idx + 1; i < siblings.size(); i++)\n\t\t\tshift(siblings.get(i), delta);\n\n\t\tNode changed;\n\t\tif (isLeafUpdate(sec, nchilds)) {\n\t\t\t// The re-parsed node is a leaf and of the same kind as `sec`: keep the\n\t\t\t// existing Node instance alive (identity preserved for listeners) and\n\t\t\t// simply move/resize it instead of swapping it out.\n\t\t\tNode nc = nchilds.get(0);\n\t\t\tint oldStart = sec.start;\n\t\t\tsec.start = oldStart + nc.start;\n\t\t\tsec.end = oldStart + nc.end;\n\t\t\tchanged = sec;\n\t\t} else {\n\t\t\tsiblings.remove(idx);\n\t\t\tint at = idx;\n\t\t\tfor (Node c : nchilds) {\n\t\t\t\tc.start += sec.start;\n\t\t\t\tc.end += sec.start;\n\t\t\t\tsiblings.add(at++, reparent(c, parent));\n\t\t\t}\n\t\t\tchanged = parent;\n\t\t}\n\n\t\tfor (Node anc = parent; anc != null; anc = anc.parent) {"
  reason: Add in-place leaf update optimization to replace()
```
/answer 76d1a7b5-5dc5-4221-b3be-e10d00dea744 allow
Control Request:
```yaml
id: 494c8780-0463-499b-8350-e3996b5224ae
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  old_text: "\t\t\tfor (int i = ai + 1; i < as.size(); i++)\n\t\t\t\tshift(as.get(i), delta);\n\t\t}\n\t\treturn parent;\n\t}\n\n\tprivate Node reparent(Node src, Node newParent) {"
  new_text: "\t\t\tfor (int i = ai + 1; i < as.size(); i++)\n\t\t\t\tshift(as.get(i), delta);\n\t\t}\n\t\treturn changed;\n\t}\n\n\t/**\n\t * Checks whether the re-parsed replacement consists of a single leaf node of\n\t * the same kind as {@code sec}, which is itself a leaf. In that case the\n\t * existing {@code sec} instance can be updated in place (new offsets) instead\n\t * of being replaced by a fresh {@link Node}, preserving node identity and\n\t * avoiding an unnecessary list remove/insert.\n\t */\n\tprivate boolean isLeafUpdate(Node sec, List<Node> nchilds) {\n\t\tif (nchilds.size() != 1 || !sec.children.isEmpty())\n\t\t\treturn false;\n\t\tNode nc = nchilds.get(0);\n\t\treturn nc.children.isEmpty() && nc.instance == sec.instance;\n\t}\n\n\tprivate Node reparent(Node src, Node newParent) {"
  reason: Return changed node and add isLeafUpdate helper
```
/answer 494c8780-0463-499b-8350-e3996b5224ae allow
Text:
 Jetzt noch die letzte Zeile `return parent;` auf `return changed;` anpassen und die Hilfsmethode `isLeafUpdate` ergänzen:
Control Request:
```yaml
id: eb940472-ec95-4aa0-825b-58accfdc4fe7
toolName: bash
arguments:
  script: find / -iname "pom.xml" -path "*xy.ai.workbench*" 2>/dev/null; find /home/user/xyan -maxdepth 2 -iname "build.gradle*" 2>/dev/null
  cwd: /home/user/xyan
  reason: find build files to compile
```
/answer eb940472-ec95-4aa0-825b-58accfdc4fe7 allow