package xy.ai.conscious.autoapprove.rules;

import java.nio.file.Path;
import java.util.Set;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;

/**
 * Holds the automatic approval of every tool call with a path argument that
 * resolves outside of the project directory, so the operator decides.
 */
public class HoldOutsideProjectRule implements IApprovalRule {

	public static final String ID = HoldOutsideProjectRule.class.getSimpleName();
	public static final String PATH_FIELDS_SETTING = "pathFields";

	public static final String[] PATH_FIELDS = { "path", "paths", "directory", "file", "files" };

	private final RuleSettings defaults;
	private final Path cwd;

	public HoldOutsideProjectRule() {
		this(Set.of(PATH_FIELDS), Path.of("").toAbsolutePath().normalize());
	}

	public HoldOutsideProjectRule(Set<String> pathFields, Path cwd) {
		this.defaults = new RuleSettings() //
				.addStrings(PATH_FIELDS_SETTING, pathFields.stream().sorted().toList());
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
		if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
			return Vote.abstain();
		Set<String> pathFields = ctx.settings(ID).getStringSet(PATH_FIELDS_SETTING);
		for (String value : PathArguments.collect(item.arguments(), pathFields)) {
			if (isOutside(value))
				return Vote.hold();
		}
		return Vote.abstain();
	}

	private boolean isOutside(String value) {
		if (value.startsWith("~"))
			return true;
		Path path = PathArguments.resolve(cwd, value);
		return path == null || !path.startsWith(cwd);
	}
}
