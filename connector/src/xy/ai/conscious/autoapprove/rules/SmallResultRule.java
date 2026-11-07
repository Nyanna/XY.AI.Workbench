package xy.ai.conscious.autoapprove.rules;

import java.util.Set;
import com.fasterxml.jackson.databind.JsonNode;
import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;

/**
 * Allows results shorter than {@link #MAX_LENGTH} characters if the tool call they belong to is listed in
 * {@link AllowedToolsRule#ALLOWED_TOOLS}. Larger results are left to the operator.
 */
public class SmallResultRule implements IApprovalRule {

	public static final int MAX_LENGTH = 3000;

	private final Set<String> allowed;

	public SmallResultRule() {
		this(AllowedToolsRule.ALLOWED_TOOLS);
	}

	public SmallResultRule(String... toolNames) {
		this.allowed = Set.of(toolNames);
	}

	@Override
	public Vote vote(ApprovalContext ctx) {
		ApprovalItem item = ctx.item();
		ApprovalItem call = ctx.call();
		if (item == null || call == null || call.tool() == null)
			return Vote.abstain();
		if (item.kind() != Kind.ControlResult && item.kind() != Kind.ToolResult)
			return Vote.abstain();
		if (!allowed.contains(call.tool()))
			return Vote.abstain();
		return length(item) < MAX_LENGTH ? Vote.allow() : Vote.abstain();
	}

	private static int length(ApprovalItem item) {
		JsonNode result = item.result();
		if (result.isMissingNode())
			return item.yaml().length();
		return result.isTextual() ? result.asText().length() : result.toString().length();
	}
}
