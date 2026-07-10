package xy.ai.workbench.connectors.claudecode;

import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

/**
 * Builds JSON structures and CLI commands for the Claude Code Connector.
 * Handles construction of requests, approve/deny messages, and process commands.
 */
public class ClaudeCodeRequestBuilder {

	/**
	 * Builds a prompt JSON structure for a user message. The prompt text is added
	 * via the shared mapper (see {@link JsonUtil}), which escapes it correctly and
	 * exactly once — no manual quoting.
	 *
	 * @param text the prompt text
	 * @return JSON string representation
	 * @throws IllegalStateException if JSON serialization fails
	 */
	public String buildPromptJson(String text) {
		try {
			ObjectNode root = JsonUtil.mapper().createObjectNode();
			root.put("type", "user");
			ObjectNode message = root.putObject("message");
			message.put("role", "user");
			ArrayNode content = message.putArray("content");
			ObjectNode textNode = content.addObject();
			textNode.put("type", "text");
			textNode.put("text", text);
			return JsonUtil.mapper().writeValueAsString(root);
		} catch (Exception e) {
			throw new IllegalStateException("Failed to build prompt JSON", e);
		}
	}

}
