Kannst du dir mal die Implementierung anschauen in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java`.
Bei Aktualisierung einer Sektion wird diese neu geparsed und ersetzt die Childs im parent. Eine nun sehr häufige Operation ist das anfügen innerhalb einer Sektion. Dabei wird einfach das letzte Child erweitert oder aber weitere Siblings auf der selben Ebene angefügt. Das neu parsen der ganzen Parent Sektion ist hier überflüssig. Es gibt bereits eine Optimierung Änderungen hinter dem letzten Element die Änderungen dem Element zuzusprechen. Vielleicht lässt sich das ganze dahingehend Optimieren das bei der Prüfung der Kompatibilität die Siblings geprüft werden und sollten lediglich welche angefügt worden sein, nicht den ganzen Parent neu parsen zu müssen.  
SystemInit:  id=c88e5d14-3368-48e9-ad1a-a2ce45596230, model=/home/user/xyan/xy.ai.workbench, cwd=claude-opus-4-8
Text:
 Ich schaue mir die Implementierung an.
Control Request:
```yaml
id: 99ec0cc4-7ba2-48a6-a4d0-9a0159d1b78a
toolName: file-stats
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  reason: Datei-Größe prüfen
```
/answer 99ec0cc4-7ba2-48a6-a4d0-9a0159d1b78a allow
ReasoningToken: 11
Control Request:
```yaml
id: 82db5a2d-d671-45b9-9925-c3e419c98bca
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  reason: Ganze Datei lesen
```
/answer 82db5a2d-d671-45b9-9925-c3e419c98bca allow
Text:
 Ich schaue mir die zugehörigen Klassen an.
Control Request:
```yaml
id: 4949d906-dc71-43b1-8f02-fda8023ade26
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast
  reason: Verwandte Dateien finden
```
/answer 4949d906-dc71-43b1-8f02-fda8023ade26 allow
Control Request:
```yaml
id: 3aaabc24-04d1-4f5d-a134-621e7504bfc1
result:
  structuredContent:
    entries:
    - IDocumentBuffer.java
    - MarkdownDocument.java
    - ModificationRange.java
    - nodes/AbstractNode.java
    - nodes/Category.java
    - nodes/Elements.java
    - nodes/HeadingSection.java
    - nodes/LineCommentBlock.java
    - nodes/LineSection.java
    - nodes/Node.java
    - nodes/PageSection.java
    - nodes/Paragraph.java
    - nodes/PrefixBlock.java
    - nodes/Root.java
    - nodes/ScriptBlock.java
```
/answer 3aaabc24-04d1-4f5d-a134-621e7504bfc1 allow
Control Request:
```yaml
id: b1892639-9a4b-4549-833c-b603bfe8b739
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools
  pattern: Scanner|LineIndex
  reason: Scanner/LineIndex finden
```
/answer b1892639-9a4b-4549-833c-b603bfe8b739 allow
Control Request:
```yaml
id: 54175aaf-7f8c-4110-b113-13def27c44dc
result:
  structuredContent:
    entries:
    - LineIndex.java
    - Scanner.java
```
/answer 54175aaf-7f8c-4110-b113-13def27c44dc allow
ReasoningToken: 714
Control Request:
```yaml
id: 6e03ca62-a5a4-462d-bed8-5a4a763e94f1
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Node.java
  reason: Node-Klasse verstehen
```
/answer 6e03ca62-a5a4-462d-bed8-5a4a763e94f1 allow
Control Request:
```yaml
id: fea96421-fba5-40e4-bb99-e43ca6b9913f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/AbstractNode.java
  reason: AbstractNode verstehen
```
/answer fea96421-fba5-40e4-bb99-e43ca6b9913f allow
Control Request:
```yaml
id: a540bf09-80e6-4ed9-aaea-0a5fa802f58f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Elements.java
  reason: Element-Struktur verstehen
```
/answer a540bf09-80e6-4ed9-aaea-0a5fa802f58f allow
Control Request:
```yaml
id: 29447091-4049-4c8d-bdcc-4eaa18399e6c
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/ModificationRange.java
  reason: ModificationRange verstehen
