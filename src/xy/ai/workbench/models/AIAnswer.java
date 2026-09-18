package xy.ai.workbench.models;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

import xy.ai.workbench.connector.harness.Prompt;

public class AIAnswer {
	public static final String RESULT = "Result Stats: ";

	private static final Pattern ID_PATTERN = Pattern.compile("id=([^,]*), ");

	public final String id;
	public final TokenStats stats = new TokenStats();
	public String answer = "";
	public String instructions = "";
	/** Prompt that produced this answer; null once no longer available (e.g. batch answers). */
	public Prompt prompt;

	public AIAnswer(String id) {
		this.id = id;
	}

	@Override
	public String toString() {
		return String.format("AIAnswer [stats=%s]", stats);
	}

	public String print() {
		return String.format("%sid=%s, %s", RESULT, id, stats.print());
	}

	public static AIAnswer fromString(String line) {
		if (line == null)
			return null;
		Matcher idMatcher = ID_PATTERN.matcher(line);
		if (!idMatcher.find())
			return null;
		TokenStats stats = TokenStats.fromString(line.substring(idMatcher.end()));
		if (stats == null)
			return null;
		AIAnswer ans = new AIAnswer(idMatcher.group(1));
		ans.stats.add(stats);
		return ans;
	}
}
