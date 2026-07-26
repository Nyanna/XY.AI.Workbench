Ich brauche Hilfe beim Denken.
Im Beispielblock wird "/answer" als "Paragraph" ausgezeichnet. Dies liegt daran, dass "Paragraph" ein gültiges Ende für die "CONTROL_REQUEST" `LineSection` ist. Ein Sektionsende geht also einem möglichen Kind voraus. Was wäre, wenn wir die Präferenz umkehren und sagen: Lies, solange es an gültigen Kindern zu interpretieren gibt und beende dann die Sektion.
Mann könnte die Logik ändern auf: Ist gültiges Kind oder Ende in `xy.ai.workbench.mdast.nodes.AbstractNode.scan(Scanner, Node)`

* AbstractNode: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/AbstractNode.java`

## Beispiel:

Control Request:
```yaml
id: 186532fe-7125-40f6-bd04-5a868b1e938e
```
/answer 186532fe-7125-40f6-bd04-5a868b1e938e allow
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=e2bd4412-7840-4b20-a887-3ec7e20e2887 | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 4531e4a7-f669-4c35-b5d6-7a81248ff09a
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/AbstractNode.java
  reason: Analyse der scan-Methode für Sektionsende vs. Kind-Präferenz
```
/answer 4531e4a7-f669-4c35-b5d6-7a81248ff09a allow
ReasoningToken: 22
Control Request:
```yaml
id: d4717ced-3385-4fa1-8f1a-bce76c957a4a
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast
  reason: "Kontext zu LineSection, CONTROL_REQUEST, Paragraph finden"
```
/answer d4717ced-3385-4fa1-8f1a-bce76c957a4a allow
Control Request:
```yaml
id: 7d86e8a2-37c4-412e-9163-3774d7ca9a6f
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
/answer 7d86e8a2-37c4-412e-9163-3774d7ca9a6f allow
ReasoningToken: 652
Control Request:
```yaml
id: 553e66e7-2f4a-4a7b-b028-415e33e2838b
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/LineSection.java
  reason: LineSection Definition prüfen
```
/answer 553e66e7-2f4a-4a7b-b028-415e33e2838b allow
Control Request:
```yaml
id: cc850414-c57c-4b22-8c4e-866258dc7e7f
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Paragraph.java
  reason: Paragraph Definition prüfen
```
/answer cc850414-c57c-4b22-8c4e-866258dc7e7f allow
Control Request:
```yaml
id: 5d0a82b9-a0e4-42ec-a52f-0b4596bc21e0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Elements.java
  reason: Elements/Kategorien prüfen inkl CONTROL_REQUEST
```
/answer 5d0a82b9-a0e4-42ec-a52f-0b4596bc21e0 allow
Control Request:
```yaml
id: 0e9a61b3-06d8-463b-a845-029a7b3c40f0
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Node.java
  reason: Node Klasse prüfen
```
/answer 0e9a61b3-06d8-463b-a845-029a7b3c40f0 allow
Control Request:
```yaml
id: b0cfc6f2-d27d-4066-8da1-c377bce07703
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/PrefixBlock.java
  reason: PrefixBlock isStart/isValid prüfen für Risikoanalyse
```
/answer b0cfc6f2-d27d-4066-8da1-c377bce07703 allow
Control Request:
```yaml
id: ce57d648-1bc1-42fb-9744-0f8af9058a61
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/ScriptBlock.java
  reason: ScriptBlock isStart/isValid prüfen für Risikoanalyse
```
/answer ce57d648-1bc1-42fb-9744-0f8af9058a61 allow
ReasoningToken: 3210
Control Request:
```yaml
id: 02f3e7de-e5cc-4624-bb94-bc0be654ca4f
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/AbstractNode.java
  old_text: "\t\tnextChar: while (!isEnd(s, n)) {\n\t\t\tfor (var child : childNodes) {\n\t\t\t\tvar nn = new Node(n, child);\n\t\t\t\tnn.start = s.getReadCount();\n\t\t\t\tScanner sub = s.getSubscanner();\n\n\t\t\t\tif (child.scan(sub, nn)) {\n\t\t\t\t\tn.children.add(nn);\n\t\t\t\t\tnn.enableSpellcheck = child.enableSpellcheck && enableSpellcheck;\n\t\t\t\t\tcontinue nextChar;\n\t\t\t\t} else\n\t\t\t\t\tsub.reset();\n\t\t\t}\n\t\t\tif (!s.readNext())\n\t\t\t\tbreak;\n\t\t}"
  new_text: "\t\tnextChar: while (true) {\n\t\t\tfor (var child : childNodes) {\n\t\t\t\tvar nn = new Node(n, child);\n\t\t\t\tnn.start = s.getReadCount();\n\t\t\t\tScanner sub = s.getSubscanner();\n\n\t\t\t\tif (child.scan(sub, nn)) {\n\t\t\t\t\tn.children.add(nn);\n\t\t\t\t\tnn.enableSpellcheck = child.enableSpellcheck && enableSpellcheck;\n\t\t\t\t\tcontinue nextChar;\n\t\t\t\t} else\n\t\t\t\t\tsub.reset();\n\t\t\t}\n\t\t\tif (isEnd(s, n))\n\t\t\t\tbreak;\n\t\t\tif (!s.readNext())\n\t\t\t\tbreak;\n\t\t}"
  reason: Kind-Präferenz vor Sektionsende umsetzen
