package xy.ai.conscious.autoapprove.rules;

import java.util.Set;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;

public class AllowedToolsRule implements IApprovalRule {

	public static final String ID = AllowedToolsRule.class.getSimpleName();
	public static final String TOOLS = "tools";
	public static final String[] ALLOWED_TOOLS = { "ast_outline", "ast_read", "ast_validate", "run_command", "ast_find" };

	private final RuleSettings defaults;

	public AllowedToolsRule() {
		this(ALLOWED_TOOLS);
	}

	public AllowedToolsRule(String... toolNames) {
		this.defaults = new RuleSettings().addStrings(TOOLS, toolNames);
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
		if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
			return Vote.abstain();
		Set<String> allowed = ctx.settings(ID).getStringSet(TOOLS);
		return allowed.contains(item.tool()) ? Vote.allow() : Vote.abstain();
	}
}
