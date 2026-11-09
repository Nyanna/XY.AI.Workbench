package xy.ai.conscious.autoapprove.rules;

import java.nio.file.Path;
import java.util.List;
import java.util.Set;

import com.fasterxml.jackson.databind.JsonNode;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;

public class DeepPathReadRule implements IApprovalRule {

	public static final String ID = DeepPathReadRule.class.getSimpleName();
	public static final String TOOLS_SETTING = "tools";
	public static final String PATH_FIELDS_SETTING = "pathFields";
	public static final String MIN_DEPTH_SETTING = "minDepth";
	public static final String MAX_LENGTH_SETTING = "maxLength";

	public static final String[] TOOLS = { "search", "list" };
	public static final String[] PATH_FIELDS = { "path", "directory" };
	public static final int MIN_DEPTH = 3;
	public static final int MAX_LENGTH = 3000;

	private final RuleSettings defaults;
	private final Path cwd;

	public DeepPathReadRule() {
		this(Set.of(TOOLS), Set.of(PATH_FIELDS), MIN_DEPTH, Path.of("").toAbsolutePath().normalize());
	}

	public DeepPathReadRule(Set<String> tools, Set<String> pathFields, int minDepth, Path cwd) {
		this.defaults = new RuleSettings() //
				.addStrings(TOOLS_SETTING, tools.stream().sorted().toList()) //
				.addStrings(PATH_FIELDS_SETTING, pathFields.stream().sorted().toList()) //
				.addNumber(MIN_DEPTH_SETTING, minDepth) //
				.addNumber(MAX_LENGTH_SETTING, MAX_LENGTH);
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
		if (item == null)
			return Vote.abstain();
		RuleSettings settings = ctx.settings(ID);
		Set<String> tools = settings.getStringSet(TOOLS_SETTING);
		switch (item.kind()) {
		case ControlCall, ToolCall:
			return item.tool() != null && tools.contains(item.tool()) && allDeep(item, settings) ? Vote.allow() : Vote.abstain();
		case ControlResult, ToolResult:
			ApprovalItem call = ctx.call();
			if (call == null || call.tool() == null || !tools.contains(call.tool()) || !allDeep(call, settings))
				return Vote.abstain();
			return length(item) < settings.getNumber(MAX_LENGTH_SETTING) ? Vote.allow() : Vote.abstain();
		default:
			return Vote.abstain();
		}
	}

	private boolean allDeep(ApprovalItem call, RuleSettings settings) {
		List<String> values = PathArguments.collect(call.arguments(), settings.getStringSet(PATH_FIELDS_SETTING));
		if (values.isEmpty())
			return false;
		long minDepth = settings.getNumber(MIN_DEPTH_SETTING);
		for (String value : values) {
			Path path = PathArguments.resolve(cwd, value);
			if (path == null || !path.startsWith(cwd) || cwd.relativize(path).getNameCount() < minDepth)
				return false;
		}
		return true;
	}

	private static int length(ApprovalItem item) {
		JsonNode result = item.result();
		if (result.isMissingNode())
			return item.yaml().length();
		return result.isTextual() ? result.asText().length() : result.toString().length();
	}
}
