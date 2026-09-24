package xy.ai.workbench.view.diff;

import java.io.File;
import java.io.IOException;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.eclipse.core.resources.IProject;
import org.eclipse.core.resources.IWorkspaceRoot;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.jgit.api.Git;
import org.eclipse.jgit.api.errors.GitAPIException;
import org.eclipse.jgit.dircache.DirCache;
import org.eclipse.jgit.lib.CommitBuilder;
import org.eclipse.jgit.lib.ObjectId;
import org.eclipse.jgit.lib.ObjectInserter;
import org.eclipse.jgit.lib.PersonIdent;
import org.eclipse.jgit.lib.Ref;
import org.eclipse.jgit.lib.RefUpdate;
import org.eclipse.jgit.lib.RefUpdate.Result;
import org.eclipse.jgit.lib.Repository;
import org.eclipse.jgit.lib.RepositoryBuilder;
import org.eclipse.jgit.revwalk.RevCommit;
import org.eclipse.jgit.revwalk.RevWalk;

public class OpSnapshotter {

	private static final String BASE_MARKER = "refs/llm-ops/_base";
	private static final Pattern OP_SUFFIX = Pattern.compile(".*/op-(\\d+)$");
	private static final Map<String, Repository> REPOSITORY_CACHE = new ConcurrentHashMap<>();
	private static final DateTimeFormatter TS = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

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
			RepositoryBuilder gitDir = new RepositoryBuilder() //
					.setGitDir(start) //
					.readEnvironment() //
					.findGitDir();
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

		try (Git git = new Git(repository)) {
			git.add().addFilepattern(".").call();
			git.add().addFilepattern(".").setUpdate(true).call();
		} catch (GitAPIException e) {
			throw new IOException(e);
		}

		ObjectId treeId;
		try (ObjectInserter inserter = repository.newObjectInserter()) {
			DirCache cache = repository.readDirCache();
			treeId = cache.writeTree(inserter);
			inserter.flush();
		}

		ObjectId parentTree;
		try (RevWalk walk = new RevWalk(repository)) {
			parentTree = walk.parseCommit(parent).getTree();
		}

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
			throw new IOException("Ref-Update fehlgeschlagen fuer " + name + ": " + result);
	}
}
