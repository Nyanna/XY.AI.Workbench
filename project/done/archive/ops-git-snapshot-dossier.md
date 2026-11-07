# LLM Operation Snapshots via Git — Implementation Dossier

## 1. Concept Summary

Every LLM-driven file operation inside the Eclipse plugin is captured as a
**commit outside of HEAD/branch**, stored under a dedicated ref namespace
(`refs/llm-ops/chain-<base>/op-N`). This gives:

- Atomic, exact before/after snapshots (Git tree objects, not patches)
- A Human-in-the-loop (HITL) gate: execute → diff → accept/revert
- Full session replay/audit via `git bundle` or `for-each-ref`
- Zero impact on the real branch, HEAD, index, or remote

**Chain semantics:** a chain is a sequence of `op-N` commits parented on one
another, rooted at a base HEAD commit. If HEAD advances (a real commit
happens on the branch) between snapshots, the current chain is closed and a
**new chain** is started, rooted at the new HEAD. Old chains remain fully
inspectable under their own namespace.

---

## 2. Bash Reference Implementation (baseline, CLI)

```bash
#!/usr/bin/env bash
#
# llm-op-snapshot.sh — Snapshot an LLM operation as a commit outside of
#                       HEAD/branch, under refs/llm-ops/*.
#
# Usage:
#   llm-op-snapshot.sh [TARGET_DIR]
#
# TARGET_DIR: path to the git repository, defaults to CWD.

set -euo pipefail

TARGET_DIR="${1:-$(pwd)}"

if ! GIT_DIR_CHECK=$(git -C "$TARGET_DIR" rev-parse --git-dir 2>&1); then
    echo "Error: '$TARGET_DIR' is not a git repository." >&2
    echo "$GIT_DIR_CHECK" >&2
    exit 1
fi

cd "$TARGET_DIR"

if ! CURRENT_HEAD=$(git rev-parse HEAD 2>/dev/null); then
    echo "Error: no HEAD found (empty repository?)." >&2
    exit 1
fi

BASE_MARKER="refs/llm-ops/_base"

STORED_BASE=""
if git show-ref --verify --quiet "$BASE_MARKER"; then
    STORED_BASE=$(git rev-parse "$BASE_MARKER")
fi

if [ -z "$STORED_BASE" ] || [ "$STORED_BASE" != "$CURRENT_HEAD" ]; then
    git update-ref "$BASE_MARKER" "$CURRENT_HEAD"
    STORED_BASE="$CURRENT_HEAD"
fi

CHAIN_ID=$(git rev-parse --short "$STORED_BASE")
NAMESPACE="refs/llm-ops/chain-$CHAIN_ID"

LAST_N=0
LAST_OP_REF=""
while IFS= read -r ref; do
    [ -z "$ref" ] && continue
    num="${ref##*/op-}"
    if [[ "$num" =~ ^[0-9]+$ ]] && [ "$num" -gt "$LAST_N" ]; then
        LAST_N="$num"
        LAST_OP_REF="$ref"
    fi
done < <(git for-each-ref --format='%(refname)' "${NAMESPACE}/op-*" 2>/dev/null || true)

if [ -n "$LAST_OP_REF" ]; then
    PARENT=$(git rev-parse "$LAST_OP_REF")
else
    PARENT="$STORED_BASE"
fi

N=$((LAST_N + 1))
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
MSG="op-$N: $TIMESTAMP"

git add -A
TREE=$(git write-tree)

PARENT_TREE=$(git rev-parse "${PARENT}^{tree}")
if [ "$TREE" = "$PARENT_TREE" ]; then
    echo "No changes since last snapshot ($PARENT) — no new commit created." >&2
    echo "SNAPSHOT_REF="
    echo "SNAPSHOT_COMMIT=$PARENT"
    echo "SNAPSHOT_PARENT=$PARENT"
    echo "SNAPSHOT_CHAIN=$NAMESPACE"
    exit 0
fi

NEW_COMMIT=$(git commit-tree "$TREE" -p "$PARENT" -m "$MSG")
REF_NAME="${NAMESPACE}/op-$N"
git update-ref "$REF_NAME" "$NEW_COMMIT"

git diff "$PARENT" "$NEW_COMMIT"

echo "SNAPSHOT_REF=$REF_NAME"
echo "SNAPSHOT_COMMIT=$NEW_COMMIT"
echo "SNAPSHOT_PARENT=$PARENT"
echo "SNAPSHOT_CHAIN=$NAMESPACE"
```

