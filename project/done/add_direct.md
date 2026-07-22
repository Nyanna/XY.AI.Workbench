Füge `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java` eine Methode hinzu, um direkt einen Newline Offset anzupassen. Resize Shift und Puffergröße müssen beachtet werden. Der LineIndex wird in einem Anwendungsfall von einer externen Schleife befüllt.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=c2d5c029-0379-4b09-bedc-de9fce30692d | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: e3cd8eba-c187-4a00-971a-2ef8e43d2e6d
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java
  reason: Inspect LineIndex.java structure to add offset-adjustment method
```
/answer e3cd8eba-c187-4a00-971a-2ef8e43d2e6d allow
ReasoningToken: 11
Control Request:
```yaml
id: d618a078-c998-4377-a789-2eb1a172fbb0
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java
  old_text: "\tpublic int lineCount() {\n\t\treturn count + 1;\n\t}"
  new_text: "\t/**\n\t * Fuegt einen einzelnen Newline-Offset direkt am Ende des Index hinzu.\n\t * Gedacht fuer den Fall, dass eine externe Schleife den Puffer selbst\n\t * durchsucht (z.B. Zeichen fuer Zeichen oder blockweise) und dabei\n\t * gefundene '\\n'-Positionen unmittelbar melden will, anstatt den ganzen\n\t * Puffer per {@link #update(Buffer)} neu scannen zu lassen.\n\t * <p>\n\t * Die Methode kuemmert sich dabei um:\n\t * <ul>\n\t * <li>Resize: die interne Kapazitaet von {@code nlIndex} wird bei Bedarf\n\t * vergroessert ({@link #ensureCapacity(int)}), bevor der neue Eintrag\n\t * geschrieben wird.</li>\n\t * <li>Puffergroesse: {@code bufferLength} wird nachgezogen, falls der\n\t * gemeldete Offset ausserhalb der bisher bekannten Puffergroesse liegt.</li>\n\t * </ul>\n\t * Die Offsets muessen streng aufsteigend gemeldet werden (wie es beim\n\t * sequentiellen Scannen eines Puffers automatisch der Fall ist), da der\n\t * Index intern sortiert gehalten wird.\n\t *\n\t * @param offset absoluter Offset des '\\n'-Zeichens im Puffer\n\t * @throws IllegalArgumentException wenn der Offset nicht groesser als der\n\t *             zuletzt eingetragene Offset ist\n\t */\n\tpublic void addNewlineOffset(int offset) {\n\t\tif (count > 0 && offset <= nlIndex[count - 1])\n\t\t\tthrow new IllegalArgumentException(\n\t\t\t\t\t\"offset (\" + offset + \") muss groesser als der letzte Eintrag (\" + nlIndex[count - 1] + \") sein\");\n\t\tensureCapacity(count + 1);\n\t\tnlIndex[count++] = offset;\n\t\tif (offset >= bufferLength)\n\t\t\tbufferLength = offset + 1;\n\t}\n\n\tpublic int lineCount() {\n\t\treturn count + 1;\n\t}"
  reason: Neue Methode addNewlineOffset() hinzufügen für externe Befüll-Schleife
