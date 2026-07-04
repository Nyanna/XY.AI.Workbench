package xy.ai.workbench.connectors.claudecode;

import java.util.ArrayList;
import java.util.List;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.Reasoning;

/**
 * Builds JSON structures and CLI commands for the Claude Code Connector.
 * Handles construction of requests, approve/deny messages, and process commands.
 */
public class ClaudeCodeRequestBuilder {

	private static final String SCRIPT = System.getProperty("user.home")
			+ "/xyan/xy.ai.workbench/claude-code/claude-session.sh";

	private final ObjectMapper mapper;
	private final ConfigManager cfg;

	public ClaudeCodeRequestBuilder(ObjectMapper mapper, ConfigManager cfg) {
		this.mapper = mapper;
		this.cfg = cfg;
	}

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

	/**
	 * Builds the CLI command for the Claude Code process.
	 *
	 * @param profile the profile name
	 * @return list of command arguments
	 */
	public List<String> buildCommand(String profile) {
		List<String> cmd = new ArrayList<>();
		cmd.add(SCRIPT);
		cmd.add(cfg.getProfile().name); // Agent Definition
		cmd.add("--profile");
		cmd.add(profile);
		cmd.add("--verbose");
		cmd.add("--include-hook-events");
		cmd.add("--include-partial-messages");
		cmd.add("--input-format");
		cmd.add("stream-json");
		cmd.add("--output-format");
		cmd.add("stream-json");
		cmd.add("--replay-user-messages");
		cmd.add("--model");
		cmd.add(cfg.getModel().apiName);
		if (!Reasoning.Disabled.equals(cfg.getReasoning())) {
			cmd.add("--effort");
			cmd.add(cfg.getReasoning().name().toLowerCase());
		}
		cmd.add("--dangerously-skip-permissions"); // as long there is no permission prompt handling implemented
		return cmd;
	}
}
