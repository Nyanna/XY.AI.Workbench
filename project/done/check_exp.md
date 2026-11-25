Prüfe die Exception, sie sollte mindestens gefangen werden wenn sie nicht vermeidbar ist.

Exception in thread "EditorManager-Background-8" java.util.ConcurrentModificationException
        at java.base/java.util.ArrayList$Itr.checkForComodification(ArrayList.java:1096)
        at java.base/java.util.ArrayList$Itr.next(ArrayList.java:1050)
        at xy.ai.workbench.editor.spellcheck.SpellCheckReconciler.reconcileLeaves(SpellCheckReconciler.java:44)
        at xy.ai.workbench.editor.spellcheck.SpellCheckReconciler.reconcile(SpellCheckReconciler.java:27)
        at xy.ai.workbench.editor.EditorManager.lambda$0(EditorManager.java:170)
        at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1090)
        at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:614)
        at java.base/java.lang.Thread.run(Thread.java:1474)
SystemInit:  id=e3c904b3-8ff4-4c06-ab23-06d588a082f8, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 906f-22-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench
    pattern: (SpellCheckReconciler|EditorManager)\.java$
  reason: Locate the files from the stack trace
```
/answer 906f-22-0 allow
Control Request:
```yaml
id: 906f-23-22
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench
      entries:
      - "./src/xy/ai/workbench/editor:"
      - EditorManager.java
      - "./src/xy/ai/workbench/editor/spellcheck:"
      - SpellCheckReconciler.java
```
/answer 906f-23-22 allow
Control Request:
```yaml
id: 906f-25-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: reconcile|Executor|Background
    include:
    - EditorManager.java
    intent: retrieve_context
  reason: Read the reconciler structure and the call site in EditorManager
```
/answer 906f-25-0 allow
Control Request:
```yaml
id: 906f-26-25
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
        matches:
        - path: EditorManager.java
          nodes:
          - id: IxOJYs|69ogGr
            matches:
            - import java.util.concurrent.ThreadPoolExecutor;
          - id: EditorManager.QoDsj9|rEkLvR
            matches:
            - private final ThreadPoolExecutor background = createBackgroundPool();
          - id: EditorManager.createBackgroundPool
            matches:
            - "private static ThreadPoolExecutor createBackgroundPool() {"
            - "Thread t = new Thread(r, \"EditorManager-Background-\" + counter.incrementAndGet());"
            - "ThreadPoolExecutor executor = new ThreadPoolExecutor(1, maxThreads, IDLE_KEEP_ALIVE_MS, TimeUnit.MILLISECONDS,"
            - "new SynchronousQueue<>(), factory, new ThreadPoolExecutor.CallerRunsPolicy());"
          - id: EditorManager.update
            matches:
            - spell.reconcile(range);
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
        nodes:
        - id: cHfzxz|x6fLF6
          excert: import org.eclipse.jface.text.IDocument;
        - id: SpellCheckReconciler
          excert: "public class SpellCheckReconciler implements ISpellChecker {"
          children:
          - id: SpellCheckReconciler.RR20Bo|QfUT4K
            excert: private final SpellingStrategy strategy;
          - id: SpellCheckReconciler.SpellCheckReconciler
            excert: "public SpellCheckReconciler(ISourceViewer sourceViewer, EditorManager manager)"
          - id: SpellCheckReconciler.onDocumentChanged
            excert: public void onDocumentChanged(IDocument document)
          - id: SpellCheckReconciler.reconcile
            excert: public void reconcile(ModificationRange range)
          - id: SpellCheckReconciler.reconcileLeaves
            excert: "private void reconcileLeaves(Node node, int rangeStart, int rangeEnd)"
```
/answer 906f-26-25 allow
Control Request:
```yaml
id: 906f-27-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
    ids:
    - SpellCheckReconciler.reconcile
    - SpellCheckReconciler.reconcileLeaves
    - SpellCheckReconciler.onDocumentChanged
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
    ids:
    - EditorManager.update
    - EditorManager.createBackgroundPool
  reason: Read the relevant methods in full
