package xy.ai.workbench;

public class AIAnswer {
	public String id;
	public long inputToken;
	public long outputToken;
	public long reasoningToken;
	public long totalToken;
	public String answer = "";
	public String instructions = "";

	@Override
	public String toString() {
		return String.format("AIAnswer [inputToken=%s, outputToken=%s, reasoningToken=%s, totalToken=%s]", inputToken,
				outputToken, reasoningToken, totalToken);
	}

	public String print() {
		return String.format("in: %st, out: %st, reas: %st \ntotal: %st", inputToken,
				outputToken, reasoningToken, totalToken);
	}

}