```
/answer 02f3e7de-e5cc-4624-bb94-bc0be654ca4f allow
ReasoningToken: 200
Jetzt zur Einordnung, was das bewirkt und worauf zu achten ist:

## Warum das Beispiel bisher scheitert

`CONTROL_REQUEST` hat `terminalNodes`, die u. a. `Basics.PARAGRAPH` enthalten. `Paragraph.isStart` prüft nur, ob als nächstes ein `\n` folgt – das ist nach dem Fenced-Block (dem YAML mit der `id`) praktisch immer der Fall. Da `isEnd(s, n)` in der alten Schleife **vor** dem Children-Versuch geprüft wurde, "gewinnt" das Sektionsende, sobald irgendein Terminal-Knoten an der aktuellen Position *starten könnte* – unabhängig davon, ob dort eigentlich `/answer ...` als gültiges `ANSWER`-Kind steht. Die Sektion endet, und `/answer ...` wird danach nur noch als normaler `Paragraph` auf Root-Ebene erkannt.

## Die Umkehrung

Ich habe die Schleife in `scan()` so geändert, dass zuerst alle `childNodes` an der aktuellen Position durchprobiert werden (inkl. deren vollständiger `isValid`-Prüfung) und **erst wenn kein Kind passt**, `isEnd(s, n)` geprüft wird. Damit gilt jetzt: „lies weiter, solange sich ein gültiges Kind konstruieren lässt – beende die Sektion nur, wenn das nicht mehr geht.“

Im Beispiel bedeutet das: An der Position vor `/answer ...` wird zunächst `Tools.ANSWER` (ein `PrefixBlock` mit Präfix `\n/answer`) versucht. Das matcht exakt und vollständig, wird also als Kind von `CONTROL_REQUEST` akzeptiert, bevor `Paragraph` überhaupt als Sektionsende in Betracht gezogen wird.

## Warum das nicht sofort andere Fälle kaputt macht

Der naheliegende Einwand: Wenn Kinder Vorrang haben, könnte z. B. `Paragraph` als Kind einer `USER`-Section an der Stelle "gewinnen", wo eigentlich die Section durch den `AGENT`-Marker enden sollte (`Paragraph.isStart` matcht ja quasi überall auf bloßes `\n`). Das passiert hier aber nicht, weil:

- `Paragraph.isEndInner` selbst wieder alle `ROOT_ELEMENTS` (außer sich selbst) als Terminals prüft – darunter auch `AGENT`/`USER`. Direkt an der Grenzstelle würde `Paragraph` also sofort wieder enden → Länge 0.
- `Paragraph.isValid` verlangt `n.end - n.start > 3`. Ein Nullbreiten-Match ist damit ungültig, `child.scan(...)` liefert `false`, der Scanner wird zurückgesetzt, und der nächste Kandidat (bzw. am Ende `isEnd`) kommt zum Zug.

Das ist aber ein **impliziter** Schutzmechanismus, kein expliziter. Knoten, deren `isStart` sehr freizügig ist (wie reines `\n`) und deren `isValid`/`isEndInner` keine Mindestlänge/Terminal-Prüfung erzwingen, könnten durch die neue Präferenz theoretisch fälschlich an einer Sektionsgrenze "anbeißen" und sie verschlucken. `PrefixBlock` und `ScriptBlock` (wie `ANSWER`, `SCRIPTBLOCK`) sind unkritisch, da sie sehr spezifische Präfixe verlangen (`/answer`, ```` ``` ````) – die matchen nicht versehentlich an beliebigen Sektionsgrenzen.

## Empfehlung

Die Änderung ist konzeptionell stimmig und behebt genau das beschriebene Problem. Ich würde trotzdem:

1. Testen, ob bestehende Tests/Fixtures (Heading-, Page-, Chat-Grenzen) noch grün sind – speziell Fälle, wo unmittelbar nach einem `AGENT`/`USER`-Marker wieder ein Paragraph-artiger Text beginnt.
2. Ein Auge auf künftige neue Knotentypen haben: Wer einen sehr laxen `isStart` hat, sollte immer auch über `isValid`/`isEndInner` sicherstellen, dass er nicht an einer eigentlich terminierenden Position "gewinnt".

Soll ich die vorhandenen Tests laufen lassen, um das zu verifizieren?
Es gibt keine Tests. Die Prüfung kann nur logisch erfolgen. Paragraph ist ein Catch-All der alles Schlucken soll was nicht anders erlaubt. Ich habe auch evaluiert im Pargraph `isStart` selbst die Siblings zu evaluieren bevor true returned wird. Das erzeugt jedoch alles Zusätzliche reads auf dem Stream. 
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=e2bd4412-7840-4b20-a887-3ec7e20e2887 | model=claude-sonnet-5 | plugins=
ReasoningToken: 3744
Gute Beobachtung, und der Vergleich mit der Alternative (Lookahead direkt in `Paragraph.isStart`) hilft, die Korrektheit der zentralen Lösung sauber zu begründen.

