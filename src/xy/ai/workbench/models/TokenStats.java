package xy.ai.workbench.models;

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
				"TokenStats [inputToken=%s, outputToken=%s, reasoningToken=%s, totalToken=%s, cacheRead=%s, cacheCreate=%s]",
				inputToken, outputToken, reasoningToken, totalToken, cacheRead, cacheCreate);
	}

	public String print() {
		return String.format("in: %st, out: %st, reas: %st \ntotal: %st, Cache: %st/%st", inputToken, outputToken,
				reasoningToken, totalToken, cacheRead, cacheCreate);
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
