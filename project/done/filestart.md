Die AST-Parser wie `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Paragraph.java` wurden Column agnostisch geschrieben um möglichst unabhängig jederzeit in einem Stream eingesetzt werden zu können. Dies hat den Nachteil, das für die Sektionserkennung am Anfang einer Datei kein Newline gegeben ist. Analysiere und Überlege einen Lösungsansatz.
Beispielweise könnte man ein virtuelles Newline im Puffer vor dem eigentlichen ersten Zeichen einfügen, das in der Offsetberechnung nicht gewertet wird. Auch eine Variante mit mehr Overhead wäre das Einfügen eines Column-Counters. Dies reduziert jedoch auch die Abgrenzbarkeit der Sektionen.

SystemInit: cwd=/home/user/xyan/XY.Archiv | session_id=c2bb703e-211f-4914-8ce2-9d5b8bbd948b | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 041a98e6-7cf9-44c9-ae34-b4adaeea9f87
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast
  reason: Explore mdast parser structure to understand context
```
/answer 041a98e6-7cf9-44c9-ae34-b4adaeea9f87 allow
Control Request:
```yaml
id: 237d8d19-e75b-4911-92a1-f803756be553
result:
  structuredContent:
    entries:
    - IDocumentBuffer.java
    - MarkdownDocument.java
    - TextRegion.java
    - nodes/AbstractNode.java
    - nodes/Category.java
    - nodes/Elements.java
    - nodes/HeadingSection.java
    - nodes/LineSection.java
    - nodes/Node.java
    - nodes/PageSection.java
    - nodes/Paragraph.java
    - nodes/PrefixBlock.java
    - nodes/Root.java
    - nodes/ScriptBlock.java
```
/answer 237d8d19-e75b-4911-92a1-f803756be553 allow
ReasoningToken: 45
Control Request:
```yaml
id: 91f6f724-b1a3-4748-88d5-b48a8ce54b04
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Paragraph.java
  reason: Read Paragraph parser implementation
```
/answer 91f6f724-b1a3-4748-88d5-b48a8ce54b04 allow
Control Request:
```yaml
id: 56d0b204-da96-4e21-9972-b88d2fb75238
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/IDocumentBuffer.java
  reason: Read buffer interface
```
/answer 56d0b204-da96-4e21-9972-b88d2fb75238 allow
Control Request:
```yaml
id: 32b6ec9d-4bf9-4ff7-83a7-f5dbdee89454
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/AbstractNode.java
  reason: Read abstract node base class
```
/answer 32b6ec9d-4bf9-4ff7-83a7-f5dbdee89454 allow
Control Request:
```yaml
id: 69999760-0084-49d7-80e3-5464006d11ef
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/HeadingSection.java
  reason: Read heading section parser for section detection logic
```
/answer 69999760-0084-49d7-80e3-5464006d11ef allow
Control Request:
```yaml
id: e950fe91-cfa5-48dd-8bee-15941a059a94
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools
  reason: Find Scanner and LineIndex classes
```
/answer e950fe91-cfa5-48dd-8bee-15941a059a94 allow
Control Request:
```yaml
id: 18e09d96-c69f-4c80-ab89-1b74107aba5f
result:
  structuredContent:
    entries:
    - AbstractQueryListener.java
    - LineIndex.java
    - Scanner.java
    - Time.java
```
/answer 18e09d96-c69f-4c80-ab89-1b74107aba5f allow
ReasoningToken: 161
Control Request:
```yaml
id: 6ad49548-fc36-4dfa-aed1-cbd15c1b2225
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  reason: Read Scanner implementation
```
/answer 6ad49548-fc36-4dfa-aed1-cbd15c1b2225 allow
Control Request:
```yaml
id: 8a51392c-967d-4321-a7ff-ee8a6822cbb3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java
  reason: Read LineIndex/Buffer implementation
```
/answer 8a51392c-967d-4321-a7ff-ee8a6822cbb3 allow
Control Request:
```yaml
id: 468b807b-d9c5-4f05-bd28-581ac10a7119
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Root.java
  reason: Read Root node parser
```
/answer 468b807b-d9c5-4f05-bd28-581ac10a7119 allow
Control Request:
```yaml
id: ba3bc4ac-c1df-4238-83c2-9faac2c8ca88
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Elements.java
  reason: Read Elements listing
```
/answer ba3bc4ac-c1df-4238-83c2-9faac2c8ca88 allow
Control Request:
```yaml
id: 208c358a-efb2-4019-bbd2-69701b630d39
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PageSection.java
  reason: Read PageSection parser
```
/answer 208c358a-efb2-4019-bbd2-69701b630d39 allow
Control Request:
```yaml
id: 108e86be-02d1-4147-8573-770076930529
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/LineSection.java
  reason: Read LineSection parser
```
/answer 108e86be-02d1-4147-8573-770076930529 allow
Control Request:
```yaml
id: 5a67ef99-dd97-4afe-b99a-628b90a88c2a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PrefixBlock.java
  reason: Read PrefixBlock parser
```
/answer 5a67ef99-dd97-4afe-b99a-628b90a88c2a allow
Control Request:
```yaml
id: dfb115ae-bf98-4215-b5a4-b62d0ec7fe65
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/ScriptBlock.java
  reason: Read ScriptBlock parser
```
/answer dfb115ae-bf98-4215-b5a4-b62d0ec7fe65 allow
Control Request:
```yaml
id: e71bd53a-30a5-4f5c-9ea6-30c56513da66
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  reason: Read MarkdownDocument to see how scanning is initiated
```
/answer e71bd53a-30a5-4f5c-9ea6-30c56513da66 allow
ReasoningToken: 90
Control Request:
```yaml
id: 4ee4dd4f-1bd4-45c6-84e8-aa2fcdea4f8f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Node.java
  reason: Read Node class for offset semantics
```
/answer 4ee4dd4f-1bd4-45c6-84e8-aa2fcdea4f8f allow
Control Request:
```yaml
id: 3b885dc5-9ba0-448e-977b-10a2a61c8c4b
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/TextRegion.java
  reason: Read TextRegion class
