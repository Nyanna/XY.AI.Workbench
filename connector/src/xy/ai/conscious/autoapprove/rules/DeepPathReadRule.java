package xy.ai.conscious.autoapprove.rules;

import java.nio.file.Path;
import java.util.List;
import java.util.Set;

import xy.ai.conscious.autoapprove.ApprovalContext;
import xy.ai.conscious.autoapprove.ApprovalItem;

import com.fasterxml.jackson.databind.JsonNode;

/**
 * Allows calls of path-based read tools (e.g. search, list) and their results
 * shorter than {@link #MAX_LENGTH} characters, if every path-like argument
 * points at least {@code minDepth} levels below the working directory.
 */
public class DeepPathReadRule implements IApprovalRule {

	public static final String[] TOOLS = { "search", "list" };
	public static final String[] PATH_FIELDS = { "path", "directory" };
	public static final int MIN_DEPTH = 3;
	public static final int MAX_LENGTH = 3000;

	private final Set<String> tools;
	private final Set<String> pathFields;
	private final int minDepth;
	private final Path cwd;

	public DeepPathReadRule() {
		this(Set.of(TOOLS), Set.of(PATH_FIELDS), MIN_DEPTH, Path.of("").toAbsolutePath().normalize());
	}

	public DeepPathReadRule(Set<String> tools, Set<String> pathFields, int minDepth, Path cwd) {
		this.tools = Set.copyOf(tools);
		this.pathFields = Set.copyOf(pathFields);
		this.minDepth = minDepth;
		this.cwd = cwd.toAbsolutePath().normalize();
	}

	@Override
	public Vote vote(ApprovalContext ctx) {
		ApprovalItem item = ctx.item();
		if (item == null)
			return Vote.abstain();
		switch (item.kind()) {
		case ControlCall, ToolCall:
			return item.tool() != null && tools.contains(item.tool()) && allDeep(item) ? Vote.allow() : Vote.abstain();
		case ControlResult, ToolResult:
			ApprovalItem call = ctx.call();
			if (call == null || call.tool() == null || !tools.contains(call.tool()) || !allDeep(call))
				return Vote.abstain();
			return length(item) < MAX_LENGTH ? Vote.allow() : Vote.abstain();
		default:
			return Vote.abstain();
		}
	}

	private boolean allDeep(ApprovalItem call) {
		List<String> values = PathArguments.collect(call.arguments(), pathFields);
		if (values.isEmpty())
			return false;
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
