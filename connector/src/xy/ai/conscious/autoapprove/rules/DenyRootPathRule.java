package xy.ai.conscious.autoapprove.rules;

import java.nio.file.Path;
import java.util.Set;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;

/**
 * Denies the root directory for the tools and path fields of
 * {@link DeepPathReadRule}.
 */
public class DenyRootPathRule implements IApprovalRule {

	public static final String ID = DenyRootPathRule.class.getSimpleName();
	public static final String REASON = "Don't use the root directory.";

	private final RuleSettings defaults = new RuleSettings();
	private final Path cwd;

	public DenyRootPathRule() {
		this(Path.of("").toAbsolutePath().normalize());
	}

	public DenyRootPathRule(Path cwd) {
		this.cwd = cwd.toAbsolutePath().normalize();
	}

	@Override
	public String id() {
		return ID;
	}

	@Override
	public RuleSettings getDefaultSetting() {
		return defaults.clone();
	}

	@Override
	public Vote vote(ApprovalContext ctx) {
		ApprovalItem item = ctx.item();
		if (item == null || item.tool() == null)
			return Vote.abstain();
		RuleSettings deep = ctx.settings(DeepPathReadRule.ID);
		if (!deep.getStringSet(DeepPathReadRule.TOOLS_SETTING).contains(item.tool()))
			return Vote.abstain();
		if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
			return Vote.abstain();
		Set<String> pathFields = deep.getStringSet(DeepPathReadRule.PATH_FIELDS_SETTING);
		for (String value : PathArguments.collect(item.arguments(), pathFields)) {
			Path path = PathArguments.resolve(cwd, value);
			if (path != null && PathArguments.isRoot(path))
				return Vote.deny(REASON);
		}
		return Vote.abstain();
	}
}