```
/answer 3b885dc5-9ba0-448e-977b-10a2a61c8c4b allow
ReasoningToken: 460
ReasoningToken: 32816
## Problemanalyse

Ich habe mir den kompletten Scan-Mechanismus angesehen (`Scanner`, `AbstractNode`, `MarkdownDocument`, sowie `Paragraph`, `HeadingSection`, `PageSection`, `LineSection`, `PrefixBlock`, `ScriptBlock`).

### Das eigentliche Muster

Alle Sektions-/Block-Parser erkennen ihren Start über ein Prefix, das **immer mit `\n` beginnt**:

```java
// HeadingSection
prefix[0] = '\n'; ...            // "\n## "
// Paragraph
private char[] prefix = "\n".toCharArray();
// PageSection
private char[] separator = "\n---\n".toCharArray();
// LineSection
this.prefix = ("\n" + marker + "\n").toCharArray();
// PrefixBlock
this.prefix = ("\n" + marker).toCharArray();
// ScriptBlock
private char[] startBlock = "\n```".toCharArray();
```

Das ist ein bewusster und cleverer Trick: Ein Knoten „besitzt“ das `\n`, das die vorangehende Zeile abschließt. Dadurch teilen sich benachbarte Sektionen genau ein Trennzeichen, ohne dass irgendein Parser wissen muss, „wo im Dokument“ er sich befindet – er muss nur lokal prüfen: „steht hier ein Zeilenumbruch gefolgt vom Muster“. Genau das macht die Parser stream-/positionsunabhängig einsetzbar (u.a. für inkrementelles Re-Parsing in `MarkdownDocument.update()`, siehe unten).

### Warum das genau am Dateianfang bricht

An Position 0 des Puffers gibt es **kein** vorangehendes `\n` – es kann auch keins geben, es ist der Anfang. Damit schlägt `isNextSequence(prefix)` für **jeden** Sektionstyp fehl, wenn der Text z. B. direkt mit `# Titel` beginnt. In `AbstractNode.scan()` wird dann einfach zeichenweise weitergelesen (`s.readNext()`), ohne dass ein Kind-Knoten erzeugt wird – der Text vor dem ersten „echten“ `\n`-Treffer bleibt strukturlos im Elternknoten „hängen“. Das betrifft praktisch **jedes normale Dokument**, das nicht mit einer Leerzeile beginnt.

### Wichtiger Befund: Es betrifft *nur* Position 0, nicht jedes Re-Parsing

Ich habe geprüft, ob das Problem auch bei inkrementellem Re-Parsing (`MarkdownDocument.parse(absStart, absEnd)`) an beliebigen Stellen mitten im Dokument auftritt – das wäre deutlich gravierender. Ist es aber nicht:

- `absStart` wird in `update()` immer aus `sec.getOffset()` gebildet, wobei `sec` ein bereits geparster Knoten ist.
- Der `start`-Wert eines Knotens wird genau dort gesetzt, wo sein prefix-`\n` beginnt (`nn.start = s.getReadCount()` vor dem Match-Versuch).
- Für jeden Knoten außer dem allerersten im gesamten Baum ist diese Position ein **real existierendes** `\n` im Puffer.
- Nur `root` selbst hat `start == 0` und steht für den echten Dateianfang.

→ Das Problem ist **exakt auf `absStart == 0`** eingegrenzt, also den initialen Parse-Lauf und jedes Re-Parsing, das bis zur allerersten Sektion zurückgreift. Das ist gut, weil es die Lösung stark eingrenzt.

## Bewertung der beiden vorgeschlagenen Ansätze

**a) Virtuelles `\n` im Puffer, „nicht gezählt“**

Die Idee ist grundsätzlich richtig, aber die Umsetzung ist tückischer als sie aussieht. `Scanner.readCount` wird nicht nur lokal geführt, sondern kaskadiert durch die komplette Parent-Kette (`getSubscanner()` → `read()` ruft `parent.read()` auf, was auch dessen `readCount` erhöht). Ich habe das durchgerechnet: Ein einmal „konsumiertes“ virtuelles Zeichen, das an *jeder* Ebene mitgezählt wird, verschiebt danach **alle** nachfolgenden `start`/`end`-Werte im gesamten Baum um eine konstante `+1` – mit Ausnahme des allerersten `nn.start` (der bleibt zufällig korrekt bei 0, weil er *vor* dem virtuellen Read erfasst wird). Um das sauber zu machen, müsste man das Zeichen an *jeder* Kaskadenebene „unsichtbar“ für `readCount` machen (Fetch-then-conditionally-increment, symmetrisch auch bei `unread()`). Das ist machbar, aber ein invasiver Eingriff in eine zentrale, von allem genutzte Klasse – mit hohem Risiko für schwer auffindbare Off-by-One-Fehler.

**b) Spalten-/Zeilen-Counter**

Reduziert wie richtig vermutet genau die Eigenschaft, die die Architektur ausmacht: Sektionsgrenzen wären dann nicht mehr rein lokal aus dem Zeichenstrom ableitbar, sondern hingen von einem mitgeführten externen Zustand ab, der bei jedem `getSubscanner()`/Teil-Scan korrekt synchron gehalten werden müsste – de facto eine zweite, parallele Offset-Buchhaltung neben der bereits vorhandenen `LineIndex`. Das erhöht Komplexität und Sync-Risiko, ohne dass es nötig wäre.

## Empfohlener Lösungsansatz: „Document-Start“-Flag am Scanner statt virtuellem Zeichen

Statt ein Zeichen zu *simulieren* (und damit Zähllogik zu verbiegen), sollte der Scanner einfach **wissen, dass er am echten Dateianfang steht**, und die `\n`-Anforderung in diesem einen Fall überspringen, ohne irgendetwas zu konsumieren oder zu zählen:

- Neues Konstrukt: `Scanner(CharacterScanner scan, boolean documentStart)`.
- `isAtDocumentStart()`: läuft zur Wurzel der Parent-Kette hoch und liefert `true`, wenn diese Wurzel als `documentStart` markiert wurde **und** aktuell `readCount == 0` ist. Das ist eine reine Live-Abfrage, kein Zustand, der mitgeführt/synchronisiert werden muss – exakt ein Bit, kein Parallel-Tracking.
- Neue Hilfsmethode `isNextSequenceOrDocumentStart(char[] seq)`: prüft am Dateianfang das Prefix **ohne** das führende `\n`, sonst wie gehabt mit `\n`.

```java
// Scanner.java
private boolean documentStart;

public Scanner(CharacterScanner scan) { this(scan, false); }
public Scanner(CharacterScanner scan, boolean documentStart) {
    this.scan = scan;
    this.documentStart = documentStart;
}

public boolean isAtDocumentStart() {
    Scanner root = this;
    while (root.parent != null) root = root.parent;
    return root.documentStart && root.readCount == 0;
}

public boolean isNextSequenceOrDocumentStart(char[] seq) {
    if (seq.length > 0 && seq[0] == '\n' && isAtDocumentStart())
        return isNextSequence(seq, 1); // Prefix ohne führendes '\n'
    return isNextSequence(seq);
}

// isNextSequence(char[]) intern auf isNextSequence(seq, 0) umstellen
```

Anpassung in den betroffenen Knoten (mechanisch, gleiches Muster in allen 6 Klassen):

```java
// HeadingSection, Paragraph, PageSection, LineSection, PrefixBlock, ScriptBlock
protected boolean isStart(Scanner s) {
    return s.isNextSequenceOrDocumentStart(prefix); // statt isNextSequence(prefix)
}
```

Und in `MarkdownDocument`:

