package xy.ai.workbench.models;

import xy.ai.workbench.connector.harness.Prompt;

public class AIAnswer {
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
		return stats.print();
	}
}
