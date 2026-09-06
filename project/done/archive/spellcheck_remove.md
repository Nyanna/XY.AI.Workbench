Wenn sich Regionen ändern und verschieben dann werden Annotationen von `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java` nicht aus den Bereichen gelöscht für die Der Spellcheck nicht mehr gilt. 
SystemInit: cwd=/home/user/xyan/xy.ai.workbench | session_id=716600b4-fc15-4abf-8326-90c78bc14cff | model=claude-sonnet-5 | plugins=
Control Request:
```yaml
id: 7cef29a0-dedf-44dd-b4b1-c96beddcb994
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  reason: Review spellcheck reconciler for annotation region handling
```
/answer 7cef29a0-dedf-44dd-b4b1-c96beddcb994 allow
ReasoningToken: 18
Control Request:
```yaml
id: e6119ece-78bd-4035-b258-09bf5dd06b50
toolName: list
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck
  reason: List spellcheck package files
```
/answer e6119ece-78bd-4035-b258-09bf5dd06b50 allow
Control Request:
```yaml
id: 1f160d95-aba4-4b32-8334-23696ae2be60
result:
  structuredContent:
    entries:
    - LanguageToolClient.java
    - SpellCheckInstaller.java
    - SpellCheckReconciler.java
    - SpellingAnnotation.java
    - SpellingProblem.java
    - SpellingQuickAssistProcessor.java
    - SpellingStrategy.java
```
/answer 1f160d95-aba4-4b32-8334-23696ae2be60 allow
ReasoningToken: 24
Control Request:
```yaml
id: 4cdbfd8a-32ca-4735-95dc-f58a4b036126
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellingStrategy.java
  reason: Understand how annotations are added/removed on reconcile
```
/answer 4cdbfd8a-32ca-4735-95dc-f58a4b036126 allow
Text:
 Ich verstehe das Problem: Wenn ein AST-Knoten `enableSpellcheck=false` hat (weil sich Regionen verschieben/ändern), wird `mergeEnabledLeaves` einfach `false` zurückgegeben und **nichts** in `fDirtyRegions` aufgenommen. Dadurch wird `reconcile()` für diesen Bereich nie aufgerufen und `applyAnnotations()` (das alte Annotationen im Bereich entfernt) läuft nie – alte Spellcheck-Annotationen bleiben stehen, obwohl der Bereich nicht mehr gecheckt werden soll.

Ich behebe das, indem deaktivierte Bereiche separat als "nur löschen"-Regionen gesammelt und behandelt werden.
Control Request:
```yaml
id: 7e8ca366-51ed-46f0-b3b7-a9b1dfa4d550
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellingStrategy.java
  old_text: "\t@Override\n\tpublic void reconcile(DirtyRegion dirtyRegion, IRegion subRegion) {\n\t\treconcile(subRegion);\n\t}"
  new_text: "\t@Override\n\tpublic void reconcile(DirtyRegion dirtyRegion, IRegion subRegion) {\n\t\treconcile(subRegion);\n\t}\n\n\t/**\n\t * Removes any existing spelling annotations in {@code region} without\n\t * running a new LanguageTool check. Used for regions that no longer have\n\t * spellchecking enabled (e.g. because the underlying AST node moved or was\n\t * toggled off), so stale annotations don't linger.\n\t */\n\tpublic void clear(IRegion region) {\n\t\tif (fDocument == null) {\n\t\t\treturn;\n\t\t}\n\t\tint docLength = fDocument.getLength();\n\t\tint start = Math.max(0, Math.min(region.getOffset(), docLength));\n\t\tint end = Math.max(start, Math.min(start + region.getLength(), docLength));\n\t\tfinal IRegion clearedRegion = new Region(start, end - start);\n\t\tfViewer.getTextWidget().getDisplay()\n\t\t\t\t.asyncExec(() -> applyAnnotations(new ArrayList<>(), clearedRegion));\n\t}"
  reason: Add clear() method to remove stale annotations without running LT check
```
/answer 7e8ca366-51ed-46f0-b3b7-a9b1dfa4d550 allow
ReasoningToken: 1794
Text:
 Jetzt der Reconciler: separate "Clear"-Regionen für deaktivierte Bereiche einführen.
