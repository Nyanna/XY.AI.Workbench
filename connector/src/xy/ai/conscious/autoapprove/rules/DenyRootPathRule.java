package xy.ai.conscious.autoapprove.rules;

import java.nio.file.Path;
import java.util.Set;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;

/** Denies calls of path-based tools if any path-like argument points at the root directory. */
public class DenyRootPathRule implements IApprovalRule {

	public static final String REASON = "Don't use the root directory.";

	private final Set<String> tools;
	private final Set<String> pathFields;
	private final Path cwd;

	public DenyRootPathRule() {
		this(Set.of(DeepPathReadRule.TOOLS), Set.of(DeepPathReadRule.PATH_FIELDS),
				Path.of("").toAbsolutePath().normalize());
	}

	public DenyRootPathRule(Set<String> tools, Set<String> pathFields, Path cwd) {
		this.tools = Set.copyOf(tools);
		this.pathFields = Set.copyOf(pathFields);
		this.cwd = cwd.toAbsolutePath().normalize();
	}

	@Override
	public Vote vote(ApprovalContext ctx) {
		ApprovalItem item = ctx.item();
		if (item == null || item.tool() == null || !tools.contains(item.tool()))
			return Vote.abstain();
		if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
			return Vote.abstain();
		for (String value : PathArguments.collect(item.arguments(), pathFields)) {
			Path path = PathArguments.resolve(cwd, value);
			if (path != null && PathArguments.isRoot(path))
				return Vote.deny(REASON);
		}
		return Vote.abstain();
	}
}
