package xy.ai.workbench.connector.harness;

import com.fasterxml.jackson.databind.JsonNode;

/**
 * Accumulates a model answer into the fixed markdown/YAML output format
 * ({@link SessionRenderer}), mirroring what {@link SessionProcessor} parses
 * back on the next turn. Replaces the ad-hoc StringBuffer concatenation
 * previously duplicated in every connector's {@code convertResponse}.
 */
public final class SessionAnswerBuilder {

	private final StringBuilder out = new StringBuilder();

	private SessionAnswerBuilder separate() {
		if (out.length() > 0)
			out.append("\n\n");
		return this;
	}

	public SessionAnswerBuilder text(String text) {
		if (text == null || text.isBlank())
			return this;
		return separate().appendRaw(SessionRenderer.text(text));
	}

	public SessionAnswerBuilder reasoning(String text) {
		if (text == null || text.isBlank())
			return this;
		return separate().appendRaw(SessionRenderer.reasoning(text));
	}

	public SessionAnswerBuilder toolCall(String id, String name, JsonNode arguments) {
		return separate().appendRaw(SessionRenderer.toolCall(new ToolCall(id, name, arguments)));
	}

	public SessionAnswerBuilder toolResult(String id, String content) {
		return separate().appendRaw(SessionRenderer.toolResult(new ToolResult(id, content)));
	}

	private SessionAnswerBuilder appendRaw(String text) {
		out.append(text);
		return this;
	}

	public boolean isEmpty() {
		return out.length() == 0;
	}

	@Override
	public String toString() {
		return out.toString();
	}
}
