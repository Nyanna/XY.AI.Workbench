package xy.ai.workbench.models;

public class TokenStats {
	public long inputToken;
	public long outputToken;
	public long reasoningToken;
	public long totalinToken;
	public long cacheRead;
	public long cacheCreate;

	@Override
	public String toString() {
		return String.format(
				"TokenStats [inputToken=%s, outputToken=%s, reasoningToken=%s, totalInToken=%s, cacheRead=%s, cacheCreate=%s]",
				inputToken, outputToken, reasoningToken, totalinToken, cacheRead, cacheCreate);
	}

	public String print() {
		return String.format("total in: %s, out: %s, reason: %s, read: %s, write: %s, in: %s", totalinToken, inputToken,
				outputToken, reasoningToken, cacheRead, cacheCreate);
	}

	public void add(TokenStats stats) {
		inputToken += stats.inputToken;
		outputToken += stats.outputToken;
		reasoningToken += stats.reasoningToken;
		totalinToken += stats.totalinToken;
		cacheRead += stats.cacheRead;
		cacheCreate += stats.cacheCreate;
	}
}