**Revert:**

```bash
#!/usr/bin/env bash
set -euo pipefail
TARGET_DIR="${1:?target directory required}"
TARGET_COMMIT="${2:?target commit or ref required}"

cd "$TARGET_DIR"
git read-tree --reset -u "$TARGET_COMMIT"
git clean -fd
```

---

## 3. JGit Port

### 3.1 Dependencies

```xml
<!-- Maven, or equivalent OSGi bundle dependency in MANIFEST.MF -->
<dependency>
    <groupId>org.eclipse.jgit</groupId>
    <artifactId>org.eclipse.jgit</artifactId>
    <version>[6.0,)</version>
</dependency>
```

In an Eclipse plugin, JGit is already a transitive dependency of EGit
(`org.eclipse.jgit` bundle) — add it as a required plugin dependency in
`MANIFEST.MF` rather than bundling a second copy.

### 3.2 Snapshot creation (equivalent of the bash script)

```java
import org.eclipse.jgit.lib.*;
import org.eclipse.jgit.dircache.*;
import org.eclipse.jgit.revwalk.RevCommit;
import org.eclipse.jgit.api.Git;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

public class LlmOpSnapshotter {

    private static final String BASE_MARKER = "refs/llm-ops/_base";

    private final Repository repository;

    public LlmOpSnapshotter(Repository repository) {
        this.repository = repository;
    }

    public SnapshotResult snapshot() throws Exception {
        ObjectId currentHead = repository.resolve("HEAD");
        if (currentHead == null) {
            throw new IllegalStateException("No HEAD found (empty repository?)");
        }

        // --- Determine / (re)anchor the active chain -----------------------
        ObjectId storedBase = repository.resolve(BASE_MARKER);
        if (storedBase == null || !storedBase.equals(currentHead)) {
            RefUpdate baseUpdate = repository.updateRef(BASE_MARKER);
            baseUpdate.setNewObjectId(currentHead);
            baseUpdate.forceUpdate();
            storedBase = currentHead;
        }

        String chainId = storedBase.abbreviate(7).name();
        String namespace = "refs/llm-ops/chain-" + chainId;

        // --- Find last snapshot in this chain --------------------------------
        int lastN = 0;
        Ref lastOpRef = null;
        Map<String, Ref> refs = repository.getRefDatabase()
                .getRefsByPrefix(namespace + "/op-")
                .stream()
                .collect(Collectors.toMap(Ref::getName, r -> r));

        for (Map.Entry<String, Ref> e : refs.entrySet()) {
            String suffix = e.getKey().substring(e.getKey().lastIndexOf("op-") + 3);
            try {
                int n = Integer.parseInt(suffix);
                if (n > lastN) {
                    lastN = n;
                    lastOpRef = e.getValue();
                }
            } catch (NumberFormatException ignored) { /* skip non-numeric refs */ }
        }

        ObjectId parent = (lastOpRef != null)
                ? lastOpRef.getObjectId()
                : storedBase;

        int n = lastN + 1;
        String timestamp = LocalDateTime.now()
                .format(DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss"));
        String message = "op-" + n + ": " + timestamp;

        // --- Write tree from current working tree state ----------------------
        ObjectId treeId;
        try (Git git = new Git(repository)) {
            git.add().addFilepattern(".").call();
        }
        try (ObjectInserter inserter = repository.newObjectInserter()) {
            DirCache cache = repository.readDirCache();
            treeId = cache.writeTree(inserter);
            inserter.flush();
        }

        // --- No-op guard: compare against parent's tree -----------------------
        try (RevWalk walk = new RevWalk(repository)) {
            RevCommit parentCommit = walk.parseCommit(parent);
            if (parentCommit.getTree().getId().equals(treeId)) {
                return new SnapshotResult(null, parent, parent, namespace, true);
            }
        }

        // --- Build commit object, no HEAD/branch movement ----------------------
        ObjectId newCommit;
        try (ObjectInserter inserter = repository.newObjectInserter()) {
            CommitBuilder cb = new CommitBuilder();
            cb.setTreeId(treeId);
            cb.setParentId(parent);
            cb.setMessage(message);
            PersonIdent ident = new PersonIdent(repository);
            cb.setAuthor(ident);
            cb.setCommitter(ident);
            newCommit = inserter.insert(cb);
            inserter.flush();
        }

        String refName = namespace + "/op-" + n;
        RefUpdate update = repository.updateRef(refName);
        update.setNewObjectId(newCommit);
        update.update();

        return new SnapshotResult(refName, newCommit, parent, namespace, false);
    }

    public record SnapshotResult(
            String refName,
            ObjectId commit,
            ObjectId parent,
            String chainNamespace,
            boolean noOp) {}
}
```

