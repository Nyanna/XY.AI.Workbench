package xy.ai.workbench.connectors.claudecode;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

/**
 * Builds JSON structures and CLI commands for the Claude Code Connector.
 * Handles construction of requests, approve/deny messages, and process commands.
 */
public class ClaudeCodeRequestBuilder {

    private final ObjectMapper mapper = new ObjectMapper();

	/**
	 * Builds a prompt JSON structure for a user message.
	 *
	 * @param text the prompt text
	 * @return JSON string representation
	 * @throws IllegalStateException if JSON serialization fails
	 */
	public String buildPromptJson(String text) {
		try {
			ObjectNode root = mapper.createObjectNode();
			root.put("type", "user");
			ObjectNode message = root.putObject("message");
			message.put("role", "user");
			ArrayNode content = message.putArray("content");
			ObjectNode textNode = content.addObject();
			textNode.put("type", "text");
			textNode.put("text", text);
			return mapper.writeValueAsString(root);
		} catch (Exception e) {
			throw new IllegalStateException("Failed to build prompt JSON", e);
		}
	}

	/**
	 * Builds a JSON structure to approve a tool use request.
	 *
	 * @param toolUseId the tool use ID to approve
	 * @return JSON string representation
	 * @throws IllegalStateException if JSON serialization fails
	 */
	public String buildApproveJson(String toolUseId) {
		try {
			ObjectNode node = mapper.createObjectNode();
			node.put("type", "approve");
			node.put("tool_use_id", toolUseId);
			return mapper.writeValueAsString(node);
		} catch (Exception e) {
			throw new IllegalStateException("Failed to build approve JSON", e);
		}
	}

	/**
	 * Builds a JSON structure to deny a tool use request.
	 *
	 * @param toolUseId the tool use ID to deny
	 * @return JSON string representation
	 * @throws IllegalStateException if JSON serialization fails
	 */
	public String buildDenyJson(String toolUseId) {
		try {
			ObjectNode node = mapper.createObjectNode();
			node.put("type", "deny");
			node.put("tool_use_id", toolUseId);
			return mapper.writeValueAsString(node);
		} catch (Exception e) {
			throw new IllegalStateException("Failed to build deny JSON", e);
		}
	}
}