Control Request:
```yaml
id: 99e3626a-a4f2-4ef6-8ebd-5bcb589d9a5b
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  old_text: "\t// Pending dirty regions – overlapping/touching regions are merged as they\n\t// come in; disjoint regions (e.g. text before/after a non-spellchecked code\n\t// block) are kept separate so the code block in between is never checked.\n\t// Guarded by 'this'.\n\tprivate final List<int[]> fDirtyRegions = new ArrayList<>();"
  new_text: "\t// Pending dirty regions – overlapping/touching regions are merged as they\n\t// come in; disjoint regions (e.g. text before/after a non-spellchecked code\n\t// block) are kept separate so the code block in between is never checked.\n\t// Guarded by 'this'.\n\tprivate final List<int[]> fDirtyRegions = new ArrayList<>();\n\n\t// Pending regions whose spellcheck was disabled (or that moved away from a\n\t// checkable node). These must have their (possibly stale) annotations\n\t// removed, but must NOT be sent to LanguageTool. Guarded by 'this'.\n\tprivate final List<int[]> fClearRegions = new ArrayList<>();"
  reason: Track disabled/moved-away regions separately so their stale annotations get cleared
```
/answer 99e3626a-a4f2-4ef6-8ebd-5bcb589d9a5b allow
Control Request:
```yaml
id: d09b0566-52e6-4c48-9333-6a2e4ab8d128
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  old_text: "\tprivate boolean mergeEnabledLeaves(Node node) {\n\t\tif (node == null)\n\t\t\treturn false;\n\t\tif (node.children.isEmpty()) {\n\t\t\tif (!node.enableSpellcheck)\n\t\t\t\treturn false;\n\t\t\tint start = node.getOffset();\n\t\t\tint end = node.getEndOffset();\n\t\t\tmergeDirty(start, Math.max(end, start + 1));\n\t\t\treturn true;\n\t\t}\n\t\tboolean merged = false;\n\t\tfor (Node child : node.children)\n\t\t\tmerged |= mergeEnabledLeaves(child);\n\t\treturn merged;\n\t}\n\n\tprivate synchronized void mergeDirty(int start, int end) {\n\t\tint newStart = start;\n\t\tint newEnd = end;\n\t\tfor (Iterator<int[]> it = fDirtyRegions.iterator(); it.hasNext();) {\n\t\t\tint[] r = it.next();\n\t\t\t// Overlapping or directly adjacent -> merge.\n\t\t\tif (newStart <= r[1] && r[0] <= newEnd) {\n\t\t\t\tnewStart = Math.min(newStart, r[0]);\n\t\t\t\tnewEnd = Math.max(newEnd, r[1]);\n\t\t\t\tit.remove();\n\t\t\t}\n\t\t}\n\t\tfDirtyRegions.add(new int[] { newStart, newEnd });\n\t}\n\n\tprivate synchronized List<IRegion> takeDirty() {\n\t\tif (fDirtyRegions.isEmpty()) {\n\t\t\treturn null;\n\t\t}\n\t\tList<IRegion> regions = new ArrayList<>(fDirtyRegions.size());\n\t\tfor (int[] r : fDirtyRegions) {\n\t\t\tregions.add(new Region(r[0], r[1] - r[0]));\n\t\t}\n\t\tfDirtyRegions.clear();\n\t\treturn regions;\n\t}"
  new_text: "\tprivate boolean mergeEnabledLeaves(Node node) {\n\t\tif (node == null)\n\t\t\treturn false;\n\t\tif (node.children.isEmpty()) {\n\t\t\tint start = node.getOffset();\n\t\t\tint end = node.getEndOffset();\n\t\t\tif (!node.enableSpellcheck) {\n\t\t\t\t// Spellcheck no longer applies here (region was disabled or moved\n\t\t\t\t// away from a checkable node) – make sure any stale annotations in\n\t\t\t\t// this range get removed instead of silently ignoring it.\n\t\t\t\tmergeClear(start, Math.max(end, start + 1));\n\t\t\t\treturn true;\n\t\t\t}\n\t\t\tmergeDirty(start, Math.max(end, start + 1));\n\t\t\treturn true;\n\t\t}\n\t\tboolean merged = false;\n\t\tfor (Node child : node.children)\n\t\t\tmerged |= mergeEnabledLeaves(child);\n\t\treturn merged;\n\t}\n\n\tprivate synchronized void mergeDirty(int start, int end) {\n\t\tmerge(fDirtyRegions, start, end);\n\t}\n\n\tprivate synchronized void mergeClear(int start, int end) {\n\t\tmerge(fClearRegions, start, end);\n\t}\n\n\tprivate static void merge(List<int[]> regions, int start, int end) {\n\t\tint newStart = start;\n\t\tint newEnd = end;\n\t\tfor (Iterator<int[]> it = regions.iterator(); it.hasNext();) {\n\t\t\tint[] r = it.next();\n\t\t\t// Overlapping or directly adjacent -> merge.\n\t\t\tif (newStart <= r[1] && r[0] <= newEnd) {\n\t\t\t\tnewStart = Math.min(newStart, r[0]);\n\t\t\t\tnewEnd = Math.max(newEnd, r[1]);\n\t\t\t\tit.remove();\n\t\t\t}\n\t\t}\n\t\tregions.add(new int[] { newStart, newEnd });\n\t}\n\n\tprivate synchronized List<IRegion> takeDirty() {\n\t\treturn take(fDirtyRegions);\n\t}\n\n\tprivate synchronized List<IRegion> takeClear() {\n\t\treturn take(fClearRegions);\n\t}\n\n\tprivate static List<IRegion> take(List<int[]> regions) {\n\t\tif (regions.isEmpty()) {\n\t\t\treturn null;\n\t\t}\n\t\tList<IRegion> result = new ArrayList<>(regions.size());\n\t\tfor (int[] r : regions) {\n\t\t\tresult.add(new Region(r[0], r[1] - r[0]));\n\t\t}\n\t\tregions.clear();\n\t\treturn result;\n\t}"
  reason: Split merge/take logic to support separate clear-only regions
```
/answer d09b0566-52e6-4c48-9333-6a2e4ab8d128 allow
Control Request:
```yaml
id: 07e91119-bb54-4a50-a6c8-1234353cc805
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  old_text: "\tprivate void scheduleReconcile() {\n\t\tcancelPending();\n\t\tfPending = fScheduler.schedule(() -> {\n\t\t\tList<IRegion> dirty = takeDirty();\n\t\t\tif (dirty != null)\n\t\t\t\tfor (IRegion region : dirty)\n\t\t\t\t\tfStrategy.reconcile(region);\n\t\t}, fDelayMs, TimeUnit.MILLISECONDS);\n\t}"
  new_text: "\tprivate void scheduleReconcile() {\n\t\tcancelPending();\n\t\tfPending = fScheduler.schedule(() -> {\n\t\t\tList<IRegion> clear = takeClear();\n\t\t\tif (clear != null)\n\t\t\t\tfor (IRegion region : clear)\n\t\t\t\t\tfStrategy.clear(region);\n\n\t\t\tList<IRegion> dirty = takeDirty();\n\t\t\tif (dirty != null)\n\t\t\t\tfor (IRegion region : dirty)\n\t\t\t\t\tfStrategy.reconcile(region);\n\t\t}, fDelayMs, TimeUnit.MILLISECONDS);\n\t}"
  reason: Process clear regions (remove stale annotations) alongside dirty regions on each reconcile cycle
```
/answer 07e91119-bb54-4a50-a6c8-1234353cc805 allow
Control Request:
```yaml
id: 30b79117-47c3-4e4e-b879-5a6ec8b28597
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editors/spellcheck/SpellCheckReconciler.java
  reason: Final review of changes
```
/answer 30b79117-47c3-4e4e-b879-5a6ec8b28597 allow
Control Request:
```yaml
id: 536a6964-1a90-4138-a48b-14e709d8dece
toolName: bash
arguments:
  cwd: /home/user/xyan/xy.ai.workbench
  script: find . -maxdepth 2 -iname "pom.xml" -o -iname "build.gradle" | head -20
  reason: Check build tooling to compile-check changes
```
/answer 536a6964-1a90-4138-a48b-14e709d8dece deny Es gibt weder build tooling noch Tests.
ReasoningToken: 44
Text:
 Jetzt `scheduleReconcile()` anpassen, damit auch die Clear-Regionen verarbeitet werden:
