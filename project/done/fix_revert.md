`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/GitSupport.java` hat leider einen Bug. Es wird ein Revert ausgeführt, auch wenn der letzte Toolcall gar keine schreibende Operation war.

Das kann behoben werden da die ID jetzt direkt Referenziert. Die dritte Position referenziert direkt die Tool Call id ("2a", "b725" ist die Session ID). Ich schlage vor beim Revert zu prüfen, ob ein Deny überhaupt einen revertbaren Snapshot referenziert.  

Beispiel:
```yaml
id: b725-2a-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/autoapprove
```
  
```yaml
id: b725-2b-2a
result:
  structuredContent:
```
SystemInit:  id=08a5642d-8ff4-44f8-87a8-5fe03c1b255e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: ed7c-56-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/GitSupport.java
  reason: Understand structure of GitSupport.java to locate revert logic
```
/answer ed7c-56-0 allow
Control Request:
```yaml
id: ed7c-57-56
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/GitSupport.java
      nodes:
      - id: 3nz0Vj|6zBson
        signature: import xy.ai.conscious.api.IExecutor;
      - id: GitSupport
        signature: "public class GitSupport implements IGitSupport {"
        children:
        - id: GitSupport.triggerSnapshotIfNeeded
          signature: "public void triggerSnapshotIfNeeded(IExecutor exec, IModelRequest req)"
        - id: GitSupport.extractYamlId
          signature: private String extractYamlId(String yaml)
          docstring: Extracts the top-level "id:" field from a captured tool-call/result YAML block.
```
/answer ed7c-57-56 allow
Control Request:
```yaml
id: ed7c-58-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/GitSupport.java
    ids:
    - GitSupport
  reason: Read the full class to analyze revert logic
```
/answer ed7c-58-0 allow
Control Request:
```yaml
id: ed7c-59-58
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/GitSupport.java
      nodes:
      - id: GitSupport
        type: ClassOrInterfaceDeclaration
        signature: "public class GitSupport implements IGitSupport {"
        children:
        - id: GitSupport.triggerSnapshotIfNeeded
          type: MethodDeclaration
          code: "\tpublic void triggerSnapshotIfNeeded(IExecutor exec, IModelRequest req) {\n\t\tPrompt prompt = req.getPrompt();\n\t\tCommand cmd = prompt.arg.command;\n\t\tString opId = extractYamlId(prompt.arg.yaml);\n\t\tboolean revertToolResultDeny = cmd instanceof AnswerCommand ac && ac.action() == AnswerCommand.Action.Deny\n\t\t\t\t&& ac.blockType() == AnswerCommand.BlockType.Result;\n\n\t\texec.schedule(\"Workbench-Snapshot\", mon -> {\n\t\t\tOpSnapshotter snap = new OpSnapshotter(prompt.arg.project.toFile());\n\t\t\tSnapshotResult result = revertToolResultDeny ? snap.revertLast() : snap.snapshot(opId);\n\t\t\tif (result != null && DiffPanel.INSTANCE != null)\n\t\t\t\texec.asyncExec(() -> DiffPanel.INSTANCE.onSnapshot(result, snap.getRepository()));\n\t\t});\n\t}\n"
        - id: GitSupport.extractYamlId
          type: MethodDeclaration
          code: "\tprivate String extractYamlId(String yaml) {\n\t\tif (yaml == null)\n\t\t\treturn null;\n\t\tvar m = java.util.regex.Pattern.compile(\"(?m)^id:\\\\s*[\\\"']?([^\\\"'\\\\r\\\\n]+?)[\\\"']?\\\\s*$\").matcher(yaml);\n\t\treturn m.find() ? m.group(1).strip() : null;\n\t}\n"