```java
private Node parse(int absStart, int absEnd) {
    char[] slice = readChars(absStart, absEnd - absStart);
    Node rn = new Node(null, Elements.ROOT);
    Elements.ROOT.scan(new Scanner(new BufferReader(slice, 0), absStart == 0), rn);
    return rn;
}
```

**Warum das die Nachteile von a) und b) vermeidet:**

- Es wird **kein** Zeichen konsumiert oder gezählt → keinerlei Offset-Verschiebung, keine Sonderbehandlung von Einzelfeldern nötig, keine Gefahr für die zentrale, performancekritische `Scanner`-Klasse.
- Es wird **kein** paralleler Zeilen-/Spaltenzustand eingeführt → die Sektionen bleiben weiterhin durch rein lokale Mustererkennung abgrenzbar; die einzige neue Information ist ein einziges Bit „ich bin (noch) an Position 0 der Wurzel“, das sich aus vorhandenem Zustand (`readCount`) ergibt.
- Da nach jedem gescheiterten Match `reset()`/`unread()` den `readCount` der Wurzel wieder auf 0 zurücksetzt, funktioniert `isAtDocumentStart()` korrekt für *alle* Kandidaten, die an Position 0 ausprobiert werden (Root probiert nacheinander `PageSection`, alle `HeadingSection`s, `Paragraph` etc.) – sobald einer matcht und Zeichen konsumiert, kippt das Flag automatisch und dauerhaft auf `false`.
- Es passt sich nahtlos ins bestehende Re-Parsing ein: `parse()` wird nur mit `absStart == 0` als „documentStart“ markiert – für alle anderen (immer an einem realen `\n` beginnenden) Re-Parses bleibt das bisherige Verhalten unverändert korrekt.

## Randfälle, die mit abgedeckt werden sollten

- Leeres Dokument (`length()==0`).
- Dokument, das direkt mit einer `PageSection`-Trennlinie `---` beginnt.
- Dokument, das direkt mit einem ```` ``` ````-Codeblock beginnt (`ScriptBlock`).
- Bearbeitung an Offset 0, die ein Re-Parsing der ersten Sektion auslöst (`sec.getOffset() == 0`) – muss erneut korrekt als `documentStart` erkannt werden.
- Sicherstellen, dass `isEndInner()`-Implementierungen **nicht** angepasst werden müssen – die arbeiten immer auf Sub-Scannern, die zwangsläufig schon hinter Position 0 liegen, also unbetroffen.

## Kurzfazit

Der von Ihnen skizzierte Ansatz „virtuelles `\n`“ ist konzeptionell richtig, aber in der konkreten `Scanner`-Kaskadenstruktur fehleranfällig umzusetzen (globale +1-Verschiebung aller Offsets). Der Column-Counter-Ansatz löst das Problem, opfert aber genau die architektonische Stärke der Column-Agnostik. Ich empfehle stattdessen die dritte Variante – ein leichtgewichtiges „Document-Start“-Flag am `Scanner`, das die `\n`-Prüfung nur an echter Position 0 lokal überspringt, ohne Offset-Arithmetik oder Streaming-Fähigkeit anzutasten. Der Eingriff ist klein, mechanisch auf 6 Klassen + `Scanner` + `MarkdownDocument.parse()` begrenzt und risikoarm testbar.

Soll ich diesen Ansatz direkt umsetzen (inkl. Tests für die genannten Randfälle)?
Setze den Ansatz um und erweitere ihn. Dasselbe Problem gilt nämlich auch für das Dateiende und das letzte Blatt Element.
SystemInit: cwd=/home/user/xyan/XY.Archiv | session_id=c2bb703e-211f-4914-8ce2-9d5b8bbd948b | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 83ae0d1f-4114-469c-be33-b5c7ef41c61c
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  reason: Check build tooling to prepare for compiling/testing changes
  script: ls; find . -iname "pom.xml" -o -iname "build.gradle*" | head -20; find . -path "*mdast*" -iname "*Test*"
```
/answer 83ae0d1f-4114-469c-be33-b5c7ef41c61c deny es gibt weder Building Tools noch Tests
ReasoningToken: 26934
Control Request:
```yaml
id: 118738ae-1f5a-4457-a356-c9ab7cf61785
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  reason: Check for javac availability to compile a small standalone verification harness for the Scanner offset logic
  script: which javac java 2>&1; javac -version 2>&1
```
/answer 118738ae-1f5a-4457-a356-c9ab7cf61785 deny Kümmere den nur um den Code, ich kompiliere das selbst.
ReasoningToken: 312
Text:
 Ich setze den Ansatz um und erweitere ihn symmetrisch für das Dateiende / das letzte Blattelement. Vorher noch eine kurze Einordnung des erweiterten Problems, dann der Code.

**Erweiterte Analyse (Dateiende):**

Beim Nachvollziehen der `Scanner`-Mechanik ist mir aufgefallen, dass es *zwei* verschiedene End-of-File-Probleme gibt, die beide behoben werden müssen:

