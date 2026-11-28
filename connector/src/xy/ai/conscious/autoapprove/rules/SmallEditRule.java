package xy.ai.conscious.autoapprove.rules;

import java.nio.file.Path;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;
import xy.ai.conscious.autoapprove.ApprovalItem.Kind;
import xy.ai.conscious.connector.claudecode.ControlApproval;

/**
 * Allows small calls of edit tools on files an earlier call of such a tool was
 * approved for by the operator in the same session. Approvals of other tools do
 * not count.
 */
public class SmallEditRule implements IApprovalRule {

	public static final String ID = SmallEditRule.class.getSimpleName();
	public static final String TOOLS_SETTING = "tools";
	public static final String PATH_FIELDS_SETTING = "pathFields";
	public static final String MAX_LENGTH_SETTING = "maxLength";

	public static final String[] TOOLS = { "ast_edit", "write", "edit_marks", "edit_line", "edit_block", "edit_lines", "ast_create", "ast_edit_block", "ast_edit_marks", "ast_insert", "ast_replace" };
	public static final String[] PATH_FIELDS = { "path", "paths", "target", "directory", "file", "files" };
	public static final int MAX_LENGTH = 2000;

	private final RuleSettings defaults;
	private final Path cwd;
	private final ApprovalLedger ledger = new ApprovalLedger();

	public SmallEditRule() {
		this(Set.of(TOOLS), Set.of(PATH_FIELDS), MAX_LENGTH, Path.of("").toAbsolutePath().normalize());
	}

	public SmallEditRule(Set<String> tools, Set<String> pathFields, long maxLength, Path cwd) {
		this.defaults = new RuleSettings() //
				.addStrings(TOOLS_SETTING, tools.stream().sorted().toList()) //
				.addStrings(PATH_FIELDS_SETTING, pathFields.stream().sorted().toList()) //
				.addNumber(MAX_LENGTH_SETTING, maxLength);
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
	public void onApproval(ControlApproval approval) {
		if (!approval.rejected())
			ledger.approve(approval.id(), approval.arguments());
	}

	@Override
	public Vote vote(ApprovalContext ctx) {
		ApprovalItem item = ctx.item();
		if (item == null || item.tool() == null)
			return Vote.abstain();
		if (item.kind() != Kind.ControlCall && item.kind() != Kind.ToolCall)
			return Vote.abstain();
		RuleSettings settings = ctx.settings(ID);
		if (!settings.getStringSet(TOOLS_SETTING).contains(item.tool()))
			return Vote.abstain();
		ledger.track(item);
		if (item.arguments().toString().length() >= settings.getNumber(MAX_LENGTH_SETTING))
			return Vote.abstain();
		Set<String> pathFields = settings.getStringSet(PATH_FIELDS_SETTING);
		List<String> targets = PathArguments.collect(item.arguments(), pathFields);
		if (targets.isEmpty())
			return Vote.abstain();
		Set<Path> approved = approvedPaths(ledger.approved(item.id()), pathFields);
		for (String target : targets) {
			Path path = PathArguments.resolve(cwd, target);
			if (path == null || !approved.contains(path))
				return Vote.abstain();
		}
		return Vote.allow();
	}

	private Set<Path> approvedPaths(List<ApprovalLedger.Approved> calls, Set<String> pathFields) {
		Set<Path> paths = new HashSet<>();
		for (ApprovalLedger.Approved call : calls) {
			for (String value : PathArguments.collect(call.arguments(), pathFields)) {
				Path path = PathArguments.resolve(cwd, value);
				if (path != null)
					paths.add(path);
			}
		}
		return paths;
	}
}
