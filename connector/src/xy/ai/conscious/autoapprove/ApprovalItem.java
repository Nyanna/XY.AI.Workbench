package xy.ai.conscious.autoapprove;

import com.fasterxml.jackson.databind.JsonNode;

/**
 * A tool call or tool result detected in an answer.
 *
 * @param data parsed YAML block
 * @param yaml raw YAML block content without fences
 */
public record ApprovalItem(Kind kind, String id, String tool, JsonNode data, String yaml) {

	public enum Kind {
		ControlCall, ControlResult, ToolCall, ToolResult
	}

	public JsonNode arguments() {
		return data.path("arguments");
	}

	public JsonNode result() {
		return data.path("result");
	}

	/** Item belongs to a control request (MCP bound connector). */
	public boolean isControl() {
		return kind == Kind.ControlCall || kind == Kind.ControlResult;
	}
}