```
/answer 906f-27-0 allow
Control Request:
```yaml
id: 906f-28-27
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
        nodes:
        - id: SpellCheckReconciler.reconcile
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void reconcile(ModificationRange range) {\n\t\treconcileLeaves(range.getNode(), range.getStart(), range.getEnd());\n\t}\n"
        - id: SpellCheckReconciler.reconcileLeaves
          type: MethodDeclaration
          code: "\tprivate void reconcileLeaves(Node node, int rangeStart, int rangeEnd) {\n\t\tint nodeStart = node.getOffset();\n\t\tint nodeEnd = node.getEndOffset();\n\t\tif (nodeEnd <= rangeStart || nodeStart >= rangeEnd)\n\t\t\treturn;\n\n\t\tint start = Math.max(nodeStart, rangeStart);\n\t\tint end = Math.min(nodeEnd, rangeEnd);\n\n\t\tif (!node.enableSpellcheck)\n\t\t\tstrategy.clear(node, start, end);\n\t\telse if (node.children.isEmpty())\n\t\t\tstrategy.reconcile(node, start, end);\n\t\telse\n\t\t\tfor (Node child : node.children)\n\t\t\t\treconcileLeaves(child, rangeStart, rangeEnd);\n\t}\n"
        - id: SpellCheckReconciler.onDocumentChanged
          type: MethodDeclaration
          code: "\t@Override\n\tpublic void onDocumentChanged(IDocument document) {\n\t\tstrategy.setDocument(document);\n\t}\n"
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
        nodes:
        - id: EditorManager.update
          type: MethodDeclaration
          code: "\tprivate void update(ModificationRange range) {\n\t\tNode node = range.getNode();\n\t\tfor (IManagerListener l : listeners)\n\t\t\tl.onAstUpdated(node);\n\t\tif (viewer instanceof ITextViewerExtension2 ext2)\n\t\t\ttry {\n\t\t\t\text2.invalidateTextPresentation(node.getOffset(), node.length());\n\t\t\t} catch (IllegalArgumentException e) {\n\t\t\t\t// region outside the (possibly just replaced) doc - ignore.\n\t\t\t}\n\n\t\tif (spell != null && !background.isShutdown())\n\t\t\tbackground.execute(() -> {\n\t\t\t\tspell.reconcile(range);\n\t\t\t\tviewer.getTextWidget().getDisplay().asyncExec(() -> {\n\t\t\t\t\tif (viewer instanceof ITextViewerExtension2 ext2)\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\text2.invalidateTextPresentation(node.getOffset(), node.length());\n\t\t\t\t\t\t} catch (IllegalArgumentException e) {\n\t\t\t\t\t\t\t// region outside the (possibly just replaced) doc - ignore.\n\t\t\t\t\t\t}\n\t\t\t\t});\n\t\t\t});\n\t}\n"
        - id: EditorManager.createBackgroundPool
          type: MethodDeclaration
          code: "\tprivate static ThreadPoolExecutor createBackgroundPool() {\n\t\tint maxThreads = Math.max(1, Math.min(16, Runtime.getRuntime().availableProcessors()));\n\t\tThreadFactory factory = new ThreadFactory() {\n\t\t\tprivate final AtomicInteger counter = new AtomicInteger();\n\n\t\t\t@Override\n\t\t\tpublic Thread newThread(Runnable r) {\n\t\t\t\tThread t = new Thread(r, \"EditorManager-Background-\" + counter.incrementAndGet());\n\t\t\t\tt.setDaemon(true);\n\t\t\t\treturn t;\n\t\t\t}\n\t\t};\n\t\tThreadPoolExecutor executor = new ThreadPoolExecutor(1, maxThreads, IDLE_KEEP_ALIVE_MS, TimeUnit.MILLISECONDS,\n\t\t\t\tnew SynchronousQueue<>(), factory, new ThreadPoolExecutor.CallerRunsPolicy());\n\t\texecutor.allowCoreThreadTimeOut(false);\n\t\treturn executor;\n\t}\n"
```
/answer 906f-28-27 allow
ReasoningToken: 50
Control Request:
```yaml
id: 906f-29-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: children\s*(=|;)|children\.(add|remove|set|clear|addAll|removeIf|sort)|List<Node> children
    include:
    - '*.java'
    limit: 40
    intent: survey
  reason: Find where Node.children is declared and mutated