```
/answer 29447091-4049-4c8d-bdcc-4eaa18399e6c allow
ReasoningToken: 29
Text:
 Ich lese die restlichen Node-Typen, um Verschachtelung und Terminals zu verstehen.
Control Request:
```yaml
id: a037f44a-e0a2-43c2-ab99-7e6d45b8962e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/HeadingSection.java
  reason: HeadingSection verstehen
```
/answer a037f44a-e0a2-43c2-ab99-7e6d45b8962e allow
Control Request:
```yaml
id: a062104f-2b1f-4bbf-87ea-13a4d71b3485
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Paragraph.java
  reason: Paragraph verstehen
```
/answer a062104f-2b1f-4bbf-87ea-13a4d71b3485 allow
Control Request:
```yaml
id: f1d48487-f746-4380-bdf2-ecb45810bdb7
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/PageSection.java
  reason: PageSection verstehen
```
/answer f1d48487-f746-4380-bdf2-ecb45810bdb7 allow
Control Request:
```yaml
id: 29261fb4-c5d9-43c7-9b5d-40c6d9109ca2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/LineSection.java
  reason: LineSection verstehen
```
/answer 29261fb4-c5d9-43c7-9b5d-40c6d9109ca2 allow
ReasoningToken: 5055
Control Request:
```yaml
id: 993b958d-0d05-456a-8413-007027fd6d51
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  reason: Scanner-Verhalten & Grenzen verstehen
```
/answer 993b958d-0d05-456a-8413-007027fd6d51 allow
Control Request:
```yaml
id: 704554dd-d213-4fc0-83cb-be7df26a6630
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Root.java
  reason: Root verstehen
```
/answer 704554dd-d213-4fc0-83cb-be7df26a6630 allow
ReasoningToken: 180
Text:
 Ich suche nach vorhandenen Tests für diese Klasse.
Control Request:
```yaml
id: c204d98c-59a1-4a70-9c04-7813562a2a28
toolName: bash
arguments:
  script: cd /home/user/xyan && grep -rl "MarkdownDocument\|MarkdownDocumentTest\|new MarkdownDocument" --include=*.java | head -50; echo "---- test dirs ----"; find . -type d -name "*test*" | head
  cwd: /home/user/xyan
  reason: Tests und Nutzungen finden
```
/answer c204d98c-59a1-4a70-9c04-7813562a2a28 deny es gibt das build.sh aber keine Tests
ReasoningToken: 2610
Control Request:
```yaml
id: 5250bef0-4578-449e-8267-e3d820ab80ed
toolName: bash
arguments:
  script: cd /home/user/xyan && ls -la && echo "---" && cat build.sh 2>/dev/null | head -60
  cwd: /home/user/xyan
  reason: build.sh ansehen
```
/answer 5250bef0-4578-449e-8267-e3d820ab80ed deny einfach ausführen
ReasoningToken: 147
Control Request:
```yaml
id: 2cf32d03-f7f8-4bac-bcd7-04432032bb36
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/PrefixBlock.java
  reason: PrefixBlock (Streaming-Blöcke) verstehen
```
/answer 2cf32d03-f7f8-4bac-bcd7-04432032bb36 allow
Control Request:
```yaml
id: c8246925-11b5-4914-b783-0e93d9bcc4a2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java
  reason: LineIndex verstehen
```
/answer c8246925-11b5-4914-b783-0e93d9bcc4a2 allow
ReasoningToken: 8731
Control Request:
```yaml
id: a1ce1f08-814e-487d-aa11-44d1f5d37e94
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/IDocumentBuffer.java
  reason: Buffer-Interface verstehen
```
/answer a1ce1f08-814e-487d-aa11-44d1f5d37e94 allow
ReasoningToken: 898
Text:
 Ich habe jetzt ein vollständiges Bild. Bevor ich etwas ändere, baue ich eine Verifikations-Grundlage. Zunächst das Build-Setup ansehen.
Control Request:
```yaml
id: 7df5d9df-deec-4eac-80c4-602dae31ce97
toolName: bash
arguments:
  script: cd /home/user/xyan/xy.ai.workbench 2>/dev/null && pwd && find . -maxdepth 2 -name build.sh -o -maxdepth 2 -name "*.classpath" 2>/dev/null; echo "==== find build.sh in xyan ===="; find /home/user/xyan -maxdepth 3 -name build.sh 2>/dev/null
  cwd: /home/user/xyan
  reason: build.sh lokalisieren