```
/answer d618a078-c998-4377-a789-2eb1a172fbb0 allow
ReasoningToken: 763
Ich habe der Klasse `LineIndex` die Methode `addNewlineOffset(int offset)` hinzugefügt (direkt vor `lineCount()`). Sie ist für den Anwendungsfall gedacht, dass eine externe Schleife den Puffer selbst durchsucht und gefundene `\n`-Positionen einzeln meldet, statt den gesamten Puffer per `update(Buffer)` neu zu scannen.

Was die Methode konkret macht:

1. **Validierung/Monotonie**: Prüft, dass der neue Offset größer ist als der zuletzt eingetragene Wert (`nlIndex[count - 1]`), da der Index intern sortiert gehalten werden muss – wirft sonst eine `IllegalArgumentException`.
2. **Resize**: Ruft vor dem Schreiben `ensureCapacity(count + 1)` auf, damit `nlIndex` bei Bedarf vergrößert wird (analog zum bestehenden Resize-Mechanismus in `update`).
3. **Eintrag anhängen**: Schreibt den Offset an Position `count` und erhöht `count`.
4. **Puffergröße**: Zieht `bufferLength` nach, falls der gemeldete Offset größer/gleich der bisher bekannten Puffergröße ist (`bufferLength = offset + 1`), damit `lineEndOffset`/`bufferLength()` weiterhin korrekte Werte liefern.

Die Methode nutzt bewusst die bestehende private `ensureCapacity`-Logik wieder, statt eine eigene Wachstumsstrategie einzuführen, damit das Verhalten konsistent mit dem restlichen Klassencode bleibt.
Fall 1. muss behandelt werden. In diesem Fall soll ebenfalls geshiftet werden. Lagere die Logik dazu einer separaten privaten Methode.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=c2d5c029-0379-4b09-bedc-de9fce30692d | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 206f57db-9cb8-4ea8-9e9a-fa1742ef90cb
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java
  old_text: "\t * <li>Puffergroesse: {@code bufferLength} wird nachgezogen, falls der\n\t * gemeldete Offset ausserhalb der bisher bekannten Puffergroesse liegt.</li>\n\t * </ul>\n\t * Die Offsets muessen streng aufsteigend gemeldet werden (wie es beim\n\t * sequentiellen Scannen eines Puffers automatisch der Fall ist), da der\n\t * Index intern sortiert gehalten wird.\n\t *\n\t * @param offset absoluter Offset des '\\n'-Zeichens im Puffer\n\t * @throws IllegalArgumentException wenn der Offset nicht groesser als der\n\t *             zuletzt eingetragene Offset ist\n\t */\n\tpublic void addNewlineOffset(int offset) {\n\t\tif (count > 0 && offset <= nlIndex[count - 1])\n\t\t\tthrow new IllegalArgumentException(\n\t\t\t\t\t\"offset (\" + offset + \") muss groesser als der letzte Eintrag (\" + nlIndex[count - 1] + \") sein\");\n\t\tensureCapacity(count + 1);\n\t\tnlIndex[count++] = offset;\n\t\tif (offset >= bufferLength)\n\t\t\tbufferLength = offset + 1;\n\t}"
  new_text: "\t * <li>Puffergroesse: {@code bufferLength} wird nachgezogen, falls der\n\t * gemeldete Offset ausserhalb der bisher bekannten Puffergroesse liegt.</li>\n\t * <li>Reihenfolge: meldet die externe Schleife ausnahmsweise einen Offset,\n\t * der nicht groesser als der zuletzt eingetragene ist (z.B. weil ein\n\t * Puffer-Bereich erneut oder nicht streng aufsteigend gescannt wurde), so\n\t * wird der Eintrag an der korrekten sortierten Position eingefuegt und\n\t * der Rest des Index entsprechend nach hinten verschoben (siehe\n\t * {@link #insertShifted(int)}).</li>\n\t * </ul>\n\t * Im Normalfall (streng aufsteigende Offsets, wie es beim sequentiellen\n\t * Scannen eines Puffers ueblich ist) wird einfach an das Ende angehaengt.\n\t *\n\t * @param offset absoluter Offset des '\\n'-Zeichens im Puffer\n\t */\n\tpublic void addNewlineOffset(int offset) {\n\t\tensureCapacity(count + 1);\n\t\tif (count > 0 && offset <= nlIndex[count - 1])\n\t\t\tinsertShifted(offset);\n\t\telse\n\t\t\tnlIndex[count++] = offset;\n\t\tif (offset >= bufferLength)\n\t\t\tbufferLength = offset + 1;\n\t}\n\n\t/**\n\t * Fuegt {@code offset} an der passenden sortierten Position in\n\t * {@code nlIndex} ein und verschiebt alle nachfolgenden Eintraege um eine\n\t * Position nach hinten. Wird von {@link #addNewlineOffset(int)} genutzt,\n\t * wenn der gemeldete Offset nicht groesser als der bisher letzte Eintrag\n\t * ist. Die Kapazitaet von {@code nlIndex} muss vom Aufrufer bereits\n\t * sichergestellt sein.\n\t *\n\t * @param offset absoluter Offset des '\\n'-Zeichens im Puffer\n\t */\n\tprivate void insertShifted(int offset) {\n\t\tint idx = lowerBound(offset);\n\t\tint tailLen = count - idx;\n\t\tif (tailLen > 0)\n\t\t\tSystem.arraycopy(nlIndex, idx, nlIndex, idx + 1, tailLen);\n\t\tnlIndex[idx] = offset;\n\t\tcount++;\n\t}"
  reason: Fall 1 (nicht aufsteigender Offset) mit Shift-Logik in separater privater Methode behandeln
```
/answer 206f57db-9cb8-4ea8-9e9a-fa1742ef90cb allow
ReasoningToken: 798
Erledigt. Änderungen im Überblick:

