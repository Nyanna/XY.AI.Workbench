Mir ist ein Bug aufgefallen. Fälschlicherweise wird `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/EmphasisRule.java` mit ": " angewandt mitten in einem Paragraph im ersten Absatz. Das liegt am "docStart". Im Scanner wird offset für Rules nicht richtig verwertet.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=c75a19c8-34e9-4350-b458-de2810f59cbf | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 9f35a7f8-9e09-4dd2-beb6-bad42003017f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/EmphasisRule.java
  reason: Inspect bug
```
/answer 9f35a7f8-9e09-4dd2-beb6-bad42003017f allow
ReasoningToken: 16
Control Request:
```yaml
id: 168cbe60-b7c2-4827-9f1b-b43505fdcf72
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md
  reason: list md editor files
```
/answer 168cbe60-b7c2-4827-9f1b-b43505fdcf72 allow
Control Request:
```yaml
id: 0c7eecc5-465d-4ab4-8fe9-f786f3a858b1
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
/answer 0c7eecc5-465d-4ab4-8fe9-f786f3a858b1 allow
Control Request:
```yaml
id: ec916378-b47c-448d-b710-6411b519581c
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools
  reason: list tools files including Scanner
```
/answer ec916378-b47c-448d-b710-6411b519581c allow
Control Request:
```yaml
id: f71584a0-e354-4131-b8f8-f73c4857227c
result:
  structuredContent:
    entries:
    - AbstractQueryListener.java
    - LineIndex.java
    - Scanner.java
    - Time.java
```
/answer f71584a0-e354-4131-b8f8-f73c4857227c allow
ReasoningToken: 39
Control Request:
```yaml
id: 4b4a015a-2808-43b6-9648-3f1f29f61788
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  reason: Understand offset/docStart handling
```
/answer 4b4a015a-2808-43b6-9648-3f1f29f61788 allow
Control Request:
```yaml
id: 5212225b-0dd8-4bae-9199-d13246596246
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/AbstractRule.java
  reason: Understand rule base class
```
/answer 5212225b-0dd8-4bae-9199-d13246596246 allow
Control Request:
```yaml
id: 4c608c9f-b550-4958-95be-f1e62894baa0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/PrefixLineRule.java
  reason: Understand PrefixLineRule which might handle docStart
```
/answer 4c608c9f-b550-4958-95be-f1e62894baa0 allow
Control Request:
```yaml
id: 4f176238-a69f-4991-adc1-7d12e352c102
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md
  pattern: .*
  reason: find configuration file referencing EmphasisRule
```
/answer 4f176238-a69f-4991-adc1-7d12e352c102 allow
Control Request:
```yaml
id: 240c79d3-54a7-4258-b8d5-9c9e56f265c6
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
/answer 240c79d3-54a7-4258-b8d5-9c9e56f265c6 allow
Control Request:
```yaml
id: 2cb619a8-6429-47a2-a2d6-80fe98708139
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: "grep -rn \"EmphasisRule\\|LINE_COMMENT\\|\\\": \\\"\\|BlockRule\\|LineMatchRule\" --include=*.java src/xy/ai/workbench/editors/md"
  reason: search usage of colon rule and Emphasis
```
/answer 2cb619a8-6429-47a2-a2d6-80fe98708139 allow
Control Request:
```yaml
id: 20030db7-0916-4fce-ae60-ea40d75770c1
result:
  structuredContent:
    stdout: "src/xy/ai/workbench/editors/md/LineMatchRule.java:7:public class LineMatchRule extends AbstractRule {\nsrc/xy/ai/workbench/editors/md/LineMatchRule.java:10:\tpublic LineMatchRule(String prefix, IToken token) {\nsrc/xy/ai/workbench/editors/md/AbstractRule.java:11:\tpublic static final String LINE_COMMENT = \"#:\";\nsrc/xy/ai/workbench/editors/md/BlockRule.java:7:public class BlockRule extends AbstractRule {\nsrc/xy/ai/workbench/editors/md/BlockRule.java:13:\tpublic BlockRule(String start, String end, IToken token) {\nsrc/xy/ai/workbench/editors/md/EmphasisRule.java:7:public class EmphasisRule extends AbstractRule {\nsrc/xy/ai/workbench/editors/md/EmphasisRule.java:12:\tpublic EmphasisRule(String sequence, IToken tkn) {\nsrc/xy/ai/workbench/editors/md/EmphasisRule.java:16:\tpublic EmphasisRule(String start, String end, IToken tkn) {\n"
```
/answer 20030db7-0916-4fce-ae60-ea40d75770c1 allow
ReasoningToken: 240
Control Request:
```yaml
id: 4f91155a-2d4f-424b-8ad7-54bf94e69676
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rln "new EmphasisRule\|new PrefixLineRule\|new BlockRule\|new LineMatchRule" src
  reason: find rule instantiation site