### 3.3 Diff generation (unified diff, full commit — matches `git diff parent commit`)

```java
import org.eclipse.jgit.diff.DiffFormatter;
import org.eclipse.jgit.util.io.DisabledOutputStream;
import java.io.ByteArrayOutputStream;

public class LlmOpDiffReader {

    public static String unifiedDiff(Repository repository,
                                      ObjectId oldCommit,
                                      ObjectId newCommit) throws Exception {
        try (RevWalk walk = new RevWalk(repository);
             ByteArrayOutputStream out = new ByteArrayOutputStream()) {

            RevCommit oldRev = walk.parseCommit(oldCommit);
            RevCommit newRev = walk.parseCommit(newCommit);

            try (DiffFormatter formatter = new DiffFormatter(out)) {
                formatter.setRepository(repository);
                formatter.format(oldRev.getTree(), newRev.getTree());
            }
            return out.toString("UTF-8");
        }
    }
}
```

This single `format()` call over the whole tree pair reproduces exactly what
EGit's Commit Editor "Diff" tab shows: a full unified diff across *all*
changed files in one text block, not per-file fragments.

### 3.4 Revert (equivalent of `git read-tree --reset -u` + `git clean -fd`)

```java
import org.eclipse.jgit.dircache.DirCacheCheckout;
import org.eclipse.jgit.api.Git;

public class LlmOpReverter {

    public static void revertTo(Repository repository, ObjectId targetCommit)
            throws Exception {
        try (RevWalk walk = new RevWalk(repository)) {
            RevCommit target = walk.parseCommit(targetCommit);

            DirCache dc = repository.lockDirCache();
            DirCacheCheckout checkout = new DirCacheCheckout(
                    repository, dc, target.getTree());
            checkout.setFailOnConflict(true);
            checkout.checkout(); // rewrites index + working tree exactly

            // Remove files created by the operation that aren't part of the
            // target tree (DirCacheCheckout does not delete untracked files).
            try (Git git = new Git(repository)) {
                git.clean().setCleanDirectories(true).call();
            }
        }
    }
}
```

**Important:** do not use JGit's `ResetCommand` here — it is designed to move
`HEAD`/branch, which must stay untouched. `DirCacheCheckout` operates purely
on the tree/index/working-tree triad without touching refs.

### 3.5 Session export / cleanup

```java
// List all chains and snapshots (for-each-ref equivalent)
List<Ref> allOps = repository.getRefDatabase().getRefsByPrefix("refs/llm-ops/");

// Delete a single snapshot ref
RefUpdate del = repository.updateRef("refs/llm-ops/chain-abc123/op-4");
del.setForceUpdate(true);
del.delete();

// Bundle export (still simplest via CLI process call or JGit's
// org.eclipse.jgit.api.Git — JGit has no first-class bundle-create API as of
// current stable releases; shelling out to `git bundle create ...` remains
// the pragmatic choice here).
```

---

## 4. Reading EGit's History Panel Programmatically

### 4.1 Opening the standard History view for a resource/repo

```java
import org.eclipse.team.ui.history.IHistoryView;
import org.eclipse.ui.PlatformUI;

IHistoryView historyView = (IHistoryView) PlatformUI.getWorkbench()
        .getActiveWorkbenchWindow()
        .getActivePage()
        .showView(IHistoryView.VIEW_ID); // "org.eclipse.team.ui.GenericHistoryView"

// EGit registers itself as the history page provider for git-tracked
// resources/repositories. To show history for a non-branch ref or a
// RepositoryCommit object:
historyView.showHistoryFor(repositoryCommitOrResource);
```

`IHistoryView` is public Eclipse Platform Team API
(`org.eclipse.team.ui.history`) — EGit is only a plug-in behind it, so this
call is stable across EGit versions.

### 4.2 Additional Refs — where custom namespaces surface

