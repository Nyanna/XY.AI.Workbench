package xy.ai.conscious.autoapprove.rules;

import java.util.Set;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;

/**
 * Allows every call of the tools listed in {@link #ALLOWED_TOOLS}, regardless
 * of their arguments.
 */
public class AllowedToolsRule implements IApprovalRule {

	public static final String[] ALLOWED_TOOLS = { "ast_outline", "ast_read" };

	private final Set<String> allowed;

	public AllowedToolsRule() {
		this(ALLOWED_TOOLS);
	}

	public AllowedToolsRule(String... toolNames) {
		this.allowed = Set.of(toolNames);
	}

	@Override
	public Vote vote(ApprovalContext ctx) {
		ApprovalItem item = ctx.item();
		if (item == null || item.tool() == null)
			return Vote.abstain();
		if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
			return Vote.abstain();
		return allowed.contains(item.tool()) ? Vote.allow() : Vote.abstain();
	}
}
