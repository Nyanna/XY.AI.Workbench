package xy.ai.workbench.models;

public class AIAnswer {
	public final String id;
	public long inputToken;
	public long outputToken;
	public long reasoningToken;
	public long totalToken;
	public long cacheRead;
	public long cacheCreate;
	public String answer = "";
	public String instructions = "";
	
	public AIAnswer(String id) {
		this.id = id;
	}

	@Override
	public String toString() {
		return String.format("AIAnswer [inputToken=%s, outputToken=%s, reasoningToken=%s, totalToken=%s, cacheRead=%s, cacheCreate=%s]", inputToken,
				outputToken, reasoningToken, totalToken, cacheRead, cacheCreate);
	}

	public String print() {
		return String.format("in: %st, out: %st, reas: %st \ntotal: %st, Cache: %st/%st", inputToken,
				outputToken, reasoningToken, totalToken, cacheRead, cacheCreate);
	}

}
