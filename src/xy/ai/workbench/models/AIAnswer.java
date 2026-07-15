package xy.ai.workbench.models;

public class AIAnswer {
	public final String id;
	public final TokenStats stats = new TokenStats();
	public String answer = "";
	public String instructions = "";

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
