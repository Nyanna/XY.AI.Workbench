Überführe "LINE_COMMENT" in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Elements.java`, in eine eigene Implementierung.
Zusammenhängende Line-Comments sollen einen gemeinsamen Block bilden. Also beginnt ein Line-Comment bei `#:` und hört erst auf bei der nächsten Zeile ohne `#:`, ohne das letzte Newline zu konsumieren.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=54eedc5b-eb1f-4cfb-b1ac-24aae08d9009 | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: a9caec8d-950b-437b-8c49-d5c206cff940
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Elements.java
  reason: Inspect LINE_COMMENT usage
```
/answer a9caec8d-950b-437b-8c49-d5c206cff940 allow
ReasoningToken: 14
Control Request:
```yaml
id: dc21ffaf-6186-4386-8d7a-7ab5b2989426
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes
  reason: List node classes
```
/answer dc21ffaf-6186-4386-8d7a-7ab5b2989426 allow
Control Request:
```yaml
id: 821ecc14-d9ee-409e-8308-40ab9ab41de6
result:
  structuredContent:
    entries:
    - AbstractNode.java
    - Category.java
    - Elements.java
    - HeadingSection.java
    - LineSection.java
    - Node.java
    - PageSection.java
    - Paragraph.java
    - PrefixBlock.java
    - Root.java
    - ScriptBlock.java
```
/answer 821ecc14-d9ee-409e-8308-40ab9ab41de6 allow
Control Request:
```yaml
id: 8f5d13f6-05e6-4704-bd27-2a1058d9a73c
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "LINE_COMMENT" --include=*.java src | grep -v Elements.java
  reason: Find LINE_COMMENT references
```
/answer 8f5d13f6-05e6-4704-bd27-2a1058d9a73c allow
Control Request:
```yaml
id: e62144df-6ac7-4a68-b71a-ee2e37c3f910
result:
  structuredContent:
    stdout: "src/xy/ai/workbench/AISessionManager.java:223:\t\t\tif (!line.trim().startsWith(AbstractRule.LINE_COMMENT))\nsrc/xy/ai/workbench/editors/md/AbstractRule.java:11:\tpublic static final String LINE_COMMENT = \"#:\";\nsrc/xy/ai/workbench/editors/AIRuleScanner.java:135:\t\tregister(Elements.Basics.LINE_COMMENT, new PrefixLineRule(AbstractRule.LINE_COMMENT, commentToken));\nsrc/xy/ai/workbench/editors/CommentHandler.java:31:\t\t\t\tif (line.trim().startsWith(AbstractRule.LINE_COMMENT))\nsrc/xy/ai/workbench/editors/CommentHandler.java:32:\t\t\t\t\tdoc.replace(lineOffset + line.indexOf(AbstractRule.LINE_COMMENT), 2, \"\");\nsrc/xy/ai/workbench/editors/CommentHandler.java:34:\t\t\t\t\tdoc.replace(lineOffset, 0, AbstractRule.LINE_COMMENT);\n"
```
/answer e62144df-6ac7-4a68-b71a-ee2e37c3f910 allow
ReasoningToken: 37
Control Request:
```yaml
id: 49e14c50-6f71-49e3-9079-f1b90b147f58
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PrefixBlock.java
  reason: See existing PrefixBlock implementation
```
/answer 49e14c50-6f71-49e3-9079-f1b90b147f58 allow
Control Request:
```yaml
id: 82322ab5-2a1e-41cc-932f-34e9cab6713d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/AbstractNode.java
  reason: Understand AbstractNode base class
```
/answer 82322ab5-2a1e-41cc-932f-34e9cab6713d allow
Control Request:
```yaml
id: 0d0347ae-77b7-4c2d-8ede-cb1d25ca61f1
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Node.java
  reason: Understand Node base class
```
/answer 0d0347ae-77b7-4c2d-8ede-cb1d25ca61f1 allow
Control Request:
```yaml
id: aebfe63c-84ca-4132-9528-5f06eedeced8
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  reason: Understand Scanner API for implementing custom node
```
/answer aebfe63c-84ca-4132-9528-5f06eedeced8 allow
Control Request:
```yaml
id: a27f59df-f28e-43e6-812b-d42ce9f9c001
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/LineSection.java
  reason: See another node implementation example
```
/answer a27f59df-f28e-43e6-812b-d42ce9f9c001 allow
Control Request:
```yaml
id: f108d9b3-2182-4f95-a237-76987624c520
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/ScriptBlock.java
  reason: See another node implementation example
```
/answer f108d9b3-2182-4f95-a237-76987624c520 allow
ReasoningToken: 14
Control Request:
```yaml
id: a9ba9419-1058-4f75-8318-e44986c6bd7f
toolName: write
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/LineCommentBlock.java
  mode: replace
  content: "package xy.ai.workbench.mdast.nodes;\n\nimport xy.ai.workbench.tools.Scanner;\n\n/**\n * A block of one or more consecutive line-comments. A line-comment starts\n * with the given marker (e.g. \"#:\") and continues until the end of the line.\n * Consecutive lines that all start with the marker form a single, common\n * block. The block ends right before the newline that follows the last\n * comment line - that trailing newline is never consumed, so it stays\n * available for sibling scanning.\n */\npublic class LineCommentBlock extends AbstractNode {\n\tprivate char[] prefix;\n\tprivate char[] marker;\n\n\tLineCommentBlock(String marker) {\n\t\tsuper(Category.Block, Elements.NONE);\n\t\tthis.prefix = (\"\\n\" + marker).toCharArray();\n\t\tthis.marker = marker.toCharArray();\n\t}\n\n\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\treturn s.isNextSequenceBounded(prefix);\n\t}\n\n\t@Override\n\tprotected boolean isEndInner(Scanner s) {\n\t\tScanner sub = s.getSubscanner();\n\t\tboolean end;\n\t\tif (!sub.readNext())\n\t\t\tend = true;\n\t\telse if (!sub.isNewLine())\n\t\t\tend = false;\n\t\telse\n\t\t\t// next line does not start with the marker -> block ends here,\n\t\t\t// without consuming this newline\n\t\t\tend = !sub.isNextSequence(marker);\n\t\tsub.reset();\n\t\treturn end;\n\t}\n\n\t@Override\n\tpublic String toString() {\n\t\treturn String.valueOf(prefix).replace('\\n', ' ');\n\t}\n}\n"
  reason: Create dedicated implementation for line-comment blocks