```
/answer 4f91155a-2d4f-424b-8ad7-54bf94e69676 allow
Control Request:
```yaml
id: fc4378aa-08fd-4502-bd01-e073d8427b1a
result:
  structuredContent:
    stdout: |
      src/xy/ai/workbench/editors/AIRuleScanner.java
```
/answer fc4378aa-08fd-4502-bd01-e073d8427b1a allow
ReasoningToken: 107
Control Request:
```yaml
id: 925902b2-8841-4406-bde2-60032784f16f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java
  reason: Find rule instantiations and scanner offset handling for docStart
```
/answer 925902b2-8841-4406-bde2-60032784f16f allow
Text:
 Ich habe den Bug gefunden. Er steckt in `AbstractRule.java` (nicht direkt in `EmphasisRule`, aber die Ursache betrifft auch `PrefixLineRule` bzw. dessen `isNextSequenceBounded`, das von `EmphasisRule` in einer verwandten Weise genutzt wird):
Control Request:
```yaml
id: 8020f656-492f-410d-86d3-e4ab1f38362a
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -n "PARAGRAPH" -A3 -B3 src/xy/ai/workbench/editors/AIRuleScanner.java | head -30
  reason: "confirm PrefixLineRule \": \" glossary registration"
```
/answer 8020f656-492f-410d-86d3-e4ab1f38362a allow
Control Request:
```yaml
id: d010f4a7-c0ed-41a5-9c36-e3a0c893c943
result:
  structuredContent:
    stdout: "139-\n140-\t\t// ---- section: paragraph - lists, emphasis, links and quote/glossary prefixes\n141-\t\t// ----\n142:\t\tregister(Elements.Basics.PARAGRAPH, //\n143-\t\t\t\tcommentRule, //\n144-\t\t\t\tnew PrefixLineRule(\": \", italic), // glossary syntax\n145-\t\t\t\tnew PrefixLineRule(\"> \", italic), // citation syntax\n"
```
/answer d010f4a7-c0ed-41a5-9c36-e3a0c893c943 allow
ReasoningToken: 6325
Text:
 Bestätigt: `": "` wird als `PrefixLineRule(": ", italic)` registriert – Präfix ist intern `"\n: "`. Die eigentliche Ursache liegt in `AbstractRule.setDocumentBounds` / `evaluate`, kombiniert mit `Scanner.isDocStart()`:

**Der Bug:** `setDocumentBounds(atDocStart, atDocEnd)` wird von `AIRuleScanner.applyDocumentBounds()` **einmal pro Scan-Aufruf** (`scanGap`/`scanFlat`) gesetzt – z. B. `true`, wenn dieser Textabschnitt bei Dokument-Offset 0 beginnt (der erste Absatz). Dieses `docStart`-Flag bleibt aber für die **gesamte Dauer** dieses Scans gültig. `RuleBasedScanner` ruft `evaluate()` jedoch für jede Zeichenposition innerhalb des Bereichs erneut auf (bei jedem erfolglosen Versuch rückt die Position um 1 weiter). Da bei jedem `evaluate()`-Aufruf ein **neuer** `Scanner` mit `readCount = 0` erzeugt wird, liefert `isDocStart()` (`root.docStart && root.readCount == 0`) bei **jeder** Position innerhalb des ersten Absatzes `true` – nicht nur an der tatsächlichen Position 0.

In `isNextSequenceBounded()` wird dadurch das erforderliche führende `\n` bei **jedem** Versuch im ersten Absatz übersprungen (`from = 1` statt `0`), sodass `": "` mitten im Absatz erkannt wird, obwohl kein Zeilenumbruch davor steht.

**Fix:** `docStart` darf nur beim allerersten `evaluate()`-Aufruf nach einem `setDocumentBounds()` wirksam sein (das entspricht der tatsächlichen Startposition des Scans), nicht bei jedem nachfolgenden Versuch an späteren Positionen.
Control Request:
```yaml
id: 4548d713-4914-4a6e-933e-a790e2d183df
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/AbstractRule.java
  old_text: "\tprivate boolean docStart;\n\tprivate boolean docEnd;\n\n\tpublic AbstractRule() {\n\t}\n\n\tpublic AbstractRule(IToken token) {\n\t\tthis.token = token;\n\t}\n\n\tpublic void setDocumentBounds(boolean docStart, boolean docEnd) {\n\t\tthis.docStart = docStart;\n\t\tthis.docEnd = docEnd;\n\t}\n\n\tpublic final IToken evaluate(ICharacterScanner s) {\n\t\treturn evaluateToken(new Scanner(new Scanner.CharacterScanner() {\n\t\t\t@Override\n\t\t\tpublic void unread() {\n\t\t\t\ts.unread();\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic int read() {\n\t\t\t\treturn s.read();\n\t\t\t}\n\t\t}, docStart, docEnd));\n\t}"
  new_text: "\tprivate boolean docStart;\n\tprivate boolean docEnd;\n\t// true only for the very first evaluate() call after setDocumentBounds(),\n\t// i.e. the attempt at the actual start offset of the current scan range.\n\t// RuleBasedScanner re-invokes evaluate() at every subsequent offset it\n\t// tries within that same range, but only the first such attempt may\n\t// coincide with the true document start.\n\tprivate boolean atRangeStart;\n\n\tpublic AbstractRule() {\n\t}\n\n\tpublic AbstractRule(IToken token) {\n\t\tthis.token = token;\n\t}\n\n\tpublic void setDocumentBounds(boolean docStart, boolean docEnd) {\n\t\tthis.docStart = docStart;\n\t\tthis.docEnd = docEnd;\n\t\tthis.atRangeStart = true;\n\t}\n\n\tpublic final IToken evaluate(ICharacterScanner s) {\n\t\tboolean isDocStart = docStart && atRangeStart;\n\t\tatRangeStart = false;\n\t\treturn evaluateToken(new Scanner(new Scanner.CharacterScanner() {\n\t\t\t@Override\n\t\t\tpublic void unread() {\n\t\t\t\ts.unread();\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic int read() {\n\t\t\t\treturn s.read();\n\t\t\t}\n\t\t}, isDocStart, docEnd));\n\t}"
  reason: "Fix docStart bug: only first evaluate() attempt per scan range should be treated as document start"