## Warum der Fix in `AbstractNode.scan` ausreicht, ohne dass `Paragraph` selbst etwas tun muss

`Paragraph` hat keine eigenen `childNodes` (`Elements.NONE`). Wenn `Paragraph` also als Kandidat-Kind (z. B. von `CONTROL_REQUEST` über dessen Terminalliste, oder direkt als Kind in `Page`/`Heading`/`Chat`) versucht wird, läuft *sein eigener* `scan()`-Aufruf so ab:

1. `isStart` prüft nur `\n` – matcht praktisch überall.
2. Die `for`-Schleife über `childNodes` ist leer → sofort `isEnd(sub, nn)`.
3. `isEnd` ist rekursiv über die Elternkette definiert:
   ```java
   n.parent != null && n.parent.instance.isEnd(s, n.parent) || isEndInner(s)
   ```
   D. h. bevor `Paragraph` überhaupt ein Zeichen konsumiert, wird geprüft, ob **irgendein Vorfahre** (inkl. `CONTROL_REQUEST` mit seiner Terminal-Liste) an genau dieser Position bereits enden würde.

Genau an der Stelle, wo z. B. `Paragraph` fälschlich als Terminal von `CONTROL_REQUEST` "gewinnen" könnte, ist diese Bedingung fast immer sofort wahr → Länge 0 → `isValid` (`length > 3`) schlägt fehl → `child.scan(...)` liefert `false` → der Scanner wird zurückgesetzt, nichts wurde verbraucht, der nächste Kandidat bzw. am Ende `isEnd(s, n)` selbst kommt zum Zug.

**Der Mechanismus, den du als "Lookahead auf Siblings/Terminals" separat in `Paragraph.isStart` bauen wolltest, existiert also bereits generisch in `isEnd()` – er wird nur bisher an der falschen Stelle im Ablauf konsultiert (vor statt nach dem Kind-Versuch).** Die Umkehrung der Reihenfolge nutzt exakt diese vorhandene, rekursive Prüfung wieder – nur eben verzögert bis zu dem Punkt, wo sie wirklich gebraucht wird (nachdem ein "schärferes" Kind wie `ANSWER` seine Chance hatte).

## Warum das die günstigere Lösung ist (dein Einwand zu "extra reads")

- **Lookahead in `Paragraph.isStart`**: müsste bei *jedem* `\n` im Dokument (also potenziell sehr häufig, da `Paragraph` überall als Fallback auftaucht) vorab alle Terminalkandidaten testen, bevor überhaupt entschieden ist, ob `Paragraph` startet. Das sind zusätzliche, spekulative Sub-Scans, die im Erfolgsfall (Paragraph startet wirklich) komplett verschwendet sind.
- **Reihenfolge-Swap in `scan()`**: kostet nur *einen zusätzlichen* Kind-Versuch genau an der Position, wo vorher sofort `isEnd` gegriffen hätte. Das ist ein einmaliger, lokal begrenzter Mehraufwand pro Sektionsende – nicht pro Zeichen/pro `\n` im gesamten Dokument. Und die Prüfmaschinerie (`isEnd`, Subscanner, `isValid`) wird wiederverwendet statt dupliziert.

Die zentrale Lösung ist also nicht nur korrekter im Sinne deiner ursprünglichen Idee ("gültiges Kind vor Ende"), sondern auch strikt billiger als die dezentrale Alternative.

## Das eigentliche Korrektheits-Invariant

Damit diese Umkehrung nicht in Zukunft irgendwo unbemerkt bricht, hier die Bedingung, die implizit genutzt wird und die man im Hinterkopf behalten sollte:

> Ein Knoten mit schwachem `isStart` (matcht an vielen/fast allen Positionen, "Catch-All"), muss über `isValid` oder `isEndInner` sicherstellen, dass ein Match der Länge 0 (oder generell ein Match genau an einer gültigen Vorfahren-Grenze) verworfen wird.

`Paragraph` erfüllt das zufällig/bewusst durch `length > 3`. Das ist aber **nicht** durch den Typ erzwungen – es ist eine implizite Vereinbarung. Falls künftig ein neuer Catch-All-Knotentyp hinzukommt (z. B. irgendein generischer Text-Container), der `isStart` ähnlich lax hält, aber keine Mindestlängen-/Terminal-Prüfung besitzt, würde er mit der neuen Reihenfolge fälschlich Sektionsgrenzen verschlucken können (0-Zeichen- oder Teil-Match, der trotzdem als "gültig" durchgeht).

**Empfehlung:** Diese Invariante als Kommentar bei `AbstractNode.isValid`/`isEndInner` (oder im Javadoc von `scan`) festhalten, damit sie bei künftigen Node-Implementierungen nicht implizit bleibt, sondern bewusst beachtet wird. Soll ich einen entsprechenden Kommentar in `AbstractNode.java` ergänzen?