```
/answer a9ba9419-1058-4f75-8318-e44986c6bd7f allow
Control Request:
```yaml
id: 82b476f9-3683-4508-b661-4c2f1f95a318
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Elements.java
  old_text: "\t\tpublic static final PrefixBlock LINE_COMMENT = new PrefixBlock(AbstractRule.LINE_COMMENT);"
  new_text: "\t\tpublic static final LineCommentBlock LINE_COMMENT = new LineCommentBlock(AbstractRule.LINE_COMMENT);"
  reason: Use dedicated LineCommentBlock implementation for LINE_COMMENT
```
/answer 82b476f9-3683-4508-b661-4c2f1f95a318 allow
Control Request:
```yaml
id: 68128e34-8e95-42f1-b8eb-e326b5fb1f8d
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -iname pom.xml -o -iname build.gradle | head; ls
  reason: Find build setup to compile/verify
```
/answer 68128e34-8e95-42f1-b8eb-e326b5fb1f8d deny es gibt weder Built-Setup noch tests
Control Request:
```yaml
id: 7d3ccb88-1ec1-4b6d-a959-040dacb136d2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Elements.java
  max_line: 20
  reason: Verify edit applied correctly
```
/answer 7d3ccb88-1ec1-4b6d-a959-040dacb136d2 allow
Control Request:
```yaml
id: a709df98-7a06-4f05-96c7-ed01b8b7ec53
result:
  structuredContent:
    checksum: 9dd332ab2071829a1c58b59d8e7c3991e030ccd5d7ebf251f51d9cfd46916fbb
    content: "package xy.ai.workbench.mdast.nodes;\n\nimport java.util.stream.Stream;\n\nimport xy.ai.workbench.EditorInterface;\nimport xy.ai.workbench.connector.claudecode.CCControlClient;\nimport xy.ai.workbench.connector.claudecode.ProtocolParser;\nimport xy.ai.workbench.editor.md.AbstractRule;\n\n/*\n * Sorting must be inverted, root on bottom contains all\n */\npublic class Elements {\n\tpublic static final AbstractNode[] NONE = new AbstractNode[0];\n\n\tpublic static class Basics {\n\t\tpublic static final ScriptBlock SCRIPTBLOCK = new ScriptBlock();\n\t\tpublic static final LineCommentBlock LINE_COMMENT = new LineCommentBlock(AbstractRule.LINE_COMMENT);\n\t\tpublic static final Paragraph PARAGRAPH = new Paragraph(NONE); // replaced later\n\t\tpublic static final AbstractNode[] ALL = of(SCRIPTBLOCK, LINE_COMMENT, PARAGRAPH);\n"
```
/answer a709df98-7a06-4f05-96c7-ed01b8b7ec53 allow
ReasoningToken: 4198
Text:
 Nun kompilieren, um zu prüfen, ob alles passt:
ReasoningToken: 132
Es gibt kein Build-Setup/Tests in diesem Repo (Bash wurde für den Compile-Check abgelehnt), daher kann ich nicht automatisiert kompilieren – die Änderung ist aber in sich konsistent mit den bestehenden Mustern (`ScriptBlock`, `PrefixBlock`, `LineSection`).

**Was ich geändert habe:**

1. Neue Klasse `LineCommentBlock.java` (`src/xy/ai/workbench/mdast/nodes/`):
   - `isStart(s)`: erkennt den Beginn wie bisher über `\n#:` (via `isNextSequenceBounded`).
   - `isEndInner(s)`: peekt zeichenweise weiter. Trifft sie auf ein Newline, wird zusätzlich geprüft, ob direkt danach wieder der Marker `#:` folgt:
     - Ja → Block ist noch nicht zu Ende (Newline + nächste Kommentarzeile werden vom äußeren Scan-Loop mit konsumiert, der Block wächst weiter).
     - Nein (oder EOF) → Block endet genau vor diesem Newline, das letzte Newline wird **nicht** konsumiert.
   - Das Peeken erfolgt non-destruktiv über `Scanner.getSubscanner()` + `reset()`, analog zu den bestehenden Implementierungen.

2. `Elements.java`: `Basics.LINE_COMMENT` verwendet jetzt `new LineCommentBlock(AbstractRule.LINE_COMMENT)` statt `new PrefixBlock(...)`.

Damit bilden mehrere aufeinanderfolgende `#:`-Zeilen einen einzigen zusammenhängenden Knoten, und die abschließende Zeile ohne `#:` sowie deren vorangehendes Newline bleiben für das Sibling-Scanning erhalten.