```
/answer 7df5d9df-deec-4eac-80c4-602dae31ce97 deny `/home/user/xyan/xy.ai.workbench/build.sh`
ReasoningToken: 1326
Control Request:
```yaml
id: 29774b5b-c5d0-4fc8-a852-50b3700fa4bd
toolName: bash
arguments:
  script: cd /home/user/xyan/xy.ai.workbench && ./build.sh 2>&1 | tail -30
  cwd: /home/user/xyan/xy.ai.workbench
  reason: Baseline-Build ausführen
```
/answer 29774b5b-c5d0-4fc8-a852-50b3700fa4bd allow
ReasoningToken: 50
Control Request:
```yaml
id: 8615f928-ca98-4ffa-a0c7-59e06895fe6e
result:
  structuredContent:
    stdout: |
      Successfull
```
/answer 8615f928-ca98-4ffa-a0c7-59e06895fe6e allow
Control Request:
```yaml
id: 0631964c-9fce-4afe-b5a9-ebce2a26e894
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/build.sh
  reason: Classpath/Compile-Setup verstehen für Test-Harness
```
/answer 0631964c-9fce-4afe-b5a9-ebce2a26e894 deny nein keine Tests
ReasoningToken: 113
Text:
 Ich habe jetzt ein vollständiges Verständnis der Architektur. Bevor ich die Optimierung implementiere, fasse ich mein Verständnis der relevanten Mechanik zusammen und setze sie dann um.

Kernpunkte:
- Node-Offsets (`start`/`end`) sind **relativ zum jeweiligen Parent**; `getOffset()` summiert die Kette auf.
- `parse(absStart, …)` liefert eine flache ROOT-Liste; da die Block-Grammatik (Headings/Terminals) kontextfrei über globale Konstanten definiert ist, ist die Verschachtelung eines Knotens unabhängig davon, wo er hängt.
- Der teure Fall ist das **Hochklettern** (`sec = parent`), bei dem die ganze Parent-Sektion ab `parent.getOffset()` neu geparsed wird — obwohl beim reinen Anhängen alles vor `sec` unverändert ist.

Ich implementiere daher einen sicheren Fast-Path: Wenn `isCompatible` nur scheitert, weil hinten neue Geschwister angehängt wurden, die nicht in `parent` passen, wird nur der (ohnehin minimale) Reparse ab `sec` verwendet, in `head` (bleibt in `parent`) und `tail` (gehört in einen Vorfahren) aufgeteilt und eingespleißt — statt den ganzen Parent neu zu parsen. Bei jeder nicht sicher beweisbaren Konstellation wird auf das bisherige Verhalten zurückgefallen.
Control Request:
```yaml
id: 45c72178-c279-4112-a7e3-370a6be4e906
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  old_text: "\t\t\tif (parent == null || isCompatible(rn.children, sec, parent)) {\n\t\t\t\tNode changed = replace(sec, rn.children, delta);\n\t\t\t\treturn expand(changed, offset, offset + inserted);\n\t\t\t}\n\t\t\tsec = parent;\n\t\t}\n\t}"
  new_text: "\t\t\tif (parent == null || isCompatible(rn.children, sec, parent)) {\n\t\t\t\tNode changed = replace(sec, rn.children, delta);\n\t\t\t\treturn expand(changed, offset, offset + inserted);\n\t\t\t}\n\n\t\t\t// Fast path: only new siblings were appended at the tail that do not\n\t\t\t// fit into `parent` but belong to an ancestor. Splice them in instead\n\t\t\t// of re-parsing the whole parent section on the next loop iteration.\n\t\t\tNode appended = appendSiblings(sec, parent, rn.children, delta);\n\t\t\tif (appended != null)\n\t\t\t\treturn expand(appended, offset, offset + inserted);\n\n\t\t\tsec = parent;\n\t\t}\n\t}\n\n\t/**\n\t * Handles the common \"append within a section\" case without re-parsing the\n\t * whole parent. The re-parse (anchored at {@code sec}) is split into a\n\t * {@code head} that stays inside {@code parent} (starting with the re-parsed\n\t * {@code sec}) and a {@code tail} of newly appended nodes that overflow into\n\t * the nearest ancestor able to contain them.\n\t *\n\t * <p>\n\t * Only applied when the edit is provably a clean tail append (spine of\n\t * last-children, every touched ancestor ending exactly at {@code sec}).\n\t * Returns the highest changed node, or {@code null} to fall back to the\n\t * generic re-parse/climb behavior.\n\t */\n\tprivate Node appendSiblings(Node sec, Node parent, List<Node> rchilds, int delta) {\n\t\tif (rchilds.isEmpty() || rchilds.get(0).instance != sec.instance)\n\t\t\treturn null;\n\t\tif (!isSpineTail(sec, parent))\n\t\t\treturn null;\n\n\t\tint absStart = sec.getOffset();\n\t\tint oldSecEnd = absStart + sec.length();\n\n\t\t// Split re-parsed nodes: leading nodes that fit into parent vs. overflow.\n\t\tint split = 1; // rchilds[0] mirrors sec and therefore fits into parent\n\t\twhile (split < rchilds.size() && parent.instance.containChild(rchilds.get(split).instance))\n\t\t\tsplit++;\n\t\tif (split == rchilds.size())\n\t\t\treturn null; // nothing overflows -> handled by isCompatible\n\n\t\tList<Node> head = rchilds.subList(0, split);\n\t\tList<Node> tail = rchilds.subList(split, rchilds.size());\n\n\t\t// Find the nearest ancestor able to host the whole overflow while every\n\t\t// intermediate ancestor cleanly ends at sec (last-child, no trailing).\n\t\tNode host = null;\n\t\tfor (Node anc = parent.parent; anc != null; anc = anc.parent) {\n\t\t\tif (anc.getEndOffset() != oldSecEnd)\n\t\t\t\treturn null;\n\t\t\tif (canContainAll(anc, tail)) {\n\t\t\t\thost = anc;\n\t\t\t\tbreak;\n\t\t\t}\n\t\t\tif (!isLastChild(anc))\n\t\t\t\treturn null;\n\t\t}\n\t\tif (host == null)\n\t\t\treturn null;\n\n\t\tint headEndAbs = absStart + head.get(head.size() - 1).end;\n\t\tint tailEndAbs = absStart + tail.get(tail.size() - 1).end;\n\n\t\t// 1) replace sec with head inside parent\n\t\tList<Node> siblings = parent.children;\n\t\tsiblings.remove(siblings.size() - 1); // sec is the last child\n\t\tfor (Node c : head) {\n\t\t\tc.start += sec.start;\n\t\t\tc.end += sec.start;\n\t\t\tsiblings.add(reparent(c, parent));\n\t\t}\n\n\t\t// 2) parent and every ancestor up to host now end after the head content\n\t\tfor (Node anc = parent; anc != host; anc = anc.parent)\n\t\t\tsetEndOffset(anc, headEndAbs);\n\n\t\t// 3) attach the overflow as new trailing children of host\n\t\tint hostOffset = host.getOffset();\n\t\tfor (Node c : tail) {\n\t\t\tc.start = absStart + c.start - hostOffset;\n\t\t\tc.end = absStart + c.end - hostOffset;\n\t\t\thost.children.add(reparent(c, host));\n\t\t}\n\t\tsetEndOffset(host, tailEndAbs);\n\n\t\t// 4) propagate the growth to the ancestors above host\n\t\tfor (Node anc = host.parent; anc != null; anc = anc.parent) {\n\t\t\tanc.end += delta;\n\t\t\tNode ap = anc.parent;\n\t\t\tif (ap == null)\n\t\t\t\tcontinue;\n\t\t\tList<Node> as = ap.children;\n\t\t\tint ai = as.indexOf(anc);\n\t\t\tfor (int i = ai + 1; i < as.size(); i++)\n\t\t\t\tshift(as.get(i), delta);\n\t\t}\n\t\treturn host;\n\t}\n\n\tprivate boolean isSpineTail(Node sec, Node parent) {\n\t\tList<Node> siblings = parent.children;\n\t\tif (siblings.isEmpty() || siblings.get(siblings.size() - 1) != sec)\n\t\t\treturn false;\n\t\treturn parent.getEndOffset() == sec.getEndOffset();\n\t}\n\n\tprivate boolean isLastChild(Node node) {\n\t\tNode parent = node.parent;\n\t\tif (parent == null)\n\t\t\treturn false;\n\t\tList<Node> siblings = parent.children;\n\t\treturn !siblings.isEmpty() && siblings.get(siblings.size() - 1) == node;\n\t}\n\n\tprivate boolean canContainAll(Node parent, List<Node> nodes) {\n\t\tfor (Node n : nodes)\n\t\t\tif (!parent.instance.containChild(n.instance))\n\t\t\t\treturn false;\n\t\treturn true;\n\t}\n\n\tprivate void setEndOffset(Node node, int absEnd) {\n\t\tnode.end = absEnd - (node.parent != null ? node.parent.getOffset() : 0);\n\t}"
  reason: Fast-Path für reines Anhängen implementieren
