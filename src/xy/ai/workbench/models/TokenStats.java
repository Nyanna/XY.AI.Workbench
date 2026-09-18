package xy.ai.workbench.models;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class TokenStats {
	public long inputToken;
	public long outputToken;
	public long reasoningToken;
	public long totalToken;
	public long cacheRead;
	public long cacheCreate;

	@Override
	public String toString() {
		return String.format(
				"TokenStats [inputToken=%s, outputToken=%s, reasoningToken=%s, totalInToken=%s, cacheRead=%s, cacheCreate=%s]",
				inputToken, outputToken, reasoningToken, totalToken, cacheRead, cacheCreate);
	}

	private static final Pattern PATTERN = Pattern
			.compile("total: (\\d+), in: (\\d+), out: (\\d+), reason: (\\d+), read: (\\d+), write: (\\d+)");

	public String print() {
		return String.format("total: %s, in: %s, out: %s, reason: %s, read: %s, write: %s", totalToken, inputToken,
				outputToken, reasoningToken, cacheRead, cacheCreate);
	}

	/**
	 * Parses stats previously produced by {@link #print()}, or {@code null} when
	 * {@code s} does not contain such a fragment.
	 */
	public static TokenStats fromString(String s) {
		if (s == null)
			return null;
		Matcher m = PATTERN.matcher(s);
		if (!m.find())
			return null;
		TokenStats stats = new TokenStats();
		stats.totalToken = Long.parseLong(m.group(1));
		stats.inputToken = Long.parseLong(m.group(2));
		stats.outputToken = Long.parseLong(m.group(3));
		stats.reasoningToken = Long.parseLong(m.group(4));
		stats.cacheRead = Long.parseLong(m.group(5));
		stats.cacheCreate = Long.parseLong(m.group(6));
		return stats;
	}

	public void add(TokenStats stats) {
		inputToken += stats.inputToken;
		outputToken += stats.outputToken;
		reasoningToken += stats.reasoningToken;
		totalToken += stats.totalToken;
		cacheRead += stats.cacheRead;
		cacheCreate += stats.cacheCreate;
	}
}
