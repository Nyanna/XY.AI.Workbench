package xy.ai.workbench.view.diff;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.concurrent.ConcurrentHashMap;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.eclipse.core.resources.IProject;
import org.eclipse.core.resources.IWorkspaceRoot;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.jgit.dircache.DirCache;
import org.eclipse.jgit.dircache.DirCacheBuilder;
import org.eclipse.jgit.dircache.DirCacheEntry;
import org.eclipse.jgit.ignore.IgnoreNode;
import org.eclipse.jgit.ignore.IgnoreNode.MatchResult;
import org.eclipse.jgit.lib.CommitBuilder;
import org.eclipse.jgit.lib.Constants;
import org.eclipse.jgit.lib.FileMode;
import org.eclipse.jgit.lib.IndexDiff;
import org.eclipse.jgit.lib.ObjectId;
import org.eclipse.jgit.lib.ObjectInserter;
import org.eclipse.jgit.lib.ObjectReader;
import org.eclipse.jgit.lib.PersonIdent;
import org.eclipse.jgit.lib.Ref;
import org.eclipse.jgit.lib.RefUpdate;
import org.eclipse.jgit.lib.RefUpdate.Result;
import org.eclipse.jgit.lib.Repository;
import org.eclipse.jgit.revwalk.RevCommit;
import org.eclipse.jgit.revwalk.RevWalk;
import org.eclipse.jgit.storage.file.FileRepositoryBuilder;
import org.eclipse.jgit.treewalk.FileTreeIterator;
import org.eclipse.jgit.treewalk.TreeWalk;
import org.eclipse.jgit.treewalk.WorkingTreeIterator;
import org.eclipse.jgit.treewalk.filter.PathFilterGroup;

public class OpSnapshotter {

	private static final String BASE_MARKER = "refs/llm-ops/_base";
	private static final Pattern OP_SUFFIX = Pattern.compile(".*/op-(\\d+)$");
	private static final Map<String, Repository> REPOSITORY_CACHE = new ConcurrentHashMap<>();
	private static final DateTimeFormatter TS = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
	private static final String DIFFIGNORE_FILE = ".diffignore";

	private final Repository repository;

	public OpSnapshotter(Repository repository) {
		this.repository = repository;
	}

	public OpSnapshotter(File start) throws IOException {
		this(loadRepository(start));
	}

	public OpSnapshotter() throws IOException {
		this(openWorkspaceRepository());
	}

	private static Repository openWorkspaceRepository() throws IOException {
		IWorkspaceRoot root = ResourcesPlugin.getWorkspace().getRoot();
		File start = null;
		for (IProject p : root.getProjects())
			if (p.isAccessible() && p.getLocation() != null) {
				start = p.getLocation().toFile();
				break;
			}
		if (start == null)
			start = root.getLocation().toFile();
		return loadRepository(start);
	}

	public static Repository loadRepository(File start) throws IOException {
		String key = start.getCanonicalPath();
		Repository cached = REPOSITORY_CACHE.get(key);
		if (cached != null)
			return cached;
		synchronized (REPOSITORY_CACHE) {
			cached = REPOSITORY_CACHE.get(key);
			if (cached != null)
				return cached;
			FileRepositoryBuilder gitDir = new FileRepositoryBuilder() //
					.findGitDir(start);
			if (gitDir == null)
				throw new IOException("No GIT Repository found " + start);
			Repository repository = gitDir.build();
			REPOSITORY_CACHE.put(key, repository);
			return repository;
		}
	}

	public Repository getRepository() {
		return repository;
	}