```
/answer 45c72178-c279-4112-a7e3-370a6be4e906 allow
Control Request:
```yaml
id: baa5ad96-8c02-45c3-937c-8d052e85245f
toolName: bash
arguments:
  script: cd /home/user/xyan/xy.ai.workbench && ./build.sh 2>&1 | tail -30
  cwd: /home/user/xyan/xy.ai.workbench
  reason: Build nach Änderung prüfen
```
/answer baa5ad96-8c02-45c3-937c-8d052e85245f allow
Control Request:
```yaml
id: 0805acb3-d237-4d1f-9dd6-1a07c72127c3
result:
  structuredContent:
    stdout: |
      Successfull
```
/answer 0805acb3-d237-4d1f-9dd6-1a07c72127c3 allow
Control Request:
```yaml
id: ff66cf39-2074-472b-9c47-f9b27b659949
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java
  reason: Endfassung prüfen
  start: "\tpublic ModificationRange update"
  end: "\tprivate void setEndOffset"
```
/answer ff66cf39-2074-472b-9c47-f9b27b659949 allow
Control Request:
```yaml
id: 688186d3-7ff3-4aba-8f52-025d6d76a5c2
result:
  structuredContent:
    checksum: ef8328a6a14162d5a1c7986ca7ea014484ef3dd0c7ff98aeca1e68aafccab4d2
    content: "\tpublic ModificationRange update(int offset, int removed, int inserted) {\n\t\tint lo = offset;\n\t\tint hi = offset + removed;\n\t\tint delta = inserted - removed;\n\t\tlines.update(buffer, offset, removed, inserted);\n\n\t\tNode sec = findForUpdate(lo, hi);\n\t\twhile (true) {\n\t\t\tNode parent = sec.parent;\n\t\t\tint absStart = sec.getOffset();\n\t\t\tint newLen = Math.max(hi - absStart, sec.length()) + delta;\n\t\t\tNode rn = parse(absStart, absStart + newLen);\n\n\t\t\tif (parent == null || isCompatible(rn.children, sec, parent)) {\n\t\t\t\tNode changed = replace(sec, rn.children, delta);\n\t\t\t\treturn expand(changed, offset, offset + inserted);\n\t\t\t}\n\n\t\t\t// Fast path: only new siblings were appended at the tail that do not\n\t\t\t// fit into `parent` but belong to an ancestor. Splice them in instead\n\t\t\t// of re-parsing the whole parent section on the next loop iteration.\n\t\t\tNode appended = appendSiblings(sec, parent, rn.children, delta);\n\t\t\tif (appended != null)\n\t\t\t\treturn expand(appended, offset, offset + inserted);\n\n\t\t\tsec = parent;\n\t\t}\n\t}\n\n\t/**\n\t * Handles the common \"append within a section\" case without re-parsing the\n\t * whole parent. The re-parse (anchored at {@code sec}) is split into a\n\t * {@code head} that stays inside {@code parent} (starting with the re-parsed\n\t * {@code sec}) and a {@code tail} of newly appended nodes that overflow into\n\t * the nearest ancestor able to contain them.\n\t *\n\t * <p>\n\t * Only applied when the edit is provably a clean tail append (spine of\n\t * last-children, every touched ancestor ending exactly at {@code sec}).\n\t * Returns the highest changed node, or {@code null} to fall back to the\n\t * generic re-parse/climb behavior.\n\t */\n\tprivate Node appendSiblings(Node sec, Node parent, List<Node> rchilds, int delta) {\n\t\tif (rchilds.isEmpty() || rchilds.get(0).instance != sec.instance)\n\t\t\treturn null;\n\t\tif (!isSpineTail(sec, parent))\n\t\t\treturn null;\n\n\t\tint absStart = sec.getOffset();\n\t\tint oldSecEnd = absStart + sec.length();\n\n\t\t// Split re-parsed nodes: leading nodes that fit into parent vs. overflow.\n\t\tint split = 1; // rchilds[0] mirrors sec and therefore fits into parent\n\t\twhile (split < rchilds.size() && parent.instance.containChild(rchilds.get(split).instance))\n\t\t\tsplit++;\n\t\tif (split == rchilds.size())\n\t\t\treturn null; // nothing overflows -> handled by isCompatible\n\n\t\tList<Node> head = rchilds.subList(0, split);\n\t\tList<Node> tail = rchilds.subList(split, rchilds.size());\n\n\t\t// Find the nearest ancestor able to host the whole overflow while every\n\t\t// intermediate ancestor cleanly ends at sec (last-child, no trailing).\n\t\tNode host = null;\n\t\tfor (Node anc = parent.parent; anc != null; anc = anc.parent) {\n\t\t\tif (anc.getEndOffset() != oldSecEnd)\n\t\t\t\treturn null;\n\t\t\tif (canContainAll(anc, tail)) {\n\t\t\t\thost = anc;\n\t\t\t\tbreak;\n\t\t\t}\n\t\t\tif (!isLastChild(anc))\n\t\t\t\treturn null;\n\t\t}\n\t\tif (host == null)\n\t\t\treturn null;\n\n\t\tint headEndAbs = absStart + head.get(head.size() - 1).end;\n\t\tint tailEndAbs = absStart + tail.get(tail.size() - 1).end;\n\n\t\t// 1) replace sec with head inside parent\n\t\tList<Node> siblings = parent.children;\n\t\tsiblings.remove(siblings.size() - 1); // sec is the last child\n\t\tfor (Node c : head) {\n\t\t\tc.start += sec.start;\n\t\t\tc.end += sec.start;\n\t\t\tsiblings.add(reparent(c, parent));\n\t\t}\n\n\t\t// 2) parent and every ancestor up to host now end after the head content\n\t\tfor (Node anc = parent; anc != host; anc = anc.parent)\n\t\t\tsetEndOffset(anc, headEndAbs);\n\n\t\t// 3) attach the overflow as new trailing children of host\n\t\tint hostOffset = host.getOffset();\n\t\tfor (Node c : tail) {\n\t\t\tc.start = absStart + c.start - hostOffset;\n\t\t\tc.end = absStart + c.end - hostOffset;\n\t\t\thost.children.add(reparent(c, host));\n\t\t}\n\t\tsetEndOffset(host, tailEndAbs);\n\n\t\t// 4) propagate the growth to the ancestors above host\n\t\tfor (Node anc = host.parent; anc != null; anc = anc.parent) {\n\t\t\tanc.end += delta;\n\t\t\tNode ap = anc.parent;\n\t\t\tif (ap == null)\n\t\t\t\tcontinue;\n\t\t\tList<Node> as = ap.children;\n\t\t\tint ai = as.indexOf(anc);\n\t\t\tfor (int i = ai + 1; i < as.size(); i++)\n\t\t\t\tshift(as.get(i), delta);\n\t\t}\n\t\treturn host;\n\t}\n\n\tprivate boolean isSpineTail(Node sec, Node parent) {\n\t\tList<Node> siblings = parent.children;\n\t\tif (siblings.isEmpty() || siblings.get(siblings.size() - 1) != sec)\n\t\t\treturn false;\n\t\treturn parent.getEndOffset() == sec.getEndOffset();\n\t}\n\n\tprivate boolean isLastChild(Node node) {\n\t\tNode parent = node.parent;\n\t\tif (parent == null)\n\t\t\treturn false;\n\t\tList<Node> siblings = parent.children;\n\t\treturn !siblings.isEmpty() && siblings.get(siblings.size() - 1) == node;\n\t}\n\n\tprivate boolean canContainAll(Node parent, List<Node> nodes) {\n\t\tfor (Node n : nodes)\n\t\t\tif (!parent.instance.containChild(n.instance))\n\t\t\t\treturn false;\n\t\treturn true;\n\t}\n\n\tprivate void setEndOffset"
