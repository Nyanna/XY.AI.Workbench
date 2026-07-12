package xy.ai.workbench.views.explorer;

import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.runtime.IPath;
import org.eclipse.core.runtime.Path;

/**
 * Determines whether a workspace resource is excluded by one or more
 * ".gitignore" files located between the project root and the resource
 * itself, applying the same precedence rules as Git (rules of a
 * ".gitignore" file closer to the resource, respectively lines further down
 * within a file, take precedence over earlier ones; "!" negates a
 * previous match).
 */
public final class GitIgnoreFilter {

	private GitIgnoreFilter() {
	}

	private static final Map<String, CachedRules> CACHE = new HashMap<>();

	public static boolean isIgnored(IResource resource) {
		if (resource == null || resource.getType() == IResource.PROJECT)
			return false;

		IPath relativePath = resource.getProjectRelativePath();
		if (relativePath == null || relativePath.isEmpty())
			return false;

		String[] segments = relativePath.segments();
		if (segments.length > 0 && ".git".equals(segments[0]))
			return true;

		boolean isDirectory = resource instanceof IContainer;
		boolean ignored = false;
		IContainer dir = resource.getProject();

		for (int i = 0; i < segments.length; i++) {
			List<GitIgnoreRule> rules = getRules(dir);
			if (!rules.isEmpty()) {
				StringBuilder relPath = new StringBuilder();
				for (int j = i; j < segments.length; j++) {
					if (relPath.length() > 0)
						relPath.append('/');
					relPath.append(segments[j]);
				}
				String path = relPath.toString();
				for (GitIgnoreRule rule : rules)
					if (rule.matches(path, isDirectory))
						ignored = !rule.isNegate();
			}
			if (i < segments.length - 1) {
				IResource child = dir.findMember(segments[i]);
				if (child instanceof IContainer)
					dir = (IContainer) child;
				else
					break;
			}
		}
		return ignored;
	}

	private static List<GitIgnoreRule> getRules(IContainer dir) {
		IFile gitignore = dir.getFile(new Path(".gitignore"));
		if (!gitignore.exists())
			return Collections.emptyList();

		String key = gitignore.getFullPath().toString();
		long stamp = gitignore.getModificationStamp();

		synchronized (CACHE) {
			CachedRules cached = CACHE.get(key);
			if (cached != null && cached.stamp == stamp)
				return cached.rules;

			List<GitIgnoreRule> rules = parse(gitignore);
			CACHE.put(key, new CachedRules(stamp, rules));
			return rules;
		}
	}

	private static List<GitIgnoreRule> parse(IFile gitignore) {
		List<GitIgnoreRule> rules = new ArrayList<>();
		try (var in = gitignore.getContents()) {
			String content = new String(in.readAllBytes(), StandardCharsets.UTF_8);
			for (String line : content.split("\r?\n")) {
				GitIgnoreRule rule = GitIgnoreRule.parse(line);
				if (rule != null)
					rules.add(rule);
			}
		} catch (Exception e) {
			// unreadable ".gitignore" files are simply ignored
		}
		return rules;
	}

	private static final class CachedRules {
		final long stamp;
		final List<GitIgnoreRule> rules;

		CachedRules(long stamp, List<GitIgnoreRule> rules) {
			this.stamp = stamp;
			this.rules = rules;
		}
	}
}
