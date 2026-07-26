Implementiere für `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/AbstractRule.java` den "docStart" und "docEnd" Support in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java`.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=ec7c7902-d17e-4a8d-a0ac-b4ae995c307a | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 186532fe-7125-40f6-bd04-5a868b1e938e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/AbstractRule.java
  reason: Inspect AbstractRule for docStart/docEnd fields
```
/answer 186532fe-7125-40f6-bd04-5a868b1e938e allow
Control Request:
```yaml
id: c65381eb-a179-496a-a4fc-6ffcdd61c74d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java
  reason: Inspect AIRuleScanner to add docStart/docEnd support
```
/answer c65381eb-a179-496a-a4fc-6ffcdd61c74d allow
ReasoningToken: 10
Control Request:
```yaml
id: 6ee9bb36-858f-45fe-ac07-e56b733b7d58
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md
  reason: List md rule files
```
/answer 6ee9bb36-858f-45fe-ac07-e56b733b7d58 allow
Control Request:
```yaml
id: e73d1ee4-7e11-49be-abff-124cfa7fddd1
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
/answer e73d1ee4-7e11-49be-abff-124cfa7fddd1 allow
Control Request:
```yaml
id: be82826d-0215-479f-a191-9c4e4f759cf9
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: grep -rn "docStart\|docEnd" --include=*.java .
  reason: search for existing docStart/docEnd usage
```
/answer be82826d-0215-479f-a191-9c4e4f759cf9 allow
Control Request:
```yaml
id: ea0c10eb-1036-4db1-88d3-17cab95861bc
result:
  structuredContent:
    stdout: "./src/xy/ai/workbench/tools/Scanner.java:12:\tprivate boolean docStart;\n./src/xy/ai/workbench/tools/Scanner.java:13:\tprivate boolean docEnd;\n./src/xy/ai/workbench/tools/Scanner.java:43:\tpublic Scanner(CharacterScanner scan, boolean docStart, boolean docEnd) {\n./src/xy/ai/workbench/tools/Scanner.java:45:\t\tthis.docStart = docStart;\n./src/xy/ai/workbench/tools/Scanner.java:46:\t\tthis.docEnd = docEnd;\n./src/xy/ai/workbench/tools/Scanner.java:117:\t\treturn root.docStart && root.readCount == 0;\n./src/xy/ai/workbench/tools/Scanner.java:124:\t\treturn root.docEnd;\n"
```
/answer ea0c10eb-1036-4db1-88d3-17cab95861bc allow
ReasoningToken: 115
Control Request:
```yaml
id: 13e91e50-e661-4de9-94aa-410012579639
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  reason: Understand docStart/docEnd semantics in Scanner
```
/answer 13e91e50-e661-4de9-94aa-410012579639 allow
ReasoningToken: 55
Control Request:
```yaml
id: a6955e63-960a-49f3-b4fe-f9db6bcc4d12
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/BlockRule.java
  reason: Check BlockRule usage of docStart/docEnd
```
/answer a6955e63-960a-49f3-b4fe-f9db6bcc4d12 allow
Control Request:
```yaml
id: c46f97e9-71a1-4630-8fdc-99be7d7a3611
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/HeaderRule.java
  reason: Check HeaderRule usage of docStart/docEnd
```
/answer c46f97e9-71a1-4630-8fdc-99be7d7a3611 allow
Control Request:
```yaml
id: 599f7b3f-ad4c-491c-968b-4db71bb39fbd
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/PrefixLineRule.java
  reason: Check PrefixLineRule usage
```
/answer 599f7b3f-ad4c-491c-968b-4db71bb39fbd allow
Control Request:
```yaml
id: 25583156-dd14-4980-812b-796e330486cd
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/LineMatchRule.java
  reason: Check LineMatchRule usage