```
/answer 688186d3-7ff3-4aba-8f52-025d6d76a5c2 allow
ReasoningToken: 20473
Text:
 Jetzt kompiliere ich, um sicherzustellen, dass alles baut.
Text:
 Der Build ist erfolgreich. Lass mich die finale Fassung noch einmal durchsehen.
ReasoningToken: 22
ReasoningToken: 55
Der Build ist grün. Hier eine Zusammenfassung.

## Was ich gemacht habe

Ich habe die Architektur analysiert (`MarkdownDocument`, `Node`, `AbstractNode.scan`, die Node-Typen und den `Scanner`) und die von dir beschriebene Optimierung in `update(...)` umgesetzt.

### Ausgangslage / Problem
- Der bestehende Fast-Path (`findForUpdate` → letztes Leaf + `isCompatible`) deckt bereits die Fälle ab, in denen das letzte Child erweitert wird **oder** neue Geschwister angehängt werden, **die in denselben Parent passen** (`isCompatible` prüft, dass *alle* reparsten Knoten `containChild`-fähig sind).
- Sobald aber ein angehängtes Geschwister strukturell **höher** gehört (z. B. ein `# Heading` hinter einem Paragraph in einer `## Sektion`), scheitert `isCompatible`, es wird `sec = parent` gesetzt und im nächsten Durchlauf die **gesamte** Parent-Sektion ab `parent.getOffset()` neu geparsed – genau das überflüssige Neuparsen.