	public synchronized SnapshotResult snapshot(String triggerLabel) throws IOException {
		ObjectId currentHead = repository.resolve("HEAD");
		if (currentHead == null)
			throw new IOException("No HEAD found");

		ObjectId storedBase = resolveRef(BASE_MARKER);
		if (storedBase == null || !storedBase.equals(currentHead)) {
			updateRef(BASE_MARKER, currentHead);
			storedBase = currentHead;
		}

		String chainId = repository.newObjectReader().abbreviate(storedBase, 7).name();
		String namespace = "refs/llm-ops/chain-" + chainId;

		int lastN = 0;
		String lastOpRefName = null;
		for (Ref ref : repository.getRefDatabase().getRefsByPrefix(namespace + "/op-")) {
			Matcher m = OP_SUFFIX.matcher(ref.getName());
			if (m.matches()) {
				int n = Integer.parseInt(m.group(1));
				if (n > lastN) {
					lastN = n;
					lastOpRefName = ref.getName();
				}
			}
		}
		ObjectId parent = lastOpRefName != null ? resolveRef(lastOpRefName) : storedBase;

		ObjectId parentTree;
		try (RevWalk walk = new RevWalk(repository)) {
			parentTree = walk.parseCommit(parent).getTree();
		}

		ObjectId treeId = buildTreeFromDelta(parent, parentTree);

		if (treeId.equals(parentTree))
			return new SnapshotResult(false, namespace, null, parent, parent, triggerLabel);

		int n = lastN + 1;
		String message = "op-" + n + " [" + (triggerLabel == null ? "?" : triggerLabel) + "] "
				+ LocalDateTime.now().format(TS);

		ObjectId newCommit;
		try (ObjectInserter inserter = repository.newObjectInserter()) {
			CommitBuilder cb = new CommitBuilder();
			cb.setTreeId(treeId);
			cb.setParentId(parent);
			PersonIdent ident = new PersonIdent("xy.ai.workbench", "noreply@xy.ai.workbench");
			cb.setAuthor(ident);
			cb.setCommitter(ident);
			cb.setMessage(message);
			newCommit = inserter.insert(cb);
			inserter.flush();
		}

		String refName = namespace + "/op-" + n;
		updateRef(refName, newCommit);

		return new SnapshotResult(true, namespace, refName, newCommit, parent, triggerLabel);
	}

	private ObjectId buildTreeFromDelta(ObjectId parentCommit, ObjectId parentTree) throws IOException {
		IndexDiff diff = new IndexDiff(repository, parentCommit, new FileTreeIterator(repository));
		diff.diff();

		Set<String> changedPaths = new HashSet<>();
		changedPaths.addAll(diff.getAdded());
		changedPaths.addAll(diff.getChanged());
		changedPaths.addAll(diff.getRemoved());
		changedPaths.addAll(diff.getMissing());
		changedPaths.addAll(diff.getModified());
		changedPaths.addAll(diff.getUntracked());
		changedPaths.addAll(diff.getConflicting());

		IgnoreNode diffIgnore = loadDiffIgnore();
		changedPaths.removeIf(path -> isDiffIgnored(path, diffIgnore));

		Map<String, TreeEntry> entries = collectTreeEntries(parentTree);

		if (!changedPaths.isEmpty()) {
			File workTree = repository.getWorkTree();
			Set<String> existing = new HashSet<>();
			for (String path : changedPaths)
				if (new File(workTree, path).exists())
					existing.add(path);
				else
					entries.remove(path);

			if (!existing.isEmpty())
				try (ObjectReader reader = repository.newObjectReader();
						ObjectInserter inserter = repository.newObjectInserter();
						TreeWalk walk = new TreeWalk(repository, reader)) {
					walk.addTree(new FileTreeIterator(repository));
					walk.setRecursive(true);
					walk.setFilter(PathFilterGroup.createFromStrings(existing));

					while (walk.next()) {
						WorkingTreeIterator wti = walk.getTree(0, WorkingTreeIterator.class);
						if (wti == null)
							continue;
						String path = walk.getPathString();
						TreeEntry e = new TreeEntry();
						e.mode = wti.getEntryFileMode();
						try (InputStream in = wti.openEntryStream()) {
							e.id = inserter.insert(Constants.OBJ_BLOB, wti.getEntryLength(), in);
						}
						entries.put(path, e);
					}
					inserter.flush();
				}
		}

		try (ObjectInserter inserter = repository.newObjectInserter()) {
			DirCache inCore = DirCache.newInCore();
			DirCacheBuilder builder = inCore.builder();
			for (Map.Entry<String, TreeEntry> e : entries.entrySet()) {
				DirCacheEntry dce = new DirCacheEntry(e.getKey());
				dce.setFileMode(e.getValue().mode);
				dce.setObjectId(e.getValue().id);
				builder.add(dce);
			}
			builder.finish();
			ObjectId treeId = inCore.writeTree(inserter);
			inserter.flush();
			return treeId;
		}
	}

	private boolean isDiffIgnored(String path, IgnoreNode node) {
		String[] segments = path.split("/");
		StringBuilder sb = new StringBuilder();
		for (int i = 0; i < segments.length; i++) {
			if (i > 0)
				sb.append('/');
			sb.append(segments[i]);
			boolean isDir = i < segments.length - 1;
			if (node.isIgnored(sb.toString(), isDir) == MatchResult.IGNORED)
				return true;
		}
		return false;
	}

