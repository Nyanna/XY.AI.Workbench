package xy.ai.workbench.connector.harness;

import com.fasterxml.jackson.databind.JsonNode;

/** A tool invocation, extracted from session text or produced by a model answer. */
public final class ToolCall {
	public final String id;
	public final String name;
	public final JsonNode arguments;

	public ToolCall(String id, String name, JsonNode arguments) {
		this.id = id;
		this.name = name;
		this.arguments = arguments;
	}
}