### Die Optimierung (`appendSiblings`)
Zwischen dem `isCompatible`-Zweig und dem Hochklettern setzt jetzt ein Fast-Path an, der den ohnehin minimalen Reparse ab `sec` wiederverwendet, statt den Parent komplett neu zu parsen:

1. Der Reparse wird in `head` (führende Knoten, die in `parent` passen – beginnend mit dem reparsten `sec`) und `tail` (überlaufende neue Knoten) aufgeteilt.
2. `head` ersetzt `sec` in `parent` (mehrere Geschwister möglich).
3. `tail` wird an den **nächstgelegenen Vorfahren** gehängt, der ihn aufnehmen kann – exakt die Verschachtelung, die ein Vollreparse ergäbe.
4. Die Ends von `parent`/Zwischen-Vorfahren und dem Host werden gesetzt, oberhalb wird `+= delta` propagiert (wie im bestehenden `replace`).

### Sicherheit / Korrektheit
- Genutzt wird, dass die Block-Grammatik kontextfrei über globale Terminals definiert ist – die Verschachtelung eines Knotens ist unabhängig davon, wo er hängt; unveränderter Inhalt vor `sec` ergäbe bei einem Vollreparse identische Knoten.
- Der Fast-Path greift **nur** bei einem beweisbar sauberen Tail-Append: `rchilds[0]` entspricht `sec`, `sec` ist letztes Child, und jeder betroffene Vorfahre endet exakt bei `sec` (Last-Child-Spine, kein nachlaufender Text). In allen anderen Fällen wird `null` zurückgegeben und das **bisherige Verhalten** (Hochklettern + Vollreparse) läuft unverändert weiter – schlimmstenfalls also kein Nachteil gegenüber vorher.

Da du gesagt hast, dass es keine Tests gibt, habe ich nur über `build.sh` verifiziert (`Successfull`). Wenn du möchtest, kann ich den Rückgabewert-`node` für den `expand`-Aufruf noch feiner wählen (aktuell der Host, was dem bisherigen Klettern-Verhalten entspricht).