EGit's "Git Repositories" view has an **Additional Refs** category that
lists any ref outside `refs/heads/*`, `refs/remotes/*`, `refs/tags/*` — this
is where `refs/llm-ops/chain-*/op-N` will automatically appear, *provided
each ref points to a commit* (not a raw tree — see §5, this was the reason
for wrapping trees in commit objects via `commit-tree`/`CommitBuilder`
rather than leaving raw `write-tree` results unwrapped).

### 4.3 Opening EGit's Commit Editor for one specific commit

Public, version-stable command indirection (avoids importing
`org.eclipse.egit.ui.internal.*`, which is not exported):

```java
import org.eclipse.core.commands.*;
import org.eclipse.ui.commands.ICommandService;
import org.eclipse.ui.handlers.IHandlerService;
import java.util.Collections;

IHandlerService handlerService = PlatformUI.getWorkbench()
        .getService(IHandlerService.class);
ICommandService commandService = PlatformUI.getWorkbench()
        .getService(ICommandService.class);

Command cmd = commandService.getCommand("org.eclipse.egit.ui.commit.OpenCommit");
ParameterizedCommand pc = ParameterizedCommand.generateCommand(
        cmd, Collections.singletonMap("commitId", newCommit.getName()));
handlerService.executeCommand(pc, null);
```

**Caveat:** verify this command ID against the exact EGit version you build
against (`plugin.xml` of `org.eclipse.egit.ui`, search for
`org.eclipse.egit.ui.commit.OpenCommit`) — command IDs are more stable than
internal classes but not guaranteed across major EGit releases.

**Limitation this dossier's §5 solves:** this command always opens a new
editor tab per commit (Eclipse editors are multi-instance by identity of
input). For a persistent single panel, do not use this path — use §5
instead.

---

## 5. Standalone Singleton Panel — Full-Operation Unified Diff (Commit-Viewer style)

### 5.1 Why a `ViewPart`, not an `EditorPart`

