package xy.ai.conscious.autoapprove.rules;

import java.util.Set;

import com.fasterxml.jackson.databind.JsonNode;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;

/**
 * Allows small results of the tools allowed by {@link AllowedToolsRule}.
 */
public class SmallResultRule implements IApprovalRule {

	public static final String ID = SmallResultRule.class.getSimpleName();
	public static final String MAX_LENGTH_SETTING = "maxLength";
	public static final int MAX_LENGTH = 3000;

	private final RuleSettings defaults;

	public SmallResultRule() {
		this(MAX_LENGTH);
	}

	public SmallResultRule(long maxLength) {
		this.defaults = new RuleSettings().addNumber(MAX_LENGTH_SETTING, maxLength);
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
		ApprovalItem call = ctx.call();
		if (item == null || call == null || call.tool() == null)
			return Vote.abstain();
		if (item.kind() != Kind.ControlResult && item.kind() != Kind.ToolResult)
			return Vote.abstain();
		Set<String> allowed = ctx.settings(AllowedToolsRule.ID).getStringSet(AllowedToolsRule.TOOLS);
		if (!allowed.contains(call.tool()))
			return Vote.abstain();
		return length(item) < ctx.settings(ID).getNumber(MAX_LENGTH_SETTING) ? Vote.allow() : Vote.abstain();
	}

	private static int length(ApprovalItem item) {
		JsonNode result = item.result();
		if (result.isMissingNode())
			return item.yaml().length();
		return result.isTextual() ? result.asText().length() : result.toString().length();
	}
}