	private Map<String, TreeEntry> collectTreeEntries(ObjectId treeId) throws IOException {
		Map<String, TreeEntry> map = new TreeMap<>();
		try (ObjectReader reader = repository.newObjectReader(); TreeWalk walk = new TreeWalk(repository, reader)) {
			walk.addTree(treeId);
			walk.setRecursive(true);
			while (walk.next()) {
				TreeEntry e = new TreeEntry();
				e.mode = walk.getFileMode(0);
				e.id = walk.getObjectId(0);
				map.put(walk.getPathString(), e);
			}
		}
		return map;
	}

	private static class TreeEntry {
		FileMode mode;
		ObjectId id;
	}

	private IgnoreNode loadDiffIgnore() throws IOException {
		IgnoreNode node = new IgnoreNode();
		File workTree = repository.getWorkTree();
		File diffIgnoreFile = new File(workTree, DIFFIGNORE_FILE);
		if (diffIgnoreFile.isFile())
			try (InputStream in = Files.newInputStream(diffIgnoreFile.toPath())) {
				node.parse(in);
			}
		return node;
	}

	public SnapshotResult findLatest() throws IOException {
		Ref best = null;
		int bestTime = Integer.MIN_VALUE;
		for (Ref ref : repository.getRefDatabase().getRefsByPrefix("refs/llm-ops/")) {
			if (!OP_SUFFIX.matcher(ref.getName()).matches())
				continue;

			try (RevWalk walk = new RevWalk(repository)) {
				RevCommit c = walk.parseCommit(ref.getObjectId());
				if (c.getCommitTime() > bestTime) {
					bestTime = c.getCommitTime();
					best = ref;
				}
			}
		}
		if (best == null)
			return null;

		String chain = best.getName().substring(0, best.getName().lastIndexOf('/'));
		try (RevWalk walk = new RevWalk(repository)) {
			RevCommit c = walk.parseCommit(best.getObjectId());
			ObjectId parent = c.getParentCount() > 0 ? c.getParent(0) : best.getObjectId();
			return new SnapshotResult(true, chain, best.getName(), best.getObjectId(), parent, null);
		}
	}

	/**
	 * Reverts the last snapshot commit of the current chain: restores the working
	 * tree to the state of its parent and removes the op-ref, so the next
	 * {@link #snapshot(String)} call continues from that parent.
	 *
	 * @return the resulting current state (for diff display), or {@code null} if
	 *         there is nothing to revert.
	 */
	public synchronized SnapshotResult revertLast() throws IOException {
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

	/** Restores the working tree paths that differ between the two trees to their 'toTree' state. */
	private void checkoutDelta(ObjectId fromTree, ObjectId toTree) throws IOException {
		File workTree = repository.getWorkTree();
		try (ObjectReader reader = repository.newObjectReader(); TreeWalk walk = new TreeWalk(repository, reader)) {
			walk.addTree(fromTree);
			walk.addTree(toTree);
			walk.setRecursive(true);
			while (walk.next()) {
				File file = new File(workTree, walk.getPathString());
				ObjectId toId = walk.getObjectId(1);
				if (toId.equals(ObjectId.zeroId())) {
					Files.deleteIfExists(file.toPath());
					deleteEmptyParents(workTree, file.getParentFile());
				} else {
					file.getParentFile().mkdirs();
					try (InputStream in = reader.open(toId).openStream()) {
						Files.copy(in, file.toPath(), java.nio.file.StandardCopyOption.REPLACE_EXISTING);
					}
					if (walk.getFileMode(1) == FileMode.EXECUTABLE_FILE)
						file.setExecutable(true, false);
				}
			}
		}
	}

	private void deleteEmptyParents(File root, File dir) {
		while (dir != null && !dir.equals(root)) {
			String[] children = dir.list();
			if (children == null || children.length > 0)
				break;
			File parent = dir.getParentFile();
			dir.delete();
			dir = parent;
		}
	}

	private void deleteRef(String name) throws IOException {
		RefUpdate update = repository.updateRef(name);
		update.setForceUpdate(true);
		Result result = update.delete();
		if (result != Result.NEW && result != Result.FORCED && result != Result.FAST_FORWARD
				&& result != Result.NO_CHANGE)
			throw new IOException("Ref-Delete failed for " + name + ": " + result);
	}

	private ObjectId resolveRef(String name) throws IOException {
		Ref ref = repository.exactRef(name);
		return ref == null ? null : ref.getObjectId();
	}

	private void updateRef(String name, ObjectId id) throws IOException {
		RefUpdate update = repository.updateRef(name);
		update.setNewObjectId(id);
		update.setForceUpdate(true);
		Result result = update.update();
		if (result != Result.NEW && result != Result.FORCED && result != Result.FAST_FORWARD
				&& result != Result.NO_CHANGE)
			throw new IOException("Ref-Update failed for " + name + ": " + result);
	}
}