```
/answer 25583156-dd14-4980-812b-796e330486cd allow
ReasoningToken: 1240
Control Request:
```yaml
id: 8762f50c-f146-4fe1-a416-fa5f68da494a
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md
  script: grep -l "extends AbstractRule" *.java; echo ---; grep -n "class " *.java
  reason: check which rules extend AbstractRule
```
/answer 8762f50c-f146-4fe1-a416-fa5f68da494a allow
Control Request:
```yaml
id: 3cf02146-77f4-46df-a9e6-a82ecfb6cba2
result:
  structuredContent:
    stdout: |
      BlockRule.java
      EmphasisRule.java
      HeaderRule.java
      LineMatchRule.java
      LinkRule.java
      ListRule.java
      PrefixLineRule.java
      WordRule.java
      ---
      AbstractRule.java:10:public abstract class AbstractRule implements IRule {
      BlockRule.java:7:public class BlockRule extends AbstractRule {
      EmphasisRule.java:7:public class EmphasisRule extends AbstractRule {
      HeaderRule.java:7:public class HeaderRule extends AbstractRule {
      LineMatchRule.java:7:public class LineMatchRule extends AbstractRule {
      LinkRule.java:22:public class LinkRule extends AbstractRule {
      ListRule.java:7:public class ListRule extends AbstractRule {
      PrefixLineRule.java:7:public class PrefixLineRule extends AbstractRule {
      WordRule.java:7:public class WordRule extends AbstractRule {
```
/answer 3cf02146-77f4-46df-a9e6-a82ecfb6cba2 allow
ReasoningToken: 1498
Control Request:
```yaml
id: 505698ff-53d8-4efc-8432-6db568bedafd
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/AbstractRule.java
  old_text: "public abstract class AbstractRule implements IRule {\n\tpublic static final String LINE_COMMENT = \"#:\";\n\tprivate IToken token = Token.UNDEFINED;\n\n\tpublic AbstractRule() {\n\t}\n\n\tpublic AbstractRule(IToken token) {\n\t\tthis.token = token;\n\t}\n\n\tpublic final IToken evaluate(ICharacterScanner s) {\n\t\treturn evaluateToken(new Scanner(new Scanner.CharacterScanner() {\n\t\t\t@Override\n\t\t\tpublic void unread() {\n\t\t\t\ts.unread();\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic int read() {\n\t\t\t\treturn s.read();\n\t\t\t}\n\t\t}));\n\t}"
  new_text: "public abstract class AbstractRule implements IRule {\n\tpublic static final String LINE_COMMENT = \"#:\";\n\tprivate IToken token = Token.UNDEFINED;\n\n\t/**\n\t * Whether the current scan position/range abuts the real start/end of the\n\t * document. Set by the driving token scanner (see AIRuleScanner) before each\n\t * sub-range is scanned, so that rules relying on a leading/trailing line break\n\t * (e.g. {@code \\n} delimited prefixes) can still match right at the document\n\t * boundaries, where no such surrounding character actually exists.\n\t */\n\tprivate boolean docStart;\n\tprivate boolean docEnd;\n\n\tpublic AbstractRule() {\n\t}\n\n\tpublic AbstractRule(IToken token) {\n\t\tthis.token = token;\n\t}\n\n\t/**\n\t * Informs this rule whether the range it is about to scan starts at the\n\t * document's very first character resp. ends at the document's very last\n\t * character.\n\t */\n\tpublic void setDocumentBounds(boolean docStart, boolean docEnd) {\n\t\tthis.docStart = docStart;\n\t\tthis.docEnd = docEnd;\n\t}\n\n\tpublic final IToken evaluate(ICharacterScanner s) {\n\t\treturn evaluateToken(new Scanner(new Scanner.CharacterScanner() {\n\t\t\t@Override\n\t\t\tpublic void unread() {\n\t\t\t\ts.unread();\n\t\t\t}\n\n\t\t\t@Override\n\t\t\tpublic int read() {\n\t\t\t\treturn s.read();\n\t\t\t}\n\t\t}, docStart, docEnd));\n\t}"
  reason: Add docStart/docEnd state and pass it to underlying Scanner
```
/answer 505698ff-53d8-4efc-8432-6db568bedafd allow
ReasoningToken: 1053
Control Request:
```yaml
id: 1e714efb-cf09-43dd-b40f-3e7ce7ce14ec
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java
  old_text: "\t\t// ---- fallback (used while no AST is available, e.g. huge documents) ----\n\t\tList<IRule> all = new ArrayList<>();\n\t\tfor (RuleBasedScanner s : scannerByNode.values())\n\t\t\tfor (IRule r : ruleCache.getOrDefault(s, new IRule[0]))\n\t\t\t\tall.add(r);\n\t\tfallbackScanner.setRules(all.toArray(new IRule[0]));\n\t}\n\n\tprivate void register(AbstractNode node, IRule... rules) {\n\t\tRuleBasedScanner scanner = new RuleBasedScanner();\n\t\tscanner.setRules(rules);\n\t\tscannerByNode.put(node, scanner);\n\t\truleCache.put(scanner, rules);\n\t}"
  new_text: "\t\t// ---- fallback (used while no AST is available, e.g. huge documents) ----\n\t\tList<IRule> all = new ArrayList<>();\n\t\tfor (RuleBasedScanner s : scannerByNode.values())\n\t\t\tfor (IRule r : ruleCache.getOrDefault(s, new IRule[0]))\n\t\t\t\tall.add(r);\n\t\tIRule[] allRules = all.toArray(new IRule[0]);\n\t\tfallbackScanner.setRules(allRules);\n\t\truleCache.put(fallbackScanner, allRules);\n\t}\n\n\tprivate void register(AbstractNode node, IRule... rules) {\n\t\tRuleBasedScanner scanner = new RuleBasedScanner();\n\t\tscanner.setRules(rules);\n\t\tscannerByNode.put(node, scanner);\n\t\truleCache.put(scanner, rules);\n\t}\n\n\t/**\n\t * Tells every {@link AbstractRule} used by {@code scanner} whether the\n\t * [start, end) range about to be scanned touches the real start resp. end of\n\t * {@code document}, so that rules can tolerate the missing leading/trailing\n\t * line break that would otherwise be expected there.\n\t */\n\tprivate void applyDocumentBounds(RuleBasedScanner scanner, IDocument document, int start, int end) {\n\t\tIRule[] rules = ruleCache.get(scanner);\n\t\tif (rules == null)\n\t\t\treturn;\n\n\t\tboolean atDocStart = start == 0;\n\t\tboolean atDocEnd = end == document.getLength();\n\t\tfor (IRule rule : rules)\n\t\t\tif (rule instanceof AbstractRule abstractRule)\n\t\t\t\tabstractRule.setDocumentBounds(atDocStart, atDocEnd);\n\t}"
  reason: Add helper to propagate docStart/docEnd flags to rules
```
/answer 1e714efb-cf09-43dd-b40f-3e7ce7ce14ec allow
Control Request:
```yaml
id: 8ab1702e-dbda-497f-8b2a-f5d312764894
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java
  old_text: "\t\tint widenedEnd = Math.min(document.getLength(), end + 1);\n\t\tscanner.setRange(document, start, widenedEnd - start);"
  new_text: "\t\tapplyDocumentBounds(scanner, document, start, end);\n\n\t\tint widenedEnd = Math.min(document.getLength(), end + 1);\n\t\tscanner.setRange(document, start, widenedEnd - start);"
  reason: Apply document bounds before scanning a gap
```
/answer 8ab1702e-dbda-497f-8b2a-f5d312764894 allow
Control Request:
```yaml
id: 0b65c3a7-7cf1-47b5-9e0c-69212e0103c3
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/AIRuleScanner.java
  old_text: "\tprivate void scanFlat(RuleBasedScanner scanner, IDocument document, int lo, int hi) {\n\t\tscanner.setRange(document, lo, hi - lo);"
  new_text: "\tprivate void scanFlat(RuleBasedScanner scanner, IDocument document, int lo, int hi) {\n\t\tapplyDocumentBounds(scanner, document, lo, hi);\n\t\tscanner.setRange(document, lo, hi - lo);"
  reason: Apply document bounds before flat scanning
```
/answer 0b65c3a7-7cf1-47b5-9e0c-69212e0103c3 allow
Text:
 Jetzt passe ich `AIRuleScanner` an, um diese Flags korrekt zu setzen:
Control Request:
```yaml
id: a9a23ae3-7c55-4b41-87c0-965507b30d56
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/EmphasisRule.java
  reason: check EmphasisRule for docStart relevant patterns
```
/answer a9a23ae3-7c55-4b41-87c0-965507b30d56 allow
Control Request:
```yaml
id: 3a8cd970-c441-4617-b1e8-341f796bce03
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/ListRule.java
  reason: check ListRule for docStart relevant patterns
```
/answer 3a8cd970-c441-4617-b1e8-341f796bce03 allow
Control Request:
```yaml
id: ee404e0a-a4a8-4102-8254-22b52a2ea4b6
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/LinkRule.java
  reason: check LinkRule for docStart relevant patterns
```
/answer ee404e0a-a4a8-4102-8254-22b52a2ea4b6 allow
Text:
 Nun rufe ich `applyDocumentBounds` an den beiden Scan-Stellen (`scanGap` und `scanFlat`) auf, bevor die jeweilige Range gescannt wird:
Control Request:
```yaml
id: aa6182a6-fc39-4044-bad3-77bdb2841f8b
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/BlockRule.java
  old_text: "\t\tif (!s.isNextSequence(startBlock))\n\t\t\treturn s.reset();"
  new_text: "\t\tif (!s.isNextSequenceBounded(startBlock))\n\t\t\treturn s.reset();"
  reason: Tolerate missing leading newline at document start
```
/answer aa6182a6-fc39-4044-bad3-77bdb2841f8b allow
Control Request:
```yaml
id: 048c68d6-07d2-49ce-a9c7-a9ab703a9fae
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/PrefixLineRule.java
  old_text: "\t\tif (!s.isNextSequence(prefix))\n\t\t\treturn s.reset();"
  new_text: "\t\tif (!s.isNextSequenceBounded(prefix))\n\t\t\treturn s.reset();"
  reason: Tolerate missing leading newline at document start
```
/answer 048c68d6-07d2-49ce-a9c7-a9ab703a9fae allow
Control Request:
```yaml
id: 11c6caad-f891-4451-92e8-c5bfb6900f78
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/LineMatchRule.java
  old_text: "\t\tif (!s.isNextSequence(match))\n\t\t\treturn false;\n\t\ts.unread(); // unread NL\n\t\treturn true;"
  new_text: "\t\tif (!s.isNextSequenceBounded(match))\n\t\t\treturn false;\n\t\tif (!s.isDocEnd() || !s.isEOF())\n\t\t\ts.unread(); // unread NL\n\t\treturn true;"
  reason: Tolerate missing leading newline at document start and missing trailing newline at document end
```
/answer 11c6caad-f891-4451-92e8-c5bfb6900f78 allow
ReasoningToken: 895
Control Request:
```yaml
id: a5489f22-e317-443b-ab9a-d971b77f3d91
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/LineMatchRule.java
  old_text: "\t\tif (!s.isNextSequenceBounded(match))\n\t\t\treturn false;\n\t\tif (!s.isDocEnd() || !s.isEOF())\n\t\t\ts.unread(); // unread NL\n\t\treturn true;"
  new_text: "\t\tif (!s.isNextSequenceBounded(match))\n\t\t\treturn false;\n\t\tif (!s.isEOF())\n\t\t\ts.unread(); // unread NL, unless doc end was reached without a real trailing newline\n\t\treturn true;"
  reason: Only unread the trailing newline when it was actually consumed
```
/answer a5489f22-e317-443b-ab9a-d971b77f3d91 allow
ReasoningToken: 216
ReasoningToken: 860
Control Request:
```yaml
id: 79e16113-757f-4ff4-8cda-03ddec75a6b2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/BlockRule.java
  reason: Check trailing unread logic near doc end
```
/answer 79e16113-757f-4ff4-8cda-03ddec75a6b2 allow
ReasoningToken: 391
Control Request:
```yaml
id: fb9fd205-0f36-4833-a94c-797531b1c0bb
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/BlockRule.java
  old_text: "\t\tif (!endblock)\n\t\t\treturn s.reset();\n\t\ts.unread();\n\t\treturn true;"
  new_text: "\t\tif (!endblock)\n\t\t\treturn s.reset();\n\t\tif (!s.isEOF())\n\t\t\ts.unread(); // unread trailing NL, unless doc end was reached without a real one\n\t\treturn true;"
  reason: Only unread the trailing newline of endBlock when it was actually consumed
```
/answer fb9fd205-0f36-4833-a94c-797531b1c0bb allow
ReasoningToken: 129
Control Request:
```yaml
id: 2e8a3761-cea3-40f4-84ed-fe78ded8ef1e
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -name "*.classpath" -o -name "pom.xml" -o -name "build.xml" | head -20
  reason: check build setup to attempt compile
```
/answer 2e8a3761-cea3-40f4-84ed-fe78ded8ef1e deny Es gibt weder Build Setup noch Tests
ReasoningToken: 215
Kein Build-Setup vorhanden, daher keine automatisierte Kompilierung möglich. Ich fasse die Änderungen zusammen:

## Zusammenfassung

**`AbstractRule.java`**
- Neue Felder `docStart`/`docEnd` sowie Methode `setDocumentBounds(boolean, boolean)`, mit der der treibende Scanner mitteilen kann, ob der gerade zu scannende Bereich am tatsächlichen Dokumentanfang/-ende liegt.
- `evaluate(...)` erzeugt den internen `Scanner` jetzt mit diesen Flags (`new Scanner(wrapper, docStart, docEnd)`), statt sie immer als `false` zu übergeben.

**`AIRuleScanner.java`**
- Neue Hilfsmethode `applyDocumentBounds(RuleBasedScanner, IDocument, int start, int end)`, die anhand von `start == 0` bzw. `end == document.getLength()` ermittelt, ob der Bereich am Dokumentanfang/-ende liegt, und dies über `ruleCache` an alle zugehörigen `AbstractRule`-Instanzen weiterreicht.
- Aufruf dieser Methode sowohl in `scanGap(...)` (AST-basierter Pfad) als auch in `scanFlat(...)` (Fallback-Pfad ohne AST), jeweils vor `scanner.setRange(...)`.
- Der `fallbackScanner` wird nun ebenfalls in `ruleCache` registriert, damit auch er die Bounds korrekt weitergeben kann.

**Begleitende Anpassungen in den Regeln** (damit die neuen Flags auch wirken, da `Scanner.isNextSequenceBounded` die Toleranz für ein fehlendes führendes `\n` am Dokumentanfang nur dort aktiviert):
- `PrefixLineRule`, `LineMatchRule`, `BlockRule`: `isNextSequence(...)` → `isNextSequenceBounded(...)` für die führende `\n`-Sequenz.
- `LineMatchRule` und `BlockRule`: das abschließende `unread()` des Zeilenumbruchs wird jetzt nur ausgeführt, wenn tatsächlich ein realer `\n` gelesen wurde (`!s.isEOF()`), damit am Dokumentende ohne abschließenden Zeilenumbruch kein falsches Zeichen zurückgelegt wird.

Damit werden Zeilen-/Block-Marker (z.B. `User:`, `Agent:`, Codeblöcke) auch dann korrekt erkannt, wenn sie exakt am Anfang oder Ende des Dokuments stehen (kein umgebender Zeilenumbruch vorhanden).