```
/answer ed7c-59-58 allow
ReasoningToken: 27
Control Request:
```yaml
id: ed7c-5a-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: class OpSnapshotter|revertLast|SnapshotResult\(
    include:
    - '*.java'
    exclude:
    - bin/**
    - target/**
  reason: Locate OpSnapshotter to see how snapshots are stored and identified
```
/answer ed7c-5a-0 allow
Control Request:
```yaml
id: ed7c-5b-5a
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/diff/SnapshotResult.java
        matches:
        - "public record SnapshotResult(boolean created, String chain, String ref, ObjectId commit, ObjectId parent,"
      - path: xy/ai/workbench/view/diff/OpSnapshotter.java
        matches:
        - "public class OpSnapshotter {"
        - "return new SnapshotResult(false, namespace, null, parent, parent, triggerLabel);"
        - "return new SnapshotResult(true, namespace, refName, newCommit, parent, triggerLabel);"
        - "return new SnapshotResult(true, chain, best.getName(), best.getObjectId(), parent, null);"
        - "public synchronized SnapshotResult revertLast() throws IOException {"
        - "return new SnapshotResult(false, namespace, null, storedBase, storedBase, null);"
        - "return new SnapshotResult(true, namespace, newRefName, newHead, newHeadParent, null);"
      - path: xy/ai/workbench/eclipse/GitSupport.java
        matches:
        - "SnapshotResult result = revertToolResultDeny ? snap.revertLast() : snap.snapshot(opId);"
```
/answer ed7c-5b-5a allow
Control Request:
```yaml
id: ed7c-5c-0
toolName: ast_outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
  reason: Outline OpSnapshotter
```
/answer ed7c-5c-0 allow
Control Request:
```yaml
id: ed7c-5d-5c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
      nodes:
      - id: pguH5b|dZ9ifv
        signature: import java.io.File;
      - id: FkNxGe|vyOJXB
        signature: import org.eclipse.jgit.lib.ObjectInserter;
      - id: OpSnapshotter
        signature: "public class OpSnapshotter {"
        children:
        - id: OpSnapshotter.uOcuA9|XCFEJm
          signature: private static final String BASE_MARKER = "refs/llm-ops/_base";
        - id: OpSnapshotter.OpSnapshotter
          signature: public OpSnapshotter(Repository repository)
        - id: OpSnapshotter.OpSnapshotter_2
          signature: public OpSnapshotter(File start) throws IOException
        - id: OpSnapshotter.OpSnapshotter_3
          signature: public OpSnapshotter() throws IOException
        - id: OpSnapshotter.openWorkspaceRepository
          signature: private static Repository openWorkspaceRepository() throws IOException
        - id: OpSnapshotter.loadRepository
          signature: public static Repository loadRepository(File start) throws IOException
        - id: OpSnapshotter.getRepository
          signature: public Repository getRepository()
        - id: OpSnapshotter.snapshot
          signature: public synchronized SnapshotResult snapshot(String triggerLabel) throws IOExcep…
        - id: OpSnapshotter.buildTreeFromDelta
          signature: "private ObjectId buildTreeFromDelta(ObjectId parentCommit, ObjectId parentTree)…"
        - id: OpSnapshotter.isDiffIgnored
          signature: "private boolean isDiffIgnored(String path, IgnoreNode node)"
        - id: OpSnapshotter.collectTreeEntries
          signature: "private Map<String, TreeEntry> collectTreeEntries(ObjectId treeId) throws IOExc…"
        - id: OpSnapshotter.TreeEntry
          signature: "private static class TreeEntry {"
        - id: OpSnapshotter.loadDiffIgnore
          signature: private IgnoreNode loadDiffIgnore() throws IOException
        - id: OpSnapshotter.findLatest
          signature: public SnapshotResult findLatest() throws IOException
        - id: OpSnapshotter.revertLast
          signature: public synchronized SnapshotResult revertLast() throws IOException
          docstring: "Reverts the last snapshot commit of the current chain: restores the working"
        - id: OpSnapshotter.checkoutDelta
          signature: "private void checkoutDelta(ObjectId fromTree, ObjectId toTree) throws IOExcepti…"
          docstring: Restores the working tree paths that differ between the two trees to their 'toT…
        - id: OpSnapshotter.deleteEmptyParents
          signature: "private void deleteEmptyParents(File root, File dir)"
        - id: OpSnapshotter.deleteRef
          signature: private void deleteRef(String name) throws IOException
        - id: OpSnapshotter.resolveRef
          signature: private ObjectId resolveRef(String name) throws IOException
        - id: OpSnapshotter.updateRef
          signature: "private void updateRef(String name, ObjectId id) throws IOException"
```
/answer ed7c-5d-5c allow
Control Request:
```yaml
id: ed7c-5e-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: triggerSnapshotIfNeeded|IGitSupport
    include:
    - '*.java'
    exclude:
    - bin/**
  reason: Find callers/interface of the git support
```
/answer ed7c-5e-0 allow
Control Request:
```yaml
id: ed7c-5f-5e
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/eclipse/GitSupport.java
        matches:
        - import xy.ai.conscious.api.ext.IGitSupport;
        - "public class GitSupport implements IGitSupport{"
        - "public void triggerSnapshotIfNeeded(IExecutor exec, IModelRequest req) {"
```
/answer ed7c-5f-5e allow
Control Request:
```yaml
id: ed7c-60-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    ids:
    - OpSnapshotter.snapshot
    - OpSnapshotter.findLatest
    - OpSnapshotter.revertLast
    - OpSnapshotter.uOcuA9|XCFEJm
  reason: Understand snapshot labels/ref naming and revert logic
```
/answer ed7c-60-0 allow
Control Request:
```yaml
id: ed7c-61-60
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
      nodes:
      - id: OpSnapshotter.snapshot
        type: MethodDeclaration
        code: "\tpublic synchronized SnapshotResult snapshot(String triggerLabel) throws IOException {\n\t\tObjectId currentHead = repository.resolve(\"HEAD\");\n\t\tif (currentHead == null)\n\t\t\tthrow new IOException(\"No HEAD found\");\n\n\t\tObjectId storedBase = resolveRef(BASE_MARKER);\n\t\tif (storedBase == null || !storedBase.equals(currentHead)) {\n\t\t\tupdateRef(BASE_MARKER, currentHead);\n\t\t\tstoredBase = currentHead;\n\t\t}\n\n\t\tString chainId = repository.newObjectReader().abbreviate(storedBase, 7).name();\n\t\tString namespace = \"refs/llm-ops/chain-\" + chainId;\n\n\t\tint lastN = 0;\n\t\tString lastOpRefName = null;\n\t\tfor (Ref ref : repository.getRefDatabase().getRefsByPrefix(namespace + \"/op-\")) {\n\t\t\tMatcher m = OP_SUFFIX.matcher(ref.getName());\n\t\t\tif (m.matches()) {\n\t\t\t\tint n = Integer.parseInt(m.group(1));\n\t\t\t\tif (n > lastN) {\n\t\t\t\t\tlastN = n;\n\t\t\t\t\tlastOpRefName = ref.getName();\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tObjectId parent = lastOpRefName != null ? resolveRef(lastOpRefName) : storedBase;\n\n\t\tObjectId parentTree;\n\t\ttry (RevWalk walk = new RevWalk(repository)) {\n\t\t\tparentTree = walk.parseCommit(parent).getTree();\n\t\t}\n\n\t\tObjectId treeId = buildTreeFromDelta(parent, parentTree);\n\n\t\tif (treeId.equals(parentTree))\n\t\t\treturn new SnapshotResult(false, namespace, null, parent, parent, triggerLabel);\n\n\t\tint n = lastN + 1;\n\t\tString message = \"op-\" + n + \" [\" + (triggerLabel == null ? \"?\" : triggerLabel) + \"] \"\n\t\t\t\t+ LocalDateTime.now().format(TS);\n\n\t\tObjectId newCommit;\n\t\ttry (ObjectInserter inserter = repository.newObjectInserter()) {\n\t\t\tCommitBuilder cb = new CommitBuilder();\n\t\t\tcb.setTreeId(treeId);\n\t\t\tcb.setParentId(parent);\n\t\t\tPersonIdent ident = new PersonIdent(\"xy.ai.workbench\", \"noreply@xy.ai.workbench\");\n\t\t\tcb.setAuthor(ident);\n\t\t\tcb.setCommitter(ident);\n\t\t\tcb.setMessage(message);\n\t\t\tnewCommit = inserter.insert(cb);\n\t\t\tinserter.flush();\n\t\t}\n\n\t\tString refName = namespace + \"/op-\" + n;\n\t\tupdateRef(refName, newCommit);\n\n\t\treturn new SnapshotResult(true, namespace, refName, newCommit, parent, triggerLabel);\n\t}\n"
      - id: OpSnapshotter.findLatest
        type: MethodDeclaration
        code: "\tpublic SnapshotResult findLatest() throws IOException {\n\t\tRef best = null;\n\t\tint bestTime = Integer.MIN_VALUE;\n\t\tfor (Ref ref : repository.getRefDatabase().getRefsByPrefix(\"refs/llm-ops/\")) {\n\t\t\tif (!OP_SUFFIX.matcher(ref.getName()).matches())\n\t\t\t\tcontinue;\n\n\t\t\ttry (RevWalk walk = new RevWalk(repository)) {\n\t\t\t\tRevCommit c = walk.parseCommit(ref.getObjectId());\n\t\t\t\tif (c.getCommitTime() > bestTime) {\n\t\t\t\t\tbestTime = c.getCommitTime();\n\t\t\t\t\tbest = ref;\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t\tif (best == null)\n\t\t\treturn null;\n\n\t\tString chain = best.getName().substring(0, best.getName().lastIndexOf('/'));\n\t\ttry (RevWalk walk = new RevWalk(repository)) {\n\t\t\tRevCommit c = walk.parseCommit(best.getObjectId());\n\t\t\tObjectId parent = c.getParentCount() > 0 ? c.getParent(0) : best.getObjectId();\n\t\t\treturn new SnapshotResult(true, chain, best.getName(), best.getObjectId(), parent, null);\n\t\t}\n\t}\n"
      - id: OpSnapshotter.revertLast
        type: MethodDeclaration
        code: "\tpublic synchronized SnapshotResult revertLast() throws IOException {\n\t\tObjectId storedBase = resolveRef(BASE_MARKER);\n\t\tif (storedBase == null)\n\t\t\treturn null;\n\n\t\tString chainId = repository.newObjectReader().abbreviate(storedBase, 7).name();\n\t\tString namespace = \"refs/llm-ops/chain-\" + chainId;\n\n\t\tint lastN = 0;\n\t\tfor (Ref ref : repository.getRefDatabase().getRefsByPrefix(namespace + \"/op-\")) {\n\t\t\tMatcher m = OP_SUFFIX.matcher(ref.getName());\n\t\t\tif (m.matches())\n\t\t\t\tlastN = Math.max(lastN, Integer.parseInt(m.group(1)));\n\t\t}\n\t\tif (lastN == 0)\n\t\t\treturn null;\n\n\t\tString lastOpRefName = namespace + \"/op-\" + lastN;\n\t\tObjectId toRevert = resolveRef(lastOpRefName);\n\n\t\tObjectId newHead;\n\t\ttry (RevWalk walk = new RevWalk(repository)) {\n\t\t\tRevCommit revertedCommit = walk.parseCommit(toRevert);\n\t\t\tnewHead = revertedCommit.getParentCount() > 0 ? revertedCommit.getParent(0) : storedBase;\n\t\t\tObjectId newHeadTree = walk.parseCommit(newHead).getTree();\n\t\t\tcheckoutDelta(revertedCommit.getTree(), newHeadTree);\n\t\t}\n\n\t\tdeleteRef(lastOpRefName);\n\n\t\tif (newHead.equals(storedBase))\n\t\t\treturn new SnapshotResult(false, namespace, null, storedBase, storedBase, null);\n\n\t\tString newRefName = namespace + \"/op-\" + (lastN - 1);\n\t\tObjectId newHeadParent;\n\t\ttry (RevWalk walk = new RevWalk(repository)) {\n\t\t\tRevCommit c = walk.parseCommit(newHead);\n\t\t\tnewHeadParent = c.getParentCount() > 0 ? c.getParent(0) : storedBase;\n\t\t}\n\t\treturn new SnapshotResult(true, namespace, newRefName, newHead, newHeadParent, null);\n\t}\n"
      - id: OpSnapshotter.uOcuA9|XCFEJm
        type: StatementGroup
        code: "\tprivate static final String BASE_MARKER = \"refs/llm-ops/_base\";\n\tprivate static final Pattern OP_SUFFIX = Pattern.compile(\".*/op-(\\\\d+)$\");\n\tprivate static final Map<String, Repository> REPOSITORY_CACHE = new ConcurrentHashMap<>();\n\tprivate static final DateTimeFormatter TS = DateTimeFormatter.ofPattern(\"yyyy-MM-dd HH:mm:ss\");\n\tprivate static final String DIFFIGNORE_FILE = \".diffignore\";\n\n\tprivate final Repository repository;\n"
```
/answer ed7c-61-60 allow
Thinking:
Der Snapshot-Commit speichert das Trigger-Label, sodass ich beim Deny prüfen kann, ob der letzte Snapshot zum verworfenen Result gehört. Dafür ergänze ich einen Guard in `revertLast`.


Control Request:
```yaml
id: ed7c-62-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    id: OpSnapshotter.revertLast
    old_text: "public synchronized SnapshotResult revertLast() throws IOException {"
    new_text: "public synchronized SnapshotResult revertLast(String expectedLabel) throws IOException {"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    id: OpSnapshotter.revertLast
    old_text: "\t\t\tRevCommit revertedCommit = walk.parseCommit(toRevert);\n"
    new_text: "\t\t\tRevCommit revertedCommit = walk.parseCommit(toRevert);\n\t\t\tif (expectedLabel != null && !expectedLabel.equals(labelOf(revertedCommit)))\n\t\t\t\treturn null;\n"
  reason: Make revertLast only revert when the last snapshot belongs to the denied operation
```
/answer ed7c-62-0 allow
ReasoningToken: 1191
Control Request:
```yaml
id: ed7c-63-62
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
      result: success
      id: OpSnapshotter.revertLast
      new_id: OpSnapshotter.revertLast
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
      error: Text not found in node; its content appears to have changed.
      id: OpSnapshotter.revertLast
      reason: content_changed
      guess: |2-
            try (RevWalk walk = new RevWalk(repository)) {
              RevCommit revertedCommit = walk.parseCommit(toRevert);
```
/answer ed7c-63-62 allow
Control Request:
```yaml
id: ed7c-64-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    id: OpSnapshotter.revertLast
    position: before
    source: "\tprivate static String labelOf(RevCommit commit) {\n\t\tMatcher m = LABEL_IN_MESSAGE.matcher(commit.getShortMessage());\n\t\treturn m.matches() ? m.group(1) : null;\n\t}\n"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    id: OpSnapshotter.uOcuA9|XCFEJm
    position: after
    source: "\tprivate static final Pattern LABEL_IN_MESSAGE = Pattern.compile(\"^op-\\\\d+ \\\\[(.*)\\\\] \\\\d{4}-.*$\");\n"
  reason: Add helper to read the trigger label from snapshot commit message and its pattern
```
/answer ed7c-64-0 allow
Control Request:
```yaml
id: ed7c-65-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/GitSupport.java
    id: GitSupport.triggerSnapshotIfNeeded
    old_text: snap.revertLast()
    new_text: snap.revertLast(opId)
  reason: Pass the denied result's id so only a matching snapshot is reverted
```
/answer ed7c-65-0 allow
Text:
 Die zweite Änderung an `revertLast` (der Guard) wurde nicht übernommen. Ich lese die Methode neu und wende sie erneut an.
ReasoningToken: 38
Control Request:
```yaml
id: ed7c-66-0
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    ids:
    - OpSnapshotter.revertLast
  reason: Re-read current state of revertLast before reapplying the guard edit
```
/answer ed7c-66-0 allow
Control Request:
```yaml
id: ed7c-67-66
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
      nodes:
      - id: OpSnapshotter.revertLast
        type: MethodDeclaration
        code: |2
            public synchronized SnapshotResult revertLast(String expectedLabel) throws IOException {
              ObjectId storedBase = resolveRef(BASE_MARKER);
              if (storedBase == null)
                return null;
              String chainId = repository.newObjectReader().abbreviate(storedBase, 7).name();
              String namespace = "refs/llm-ops/chain-" + chainId;
              int lastN = 0;
              for (Ref ref : repository.getRefDatabase().getRefsByPrefix(namespace + "/op-")) {
                Matcher m = OP_SUFFIX.matcher(ref.getName());
                if (m.matches())
                  lastN = Math.max(lastN, Integer.parseInt(m.group(1)));
              }
              if (lastN == 0)
                return null;
              String lastOpRefName = namespace + "/op-" + lastN;
              ObjectId toRevert = resolveRef(lastOpRefName);
              ObjectId newHead;
              try (RevWalk walk = new RevWalk(repository)) {
                RevCommit revertedCommit = walk.parseCommit(toRevert);
                newHead = revertedCommit.getParentCount() > 0 ? revertedCommit.getParent(0) : storedBase;
                ObjectId newHeadTree = walk.parseCommit(newHead).getTree();
                checkoutDelta(revertedCommit.getTree(), newHeadTree);
              }
              deleteRef(lastOpRefName);
              if (newHead.equals(storedBase))
                return new SnapshotResult(false, namespace, null, storedBase, storedBase, null);
              String newRefName = namespace + "/op-" + (lastN - 1);
              ObjectId newHeadParent;
              try (RevWalk walk = new RevWalk(repository)) {
                RevCommit c = walk.parseCommit(newHead);
                newHeadParent = c.getParentCount() > 0 ? c.getParent(0) : storedBase;
              }
              return new SnapshotResult(true, namespace, newRefName, newHead, newHeadParent, null);
            }
```
/answer ed7c-67-66 allow
Control Request:
```yaml
id: ed7c-68-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    id: OpSnapshotter.revertLast
    old_text: RevCommit revertedCommit = walk.parseCommit(toRevert);
    new_text: |-
      RevCommit revertedCommit = walk.parseCommit(toRevert);
            if (expectedLabel != null && !expectedLabel.equals(labelOf(revertedCommit)))
              return null;
  reason: Apply the label guard that failed in the first batch
```
/answer ed7c-68-0 allow
ReasoningToken: 25
Control Request:
```yaml
id: ed7c-69-0
toolName: search
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: revertLast\(
    include:
    - '*.java'
    exclude:
    - bin/**
    - target/**
  reason: Check for other callers of revertLast broken by the signature change
```
/answer ed7c-69-0 allow
Control Request:
```yaml
id: ed7c-6a-69
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src
      matches:
      - path: xy/ai/workbench/view/diff/OpSnapshotter.java
        matches:
        - "public synchronized SnapshotResult revertLast(String expectedLabel) throws IOException {"
      - path: xy/ai/workbench/eclipse/GitSupport.java
        matches:
        - "SnapshotResult result = revertToolResultDeny ? snap.revertLast(opId) : snap.snapshot(opId);"
```
/answer ed7c-6a-69 allow
Control Request:
```yaml
id: ed7c-6b-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/eclipse/GitSupport.java
  reason: Verify both edited files still parse
```
/answer ed7c-6b-0 allow
Result Stats: id=08a5642d-8ff4-44f8-87a8-5fe03c1b255e, total: 23886, in: 20, out: 5141, reason: 0, read: 103319, write: 18725, start: 1794174583541, end: 1794174590936, time: 7327, total: 42517, io: 0, control: 68, sent: 0, recv: 196
Ein Deny auf ein Tool-Result macht jetzt nur noch dann einen Revert, wenn der letzte Snapshot zu diesem Result gehört. Ich habe es nicht kompiliert und nicht gegen einen echten Ablauf getestet. Beide Dateien parsen fehlerfrei.

Ich habe deinen Vorschlag etwas anders umgesetzt als beschrieben. Ich vergleiche die ID des verworfenen Results direkt mit dem Label des letzten Snapshots, statt die dritte Position der ID auszuwerten.

- **`OpSnapshotter.revertLast(String expectedLabel)`:** Die Methode liest das Trigger-Label aus der Message des letzten `op-N`-Commits (`op-N [label] timestamp`). Weicht es von `expectedLabel` ab, passiert nichts: kein Checkout, kein Löschen der Ref, Rückgabe `null`. Mit `expectedLabel == null` verhält sie sich wie vorher. Dafür gibt es die neue Hilfsmethode `labelOf` und das Pattern `LABEL_IN_MESSAGE`.
- **`GitSupport.triggerSnapshotIfNeeded`:** Es ruft jetzt `snap.revertLast(opId)` auf. `opId` ist die ID des verworfenen Results.

Bei einem Lese-Toolcall wie `list` entsteht kein Snapshot, weil sich der Tree nicht ändert. Der letzte Snapshot gehört dann zu einer früheren Operation, das Label passt nicht, und es wird nichts zurückgesetzt.

**Annahme:** Ich gehe davon aus, dass der Snapshot nach einem schreibenden Toolcall mit der ID des Result-Prompts beschriftet wird (z. B. `b725-2b-2a`). Das ist dieselbe ID, die später beim Deny ankommt. Falls das Label anders aufgebaut ist, z. B. mit der Tool-Call-ID `b725-2a-0`, würde der Revert nie ausgelöst. Dann müsste der Vergleich über die Tool-Call-ID (`2a`) laufen. Sag mir, wenn du das so brauchst.

Die Datei `OpSnapshotter.java` hat Einrückung mit Leerzeichen statt Tabs. Das stammt nicht von mir, die Datei ist offenbar schon umformatiert worden. Meine Einfügungen folgen dem jeweils umgebenden Stil, der Mix ist also möglich.
Die erste Position ist die Session, die zweite ein fortlaufenden Zähler ("0" bei einem Tool-Call selbst da dieser keine Referenz hat), die dritte der Referenzierte Tool-Call.
SystemInit:  id=08a5642d-8ff4-44f8-87a8-5fe03c1b255e, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Text:
 Danke, das ändert den Vergleich. Mein exakter Label-Vergleich würde scheitern, wenn der Snapshot mit der Tool-Call-ID (`b725-2a-0`) statt mit der Result-ID (`b725-2b-2a`) beschriftet ist. Ich normalisiere beide IDs auf den Tool-Call-Schlüssel `session-2a`. Dazu nehme ich die Referenz, falls sie nicht `0` ist, sonst den Zähler.
Control Request:
```yaml
id: ed7c-6c-0
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    id: OpSnapshotter.revertLast
    old_text: if (expectedLabel != null && !expectedLabel.equals(labelOf(revertedCommit)))
    new_text: if (expectedLabel != null && !toolCallKey(expectedLabel).equals(toolCallKey(labelOf(revertedCommit))))
  reason: Compare snapshot label and denied id via the referenced tool call instead of exact equality
```
/answer ed7c-6c-0 allow
ReasoningToken: 542
Control Request:
```yaml
id: ed7c-6d-0
toolName: ast_insert
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
    id: OpSnapshotter.labelOf
    position: after
    source: "\t/** Maps \"session-counter-ref\" to \"session-toolCallCounter\": the ref if set (\"0\" = none), else the own counter. */\n\tstatic String toolCallKey(String id) {\n\t\tif (id == null)\n\t\t\treturn \"\";\n\t\tString[] parts = id.split(\"-\");\n\t\tif (parts.length < 3)\n\t\t\treturn id;\n\t\tString ref = parts[parts.length - 1];\n\t\tString counter = parts[parts.length - 2];\n\t\tString session = String.join(\"-\", java.util.Arrays.copyOf(parts, parts.length - 2));\n\t\treturn session + \"-\" + (\"0\".equals(ref) ? counter : ref);\n\t}\n"
  reason: Add helper that derives the tool-call key from an id of the form session-counter-ref
```
/answer ed7c-6d-0 allow
Control Request:
```yaml
id: ed7c-6e-0
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java
  reason: Verify the file still parses after the edit
```
/answer ed7c-6e-0 allow
Result Stats: id=08a5642d-8ff4-44f8-87a8-5fe03c1b255e, total: 28970, in: 26, out: 7212, reason: 0, read: 163249, write: 21732, start: 1794174769298, end: 1794174773318, time: 3965, total: 56128, io: 0, control: 55, sent: 0, recv: 100
Der Vergleich in `revertLast` läuft jetzt über den Tool-Call-Schlüssel statt über exakte Gleichheit. Die Datei parst fehlerfrei, kompiliert oder getestet habe ich nichts.

`OpSnapshotter.toolCallKey(id)` zerlegt die ID in `session-zähler-ref`. Der Schlüssel ist `session-<ref>`, bei Ref `0` (also beim Tool-Call selbst) `session-<zähler>`. Beispiele:
- `b725-2a-0` ergibt `b725-2a`.
- `b725-2b-2a` ergibt `b725-2a`.

`revertLast(expectedLabel)` setzt nur zurück, wenn der Schlüssel der verworfenen ID mit dem Schlüssel des Labels im letzten Snapshot übereinstimmt. Es spielt keine Rolle, ob der Snapshot mit der Tool-Call-ID oder mit der Result-ID beschriftet wurde. Bei einem `list`-Call entsteht kein Snapshot für `2a`. Der letzte Snapshot gehört dann zu einem anderen Tool-Call, und es passiert kein Revert.

Ein Snapshot mit einem Label ohne diese Struktur (weniger als drei Segmente) wird nur bei exakt gleichem String als passend gewertet.
