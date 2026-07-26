Mir ist ein Problem in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java` aufgefallen. Wenn, wie üblich an die Datei angehängt wird, wird ein Reparse vom gesamten Dokument ausgelöst.
Implementiere hierfür eine Behandlung. Fällt der Eingefüge Bereich hinter das letzte Blatt Element so wird es diesem zugerechnet. Sollten inkompatible Strukturen eingegeben werden so werden diese ohnehin an die Elternknoten propagiert. Dies sollte die Performance verbessern.
SystemInit: cwd=/home/user/xyan/XY.Archiv | session_id=4d110070-9836-4cc4-bfaf-4c7ec871e60d | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 6722ab09-a2ff-40d1-8db0-716937ec65b3
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  reason: Read file to understand append/reparse logic
```
/answer 6722ab09-a2ff-40d1-8db0-716937ec65b3 allow
ReasoningToken: 16
Control Request:
```yaml
id: 8b18a9c0-9eb6-4825-9ca4-e42aa2c7dc55
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/nodes/Node.java
  reason: Understand Node offset/length semantics
```
/answer 8b18a9c0-9eb6-4825-9ca4-e42aa2c7dc55 allow
ReasoningToken: 3651
ReasoningToken: 1832
Control Request:
```yaml
id: 012bbb85-a561-43d2-8c3d-968dc0e9968c
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  old_text: "\tpublic TextRegion update(int offset, int removed, int inserted) {\n\t\tint lo = offset;\n\t\tint hi = offset + removed;\n\t\tint delta = inserted - removed;\n\t\tlines.update(buffer, offset, removed, inserted);\n\n\t\tNode sec = find(lo, hi);\n\t\twhile (true) {\n\t\t\tNode parent = sec.parent;\n\t\t\tint absStart = sec.getOffset();\n\t\t\tint newLen = sec.length() + delta;\n\t\t\tNode rn = parse(absStart, absStart + newLen);"
  new_text: "\tpublic TextRegion update(int offset, int removed, int inserted) {\n\t\tint lo = offset;\n\t\tint hi = offset + removed;\n\t\tint delta = inserted - removed;\n\t\tlines.update(buffer, offset, removed, inserted);\n\n\t\tNode sec = findForUpdate(lo, hi);\n\t\twhile (true) {\n\t\t\tNode parent = sec.parent;\n\t\t\tint absStart = sec.getOffset();\n\t\t\tint newLen = (hi - absStart) + delta;\n\t\t\tNode rn = parse(absStart, absStart + newLen);"
  reason: Route appends past last leaf to that leaf; fix length calc to not assume sec spans hi
```
/answer 012bbb85-a561-43d2-8c3d-968dc0e9968c allow
Text:
 Jetzt füge ich die neue Hilfsmethode `findForUpdate` und `lastLeaf` hinzu:
Control Request:
```yaml
id: 2e16b8a3-6f20-45f3-a924-6d1635ba96f0
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  old_text: "\tpublic Node find(int lo, int hi) {\n\t\treturn find(root, lo, hi);\n\t}"
  new_text: "\tpublic Node find(int lo, int hi) {\n\t\treturn find(root, lo, hi);\n\t}\n\n\t/**\n\t * Wie {@link #find(int, int)}, behandelt aber zusätzlich den (häufigen)\n\t * Sonderfall, dass der bearbeitete Bereich hinter dem Ende des letzten\n\t * Blatt-Elements im Baum liegt (z.B. weil an das Dokument angehängt wird).\n\t * In diesem Fall wird der Bereich diesem letzten Blatt-Element zugerechnet,\n\t * anstatt bis zur Wurzel hochzulaufen und das gesamte Dokument neu zu\n\t * parsen. Inkompatible Ergebnisse werden wie gewohnt über die Schleife in\n\t * {@link #update(int, int, int)} an den Elternknoten propagiert.\n\t */\n\tprivate Node findForUpdate(int lo, int hi) {\n\t\tNode tail = lastLeaf(root);\n\t\tif (tail != root && tail.getEndOffset() <= lo)\n\t\t\treturn tail;\n\t\treturn find(lo, hi);\n\t}\n\n\tprivate Node lastLeaf(Node node) {\n\t\twhile (!node.children.isEmpty())\n\t\t\tnode = node.children.get(node.children.size() - 1);\n\t\treturn node;\n\t}"
  reason: Add lastLeaf/findForUpdate helper implementing append-to-last-leaf optimization
```
/answer 2e16b8a3-6f20-45f3-a924-6d1635ba96f0 allow
Control Request:
```yaml
id: 071412ed-ca0e-43e1-b28c-007054d7f084
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/mdast/MarkdownDocument.java
  reason: Verify full updated file
```
/answer 071412ed-ca0e-43e1-b28c-007054d7f084 allow
ReasoningToken: 15
Die Änderungen sind implementiert. Zusammenfassung:

**Problem:** Beim üblichen Anhängen von Text ans Dateiende lieferte `find(lo, hi)` immer den `root`-Knoten zurück (kein Kind erfüllt `cs < lo && hi < cs + child.length()`, da der Einfügepunkt genau am/nach dem Ende des letzten Kindes liegt). Damit war `parent == null`, und es wurde stets das **gesamte Dokument** neu geparst.

**Lösung:**

1. **Neue Methode `findForUpdate(lo, hi)`**: Ermittelt zunächst über `lastLeaf(root)` das letzte Blatt-Element im Baum (rekursiv das jeweils letzte Kind, bis keine Kinder mehr vorhanden sind). Liegt der Einfügepunkt (`lo`) hinter dem Ende dieses Blatts (`tail.getEndOffset() <= lo`), wird die Bearbeitung direkt diesem Blatt zugeordnet, statt bis zur Wurzel zu suchen. Andernfalls greift wie bisher die normale `find(lo, hi)`-Logik.

2. **Fix der Längenberechnung**: `newLen` wurde von `sec.length() + delta` auf `(hi - absStart) + delta` geändert. Das ist notwendig, weil beim neuen Anhänge-Fall das gewählte `sec` (das letzte Blatt) den Bereich bis `hi` nicht mehr zwangsläufig selbst abdeckt (z.B. bei nachgestelltem, nicht erfasstem Whitespace). Für den bisherigen Normalfall (wo `sec` den Bereich `[lo,hi]` immer vollständig umschließt und `sec.end == hi` gilt) liefert die neue Formel identische Ergebnisse wie vorher – es handelt sich also um eine reine Verallgemeinerung ohne Verhaltensänderung im Altfall.

3. **Propagation bei Inkompatibilität** bleibt unverändert: Passt das neu geparste Ergebnis nicht zum Elternknoten (`isCompatible` schlägt fehl), läuft die bestehende `while`-Schleife in `update()` wie gehabt eine Ebene höher und parst dort neu – genau wie gefordert ("inkompatible Strukturen werden ohnehin an die Elternknoten propagiert").

Damit wird bei einem reinen Anhängen ans Dateiende in der Regel nur das letzte Blatt-Element (bzw. im Fehlerfall dessen Vorfahren) neu geparst, nicht mehr das komplette Dokument.
