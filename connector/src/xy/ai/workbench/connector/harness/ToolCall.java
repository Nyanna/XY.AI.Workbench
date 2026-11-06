package xy.ai.workbench.connector.harness;

import com.fasterxml.jackson.databind.JsonNode;

public class ToolCall {
	public final String id;
	public final String name;
	public final JsonNode arguments;

	public ToolCall(String id, String name, JsonNode arguments) {
		this.id = id;
		this.name = name;
		this.arguments = arguments;
	}
}