```
/answer 906f-29-0 allow
Control Request:
```yaml
id: 906f-2a-29
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src
        matches:
        - path: xy/ai/workbench/view/diff/OpSnapshotter.java
          matches:
          - "String[] children = dir.list();"
          - if (children == null || children.length > 0)
        - path: xy/ai/workbench/editor/mdast/MarkdownDocument.java
          matches:
          - List<Node> siblings = parent.children;
          - "host.children.add(reparent(c, host));"
          - List<Node> as = ap.children;
          - root.children.clear();
          - "root.children.add(reparent(c, root));"
        - path: xy/ai/workbench/editor/mdast/nodes/AbstractNode.java
          matches:
          - n.children.add(nn);
        - path: xy/ai/workbench/editor/mdast/nodes/Node.java
          matches:
          - public final List<Node> children = new ArrayList<>();
        - path: xy/ai/workbench/editor/outline/NodeElement.java
          matches:
          - "private NodeElement[] children;"
          - this.children = node.children.stream()
          - return children;
```
/answer 906f-2a-29 allow
Control Request:
```yaml
id: 906f-2d-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: siblings|reparent|root\.children|ModificationRange\(|update\(|synchronized|Lock
    include:
    - MarkdownDocument.java
    - EditorManager.java
    limit: 40
    intent: retrieve_context
  reason: Understand threading around AST mutation and update()
```
/answer 906f-2d-0 allow
Control Request:
```yaml
id: 906f-2e-2d
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
        matches:
        - path: mdast/MarkdownDocument.java
          nodes:
          - id: MarkdownDocument.update
            matches:
            - "public ModificationRange update(int offset, int removed, int inserted) {"
            - "lines.update(buffer, offset, removed, inserted);"
            - "// Fast path: only new siblings were appended at the tail that do not"
          - id: MarkdownDocument.appendSiblings
            matches:
            - List<Node> siblings = parent.children;
            - siblings.remove(siblings.size() - 1); // sec is the last child
            - "siblings.add(reparent(c, parent));"
            - "host.children.add(reparent(c, host));"
          - id: MarkdownDocument.isSpineTail
            matches:
            - List<Node> siblings = parent.children;
            - if (siblings.isEmpty() || siblings.get(siblings.size() - 1) != sec)
          - id: MarkdownDocument.isLastChild
            matches:
            - List<Node> siblings = parent.children;
            - return !siblings.isEmpty() && siblings.get(siblings.size() - 1) == node;
          - id: MarkdownDocument.replace
            matches:
            - root.children.clear();
            - "root.children.add(reparent(c, root));"
            - List<Node> siblings = parent.children;
            - int idx = siblings.indexOf(sec);
            - for (int i = idx + 1; i < siblings.size(); i++)
            - "shift(siblings.get(i), delta);"
            - siblings.remove(idx);
            - "siblings.add(at++, reparent(c, parent));"
          - id: MarkdownDocument.reparent
            matches:
            - "private Node reparent(Node src, Node newParent) {"
          - id: MarkdownDocument.expand
            matches:
            - "return new ModificationRange(node, start, end);"
        - path: EditorManager.java
          nodes:
          - id: EditorManager.setSpellChecker
            matches:
            - "update(new ModificationRange(ast.getRoot(), 0, doc.getLength()));"
          - id: EditorManager.update
            matches:
            - "private void update(ModificationRange range) {"
          - id: EditorManager.Flush.run
            matches:
            - "update(ast.update(r.offset(), r.value(), r.length()));"
          - id: EditorManager.TextInput.inputDocumentChanged
            matches:
            - "initial = ast.update(0, 0, buffer.length());"
            - update(initial);
```
/answer 906f-2e-2d allow
ReasoningToken: 60
Control Request:
```yaml
id: 906f-2f-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
    node_type: CompilationUnit|ClassDeclaration|MethodDeclaration
    name: SpellCheckReconciler
  reason: Read the whole reconciler class including constructor and imports
```
/answer 906f-2f-0 allow
Control Request:
```yaml
id: 906f-30-2f
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
        nodes: []
```
/answer 906f-30-2f allow
Thinking:
Die Ursache liegt nun bei der nebenläufigen Änderung von `Node.children`, das beim Editieren verändert wird, während der Spellcheck im Background-Thread darüber iteriert. Als Nächstes prüfe ich, in welchem Thread die Mutation läuft und was `strategy.reconcile` am Dokument macht.


Control Request:
```yaml
id: 906f-31-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
    ids:
    - EditorManager.Flush.run
    - EditorManager.setSpellChecker
  reason: Check which thread runs ast.update (the mutation) and how spell checker is set
```
/answer 906f-31-0 allow
Control Request:
```yaml
id: 906f-32-31
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
        nodes:
        - id: EditorManager.Flush.run
          type: MethodDeclaration
          code: "\t\t@Override\n\t\tpublic void run() {\n\t\t\tif (pending.isEmpty() || ast == null || buffer == null)\n\t\t\t\treturn;\n\n\t\t\tif (pending.millisSinceLastInsert() < DEBOUNCE_DELAY_MS) {\n\t\t\t\tscheduleFlush();\n\t\t\t\treturn;\n\t\t\t}\n\t\t\t// LOG.info(\"AST Flushed\");\n\n\t\t\tList<RegionList.Region<Integer>> regions = new ArrayList<>(pending.asList());\n\t\t\tpending.clear();\n\t\t\tregions.sort(Comparator.comparingInt(RegionList.Region::offset));\n\n\t\t\tfor (RegionList.Region<Integer> r : regions)\n\t\t\t\tupdate(ast.update(r.offset(), r.value(), r.length()));\n\t\t}\n"
        - id: EditorManager.setSpellChecker
          type: MethodDeclaration
          code: "\tpublic void setSpellChecker(ISpellChecker spellChecker) {\n\t\tthis.spell = spellChecker;\n\t\tif (spellChecker != null && doc != null && ast != null) {\n\t\t\tspellChecker.onDocumentChanged(doc);\n\t\t\tupdate(new ModificationRange(ast.getRoot(), 0, doc.getLength()));\n\t\t}\n\t}\n"
```
/answer 906f-32-31 allow
ReasoningToken: 33
Control Request:
```yaml
id: 906f-33-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
    pattern: Flush|asyncExec|timerExec|Display
    include:
    - EditorManager.java
    limit: 20
    intent: retrieve_context
  reason: Find how Flush is scheduled
```
/answer 906f-33-0 allow
Control Request:
```yaml
id: 906f-34-33
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor
        matches:
        - path: EditorManager.java
          nodes:
          - id: IxOJYs|69ogGr
            matches:
            - import org.eclipse.swt.widgets.Display;
          - id: EditorManager.fuKAs3|U5U9yd
            matches:
            - private Display display;
            - private final Runnable flush = new Flush();
          - id: EditorManager.install
            matches:
            - display = viewer.getTextWidget().getDisplay();
          - id: EditorManager.changed
            matches:
            - scheduleFlush();
          - id: EditorManager.update
            matches:
            - "viewer.getTextWidget().getDisplay().asyncExec(() -> {"
          - id: EditorManager.scheduleFlush
            matches:
            - "private void scheduleFlush() {"
            - "display.timerExec(DEBOUNCE_DELAY_MS, flush);"
          - id: EditorManager.cancelPending
            matches:
            - "display.timerExec(-1, flush);"
          - id: EditorManager.Flush
            matches:
            - "private class Flush implements Runnable {"
          - id: EditorManager.Flush.run
            matches:
            - scheduleFlush();
            - // LOG.info("AST Flushed");
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckReconciler.java
        nodes:
        - id: cHfzxz|x6fLF6
          excert: import org.eclipse.jface.text.IDocument;
        - id: SpellCheckReconciler
          excert: "public class SpellCheckReconciler implements ISpellChecker {"
          children:
          - id: SpellCheckReconciler.RR20Bo|QfUT4K
            excert: private final SpellingStrategy strategy;
          - id: SpellCheckReconciler.SpellCheckReconciler
            excert: "public SpellCheckReconciler(ISourceViewer sourceViewer, EditorManager manager)"
          - id: SpellCheckReconciler.onDocumentChanged
            excert: public void onDocumentChanged(IDocument document)
          - id: SpellCheckReconciler.reconcile
            excert: public void reconcile(ModificationRange range)
          - id: SpellCheckReconciler.reconcileLeaves
            excert: "private void reconcileLeaves(Node node, int rangeStart, int rangeEnd)"
```
/answer 906f-34-33 allow
Thinking:
Der Absturz stammt von einem Race: `Flush.run` verändert den AST im UI-Thread, während der Spellcheck im Pool-Thread dieselben Knoten liest. Ich fange die Exception an der Task-Grenze ab, weil jede Mutation ohnehin ein neues Update und damit einen frischen Lauf auslöst.


ReasoningToken: 577
Control Request:
```yaml
id: 906f-36-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
    id: EditorManager.update
    old_text: "\t\t\t\tspell.reconcile(range);\n"
    new_text: "\t\t\t\ttry {\n\t\t\t\t\tspell.reconcile(range);\n\t\t\t\t} catch (ConcurrentModificationException e) {\n\t\t\t\t\t// AST was mutated on the UI thread; the resulting update() schedules a fresh reconcile.\n\t\t\t\t\treturn;\n\t\t\t\t}\n"
    addImport:
    - import java.util.ConcurrentModificationException;
  reason: Catch the ConcurrentModificationException in the background spellcheck task
```
/answer 906f-36-0 allow
ReasoningToken: 32
Control Request:
```yaml
id: 906f-3a-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
    ids:
    - EditorManager.update
  reason: Verify the edit
```
/answer 906f-3a-0 allow
Control Request:
```yaml
id: 906f-3c-3a
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/EditorManager.java
        nodes:
        - id: EditorManager.update
          type: MethodDeclaration
          code: |2
              private void update(ModificationRange range) {
                Node node = range.getNode();
                for (IManagerListener l : listeners) l.onAstUpdated(node);
                if (viewer instanceof ITextViewerExtension2 ext2)
                  try {
                    ext2.invalidateTextPresentation(node.getOffset(), node.length());
                  } catch (IllegalArgumentException e) {
                    // region outside the (possibly just replaced) doc - ignore.
                  }
                if (spell != null && !background.isShutdown())
                  background.execute(() -> {
                    try {
                      spell.reconcile(range);
                    } catch (ConcurrentModificationException e) {
                      // AST was mutated on the UI thread; the resulting update() schedules a fresh reconcile.
                      return;
                    }
                    viewer.getTextWidget().getDisplay().asyncExec(() -> {
                      if (viewer instanceof ITextViewerExtension2 ext2)
                        try {
                          ext2.invalidateTextPresentation(node.getOffset(), node.length());
                        } catch (IllegalArgumentException e) {
                          // region outside the (possibly just replaced) doc - ignore.
                        }
                    });
                  });
              }
```
/answer 906f-3c-3a allow
Result Stats: id=e3c904b3-8ff4-4c06-ab23-06d588a082f8, total: 15426, in: 18, out: 4239, reason: 0, read: 95543, write: 11169, start: 1795643390004, end: 1795643397252, time: 7182, total: 58829, io: 0, control: 66, sent: 0, recv: 170
Ich habe die Exception im Background-Task von `EditorManager.update` abgefangen. Kompiliert und getestet habe ich die Änderung nicht.

**Ursache**
- `Node.children` ist eine einfache `ArrayList`.
- `Flush.run` verändert den AST im UI-Thread (`display.timerExec` → `ast.update` → `MarkdownDocument.replace` und `appendSiblings`, die `children` anpassen).
- Gleichzeitig iteriert `SpellCheckReconciler.reconcileLeaves` in einem `EditorManager-Background-N`-Thread per for-each über `node.children`. Das wirft die `ConcurrentModificationException`.
- Der Pool hat bis zu 16 Threads, es können also mehrere Reconciles parallel laufen.

**Änderung**
- Die Änderung steht in `EditorManager.update()`. `spell.reconcile(range)` ist jetzt in `try/catch (ConcurrentModificationException)` gefasst.
- Bei einem Konflikt endet der Task ohne weitere Aktion.
- Der Import `java.util.ConcurrentModificationException` ist ergänzt.
- Das Verwerfen ist unkritisch: Die Mutation, die den Konflikt auslöst, ruft selbst `update()` auf. Das startet einen neuen Reconcile für den geänderten Bereich.

**Warum kein Locking**
- Ein Snapshot (`new ArrayList<>(node.children)`) würde das Race nur verkleinern, nicht beseitigen.
- Auch die Offsets (`shift(...)`) werden während der Mutation verändert. Der Reconcile rechnet dann ohnehin mit veralteten Werten.
- Eine saubere Lösung wäre, den AST im UI-Thread zu lesen oder zu snapshotten. Alternativ könnte man die Mutation und den Reconcile mit einem gemeinsamen Lock schützen. Beides greift tiefer in die Architektur ein. Ich habe es nicht umgesetzt.

**Offen:** `ast.update` und der Reconcile können weiterhin Offsets lesen, die gerade verschoben werden. Das gibt keine Exception, kann aber kurzzeitig falsche Spellcheck-Markierungen liefern. Der nächste Reconcile korrigiert sie.