1. **Muster mit abschließendem `\n` als Grenzzeichen** (`PageSection.separator = "\n---\n"`, `LineSection.prefix = "\n"+marker+"\n"`, `ScriptBlock.endBlock = "\n```\n"`): Genau wie beim führenden `\n` am Dateianfang kann dieses *abschließende* `\n` fehlen, wenn das jeweilige Muster exakt am echten Dateiende steht (keine Zeile mehr danach). Das ist die exakte Spiegelung des Start-Problems.
2. **Ein generischerer, bisher unbemerkter Fehler in `Scanner.read()`**: `readCount++` wird dort *unbedingt* ausgeführt, auch wenn der Read tatsächlich auf EOF trifft (kein reales Zeichen mehr vorhanden). Da die Scan-Schleife in `AbstractNode.scan()` bei echtem Dateiende regulär über einen fehlgeschlagenen `s.readNext()` beendet wird, wird für **jedes** Blatt-Element, dessen Scan durch echtes EOF endet (nicht durch ein explizites Muster), `n.end` künstlich um 1 zu groß berechnet – und das kaskadiert über die Scanner-Elternkette bis zur Wurzel. Das betrifft also nicht nur bestimmte Muster, sondern grundsätzlich das letzte Blattelement (und alle seine Vorfahren) im gesamten Baum.

Ich behebe daher beides: den generischen Zählfehler direkt an der Wurzel (`Scanner.read()`), und zusätzlich – symmetrisch zu `isAtDocumentStart()` – ein `isAtDocumentEnd()`, das an den Stellen greift, wo ein Muster ein abschließendes `\n` als „geteiltes Grenzzeichen“ voraussetzt.

Jetzt die Umsetzung:
Control Request:
```yaml
id: ab08d3ec-85c5-4a9e-8129-1756841de03a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  reason: Re-check current exact content before editing
```
/answer ab08d3ec-85c5-4a9e-8129-1756841de03a allow
ReasoningToken: 386
Control Request:
```yaml
id: 98577070-4461-43a7-ae5b-720f049140af
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  old_text: "\tprivate CharacterScanner scan;\n\tprivate Scanner parent;\n\tprivate LineIndex lineIndex;\n\tprivate int p;\n\tprivate int c;\n\tprivate int readCount = 0;"
  new_text: "\tprivate CharacterScanner scan;\n\tprivate Scanner parent;\n\tprivate LineIndex lineIndex;\n\tprivate int p;\n\tprivate int c;\n\tprivate int readCount = 0;\n\n\t// Only meaningful on the outermost/root scanner of a scan tree (see\n\t// isAtDocumentStart()/isAtDocumentEnd()). They mark whether the slice being\n\t// scanned actually touches the real, absolute start/end of the whole\n\t// document, as opposed to an arbitrary sub-range used for incremental\n\t// re-parsing. Section/Block parsers use a leading/trailing '\\n' as a shared\n\t// boundary character with their neighbour - these flags let them tolerate\n\t// the (unavoidable) absence of that boundary character when there truly is\n\t// no neighbour, without requiring a real or virtual extra character to be\n\t// read/counted.\n\tprivate boolean documentStart;\n\tprivate boolean documentEnd;"
  reason: Add BOF/EOF tracking fields to Scanner
```
/answer 98577070-4461-43a7-ae5b-720f049140af allow
ReasoningToken: 1612
Control Request:
```yaml
id: 926a4d9c-53d7-4141-b1fc-7ce0c243852c
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  old_text: "\tpublic Scanner(CharacterScanner scan) {\n\t\tthis.scan = scan;\n\t}"
  new_text: "\tpublic Scanner(CharacterScanner scan) {\n\t\tthis(scan, false, false);\n\t}\n\n\t/**\n\t * @param documentStart true if this scanner's very first character (offset\n\t *                      0 within the underlying {@link CharacterScanner})\n\t *                      corresponds to the absolute start of the whole\n\t *                      document, i.e. this is NOT a partial/incremental\n\t *                      re-parse slice starting somewhere mid-document.\n\t * @param documentEnd   true if running out of input on this scanner\n\t *                      corresponds to the absolute end of the whole\n\t *                      document, i.e. this is NOT a partial/incremental\n\t *                      re-parse slice ending somewhere mid-document.\n\t */\n\tpublic Scanner(CharacterScanner scan, boolean documentStart, boolean documentEnd) {\n\t\tthis.scan = scan;\n\t\tthis.documentStart = documentStart;\n\t\tthis.documentEnd = documentEnd;\n\t}"
  reason: Add documentStart/documentEnd aware constructor
```
/answer 926a4d9c-53d7-4141-b1fc-7ce0c243852c allow
Control Request:
```yaml
id: 0ca03434-1acc-4952-bb27-25df08ae9d2b
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  old_text: "\tpublic boolean isNextSequence(char[] seq) {\n\t\tScanner sub = getSequenceScanner();\n\t\tint sr = 0;\n\t\tfor (; sr < seq.length && sub.readNext(); sr++)\n\t\t\tif (sub.getChar() != seq[sr])\n\t\t\t\treturn sub.reset();\n\t\treturn sr == seq.length ? true : sub.reset();\n\t}"
  new_text: "\tpublic boolean isNextSequence(char[] seq) {\n\t\treturn matchSequence(seq, 0);\n\t}\n\n\t/**\n\t * Like {@link #isNextSequence(char[])}, but tolerant at the true\n\t * boundaries of the whole document (see {@link #isAtDocumentStart()} /\n\t * {@link #isAtDocumentEnd()}):\n\t * <ul>\n\t * <li>if {@code seq} starts with '\\n' and this scanner is positioned at\n\t * the absolute start of the document, that leading '\\n' is treated as\n\t * already satisfied (there simply is no character before the document to\n\t * read/count);</li>\n\t * <li>if {@code seq} ends with '\\n' and matching runs into the absolute\n\t * end of the document exactly at that last character, the trailing '\\n'\n\t * is likewise treated as satisfied (there is no character after the\n\t * document to read/count).</li>\n\t * </ul>\n\t * In both cases nothing is read or counted for the skipped boundary\n\t * character - it is skipped, not virtually consumed - so offset\n\t * arithmetic elsewhere stays exact.\n\t */\n\tpublic boolean isNextSequenceBounded(char[] seq) {\n\t\tint from = seq.length > 0 && seq[0] == '\\n' && isAtDocumentStart() ? 1 : 0;\n\t\treturn matchSequence(seq, from);\n\t}\n\n\tprivate boolean matchSequence(char[] seq, int from) {\n\t\tScanner sub = getSequenceScanner();\n\t\tint sr = from;\n\t\tfor (; sr < seq.length && sub.readNext(); sr++)\n\t\t\tif (sub.getChar() != seq[sr])\n\t\t\t\treturn sub.reset();\n\t\tif (sr == seq.length)\n\t\t\treturn true;\n\t\t// ran out of real input before completing the match; tolerate a\n\t\t// missing trailing '\\n' exactly at the true document end\n\t\tif (sr == seq.length - 1 && seq[sr] == '\\n' && isAtDocumentEnd())\n\t\t\treturn true;\n\t\treturn sub.reset();\n\t}\n\n\t/**\n\t * @return true if this scanner is currently positioned exactly at the\n\t *         absolute start of the whole document (not merely at the start\n\t *         of an incremental re-parse slice), i.e. no real character has\n\t *         been read yet and none can ever precede this position.\n\t */\n\tpublic boolean isAtDocumentStart() {\n\t\tScanner root = this;\n\t\twhile (root.parent != null)\n\t\t\troot = root.parent;\n\t\treturn root.documentStart && root.readCount == 0;\n\t}\n\n\t/**\n\t * @return true if this scanner's underlying slice ends at the absolute\n\t *         end of the whole document (not merely at the end of an\n\t *         incremental re-parse slice), i.e. hitting EOF here means there\n\t *         truly is no further character, ever.\n\t */\n\tpublic boolean isAtDocumentEnd() {\n\t\tScanner root = this;\n\t\twhile (root.parent != null)\n\t\t\troot = root.parent;\n\t\treturn root.documentEnd;\n\t}"
  reason: Introduce boundary-tolerant sequence matching for BOF and EOF
```
/answer 0ca03434-1acc-4952-bb27-25df08ae9d2b allow
Control Request:
```yaml
id: 8222ab13-303e-4384-82f5-d3469759ea01
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  old_text: "\tprivate int read() {\n\t\treadCount++;\n\t\tp = c;\n\t\treturn (parent != null ? (c = parent.read()) : (c = scan.read()));\n\t}"
  new_text: "\tprivate int read() {\n\t\tint prev = c;\n\t\tint next = (parent != null ? parent.read() : scan.read());\n\t\t// don't count a read that ran into EOF: no real character was\n\t\t// consumed, so it must not shift any offset computed from\n\t\t// getReadCount() (this is what makes the true, absolute end of the\n\t\t// document detectable/consistent, symmetric to isAtDocumentStart()).\n\t\tif (next != CharacterScanner.EOF)\n\t\t\treadCount++;\n\t\tp = prev;\n\t\tc = next;\n\t\treturn next;\n\t}"
  reason: "Prevent EOF probes from inflating readCount, fixing last-leaf end offset inflation"
```
/answer 8222ab13-303e-4384-82f5-d3469759ea01 allow
Control Request:
```yaml
id: cd93881d-d405-4b83-91da-63549fb01e84
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/Scanner.java
  reason: Verify full updated Scanner.java
```
/answer cd93881d-d405-4b83-91da-63549fb01e84 allow
ReasoningToken: 54
ReasoningToken: 20
ReasoningToken: 24
Control Request:
```yaml
id: 2bcb78e4-f07f-4fb6-ad46-71fb7c042c35
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/HeadingSection.java
  old_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\treturn s.isNextSequence(prefix);\n\t}"
  new_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\treturn s.isNextSequenceBounded(prefix);\n\t}"
  reason: Use BOF/EOF-tolerant sequence matching for heading start
```
/answer 2bcb78e4-f07f-4fb6-ad46-71fb7c042c35 allow
ReasoningToken: 2178
Control Request:
```yaml
id: d241d386-e698-41d4-8ee4-24c8a73fa2ea
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Paragraph.java
  old_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\treturn s.isNextSequence(prefix);\n\t}"
  new_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\treturn s.isNextSequenceBounded(prefix);\n\t}"
  reason: Use BOF-tolerant sequence matching for paragraph start
```
/answer d241d386-e698-41d4-8ee4-24c8a73fa2ea allow
Control Request:
```yaml
id: 10cebb06-7eeb-4b45-b9d2-d11538cb3fdd
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PageSection.java
  old_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\tif (!s.isNextSequence(separator))\n\t\t\treturn false;\n\t\ts.unread(); // keep trailing NL for child scanning\n\t\treturn true;\n\t}"
  new_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\tif (!s.isNextSequenceBounded(separator))\n\t\t\treturn false;\n\t\t// keep trailing NL for child scanning - unless the separator's\n\t\t// trailing '\\n' was implicitly satisfied by the real end of the\n\t\t// document (isNextSequenceBounded above), in which case there is no\n\t\t// character left to give back.\n\t\tif (!s.isEOF())\n\t\t\ts.unread();\n\t\treturn true;\n\t}"
  reason: "Use BOF/EOF-tolerant matching for page separator, guard trailing unread"
```
/answer 10cebb06-7eeb-4b45-b9d2-d11538cb3fdd allow
Control Request:
```yaml
id: 73cddf08-d905-486d-be7c-25da13b60dfb
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/LineSection.java
  old_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\tif (!s.isNextSequence(prefix))\n\t\t\treturn false;\n\t\ts.unread(); // keep trailing NL for child scanning\n\t\treturn true;\n\t}"
  new_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\tif (!s.isNextSequenceBounded(prefix))\n\t\t\treturn false;\n\t\t// keep trailing NL for child scanning - unless the marker line's\n\t\t// trailing '\\n' was implicitly satisfied by the real end of the\n\t\t// document (isNextSequenceBounded above), in which case there is no\n\t\t// character left to give back.\n\t\tif (!s.isEOF())\n\t\t\ts.unread();\n\t\treturn true;\n\t}"
  reason: "Use BOF/EOF-tolerant matching for line-section marker, guard trailing unread"
```
/answer 73cddf08-d905-486d-be7c-25da13b60dfb allow
Control Request:
```yaml
id: 230e7465-2a63-40ab-9717-e6d89586d906
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PrefixBlock.java
  old_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\treturn s.isNextSequence(prefix);\n\t}"
  new_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\treturn s.isNextSequenceBounded(prefix);\n\t}"
  reason: Use BOF-tolerant sequence matching for prefix block start
```
/answer 230e7465-2a63-40ab-9717-e6d89586d906 allow
Control Request:
```yaml
id: 005df98a-acb5-4673-ac0a-70b20956bfa0
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/ScriptBlock.java
  old_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\tif (!s.isNextSequence(startBlock))\n\t\t\treturn false;\n\n\t\tboolean endblock = false, basicEnd = false;\n\t\twhile (s.getReadCount() < LIMIT && s.readNext() && !(endblock = s.isNextSequence(endBlock))\n\t\t\t\t&& !(basicEnd = s.isNextSequence(intermediateBreak)))\n\t\t\t; // consume\n\n\t\tif (basicEnd || !endblock)\n\t\t\treturn false;\n\n\t\ts.unread(); // keep trailing NL for sibling scanning\n\t\treturn true;\n\t}"
  new_text: "\t@Override\n\tprotected boolean isStart(Scanner s) {\n\t\tif (!s.isNextSequenceBounded(startBlock))\n\t\t\treturn false;\n\n\t\tboolean endblock = false, basicEnd = false;\n\t\twhile (s.getReadCount() < LIMIT && s.readNext() && !(endblock = s.isNextSequenceBounded(endBlock))\n\t\t\t\t&& !(basicEnd = s.isNextSequence(intermediateBreak)))\n\t\t\t; // consume\n\n\t\tif (basicEnd || !endblock)\n\t\t\treturn false;\n\n\t\t// keep trailing NL for sibling scanning - unless the closing fence's\n\t\t// trailing '\\n' was implicitly satisfied by the real end of the\n\t\t// document (isNextSequenceBounded above), in which case there is no\n\t\t// character left to give back.\n\t\tif (!s.isEOF())\n\t\t\ts.unread();\n\t\treturn true;\n\t}"
  reason: "Use BOF/EOF-tolerant matching for script block fences, guard trailing unread"
```
/answer 005df98a-acb5-4673-ac0a-70b20956bfa0 allow
ReasoningToken: 38
Control Request:
```yaml
id: c0ff2d39-7453-4676-a757-ced67188a32d
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  old_text: "\tprivate Node parse(int absStart, int absEnd) {\n\t\tchar[] slice = readChars(absStart, absEnd - absStart);\n\t\tNode rn = new Node(null, Elements.ROOT);\n\t\tElements.ROOT.scan(new Scanner(new BufferReader(slice, 0)), rn);\n\t\treturn rn;\n\t}"
  new_text: "\tprivate Node parse(int absStart, int absEnd) {\n\t\tchar[] slice = readChars(absStart, absEnd - absStart);\n\t\tNode rn = new Node(null, Elements.ROOT);\n\t\t// Tell the scanner whether this slice actually touches the real,\n\t\t// absolute start/end of the document (as opposed to an arbitrary\n\t\t// sub-range picked for incremental re-parsing), so section/block\n\t\t// parsers can correctly recognize a section that starts/ends right\n\t\t// at the document boundary, where the usual shared '\\n' boundary\n\t\t// character with a (non-existing) neighbour is unavailable.\n\t\tboolean documentStart = absStart == 0;\n\t\tboolean documentEnd = absEnd == buffer.length();\n\t\tScanner scanner = new Scanner(new BufferReader(slice, 0), documentStart, documentEnd);\n\t\tElements.ROOT.scan(scanner, rn);\n\t\treturn rn;\n\t}"
  reason: Propagate document-start/end context into parse for boundary-aware scanning
```
/answer c0ff2d39-7453-4676-a757-ced67188a32d allow
Control Request:
```yaml
id: c06a2e9b-6e6b-4491-b706-79e6be25e448
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src
  pattern: .*\.java$
  reason: Find other usages of Scanner construction relevant to document boundaries
```
/answer c06a2e9b-6e6b-4491-b706-79e6be25e448 allow
Control Request:
```yaml
id: 7be9acf2-826a-44a1-9be6-91f86d23f77d
result:
  content:
  - type: text
    text: Too many entries (122) exceed the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
  isError: true
```
/answer 7be9acf2-826a-44a1-9be6-91f86d23f77d allow
Control Request:
```yaml
id: f232285e-f53c-42ca-b9b8-0bd612c9cfb0
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  reason: Search for other Scanner instantiations that might need document-boundary awareness
  script: grep -rn "new Scanner(" .
```
/answer f232285e-f53c-42ca-b9b8-0bd612c9cfb0 allow
Control Request:
```yaml
id: c4adf74d-37bc-4b33-9a70-570669ce9078
result:
  structuredContent:
    stdout: "./xy/ai/workbench/mdast/MarkdownDocument.java:52:\t\tScanner scanner = new Scanner(new BufferReader(slice, 0), documentStart, documentEnd);\n./xy/ai/workbench/tools/Scanner.java:32:\t\t\tsubScanner = new Scanner(this);\n./xy/ai/workbench/tools/Scanner.java:42:\t\t\tisNextSequenceSub = new Scanner(this);\n./xy/ai/workbench/editors/md/AbstractRule.java:22:\t\treturn evaluateToken(new Scanner(new Scanner.CharacterScanner() {\n"
```
/answer c4adf74d-37bc-4b33-9a70-570669ce9078 allow
ReasoningToken: 69
ReasoningToken: 95
Control Request:
```yaml
id: 44777872-1aa6-4fcc-bf3e-edcabe908bd6
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/md/AbstractRule.java
  reason: Check if AbstractRule uses the same newline-prefix convention affected by BOF/EOF issue
```
/answer 44777872-1aa6-4fcc-bf3e-edcabe908bd6 allow
ReasoningToken: 210
Control Request:
```yaml
id: 6cf7736b-66b7-4d97-b818-e6d674761fca
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench/src
  reason: Check subclasses of AbstractRule for reliance on getReadCount semantics at EOF
  script: grep -rln "extends AbstractRule" .; echo ---; grep -rn "getReadCount" .
```
/answer 6cf7736b-66b7-4d97-b818-e6d674761fca allow
Control Request:
```yaml
id: 6fa37bf1-1af9-4f09-86e9-26025538a498
result:
  structuredContent:
    stdout: "./xy/ai/workbench/editors/md/PrefixLineRule.java\n./xy/ai/workbench/editors/md/LineMatchRule.java\n./xy/ai/workbench/editors/md/ListRule.java\n./xy/ai/workbench/editors/md/WordRule.java\n./xy/ai/workbench/editors/md/BlockRule.java\n./xy/ai/workbench/editors/md/EmphasisRule.java\n./xy/ai/workbench/editors/md/HeaderRule.java\n./xy/ai/workbench/editors/md/LinkRule.java\n---\n./xy/ai/workbench/mdast/nodes/AbstractNode.java:38:\t\t\t\tnn.start = s.getReadCount();\n./xy/ai/workbench/mdast/nodes/AbstractNode.java:51:\t\tn.end = n.start + s.getReadCount();\n./xy/ai/workbench/mdast/nodes/ScriptBlock.java:22:\t\twhile (s.getReadCount() < LIMIT && s.readNext() && !(endblock = s.isNextSequenceBounded(endBlock))\n./xy/ai/workbench/tools/Scanner.java:205:\t\t// getReadCount() (this is what makes the true, absolute end of the\n./xy/ai/workbench/tools/Scanner.java:223:\tpublic int getReadCount() {\n./xy/ai/workbench/editors/md/PrefixLineRule.java:22:\t\twhile (s.getReadCount() <= MAX_READ && s.readNext() && !(nl = s.isNewLine()))\n./xy/ai/workbench/editors/md/BlockRule.java:26:\t\twhile (s.getReadCount() < LIMIT && s.readNext() && !(endblock = s.isNextSequence(endBlock))\n./xy/ai/workbench/editors/md/EmphasisRule.java:31:\t\twhile (s.getReadCount() <= MAX_READ && s.readNext() && !s.isNewLine() && !(nextSequence = s.isNextSequence(eseq)))\n"
```
/answer 6fa37bf1-1af9-4f09-86e9-26025538a498 allow
ReasoningToken: 565
Control Request:
```yaml
id: 7e8b1d11-05a3-4a15-b5a1-515f0d0f640e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/ScriptBlock.java
  reason: Final review of ScriptBlock changes
```
/answer 7e8b1d11-05a3-4a15-b5a1-515f0d0f640e allow
Control Request:
```yaml
id: 88a920e7-505c-4674-a62d-c2d07ff9cbe6
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  reason: Final review of MarkdownDocument changes
```
/answer 88a920e7-505c-4674-a62d-c2d07ff9cbe6 allow
Control Request:
```yaml
id: 3d13cd2d-273d-4301-8ef5-a5574a01c69e
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/LineSection.java
  reason: Final review of LineSection changes
```
/answer 3d13cd2d-273d-4301-8ef5-a5574a01c69e allow
Control Request:
```yaml
id: f0a26185-ce5b-40ab-9375-2e4651c1bfbd
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PageSection.java
  reason: Final review of PageSection changes
```
/answer f0a26185-ce5b-40ab-9375-2e4651c1bfbd allow
ReasoningToken: 2818
Control Request:
```yaml
id: 4279dfcf-e385-4a0e-89b9-919c275ec29b
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Paragraph.java
  reason: Final check
```
/answer 4279dfcf-e385-4a0e-89b9-919c275ec29b allow
Control Request:
```yaml
id: bfd27504-1b02-41e9-ab10-837be1558369
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PrefixBlock.java
  reason: Final check
```
/answer bfd27504-1b02-41e9-ab10-837be1558369 allow
Control Request:
```yaml
id: 49ca1f57-8dbc-4db4-9a8d-6e628d4ed7f2
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/HeadingSection.java
  reason: Final check
```
/answer 49ca1f57-8dbc-4db4-9a8d-6e628d4ed7f2 allow
ReasoningToken: 98
Alle Änderungen sind vollständig. Zusammenfassung, was umgesetzt wurde:

## 1. `Scanner.java` – Kernmechanik

- **`documentStart`/`documentEnd`-Flags** an der Wurzel des Scanner-Baums (per neuem Konstruktor `Scanner(CharacterScanner, boolean, boolean)`), abfragbar via `isAtDocumentStart()` (readCount der Wurzel == 0) und `isAtDocumentEnd()`.
- **`isNextSequenceBounded(char[])`**: identisch zu `isNextSequence`, überspringt aber ein führendes `\n` am echten Dateianfang bzw. toleriert ein fehlendes abschließendes `\n` am echten Dateiende – ohne je ein Zeichen zu lesen/zu zählen. Dadurch bleibt jede Offset-Arithmetik exakt (kein "virtuelles Zeichen", das irgendwo kompensiert werden müsste).
- **Generischer Bugfix in `read()`**: Ein fehlgeschlagener Read (echtes EOF) erhöht `readCount` nicht mehr. Vorher wurde bei *jedem* Blattknoten, dessen Scan durch echtes Dateiende (statt durch ein erkanntes Muster) beendet wurde, `n.end` um 1 zu groß berechnet – kaskadierend bis zur Wurzel. Das war der von Ihnen vermutete "Fall des letzten Blattelements" in seiner allgemeinsten Form, unabhängig von einem konkreten `\n`-Muster.

## 2. Betroffene Knotenklassen

`HeadingSection`, `Paragraph`, `PrefixBlock` nutzen jetzt `isNextSequenceBounded` für ihr Prefix (nur BOF relevant, da ihr Muster nicht mit `\n` endet).

`PageSection`, `LineSection`, `ScriptBlock` nutzen `isNextSequenceBounded` für Muster, die **mit `\n` enden** (`"\n---\n"`, `"\nUSER\n"`, `"\n```\n"`) – hier greift zusätzlich die EOF-Toleranz. Das anschließende `s.unread()` ("Trenn-Zeichen für Kind-Scanning aufheben") wird jetzt mit `if (!s.isEOF())` abgesichert, da am echten Dateiende kein Zeichen mehr zum "Zurückgeben" existiert.

## 3. `MarkdownDocument.parse()`

Setzt `documentStart = (absStart == 0)` und `documentEnd = (absEnd == buffer.length())`. Das ist korrekt sowohl für den initialen Voll-Parse als auch für inkrementelles Re-Parsing: Ein Re-Parse berührt die echten Dokumentgrenzen nur dann, wenn tatsächlich der erste bzw. letzte Knoten des Baums betroffen ist – für alle anderen Fälle bleibt das bisherige, rein lokale Verhalten unverändert (dort existiert immer ein reales `\n` an der Grenze).

**Damit ist die Column-Agnostik vollständig erhalten:** Es wird nirgends ein Zeichen simuliert oder ein Positions-/Spaltenzähler mitgeführt – nur zwei Bits am Scan-Baum-Root sagen "hier gibt es keinen Nachbarn, verlange das geteilte Grenzzeichen nicht". Alle Parser bleiben davon unbeeinflusst, wenn sie mitten im Stream eingesetzt werden (Standardfall bei Re-Parsing).

Da hier kein Build/Test vorhanden ist, empfehle ich beim Kompilieren gezielt folgende Fälle zu prüfen: Dokument beginnt direkt mit `# Titel`, mit `---`, mit ` ``` `-Codeblock; Dokument endet ohne Trailing-Newline nach `---`, nach einer `USER`/`AGENT`-Markerzeile, nach ` ``` `; sowie ein Edit an Offset 0 bzw. am Dateiende, der ein Re-Parsing der ersten/letzten Sektion auslöst.
Mir ist noch eine Optimierung eingefallen. Wir brauchen die boolschen Flags nicht durschleifen wenn wir für die Bedingung denn Offset des Root Scanners nehmen wurden? Start trifft dann zu wenn dieser exakt null ist und Dateiende trifft zu wenn der Offset mit der Länge des Buffers übereinstimmt. Funktioniert das oder siehst du Probleme?
SystemInit: cwd=/home/user/xyan/XY.Archiv | session_id=c2bb703e-211f-4914-8ce2-9d5b8bbd948b | model=claude-sonnet-5 | plugins=
ReasoningToken: 10448
Kurze Antwort: **So direkt funktioniert es nicht**, und zwar aus einem sehr konkreten Grund in der aktuellen Architektur. Ich erkläre das Problem und biete danach eine sichere Variante an, die Ihre Grundidee (Boolean-Flags loswerden, stattdessen einen Offset vergleichen) tatsächlich umsetzen kann.

## Warum „Offset des Root-Scanners == 0“ nicht direkt geht

`MarkdownDocument.parse(absStart, absEnd)` erzeugt bei **jedem** Aufruf einen **neuen** `Scanner` über einen **neuen** `BufferReader`, der nur die Slice `[absStart, absEnd)` kennt und intern bei `pos = 0` beginnt:

```java
char[] slice = readChars(absStart, absEnd - absStart);
Scanner scanner = new Scanner(new BufferReader(slice, 0), ...);
```

`readCount` dieses Root-Scanners startet also **immer bei 0** – egal ob `absStart == 0` (echter Dateianfang) oder `absStart == 12345` (Re-Parse mitten im Dokument). `getReadCount()` ist bewusst **slice-relativ**, nicht absolut, weil genau diese Relativität es `AbstractNode.scan()` und `MarkdownDocument.replace()` erlaubt, die zurückgegebenen `Node.start`/`.end`-Werte anschließend per `c.start += sec.start` korrekt in den Gesamtbaum einzuhängen.

Würden wir `isAtDocumentStart()` einfach als `readCount == 0` definieren, wäre das bei **jedem einzigen** `parse()`-Aufruf zu Beginn wahr – also auch bei ganz normalem Re-Parsing mitten im Dokument. Das wäre eine Regression: Wir würden an beliebigen Stellen im Dokument plötzlich wieder das führende `\n` "wegschummeln", obwohl dort echte Nachbarn existieren. Das genau war ja der Grund, warum wir das Flag ursprünglich brauchen – es unterscheidet "Anfang dieses Scans" von "Anfang des ganzen Dokuments", und diese beiden fallen nur zufällig bei `absStart == 0` zusammen.

## Der Denkfehler, sauber gemacht: readCount absolut machen?

Man könnte natürlich `readCount` beim Root-Scanner mit `absStart` statt `0` initialisieren, dann wäre `readCount` ab da absolut und `readCount == 0` bzw. `readCount == buffer.length()` würden tatsächlich funktionieren. Das hat aber einen Haken: `readCount` ist derselbe Wert, den `AbstractNode.scan()` für `nn.start = s.getReadCount()` verwendet, also für die **relativen** Kindknoten-Offsets. Würde man ihn absolut machen, müsste `parse()` am Ende **alle** zurückgegebenen `start`/`end`-Werte wieder um `absStart` zurückrechnen, bevor `MarkdownDocument.replace()` sie (die schon mit `+= sec.start` rechnet) weiterverarbeitet – sonst zählt `absStart` doppelt. Das ist exakt dieselbe Klasse von Fehler, vor der ich beim „virtuellen Zeichen“-Ansatz gewarnt hatte: ein gemeinsam genutzter Zähler mit zwei verschiedenen Bedeutungen ist eine verlässliche Quelle für Off-by-N-Fehler, und die Korrektur müsste in `MarkdownDocument.replace()` erfolgen – einer Stelle, die wir eigentlich gar nicht anfassen wollten.

## Die sichere Variante Ihrer Idee

Man kann Ihren Kerngedanken trotzdem retten, ohne `readCount` anzufassen: einen **separaten** Absolut-Offset am Root mitführen, der ausschließlich für die Grenz-Prüfung existiert und nichts mit der Node-Offset-Berechnung zu tun hat:

```java
private int documentOffset = -1; // absolute Position von readCount==0; -1 = unbekannt/kein Dokumentkontext
private int documentLength = -1; // Gesamtlänge des Dokuments; -1 = unbekannt

public Scanner(CharacterScanner scan, int documentOffset, int documentLength) {
    this.scan = scan;
    this.documentOffset = documentOffset;
    this.documentLength = documentLength;
}

public boolean isAtDocumentStart() {
    Scanner root = root();
    return root.documentOffset == 0 && root.readCount == 0;
}

public boolean isAtDocumentEnd() {
    Scanner root = root();
    return root.documentLength >= 0 && root.documentOffset + root.readCount == root.documentLength;
}
```

Und in `MarkdownDocument.parse()`:
```java
new Scanner(new BufferReader(slice, 0), absStart, buffer.length());
```

Das ist konzeptionell genau das, was Sie vorschlagen (Vergleich gegen einen Offset statt gegen zwei Booleans), nur ohne `readCount` selbst umzudeuten – dadurch bleibt die bestehende, bereits verifizierte Offset-Arithmetik in `AbstractNode`/`MarkdownDocument.replace()` komplett unberührt.

**Wichtig aber**: Das reduziert die Menge an durchzuschleifender Information nicht wirklich – statt zwei `boolean` schleifen wir zwei `int` durch (Basis-Offset und Gesamtlänge). Wir brauchen weiterhin beide Werte, weil "ist Slice-Anfang == 0" und "ist Slice-Ende == Pufferlänge" zwei voneinander unabhängige Fakten sind, die sich nicht aus einer einzigen Zahl ableiten lassen. Der einzige echte Vorteil wäre: `documentOffset`/`documentLength` sind etwas generischer/aussagekräftiger als die zwei zweckgebundenen Booleans, und ließen sich vielleicht künftig auch anderweitig nutzen (z. B. echte absolute Debug-Offsets aus dem Scanner heraus).

## Mein Vorschlag

Da der aktuelle Zwei-Booleans-Ansatz bereits korrekt, minimal und risikofrei ist, und die Offset-Variante keine tatsächliche Vereinfachung bringt (gleiche Menge an Zustand, nur anders benannt), würde ich **so lassen, wie es ist** – außer Sie sehen einen konkreten zukünftigen Nutzen für einen generischen "absoluten Scanner-Offset" (dann setze ich das oben skizzierte `documentOffset`/`documentLength`-Schema gerne um, unter Beibehaltung der Trennung von `readCount`).

Soll ich es umstellen, oder bei den zwei Flags belassen?
Nein danke, ich stimme dir zu. Die Semantik ändert sich und wird nur verlagert. Absolute Streamoffsets durchzuschleifen widerspricht dem agnostischen Ansatz.
SystemInit: cwd=/home/user/xyan/XY.Archiv | session_id=c2bb703e-211f-4914-8ce2-9d5b8bbd948b | model=claude-sonnet-5 | plugins=
Gerne. Dann bleibt es bei den zwei Flags (`documentStart`/`documentEnd`) in `Scanner.java`, `MarkdownDocument.java` und den sechs Node-Klassen (`HeadingSection`, `Paragraph`, `PageSection`, `LineSection`, `PrefixBlock`, `ScriptBlock`) genau so, wie umgesetzt.

Kurz zusammengefasst, warum das die richtige Wahl ist: Die zwei Booleans transportieren nur ein minimales, lokal auswertbares Bit an Kontext ("berührt dieser Scan die echte Dokumentgrenze") – sie beschreiben eine Eigenschaft des *aktuellen Scan-Aufrufs*, nicht eine fortlaufende absolute Position. Absolute Offsets durchzureichen (oder gar `readCount` selbst absolut zu machen) würde genau die Eigenschaft aufgeben, die den ganzen Parser-Stack column-/positionsagnostisch hält: dass jeder Knoten rein aus lokal gelesenen Zeichen entscheidet, ohne globalen Zustand kennen zu müssen. Die Flags sind insofern kein Kompromiss, sondern die konsequente Fortführung desselben Prinzips – lokal auswertbar, aber ausreichend, um die beiden nicht lokal erkennbaren Sonderfälle (kein Vorgänger, kein Nachfolger) korrekt zu behandeln.

Der Stand ist damit final:

- **BOF**: `isNextSequenceBounded` überspringt ein führendes `\n`, wenn `isAtDocumentStart()`.
- **EOF-Muster**: dieselbe Methode toleriert ein fehlendes abschließendes `\n`, wenn `isAtDocumentEnd()`; die drei betroffenen `unread()`-Aufrufe (`PageSection`, `LineSection`, `ScriptBlock`) sind entsprechend mit `if (!s.isEOF())` abgesichert.
- **Generischer EOF-Zählfehler**: `Scanner.read()` erhöht `readCount` nicht mehr bei einem gescheiterten (EOF-)Read, wodurch das jeweils letzte Blattelement (und alle Vorfahren) keinen um 1 zu großen `.end` mehr bekommt.
- **`MarkdownDocument.parse()`**: setzt `documentStart = absStart == 0` und `documentEnd = absEnd == buffer.length()`, korrekt sowohl für den initialen Voll-Parse als auch für jedes inkrementelle Re-Parsing.

Falls beim Kompilieren/Testen doch noch Randfälle auffallen (z. B. die genannten Fälle: Dokument beginnt/endet direkt mit `---`, ` ``` ` oder einer `USER`/`AGENT`-Markerzeile ohne Trailing-Newline), sag einfach Bescheid.