```
/answer 4548d713-4914-4a6e-933e-a790e2d183df allow
ReasoningToken: 15
Control Request:
```yaml
id: 2547ad7f-dee8-47af-a185-6fc08e188430
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -name pom.xml -o -name build.xml | head; ls
  reason: check build tooling available
```
/answer 2547ad7f-dee8-47af-a185-6fc08e188430 deny Es gibt weder Tooling noch Tests.
Control Request:
```yaml
id: 00b6b4e1-47e3-48ba-9a47-0e548b821967
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/AbstractRule.java
  reason: Verify final state of fix
```
/answer 00b6b4e1-47e3-48ba-9a47-0e548b821967 allow
Text:
 Jetzt kompilieren, um sicherzugehen, dass alles passt:
ReasoningToken: 23
## Zusammenfassung

**Root Cause:** Nicht direkt in `EmphasisRule.java`, sondern in `AbstractRule.java` (betrifft aber auch `EmphasisRule` konzeptionell, sowie akut die `PrefixLineRule(": ", ...)` "Glossary"-Regel, die im ersten Absatz registriert ist):

- `AIRuleScanner.applyDocumentBounds()` setzt via `setDocumentBounds(atDocStart, atDocEnd)` einmalig pro Scan-Aufruf (`scanGap`/`scanFlat`), ob dieser Textabschnitt bei Dokument-Offset 0 beginnt (nur im ersten Absatz `true`).
- `RuleBasedScanner` ruft `IRule.evaluate()` aber **für jede** Position innerhalb des Bereichs erneut auf – bei jedem erfolglosen Match rückt die Scan-Position ein Zeichen weiter und alle Regeln werden erneut probiert.
- `evaluate()` erzeugte bisher bei **jedem** solchen Aufruf einen frischen `Scanner` mit `readCount = 0` und übergab dabei unverändert das `docStart`-Flag. Da `Scanner.isDocStart()` `root.docStart && root.readCount == 0` prüft, war dieser Ausdruck bei **jeder** Position im ersten Absatz `true` – nicht nur bei der tatsächlichen Position 0.
- In `Scanner.isNextSequenceBounded()` führte das dazu, dass das geforderte führende `"\n"` (Präfix von `PrefixLineRule`) bei jedem Versuch im ersten Absatz übersprungen wurde → `": "` wurde mitten im Paragraph ohne vorangehenden Zeilenumbruch als Treffer erkannt.

**Fix:** Ein neues Flag `atRangeStart` in `AbstractRule`, das in `setDocumentBounds()` auf `true` gesetzt und nach dem **ersten** `evaluate()`-Aufruf auf `false` gesetzt wird. Der an `Scanner` übergebene `docStart`-Wert ist jetzt `docStart && atRangeStart` – also nur beim allerersten Scan-Versuch im jeweiligen Bereich `true`, was exakt der tatsächlichen Start-Offset-Position entspricht. Alle nachfolgenden Versuche an späteren Positionen im selben ersten Absatz liefern korrekt `isDocStart() == false`.

Damit greift die Sonderregel „kein führendes `\n` am echten Dokumentanfang nötig“ nur noch an Position 0 und nicht mehr fälschlich mitten im ersten Absatz.