ReasoningToken: 109
Text:
 Sieht gut aus. Jetzt versuche ich, das Projekt zu bauen, um sicherzustellen, dass es kompiliert.
Kein Build-Tooling verfügbar (Eclipse-Plugin-Projekt), daher keine automatisierte Kompilierprüfung möglich. Ich habe die Änderungen manuell auf Konsistenz geprüft (alle nötigen Imports waren bereits vorhanden).

## Zusammenfassung der Änderung

**Problem:** In `SpellCheckReconciler.mergeEnabledLeaves(...)` wurde bei einem Blattknoten mit `enableSpellcheck == false` einfach `false` zurückgegeben, ohne die betroffene Region irgendwo zu vermerken. Wenn sich Regionen durch AST-Änderungen verschoben oder Spellcheck für einen Bereich deaktiviert wurde, wurde dieser Bereich also nie an `SpellingStrategy.reconcile(...)` übergeben – und damit auch nie die dort in `applyAnnotations(...)` enthaltene Logik zum Entfernen alter Annotationen ausgeführt. Bereits gesetzte Spellcheck-Annotationen blieben also als „Karteileichen“ an falscher/veralteter Stelle stehen.

**Fix:**
1. **`SpellingStrategy`**: neue Methode `clear(IRegion)` hinzugefügt, die für eine Region ausschließlich vorhandene Spelling-Annotationen entfernt, ohne LanguageTool erneut aufzurufen.
2. **`SpellCheckReconciler`**: 
   - Neue Liste `fClearRegions` (parallel zu `fDirtyRegions`) für Bereiche, deren Spellcheck deaktiviert wurde oder die sich verschoben haben.
   - `mergeEnabledLeaves(...)` merged deaktivierte Blattknoten jetzt in `fClearRegions` statt sie zu ignorieren.
   - `merge`/`take`-Logik wurde generalisiert (gemeinsame Hilfsmethoden für beide Listen).
   - `scheduleReconcile()` verarbeitet vor dem eigentlichen Reconcile zuerst alle `clear`-Regionen, damit veraltete Annotationen zuverlässig entfernt werden.