Eclipse `ViewPart`s are singletons by default (`allowMultiple` defaults to
`false` in the view's `plugin.xml` extension) — exactly the behavior needed
here, unlike `IEditorPart`, which opens a new instance per distinct input.

### 5.2 Rendering approach: raw unified diff text, not per-file Compare widgets

Since `DiffFormatter.format(oldTree, newTree)` (§3.3) already produces the
same aggregated unified-diff text that EGit's own Commit Editor "Diff" tab
displays, the panel needs only **one text viewer** fed with that string —
no per-file `CompareEditorInput` instantiation, no `DiffEntry` iteration,
no per-file widget lifecycle management.

### 5.3 Full ViewPart implementation

```java
package com.example.llmops.ui;

import org.eclipse.jface.text.*;
import org.eclipse.jface.text.presentation.PresentationReconciler;
import org.eclipse.jface.text.rules.*;
import org.eclipse.jface.text.source.SourceViewer;
import org.eclipse.jface.viewers.*;
import org.eclipse.swt.SWT;
import org.eclipse.swt.custom.SashForm;
import org.eclipse.swt.widgets.*;
import org.eclipse.ui.part.ViewPart;
import org.eclipse.jgit.lib.*;
import org.eclipse.jgit.revwalk.RevCommit;
import org.eclipse.jgit.revwalk.RevWalk;

import java.util.*;
import java.util.stream.Collectors;

public class LlmOpsPanel extends ViewPart {

    public static final String VIEW_ID = "com.example.llmops.ui.LlmOpsPanel";

    private Repository repository; // injected/set externally after view creation

    private TreeViewer chainViewer;
    private SourceViewer diffViewer;

    @Override
    public void createPartControl(Composite parent) {
        SashForm mainSash = new SashForm(parent, SWT.VERTICAL);

        // --- Ref/Chain section --------------------------------------------
        Composite refSection = new Composite(mainSash, SWT.NONE);
        refSection.setLayout(new FillLayout());
        chainViewer = new TreeViewer(refSection, SWT.SINGLE | SWT.FULL_SELECTION);
        chainViewer.setContentProvider(new ChainTreeContentProvider());
        chainViewer.setLabelProvider(new ChainTreeLabelProvider());
        chainViewer.addSelectionChangedListener(this::onSnapshotSelected);

        // --- Unified diff section -------------------------------------------
        Composite diffSection = new Composite(mainSash, SWT.NONE);
        diffSection.setLayout(new FillLayout());
        diffViewer = new SourceViewer(diffSection, null,
                SWT.V_SCROLL | SWT.H_SCROLL | SWT.MULTI);
        diffViewer.setEditable(false);
        diffViewer.configure(new UnifiedDiffViewerConfiguration());

        mainSash.setWeights(new int[]{30, 70});
    }

    /** Called once by the plugin after view creation to bind a repository. */
    public void setRepository(Repository repository) {
        this.repository = repository;
        refresh();
    }

    public void refresh() {
        if (repository == null) return;
        List<ChainNode> chains = loadChains(repository);
        chainViewer.setInput(chains);
    }

    private void onSnapshotSelected(SelectionChangedEvent event) {
        IStructuredSelection sel = (IStructuredSelection) event.getSelection();
        Object element = sel.getFirstElement();
        if (!(element instanceof OpNode opNode)) return;

        try {
            String diffText = LlmOpDiffReader.unifiedDiff(
                    repository, opNode.parent, opNode.commit);
            diffViewer.setDocument(new Document(diffText));
        } catch (Exception e) {
            diffViewer.setDocument(new Document("Error loading diff: " + e.getMessage()));
        }
    }

    @Override
    public void setFocus() {
        chainViewer.getControl().setFocus();
    }

    // --- Tree model -----------------------------------------------------------

    private List<ChainNode> loadChains(Repository repo) {
        Map<String, List<Ref>> byChain = new TreeMap<>();
        try {
            repo.getRefDatabase().getRefsByPrefix("refs/llm-ops/chain-")
                .forEach(ref -> {
                    String[] parts = ref.getName().split("/");
                    String chainName = parts[2]; // "chain-<hash>"
                    byChain.computeIfAbsent(chainName, k -> new ArrayList<>()).add(ref);
                });
        } catch (Exception e) {
            return Collections.emptyList();
        }

        List<ChainNode> result = new ArrayList<>();
        for (Map.Entry<String, List<Ref>> e : byChain.entrySet()) {
            ChainNode chainNode = new ChainNode(e.getKey());
            List<Ref> ops = e.getValue().stream()
                    .sorted(Comparator.comparingInt(r -> extractOpNumber(r.getName())))
                    .collect(Collectors.toList());

            ObjectId prevParent = resolveChainBase(repo, e.getKey());
            for (Ref ref : ops) {
                chainNode.children.add(new OpNode(
                        ref.getName(), ref.getObjectId(), prevParent));
                prevParent = ref.getObjectId();
            }
            result.add(chainNode);
        }
        return result;
    }

    private ObjectId resolveChainBase(Repository repo, String chainName) {
        // chain-<7charhash> -> resolve the abbreviated base commit
        String abbrev = chainName.substring("chain-".length());
        try {
            return repo.resolve(abbrev);
        } catch (Exception e) {
            return null;
        }
    }

    private int extractOpNumber(String refName) {
        String suffix = refName.substring(refName.lastIndexOf("op-") + 3);
        try {
            return Integer.parseInt(suffix);
        } catch (NumberFormatException ex) {
            return 0;
        }
    }

    // --- Model classes ----------------------------------------------------------

    static class ChainNode {
        final String name;
        final List<OpNode> children = new ArrayList<>();
        ChainNode(String name) { this.name = name; }
    }

    static class OpNode {
        final String refName;
        final ObjectId commit;
        final ObjectId parent;
        OpNode(String refName, ObjectId commit, ObjectId parent) {
            this.refName = refName;
            this.commit = commit;
            this.parent = parent;
        }
    }

    static class ChainTreeContentProvider implements ITreeContentProvider {
        @SuppressWarnings("unchecked")
        @Override
        public Object[] getElements(Object input) {
            return ((List<ChainNode>) input).toArray();
        }
        @Override
        public Object[] getChildren(Object parent) {
            if (parent instanceof ChainNode cn) return cn.children.toArray();
            return new Object[0];
        }
        @Override
        public Object getParent(Object element) { return null; }
        @Override
        public boolean hasChildren(Object element) {
            return element instanceof ChainNode cn && !cn.children.isEmpty();
        }
    }

    static class ChainTreeLabelProvider extends LabelProvider {
        @Override
        public String getText(Object element) {
            if (element instanceof ChainNode cn) return cn.name;
            if (element instanceof OpNode on) return on.refName;
            return super.getText(element);
        }
    }

    /** Minimal +/-/@@ line highlighting for unified diff text. */
    static class UnifiedDiffViewerConfiguration extends SourceViewerConfiguration {
        @Override
        public IPresentationReconciler getPresentationReconciler(ISourceViewer sv) {
            PresentationReconciler reconciler = new PresentationReconciler();
            RuleBasedScanner scanner = new RuleBasedScanner();
            scanner.setRules(new IRule[] {
                new EndOfLineRule("+", token(0, 128, 0)),   // additions: green
                new EndOfLineRule("-", token(160, 0, 0)),   // deletions: red
                new EndOfLineRule("@@", token(0, 0, 160)),  // hunk headers: blue
            });
            DefaultDamagerRepairer ddr = new DefaultDamagerRepairer(scanner);
            reconciler.setDamager(ddr, IDocument.DEFAULT_CONTENT_TYPE);
            reconciler.setRepairer(ddr, IDocument.DEFAULT_CONTENT_TYPE);
            return reconciler;
        }

        private IToken token(int r, int g, int b) {
            return new Token(new TextAttribute(
                    org.eclipse.jface.resource.JFaceResources.getColorRegistry()
                        .get("llmops." + r + "." + g + "." + b) != null
                        ? null : null)); // simplified; register real Color via ColorRegistry
        }
    }
}
```

### 5.4 View registration (`plugin.xml`)

```xml
<extension point="org.eclipse.ui.views">
    <view
        id="com.example.llmops.ui.LlmOpsPanel"
        name="LLM Operations"
        class="com.example.llmops.ui.LlmOpsPanel"
        allowMultiple="false"
        restorable="true">
    </view>
</extension>
```

`allowMultiple="false"` (the default, stated explicitly here for clarity)
guarantees the singleton behavior — repeated `showView(VIEW_ID)` calls from
the hook return the same instance rather than creating a new one:

```java
LlmOpsPanel panel = (LlmOpsPanel) PlatformUI.getWorkbench()
        .getActiveWorkbenchWindow().getActivePage()
        .showView(LlmOpsPanel.VIEW_ID);
panel.setRepository(repository);
panel.refresh();
```

### 5.5 Revert action wiring

Add a toolbar button (`org.eclipse.ui.menus` contribution to the view's
`id:toolbar`) invoking:

```java
LlmOpReverter.revertTo(repository, selectedOpNode.parent);
panel.refresh();
```

---

## 6. Key Pitfalls Checklist

| # | Pitfall | Mitigation |
|---|---------|------------|
| 1 | Raw `write-tree` results don't show in EGit's Additional Refs | Wrap every snapshot in a commit object (`commit-tree` / `CommitBuilder`), never leave a ref pointing at a bare tree |
| 2 | Chain forms a fan instead of a line in history views | Always parent `op-N` on `op-(N-1)`, not on the fixed base commit |
| 3 | `ResetCommand` / `git reset --hard` moves HEAD/branch | Use `DirCacheCheckout` (JGit) / `git read-tree --reset -u` (CLI) for revert instead |
| 4 | Untracked new files survive a revert | Always follow revert with `git clean -fd` / JGit `CleanCommand` |
| 5 | Loose/unreferenced tree objects get garbage-collected | Every snapshot must be reachable via a ref (`refs/llm-ops/...`), not just a returned hash held in memory |
| 6 | Reflog assumed to cover this — it doesn't | Reflog only tracks ref *movements* for `HEAD`/`refs/heads/*` by default, carries no diff content, and isn't enabled for custom namespaces unless forced |
| 7 | Push accidentally includes `refs/llm-ops/*` | Default `git push` only sends configured refspecs (typically `refs/heads/*`); do not add an explicit push refspec for `refs/llm-ops/*` |
| 8 | EGit Commit Editor opens a new tab per commit | Use a singleton `ViewPart` (§5) with `DiffFormatter`-generated text embedded directly, instead of invoking the `OpenCommit` editor command |
| 9 | No-op snapshots pollute the chain | Compare `write-tree` result against parent's tree before committing; skip if identical |
| 10 | Concurrent external git operations during LLM edit | Ensure the plugin's hooks exclusively bracket the LLM operation; no manual edits/other git processes should run between pre- and post-hook |
