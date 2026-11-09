package xy.ai.conscious.autoapprove.rules;

import java.util.Locale;
import java.util.Set;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;

/**
 * Holds the automatic approval for markdown access with the tools allowed by
 * {@link AllowedToolsRule}.
 */
public class HoldMarkdownRule implements IApprovalRule {

	public static final String ID = HoldMarkdownRule.class.getSimpleName();
	public static final String PATH_FIELDS_SETTING = "pathFields";
	public static final String PATTERN_FIELDS_SETTING = "patternFields";

	public static final String[] PATH_FIELDS = { "path", "paths" };
	public static final String[] PATTERN_FIELDS = { "include" };

	private static final String EXTENSION = ".md";

	private final RuleSettings defaults;

	public HoldMarkdownRule() {
		this(Set.of(PATH_FIELDS), Set.of(PATTERN_FIELDS));
	}

	public HoldMarkdownRule(Set<String> pathFields, Set<String> patternFields) {
		this.defaults = new RuleSettings() //
				.addStrings(PATH_FIELDS_SETTING, pathFields.stream().sorted().toList()) //
				.addStrings(PATTERN_FIELDS_SETTING, patternFields.stream().sorted().toList());
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
		if (!ctx.settings(AllowedToolsRule.ID).getStringSet(AllowedToolsRule.TOOLS).contains(item.tool()))
			return Vote.abstain();
		if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
			return Vote.abstain();
		RuleSettings settings = ctx.settings(ID);
		for (String value : PathArguments.collect(item.arguments(), settings.getStringSet(PATH_FIELDS_SETTING))) {
			if (value.toLowerCase(Locale.ROOT).endsWith(EXTENSION))
				return Vote.hold();
		}
		for (String value : PathArguments.collect(item.arguments(), settings.getStringSet(PATTERN_FIELDS_SETTING))) {
			if (value.toLowerCase(Locale.ROOT).contains(EXTENSION))
				return Vote.hold();
		}
		return Vote.abstain();
	}
}