**`addNewlineOffset(int offset)`** wirft jetzt keine Exception mehr für Fall 1 (Offset ≤ letzter Eintrag), sondern behandelt ihn:
- Kapazität wird vorab per `ensureCapacity(count + 1)` sichergestellt (gilt für beide Fälle).
- Ist `offset` größer als der letzte Eintrag → einfacher Append ans Ende (bisheriges Verhalten).
- Ist `offset` ≤ letzter Eintrag → Delegation an die neue private Methode `insertShifted(offset)`.
- `bufferLength` wird wie zuvor nachgezogen.

**Neue private Methode `insertShifted(int offset)`**:
- Ermittelt via `lowerBound(offset)` die korrekte sortierte Einfügeposition `idx`.
- Verschiebt mit `System.arraycopy(nlIndex, idx, nlIndex, idx + 1, tailLen)` alle nachfolgenden Einträge um eine Position nach hinten (Shift), um Platz zu schaffen.
- Schreibt `offset` an Position `idx` und erhöht `count`.
- Setzt voraus, dass die Kapazität bereits durch den Aufrufer sichergestellt wurde (Dokumentiert im Javadoc).

Die JavaDocs von `addNewlineOffset` wurden entsprechend erweitert, um den Shift-Fall zu beschreiben und auf `insertShifted` zu verweisen.
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java` addOffset() muss prüfen, ob exakt der Offset nicht bereits existiert.
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=b01b9432-4312-4c63-9fd4-48173103839c | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: e16d4dc4-db68-48ce-b83a-b47f2af8ea46
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java
  reason: Datei lesen um addOffset() zu finden
```
/answer e16d4dc4-db68-48ce-b83a-b47f2af8ea46 allow
ReasoningToken: 16
ReasoningToken: 574
Control Request:
```yaml
id: 22e7ba36-bc9f-474a-9d6c-03a5c89439bd
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/tools/LineIndex.java
  old_text: "\tpublic void addOffset(int offset) {\n\t\t// TODO check if set\n\t\tensureCapacity(count + 1);\n\t\tif (count > 0 && offset <= nlIndex[count - 1])\n\t\t\tinsertShifted(offset);\n\t\telse\n\t\t\tnlIndex[count++] = offset;\n\t\tif (offset >= bufferLength)\n\t\t\tbufferLength = offset + 1;\n\t}\n\n\tprivate void insertShifted(int offset) {\n\t\tint idx = lowerBound(offset);\n\t\tint tailLen = count - idx;\n\t\tif (tailLen > 0)\n\t\t\tSystem.arraycopy(nlIndex, idx, nlIndex, idx + 1, tailLen);\n\t\tnlIndex[idx] = offset;\n\t\tcount++;\n\t}"
  new_text: "\tpublic void addOffset(int offset) {\n\t\tif (count > 0 && offset <= nlIndex[count - 1]) {\n\t\t\tint idx = lowerBound(offset);\n\t\t\tif (idx < count && nlIndex[idx] == offset)\n\t\t\t\treturn; // offset already present, nothing to do\n\t\t\tensureCapacity(count + 1);\n\t\t\tinsertShifted(idx, offset);\n\t\t} else {\n\t\t\tensureCapacity(count + 1);\n\t\t\tnlIndex[count++] = offset;\n\t\t}\n\t\tif (offset >= bufferLength)\n\t\t\tbufferLength = offset + 1;\n\t}\n\n\tprivate void insertShifted(int idx, int offset) {\n\t\tint tailLen = count - idx;\n\t\tif (tailLen > 0)\n\t\t\tSystem.arraycopy(nlIndex, idx, nlIndex, idx + 1, tailLen);\n\t\tnlIndex[idx] = offset;\n\t\tcount++;\n\t}"
  reason: "addOffset soll prüfen, ob der Offset bereits existiert, bevor eingefügt wird"
```
/answer 22e7ba36-bc9f-474a-9d6c-03a5c89439bd allow
Erledigt. `addOffset()` prüft jetzt vorab per `lowerBound`, ob der exakte Offset bereits im Index enthalten ist (`nlIndex[idx] == offset`) – ist das der Fall, wird sofort zurückgekehrt, ohne einen Duplikat-Eintrag einzufügen oder `count`/`bufferLength` unnötig zu verändern. Der bisherige TODO-Kommentar ist entfernt, und `insertShifted` nimmt jetzt den bereits berechneten Index als Parameter entgegen, um die Suche nicht doppelt auszuführen.
