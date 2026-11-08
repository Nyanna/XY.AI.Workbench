package xy.ai.conscious.autoapprove.rules;

import java.util.Locale;
import java.util.Set;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;

/**
 * Holds back calls of auto-approved tools that target markdown files (prompts, documentation), so they
 * are not approved automatically.
 */
public class HoldMarkdownRule implements IApprovalRule {

	public static final String[] PATH_FIELDS = { "path", "paths" };
	public static final String[] PATTERN_FIELDS = { "include" };

	private static final String EXTENSION = ".md";

	private final Set<String> tools;
	private final Set<String> pathFields;
	private final Set<String> patternFields;

	public HoldMarkdownRule() {
		this(Set.of(AllowedToolsRule.ALLOWED_TOOLS), Set.of(PATH_FIELDS), Set.of(PATTERN_FIELDS));
	}

	public HoldMarkdownRule(Set<String> tools, Set<String> pathFields, Set<String> patternFields) {
		this.tools = Set.copyOf(tools);
		this.pathFields = Set.copyOf(pathFields);
		this.patternFields = Set.copyOf(patternFields);
	}

	@Override
	public Vote vote(ApprovalContext ctx) {
		ApprovalItem item = ctx.item();
		if (item == null || item.tool() == null || !tools.contains(item.tool()))
			return Vote.abstain();
		if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
			return Vote.abstain();
		for (String value : PathArguments.collect(item.arguments(), pathFields)) {
			if (value.toLowerCase(Locale.ROOT).endsWith(EXTENSION))
				return Vote.hold();
		}
		for (String value : PathArguments.collect(item.arguments(), patternFields)) {
			if (value.toLowerCase(Locale.ROOT).contains(EXTENSION))
				return Vote.hold();
		}
		return Vote.abstain();
	}
}
